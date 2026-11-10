# Game 1120 (global effect (1992)(millenium)(m4)[cr](disk 1 of 3)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01120). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910336 top-level bytes in 48 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 33 of 33 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 307 of 307 named (0 author symbols), 307 of 307 described (66 by rule, 205 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 970 of 970 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9386 of 9386 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 365334 |
| code | 28118 |
| data | 485402 |
| padding | 12026 |
| unused | 19456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Startup | 2 | 230 | 7 |
| Stack | 2 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Status | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Ask | 2 | 117 | 2 |
| Avail | 11 | 376 | 12 |
| AddBuffers | 2 | 117 | 2 |
| Break | 2 | 117 | 2 |
| Global | 12 | 45 | 2 |
| Assign | 2 | 117 | 2 |
| EndCli | 2 | 117 | 2 |
| Dir | 1 | 911 | 8 |
| setup | 12 | 45 | 2 |
| Echo | 2 | 117 | 2 |
| Quit | 2 | 117 | 2 |
| FailAt | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| MakeDir | 2 | 117 | 2 |
| Cd | 2 | 117 | 2 |
| DisplayBack | 1 | 9 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 12 | 806 | 37 |
| Copper | 2 | 16 | 1 |
| LoadWB | 12 | 538 | 30 |
| disk-validator | 1 | 386 | 1 |
| ram-handler | 2 | 10 | 1 |
| icon.library | 10 | 1375 | 109 |
| Global Effect (1992)(Millenium)(M4)[cr](Disk 1 of 3)[a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2607 | 58 |

## Cases handled

- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 705 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- global effect (1992)(millenium)(m4)[cr](disk 1 of 3)[a]/Global Effect (1992)(Millenium)(M4)[cr](Disk 1 of 3)[a].adf:C/Startup: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- global effect (1992)(millenium)(m4)[cr](disk 1 of 3)[a]/Global Effect (1992)(Millenium)(M4)[cr](Disk 1 of 3)[a].adf:C/setup: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_command_input` (Dir): Parses user input from a command interface and updates a state structure based on specific key presses. Kind: sabotage path. Pass path: the branch where iStack_20/iStack_1c/etc are 0, leading to the standard menu/command processing logic.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_and_branch`: Copies the contents of one register to another and then performs a short relative branch.

### Startup

- `parse_amiga_hunk_table`: Iterates through a memory buffer to identify and process AmigaDOS hunk types.
- `decrement_value_loop`: Decrements a word value repeatedly until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_copy_buffer`: Shifts data within a memory buffer by checking a bit flag and performs block memory copies or byte-wise shifts using a loop.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_manipulation`: Performs complex bitwise shifts and rotations on data within a loop.

### Stack

- `update_buffer_and_copy_data_3`: Iterates through a linked list to find a maximum value, then populates a buffer with a header and a sequence of incrementing values.
- `move_and_branch_2`: Moves a long value from register d1 to d0 and then performs a relative branch.

### Else

- `process_linked_list_and_update_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `move_and_branch_5`: Moves a long word from one register to another and then performs a relative branch.

### Status

- `process_linked_list_and_update_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then populates a data buffer with a header and subsequent values.
- `transfer_register_and_branch_6`: Moves the contents of one data register to another and performs a short relative branch to another location.

### Delete

- `update_linked_list_and_copy_buffer`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer header with this value.
- `copy_d1_to_d0_then_branch`: This routine is exactly two instructions: it copies the long value in register d1 into register d0, then branches unconditionally to a point inside another routine.

### Ask

- `process_linked_list_and_update_buffer_3`: Traverses a linked list to find the maximum value in a specific field and then populates a data buffer with a header and subsequent values.
- `move_register_and_branch`: Copies the contents of one data register into another and then performs a short relative branch to another location.

### Avail

- `parse_command_line_arguments_3`: Parses a string into individual tokens, handling quoted substrings and performing character substitution for specific escape sequences.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `sum_memory_region_sizes`: Iterates through a linked list of memory regions and sums the size of any region whose attribute flags match a provided mask.
- `print_memory_availability_table`: Calculates and prints a table showing available, in-use, and maximum memory block sizes, with optional filtering by chip, fast, or total memory types.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `invoke_and_checksum_stack_2`: Invokes a function pointer stored in a global variable and performs a rolling checksum calculation on the local stack buffer until a null terminator is reached.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### AddBuffers

- `allocate_and_initialize_buffer`: Traverses a linked list to find the maximum value in a specific field, then allocates a new buffer based on that value.
- `move_and_branch_6`: Moves a long value from one register to another and then performs a short branch.

### Break

- `update_linked_list_and_buffer`: Traverses a linked list to find the maximum value in a specific field, then populates a data buffer with a header and a sequence of incrementing values.
- `move_and_branch_7`: Copies the contents of one register to another and executes a relative branch.

### Global

- `execute_initialization_sequence`: Performs a series of function calls and checks a value in memory against a constant to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Assign

- `update_buffer_header_and_copy_data`: Traverses a linked list to find a maximum value, then writes a header and a sequence of incrementing values into a buffer.
- `transfer_register_and_branch`: Copies the contents of one data register to another and performs a short relative branch to another location.

### EndCli

- `update_buffer_and_copy_metadata`: Iterates through a linked list to find a maximum value, then initializes a buffer with a header and increments values in a loop.
- `transfer_register_and_branch_2`: Copies the contents of one data register to another and then performs a short relative branch to another location in the code.

### Dir

- `list_directory_contents_2`: Parses a directory path and constructs a bitmask of search options based on specific characters found in the input string.
- `list_directory_contents`: Iterates through a filesystem directory structure to retrieve and display file information.
- `process_command_input`: Parses user input from a command interface and updates a state structure based on specific key presses.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.
- `calculate_string_length`: Calls a function via a pointer in a global variable and then calculates the length of the resulting null-terminated string.
- `copy_bytes_with_termination`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `compare_strings_case_insensitive`: Compares two byte strings pointed to by two registers character by character.
- `copy_c_string_2`: Advances to the terminating zero of the string at A0, then copies the bytes from A1 onto its end, including the zero.

### setup

- `verify_and_patch_memory`: Performs a series of function calls and checks a value in memory against a constant to update a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.

### Echo

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a buffer with a header and performs a series of memory copies between a source and a destination structure.
- `move_and_branch`: Moves the contents of one register to another and then performs a short relative branch.

### Quit

- `update_buffer_and_copy_data`: Traverses a linked list to find a maximum value, then initializes a buffer with a header and populates it using a loop.
- `transfer_register_and_jump`: Copies the contents of one data register to another and performs a short relative branch into another routine.

### FailAt

- `update_buffer_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field, then updates a buffer header and performs a series of memory copies between a source and a destination structure.
- `transfer_register_and_branch_3`: Copies the contents of one data register to another and then performs a short relative branch to another location.

### Execute

- `traverse_list_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize and fill a data buffer.
- `transfer_register_and_branch_4`: Copies the contents of one data register to another and performs a short relative branch to another location in the code.

### MakeDir

- `find_max_and_initialize_buffer`: Iterates through a linked list to find the maximum value in a specific field, then allocates and initializes a data buffer with a header and sequential values.
- `transfer_register_and_branch_5`: Moves a longword from one data register to another and performs a short relative branch to another location.

### Cd

- `update_buffer_and_copy_data_4`: Iterates through a linked list to find a maximum value, then populates a buffer with a header and a sequence of incrementing values.
- `move_and_branch_3`: Copies the contents of register d1 into register d0 before branching to another location.

### DisplayBack

- `initialize_graphics_state`: Retrieves a pointer from a global variable, calls a function at an offset within that pointer, updates the global variable with a value from the result, and then clears the global variable before calling another function and setting a specific value.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_initialize_buffer_2`: Traverses a linked list to identify the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `move_and_branch_4`: Transfers a long value from d1 to d0 and executes a short branch.

### IconX

- `parse_command_line_arguments`: Parses a command line string into an array of pointers to individual arguments.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `join_strings_into_buffer`: Copies a null-terminated string into a destination buffer and appends a second null-terminated string to it.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
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
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_and_return`: Stores a single byte from a register into a memory location pointed to by another register.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_register_to_pointer`: Stores the contents of a data register into the memory location pointed to by an address held in an address register.

### Copper

- `initialize_system_ports`: Invokes several function pointers stored at offsets within a global structure.

### LoadWB

- `parse_command_line_arguments_2`: Parses a command line string into individual arguments, handling quoted strings and converting escape sequences like newlines and escape characters.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `initialize_workbench_data`: Performs complex data initialization involving string comparisons, linked list construction, and multiple library function calls to manage system resources and memory.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `compare_string_with_global`: Compares a string against a global buffer, potentially inserting a specific five-byte sequence if a null terminator is encountered during the comparison.
- `write_buffer_bytes`: Writes specific byte values to five consecutive offsets relative to a pointer held in a register.
- `compare_memory_blocks_with_null_check`: Checks a byte at a specific location and, if non-zero, compares a block of memory against that location byte-by-byte until a mismatch is found or a zero byte is encountered.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch_3`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over the local stack buffer until a null terminator is found.
- `store_byte_via_pointer_2`: Stores a single byte from a data register into a memory location pointed to by an address register.

### disk-validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### ram-handler

- `execute_indirect_call`: Loads a pointer from a global variable and calls the routine located at that address.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_icon_attribute`: As library code, it either modifies a linked list by inserting a new node or sets a specific bit in a byte at an offset from a pointer.
- `set_icon_attribute`: Performs a bitwise OR operation to set the third bit of a byte at a specific offset from a pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_library_status`: As library code, it updates a structure with values from registers and calls a function pointer, potentially calling another routine if the result is zero.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_data_word`: Contains a single instruction that clears the data register.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `call_recursive_routine`: Library code that calls another routine and then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_modify_icon_attributes`: This library routine acts as a wrapper that calls a function to modify icon attributes.
- `perform_recursive_call`: Library code that executes a recursive call with an incremented parameter until a condition is met.
- `call_other_routine`: This library code calls another routine and then returns.
- `empty_library_stub`: An empty library routine that immediately calls itself.
- `execute_unnamed_routine`: This library routine calls another routine and then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `clear_data_register`: This routine performs a logical OR operation on a data register with zero.
- `update_icon_structure`: Library code that validates a structure, copies a block of data into it, and updates several specific offsets within that structure.
- `copy_and_update_structure`: Library code that copies a block of eleven words from one memory location to another and then updates several fields in a destination structure using values from a source structure, including a potential pointer update.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_structure_attributes`: Updates an icon structure by iterating through its attribute lists and managing child elements or linked data.
- `update_icon_attribute_list`: Updates an icon structure by iterating through a list of attributes or child elements.
- `propagate_icon_changes`: As library code, it iterates through a list of function pointers and updates a structure with returned values.
- `dispatch_icon_events`: As library code, it iterates through a list of function pointers, calling each one and updating a pointer and a counter until a limit is reached, then performs conditional updates to a structure.
- `clear_register_and_branch`: As library code, it clears a register and performs a short branch.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `recursively_update_icon_structure`: This routine traverses a hierarchical icon structure by recursively visiting child elements, sub-menus, and pointer tables.
- `process_icon_structure`: This routine traverses a hierarchical icon structure by checking specific attribute flags and iterating through various pointer tables.
- `iterate_icon_tables`: As library code, it traverses multiple null-terminated tables of pointers and executes a callback for each entry.
- `iterate_icon_table`: Library code that traverses a null-terminated table of pointers, executing a callback for each entry and performing conditional logic based on the results.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `manipulate_stack_and_pointer`: Library code that pushes a register onto the stack, loads a value from the stack into a register, modifies that value, and performs a short branch.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `validate_and_update_icon_attribute`: As library code, it attempts to retrieve a structure and, if successful, performs a series of conditional checks and updates to specific offsets within that structure based on the results of subsequent routine calls.
- `call_routine_with_stack_argument`: As library code, it pushes a pointer to a stack-based structure and calls another routine, then returns a value based on the result.
- `return_zero_3`: Library code that returns a value of zero.
- `update_icon_table`: As library code, it decrements a value through a pointer and uses the result to index into a table to store two 32-bit values at a specific offset.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `modify_icon_attributes`: Library code that retrieves a base address, performs conditional checks, and calls various library functions to update structure attributes or values.
- `branch_to_offset`: Library code that performs a short branch to a relative offset.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `call_subroutine_with_parameters`: As library code, this routine saves registers, retrieves two values from the stack, calls another routine, and then performs conditional branching based on the result.
- `search_for_byte_sequence`: As library code, it calls a routine to check equality, then enters a loop that increments a pointer and checks for a specific byte value.
- `compare_byte_sequence`: As library code, it compares a byte at a pointer to a value in a register and returns zero if they do not match.
- `call_nested_routine`: Library code that saves registers to the stack, loads two values from the stack, pushes a third value, calls another routine, and then restores registers before branching.
- `execute_callback_routine`: As library code, it calls a routine and calculates the difference between a returned value and a value stored on the stack.
- `traverse_linked_list_structure`: As library code, it iterates through a structure by calling a routine to retrieve the next element and performs checks on the returned values.
- `increment_register_and_branch`: As library code, it tests a register and, if non-zero, increments its value before branching.
- `check_register_and_return`: As library code, this routine tests a register and either returns immediately or restores multiple registers from the stack before returning.
- `parse_string_to_integer`: Library code that converts a sequence of numeric characters into an integer value and writes the result into a structure.
- `execute_library_function`: As library code, this routine prepares arguments on the stack, calls another routine, and performs conditional branching based on the return value.
- `initialize_numeric_register`: As library code, it clears a register and branches to another location.
- `parse_numeric_string`: As library code, it converts a string of digits into a numeric value through a loop and performs subsequent checks based on the result.
- `calculate_offset_value`: As library code, it compares a value from the stack against constants and performs arithmetic to calculate a result based on the comparison.
- `return_error_code`: This library routine returns a value representing an error or invalid state.
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
- `set_icon_value`: As library code, it attempts to retrieve a value and, if successful, writes a constant to a specific offset within a structure.
- `return_zero`: Library code that returns zero.
- `call_scheduled_routine`: Library code that pushes arguments onto the stack, calls another routine, and then performs conditional logic based on the result.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `reschedule_library_task`: As library code, it pushes several values from the stack and calls another routine before performing a comparison and a conditional branch.
- `return_void`: Library code that performs an immediate return.
- `rotate_32bit_value_2`: Library code that performs a bitwise rotation on a 32-bit value using a loop and a carry flag.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_indirect`: Library code that stores a byte into a memory location pointed to by a register.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `verify_byte_match`: As library code, it compares a byte from a memory location to a value provided on the stack and decrements a pointer if they match.
- `clear_register_and_branch_2`: This library routine clears a data register and performs a short branch to another location.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `find_null_terminator`: Library code that iterates through memory checking for a zero byte and then adjusts a pointer before branching.
- `check_byte_equality`: As library code, it compares a byte from one memory location to a byte from another, returning zero if they match and the second byte is zero.
- `conditionally_set_register`: This library routine sets a data register to a small value or a negative value based on a comparison and then returns.
- `test_byte_and_branch`: As library code, it checks a byte at a memory location and branches based on whether that byte is zero or if a counter is greater than zero.
- `call_byte_copy_routine`: This library routine calls another routine that copies bytes using two decrementing counters.
- `copy_bytes_with_counters`: Library code that uses two decrementing counters to copy bytes from one location to another in a loop.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_registers_from_stack`: As library code, it saves several registers to the stack and loads three values from specific offsets on the stack into registers before branching.
- `iterate_library_tasks`: As library code, it repeatedly calls a routine and checks the results, updating a value in a structure until a zero value is encountered.
- `return_zero_2`: This library routine performs no operations and returns zero to the caller.
- `modify_icon_structure`: As library code, it performs a series of conditional checks and nested calls to update values within a structure pointed to by a parameter.
- `return_negative_one`: Library code that returns a value of negative one.
- `save_stack_parameters`: This routine saves several registers to the stack and copies three consecutive longwords from the stack into registers.
- `propagate_icon_event`: As library code, it pushes several values onto the stack, calls another routine, and then performs conditional checks on the results.
- `process_event_queue`: As library code, it executes a loop that processes elements from a stack or list, performing several checks and updates to a structure based on the results of nested routine calls.
- `return_error_constant`: This library routine returns a value representing an error or invalid state.

### Global Effect (1992)(Millenium)(M4)[cr](Disk 1 of 3)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_and_loop`: Calls a specific routine four times and then enters an infinite loop.
- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_command_buffer`: Clears a memory region and parses a command string into a structured buffer.
- `cleanup_linked_list`: Checks a condition and calls several functions, including one that iterates through a linked list to free its elements.
- `call_indirect_function`: Loads a function pointer from an offset relative to a value stored on the stack and executes it.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_buffered_operation`: Performs a table lookup to fetch and execute a function pointer with provided parameters, while checking a global status flag and updating a display buffer index.
- `parse_printf_format_specifier`: Parses a printf-style conversion specifier by reading flags, width, precision, and length modifiers.
- `parse_format_string`: Recursively walks a null-terminated string and calls a callback for each character, attempting to handle percent-prefixed format specifiers.
- `parse_command_string`: Parses a space-delimited string into an array of pointers, supporting quoted substrings with internal escape sequences for special characters.
- `decode_and_copy_string_data`: Decodes a string by adjusting byte values based on a bitmask and then copies data from a global variable into local stack buffers.
- `update_memory_block`: A routine that checks a flag in a structure and, if set, retrieves a memory block from a global pointer to update several fields within the structure.
- `update_display_buffer_index`: The routine processes a display buffer structure and an index to update internal counters, flags, and buffer pointers.
- `process_display_buffer`: Updates a display buffer structure by managing control flags, a counter, and a data pointer.
- `iterate_table_entries`: Iterates backwards through a table of entries and calls another routine for each entry that meets specific bitwise criteria.
- `transform_string_bytes`: Iterates through a null-terminated string and modifies each byte based on a bitmask lookup.
- `copy_string_with_limit`: Copies a source string to a destination buffer up to a specified maximum length.
- `format_integer_string`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a string, storing the result in an integer pointer, and returns the number of characters consumed.
- `insert_into_sorted_list`: Inserts a new memory block into a linked list of sorted blocks while updating a global size counter.
- `execute_table_function`: Retrieves an address from a table using an index and calls the function at that address, returning a failure value if the index is invalid or if a global error flag is set.
- `manage_file_access`: Iterates through a table to find a specific entry and attempts to open, lock, or delete a file based on various bitwise flags and parameters.
- `copy_memory_block`: Copies a block of memory from one address to another.
- `free_linked_list_elements`: Iterates through a linked list and calls a function pointer stored in a global variable to free each element.
- `perform_table_seek`: Fetches a function pointer from a table and executes it with provided parameters, while checking a global status flag for errors.
- `allocate_and_link_buffer`: Allocates memory and inserts a new structure into several linked lists within a global structure.
- `manage_sorted_memory_blocks`: This routine traverses a linked list to find and split a memory block of a specific size, updating a global size counter and adjusting pointers accordingly.
- `push_byte_to_queue`: Increments a counter in a global structure and stores a byte into a buffer pointed to by a table entry.
- `execute_empty_recursion`: The routine performs a recursive call to itself using an undefined value from a register.
- `parse_display_command`: The routine parses a command string and uses another routine to perform an action based on the first character and a flag at the second position.
- `clear_display_state`: The routine clears a status flag in a structure and conditionally invokes another routine based on a byte flag and two fields in the structure.
- `traverse_and_call_blocks`: Iterates through a linked list of blocks and calls another routine with specific data from the block if certain bit flags and size conditions are met.
- `perform_write_operation`: Calls a function through a pointer and, if it returns a failure code, retrieves an error value and stores it in a global variable.
- `perform_seek_operation`: Calls a function through a pointer and modifies a parameter based on the result or a third argument, potentially calling another function to adjust the position.
- `handle_file_read_error`: Attempts to execute a function via a pointer stored in a table and checks for a failure result.
- `perform_open_operation`: Calls a function through a pointer and, if it returns zero, retrieves an error value and stores it in a global variable.
- `lock_and_open_file`: Attempts to lock a resource and then open it, handling errors by unlocking or storing an error value if the open fails.
- `delete_and_open_file`: Attempts to lock and delete a file before attempting to open it, managing error states and unlocking if necessary.
- `close_file_handle`: Calls a function through a pointer to close a resource and returns zero.
- `copy_and_call_function`: Copies a sequence of bytes from a source pointer to a local stack buffer and then executes a function pointer retrieved from a global variable.
- `compute_fixed_point_product`: Takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point`: Performs a normalized fixed-point division on two 32-bit values, handling sign logic and bit shifting to maintain precision.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `compare_byte_with_constant`: Loads a pointer from a table, retrieves a byte from an offset of that pointer, and compares it against a constant value.
- `validate_and_execute_task`: Checks a task structure for specific properties and executes a function pointer if certain conditions are met.
- `call_intuition_library`: Calls a function from the intuition library and stores the result in a global variable.
- `update_and_test_status`: Stores a calculated value into a global variable and calls another routine, returning zero or a maximum unsigned value based on the result.
- `return_zero_constant`: The routine returns a constant zero value without performing any other operations or memory accesses.
- `close_table_entry`: Retrieves an entry from a table and either calls a function through a pointer within that entry or clears the entry, depending on a bit flag.
- `get_table_entry_address`: Checks if an index is within valid bounds and if the corresponding entry in a table is non-zero.
- `check_and_clear_callback`: Checks a global function pointer and a structure field for specific bitwise flags and non-zero values.
- `find_or_allocate_block`: Traverses a linked list to find a block, and if none is found, calls another routine to allocate a new block and initializes it with zeros.
- `perform_recursive_call_2`: Stores two values into a structure and then performs a recursive call to itself.
- `read_byte_sequence`: This routine decrements a counter and reads bytes from a memory location into a destination buffer until a specific value is encountered or the counter reaches zero.
- `copy_string_to_buffer`: Iterates through a null-terminated string, copying characters to a destination buffer or calling another routine if a counter reaches zero, and returns the string length.
- `clear_display_state_alt`: Clears a status flag in a structure and conditionally invokes another routine based on a byte flag and two fields in the structure.
- `copy_null_terminated_string_2`: Copies characters from a source pointer to a destination pointer until a null terminator is encountered.
