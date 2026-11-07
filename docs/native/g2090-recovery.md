# Game 2090 (secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02090). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3608608 top-level bytes in 50 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 1225 of 1225 named (0 author symbols), 1225 of 1225 described (67 by rule, 1114 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4068 of 4068 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2435268 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 34193 of 34193 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 603380 |
| code | 105328 |
| data | 2731286 |
| padding | 12966 |
| unused | 155648 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 117 | 2 |
| else | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| newcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| Failat | 2 | 117 | 2 |
| execute | 2 | 117 | 2 |
| copy | 2 | 117 | 2 |
| makedir | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| endif | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| iconx | 12 | 806 | 37 |
| loadwb | 12 | 536 | 30 |
| wait | 2 | 117 | 2 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:defjam | 1 | 155 | 3 |
| disk-validator | 1 | 386 | 1 |
| icon.library | 10 | 1375 | 109 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:monkey_island | 5 | 28660 | 983 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 3 of 4).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 717 | 31 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| music.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 2 of 4).adf:DISK02.LEC (raw code file, load address unknown) | 1 | 0 | 0 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 3 of 4).adf:DISK03.LEC (raw code file, load address unknown) | 1 | 0 | 0 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 4 of 4).adf:DISK04.LEC (raw code file, load address unknown) | 1 | 0 | 0 |
| Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 4 of 4).adf:savegame._a_ (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address (play-rs-m1500): 670 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a](disk 1 of 4)/Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:Rooms/music.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a](disk 2 of 4)/Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 2 of 4).adf:DISK02.LEC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a](disk 3 of 4)/Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 3 of 4).adf:DISK03.LEC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a](disk 4 of 4)/Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 4 of 4).adf:DISK04.LEC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a](disk 4 of 4)/Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 4 of 4).adf:savegame._a_: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- secret of monkey island, the (1990)(lucasfilm games)(fr)[cr defjam][a](disk 1 of 4)/Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:defjam: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 5 never-loaded region(s), 2435268 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `call_and_update_globals` (runtime segment an address (play-rs-m1500)): Sets a global variable, calls a function pointer derived from that variable, and then stores the variable and three bitwise-modified versions of it into other global variables. Kind: disk hardware/track check. Pass path: the routine following the indirect call (setting _u22__sub_09D8, etc.).

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen bytes from a source location to a destination location using a loop.

### else

- `allocate_and_initialize_data_buffer`: Traverses a linked list to find the maximum value in a specific field, then allocates a buffer and populates it with a header and data.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination buffer and then executes a function pointer.

### assign

- `allocate_and_initialize_data_block_2`: Traverses a linked list to find the maximum value in a specific field, then allocates a buffer and populates it with a header and copied data.
- `copy_small_buffer`: The routine copies sixteen bytes from a source location to a destination location and then calls another routine.

### endcli

- `allocate_and_initialize_data_buffer_4`: Traverses a linked list to find the maximum value in a specific field, then allocates a buffer and populates it with a sequence of incrementing values.
- `copy_data_block_2`: Copies sixteen bytes from a memory location offset by a pointer into a destination buffer and calls another routine.

### newcli

- `allocate_and_initialize_data_buffer_5`: Traverses a linked list to find the maximum value in a specific field, then allocates a buffer and populates it with a header and a sequence of incrementing values.
- `copy_memory_block_3`: Copies a fixed number of words from a source address to a destination address.

### echo

- `reallocate_and_extend_data_buffer`: Traverses a linked list to find the maximum value in a specific field, then expands a data structure by writing a header and copying data blocks.
- `copy_memory_block_4`: Copies sixteen bytes from a source address to a destination address using a loop.

### Failat

- `allocate_and_initialize_data_buffer_6`: Traverses a linked list to determine a maximum value, then allocates a buffer and populates it with a sequence of incrementing values.
- `transfer_data_chunk`: Iterates through a fixed number of words to move data from one memory location to another.

### execute

- `reallocate_and_initialize_buffer`: Traverses a linked list to find a maximum value and then expands a data structure by writing a header and filling a memory block with incrementing values.
- `duplicate_data_sequence`: Performs a block copy of sixteen bytes from a specified offset in a structure to a target buffer.

### copy

- `reallocate_and_copy_data_buffer`: Traverses a linked list to find the maximum value in a specific field, then reallocates a buffer and populates it with a header and copied data.
- `move_word_array`: Copies a sequence of eight words from a source pointer to a destination pointer.

### makedir

- `allocate_and_initialize_data_buffer_7`: Traverses a linked list to find a maximum value, then allocates a buffer and populates it with a header and specific data patterns.
- `bulk_copy_bytes`: Transfers a block of sixteen bytes from a source memory area to a destination area.

### cd

- `allocate_and_initialize_data_block`: Traverses a linked list to find the maximum value in a specific field, then allocates a memory block and populates it with a header and data.
- `copy_memory_block_and_call_2`: Copies sixteen bytes from a memory location to a destination buffer using a loop and then executes a function pointer.

### endif

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `allocate_and_initialize_data_buffer_2`: Traverses a linked list to find the maximum value in a specific field, then allocates a buffer and populates it with a header and a sequence of incrementing values.
- `copy_memory_block`: Copies sixteen bytes from a memory location to a destination buffer.

### iconx

- `initialize_application_environment`: Prepares the execution environment by restoring patch states and determining whether to launch via the command line or the Workbench.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and a name, including a wait command with a long timeout and an endcli redirected to NIL, then runs it through the DOS library.
- `install_library_functions`: Creates and installs a library from a descriptor by making the library structure and setting up its function table.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `concatenate_and_callback`: Copies a source string into a destination buffer, appends a second string to that buffer, and then passes the resulting buffer and the total length to a third routine.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_and_return`: Stores a byte from a data register into a memory location pointed to by a register and returns.

### loadwb

- `initialize_execution_environment`: Retrieves system state and parses command-line arguments or environment strings into a structured format.
- `update_state_and_run_callbacks`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_tool_from_resident_tag`: XOR-decodes a global string with a key applied twice, allocates a memory block, and uses various library calls to start a tool from a resident tag.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `execute_indirect_function_3`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_workbench_version`: Compares a string against a global variable to verify a Workbench version.
- `write_space_sequence`: Writes a sequence of four space characters starting at a specific offset from a pointer.
- `compare_string_contents`: Compares two null-terminated strings byte by byte until a mismatch or a null terminator is found.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `call_reschedule_wrapper`: Calls a function pointer stored at a fixed offset within a global structure.
- `execute_indirect_function`: Fetches a function pointer from a memory location using an offset and executes it.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `execute_indirect_function_2`: Fetches a function pointer from a global variable and executes it.
- `invoke_recursive_handler`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `invoke_and_verify_buffer`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_address`: Stores a byte from a register into a memory location pointed to by another register.

### wait

- `allocate_and_initialize_data_buffer_3`: Traverses a linked list to find the maximum value in a specific field, then allocates a buffer and populates it with a header and a sequence of incrementing values.
- `copy_memory_block_and_call_3`: Copies sixteen words from a memory location offset by a register value to a destination pointer, then calls another routine.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:defjam

- `apply_color_palette_shift`: Performs a recursive bitwise right shift on data and stores the resulting values into a memory location.
- `store_shifted_word`: Loads a longword from a memory location and stores a word at a different location composed of a high byte from an undefined value and the low byte of the loaded longword.
- `perform_recursive_shift`: Performs a recursive bitwise right shift on a register until it reaches zero.

### disk-validator

- `format_validation_string`: Converts a numeric value into a character representation and stores it in a buffer.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_status`: As library code, it modifies a status byte or manipulates a linked list based on a flag in a structure.
- `set_status_bit`: Sets the eighth bit of a byte located at an offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_callback_and_store`: Copies two 32-bit values into a structure, executes a function via a pointer, and either calls another routine or stores the results in global variables based on the return value.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_empty_routine`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `call_recursive_routine`: This library routine calls another routine and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `empty_library_routine`: This library code is empty.
- `trigger_infinite_recursion`: This routine performs a direct call to itself without any conditional branching or termination logic.
- `call_other_routine`: This library code calls another routine and then returns.
- `execute_recursive_search`: This library code calls a routine to perform a recursive search through a list.
- `call_other_routine_3`: Library code that calls another routine.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `recursively_traverse_and_clear_nodes`: This routine performs a recursive traversal of a linked structure.
- `update_icon_structure`: Library code that validates a resource, copies a block of ten words to a destination, and assigns several individual fields within a structure.
- `copy_block_and_assign_fields`: Library code that performs a loop to copy ten words from a source location to a destination, followed by several individual field assignments from a source structure to a destination structure.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_structure_contents`: This routine updates the internal data of an icon structure by recursively traversing and modifying linked lists and associated data buffers.
- `recursively_update_icon_chain`: This routine performs a recursive traversal of a linked list or structure related to icon data.
- `update_icon_attributes`: As library code, this routine updates fields in a structure by calling other routines and iterating through a linked list to verify and assign attributes.
- `execute_and_return_zero_2`: As library code, it calls a function through a pointer repeatedly while updating a counter and a global variable.
- `execute_and_return_zero`: As library code, it calls another routine and then returns zero.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `update_icon_metadata_structures`: Iterates through various icon property fields and linked lists to update their contents.
- `process_icon_metadata`: Iterates through various icon-related data structures, including linked lists and tables, to update icon properties and trigger associated callback routines.
- `propagate_icon_changes`: As library code, this routine iterates through various tables and linked lists, invoking callback routines for each entry and updating structure fields based on the results.
- `iterate_and_callback`: Library code that iterates through a null-terminated table of pointers, executes a callback for each entry, and performs conditional logic based on the results.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list`: Library code that recursively traverses a linked list structure using a provided pointer and a termination condition.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `validate_and_process_resource`: This routine checks the result of a function call and, if non-zero, performs a sequence of conditional calls using derived offsets.
- `call_empty_routine`: This routine calls another routine and then returns.
- `return_zero_3`: Library code that returns zero.
- `update_icon_data_table`: As library code, it decrements a counter and uses the result to store two values into a table at a calculated offset.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `process_icon_chain`: Library code that initializes data, traverses a linked list, and conditionally updates a table or calls other library functions based on the results.
- `return_void_2`: Library code that performs no operations and returns.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_delimiter_in_list`: This library code iterates through a list of pointers to find a specific character delimiter within a string.
- `find_string_delimiter`: Library code that searches through a list of pointers to find a specific character delimiter within a string.
- `find_string_delimiter_2`: As library code, this routine iterates through a list of pointers to find a specific character delimiter within a string.
- `search_list_recursively`: This library code performs a recursive search through a list by comparing values and iterating through elements until a match is found or the list ends.
- `search_list_elements_2`: As library code, this routine iterates through a list and performs arithmetic comparisons on retrieved values until a termination condition is met.
- `search_list_elements`: Library code that iterates through a list and performs arithmetic comparisons on retrieved values until a termination condition is met.
- `loop_with_arithmetic_check`: As library code, it executes a loop that performs arithmetic on a value and calls other routines until a specific condition is satisfied.
- `search_list_for_match`: This library code iterates through a list and performs arithmetic comparisons on retrieved values until a termination condition is met.
- `parse_complex_string`: This library code extracts a numeric value from a specific offset within a string and then copies a subsequent string segment into a destination buffer.
- `parse_numeric_string_3`: This library code converts a sequence of characters into a numeric value by iterating through the string and multiplying the running total by ten for each digit.
- `convert_numeric_string`: Library code that iterates through a character sequence to calculate a numeric value.
- `parse_numeric_string_2`: Library code that converts a sequence of characters into a numeric value and performs a lookup in a table.
- `map_input_range`: As library code, it checks if a value is within a specific range and returns an offset or a negative one.
- `return_error_code`: This routine returns a value representing an error or invalid state.
- `call_indirect_handler`: Library code that calls a routine by dereferencing a pointer located at a specific offset within a table.
- `call_indirect_handler_offset`: Library code that calls a routine by dereferencing a pointer located at a different offset within a table.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `initialize_icon_data`: As library code, it calls other library functions to populate a table with a specific value if a condition is met.
- `return_zero`: This library routine is empty and simply returns zero to the caller.
- `compare_and_return`: Calls another routine and returns zero if the result does not match the fourth parameter, otherwise returns the result of that call.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `validate_reschedule_result`: As library code, it calls a rescheduling routine and returns zero if the result does not match a provided parameter.
- `return_void`: This library routine is empty and performs no operations before returning.
- `rotate_32bit_value_2`: Library code that performs a bitwise rotation on a 32-bit value through sixteen iterations.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_pointer`: Library code that stores a single byte from a register into a memory location pointed to by another register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_character_in_string`: As library code, it searches for the first occurrence of a specific character within a null-terminated string.
- `return_zero_constant_2`: As library code, it simply returns zero.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string`: Library code that calculates the length of a source string and then copies it to a destination buffer.
- `compare_strings`: Library code that compares two strings and returns zero if they are identical or if a null terminator is reached, otherwise returns one.
- `return_constant_one`: Library code that returns the integer value one.
- `skip_string_characters`: Library code that iterates through a string and decrements a counter until a null terminator is found or the counter reaches zero.
- `call_other_routine_2`: Library code that calls another routine and returns.
- `copy_with_limit`: Library code that copies bytes from a source to a destination up to a specified count or until a null terminator is reached, then pads the remainder with null bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_linked_list_2`: Library code that traverses a linked list and validates the contents of each node based on a specific offset and size calculation.
- `validate_reschedule_result_2`: As library code, it iterates through a list and checks if a value at a specific offset is zero.
- `return_zero_2`: This library routine is empty and simply returns a value of zero to the caller.
- `validate_and_update_icon_data`: As library code, it performs a sequence of recursive calls to validate parameters and compute values, eventually storing a result into a memory offset of a pointer returned by a previous call.
- `return_negative_one`: Library code that returns a value of all bits set.
- `update_list_attributes`: Library code that iterates through a linked list to verify attributes and updates a field in a structure using a value retrieved from another routine.
- `traverse_icon_list`: As library code, it iterates through a linked list, updating pointers and values until a termination condition is met.
- `validate_and_update_icon_attributes`: Iterates through a linked list to verify specific attribute values and updates a structure's field with a value from a register.
- `return_all_ones`: This routine returns a value consisting of all bits set to one.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:monkey_island

