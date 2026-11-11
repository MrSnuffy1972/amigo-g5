# Game 1750 (a-train (1992)(ocean)(de)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01750). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1804112 top-level bytes in 107 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 51 of 51 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 4474 of 4474 named (0 author symbols), 4474 of 4474 described (220 by rule, 4185 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1627 of 1627 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 102323 of 102340 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 894836 |
| code | 328386 |
| data | 442836 |
| padding | 24902 |
| unused | 113152 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Stack | 2 | 117 | 2 |
| Else | 2 | 117 | 2 |
| setpatch | 24 | 833 | 45 |
| EndSkip | 1 | 2 | 1 |
| Quit | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| GetEnv | 2 | 117 | 2 |
| Skip | 2 | 117 | 2 |
| SetEnv | 2 | 117 | 2 |
| Failat | 2 | 117 | 2 |
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
| usa1 | 1 | 3 | 1 |
| parallel.device | 3 | 285 | 26 |
| clipboard.device | 10 | 1395 | 131 |
| printer.device | 47 | 6411 | 346 |
| EpsonQ | 15 | 563 | 42 |
| CBM_MPS1000 | 15 | 562 | 43 |
| HP_LaserJet | 15 | 604 | 47 |
| EpsonX[CBM_MPS-1250] | 15 | 702 | 51 |
| Okimate_20 | 14 | 462 | 30 |
| HP_DeskJet | 15 | 604 | 47 |
| A-Train (1992)(Ocean)(De)(Disk 1 of 2).adf:Installer | 6 | 45 | 2 |
| icon.library | 10 | 1375 | 109 |
| info.library | 25 | 2598 | 167 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 76 |
| A-Train (1992)(Ocean)(De)(Disk 1 of 2).adf:A-Train | 3 | 58787 | 2480 |
| More | 19 | 2290 | 76 |
| A-Train (1992)(Ocean)(De)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| A-Train (1992)(Ocean)(De)(Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 32 |

## Cases handled

- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- a-train (1992)(ocean)(de)(disk 1 of 2)/A-Train (1992)(Ocean)(De)(Disk 1 of 2).adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: If a pointer is not null, it copies a fixed number of words from a source table to a destination on the stack before calling another routine.

### Else

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table header with this maximum value and a sequence of incrementing or fixed values.
- `copy_stack_data_2`: Copies a fixed amount of data from a source pointer to a stack location if the stack pointer is non-zero.

### setpatch

- `initialize_setpatch_environment`: Initializes the patching environment by locating a target library and parsing a configuration string or file path.
- `update_patch_state`: Updates a state structure with a new parameter and executes a sequence of conditional cleanup callbacks or state-resetting functions stored in a table, passing a specific identifier or value to each call.
- `execute_conditional_chain_2`: Calls a routine, then if a certain global variable is non-zero, calls a routine with a parameter, passes that result to another routine, and finally calls a routine unconditionally.
- `install_setpatch_handler`: Installs a SetPatch handler and performs a series of conditional patches and library initializations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: Calls a routine through a pointer stored in a global variable and returns the value produced by that indirect call.
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
- `accumulate_and_store_sum`: Iterates through a table of shorts, accumulating their values into a local variable, and stores the bitwise NOT of the final sum into a global location.
- `apply_conditional_patch`: Checks a global variable and calls a routine through a table; if the return value is non-zero and a size field is small, it updates a global variable, increments and then decrements a byte in a global, calls another routine twice via a table, and returns a flag.
- `check_flag_and_copy_word`: Calls a routine and tests its return value; if non-zero, it copies a 32-bit word from a fixed offset in a global variable into another global variable.
- `execute_conditional_function_4`: Checks if a global variable or a specific constant matches a value, and if so, calls an indirect function pointer.
- `branch_on_condition`: Checks if a register contains a specific value and, if not, calls an indirect function pointer.
- `update_interrupt_enable_status`: Updates a global variable and modifies the interrupt enable bit of the INTENA register before calling another routine via an indirect call.
- `execute_indirect_call_2`: Executes a function pointer retrieved from a global address plus an offset.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `empty_routine_2`: the routine is empty and performs no operations.
- `return_input_value_2`: Returns the value previously held in a register unchanged.
- `execute_multiple_calls_2`: Repeatedly calls another routine and returns its last return value.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `patch_function_table_2`: Checks a global value and calls a function pointer located at an offset from a global base.
- `patch_function_table`: Uses a library routine to find a target, copies a value from a source, and then calls a library routine repeatedly while decrementing a counter.
- `get_masked_pointer_value_2`: Returns a pointer or value with the highest bit set.
- `get_masked_pointer_value`: Calls a library routine using an offset from a register and returns a pointer or value with the highest bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `restore_processor_state`: Copies a value from d1 to a stack offset and restores the processor state via an exception return.
- `call_and_execute_subroutine`: Calls a routine using a pointer from a global variable and then calls another routine with a value loaded from a global variable.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `compute_word_offset_2`: Reads a 16-bit value from two bytes after an input pointer and returns a new pointer adjusted by that value.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found; if the inverted checksum is non unable to be zero, it passes the buffer and checksum to another routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### EndSkip

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Quit

- `find_max_and_prepare_buffer_7`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table with a header and a sequence of incrementing values.
- `copy_data_and_call_function`: Copies a fixed number of words from a source address to a destination and then calls another routine.

### Echo

- `find_max_and_prepare_buffer_8`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table with a header and a sequence of incrementing values.
- `copy_data_and_call`: Copies a fixed number of words from a table to a stack location and then calls another routine.

### Lab

- `return_zero_30`: the routine returns a zero value and performs no other operations.

### GetEnv

- `find_max_and_prepare_buffer_9`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table header with this maximum value and a sequence of incrementing or fixed values.
- `copy_memory_block_4`: Copies a fixed number of words from a source address to a destination address and then calls another routine.

### Skip

- `find_max_and_initialize_buffer`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer or table with a sequence of incrementing values and copies a fixed number of words from a source table to a destination, or vice versa, depending on certain conditions.
- `copy_memory_block_5`: Copies a fixed number of words from a source pointer to a destination pointer if the destination pointer is not null.

### SetEnv

- `reallocate_and_initialize_buffer`: Finds the maximum value in a linked list, then reallocates a buffer or table to accommodate this maximum value plus a header, and copies or initializes data or stack-based parameters into this new memory area.
- `execute_subroutine`: Executes a routine stored in a pointer after a block of memory has been copied.

### Failat

- `find_max_and_prepare_buffer_3`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table with a header and copies a fixed number of words from a source table to a destination, potentially updating a pointer or a return value or a global variable.
- `copy_stack_data_3`: Iterates through a source buffer to copy a sequence of words to a stack-based destination.

### CD

- `find_max_and_prepare_buffer_4`: Traverses a linked list to find the maximum value in a specific field, then initializes a buffer or table with a header and copies a fixed amount of data from a source table to a destination, potentially updating a function pointer or a linked list entry.
- `copy_stack_data_4`: Performs a block copy of data from a table to the stack and then executes a function pointer.

### Date

- `update_buffer_header_and_copy_data`: Finds the maximum value in a linked list and writes a header or a sequence of incrementing values into a buffer, then conditionally copies a fixed block of data from a source table to a destination and vice versa.
- `copy_stack_data_5`: Checks if a pointer is non-null, then copies a block of data from a source address to a stack address.

### EndIf

- `return_zero`: The routine is empty and returns zero.

### If

- `find_max_and_prepare_buffer_5`: Iterates through a linked list to find the maximum value in a specific field, then initializes a buffer or table with a header and a sequence of incrementing values.
- `copy_stack_data_6`: Copies a fixed number of words from a source pointer to a stack-based destination and calls a function.

### IconX

- `initialize_program_environment`: Initializes the application state by retrieving the current directory, allocating memory for a state structure, and parsing command-line arguments or Workbench-provided strings.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Determines whether to start from the command line or Workbench, allocates memory, and patches a function table with a sequence of addresses.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies a source string into a destination buffer and appends a second string to it, then passes the buffer and the new length to another routine.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_handler`: Calls a function pointer retrieved from a global table offset.
- `execute_indirect_function`: loads a function pointer from a global table and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `invoke_with_stack_pointer`: calls another routine passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_pointer`: loads a function pointer from a global offset and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `execute_indirect_function_2`: loads a function pointer from a global table using a fixed offset and executes it.
- `execute_indirect_function_3`: loads a function pointer from a fixed offset within a global table and executes it.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, passing the buffer and inverted checksum to another routine if non-zero.
- `store_byte_to_address`: Stores a byte from a register into a memory location pointed to by a register.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer`: stores the contents of a data register into a memory location pointed to by a register.

### LoadWB

- `initialize_boot_environment`: Determines the execution context by checking for a command-line or Workbench start, then parses the current directory or a provided string to populate a state structure with file paths and task pointers.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, which leaves it unchanged, and allocates a 160-byte block using icon, intuition, and workbench library calls.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it or higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry and storing the looked-up value, while linking the nodes into a list.
- `execute_library_handler_offset_60`: calls a function pointer located at an offset from a global variable and invokes a library routine.
- `execute_indirect_function_4`: loads a function pointer from a memory offset and executes it.
- `execute_indirect_function_5`: loads a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_6`: loads a function pointer from a memory location offset from a global pointer and executes it.
- `execute_indirect_function_7`: loads a function pointer from a fixed offset within a global table and calls it.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `execute_indirect_function_8`: Loads a function pointer from a global table using a fixed offset and executes it.
- `execute_indirect_function_9`: Calls a function pointer located at an offset from a global variable.
- `execute_indirect_function_10`: Calls a function pointer located at a different offset from a global variable.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_and_pad_version_string`: Compares a source string against a global string and, if a version-specific character is detected, pads the destination with spaces.
- `write_string_to_memory`: stores a sequence of bytes representing a string or a space-padded character into a memory location relative to a pointer held in a register.
- `compare_string_prefix`: Compares a string from a global source to a destination string, stopping if a null terminator or a mismatch is found.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does or does not do any work.
- `execute_reschedule_handler`: calls a function pointer located at an offset from a global variable and invokes the reschedule library routine.
- `execute_dispatch_handler`: calls a function pointer located at an offset from a global variable and invokes the dispatch library routine.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `execute_indirect_function_pointer_2`: loads a function pointer from a global memory location and executes it.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot, then returns.
- `invoke_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack-allocated buffer until a null terminator is encountered.
- `store_byte_to_address_2`: stores a byte from a register into a memory location pointed to by a register.

### SetClock

- `parse_current_directory_path`: Retrieves the current directory path and parses it into a sequence of tokens, handling quoted strings and replacing certain characters with newlines or escape codes.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `execute_handler_logic`: Performs a series of conditional checks on return values from a routine, potentially updating a global variable or writing constant values to a global array, and eventually jumps to a global address.
- `compare_global_bits`: Compares the lower bits of a global variable with a certain value and returns the result.
- `write_global_and_check`: Writes several constant values to a global array and returns a comparison of a global variable's bits.
- `update_request_recursively`: If a pointer is returned by a routine, it writes a constant value to a field within that structure, recursively calls itself with that pointer, and then reads a value from a different field in the same structure before calling itself again.
- `update_structure_and_sum`: Writes a value to a structure field, calculates a weighted sum, and clears a longword field in a structure.
- `delete_io_request_recursively`: Calls a routine to obtain a pointer, then if a pointer is returned, calls another routine with a pointer and a constant, and if a third call returns a non-zero value, deletes the pointer via another routine.
- `delete_io_request`: If a pointer is non-zero, it recursively calls a routine on a nested pointer before clearing a field and deleting a port or I/O request.
- `calculate_weighted_sum`: Calculates a complex weighted sum or checksum using values or pointers from a table and returns the result.
- `populate_timing_table`: Calculates a remaining value by decrementing a counter based on conditional checks and then writes a series of retrieved values into a table in memory, terminating with a zero-initialization.
- `compare_strings_with_case_insensitivity_2`: Iterates through a string, converting characters to lowercase to compare them, and returns a comparison result or zero if a mismatch or null.
- `compare_strings_with_case_insensitivity`: Iterates through a string, converting characters to lowercase to compare them, and returns a comparison result or zero if a mismatch or null terminator is found.
- `compare_and_return_constant`: Compares a register value against a value at a memory location and returns a constant value based on the comparison result.
- `return_constant_3`: Returns the constant value one.
- `convert_to_lowercase`: Checks if a character is an uppercase letter and converts it to lowercase if so, otherwise returns the sign-extended input.
- `tiny_ext_ext_rts`: Small routine of 3 instructions that sign-extends d0, then sign-extends d0, then returns.
- `decrement_and_check_flag`: Updates a memory location with a constant and then executes a loop that decrements a register while checking a bit in a memory location.
- `update_global_memory_on_condition`: Writes a constant through a pointer, then checks a bit in that memory location while decrementing a counter, and writes further constants if a certain condition is met.
- `write_constant_to_pointer`: Writes two constant values through a pointer-based address.
- `update_memory_with_constant`: Writes a constant value to two different offsets relative to a global pointer.
- `update_offset_on_condition`: Writes a constant through a pointer, then checks a bit in that memory location while decrementing a counter, and writes a constant to a fixed offset if a certain condition is met.
- `write_constant_to_offset`: Stores the constant value four into a memory location at a fixed offset from a pointer held in a register.
- `call_other_routine_with_stack_2`: Calls another routine, passing the two incoming parameters and a pointer to a local stack slot, then returns.
- `execute_indirect_function_11`: Calls a function pointer located at an offset from a global variable, following an allocation routine.
- `execute_handler_pointer`: Calls a function pointer stored at a location relative to a global pointer.
- `dispatch_handler_pointer`: Calls a function pointer located at an offset from a global pointer using a dispatch-style call.
- `reschedule_handler_pointer`: Calls a function pointer located at an offset from a global pointer, used in a rescheduling context.
- `call_library_function`: Calls a function pointer located at an offset from a global pointer via a library call.
- `execute_clock_handler`: loads a function pointer from a global table and executes it.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_function_12`: loads a function pointer from a global offset and executes it.
- `call_clock_handler`: loads a function pointer from a global table and executes it.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `invoke_and_checksum_buffer`: Invokes a function pointer stored at an offset from a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_address_3`: Stores a byte from a register into a location pointed to by a register.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction or a 16-bit modulo operation using repeated doubling and subtraction, returning a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_2`: Calls another routine and returns without performing any further operations.
- `call_helper_return_register_value_2`: Calls another routine and returns the value that the called routine leaves in a data register.
- `negate_result_on_flag_6`: Calls another routine and negates the returned value if the helper routine leaves a non-zero value in an address register.
- `call_and_negate_on_flag_3`: Calls a helper routine and returns its first result, or the negation of that result if a second register contains a non-zero value.
- `allocate_indexed_record_block`: Allocates a 34-byte block using a library call and, if successful, populates the block with a pointer, a flag, a constant, and a slot index before calling further library functions.
- `release_record_chain`: Recursively processes a linked chain of records, setting a type byte to a certain value and a long field to a certain value before releasing the block and calling another routine.
- `store_word_and_pointer_in_node`: Calls an allocation routine and, if a block is returned, stores a byte, a word, and a pointer into specific offsets within that block.
- `reset_node_and_release_it`: If a pointer is non-null, sets a byte field and two longword fields to certain values, then passes a pointer and a word from the structure to another routine.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Wait

- `find_max_and_prepare_buffer_6`: Iterates through a linked list to find the maximum value in a field, then initializes a buffer or table header with this value and a sequence of incrementing or fixed data.
- `copy_buffer_and_call`: Copies a fixed number of words from a source buffer to a destination stack location and then calls a function pointer.

### Version

- `parse_command_line_arguments`: Parses a string from the current directory or command line, handling quoted strings and replacing certain characters with newlines or escape codes, and stores the resulting tokens in a memory buffer.
- `dispatch_message_event`: Updates a message structure with a new pointer and dispatches or handles specific event flags by calling secondary routines.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_reply_2`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_signed_integer`: Checks for a leading plus or minus sign and converts a sequence of ASCII digit characters into a signed integer.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating if the first string is less than or equal to the second string, or if they are equal.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `execute_indirect_function_13`: loads a function pointer from a memory location and executes it.
- `execute_indirect_function_14`: loads a function pointer from a global table and executes it via an indirect call.
- `invoke_recursive_stack_handler`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_15`: loads a function pointer from a global table using a constant offset and executes it.
- `execute_indirect_function_16`: loads a function pointer from a global table using an offset and executes it.
- `execute_indirect_function_17`: Loads a function pointer from a memory location offset from a global pointer and executes it.
- `execute_indirect_call`: Retrieves a function pointer from a global structure and executes it.
- `execute_indirect_function_18`: loads a function pointer from a global table using a fixed offset and calls it.
- `execute_indirect_function_pointer_3`: Loads a function pointer from a location relative to a global pointer and executes it.
- `invoke_and_checksum_stack_buffer_2`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_register_byte`: Stores the contents of a data register byte into a memory location pointed to by a register.

### Preferences

- `parse_preferences_string`: Parses a string or a quoted string, handling certain character replacements, and populates a buffer or a table with the processed tokens.
- `execute_library_calls_with_data`: Stores a pointer into a location relative to a global variable and executes several function pointers using values or pointers from a global variable.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `execute_library_calls_with_data_2`: Stores a pointer into a location relative to a global variable and executes several function pointers using values or pointers from a global.
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
- `parse_and_apply_preferences`: Iterates through a table of preference entry structures to validate and apply configuration settings.
- `initialize_preference_structures`: Initializes a table of preference entry structures by allocating memory, copying or clearing buffers, and setting up pointer chains.
- `handle_preferences_menu`: Loads a preferences block from a global table and, if present, updates the relevant configuration values and display settings, then runs a message loop that dispatches menu selections to their handlers and replies to messages until the dialog is closed.
- `cleanup_resources_and_libraries`: Releases a linked list of memory blocks, closes certain libraries, and resets an interrupt vector.
- `set_preference_values`: The routine calls another routine twice with different first arguments, passing a calculated address from a global variable and the input parameter.
- `iterate_and_validate_params`: Iterates through a sequence of word-sized values from a memory block, passing each to another routine along with a constant and an index, where the loop range depends on the first input parameter.
- `validate_and_pass_params`: Zeroes a parameter if it exceeds a limit, then decomposes another parameter into its nibbles and passes the parts to a library routine.
- `initialize_preference_entries`: Walks through a table of pointers, copying values from a global variable array into each structure's fields and clearing several fields beforehand.
- `process_preference_selection_2`: Handles a specific preference menu selection by branching on an input parameter to configure display modes, manage memory allocations, or update configuration tables.
- `process_preference_selection`: Reads a selection index from a data structure and dispatches to specific handlers to update global configuration variables, performing arithmetic adjustments, memory copies, and indirect calls based on the selected option type.
- `process_preference_buffers`: Checks whether a short parameter is nonzero and, if so, invokes another routine with a base value, then processes three memory locations by extracting nibbles from a word and clamping a global value into a signed range before storing the results.
- `update_preference_bounds_2`: Calculates new range or offset values using bitwise shifts and arithmetic on input parameters and pointer-based data, then calls a routine to apply these clamped or adjusted values to a preference structure.
- `clamp_and_apply_preferences`: Clamps two byte parameters to a signed range and adjusts them based on offset values within a data structure if a condition is met, storing the results back.
- `return_constant_based_on_input`: Returns the value three when the input parameter equals nine, and returns four for any other input.
- `return_constant_based_on_parity`: Returns one constant when the least significant bit of the input is zero, and returns a nearby constant otherwise.
- `compare_strings_case_insensitive`: Compares two null-terminated strings character by character, uppercasing each byte before comparing, and reports whether they match.
- `execute_forbid_wrapper`: Calls the function that disables certain system-wide interrupts or resources via the exec library.
- `execute_permit_wrapper`: Calls the function that re-enables certain system-wide interrupts or resources via the exec library.
- `execute_indirect_function_pointer_4`: loads a function pointer from a global table and executes it.
- `free_memory_wrapper`: Calls a routine that releases a previously allocated block of memory back to the system.
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
- `execute_indirect_function_21`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_22`: Loads a function pointer from a global table and executes it.
- `execute_indirect_function_23`: loads a function pointer from a global table and executes it.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `allocate_indexed_record_block_2`: Allocates a 34-byte block, populates it with a parameter, a flag, a constant, and a slot index, and then calls several library functions before returning the block or zero.
- `release_record_chain_2`: Recursively releases a linked chain of records, marking the current record's type and long field with specific values before proceeding to the next link.
- `format_date_string`: Initializes a memory buffer with date separator characters and a null terminator.
- `parse_date_string`: Iterates through a global string buffer to validate and extract date components.
- `convert_numbers_to_ascii`: Splits each of three integer parameters into tens and units digits and stores the corresponding ASCII characters into consecutive global memory locations.
- `apply_ascii_configurations`: Calls a library function with several constant arguments, copies values from global memory into other global variables, and computes a table index before making a final call.
- `calculate_timer_intervals`: Computes timer intervals by parsing ASCII date/time components, performing arithmetic adjustments, and configuring timer device requests.
- `validate_and_convert_preferences`: Validates that certain character-based configuration values fall within allowed ranges and resets invalid entries to zero.
- `clamp_and_convert_values`: Clamps two integer parameters to specific ranges and then calls a conversion routine to transform them into ASCII characters.
- `clamp_and_convert_values_2`: Validates and clamps several global character-based or integer-based variables to specific ranges, then calls another routine with the processed values.
- `clamp_and_convert_values_alt_2`: Clamps two integer parameters to specific ranges, ensuring they are within certain bounds, and then calls a conversion routine to transform certain values.
- `clamp_and_convert_values_alt`: Clamps two integer parameters to specific ranges, ensuring they are within certain bounds, and then calls a conversion routine to transform them into ASCII characters.
- `setup_timer_request_block`: Opens the timer device and allocates an extended I/O request block, then populates the block's fields with a calculated delay, a flag, and a zeroed field, followed by a loop that clears a sequence of memory locations if a certain condition is met.
- `initialize_preference_chain`: Recursively initializes a linked list of configuration or preference records by checking for null pointers and updating a global bitmask.
- `update_configuration_state`: Opens a device, updates a global bitmask, and conditionally executes routines based on bitmask or pointer-based checks.
- `update_configuration_bitmask`: Updates a global variable by incrementing it and checks or sets specific bits in a bitmask based on values read from a memory structure.
- `return_immediately_7`: This routine performs no operations and returns to the caller immediately.
- `initialize_timer_task`: Initializes a library instance, updates a global bitmask, and stores a task pointer into a global variable while linking it to a global variable.
- `teardown_io_and_task`: Checks a bit in a global structure and calls another routine if the bit is clear.
- `setup_io_request`: It stores two pointer values and a constant into fixed offsets of a memory structure passed as an argument, then invokes another routine with that same structure.
- `check_global_flag_change`: It reads a global variable, calls a delay function, and returns a boolean indicating whether the lower four bits of that global variable have changed.
- `wait_for_condition_timeout`: It conditionally writes constant values to a global structure, sets a bit in a word within that structure, and loops decrementing a counter until a specific bit in the structure is cleared or the counter expires.
- `clear_global_bit`: Clears a specific bit in a global variable using a bitwise AND operation.
- `save_system_configuration`: Opens a system configuration file for writing and closes it if opening fails.
- `process_system_configuration`: Invokes a routine to read a certain amount of data, checks a return value against a constant, and potentially updates a pointer or calls another routine.
- `return_value_in_d0`: Moves the contents of d2 into d0, restores d2 through d4 and a0 through a1 from the stack, and returns.
- `execute_scheduled_handler`: Executes a function pointer retrieved from a global variable offset.
- `execute_indirect_function_24`: loads a function pointer from a global table and executes it.
- `call_init_code`: Library stub: loads its arguments and the exec library base, calls InitCode and returns its result to the caller.
- `execute_indirect_function_pointer_5`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_6`: Loads a function pointer from a global table and executes it.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `execute_indirect_handler`: Loads a function pointer from a global table and executes it.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_set_int_vector`: Library stub: loads its arguments and the exec library base, calls SetIntVector and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_allocate`: Library stub: loads its arguments and the exec library base, calls Allocate and returns its result to the caller.
- `execute_indirect_function_25`: dereferences a pointer stored in a global table and executes the resulting function pointer.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `execute_indirect_function_26`: loads a function pointer from a global offset and executes it.
- `call_free_entry`: Library stub: loads its arguments and the exec library base, calls FreeEntry and returns its result to the caller.
- `execute_indirect_function_pointer_7`: Loads a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_27`: loads a function pointer from a global table and executes it.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_add_library`: Library stub: loads its arguments and the exec library base, calls AddLibrary and returns its result to the caller.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `pass_library_base_to_stub`: Passes a library base pointer from a global variable and the address of a stack argument to a library call stub.
- `process_preference_list`: Recursively traverses a linked list of structures, checking a type field and potentially descending into nested or sub-lists while updating a global pointer.
- `traverse_preference_list_3`: Iterates through a linked list of structures, checking a type field and potentially calling other routines or recursing.
- `traverse_preference_list_2`: Iterates through a linked list of structures, checking a type field and potentially calling other routines or recursing if certain conditions are met.
- `traverse_preference_list`: Iterates through a linked list of structures, checking a type field and potentially calling other routines or recursing if certain conditions or type-specific values are met.
- `recurse_indefinitely`: This routine calls itself indefinitely, causing an infinite loop or stack overflow, before returning a zero value.
- `recurse_indefinitely_2`: Calls itself indefinitely with a pointer to a global variable before returning zero.
- `process_preference_list_node`: Recursively traverses a linked list of preference structures, checking a type field and potentially descending into nested or sub-lists.
- `return_empty_routine_2`: This routine performs no operations and immediately returns to the caller.
- `copy_struct_fields_tail`: Copies five word-sized fields from specific offsets in a source structure to the corresponding offsets in a destination structure.
- `process_preference_type`: Loads a pointer and a value from a table, then uses a switch statement to perform various assignments or calls other routines based on a type or flag.
- `update_global_pointer`: Passes the address of a global variable to another routine and returns a constant value of one.
- `return_empty_routine_3`: the routine performs no operations and immediately returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `configure_intuition_object`: Loads an object pointer from a global variable, sets a field to one, calls a library function with parameters from a table, and sets that same field to all ones.
- `find_index_limit_2`: Increments a counter while a value in a table is less than a provided parameter, then returns the decremented counter.
- `find_index_limit`: Increments a register in a loop until a value in a table exceeds a certain threshold, then returns the decremented counter.
- `decrement_register_value`: The routine returns the value of a specific register minus one.
- `initialize_preference_buffers`: It clears a global flag, then loops over a table of six entries, calling a helper routine to allocate or retrieve buffers for each entry and storing the result.
- `load_intuition_library`: Calls another routine with a pointer to a global variable and an integer argument.
- `invoke_library_vector_2`: Checks a bit in a global variable and calls a library routine using a specific offset or a fixed address.
- `invoke_library_vector`: Acquires a library semaphore and invokes a library vector using an offset derived from a global variable.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `parse_and_sort_preferences`: Iterates through a list of DOS-style directory or file entries, constructs a padded string or label for each, and builds a sorted table of pointers to these entries in memory.
- `compare_string_prefixes`: Compares two null-terminated strings character by character.
- `allocate_and_copy_entry`: Allocates a memory block via the exec library and stores the returned pointer in a global variable.
- `free_linked_list_nodes`: Iterates through a linked list, releasing each memory block back to the system using a wrapper, and resets a global pointer and counter.
- `initialize_string_buffer`: Iterates through a buffer to clear it with null bytes or spaces, or copies a string from a global variable into it if a certain condition is met.
- `check_string_prefix_match`: Initializes a local buffer with spaces and copies a string from a global variable into it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_function_a`: Executes a routine located at an offset from a global pointer.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `execute_indirect_function_19`: Loads a function pointer from a global offset and executes it.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `execute_indirect_function_20`: loads a function pointer from a global offset and executes it.
- `iterate_table_entries`: Iterates through a range of indices, retrieving a pointer from a global table for each and passing it to another routine along with a global variable.
- `schedule_table_entries`: Loops over a range of indices and calls another routine for each entry in a global table, passing the table entry address and a constant value to it.
- `process_preference_switch`: Reads configuration values from memory structures based on a global switch index.
- `configure_display_modes_2`: Iterates through a 16x16 grid of bits derived from a global configuration table.
- `calculate_string_length_2`: Returns the number of characters in a null-terminated string before its terminator.
- `configure_display_regions`: Acquires a semaphore, then calls a library routine multiple times with coordinates and parameters derived from the input arguments, and releases the semaphore upon completion.
- `draw_rectangle_sequence`: Acquires a semaphore, then calls a library routine repeatedly with coordinates derived from the input parameters to draw shapes, and releases the semaphore after thedrawing operations.
- `select_and_pass_constant`: Selects one of two constant values based on an input parameter and passes it to another routine along with fixed coordinates.
- `initialize_library_instance`: Opens two libraries and, if either fails, closes one and returns zero; otherwise it initializes several fields of the first library structure with calculated addresses and flags.
- `wait_for_signal_completion`: Sets a flag and signals a task when a global variable is nonzero, then loops calling a delay routine until that variable becomes zero, and finally signals another task if given a nonzero argument.
- `call_global_and_checksum_buffer_3`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, passing the buffer and checksum to another routine if the inverted checksum is non-zero.
- `store_register_to_memory`: Stores the contents of a register into a memory location pointed to by another register.
- `modular_inverse_32bit_2`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_3`: Calls a helper routine and returns.
- `call_helper_return_register_value_3`: Calls a helper routine and returns the value left in a data register.
- `negate_result_on_flag_7`: Calls a helper routine and negates the returned value if a flag or certain register condition is non-zero.
- `call_and_negate_on_flag_4`: Calls a helper routine and returns its first result, or returns the negation of that result if a certain register or flag is non-zero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer_2`: stores a byte from a data register into a memory location pointed to by an address register.
- `handle_preference_input`: Sends a message to a library, stores several derived values into a structure, and then loops reading messages and dispatching on their status to update preference settings.
- `copy_structured_data_fields`: Copies a word from offset two and certain bytes from offsets 183, 182, and 184 of a source address into the matching offsets of a destination address.
- `update_menu_index_and_recurse`: Updates or wraps a global index or a bit-masked flag based on a value read from a table, then recursively calls itself with a modified parameter or returns a constant.
- `return_constant_value_2`: The routine returns a constant integer value.
- `execute_multiple_library_calls`: Calls a library function multiple times with certain arguments and updates a global variable with a value from a pointer.
- `update_library_vector_flags`: Calls a library function multiple times with certain arguments and updates a global variable by ORing it with a masked version of itself.
- `handle_preference_input_loop`: Initializes display configuration and preference data, then enters a loop to wait for and retrieve input messages.
- `copy_and_schedule_tasks`: Copies several fixed fields and a looped block of word-sized entries from one memory location to another.
- `initialize_display_configuration`: Frees previously allocated memory and cleans up resources, then copies configuration values into global variables and configures display colors and modes using library functions.
- `process_input_event`: Reads an event code from a structure and branches on it to update a global state or a selector.
- `decode_and_calculate_values`: If a global flag is certain, it decodes a 16-bit value into nibbles and passes them to another routine, repeating this for several pointers in a list.
- `store_word_recursive`: Writes a word built from three parameters into a global table at an index derived from a second parameter, then calls itself recursively with an incremented index.
- `configure_display_colors`: Reads byte values from global memory and passes them, with small offsets, to a library routine four times, likely configuring display color registers.
- `schedule_preference_tasks`: Invokes another routine three times using a global pointer, a pointer to a global variable, and a constant argument, acting as a wrapper to schedule preference operations.
- `update_display_parameter_offsets`: Modifies a parameter and calls another routine repeatedly with new coordinates or offsets, conditionally branching based on a global variable's value.
- `configure_display_modes`: Calls a library routine to query a display state, then iterates through a 16x16 grid of bits derived from a global configuration table.
- `modify_word_bits`: Clears specific bits in two adjacent or nearby word-sized memory locations using a mask derived from a bit-shift, then sets those bits based on a provided parameter.
- `query_display_state`: Calls library routines to query display state, checks a return value, and conditionally calls another routine with calculated offsets before a final call.
- `initialize_preference_string_buffer`: Clears or populates a string buffer by copying or zeroing bytes, and updates global pointers or flags based on preference structure or command processing.
- `copy_struct_fields_partial`: Copies specific word-sized and byte-sized fields from a source structure to a destination structure at matching offsets.
- `process_preference_command_switch`: Dispatches a command or action based on a value read from a preference structure.
- `apply_multiple_preferences`: Executes a series of indirect function calls with various hardcoded or pointer-derived arguments to apply or reset certain settings.
- `update_preference_bounds`: Checks and updates global preference limit or counter values, or synchronizes a global variable with another or a limit-checking value.
- `allocate_preference_entry`: Allocates a memory entry via the exec library and schedules a task to initialize it, then invokes a function pointer from a global table to process the new entry.
- `init_preference_list`: Sets a global initialization flag on first entry, then adjusts a stored value against a global variable before invoking another routine.
- `init_preference_entries`: Allocates and initializes a linked list of preference entries using exec library calls, scheduling the initialization task and managing memory for the entry nodes.
- `initialize_preference_entry`: Checks a global variable and, if it is less than one, sets a global flag and calls another routine.
- `init_preference_entry`: Allocates a new entry in the exec list and initializes its fields using values from a global variable and a table offset before linking it into the preference list.
- `schedule_table_entries_2`: Iterates over a range of indices and calls another routine for each entry in a global table, then derives several bit fields from a configuration word and passes computed offsets to another routine.
- `process_gadget_list_and_update_state`: Iterates through a linked list of gadget structures, checking a type field and updating a global state or pointer based on the return value of a processing routine.
- `traverse_linked_list_and_update_pointer`: Iterates through a linked list, updating a pointer with the return value of the next element's processing and occasionally calling a routine with a bit-masked value or a pointer-referenced bit.
- `copy_global_struct`: copies a structure or a set of fields from a global variable to a destination address using a routine that copies certain fields.
- `copy_global_struct_2`: Calls another routine to copy a structure or a set of fields from a global variable to a destination address.
- `traverse_linked_list_and_update_pointer_2`: Iterates through a linked list, updating a pointer with the return value of the next element's processing and occasionally calling a routine.
- `return_immediately`: this routine is empty and performs no operations before returning.
- `copy_struct_fields`: It copies certain fields, including two words and two bytes, from a source structure pointed to by the first argument to a destination structure pointed to by the second argument.
- `update_preference_state`: Updates a global bitmask or state variable based on a value read from a pointer-referenced structure.
- `invoke_routine_with_pointer`: Calls another routine while passing the address of a global variable as an argument and returns a constant value.
- `return_immediately_6`: This routine is empty and performs no operations before returning to its caller.
- `update_gadget_visual_states_2`: Checks various bits in a global variable to call another routine with specific arguments, effectively updating the visual state of UI gadgets.
- `update_gadget_visual_states`: Checks a bitmask within a preference structure to trigger a sequence of function calls that update the visual state or properties of UI gadgets.
- `configure_intuition_gadgets`: Checks various bits within a preference structure and calls a function with a command ID or a specific value based on those bits.
- `update_preference_bitmask_2`: Checks specific bits within a preference structure and calls a function pointer with a command ID based on those bits.
- `update_preference_bitmask`: Checks a bitmask within a preference structure and calls a function pointer with a command ID corresponding to the detected bit.
- `update_preference_status_bitmask`: Checks a bitmask within a preference structure and calls a function pointer with a command ID corresponding to the detected bit.
- `update_preference_function_pointer`: Checks specific bits within a preference structure and calls a function pointer with a certain command ID and parameters.
- `dispatch_preference_status_2`: Checks bits in a structure and calls a function pointer with different arguments based on those bits, then calls another routine with a value derived from a local variable.
- `invoke_function_with_status_check`: Checks a bit in a structure and calls a function pointer with different arguments based on that bit's state.
- `dispatch_preference_status`: Calls a function pointer with specific arguments and then calls another routine based on a bitmask check in a global variable.
- `handle_preference_dispatch_2`: Reads a status field from a structure and dispatches on its value to acquire a semaphore, call a library function several times, and update a structure.
- `setup_intuition_gadgets`: Loads an object pointer from a global variable and calls multiple library functions on it, passing certain constant parameters and values retrieved from a global array indexed by a mask.
- `parse_numeric_string_4`: Parses a string or a portion of a string to convert numeric characters or a decimal-separated number into an integer.
- `skip_string_whitespace`: Skips any space characters or non-numeric characters at the start of a string until a valid character or null terminator is found.
- `validate_digit_string`: Checks if a string contains only valid numeric characters.
- `calculate_scaled_values`: Calculates and returns a pair of values where the second value is a multiple of an input and the first value is a multiple of that same input, depending on certain flags.
- `parse_numeric_string_2`: Parses a string of characters to convert a decimal or decimal-like value into an integer, handling a decimal point and certain character offsets.
- `format_numeric_string_2`: Copies a range of bytes from a global array and then formats a numeric value into a string buffer, optionally inserting a decimal point and handling a fixed number of digits.
- `copy_and_format_string`: Copies a range of bytes from a global array to a destination buffer and calls a routine to format a number as a string.
- `format_numeric_string`: Converts a sequence of characters into a decimal string representation in a buffer, optionally inserting a decimal point and adding an ASCII offset.
- `format_number_string`: It iterates through a data structure to build a decimal string representation in a buffer, inserting a decimal point based on flags and converting each digit, then null-terminates the resulting string.
- `initialize_preferences_display`: Extracts a nibble from a global word and then issues a series of library calls with derived coordinates and constant values to configure text formatting and layout parameters.
- `check_flag_and_allocate`: Checks a specific bit in a global variable; if set, it performs several library calls including memory allocation and returns the result.
- `call_remote_int_server`: Invokes the remote interrupt server twice, once for each of two consecutive global variables, passing a fixed zero argument each time, then returns.
- `manage_library_semaphore_access`: Acquires a semaphore, then calls a library function several times using a parameter and values read from a global variable array, releasing the semaphore before returning.
- `store_calculated_short_value`: Computes a value from three short parameters and stores it into a global variable array at an index derived from the second parameter, then calls another routine with adjusted arguments.
- `process_three_buffers`: Extracts nibbles from a word in a global structure, then iterates over three buffer pointers, calling routines to process each buffer's data and state.
- `forward_argument_with_code`: Passes its one argument to another routine together with the constant 48.
- `forward_single_argument`: Passes its one argument on to another routine and returns nothing.
- `store_word_and_pointer_in_node_2`: Allocates a block and, if successful, stores a byte, a word, and a pointer into fixed offsets within that block.
- `reset_node_and_release_it_2`: If a pointer is non-null, sets a byte field to a certain value and clears two longword fields before passing the pointer to a release routine.

### SetMap

- `initialize_system_environment`: Initializes system-level structures and loads a keyboard keymap by parsing a string or resource.
- `update_and_invoke_cleanup_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `execute_indirect_call_3`: Executes a function pointer stored at an offset within a global variable.
- `execute_indirect_call_4`: Performs an indirect call to a routine located at a specific offset from a global pointer.
- `execute_indirect_call_5`: Calls a routine via a pointer retrieved from a global variable offset.
- `release_saved_pointers`: Iteratively calls a routine to release pointers stored in a table by decrementing a counter, then calls the routine on a remaining block, certain other global pointers, and a provided parameter.
- `save_arguments_and_run`: Stores its two arguments in global variables, then calls a routine with fixed size and count values and a final routine with a fixed code.
- `load_keymap_for_console`: Loads a keyboard keymap from a directory or resource, converting the requested name to lowercase and populating a buffer.
- `execute_indirect_call_6`: Invokes a routine using a pointer found at a negative offset from a global variable.
- `execute_indirect_call_7`: Executes a routine located at a certain offset relative to a global pointer.
- `execute_indirect_call_8`: Performs a jump to a routine address stored in a global variable offset.
- `execute_indirect_call_9`: Calls a routine using a pointer retrieved from a global variable.
- `execute_indirect_call_10`: Executes a function pointer located at a negative offset from a global variable.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `execute_indirect_call_11`: Performs an indirect call to a routine via a pointer in a global variable.
- `execute_indirect_call_12`: Invokes a routine using a pointer stored at an offset within a global variable.
- `execute_indirect_function_28`: loads a function pointer from a global table using a constant offset and calls it.
- `call_set_map_handler`: fetches a function pointer from a global table using a constant offset and executes it.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `execute_indirect_function_29`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_30`: loads a function pointer from a global table and executes it.
- `call_indirect_handler`: Loads a function pointer from a global table and executes it.
- `execute_indirect_function_31`: loads a function pointer from a global table and calls it.
- `execute_indirect_function_32`: loads a function pointer from a global offset and calls it.
- `forward_library_argument`: passes a library base pointer from a global variable and the address of a stack argument to a library call stub.
- `execute_indirect_function_33`: fetches a function pointer from a global table and executes it.
- `call_global_and_checksum_buffer_4`: Invokes a function pointer from a global variable and then calculates a rolling checksum over a stack buffer, passing the buffer and the inverted checksum to another routine if the checksum is non-zero.
- `store_register_to_pointer_3`: stores the contents of a data register into a memory location pointed to by an address held in an address register.

### Format

- `initialize_boot_environment_2`: Determines the startup context by checking for a command line or Workbench, then parses a directory string or command-line argument to populate a structure with pointers and processed string data.
- `update_and_cleanup_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_alloc_mem_2`: Calls a function pointer located at an offset from a global pointer, which executes a memory allocation routine.
- `call_remove_int_server`: Calls a function pointer located at an offset from a global pointer, which executes an interrupt server removal routine.
- `call_init_resident_2`: Calls a function pointer located at an offset from a global pointer, which executes a resident initialization routine.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `call_switch_context`: Calls a function pointer located at an offset from a global pointer, which executes a context switching routine.
- `call_forbid_interrupts`: Calls a function pointer located at an offset from a global pointer, which executes a routine to disable certain system interrupts.
- `execute_indirect_function_34`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_35`: loads a function pointer from a global table and jumps to it.
- `execute_indirect_function_pointer_8`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_9`: loads a function pointer from a global table and jumps to it.
- `execute_indirect_routine`: loads a function pointer from a global table and jumps to it.
- `execute_indirect_function_pointer_10`: Loads a function pointer from a global table and executes it.
- `parse_format_arguments`: Parses command line arguments to identify the drive, volume name, and formatting options.
- `release_format_buffers_and_handles`: Checks several global pointers and, if they are non-zero, calls a release routine with a specific parameter or pointer before calling another routine with the original argument.
- `dispatch_disk_error_messages`: Checks a status or error code and dispatches a corresponding error message string or error-handling routine.
- `report_disk_errors`: Checks a condition and either calls several routines with a global variable or calls a routine with a specific constant and a global variable.
- `store_matched_table_byte`: Searches an eight-entry global table for a specific byte, and if found, stores that byte into a field of a structure obtained from another routine.
- `find_drive_list_entry`: Walks a linked list of drive entries until one matches the given number or pointer, marks that entry, and returns 1.
- `calculate_string_length_3`: calculates the length of a null-terminated string by counting the number of bytes before the terminator.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers until a zero byte is encountered.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32, otherwise returning the original value.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one character at a time after converting each to uppercase, returning 1 if they are equal or 0 if a difference is found.
- `truncate_string_at_colon`: Replaces the first colon or the existing null terminator in a string with a null terminator.
- `invoke_single_value_status`: Passes a single-value stack buffer containing the first argument to another routine and returns zero if the call succeeds or a specific non-zero value if it fails.
- `create_linked_block_from_source`: Allocates a memory block and initializes its header with pointers to form a linked list, then copies up to seven words from a source array into the block's trailing data area.
- `execute_indirect_function_pointer_11`: loads a function pointer from a global table and jumps to it.
- `execute_indirect_function_36`: loads a function pointer from a table and executes it.
- `execute_indirect_function_37`: Loads a function pointer from a fixed offset within a global table and executes it.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_38`: loads a function pointer from a global table and executes it.
- `execute_indirect_pointer`: Loads a pointer from a global variable and jumps to that address.
- `execute_indirect_pointer_2`: Retrieves a function pointer from a global variable and executes it.
- `execute_indirect_pointer_3`: Performs an indirect call to an address stored in a global variable.
- `execute_indirect_pointer_4`: Loads a target address from a global variable and branches to it.
- `execute_indirect_pointer_5`: Reads a pointer from a global variable and transfers control to that location.
- `execute_indirect_pointer_6`: Executes a routine located at an address stored in a global variable.
- `execute_indirect_pointer_7`: Branches to a function pointer retrieved from a global variable.
- `execute_indirect_pointer_8`: Loads a pointer from a global variable and jumps to the destination.
- `execute_indirect_pointer_9`: Uses a pointer stored in a global variable to call another routine.
- `execute_indirect_pointer_10`: Retrieves an address from a global variable and jumps to that address.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_indirect_from_table`: loads a pointer from a global variable and calls it.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_from_table_2`: executes a function pointer located at an offset from a global variable.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_indirect_from_table_8`: calls a routine using a pointer retrieved from a global variable.
- `call_indirect_from_table_9`: jumps to a routine address stored in a global variable.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_and_verify`: Formats a disk track by track, building track data in a buffer and writing it through the disk device, then verifies the tracks.
- `write_value_to_output_handle`: Calls two output routines with fixed arguments of 20, 30 and 1 on a handle taken from a structure, then passes a value from another routine through the same handle.
- `call_global_and_checksum_buffer_5`: Executes a function pointer from a global variable and then performs a rolling checksum calculation on a stack buffer, calling another routine with the buffer and inverted checksum if the result is non-zero.
- `store_register_to_pointer_4`: stores the contents of a data register into a memory location pointed to by an address register.
- `rotate_32bit_value_3`: Performs a bitwise left rotation on a 32-bit value using a loop that handles carry-out or applies a modulo-based reduction if a certain threshold is met.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_and_rotate_32bit`: Calculates a 32-bit product of two 16-bit values and returns a result where the lower or upper halves are rotated.
- `call_indirect_function_7`: Calls a function pointer stored in a global variable.
- `store_byte_to_address_4`: Stores a single byte from a register into a memory location pointed to by a register.
- `allocate_indexed_record_block_3`: Attempts to allocate a block of memory using a slot index, then initializes the block with a provided parameter, a flag, a constant, and the slot index before storing a pointer or a modified address back into the block.
- `release_record_chain_3`: Recursively releases a linked chain of records, processing the linked record first when its link field isset.
- `forward_argument_with_code_2`: Passes its one argument to another routine together with the constant 48.
- `forward_argument_to_reset_routine`: Passes a single pointer argument directly to another routine without any intermediate processing.
- `initialize_system_structures`: Initializes several global pointers and variables by retrieving values from a memory block, potentially setting them to default values if the block is null, and performs a series of function calls to manage these structures.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_indirect_from_table_3`: dereferences a pointer stored in a global variable and jumps to that address.
- `call_indirect_from_table_4`: calls a routine located via an offset from a global variable.
- `call_indirect_from_table_5`: performs an indirect call to a routine pointer stored in a global variable.
- `initialize_memory_block`: Writes the address of a pointer, an incremented version of that address, and a zero value into a sequence of memory locations.
- `store_word_and_pointer_in_node_3`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `reset_node_and_release_it_3`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.
- `call_indirect_from_table_6`: retrieves a function pointer from a global variable and transfers control to it.
- `call_indirect_from_table_7`: executes a routine pointer found at a specific offset from a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start`: Opens the DOS library and allocates a control block, then parses the command line string into a table of argument pointers while handling double quotes and escape characters.
- `call_supervisor_function`: Calls a function pointer located at an offset from a global pointer.
- `execute_indirect_function_39`: Loads a function pointer from a global table and executes it.
- `execute_indirect_function_40`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_41`: loads a function pointer from a global table and executes it via a dispatch mechanism.
- `execute_indirect_pointer_11`: loads a function pointer from a calculated offset within a global table and executes it.
- `execute_indirect_function_42`: loads a function pointer from a global table and executes it.
- `dereference_function_pointer`: loads a function pointer from a global table and executes it.
- `execute_enable_handler`: Calls a function unable to be reached via a direct pointer, but rather through an absolute move or offset from a global variable.
- `execute_indirect_function_43`: loads a function pointer from a global table and executes it.
- `open_cli_window_and_run_startup`: Creates a console or shell window based on a specification, initializes a stack, and executes the corresponding startup script.
- `build_linked_node_chain`: Walks a list of entries, allocating an 8-byte node for each and storing a value from the entry in it, then links the nodes together with shifted pointers.
- `traverse_and_free_linked_nodes`: Iterates through a linked list, calling another routine on a secondary item within each node and then calling another routine to free the node using its size.
- `parse_signed_decimal_2`: Converts a null-terminated string containing a signed decimal integer into a signed integer.
- `execute_indirect_function_44`: loads a function pointer from a global table and jumps to it.
- `execute_indirect_function_pointer_12`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_13`: Loads a function pointer from an offset within a global table and executes it.
- `execute_indirect_function_pointer_14`: Loads a function pointer from a fixed offset within a global table and executes it.
- `call_indirect_handler_2`: Calls a function pointer stored at an offset within a global variable.
- `execute_indirect_function_45`: Loads a function pointer from a global table and jumps to it.
- `execute_indirect_function_46`: Loads a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_47`: loads a function pointer from a global table and executes it.
- `modular_inverse_32bit_3`: Performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction.
- `compute_32bit_product_with_rotation`: Computes a 32-bit product of two 16-bit values and returns a value with a rotated 16-bit register.
- `negate_result_on_nonzero_flag`: Calls a routine to update interrupt status and returns its return value, or the negation of that return value if the called routine leaves a non-zero value in an address register.

### DiskCopy

- `initialize_program_environment_2`: Allocates a block of memory and initializes a control structure by copying a template, setting up pointers, and configuring a stack or buffer area.
- `cleanup_system_resources`: This routine executes a callback function, frees memory, and performs several operations involving system structures and library calls to release resources.
- `parse_command_line_arguments_2`: Parses a string into individual arguments by handling whitespace and quoted substrings, storing the resulting pointers in a table.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies d0 to d5, then loads 0 into d0, then returns.
- `initialize_disk_copy_parameters`: Initializes or updates a global structure with disk source and destination information, including memory allocation for buffers and checking for valid device or file pointers.
- `dispatch_conditional_request`: Checks a global value and either calls a routine with specific parameters or executes a callback using a global structure.
- `execute_callback_chain_3`: Checks several conditional pointers and executes them, while also iterating through a table to call functions and clear entries.
- `manage_disk_copy_operation`: Coordinates a disk copying process by iterating through a table of data blocks, performing I/O operations, and updating progress status.
- `set_display_context_pointer`: Sets a pointer in a structure to a global or a provided address and calls another routine with a specific error code.
- `check_task_status_flag`: Checks a bit in a return value from a function pointer and a flag in a structure, then returns a boolean or calls another routine.
- `process_callback_chain`: Iterates through a chain of function pointers, updating a global variable with a return value and checking a task status flag.
- `execute_disk_copy_logic`: Performs a series of checks on pointers and values, calls other routines to update a structure, and stores the results of certain function calls into a structure.
- `execute_io_requests`: Calls a function pointer using a value from a structure, then calls another function pointer twice using a value from a global variable.
- `poll_status_and_allocate`: Repeatedly checks a character value and, if non-zero, performs memory allocation or error handling based on a global state.
- `populate_memory_block`: Allocates a block of memory and fills it with values obtained from a library call, potentially freeing previously allocated blocks if certain conditions are met.
- `format_progress_string`: Calculates a value based으로 adding two structure offsets and calls a routine to format a string using a pointer and calculated bounds.
- `compare_memory_blocks`: Compares two memory blocks of a certain length and returns one if they match and zero otherwise.
- `verify_disk_info_and_strings`: Uses library calls to check file or directory information and validates string or pointer-based data or structures.
- `find_task_id`: Searches a table for a matching byte and stores a value into a task structure field.
- `get_read_result`: Calls a function and returns its result or a failure code based on the return value.
- `update_structure_fields`: This routine writes several values into specific offsets of a structure and then adds a calculated value to a field within that structure.
- `set_io_parameters`: Calls a function, checks a return value, and writes a constant value to a structure offset.
- `update_io_request_status`: Checks a return value from a function pointer, then writes a constant value to a structure offset and calls another function twice with different constant values written to that same offset.
- `verify_and_update_disk_buffer`: Checks a magic number in a memory buffer, updates a structure field with a checksum-like calculation, and performs a memory copy or string-length-based update if certain conditions are met.
- `execute_io_and_check_status`: Calls a routine with a calculated size, then calls another routine and returns a boolean indicating if the result was zero.
- `execute_conditional_io`: Checks a return value and, if zero, conditionally executes a routine or accesses an array element via a pointer or index.
- `search_string_in_list`: Traverses a linked list of structures to find a string match using a case-insensitive comparison.
- `copy_memory_block`: Copies a specified number of bytes from a source memory location to a destination memory location using a loop.
- `calculate_string_bounds`: Calculates the length of a string and checks if its rendered width or position meets certain criteria using graphics library calls.
- `convert_to_lowercase_2`: Checks if a character is a lowercase letter and, if so, converts it to uppercase by subtracting a constant.
- `read_byte_from_offset`: Returns a byte loaded from an offset relative to a register value.
- `trim_trailing_colon`: Iterates through a null-terminated string and replaces a colon with a null terminator if one is found at the end of the string.
- `ensure_colon_presence_2`: Searches a string for a colon and inserts a null terminator or a colon if a terminator is found instead.
- `ensure_colon_presence`: Searches for a colon in a string, ensuring one exists, and then modifies the string by inserting a zero byte at a specific offset.
- `allocate_and_initialize_task_struct`: Allocates memory, initializes a structure with a pointer to a task, and sets certain offsets using library calls and a global variable.
- `return_immediately_2`: The routine is empty and performs no operations before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `append_to_string_buffer`: Appends a byte or a string to a managed buffer while managing buffer capacity and pointer increments.
- `load_character_constant`: Moves a word from a pointer into a register and sets a register to a specific character value.
- `convert_string_case`: Iterates through a null-terminated string and modifies characters based on a bitmask check against a global table.
- `calculate_string_length_4`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `format_and_reverse_string`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_decimal_3`: Parses a signed decimal integer from a string, storing the result in an integer pointer, and returns the number of characters consumed.
- `push_byte_to_list`: Increments a counter in a global structure and stores a byte into a buffer pointed to by another global variable, then advances that pointer.
- `update_global_structure`: Stores a value into a global variable and then clears another global variable after calling a nested routine.
- `update_string_buffer_index`: Decrements a counter in a structure and increments a pointer or a counter within a buffer if a certain condition is met.
- `append_offset_to_string_buffer`: calculates a value by adding an offset to a local variable and passes the result to a routine that appends data to a managed buffer.
- `format_and_append_string`: Resets a value in a global structure, parses a format string, and returns a value from a global variable.
- `fetch_table_entry_pointer`: Retrieves a pointer from a table and uses it to call another routine, returning a failure code if a global variable is non-zero.
- `search_and_split_list`: Searches a linked list for an entry matching a specific size, potentially splitting an existing node to accommodate a new size, and allocates memory if no match is found.
- `traverse_resource_list`: Iterates through a linked list of pointers, checking specific bit flags in a byte at an offset before recursively calling another routine with data from the current node.
- `execute_dos_write`: Calls a function via a pointer in a global structure and, if it returns a failure, retrieves an error value and stores it in a global variable.
- `perform_dos_seek`: Calls a function via a pointer in a global structure to perform a seek operation.
- `execute_dos_read`: Calls a function via a pointer in a global structure to perform a read operation.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `close_file_handle`: Calls a function via a pointer in a global structure to close a resource.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point_values`: Performs a complex fixed-point division using bit shifts and masking to handle signed or unsigned operands.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_and_initialize_structure`: The routine allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `get_table_entry_address`: The routine checks if an index is within valid bounds and if the corresponding entry in a table is non-zero, then returns the address of that entry or a default value.
- `invoke_auto_request_callback`: The routine calls a function pointer located within a structure pointed to by an offset from a global variable.
- `free_linked_list_elements`: The routine traverses a linked list, calling a cleanup routine for each element, and then clears the head and tail pointers of a structure.
- `append_to_linked_list`: The routine allocates memory for a new element, initializes its fields, and inserts it into both a head pointer and a tail pointer within a structure.
- `reset_structure_fields`: Writes a byte and two words to a structure pointed to by the argument and then calls another routine via a global variable table.
- `update_message_target_and_type`: If a non-zero parameter is provided, it calls a routine via a pointer in a global table and, if a pointer is returned, writes a message type, a zero byte, the original parameter, and a 16-bit value into the returned structure's fields.
- `dispatch_table_call`: Retrieves a pointer from a table and calls another routine, returning a failure code if a global variable is non-zero or if a write operation fails.
- `parse_formatted_numeric_string`: Parses a string containing numeric values with various format specifiers including signs, decimal points, and base indicators.
- `process_format_string`: Iterates through a string and calls another routine to parse numeric values when a percent sign is unable to be skipped or otherwise encountered.
- `assign_list_node_pointer`: Checks if a structure is valid and not flagged, then attempts to assign a pointer from a global source to several offsets within the structure.
- `iterate_and_call_resources`: Iterates through a table of structures, checking specific bits in each entry to decide whether to call another routine recursively.
- `insert_into_sorted_list`: Inserts a memory block into a linked list of sorted nodes, updating the list structure and a global count.
- `fetch_and_validate_table_entry`: Retrieves an address from a table using an index and performs a secondary lookup through a pointer found at that address.
- `copy_memory_block_overlap_safe`: If a count is provided, this routine copies a block of memory, choosing to copy backwards if the source address is less than or equal to the destination address, or forwards otherwise.
- `check_and_clear_pointer`: Checks specific bits in a return value from a function call and, if a global pointer is non-null and a secondary routine returns a non-zero value, clears that pointer and calls another routine.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Locates a fast memory block and parses the command line string into a linked list of null-terminated argument strings, handling quoted strings and converting certain characters like newline or escape sequences.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the or chip block.
- `call_first_handler`: The routine calls a routine via a pointer stored in a global variable table.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_15`: loads a function pointer from a location relative to a global base address and executes it.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### Disk-Validator

- `format_string_and_initialize_buffer`: Converts an integer or character to a string representation in a buffer and initializes a bitmask-based lookup table or array with specific bit-shifted values or certain constants.

### Newcon-Handler

- `initialize_loader_structures`: Initializes a control structure or object by clearing a range of memory and setting up pointers or offsets.
- `execute_indirect_calls`: executes a sequence of indirect calls using a pointer stored in a register and an undeterminable jump table.

### Shell-Seg

- `initialize_pointer_tables`: Initializes several pointers in a structure or table with or relative addresses or offsets, and copies certain values from a source structure to a destination structure or table.

### Ram-Handler

- `execute_indirect_call_13`: Performs an indirect call to a function pointer retrieved from a global variable.

### serial.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_device_state`: Updates several fields in a structure, including incrementing and decrementing a counter, performing a library call, and storing a value into multiple offsets.
- `check_status_flag`: Checks a specific bitmask against a byte in a structure and calls another routine if the condition is met.
- `update_status_byte`: Stores a byte value into a structure and calls another routine if a specific bitmask check passes.
- `toggle_interrupt_enable`: Sets and then clears a bit in a byte at an offset from a pointer while updating a global variable and modifying the interrupt enable bit in the interrupt enable register.
- `process_serial_node`: Checks a flag in a structure and either clears a flag or moves a node from a list to a new destination while decrement/incrementing a counter.
- `process_linked_list_node_2`: Checks a flag in a structure and either clears a flag or moves a node from a list to a new destination while decrementing a counter.
- `decrement_counter_and_clear_flag_2`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in a global variable.
- `handle_serial_buffer`: Checks a pointer against a global, potentially modifies a flag and a hardware register, and calls a library function or another routine.
- `decrement_counter_and_clear_flag`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in the structure, updates a pointer stored at an offset from that word, decrements a short counter in a6, modifies two flag bytes in the structure and clears a byte.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `execute_multiple_callbacks`: Executes a sequence of several different routine pointers retrieved from a table and updates a list pointer.
- `execute_multiple_calls`: Performs a sequence of library calls and updates a pointer-based structure before returning a value.
- `register_serial_device_2`: Updates a device list and modifies hardware interrupt enable bits while potentially adding a new device structure to a linked list.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `register_serial_device`: The routine links a serial device structure into a global device list and updates a device counter.
- `process_serial_device_interrupt`: Checks device status flags and handles the removal of a device from a linked list if certain conditions or counters expire.
- `decrement_device_counter`: Decrements a value in a structure and, if the new value is zero, updates a global variable, clears a flag, and calls another routine.
- `remove_device_node`: Removes a node from a linked list and updates a device counter and status flags.
- `update_serial_buffer_and_status`: Updates a buffer pointer and a counter, checks a status flag, and conditionally modifies a bitmask or a global value before calling a function pointer.
- `update_device_status_and_interrupts`: Updates a global variable and a counter, checks a list of bytes to determine if a certain value should be written to a global variable, and potentially updates an interrupt request register.
- `update_device_status_3`: Updates a global variable and a counter, and checks a list of bytes to determine if a certain value should be written to a global variable.
- `remove_device_from_list`: Removes a node from a linked list and updates a device counter and status flags, potentially calling another routine if a certain value is met.
- `set_global_and_interrupt`: Sets a global variable to a constant value and modifies the interrupt enable bit of the INTENA register.
- `check_serial_interrupt_status`: Reads a byte from a global variable and combines it with a value from a hardware register to update a global status flag.
- `configure_device_parameters_2`: Sets a global variable, calls another routine, and if certain structure members are non-null, calls another routine again before clearing a bit in a structure and writing several 32-bit constants to a structure.
- `check_status_and_call`: Calls a routine and then checks a bitmask against a structure byte to decide whether to call a third routine.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `check_device_status`: Checks a pointer against a global and modifies a status byte or calls another routine if a certain bitmask or pointer is present.
- `modify_buffer_and_interrupt_state`: Clears a bit in a structure, calls another routine if a pointer is non-null, and sets a global variable before potentially calling another routine based on a bitmask check.
- `set_interrupt_and_check`: Sets a global variable and modifies the interrupt enable bit in a hardware register before checking a bitmask and calling a routine.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `traverse_and_update_list`: Iterates through a linked list, updating pointers to redirect or remove nodes, and calls another routine for each node processed.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `update_device_state_2`: Updates a structure with values from a global byte and a hardware-related offset, then checks a bitmask to potentially call a routine.
- `link_device_structure`: Links a serial device structure into a global device list and updates a device counter, while also setting a bit in a flag byte and calling another routine.
- `set_flag_and_call_routine`: The routine sets a bit in a byte located via a pointer, calls another routine, stores a value returned in A1 into a word at an offset from the same pointer, and writes a constant to a global variable.
- `clear_serial_flag_and_disable_interrupts`: The routine clears a flag in a byte field of a structure pointed to by a1, and if a condition on that byte and a short field is met it sets another flag in the same byte, clears a pointer field, writes a value to a global variable, and calls another routine via a pointer stored in the structure.
- `reset_device_parameters`: Sets a global variable and, if a certain value is not present, resets several fields in a structure and modifies a hardware register.
- `return_from_subroutine_5`: This routine performs a single return instruction and does nothing else.
- `update_device_status_flags`: Checks a bitmask in a status byte and conditionally clears or sets a bit, while potentially performing a recursive call and updating a global state or a secondary status flag.
- `test_device_status_bitmask_2`: Checks if a specific bitmask is absent from a byte in a structure and, if so, executes a routine via a pointer.
- `test_device_status_bitmask`: Checks if a specific bitmask is absent from a byte in a structure and, if so, executes a routine via a pointer.
- `configure_device_parameters`: Validates a range of input values and copies several fields from a source structure to a destination structure, including updating status flags and calling other routines.
- `compute_serial_status_word`: The routine builds a 16-bit status word from a byte in a structure and a flag in another structure, then stores the word into a global variable.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `set_byte_constant`: Writes a constant byte value to an offset of a pointer.
- `calculate_serial_divisor`: Calculates a divisor-based value and stores it in a structure field and a global variable, while also modifying a byte in a structure based on certain conditions.
- `compute_serial_divisor_and_flags`: The routine divides an input value by the low word of another input and stores the result in a structure field, copies that field to a global variable, clears a flag bit in a byte field, and conditionally sets a high bit in the global variable and a flag bit in the byte field based on two structure bytes.
- `initialize_serial_device_parameters`: Checks if a device pointer is valid and configures a device structure with a specific baud rate, or certain control flags, and updates a global variable.
- `configure_serial_device_state`: Writes constant values and pointers to a device structure and updates a global variable.
- `initialize_structure_fields`: Stores several values, including a pointer to a global variable, into a structure.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `update_serial_device_state`: Updates a device structure by modifying status flags, updating a pointer to a global variable, and setting a function pointer to another routine.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `return_zero_2`: The routine returns a zero value.
- `update_linked_list`: Updates a pointer-based linked list and modifies a byte in a structure before calling another routine.
- `update_serial_buffer_list`: Iterates through a linked list of data buffers, updating buffer pointers, decrementing a remaining-buffer count, and modifying buffer status flags or pointer offsets.
- `process_serial_buffer_byte`: Iterates through a linked list of data buffers to extract or write bytes, updating buffer pointers and decrementing a remaining-buffer counter.
- `update_serial_buffer_chain`: Iterates through a linked list of data buffers, updating buffer pointers, decrementing a remaining-buffer counter, and updating status flags or length-check variables.
- `update_serial_buffer_state`: Updates a linked list of data buffers by decrementing a counter and updating buffer pointers or length-check flags.
- `process_serial_buffer_node`: Iterates through a linked list of data buffers to extract or write bytes, updating a pointer and a counter or length-check variable.
- `return_empty_routine_4`: The routine performs no operations and immediately returns to its caller.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `process_serial_buffer_nodes_2`: Iterates through a linked list of data buffers, updating buffer pointers, decrementing a node counter, and clearing or setting status flags or length-check variables.
- `update_serial_buffer_state_2`: Updates status flags and pointer or length-check variables while iterating through a linked list of data buffers.
- `process_serial_buffer_nodes`: Iterates through a linked list of data buffers to extract or write bytes, updating a pointer and a counter or length-check variable.
- `update_structure_bit_and_copy`: Sets the highest bit of a word in a structure and sets a bit in a byte.
- `handle_serial_buffer_overflow`: Checks a buffer limit and, if exceeded, resets a pointer or updates a linked list or circular buffer.
- `manage_serial_buffer_and_interrupts`: Updates a pointer-based buffer with a byte and increments a counter, while conditionally disabling or enabling interrupts via the certain bit in the interrupt enable register.
- `update_serial_buffer_logic`: Checks a bit in a structure and, if certain conditions are met, updates a pointer-based buffer with a byte and increments a counter.
- `append_to_list_with_check`: Checks a value against a limit, appends a byte to a pointer-based buffer, and increments a counter while potentially updating a pointer.
- `update_serial_buffer`: Checks a status flag and, if certain conditions are met, writes a byte from a register to a buffer, increments a pointer, and updates a counter.
- `update_structure_value_2`: Sets the highest bit of a byte in a structure and copies a value from one offset to another if a certain field is zero.
- `append_to_list_and_update`: Checks if a certain value is less than another, then writes a byte to a pointer-based buffer, increments a counter, and potentially resets a pointer if a limit is reached.
- `update_structure_value`: Checks if a value in a structure is zero and, if so, copies a value from another offset in that structure.
- `execute_conditional_function_pointer`: Checks if a value in a global structure is greater than zero, then executes a function pointer retrieved from an offset within a passed pointer or table.
- `modify_buffer_byte`: Copies a byte from one structure offset to another, clears a bit in a byte, and conditionally clears a bit in a global variable or resets a global variable to a constant.
- `modify_buffer_state`: Copies a byte from a structure to another location, sets a bit in a status byte, and conditionally modifies a global variable or resets a byte in a structure.
- `clear_structure_byte`: Sets a byte in a structure to zero.
- `set_structure_flags`: Sets a bit in a structure byte and, if that bit was previously unset, resets a long value, calculates a new value based on a shifted register, and executes an indirect function call.
- `clear_bit_and_return_value`: Clears a bit in a byte within a structure and returns a value from a global variable if a certain bit is set.
- `return_immediately_8`: The routine performs no operations and returns immediately to the caller.
- `process_serial_byte`: Reads a byte from the serial device and appends it to a circular buffer or linked list, while checking for specific control characters or buffer overflow conditions.

### usa1

- `tiny_ori_ori_ori_subi`: Small routine of 4 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0, then subtracts 0 from (a6)+.

### parallel.device

- `return_all_ones`: Returns a value where all bits are set to one.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `set_structure_byte`: Stores a constant byte into a structure field.
- `update_linked_list_2`: Removes a node from a linked list and updates a counter, or resets a pointer if the node matches a certain value.
- `process_linked_list_node`: Manipulates a linked list by updating pointers, decrements a counter in a structure, and conditionally calls another routine.
- `check_pointer_and_call`: Checks if a pointer in a structure is non-zero and, if so, calls another routine.
- `return_empty_routine_5`: This routine is empty and performs no operations before returning.
- `clear_status_bit`: Clears a bit in a byte through a pointer and calls a function through a pointer retrieved from a table.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `traverse_and_clear_list`: Iterates through a linked list, updating pointers to remove nodes, and resets a structure member to zero.
- `reset_structure_value`: Sets a value in a structure to zero.
- `set_status_bits`: Sets a bit in a byte, stores a byte value, and calls a function through a pointer retrieved from a table.
- `update_hardware_state`: Updates a bitmask in a global variable and a structure, then writes a byte to a structure and updates a CIA register.
- `copy_conditional_fields`: Copies a byte from one structure to another and conditionally copies two longwords if a certain bit is set.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.
- `initialize_device_handler`: Initializes a structure or device handler by setting pointers and status flags, and potentially calls a routine to check a linked list or buffer.
- `return_null_pointer`: The routine returns a null pointer value.
- `set_flags_and_zero`: Sets two bits in a byte and clears a long word in a structure.
- `insert_linked_node`: Clears a bit in a byte, and if a certain bit is set, updates a byte, inserts a new node into a linked list, and increments a counter in a structure.
- `update_device_status_2`: updates a byte or a flag or a pointer or a long word or a structure or a CIA register or a bit.
- `update_device_status`: Modifies bits in a byte, copies a long word and a pointer to a structure, and updates various flags based on comparisons and a CIA register.
- `process_buffer_nodes`: Checks a short value, and if non-zero, updates a pointer-based list and certain flags, while potentially iterating through a buffer or checking a limit.
- `process_buffer_nodes_2`: Iterates through a linked list of buffer nodes, updating pointers and counters while checking certain bit flags and value limits.
- `append_to_buffer`: Appends a byte to a buffer, increments a counter, and conditionally calls another routine or updates flags.
- `append_to_buffer_logic`: Copies a longword, clears certain bits in a byte, executes an indirect function, and conditionally clears a bit in a byte or executes another routine if a counter is zero.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `recurse_on_pointer`: Performs a recursive call to itself without modifying any registers or data.
- `recursively_calls_itself`: This routine performs an infinite recursive call to itself, or otherwise acts as an empty wrapper that calls itself before returning.
- `decrement_and_test_zero`: Decrements a value in a structure and, if the result is zero, calls another routine.
- `check_global_flag`: Checks a global flag and either executes two indirect functions using pointers from a table or sets a bit in a global variable.
- `decrement_and_check_threshold`: Decrements a byte in a structure and, if the new value is below a threshold, calls a function pointer from a table.
- `update_structure_bits`: Checks if specific bits in a byte are unset and, if so, writes a constant byte to a structure before calling another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `evaluate_recursive_branching_logic`: Stores a parameter in a global variable and calls a routine to check a condition, then executes a sequence of further routine calls or indirect function calls based on the returned boolean or a calculated offset.
- `execute_conditional_logic_3`: Stores a value in a global variable and executes several different routines or indirect function calls based on the return value of a routine.
- `invoke_indirect_functions`: Executes a routine using a pointer or a library function offset via a global variable or a table.
- `allocate_and_patch_memory_2`: Calls a routine to allocate memory, and if successful, clears a word or sets a longword to a new value before calling another routine.
- `clear_and_call_indirect`: Clears a longword at an offset in a structure and then executes a routine using a function pointer from a global table.
- `clear_memory_word`: Clears a word at a specific offset relative to a register through a pointer.
- `process_clipboard_string`: Recursively calls other routines and modifies a value in a structure, potentially returning a value or calling a routine with a pointer and a new value.
- `extract_font_names`: Uses a pointer to extract or copy a string or name from a font metadata structure or table.
- `traverse_linked_list_7`: Iterates through a linked list, calling a routine for each node, and finally clears a longword in a structure.
- `traverse_linked_list_5`: Traverses a linked list, calling a routine for each node, and finally clears a longword at an offset in a structure.
- `copy_clipboard_buffer_segments_2`: Traverses a linked list of memory buffers to copy a specific number of bytes from a source buffer to a destination buffer, handling segment boundaries.
- `copy_buffer_segments_6`: Traverses a linked list of memory buffers to copy a specific number of bytes from a source buffer to a destination buffer, handling segment boundaries.
- `copy_clipboard_buffer_segments`: Traverses a linked list of memory buffers to copy a specific number of bytes from a source buffer to a destination buffer.
- `copy_buffer_segments_5`: Traverses a linked list of buffers and copies bytes from a source buffer to a destination buffer while handling segment boundaries.
- `copy_buffer_segments_4`: Copies bytes from a source buffer to a destination buffer while handling segment boundaries and updating a pointer in a structure.
- `copy_buffer_segments_2`: Copies bytes from a source buffer to a destination buffer, handling potential segment boundaries, and finally updates a pointer in a structure.
- `copy_buffer_segments`: Copies bytes from a source buffer to a destination buffer, handling buffer wraps or segment boundaries.
- `copy_buffer_with_wrap`: Copies bytes from a source buffer to a destination buffer while managing wrap-around logic or buffer limits.
- `copy_buffer_segments_3`: Copies bytes from a source buffer to a destination buffer, handling segment boundaries and updating a pointer in a structure.
- `append_linked_buffer_chain_2`: Traverses a linked list of memory buffers to find or create a destination capable of holding a sequence of bytes, then copies data from a source pointer into the buffers.
- `update_clipboard_buffer`: Iterates through a linked list of memory blocks to copy or append a sequence of bytes into a target buffer, while checking or updating buffer boundaries and pointer offsets.
- `append_linked_buffer_chain`: Traverses a linked list of memory blocks to find or create a destination buffer capable of holding a sequence of bytes.
- `append_data_to_linked_buffer_chain`: Appends a source buffer to a linked list of memory buffers, traversing the list to find or create a buffer with sufficient remaining capacity.
- `copy_data_to_linked_list`: Traverses a linked list of buffers to copy bytes from a source pointer to a destination offset, updating pointers and handling buffer boundaries.
- `copy_linked_list_buffers`: Traverses a linked list of memory buffers to copy a stream of bytes from a source pointer into the buffers, updating the destination pointer and handling buffer or list boundaries.
- `copy_linked_list_data`: Traverses a linked list of buffers to copy bytes from a source pointer to a destination offset, updating pointers and allocating memory if necessary.
- `copy_memory_block_8`: Copies bytes from a source pointer to a destination offset within a buffer, potentially updating a pointer or calling another routine if a block ends.
- `copy_and_patch_memory`: Copies bytes from a source to a destination, potentially calling another routine to handle new memory blocks or pointer updates.
- `copy_memory_block_6`: Copies bytes from a source pointer to a destination offset within a buffer.
- `copy_bytes_with_pointer_traversal`: Copies bytes from a source pointer to a destination offset within a buffer and traverses a linked list of pointers, potentially calling another routine or updating a global pointer if a null pointer or certain boundary condition is encountered.
- `modify_byte_flag`: Checks specific bits in a byte at an offset and clears or sets the highest bit or a lower bit.
- `modify_byte_bits`: Checks if a bit is set in a byte, and if a certain condition is met, clears or sets specific bits in a byte at an offset.
- `compare_pointer_values`: Compares a value at an offset in a structure pointed to by a pointer against a value in another structure and returns a boolean result.
- `return_sign_extended_value`: Loads a constant into a register and returns with the sign-extended value.
- `return_zero_constant_2`: The routine returns a constant zero value using a sign-extended return or a direct zero-load instruction.
- `tiny_moveq_ext_rts`: Small routine of 3 instructions that loads 1 into d0, then sign-extends d0, then returns.
- `copy_structure_fields_2`: Library code that copies several words and a longword from one structure or object to another and then calls another routine.
- `recalculate_buffer_bounds_and_segments`: Updates a structure's length and boundary pointers while checking or adjusting segment offsets and flags.
- `update_clipboard_buffer_state_logic`: Updates a clipboard structure's buffer boundaries, length, and status flags, or handles segment re-allocation and pointer adjustments based on certain state-dependent conditions or buffer-end checks.
- `manage_clipboard_buffer_state`: Updates the internal state, buffer boundaries, and flags of a clipboard structure, including adjusting buffer lengths and managing segment offsets or flags.
- `update_clipboard_buffer_state_5`: Updates the internal state and boundary pointers of a clipboard structure, managing buffer lengths and adjusting pointer offsets or segment flags.
- `update_clipboard_state_machine_4`: Updates the internal state and buffer boundaries of a clipboard structure by checking a state-dependent flag and adjusting pointer offsets or buffer lengths.
- `update_clipboard_buffer_state_4`: Updates the internal state, buffer boundaries, and flags of a clipboard structure, including adjusting buffer lengths and managing segment offsets or clearing certain flags.
- `update_clipboard_buffer_state_3`: Updates the internal state, buffer boundaries, and flags of a clipboard structure, including adjusting buffer lengths and managing segment or buffer-related flags or counters.
- `update_clipboard_buffer_bounds_2`: Updates the internal boundaries, lengths, and flags of a clipboard buffer structure.
- `update_clipboard_buffer_state_2`: Updates the internal state, buffer boundaries, and flags of a clipboard structure, including adjusting buffer lengths and managing segment or status flags based on certain state conditions.
- `update_clipboard_state_machine_3`: Updates the internal state and buffer boundaries of a clipboard structure, managing buffer size or clearing status flags based on the current state or incoming data.
- `update_clipboard_state_machine_2`: Updates the internal state and buffer boundaries of a clipboard structure, managing buffer size or clearing status flags based on the current state or pointer-based buffer contents.
- `update_clipboard_state_machine`: Updates the internal state and buffer boundaries of a clipboard structure, managing buffer size or clearing status flags based on the current state or pointer-based metadata.
- `update_clipboard_buffer_size`: Updates the length and offset or status flags within a clipboard buffer structure.
- `update_clipboard_buffer_bounds`: Updates or resets the length and offset or status flags within a clipboard buffer structure.
- `update_clipboard_buffer_state`: Updates the length and offset or status flags of a clipboard buffer structure.
- `reset_clipboard_buffer_state`: Resets a buffer's length or offset and clears a status flag or byte.
- `process_linked_list_node_5`: Iterates through a linked list of structures, updating pointer offsets, checking bit-flags, and adjusting length or boundary counters based on the contents of the next node or a global limit.
- `traverse_linked_list_with_limit_2`: Iterates through a linked list, checking a value at an offset against a limit, and calls another routine if a null pointer or limit is reached.
- `traverse_linked_list_3`: Traverses a linked list of pointers.
- `traverse_linked_list_with_limit`: Iterates through a linked list of pointers, comparing a value at an offset of a pointer against a limit, until a null pointer or the limit-exceeding condition is met.
- `set_byte_value`: Writes a constant byte value to an offset within a structure.
- `set_structure_member`: Stores a constant byte value into a memory location relative to a pointer.
- `set_structure_member_2`: Updates a specific byte within a structure or a data block using a pointer.
- `set_structure_member_3`: Writes a constant byte to an offset within a structure.
- `set_structure_member_4`: Performs a single byte write to a memory location relative to a pointer.
- `update_structure_byte_sequence`: Updates a single byte within a memory structure or a value at a specific offset relative to a pointer using a constant or register-provided offset.
- `traverse_linked_list_13`: Traverses a list of structures while updating a pointer and setting a flag bit in a byte.
- `traverse_list_offset`: Library code that calls another routine with an incremented pointer or offset.
- `increment_list_counter`: Traverses a list and increments a counter stored in a structure.
- `traverse_list_offset_large`: Library code that calls another routine with a significantly larger incremented pointer or offset.
- `copy_structure_field`: Copies a longword from an offset relative to a pointer to another offset in a structure.
- `copy_longword_from_offset`: Copies a longword from an offset relative to a pointer-based address into a different offset relative to a pointer-based address.
- `insert_clipboard_node`: Traverses a linked list of clipboard nodes to find an insertion point and allocates a new node if a null pointer or certain conditions are met.
- `manage_clipboard_node_list`: Traverses a linked list of clipboard-related structures to find or remove a specific node.
- `allocate_and_reconfigure_clipboard_node`: Attempts to allocate a new memory block for a clipboard node and configures its metadata, including size and flags.
- `reallocate_clipboard_buffer`: Checks if a string or buffer can fit within a certain memory range and resizes or moves the buffer if necessary.
- `update_memory_block`: Allocates memory and conditionally clears or patches a word or longword, then iterates through a linked list to execute functions with bit-shifted or pointer-derived arguments.
- `allocate_and_patch_memory_3`: Allocates memory and conditionally clears or patches a word or longword, then iterates through a linked list to execute functions.
- `traverse_and_execute_list`: Iterates through a linked list, clearing a structure member if a certain condition is met, and executes functions via a global table or offset.
- `execute_recursive_pointer`: Recursively calls another routine using a pointer or a bit-shifted value derived from a byte, and returns a byte from a structure.
- `increment_and_recurse_pointer`: Increments a value at a memory offset and recursively calls other routines using pointers or library vectors based on a conditional check.
- `process_clipboard_data`: Recursively traverses a linked list while clearing a word in a structure and potentially updating a pointer or a global counter.
- `traverse_and_reset_structure`: Traverses a linked list while resetting a word in a structure to zero based on certain conditions and updating a global counter.
- `traverse_and_reset_flag`: Traverses a linked list while checking a flag and resetting a value in a structure if a certain condition or a comparison fails.
- `traverse_linked_list_and_reset_word`: Traverses a linked list while resetting a word in a structure to zero and updating a global counter.
- `clear_buffer_and_decrement`: Clears two longwords in a structure and decrements a global counter.
- `check_and_set_bit_conditional`: Checks if a global flag is unset, and if so, executes two indirect functions using pointers from a table and returns a value from a third pointer.
- `set_byte_bit`: Sets the third bit of a byte at a fixed offset from a register.
- `modify_and_call_function`: Modifies a byte in a structure and executes a function pointer from a table if a certain value is below a threshold.
- `check_bits_and_set_byte`: Checks certain bits in a byte and, if they are not set, writes a constant byte to a structure and calls another routine.
- `execute_handler_pointer_2`: Loads a pointer from a global variable and jumps to it.
- `execute_indirect_function_48`: loads a function pointer from a global offset and executes it.
- `execute_indirect_function_pointer_19`: loads a function pointer from a global variable and executes it.
- `execute_indirect_function_pointer_20`: loads a function pointer from a global variable and jumps to it.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_21`: Loads a function pointer from a global pointer-to-pointer and executes it.
- `execute_indirect_function_pointer_22`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_23`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_24`: Loads a function pointer from a global offset and executes it.
- `execute_indirect_function_pointer_25`: Loads a function pointer from a global table and executes it.
- `execute_indirect_function_49`: loads a function pointer from a global offset and executes it.
- `execute_indirect_pointer_12`: Executes a function call using a pointer retrieved from a global table.
- `execute_indirect_function_pointer_26`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_27`: loads a function pointer from a global offset and executes it.
- `execute_indirect_function_pointer_28`: loads a function pointer from a global offset and executes it.
- `execute_indirect_function_pointer_29`: Loads a function pointer from a table offset and executes it.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_30`: loads a function pointer from a global offset and executes it.
- `call_library_vector_432`: Library stub: loads a library base from a global variable and calls the library function at offset minus 432 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_31`: Loads a function pointer from a global table and executes it.
- `update_pointer_table`: Stores a pointer to a memory location into a table and increments that pointer, then clears a subsequent word.
- `call_offset_function`: Calls a function pointer located at a fixed offset from a global variable.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_offset_function_2`: Calls a function pointer located at a different fixed offset from a global variable.
- `execute_indirect_function_pointer_16`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_17`: Loads a function pointer from a global table using an offset and executes it.
- `execute_indirect_function_pointer_18`: Loads a function pointer from a global table and executes it.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_indirect_function_89`: Loads a function pointer from a global offset and executes it.
- `store_byte_to_pointer`: stores a byte from a data register into a memory location pointed to by a register.
- `check_and_set_bits`: checks if certain bits in a byte are unset and writes a constant byte to a structure if they are.
- `execute_recursive_call_4`: The routine calls itself indefinitely, creating an infinite loop of self-recursion.
- `perform_no_operation`: the routine is empty and performs no actions.

### printer.device

- `return_error_value`: Returns a constant value representing an error or invalid state.
- `execute_indexed_pointer_2`: Updates a byte in a structure and executes a function call using a pointer retrieved from a table and an offset calculated from a register.
- `execute_indexed_pointer`: Executes a function call using a pointer retrieved from a table and an offset calculated from a register.
- `update_linked_list_nodes`: Updates pointers in a linked list and modifies bits in a byte at a specific offset within a structure.
- `update_linked_list_node`: Updates a pointer in a structure and conditionally calls another routine.
- `manage_linked_list`: Sets a bit in a byte, performs a linked list insertion if a flag is set, and calls functions through pointers retrieved from a table.
- `handle_printer_status_and_update_buffer`: Checks a function pointer in a structure and, if non-zero, stores a register value into a structure offset and calls another routine.
- `store_pointer_and_call`: Stores a value from a register into a structure offset and calls a routine through a pointer found in a structure.
- `initialize_printer_buffer`: Checks several library function return values and, if a certain condition is met, initializes a memory range with a constant value and updates several pointers.
- `execute_multiple_functions`: Executes a sequence of different function pointers retrieved from a global variable or a library offset.
- `execute_handler_callback`: Loads a function pointer from a memory location and calls it, while also calling a routine that prepares a date string buffer.
- `traverse_linked_list_10`: Iterates through a linked list while setting or clearing status bits in a byte.
- `execute_recursive_call`: The routine calls itself indefinitely.
- `set_status_bit`: Sets a bit in a byte at a specific offset within a structure.
- `clear_status_bit_2`: Clears a bit in a byte at a specific offset within a structure.
- `traverse_linked_list_8`: Iterates through a linked list by checking a pointer or a value at an offset until a null terminator is encountered.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `execute_infinite_recursion`: An empty routine that calls itself indefinitely.
- `initialize_printer_request_list`: Initializes a linked list or data structure, clears a memory range, and iterates through a list to clear specific bits in a global byte or set a bit in a node's metadata before executing a callback or a function pointer.
- `modify_bit_in_memory`: Clears or sets a specific bit within a byte at a memory location or modifies a bit in a byte or bitset-like structure.
- `process_printer_requests`: Iterates through a linked list, setting a bit in a byte and calling a function for each node, while also clearing bits in a global byte based on certain values.
- `dereference_pointer_and_jump`: Loads a pointer from a memory location and executes a jump to the address contained at that destination.
- `update_status_and_store_registers_2`: sets a bit in a global byte, stores two registers into a global table, and calls another routine.
- `update_status_and_store_registers`: Updates a bit in a global byte and stores two registers into a global table before calling another routine.
- `adjust_stack_and_jump`: Increments the stack pointer by eight bytes and performs a short branch to another routine.
- `update_status_and_values`: Sets a bit in a global byte and stores two registers into a global table before calling another routine twice.
- `check_status_bits`: Checks a byte in a structure and clears specific bits if certain values match a pointer or a global byte.
- `clear_status_bits`: Traverses a structure and clears specific bits in a global byte if certain conditions or pointer comparisons are met.
- `clear_status_bits_recursively_5`: Recursively clears specific bits in a global byte while traversing a linked list or structure.
- `clear_status_bits_recursively_3`: Recursively clears bits in a global byte while traversing a structure and checking for a specific return value or error condition.
- `traverse_linked_list_6`: Recursively traverses a linked list or structure, clearing specific bits in a global byte or returning a value if a certain condition is met.
- `reset_structure_fields_2`: Writes a constant to a structure offset, copies a value from a global table, and clears other structure fields before calling a routine through a pointer.
- `clear_status_byte`: Sets a byte at a specific offset to zero and calls a routine through a pointer found in a structure.
- `execute_conditional_calls`: Checks a byte in a structure and, if it does not match a certain character, executes two indirect function calls.
- `execute_indirect_calls_2`: Executes two routines through pointers located within a structure.
- `recursively_clear_status_bits`: Recursively checks a status byte against specific bitmask constants and clears individual bits if a match is found.
- `process_status_bit_clearing`: Iteratively checks a value against a set of constants and clears specific bits in a byte within a structure.
- `clear_status_bits_recursively_6`: Recursively clears a specific bit in a byte within a structure or a global variable.
- `clear_status_bits_recursively_4`: Clears specific bits in a global byte or a structure member while recursively calling another routine.
- `process_structure_nodes`: Iterates through a structure, clearing bits in a global byte and executing indirect function calls if certain values match.
- `clear_status_bits_recursively_2`: Updates a pointer-based structure and checks a buffer for a carriage return or error code before recursively clearing certain bits in a byte.
- `update_printer_status_and_clear_bits`: Updates a pointer-based structure and checks a string or buffer for a carriage return and an error code before recursively clearing the least significant bits of a global byte.
- `clear_status_bits_recursively`: Checks a return value and recursively clears specific bits in a global byte if certain conditions are met.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `update_list_and_copy_string`: Sets a status bit in a byte and performs a linked list insertion if a flag is set, then iterates through an array of words to find a marker before copying a null-terminated string into a lookup table.
- `reset_structure_field`: Clears a 32-bit field in a structure and conditionally updates it with a value from another field based on the return value of a routine.
- `update_list_and_copy_string_2`: Sets a status bit in a byte and performs a linked list insertion if a flag is set, then searches for a marker in an array of words to copy a null-terminated string into a lookup table.
- `update_list_and_call_library`: Sets a status bit in a byte and performs a linked list insertion if a flag is set, then calls a library function via a vector.
- `dispatch_pending_entry`: Checks a condition via a routine call, and if met, populates a structure with two parameters and executes several routines through a global base.
- `update_device_status_4`: Copies a block of data from a global source to a local buffer and then writes a byte or a word to a pointer, depending on a global flag.
- `copy_and_initialize_memory`: Copies a block of data from a source pointer to a destination pointer and sets a value in a global variable.
- `parse_string_to_buffer_3`: Iterates through a source string to find a specific delimiter, copying or skipping data into a destination table while updating a counter.
- `parse_stream_to_table`: Copies bytes from a source stream into a destination table while incrementing a counter, then recursively processes or skips data based on a delimiter or a secondary routine's return value.
- `copy_and_parse_buffer_stream`: Copies bytes from a source pointer to a destination table while incrementing counters.
- `search_and_copy_buffer`: Searches a memory range for a specific byte value and copies a sequence of bytes from a source pointer to a destination table.
- `scan_and_copy_buffer`: Iterates through a memory range to find a specific character, then copies a sequence of bytes from a source pointer into a destination table.
- `parse_stream_to_table_2`: Iterates through a source stream to find a specific delimiter, copying or skipping data into a destination table while updating a counter.
- `copy_and_increment_pointer`: Copies a byte from a source pointer to a destination pointer while incrementing both pointers and a counter.
- `clear_memory_offset`: Writes zero to a memory location offset from a pointer.
- `transform_input_character_table`: Iterates through a table to check for matching input characters and replaces or transforms them with new values or special characters.
- `map_input_character`: Checks a byte against a table and writes a specific character or a transformed value to a string buffer.
- `map_input_to_char_3`: Performs a lookup in a table to map an input byte to a character or a constant, and updates a global variable.
- `map_input_to_character_4`: Maps an input byte to a character using a lookup table and shifts a buffer left while incrementing a counter.
- `map_input_to_char_2`: Maps an input byte to a character using a table, shifts a buffer left, and increments a counter in a structure.
- `map_input_to_char`: Maps an input byte to a character using a table and shifts a buffer left, while updating a counter in a structure.
- `map_input_to_character_3`: Searches a table for a matching byte and writes a corresponding character to a destination buffer, then shifts the buffer left.
- `map_input_to_character_2`: Searches a table for a matching byte and writes a corresponding character to a destination buffer, then shifts a buffer or string left by one byte.
- `map_input_to_character`: Searches a table for a matching byte and writes a corresponding character to a destination buffer.
- `search_and_shift_string_2`: Searches a table for a matching byte, updates a global variable via a routine call, and shifts bytes in a buffer leftward while potentially updating a global or a pointer.
- `search_and_shift_string`: Searches a table for a matching byte, updates a global variable via a routine call, and shifts bytes in a buffer leftward while potentially replacing a character with a constant or a calculated value.
- `update_input_buffer_and_map_char`: Searches a table for a matching byte and, if found, shifts the contents of a buffer leftward while replacing a character in a string with a value derived from a lookup table or a constant.
- `process_printer_escape_sequences_3`: Parses a stream of bytes to identify and handle escape sequences, such as character or command-based formatting, and updates a destination buffer.
- `process_printer_escape_sequences_2`: Parses a stream of bytes to identify and handle escape sequences, such as character or command-based formatting, and updates a destination buffer.
- `parse_printer_escape_sequences`: Parses a stream of bytes to identify and handle escape sequences, such as character or command-based formatting, and updates a destination buffer or a global state.
- `process_printer_escape_sequence_2`: Parses a stream of bytes to handle escape sequences and command-based formatting, updating a destination buffer with characters or control codes.
- `parse_command_string_sequence`: Parses a sequence of command bytes and associated string arguments, updating a destination buffer and handling control characters or string-based parameters.
- `process_string_command_sequence`: Parses a sequence of command bytes and associated string arguments, updating a destination buffer and handling control characters or string-based tokens.
- `parse_string_command_sequence`: Parses a sequence of command bytes and associated string arguments, updating a destination buffer and handling control characters or string-based command lookups.
- `process_printer_escape_sequence`: Parses a stream of bytes to handle escape sequences and command-based formatting, updating a destination buffer with characters or control codes.
- `process_printer_command_stream`: Parses a stream of bytes to handle escape sequences and command-based formatting, updating a destination buffer with characters or control codes.
- `parse_string_stream`: Parses a stream of bytes from a buffer, handling escape sequences and updating a pointer.
- `parse_command_stream`: Parses a stream of bytes to execute or decode specific command sequences, updating a destination buffer and a pointer or counter.
- `expand_string_tokens`: Parses a string to resolve backslash-delimited path components and drive identifiers, appending the resolved characters or special tokens to a destination buffer.
- `expand_path_string`: Parses a file path string to resolve drive identifiers and handle directory separators, writing the resulting path or drive index to a destination buffer.
- `parse_string_path`: Parses a string or file path, checking for backslashes and drive-like identifiers, and writes the resulting characters or converted numeric values into a destination buffer.
- `parse_path_string`: Parses a string or file path, converting numeric drive identifiers and handling backslashes or forward slashes.
- `parse_device_string_path`: Parses a string or path-like buffer to extract or convert numeric or character-based identifiers.
- `parse_printer_command_string`: Parses a string to extract or convert numeric or character-based printer commands, such as port or file identifiers, and writes the resulting value or character to a destination buffer.
- `parse_string_to_integer`: converts a sequence of up to three ASCII digit characters into an integer and stores it in a buffer.
- `copy_and_increment_string_buffer`: Copies a byte from a source pointer to a destination pointer and increments a counter or pointer.
- `parse_printer_escape_sequence`: Parses a stream of bytes to identify and handle escape sequences, such as character or command-based formatting, and updates a destination buffer.
- `adjust_global_values`: Modifies a global variable by subtracting a calculated difference and sets another global variable to zero.
- `copy_bytes_to_buffer`: Copies a sequence of bytes from a global table to a destination buffer while incrementing a pointer, provided a certain length check passes.
- `copy_bytes_to_pointer`: Copies a sequence of bytes from a global table to a destination pointer while incrementing that pointer.
- `decrement_global_pointer`: Decrements a global variable by one.
- `copy_memory_to_pointer`: Copies a sequence of bytes from a global table to a destination pointer and increments that pointer.
- `copy_memory_range`: Copies a sequence of bytes from a source table to a destination pointer and increments the destination pointer.
- `parse_input_digits`: Parses a sequence of characters to convert digit characters into an integer value stored in a buffer.
- `parse_input_characters`: Checks if a character matches certain control or digit values and updates a memory location or a global variable with a modified value.
- `transform_input_characters_2`: Checks if a character matches certain control or digit values and updates a memory location with a modified value.
- `transform_input_characters`: Checks if a character is a digit and, if so, updates a memory location with a modified value based on that character.
- `parse_input_characters_2`: Checks if a character is a digit or a control character, updating a memory location with a modified value or a pointer to a buffer.
- `get_next_table_value`: Increments a value at a pointer and retrieves a byte from a global table using the previous value.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 0 into d0, then copies d1 to d0, then returns.
- `validate_decrementing_index`: Decrements a global variable and checks a bitmask against a value in a table until a specific bit pattern or a certain value is reached.
- `decrement_and_validate_index`: Decrements a value through a pointer and continues looping as long as a specific bitmask check on a table entry fails, then sets a global variable to one.
- `reset_global_state_2`: Clears several global variables and, if certain conditions are met, calls two routines and sets a set of global variables to specific values.
- `copy_data_to_globals`: Copies six 16-bit values from a structure into a set of global variables.
- `compare_global_table_values`: Compares several words in a global table against registers and returns zero if they all match, or a certain value if any do not match.
- `return_zero_3`: The routine is empty and always returns a zero value.
- `parse_string_recursively`: Recursively searches through a memory range for a specific character or a null terminator.
- `search_string_for_delimiter_4`: Searches a memory range for a specific character and recursively processes or skips segments if a match or a null terminator is encountered.
- `search_string_for_delimiter_3`: Searches a memory range for a specific character, recursively processing or skipping segments if a match or a null terminator is encountered.
- `search_string_range`: Iterates through a memory range to find a matching character and calls another routine recursively.
- `traverse_string_range`: Iterates through a range of memory, checking for matching characters and recursively calling another routine.
- `copy_string_with_replacement_3`: Copies a string from a source pointer to a destination buffer, replacing a specific byte value with a null terminator and replacing another byte value with a byte from a pointer.
- `copy_string_with_replacement_2_2`: Copies a string from a source to a destination, replacing certain characters with a value from a pointer, and returns a pointer or a certain value.
- `copy_string_with_replacement`: Copies a string from a source to a destination, replacing certain characters, and returns a pointer or a certain value if a specific character is encountered.
- `copy_string_with_replacement_2`: Copies a string from a source to a destination, replacing certain characters with a value from a pointer, and returns a pointer or a certain value if a specific charact.
- `update_printer_device_state`: Updates the printer device status and flags by checking a bitmask and calling a library function with a modified buffer pointer.
- `update_printer_status_flags_2`: Updates a status bitmask or flags field within a structure based on certain hardware or device state conditions.
- `update_printer_status_flags`: Updates a bitmask or flags field within a structure based on certain hardware or device state conditions.
- `configure_printer_status_flags`: Updates a status bitmask or flags field within a structure based on certain hardware or device state conditions.
- `update_printer_timing_parameters_10`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `calculate_printer_timing_parameters`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_9`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_8`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_4`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_7`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_5`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_6`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `update_printer_timing_parameters_3`: Calculates and updates timing or buffer offset values within a structure based on a scaling factor and a global limit.
- `recalculate_printer_timing_parameters_3`: Calculates a new timing or buffer offset value based on a scaling factor and a global limit, then updates a timing or buffer offset structure.
- `update_printer_timing_parameters_2`: Updates a structure containing timing or buffer offset values based on a returned value or a global limit.
- `recalculate_printer_timing_parameters_2`: Calculates a new timing or buffer offset value based on a scaling factor and a global limit, then updates a timing or buffer offset structure.
- `recalculate_printer_timing_parameters`: Updates a timing or buffer offset structure by recalculating or clamping timing values based on a provided or global limit.
- `update_printer_timing_parameters`: Updates a structure containing timing or buffer offset values based on a returned value or a global limit.
- `update_printer_buffer_state_2`: Updates a structure containing buffer pointers and error flags by shifting or recalculating certain offset values.
- `update_printer_buffer_status`: Updates buffer pointer offsets and error flags within a structure, including a conditional rotation or shift of buffer pointer offsets and a bitwise modification of a status or error flag.
- `rotate_printer_buffer_pointers`: Rotates or shifts buffer pointer offsets and updates error flags or status bits within a structure.
- `update_printer_buffer_state`: Updates a structure containing buffer pointers and error flags by shifting or recalculating certain offset values.
- `adjust_printer_buffer_bounds_2`: Updates or clamps buffer pointer and offset limits by comparing current values against a maximum capacity and a target value.
- `update_printer_buffer_bounds_3`: Updates buffer size and offset limits by comparing current pointer values against maximum capacity and adjusting or clamping them.
- `update_printer_buffer_bounds_2`: Updates a buffer structure by recalculating pointer offsets and checking size limits against a maximum capacity.
- `recalculate_printer_buffer_bounds_2`: Updates a buffer structure by adjusting pointer offsets and checking size limits against a maximum capacity.
- `adjust_printer_buffer_bounds`: Updates a buffer structure by recalculating pointer offsets, checking size limits, and updating associated status or error flags.
- `update_printer_buffer_bounds`: Recalculates the start and end pointers or size limits of a buffer and updates associated status or error flags.
- `recalculate_printer_buffer_bounds`: Updates a buffer structure by recalculating size or pointer offsets and checking a list of linked error or status flags.
- `update_printer_buffer_metadata`: Updates a buffer structure by recalculating size or pointer offsets and checking a list of linked error or status flags.
- `validate_and_reallocate_printer_buffer_2`: Checks the integrity of a printer buffer structure and certain flags, then reallocates or copies the buffer structure to a new memory location if certain size or content conditions are met.
- `validate_and_reallocate_printer_buffer`: Checks the integrity and size of a printer buffer structure, potentially reallocating or expanding a memory block to hold a copy of the source structure.
- `reallocate_printer_buffer_5`: Checks the size and validity of a printer buffer structure and reallocates or expands a new memory block to hold a copy of the source structure.
- `allocate_and_initialize_buffer`: Checks a list of pointers for a certain bit flag, then allocates a new memory block and copies a source structure or table into it.
- `expand_printer_buffer_structure`: Checks a list of pointers for a specific flag and recursively allocates or expands a memory block to store a copy of a source structure.
- `allocate_and_initialize_buffer_2`: Allocates a new memory block and initializes its contents by copying data from an existing source buffer.
- `initialize_printer_buffer_structure`: Allocates and initializes a new memory block for a printer buffer, copying an initial block of data and then recursively allocating or expanding subsequent memory segments or metadata structures.
- `reconfigure_printer_buffer_2`: Resizes and reinitializes a memory buffer by copying a block of data and calculating a new buffer size based on a bit-mask or bit-shift.
- `initialize_printer_buffer_mask`: Initializes a memory buffer by copying a small block of data, calculates a new buffer size based on a bit-masking or XOR-obfuscated pattern, and populates the buffer with bit-shifted or XOR-obfuscated pixel-like data.
- `reconfigure_printer_buffer_and_mask`: Reallocates a memory buffer and populates it with bit-masked or XOR-obfuscated pixel-like data.
- `reallocate_and_populate_printer_buffer_2`: Reallocates a memory buffer and populates it with bit-masked or XOR-obfuscated pixel-like data.
- `reallocate_and_populate_printer_buffer`: Expands a memory buffer by calculating a new capacity and populating it with bit-masked or XOR-obfuscated pixel-like data.
- `expand_printer_buffer_and_reallocate_2`: Expands a memory buffer by calculating a new capacity, reallocating a larger memory area, and populating the new buffer with bit-masked color data.
- `expand_printer_buffer_and_reallocate`: Expands a memory buffer by calculating a new capacity, reallocating a larger memory area, and populating the new buffer with bit-masked color or pattern data.
- `resize_printer_buffer_2`: Calculates a new buffer capacity and attempts to allocate a new memory area using a pointer-based buffer management structure.
- `reallocate_printer_buffer_4`: Calculates a new buffer size based on an existing capacity and attempts to allocate a new memory area.
- `resize_printer_buffer`: Calculates a new buffer size based on a certain capacity and attempts to allocate a new memory area.
- `reallocate_printer_buffer_3`: Calculates a new buffer size based on an existing offset and capacity, then attempts to allocate a new memory area.
- `reallocate_printer_buffer_and_copy_data_3`: Calculates a new buffer size based on an existing offset and capacity, then attempts to allocate a new memory area.
- `reallocate_printer_buffer_and_copy_data_2`: Checks buffer capacity flags and reallocates memory buffers of a certain size, then copies or initializes data within the new buffer.
- `reallocate_printer_buffer_and_copy_data`: Resizes a memory buffer by allocating a new area, copying existing data, and potentially clearing or initializing a portion of the new buffer.
- `reallocate_printer_buffer_2`: Resizes a memory buffer by calculating new or old memory offsets and copying or clearing data.
- `reallocate_printer_buffer`: Resizes or resets a memory buffer and updates its status flags or length based on the current printer state.
- `reconfigure_printer_buffer`: Resizes or resets a memory buffer and updates its status flags or length based on the current printer state.
- `update_printer_buffer_contents`: Updates or resets a memory buffer with new data or default values based on the current printer status and buffer capacity.
- `sync_printer_buffer_state`: Synchronizes a memory buffer with new data or resets buffer contents to a default value based on status flags.
- `update_printer_buffer_and_state_2`: Updates a memory buffer with new values and modifies status flags or buffer lengths based on the result of a library call.
- `update_printer_buffer_and_state`: Iterates through a memory buffer to update its contents using a helper routine, then checks certain flags to potentially reconfigure the buffer size or reset certain memory ranges to a constant value.
- `reset_printer_buffer_and_reconfigure`: Initializes a memory buffer with a constant value and updates a control structure or global state.
- `configure_printer_buffer_and_clear`: Sets a buffer or a range of memory to a constant value or zero, while updating a control structure or a global state based on certain hardware or configuration flags.
- `process_buffer_range`: Iterates through a memory range and calls another routine to fill a buffer with a constant value.
- `fill_buffer_with_constant`: Iterates through a memory range to fill a buffer with a constant value or calls another routine to process a memory block.
- `initialize_library_data`: Fills a memory block with a constant value and executes a sequence of library-related initialization routines.
- `fill_memory_and_initialize`: Fills a memory block with a constant value and executes a sequence of library-related initialization and processing routines.
- `process_buffer_with_checksum`: Invokes a function pointer and, if certain conditions are met, fills a memory block with a constant value or calls another routine to handle a block.
- `allocate_and_fill_buffer`: Allocates a block of memory and, if successful, fills a portion of it with a certain value or calls another routine to handle the block.
- `process_buffer_loop`: Iterates through a memory range, potentially filling a memory block with a constant value or calling another routine with a pointer and a length.
- `negate_result_of_subroutine`: Calls a subroutine and returns either its return value or the negation of that value depending on a flag or a register's contents.
- `call_subroutine_and_negate`: Calls another routine and returns its result or the negation of that result based on a value in an address register.
- `update_bitfield_from_table`: Fetches a value from a table using an index and performs a bitwise merge into a provided pointer.
- `dispatch_event_handlers`: Executes a series of function pointers stored in a structure, passing various parameters and updating a character in a destination buffer.
- `accumulate_repeated_calls_2`: Recursively calls another routine while incrementing a register or a memory location until a certain counter or condition is met.
- `accumulate_repeated_calls`: Recursively calls another routine while accumulating a value in a register until a counter reaches a certain value.
- `iterate_bitstream_expansion`: Repeatedly calls another routine and accumulates a value from a register until a counter reaches a certain value.
- `return_all_ones_2`: Returns a long word containing all ones.
- `expand_bitstream_data_4`: Calls a routine to expand a packed bitstream of 16-bit words into a destination buffer.
- `expand_bitstream_data_3_2`: unpacks a bitstream by reading a sequence of pointers and extracting individual bits into a destination buffer.
- `expand_bitstream_data_3`: Iterates through a list of pointers to expand compressed or packed bitstream data into a target memory buffer.
- `shift_bit_stream_left`: Iterates through a sequence of words, performing a left shift on each and propagating the bit that was shifted out of the previous word into the least significant bit of the current word.
- `shift_bitstream_and_expand_data`: Iterates through a list of pointers to shift bits left and injects or propagates certain bit values into a block of memory.
- `expand_bitstream_data`: Iterates through a list of pointers to shift bits left and inject or propagate bit values into a block of memory.
- `preserve_and_call_buffer_routine`: Saves several registers to the stack, calls shift_and_copy_buffer_2, restores the registers, and returns.
- `shift_and_copy_buffer_2`: Shifts bits left within words of a destination buffer while propagating a carry bit and copying bytes from a source buffer.
- `shift_and_propagate_bits_2`: Copies a block of data from a source table and then shifts bits left while propagating a carry bit through a destination buffer.
- `shift_and_propagate_bits`: Shifts bits left and propagates the carry bit through a buffer while copying data from a source table.
- `shift_and_copy_data`: Shifts bits left and propagates the carry bit through a buffer, then copies bytes from a source to a destination.
- `shift_and_copy_buffer`: Shifts bits left within words of a destination buffer while propagating the carry bit, then copies bytes from a source buffer into the destination.
- `execute_indirect_function_50`: Executes a function pointer retrieved from an offset of a global pointer.
- `execute_indirect_function_alt`: Executes a function pointer retrieved from a different offset of a global pointer.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `rotate_bits_and_modulo`: Performs a bitwise rotation and arithmetic operations involving a modulo-like check against a value in a register.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_and_shift_registers`: Calculates a value using partial products of registers and returns a bit-shifted version of a register.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `negate_open_result_on_error`: returns the result of an open operation or its negation if the operation failed or returned a non-zero error code.
- `execute_indirect_function_pointer_32`: Loads a function pointer from a memory location offset from a global pointer and executes it.
- `execute_indirect_function_pointer_33`: loads a function pointer from a memory location offset by a constant and executes it.
- `execute_indirect_function_51`: Loads a function pointer from a memory location using an offset and executes it.
- `call_library_vector_534`: Library stub: loads a library base from a global variable and calls the library function at offset minus 534 of that base, returning its result to the caller.
- `interpolate_buffer_values_7`: Calls another routine with a pointer and a bit-shifted value derived from a structure field.
- `attenuate_buffer_elements_2`: Reduces certain byte values within a buffer by calculating a difference relative to a neighbor and subtracting a fraction of that difference.
- `apply_attenuation_block`: Calls another routine using a pointer and a length or offset retrieved from a structure.
- `apply_attenuation_to_block`: Performs differential attenuation on a block of memory.
- `attenuate_buffer_elements`: Iterates through a buffer of 4-byte structures and conditionally reduces certain byte values.
- `reduce_buffer_elements`: Iterates through a buffer of 4-byte structures and reduces certain byte values based on a calculated difference and a flag in a global variable.
- `reduce_buffer_elements_linearly`: Iterates through a buffer of 4-byte structures, calculating a reduction amount based on the difference between a source value and a threshold, then subtracts that amount from the buffer's components.
- `reduce_buffer_values_4`: Calculates a reduction amount based on buffer values and subtracts it from a destination buffer.
- `reduce_buffer_values_3`: Subtracts a calculated reduction amount from bytes in a buffer.
- `reduce_buffer_values_2`: Reduces values in a buffer by calculating differences between bytes and applying a reduction amount, controlled by a loop counter.
- `reduce_buffer_values`: Iteratively reduces values in a buffer by calculating or using differences between bytes, controlled by a loop counter and certain bit checks.
- `interpolate_buffer_values_6`: Iterates through a buffer to perform a weighted average or linear interpolation between adjacent byte values based on flags in a global variable.
- `interpolate_buffer_values_5`: Iterates through a buffer to perform a weighted average or linear interpolation between adjacent byte values based on a flag in a global variable.
- `interpolate_buffer_values_4_2`: Iterates through a buffer to perform a weighted average or linear interpolation between adjacent byte values.
- `interpolate_buffer_values_4`: Calculates a weighted or averaged value and adds it to a destination buffer.
- `interpolate_buffer_values_3`: Adds a weighted or averaged value to bytes in a buffer.
- `interpolate_buffer_values_2`: Performs a weighted or averaged update of bytes in a buffer based on the difference between adjacent or nearby values.
- `interpolate_buffer_values`: Iterates through a memory buffer to perform a weighted or averaged update of certain bytes based on the difference between adjacent or nearby values.
- `expand_bitstream_buffer_2_2_2`: Expands or transforms a bitstream buffer by using a lookup table and bitwise XOR operations.
- `expand_bitstream_buffer_2_2`: Expands or transforms a bitstream buffer by using a lookup table and bitwise XOR operations.
- `expand_bitstream_buffer_2`: Iterates through a source buffer to decompress or transform data using a lookup table and bitwise XOR operations.
- `expand_bitstream_buffer`: Copies a block of data and expands or transforms certain words based on a lookup table or bitwise operations.
- `expand_bitstream_data_5`: Expands a bitstream or lookup table by reading words from a source pointer and writing either a direct table lookup or a bitwise XOR-transformed byte to a destination buffer.
- `expand_and_transform_stream`: Iterates through a source buffer to perform conditional bitwise XOR transformations and table-based lookups, writing the resulting bytes to a destination buffer.
- `process_buffer_stream_2`: Processes a stream of bytes by applying bitwise transformations and storing the results into a destination buffer based on certain bit-flags.
- `process_buffer_stream`: Iterates through a source buffer and writes or modifies bytes in a destination buffer based on certain bitwise conditions and a counter.
- `expand_bitstream_data_6`: Expands a bitstream or lookup table by reading words from a source pointer and writing either a direct table lookup or a bitwise XOR-transformed value.
- `copy_and_process_buffer`: Copies a block of data from a source pointer to a destination pointer, invoking another routine to process each copied word.
- `copy_buffer_data`: Copies a block of data from a source buffer to a destination buffer while calling another routine to process each element.
- `transform_buffer_bits`: Iterates through a buffer and writes a calculated byte value to a specific offset in each element based on bitwise checks of certain bytes.
- `transform_buffer_with_conditional_logic_2`: Iterates through a buffer and writes a value to a destination offset based on bitwise checks of the source data.
- `transform_buffer_with_conditional_logic`: Iterates through a source buffer and updates a destination buffer with a value derived from bitwise operations and conditional checks on the source data.
- `map_buffer_bit_transform`: Iterates through a memory buffer to calculate and store new byte values into a destination offset based on bitwise checks of the source data.
- `transform_buffer_elements`: Iterates through a memory buffer to calculate and store new byte values into a destination offset based on bitwise checks of the source data.
- `update_buffer_values_2`: Updates a destination buffer with a calculated value derived from bitwise operations on a source buffer.
- `update_buffer_values`: Updates a byte in a buffer with a value derived from a bitwise operation on other buffer contents, repeating until a counter reaches zero.
- `expand_and_accumulate_data_stream_3`: Iterates through a source table to expand or modify a destination buffer by reading or accumulating values.
- `expand_and_accumulate_data_stream_2`: Expands or modifies a data stream by reading from a source table and writing to a destination buffer, with logic to handle bit-masking or bit-shifting certain values into a destination stream.
- `expand_and_accumulate_data_stream`: This routine expands or modifies a data stream by reading from a source table and writing to a destination buffer.
- `expand_bitstream_control_byte`: Reads a control byte to determine a bit-mask or expansion factor, then writes a transformed byte to a destination buffer.
- `expand_bitstream_data_2`: Decodes a variable-length bitstream or compressed-style data stream by reading a control byte, calculating a scaling or expansion factor, and writing the resulting bytes to a destination buffer.
- `expand_and_copy_data_alt`: Expands or modifies a data stream by copying long words and applying a transformation or scaling factor to certain bytes using a different entry point.
- `expand_and_copy_data`: Expands or modifies a data stream by copying long words and applying a transformation or scaling factor to certain bytes.
- `expand_nibbles_from_table_2`: Decodes or expands data from a source pointer to a destination, using a lookup table or bitwise masking to determine which bytes or words to write.
- `expand_nibbles_from_table`: Decodes a bitstream by writing a byte to a destination, then potentially expanding or copying further data using a lookup table.
- `expand_and_copy_nibbles`: Iterates through a source buffer, using a lookup table to expand or transform low-nibble or high-nibble values into a destination buffer.
- `copy_and_transform_data_alt`: Copies long words from a source to a destination while applying a conditional transformation or expansion based on a lookup table using a different entry point.
- `copy_and_transform_data`: Copies long words from a source to a destination while applying a conditional transformation or expansion based on a lookup table.
- `copy_long_words_2`: Copies a specified number of long words from a source table to a destination buffer using a pointer and a counter, optionally adding the source value to the existing destination value.
- `copy_long_words`: Copies a specified number of long words from a source table to a destination buffer using a pointer and a counter.
- `process_data_loop_2`: Iterates through a data structure, calculates an offset difference, and stores a returned value into a memory location or a destination buffer.
- `process_data_loop`: Iterates through a data structure, calculates an offset difference, and stores a returned value into a memory location.
- `process_callback_list_with_bounds_3`: Iterates through a list of pointers and executes a callback routine if a value or a certain condition involving a byte-checked threshold or certain conditions or a counter-based checks are met.
- `process_callback_list_with_bounds_2`: Iterates through a list of pointers and executes a callback routine if a value or a certain condition involving a byte-checked threshold or a bound-check is met.
- `process_callback_list`: Iterates through a list of pointers and executes a callback routine if a byte in a data structure is greater than a certain value.
- `process_callback_with_offset_2`: Iterates through a list of pointers, updating a value and executing a callback routine if a certain condition is met.
- `process_callback_with_offset`: Iterates through a list of pointers and executes a callback routine if a byte, calculated using an offset and a masked value, is greater than a certain byte.
- `process_callback_list_with_bounds`: Iterates through a list of pointers, checking a value against certain bounds or thresholds and executing a callback routine if a condition is met.
- `execute_callback_chain`: Iterates through a list of pointers, executing a routine with certain parameters and updating a counter or a destination pointer.
- `execute_callback_chain_2`: Iterates through a list of pointers and executes a callback routine if certain conditions or counter-based checks are met.
- `execute_callback_with_modulo`: Calculates a value using a modulo operation against a global variable and executes a passed function pointer with specific arguments.
- `calculate_scaled_minimum`: Calculates a weighted average or minimum of three bytes and scales the result if a certain bit in a global variable is set.
- `find_and_scale_minimum_byte`: finds the minimum value among three consecutive bytes and, if a specific bit in a global variable is set, scales that minimum by a factor.
- `swap_buffer_offsets`: Swaps two long words and two words located at specific offsets within a provided memory block.
- `sync_buffer_offsets`: Compares two short words at specific offsets in a block and updates one if they are not equal.
- `calculate_offset_difference`: Checks if two short words at specific offsets are equal, and if so, stores the difference between two other values into one of those offsets.
- `swap_long_words`: Swaps two consecutive long words at specific offsets within a provided memory block.
- `decrement_and_wrap_counter`: Subtracts a value from a counter and, if the result is zero or negative, increments a parameter and resets the counter using a third value.
- `populate_buffer_with_clamped_values`: Iterates through a memory range to fill a destination buffer with values obtained from another routine, applying a floor or a maximum clamp or a decrementing/incrementing offset depending on certain flags in a control structure.
- `iterate_and_fill_buffer`: Iterates through a memory range, calling another routine with a fixed step, and potentially writes a clamped value to a buffer.
- `iterate_with_fixed_step_3`: Iterates through a memory range or a set of parameters by decrementing a counter and incrementing a pointer or value by a fixed offset.
- `decrement_and_call_routine`: Repeatedly calls another routine while decrementing a value until it no longer exceeds a certain threshold.
- `iterate_and_update_device_status`: Iterates through a range of values while conditionally updating a device status or flag in a structure.
- `update_buffer_with_offset_decrement`: Decrements a value or pointer-based counter and updates a memory location with a returned value from a sub-routine.
- `process_buffer_with_conditional_fill`: Iterates through a memory buffer, performing conditional checks and potentially filling a range of memory with a constant value.
- `update_buffer_with_conditional_fill_2_2`: Iterates through a memory buffer, performing conditional checks and potentially filling a range of memory with a constant value.
- `update_buffer_with_conditional_fill_2`: Iterates through a memory buffer, performing conditional checks and potentially filling a range of memory with a constant value.
- `update_buffer_with_conditional_fill`: Iterates through a memory buffer to perform conditional checks and updates a counter, while potentially filling a range of memory with a constant value.
- `iterate_and_modify_buffer_contents`: Iterates through a memory buffer, performing conditional checks and updating a counter, while potentially filling a range of memory with a constant value.
- `process_buffer_and_clear_memory`: Iterates through a memory buffer to perform conditional checks and updates a counter, while potentially filling a range of memory with a constant value and performing pointer-based swaps or clearing operations.
- `store_byte_sequence_2`: Stores a sequence of bytes into a destination buffer, either using a fixed or a variable-length stride or pattern.
- `store_byte_sequence`: Iterates a specified number of times to store bytes from a source into a destination buffer at four-byte increments.
- `process_memory_block_with_padding`: Iteratively traverses a memory block, conditionally filling or updating memory or node structures with constant values or incremented counters.
- `increment_node_counter_and_fill_memory`: Increments a counter or decrements a size field within a node structure while conditionally filling a memory block with a constant value.
- `update_node_and_fill_memory`: Updates a node's pointer and counter while conditionally filling a memory block with a constant value.
- `update_buffer_and_decrement_counters`: Decrements a counter and updates a short value in memory while checking certain flags or conditions.
- `initialize_memory_block_with_padding`: Initializes a memory block by setting a range of words to a maximum value and decrementing a counter, while performing conditional checks on a pointer-based structure and updating a counter or pointer-related value.
- `update_buffer_and_counters`: Iterates through a memory block or linked list, updating counters and certain memory values based on flags or pointer-based conditions.
- `update_linked_list_counters`: Iterates through a linked list or pointer chain, performing conditional checks on structure flags and updating counters or pointers.
- `process_linked_list_and_fill_memory`: Iterates through a linked list or pointer chain, performing conditional checks on structure flags and updating counters or pointers.
- `update_memory_block_and_counters_2`: Decrements a counter and updates a memory block by filling a range with a constant value, while simultaneously decrementing or incrementing a counter or pointer-related value.
- `update_memory_block_and_counters`: Decrements a counter and updates a memory block by filling a range with a constant value, while simultaneously decrementing or incrementing secondary counters and checking certain flag bits or status values.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `update_printer_string_buffer`: Checks a string table or pointer, and if a non-empty string is found, copies that string into a global buffer while updating a pointer or offset in a structure.
- `copy_string_and_update_table`: Copies a null-terminated string to a global buffer and updates a table or structure with a new pointer or offset.
- `return_error_constant`: Returns a constant error value representing a failed or invalid state.
- `update_string_and_table`: Copies a null-terminated string to a global location and updates a pointer or value within a structure or table.
- `patch_function_table_3`: Updates a function table or a pointer and copies a string from a source pointer to a destination pointer.
- `process_printer_string_and_node`: Checks a pointer in a node, recursively processes a linked list or string, and updates node metadata or calls a function via a pointer.
- `execute_subroutines_and_return_error`: Executes two subroutines and then returns a constant error value.
- `execute_callback_and_recurse`: Checks a global value, calls a function pointer from a unable-to-determine table, and then calls itself with a value from a pointer.
- `compare_strings_case_insensitive_2`: Compares two strings while ignoring the case of certain characters and returns a status code based on whether the strings match or reach an end.
- `return_conditional_one`: Returns one if a register contains zero, otherwise returns one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `fetch_routine_pointer`: loads a function pointer from a global table and executes it.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_indirect_function`: loads a function pointer from a global table and executes it.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `dereference_indirect_pointer`: loads a pointer from a global table and executes the code located at that address.
- `rotate_three_long_words`: Performs a circular rotation of three long words stored at specific offsets within a memory block.
- `patch_buffer_with_pattern_matching`: Iterates through a memory block to detect specific duplicate or pattern-based values and recursively calls a helper routine to resize or shift data.
- `resize_and_copy_buffer`: Updates a memory buffer's size and copies a block of data using a pointer-based loop while updating a counter and calculating a new offset or size or certain values.
- `reallocate_and_copy_buffer`: Updates a buffer's size and copies a block of memory using a pointer-based loop while simultaneously updating a counter and calculating new size or offset values.
- `reallocate_and_patch_buffer`: Checks for specific pattern matches or duplicate values within a memory block and, if detected, recursively calls itself to resize or shift the buffer, followed by a block-copying loop that updates a counter and copies data from a source pointer to a destination pointer.
- `copy_and_recalculate_buffer_size_2`: Copies a block of memory using a pointer-based loop while simultaneously updating a counter and calculating new size or offset values.
- `copy_and_recalculate_buffer_size`: Copies a block of memory using a pointer-based loop while simultaneously updating a counter and calculating new size or offset values.
- `transform_and_accumulate_buffer_values`: Iterates through a buffer to apply a non-linear scaling or clamping logic to certain byte values, then accumulates the resulting or intermediate values into a target buffer.
- `update_scaled_buffer_values`: Iteratively calculates scaled or clamped byte values based on a comparison with a threshold and writes them into a buffer.
- `update_buffer_with_scaled_values_2`: Iteratively calculates scaled or clamped byte values based on a comparison with a threshold and writes them into a buffer.
- `update_buffer_with_scaled_values`: Iteratively calculates scaled or clamped byte values based on a comparison with a threshold and writes them into a buffer.
- `populate_buffer_with_pattern`: Iterates through a buffer to write a calculated sequence of bytes based on a pattern-generating logic.
- `initialize_buffer_pattern`: Writes a repeating pattern of calculated byte values into a buffer and updates a counter or pointer-related value based on a loop condition.
- `swap_and_clear_pointer`: Swaps two long words at specific offsets within a memory block and then clears the value at the new second offset.
- `return_null_value`: The routine returns a zero or null value.
- `return_max_int`: Returns a constant value of all ones.

### EpsonQ

- `return_zero_4`: The routine returns a zero value to the caller.
- `execute_nested_function_calls`: Performs a series of nested function calls using pointers or offsets from a global variable and returns a failure code if any call returns zero.
- `invoke_indirect_function_3`: Retrieves a function pointer from a global table using a fixed offset and executes it twice, passing the result of the first call as an argument to the second.
- `execute_repeated_function_calls`: Calls a function pointer twice with the value held in an unable-to-track register.
- `return_zero_5`: the routine returns a zero value without performing any operations.
- `return_zero_6`: the routine returns a zero or null value.
- `perform_bit_rotation_and_modulo`: Performs a bit-shifting loop that updates a value and checks for a carry, or calculates a modulo-based result involving a division.
- `execute_shift_loop`: Executes a loop that performs bitwise shifts and conditional subtractions on a value.
- `multiply_and_rotate_words`: calculates a sum of products involving the upper and lower halves of two registers and returns a value containing a bit-rotated version of one register.
- `invoke_pointer_chain_helper`: Calls a single helper routine and returns, performing no other operations.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `update_bitmask_from_table_4`: Iterates through a table of values and, if a value or its comparison against a threshold meets certain criteria, sets a specific bit in a destination bitmask buffer.
- `update_bitmask_from_table_3`: Iterates through a list of pointers and a counter, setting bits in a destination buffer if a value from a table is less than a certain threshold.
- `update_bitmask_from_pointers`: Iterates through a list of pointers and sets a bit in a destination buffer if a value from a table exceeds a certain threshold.
- `update_bitmask_from_pointer_list`: Iterates through a list of pointers and, if a value from a table exceeds a certain threshold, sets a bit in a destination buffer.
- `update_bitmask_from_table`: Iterates through a list of pointers and a counter, comparing a value from a table against a character and setting a bit in a destination buffer if the condition is met.
- `compute_modular_multiplication`: calculates a value using a sequence of bit-shifts and subtractions or a direct modulo-based division and remainder calculation.
- `generate_string_buffer_and_call_function`: This routine calculates buffer offsets and populates a memory area with a sequence of bytes, including control characters and values from a global variable, before calling a function pointer with the prepared buffer.
- `return_from_subroutine_3`: The routine executes a single return instruction to exit the current function.
- `process_printer_escape_sequences`: Updates a printer buffer with escape sequences and control characters based on a state-machine or command-driven logic.
- `generate_printer_escape_sequence`: Copies a sequence of bytes from a global table into a buffer and appends specific control or escape sequences based on the current printer state.
- `update_printer_escape_sequence_2_3`: Updates a buffer with specific control or escape sequences based on the current state of a printer-related byte.
- `update_printer_escape_sequence_2`: Updates a buffer with specific control or escape sequences based on the current state of a printer-related byte.
- `update_printer_escape_sequence_2_2`: Updates a buffer with specific control or escape sequences based on the current state of a printer-related byte.
- `update_printer_escape_sequence`: Updates a buffer with specific control or escape sequences based on the current state of a printer-related byte.
- `copy_byte_sequence_3`: Decrements a byte from a source, copies a second byte, and then copies a sequence of eight bytes to a destination pointer.
- `copy_byte_sequence`: Copies a fixed number of bytes from a source pointer to a destination pointer.
- `update_printer_state`: Checks a byte at a pointer against several constants and writes specific bytes or escape sequences to a destination buffer, or clears a byte and writes a constant if a certain condition is met.
- `update_memory_on_condition`: If a byte at a pointer is non-zero, it clears that byte and writes a constant value to a different memory location, otherwise it returns a constant value of all ones.
- `return_constant_value_3`: The routine returns a constant value of all ones.
- `update_printer_buffer`: Checks a byte at a pointer against several constants and writes specific bytes or escape sequences to a destination buffer.
- `reset_memory_on_condition`: If a byte at a pointer is less than one, it returns a value of all bits set to one; otherwise, it clears that byte and writes a constant value to another memory location.
- `return_all_ones_8`: returns a constant value consisting of all bits set to one.
- `emit_printer_escape_sequence_2`: Checks a byte against several constants to write specific bytes or escape sequences to a destination buffer, including a conditional write of a byte from a global memory location.
- `write_printer_command_sequence`: Writes a sequence of two constant bytes and one byte from a global memory location to a destination pointer.
- `emit_printer_escape_sequence`: Checks a value in a table and writes an escape sequence or a single byte to a destination.
- `write_conditional_printer_command`: Checks a value in a table and writes a sequence of bytes or a single byte to a destination or a global address based on that value.
- `write_byte_on_condition`: If a value pointed to by a register is zero, it stores a byte into a memory location offset from another pointer.
- `recursively_update_pointer`: Checks a value in a global variable to determine a byte, stores that byte at an offset from a pointer, and then recursively calls itself while updating a pointer with a returned value and a value from a table.
- `update_pointer_and_store_value`: Stores a byte at an offset from a pointer, then recursively calls itself and stores a returned value at a different offset from the original pointer.
- `call_indirect_from_table_10`: Calls a routine located at an offset from a global pointer.
- `call_indirect_with_offset`: Calls a routine located at a different offset from a global pointer.

### CBM_MPS1000

- `return_zero_7`: The routine returns a zero value and performs no other operations.
- `execute_nested_function_pointers`: Calls a function pointer and, if it returns non-zero, calls a second function pointer and then a third, potentially calling a cleanup routine.
- `execute_function_pointer`: Calls a function pointer and then calls the same function pointer again with an argument, returning a constant value.
- `execute_triple_indirect_call`: Calls a routine located at an offset from a global pointer twice with an address register argument and returns zero.
- `return_zero_empty`: An empty routine that returns zero.
- `return_zero_8`: The routine is empty and always returns a zero value to the caller.
- `recalculate_buffer_bounds`: A recursive routine that calculates or updates memory offsets and buffer boundaries based on a type-check or flag, potentially calling a string-writing function and a function pointer from a global table.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `write_string_to_buffer`: Searches for a null-terminated string, writes a header containing a global variable and a length, and copies the string or a modified version to a destination buffer.
- `write_sequence_to_memory`: Searches for a null-terminated string, then writes a sequence of bytes including a global variable and a counter to a memory location.
- `write_four_bytes`: Checks a global variable against a constant and writes a sequence of four bytes to a memory location calculated from two pointers.
- `clear_memory_block_3`: Fills a block of memory with zeros, using either word-sized or byte-sized writes depending on the alignment of the pointer.
- `clear_memory_block`: Fills a block of memory with zeros using a counter and an incrementing pointer.
- `perform_multi_word_division`: Performs a complex division-like operation or bit-shifting loop on a 32-bit value using a secondary routine.
- `process_bit_shifts`: Performs a series of bitwise shifts and conditional subtractions using a loop counter.
- `calculate_product_sum`: Calculates a value based on partial products of two registers and returns a concatenated result.
- `invoke_pointer_chain_helper_2`: Calls a routine that updates a pointer and returns, performing no other operations.
- `call_and_negate_on_flag_2`: Calls a helper routine and returns its first result, or the negation of that result if a certain register is non-zero.
- `process_printer_command_2`: Updates a printer buffer with escape sequences and control characters based on a command type or printer configuration.
- `format_string_with_control_codes`: Copies a source string into a destination buffer while prepending or injecting specific escape sequences and control bytes based on certain flag checks.
- `copy_and_format_string_2`: Copies a null-terminated string or buffer into a destination and appends a sequence of control or escape bytes.
- `format_printer_buffer`: Copies a string or buffer into a destination and appends specific escape sequences or control bytes based on a configuration or command byte.
- `handle_printer_escape_sequence`: Checks a command byte against several constants to write specific escape sequences or control bytes to a destination buffer.
- `process_printer_command`: Checks a byte at a pointer against several constants and writes specific bytes or escape sequences to a destination or a stack location.
- `copy_bytes_to_destination`: Copies a fixed number of bytes from one memory location to another using an incrementing pointer.
- `set_printer_command`: Checks a byte at a pointer against several constants and writes specific bytes or escape sequences to a destination or a stack location, or sets a byte if a certain condition is met.
- `set_byte_on_condition_4`: Checks if a value at a pointer is negative, and if so, sets that value to zero and writes a constant to another pointer.
- `load_constant_ff`: Loads the constant 255 into d0, moves it into d1, and branches to another routine.
- `set_printer_state`: Checks a byte at a pointer against several constants and writes specific bytes to a destination or a stack location.
- `update_pointer_value_conditionally`: Checks if a byte at a pointer is less than one; if so, returns a value with all bits set, otherwise clears that byte and writes a constant byte to a different pointer.
- `return_all_ones_9`: An empty routine that returns a value consisting of all bits set to one.
- `set_values_based_on_input`: Checks the value at a pointer against several constants and writes specific bytes to a destination or a stack location.
- `set_byte_on_condition`: Checks if a value at a pointer is zero and, if so, writes a constant byte to an offset of another pointer.
- `update_object_properties`: Updates a structure or object with a value or constant based on a conditional check of a global variable, then populates further fields using values retrieved from a table indexed by a shifted parameter.
- `update_structure_from_table`: Updates a structure with values from a table and sets a field to a constant or a value based on a comparison.
- `update_conditional_value`: Checks a value at an offset from a pointer and writes a constant to another offset in a structure, then returns a byte from a global table.
- `set_constant_and_get_byte`: Stores a constant value into a memory location offset from a pointer and returns a byte from a table.
- `update_bitmask_from_table_5`: Iterates through a list of pointers to a table of values, checking if those values exceed a certain threshold, and if so, sets a bit in a destination buffer using a calculated bit-shift or bitmask.
- `set_bits_with_shift`: Iterates through a buffer and sets a bit in a destination buffer based on a bit-shift or bitmask, using a pointer-based list of pointers.
- `set_bits_in_buffer`: Iterates through a buffer and sets a specific bit in each byte using a bitmask or bit-shift operation.
- `set_bits_from_comparison`: Compares values from a table against a source and sets specific bits in a destination buffer based on the comparison result.
- `call_indirect_function_2`: Calls a routine located at an address stored in a memory location offset from a global pointer.
- `call_indirect_function_offset`: Calls a routine located at a different memory location offset from a global pointer.

### HP_LaserJet

- `return_zero_9`: The routine returns a zero value without performing any operations.
- `execute_function_pointers_with_cleanup`: Executes a sequence of function pointers and calls a cleanup routine if certain return values are non-zero.
- `execute_chained_function_pointers`: Calls a function pointer retrieved from a table and then calls the same function again passing the return value of the first call as an argument.
- `call_function_thrice`: Calls a routine located at a memory location offset from a global pointer three times.
- `return_zero_10`: The routine returns a constant zero value.
- `process_printer_control_string`: Parses a string containing control codes to configure printer-specific settings such as font size, character spacing, or alignment.
- `parse_string_control_codes`: Copies a sequence of bytes from a global table into a destination buffer while interpreting or acting upon specific control characters.
- `process_string_control_codes_2`: Parses a sequence of bytes from a global array and updates a destination buffer while handling control characters that modify a state-tracking or cursor-positioning variable.
- `update_string_buffer_and_state`: Copies a sequence of bytes from a global array into a destination buffer and updates a state-tracking or control character.
- `copy_and_process_bytes`: Copies a fixed number of bytes from a global array into a destination buffer and then calls another routine twice with a pointer-derived index and an incremented offset.
- `process_string_control_codes_9`: Updates a string buffer or pointer based on specific control characters, handling escape sequences, character increments, and copying a fixed number of bytes from a table or a pointer-based source.
- `process_string_control_codes_8`: Updates a string or buffer index or pointer based on a control byte, and copies a fixed number of bytes from a global table to a destination.
- `update_string_cursor_and_buffer`: Updates a string or buffer index or pointer based on a control byte, and copies a fixed number of bytes from a global table to a destination if certain conditions or control codes are met.
- `update_string_state_logic_2`: Updates a state or index or pointer based on a comparison of a control byte and a counter, or copies a fixed number of bytes from a global table to a destination if certain conditions are met.
- `update_string_state_logic`: Checks a value through a pointer to modify it or copy a fixed number of bytes from a global table to a destination.
- `update_string_state`: Checks a value through a pointer to modify it or copy a fixed number of bytes from a global table to a destination, while also decrementing.
- `update_state_and_buffer`: Checks a value through a pointer to modify it or copy a fixed number of bytes from a global table to a destination, while also decrementing a value or updating a pointer-based memory location.
- `update_buffer_on_condition_2`: Checks a value through a pointer and either decrements a value, copies a fixed number of bytes from a global table to a destination, or writes a byte.
- `update_buffer_on_condition`: Checks a value through a pointer and either decrements a value, copies a fixed number of bytes from a global table to a destination, or writes a byte to a memory location offset from a pointer.
- `update_pointer_and_copy_data`: Decrements a value through a pointer and copies a fixed number of bytes from a global table to a destination buffer.
- `decrement_and_copy_data`: Decrements a value through a pointer and copies a fixed number of bytes from a global table to a destination buffer.
- `copy_and_format_numeric_data_2`: copies a fixed number of bytes from a table into a destination buffer and then calls a routine twice to format or calculate numeric output.
- `copy_and_format_numeric_data`: Copies a fixed number of bytes from a global table into a destination buffer and then calls a routine twice to write a digit and a calculated difference or value into a string buffer.
- `copy_conditional_data`: If a byte at a pointer equals a certain value, copies a sequence of bytes from a global array into a destination buffer and calls another routine twice.
- `copy_global_data`: Copies a sequence of bytes from a global array into a destination buffer and calls another routine twice with specific arguments.
- `set_byte_on_condition_2`: If the value pointed to by an address register is zero, stores a byte into a memory location offset from a pointer.
- `format_numeric_string_3`: Writes a digit or a two-digit representation of a byte into a string buffer followed by a character representation of a returned value.
- `format_byte_to_string`: Converts a byte value into a string of up to three ASCII characters.
- `format_digit_to_string`: Writes a digit or a two-digit representation of a byte into a string buffer, followed by a character representation of a returned value.
- `write_digit_characters`: Writes a constant byte and a character representation of a returned byte to a memory buffer or string.
- `clear_global_on_match`: If a parameter matches a specific character, clears a value in a global variable.
- `reset_global_and_call_helpers`: If a global variable is non-zero, calls two different routines and then clears that global variable.
- `transform_register_values`: Performs bitwise shifts and conditional arithmetic on a 32-bit value, or calculates a remainder-based result if a certain condition is met.
- `shift_and_compare_bits`: Iteratively shifts a bit pattern while checking for a carry and comparing the resulting value against a global integer.
- `compute_complex_product`: Calculates a value using a combination of partial products and bitwise shifts.
- `update_pointer_chain`: Updates a pointer or a chain of pointers by storing an incremented address into a target memory location.
- `get_pointer_chain_result`: Calls a routine to setup a pointer chain and returns the value left in a data register.
- `negate_result_on_flag_2`: calls another routine and negates the returned value if the flag or a returned value in an address register is non-zero.
- `negate_result_on_condition`: Calls another routine and returns its result, or returns the negation of that result if a second register-held value is non-zero.
- `update_string_buffer_and_print`: Calculates string lengths and writes ASCII-encoded numeric characters into a buffer, followed by a call to a printing routine.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `set_bitmask_from_pointer_list`: Iterates through a list of pointers and sets a bit in a bitmask buffer if a value from a table exceeds a threshold, using a calculated offset.
- `set_bit_mask_buffer`: Iterates through a list of words and sets a bit in a bitmask buffer based on a calculated offset.
- `set_bit_in_buffer`: Iterates through a pointer-based list and sets a bit in a bitmask buffer if a certain condition is met.
- `call_indirect_function_3`: Loads a function pointer from a global offset and calls it.
- `call_indirect_function_offset_2`: Loads a function pointer from a different global offset and calls it.
- `update_table_values`: Reads a value from a table, scales it, and stores the result and a byte or word into a global structure.

### EpsonX[CBM_MPS-1250]

- `return_zero_11`: The routine is empty and simply returns a zero value to the caller.
- `traverse_linked_list_nodes`: Iteratively calls functions via pointers stored in a structure and returns a failure code if any call returns zero.
- `invoke_indirect_function_4`: Retrieves a function pointer from a table using a fixed offset and executes it twice, passing the result of the first call as an argument to the second.
- `execute_indirect_calls_3`: Calls a routine or function pointer or a pointer to a function pointer multiple times.
- `return_zero_12`: The routine returns a zero-valued integer or null pointer.
- `return_zero_13`: The routine returns a zero or null value without performing any operations.
- `perform_multiplication_loop`: Performs a loop-based multiplication or bit-shifting routine that modifies a 32-bit value and handles overflow or modulo-like logic.
- `shift_and_compare_bits_2`: Performs a bitwise left shift and conditional subtraction or addition based on a comparison with a global variable.
- `calculate_mixed_product`: Calculates a complex product of register halves and returns a concatenated result.
- `invoke_pointer_chain_helper_3`: Calls a single helper routine and returns without performing any further operations.
- `negate_result_on_flag_3`: Calls a helper routine and returns its result, or returns the negation of that result if a second register-held value is non-zero.
- `apply_bitmask_from_table_2`: Iterates through a table of values and, if a value meets certain comparison criteria, sets a specific bit in a destination buffer.
- `apply_bitmask_from_table`: Iterates through a table of values and, if a value meets certain comparison criteria, sets a specific bit in a destination buffer.
- `set_bits_from_table`: Iterates through a table of values and sets specific bits in a destination buffer if certain comparison conditions or range checks are met.
- `set_bits_from_table_2`: Iterates through a table and sets a bit in a destination buffer if a value in a source structure is less than a value in a secondary table or certain bitmask conditions are met.
- `set_bits_with_range`: Iterates through a table and sets bits in a destination buffer based on a comparison against a value in a pointer-based structure.
- `set_bits_from_table_comparison`: Iterates through a list of source values and compares them against a secondary table, setting a bit in a destination buffer if a value in the table is less than the source value.
- `update_attribute_metadata`: Checks a value in a global structure to set a byte, then updates a secondary global structure with values or a return value from another routine.
- `update_buffer_geometry`: Calculates and stores new buffer or vertex size parameters based on a type-check or flag, then recursively updates or traverses a linked list or buffer structure.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `format_string_with_escapes`: Iterates through a buffer to find a null terminator and writes a carriage return followed by an escape sequence containing a value from a table.
- `format_string_with_escape_codes`: Iterates through a buffer to find a null terminator, then writes a carriage return and an escape sequence containing a value from a table and a global variable to a destination buffer.
- `write_string_with_escape_codes`: Iterates through a string and writes carriage returns and escape-sequence-based control characters or data to a destination buffer.
- `format_string_with_escape_codes_2`: Iterates through a buffer to find a null terminator, then writes a carriage return and an escape sequence containing a value from a table and other data.
- `clear_memory_block_4`: Clears a block of memory by storing zero or a null byte at a pointer and incrementing the pointer.
- `zero_memory_block`: Clears a block of memory by storing a zero at a pointer and decrementing a counter while incrementing the pointer.
- `process_printer_escape_sequences_4`: Parses a stream of control characters or escape sequences to configure printer-specific settings such as character width, font or alignment, and buffer state.
- `parse_string_control_sequences`: Parses a buffer of characters to handle escape sequences and control characters, updating a string buffer and a state or index variable.
- `process_string_control_escape_2`: Processes escape sequences or control characters to update a string buffer and a state or index variable.
- `process_string_control_escape`: Updates a control byte and an associated index or state, or writes an escape sequence to a buffer based on conditional checks of a control character.
- `update_string_control_state_2`: Updates a control byte and an associated index or state or writes an escape sequence to a buffer based on conditional checks of a control character.
- `update_string_state_logic_5`: Updates a string or control byte and its associated index or state based on a series of conditional checks.
- `copy_byte_sequence_4`: Modifies a byte at an offset in a destination buffer and copies a sequence of eight bytes from a source pointer to a destination pointer.
- `copy_byte_sequence_2`: Copies a sequence of bytes from a source pointer to a destination pointer.
- `update_string_state_logic_3`: updates a state-dependent string or control sequence by checking a state variable and writing specific byte sequences or single bytes to a destination buffer.
- `update_buffer_if_negative`: Checks if a byte at a pointer is negative, and if so, resets that byte to zero, writes a constant value to a different pointer, and returns a constant.
- `return_constant_value_4`: Returns a constant 32-bit value containing all bits set.
- `handle_control_codes`: Checks a value through a pointer and, depending on its value, either resets a pointer, modifies a byte, or writes a sequence of constant bytes and a byte from a memory location.
- `write_byte_if_zero_2`: Checks if a byte at a pointer is zero, then either sets a 32-bit register to all ones or clears a byte and sets a 16-bit value to a constant.
- `return_all_ones_10`: Returns a 32-bit value containing all bits set.
- `write_conditional_sequence_3`: Writes a sequence of constant bytes and a byte from a memory location or a value from a stack-based pointer if certain conditions or pointer.
- `write_conditional_sequence_2`: Writes a sequence of constant bytes and a byte from a memory location or a value from a stack-based pointer if certain conditions or pointer values are met.
- `write_conditional_sequence`: Checks a value at a pointer and, depending on its value, either modifies a byte or writes a sequence of constant bytes and a byte from a memory location to a destination pointer.
- `write_byte_sequence`: Writes a sequence of constant bytes and one byte from a memory location offset to a destination pointer.
- `write_byte_if_zero`: Checks if the value pointed to by a register is zero, and if so, writes a constant byte to a memory location offset from another pointer.
- `update_structure_from_table_2`: Updates a structure with a value from a table and sets a field to a constant or a value based on a comparison.
- `update_structure_values`: Updates a structure with a constant, calls another routine, and then sets a structure member to a value from a table or a constant based on a comparison.
- `update_config_value`: Checks a value at an offset from a pointer and writes a constant to another offset from that same pointer.
- `update_offset_and_fetch_table_byte`: Stores a constant byte into a memory location offset from a pointer and returns a byte from a table using an index in a register.
- `execute_indirect_function_pointer_34`: Loads a function pointer from a memory location offset from a global pointer and executes it.
- `execute_indirect_function_pointer_35`: loads a function pointer from a table and executes it.

### Okimate_20

- `return_zero_14`: The routine is empty and returns a zero value to the caller.
- `execute_nested_function_pointers_2`: Calls a function pointer retrieved from a global variable or a parameter, and if the return value is non-zero, calls it again or calls a different function pointer from a structure.
- `execute_nested_function_pointers_3`: Calls a function pointer and, if it returns non-zero, calls a second function pointer.
- `execute_triple_indirect_call_2`: Calls a routine located at an offset from a global pointer twice with an orfset register value.
- `return_zero_15`: The routine is empty and returns a zero value.
- `return_zero_16`: The routine is empty and returns a constant zero value.
- `reduce_long_integer_modulo`: Calculates the remainder of a 32-bit or 64-bit division or performs a conditional bit-shift-based reduction of a large value against a limit.
- `shift_and_check_limit`: Performs bitwise shifts and checks a value against a limit, potentially updating a register or a temporary variable.
- `compute_complex_product_2`: Calculates a value based on a combination of products and bitwise shifts of two input registers and returns the result.
- `invoke_helper_routine`: Calls a single helper routine and returns.
- `get_marker_value`: calls a routine to scan for a marker and returns the resulting value left in a data register.
- `negate_scan_result_on_condition`: Calls a routine to scan for a marker and returns the first result or its negation if a secondary register-based flag is non-zero.
- `generate_string_buffer`: Constructs a sequence of escape-sequence-based string commands in a temporary buffer, involving string concatenation and conditional checks on a global state or flag.
- `return_from_subroutine_6`: Performs a return from the current subroutine.
- `handle_escape_sequences`: Updates a pointer-based value or writes escape sequences to a destination buffer based on the value at a pointer, and copies a range of bytes from a global table to a destination buffer.
- `copy_and_append_escape_sequences`: Copies a range of bytes from a global table to a destination buffer and appends specific escape sequences or bytes based on certain values in a structure.
- `write_escape_sequences`: Checks a value at an offset from a pointer and writes a sequence of bytes, including escape characters, to a destination buffer.
- `handle_escape_sequence_state`: Updates a byte or a pointer-based value or writes escape sequences to a destination buffer based on the value at a pointer.
- `update_escape_sequence_state`: Checks a value through a pointer against several constants and updates a byte or a pointer-based value.
- `set_buffer_on_negative_value`: Checks if a byte at a pointer is negative, and if so, clears that byte and sets a value in a different pointer-based location to a constant.
- `return_constant_value_5`: Returns a constant 32-bit value representing all bits set.
- `update_state_and_value`: Checks a value through a pointer against several constants and updates a target byte or a secondary pointer-based value accordingly.
- `update_value_from_type_2`: Checks if a byte at a pointer is less than one, then either sets a value to a constant or clears a byte and writes a constant to a pointer.
- `return_all_bits_set_3`: Returns a constant value where all bits are set.
- `update_value_from_type`: Compares a value from a pointer against several constants and writes a new byte or a constant to a destination or a stack location.
- `update_masked_table_on_decrement_2`: Iterates through a list of pointers and a secondary list of counters, checking if a value from a lookup table or a decremented counter is less than a certain threshold, and if so, sets a bit in a target table using a bitwise OR with a masked value.
- `update_masked_table_on_decrement`: Iterates through a list of pointers, decrementing a counter-value stored at each pointer and setting a bit in a target table if a masked pointer or a bit-inverted value meets a certain condition.
- `process_list_with_conditional_bit_set`: Iterates through a list, decrementing counters and updating a byte in a table if a certain condition involving a masked pointer or value is met.
- `execute_indirect_function_pointer_36`: Loads a function pointer from a memory location offset from a global pointer and executes it.
- `execute_indirect_function_pointer_37`: loads a function pointer from a memory location offset by a constant and executes it.

### HP_DeskJet

- `return_zero_17`: The routine returns a zero or a null pointer value.
- `verify_nested_function_calls`: Checks if a function pointer, retrieved via an offset from a global variable, returns a non-zero value, repeating this check up to three times.
- `invoke_nested_function_pointers`: Retrieves a function pointer from a table and executes it, then executes the same function pointer again passing the return value as an argument.
- `execute_repeated_indirect_call`: Calls a routine located at an offset from a global pointer twice, passing a register as an argument.
- `return_null_value_2`: The routine returns a zero or null value to the caller.
- `process_string_control_codes_5`: Parses a sequence of bytes from a global table and updates a destination buffer while interpreting or acting upon specific control characters.
- `copy_and_parse_string_control_codes`: Copies a sequence of bytes from a global table into a destination buffer and parses control characters to modify string state, such as adjusting a character offset or updating a pointer-based buffer or length.
- `process_string_control_codes_3`: Parses a sequence of bytes from a global table and updates a destination buffer, while checking for and acting upon specific control characters or escape sequences.
- `update_string_buffer_state`: Updates a string buffer or state machine by copying bytes from a global array and modifying characters or control codes based on certain conditions.
- `copy_and_process_buffer_2`: Copies a fixed number of bytes from a global array into a destination buffer and then calls another routine twice with or-ed or offset-calculated arguments based on a pointer and an integer.
- `process_string_control_codes_10`: updates string buffer or pointer based on control characters.
- `process_string_control_command`: Updates a string or buffer index or pointer based on a control byte, and copies a fixed number of bytes from a global table to a destination.
- `update_string_cursor_and_buffer_2`: Updates a string or index pointer based on a control byte and a counter, or copies a fixed number of bytes from a global table to a destination buffer.
- `update_string_state_logic_4`: Updates a string or state-related buffer by checking a control byte and a counter, potentially copying bytes from a global table and recursively calling another routine to modify string or state pointers.
- `update_string_control_state`: Updates a control byte and a character or index based on specific value comparisons, or modifies a string buffer and a global table or pointer.
- `update_string_index_and_buffer`: Updates a character or index or copies a small fixed-size block of data from a global table to a destination, while conditionally modifying a control byte or a string pointer based on certain value checks.
- `update_state_and_buffer_2`: Updates a state-tracking byte and conditionally copies a fixed-size block of data from a global table to a destination, while also performing conditional byte writes and recursive calls to another routine.
- `update_buffer_on_condition_3`: Checks a value through a pointer and either decrements a value, copies a fixed or fixed-size amount of bytes from a global table to a destination, or writes a byte.
- `update_conditional_buffer`: Checks a byte value and either decrements a value through a pointer, copies a fixed number of bytes from a global table to a destination, or stores a byte into a memory location offset from a pointer.
- `update_pointer_and_copy_table`: Checks a value through a pointer and either decrements a byte or replaces it, then copies a fixed number of bytes from a global table to a destination buffer.
- `decrement_and_copy_table_data`: Decrements a value pointed to by a pointer in a structure and copies a fixed number of bytes from a global table to a destination buffer.
- `copy_numeric_data`: Copies a fixed number of bytes from a global table into a destination buffer and calls a routine to format numeric data.
- `copy_and_format_string_v3`: Copies a sequence or sequence of bytes from a global array into a destination buffer and calls another routine twice with specific arguments.
- `update_printer_buffer_on_condition`: Checks a byte at a pointer, and if it matches a specific value, copies a sequence of bytes from a global table into a destination buffer before calling another routine twice with different arguments.
- `copy_small_data_block`: copies a fixed number of bytes from a global table to a destination memory location.
- `set_byte_on_condition_3`: Checks if a value at a pointer is zero and, if so, writes a byte to an offset of another pointer.
- `encode_numeric_prefix`: Calculates a leading digit and a secondary digit or character to store a prefix in a string buffer.
- `format_digit_to_string_2`: Writes a digit or a two-digit representation of a byte into a string buffer.
- `format_digit_string`: Writes a digit character and a second digit character or a character representation of a byte to a memory buffer.
- `write_digit_to_buffer`: Writes a fixed character and then writes a digit converted from a returned value to a buffer using a pointer-based offset.
- `clear_global_on_match_2`: Checks if a parameter equals a specific character and, if so, clears a value at an offset of a global variable.
- `execute_and_clear_flag`: If a global variable offset is non-zero, calls two different functions via pointers and then clears that global variable offset.
- `transform_value`: Performs bitwise shifts and conditional subtractions on a 32-bit value, or calculates a remainder-based result.
- `shift_and_compare_bits_3`: Performs bitwise shifts and comparisons in a loop, potentially modifying a value if it falls below a certain threshold.
- `calculate_complex_product`: Calculates a value using a combination of partial products and bitwise shifts, then returns it.
- `call_helper_and_return`: Calls another routine and returns, with no other work.
- `get_pointer_chain_result_2`: calls a routine to setup a pointer chain and returns the value left in a data register.
- `negate_result_on_flag_4`: Calls a helper routine and negates the returned value if the helper leaves a non-zero value in an address register.
- `negate_result_on_flag_5`: Calls a helper routine and returns its result, or returns the negation of that result if a second register-based flag or value is non-zero.
- `update_string_buffer_and_print_2`: Calculates string lengths and writes ASCII-encoded numeric characters or escape sequences into a buffer, followed by a call to a printing or output routine.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `set_conditional_bitmask`: Iterates through a list of pointers and checks a condition involving a character or a bitmask, then sets a bit in a target buffer.
- `mark_range_in_bitmask`: Iterates through a list of pointer-based structures to check a comparison value, and if a condition is met, sets a contiguous range of bits in a bitmask buffer.
- `update_bitmask_from_table_2`: Iterates through a table and, based on a comparison, sets a bit in a bitmask located at a calculated offset.
- `call_indirect_from_table_11`: Calls a routine whose address is loaded from a table offset by a constant.
- `call_indirect_from_table_12`: Calls a routine whose address is loaded from a table offset by a different constant.
- `update_global_data`: Calculates several values based on an input, then stores or shifts these values into various global or memory locations.

### A-Train (1992)(Ocean)(De)(Disk 1 of 2).adf:Installer

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### icon.library

- `return_negative_one`: Library code that returns a value of negative one.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_linked_list_node_2`: Library code that modifies a linked list node by updating pointers and potentially setting a flag bit.
- `set_bit_in_byte`: Library code that sets the third bit of a byte at an offset from an address held in a register.
- `return_zero_18`: Library code that returns zero.
- `copy_and_update_pointers`: Library code that copies two 32-bit values from an address derived from a register into two global variables, calls another routine, and conditionally updates a global pointer with a new address or a return value.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `return_empty_routine_6`: This library routine is empty.
- `return_from_routine`: This routine performs an unconditional jump or return to a previously saved or or target address, effectively acting as a way to exit a routine or jump to a new instruction pointer.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_with_increment`: This library routine calls another routine while incrementing a parameter and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `update_icon_attributes_wrapper`: Library code that calls a routine to update icon attributes.
- `perform_incremental_recursion_2`: This library routine executes a recursive call with an incremented parameter until a stopping condition is met.
- `call_other_routine`: This library code calls another routine and then returns.
- `compare_list_element_attribute`: This library routine checks a list for an element by calculating a difference and verifying a specific condition.
- `call_parse_string_to_integer`: Saves two registers to the stack, calls parse_string_to_integer_2, restores the registers, and returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `traverse_linked_list`: recursively traverses a linked list and checks for a non-zero return value.
- `copy_icon_node_data`: This library routine copies a block of ten 32-bit words from a source offset to a destination offset and then copies several individual bytes.
- `copy_icon_data_and_link_node`: Library code that copies a block of ten 32-bit words from a source offset to a destination offset and then copies several individual bytes and words to a destination structure.
- `return_zero_stub`: Clears register d0 and restores registers d2 through d4 and a2 from the stack before returning.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_node_attributes`: Recursively updates or initializes pointer-based attributes within an icon structure, including the allocation and patching of memory for child nodes or attribute lists.
- `recursively_update_icon_node_attributes`: This library routine recursively traverses a structure or linked list to update or initialize pointer-based attributes, such as child nodes or metadata, based on conditional checks of existing values and function return codes.
- `traverse_icon_list_3`: Library code that traverses a linked list or structure, updating pointers or global variables with values from routine calls.
- `library_loop_and_set`: Library code that executes a loop calling a routine and updates a pointer, then conditionally sets a global variable or a memory location.
- `return_zero_stub_4`: Calls another routine and returns zero.
- `restore_registers_and_return`: Moves a longword from d4 to d0, restores d2 through d5 and a2 through a5 from the stack, unlocks a6, and returns.
- `traverse_icon_metadata_links`: This routine traverses a linked or null-terminated list of metadata pointers within an icon structure, executing a callback function for each entry.
- `process_icon_metadata_links`: This library routine traverses a series of linked or null-terminated pointers within an icon structure, executing a callback function for each entry.
- `traverse_icon_list`: Library code that traverses a linked list or table of pointers, performing conditional checks or updates to a counter and updating a global variable or register.
- `iterate_icon_table`: Library code that iterates through a table of pointers, calling a callback for each entry and potentially processing a secondary list of pointers.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list_2`: Library code that traverses a linked list, calling a routine on each node and checking against a termination address.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_conditional_library_chain`: Library code that calls a routine and, if it returns a non-zero value, proceeds to call other routines using offsets or or additional parameters.
- `return_zero_always`: Library code that calls another routine and returns a zero value.
- `return_zero_21`: This library routine always returns a zero value or a null pointer.
- `update_library_attribute`: As library code, it decrements a short counter through a pointer and, if certain conditions are met, writes a value to a calculated offset in a structure.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `update_icon_attributes`: This library routine calls other library routines to check or update attributes within a structure, potentially updating a pointer or a value at a specific offset if certain library calls return non-zero.
- `return_void`: Library code that performs no action and returns.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `search_string_for_delimiter_2`: Library code that iterates through a list of pointers to check if any pointed-to string contains an equals sign character.
- `find_string_terminator`: Library code that traverses a pointer-based list until a specific character or a null terminator is encountered.
- `search_string_for_delimiter`: Iterates through a null-terminated list of pointers to check if any pointed-to string contains an equals sign character.
- `search_list_element`: As part of library code, this routine calls a routine to check a list, then enters a loop that calculates a difference and checks if a certain value matches a previously retrieved value.
- `search_list_for_value_2`: Iterates through a list of integers, calculating the difference between a returned value and a pointer, and checks if any element matches a target value or if a null pointer is encountered.
- `evaluate_list_elements`: Library code that iterates through a list and checks if a returned value matches a specific value or if a pointer becomes null.
- `recursively_evaluate_integer_difference`: Library code that recursively calls another routine to calculate or verify a difference between a returned value and a counter, continuing until a zero or a specific condition is met.
- `compare_list_element_to_target`: Library code that checks if the difference between a value retrieved from a list and a pointer matches a specific register value.
- `parse_string_to_integer_2`: Library code that parses a string of numeric characters into an integer value and writes the result to a destination buffer.
- `parse_and_execute_string`: As library code, it parses a string of digits into an integer and uses the result to perform indirect calls or further parsing.
- `parse_numeric_string_5`: As part of library code, this routine parses a string of digits into an integer and, if the result is non-zero, calls other routines to check a pointer against a global variable.
- `parse_numeric_string_3`: Library code that parses a string of digits into an integer and checks if the resulting value falls within a certain range.
- `map_input_range`: Library code that checks if a value is within a specific range and returns an offset or a negative constant.
- `return_all_ones_3`: This library routine returns a 32-bit value where all bits are set to one.
- `execute_indirect_function_92`: loads a function pointer from a global table and executes it.
- `jump_to_offset_address_2`: Library code that jumps to a function unable to be determined from the provided code.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_function_93`: Loads a function pointer from a location relative to a global base and executes it.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `jump_to_offset_address`: Library code that jumps to a function pointer located at a specific offset from a global address.
- `execute_indirect_function_pointer_48`: Loads a function pointer from a memory location and executes it.
- `execute_indirect_function_91`: Loads a function pointer from a calculated offset relative to a global pointer and executes it.
- `execute_indirect_function_pointer_49`: Loads a function pointer from a table and executes it.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `verify_and_set_attribute`: checks if a library routine returns a non-zero value and, if so, writes a constant value to an offset within the returned pointer and to a memory location pointed to by a parameter.
- `return_zero_19`: This library routine returns zero.
- `compare_and_set_zero`: This library routine calls another routine and sets a return value to zero if the returned value does not match a provided parameter.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `compare_and_reset`: Calls another routine and returns zero if the returned value does not match a provided parameter.
- `return_immediately_3`: This library routine is empty and performs no operations before returning.
- `reduce_large_integer_modulo`: Performs a bit-shifting loop to reduce a 32-bit integer or calculate a remainder relative to a secondary value.
- `perform_bit_shift_loop`: Library code that executes a loop performing bit-shifting and conditional subtraction or addition based on a comparison.
- `calculate_complex_product_2`: This library routine calculates a value based on a combination of bit-shifted and masked input registers and returns a bit-swapped version of an input register.
- `execute_indirect_function_90`: Library code that calls a function pointer retrieved from a global variable offset.
- `store_byte_to_address_5`: This library routine stores a single byte from a register into a memory location pointed to by another register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_char_in_string_2`: Library code that scans a string for a specific character and returns its pointer.
- `return_zero_stub_3`: Library code that returns a constant zero.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_content`: Library code that finds the end of a source string and copies the contents of a second string to that location.
- `compare_string_segments`: Library code that compares two strings up to a certain length and checks for a null terminator.
- `return_constant_one_3`: Library code that always returns the value one.
- `iterate_string_length`: decrements a counter while advancing a pointer through a string until a null terminator or a zeroed counter is reached.
- `execute_string_copy`: Library code that calls a routine to copy a string.
- `copy_string_with_limit_3`: Library code that copies a null-terminated string from a source pointer to a destination pointer while decrementing a counter.
- `do_nothing`: this library routine is empty.
- `initialize_pointer_structure`: This library routine writes a pointer to a memory location, increments that pointer, and then clears a subsequent memory location.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_icon_list_with_predicate`: Iterates through a linked list of structures, performing a conditional check or a nested recursive check on a member of each node, and terminates if a null pointer or a failed condition is encountered.
- `traverse_list_with_null_check`: Library code that traverses a list of pointers, checking if a certain value is zero, and returns a boolean result.
- `return_zero_20`: This library routine always returns a zero value.
- `library_recursive_logic`: Library code that recursively calls other library functions and stores a result in a memory location offset from a pointer.
- `return_max_value`: This library routine returns a value of all ones.
- `traverse_icon_list_2`: This library code traverses a structure, updating a pointer or a global variable with a value from a routine call, and returns a failure code.
- `traverse_icon_list_recursive`: This library code traverses a structure, updating a pointer or a or a global variable with a value from a routine call, and returns a failure code.
- `traverse_linked_list_12`: Library code that traverses a structure, updating a pointer or a global variable with a value from a routine call, and returns a failure code if a null pointer or zero return is encountered.
- `return_max_int_2`: Library code that returns a constant value of all ones.

### info.library

- `return_empty_routine_7`: the routine contains no executable instructions and performs no operations before returning.
- `store_library_state`: This library routine stores incoming register values into a global variable and a table, and writes a pointer through an offset in a structure, then stores that pointer into another global variable.
- `execute_recursive_call_3`: This routine performs a self-referential call to itself, creating an infinite recursion.
- `call_library_helper_2`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_library_subroutine`: This library code pushes a pointer from a6 onto the stack, calls another routine, and then increments the stack pointer.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `reset_library_state_recursive`: This library code calls itself indefinitely, resulting in infinite recursion, before finally calling a routine that resets the library state.
- `clear_flag_and_increment_counter_2`: This library routine clears a specific bit in a byte field of a structure pointed to by its argument and increments a 16-bit counter field within the same structure.
- `process_structure_nodes_2`: Library code that checks a flag in a structure and, if certain conditions are met, calls other library functions using offsets or pointers from the structure.
- `call_library_functions_with_offset_args`: This library routine retrieves a library base from a global variable and calls two library functions using offsets and calculated arguments.
- `decrement_and_recursively_check_flag`: Decrements a short integer located at an offset in a structure and, if the result reaches zero and a specific bit in a flag byte is set, recursively calls itself.
- `return_zero_22`: This library routine returns a zero value.
- `recursively_call_pointers`: The routine checks several global pointers and arguments, then calls itself recursively if a pointer is non-zero.
- `map_library_data_offsets`: Calculates and stores a series of pointers or offsets into a global structure or table, using a memory allocation routine to determine a base address.
- `initialize_info_library_struct`: Initializes a library structure or table by calculating and storing several offsets or pointers derived from a base address.
- `update_library_status`: Library code that calls a function and updates several global variables with the return value or a constant, or calls another routine if a certain condition is met.
- `test_function_return_value`: Library code that calls a function and checks if the return value is non-zero to decide whether to store a result in a global variable and call another routine with a constant.
- `check_function_return_value_2`: Library code that calls a function with a pointer and a zero argument, then checks if the returned value is non-zero to decide whether to update a global variable.
- `check_function_return_value`: This library routine calls a function with a pointer and a zero argument, then checks if the returned value is non-zero.
- `return_one`: This library routine returns a one value.
- `get_linked_list_value`: Library code that checks a byte at an offset in a structure and, if certain conditions are met, follows a chain of pointers or returns a value from a structure offset.
- `traverse_linked_list_node`: Checks a byte at a specific offset in a structure and, if certain conditions are met, follows a chain of pointers through a linked list to return a value from a distant offset.
- `get_pointer_from_structure`: This library routine traverses a chain of pointers within a structure to retrieve or return a value from a specific offset.
- `fetch_indirect_value_2`: This library routine loads a longword from a pointer-offset or returns zero if a register is zero.
- `fetch_indirect_value`: Library code that loads a longword from a pointer-offset or returns zero if a register is zero.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `update_structure_field`: Library code that stores a function return value into a structure offset and conditionally retrieves a pointer from a structure or a global variable to pass to another routine.
- `check_library_status_and_set_flag`: Library code that checks a boolean flag and a function return value to decide whether to update a global flag and retrieve a value from a nested pointer.
- `update_structure_members`: As part of library code, this routine calls several routines to check or update pointers and counters in a structure, and if certain conditions are met, copies a value and updates a pointer or a counter.
- `update_structure_pointers`: Library code that updates a counter in a table, copies a value, and updates pointers or values within a structure based on certain conditions.
- `update_table_and_pointers_2`: Library code that updates a counter in a table and certain pointers in a structure, while also calling a library function and storing a result in a structure offset.
- `update_table_and_pointers`: Library code that executes an indirect function, updates a counter in a table, and updates pointers in a structure if certain conditions are met.
- `update_table_entry_and_pointers`: This library routine updates a counter in a table, copies a string, and updates pointers in a structure if a certain condition is met.
- `copy_string_and_update_fields`: This library routine copies a null-terminated string from a source pointer to a destination pointer and then overwrites two dwords in a target structure using dwords from two other structures.
- `execute_conditional_library_chain_2`: Library code that executes a sequence of conditional library calls and indirect function pointers, updating several global variables and a stack-based pointer.
- `execute_nested_conditional_callbacks`: Executes a sequence of conditional library calls and indirect function pointers, updating a global variable or a pointer-based memory location with the result of a previous call.
- `execute_nested_callbacks`: Library code that executes a sequence of conditional or indirect calls, potentially updating a pointer or a global variable with a returned value.
- `evaluate_conditional_function_chain`: Tests a return value and, if non-zero, executes a sequence of indirect calls or conditional calls using a pointer and a value retrieved from a table.
- `execute_conditional_pointer_logic`: Library code that updates a global variable with a returned value and executes a sequence of conditional calls or indirect calls based on certain flags or pointer values.
- `execute_conditional_pointer`: Library code that calls a routine via a pointer and executes a conditional branch or a sequence of calls based on a global variable.
- `execute_conditional_function_8`: Library code that calls a function and, depending on the return value and a global variable, either calls another function or executes a sequence of calls and updates a structure field.
- `validate_library_parameters`: As part of library code, this routine checks a global variable, calls a library function via a global base, and executes a parameter-validated routine.
- `execute_indirect_function_55`: Loads a function pointer from a global offset and executes it.
- `execute_indirect_function_56`: loads a function pointer from a global table and executes it.
- `execute_indirect_function_57`: Library code that calls a function pointer stored at a location relative to a global pointer.
- `execute_indirect_function_58`: Library code that jumps to a function address retrieved from a global pointer offset.
- `execute_indirect_function_59`: Library code that performs an indirect call to a function pointer located in a data table.
- `execute_indirect_function_60`: Library code that executes a function pointer found via an offset from a global variable.
- `execute_indirect_function_61`: Library code that dereferences a pointer from a global table to call another routine.
- `execute_indirect_function_62`: Library code that calls a routine using a pointer stored in a global memory area.
- `execute_indirect_function_63`: Library code that executes a function pointer located at a specific offset from a global address.
- `execute_indirect_function_64`: Library code that performs an indirect jump to a function pointer stored in a global table.
- `execute_indirect_function_65`: Library code that calls a function pointer retrieved from a global pointer or table.
- `execute_indirect_function_66`: Library code that executes a function pointer located via an offset from a global variable.
- `execute_indirect_handler_2`: Library code that loads a pointer from a global variable and jumps to it.
- `execute_indirect_function_pointer_38`: Loads a function pointer from a table and executes it.
- `call_indirect_via_table_4`: This library code calls another routine through an indirect pointer taken from a global table.
- `execute_indirect_function_67`: Loads a function pointer from a global offset and executes it.
- `execute_indirect_function_68`: loads a function pointer from a global offset and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_39`: loads a function pointer from a global offset and executes it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `invoke_indirect_function`: Loads a function pointer from a global offset and calls it.
- `execute_indirect_function_pointer_40`: Loads a function pointer from a global table and executes it.
- `execute_indirect_call_14`: Library code that calls a function pointer stored at an offset from a global pointer.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `execute_indirect_call_15`: Library code that calls a function pointer stored at a different offset from a global pointer.
- `execute_indirect_handler_3`: Library code that dereferences a pointer stored in a global variable and executes the target routine.
- `execute_indirect_handler_4`: Library code that calls a routine via a pointer located at an offset from a global variable.
- `execute_indirect_handler_5`: Library code that performs an indirect call to a routine pointer stored in a global variable.
- `execute_indirect_handler_6`: Library code that executes a function pointer retrieved from a global memory location.
- `execute_indirect_handler_7`: Library code that jumps to a routine address found by dereferencing a pointer in a global variable.
- `execute_indirect_handler_8`: Library code that calls a routine using a pointer stored in a global variable.
- `execute_indirect_handler_9`: Library code that dereferences a pointer from a global variable and transfers control to that address.
- `execute_indirect_handler_10`: Library code that executes a routine pointer located via a global variable offset.
- `execute_indirect_handler_11`: Library code that loads a routine pointer from a global variable and jumps to that address.
- `call_indirect_via_global`: This library code calls another routine indirectly through a pointer stored in a global variable.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_on_type_value`: This library routine inspects a value from a structure and executes different library routines or updates a global variable based on that value.
- `return_constant_one`: Library code that returns a constant value of one.
- `toggle_global_bit_mask`: Library code that determines a bitmask value based on a field in a structure and XORs this value into a global variable.
- `store_register_to_global`: As part of library code, this routine stores a register value into a global variable offset by a fixed amount and then calls another routine using a pointer from a structure.
- `validate_string_attribute_metadata`: Verifies string attributes and updates a string table or pointer if certain conditions are met.
- `check_string_attributes`: Library code that checks a flag in a structure and conditionally calls other routines using a pointer or a global variable.
- `process_string_and_attributes`: This library routine copies a string, calls a library function, and performs several conditional checks involving pointer offsets and string copies, potentially updating a local variable with a value from a structure.
- `test_record_allocations`: Library code that attempts to allocate a sequence of records using a list of provided parameters and returns one if all allocations succeed or zero if any allocation fails.
- `traverse_function_chain`: Library code that calls a routine pointer repeatedly until a specific error value is returned.
- `check_function_chain`: Library code that checks if a function pointer returns a specific error value and calls itself recursively if the value is not that error code.
- `recursively_call_function`: Library code that calls a function and continues to call itself recursively as long as the returned value is not negative one.
- `call_other_and_return_one`: The library routine calls another routine and then returns one.
- `initialize_info_library_tables`: Library code that initializes internal tables by repeatedly calling another routine with various constant arguments and a pointer derived from the input parameter.
- `execute_list_of_library_calls`: Library code that calls another routine multiple times with a fixed first parameter and varying other constant arguments.
- `initialize_display_palette_entries`: Library code that calls another routine or a sequence of calls with a fixed set of arguments, passing the same first parameter each time and varying the remaining constants.
- `register_info_table_entries`: Library code that calls another routine ten times with a fixed set of integer arguments, passing the same first parameter each time and varying the remaining constants.
- `update_info_table_entries`: Library code that calls another routine or library function nine times with a fixed set of six integer arguments each time, passing a pointer that cannot return a value.
- `traverse_metadata_structure_2`: Recursively traverses a metadata structure or table to perform conditional updates or metadata-based calls.
- `traverse_metadata_structure`: Library code that recursively traverses a structure and calls a routine to initialize block metadata or calls another routine based on a pointer comparison or a non-zero register value.
- `call_library_function_with_offset`: Library code that calls a library function using a pointer retrieved from a structure member, and conditionally calls another routine if a register contains a non-zero value.
- `process_pointer_list_and_store`: Library code that iterates through a list of pointers, calls a routine to get a value, and stores that value into a structure member or a global pointer.
- `update_pointer_list`: Library code that increments a pointer, copies a value, and iterates through a list of pointers while updating a structure and storing results from a called routine.
- `process_pointer_list`: Library code that iterates through a list of pointers, calling a routine and storing a returned value into a structure member.
- `process_pointer_list_and_copy`: Library code that iterates through a list of pointers, calling a routine and storing a returned value into a structure member, while also performing string copies.
- `branch_on_pointer_equality`: Library code that checks if a pointer or a dereferenced pointer matches a certain value, then calls other routines or updates a structure member with a value from another pointer.
- `update_pointer_from_structure`: Library code that calls a routine with a pointer from a structure, then updates that structure member with a value from another pointer.
- `update_pointer_on_mismatch`: Library code that checks if a pointer or a dereferenced pointer matches a certain value, then calls other routines or replaces a structure member with a value retrieved by dereferencing a pointer.
- `update_pointer_dereference`: Library code that calls a routine and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `modify_global_state_and_execute`: Library code that sets a global variable, checks a condition, calls several library functions, and updates a global pointer.
- `execute_sequential_callbacks_2`: Library code that executes a sequence of callbacks, potentially updating a global pointer with a value retrieved from a dereferenced pointer or a table.
- `update_pointer_from_table_2`: Library code that retrieves a value from a dereferenced pointer, calls another routine, and potentially updates a global pointer with a value from a table.
- `execute_sequential_callbacks`: Library code that calls another routine twice using pointers or offsets or registers.
- `update_string_table`: Library code that calculates the size of a linked list, allocates a table, and populates it with data from the list.
- `process_string_list_and_table_3`: Library code that calculates the size of a linked list, allocates or updates a table with a certain size, and then iterates through the list to copy or process string pointers and their associated data.
- `process_string_list_and_table_2`: Library code that iterates through a linked list to calculate a size, updates a table with the result, and then copies strings from the list into a destination buffer.
- `process_string_list_and_table`: Library code that iterates through a linked list, calculates a size, and updates a table with results from library calls or constant values.
- `process_string_table_2`: Library code that iterates through a list of strings, calls a routine to find or create entries in a table, and updates that table with the results.
- `iterate_string_list`: Library code that calculates a string length and then iterates through a list of pointers, calling other functions and storing results in a destination array.
- `iterate_string_list_and_update`: Library code that iterates through a list of pointers, calling a routine and updating a table with a returned value or a constant.
- `execute_indirect_function_52`: Loads a function pointer from a global offset and executes it.
- `execute_indirect_function_53`: loads a function pointer from a global offset and calls it.
- `execute_indirect_function_54`: loads a function pointer from a global offset and executes it.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `configure_string_formatting_parameters`: This library routine updates a global buffer or table with a sequence of constant values and pointers, depending on a switch-case check of a configuration flag.
- `empty_routine_3`: Library code that is empty and returns immediately.
- `validate_and_execute_conditional_logic`: This routine checks the return values of several library function calls and, if all return non-zero, executes a specific sequence of further library calls or returns a byte from a table or structure based on a switch-case condition.
- `execute_variable_calls`: Library code that executes a certain number of function calls, stores the last return value in a pointer, and then executes a variable number of additional calls based on a byte in a global structure.
- `execute_variable_number_of_calls`: This library routine checks a byte in a global structure and executes a specific number of calls to another routine.
- `return_from_subroutine`: This routine performs a single-instruction return or a simple jump to return control to a caller.
- `validate_record_allocation`: Library code that checks a global flag and calls a routine to allocate a record, returning a status or zero if an error occurs.
- `allocate_and_verify_record`: Library code that calls a routine to allocate a record and returns a status or zero if an error occurs.
- `return_from_subroutine_7`: This library code performs a return from the current subroutine.
- `initialize_and_verify_records`: Library code that writes specific values to a table and then attempts to allocate and verify a sequence of records.
- `test_function_pointer_success`: Library code that calls a function pointer and returns one if the return value is not negative one, otherwise returns zero.
- `verify_function_success`: executes a function pointer and checks if the return value is not negative one.
- `check_function_return_2`: Library code that calls a routine and returns true if the return value is not negative one.
- `return_constant_one_2`: This library routine returns the integer value one.
- `traverse_linked_list_recursively`: Library code that recursively traverses a linked list and executes an indirect call using a bit-masked value if a certain condition is not met.
- `return_constant_zero`: Library code that returns a constant value of zero.
- `traverse_linked_list_9`: Library code that recursively traverses a linked list, performing different actions or returning a constant based on a value found in a structure member.
- `execute_self_recursive`: Library code that calls itself once.
- `execute_indirect_with_constant`: Library code that calls a routine and returns a constant zero.
- `execute_two_calls`: Library code that calls another routine twice and returns a constant value.
- `execute_multiple_callbacks_2`: Library code that calls another routine or a global function pointer multiple times based on a condition checked against a global variable, then calls another routine with a specific pointer and constant, and finally calls another routine with a certain pointer and a constant.
- `setup_string_template`: Library code calculates a bitmask from a value in a table and calls a string-formatting routine multiple times with different bit-masked arguments and a global pointer.
- `format_string_buffer`: Library code that prepares a temporary buffer with a specific string prefix and calls a string-formatting routine with a variable number of arguments.
- `format_string_arguments`: Library code that prepares a pointer-based argument list or a temporary buffer for a string-formatting routine that uses a pattern or template string and multiple string arguments.
- `calculate_string_length_7`: counts the number of bytes in a null-terminated string.
- `calculate_string_length_5`: Library code that increments a register while checking if a null terminator has been reached at an offset from a pointer.
- `copy_cstring_to_buffer_2`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `copy_memory_block_7`: copies a specified number of bytes from a source address to a destination address using a loop.
- `copy_memory_block_2`: Library code that copies a fixed number of bytes from a source pointer to a destination pointer.
- `compare_strings_equal`: compares two null-terminated strings and returns true if they are identical.
- `compare_strings_2`: Library code that compares two null-terminated strings and returns true if they are equal.
- `get_string_length`: Library code that counts the number of bytes in a null-terminated string.
- `execute_library_operations`: Library code that calls several other library functions, including a routine that initializes a structure array and routines that use conditional logic or call library vectors.
- `call_string_formatter_2`: Library code that calls another routine using a pointer to a global variable and a local buffer, then calls another routine using the populated buffer and other passed parameters.
- `copy_string_from_length_table`: Library code that reads a length or count from a table and copies that number of bytes to a destination before appending a null terminator.
- `copy_string_data`: Library code that copies bytes from a source location to a destination location and appends a null terminator.
- `initialize_record_block_metadata`: Library code that populates a memory block with a set of initial values, including null-terminating certain fields and setting a pointer to a secondary buffer, before calling a routine to allocate an indexed record block.
- `call_library_function_via_offset`: Library code that calls a library function using a pointer retrieved from a global variable offset.
- `execute_library_calls`: Library code that calls a library function or another routine multiple times using a pointer retrieved from a structure member.
- `execute_indirect_function_94`: Library code that calls a function pointer retrieved from a global offset.
- `store_byte_to_address_6`: copies a byte from a data register into a memory location pointed to by an address register.
- `call_indirect_via_table_2`: The routine calls another routine indirectly through a pointer stored in a global variable table.
- `call_indirect_via_table_3`: The routine calls another routine through an indirect pointer taken from a global table.

### version.library

- `return_all_ones_4`: Returns a 32-bit value where all bits are set to one.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call`: Library code that decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `get_library_version_status`: Library code that checks a flag in a structure and either calls two indirect functions or sets a bit in a byte.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_zero_23`: This library routine always returns a zero or null value.

### diskfont.library

- `check_and_set_font_handle_2`: Library code that calls a function via a table entry and, if the result is non-zero, stores a value into a structure and calls another function.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_library_structure`: Library code that opens a library, checks for valid handles, and initializes a structure with a pointer to a local buffer and a null terminator.
- `release_library_resources`: Library code that calls functions to close a library and free memory.
- `process_linked_list_elements`: Library code that iterates through a linked list, removing or updating certain elements if a certain condition is met.
- `manage_list_element`: Library code that updates a linked list or modifies a flag in a structure and calls library routines to free or close resources.
- `set_byte_flag`: Library code that sets the high bit of a byte at an offset from a pointer and returns zero.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_check_counter_2`: Library code that decrements a value in a structure and calls another routine if the value reaches zero and a specific bit is set.
- `return_zero_24`: Library code that returns a zero value.
- `map_font_metadata_labels`: Iterates through a linked list of font or character records to extract and copy string labels into a destination buffer.
- `copy_string_list_to_buffer`: Iterates through a linked list of strings, calculating the length of each string and copying the characters into a destination buffer if enough space remains.
- `parse_string_and_populate_struct`: Iterates through a linked list of strings, calculating string lengths and copying string contents into a destination buffer or structure.
- `parse_font_record_metadata_ext_2`: This library routine parses a stream of font or character record headers to extract metadata and copy string labels into a destination buffer.
- `parse_font_record_metadata_2`: This library routine parses a stream of font or character record headers to extract metadata and copy string labels into a destination buffer.
- `extract_font_record_metadata`: This library routine scans a data stream to identify specific font or character record headers and extracts or copies metadata, such as a pointer or length, into a destination structure.
- `parse_font_record_metadata`: This library routine parses a stream of data to identify specific font or character record headers and extracts metadata, such as a pointer to a string, a length, and a flag, into a structured table or buffer.
- `parse_font_record_metadata_ext`: This library routine parses a stream of font or character record headers to extract metadata and copy string labels into a destination buffer.
- `return_zero_and_restore_registers`: Clears register d0 and restores registers d2 through d6 and a2 through a4 from the stack before returning.
- `calculate_weighted_difference`: Library code that calculates a value based on the difference between two memory offsets and subtracts values from a global table if certain bits match.
- `decrement_value_with_mask`: Library code that checks certain bits against a global table and subtracts values from a register if a bitmask check passes.
- `decrement_table_values_with_bitmask_check`: Subtracts values from a register using an indexed table lookup while checking bitmasks against a remote byte.
- `iterate_and_subtract_table_values`: Iterates through a table of integers, subtracting values from a register while checking bitmasks against a byte in a remote memory location.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_glyph_table_entry`: Searches a table of font or glyph structures to find the entry with the maximum size or value, then updates a corresponding table or pointer.
- `update_font_glyph_table`: Iterates through a table of font structures to find a glyph with the maximum size or value, then updates a corresponding table or pointer.
- `optimize_font_glyph_table`: Iterates through a table of font structures to find a glyph with the maximum size or value, then updates a corresponding table or pointer.
- `find_max_font_size_2_2`: Iterates through a table of font or glyph-related structures to identify the entry with the largest size or value, while clearing a specific or updating a pointer-based structure.
- `find_max_font_size_2`: Iterates through a table of font or glyph-related structures to identify the entry with the largest size or value, while clearing a specific offset or updating a global pointer.
- `find_largest_font_size`: Iterates through a table of font structures to identify the entry with the maximum font size.
- `find_max_font_size`: iterates through a table of font or glyph-related structures to find the entry with the largest size or value, while clearing a specific offset in the target structure and updating a global tracking pointer or counter.
- `populate_font_table`: This library routine parses a font resource to build a lookup table, calculating offsets and copying character or attribute data into a newly allocated memory block.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `forbid_resources`: Library code that calls a routine to forbid certain system resources.
- `permit_resources`: Library code that calls a routine to permit certain system resources.
- `allocate_memory`: Library code that calls a routine to allocate memory.
- `free_memory`: Library code that calls a routine to free memory.
- `add_to_tail`: Library code that calls a routine to add an item to the end of a list.
- `call_dos_open`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS open routine.
- `call_dos_close`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS close routine.
- `call_dos_read`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS read routine.
- `call_dos_lock`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS lock routine.
- `call_dos_unlock`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS unlock routine.
- `call_dos_examine`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS examine routine.
- `call_dos_examine_next`: Library code that calls a function pointer located at an offset from a global pointer, specifically targeting a DOS examine next routine.
- `get_current_directory`: Library code that calls a routine to retrieve the current directory.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `load_segment`: Library code that calls a routine to load a segment.
- `unload_segment`: Library code that calls a routine to unload a segment.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `find_string_end`: Library code that increments a pointer until a null terminator is reached and then calls another routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length_2`: calculates the number of bytes in a null-terminated string.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_string_content`: Library code that compares two null-terminated strings and returns a constant value or a signed-style comparison result.
- `compare_string_bytes`: Library code that iterates through a string and returns a constant or a flag based on a comparison of characters at certain offsets.
- `compare_string_order`: Library code that compares two null-terminated strings and returns a value indicating if the second string is or contains a certain order relative to the first.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 1 into d0, then returns.
- `find_string_char_forward`: Library code that searches a string for a specific character and returns a pointer to it or a null pointer.
- `compare_string_characters`: compares a byte in a register against a sequence of bytes pointed to by a register.
- `find_string_char`: Library code that searches for the first occurrence of a character within a string and returns a pointer to that character or a null pointer if not found.
- `search_backwards_for_byte`: searches backwards through a memory buffer for a specific byte value.
- `search_for_byte_in_pointer_range`: Iterates backwards from a pointer, comparing a byte to the value at the current address, and returns the pointer if a match is found or null if the pointer reaches a certain limit.
- `search_backwards_for_byte_2`: scans a memory buffer backwards from a starting pointer to find a specific byte value.
- `search_for_delimiters`: Library code that searches backwards through a buffer for a specific delimiter and returns an incremented pointer if a match is found.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `toggle_interrupt_state`: Modifies the interrupt enable bit of the hardware interrupt enable register and updates a global variable before calling another routine via an indirect call.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### A-Train (1992)(Ocean)(De)(Disk 1 of 2).adf:A-Train

- `initialize_memory_and_globals`: Clears a range of memory with zeros and updates a global variable and a pointer-based memory location with a constant value.
- `parse_resource_file_header_3`: Attempts to open a resource file and iterates through its header or table-based structure to validate or extract certain identifiers.
- `update_string_buffer`: Updates a string buffer in memory by copying or formatting a string, then updates a graphics buffer or font-related memory area.
- `copy_word_and_set_byte`: Copies a word from a global memory location to another global memory location and then writes a byte to a preceding memory address.
- `copy_and_update_view`: Copies a word from a global memory location to another and then calls a routine to update a view window or redraw a rectangular area.
- `update_view_window_scrolling`: Checks if a window or view-port position has changed significantly and updates screen clearing or scrolling or redraws certain rectangular regions.
- `update_screen_scrolling_and_clear`: Calculates a coordinate offset and clears specific rectangular regions of the screen using a library call if certain movement or windowing conditions are met.
- `update_screen_scrolling`: Updates global pointers and certain table values before performing conditional horizontal and vertical screen scrolling or windowing.
- `update_conditional_pointers`: Performs conditional calls to other routines while updating several global pointers and clearing a long value in a table.
- `dispatch_conditional_function_calls`: Checks a value and calls a function with or without certain offset-based parameters, then updates several global pointers and clears a long value in a table.
- `update_conditional_pointers_alt`: Updates certain global pointers and clears a long value in a table, then performs conditional calls to other routines.
- `update_conditional_pointers_with_range`: Checks a value and calls a routine with specific parameters if that value is negative or positive, then updates global pointers.
- `execute_complex_logic`: Performs conditional calls to other routines and updates several global variables or pointers.
- `update_viewport_bounds`: Calculates and updates the visible screen area or viewport boundaries based on current camera or object positions.
- `update_map_cell_state`: Checks if a certain value has changed and, if a counter reaches a certain threshold, iterates through a large memory buffer to find and update specific cell or tile values.
- `populate_data_table`: This routine iterates through a sequence of or certain indices, calling a function to retrieve a value and storing the result into a global table or array.
- `set_grid_value_on_search`: Searches a grid for a value and, if found, updates a byte in a table using a calculated offset.
- `search_grid_for_value`: iterates through a two-dimensional table to find a matching value and records its row and column indices.
- `trace_path_on_grid`: Calculates a path or boundary by traversing a table of offsets based on a starting coordinate.
- `update_counter_and_branch_logic_2`: Increments a counter and resets it if it exceeds a threshold determined by a lookup table, then conditionally executes a function pointer and updates state or other counters based on the return value.
- `update_counter_and_branch_logic`: Increments a counter and resets it if it exceeds a threshold determined by a lookup table, then recursively calls another routine or updates a set of memory-mapped or global values based on the counter's masked lower bits.
- `update_coordinate_state_large`: Checks a value or a pointer-based function and, depending on the result, updates several memory locations with or from other memory location.
- `update_coordinate_state_small`: Checks a value or a pointer-based function and, depending on theresult, updates several memory locations with or from other memory location.
- `update_coordinate_state_small_2`: Sets a global variable to a value based on an input, then conditionally updates several memory locations or calls another routine.
- `update_coordinate_state_2`: Checks a value or a pointer-based function and, depending on the result, updates several memory locations with or from other memory locations or constants.
- `trace_grid_path_validity_2`: Iteratively updates two sets of coordinates using a lookup table and checks if the resulting destination coordinates or an intermediate step are valid.
- `copy_table_values`: Copies several values from one part of a table to another and clears a global variable.
- `modify_conditional_threshold`: Checks if a register value falls within certain ranges and, if certain conditions in a memory location are met, writes a constant value to a memory location offset from a pointer.
- `update_conditional_limit_value_threshold`: Checks if a value in a register or table falls within a specific range and, if it matches a certain constant, writes a new value to a memory location or updates a secondary offset-based address.
- `update_conditional_limit_value`: Checks if a value in a table equals a specific constant to write a new value to a memory location, or checks if a value is within a certain range and a secondary condition is met to write a large constant to another memory location.
- `update_limit_value`: Checks if a value in a table is equal to a certain constant and, if a condition is met, writes a large constant value to a memory location.
- `jump_to_update_counter_and_branch_logi`: Jump stub: passes control straight on to update_counter_and_branch_logic_2.
- `trace_grid_path_validity`: Iteratively updates two sets of coordinates using a lookup table and checks if the resulting destination coordinates or an intermediate step fall outside a valid range.
- `check_grid_bounds`: Checks if certain coordinates or values fall within a specific range and returns a constant or a flag.
- `check_coordinate_bounds`: Retrieves a value from a table using a calculated offset and checks if that value or a second retrieved value falls within a specific range.
- `check_value_range_2`: Checks if a value in a register is within a specific range and returns a constant or a maximum integer value based on the result.
- `clear_d0_and_branch`: Clears the d0 register and branches to another location.
- `update_coordinate_state`: Updates global coordinates using a table lookup and checks if a certain value exceeds a limit or if a counter needs clamping.
- `update_coordinate_and_counter`: Updates a global coordinate or pointer using a value from a table and checks if a counter or certain value exceeds a limit.
- `update_path_state`: Updates a memory location with values from a table or a calculation, modifies a pointer or counter, and potentially resets a state or decrements a counter if a certain condition is met.
- `update_path_and_counter`: Calculates a new coordinate using a table lookup based on a condition and decrements a global counter if a certain condition is met.
- `update_path_and_decrement_counter`: Updates a global coordinate using a table lookup and decrements a counter if a certain condition is met.
- `test_and_restore_registers`: Tests a value at an offset from a pointer and restores registers if the test is non-zero.
- `reset_grid_attributes`: If a value in a table matches certain criteria, it clears a byte in a table and updates a second byte in a different table using a function call.

The function list continues in: g1750-recovery-functions-2.md, g1750-recovery-functions-3.md.