- `initialize_system_environment`: Clears a memory block and initializes global pointers and system structures.
- `copy_memory_block_5`: Copies a block of memory from one location to another using a loop.
- `write_constant_before_pointer`: Writes a fixed 32-bit constant four bytes before an address argument when both checked values differ from minus one.
- `initialize_file_system_buffers`: Clears a memory range and retrieves directory information or file paths from a structure.
- `handle_interrupt_reply`: The routine conditionally calls a function pointer from a structure, calls an empty stub, calls a function pointer from a global variable, calls another routine, and if two fields in a structure are non-zero it calls a function pointer from a global variable, calls another function pointer from a global variable, and calls another function pointer from a global variable.
- `return_immediately_9`: An empty routine that returns immediately.
- `return_immediately_10`: This routine performs no operations and immediately returns to its caller.
- `free_linked_list_2`: Walks a linked list stored in a global variable, calls another routine for each node, and clears two global pointers.
- `dispatch_unnamed_routine`: A single-instruction routine that calls another routine.
- `store_globals_and_set_flag`: Calls another routine twice and then returns.
- `call_other_routine_5`: The routine calls another routine and then returns.
- `call_routine_and_set_flag`: The routine calls another routine twice and then stores a short value into memory at an offset from a pointer held in a register.
- `clear_pointer_offset_word`: The routine calls another routine twice, then stores a zero value at a specific offset from a pointer derived from the first call.
- `read_word_from_pointer_offset`: The routine calls another routine and then returns the word stored at an offset from a pointer left in a register by that call.
- `return_routine_address`: The routine returns its own address.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `clear_multiple_globals`: Sets numerous global variables and memory locations to zero or specific constant values.
- `wait_for_vertical_sync`: Checks global flags and conditionally calls another routine, then enters a loop that waits for a value derived from a global variable to satisfy a condition before updating a global variable.
- `clear_state_variables`: Resets a series of global variables to zero and assigns specific constant values to others.
- `initialize_state_and_clear_flags_2`: Stores the result of another routine into a global variable, clears two other global variables, and calls another routine.
- `setup_memory_pointers_and_clear_table`: Sets several global pointers based on offsets from a base address and performs a loop to clear a block of memory.
- `update_handler_pointer`: The routine stores a global variable into two other global locations, updates a global variable with its own address, and sets a global variable to a constant value.
- `restore_and_clear_pointers`: The routine sets a global variable to a constant, calls another routine, and then restores and clears several global pointers if they are non-zero.
- `update_indexed_data_2`: Computes an index from a register, checks a byte at an offset, and if zero, copies two words from a table to a global variable and clears a word.
- `perform_empty_operation`: This routine is empty and performs no actions before returning.
- `clear_state_variables_2`: Clears multiple consecutive global variables and sets a specific global variable to a constant value.
- `branch_on_memory_value`: Checks a value at a calculated memory offset and a global variable to determine which of two other routines to call.
- `wait_for_vertical_position`: Checks global flags and conditionally calls other routines, then enters a loop that waits for a value derived from a global variable to satisfy a specific bitwise comparison.
- `wait_for_vpos_and_clear_flag`: Calls another routine and conditionally calls another routine, then waits for a vertical position register to reach a value derived from a short in a structure before copying that short to a global variable and clearing it.
- `decrement_and_check_counter`: Decrements a word stored at a specific offset within a structure.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `execute_script_command_logic_2`: Processes a command byte fetched from a data stream by branching into specific logic for variable manipulation, index updates, or table lookups.
- `parse_script_command_byte`: Reads a command byte from a data stream and executes a specific logic branch based on its value.
- `execute_script_command_logic`: Processes a command byte by updating a data structure through a switch statement.
- `update_state_structure_via_switch`: Updates various fields within a data structure based on a command byte using a large switch statement.
- `update_conditional_value_2`: This routine sets a byte to a maximum value and then either resets a value in a structure to a base value or increments it based on a comparison between two elements.
- `process_script_instruction`: Iterates through a sequence of data by incrementing an index and executing different logic branches based on the value of the current byte.
- `update_conditional_value`: Sets a byte to a maximum value, then either resets a value in a structure to a base value or increments it based on a comparison between two elements in that structure.
- `increment_and_set_byte`: Increments a word at a specific offset within a structure and subsequently stores a byte value into another memory location.
- `increment_word_at_offset`: Increments a 16-bit value stored at a specific offset from a pointer.
- `clear_memory_fields`: Sets a 16-bit value at an offset to zero and a single byte at another offset to the maximum value.
- `parse_entity_data_list`: Reads a sequence of values from a memory table and populates a structured data block with pointers and status flags.
- `clear_offset_byte_2`: The routine stores a zero byte at a specific offset from a pointer held in a register.
- `clear_offset_byte`: Clears a single byte at a specific offset from a pointer held in a register.
- `interpolate_tween_values`: Performs linear interpolation and fixed-point arithmetic to transition values between two states over a specified number of frames.
- `tiny_addq_2`: Small routine of 1 instruction that adds 4 to a5.
- `update_interpolation_and_timer_states`: Decrements internal counters and performs fixed-point arithmetic to interpolate values between two states.
- `update_animation_and_timer_states`: Decrements various internal timers and counters to manage frame-based transitions and interpolation.
- `update_text_buffer_and_scrolling`: Manages a text string buffer by handling character decrements, cursor positioning, and scrolling logic.
- `update_script_state`: Updates script execution timers, decrements counters, and processes opcode-driven state changes.
- `process_script_opcode`: Fetches an opcode from a command stream and updates various state variables, including an index and a pointer to a data structure.
- `execute_script_opcode_2`: Fetches an opcode from a command stream and dispatches a specific logic branch based on its value.
- `execute_script_opcode_stream_2`: Iterates through a command stream by fetching opcodes and executing logic based on a switch-case dispatch.
- `execute_script_opcode_stream`: Iterates through a command stream by fetching opcodes and executing logic based on a switch-case dispatch.
- `process_command_stream_opcodes`: Iterates through a stream of command bytes and executes specific logic based on a switch-case dispatch.
- `dispatch_script_opcode`: Increments a script pointer and fetches the next opcode from a data table.
- `dispatch_script_opcodes`: Iterates through a command stream by fetching opcodes and executing specific logic branches based on the instruction value.
- `parse_command_stream_dispatch`: Iterates through a stream of command bytes and executes specific logic based on a switch-case dispatch.
- `process_event_command_stream`: Parses a stream of command bytes and updates various state structures based on a switch-case dispatch.
- `select_table_entry_and_store`: Computes an index from a global variable, selects one of several fixed tables based on bit tests, reads a word from the chosen table via a pointer and stores it twice to a global variable, then writes a constant byte to the pointer target and returns an input value.
- `execute_script_opcode_stream_3`: Iterates through a command stream by fetching opcodes and executing logic based on a switch-case dispatch.
- `update_object_state_by_packed_data`: Extracts multiple values from a packed 32-bit integer and updates a destination structure based on a type identifier.
- `update_object_state_by_type_id`: Updates a destination structure with specific fields extracted from a source buffer based on a type identifier.
- `update_object_state_by_type`: Updates a destination structure with various data fields extracted from a source buffer based on a type identifier.
- `update_object_state_structure_2`: Populates a destination structure with data extracted from a source buffer based on a comparison value.
- `update_object_state_structure`: Depending on a comparison value, this routine populates a destination structure with data extracted from a source buffer, including bit-shifted bytes and pointer arithmetic based on a table lookup.
- `update_pointer_from_indexed_table`: The routine copies a byte and a constant to a destination structure, updates a field within that structure based on a masked byte from a source structure, and then loads a word from a table using that masked byte as an index.
- `update_indexed_data`: Computes an index from a register, checks a byte at an offset, and if zero, copies two words from a table to a global variable and clears a byte at the indexed location.
- `clear_audio_and_state_variables`: Writes zero to several global variables and memory locations, including audio volume registers and DMA and interrupt control registers, effectively resetting state.
- `initialize_audio_and_script_state`: Clears a set of global variables and audio volume registers, resets DMA and interrupt state, and then loads a block of eight 16-bit entries from a table pointed to by a global variable and a register-derived offset into RAM.
- `compare_and_set_flags`: The routine compares a value in a register with a global variable and updates several global flags based on the comparison result and the value's magnitude.
- `copy_byte_and_clear_words`: The routine copies a byte from one offset to another within an unspecified structure, clears a byte at a different offset, and clears a word pointed to by an uninitialized pointer.
- `initialize_state_from_input`: The routine stores constants and the low word of an input value into global variables, computes an address using a table indexed by the high byte of the input and stores the result into a global variable, and clears two other global variables.
- `update_entry_from_table_2`: Writes several input values and constants into global variables and returns.
- `update_entry_from_table`: The routine selects a base address from a global variable or a table, checks a byte at a computed offset and calls another routine if it matches, otherwise reads bytes from the selected entry, computes an index and updates several global fields with values derived from the entry and a table, using a conditional mask.
- `initialize_state_and_clear_flags`: The routine stores the result of another routine into a global variable, clears two global variables, and calls another routine before returning a concatenated value.
- `set_audio_state_and_call`: The routine obtains a value from another routine, masks and combines it with a constant to store in a global variable, clears another global variable, and calls another routine before returning a concatenated value.
- `configure_timer_and_pointer`: The routine clears a global variable, sets it based on a byte test and a memory byte, writes constants to global variables, and conditionally initializes a pointer and several global values.
- `save_global_and_wait`: The routine stores a constant into a global variable and conditionally calls another routine based on a flag byte and a signed byte.
- `manage_interrupt_state`: The routine checks various bits in a global variable and conditionally calls itself or modifies a global variable based on those bits.
- `check_vpos_and_call`: Tests a global variable range, an input bit, and a flag byte, and if the conditions are met it calls another routine.
- `initialize_engine_state`: Performs a comprehensive initialization of global tables, timers, sprite bounding boxes, and script states.
- `execute_multiple_subroutines`: Performs a sequence of calls to various routines, including one that passes a word value as an argument.
- `clear_message_state`: Sets three specific global words to zero.
- `store_processor_registers`: The routine copies the contents of D1 through D7, A1 through A6, and the value from the stack into successive entries of the pointer passed.
- `decrement_active_timers`: The routine loops over twenty entries, and for each entry marked active it subtracts a high word from a stored integer, marking the entry done.
- `traverse_script_hierarchy`: Iterates through a table of entries to find a match for a specific identifier, then recursively calls another routine to process the resulting script pointer.
- `decrement_counter_and_dispatch_alt`: The routine decrements a byte in a global variable in a loop and calls another routine if a value at a calculated offset is not negative one.
- `update_sprite_bounding_boxes`: Iterates through a list of sprite data blocks to calculate and update their bounding dimensions.
- `update_ui_display_parameters`: Validates a state flag and compares global values before calculating and clamping UI coordinate and scale values from a table.
- `update_sprite_clipping_region`: Calculates and updates the bounding box and memory offsets for a masked sprite region.
- `process_recursive_value`: Checks if a parameter is zero and, if not, calls another routine recursively before calling a third routine with the results.
- `dispatch_indirect_call`: Validates a parameter against a range to index into a table of function pointers for an indirect call, or executes calls via offsets from global pointers.
- `search_and_clear_table_entry`: Iterates through a table to find an entry matching a specific byte and two non-zero status flags.
- `return_null_string_reference`: This empty routine is called in relation to a string constant containing the text NuNU.
- `traverse_linked_list_and_recurse`: Iterates through a linked list starting from a global address, checking bit flags and comparing two word values within each entry.
- `update_state_flags`: Checks a byte within a parameter and, if it meets specific criteria, calls another routine and sets a byte in a global variable.
- `call_handler_twice`: An empty routine that calls another routine twice before returning.
- `return_without_action`: The routine consists solely of a return instruction, performing no work of its own.
- `iterate_flag_check_2`: Loops a short counter from one up to a limit, stores the counter into a global byte, and compares a byte from a table indexed by the counter against a global byte.
- `execute_single_instruction`: This routine performs a single instruction and calls another routine.
- `find_matching_entry_2`: The routine searches backwards through two parallel ranges of global data using a parameter, returning the first matching index or zero when no match is found.
- `update_script_object_index`: Compares a passed index against a global value and checks specific bytes in tables.
- `clear_and_update_script_state`: Clears a fixed number of words in a global buffer and conditionally updates several global values based on comparisons between table entries and global bytes.
- `initialize_global_tables`: The routine clears a global byte, calls several other routines, conditionally calls another routine based on a global short, and zeroes multiple ranges of words in global areas.
- `initialize_game_state_3`: Clears and initializes a large set of global variables and tables, sets several constants, and calls other routines to set up the engine.
- `reset_object_fields_by_index`: The routine uses an index from a parameter to clear and set multiple bytes and words in a large structure referenced via an uninitialized pointer, initializing fields conditionally based on a flag in the parameter.
- `process_script_command_loop`: The routine initializes a global flag, then enters a loop that calls several helper routines to fetch and process script data, copies a limited number of bytes from a global area into a buffer derived from a parameter, stores a fetched value into a global word, and continues looping while invoking further script processing routines.
- `initialize_and_process_node`: Calls another routine with a global offset and a constant argument, then calls another routine with no arguments.
- `initialize_and_process_nodes`: Sets a global byte to one and executes a loop three times calling two other routines.
- `fill_struct_and_process`: Computes an offset from a parameter, writes several fields into a global structure at that offset, derives an address from another parameter and stores it, conditionally adjusts the address, calls another routine to obtain a value and then calls a processing routine with that value and the address, repeats the processing call when a field is non-zero, and finally calls another routine.
- `fill_struct_array_with_params`: The routine loops over a fixed count of entries, computes an address using the loop index and a parameter, and writes the parameter and part of a second parameter into bytes within each entry.
- `find_matching_range_and_store`: The routine initializes a word in a structure to a sentinel, scans three entries for a range containing a parameter value, stores the matching entry index, forces a default index if a flag is set, and returns the stored index.
- `generate_bitmask_table`: Copies a byte from one global location to another and then populates a two hundred fifty-six entry table by performing bitwise operations on a set of values.
- `execute_script_logic`: Calls another routine with a global address and a parameter, calls another routine with a global address, and conditionally calls another routine with a constant if a global word matches a specific value.
- `initialize_pointers_and_counter_2`: The routine stores the address of a structure within the same block into a global variable and writes the constant three into another global.
- `return_without_action_2`: The routine consists of a single return instruction with no body: it performs no work, reads no hardware registers and makes no library calls, so it serves only as an empty placeholder.
- `initialize_engine_subsystems`: Attempts to load multiple resource handles through a global function table and stores the results in a central state structure.
- `update_resource_metadata`: Retrieves data from a resource management system and stores the resulting pointers and values into a structured data block.
- `accumulate_and_check_limit`: The routine reads a global word, adds a parameter-derived value to two global words, conditionally calls another routine when a global word is less than another global word, and returns the originally read value.
- `increment_counter_and_advance_pointer`: Increments a short counter stored in a global variable and, if the counter exceeds a threshold, calls another routine with a fixed offset argument.
- `initialize_adjacency_matrix`: Initializes a square matrix by iterating through a nested loop to set diagonal elements to zero and non-diagonal elements based on a comparison check.
- `process_linked_list_data`: Iterates through a linked list of structures to extract and transform data into a temporary buffer.
- `update_collision_mask_from_coordinates`: Iterates through a list of coordinate structures to update a bitmask or byte array based on their values.
- `process_coordinate_list_bounds`: Iterates through a linked list of coordinate structures to calculate minimum and maximum values.
- `process_coordinate_list_data`: Iterates through a linked list of coordinate structures to extract and transform data into a temporary buffer.
- `execute_dijkstra_pathfinding`: Implements a shortest-path algorithm by iteratively updating edge weights and distances within a linked list of nodes.
- `update_pathfinding_nodes`: Iterates through a linked list of nodes to calculate edge weights and update shortest distance values.
- `compute_dijkstra_shortest_path_2`: Implements Dijkstra's algorithm to calculate the shortest path between nodes in a graph.
- `compute_dijkstra_shortest_path`: Implements Dijkstra's algorithm to find the shortest path between nodes in a graph.
- `sort_linked_list_nodes`: Traverses a linked list of nodes to calculate values based on table lookups and reorders the list elements.
- `update_priority_queue_nodes`: Iterates through a linked list to find the node with the minimum value and updates the priority values of its neighbors.
- `reorder_priority_linked_list`: Iterates through a linked list to find the minimum priority value and reorders the list elements based on these values.
- `update_sorted_linked_list`: Iterates through a linked list to compare values and updates a sorted structure by reordering pointers and adjusting priority values.
- `update_priority_queue`: This routine implements a priority queue management system, likely a heap or a similar sorted structure.
- `update_priority_queue_values`: Iterates through a linked list to calculate new priority values based on a combination of data from a table and a global variable.
- `sort_linked_list_elements_2`: Iterates through a linked list to compare and reorder elements based on values retrieved from a table.
- `process_linked_list_elements`: Iterates through a linked list to calculate and store values for each node based on an offset and a global value.
- `sort_linked_list_elements`: Iterates through a linked list to find and sort elements based on a calculated value derived from a table.
- `reorder_linked_list_elements`: Iterates through a linked list to compare adjacent elements and reorder them based on values retrieved from a table.
- `update_coordinate_history_bounds_2`: Compares a new set of coordinate values against a stored history and updates the minimum and maximum bounds of those coordinates.
- `update_coordinate_history_bounds`: Compares a new set of coordinate values against a stored history and updates the minimum and maximum bounds of those coordinates.
- `validate_coordinate_sequence`: Compares a set of recently stored coordinate values against a new set of coordinates.
- `update_coordinate_bounds`: Compares a pair of short integers against existing minimum and maximum boundary values stored in a structure and a global table.
- `recalculate_bounding_box_extents`: Updates the minimum and maximum coordinate extents of a bounding box by comparing current coordinate values against stored historical bounds.
- `update_bounding_box_extents`: Updates the minimum and maximum coordinate extents by comparing current values against a history of coordinate points.
- `update_bounding_box_from_history`: Calculates the minimum and maximum coordinate extents by iterating through a series of circular coordinate history buffers.
- `update_coordinate_history_buffers`: Shifts values through several circular buffers and updates a history of coordinates.
- `shift_coordinate_history_buffers`: Shifts a series of stored coordinate values through a circular or linear buffer by moving existing entries to subsequent memory locations.
- `initialize_global_state`: Writes a constant offset to a global word, clears a global word, and obtains a pointer from another routine to clear the pointed-to data and the following word.
- `insert_linked_node`: Inserts a new node into a linked list by updating pointers within the chain based on a provided value.
- `update_linked_list`: Updates a linked structure by inserting a new node and updating pointers within the chain.
- `update_pointer_chain`: Reads a value from a global word, stores a result from another routine through a pointer, updates a linked structure by copying a pointer and writing a byte from a global word, and returns a value from a global word.
- `update_linked_structure_pointers`: Checks values at specific offsets within two structures to conditionally update their internal pointers.
- `update_conditional_pointers`: Checks values at specific offsets within a structure to determine which memory locations to update with new pointer or data values.
- `update_pointer_conditional`: Stores a value at an offset from a pointer if a specific target value is zero, otherwise stores it at a different offset.
- `check_pointer_validity`: Stores a value at an offset from a pointer and returns the pointer if its target value is zero, otherwise returns zero.
- `return_immediately_3`: The routine executes four instructions and returns without performing any observable work or modifying any data.
- `copy_table_words`: Computes an offset into a table using a global variable and a parameter, then copies eight words from that table into a series of global memory locations.
- `write_constant_words`: Stores two constant 32-bit words into a memory location relative to a pointer.
- `update_flags_and_return_value`: Performs two short loops that read and modify two global words using bit tests on bytes within them, calls another routine, and returns a short value from a data register.
- `clamp_and_step_position`: Clamps a word in a global variable to a range defined by two other words, aligning the value to an 8-byte boundary and stepping it by 8 when a flag is clear, otherwise snapping it.
- `clamp_and_compute_bounds`: The routine ensures a word in a global variable is at least a constant, otherwise it adjusts it using another word from a global variable, then derives two bounds from the word by shifting and adding constants and stores them in two global words.
- `clamp_and_propagate_value`: Computes the absolute difference between a global word and a parameter word, stores the parameter into two global words, clamps the global word between two other global words, copies it to another global word and conditionally calls another routine, then conditionally calls another routine.
- `search_linked_list_for_bytes`: Initializes two global words based on a flag parameter using another routine that builds a long from bytes, then walks a linked structure by repeatedly updating global pointers and comparing two bytes from the parameter against bytes in the structure, returning the matching node pointer or zero.
- `initialize_script_state`: Copies a global byte to another global byte, checks a global byte flag and conditionally updates script-related bytes and calls other routines, iterates over a range of indices calling a routine when a table byte is non-zero, fills a range of bytes with sequential values, stores a parameter word into a global word, sets a global byte flag, and performs additional global word and byte stores with conditional updates.
- `update_script_state_and_flags`: The routine manipulates bytes and words in global variables based on its parameter, conditionally updates a byte via a table lookup, clears or preserves several global fields, calls three helper routines, and loops over a count from a global word to increment bytes in a global area under conditions.
- `initialize_script_data_structures`: Clears two global words, computes an address from a global variable and a byte-indexed table, and stores it in two global variables.
- `initialize_script_entry_table`: The routine computes an address from a global variable and a byte-indexed table, stores it in a global variable, clears a short counter, then iterates four times calling another routine with a running pointer and increments the counter and pointer when the call returns non-zero.
- `iterate_and_check_flag`: The routine loops over a range of indices and calls another routine for each index where a byte in a table matches a global byte.
- `populate_script_object_table`: Loads a script pointer into a global variable and iterates through a sequence of script entries based on a byte count.
- `dispatch_script_handler_and_reset`: Adjusts an index using a table lookup, ensures a script state pointer is set from a global variable if it is zero, writes a flag byte based on a byte from a table, clears a range of words in a global table starting at an offset defined by a global variable, and then calls another routine.
- `set_script_state_if_uninitialized`: Checks a condition involving a parameter and a table entry; if the condition holds it calls another routine and writes a value from a global variable into a table entry indexed by the parameter.
- `initialize_script_slot_if_empty`: Tests a table entry indexed by a parameter; if the entry is zero it calls another routine and stores a value from a global variable into that table entry.
- `assign_script_pointer_if_missing`: Checks a table entry indexed by a parameter; if the entry is zero it calls another routine and writes a value from a global variable into that table entry.
- `update_script_entry_with_flag`: Conditionally calls another routine twice based on two global words, and if a global pointer is non-zero obtains a byte from another routine and writes several indexed bytes and words into tables derived from a global base, clears three indexed bytes, then calls another routine.
- `update_script_entry_with_flag_2`: Conditionally calls another routine twice based on two global words, and if a different global pointer is non-zero obtains a byte from another routine and writes several indexed bytes and words into tables derived from a global base, clears three indexed bytes, then calls another routine.
- `copy_string_or_dispatch`: The routine copies a null-terminated string from a parameter into a buffer in a global area, or dispatches to another routine depending on two global flags.
- `process_script_command`: Compares a parameter against a global short and updates it, then clears a global word.
- `clear_array_elements`: If a condition is met, it iterates through an array and sets elements to zero if they do not equal a specific value.
- `update_pointer_table_2`: The routine iterates through a loop based on a returned value, accessing a table via a global pointer and either clearing an entry or updating it with a value from another routine.
- `increment_counter_and_dispatch`: The routine increments a short counter in a global variable and branches on its value.
- `load_resource_descriptors`: This routine reads multiple blocks of data from a source using a running accumulator to track offsets, decrypting or copying values into a series of global descriptors.
- `load_script_resources`: Reads multiple data blocks from a source by decrypting and copying bytes into specific memory locations.
- `zero_multiple_tables`: The routine zeroes several groups of words and bytes referenced through global pointers, using counts from global variables, and also clears two fixed-size byte areas in global memory.
- `dispatch_script_command_logic`: This routine acts as a central dispatcher that executes various script-related operations by calling subroutines to update global shorts, manage script states, manipulate counters, or execute conditional sequences of subroutines.
- `process_script_state`: The routine reads and updates several global words and bytes, calls other routines with parameters derived from those globals, and returns a flag.
- `fetch_global_word`: Returns a word stored at a fixed global offset.
- `decrypt_four_bytes`: Calls another routine to decrypt four bytes from a global pointer, stores a flag based on the result, and returns the four bytes xored with a global byte.
- `process_pending_block_request`: Checks a global variable for a non-zero value and calls another routine to process it before clearing that variable.
- `decrypt_two_bytes`: Calls another routine to decrypt two bytes from a global pointer, stores a flag based on the result, and returns the two bytes xored with a global byte.
- `update_conditional_flag`: Checks a global byte and a character at a specific offset, then updates a global byte and increments a value in a table if certain conditions are met.
- `get_byte_at_pointer`: Returns the byte stored at the memory location pointed to by the provided parameter.
- `clear_global_word`: Sets a word at a fixed global offset to zero.
- `populate_interpolated_buffer`: Calculates a sequence of two hundred byte values using linear interpolation between two points and stores them in a destination buffer.
- `initialize_sequential_tables`: Fills two separate memory areas with sequential short values starting from zero.
- `reset_script_state_2`: Clears various flags and counters in a set of tables and updates a global byte based on a specific condition.
- `decrement_indexed_counters`: Decrements a global byte and uses the new value to index into a table, which then decrements a byte at a location determined by the table value.
- `xor_and_set_flag`: Calls another routine, stores a flag derived from whether the result is equal to a specific value into a global variable, and returns the result of a byte xored with the returned value.
- `process_script_state_flags`: Clears a global byte and conditionally calls another routine based on two global shorts and a global byte.
- `set_script_object_field`: The routine clears a global word, computes an index from a parameter and a global short, sets a byte in a table at that index, writes the index and two other globals, and calls another routine with an address built from globals.
- `update_script_object_state`: The routine updates fields of a script object stored in a global variable, computes offsets from several object fields and writes them to other object fields, and conditionally iterates over a count stored in a global variable calling another routine for each entry and updating the object fields with the returned values.
- `decode_script_byte`: Reads a byte from a pointer stored in a global variable, increments that pointer, and subtracts ten from the byte.
- `set_byte_zero`: Sets a single byte at a specific location in global memory to zero.
- `decompress_bitstream_to_buffer`: Iterates through a compressed input stream to expand data into a destination buffer using a lookup table.
- `expand_bitstream_data`: Reads a compressed bitstream from a source buffer and expands it into a destination buffer using a lookup table.
- `decode_compressed_data_stream`: Decompresses a bitstream by reading control bytes and expanding them into a destination buffer using a lookup table.
- `clear_global_flags`: Resets several global bytes and words, and conditionally updates a global byte if a value in a table matches a specific byte.
- `initialize_handler_state_3`: Calls another routine with a global word, sets a global byte to one, clears a forty-byte block of memory, and writes a constant.
- `decompress_data_stream`: Decompresses a stream of data using a variable-length encoding scheme and writes the resulting bytes into a destination buffer.
- `copy_bytes_with_stride_2`: Copies bytes from a source pointer to a destination pointer using a fixed stride for each byte.
- `reset_handler_state`: Initializes a handler by calling another routine with a value from a table, setting a global byte to one, and zeroing out a forty-byte memory block.
- `set_byte_zero_alt`: Writes a zero byte to a specific offset within a global memory area.
- `store_byte_via_index_and_recurse`: The routine reads a byte from a global variable, uses it as an index to store a byte from the first argument into a table located via a global base, increments the index byte and writes it back, and if the index exceeds a threshold it calls another routine.
- `copy_palette_entries`: If a byte in global memory is non-zero, the routine copies a number of bytes from a table indexed by words in global memory into a destination area in global memory.
- `find_matching_index_in_table`: Retrieves a value from a table using a shifted input parameter and iterates through a series of tables to find an index where a bitwise mask matches, a specific bit is clear, and a value falls within a defined range.
- `search_table_for_value`: Iterates through a table to find an index where a masked value matches a specific condition and a secondary range check passes.
- `search_table_for_index`: Iterates through a table to find an index where a masked value matches a condition and a secondary range check passes.
- `find_matching_coordinate`: Iterates through a table to find an entry where a specific bit is cleared and a secondary value is zero, then performs range checks against provided parameters.
- `search_coordinate_ranges`: Iterates through a table by incrementing a global index to find an entry where a specific bit is cleared and a secondary value is zero.
- `search_table_for_coordinate_range`: Iterates through a table by incrementing a global index to find an entry where a specific bit is cleared and a secondary value is zero.
- `find_matching_index`: Increments a byte value in a global variable to search through a table for an entry that satisfies multiple bitmask and range constraints.
- `decrement_and_process_byte`: The routine copies a byte from a global variable to another global byte and then repeatedly decrements that byte while it is non-zero.
- `iterate_and_clear_counter`: The routine checks a global counter byte and, if it is non-zero, iterates from zero up to that count.
- `iterate_and_update_script_object`: The routine conditionally clears a parameter byte, reads several bytes from tables indexed by a global byte, checks range conditions, loops over a range updating a script object fields and calling another routine for each matching index.
- `copy_script_data_to_buffer`: The routine calculates a starting address from a table and a global, then copies bytes from specific memory locations into a destination buffer, followed by a loop that copies remaining bytes from a source pointer.
- `copy_bytes_with_pointer`: The routine repeatedly copies a byte from a source pointer to a destination pointer, advances both pointers, updates a pointer in global memory, and repeats for a count given by registers.
- `process_range_and_update_tables`: The routine saves and restores a global word, reads a short from a table, loops over a byte count updating a script object fields and calling another routine, then updates several short tables with computed sums.
- `execute_conditional_logic_3`: The routine performs conditional checks and recursive calls based on a parameter and global values, returning a status flag.
- `execute_conditional_logic_2`: Performs a series of subroutine calls and conditional checks based on global values to return a status flag.
- `execute_conditional_logic`: The routine performs several calls and checks a short value in a global variable to decide whether to call another routine with a calculated parameter or call two routines and return a boolean value.
- `execute_conditional_calls`: Calls another routine with a calculated parameter if a specific value is zero, otherwise calls another routine without parameters, and then calls another routine again before returning a constant.
- `call_subroutines_and_return_one`: Calls two instances of another routine and returns the value one.
- `validate_resource_integrity_2`: Iterates through multiple data tables and memory buffers to check for zero values, specific bit patterns, or non-null characters.
- `validate_resource_integrity`: Iterates through multiple tables and memory buffers to verify data consistency and non-zero values.
- `verify_resource_data_integrity`: Iterates through multiple data tables and memory buffers to check for zero values, specific bit patterns, or non-null characters.
- `validate_asset_integrity`: Iterates through multiple data tables to verify the presence and validity of specific byte values and non-zero entries.
- `validate_asset_integrity_tables`: Iterates through multiple data tables to verify the presence and validity of specific byte values and non-zero entries.
- `execute_conditional_logic_loops`: Performs multiple loops that check specific bit flags and byte values in a table to decide whether to call another routine with specific parameters.
- `process_nested_conditional_loops`: Iterates through multiple ranges defined by a table and global variables, calling another routine with varying arguments based on byte comparisons and conditional checks.
- `process_conditional_loops`: The routine iterates through a range defined by a global variable, performing conditional calls, and then executes nested loops that check memory and call other routines based on a global flag.
- `iterate_and_process_data`: The routine loops through a range of values, conditionally calling another routine multiple times based on memory contents, and then executes a secondary loop that increments a counter while calling another routine.
- `iterate_with_conditional_calls`: The routine loops a counter from zero upwards, repeatedly calling another routine and passing the counter, then calling another routine twice, and fetching a byte from another routine.
- `reset_and_process_nodes`: The routine clears several byte arrays using counts from global variables, zeroes a 200-byte block, then iterates over two fixed ranges of tables.
- `select_behavior_by_byte`: Selects behavior based on a byte from a global variable, calling another routine with a parameter derived from the input or updating a global variable and returning a computed value.
- `initialize_game_state_2`: Writes a byte from a parameter into a global variable and then repeatedly calls another routine with a series of fixed offsets added to a base pointer to initialize many fields in a global data structure.
- `process_xor_word_and_copy_2`: Performs an XOR operation on a 32-bit value using a global byte, then conditionally calls another routine to copy a byte and updates a global counter and a flag.
- `fetch_table_pointer`: Calls another routine and returns a pointer retrieved from a table addressed by a global variable.
- `copy_bytes_in_chunks`: Loops while a counter is positive, limiting each iteration to a maximum size, and calls another routine to copy data.
- `call_reset_structure_and_seek_2`: Calls another routine using a pointer retrieved from a global variable and a passed parameter.
- `process_xor_byte_and_copy_2`: Performs an XOR operation on bytes from a parameter using a global byte, then conditionally calls another routine to copy a byte and updates a global counter and a flag.
- `get_conditional_byte`: Checks a global flag and a bit in a table entry to conditionally return a byte from a table.
- `store_table_byte`: Calls another routine and stores a byte into a table at an offset derived from a parameter.
- `reset_state_and_mask_table_2`: Clears two global words and a global byte before calling another routine.
- `call_conditional_handler`: Checks a bit in a table entry and calls another routine if the bit is clear.
- `dispatch_script_data`: Reads a byte to index a table of pointers, then iterates through a data block to perform operations based on a flag.
- `execute_pointer_call_2`: Calls another routine using a pointer retrieved from a global variable.
- `read_long_value`: Reads four consecutive bytes from a memory location and combines them into a single 32-bit integer.
- `xor_buffer_in_chunks`: Iterates through a buffer in chunks of up to sixteen thousand three hundred eighty-four bytes, decrypting each chunk using another routine and a global byte.
- `update_flag_and_counter`: Calls another routine, and if the result is zero, calls another routine, stores a flag based on a comparison into a global variable, and increments a global variable.
- `copy_c_string_2`: The routine copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers until the terminating zero byte is written.
- `process_sprite_range_and_copy`: The routine calls another routine to copy a null-terminated string from a parameter into a stack buffer, computes a short value from two globals, calls another routine with that value and a constant pair, and calls another routine with a derived offset.
- `copy_object_attribute_data`: Copies a sequence of bytes from a source table into a series of global memory locations.
- `fill_script_buffers`: The routine loops a fixed number of times, writes a byte into a global byte, calls another routine, and copies a null-terminated string from a global area into a buffer in a global variable.
- `store_rectangle_and_metadata`: The routine normalizes two coordinate values using a fourth parameter, then stores rectangle bounds and several metadata bytes and words into multiple global arrays indexed by a parameter.
- `clear_object_table_and_draw`: Clears a small table of word entries to a sentinel value and then, depending on a flag stored in a global variable, calls another routine multiple times with coordinates derived from a global position and size values together with several metadata bytes and words loaded from global variables.
- `update_display_state`: The routine computes a value from two global words and calls another routine, then calls another routine with a range of values.
- `initialize_script_entry_2`: Reads a short entry from a table indexed by a parameter and, if it is not -1, loads related shorts and a flag byte from several tables.
- `find_containing_rectangle`: The routine searches a fixed set of rectangles stored in global memory for one that contains the point given by a parameter and returns its index or a sentinel value if none matches.
- `initialize_script_entry`: It calls update_script_entry_2 and initializes a script buffer pointer, updates several fields in a global entry indexed by the parameter, and scans a null-terminated value.
- `adjust_coordinate_differences`: The routine makes two short fields of the second parameter absolute by subtracting from the first parameter, then calls another routine four times with combinations of those fields.
- `invoke_indirect_handler_3`: An empty routine that calls another routine indirectly.
- `handle_recursive_data_2`: The routine calls itself recursively with an offset if a byte in a parameter is equal to one or two, then stores the result into a global variable.
- `filter_sort_and_update_object_state`: Scans a table for entries matching a specific byte, sorts the matching indices based on a secondary value, and then iterates through the sorted list to update a set of global state variables and properties from various data tables.
- `call_indirect_pointer`: An empty routine that calls another routine through a pointer stored in a global variable.
- `copy_object_data_from_table_2`: Computes a pointer into a table using an index and copies bytes from that entry into several global variables, including a loop that populates a sequence of bytes.
- `propagate_flags_by_mask_2`: Iterates over a table of words and, if a masked value is non-zero, sets specific bytes in a global array based on a comparison with another table.
- `reset_state_and_dispatch_2`: It checks if a word in a global structure is non-zero, clears it, and recursively calls itself.
- `poll_status_register_2`: The routine repeatedly executes another routine until it returns a non-zero value.
- `reset_structure_and_clear_pointer`: Checks a pointer stored in a global variable and if non-zero calls another routine with that pointer, then clears the global pointer.
- `forward_parameter_to_helper_2`: Passes its single input parameter directly to another routine and returns.
- `process_single_node`: The routine computes an offset from the parameter, checks a table entry for non-zero, and if present calls another routine with that entry and then clears the table entry.
- `process_script_nodes`: It computes an offset from the parameter, checks a table entry for non-zero and calls another routine with that entry, and clears a global byte while repeatedly calling another routine while comparing a parameter with a returned value and checking a return.
- `search_script_nodes_with_flag`: Clears a global byte and repeatedly calls another routine while comparing a parameter with a returned value and checking a returned short.
- `select_max_priority_node`: The routine scans a list of nodes up to a count from a global variable, finds the node with the highest priority byte after masking, stores the selected node in a global variable, clears the node and its flag, and calls another routine with the stored node.
- `find_zero_flag_node`: The routine walks a list of nodes up to a count from a global variable, selects the first node whose flag byte is zero after masking and whose identifier differs from a table value, stores the node in a global variable, clears the node, and calls another routine with it.
- `find_zero_flag_node_alt`: The routine walks a list of nodes stored at a fixed offset, selects the first node whose flag byte is zero after masking, stores the node in a global variable, clears the node, and calls another routine with it.
- `find_zero_flag_node_alt2`: The routine walks a list of nodes up to a count from a global variable, selects the first node whose flag byte is zero after masking, stores the node in a global variable, clears the node, and calls another routine with it.
- `dispatch_node_selection`: Copies a byte from a global variable to another, then loops incrementing a global byte and dispatching to one of four node-selection helpers based on its value.
- `process_script_entries`: The routine calls another routine with two global pointers, then loops a number of times taken from a global short, calling another routine each iteration with a global pointer and a global pointer, and finally calls a routine with a global pointer and a constant.
- `rotate_bytes`: Takes a 32-bit value and returns it with its four bytes rotated to new positions.
- `register_script_command_pointers`: The routine walks a list of script command entries and stores the address of each entry into several global tables indexed by a byte from the entry and by values read from global variables.
- `process_script_command_list`: The routine processes a script command list pointed to by a global variable, iterating over its entries and invoking another routine with derived parameters for each entry.
- `parse_script_bytecode_2`: Iterates through a buffer of script instructions, using a switch-case or jump table to dispatch and execute different instruction types.
- `find_maximum_chain_sum`: Walks a list of linked structures using a global pointer and a global count, calling another routine on each node's second word and accumulating the sum per chain to return the maximum value.
- `insert_node_with_byte_rotation`: Computes a rotated value from a node field and decides whether to insert a new node before or after an existing node based on a distance check, updating pointers and storing rotated values.
- `copy_words`: Copies a block of words from a source pointer to a destination pointer.
- `search_range_for_flagged_node`: Scans a range of nodes bounded by a base address and a limit, using a rotated value from each node to step through the range and checking a global table for a specific flag.
- `process_node_chain`: Walks a linked list of nodes bounded by a limit and updates each node's fields using several helper routines, then returns the final pointer.
- `process_node_chain_2`: Builds a chain of nodes from global data and walks the chain to find a node matching a parameter.
- `insert_node_with_byte_rotation_2`: Ensures a parameter is at least eight, aligns it down to an eight-byte boundary, then walks a linked list of nodes from a global pointer to find an insertion point.
- `link_node_and_update_neighbors`: Updates predecessor and successor links, merges or adjusts neighbor link values using rotated sums, and returns a rotated value derived from a predecessor.
- `collect_and_sort_fixed_point_samples`: Allocates a temporary buffer to collect fixed-point samples and associated pointers from a global source, sorts the collected data using a bubble sort, builds a linked list from the sorted pointers, and calculates a sum of the sample and pointer values into an output array.
- `execute_callback_list`: Iterates a number of times determined by a global variable and executes a function pointer stored in a global variable during each iteration.
- `divide_signed_fixed_point`: Performs signed division on two fixed-point values by normalizing the divisor and calculating the quotient and remainder.
- `parse_linked_list_command`: Traverses a linked list to find a specific node, allocates memory if the node is not found, and then calls another routine to parse a command.
- `invoke_with_parameters`: Stores a parameter and a zero value into global variables before calling another routine and returning a value from a global variable.
- `calculate_string_length_2`: Calculates the length of a null-terminated byte string by advancing a pointer until a zero byte is encountered.
- `evaluate_conditional_structure`: The routine conditionally obtains a value from another routine using a derived index, then calls another routine with fields from a structure.
- `compute_fixed_point_product_2`: Calculates the product of two 32-bit fixed-point values using their high and low words.
- `copy_c_string_4`: Copies bytes from a source pointer to a destination pointer until a null byte is encountered.
- `clear_state_and_fetch_value`: The routine clears a global word, loops calling another routine until a global word becomes non-zero, optionally calls another routine and captures its result, then returns a combined value formed from that result and a global word.
- `update_script_state_and_dispatch`: The routine updates several global words based on other globals, clamps a computed offset, and returns early if a script counter is zero.
- `increment_counter_and_copy_word`: Reads a short from a global variable, increments it, and if the previous value was zero sets a byte flag, calls another routine, and copies a word from one global location to another.
- `decrement_counter_and_copy_word`: Reads a short from a global variable, decrements it, and if the result is zero or the original was less than one clears a word and a byte flag, calls another routine, and copies a word between two global locations.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source address derived from a table and a parameter into a fixed buffer in a global variable, then writes a terminating zero byte.
- `initialize_script_object`: The routine checks a global flag and branches to set up script object globals, writes several constants into global words, updates a global entry indexed by a parameter, and loops calling another routine while a global word is zero.
- `handle_script_string`: The routine measures the length of a null-terminated string in a global buffer, clears its last byte, and calls another routine with the buffer pointer.
- `process_script_string`: The routine measures the length of a null-terminated string in a global buffer, clears its last byte, and calls another routine with the buffer pointer.
- `copy_string_and_execute_callback_2`: Copies a null-terminated string into a global buffer and then executes a function pointer retrieved from a global structure.
- `clear_flag_conditionally_2`: It conditionally calls another routine if a global byte is non-zero, then stores zero into that global byte and returns.
- `return_no_operation_2`: The routine consists of a single return instruction with no operations before it, so it performs no work and simply returns control to its caller, decrement_counter_and_copy_word.
- `update_and_return_word`: Clears a global word, calls another routine and stores its result in a global word, then calls another routine and returns the stored value.
- `dequeue_message_index_2`: Compares two global short values and, if they differ, retrieves a value from a table using the first short as an index, clears that table entry, and increments the first short value.
- `jump_to_return_immediately_2`: Jump stub: passes control straight on to return_immediately_2.
- `dispatch_script_command_2`: This routine performs the same logic as the dispatch script command routine, processing command values based on a global short and updating various script-related globals.
- `return_no_operation`: The routine consists of a single return instruction and performs no work.
- `fill_buffer_from_value_2`: stores a byte value into a memory buffer for a specified number of repetitions.
- `setup_script_state`: The routine initializes several global values, updates a pointer, and conditionally executes different branches based on a parameter value.
- `clear_global_byte`: Stores a zero byte into a specific offset within a global structure.
- `return_no_operation_3`: The routine consists of a single return instruction and performs no work, so it acts as an empty placeholder with no side effects.
- `initialize_format_state_2`: Clears a 16-bit global variable, stores a 32-bit parameter into a global variable, and calls another routine.
- `calculate_step_increments`: Calculates the absolute differences between two sets of coordinates and loops to accumulate sums and conditionally call another routine.
- `update_handler_parameter`: Sets two global short values using an input parameter, treating negative one as zero, and calls another routine if the resulting value is zero.
- `set_first_element_and_return_concat_2`: stores the first word of a pointer into a specific offset of a table and returns a combined value containing a modified integer and a word from the second argument.
- `set_flag_and_branch_2`: It calls another routine, then branches based on a global byte to call a different set of other routines and sets that global byte to one.
- `reset_script_state`: Clears a fixed range of bytes in a global area, sets a global byte to 255, then scans a range of flags and for matching entries sets a global index and calls three helper routines.
- `fetch_byte_from_pointer`: Loads a byte pointer from a global variable, increments the stored pointer value, and returns the byte pointed to by the original pointer.
- `fetch_word_from_pointer`: Loads a byte pointer from a global variable, advances the stored pointer, reads two consecutive bytes from the original address and returns them as a little-endian word.
- `process_byte_stream_loop`: loops while a byte in a global variable is not negative one, fetching bytes from a pointer to update a global byte, copy a word, set a table entry, and execute a function pointer from a table.
- `process_pending_command`: Checks a global byte flag and, if it is not -1, calls three helper routines to update pointers and resolve a script object reference.
- `update_pointer_from_index`: If a byte flag in a global variable is not 255, it loads an indexed pointer from a table using that byte as an index and adds an offset from a global variable, storing the result back into a global pointer variable.
- `store_pointer_delta_to_table`: If a byte flag in a global variable is not 255, it computes the difference between two global integer values and stores the result into a table entry indexed by the flag byte.
- `call_twice`: calls another routine two times in succession.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `resolve_script_object_reference`: Resolves a script object reference by reading a flag byte from a global variable and using an index to look up a script entry.
- `iterate_entries_clear_flag`: Iterates over a fixed range of entries and clears a flag byte when a type byte matches specific values and a condition on a global byte holds.
- `process_matching_entry_by_id`: When a parameter is non-zero, it scans a range of entries for a matching identifier and type, clears related fields and updates a global byte.
- `mark_matching_entries_3`: Iterates over a range of short entries and, when an entry matches a value from a global variable, writes sentinel values to related locations.
- `mark_matching_entries_2`: Iterates over a range of short entries and, when an entry matches a value from a global variable, writes sentinel values to related location.
- `mark_matching_entries`: Iterates over a range of short entries and, when an entry matches a value from a global variable, writes sentinel values to related locations in global memory and marks the entry.
- `search_string_table_for_char`: Iterates through a list of pointers stored in a table, incrementing a global offset to check each string.
- `search_pointer_list`: Iterates through a list of pointers, incrementing a global offset, until it encounters a null pointer or a specific character match.
- `search_string_for_null`: Iterates through a list of pointers stored in a table, incrementing a counter for each entry, until it finds a null pointer or a specific byte value.
- `update_script_entry`: Updates tables indexed by global bytes, stores a byte from a parameter into a global byte, calls three helper routines, then conditionally restores a global index based on table comparisons.
- `insert_script_table_entry`: Checks if a parameter is non-zero and a second parameter is non-zero, then writes several values into a table at an index derived from a third parameter.
- `clear_script_table_entries`: Clears up to sixteen word-sized entries in a table indexed by a global byte.
- `copy_script_table_entries`: Copies up to sixteen words from a source area to a destination area indexed by a global byte.
- `copy_script_entries`: Copies words from a source area to a destination area indexed by global bytes in a loop of up to sixteen iterations.
- `dispatch_script_operations`: This routine consists of 97 instructions and calls five helper routines for assigning script pointers, processing pending commands, matching entries by id, updating script entries, and finding the next non-zero entry.
- `find_next_nonzero_entry`: Scans a byte table starting at index one for the first non-zero entry and returns its index.
- `update_table_and_counters_2`: Iterates through a range to set a bit and increment a byte in a table, increments six bytes in another table, and clears a bit and a byte in another location.
- `update_table_and_counters`: iterates through a range to set a bit and increment a byte in a table, increments six bytes in another table, and clears a bit and a byte in a third location based on a word value.
- `increment_counter_and_clear_flag`: The routine increments a byte in a global variable six times while incrementing a byte in a table indexed by that counter, then reads a word from a global variable and, if it is not a sentinel, clears a bit in a byte in a table and zeroes a byte in another table.
- `decrement_table_values_2`: Decrements bytes in a table while clearing a bit if a counter reaches zero, and decrements six consecutive bytes in another table, clamping at zero.
- `decrement_table_values`: decrements bytes in a table while clearing a bit if a counter reaches zero, and decrements six consecutive bytes in another table, clamping them at zero.
- `decrement_six_bytes`: The routine decrements six consecutive bytes in a table, zeroing each when it would underflow, using a word counter that starts from an input value.
- `conditional_dispatch_call`: Checks a global word and conditionally calls another routine, then returns.
- `resolve_script_value`: Computes a value from a parameter using several conditional branches, table lookups and helper calls, then returns the computed value.
- `modify_script_data_2`: Updates a global table or a byte in memory based on bits in a global word.
- `modify_script_data`: Depending on flags, either stores a word into a table or performs bitwise operations on a byte using a value from a table.
- `modify_byte_with_mask`: Performs a bitwise NOT on a byte and then ANDs the result with another byte.
- `store_script_word`: If a flag bit in a global byte is set, calls a helper routine and stores a register value into a table indexed by global bytes.
- `add_masked_word_and_clear_bit_2`: Adds a value to a global word and clears a specific bit in its low byte based on a condition.
- `add_masked_word_and_clear_bit`: The routine adds a masked word from an input register to a word in a global variable and then clears a bit in the low byte of that global variable.
- `resolve_conditional_value`: Checks a bitmask against a global byte and calls subroutines to return either a byte or a result from another routine.
- `forward_other_routine_result`: The routine calls another routine and returns its result unchanged.
- `conditional_script_calls`: Checks a bitmask against a global byte and conditionally calls another routine once or twice.
- `fetch_word_and_return`: The routine calls another routine and returns without performing further operations.
- `parse_script_string_data_2`: It clears sixteen word-sized entries in a table and then reads a sequence of bytes and words from a source, storing them into a table until a specific byte is encountered.
- `parse_script_string_data`: Clears sixteen word-sized entries in a table and then reads a sequence of bytes and words from a source, storing them into a table until a sentinel value is encountered.
- `fetch_and_store_bytes`: Loads a byte pointer from a global variable, increments the stored pointer value, and returns the byte pointed to by the original pointer while storing results in a table.
- `conditional_script_calls_2`: Checks a bitmask against a global byte and conditionally calls another routine once or twice, performing no other data operation.
- `fetch_script_byte_via_index`: Calls another routine, stores the returned word into a global word, and returns a byte from a table indexed by that word.
- `store_word_and_return_byte`: The routine stores the low word of its parameter into a word in a global variable and returns a byte from a table indexed by that parameter.
- `call_write_script_byte`: Calls a routine that writes a byte from a parameter into a global byte indexed by a global word.
- `write_script_byte`: Calls a helper routine and stores a byte from a parameter into a global byte indexed by a global word.
- `find_first_zero_index_2`: Iterates through a table of short integers starting from a base address plus an offset, incrementing a counter until a zero entry is encountered.
- `search_table_for_zero`: The routine iterates through a table of words to find the first zero entry and returns its index.
- `find_first_zero_index`: Iterates through a table of short integers starting from a base address plus an offset, incrementing a counter until a zero entry is encountered.
- `validate_and_calculate_max_diff`: Checks a series of conditions involving memory comparisons and character equality; if conditions are met, it updates a byte in memory and returns the maximum absolute difference between two pairs of short integers retrieved from memory via another routine.
- `calculate_max_absolute_difference`: The routine calculates the absolute difference between two pairs of values stored at specific memory offsets and returns the larger of the two results.
- `calculate_maximum_absolute_difference`: Calculates the absolute difference between two pairs of values stored at a specific memory offset and returns the larger of the two results.
- `calculate_max_difference`: Calculates the absolute difference between two pairs of values and returns the largest of those two differences.
- `stub_noop_returns_immediately`: This routine performs no operations of its own and immediately returns to its caller.
- `return_immediately_11`: This routine executes five instructions and returns without modifying any registers or memory.
- `return_early`: The routine consists of four instructions and immediately returns without modifying registers or memory.
- `decrement_timers_and_mark_done`: The routine loops over twenty entries, and for each entry marked active it subtracts a high word from a stored integer, marking the entry done and clearing its timer when the value underflows.
- `copy_string_and_dispatch`: Copies a null-terminated string from a pointer into a local buffer and then calls another routine with the buffer and a constant offset, followed by a call with the buffer and a parameter.
- `execute_conditional_subroutine_sequence`: The routine calls another routine with a global address and an argument, then calls another routine with a global address.
- `combine_bytes_to_word`: Takes a pointer to a memory location and returns a 16-bit value formed by concatenating the byte at that address with the subsequent byte.
- `append_string_to_buffer`: The routine finds the end of a destination buffer and copies a source string into it.
- `accumulate_word_to_global`: The routine calls another routine to obtain a word, adds it to a global integer, and stores the sum back to that global location.
- `execute_conditional_callback`: Performs a conditional subroutine call based on a byte value compared against a table and clears specific bits in a memory location.
- `process_conditional_entry`: Checks a byte value against a table and, if a condition is met, performs a conditional subroutine call and clears specific bits in a memory location pointed to by a table entry.
- `clear_matching_entries`: Calls another routine and then stores a constant byte into a global byte.
- `process_conditional_pointer_decrement`: Tests a byte value against several conditions and, depending on the result, either stores the byte into a memory location or decrements a global integer by two and calls itself recursively.
- `decrement_pointer_on_condition`: Decrements a global integer value by two if a specific global short value is non-zero.
- `decrement_on_byte_test`: Tests a byte from a pointer-based location and, if a specific bit condition is met, decrements a global integer by two before calling itself recursively.
- `decrement_pointer_on_bit_test`: Decrements a global integer value by two if a specific bit is set within a byte accessed via a pointer stored in a global short.
- `adjust_global_pointer`: Resolves a value from a subroutine and, if a condition in a table is met, decrements a global integer by two and calls another routine.
- `adjust_pointer_on_mismatch`: Compares a shifted word from a global variable with a divided short from another global location.
- `store_byte_to_global`: The routine calls another routine and then stores a constant byte into a global variable.
- `execute_looping_script_commands`: The routine retrieves a count from a pointer, then loops a specified number of times, performing different actions based on a bit in a global byte and incrementing a global short each iteration.
- `modify_script_data_2_2`: It increments a value at a global location within a loop and conditionally calls other routines based on a bit in a global byte.
- `store_constant_value_entry`: It calls another routine, loads a constant word into a register, and calls another routine with that constant as an argument.
- `resolve_and_dispatch_value`: It calls another routine, loads a word from a global location into a register, and calls another routine with that word as an argument.
- `fetch_and_forward_value`: The routine reads a word from a global variable, calls another routine, and passes the read word to another routine.
- `process_conditional_script_value`: The routine resolves a value and, if that value is non-zero, performs an action using a value from a global table.
- `self_recursive_call`: The routine calls another routine and then returns.
- `load_word_from_offset`: The routine calls another routine, loads a word from an address computed from a parameter plus a constant offset, calls another routine, calls another routine with the loaded word, calls another routine, and returns.
- `read_table_value_and_dispatch`: The routine reads a word from a table via an offset stored in a global variable, then calls another routine with that value and performs additional calls to other routines.
- `read_word_from_table_and_dispatch`: The routine reads a word from a table via an offset, then calls another routine with that value.
- `read_table_word_and_dispatch`: The routine reads a word from a table via a pointer stored in a global variable, then calls another routine with that word.
- `read_table_word_and_dispatch_2`: The routine reads a word from a table using an offset derived from a global variable, then calls another routine with that word.
- `execute_conditional_double_call`: Executes a routine, then executes a second routine and checks its return value to determine which of two subsequent routine calls to perform.
- `execute_double_call`: Calls another routine two times in succession.
- `execute_empty_double_call`: Obtains a word via script value resolution and then performs two successive calls to another routine based on the result.
- `execute_empty_call`: This routine calls another routine twice in succession and then returns.
- `compare_script_values`: Fetches multiple values through subroutines and executes different routines based on a comparison between two of those values.
- `execute_double_call_2`: Calls another routine twice in succession.
- `compare_script_values_inverted`: Fetches multiple values through subroutines and executes different routines based on an inverted comparison of two of those values.
- `call_subroutine_twice`: Calls another routine two times in succession.
- `compare_script_values_alt`: Fetches multiple values through subroutines and executes different routines based on a comparison of two of those values.
- `execute_double_call_3`: Calls another routine two times in succession.
- `execute_script_logic_sequence`: Fetches a sequence of values from a data stream to perform conditional operations and arithmetic updates on global variables.
- `call_subroutine_twice_2`: Triggers another routine two times.
- `compare_result_with_function`: Fetches multiple values through subroutines and compares a result from a specific function against a value to decide which routine to call.
- `call_subroutine_twice_3`: Performs two consecutive calls to another routine.
- `check_equality_of_values`: Fetches multiple values through subroutines and executes different routines based on whether two of those values are equal.
- `execute_double_call_4`: Calls another routine twice in succession.
- `initialize_global_flag`: Stores a constant word in a local variable, calls another routine with no argument, then calls another routine passing the stored constant.
- `empty_routine_2`: The routine is empty; it calls another routine twice and then returns without performing any other data operations.
- `initialize_input_buffer`: The routine clears a 32-byte stack buffer, calls another routine to obtain a 16-bit value, passes that value with the buffer to another routine, and finally calls another routine with the low byte of the value and the buffer.
- `resolve_and_read_script`: Loads a global byte and a word from another routine, calls a routine with a stack buffer, then calls another routine with a combined value derived from the word and a bit test of the byte and a value from the buffer.
- `process_script_command_and_clear`: Calls a routine to obtain a word and fills a stack buffer, computes table pointers from a global byte, conditionally calls another routine, masks a byte at a computed address, clears a word and two bytes, writes a byte to a global byte, then calls another routine with a combined value.
- `execute_double_call_5_2`: The routine calls another routine twice, compares a bit-shifted version of the first result with the second result, and then calls another routine based on that comparison.
- `execute_double_call_5`: Calls another routine twice in succession.
- `execute_conditional_logic_sequence`: Calls a routine to obtain a value, then uses that value as an index into a table to call another routine with the retrieved parameter.
- `call_subroutine_twice_4`: Executes another routine two times.
- `execute_conditional_script_sequence`: Evaluates a bitmask against a global byte to determine which of two subroutines to call.
- `execute_double_call_6`: Calls another routine two times in succession.
- `set_table_bits_4`: Searches a table for a matching word and, upon finding it, sets the high bit of multiple bytes in a secondary table based on a count.
- `set_table_bits_2`: Iterates through a table to find a matching value and then sets the high bit of several bytes in another table based on a count.
- `set_table_bits`: The routine iterates through a table and sets the high bit of each byte based on a count.
- `set_table_bits_3`: Searches a table for a matching value and, upon finding it, sets the high bit of multiple bytes in a secondary table based on a count stored in the first table.
- `conditional_flag_check`: The routine obtains a word from another routine, calls another routine with that word, calls another routine with a constant, calls another routine, and conditionally calls another routine if a byte at an offset from a parameter is non-zero before returning.
- `update_object_state_and_trigger_scripts`: The routine updates two word tables indexed by a byte from a global variable using values from two other global words, then writes those values into two byte tables.
- `execute_script_command`: Fetches a value from a pointer, copies a block of bytes from a source to a destination, and updates several global tables and flags based on the results.
- `copy_bytes_and_schedule_script`: Copies a block of bytes from one memory area to another using a counter, calls another routine to obtain a byte, stores that byte and a flag into a structure, writes pointers into global tables indexed by the byte, updates a flag byte in a table, and calls several helper routines.
- `compare_and_set_flag`: Loads a word from a helper and compares it with a global byte, conditionally calls another routine, calls another routine, and stores a constant byte into a global byte.
- `execute_script_conditional_sequence`: The routine performs a series of conditional checks and calls, potentially updating several global words and bytes, clearing table entries, and setting state flags based on the results of nested routine calls.
- `update_script_state_3`: The routine copies a word from a global variable to a byte field, calls another routine, and conditionally updates script-related words and bytes based on a global word flag.
- `execute_sequence_of_subroutines`: Calls three consecutive subroutines in succession before returning.
- `compact_matching_list_2_2`: The routine searches a list for a matching word, clears the entry, and shifts subsequent non-zero elements forward in the list.
- `compact_matching_list`: Searches a list for a value matching a global short, and if found, clears the entry and shifts subsequent non-zero elements forward in a list to fill the gap.
- `compact_non_zero_list`: The routine iterates through a list, shifting non-zero short values forward and updating associated data in a global structure.
- `compact_matching_list_2`: Searches a list for a matching word, clears the entry, and shifts subsequent non-zero elements forward in the list.
- `combine_flags_and_dispatch`: The routine calls another routine to obtain a word, copies a global word into a global word, calls another routine to obtain a second word, combines the two words with a mask, conditionally calls another routine, and then calls three helper routines with the combined values and a derived byte.
- `backup_and_restore_entity_state_5`: Calculates an index based on a signed byte value to select a specific entry in a table, then copies a set of state parameters from the table to a temporary buffer.
- `backup_and_restore_entity_state`: Saves current entity state data from a table into a temporary buffer and restores it once a data stream parsing operation is completed or interrupted.
- `backup_and_restore_entity_state_2`: Saves a set of entity state parameters from a table into temporary memory and restores them back to the table once a data stream parsing operation is completed or interrupted.
- `backup_and_restore_entity_state_3`: Saves a subset of an entity's current state data to a temporary buffer and later restores it if a specific condition is met.
- `backup_and_restore_entity_state_4`: Saves the current values of an entity state table into a temporary buffer and later restores them if a specific condition is met.
- `update_entity_state_table_4`: Backs up current entity state data to a temporary buffer and then overwrites the table entries with new values parsed from a data stream.
- `parse_script_command_data`: Reads a sequence of command identifiers and parameters from a data stream to update an entity state table.
- `parse_script_command`: Reads a sequence of bytes to parse command identifiers and their associated parameters.
- `update_entity_state_table_3`: Reads a bitmask and updates a structured table of entity properties based on specific flag combinations.
- `update_entity_state_table`: Updates a table of entity properties based on a status byte, performing conditional logic to set or clear flags and store values from parameters into specific offsets of the table.
- `update_entity_state_table_2`: Updates a table of entity properties by reading state values from another routine and writing them to specific offsets based on an index.
- `populate_dialogue_attribute_table`: Parses a sequence of command bytes to fill a structured table with specific attribute values and flags.
- `parse_script_command_2`: Reads a sequence of values from a data stream to populate a command structure in a table.
- `update_script_pointer`: The routine copies a long value from one memory location to another and uses a byte to determine which subroutine to call before restoring the original long value.
- `copy_script_buffer_pointer`: The routine copies a long value from one memory location to another and stores the resulting address into a global pointer.
- `copy_long_word`: Copies a long value from one memory location to another.
- `copy_long_word_2`: Moves a four-byte value from one memory address to another.
- `copy_buffer_pointer`: Stores the address of a local stack buffer into a global pointer and copies a long value from one memory location to another.
- `parse_script_parameter_data`: Reads a sequence of command identifiers and parameters from a data stream to update an entity state table.
- `store_entity_state`: Copies a word from a global location into a global byte.
- `store_script_byte`: The routine obtains a byte from another routine, stores it into a global byte, and then calls another routine.
- `initialize_call_state`: Calls another routine to clear state, loads a constant into a register, obtains a value from another routine, combines it with the constant via another routine, passes the combined result with the earlier value to another routine, and finally calls another routine with no arguments.
- `find_minimum_value`: The routine calls another routine twice, reads a short from a global variable to initialize a loop counter, and repeatedly obtains a short from another routine using the counter, keeping the smallest value seen.
- `set_flag_and_call_handlers`: The routine calls another routine, loads a constant into a register, calls another routine, calls another routine with the constant as argument, then calls another routine.
- `call_two_handlers_with_flag`: The routine calls another routine, loads a constant into a register, calls another routine, and calls another routine with the constant as argument.
- `validate_script_conditions`: The routine checks if a resolved value is non-zero and, if so, performs further checks and potentially calls another routine.
- `call_recursive_routine_2`: This routine calls another routine and then returns.
- `search_script_entry_table`: Iterates through a table to find an entry where a short value, a specific byte flag, and a non-zero byte match given criteria.
- `search_table_for_match`: Iterates through a table of shorts and bytes, checking for specific values and a non-zero character, up to a maximum of nineteen iterations.
- `search_script_table`: The routine iterates through a table, comparing shorts and bytes against specific values until a match is found or a maximum iteration count is reached.
- `store_script_value_to_table`: Reads a byte from a global structure, obtains a word via script value resolution, and stores that word into a table indexed by the byte with a stride of four.
- `build_pointer_delta_table`: Reads a byte from a global variable and calls another routine three times to obtain bytes, combining them into a 32-bit value stored at an indexed location in a global table.
- `set_variable_and_call_routines`: The routine loads a constant word into a local variable, calls another routine with no argument, then calls the same routine passing the variable as an argument.
- `call_reset_routine`: Calls another routine that clears two global words and a global byte.
- `set_word_and_execute_sequence`: Loads a constant word into a local variable, calls another routine with no arguments, and then calls that same routine using the local variable as an argument.
- `store_word_to_global_and_call_helpers`: The routine stores a word from the parameter into an offset of a global variable and then calls three other routines in sequence.
- `initialize_global_flag_2`: The routine stores a constant word in a local variable, calls another routine with no argument, then calls another routine passing the stored constant, and returns.
- `update_script_object_state_2`: Fetches a byte via another routine, stores it into a global byte, calls another routine with a constant byte, then calls another routine to obtain a short.
- `dispatch_conditional_state`: Loads a byte from a helper call into a global byte, calls another routine with a constant value, and if a helper returns a specific value, updates two global words before calling another routine with a table-indexed pointer.
- `store_tables_and_call`: Stores two words and a byte from parameters into a table indexed by a parameter word, updates a flag byte, and conditionally calls another routine.
- `update_spatial_fields`: Compares bytes from a table against a global byte, calculates a value based on a table and a conditional offset, and updates several global fields including a short and two words.
- `update_object_spatial_fields`: Compares a byte from a table with a global byte, and if they match, calculates a new value using a table and updates several global fields including a short and a byte.
- `update_object_fields_2`: The routine fetches a byte, performs arithmetic on a value from a table, updates several global fields including a computed short and a byte, and calls another routine.
- `update_object_fields`: Writes a computed short to a global field, clears a byte in global memory, and copies two words from global memory to other global fields.
- `update_object_state_fields`: The routine calls another routine to obtain a byte and stores it in a global byte, calls another routine with a constant and stores the returned word in a global word, calls another routine and stores the result in another global word, writes a constant byte to a global byte, then calls another routine.
- `load_and_dispatch_script`: Loads a byte and two words from helper calls into global bytes and words, then calls a routine with those values and a table-indexed byte, finally calling another routine.
- `conditional_state_update`: Stores a byte from a helper call into a global byte, conditionally sets a global pointer and calls another routine based on table and global values, stores a byte from a helper into a table entry, and conditionally calls another routine with zero arguments.
- `invoke_handler_chain`: The routine calls another routine, loads a constant into a register, calls another routine and stores its result, calls another routine with the constant and stores its result, calls another routine with the two results, and finally calls another routine.
- `update_indexed_data_tables`: Increments a global byte and uses it as an index to store a value in a table, update two other table entries, and set a flag based on a comparison.
- `update_indexed_table`: The routine writes a byte to a table at an index, calls another routine to update two other table entries, and then sets a byte to zero or one based on a comparison.
- `set_flag_and_clear_byte`: The routine writes one to a byte pointed to by a register and clears a byte in a table indexed by a byte from a global variable.
- `increment_counter_and_update_tables`: The routine increments a byte at an offset derived from a global byte, updates several global bytes and words, conditionally calls another routine, and clears a word and a byte in global tables.
- `decrement_counter_and_restore_state`: The routine decrements a byte at an offset derived from a global byte, copies a word from a table indexed by a global byte into a global word, clears a byte and a word in global tables, and conditionally calls another routine.
- `update_conditional_data`: Checks a returned byte value to either clear a word, a byte, and a global word, or to calculate a difference and store values from two memory locations before calling another routine twice.
- `clear_pointers_and_word`: The routine clears a word pointed to by a register, clears a byte pointed to by another register, and clears a word in a global variable.
- `set_flags_and_call`: Stores a constant byte into a global byte, loads a word from a helper into a global word, calls another routine with a constant word, and stores zero into a global byte.
- `set_flag_and_store_value`: The routine writes a constant byte to a global byte, calls another routine and stores the returned word in a global word, then writes a constant byte to another global byte.
- `check_script_flag`: The routine reads a byte from a global variable, calls another routine twice with a constant, and conditionally calls another routine if the byte changed.
- `double_helper_call`: Calls another routine twice and returns.
- `update_script_entry_3`: Stores a byte from a parameter into a global byte, sets a global byte flag, loads a byte from a table indexed by the parameter and conditionally updates a global word and calls another routine, computes a difference between a table word and a global word and conditionally calls another routine, then iterates over a range updating bytes in a table where a condition matches.
- `execute_conditional_recursion`: The routine calls another routine and then calls another routine based on the returned value.
- `execute_recursive_call`: Performs a single subroutine call to itself before returning.
- `process_input_command_byte`: Fetches a byte from a pointer and branches based on its lower five bits to modify two specific byte counters or trigger recursive calls.
- `copy_bytes_to_words`: The routine copies two individual bytes from a global variable into two separate words in another global variable.
- `call_script_dispatch_sequence`: Calls another routine, loads a constant into a register, calls another routine, then calls another routine with the constant.
- `call_other_routine_with_masked_short`: The routine calls another routine, loads a constant into a short, calls another routine, then calls another routine with the short as argument.
- `process_conditional_script`: Compares a global word against a value and conditionally executes various subroutines based on the result and subsequent byte values.
- `dispatch_script_byte_2`: The routine retrieves a value and, if it is not zero or negative one, loads a byte from a global offset and passes it to two subsequent routines.
- `dispatch_script_byte`: The routine checks if a global value is non-zero, then loads a byte from a global location and passes it to two subsequent routines.
- `load_byte_and_dispatch`: Loads a byte from a table indexed by an offset into a register, calls another routine, then calls another routine with the loaded value.
- `execute_masked_logic`: Evaluates a comparison between a global word and a retrieved value to trigger specific branching logic and subroutine calls.
- `dispatch_masked_byte`: The routine retrieves a value and, if it is not zero or negative one, loads a byte from a global offset and dispatches it via two other routines.
- `dispatch_script_byte_masked`: The routine checks a global value and, if non-zero, loads a byte from a global offset and dispatches it via two other routines.
- `read_masked_byte_and_dispatch`: The routine reads a byte from a global variable, zero-extends it to a word, calls another routine, then calls another routine with the word value.
- `call_other_routine_with_masked_short_2`: The routine calls another routine, loads a constant into a register, calls another routine, then calls another routine with the register value.
- `call_other_routine_with_masked_short_3`: The routine calls another routine, loads a constant into a short, calls another routine, and then calls another routine with the short as argument.
- `call_and_store_mask`: The routine invokes another routine and then masks a global short, branching on the result.
- `evaluate_spatial_state`: The routine retrieves several values, compares a difference between two global shorts against a value from a table, and writes a byte to a table based on the result.
- `evaluate_distance_threshold`: Calculates the absolute difference between a value and a memory entry, then compares this difference against a scaled value from a table.
- `set_conditional_byte`: Compares a value from a table against a register and stores either the value two or three into a memory location pointed to by a register.
- `set_flag_byte_2`: Stores the constant byte value two into a memory location pointed to by a register.
- `update_conditional_tables`: Compares a byte from a table against another byte and conditionally updates various table entries or clears them based on a bitmask result.
- `handle_script_command_2`: Checks a value against constants to conditionally store masked bytes into tables indexed by a global byte.
- `store_masked_byte_to_table`: Checks a specific value and a flag before storing a bit-masked byte into a table at an offset determined by a global byte.
- `set_table_byte`: The routine stores a byte derived from a register into a table at an offset determined by a global byte.
- `return_immediately_12`: The routine performs no functional operations and returns to the caller.
- `conditionally_store_data`: The routine fetches a word and two bytes, then conditionally stores the word and the bytes into different global locations based on the value of the third byte, and finally writes a constant byte to a global variable.
- `conditionally_store_words_and_set_flag`: The routine conditionally stores two words from registers into a global variable when a short register equals one, then writes one to a byte in a global variable.
- `copy_bytes_from_pointer`: The routine repeatedly fetches a byte through a pointer stored in a global variable and writes it into a buffer pointed to by the same global variable, handling a sentinel value by writing three extra bytes and skipping ahead.
- `return_early_2`: The routine is empty and performs no operations.
- `process_conditional_command_byte`: Fetches a byte and evaluates it against a bitmask to execute a series of branching actions.
- `return_immediately_4`: This routine performs no operations and returns to its caller immediately.
- `dispatch_command_byte_3`: The routine fetches a byte and conditionally calls other routines based on whether the byte matches specific constant values.
- `calculate_string_length_3`: Measures the length of a null-terminated string located in a global buffer and clears the final byte of the string.
- `call_sequence_with_constant`: Calls another routine multiple times, passing a specific constant value as an argument during one of those calls.
- `dispatch_conditional_script`: Checks a bitmask against a global byte and conditionally calls another routine once or twice based on the result.
- `fetch_and_store_byte`: The routine fetches a byte from a pointer, stores it in a global variable, and conditionally calls another routine to store a resulting short into two global bytes.
- `recursive_other_routine_call`: The routine consists of a single call to another routine which is the routine itself, so it recurses without any further processing.
- `resolve_conditional_logic`: The routine fetches a byte, stores it in a table, and then performs different sequences of calls and bitwise operations based on the value of that byte.
- `resolve_conditional_byte`: The routine stores a byte from a parameter into a table and then resolves a conditional value based on a bitmask.
- `resolve_conditional_word`: The routine stores a word from a parameter into a table and then resolves a conditional value based on a bitmask.
- `resolve_conditional_word_with_logic`: The routine stores a word from a parameter into a table and then resolves a conditional value using bitwise operations on the result.
- `return_from_subroutine_3`: This routine performs a simple return from a subroutine.
- `process_script_command_stream`: Iterates through a command stream to update the properties of a script object.
- `update_object_state_table`: Iterates through a table of object definitions and modifies their properties based on a command byte.
- `update_script_object_properties`: Iterates through a command stream to modify various attributes of a script object.
- `process_recursive_byte_stream`: Iterates through a byte stream to derive addresses and calls itself recursively to process subsequent data segments.
- `traverse_byte_sequence`: Iterates through a sequence of bytes, calling another routine and updating a register with the value of the current byte until a zero byte is encountered.
- `process_byte_stream_recursive`: Iterates through a sequence of bytes, performing a calculation on each non-zero byte to derive a new address and calling itself recursively.
- `dispatch_instruction_data`: Fetches a byte from a pointer and executes different logic based on the lower five bits of that byte.
- `return_immediately_5`: This routine performs no operations and returns to its caller.
- `process_script_table_updates`: This routine checks a value from a global variable and performs different loop-based operations on a global memory area, such as setting or clearing bytes based on table lookups and conditional checks.
- `update_script_byte`: Checks a condition based on a table lookup and updates a byte in a global memory area with a value from another table.
- `update_script_objects`: The routine iterates through a table and, if a condition is met, updates a global byte using a value from another table and calls other routines.
- `clear_script_byte`: Iterates through a table and, if specific conditions are met through nested routine calls, clears a byte in a global memory area.
- `clear_script_objects`: The routine loops through a table, calls other routines based on returned values, and clears a byte in a global area if certain conditions are met.
- `iterate_script_table`: The routine iterates through a range of indices and calls another routine if a value retrieved from a table is non-zero.
- `iterate_table_with_callback`: The routine loops a number of times determined by a counter in a register and a limit in another register.
- `clear_and_recurse_node`: Extracts the high word of a parameter to index into a table of pointers, then recursively calls another routine with a non-zero entry before clearing that entry to zero.
- `allocate_script_data`: Extracts a high word to index a table of pointers for a recursive call and allocates and copies a sequence of bytes into a new memory block.
- `allocate_and_copy_script_data`: Counts elements in a pointer list and copies a sequence of bytes into a newly allocated memory block.
- `process_script_node_data`: Iterates through a pointer list to count elements, then copies a sequence of bytes from a source buffer into a new memory block.
- `copy_data_from_pointer`: Copies a block of bytes from a source address determined by a pointer to a destination address.
- `copy_data_from_buffer`: Copies a sequence of bytes from one memory buffer to another using a loop.
- `copy_bytes_and_update_counter`: The routine copies a block of bytes from one buffer to another using a loop controlled by a stack value, writes constant values to the destination, conditionally adds the loop count to a field in a global variable, and returns the loop count.
- `dispatch_byte_stream_command`: Reads a byte from a pointer-incrementing stream and executes a specific logic branch based on the lower five bits of that byte.
- `return_from_subroutine`: performs a standard return from a subroutine.
- `execute_script_bytecode`: Interprets a stream of command opcodes by performing various data manipulations, such as storing values into specific tables or updating state variables.
- `execute_script_opcode`: Processes a command byte from a data stream by performing specific operations such as reading values, updating table entries, or incrementing a pointer.
- `parse_script_bytecode`: Iterates through a data stream to interpret command opcodes and execute corresponding actions.
- `initialize_call_state_2`: The routine calls another routine to clear state, loads a constant into a register, obtains a value from another routine, combines it with the constant via another routine, passes the combined result with the earlier value to another routine, and finally calls another routine with no arguments.
- `initialize_call_state_3`: Clears a state via another routine, loads a constant into a register, obtains a value from another routine, combines the constant and the value via another routine, and passes the combined result and the original value to another routine before calling a final routine with no arguments.
- `apply_bitwise_table_operations_2`: The routine reads a value from a stream, updates a global byte, and enters a loop where it performs bitwise XOR or OR operations on table entries based on a secondary value, or clears three consecutive bytes in a table if the secondary value is zero.
- `update_table_values`: Clears three consecutive bytes in a table or performs bitwise XOR or OR operations between a table value and a byte based on a flag.
- `apply_bitwise_mask_to_table`: The routine performs bitwise XOR or OR operations between a value from a table and a byte in another table, based on a bit flag.
- `apply_bitwise_table_operations`: Performs bitwise XOR and OR operations between values in different tables and clears entries in a third table based on a loop.
- `evaluate_conditional_logic_sequence`: Iterates through a sequence to perform bitwise AND operations between two tables and conditionally calls another routine based on the result.
- `evaluate_conditional_logic`: Performs a bitwise AND between two values from different tables and sets a global byte based on the result and a flag.
- `execute_double_call_7`: Calls another routine twice in succession.
- `populate_table_from_stream`: Reads a byte from a pointer and uses it as an index to store a previously fetched byte into a table.
- `store_byte_to_table_and_fetch`: The routine loops reading a byte from another routine and using it as an index.
- `initialize_partial_state`: The routine clears state via another routine, loads a constant into a register, calls another routine, passes the constant to another routine, and returns.
- `search_matching_table_entry`: Iterates through a table to find an entry matching a bitmask and a short value, while also updating a global word and checking conditional values.
- `search_matching_entry`: The routine iterates through a table to find an entry where a specific bitmask matches a value and a short value matches a target, up to a fixed limit.
- `search_table_for_match_2`: Iterates through a table to find an entry where a specific bitmask and a short value match target criteria.
- `execute_empty_loop`: Performs a series of calls and then executes a loop that increments a counter up to a fixed limit.
- `increment_counter_loop`: The routine increments an index in a loop until it reaches a limit, then calls another routine.
- `resolve_script_value_2`: Calls another routine with a constant, then calls another routine with that constant and a returned word, stores the returned byte into a global byte, and performs a series of further calls with the constant and previously obtained values.
- `increment_counter_and_store_word`: The routine reads a byte from a global variable, increments it and writes it back, then stores a word from the parameter into a table indexed by the previous counter value within a global variable.
- `decrement_counter_and_fetch_word`: The routine decrements a byte counter in a global variable, writes the decremented value back, and returns a word from a table indexed by that counter within the same global variable.
- `dispatch_script_commands_2`: Reads bytes from a stream and dispatches them to specific subroutines or function pointers based on their value.
- `execute_command_sequence`: Reads a command byte from a stream and dispatches various actions, including executing function pointers from a table or branching to other routines.
- `execute_command_sequence_type_a`: The routine processes a sequence of commands by reading values and branching to different subroutines or calling a function from a table based on the command type.
- `process_state_machine_logic`: Reads a byte from a global variable and performs conditional branching based on its value.
- `dispatch_command_byte`: Reads a byte from a global variable and performs different actions based on its value.
- `dispatch_command_byte_2`: Reads a byte from a memory location and performs a series of conditional jumps or calls based on its value.
- `execute_script_commands`: Processes a sequence of commands by reading a value and branching to different routines or executing a function pointer from a table based on the value.
- `dispatch_script_commands`: The routine fetches a byte from a stream, updates a global byte, and enters a loop that dispatches function pointers from a table or executes specific sequences of routines based on the value of the byte.
- `call_subroutine_with_offset`: Calls another routine with a constant offset.
- `add_offset_to_global_variable`: The routine adds a constant offset to a value taken from a global variable and passes the result to another routine.
- `validate_state_and_condition`: Compares a passed word against two consecutive global words and checks if a secondary routine returns a non-zero value.
- `decrement_flagged_entries_2_2`: The routine loops over a range defined by a global byte, comparing each byte in a table with a parameter, and if a match is found, clears a flag and decrements a byte in a table.
- `initialize_script_slot`: The routine checks if a table entry is zero and, if so, calls another routine and stores a value from a global variable into that entry.
- `reset_state_and_mask_table_3`: Clears two global words and a global byte before calling another routine.
- `copy_script_data_block_2`: The routine checks a table entry, updates two global words with a parameter, and copies sixteen bytes from a source area to a destination area.
- `increment_index_and_store_byte`: Obtains a word from another routine and combines it with a global byte.
- `clamp_and_propagate_value_2`: Computes the absolute difference between a global word and a parameter word, stores the parameter into two global words, and clamps the global value within a specific range.
- `find_matching_coordinate_2`: Iterates through a table to find an entry where a specific bit is cleared and a secondary value is zero, then performs range checks against coordinates to return a word from a table.
- `build_adjacency_matrix_and_path_data`: Initializes a square adjacency matrix by checking connectivity between nodes and then populates auxiliary tables with path information and node weights.
- `update_sprite_clipping_bounds`: Adjusts and clamps a pair of short range limits within specified boundaries.
- `find_matching_index_in_table_2`: Retrieves a value from a table using a shifted input parameter and iterates through a series of tables to find an index where a bitwise mask matches specific criteria.
- `execute_sequence_with_constant`: Calls another routine multiple times, including one call that passes a specific constant value as an argument.
- `modify_global_flags`: Performs two loops that read and modify two global words using bit tests on bytes within them, calls another routine, and returns a short.
- `process_with_build_long`: Calls a routine that constructs a 32-bit value from four consecutive bytes pointed to by its parameter.
- `fetch_long_from_pointer`: This routine calls another routine to construct a 32-bit integer from four consecutive bytes at a location specified by a parameter.
- `set_buffer_pointer_and_parse`: Stores the address of a local stack buffer into a global pointer, calls another routine with a value from a global word, and writes the returned value back to that global word.
- `setup_script_object_and_parse`: Stores a stack buffer address into a global pointer, initializes script object globals, and parses a byte stream into that buffer.
- `update_script_entry_2`: Initializes a script buffer pointer and updates several fields in a global entry indexed by the parameter.
- `update_state_from_stream`: Updates several global words and bytes based on conditions found in a data stream and handles script command processing.
- `update_global_state_from_stream`: Checks a condition against global bytes and, if met, updates several global words and bytes using values from a data stream.
- `handle_script_command_3`: The routine checks a global byte, copies values from a stream to global variables, updates a pointer, and conditionally resets several global words and bytes based on a value in a table.
- `reset_script_state_3`: Copies a byte from a table into a global variable, clears two global words, sets two global bytes to 255, and calls another routine.
- `update_script_state_2`: Validates a flag and a pair of global words against a global short, then computes two global words from table entries indexed by a global short and clamps their values.
- `clear_flags_and_conditional_copy`: Clears two global bytes and a global byte, checks a global short against a range and a byte table match, and conditionally stores a byte and calls another routine.
- `clear_flags_and_copy_words`: The routine clears two global bytes, conditionally calls another routine with two words from global memory and sets a global word to a sentinel, then copies two word pairs from global memory to other global words.
- `forward_parameter_to_helper`: The routine immediately forwards its parameter to another routine and returns.
- `convert_number_to_ascii_3`: Obtains a short via script value resolution and writes its decimal digits into a pointer-based buffer, handling negative signs by writing a hyphen.
- `parse_script_byte_stream`: The routine walks a byte stream pointed to by its parameter, copying non-control bytes to a buffer via a global pointer and handling control bytes by dispatching to another routine or writing bytes to a global buffer.
- `search_table_and_dispatch`: Obtains a word via script value resolution and searches three byte-indexed tables for a match, then calls another routine with a computed offset or a fixed offset.
- `conditional_dispatch_by_value`: Obtains a short via script value resolution and either calls another routine with a fixed offset if the value is zero, or calls two other routines using values derived from global memory.
- `select_and_update_pointer`: The routine checks a parameter field and either loads a fixed address or adds a constant to a global pointer, then calls another routine with the resulting pointer.
- `read_long_from_pointer`: Reads a 32-bit long integer from a memory location pointed to by a parameter.
- `clear_buffer_and_store_params`: The routine clears a fixed number of bytes in a global buffer, stores two words from the parameters into global words, and conditionally calls another routine.
- `find_and_dispatch_script`: The routine checks a global byte and a global word, then searches a set of tables for a matching entry and dispatches a call based on the found entry, falling back to alternative lookups when the word is large.
- `set_pointer_from_table`: The routine computes an address from a table indexed by the high word of the parameter, adds a constant, stores the result into a global variable, and if the result is zero replaces it with a fixed offset from a global variable.
- `update_global_pointer_2`: Compares a parameter against a value in a global variable and either updates a global variable with a constant or increments it by a value fetched from a table.
- `invoke_lookup_helper`: The routine is empty except for a single call to another routine and an immediate return.
- `lookup_parameter_in_tables`: The routine masks a byte from a global variable with the low word of the parameter, stores the result, then searches two different tables for a match and writes several fields in a global variable before returning a status code.
- `search_and_update_script_state`: The routine clears a fixed number of words in a global buffer, checks a condition using global bytes and words, and conditionally updates several global words and bytes and calls another routine.
- `dispatch_conditional_calls`: Compares a value from a global variable against a parameter and executes a sequence of conditional calls to other routines based on the comparison and their return values.
- `dispatch_conditional_call`: Calls another routine and returns a value based on the result of a comparison against zero.
- `conditionally_execute_and_return_status`: Checks a short value against zero to determine the execution path.
- `call_and_return_zero`: Calls another routine and returns zero.
- `copy_state_from_parameter`: Stores a masked byte from a table indexed by the parameter into a global byte, calls another routine with a word loaded from a table indexed by the parameter, and copies two words from global variables into two other global words.
- `compare_and_load_table_data`: Compares two bytes from different memory locations and, if they are equal, copies two words from two separate tables into a global variable.
- `copy_two_words_from_tables`: The routine copies two words from two tables indexed by a register into two fields of a global variable and returns a constant.
- `handle_script_command`: Calls helper routines, checks a byte-indexed table for a match, updates several global words and bytes, conditionally clears tables and copies data, then stores a byte from a parameter into global memory.
- `decode_script_word_stream`: Reads a 16-bit word stream from a script buffer pointed to by a global pointer, using a helper to build words from bytes.
- `decode_script_and_update_flag`: Compares a byte from a table against a parameter and, if they differ, iterates through a sequence of words to call another routine before updating the table byte.
- `decrement_counter_and_refresh`: The routine checks a global byte and, if non-zero, decrements a byte in a table indexed by it and clears several global bytes and words.
- `initialize_sprite_state`: The routine writes a pointer derived from a global byte and a table into a global word, clears several global bytes and words, conditionally clears a flag, builds a 16-bit value from two bytes via another routine, stores it, and loops over a range updating a global byte and conditionally calling another routine.
- `update_sprite_animation_state`: Reads a sprite index from a global byte to fetch animation data from tables, then updates the current frame, sprite dimensions, and bounding box coordinates.
- `reset_data_block_fields`: Sets a specific word at a calculated offset to zero and then iterates through a memory block, writing the value sixty-five thousand five hundred thirty-five to four specific word offsets within each iteration.
- `clear_word_fields`: The routine repeatedly writes the word value 65535 to four word offsets within a block pointed to by a global variable, advancing the pointer by two bytes each iteration until it reaches a limit held in another register.
- `copy_table_forward`: The routine copies 32 bytes from a source table indexed by a byte in a global variable to a destination table in a global variable, then copies the following word.
- `copy_table_backward`: The routine copies 32 bytes from a destination table in a global variable to a source table indexed by a byte in a global variable, then copies the following word.
- `interpolate_byte_values`: Calculates and stores two hundred interpolated byte values into a destination buffer based on provided start, end, and range parameters.
- `decode_byte_stream`: The routine reads bytes from a pointer stored in a global variable, stores parts of each byte into two global bytes, handles a nibble-zero case by consuming an extra byte, and decrements a counter loaded from a global word, scaled by a parameter, until the counter reaches zero, then shifts a byte right by four bits and returns.
- `update_sprite_bounding_box`: Reads a stream of bytes to update sprite dimensions and calculates new bounding box coordinates.
- `propagate_flags_by_mask`: The routine iterates over 40 entries of a word table indexed by a global word, and for each non-zero masked entry it sets a byte in a global byte array to one when the entry overlaps with any of 13 mask words from a global table and the values differ.
- `update_sprite_bounds`: The routine iterates over a set of blocks using a global counter and updates per-block bounding bytes and a flag field by XORing a global flag with a table entry.
- `update_range_bounds`: The routine checks bounds using a short from a table indexed by a parameter and a global variable, clamps parameters, and conditionally ORs a value into a range of words in a global table and updates two byte fields in a per-entry structure for a range of indices.
- `update_object_spatial_state_2`: Updates an object's coordinate bounds and state flags by comparing current position data against target values.
- `update_object_spatial_state`: Updates an object's coordinate bounds and state flags by comparing current position data against target values.
- `update_object_bounding_box`: Compares a target coordinate pair against a stored rectangle and updates the object's spatial bounds and state flags.
- `update_object_state_2`: Compares global variables against values in a table and, depending on the result, either updates specific table entries or stores new word and byte values into the table.
- `store_object_state`: The routine stores two words from global variables into two global word arrays indexed by a register, stores two bytes from global variables into two global byte arrays indexed by a byte in a global variable, updates a flag byte with a bitwise operation, and copies a byte from one global array to another indexed by the same byte.
- `process_script_state_flags_2`: Reads a byte from a global variable to index several tables in global memory, updating flag bytes and short values based on comparisons of script state data.
- `update_fixed_point_state`: The routine compares two short fields from tables indexed by a global byte with the parameter, and if they differ it computes fixed-point products and divisions using helper routines, stores updated short and int values into several global tables, and returns a value from another routine.
- `update_script_state_flags`: Reads a byte from a global variable to index several tables in global memory, updating flag bytes and short values based on comparisons of signed values and results from other routines.
- `interpolate_table_value`: Copies eight words from a global variable area onto the stack and performs a search to find a valid interval.
- `compute_table_byte_index`: The routine reads a byte from a global variable via a pointer, uses it to index several tables in a global data area and conditionally toggles a flag byte and two boolean flags based on bits from another routine result and a table entry.
- `set_flag_byte`: The routine stores the constant byte value 4 into a byte within a global byte array at an offset derived from the high 16 bits of its parameter.
- `search_best_candidate_with_fallback`: The routine tests a flag in a table entry and, if clear, repeatedly iterates a decreasing counter while calling other routines to evaluate candidates.
- `check_range_overlap`: The routine calls another routine and, if a field of the second parameter is non-zero, computes two intervals from the parameters and compares them against eight short limits stored in a global variable area.
- `find_best_intersection`: The routine initializes a global word to a maximum value, then iteratively calls another routine using various parameters derived from globals and inputs.
- `clip_line_segment`: Computes the intersection of two line segments given by their endpoints and writes the intersection point into a global variable.
- `compute_squared_distance`: The routine computes the absolute differences of the low and high 16-bit components of two arguments, rejects the pair if either difference is 256 or more, otherwise forms a 32-bit result whose low 16 bits hold the sum of the squares of the differences and whose high 16 bits hold the high 16 bits of the square of the first difference, returning all bits set when rejected.
- `test_point_in_rectangle`: Checks whether a point defined by two parameters lies within a rectangle defined by eight short values stored in a global variable and whether four orientation tests on those values succeed.
- `orientation_test`: The routine returns true when the signed 16-bit cross product of the vectors from the first point to the second and to the third is less than or equal to zero, computed as the comparison of two 16-bit products of coordinate differences.
- `add_offset_to_global`: The routine reads two integers from a global variable area and an index derived from a byte and a table pointer, adds them, and stores the result into another global variable location.
- `fetch_pointer_from_table`: The routine calls another routine and returns a pointer from a table addressed by a global variable.
- `fetch_table_byte`: The routine calls another routine and returns a byte from a table addressed by a global variable and an index derived from the parameter.
- `conditional_fetch_table_byte`: The routine checks a flag in a global variable and a bit in a table entry; if the condition holds it calls another routine and returns a byte from a table addressed by a global variable and the parameter, otherwise it returns zero.
- `store_byte_to_table`: The routine calls another routine and stores a byte from the parameter into a table addressed by a global variable and an index derived from the parameter.
- `conditional_call_handler`: The routine checks a bit in a table entry referenced via a global variable; if the bit is clear it calls another routine, otherwise it returns without further action.
- `store_word_to_table`: The routine calls another routine and stores two bytes from the parameter into consecutive bytes of a table addressed by a global variable and an index derived from the parameter.
- `copy_table_words_to_globals`: The routine calls another routine, computes a table base from a global variable and the parameter, then reads eight words from the table via another routine and stores them into several global variables.
- `find_byte_in_table`: The routine calls another routine, selects a pointer based on a global flag, then searches a byte table for a matching range and returns a byte from the table or a default value.
- `iterate_flag_check`: The routine loops a short counter from one up to a limit, stores the counter into a global byte, compares a byte from a table indexed by the counter with a global byte, and conditionally calls another routine.
- `build_and_sort_match_table`: The routine scans a byte table for entries matching a global byte, collects their indices into a temporary table, and sorts that table by a weight table using a bubble sort.
- `copy_table_entries_by_index`: Reads a byte index from a global variable and copies several word and byte values from fixed global locations into tables indexed by that byte, then calls another routine and returns a masked result, updating a byte in a table conditionally based on the low bits of the returned value.
- `update_script_tables`: The routine stores words and bytes from parameters into tables indexed by a parameter word, updates a flag byte, and conditionally calls other routines based on table entries and global bytes.
- `conditional_flag_update_and_increment`: The routine checks a global byte and a byte at an offset derived from a parameter, stores a byte from the parameter into a global byte, calls other routines, increments a byte at a computed address, and clears or sets bytes in global tables.
- `clear_script_state`: Clears per-script flags and counters in a set of tables indexed by a script identifier.
- `reset_script_data`: Sets a byte in a table, conditionally writes a byte to a memory location, and fills a sixteen-byte block with a constant value.
- `fill_memory_block`: Writes a byte value to a memory location and then fills a sixteen-byte block with the value two hundred fifty-five.
- `fill_bytes_with_ff`: The routine loops a fixed number of times, writing the byte value 255 to successive bytes addressed by two registers and a global base.
- `find_matching_entry_and_clear_table`: The routine searches a small range for an entry matching two global bytes and a non-zero byte, writes a constant byte to a global byte, calls another routine, and clears a fixed number of bytes in a global table.
- `copy_bytes_with_fallback`: The routine computes a pointer from a byte-indexed table and stores it into a global variable, then copies sixteen bytes from a source area selected by a byte flag, falling back to another source when the flag is -1.
- `set_script_state_if_uninitialized_2`: The routine checks if a parameter is non-zero and if a specific table entry is zero, then calls another routine and writes a global value to that entry.
- `update_script_object_state_3`: The routine updates a script object's state by reading various values from tables indexed by a parameter.
- `render_text_if_pending`: The routine computes an index from the parameter, checks a global short for a sentinel, clears a global byte, calls another routine with two words from global tables indexed by the computed offset, and sets the checked short to a sentinel.
- `loop_script_updates`: The routine executes a loop one hundred times calling another routine with a zero argument, then calls that same routine once more without an argument.
- `loop_update_script_objects`: Loops a fixed number of times calling another routine with an incrementing register and calls it once more before returning.
- `find_matching_entry`: The routine searches backwards through two parallel ranges of global data using a parameter, returning the first matching index or zero when the search ends.
- `update_script_object_fields_2`: The routine processes a script object index passed in a parameter.
- `search_matching_indices_3`: The routine iterates through two tables to find an index where pairs of bytes at corresponding offsets match two specified word values.
- `search_matching_indices`: The routine iterates through a table to find an index where two consecutive byte values match two specified word values, returning the index or zero if no match is found within one hundred entries.
- `search_matching_indices_2`: Iterates through two tables to find an index where pairs of bytes at corresponding offsets match two specified word values, returning the index or zero if no match is found within one hundred iterations.
- `clear_script_object_fields`: Checks a field of its parameter and if non-zero clears two bytes at offsets derived from that field, calls another routine, and conditionally clears a byte at a derived address.
- `update_script_object_fields`: The routine saves and restores a global word, reads a short from a table, loops over a byte count updating a script object fields and calling another routine.
- `check_param_and_set_flags`: Checks a field of the parameter and, when it is non-zero and not 129, calls another routine, writes a word to a global variable, then conditionally calls another routine based on the field value before writing a byte to a global variable.
- `initialize_script_object_state`: Initializes script object globals by copying a word, setting a pointer, and updating a state field.
- `fill_palette_from_script_table`: Copies a block of data from a script object table into a temporary buffer and adjusts one field.
- `draw_sprite_masked_region`: Prepares a masked rectangular region by calculating clamped bounds and marking bytes in two buffers.
- `initialize_dialogue_state`: Fills a table with values from another routine, builds an index table, and shuffles that index table using a pseudo-random sequence.
- `handle_recursive_data`: If a specific byte in a parameter is one or two, the routine calls itself recursively with an offset and stores the result in a global variable.
- `reset_structure_and_seek_2`: The routine checks a pointer stored in a global variable and if non-zero calls another routine with that pointer, then clears the global pointer.
- `get_word_from_global`: The routine returns a word stored at a fixed global offset.
- `clear_word_at_global`: The routine clears a word at a fixed global offset.
- `test_flag_bit`: The routine reads a byte via a pointer stored in the global variable area, tests a specific bit, and returns a boolean result.
- `call_reset_structure_and_seek`: The routine calls another routine with a pointer from a global variable and the passed parameter.
- `execute_pointer_call`: The routine calls another routine using a pointer retrieved from a global variable.
- `copy_bytes_with_loop_2`: The routine loops while a counter is positive, limiting each iteration to 16384 bytes, and calls another routine with the current pointer, chunk size and a pointer from a global variable, advancing the pointer by the chunk size each iteration.
- `xor_decrypt_buffer`: Loops while a length counter is positive, clamping each chunk to a maximum size, calls another routine with a pointer, a chunk size and a pointer from a global variable, then XORs each byte of the buffer with a byte from a global variable and advances the pointer, updating the remaining length.
- `call_other_routine_4`: The routine calls another routine and then returns.
- `fetch_script_byte`: Calls another routine with a pointer retrieved from a global variable.
- `process_conditional_buffer`: Retrieves a word from a global variable and, if it is zero, calls another routine, updates a flag, and increments a global counter.
- `reset_word_and_increment_counter`: The routine clears a word at a fixed global offset and increments a short counter at another fixed global offset by one.
- `process_xor_byte_and_copy`: The routine performs an XOR operation on bytes from a parameter using a global byte, then conditionally calls another routine to copy a byte and updates a global word and counter.
- `reset_word_and_add_two`: The routine clears a word at a fixed global offset and adds two to a short counter at another fixed global offset.
- `process_xor_word_and_copy`: The routine performs an XOR operation on a 32-bit value using a global byte, then conditionally calls another routine to copy a byte and updates a global word and counter.
- `reset_word_and_add_four`: The routine clears a word at a fixed global offset and adds four to a short counter at another fixed global offset.
- `xor_byte_with_word`: Calls another routine, stores a flag derived from the result into a word in a global variable, then returns the result xored with a byte from a global variable.
- `xor_decrypt_two_bytes`: The routine calls another routine with a stack buffer, a size of one and a pointer from a global variable, stores a flag derived from the returned short into a global variable, XORs two bytes from the stack buffer with a byte from a global variable and returns the combined result.
- `xor_decrypt_four_bytes`: The routine calls another routine with a four-byte stack buffer, a size of one and a pointer from a global variable, stores a flag derived from the returned short into a global variable, XORs each of the four bytes from the stack buffer with a byte from a global variable and returns the combined 32-bit result.
- `decode_and_update_range`: Reads a short from a global table indexed by a byte in a global variable, loads a pointer from the resulting address, and copies several bytes from that pointer into global variables and a local buffer.
- `calculate_text_render_coordinates`: Computes screen coordinates and buffer offsets for text rendering by processing input structures, clamping bounds, and applying global offsets.
- `update_sprite_range_bounds`: Updates a pair of short range limits stored in two structures, clamps the limits to fixed ranges, computes a byte offset using a table indexed by a global variable and a global base pointer, and then calls another routine repeatedly to fill a buffer with a repeated value derived from the computed offset and the range size.
- `compute_sprite_position`: Validates two parameters and, if a range lookup succeeds, computes two 32-bit positions from a table indexed by a global word and stores them through pointers in a global structure, adjusting them with an offset and a nibble from the second parameter.
- `add_offset_and_compute_position`: The routine adds a global short to each of two parameters and passes the results to another routine.
- `compute_sprite_position_2`: The routine subtracts two parameters, takes absolute values, determines a maximum, then loops while accumulating sums and conditionally calls another routine with an updated short value.
- `find_matching_range`: Initializes a word in a structure to a sentinel value and scans a table of three entries to find a range containing a parameter value, storing the index of the match.
- `dispatch_script_command`: The routine reads a short from a global variable and toggles a byte if the value matches a constant, otherwise processes a different command value by fetching words via other routines, updating several global words and bytes, and returning a status flag.
- `process_script_input_buffer`: The routine initializes script globals and format state, then reads bytes from a script buffer into a global buffer while maintaining a stack index, inserting terminators and updating the buffer on each step.
- `initialize_script_and_dispatch_loop`: The routine calls another routine with two arguments, then repeatedly calls that same routine until it returns a value other than one, and finally calls it again with zero as an argument.
- `dispatch_node_selection_2`: Copies a byte from a global variable to another, then loops incrementing a global byte and dispatching to one of four node-selection helpers.
- `process_node_chain_3`: The routine builds a chain of nodes from global data and walks the chain to find a node matching a parameter.
- `initialize_script_object_2`: Sets up script object globals, writes several constants into global words, and parses a byte stream into a buffer.
- `initialize_script_state_duplicate`: Performs a complex initialization involving copying global bytes, checking flags, iterating through tables, and setting multiple global words and bytes based on a parameter.
- `loop_until_nonzero_return`: The routine repeatedly calls another routine in a loop until the returned value as a short is non-zero, then returns that final non-zero value.
- `poll_status_register`: Repeatedly executes another routine until it returns a non-zero value.
- `dequeue_message_index`: Reads a short index from a global variable, compares it with another global short, and if they differ it returns and clears an entry from a global table indexed by the short, then increments the index modulo sixteen.
- `return_no_op`: Consists of a single return instruction with no other effect, so it does nothing when called.
- `return_immediately_6`: This routine performs no operations and returns immediately to the caller.
- `dequeue_and_update_state`: The routine clears a global word, calls another routine and stores its result in a global word, calls another routine, and returns the stored word.
- `reset_message_state`: Clears three global words used for message handling and then calls another routine.
- `set_byte_zero_2`: Stores a zero byte into a specific location in a global structure.
- `set_byte_one`: Stores a one byte into a specific location in a global structure.
- `update_indexed_table_entry`: Checks a field of a parameter and, if non-zero, reads a short from a global variable, increments it modulo sixteen, and stores the parameter field into a table indexed by the original short.
- `process_message_loop`: Repeatedly calls another routine with a global pointer and a buffer until it returns zero or a counter reaches a limit, then performs specific actions based on the returned integer.
- `process_message_and_reply`: Obtains a message pointer from a global table and, if present, copies several fields from the message into a caller-supplied buffer.
- `copy_object_data_from_table`: The routine computes a pointer into a table using a parameter index and copies bytes from that entry into several global variables.
- `store_table_entry_bytes`: The routine stores two bytes derived from the parameters into a table slot indexed by the parameter value.
- `update_object_state`: Copies two words from a global structure, queries two flags, sets a global word based on the results, updates two global shorts when a byte and a char change, and then selects a byte from a small table to build an indirect call.
- `read_object_words_from_pointer`: The routine reads an integer from a global variable, uses it as a base address, and copies two words from fixed offsets into the two output pointers.
- `return_immediately_7`: This routine is empty and performs no operations before returning to the caller.
- `toggle_flag_bit_one`: The routine tests bit one of a global byte, returns whether it was set, and if set clears the bit by XORing the byte.
- `toggle_flag_bit_two`: The routine tests bit two of a global byte, returns whether it was set, and if set clears the bit by XORing the byte.
- `handle_object_state_flags`: The routine updates word and byte flags at fixed offsets in a structure based on a code value in a parameter block, modifies the code value in the parameter block, and returns the updated code.
- `allocate_and_initialize`: The routine calls a function via a global variable table, stores the returned pointer into a global variable, and then calls another function via a pointer stored in a global variable.
- `free_and_output`: The routine calls a function via a pointer stored in a global variable and then calls another function via a global variable table.
- `invoke_indirect_handler_2`: An empty routine that calls an indirect handler.
- `call_indirect_via_table_2`: An empty routine that calls another routine indirectly through a pointer in a global variable.
- `conditional_indirect_dispatch`: Tests a byte in a structure and calls one of two other routines indirectly based on the result.
- `store_handler_parameter`: Stores the incoming short parameter into two global shorts, handling a sentinel value, and calls another routine when the parameter is zero.
- `call_handlers_and_compute`: Calls two indirect handlers obtained from a global pointer, then calls another routine with a value built from stack data, a global short, and a parameter.
- `reset_state_and_dispatch`: The routine checks if a word in a global structure is non-zero, clears it, and recursively calls itself.
- `wait_and_invoke_handlers`: Waits for a word change, then invokes two indirect handlers obtained from a global pointer via offsets, waits again, and invokes two more indirect handlers via offsets.
- `toggle_state_flags`: The routine conditionally calls another routine and clears a global byte, copies and toggles a global byte, calls another routine, and if a global short is non-zero it calls another routine, flips a global short, and calls another routine.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `initialize_handler_state`: The routine calls another routine with a global word, sets a global byte, clears a block of 40 bytes in global memory, and writes a constant word to 20 consecutive words in global memory.
- `initialize_handler_state_2`: Calls another routine with a global word, sets a global byte to one, clears a forty-byte block of memory to zero, and writes the constant value two hundred to twenty consecutive words in global memory.
- `wait_and_process_param`: Waits for a word change, invokes two indirect handlers from a global pointer, calls another routine with the incoming parameter, then waits again and invokes two more indirect handlers.
- `wait_and_execute_handlers`: Spins until a 32-bit global variable changes, then executes two indirect handlers from a table, calls another routine, and finally executes two more indirect handlers from the table.
- `pass_argument_to_decoder`: Fetches a value from a memory location and passes it as an argument to a routine that initializes decoding tables.
- `modify_global_words`: The routine performs two loops that modify two global words by checking specific bits and adding the result to the original value multiplied by two.
- `store_byte_at_offset`: Stores a single byte into a memory location calculated from a global pointer and a fixed offset.
- `build_lookup_table_from_bits`: The routine calls another routine, copies a byte from a global variable into a global word, calls another routine, then builds 256 entries in a global table by combining words from a small table according to the bits of each index.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work.
- `initialize_three_structures`: The routine calls another routine three times with three different constant arguments.
- `fill_two_short_tables`: The routine writes sequential short values from zero upward into two separate areas referenced through a global variable pointer, advancing the destination by two bytes each iteration for 32 iterations.
- `decode_and_dispatch_script`: Reads a byte from a global variable to index a table of routine pointers, stores the selected pointer in a global variable, and if a flag is set and the index is non-zero, calls another routine with an offset address.
- `sync_short_table_entries`: Iterates over a fixed number of short pairs, compares two tables and updates the second with the first when they differ, then calls another routine with an indexed value.
- `copy_script_data_block`: This routine checks a value in a table, updates two global words with a parameter, and copies sixteen bytes from a source area to a destination area.
- `copy_bytes_via_table`: The routine copies 16 bytes from a source area addressed by a global variable pointer and an index table into a destination area referenced through a global variable, using an offset derived from another global variable.
- `copy_string_and_dispatch_2`: Copies a null-terminated string from a pointer into a local buffer and then calls another routine twice with the buffer and different offsets.
- `process_script_loop`: Enters a loop that fetches data from various routines, copies a sequence of bytes into a table, and updates a global word value.
- `fill_struct_array`: Loops over a fixed number of entries and writes a parameter and another value into specific offsets of a calculated address within a structure array.
- `copy_string_and_execute_callback`: Copies a null-terminated string from a source pointer into a destination buffer within a global structure, then executes a function pointer retrieved from an offset within that same structure.
- `initialize_game_state`: Attempts to obtain several resources via indirect calls through a global table and stores the results in a global structure.
- `dispatch_indirect_calls`: The routine uses a parameter to index into a table of function pointers to perform an indirect call, or executes cleanup routines via pointers in a global table.
- `cleanup_and_exit`: Performs multiple function calls via pointers in a global table, clears memory, and closes libraries before returning a constant.
- `open_libraries_and_set_flags`: The routine obtains library bases via indirect calls, stores them into global variables, checks a flag in one library structure and writes two word values into global variables accordingly, then calls another routine and stores a stack value into a global variable before returning a success flag.
- `close_libraries_twice`: The routine performs two close-library calls via indirect function pointers derived from a global variable, using the same offset twice.
- `allocate_memory_blocks`: The routine calls two functions via a library base stored in a global variable, obtains a memory pointer via an indirect call, stores it and several offsets into multiple global variables, repeats the allocation for a second set of pointers, and returns a flag indicating success.
- `free_memory_twice`: The routine performs two free-memory calls via indirect function pointers derived from a global variable, using the same offset twice.
- `initialize_interrupt_structure`: Writes constant bytes and pointers into a structure in memory, stores a self-reference as a code pointer, and calls an indirect routine obtained from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `check_current_directory_info`: The routine calls two functions via a library base stored in a global variable, then calls another function via the same base and returns a boolean indicating whether a field in a structure pointed to by a global variable equals a constant.
- `copy_strings_from_table`: The routine disables interrupts with Forbid and enables them with Permit, then iterates a table referenced via a global variable.
- `check_file_info`: The routine obtains a pointer from a global variable and calls a library function through it.
- `copy_strings_and_check_file`: If a global byte is zero, it copies strings from a table, checks file info via a library call, initializes format state, and then iterates a table to perform further string copies and file checks.
- `fill_buffer_from_value`: The routine receives a byte pointer and a parameter containing a byte value and a repeat count.
- `copy_bytes_between_buffers`: The routine receives two byte pointers and a parameter containing a length.
- `execute_callback_list_2`: Iterates a number of times determined by a global short and executes a function pointer stored in a global variable during each iteration.
- `check_condition_and_return_status`: The routine compares a parameter against two global words and, if they match and another routine returns a non-zero value, returns one.
- `return_zero_constant`: The routine contains no operations and returns the constant zero.
- `update_global_state_from_param`: The routine decrements a byte in a table indexed by a parameter, conditionally calls other routines and sets a flag, reads bytes via a pointer derived from the parameter, stores several values into global words, and copies a global word to another global location.
- `accumulate_script_chunks`: The routine loops while a count parameter is positive, clamping the count to a maximum, calls another routine with a pointer, a clamped count and a parameter, adds the returned short to an accumulator, subtracts the returned value from the count and advances the pointer, repeating while the returned value equals the clamped count.
- `allocate_and_process_resources`: Allocates and frees memory while invoking several helper routines to copy strings, check a file, accumulate script chunks, dispatch conditionally, compute an adjusted offset, reset a structure and seek, and process an entry while clearing flags.
- `decrement_flagged_entries_2`: The routine loops over a range defined by a global byte, comparing each byte in a table with a parameter, and if a match is found, clears a word and conditionally decrements a byte in another table.
- `decrement_flagged_entries`: The routine loops over a range defined by a byte in a global variable, comparing each byte in a table with a register value.
- `reset_state_and_mask_table`: The routine clears two global words, calls another routine, clears a global byte, and then masks a range of bytes in a table to keep only the high bit.
- `invoke_handler_twice`: The routine calls another routine twice in succession and then returns.
- `initialize_pointers_and_counter`: The routine stores the address of a structure within the same block into a global variable and writes the constant three into another global variable.
- `free_memory_and_clear_flags`: The routine calls exec.FreeMem and then returns.
- `register_entry_and_return_index`: The routine calls another routine to obtain a word, builds a combined value from that word and a global byte, and if the byte is below a limit it increments the global byte, stores a byte derived from the parameter into a table, increments a byte in another table, and returns a word derived from the parameter.
- `decrement_counter_and_dispatch`: The routine repeatedly decrements a byte in a global variable while it is non-zero, and conditionally calls another routine based on a byte looked up via an offset from that decremented value.
- `decrement_counters_by_index`: The routine repeatedly decrements a counter byte in a global variable while it is non-zero, reads an index from a table using the decremented value, and if the index is valid and a counter in another table is non-zero it decrements that counter.
- `call_subroutine_twice_5`: Calls another routine two times and then returns.
- `dispatch_conditional_sequence`: Calls two routines with a global address and a parameter, then conditionally calls a third routine if a global word matches a specific value.
- `clear_offset_word`: The routine calls another routine twice and then stores a zero word at a specific offset from a pointer returned by one of those calls.
- `call_other_routine_6`: Calls another routine and then returns.
- `read_offset_word`: The routine calls another routine and returns a word stored at a specific offset from a pointer returned by that call.
- `call_and_store_short`: Calls another routine twice and then stores a short value into memory at an offset from a pointer held in a register.
- `set_flag_and_branch`: The routine calls another routine, then branches based on a global byte to call a different set of other routines.
- `clear_flag_conditionally`: The routine conditionally calls another routine if a global byte is non-zero, then stores zero into that global byte and returns.
- `copy_byte_to_offset`: The routine copies a byte from one offset within a block to another offset within the same block.
- `toggle_byte_flag`: The routine reads a byte from one offset within a block, XORs it with one, and writes the result to another offset within the same block.
- `load_script_object_state`: The routine copies words from a script object entry into global variables, conditionally adjusts a field, calls another routine, clears a global word and then iterates over a fixed range updating global words and writing back bytes to the object entry.
- `clamp_and_update_fields`: The routine clamps two 16-bit values read through a global base pointer within a third 16-bit range, updates a 32-bit field using a calculation involving those values and a table entry, and updates two 16-bit fields in a table based on the clamped values.
- `compute_and_call_routine`: The routine builds an address from several values read through a global base pointer and calls another routine with that address and two additional arguments derived from the same base pointer.
- `update_palette_and_trigger_callbacks`: The routine copies two 20-byte blocks from the arguments into a global area and mirrors the values into a global table by splitting each 32-bit word into two 16-bit components.
- `store_value_from_global`: Reads a 32-bit value from a global variable and writes it to a location computed from another global variable and a fixed offset.
- `return_immediately_stub_3`: The routine consists of a single return instruction and performs no work, so it does nothing when called.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work.
- `set_flag_and_copy_word`: Tests a bit in the parameter, writes a 16-bit flag to a global variable and a constant to a global variable, then copies a 32-bit word from a global variable to another global variable.
- `wait_for_word_change`: Spins in a loop until a 32-bit value in a global variable differs from another 32-bit value in a global variable, then returns.
- `update_short_entries`: If a condition on the parameter and a global short holds, it writes the parameter’s high word to a global short and calls an indirect function from a global table; it then stores the parameter’s low word into a global table entry indexed by the parameter’s high byte and into a similarly indexed global variable.
- `invoke_indirect_handler`: Calls an indirect function obtained by adding a constant offset to a pointer stored in a global variable.
- `call_indirect_via_table`: The routine calls another routine indirectly through a pointer stored in a global variable.
- `compute_and_store_scaled_values`: Computes a value by adding two hundred to the input parameter, shifting it left by eight bits, and performing a bitwise OR with one, then stores this result in two global variables.
- `decompress_and_copy_data`: Decompresses a stream of data using a variable-length encoding scheme and writes the resulting bytes into a destination buffer.
- `clear_memory_blocks`: Iterates through a series of memory blocks, either zeroing them out or filling them with zero based on a condition checked against a string.
- `store_words_with_offset`: The routine writes the same 32-bit value from a register to two consecutive words in memory pointed to by a pointer, then advances the pointer by an offset derived from a register and repeats the store while decrementing a counter until it reaches -1.
- `copy_bytes_with_stride`: The routine copies bytes from a source pointer to a destination pointer, advancing the destination by a fixed stride for each byte copied.
- `decode_and_copy_bytes_3`: The routine uses a global base pointer to index four tables and decodes nibble-based indices from a word buffer, writing bytes and words into a destination buffer in a repeated loop controlled by a counter.
- `initialize_decode_tables`: The routine stores the argument into a global variable and fills several tables derived from that base with values computed from the bits of two 5-bit counters.
- `return_constant_2`: Returns the constant 24576 in d0 to the caller without any other effect.
- `expand_bitstream_run_length_data`: Reads a compressed bitstream and expands it by repeating values based on encoded run-length counts.
- `decompress_bitstream_data`: Reads a compressed data stream from a source buffer and expands it into a destination memory area.
- `return_null_value`: This routine is empty and returns a null value to the caller.
- `decompress_bitstream_data_3`: Reads a variable-length encoded bitstream and writes decoded values to a destination buffer.
- `decompress_bitstream_data_2`: Reads a compressed data stream using a variable-length encoding scheme and writes the resulting bytes to a destination buffer.
- `return_uninitialized_stack_value`: This routine is empty and returns an uninitialized value from the stack.
- `update_global_pointer`: The routine adds a constant offset from a table to a global variable, increments the global variable, and returns zero.
- `return_byte_from_pointer`: The routine returns the byte stored at the memory location pointed to by its parameter.
- `return_word_from_bytes`: The routine builds a 16-bit value from two consecutive bytes pointed to by its parameter and returns it.
- `return_long_from_bytes`: The routine builds a 32-bit value from four consecutive bytes pointed to by its parameter and returns it.
- `setup_blitter_and_wait`: Programs the Amiga custom chip registers for a blitter copy, writes computed 32-bit sums and 16-bit constants into a structure pointed to by a base address, then calls another routine repeatedly and spins until a flag bit is cleared.
- `program_blitter_and_wait`: Writes three values into a blitter-style register block addressed by A0: two offsets computed from D6 plus A3 and D6 plus A4, and a 16-bit value from D4 into the third slot.
- `wait_for_bit_clear`: The routine enters a loop that repeatedly checks a specific bit in a byte at a fixed offset from a pointer until that bit is cleared.
- `configure_blitter_copper_wait`: The routine programs the Amiga custom chip blitter registers and copper registers to set up a blit operation, then calls another routine to wait for the blitter to finish.
- `blit_rectangular_area_with_plane_merging`: The routine programs the blitter to copy data from a source plane into a destination plane, merging the source bits into the destination for multiple scanlines to fill a rectangular area.
- `copy_struct_blocks`: Uses a global variable as a loop counter and as the base for two pointers to repeatedly copy two consecutive 32-bit words from one block to another.
- `conditional_copy_struct_fields`: Iterates over a set of blocks using a global counter and three pointers derived from a global variable.
- `decode_and_write_bytes`: Processes a byte stream using a global variable as a key and a global table as a lookup, updating a 32-bit state and writing transformed bytes through a pointer that is advanced each iteration.
- `return_no_op_stub`: The routine consists of a single return instruction and performs no work: it touches no registers, hardware, library calls, or data.
- `return_immediately_8`: This routine performs no operations and returns immediately to its caller.
- `interpret_script_and_manage_resources`: Processes a data stream by seeking file data, handling indexed entries with flags, allocating memory blocks, and executing wrapped disk reads.
- `parse_script_data_stream`: Processes a data structure containing a pointer to a byte stream and various status flags.
- `process_script_resource_block`: Processes a script resource block by recursively traversing sub-blocks or linked data and reading bytes from a pointer field.
- `tokenize_string_input`: Parses a string into an array of pointers by splitting on whitespace or handling quoted substrings.
- `execute_single_instruction_2`: An extremely short routine consisting of a single instruction that calls another routine.
- `adjust_string_bytes`: The routine walks a zero-terminated byte string pointed to by the argument and, for each byte, checks a flag in a global table indexed by the byte value.
- `calculate_string_length`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `skip_bytes_by_table`: The routine examines the first byte of the buffer pointed to by the argument and, while a flag in a global table indexed by that byte is set, advances the pointer by one.
- `format_and_reverse_string`: Repeatedly calls another routine to format a number, stores bytes from a register through a pointer, and reverses a twelve-byte buffer in place.
- `format_number_string`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii_2`: Converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `expand_and_parse_nibbles`: Expands a packed nibble value using a lookup table, writes the resulting bytes in reverse order to a destination, and parses a numeric string into a signed short.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_integer_string`: Parses a string of numeric characters into a signed short value, handling optional leading plus or minus signs.
- `parse_numeric_string`: Parses a string of numeric characters into a signed short value stored in a pointer.
- `parse_script_number`: Parses a script number from a byte stream using a table-driven digit classification, optionally reading a sign and a fixed-point value.
- `get_and_increment_byte`: The routine reads a byte from a location pointed to by a global variable, increments that global pointer, and returns the byte or a specific value if the byte was zero.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies (a7)+ to d7, then returns.
- `store_byte_to_buffer`: The routine increments a counter stored in a global variable, writes a byte from the argument to the location pointed to by a global pointer, and then increments that global pointer.
- `initialize_format_state`: The routine clears a 16-bit counter in a global variable, stores an argument into a global variable, calls another routine with the argument and a stack address, clears a byte through a global pointer, and returns the counter.
- `store_registers_to_pointer`: The routine copies the contents of D1 through D7, A1 through A6, and the value from the stack into successive entries of the pointer passed in A0.
- `set_first_element_and_return_concat`: The routine reads a field from the second argument, forces a zero value to one, stores the first word of the first argument into an element of the first argument, and returns a 32-bit value formed by concatenating the adjusted field with the read word.
- `manage_file_handle_table`: The routine scans a table of entries to find the first available slot and then performs a file operation by calling various wrappers based on bitwise flags in a parameter.
- `seek_file_wrapper`: The routine obtains a value from another routine and if non-zero calls another routine with a value loaded from an offset, then checks a global word and returns an error code if set, otherwise returns the result.
- `absolutize_signed_integer`: The routine receives a signed integer in a register, negates it when it is negative, and returns the absolute value.
- `find_memory_block`: Searches a linked list of memory blocks for a block of at least a requested size, splitting a larger block if needed and updating a global counter.
- `copy_bytes_with_loop`: The routine iterates over a byte buffer using a counter stored in a structure and copies each byte through a pointer while decrementing a short counter in the structure.
- `compute_adjusted_offset`: The routine checks a flag byte in a structure and conditionally calls another routine with a derived index.
- `reset_structure_and_seek`: The routine conditionally adjusts a parameter using a short field from a structure based on flag bits, then clears several fields in the structure and masks flag bits.
- `process_script_resource_block_2`: The routine iterates over a nested count structure, decrementing a short counter and advancing a byte pointer stored in a structure, reading a byte or calling another routine when the counter reaches zero, writing each byte to an output buffer and returning the outer loop index when a sentinel value is encountered.
- `process_buffer_data`: A thirteen-instruction routine that calls another routine, consumes a byte from a buffer, and performs a pass-through call to another routine.
- `consume_byte_from_buffer`: The routine decrements a counter in a structure and either calls another routine or copies a byte from a buffer to a destination and increments the buffer pointer.
- `pass_through_call`: Calls another routine and returns its result unchanged.
- `increment_and_fetch_script_byte`: Increments a global counter and decrements a short counter in a structure, then either recursively calls itself or advances a pointer within a structure to return a byte.
- `invoke_script_interpreter`: The routine calls another routine and returns without further processing.
- `store_params_and_invoke`: The routine stores zero into a short at a fixed offset in a global variable, stores a parameter into a word at a fixed offset in a global variable, calls another routine with the first parameter, the second parameter and a stack address, calls another routine, and returns the short from the fixed offset in the global variable.
- `parse_command_and_prepare_struct`: Examines a command string and a structure to decide flags and an index, obtains a value from another routine using a derived index, and then clears several fields in the structure while writing the obtained value and computed flags into it.
- `fetch_byte_from_script_block`: Checks a flag in a script block and may invoke other routines, decrements a short counter in the block and either returns a byte from a pointer inside the block after advancing the pointer or recursively processes the block.
- `iterate_linked_list_and_process`: Walks a linked list starting from a global variable, checking specific bits in a flag byte and comparing two words within each entry before conditionally calling another routine.
- `dos_write_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer obtained from a global variable and on error stores an error word from another function pointer into globals and returns the result.
- `dos_seek_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer from a global variable and on error records an error word, then computes a return value based on parameters and a function pointer result.
- `dos_read_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer obtained from a global variable and on error stores an error word from another function pointer into globals before returning the result.
- `dos_open_wrapper`: The routine checks a signal flag via another routine, clears a global word, calls a function pointer from a global variable and if the result is zero records an error word and returns a failure value.
- `dos_lock_open_wrapper`: The routine checks a signal flag via another routine, clears a global word, attempts a lock via a function pointer and on success opens via another pointer, recording an error on failure.
- `dos_delete_open_wrapper`: The routine checks a signal flag via another routine, clears a global word, conditionally unlocks and deletes via function pointers, then opens via a function pointer and records an error on failure.
- `close_file_if_needed`: Checks a global variable and conditionally calls another routine, then calls dos.Close via an indirect pointer.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `calculate_fixed_point_division`: Performs signed fixed-point division using a normalization process and returns the result.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `lookup_table_entry`: The routine checks a parameter against a global variable and a table entry and stores a byte through a pointer.
- `parse_number_with_skip`: The routine returns zero when the argument is zero, otherwise it calls another routine with the argument, then calls another routine with the result and a stack location, and returns the value stored there.
- `free_linked_list`: The routine walks a linked list stored in a global variable, calls another routine for each node, and clears two global pointers.
- `allocate_linked_node`: The routine allocates memory with AllocMem, initializes fields of the new node from a parameter and a global variable, links it into a list stored in global variables, and returns a pointer to the node's data area.
- `process_indexed_entry_with_flag`: Obtains an index from another routine, tests a flag byte at index plus one, optionally calls another routine, then calls another routine with a value from index plus two and returns a fixed address if a global word is non-zero.
- `parse_script_value`: Parses a numeric value from a byte stream using a table-driven digit classification, handling various formats including signed integers, fixed-point numbers, and octal values.
- `parse_format_specifier`: Parses a format specifier string to extract flags, width, precision, and length, then retrieves arguments from a pointer to produce formatted output.
- `scan_format_string`: The routine walks a string looking for percent signs, skips escaped percent signs, and for each format specifier calls a parser and then calls a function pointer.
- `allocate_memory_block`: The routine checks a flag in a structure and a global variable, then obtains a block via a memory allocator and stores the result into two fields of the structure.
- `iterate_table_and_call`: The routine loops over a count stored in a global variable, reads entries from a table indexed by the loop counter, and conditionally calls another routine with a value from the table.
- `execute_callback_chain`: The routine checks a pointer in a structure and calls it if it is not null, then performs several other calls to function pointers retrieved from a global variable and a structure.
- `copy_c_string_3`: The routine copies bytes from a source pointer to a destination pointer until a null byte is encountered, advancing both pointers and returning the original source pointer.
- `insert_memory_block`: The routine inserts or updates a memory block descriptor in a linked list stored in global variables, adjusting a global counter and returning a status code.
- `execute_wrapped_dos_read`: The routine calls another routine to check a status, and if non-zero, calls a function pointer using a value from an offset.
- `copy_memory_block_2`: Copies a specified number of bytes from a source address to a destination address.
- `allocate_and_link_block`: The routine clears a pending request stored in a global variable by calling another routine with data from that variable.
- `allocate_and_parse_command`: The routine walks a linked list starting at a fixed offset in a global variable until it finds a node with a zero short field or reaches null, allocates a new block if the list is empty, initializes a fixed number of bytes in the new block to zero, links the new block into the list, and then calls another routine with two parameters and the block pointer, returning its result.
- `process_entry_and_clear_flags`: The routine conditionally obtains a value from another routine using a derived index, then calls another routine with fields from a structure.
- `clear_table_entry`: Obtains a pointer from another routine and, if non-null, checks a flag byte in the pointed-to structure to conditionally clear a word and return a status.
- `check_signal_and_clear`: The routine calls a function pointer obtained from a global variable, tests a bit mask and a global variable, calls a function pointer stored in a global variable, and if conditions hold clears the global variable and calls another routine.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Secret of Monkey Island, The (1990)(LucasFilm Games)(Fr)[cr Defjam][a](Disk 3 of 4).adf:bootblock

- `check_library_function_result`: Calls a function via a pointer located at an offset from a base address and returns zero if the call succeeds or a specific bitmask if it fails.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_trap`: The routine assigns its own address to a global variable and executes a software interrupt trap.
- `initialize_graphics_and_cop_lists`: Initializes memory buffers and populates several tables with structured data, including the construction of Copper lists and bitplane configurations.
- `tiny_ori_2`: Small routine of 1 instruction that merges 59 into d1.
- `copy_and_increment_buffer_indices`: Copies a series of bytes from a source table to a destination buffer based on an index derived from a global variable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 104 into d0.
- `update_linked_list_and_copy_data`: Manipulates a bitmask in memory and updates a linked list by shifting elements and adjusting pointers.
- `tiny_ori_4`: Small routine of 1 instruction that merges 1 into (a2).
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d5.
- `update_circular_buffer_and_reorder_data`: Manages a circular buffer by shifting elements and updating a pointer, then performs a complex data reordering by copying specific word indices from a source table to a destination table.
- `rotate_and_copy_data`: Performs a bitwise left rotation on a block of words while masking specific bits, then copies a block of data to a destination offset.
- `tiny_ori_5`: Small routine of 1 instruction that merges 42 into d5.
- `call_and_update_globals`: Sets a global variable, calls a function pointer derived from that variable, and then stores the variable and three bitwise-modified versions of it into other global variables.
- `execute_pointer_sequence`: Loads several global variables, performs a memory dereference through a pointer, and calls a function pointer.
- `tiny_and`: Small routine of 1 instruction that masks d0 with d0.
- `update_pointer_table`: Selects one of two memory locations based on a global variable and copies four words from that location to another set of global variables.
- `tiny_ori_6`: Small routine of 1 instruction that merges 64 into d0.
- `toggle_state_and_load_quad_word`: The routine toggles a global variable and selects one of two memory addresses to load four consecutive longwords into four separate global variables.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `calculate_blitter_coordinates`: Performs several recursive calls and updates multiple global variables with calculated values.
- `modify_bitmask_data`: Performs bitwise operations to clear a bit at a specific index and sets bits in two different memory locations.
- `tiny_ori_7`: Small routine of 1 instruction that merges 1 into -(a6).
- `execute_recursive_calls`: This routine performs three consecutive calls to itself before returning.
- `wrap_signed_value`: Performs a range check on a value through a pointer to wrap it within a specific signed boundary and then stores the result of another routine into a table.
- `nop_operation`: This routine is empty and performs no operations.
- `process_instruction_list`: Iterates through a list of data, performing conditional jumps and memory lookups based on values retrieved from a table.
- `check_point_orientation`: Calculates a cross product of two vectors derived from a table to set a global variable based on the result's sign.
- `tiny_ori_8`: Small routine of 1 instruction that merges 1 into d0.
- `tiny_ori_9`: Small routine of 1 instruction that merges 0 into d6.
- `transform_coordinate_list`: Iterates through a list of coordinates, performing linear transformations and updating a table with the results.
- `calculate_matrix_elements`: Loads values from a table and performs multiple multiplications and shifts to populate several global variables with matrix-related data.
- `calculate_geometry_parameters`: Computes differences between input values to determine specific parameters and stores the results in a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_system_trap`: Calls two function pointers, modifies a global variable, and executes a software interrupt.
- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.
