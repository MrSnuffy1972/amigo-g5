# Game 1284 (a-train (1992)(ocean)(disk 1 of 2)[h skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01284). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903312 top-level bytes in 95 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 66 of 66 units identical (bytes and relocations), 27 raw-word lines |
| Functions named and described | pass, 4022 of 4022 named (0 author symbols), 4022 of 4022 described (172 by rule, 3714 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1903 of 1903 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 37376 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 93833 of 93839 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12281 |
| code | 299848 |
| data | 471580 |
| padding | 22835 |
| unused | 96768 |

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
| EpsonQ | 15 | 563 | 42 |
| CBM_MPS1000 | 15 | 562 | 43 |
| HP_LaserJet | 15 | 604 | 47 |
| EpsonX[CBM_MPS-1250] | 15 | 702 | 51 |
| Okimate_20 | 14 | 462 | 30 |
| HP_DeskJet | 15 | 604 | 47 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf:Installer | 6 | 45 | 2 |
| icon.library | 10 | 1375 | 109 |
| info.library | 25 | 2598 | 167 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 76 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf:A-Train | 3 | 55618 | 2429 |
| More | 19 | 2290 | 76 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 32 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf bytes 1024-15360 (code never loaded by a reference run) | 1 | 0 | 0 |
| A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf bytes 878080-901120 (code never loaded by a reference run) | 1 | 25 | 1 |

## Cases handled

- a-train (1992)(ocean)(disk 1 of 2)[h skr]/A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf: file devs/clipboard.device (header 6372 bytes, blocks hold 6832): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address (play-rs-m1500): 31 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- a-train (1992)(ocean)(disk 1 of 2)[h skr]/A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf: bytes 1024-15360 look like code no run loaded: recovered position-independently (load address unknown)
- a-train (1992)(ocean)(disk 1 of 2)[h skr]/A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf: bytes 878080-901120 look like code no run loaded: recovered position-independently (load address unknown)
- a-train (1992)(ocean)(disk 1 of 2)[h skr]/A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf: 2 logged loads cover 37376 bytes of the image
- a-train (1992)(ocean)(disk 1 of 2)[h skr]/A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf:Installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 37376 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `parse_command_line_arguments_3` (FF): Parses a command line string to identify and extract arguments starting with a hyphen. Kind: command line argument validation/sabotage. Pass path: The loop continues to parse arguments, eventually reaching the `u7__l0_0146` label and returning via the `puVar9` pointer (likely returning a status code or pointer to the parsed data)..
- `compare_font_dimensions` (FF): Compares dimensions and properties of two structures and updates a character map if they match. Kind: sabotage path. Pass path: the branch where iVar2 != 0 and the dimension comparisons match, leading to param_1[1] = iVar2.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: If a pointer is not null, it copies sixteen words from a source offset to a destination buffer and calls another routine.

### Stack

- `traverse_list_and_expand_buffer`: Traverses a linked list to find the maximum value in a specific field, then expands a data buffer by writing a header and copying a block of sixteen words from a source to a destination.
- `copy_data_block_2`: Copies a block of sixteen words from a source address to a destination address if the destination pointer is valid.

### Else

- `find_max_and_populate_buffer_4`: Iterates through a linked list to identify the maximum value in a specific field, then uses that value to calculate offsets and populate a data structure.
- `duplicate_data_sequence`: Performs a sixteen-byte memory copy from a source location to a destination and then calls a subroutine.

### Mount

- `traverse_list_and_update_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_3`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.

### SetPatch

- `patch_dos_library_structures`: This routine attempts to locate and modify internal structures within the DOS library.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `update_task_status`: Retrieves a task pointer, modifies a value at an offset within that task's structure using a specific constant, and calls an indirect routine.
- `execute_conditional_callback_chain`: The routine calls another routine, checks a global variable, and if non-zero, calls another routine with an input parameter, passes that result to a third routine, and finally calls another routine unconditionally.
- `manage_resource_nodes`: Iterates through a linked list of nodes to find specific entries based on string matching and bitwise conditions.
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
- `calculate_checksum_and_call`: The routine iterates through a table of short integers to calculate a sum, stores the bitwise NOT of that sum into a structure, and then performs an indirect call.
- `patch_resource_and_toggle_interrupts`: The routine calls a routine through a table and checks if the return value is non-zero and if a size field is small.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `dispatch_on_condition`: Checks if a value matches a global variable or negative one, and if not, calls a routine via a pointer in a global structure.
- `dispatch_if_not_negative`: Checks if a value in a register is not equal to negative one, and if so, calls another routine via a pointer stored in a global variable.
- `enable_interrupts_and_call`: Sets a global variable, modifies a memory location, and performs an indirect call while interacting with the interrupt enable bit of the hardware.
- `execute_indirect_call`: The routine performs an indirect call to a function pointer located at an offset from a value in a register.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `return_value`: The routine returns a value to the caller.
- `pass_input_value`: Returns the value contained in a data register without modification.
- `call_other_routine_2`: Calls another routine multiple times before returning a value.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `check_patch_condition`: Checks a value in a global variable against a constant and calls a function through a pointer at a fixed offset from that global variable.
- `copy_list_elements`: Copies a sequence of elements from one list to another by iterating through a structure and updating pointers via library calls.
- `mask_and_return_value`: Iterates through a loop to call a routine via a pointer and returns a value from a pointer with the highest bit set.
- `get_masked_value`: Calls an indirect routine and returns the result of a value from a register with the highest bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `restore_processor_state`: Moves the value in a0 to a5, pushes the value in d1 to the stack at offset 2, masks the stack pointer to a 12-bit word boundary, and performs a return from exception.
- `find_node_by_string`: The routine iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `reset_global_variable`: Sets a global variable to zero and returns its previous value.
- `execute_conditional_callback`: Calls a routine with a value from an offset in a pointer.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `calculate_word_offset`: Reads a value from a pointer and performs bitwise shifts and sign extensions to calculate a new pointer address.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `traverse_linked_list_3`: Performs bitwise comparisons on data and traverses a linked list to call another routine based on specific bit flags.
- `restore_registers_and_return`: Moves a longword from a pointer in a2 to a1, restores registers a2 and a3 from the stack, loads a longword from a constant into the stack, and returns.
- `select_register_value_and_return`: Moves the contents of a0 to d0 and returns if a0 is zero, otherwise moves the contents of a1 to d0 and returns.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Path

- `find_max_and_initialize_buffer_6`: Iterates through a linked list to determine the maximum value in a specific field, then allocates and initializes a data buffer.
- `copy_and_call_routine`: Copies a sequence of four words from a table to a destination and then calls another routine.

### Break

- `process_linked_list_and_expand_buffer`: Traverses a linked list to find the maximum value in a specific field, then expands a data buffer by writing a header and copying data blocks.
- `copy_memory_block_10`: Copies a fixed number of bytes from a source address to a destination address.

### FF

- `parse_command_line_arguments_3`: Parses a command line string to identify and extract arguments starting with a hyphen.
- `execute_indirect_function_pointer_19`: Loads a function pointer from a global variable and executes it.
- `dispatch_sequence_of_routines`: Writes a series of values into a structure via a pointer and calls a sequence of routines, followed by conditional calls to two other routines using pointers from a global variable.
- `call_indirect_address`: Calls a routine by dereferencing a pointer stored at a specific offset from a register.
- `measure_string_then_dispatch`: Walks the string passed in param_1 up to its terminating zero byte, then branches on a global flag: if the flag is zero it calls two indirect library-table routines and returns the result of the second; otherwise it runs an indirect call sequence, stores values into a shared structure, invokes a helper for each of four entries, and returns the stored value.
- `execute_conditional_logic_8`: Iterates through a null-terminated string and executes a series of function calls through a pointer table based on specific conditions.
- `swap_table_elements_2`: Swaps word-sized values between two memory-resident tables using a loop that runs ten times.
- `swap_table_elements`: Swaps word-sized values between two memory-resident tables or arrays using a counter-controlled loop.
- `initialize_and_copy_structure_data`: Initializes specific fields within a structure and copies a sequence of six bytes from a source location to the structure's offset.
- `cleanup_font_resources`: Checks bitmasks in a global variable to conditionally execute a series of function pointers and clean up font-related structures.
- `update_char_map`: Writes a character code or a modified character code to a memory location and recursively calls other routines or updates a pointer-based structure.
- `process_recursive_pointer`: If a value at an offset or a certain memory location is non-zero, it calls another routine with certain parameters and then clears that memory location.
- `compare_font_dimensions`: Compares dimensions and properties of two structures and updates a character map if they match.
- `execute_multiple_function_pointers`: Loads and executes a series of function pointers from a structure, including conditional execution based on a pointer's nullity.
- `update_structure_member_and_loop`: Copies a value from a structure member to a remote memory location, clears that member if it is non-zero, and then enters a loop that executes further routines until a returned value or a checked value becomes zero.
- `traverse_linked_list_recursively`: Recursively traverses a linked list while copying a value from a structure member to a global variable and clearing that member if it is non-zero.
- `update_function_table_state`: Updates a structure member with a value from a remote memory location and executes certain function pointers based on a bitmask check.
- `execute_two_routines`: Executes a routine with a provided parameter and then executes a second routine with a zero parameter.
- `initialize_jump_table_and_buffer`: Initializes a jump table with a specific pattern and copies a sequence of instructions from a routine into a target buffer.
- `call_indirect_function_52`: Calls a function through a pointer located at an offset from a register.
- `update_memory_buffer`: Copies a block of data from a source pointer, whose length is determined by a value in a global variable, to a destination buffer.
- `manage_resource_status`: Checks a value in a global structure, clears it if non-zero, and conditionally calls two different indirect functions based on the results of a comparison.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a2, then returns.
- `copy_stack_pointer`: Copies the value from the stack pointer to a register.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d1, then copies (a0)+ to d1.
- `copy_scaled_byte`: Shifts a word left, uses it to calculate a relative address, and moves a byte from that address to a destination pointer.
- `expand_data_table`: Iterates through a source list to populate a destination table with values derived from a lookup table, then updates a global variable and calls several other routines.
- `call_indirect_offset_162`: Calls a routine located at an offset from a pointer stored in a table.
- `call_indirect_via_offset_6`: Retrieves a function pointer from a table using an offset from a base address and executes it.
- `execute_indirect_function_pointer_16`: Retrieves a function pointer from a nested offset within a structure pointed to by a register and executes it.
- `call_indirect_via_offset_7`: Accesses a pointer through a double offset from a base address and executes the function at that location.
- `call_indirect_via_offset_8`: Retrieves a function pointer from a table using an offset from a base address and executes it.
- `call_indirect_function_via_offset_2`: Fetches a function pointer from a table using an offset from a base address and executes it.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `execute_indirect_function_52`: Retrieves a function pointer from a nested offset within a structure pointed to by a register and executes it.

### EndSkip

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Resident

- `find_max_and_prepare_buffer_4`: Traverses a linked list to find the maximum value in a specific field, then initializes a data structure with a header and populates it by copying blocks of data from a source to a destination buffer.
- `copy_memory_block_11`: Copies sixteen bytes from a source location to a destination location if the destination pointer is not null, then calls another routine.

### Assign

- `traverse_list_and_expand_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then expands a data buffer by writing a header and copying data blocks.
- `copy_data_block_3`: Performs a sixteen-word memory copy from a specified offset to a destination and then executes another routine.

### EndCLI

- `find_max_and_populate_buffer`: Traverses a linked list to determine the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `copy_data_block_4`: Checks if a destination pointer is non-zero, then copies sixteen words from a source location before calling another routine.

### Dir

- `find_max_and_initialize_buffer`: Iterates through a linked list to find the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `copy_data_block_5`: Transfers sixteen words from a source pointer to a destination pointer and executes a subsequent routine.

### NewCLI

- `find_max_and_populate_buffer_2`: Iterates through a linked list to determine the maximum value in a specific field, then initializes a data structure with a header and a sequence of incrementing values.
- `copy_data_block_6`: Copies sixteen words from a source offset to a destination buffer and calls a function pointer.

### NewShell

- `find_max_and_prepare_buffer_2`: Iterates through a linked list to identify the maximum value in a specific field.
- `copy_data_block_7`: Iterates sixteen times to copy words from a source address to a destination address if the destination is not null.

### Quit

- `find_max_and_prepare_buffer_3`: Iterates through a linked list to identify the maximum value in a specific field and then initializes a data buffer.
- `copy_data_block_8`: Copies a sequence of sixteen words from a source to a destination and then calls another routine.

### Echo

- `traverse_list_and_populate_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_data_block_9`: Copies sixteen words from a source memory location to a destination and executes another routine.

### Lab

- `return_zero`: Empty routine that returns zero.

### GetEnv

- `find_max_and_initialize_buffer_2`: Traverses a linked list to identify the maximum value in a specific field and then initializes a data structure by writing a header and copying a block of data.
- `copy_memory_block`: If a pointer is not null, it copies sixteen bytes from a source offset to a destination buffer and calls another routine.

### Skip

- `find_max_and_populate_buffer_3`: Iterates through a linked list to identify the maximum value in a specific field and then writes a header and a sequence of incrementing values into a buffer.
- `transfer_data_chunk`: Copies a fixed-size block of data from a source address to a destination address before executing a function pointer.

### SetEnv

- `find_max_and_initialize_buffer_3`: Traverses a linked list to find the maximum value in a specific field and then populates a data structure with a header and a sequence of incrementing values.
- `move_memory_segment`: Checks for a valid pointer to copy sixteen bytes of data to a target location and invokes a function.

### Failat

- `find_max_and_initialize_buffer_4`: Traverses a linked list to find the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `write_data_buffer`: Transfers sixteen bytes from a source pointer to a destination and executes a stored function pointer.

### Execute

- `find_max_and_initialize_buffer_5`: Traverses a linked list to identify the maximum value in a specific field and then initializes a destination buffer with a header and a sequence of incrementing values.
- `relocate_byte_array`: Copies sixteen bytes of data from a specific offset to a destination and then calls another routine.

### SKID_ROW

- `invoke_sequential_pointers`: Invokes a sequence of four function pointers stored in a global structure, passing the result of the first call as the base address for the second and third, then returns zero.

### Copy

- `traverse_list_and_copy_block`: Iterates through a linked list to find the maximum value in a specific field, then initializes a header and performs a block memory copy between two buffers if a condition is met.
- `fill_destination_buffer`: Copies sixteen bytes from a source address to a destination and executes a function pointer.

### Makedir

- `allocate_and_initialize_data_block`: Traverses a linked list to find the maximum value in a specific field, then allocates a memory block based on that value.
- `replicate_memory_contents`: Copies a sixteen-byte block of data to a destination and then calls a subroutine.

### CD

- `find_max_and_populate_buffer_5`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `stream_data_bytes`: Copies sixteen bytes from a source pointer to a destination and executes a function pointer.

### Date

- `find_max_and_populate_buffer_6`: Iterates through a linked list to find the maximum value in a specific field, then uses that value to initialize a data structure and fill a buffer with a sequence of incrementing words.
- `copy_data_block_10`: Copies sixteen words from a source address to a destination address if the destination pointer is not null.

### EndIf

- `return_zero_2`: An empty routine that returns zero.

### If

- `traverse_list_and_populate_buffer_2`: Traverses a linked list to find the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_2`: Copies sixteen bytes of data from a memory location to a destination pointed to by a stack value.

### IconX

- `initialize_application_environment`: Prepares the execution environment by retrieving system state, allocating memory, and parsing command-line arguments.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start, referencing text for Workbench use only and a console window specification.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a 260-byte buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `concatenate_strings_and_call_helper`: Copies a source string into a destination buffer, appends a second string to that same buffer, and then calls another routine using the buffer pointer and the new total length as arguments.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_indirect_pointer`: Calls a function pointer retrieved from an offset within a global structure.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from a location within a global structure and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `execute_indirect_function_pointer`: Fetches a function pointer from an offset within a global table and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_indirect_function`: Fetches a function pointer from an offset within a global table and executes it.
- `call_indirect_function_2`: Fetches a function pointer from a table at a specific offset and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a location within a global data structure and executes it.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a location offset from a global variable and executes it.
- `invoke_global_and_verify_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer`: Stores a byte from a data register into a memory location pointed to by an address register.

### LoadWB

- `determine_startup_environment`: Determines whether to launch the application via a command-line interface or the Workbench environment.
- `update_and_cleanup_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `start_tool_from_resident_tag`: XOR-decodes a global string with one key applied twice, which leaves it unchanged, and allocates a 160-byte block.
- `compare_null_terminated_strings`: Compares two zero-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_function`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_function_2`: Fetches a function pointer from an offset within a table and executes it.
- `call_indirect_function_4`: Fetches a function pointer from a table and executes it.
- `call_library_function_3`: Fetches a function pointer from an offset within a table and executes it.
- `call_indirect_function_5`: Fetches a function pointer from a location within a global table and executes it.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_indirect_function_6`: Fetches a function pointer from a global table and executes it.
- `call_indirect_handler`: Calls a function through a pointer located at an offset from a global variable.
- `call_indirect_handler_2`: Calls a function through a pointer located at a different offset from a global variable.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `verify_workbench_version`: Compares a string against a global constant and writes spaces into a memory location if a match is found.
- `write_space_sequence`: Stores a sequence of four space characters into a memory location starting at an offset from a pointer.
- `compare_memory_buffers`: Compares two null-terminated strings byte by byte until a mismatch or a null terminator is encountered.
- `stub_return_only`: Contains only a return instruction, with no calls, memory access or hardware use, so it does no work.
- `call_reschedule_2`: Calls a function pointer retrieved from a table offset.
- `execute_indirect_function_5`: Fetches a function pointer from a global table using a fixed offset and executes it.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_indirect_function_3`: Fetches a function pointer from a global table and executes it.
- `call_other_routine_with_stack_2`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `invoke_and_verify_stack_checksum_2`: Invokes a function pointer stored in a global variable and performs a rolling checksum calculation over a stack-based buffer until a null terminator is reached.
- `store_byte_via_pointer_2`: Stores a byte from a register into a memory location pointed to by another register.

### SetClock

- `parse_command_line_arguments`: Parses a command line string into individual tokens, handling quoted substrings and converting specific character sequences into newline or escape characters.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `manage_clock_operations`: Performs a series of conditional checks and nested calls to manage system time and date settings based on provided parameters.
- `compare_global_bits`: Compares the lower bits of a global variable with a certain value and returns the result.
- `write_global_and_check`: Writes several constant values to a global array and returns a comparison of a global variable's bits.
- `recursively_clear_io_request`: If a pointer is returned, it writes a constant value to a field within a structure, recursively calls itself on that pointer, retrieves a value from another field, and calls itself again.
- `update_structure_and_sum`: Writes a value to a structure field, calculates a weighted sum, and clears a longword field in a structure.
- `delete_io_request_recursively`: Obtains a pointer via a library call, then conditionally calls another routine and deletes the resulting I/O request or pointer.
- `delete_io_request`: If a pointer is non-zero, it recursively calls a routine on a nested pointer before clearing a field and deleting a port or I/O request.
- `calculate_weighted_sum`: Calculates a complex weighted sum or checksum using values or or pointers from a table and returns the result.
- `populate_timing_table`: Calculates a remaining value by decrementing a counter based on conditional checks and then writes a series of retrieved values into a table in memory, terminating with a zero-initialization.
- `compare_strings_case_insensitive_4`: Performs a case-insensitive comparison between two strings and returns a result indicating equality or relative order.
- `compare_strings_case_insensitive_2`: Compares two strings by iterating through them and performing case-insensitive character comparisons.
- `compare_and_return_one`: Compares a byte in a register against a value in memory and returns one if the register value is not less than the memory value, otherwise returns negative one.
- `return_constant_one`: Returns the integer value one.
- `convert_to_uppercase`: Converts a character to uppercase by adding thirty-two if it is within the range of 'A' to 'Z', otherwise returns the sign-extended input.
- `tiny_ext_ext_rts`: Small routine of 3 instructions that sign-extends d0, then sign-extends d0, then returns.
- `decrement_and_check_flag`: Updates a memory location with a constant and then executes a loop that decrements a register while checking a bit in a memory location.
- `write_constants_to_memory`: Writes two constant values through a pointer-based address after a loop that decrements a counter based on a bit test of a memory location.
- `write_constant_to_pointer`: Writes two constant values through a pointer-based address.
- `update_memory_with_constant`: Writes a constant value to two different offsets relative to a global pointer.
- `write_constant_to_offset_2`: Writes a constant value to a memory location at a fixed offset from a pointer after a loop that decrements a counter based on a bit test of a memory location.
- `write_constant_to_offset`: Stores the constant value four into a memory location at a fixed offset from a pointer held in a register.
- `call_other_routine_with_stack_3`: Passes two parameters and a pointer to a local stack slot to another routine before returning.
- `execute_indirect_function_6`: Fetches a function pointer from a global table and executes it.
- `call_alloc_entry_2`: Calls a function pointer located at an offset from a global variable.
- `execute_indirect_function_7`: Fetches a function pointer from a global table at a specific offset and executes it.
- `execute_indirect_function_8`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_indirect_function_7`: Fetches a function pointer from a global variable at a specific offset and executes it.
- `call_library_function_4`: Fetches a function pointer from an offset within a global table and executes it.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_function_via_pointer`: Fetches a function address from a specific offset within a global structure and executes it.
- `execute_indirect_function_9`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_indirect_function_8`: Fetches a function pointer from a global table and executes it.
- `invoke_and_checksum_stack`: Invokes a function pointer stored at an offset from a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer_2`: Stores a byte from a register into the memory location pointed to by another register.
- `modular_inverse_32bit`: Performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `call_modular_inverse_helper`: Calls a single helper routine and returns.
- `call_helper_return_d1`: Calls the helper routine modular_inverse_32bit and returns the 32-bit value the helper leaves in data register D1 as the result.
- `negate_result_on_flag_4`: Calls another routine and negates the returned value if the helper leaves a non-zero result in an address register.
- `call_and_negate_on_flag_5`: Calls a helper routine and returns its first result, or the negation of that result if a second result in an address register is nonzero.
- `allocate_indexed_record_block`: Obtains a slot index from another routine and, when valid, allocates a 34-byte block through a library call.
- `release_record_chain`: Recursively releases a linked chain of records by processing the linked record first.
- `store_word_and_pointer_in_node`: Calls a library-backed allocation routine and, if a block is returned, stores a byte, a word, and a pointer into fixed offsets within that block.
- `reset_node_and_release_it`: If a pointer is non-null, it sets a byte field to 255 and clears two longword fields before passing the pointer and a word field to another routine.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Wait

- `find_max_and_populate_buffer_7`: Traverses a linked list to identify the maximum value in a specific field and then writes a header and a sequence of incrementing values into a destination buffer.
- `copy_memory_block_4`: Copies sixteen words from a source address to a destination address using a loop.

### Version

- `parse_command_line_arguments_2`: Parses a command line string into individual arguments by handling spaces, tabs, and quoted substrings.
- `dispatch_message_callback`: Updates a message structure with a new parameter and executes a sequence of function calls from a table based on the contents of the message fields.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_reply_2`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Converts a numeric string into a signed integer by processing an optional sign character followed by a sequence of digit characters.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_3`: Calls a function pointer stored at a specific offset within a global structure.
- `execute_indirect_function_pointer_2`: Fetches a function pointer from a table using a fixed offset and executes it.
- `recurse_with_stack_slot_2`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `call_library_function_a`: Calls a function pointer located at a specific offset within a global structure.
- `call_library_function_b`: Calls a function pointer located at a different specific offset within a global structure.
- `call_indirect_pointer_2`: Calls a function pointer located at a fixed negative offset from a global variable.
- `execute_indirect_function_10`: Retrieves a function pointer from an offset within a global table and executes it.
- `call_indirect_function_9`: Fetches a function pointer from a specific offset within a global structure and executes it.
- `call_indirect_function_10`: Fetches a function pointer from a global table using a fixed offset and executes it.
- `dispatch_then_measure_string_length`: Calls a function pointer fetched from a global-relative offset with no arguments, then scans a 140-byte stack buffer for its null terminator while counting down a 32-bit counter from all-ones, and if the complemented count is non-zero it forwards the buffer, the length, and the routine's parameter to another routine.
- `store_byte_to_pointer_3`: Stores a single byte from a register into a memory location pointed to by a register.

### Preferences

- `parse_command_line_string`: Parses a string by iterating through characters to extract tokens, handling quoted substrings and converting specific character sequences into newline or escape characters.
- `execute_conditional_callbacks`: Stores a parameter into a local variable and executes two different sets of function pointers stored in global variables if they are non-zero.
- `invoke_two_function_pointers`: Calls two function pointers located at fixed negative offsets from a register.
- `execute_conditional_logic_3`: Calls a routine via a pointer from a library, updates a global variable, and conditionally executes several other routines using values from a local variable.
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
- `process_preference_settings`: Iterates through a list of preference entries to validate and apply configuration values.
- `initialize_preference_system`: Allocates memory and initializes a large table of preference entries by populating them with values from a global array.
- `handle_preferences_menu`: Loads a preferences block from a global table and, if present, updates the relevant configuration values and display settings, then runs a message loop that dispatches menu selections to their handlers and replies to messages until the dialog is closed.
- `cleanup_resources_and_reset_interrupts`: Frees a fixed-size memory block based on global counters, traverses and frees a linked list of memory blocks, closes libraries, and resets interrupt vectors.
- `set_preference_values`: The routine calls another routine twice with different first arguments, passing a calculated address from a global variable and the input parameter.
- `iterate_and_validate_params`: Iterates through a sequence of word-sized values from a memory block, passing each to another routine along with a constant and an index.
- `validate_and_pass_params`: Zeroes a parameter if it exceeds a limit, then decomposes another parameter into its nibbles and passes the parts to a library routine.
- `initialize_preference_entries`: Walks through a table of pointers, copying values from a global variable array into each structure's fields and clearing several fields beforehand.
- `process_preference_selection_2`: Handles a specific preference menu selection by branching on an input parameter to configure display modes, manage memory allocations, or update configuration tables.
- `process_preference_selection`: Reads a selection index from a data structure and dispatches to specific handlers to update global configuration variables, performing arithmetic adjustments, memory copies, and indirect calls based on the selected option type.
- `process_preference_buffers`: Checks whether a short parameter is nonzero and, if so, invokes another routine with a base value, then processes three memory locations by extracting nibbles from a word and clamping a global value into a signed range before storing the results.
- `apply_preference_clamps`: Clamps input parameters to specific signed ranges and adjusts them based on offset values within a data structure if a condition is met.
- `clamp_and_apply_preferences`: Clamps two byte parameters to a signed range and adjusts them based on offset values within a data structure if a condition is met, storing the results back.
- `return_constant_based_on_input`: Returns the value three when the input parameter equals nine, and returns four for any other input.
- `return_constant_based_on_parity`: Returns one constant when the least significant bit of the input is zero, and returns a nearby constant otherwise.
- `compare_strings_case_insensitive`: Compares two null-terminated strings character by character, uppercasing each byte before comparing, and reports whether they match.
- `forbid_exec`: Calls the function located at an offset from a global pointer to forbid execution.
- `permit_exec`: Calls the function located at an offset from a global pointer to permit execution.
- `allocate_memory`: Calls the function located at an offset from a global pointer to allocate memory.
- `free_memory`: Calls the function located at an offset from a global pointer to free memory.
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
- `call_library_offset_78_alt`: Calls a function at a specific offset from a different global pointer using a library offset of negative seventy-eight.
- `call_library_offset_90`: Calls a function at a specific offset from a different global pointer using a library offset of negative ninety.
- `call_indirect_function_11`: Calls a function through a pointer stored at an offset from a global variable.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `allocate_indexed_record_block_2`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block through a library call with flags 65537.
- `release_record_chain_2`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `format_date_string`: Initializes a memory buffer with date separator characters and a null terminator.
- `parse_date_string`: Iterates through a global string buffer to validate and extract date components.
- `convert_numbers_to_ascii`: Splits each of three integer parameters into tens and units digits and stores the corresponding ASCII characters into consecutive global memory locations.
- `apply_ascii_configurations`: Calls a library function with several constant arguments, copies ASCII values from global memory into other global variables, and computes a table index from digit values before making a final call.
- `calculate_timer_intervals`: Computes timer intervals by parsing ASCII date/time components, performing arithmetic adjustments, and configuring timer device requests.
- `validate_and_increment_preference_values`: Validates several two-digit numeric values stored in memory by checking if they fall within specific ranges and parity constraints.
- `clamp_and_convert_values`: Clamps two integer parameters to specific ranges and then calls a conversion routine to transform them into ASCII characters.
- `clamp_and_convert_values_2`: Adjusts several character-based values to ensure they fall within specific bounds and then calls another routine with the resulting numeric values.
- `clamp_and_convert_values_alt_2`: Clamps two integer parameters to specific ranges, ensuring they are within bounds, and then calls a conversion routine to transform them into integers.
- `clamp_and_convert_values_alt`: Clamps two integer parameters to specific ranges, ensuring they are within bounds, and then calls a conversion routine to transform them into ASCII characters.
- `initialize_timer_device`: Opens the timer device and allocates an extended I/O request block, then configures the block's fields and waits for a timeout condition before returning a success status.
- `manage_io_timer_loop`: Performs bitwise checks on a global variable and structure fields within a loop, conditionally calling other routines and updating a counter.
- `manage_timer_loop`: Opens a device and enters a loop that performs bitwise checks and updates a global variable.
- `process_preference_loop`: Executes a loop that performs bitwise operations on a global variable and calls various routines based on the resulting flags and structure members.
- `return_from_subroutine`: This is an empty routine that performs no operations before returning.
- `initialize_timer_task`: Opens the timer device, allocates an I/O request, and starts a background task that loops forever handling timer signals and I/O.
- `terminate_io_and_task`: Checks a bit in a global structure and calls a routine with a stored pointer if the bit is not set.
- `setup_io_request`: It stores two pointer values and a constant into fixed offsets of a memory structure passed as an argument.
- `check_global_flag_change`: It reads a global variable, calls a delay function, and returns a boolean indicating whether the lower four bits of that global variable have changed.
- `wait_for_condition_timeout`: It conditionally writes constant values to a global structure, sets a bit in a word within that structure, and loops decrementing a counter until a specific bit in the structure is cleared or the counter expires.
- `clear_structure_bit`: Clears a specific bit within a word of a global structure.
- `save_system_configuration`: Opens a system configuration file for writing and closes it if opening fails.
- `read_configuration_file`: Attempts to open a file specified by a string, reads a fixed amount of data into a buffer, and closes the file.
- `return_long_value`: Moves the contents of register d2 into d0 and returns.
- `call_indirect_function_12`: Executes a routine located at a specific offset from a global variable.
- `call_indirect_function_13`: Performs an indirect call to a routine using an offset from a global variable.
- `call_init_code`: Library stub: loads its arguments and the exec library base, calls InitCode and returns its result to the caller.
- `call_indirect_function_14`: Invokes a function pointer retrieved from a global variable at a fixed offset.
- `call_indirect_function_15`: Calls a routine via a pointer found at a specific offset within a global variable.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_indirect_function_16`: Redirects execution to a routine stored at an offset from a global variable.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `call_set_int_vector`: Library stub: loads its arguments and the exec library base, calls SetIntVector and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_allocate`: Library stub: loads its arguments and the exec library base, calls Allocate and returns its result to the caller.
- `call_indirect_function_17`: Executes a subroutine by dereferencing a pointer at a specific offset from a global variable.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `execute_indirect_function_11`: Fetches a function pointer from an offset within a global variable and executes it.
- `call_free_entry`: Library stub: loads its arguments and the exec library base, calls FreeEntry and returns its result to the caller.
- `call_indirect_function_18`: Performs an indirect jump to a routine using a pointer from a global variable.
- `call_indirect_function_19`: Calls a routine through a pointer at a specific offset from a global variable.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_add_library`: Library stub: loads its arguments and the exec library base, calls AddLibrary and returns its result to the caller.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `forward_library_argument`: Passes a library base pointer from a global variable and the address of a stack argument to a library call stub.
- `process_preference_list`: Iterates through a linked list of structures to execute specific logic based on field values.
- `iterate_preference_list`: Iterates through a linked list of structures, executing logic based on field values and updating pointers.
- `traverse_preference_linked_list`: Iterates through a linked list of structures, updating pointers and executing logic based on specific field values.
- `process_preference_command_2`: Iterates through a linked list of structures and executes different code paths based on a field value within each structure.
- `call_subroutine_and_return`: Pushes two arguments onto the stack, calls a routine at a fixed address, clears d0, and returns.
- `pass_argument_to_empty_routine`: Passes the address of a global variable to another routine and then returns.
- `process_preference_loop_2`: Iterates through a linked list of structures, executing logic based on field values and updating pointers until a specific condition is met.
- `return_immediately_2`: This routine performs no operations and returns to its caller immediately.
- `copy_struct_fields_tail`: Copies five word-sized fields from specific offsets in a source structure to the corresponding offsets in a destination structure.
- `process_preference_command`: Reads values from a pointer-based structure and executes different code paths in a switch statement based on a specific field.
- `call_routine_with_argument`: Calls another routine while passing the address of a global variable as an argument.
- `return_immediately_3`: This routine is empty and performs no operations before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `configure_intuition_object`: It loads an object pointer from a global variable and sets one of its fields to one.
- `find_table_index`: Iterates through a table of words to find the index of the largest element that is less than the provided input.
- `find_index_in_table`: It iterates through a table of words until it finds a value greater than or equal to the input, returning the index of the previous element.
- `decrement_register_value`: The routine returns the value of a specific register minus one.
- `initialize_preference_buffers`: It clears a global flag, then loops over a table of six entries, calling a helper routine to allocate or retrieve buffers for each entry and storing the result.
- `load_intuition_library`: The routine calls another routine with a pointer to a global variable and an integer argument.
- `invoke_library_vector_2`: Checks a bit in a global variable and calls a library routine using an offset derived from another global variable.
- `invoke_library_vector`: Acquires a library semaphore and invokes a library vector using an offset derived from a global variable.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `parse_dos_directory_to_table`: Iterates through a directory using DOS library functions to count and extract entries.
- `compare_string_prefixes`: Compares two null-terminated strings character by character.
- `allocate_and_copy_entry`: Allocates a memory block via the exec library and stores the returned pointer in a global variable.
- `free_linked_list_entries_2`: Iterates through a linked list of memory blocks, freeing each block using the exec library until the end of the list is reached.
- `search_and_clear_buffer`: Iterates through a list of pointers to find a non-zero entry, then clears a local buffer with zeros or spaces depending on the search result.
- `check_string_match`: Initializes a local buffer with spaces and copies characters from a global variable into it.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_offset_72`: Calls a function at a specific offset from a global pointer using a library offset of negative seventy-two.
- `call_library_offset_78`: Calls a function at a specific offset from a global pointer using a library offset of negative seventy-eight.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_offset_318`: Calls a function at a specific offset from a global pointer using a library offset of negative three hundred eighteen.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_offset_354`: Calls a function at a specific offset from a global pointer using a library offset of negative three hundred fifty-four.
- `iterate_table_entries`: Iterates through a range of indices, retrieving a pointer from a global table for each and passing it to another routine along with a global variable.
- `schedule_table_entries`: Loops over a range of indices and calls another routine for each entry in a global table, passing the table entry address and a constant value to it.
- `process_preference_switch`: Reads configuration values from memory structures based on a global switch index.
- `configure_display_modes_2`: Iterates through a 16x16 grid of bits derived from a global configuration table.
- `calculate_string_length_2`: Returns the number of characters in a null-terminated string before its terminator.
- `configure_display_regions`: Acquires a semaphore, then calls a library routine multiple times with coordinates and parameters derived from the input arguments, and releases the semaphore upon completion.
- `draw_rectangle_sequence`: Acquires a semaphore, then calls a library routine repeatedly with coordinates derived from the input parameters to draw shapes, and releases the semaphore after the drawing operations.
- `select_and_pass_constant`: Selects one of two constant values based on an input parameter and passes it to another routine along with fixed coordinates.
- `initialize_library_instance`: Opens two libraries and, if either fails, closes one and returns zero; otherwise it initializes several fields of the first library structure with calculated addresses and flags.
- `wait_for_signal_completion`: Sets a flag and signals a task when a global variable is nonzero, then loops calling a delay routine until that variable becomes zero, and finally signals another task if given a nonzero argument.
- `format_string_via_rawdofmt`: Calls exec.RawDoFmt to format a message into a local buffer, the buffer being passed onward through the forwarded argument.
- `store_byte_via_pointer_3`: Stores a single byte from a data register into a memory location pointed to by an address register.
- `modular_inverse_32bit_2`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `call_modular_inverse_helper_2`: Calls a routine that performs a 32-bit modular reduction or 16-bit modulo operation and then returns.
- `fetch_modular_inverse`: Calls a helper routine and returns the value left in a data register.
- `negate_result_on_flag_5`: Calls a helper routine and negates the returned value if the helper leaves a non-zero result in an address register.
- `call_and_negate_on_flag_6`: Calls a helper routine and returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_via_pointer_4`: Stores a byte from a data register into the memory location pointed to by an address register.
- `handle_preference_input`: Sends a message to a library, stores several derived values into a structure, and then loops reading messages and dispatching on their status to update preference settings.
- `copy_structured_data_fields`: Copies a word from offset two and three bytes from offsets 183, 182, and 184 of a source address into the matching offsets of a destination address.
- `update_menu_selection_index`: Increments or decrements index counters within a global array based on a command ID read from a data structure.
- `return_constant_two_2`: Returns the constant value two.
- `update_library_vector_flags`: Calls another routine multiple times with a base pointer from a global variable and constants 1, 1, 0, and 430, then with a value from a table indexed by a global variable and constant 5, before updating a global variable by ORing it with its own masked value.
- `update_library_vector_flags_2`: The routine calls another routine multiple times using a base pointer from a global variable and various constants, including one value retrieved from a table.
- `handle_input_events_loop`: Initializes display configuration and preference data, then enters a loop to wait for and retrieve input messages.
- `copy_and_schedule_tasks`: Copies several fixed fields and a looped block of word-sized entries from one memory location to another.
- `initialize_display_configuration`: Frees previously allocated memory and cleans up resources, then copies configuration values into global variables and configures display colors and modes using Exec library functions.
- `process_input_event`: Reads an event code from a structure and branches on it to update global state, setting a selector for several codes and returning early for others.
- `decode_and_calculate_values`: When a global flag is nonzero, it decodes a 16-bit value into nibbles and passes them to another routine.
- `store_word_recursive`: Writes a word built from three parameters into a global table at an index derived from a second parameter, then calls itself recursively with an incremented index.
- `configure_display_colors`: Reads byte values from global memory and passes them, with small offsets, to a library routine four times, likely configuring display color registers.
- `schedule_preference_tasks`: Invokes another routine three times with a global pointer, a pointer to a global variable, and a constant argument, acting as a wrapper to schedule preference operations.
- `query_and_adjust_state`: Adjusts a parameter, then calls another routine repeatedly with coordinates derived from the input parameters and conditional flags based on a global variable.
- `configure_display_modes`: Calls a library routine to query a display state, then iterates through a 16x16 grid of bits derived from a global configuration table.
- `modify_word_bits`: Clears specific bits in two global word variables using a mask derived from a shift amount, then sets bits based on another parameter.
- `query_display_state`: Calls library routines to query display state, checks a return value, and conditionally calls another routine with calculated offsets before a final call.
- `apply_preference_configuration`: Iterates through a configuration table to update local settings by copying specific fields and dispatching commands.
- `copy_struct_fields_partial`: Copies specific word-sized and byte-sized fields from a source structure to a destination structure at matching offsets.
- `dispatch_node_command`: Executes a command based on a type identifier found within a data structure.
- `dispatch_configuration_updates`: Calls another routine multiple times with various constant parameters and values retrieved from a list and a data structure.
- `update_preference_bounds`: Updates two global variables based on comparison logic between a third global variable and a constant.
- `allocate_preference_entry`: Allocates a memory entry via the exec library and schedules a task to initialize it, then invokes a function pointer from a global table to process the new entry.
- `init_preference_list`: Sets a global initialization flag on first entry, then adjusts a stored value against a global variable before invoking another routine.
- `init_preference_entries`: Allocates and initializes a linked list of preference entries using exec library calls, scheduling the initialization task and managing memory for the entry nodes.
- `initialize_preference_entry`: Checks a global variable and, if it is less than one, sets a global flag and calls another routine.
- `init_preference_entry`: Allocates a new entry in the exec list and initializes its fields using values from a global variable and a table offset before linking it into the preference list.
- `process_configuration_table`: Iterates over a range of indices and calls another routine for each entry in a global table, then derives several bit fields from a configuration word and passes computed offsets to another routine.
- `process_preference_menu_events`: Iterates through a list of menu entries to retrieve and process messages.
- `dispatch_preference_menu_commands_2`: Processes menu selection commands by updating a global state bitmask and executing corresponding sub-routines.
- `pass_global_pointer`: Passes the address of a global variable as an argument to another routine and returns.
- `invoke_recursive_call`: This routine performs a direct call to itself before returning zero.
- `dispatch_preference_menu`: Retrieves a message from a list and processes it by iterating through a structure until a specific condition is met.
- `return_immediately_4`: The routine performs no operations and returns immediately to the caller.
- `copy_struct_fields`: It copies five specific fields, two words, two words and two bytes, from a source structure pointed to by the first argument to a destination structure pointed to by the second argument.
- `dispatch_preference_menu_commands`: Processes menu selection commands by updating a global state bitmask and executing corresponding sub-routines.
- `call_routine_with_pointer`: Passes the address of a global variable as an argument to another routine and returns one.
- `return_immediately_5`: This routine performs no operations and returns immediately to its caller.
- `update_interface_gadgets`: Evaluates various bitwise flags within a global variable to call another routine with different integer constants and parameters.
- `update_preference_gadgets_2`: Updates various interface elements by checking bitwise flags in a structure and calling a routine with specific integer constants.
- `update_preference_gadgets`: Evaluates various bitwise flags within a structure to call a routine with specific integer constants, effectively updating the state of multiple interface elements.
- `update_preference_display_3`: Checks specific bits in a structure and calls another routine with different constant values based on those bitwise results.
- `update_preference_display_2`: Calls a routine with specific constants and selects a value based on bitwise checks of a structure before calling another routine.
- `update_preference_display`: Evaluates a bitmask within a structure to select a specific constant value and passes it to a routine.
- `select_preference_index`: Evaluates bitmask flags within a structure to determine a specific constant value.
- `dispatch_interrupt_status`: Checks specific bits in a structure to select one of several constant values and passes them to another routine.
- `dispatch_preference_interrupt_2`: Checks a bit in a global structure and calls a function with different arguments based on the result, then calls another routine.
- `dispatch_preference_interrupt`: Calls a routine with specific arguments and then conditionally executes another routine based on a bitmask check of a value in a structure.
- `handle_preference_dispatch`: Calls two routines using a value from a register and a value from a stack location.
- `setup_intuition_gadgets`: It loads an object pointer from a global variable and calls multiple library functions on it, passing various constant parameters and values retrieved from a global array indexed by a mask.
- `parse_numeric_string_3`: Converts a string of characters into an integer, skipping leading spaces and handling a single decimal point.
- `skip_string_whitespace`: Advances a pointer past leading spaces and non-numeric characters in a string.
- `validate_numeric_string`: Checks if a string contains only numeric characters.
- `calculate_scaled_values`: It calculates two values based on an input, applying different multipliers depending on the state of other registers and flags.
- `parse_numeric_string_2`: Converts a string of numeric characters into a combined integer and fractional value.
- `format_decimal_string_4`: Either copies a block of memory from a global table or converts five numeric values into a decimal string representation in a destination buffer.
- `copy_memory_block_and_call_indirect`: Copies a block of bytes from a global variable to a destination buffer using a length specified in a structure.
- `format_decimal_string_2`: Converts a sequence of five values into a decimal string representation in a buffer.
- `format_number_string`: It iterates through a data structure to build a decimal string representation in a buffer, inserting a decimal point based on flags and converting each digit, then null-terminates the resulting string.
- `initialize_preferences_display`: Extracts a nibble from a global word and then issues a series of library calls with derived coordinates and constant values to configure text formatting and layout parameters.
- `check_flag_and_allocate`: Checks a specific bit in a global variable; if set, it performs several library calls including memory allocation and returns the result.
- `call_remote_int_server`: Invokes the remote interrupt server twice, once for each of two consecutive global variables, passing a fixed zero argument each time, then returns.
- `manage_library_semaphore_access`: Acquires a semaphore, then calls a library function several times using a parameter and values read from a global variable array, releasing the semaphore before returning.
- `store_calculated_short_value`: Computes a value from three short parameters and stores it into a global variable array at an index derived from the second parameter, then calls another routine with adjusted arguments.
- `process_three_buffers`: Extracts nibbles from a word in a global structure, then iterates over three buffer pointers, calling routines to process each buffer's data and state.
- `forward_argument_with_code`: Passes its one argument to another routine together with the constant 48.
- `forward_single_argument`: Passes its one argument on to another routine and returns nothing.
- `store_word_and_pointer_in_node_2`: Calls a library-backed allocation routine with a second argument, and if it returns a non-null block, stores a byte value, a word from the second argument, and the first argument pointer into fixed offsets of that block.
- `reset_node_and_release_it_2`: If the given pointer is non-null, it sets a byte field to 255 and clears two longword fields of the structure, then passes the pointer and a word field to another routine and returns its result.

### SetMap

- `initialize_system_environment_2`: Initializes global state structures and loads configuration data from the filesystem.
- `update_and_cleanup_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_dispatch_function`: Calls a function located at a specific offset within a global structure.
- `execute_indirect_function_12`: Fetches a function pointer from a location within a global table and executes it.
- `execute_indirect_function_13`: Fetches a function pointer from a global variable and executes it.
- `release_recursive_pointer_blocks`: Iterates through a counted table of pointers to recursively release each non-zero entry, then recursively releases additional blocks stored in global variables before finally releasing the block passed as a parameter.
- `save_arguments_and_run`: Stores its two arguments in global variables, then calls a routine with fixed size and count values and a final routine with a fixed code.
- `load_keymap_for_console`: Loads a keyboard keymap from the keymaps directory or a resource, converts the requested name to lowercase, and allocates a buffer to store the keymap data.
- `execute_indirect_function_14`: Fetches a function pointer from a location within a global table and executes it.
- `execute_indirect_function_15`: Fetches a function pointer from a table and executes it.
- `execute_indirect_function_16`: Fetches a function pointer from a location within a global data structure and executes it.
- `execute_indirect_function_17`: Fetches a function pointer from a location within a global data structure and executes it.
- `execute_indirect_function_18`: Fetches a function pointer from a location within a global structure and executes it.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `execute_indirect_function_19`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_indirect_handler_3`: Executes a subroutine through a pointer stored at a specific offset within a global variable.
- `call_indirect_handler_4`: Performs an indirect call to a function address located at a fixed offset from a global variable.
- `call_indirect_handler_5`: Calls a function by dereferencing a pointer found at a specific offset in a global variable.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_handler_6`: Loads a function pointer from a global variable offset and jumps to it.
- `call_indirect_handler_7`: Invokes a routine via a pointer retrieved from a global variable at a specific offset.
- `call_indirect_handler_8`: Executes a subroutine using a pointer located at an offset from a global variable.
- `call_indirect_handler_9`: Calls a function through a pointer stored at a specific offset within a global variable.
- `call_indirect_handler_10`: Performs an indirect jump to a routine address found at a fixed offset in a global variable.
- `forward_argument_to_library_stub_2`: Passes an argument, a library base pointer from a global variable, and a stack address to another routine.
- `call_indirect_handler_11`: Dereferences a pointer from a global variable offset to call another routine.
- `execute_global_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_indirect`: Stores a single byte into a memory location pointed to by a register.

### Format

- `initialize_system_environment`: Initializes the application state by querying the current directory, allocating memory, and parsing command-line arguments.
- `update_and_cleanup_state_3`: Retrieves a pointer from a global structure and stores a parameter into a specific slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_alloc_mem_2`: Calls a function pointer located at a fixed offset from a global variable.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `execute_indirect_function_20`: Fetches a function pointer from a global variable and executes it.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `execute_indirect_function_21`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_22`: Fetches a function pointer from a global table and executes it.
- `call_dispatch_function_2`: Calls a function pointer retrieved from a specific offset within a global variable.
- `execute_indirect_function_23`: Fetches a function pointer from a memory location using an offset and executes it.
- `execute_indirect_function_24`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_25`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_function_26`: Fetches a function pointer from a memory location and jumps to it.
- `execute_indirect_function_27`: Fetches a function pointer from a location within a global structure and executes it.
- `format_disk_from_arguments`: Parses command line arguments to identify the drive, volume name, and formatting options.
- `release_format_buffers_and_handles`: Releases the buffers and handles that the format routine allocated, calling a release routine for each field that is set.
- `handle_disk_io_errors`: Evaluates a status code returned from a disk operation to determine the specific error type.
- `report_disk_errors`: Checks a condition and either calls several routines with a global variable or calls a routine with a specific constant and a global variable.
- `store_matched_table_byte`: Searches an eight-entry global byte table for the given value and records the matching entry in a field of a structure obtained from another routine.
- `find_drive_list_entry`: Walks a linked list of drive entries until one matches the given number or pointer, marks that entry, and returns 1.
- `calculate_string_length_3`: Iterates through a null-terminated string to determine the number of characters preceding the terminator.
- `copy_cstring_to_buffer`: Library code that copies a null-terminated byte string from a source pointer to a destination pointer, advancing both pointers and writing each byte until a zero byte is encountered.
- `convert_char_to_uppercase`: Returns the character with lowercase ASCII letters converted to uppercase by subtracting 32.
- `compare_strings_ignoring_case`: Compares two null-terminated strings one character at a time after converting each to uppercase.
- `truncate_string_at_colon`: Replaces the first colon in a string with a null terminator, cutting the string short.
- `call_with_single_value_status`: Passes its first argument in a one-slot stack buffer to another routine with code 31 and length one.
- `create_block_from_source_words`: Allocates a sixty-eight-byte object and copies up to seven words from a source array into the block.
- `call_library_offset_function`: Calls a function pointer retrieved from a specific offset within a different global variable.
- `execute_indirect_function_pointer_3`: Fetches a function pointer from an offset within a global table and executes it.
- `call_indirect_function_20`: Fetches a function pointer from a table and executes it.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `execute_indirect_function_28`: Fetches a function pointer from a specific offset within a global table and executes it.
- `call_indirect_function_21`: Fetches a function pointer from a table and executes it.
- `call_indirect_function_22`: Executes a subroutine by dereferencing a pointer located at a fixed offset in a global table.
- `call_indirect_function_23`: Performs an indirect call to a routine retrieved from a global data structure.
- `call_indirect_function_24`: Branches to an address obtained from a pointer stored in a global variable.
- `call_indirect_function_25`: Invokes a routine via a pointer found at a specific offset in a global variable.
- `call_indirect_function_26`: Calls a subroutine using a pointer stored within a global data area.
- `call_indirect_function_27`: Redirects execution to a function pointer located at a fixed offset in a global variable.
- `call_indirect_function_28`: Uses a pointer from a global variable to call another routine.
- `call_indirect_function_29`: Executes a routine by dereferencing a pointer at a specific offset in a global variable.
- `call_indirect_function_30`: Calls a subroutine through a pointer stored at a constant offset in a global variable.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_indirect_pointer_3`: Calls a subroutine located at an offset from a global pointer.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_pointer_4`: Executes a function pointer retrieved from a specific offset within a global variable.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to the library call stub at offset minus 132.
- `call_indirect_pointer_10`: Calls a subroutine using an address stored within a global variable.
- `call_indirect_pointer_11`: Performs an indirect jump to a routine address held in a global table.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_tracks_and_verify`: Formats a disk track by track, building the track data in a buffer and writing it through the disk device, then verifies the tracks.
- `write_value_to_output_handle`: Calls two output routines with fixed arguments of 20, 30 and 1 on a handle taken from a structure, then passes a value from another routine through the same handle.
- `invoke_and_verify_stack_buffer`: Invokes a function pointer stored in a global structure and then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_via_a3`: Writes the low byte of D0 to the memory location pointed to by address register A3, then returns.
- `divide_64_bit_integer`: Performs a 64-bit division by a 32-bit divisor using a bit-shifting loop.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_and_rotate_32bit`: Calculates a 32-bit product of two 16-bit values and returns it combined with a rotated version of the second input.
- `call_global_pointer`: Calls a routine through a pointer stored at a specific offset within a global variable.
- `store_byte_to_pointer_4`: Stores a single byte from a register into a memory location pointed to by another register.
- `allocate_indexed_record_block_3`: Obtains a slot index from another routine and, if valid, allocates a 34-byte block.
- `release_record_chain_3`: Recursively traverses a linked list of records, processing the linked record first if a pointer is present.
- `pass_argument_with_constant`: Passes its single argument to another routine along with the constant forty-eight.
- `forward_argument_to_reset_routine`: Passes its single argument to another routine and returns.
- `initialize_system_structures`: Initializes several global pointers and variables by retrieving values from a memory block, potentially setting them to default values if the block is null, and performs a series of function calls to manage these structures.
- `execute_with_status_check`: Calls a routine via a global pointer and returns either a value from a stack slot or a negative error code.
- `call_indirect_pointer_5`: Performs an indirect call to a routine stored in a table.
- `call_indirect_pointer_6`: Jumps to a subroutine address calculated from a global variable.
- `call_indirect_pointer_7`: Calls a function by dereferencing a pointer stored in a global table.
- `initialize_pointer_structure`: Stores a pointer into a memory location, increments that pointer, and then stores the original pointer and a zero into subsequent memory slots.
- `allocate_and_initialize_node`: Attempts to allocate a memory block using a library routine with a specific size.
- `reset_node_and_release_it_3`: If the provided pointer is non-null, it sets a byte at an offset of eight to 255, clears two longword fields at offsets of twenty and twenty-four, and then calls another routine with the pointer and a word field from offset eighteen.
- `call_indirect_pointer_8`: Invokes a routine via a pointer found at a fixed offset from a global address.
- `call_indirect_pointer_9`: Executes a subroutine through a pointer located in a global data structure.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start`: Opens the DOS library and allocates a control block to parse the command line into individual argument strings.
- `call_supervisor_function`: Calls a function through a pointer located at a specific offset from a global base address.
- `execute_indirect_function_29`: Fetches a function pointer from a global variable and executes it.
- `call_switch_function`: Calls a function pointer stored at a specific offset within a global table.
- `call_dispatch_function_3`: Executes a routine located at a fixed offset from a global variable.
- `call_make_library_function`: Invokes a function pointer retrieved from a global data structure.
- `call_make_functions_function`: Performs a call to a function pointer found at a specific offset in a global table.
- `call_find_resident_function`: Calls a routine through a pointer stored in a global variable.
- `call_enable_function`: Executes a function pointer located at a specific offset relative to a global table.
- `call_alloc_entry_function`: Calls a function pointer retrieved from a global data structure.
- `open_cli_window_and_run_startup`: Creates a console window and shell control block based on a specification, then executes the system startup scripts.
- `build_linked_node_chain`: Walks a list of entries, allocating an 8-byte node for each and storing a value from the entry in it, then links the nodes together with shifted pointers.
- `traverse_and_free_linked_list`: Recursively traverses a linked list by calling another routine on the second element of each node before freeing the node itself.
- `parse_signed_decimal_2`: Converts a null-terminated string representing a signed decimal integer into a numeric value.
- `call_library_offset_function_2`: Calls a function pointer at a specific offset from a global variable.
- `call_library_offset_function_alt`: Executes a routine via a pointer stored in a global table at a specific offset.
- `call_library_offset_function_final`: Invokes a function pointer located at a fixed offset from a global variable.
- `call_indirect_function_31`: Fetches a function pointer from a specific offset within a global data structure and executes it.
- `call_indirect_pointer_12`: Calls a function pointer located at a different offset from a global variable.
- `execute_indirect_function_30`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function_pointer_4`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function_31`: Fetches a function pointer from a location within a global data structure and executes it.
- `perform_modular_reduction`: Calculates a 32-bit modulo operation using repeated doubling and subtraction if the divisor exceeds 16 bits, or performs 16-bit division and remainder operations if the divisor fits within 16 bits.
- `multiply_32_bit_integers_3`: Performs a signed 32-bit multiplication of two 32-bit values by combining the results of four 16-bit partial products.
- `call_and_negate_on_flag_7`: Calls a routine and returns its first result, or the negation of that result if the second result is nonzero.

### DiskCopy

- `initialize_application_context`: Allocates memory for a new data structure and populates it by copying values from a global variable and performing pointer arithmetic.
- `cleanup_system_resources`: This routine executes a callback function, frees memory, and performs several operations involving system structures and library calls to release resources.
- `parse_command_line_arguments_4`: Parses a string into individual arguments by handling whitespace and quoted substrings, storing the resulting pointers in a table.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies d0 to d5, then loads 0 into d0, then returns.
- `initialize_disk_copy_environment`: Allocates memory and initializes structures required for a disk copying operation.
- `dispatch_conditional_request`: Checks a value in a structure and either calls a routine with several parameters or calls another routine using a global variable and a pointer from that structure.
- `execute_callback_chain_3`: Checks several conditional pointers and executes them, while also iterating through a table to call functions and clear entries.
- `manage_disk_copy_operation`: Coordinates a disk copying process by iterating through a table of data blocks, performing I/O operations, and updating progress status.
- `dispatch_error_message`: Sets a pointer in a structure and calls a routine to handle a message based on a provided index.
- `check_signal_status`: Calls a routine to check a bitmask and returns a value based on the result and a character check.
- `handle_disk_operation`: Performs conditional logic based on a structure value, potentially executing a series of function calls via pointers and returning a boolean status.
- `verify_disk_contents_2`: Compares two values and executes a chain of callbacks and system calls to validate disk information.
- `execute_io_sequence_2`: Performs a series of I/O operations by calling function pointers from a table and updating a status field in a provided memory structure.
- `poll_status_and_allocate`: Repeatedly checks a character value and, if non-zero, performs memory allocation or error handling based on a global state.
- `manage_memory_pool`: Allocates memory, populates a list of blocks, and manages the deallocation of zeroed blocks within a loop.
- `format_progress_string`: Calculates a value from two structure members and calls a routine to format and display a string with remaining counts.
- `compare_memory_blocks`: Compares two blocks of data of a specified length and returns one if they match and zero otherwise.
- `verify_disk_contents`: Checks disk information and file existence using system calls, potentially triggering error messages or updating a global structure based on the results.
- `search_task_list`: Iterates through a list to find a matching byte value and stores a corresponding value into a memory location.
- `execute_dos_read`: Calls a function via a pointer and returns a status code or a value from the stack.
- `update_structure_fields`: This routine writes several values into specific offsets of a structure and then adds a calculated value to a field within that structure.
- `perform_io_operation`: Calls multiple function pointers from a global table and writes specific values to a provided memory structure.
- `execute_io_sequence`: Performs a series of I/O operations by calling function pointers from a table and updating a status field in a provided memory structure with specific values.
- `process_io_buffer`: Manages a memory buffer by performing I/O operations, copying memory blocks, and calculating string lengths based on structure data.
- `perform_io_operation_2`: Executes an I/O operation with specific parameters and returns a boolean indicating success.
- `execute_io_sequence_3`: Evaluates conditions involving a structure and a global variable to decide whether to call a routine with specific parameters or increment a byte in a structure.
- `search_linked_list`: Traverses a linked list to find an entry matching a specific string and value.
- `copy_memory_block_5`: Copies a specified number of bytes from a source memory location to a destination memory location using a loop.
- `render_text_with_bounds_check`: Calculates a string length and performs a loop to check character bounds before invoking several routines via pointers in a global table.
- `convert_char_to_lowercase`: Checks if a character is a lowercase letter and, if so, subtracts thirty-two from it; otherwise, it returns the original value as a signed integer.
- `load_byte_from_offset`: Loads a single byte from a memory location calculated by adding an offset to a value in a register.
- `strip_trailing_colon`: Iterates through a null-terminated string and replaces a colon with a null byte if it is the final character before the terminator.
- `ensure_colon_presence_2`: Searches a string for a colon; if the end of the string is reached without finding one, it inserts a colon, and then ensures a null byte follows the colon.
- `ensure_colon_presence`: Searches for a colon in a string, ensuring one exists, and then modifies the string by inserting a zero byte at a specific offset.
- `allocate_task_structure`: Allocates memory and initializes a task structure using various system library calls and global pointers.
- `nop_operation`: This routine is empty and performs no operations.
- `return_no_op_5`: The routine consists of a single return instruction and performs no work.
- `append_to_string_buffer`: Appends a byte or a string to a managed buffer while managing buffer capacity and pointer increments.
- `uppercase_string_in_place`: Walks a null-terminated byte string, and for each character consults a 256-entry character-class table (indexed relative to A4) and, when the class flag bit 1 is set, subtracts 32 from the character, which converts lowercase letters to uppercase; the result is written back into the same buffer and the original pointer is returned.
- `convert_string_case`: Iterates through a null-terminated string and modifies characters based on a bitmask check against a global table.
- `calculate_string_length_4`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `format_numeric_string`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a twelve-byte buffer in place before terminating it with a null byte.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_decimal_3`: Parses a signed decimal integer from a string, storing the result in an integer pointer, and returns the number of characters consumed.
- `push_byte_to_list`: Increments a counter in a global structure and stores a byte into a buffer pointed to by another global variable, then advances that pointer.
- `update_global_structure`: Stores a value into a global variable and then clears another global variable after calling a nested routine.
- `update_buffer_pointer_2`: Increments a value at a specific offset in a structure and either decrements a counter or advances a pointer and stores a byte.
- `append_offset_to_string_buffer`: Calculates a value by adding an offset to a local variable and passes the result to a routine that appends data to a managed buffer.
- `format_and_append_string`: Resets a value in a global structure, parses a format string, and returns a value from a global variable.
- `fetch_table_entry_pointer`: Retrieves a pointer from a table and uses it to call another routine, returning a failure code if a global variable is non-zero.
- `search_and_split_list`: Searches a linked list for an entry matching a specific size, potentially splitting an existing node to accommodate a new size, and allocates memory if no match is found.
- `traverse_pointer_table`: Iterates through a linked list of pointers, checking specific bit flags in a byte at an offset before recursively calling another routine with data from the current node.
- `execute_dos_write`: Calls a function via a pointer in a global structure and, if it returns a failure, retrieves an error value and stores it in a global variable.
- `perform_dos_seek`: Calls a function via a pointer in a global structure to perform a seek operation.
- `execute_dos_read_2`: Calls a function via a pointer in a global structure to perform a read operation.
- `cleanup_display_context`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `close_file_handle`: Calls a function via a pointer in a global structure to close a resource.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_fixed_point`: Performs a normalized fixed-point division of two 32-bit values, handling sign logic and bit-shifting to maintain precision.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `allocate_and_initialize_structure`: The routine allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `get_table_entry_address`: The routine checks if an index is within valid bounds and if the corresponding entry in a table is non-zero, then returns the address of that entry or a default value.
- `invoke_auto_request_callback`: The routine calls a function pointer located within a structure pointed to by an offset from a global variable.
- `free_linked_list_elements`: The routine traverses a linked list, calling a cleanup routine for each element, and then clears the head and tail pointers of a structure.
- `append_to_linked_list`: The routine allocates memory for a new element, initializes its fields, and inserts it into both a head pointer and a tail pointer within a structure.
- `clear_message_buffer`: The routine writes a byte and two words to a structure pointed to by its argument and then calls another routine via a global variable table.
- `set_message_type_and_target`: When a non-zero parameter is passed, the routine calls another routine via a function pointer stored in a global variable table and, if the call returns a non-zero pointer, writes a message type byte, a zero byte, the original parameter as an integer, and a 16-bit value into fields of the returned structure.
- `dispatch_table_call`: Retrieves a pointer from a table and calls another routine, returning a failure code if a global variable is non-zero or if a write operation fails.
- `parse_formatted_numeric_string`: Parses a string containing numeric values with various format specifiers including signs, decimal points, and base indicators.
- `process_format_string`: This routine iterates through a string and calls another routine to parse numeric values when a percent sign is encountered.
- `assign_list_node_pointer`: Checks if a structure is valid and not flagged, then attempts to assign a pointer from a global source to several offsets within the structure.
- `iterate_and_call_resources`: Iterates through a table of structures, checking specific bits in each entry to decide whether to call another routine recursively.
- `insert_into_sorted_list`: Inserts a memory block into a linked list of sorted nodes, updating the list structure and a global count.
- `fetch_and_validate_table_entry`: Retrieves an address from a table using an index and performs a secondary lookup through a pointer found at that address.
- `copy_memory_block_6`: Copies a specified number of bytes from a source address to a destination address.
- `check_and_clear_pointer`: Checks specific bits in a return value from a function call and, if a global pointer is non-null and a secondary routine returns a non-zero value, clears that pointer and calls another routine.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Locates a fast memory block and parses the operating system command line string into a linked list of null-terminated argument strings.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the or chip block.
- `call_indirect_function_32`: Fetches a function pointer from an offset within a global structure and executes it.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_function_via_pointer_2`: Fetches a function address from a memory location using an offset and executes it.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it into a buffer.

### Newcon-Handler

- `initialize_hunk_relocation_structure`: Initializes a data structure by zeroing out multiple fields and calculating word-aligned offsets.
- `execute_indirect_function_pointers`: Calls three functions via a pointer stored in a register followed by an indirect call through an unidentified jump table.

### Shell-Seg

- `initialize_data_structures`: Initializes several internal pointers and offsets within a data structure using word-aligned addresses.

### Ram-Handler

- `call_indirect_function_33`: Calls a function through a pointer retrieved from a global variable and an offset.

### serial.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_device_state`: Updates several fields in a structure, including incrementing and decrementing a counter, performing a library call, and storing a value into multiple offsets.
- `check_status_flag`: Checks a specific bitmask against a byte in a structure and calls another routine if the condition is met.
- `update_status_byte`: Stores a byte into a structure and calls another routine if a specific bitmask check passes.
- `toggle_interrupt_enable`: Sets and then clears a bit in a byte at an offset from a pointer while updating a global variable and modifying the interrupt enable bit in the interrupt enable register.
- `process_device_request_2`: Manipulates a linked list of structures by clearing flags, updating pointers, or decrementing counters based on specific type values and status bits.
- `process_linked_list_node_2`: Checks a flag in a structure to either clear a global value or move a node within a linked list and decrement a counter.
- `decrement_counter_and_clear_flag_2`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in a second word.
- `process_device_request`: Compares a pointer against a global value and either clears specific bits in a structure or performs a data copy and counter decrement.
- `decrement_counter_and_clear_flag`: The routine checks a flag byte in the structure pointed to by a1 and, if set, copies the structure’s first word through a pointer stored in the structure, updates a pointer stored at an offset from that word, decrements a short counter in a6, modifies two flag bytes in the structure and clears a byte.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `test_offset_word`: Clears a register and branches if a word at a specific offset from a pointer is zero.
- `execute_multiple_calls`: Performs a sequence of library calls and updates a pointer-based structure before returning a value.
- `register_device_structure`: Inserts a structure into a linked list and updates device counters while modifying hardware interrupt enable bits.
- `return_from_subroutine_3`: The routine is empty and performs no operations before returning.
- `register_serial_device`: The routine links a serial device structure into a global device list and updates a device counter.
- `process_serial_interrupt_data`: Processes incoming serial data by checking status flags, managing a linked list of data buffers, and updating hardware-related bitmasks.
- `decrement_and_check_counter_3`: Decrements a short value at an offset from a pointer and, if the result is zero, updates several fields in a structure and calls another routine.
- `execute_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `update_serial_status`: Updates a bitmask in a global variable based on a comparison, increments two counters, and conditionally executes another routine via a pointer.
- `update_serial_interrupt_status`: Updates a bitmask in a global variable based on a comparison, increments two counters, and conditionally executes a routine via a pointer.
- `update_device_status_2`: Updates a global variable using a bitwise operation, increments two counters, and conditionally executes a routine via a pointer after checking a buffer.
- `remove_linked_list_node`: Removes a node from a linked list, updates structure members, and potentially calls another routine.
- `set_global_and_interrupt`: Sets a global variable to one and modifies the interrupt enable bit in the hardware interrupt mask register.
- `check_serial_interrupt_status`: Reads a byte from a global variable and combines it with a value from a hardware register to update a global status flag.
- `process_serial_request`: Sets a global variable, calls several routines, populates a structure with multiple 32-bit values, and conditionally executes a routine via a pointer.
- `check_and_call_handler`: Calls another routine and then executes a callback if a bitmask check on a structure returns zero.
- `copy_serial_state`: The routine copies a 32-bit value from an offset within a structure pointed to by a register to two other offsets in the same structure, then clears a 32-bit field at a third offset.
- `update_status_flags_2`: Sets a bit in a structure and conditionally calls another routine or a function pointer based on the values of structure members.
- `handle_hardware_interrupt`: Clears a bit in a byte within a structure and conditionally executes a routine via a pointer.
- `toggle_interrupt_state`: Modifies global variables and calls another routine before checking a bitmask to trigger a callback.
- `clear_serial_flag`: The routine calls another routine twice and then clears a 32-bit field in a structure pointed to by an address register.
- `traverse_and_unlink_list`: Iterates through a linked list, unlinking nodes and calling another routine for each element.
- `return_no_op_9`: The routine consists of a single return instruction and performs no work.
- `set_serial_flags_and_maybe_call`: The routine sets a flag byte in a structure pointed to by a1 and writes a constant to the next byte.
- `sync_device_parameters`: Copies data between structures and updates status bytes based on specific bitwise conditions.
- `link_device_structure`: Clears a bit in a structure and, if certain conditions are met, inserts the structure into a linked list and increments a counter.
- `set_flag_and_call_routine`: The routine sets a bit in a byte located via a pointer, calls another routine, stores a value returned in A1 into a word at an offset from the same pointer, and writes a constant to a global variable.
- `clear_serial_flag_and_disable_interrupts`: The routine clears a flag in a byte field of a structure pointed to by a1, and if a condition on that byte and a short field is met it sets another flag in the same byte, clears a pointer field, writes a value to a global variable, and calls another routine via a pointer stored in the structure.
- `reset_device_parameters`: Sets a global variable and resets several fields in a structure if a specific byte check fails.
- `return_immediately_7`: This routine performs no operations and returns immediately to the caller.
- `update_device_status_flags`: Modifies bitmasks within a device structure and updates a global variable based on specific status conditions.
- `check_device_status`: Clears a bit in a global variable and conditionally executes another routine or a function pointer based on several status checks.
- `execute_status_check`: Sets a global variable, calls another routine, and conditionally executes a routine via a pointer.
- `process_device_parameters`: Validates a range of values within a structure and, if successful, copies several fields to a different structure and executes multiple subroutines.
- `compute_serial_status_word`: The routine builds a 16-bit status word from a byte in a structure and a flag in another structure, then stores the word into a global variable.
- `decrement_indexed_table_lookup`: The routine checks a flag byte in a structure pointed to by a register and conditionally decrements a short index.
- `write_constant_byte`: Stores the value five into a byte at a specific offset from a pointer held in a register.
- `calculate_serial_divisor`: calculates a divisor based on a value in a structure and a constant, then stores the result in a structure field and a global variable while modifying specific bits in a byte.
- `compute_serial_divisor_and_flags`: The routine divides an input value by the low word of another input and stores the result in a structure field, copies that field to a global variable, clears a flag bit in a byte field, and conditionally sets a high bit in the global variable and a flag bit in the byte field based on two structure bytes.
- `configure_serial_device`: Initializes a structure with specific values and function pointers based on the result of a device call.
- `configure_serial_structure`: Sets specific byte and longword values within a structure, assigns a global variable, and updates function pointers.
- `initialize_structure_fields`: stores several values into a structure, including a pointer to a global variable and a specific byte value.
- `set_serial_device_io_request`: The routine writes a constant byte to an offset in a structure pointed to by a0, stores a pointer to a global variable at another offset, and copies two caller-provided values into two further offsets.
- `set_serial_device_request`: The routine fills a request block in memory with a pointer to a global variable and several byte fields, then calls a function through a pointer obtained from a table.
- `configure_serial_device_structure`: Initializes a device structure by setting specific byte and longword fields, updates a function pointer within the structure, and assigns values to two separate offsets.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `return_zero_3`: This routine is empty and returns zero.
- `update_linked_list`: Manipulates a linked list structure by inserting a new node and updating status flags based on conditional checks.
- `process_serial_buffer_queue_3`: Iterates through a linked list of data buffers to extract bytes and update buffer status flags.
- `consume_serial_buffer_queue`: Iterates through a linked list of data buffers to extract bytes and update buffer status flags.
- `process_serial_data_queue`: Iterates through a linked list of data buffers to extract bytes and update buffer status flags.
- `process_serial_buffer_queue`: Iterates through a linked list of data buffers to extract bytes and update buffer status flags.
- `process_serial_buffer`: Iterates through a linked list of data buffers to process incoming bytes.
- `return_no_op_stub_2`: The routine consists of a single return instruction and performs no work, so it acts as an empty placeholder that callers can invoke without effect.
- `identify_routine`: This routine is not empty; it contains ten instructions but the decompiler has not produced meaningful pseudo-code.
- `update_serial_status_and_process_buffers`: Updates status flags based on bitwise operations and iterates through a linked list of data buffers to extract bytes.
- `process_serial_data_buffer`: Iterates through a linked list of data buffers to extract bytes and update status flags.
- `process_serial_buffer_queue_2`: Iterates through a linked list of data buffers to extract bytes and update status flags.
- `sync_buffer_and_call_3`: Sets a bit in a byte, copies a long word if a specific offset is zero, and calls a function through a pointer.
- `manage_serial_input_buffer`: Checks serial device status and manages a circular buffer for incoming data.
- `update_serial_buffer_2`: Manages a circular data buffer by storing a byte from a register and incrementing a counter.
- `process_serial_status_and_buffer`: Checks status flags to conditionally call another routine, then updates a circular buffer by storing a byte and incrementing a counter if a limit has not been reached.
- `update_buffer_and_call`: Manages a buffer by inserting a byte, incrementing a pointer and counter, and conditionally copying a long word or calling a function through a pointer.
- `update_serial_buffer`: Manages a circular buffer by storing a byte from a register into a memory location and incrementing a counter.
- `sync_buffer_and_call_2`: Sets a bit in a byte, copies a long word from one location to another if a specific offset is zero, and calls a function through a pointer.
- `update_buffer_pointer`: Increments a pointer and a counter, updates a buffer with a byte, and conditionally copies a long word or calls a function through a pointer.
- `sync_buffer_and_call`: Copies a long word from one location in a structure to another if a specific offset is zero, then calls a function through a pointer if certain conditions are met.
- `check_buffer_status`: Checks if a value in a structure is greater than zero before calling another routine via a pointer.
- `update_hardware_state_2`: Copies a byte within a structure, modifies a bitmask in a byte, and updates a global variable based on a bitmask or a specific flag.
- `modify_hardware_state`: Updates specific bytes in a structure, modifies a global variable based on a bitmask, and interacts with hardware registers to control interrupt enabling.
- `clear_byte_at_offset`: sets a byte at a specific offset within a structure to zero.
- `set_buffer_parameters`: Sets a bit in a byte, clears a long word, calculates a value based on a shifted word, and calls a function through a pointer.
- `clear_bit_and_call`: Clears a bit in a byte and, if a specific bit is set and a global value is zero, copies a long word and calls a function through a pointer.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_serial_input_buffer`: Reads a byte from a device and manages a circular buffer for incoming data.

### usa1

- `perform_xor_operations`: performs bitwise XOR operations on a word and a byte at specific offsets from a pointer.

### parallel.device

- `return_error_code`: returns a value of all bits set.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `set_byte_at_offset`: stores a specific byte value into a structure at a fixed offset.
- `update_linked_list_2`: Removes a node from a linked list and updates status flags and a counter within a structure.
- `process_linked_list_node`: Manipulates a linked list by updating pointers, decrements a counter in a structure, and conditionally calls another routine.
- `check_pointer_and_call`: Checks if a value in a structure is non-zero and returns the result of another routine.
- `return_immediately_stub_2`: A single-instruction routine whose body is only a return, so it performs no work and returns control to its caller.
- `clear_status_bit`: Clears a bit in a byte through a pointer and calls a function through a pointer retrieved from a table.
- `return_immediately_stub_6`: The routine is a single return instruction with no body, so it performs no work and returns immediately to its caller.
- `traverse_linked_list_4`: Iterates through a linked list to re-link nodes and clears a word in a structure upon completion.
- `zero_structure_word`: Stores a zero word into a location within a structure.
- `set_status_bits`: Sets a bit in a byte, stores a byte value, and calls a function through a pointer retrieved from a table.
- `update_hardware_state`: Manipulates bits in a global variable and a structure, then stores a byte into a structure and updates a CIA register.
- `copy_structure_data`: Copies a byte and potentially two long words from one structure to another based on specific bit flags.
- `copy_structure_fields`: Copies a byte from one structure to another and, if a specific bit is set, copies two long words as well.
- `initialize_device_handler`: Sets up a device structure by assigning pointers, updating status flags, and calling a function through a pointer.
- `return_zero_22`: The routine is empty and returns zero to the caller.
- `set_flags_and_zero`: Sets two bits in a byte and clears a long word in a structure.
- `insert_linked_node`: Clears a bit in a byte and, if a flag is set, inserts a new node into a linked list and increments a counter in a structure.
- `update_device_status_wrapper`: This routine acts as a wrapper that calls a function to update device status by modifying a byte, copying a long word and a pointer to a structure, and updating flags based on a CIA register comparison.
- `update_device_status`: Modifies bits in a byte, copies a long word and a pointer to a structure, and updates various flags based on comparisons and a CIA register.
- `process_device_data`: Manages a linked list, decrements a counter, and performs conditional byte transfers and comparisons based on device status flags.
- `process_device_data_2`: Manages a linked list and performs conditional byte transfers and counter increments based on device status and buffer contents.
- `buffer_data_and_call`: Stores a byte into a buffer, increments a counter, and conditionally calls a function through a pointer.
- `process_buffer_data`: Copies a long word, modifies bitmasks in two different bytes, calls a function through a pointer, and conditionally calls another function through a pointer.

### EpsonQ

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_library_calls_4`: Calls a function from a library multiple times and, if those calls return non-zero, calls another function through a pointer.
- `execute_library_calls`: Calls a library function twice and returns a value of negative one.
- `execute_indirect_calls`: Calls a routine through a pointer found at an offset from a global variable three times.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `return_zero_4`: This routine is empty and simply returns zero to the caller.
- `calculate_32bit_modulo`: Performs a 32-bit division and modulo operation using a loop for large divisors or standard arithmetic for smaller ones.
- `rotate_32bit_value_2`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32bit_values`: Calculates the product of two 32-bit integers and performs a bitwise rotation on one of the inputs.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `update_bitmask_buffers`: Iterates through a table of values and compares them against a source array to set specific bits in multiple destination buffers.
- `set_bits_from_table`: Iterates through a table to compare values and sets specific bits in multiple destination buffers based on those comparisons.
- `apply_bitfield_updates_2`: Uses nested loops to compare values from a table against a source and sets bits in a destination buffer based on the comparison results.
- `apply_bitfield_updates`: Uses a table of values to determine how many bits to set in a destination buffer through nested loops.
- `update_bitfield_from_table`: Iterates through two nested loops to compare values from a table and sets bits in a destination buffer based on the comparison results.
- `divide_32_bit_integer`: Performs a long division of a 32-bit dividend by a 32-bit divisor.
- `format_printer_command_buffer`: Constructs a sequence of escape codes and data strings into a buffer to be sent to a printer.
- `return_from_subroutine_4`: The routine executes a single return instruction to exit the current function.
- `update_epson_printer_state`: Updates a printer configuration structure by copying values from a global table and setting specific control flags based on hardware status.
- `update_printer_state_buffer`: Copies a sequence of bytes from a global table into a destination buffer and updates internal state flags based on hardware status values.
- `parse_epson_escape_sequences`: Processes input characters to identify and translate Epson printer control sequences.
- `process_epson_escape_sequences`: Evaluates input characters to determine corresponding Epson printer control sequences.
- `map_epson_escape_sequences`: Translates specific input characters into corresponding Epson printer control sequences by writing escape codes and parameters into a destination buffer.
- `translate_epson_escape_sequences`: Converts input characters into specific Epson printer escape sequences by writing control bytes to a destination buffer.
- `copy_byte_sequence_3`: Copies a sequence of eight bytes from one memory location to another.
- `copy_byte_sequence`: Copies a sequence of eight bytes from one memory location to another using a loop.
- `map_epson_escape_sequence_2`: Translates specific input bytes into escape sequences by writing values to a destination buffer.
- `write_byte_on_condition_3`: If a byte at a pointer is negative, it clears that byte and stores a specific value in a word; otherwise, it returns a value with all bits set.
- `return_maximum_value_5`: The routine returns a 32-bit value consisting of all bits set to one.
- `map_epson_escape_sequence`: Maps specific input bytes to escape sequences by writing values to a destination buffer.
- `initialize_printer_state`: Sets a byte to zero and a word to a specific value if a condition is met, otherwise returns a value with all bits set.
- `return_all_ones_6`: Returns a 32-bit value where every bit is set to one.
- `map_escape_sequence`: Maps a byte value to a specific sequence of bytes or values by writing to a destination buffer based on multiple conditional checks.
- `write_fixed_bytes`: Writes two specific bytes and one byte from a memory location into a destination buffer.
- `write_printer_escape_sequence_2`: Checks a byte value and writes either a two-byte or three-byte sequence to a memory location.
- `write_printer_escape_sequence`: Checks a status byte and either writes a multi-byte escape sequence to a memory location or writes a single byte to an offset of a pointer if the status is zero.
- `write_byte_on_condition`: If a value pointed to by a register is zero, it stores a byte into a memory location offset from another pointer.
- `update_printer_status_2`: Sets a byte in a structure based on a value in another structure and then performs recursive calls to update further structure members.
- `update_structure_member`: Stores a value into a structure offset, performs a recursive call, and then writes a result into another structure offset.
- `execute_indirect_function_32`: Fetches a function pointer from a location offset from a global variable and executes it.
- `call_indirect_function_34`: Calls a function through a pointer stored at a different offset from a global variable.

### CBM_MPS1000

- `return_zero_5`: This routine is empty and returns zero to the caller.
- `execute_library_calls_5`: Calls a function through a pointer stored in a global variable multiple times and returns a status code.
- `execute_library_calls_2`: Calls a function through a pointer twice and returns a constant value.
- `execute_triple_call`: Calls the same function three times using a pointer from a global variable.
- `return_zero_6`: This routine performs no operations and returns a value of zero to the caller.
- `return_zero_value`: This routine is empty and simply returns a zero value to the caller.
- `manage_printer_buffer_and_commands`: This recursive routine manages a data buffer by calculating offsets and lengths based on specific hardware identification codes.
- `return_from_subroutine_5`: performs a return from a subroutine.
- `prepend_data_to_buffer_2`: Finds the length of a null-terminated string and copies a four-byte header containing the length and a constant value to a buffer.
- `prepend_data_to_buffer`: Searches for a null terminator in a string and then copies a four-byte sequence to the beginning of a buffer.
- `write_four_bytes`: Writes four bytes to a memory location calculated from two registers and updates a global variable if a condition is met.
- `clear_memory_block_3`: Fills a block of memory with zeros using either word-sized or byte-sized increments.
- `clear_memory_block`: Fills a block of memory with zeros using a counter and an incrementing pointer.
- `divide_32bit_value`: Performs 32-bit division by a value, using a bit-rotation loop for large divisors and standard division for smaller ones.
- `rotate_32bit_value_2_2`: Rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low words.
- `multiply_32bit_values_2`: Calculates the product of two 32-bit values and returns the result as a 64-bit value.
- `call_helper_routine`: Calls a single helper routine and returns.
- `call_and_negate_on_flag_2`: Calls another routine and returns its result, or the negation of that result if the second return value in an address register is nonzero.
- `format_printer_command_buffer_4`: Constructs a command buffer by copying a sequence of bytes from a table and appending specific escape sequences based on printer status flags.
- `build_printer_escape_sequence`: Constructs a command buffer by copying a null-terminated string and prepending specific escape sequences based on printer status flags.
- `format_printer_escape_sequence`: Copies a null-terminated string into a destination buffer and appends a multi-byte escape sequence determined by the printer's status and specific control characters.
- `process_printer_control_sequences`: Copies a string into a destination buffer and appends specific multi-byte escape sequences based on printer status and control characters.
- `format_printer_escape_sequences`: Checks printer status to select a specific control byte and writes a multi-byte escape sequence to a destination buffer.
- `map_escape_sequences`: Maps input characters to specific escape sequences or control codes and writes them to a destination buffer.
- `copy_four_bytes`: Copies four bytes from one memory location to another using a loop.
- `map_cbm_escape_sequence_2`: Converts input values into specific escape sequences and writes them to a destination buffer.
- `write_byte_conditionally_3`: If a value pointed to by a register is negative, it sets that value to zero and writes a specific word to a pointer.
- `return_constant_value_2`: This routine returns a constant value of all bits set to one.
- `map_cbm_escape_sequence`: Translates input values into specific escape sequences and writes them to a destination buffer.
- `reset_printer_buffer`: Clears a byte and sets a word to a specific value if a condition is met, otherwise returns a value with all bits set.
- `return_constant_value_10`: Returns a constant value where all bits are set to one.
- `map_input_values`: Compares a byte against several constants to assign specific values to a destination or a stack variable.
- `write_byte_conditionally`: Writes a specific byte to an offset of a pointer if a value pointed to by a register is zero.
- `update_printer_model_parameters`: Sets a status byte based on a comparison of a value at a specific offset in a global structure.
- `update_printer_status_3`: Updates a structure with values from a table based on a calculated index and sets a status field based on specific comparison results.
- `update_printer_status`: Checks a value at an offset from a pointer and writes a specific word to another offset within the same structure based on that value.
- `set_constant_value`: Stores a constant value into a memory location offset from a pointer and returns a byte from a table.
- `render_printer_bitmask`: Calculates bit offsets and iterates through a data table to set specific bits in a destination buffer.
- `set_bits_incrementally_2`: Iterates through a table to set bits in a destination buffer using a bitmask derived from a register value.
- `set_bits_incrementally`: Iterates through a table and sets bits in a destination buffer at specific increments based on table values.
- `set_bits_from_comparison`: Compares values from a table against a source and sets specific bits in a destination buffer based on the comparison result.
- `call_indirect_via_offset`: Calls a routine through a pointer located at a specific offset from a global variable.
- `call_indirect_via_offset_alt`: Calls a routine through a different pointer located at another offset from a global variable.

### HP_LaserJet

- `return_zero_7`: This routine is an empty function that returns zero to the caller.
- `execute_library_calls_alt`: Repeatedly calls a function via a pointer from a global variable and returns zero or a maximum value depending on the success of the calls.
- `execute_library_calls_8`: Calls a function through a pointer stored in a global variable twice and returns a constant value.
- `call_indirect_function_35`: calls a function pointer stored in a table three times.
- `return_zero_value_2`: This routine performs no operations and returns zero to the caller.
- `build_hp_laserjet_command_stream`: Constructs a sequence of printer commands by copying data from a global table into a destination buffer and injecting ASCII characters based on hardware status flags.
- `format_hp_laserjet_command_buffer`: Copies data from a global table into a destination buffer and modifies specific bytes based on hardware status flags.
- `process_hp_laserjet_escape_sequences`: Parses and modifies a command buffer by interpreting control characters and escape sequences.
- `format_printer_command_buffer_3`: Parses and updates a command buffer by converting numeric values into ASCII decimal strings and managing control characters.
- `copy_and_format_twelve_bytes`: Copies twelve bytes from a global variable to a destination and then calls another routine twice to format decimal values at specific offsets.
- `parse_hp_laserjet_escape_sequences`: Processes escape codes by updating a state byte and a counter based on specific control characters.
- `process_hp_laserjet_command_state`: Updates a state byte and a counter based on specific command codes.
- `update_hp_command_state`: Updates a status byte and a counter based on specific command codes.
- `update_printer_state_and_buffer`: Modifies a status byte and a character value based on specific command codes, and manages a data buffer by copying elements from a global table and calling other routines to process the updated data.
- `process_printer_commands`: Modifies a status byte and a character value based on a command code and updates a table from a global variable.
- `update_printer_status_and_counters`: Updates a status byte based on a command and modifies a counter value by incrementing, decrementing, or resetting it.
- `process_hp_command`: Interprets a command byte to modify a status value, increment or decrement a counter, or copy data from a global table to a destination.
- `process_printer_command_2`: Checks a status byte and either increments a counter, decrements a counter, or copies data from a global table to a destination.
- `process_printer_command`: Checks a status value and either decrements a pointer-based counter, sets a byte to a specific value, or copies eleven bytes from a global table to a destination.
- `update_and_copy_printer_buffer`: Checks a status value and either decrements a pointer-based counter or sets a byte to a specific value, then copies eleven bytes from a global variable to a destination buffer and calls another routine twice.
- `copy_eleven_bytes_2`: Decrements a value through a pointer and copies eleven bytes from a global variable to a destination buffer.
- `copy_and_format_eleven_bytes_2`: Copies eleven bytes from a global variable to a destination buffer.
- `copy_and_format_eleven_bytes`: Copies eleven bytes from a global variable to a destination and then calls another routine twice to format decimal values using values derived from a pointer.
- `conditionally_copy_and_format`: If a specific value is met, it copies eleven bytes from a global variable to a destination and calls another routine twice; otherwise, it may write a byte to a memory offset.
- `format_decimal_string_6`: Copies eleven bytes from a global variable to a destination and calls another routine to format decimal values.
- `write_byte_on_condition_2`: stores a byte into a memory location offset from a pointer if the value at another pointer is zero.
- `format_three_digit_decimal`: Converts a byte value into a three-digit ASCII decimal string and stores it in a destination buffer.
- `format_decimal_string_5`: Converts a byte value into a three-digit decimal string and stores it in a buffer.
- `format_decimal_string_3`: Converts a byte value into a two-digit decimal string and stores it in a buffer.
- `format_decimal_string`: Stores a character representing a digit into a buffer at a calculated offset.
- `clear_global_on_char`: sets a value in a global table to zero if a passed character matches a specific value.
- `execute_and_reset_flag`: calls two functions via pointers in a table and clears a global value if a specific global variable is non-zero.
- `perform_64bit_division`: performs division on a 64-bit value by a 16-bit or 32-bit divisor using a bit-shifting loop.
- `perform_bit_rotation`: Performs a series of bitwise shifts and subtractions on two 32-bit values in a loop.
- `calculate_complex_product`: Calculates a value based on the products of the high and low 16-bit halves of two 32-bit registers.
- `forward_call_to_helper_2`: Calls a single helper routine and returns without performing any additional operations.
- `call_helper_return_register_value`: Calls a helper and returns the value the helper leaves in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_3`: Calls a helper routine and returns its first result, or the negation of that result if the second result left in an address register is nonzero.
- `format_hp_laserjet_print_job`: Constructs and sends printer control sequences by converting numeric values into ASCII characters and prepending escape codes.
- `return_d5_value`: Moves the contents of register d5 into d0 and returns.
- `set_bits_in_buffer_3`: Performs bitwise OR operations on a destination buffer by comparing table values against a source through nested loops.
- `set_bits_in_buffer_2`: Iterates through a list of word-sized elements to perform bitwise OR operations on a destination buffer.
- `set_bits_in_buffer`: iterates through nested loops to perform bitwise operations on a buffer based on values from a table and input registers.
- `call_indirect_via_offset_2`: Calls a routine at an address determined by adding a negative offset to a global variable.
- `call_indirect_via_offset_3`: Calls a routine at an address determined by adding a different negative offset to a global variable.
- `update_table_from_data`: calculates values based on an input, then stores several derived values into a global table and several global variables.

### EpsonX[CBM_MPS-1250]

- `return_zero_8`: This routine is empty and simply returns a value of zero to the caller.
- `execute_library_calls_6`: Calls a function through a pointer stored in a global variable three times, returning zero if all calls succeed or a negative value if any fail.
- `execute_indirect_function_calls_2`: Retrieves a function pointer from a global table and executes it twice, passing a register value as an argument on the second call.
- `call_indirect_triple`: Calls a routine at an address determined by a negative offset from a global variable three times consecutively.
- `return_zero_9`: The routine performs no operations and returns a zero value to the caller.
- `return_zero_value_3`: This routine performs no operations and returns zero.
- `divide_32bit_value_2`: Performs division on a 32-bit value using a loop that rotates bits and handles carries.
- `perform_bit_rotation_2`: Rotates the bits of a 32-bit value by a fixed number of positions.
- `multiply_and_rotate_words`: Performs a complex multiplication of word-sized components from two registers and returns a combined result involving a bitwise rotation.
- `call_helper_and_return`: Calls a single helper routine and immediately returns.
- `negate_result_on_flag_2`: Calls another routine and returns its result, or the negation of that result if a second value returned in an address register is nonzero.
- `apply_bitmask_to_buffer_layers`: Iterates through a table of control values to conditionally set specific bits in multiple destination buffers.
- `apply_conditional_bitmask_to_buffers`: Iterates through a source table and compares its values against a control structure to set specific bits in multiple destination buffers.
- `set_bits_from_comparison_2`: Sets specific bits in multiple destination buffers based on comparisons between values in a source table and a control structure.
- `apply_bitmask_from_table_2`: Sets bits in a destination buffer by comparing values from a source table against a secondary table using two different logic paths.
- `apply_bitmask_from_table`: Iterates through a table to set bits in a destination buffer based on values stored in the table entries.
- `set_bits_from_table_comparison`: Compares values from a source table against a secondary table and sets specific bits in a destination buffer based on the comparison result.
- `update_global_state`: Updates several fields in a global structure based on values retrieved from a table using an index.
- `format_printer_output_buffer`: Calculates buffer sizes and memory offsets based on printer model identifiers and manages a recursive state machine for data transmission.
- `return_from_subroutine_7`: performs a standard return from a subroutine.
- `append_printer_escape_sequence_2`: Iterates through a data source to append carriage returns and specific escape sequences to a destination buffer.
- `format_printer_commands_2`: Iterates through a list of values to construct a buffer containing carriage returns and escape sequences for a printer.
- `format_printer_commands`: Constructs a sequence of escape codes and data bytes into a buffer to be sent to a printer.
- `append_printer_escape_sequence`: Appends carriage return characters and specific escape sequences to a destination buffer.
- `clear_memory_block_4`: Fills a specified block of memory with zeros using either word-sized or byte-sized increments.
- `zero_memory_block_by_count`: Fills a block of memory with zeros by decrementing a counter and incrementing a pointer.
- `process_printer_command_state`: Updates an internal state machine and command buffer based on a command identifier.
- `parse_printer_command_sequence_2`: Parses a stream of printer control commands by updating an internal state machine and populating a command buffer.
- `translate_printer_command_state`: Updates an internal state machine and command buffer based on a command identifier.
- `map_printer_command_codes`: Translates internal command identifiers into sequences of ESC/P escape codes and data bytes.
- `translate_printer_command_codes`: Converts internal command identifiers into sequences of ESC/P escape codes and data bytes.
- `process_printer_command_sequence`: Translates specific command codes into sequences of escape codes and data bytes for an Epson printer.
- `modify_and_copy_byte_sequence`: Decrements a byte from a source location and stores it at a destination offset, then copies a sequence of eight bytes from a source to a destination.
- `copy_eight_bytes`: Copies eight bytes from a source memory location to a destination memory location using a loop.
- `update_printer_state_and_commands`: Updates a printer status byte and a control flag based on a series of conditional checks.
- `reset_byte_if_negative`: Checks if a byte at a pointer is negative; if so, it sets that byte to zero and updates a word in a table, otherwise returns a value with all bits set.
- `return_negative_one_3`: Returns a value of all bits set to one.
- `process_epson_command`: Updates a status byte and a secondary value based on a command byte, occasionally writing a multi-byte sequence to a destination buffer.
- `reset_status_structure`: Checks if a byte is less than one, then either returns a value with all bits set or clears a byte and sets a word in a structure.
- `return_negative_one_6`: Returns a value with all bits set to one.
- `write_epson_sequence_2`: Checks a command byte and writes specific multi-byte sequences or control characters to a destination buffer.
- `write_epson_sequence`: Writes a specific multi-byte sequence to a destination buffer if a command byte matches a certain value, or updates a global variable if it matches another.
- `write_conditional_sequence`: Checks a short value and a byte; if conditions are met, it writes a six-byte sequence to a destination buffer, otherwise it writes a byte to an offset in a table.
- `write_byte_sequence`: Writes a sequence of six bytes to a destination buffer, including three constant values and one byte loaded from a memory location via a pointer.
- `write_byte_if_zero`: Checks if the value pointed to by a register is zero, and if so, stores a byte into a memory location at a fixed offset from another pointer.
- `update_printer_structure`: Sets a byte in a structure based on a comparison, copies two words from a table into another structure, and assigns a value to a third field based on the contents of the copied data.
- `update_printer_status_4`: Updates a structure with values from a table based on a calculated index and sets a status word based on the resulting value.
- `update_status_word`: Sets a two-byte value at an offset from a pointer based on a comparison of a value at another offset.
- `store_constant_and_fetch_table_byte`: Stores a constant byte into a memory location at a fixed offset from a pointer and returns a byte from a table using an index in a register.
- `execute_indirect_function_33`: Fetches a function pointer from a location offset from a global variable and calls it.
- `call_indirect_via_offset_4`: Calls a routine through a pointer located at a different offset from a global variable.

### Okimate_20

- `return_zero_stub_12`: The routine takes no arguments and simply returns the value 0 with no other work, so it acts as a placeholder or no-op stub.
- `validate_library_function_pointers`: Attempts to call a specific function offset from two different library base addresses stored in global variables.
- `execute_indirect_calls_4`: Calls a function through a pointer stored in a global structure twice, passing an unspecified value to the second call.
- `call_indirect_triple_2`: Calls a routine through a pointer at a specific offset from a global variable three times in succession.
- `return_zero_10`: The routine performs no operations and returns zero to the caller.
- `return_zero_value_4`: The routine returns a constant zero value to the caller.
- `divide_32bit_value_3`: Performs 32-bit division using a series of shifts and subtractions or modulo operations depending on the size of the divisor.
- `perform_bit_shifts`: executes a series of left shifts and subtractions on a 32-bit value using a loop.
- `calculate_complex_product_2`: performs a series of multiplications and bitwise shifts to combine two 32-bit values into a single result.
- `forward_call_to_helper_4`: Calls a single subroutine via a pointer and returns.
- `call_helper_and_return_register_2`: Calls a routine and returns the value left in a data register.
- `negate_scan_result`: Calls a routine to scan for a marker and returns the result, or the negation of that result if a secondary status flag is set in an address register.
- `format_string_buffer`: Constructs a formatted string buffer by iterating through a list of elements and appending control characters and data.
- `return_from_subroutine_8`: performs a return from the current subroutine.
- `update_printer_state`: Modifies bytes in a buffer and updates a status value based on various input conditions and comparisons with values in a global structure.
- `generate_escape_sequence_2`: Copies a sequence of bytes from a global table into a buffer and appends escape sequences based on values read from a structure.
- `generate_escape_sequence`: Writes specific byte sequences, including escape characters, into a memory location based on hardware or status checks.
- `update_status_bytes_2`: Modifies bytes at specific memory locations based on the value of a short integer.
- `update_status_bytes`: Modifies bytes at two different memory locations based on the value found at the second pointer.
- `clamp_and_assign_values`: Checks if a byte at a pointer is negative, then either sets that byte to zero and assigns a specific value to a word at another pointer or returns a value with all bits set.
- `return_maximum_value_6`: Returns a 32-bit value consisting of all bits set to one.
- `update_status_byte_2`: Checks a byte at a pointer and updates it or a secondary byte based on its value and a comparison against a constant.
- `reset_buffer_values`: Checks if a byte at a pointer is less than one; if so, it returns a specific value, otherwise it clears that byte and writes a constant to another location.
- `return_all_bits_set_2`: Takes no arguments and returns a 32-bit value with every bit set to one, matching the referenced helper that produces the maximum value.
- `update_values_by_lookup`: Compares a value pointed to by one register against several constants and stores specific byte values into memory locations based on those matches.
- `apply_conditional_bitmask_to_buffer`: Iterates through nested data structures to perform bitwise OR operations on a destination buffer.
- `apply_bitwise_mask_to_data`: Performs bitwise OR operations on a destination buffer based on values compared against a table within nested loops.
- `process_nested_loops`: iterates through data structures to perform bitwise operations and decrement counters based on values in a table.
- `call_indirect_function_36`: calls a function through a pointer stored at a specific offset from a global variable.
- `call_indirect_function_37`: calls a function through a pointer stored at a different offset from a global variable.

### HP_DeskJet

- `return_zero_11`: This routine returns a value of zero to the caller.
- `execute_library_calls_7`: Calls a function through a pointer stored at an offset from a global variable multiple times, returning a specific constant based on the success of these calls.
- `execute_indirect_calls_5`: Performs two consecutive calls to a function pointer retrieved from a global structure and returns a constant value.
- `execute_repeated_calls`: calls the same function pointer three times in succession.
- `return_zero_value_5`: The routine performs no operations and returns a value of zero to the caller.
- `build_hp_printer_command_buffer`: Constructs a command buffer by copying data from a global table and injecting specific ASCII characters based on hardware status flags.
- `format_printer_command_buffer_5`: Copies data from a global table into a destination buffer and appends a carriage return.
- `process_hp_deskjet_escape_sequences`: Parses and interprets control characters and escape sequences for an HP DeskJet printer.
- `parse_printer_command_sequence`: Parses a sequence of bytes to interpret control characters and command parameters.
- `copy_and_format_decimal_data`: Copies twelve bytes from a global variable into a destination buffer using a loop.
- `parse_hp_printer_escape_sequences`: Processes a stream of control characters and parameters to update a state byte and a counter.
- `update_hp_command_state_3`: Updates a status byte and a counter based on specific command codes.
- `update_hp_command_state_2`: Updates a status byte and a counter based on specific command codes.
- `update_printer_state_and_buffer_2`: Modifies a status byte and a character value based on specific command codes, and manages a data buffer by copying values from a global table and calling other routines to process the updated data.
- `process_printer_commands_2`: Modifies a status byte and a character value based on specific command codes and manages a data buffer by copying elements from a table.
- `update_printer_status_parameters`: Modifies status values or counters based on specific command bytes, including incrementing or decrementing values and copying data from a global table to a destination.
- `process_deskjet_command`: Handles printer commands by adjusting a status byte, incrementing counters, or copying data from a global table to a destination.
- `process_printer_command_3`: Evaluates a command byte to perform arithmetic on a pointer or copy a sequence of bytes from a global variable.
- `process_printer_buffer`: Checks a status byte and either decrements a counter, sets a byte at a specific offset, or copies eleven bytes from a global table to a destination.
- `update_buffer_and_call_2`: Checks a value against specific constants to either decrement a byte through a pointer or write a constant to a memory location.
- `copy_eleven_bytes_3`: Decrements a value pointed to by a memory location and copies eleven bytes from a global variable to a destination buffer.
- `copy_eleven_bytes`: Copies eleven bytes from a global variable to a destination and then calls another routine twice to format decimal values.
- `copy_and_format_decimal_pair`: Copies eleven bytes from a global table to a destination buffer and then calls two separate routines to format a decimal value and the difference between two subsequent bytes as decimal strings.
- `copy_and_format_decimal_data_2`: If a byte at a pointer equals a specific value, it copies eleven bytes from a global variable to a destination and calls two formatting routines.
- `copy_table_elements`: Copies eleven bytes from a global table into a destination buffer pointed to by a register.
- `write_byte_conditionally_2`: checks if a value at a pointer is zero and, if so, stores a byte into a memory location offset from another pointer.
- `format_three_digit_string`: Converts a numeric value into a three-digit decimal string stored in a buffer.
- `format_decimal_buffer`: Converts a byte into a three-digit decimal string and stores the characters into a buffer.
- `format_decimal_digits`: Stores a two-digit decimal representation of a byte into a buffer at a calculated offset.
- `format_numeric_character`: Writes a character representing a digit into a memory location by adding an offset to a value returned by another routine.
- `clear_flag_on_match`: If a second parameter matches a specific value, it clears a byte in a global structure.
- `execute_conditional_callbacks_2`: If a specific global value is non-zero, it calls two functions via pointers and then clears that global value.
- `divide_64bit_value`: Performs 64-bit division using a loop of bit shifts and subtractions or a modulo-based calculation.
- `rotate_and_subtract_bits`: Performs a series of sixteen bitwise left rotations and subtractions on a 64-bit value.
- `multiply_64bit_value`: Performs a 64-bit multiplication using 32-bit components and returns the result.
- `forward_call_to_helper_3`: Calls a single helper routine and then returns.
- `call_helper_and_return_register`: Calls a helper routine and returns the value left in a data register.
- `negate_result_on_flag_3`: Calls another routine and negates its return value if a flag left in an address register is nonzero.
- `call_and_negate_on_flag_4`: Calls another routine, then returns its first result or the negation of that result depending on a second result left in an address register.
- `format_printer_command_buffer_2`: Constructs and manages a buffer of escape sequences and ASCII characters for printer output.
- `restore_registers_and_return_2`: Moves the contents of register d5 into d0 and restores several data and address registers from the stack before returning.
- `apply_bitmask_to_buffer`: Iterates through a list of word-sized elements to perform bitwise OR operations on a destination buffer based on values from a table.
- `set_bits_from_table_2`: Iterates through a table of values to set specific bits within a buffer using bitwise operations.
- `set_bitfield_from_data`: Iterates through a nested loop to compare values from a source and sets bits in a destination buffer based on the comparison result.
- `call_indirect_via_table_2`: Calls a routine through a pointer stored at an offset within a global table.
- `call_indirect_via_table_3`: Calls a routine through a pointer stored at a different offset within a global table.
- `load_table_data`: Calculates offsets to extract values from a table and stores them into a global structure and several global variables.

### A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf:Installer

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### icon.library

- `return_error_constant`: Library code that returns a constant value of negative one.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_state`: As library code, it modifies a byte flag or manages a linked list of structures based on a status check.
- `set_byte_flag`: Library code that performs a bitwise OR to set the fourth bit of a byte at an offset from a provided address.
- `return_zero_12`: Library code that returns zero.
- `update_global_pointers`: Library code that copies two values into a structure, calls another routine, and updates global variables based on the result.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `return_from_subroutine_9`: This routine is an empty function that performs a single return instruction.
- `execute_empty_instruction`: This library routine contains a single instruction that performs no functional operation.
- `manage_icon_chain`: This library routine calls another routine to manage an icon chain.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `call_icon_data_copy_2`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `manage_icon_chain_2`: This library routine calls another routine to manage an icon chain.
- `recurse_with_increment`: This library routine calls another routine with the same first argument and an incremented second argument before returning.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `execute_recursive_call_2`: This routine performs a direct recursive call to itself before returning.
- `perform_incremental_recursion_2`: This library routine performs a recursive call with an incremented parameter until the callee stops the recursion.
- `call_other_routine`: This library code calls another routine and then returns.
- `execute_empty_library_call`: This library code is empty.
- `parse_numeric_string_5`: This routine converts a numeric string into an integer value and copies a portion of the source string to a destination buffer.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `traverse_and_modify_linked_list`: Recursively traverses a linked list structure to perform conditional operations on nodes.
- `process_icon_node_chain`: Library code that recursively traverses a structure via a pointer returned by another routine.
- `copy_and_update_structure`: Library code that copies a block of data from one location to another and then updates several fields in a target structure, including a potential pointer assignment.
- `return_zero_stub`: Clears the d0 register to zero, restores d2 through d4 and a2 from the stack, unlocks a6, and returns.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `update_icon_node_properties_2`: Traverses a linked list of icon nodes to update their internal structure members.
- `update_icon_node_properties`: Traverses a linked list of icon nodes to update their internal structure members.
- `process_icon_nodes_2`: As library code, it traverses a linked list of nodes and updates structure members with values returned from a callback routine.
- `process_icon_changes`: As library code, it executes a series of function calls in a loop and updates a value in a structure based on the results.
- `execute_and_return_zero`: As library code, it calls another routine and always returns zero.
- `transfer_register_and_return`: Moves a longword from register d4 to d0, restores registers d2 through d5 and a2 through a5 from a stack offset, unlocks a6, and returns.
- `dispatch_icon_callback_chain`: Iterates through a series of function pointers and data structures stored within an icon structure, executing them sequentially.
- `execute_icon_callback_chain`: Iterates through a series of function pointers stored within an icon structure and executes them sequentially.
- `iterate_icon_table_2`: As library code, it iterates through a table of pointers and executes a callback for each entry while managing a counter.
- `iterate_icon_table`: Library code that traverses a null-terminated table of pointers, executes a callback for each entry, and performs recursive calls on specific table elements.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `traverse_linked_list`: Library code that recursively traverses a linked list structure by following pointers until a termination condition is met.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `execute_conditional_callback_chain_2`: Calls a routine with specific arguments and, if it returns a non-zero value, executes a sequence of subsequent calls using offsets derived from that return value.
- `return_zero_library_code`: Library code that is empty and simply returns zero to the caller.
- `return_zero_library_code_3`: As part of library code, this routine sets register d0 to zero, restores d2, d3, a2, and the stack pointer, then returns.
- `update_icon_data`: Library code that decrements a counter through a pointer and uses the result to index a table to store two values.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `initialize_library_logic`: Library code that performs conditional checks and executes a series of routines to initialize or update internal structures based on the success of previous calls.
- `return_void_stub`: Library code that performs no operation and returns.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `find_delimiter_in_list`: As library code, it iterates through a list of pointers to find a specific character within a string pointed to by those entries.
- `find_string_character`: As library code, it searches through a list of pointers to find a specific character in a string.
- `find_string_delimiter`: As library code, it traverses a list of pointers to find a specific character within a string.
- `find_matching_difference`: As library code, this routine iterates through a linked structure to calculate the difference between a value and a target, returning success if a matching difference is found.
- `search_list_for_value_2`: As library code, it iterates through a linked structure, calculating offsets and comparing them against a target value to find a match.
- `search_linked_list_node`: Iterates through a linked structure by comparing values and incrementing a pointer or index.
- `calculate_offset_difference`: As library code, it performs a loop to calculate a difference between values derived from another routine.
- `compare_offset_differences`: Iterates through a linked structure to calculate the difference between a value and a target, returning success if a matching difference is found during traversal.
- `convert_string_to_integer_and_copy`: Library code that converts a numeric string into an integer value and then copies a portion of the source string to a destination buffer.
- `parse_numeric_string_4`: Library code that converts a string of characters into an integer and executes a function via a pointer.
- `convert_string_to_integer_2`: As library code, this routine iterates through a string of characters, converting each digit into an integer value and accumulating the result.
- `convert_string_to_integer`: As library code, it iterates through a string of characters, converting each digit into an integer value and accumulating the result through a base-10 multiplication loop.
- `convert_digit_char`: As library code, it converts a character digit to its integer value or returns negative one if out of range.
- `return_negative_one_2`: Library code that returns a value of all bits set.
- `execute_indirect_function_53`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_pointer_17`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_18`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_indirect_function_53`: This library routine fetches a function address from a calculated location and executes it.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `initialize_library_table`: Library code that calls another routine and, if successful, writes a value to a memory location and updates a pointer.
- `return_zero_13`: This library routine returns zero to the caller.
- `compare_and_return_zero`: This library routine calls another routine and returns zero if the result does not match a specific parameter, otherwise it returns the result.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `compare_and_return_zero_2`: As library code, it calls another routine and returns zero if the result does not match a provided parameter.
- `return_void`: This library routine performs no operations and returns.
- `perform_complex_division`: This library routine performs division and modulo operations on a 32-bit value using a divisor, involving bitwise shifts and conditional subtractions.
- `perform_bit_rotation_3`: Library code that performs a bitwise rotation and conditional subtraction on a value over a fixed number of iterations.
- `multiply_and_combine`: Library code that performs a complex multiplication of 16-bit components from two inputs and returns a concatenated 64-bit result.
- `call_function_via_offset`: This library routine retrieves a function pointer from a global variable using a specific offset and executes it.
- `store_byte_at_pointer`: This library routine stores a byte from a register through a pointer.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `find_char_in_string_2`: As library code, it searches for a specific character within a null-terminated string and returns its location.
- `return_zero_15`: This routine is an empty function that immediately returns zero to the caller.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `copy_string_to_destination`: This library routine finds the end of a source string and copies the contents of a second string to that location.
- `compare_string_segments`: As library code, it compares two strings up to a specified length to determine if they match.
- `return_constant_one_4`: This library routine returns the integer value one.
- `skip_string_bytes`: As library code, it iterates through a string for a specified number of bytes or until a null terminator is reached.
- `decrement_pointer_and_call`: As part of library code, this routine decrements the address in a0 by one, calls another routine, and then returns.
- `copy_and_pad_string`: This library routine copies a string of a specified length to a destination and pads the remaining space with null bytes.
- `return_immediately_9`: This library routine is empty and performs no operations before returning.
- `initialize_structure_elements`: This library routine writes the address of the argument to a location, increments that value by four, sets a second word to zero, and then stores the original address in a third word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `traverse_linked_list_6`: As library code, it iterates through a linked list of structures, calling another routine for each node until a condition is met or the list ends.
- `check_library_condition`: As library code, it repeatedly calls a function and checks if a specific value in a structure is zero.
- `return_zero_library_code_2`: This library code sets d0 to zero and returns.
- `propagate_icon_changes`: As library code, it performs a series of nested calls to update icon properties and returns a status value.
- `return_negative_one`: This library routine returns a value of all bits set to one.
- `process_icon_nodes`: As library code, it traverses a linked list of nodes, updating pointers and performing operations on each node's data through repeated function calls.
- `traverse_icon_list`: As library code, it iterates through a linked list of structures, updating pointers and values during traversal.
- `update_icon_list_node`: Iterates through a linked list of structures to update specific fields within each node.
- `return_maximum_value`: This library routine returns a value representing all bits set to one.

### info.library

- `halt_execution`: This library routine is empty.
- `store_library_state`: The routine stores the incoming register values into a global variable and a table, and writes a pointer through an offset in a structure, then stores that pointer into another global variable.
- `call_library_helper_2`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `call_decrement_and_traverse`: As library code, this routine calls a routine that decrements a value within a structure and conditionally traverses it.
- `call_recursive_handler`: This library routine calls another routine without performing any local operations.
- `return_void_2`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `reset_library_state_recursive`: This library code calls itself indefinitely, resulting in infinite recursion, before finally calling a routine that resets the library state.
- `clear_flag_and_increment_counter_2`: The routine clears a specific bit in a byte field of a structure pointed to by its argument and increments a 16-bit counter field within the same structure.
- `update_status_flag_recursively`: Checks a short value at an offset within a structure to determine whether to perform a recursive traversal or set a specific bit in a byte at a different offset.
- `call_library_functions_with_offset_args`: This library routine retrieves a library base from a global variable and calls two library functions using offsets and calculated arguments.
- `decrement_and_traverse_structure`: This library routine decrements a short value at a specific offset within a structure and conditionally calls itself based on the new value and a bit test in a byte.
- `return_zero_16`: This library routine returns zero.
- `recursively_call_pointers`: The library routine checks several global pointers and arguments, then calls itself recursively if a pointer is non-zero.
- `initialize_library_internal_data`: Allocates a memory block and populates a library-specific data structure by copying values from a global table into various calculated offsets.
- `initialize_info_library_struct`: Initializes a structure or table by calculating and storing several offsets or pointers derived from a base address.
- `verify_callback_chain`: As library code, it executes a sequence of function calls via pointers and updates global variables with the results.
- `validate_function_pointer_sequence`: Library code that repeatedly calls a function via a pointer to verify non-zero return values and updates three global variables with the results.
- `check_function_result_2`: Library code that calls a function via a pointer and checks its return value to decide whether to call another routine with three arguments.
- `check_function_result`: This library routine calls a function via a pointer, checks if the result is non-zero, and calls another routine with three arguments if the result is zero.
- `return_one`: This library routine returns one.
- `get_nested_value_3`: Library code that traverses a structure through multiple pointers to retrieve a specific value based on conditional checks of structure members.
- `traverse_nested_structure_2`: This library routine follows multiple pointers within a structure to retrieve a value, using conditional logic to select between different offsets.
- `traverse_nested_structure`: This library routine follows multiple pointers within a structure to retrieve a value, using conditional logic to select between different offsets based on the contents of the data.
- `get_nested_value_2`: This library routine retrieves a value from an offset within a structure that is pointed to by a field in another structure.
- `get_nested_value`: This library routine retrieves a value from an offset within a structure that is pointed to by a field in another structure, provided the initial pointer is not zero.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `retrieve_nested_value`: Library code that retrieves values through nested pointers and updates a global variable.
- `check_library_status_and_set_flag`: Checks a boolean flag and a function return value to decide whether to update a global flag and retrieve a value from a nested pointer.
- `update_callback_data_2`: As library code, this routine updates a counter in a structure, performs string operations, and executes a callback if a specific pointer is non-zero.
- `update_callback_data`: As library code, it updates a structure with values from a library call and conditionally executes a callback routine.
- `update_callback_state`: Library code that updates a counter in a structure and performs conditional calls to other routines.
- `update_callback_structure`: Library code that executes a function, updates a counter in a structure, and conditionally executes a callback routine while storing values from other structures.
- `update_table_and_callback`: This library routine updates a counter in a structure, performs string operations, and executes a callback if a specific pointer is non-zero.
- `copy_library_strings_and_data`: This library routine copies a string using a library function and then copies certain dwords from one structure to another.
- `execute_library_callback_chain`: As library code, it initializes several global variables and executes a complex sequence of nested callback functions via pointers.
- `execute_nested_callback_chain`: Performs a series of conditional function calls and validates return values before executing a final callback through a function pointer.
- `execute_library_callback`: Library code that performs nested function calls and conditional logic before invoking a callback function with various parameters.
- `execute_conditional_callback_chain_3`: Evaluates a series of conditional checks and nested function calls based on return values and global variables.
- `execute_callback_chain_2`: Library code that retrieves a function pointer from a global variable and executes a sequence of callbacks.
- `execute_callback_logic`: Library code that validates parameters and executes a function pointer through a table, potentially calling a callback with different arguments based on the result.
- `execute_callback_on_error`: Library code that retrieves a value and executes a function pointer with specific arguments based on the result.
- `validate_library_parameters`: As library code, this routine checks a global variable, calls a library function via a global base, and executes a parameter-validated routine.
- `execute_indirect_function_35`: Fetches a function pointer from a location within a table and executes it.
- `execute_indirect_function_36`: Fetches a function pointer from a memory location and executes it.
- `execute_indirect_function_pointer_6`: Fetches a function pointer from a memory location and executes it.
- `call_indirect_handler_12`: Library code that fetches a pointer from a table and executes it.
- `call_indirect_handler_13`: Library code that performs an indirect call to a function pointer retrieved from a table.
- `call_indirect_handler_14`: Library code that jumps to a subroutine address stored within a table.
- `call_indirect_handler_15`: Library code that executes a function located at an offset within a table.
- `call_indirect_handler_16`: Library code that loads a pointer from a table and branches to that address.
- `call_indirect_handler_17`: Library code that calls a subroutine via a pointer found in a data table.
- `call_indirect_handler_18`: Library code that performs an indirect jump to a pointer stored in a table.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_indirect_handler_19`: Library code that retrieves a function pointer from a table and executes it.
- `call_indirect_handler_20`: Library code that executes a subroutine by following a pointer in a table.
- `execute_indirect_function_40`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_function_41`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_42`: Retrieves a function pointer from a global table using an offset and executes it.
- `execute_indirect_function_pointer_11`: Retrieves a function pointer from a global table using a fixed offset and executes it.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_12`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_13`: Fetches a function pointer from a global variable and executes it.
- `execute_library_function_via_offset_2`: Retrieves a function pointer from a table using a calculated offset and executes it.
- `execute_indirect_function_43`: Fetches a function pointer from a global table using an offset and executes it.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_14`: Fetches a function pointer from a location calculated by adding an offset to a global variable and executes it.
- `call_indirect_handler_21`: Library code that branches to a subroutine address retrieved from a table.
- `call_indirect_via_table_4`: This library routine calls another routine indirectly through a pointer stored in a global variable table.
- `execute_indirect_function_pointer_7`: Retrieves a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_37`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_pointer_8`: Fetches a function pointer from an offset within a global data structure and executes it.
- `execute_library_function_via_offset`: Retrieves a function pointer from a table using a fixed offset and executes it.
- `execute_indirect_function_pointer_9`: Fetches a function pointer from a global table using a specific offset and executes it.
- `execute_indirect_function_pointer_10`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_38`: Fetches a function pointer from a location offset from a global variable and executes it.
- `execute_indirect_function_39`: Fetches a function pointer from a location offset from a global variable and executes it.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_by_type`: This library routine performs different actions based on a value retrieved from a structure, including calling other routines or updating a global variable.
- `return_constant_one_2`: Library code that returns the integer value one.
- `toggle_global_bitmask`: Library code that selects a bitmask based on a value in a structure and performs an XOR operation on a global variable.
- `store_register_to_global`: As library code, this routine stores a register value into a global variable offset by a fixed amount and then calls another routine using a pointer from a structure.
- `validate_string_attributes`: Checks a data structure for specific attribute flags and validates a string based on its type.
- `process_string_attributes`: Library code that checks a flag in a data structure to select between a global variable or a pointer, then processes a string and several other data elements through various routine calls.
- `process_string_and_attributes`: This library routine copies a string, calls a library function, and performs several conditional checks involving pointer offsets and string copies, potentially updating a local variable with a value from a structure.
- `test_multiple_handlers`: Library code that attempts to execute four different function pointers sequentially and returns one if all calls succeed or zero if any return a negative value.
- `repeat_function_call_2`: Library code that repeatedly executes a function pointer until it returns negative one.
- `repeat_function_call`: Library code that repeatedly executes a function pointer until it returns negative one, returning one if successful.
- `call_and_repeat_on_success`: Library code that executes a function pointer and calls itself recursively if the return value is not negative one.
- `call_other_and_return_one`: As library code, this routine calls another routine and then returns one.
- `initialize_info_library_tables`: Library code that initializes internal tables by repeatedly calling another routine with various constant arguments and a pointer derived from the input parameter.
- `execute_list_of_library_calls`: This library routine calls another routine multiple times with a fixed first parameter and varying other constant arguments.
- `initialize_display_palette_entries`: The routine calls another routine or a sequence of calls with a fixed set of arguments, passing the same first parameter each time and varying the remaining constants.
- `register_info_table_entries`: The routine calls another routine ten times with a fixed set of integer arguments, passing the same first parameter each time and varying the remaining constants.
- `set_info_table_entries`: The routine calls another routine ten times with a fixed set of six integer arguments each time, passing the same first parameter each time.
- `execute_conditional_recursive_traversal`: Performs recursive calls based on a comparison between a value in a structure and its incremented version.
- `execute_conditional_indirect_calls_2`: Library code that performs recursive calls based on a comparison of a value in a structure and its incremented version.
- `execute_conditional_indirect_calls`: This routine calls a function via a pointer retrieved from a structure member, and then conditionally calls another routine if a register is non-zero.
- `populate_linked_list_structure_2`: Library code iterates through a list of pointers to populate a destination structure and updates a linked list head.
- `update_linked_list_3`: Library code that iterates through a list of pointers to populate a destination structure and updates a linked list head.
- `process_pointer_list`: Library code that iterates through a list of pointers, performing operations and storing results into a structure.
- `populate_linked_list_structure`: Iterates through a list of pointers to populate a destination structure with values and linked elements.
- `update_pointer_from_structure_2`: As library code, this routine calls a routine with a pointer from a structure, then updates that structure member with a value from another pointer.
- `update_pointer_from_structure`: Library code that calls a routine with a pointer from a structure, then updates that structure member with a value from another pointer.
- `update_pointer_dereference_2`: As library code, this routine calls a routine and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `update_pointer_dereference`: Library code that calls a routine and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `modify_global_state_and_execute`: Library code that sets a global variable, checks a condition, calls several library functions, and updates a global pointer.
- `execute_sequential_callbacks_2`: As library code, this routine fetches a value from a pointer, calls a routine, and then updates a structure member with either the fetched value or a new value.
- `update_structure_member_sequentially`: Library code that fetches a value from a pointer, calls a routine, and then updates a structure member with either the fetched value or a new value from a pointer.
- `execute_sequential_callbacks`: Library code that calls another routine twice using pointers or offsets or registers.
- `convert_list_to_table_2`: Library code that traverses a linked list, allocates a contiguous memory block, and populates a table with data extracted from the list nodes.
- `convert_list_to_table`: Library code that counts elements in a linked list, allocates a contiguous memory block, and populates it with data from the list.
- `allocate_and_populate_table_from_list`: Calculates the number of elements in a linked list to determine the required memory size, allocates a contiguous table, and then iterates through the list to copy data from each node into the new table.
- `map_linked_list_to_table`: Traverses a linked list to calculate the total size required for a destination table, then iterates through the list again to populate the table with values retrieved via indirect function calls.
- `extract_list_to_table`: Library code that traverses a linked list to extract values and stores them into a destination table via indirect calls.
- `process_data_list`: Library code that iterates through a list, calculates a value based on a string length, and stores results into a table via indirect function calls.
- `populate_data_table_from_list`: Iterates through a linked list of data structures to extract specific values and store them into a destination table.
- `call_library_offset_84`: Library code that calls a function at a specific offset from a pointer stored in a global variable.
- `execute_indirect_function_34`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_pointer_5`: Fetches a function pointer from a global table using an offset and executes it.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_formatted_data_sequences`: Selects a sequence of data parameters based on a switch value and passes them to another routine for processing.
- `return_nothing`: Library code that performs an empty return.
- `initialize_resource_handles`: Attempts to acquire a series of resource handles by calling another routine with specific parameters and storing the results in global variables.
- `execute_conditional_calls`: Library code that performs a series of function calls via a pointer and then executes a variable number of additional calls based on a value in a global structure.
- `execute_repeated_calls_2`: This library routine performs a variable number of calls to another routine based on a value found in a global structure.
- `restore_library_context`: As part of library code, this routine restores d2, d3, a2, a3, and a4 from the stack, unlocks a6, and returns.
- `process_indexed_allocation`: Library code that checks a value in a global structure and either clears a global variable or calls another routine with a pointer, a global variable, and a value from the structure.
- `execute_indexed_allocation`: Library code that calls a routine with three parameters including a pointer from a table, then calls another routine and returns its result or zero if it is negative.
- `return_from_subroutine_10`: This routine is an empty function that performs no operations and returns immediately to the caller.
- `validate_constant_sequence`: Library code that populates a series of memory locations with specific constant values if a condition is met, then verifies a sequence of four function pointers.
- `validate_routine_success`: Calls a routine through a pointer up to three times.
- `verify_function_return`: Library code that calls a routine twice and returns one if the results are not negative one, and zero otherwise.
- `check_constant_return`: Library code that calls another routine and returns the result of a comparison against negative one.
- `return_constant_integer`: This library routine returns the integer value one to its caller.
- `dispatch_event_loop`: As library code, it enters a loop that either calls a subroutine via a pointer or executes a routine based on a bit-shifted value.
- `return_constant_zero`: Library code that returns the integer value zero.
- `dispatch_by_value`: Library code that inspects a value from a structure and performs different actions, including calling other routines or returning specific constants based on that value.
- `dispatch_by_type_2`: Library code that performs different actions based on a value retrieved from a structure, including calling other routines or updating a value.
- `call_nested_routine`: Calls another routine and returns zero.
- `execute_two_calls`: Library code that calls another routine twice and returns a constant value.
- `execute_multiple_callbacks_2`: As library code, this routine calls another routine or a global function pointer multiple times based on a condition checked against a global variable, then calls another routine with a specific pointer and constant, and finally calls another routine with a certain pointer and a constant.
- `setup_string_template`: Library code calculates a bitmask from a value in a table and calls a string-formatting routine multiple times with different bit-masked arguments and a global pointer.
- `format_complex_string`: Library code that prepares arguments and calls a string formatting routine using a specific pattern and a local buffer.
- `format_string_arguments`: Library code that prepares a pointer-based argument list or a temporary buffer for a string-formatting routine that uses a pattern or template string and multiple string arguments.
- `calculate_string_length_7`: Iterates through a null-terminated string to determine its total number of characters.
- `calculate_string_length_5`: Library code that increments a register while checking if a null terminator has been reached at an offset from a pointer.
- `copy_null_terminated_string_3`: Copies a null-terminated byte string from a source pointer to a destination pointer.
- `copy_byte_sequence_2`: Iterates through a specified number of bytes, copying each byte from a source memory location to a destination memory location.
- `copy_memory_block_7`: Library code that copies a fixed number of bytes from a source pointer to a destination pointer.
- `compare_null_terminated_strings_2`: Iterates through two character sequences until a mismatch or a null terminator is encountered.
- `compare_strings_2`: Library code that compares two null-terminated strings and returns true if they are equal.
- `get_string_length`: Library code that counts the number of bytes in a null-terminated string.
- `execute_library_operations`: Library code that calls several other library functions, including a routine that initializes a structure array and routines that use conditional logic or call library vectors.
- `call_string_formatter`: Library code that calls another routine using a pointer to a global variable and a local buffer, then calls another routine using the populated buffer and other passed parameters.
- `copy_length_prefixed_string`: Library code that reads a byte from a table to determine a count, copies that number of bytes to a destination, and appends a null terminator.
- `copy_bytes_with_null`: Library code that copies a sequence of bytes from a source address to a destination address and appends a null terminator at the end of the destination range.
- `initialize_record_structure`: Library code that populates two different data structures with various parameters and offsets, then executes two routines and performs a conditional call based on a return value.
- `execute_library_function`: Library code that calls a library function using a pointer retrieved from a global variable offset.
- `execute_library_calls_3`: Library code that calls a library function or another routine multiple times using a pointer retrieved from a structure member.
- `execute_indirect_function_54`: Library code that fetches a function pointer from a global variable offset and executes it.
- `store_byte`: Library code that stores a byte into a memory location.
- `call_indirect_via_offset_5`: The routine calls another routine indirectly through a pointer located at an offset from a global variable.
- `call_indirect_via_table_5`: The routine calls another routine through an indirect pointer taken from a global table.

### version.library

- `return_all_ones`: This library routine returns a value where all bits are set.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call`: Library code that decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `check_library_status`: Library code that inspects a structure to either perform two indirect function calls or set a specific bit in a byte.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_zero_17`: This library routine performs no operations and returns zero to the caller.

### diskfont.library

- `check_and_set_font_handle_2`: This library routine calls another routine via a table entry using a library base, and if the result is non-zero, it stores a value at an offset and calls a second routine via a table entry.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `cleanup_library_resources`: Attempts to close two libraries via indirect function calls through a base pointer.
- `release_library_resources`: Library code that performs two indirect function calls to release memory or close a library.
- `process_font_segments`: Library code that iterates through a linked list of structures to manage memory or segments.
- `manage_library_state`: Library code that checks specific status flags and either executes a series of function calls or sets a bit in a status byte.
- `set_high_bit`: This library routine sets the highest bit of a byte at a specific offset from a base pointer and returns zero.
- `return_no_op_handler`: This routine is a single return instruction with no body, so it performs no work and leaves all state unchanged.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_check_counter_2`: Library code that decrements a short integer in a structure and executes an indirect function if the value reaches zero and a specific bit is set.
- `return_zero_18`: This library routine performs no operations and returns zero.
- `populate_font_metadata_table`: Iterates through a linked list of font data structures to extract character properties, string lengths, and offsets.
- `populate_font_data_buffer`: Iterates through a linked list of data structures to calculate string lengths and copy character data into a destination buffer.
- `parse_font_data_stream`: Iterates through a linked list of data structures to extract and copy string content into a destination buffer.
- `parse_font_data_stream_2`: Iterates through a font data stream to extract character properties and offsets.
- `parse_font_metadata_table_2`: Iterates through a font metadata structure to extract character properties and offsets.
- `parse_font_metadata_table`: Iterates through a data structure to extract font character properties and offsets.
- `load_font_data_from_disk`: Reads font character data from a disk sector into a memory buffer.
- `parse_font_metadata_table_3`: Iterates through a font metadata structure to extract character properties and offsets.
- `return_zero_and_restore_registers`: Sets register d0 to zero, restores registers d2 through d6 and a2 through a4 from the stack, unlocks the stack pointer, and returns.
- `calculate_bitmask_score`: This library code calculates a value based on the difference between two short integers and adjusts it by subtracting values from a table based on bitwise comparisons of two bytes.
- `compare_bitmask_patterns`: Library code that iterates through bit positions to compare bytes and subtracts values from a running total based on a table.
- `test_bitmask_pattern`: Iterates through a bitmask or table using bitwise shifts and comparisons to check for specific bit patterns.
- `process_bitmask_table`: Iterates through a bitmask or table using a loop that performs bitwise shifts and comparisons.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_glyph_metadata`: Iterates through a font table to find the entry with the highest priority and updates its metadata, including status flags and reference counts.
- `validate_and_update_font_metadata`: Iterates through a font data table to identify the highest priority entry and updates its associated metadata, including status flags and reference counts.
- `update_font_priority_metadata`: Iterates through a font data table to find the entry with the highest priority value and updates its associated metadata, including status flags and counters, within a memory buffer.
- `select_highest_priority_font`: Iterates through a font data table to identify the entry with the highest priority value.
- `search_font_table_priority`: Iterates through a font data table to identify the entry with the highest priority value.
- `update_font_cache_entry`: Iterates through a font table to find the entry with the highest priority or value and updates its metadata.
- `search_font_table`: Iterates through a table of font data to find an entry matching specific criteria.
- `parse_disk_font_table`: Iterates through a data structure to locate and extract font character information.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_exec_forbid`: Library code that calls an Exec function to forbid multitasking.
- `call_library_handler`: As part of a library, this routine calls a function pointer stored at an offset within a global structure.
- `call_alloc_handler`: As part of a library, this routine calls a function pointer located at a specific offset in a global structure.
- `execute_indirect_function_44`: Fetches a function pointer from a global variable and executes it.
- `execute_indirect_function_pointer_15`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_dos_open`: Library code that calls a DOS function to open a file.
- `call_dos_close`: Library code that calls a DOS function to close a file.
- `call_dos_read`: Library code that calls a DOS function to read data.
- `call_dos_lock`: Library code that calls a DOS function to lock a file.
- `call_dos_unlock`: Library code that calls a DOS function to unlock a file.
- `call_dos_examine`: Library code that calls a DOS function to examine a file.
- `call_dos_exnext`: Library code that calls a DOS function to examine the next file in a directory.
- `call_dos_current_dir`: Library code that calls a DOS function to retrieve the current directory.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_to_string_terminator_and_call_2`: This library routine increments a pointer until it reaches a null terminator and then calls another routine with that pointer.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_null_terminated_bytes_2`: Counts the number of bytes in a null-terminated string.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_string_values`: Library code that compares two strings and returns zero if they are identical, one if the first is greater than the second, or a negative value if the second is greater.
- `compare_string_sequences`: Library code that checks for repeating characters in a string and returns a constant based on a comparison of adjacent bytes.
- `compare_string_sequences_2`: This library code compares two null-terminated strings byte by byte and returns a result indicating their relative order or equality.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `search_string_forward`: Library code that searches for a specific byte within a string by iterating forward through memory until the byte is found or a null terminator is reached.
- `search_string_loop`: Library code that iterates through a string to find a matching byte, returning a pointer to the location of the match.
- `find_character_in_string`: Library code that searches for the first occurrence of a specific byte within a string and returns a pointer to that location or zero if not found.
- `search_string_backward`: Library code that finds a resident resource and then searches backward through a string to find a specific byte.
- `search_string_reverse`: Library code that searches backward through a string for a specific byte until a boundary is reached.
- `search_string_decrement`: Library code that iterates backward through a string by decrementing a pointer until a specific byte is found.
- `search_delimiters`: Library code that searches for specific delimiter characters in a string and increments the input value if a match is found.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `execute_indirect_handler`: Sets a global variable and modifies a memory location before performing an indirect call while managing the processor interrupt enable bit.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### A-Train (1992)(Ocean)(Disk 1 of 2)[h SKR].adf:A-Train

- `initialize_program_state`: This routine clears a block of memory, updates several global values, and executes a function pointer from a global structure.
- `load_and_validate_resource_table`: Attempts to open a resource file and read a structured table into memory.
- `process_dialog_input`: Checks conditional flags to determine if a user input or selection routine should be executed.
- `update_memory_values`: This routine copies a longword from one memory location to another and stores a modified byte into a table using a calculated index.
- `call_update_routines`: Calls a routine to update memory values and a routine to update the camera viewport.
- `update_camera_viewport`: Calculates the difference between current and target screen coordinates to determine necessary scrolling offsets.
- `update_viewport_scrolling`: Calculates horizontal and vertical displacement by comparing current and target coordinates, then clears specific screen regions to facilitate smooth scrolling.
- `update_display_scrolling`: Updates display coordinates and performs complex screen scrolling operations based on conditional checks of vertical and horizontal offsets.
- `execute_conditional_logic`: Performs conditional function calls, updates several memory locations from a table, clears two longwords, and executes a subroutine.
- `update_game_state`: Updates various memory locations and longwords based on conditional checks and calls several subroutines with different parameters.
- `execute_conditional_logic_variant`: Updates memory locations with values from a table, clears two longwords, and performs conditional function calls with different arguments.
- `execute_conditional_logic_extended`: Executes conditional function calls based on a signed value, updates memory from a table, and clears two longwords.
- `execute_complex_logic`: This routine performs conditional function calls, updates several memory locations with values from a table, clears two longwords, and executes multiple other subroutines based on various state checks.
- `update_sprite_position_logic`: Calculates new coordinate offsets for two entities based on current position, velocity, and boundary constraints.
- `update_state_and_process_grid`: Compares a byte against a global variable and, if changed, checks for a sequence match to increment a counter and trigger a large loop.
- `update_grid_value`: Searches a grid for a value and, if found, updates a specific byte in a memory-mapped grid structure.
- `search_grid_for_value`: Performs a nested loop search through a two-dimensional table to find a specific short integer value.
- `calculate_grid_coordinates`: Calculates a target position by iterating through a lookup table based on input parameters.
- `update_state_counters`: Increments a counter based on a switch-case value and conditionally updates multiple memory locations from a table.
- `update_state_status_values_3`: Updates a value in a structure based on a lookup table and performs conditional data copies to various memory locations.
- `update_state_conditional_data`: Checks a value against a threshold and, if it meets specific bitwise criteria, calls a function pointer to determine a new state.
- `update_state_status_values`: Updates several longwords in a data structure based on the return value of a function call and specific bitwise conditions.
- `update_state_status_values_2`: Selects a value from a lookup table based on an input register and updates a data structure field.
- `update_state_values_3`: Updates several longwords in a data structure based on the return value of a function call and specific bitwise conditions.
- `update_state_table_values`: Evaluates a return value from a function call to conditionally update a series of longwords within a data structure.
- `copy_table_values`: Copies several longwords from one area of memory to another and clears a global byte.
- `update_conditional_table_values`: Checks a value in a register against specific thresholds to conditionally write constant values to offsets within a table pointed to by a register.
- `set_status_values`: Checks a value and updates specific offsets in a table with constant values based on certain conditions.
- `set_status_flags`: Sets a longword value in a table based on the value of a register and a conditional check.
- `update_coordinate_bounds_2`: Checks if a value is within a specific range and, if a secondary condition is met, writes a constant value to a memory location.
- `return_no_op_3`: The routine consists of a single return instruction and performs no work.
- `validate_table_indices`: Performs a loop to calculate indices using a table and then checks if values retrieved from a multi-dimensional array fall within a specific range.
- `validate_table_values`: Fetches two short values from a multi-dimensional table and checks if both fall within a specific numerical range.
- `check_value_ranges`: Fetches two short values from a multi-dimensional table and checks if both fall within a specific numerical range.
- `check_range_value`: Returns a longword containing all ones if a value falls within a specific range, otherwise returns zero.
- `return_zero_23`: An empty routine that simply returns zero.
- `update_coordinate_state_3`: Updates several values in a structure using data from a table and checks if specific bounds are exceeded to set a status flag.
- `update_grid_position`: Updates several global variables with new coordinates derived from a table and increments a counter while checking bounds.
- `update_grid_coordinate_and_timer`: Updates a coordinate pair by adding an offset retrieved from a table based on a conditional check and a parity bit.
- `calculate_position_offset`: Calculates new values for two variables based on a table lookup and a conditional check.
- `decrement_grid_counter`: Updates coordinate variables and decrements a counter, ensuring it does not fall below zero.
- `clear_grid_cell_by_type`: Checks if a specific value matches certain constants and, if so, clears a byte in a memory grid and executes a subroutine with calculated offsets.
- `update_grid_elements`: Checks if a value matches specific constants and, if so, clears two bytes in a memory grid and calls another routine with calculated offsets.
- `clear_grid_data`: Clears specific bytes in a memory grid and calls a subroutine using values derived from a table.
- `clear_and_call_subroutine`: Clears a byte in a grid and calls a subroutine using values from a table.
- `parse_input_stream`: Iterates through a data stream to identify specific character patterns and bitmask conditions to return a combined status value.
- `parse_data_stream_3`: Iterates through a data stream to identify specific character patterns and bitmask conditions.
- `parse_data_stream_2`: Iterates through a data stream, checking for specific bitmask conditions and character patterns to determine a return value.
- `parse_string_data`: Iterates through a data stream to check for specific character patterns and bitmask conditions.
- `return_from_subroutine_11`: performs a subroutine return instruction.
- `check_input_condition`: Calls another routine to retrieve data and returns a boolean value based on specific bitmask and character matches.
- `clamp_and_update_coordinates`: Clamps two input values between zero and a specific limit, updates global variables with the results, and calls a function via a pointer in a global table.
- `generate_status_bitmask`: Constructs a byte of flags by evaluating specific bit patterns and numeric ranges within an input value.
- `calculate_bitmask_flags`: Computes a byte of flags by checking various bitwise conditions and numeric ranges against input values.
- `modify_byte_based_on_range`: Checks if a byte falls within several specific numeric ranges and a bitmask condition, then sets a bit in another byte if true.
- `clamp_global_value`: Adds two to a global variable and clamps the result to a maximum threshold.
- `transform_event_bytecode_stream`: Iterates through a linked list of event structures to extract command bytes and perform conditional transformations.
- `process_event_linked_list_node`: Iterates through a linked list of event structures, performing conditional updates to a global variable based on character-based type identifiers.
- `process_event_linked_list_2`: Iterates through a linked list of event structures, executing specific logic and modifying node data based on a type identifier.
- `update_event_linked_list`: Iterates through a linked list of event structures, modifying specific bytes within each node based on a type identifier.
- `process_event_linked_list`: Iterates through a linked list of structures to update state based on event type bytes.
- `update_linked_list_event_states`: Iterates through a linked list of structures to process event type bytes.
- `process_linked_list_node_events`: Iterates through a linked list of structures, performing conditional state updates and memory shifts based on specific byte values found within each node.
- `update_linked_list_node_states`: Iterates through a linked list of structures, performing conditional updates to a global state and modifying node members based on specific byte values.
- `process_linked_list_node_types`: Iterates through a linked list of structures and performs conditional updates to a global state and node members based on specific byte values found within each node.
- `process_linked_list_nodes_4`: Iterates through a linked list of structures, performing conditional updates to node members and global state based on specific byte values.
- `process_linked_list_nodes_3`: Iterates through a linked list of structures, performing conditional updates to node members and global state based on specific byte values.
- `traverse_linked_list_and_map_values`: Iterates through a linked list of structures, performing conditional checks on a byte value within each node.
- `traverse_linked_list_and_update_state`: Iterates through a linked list of data structures, performing conditional updates to a global state and modifying memory based on specific type flags found within the list nodes.
- `process_linked_list_nodes_2`: Iterates through a linked list of structures, performing conditional arithmetic and bitwise operations on data extracted from each node.
- `traverse_linked_list_and_transform_data`: Iterates through a linked list of structures, performing conditional arithmetic transformations on a byte value extracted from each node.
- `transform_input_event_stream`: Iterates through a linked list of event structures and performs character-based transformations on specific byte values.
- `process_input_event_stream`: Iterates through a linked list of event structures, performing character-based transformations and updating a global state based on specific byte values.
- `process_linked_list_nodes`: Iterates through a linked list of structures, performing conditional logic and value transformations based on specific byte values found within each node.
- `process_linked_list_node_3`: Iterates through a linked list of structures, performing conditional branching and data manipulation based on the value of a byte within each node.
- `traverse_linked_list_and_map_values_2`: Iterates through a linked list of structures, performing conditional checks on specific byte values within each node.
- `process_event_bytecode_stream`: Interprets a stream of event structures by extracting character-based command bytes and performing conditional transformations.
- `parse_bytecode_instruction`: Iterates through a linked list of data structures to interpret command bytes.
- `restore_registers_and_return_3`: Moves the contents of register d4 into d0, restores several registers from the stack, unlocks register a5, and returns.
- `pop_circular_list_entry`: Copies two bytes from a global pointer to a destination and then iterates through a circular list to find an entry matching a specific bit pattern.
- `search_circular_list`: Iterates through a circular linked list to find an entry matching a specific bit pattern.
- `return_constant_one_7`: Returns the integer value one to the caller.
- `search_and_clear_list`: Iterates through a circular linked list to find an entry with a specific bit pattern and clears a byte in that entry if a flag is set.
- `return_constant_value_3`: This routine returns the integer value one to its caller.
- `call_two_functions_via_table`: Calls two different functions using pointers stored at specific offsets within a global structure.
- `return_global_word`: Returns a two-byte value stored in a global variable.
- `search_circular_list_2`: Iterates through a circular linked list to find an entry where the first byte matches a specific value and a specific bit is set in the second byte.
- `scan_circular_list_for_pattern`: Iterates through a circular linked list to find an entry where the first byte matches a specific value and a specific bit is set in the second byte.
- `search_list_for_value`: Iterates through a circular linked list to check if any entry contains a specific byte value and a specific bit pattern.
- `return_constant_value_4`: The routine returns the integer value one to its caller.
- `clear_list_elements_2`: Iterates through a circular linked list and sets a byte to zero if certain bit patterns are met, unless a termination bit is present.
- `clear_list_elements`: Iterates through a circular linked list, setting bytes to zero until a specific bit pattern or termination condition is met.
- `return_no_op_8`: The routine consists of a single return instruction and performs no work.
- `reset_structure_pointer`: Sets specific fields within a structure pointed to by a global variable and calls functions via pointers stored within that structure.
- `initialize_input_devices`: This routine attempts to initialize several structures by calling functions via pointers, and if successful, performs a sequence of calls to reset or configure device-related data.
- `decrement_limits`: Subtracts one from two input values and stores the results in global variables.
- `dispatch_via_function_pointers`: Updates several global variables based on input parameters and executes four different function calls using pointers from a global structure.
- `increment_recursion_counter`: Increments a global counter and calls another routine, potentially resetting other globals if the counter was zero or reaches zero again.
- `manage_recursive_callback_depth`: Increments a global counter and calls itself recursively if the counter was zero, then decrements the counter and calls itself again if the counter reaches zero.
- `execute_wrapped_routine`: Calls another routine and, if the result is non-zero, manages a global counter and resets other global variables.
- `decrement_global_and_flag`: Decrements a global variable and sets another global variable to one if the decremented value reaches zero.
- `decrement_and_flag`: Decrements a global variable and, if it reaches zero, sets a global variable to one and calls another routine.
- `increment_and_clear`: Increments a global variable and, if the previous value was zero, sets two global variables to zero and calls another routine.
- `traverse_linked_function_list`: Updates a global variable by repeatedly calling functions stored at a fixed offset from the current value of that variable.
- `clear_and_call_function`: Repeatedly calls a function via a pointer in a global structure if it is non-zero, then sets that global variable to zero multiple times.
- `call_subroutines_sequentially`: Calls two other routines in succession.
- `perform_recursive_calls`: Calls itself twice in a recursive manner.
- `execute_recursive_callback`: Triggers a routine twice in succession.
- `execute_wrapped_subroutines`: Calls two separate routines sequentially.
- `call_increment_and_clear_routines`: Calls a routine that increments a global variable and conditionally resets two others, then calls another routine.
- `decrement_global_and_call_next`: Decrements a global variable and sets a flag if it reaches zero, then calls another routine.
- `return_from_subroutine_12`: The routine performs no operations and immediately returns to the caller.
- `return_from_subroutine_13`: This routine performs a single return instruction to exit the current function.
- `return_from_subroutine_14`: The routine performs an immediate return to the calling function.
- `return_from_subroutine_15`: This routine performs no operations and immediately returns to the caller.
- `return_from_subroutine_16`: The routine performs a single return instruction and is empty.
- `initialize_graphics_view`: Initializes a graphics view and populates several global variables with pointers and values based on display properties.
- `return_without_action_3`: The routine consists of three instructions that immediately return to the caller without reading or writing any registers, memory, or hardware.
- `free_graphics_resources`: Releases various graphics resources including color maps, copper lists, and viewports by calling library functions and clearing global pointers.
- `initialize_display_buffer`: Retrieves color maps and allocates multiple memory buffers based on a provided parameter.
- `initialize_graphics_structures`: Copies several blocks of data from a source table to a destination structure and resets various internal state variables.
- `execute_flagged_callback`: Checks a global variable and, if non-zero, calls a function pointer from a table and sets another global variable to one.
- `clear_flagged_callback`: Checks a global variable and, if non-zero, calls a function pointer from a table and sets that global variable to zero.
- `compare_linked_list_values`: Iterates through a linked list to compare values against a threshold, potentially updating two passed pointers and returning a status code.
- `process_linked_list_values`: Iterates through a linked list to perform arithmetic on values based on flags and offsets.
- `compare_offset_values`: Compares two values and returns specific integer constants if their difference matches one of two predefined offsets.
- `compare_and_return_value`: Compares two values and returns three if they match a specific offset, otherwise returns zero.
- `return_zero_24`: This is an empty routine that simply returns zero to the caller.
- `process_data_list_2`: iterates through a linked list of structures to perform arithmetic calculations and updates values based on specific flags and memory offsets.
- `return_zero_stub_16`: The routine consists of two instructions that load zero into the return register and return, performing no other work.
- `update_display_state`: Performs complex data transfers between tables, clears several global variables, and executes multiple function pointers from a table.
- `clear_and_update_coordinate_bounds`: Clears existing coordinate pointers and calls a routine to reset the coordinate state.
- `reset_coordinate_state`: copies a value from one memory location to another, clears several global variables, and executes a series of function pointers from a table.
- `update_coordinate_bounds`: Updates several global variables based on input parameters and performs multiple function calls from a table.
- `manage_display_list_and_scroll_bounds`: Clears and reassigns pointers within a data structure while updating a scrolling offset and window boundary variables based on comparison logic.
- `update_offset_values`: calculates new values using a function call and subtraction, stores them in several global variables, and executes multiple function pointers from a table.
- `update_view_window_bounds`: Calculates and updates viewport coordinate offsets and window dimensions based on comparison between a current value and a range defined by multiple global variables.
- `calculate_display_offsets`: calls a function to retrieve a value, updates two global variables with the result, and executes a series of function pointers from a table.
- `adjust_view_parameters`: performs conditional logic based on global values to calculate and store new coordinate offsets before executing several function pointers from a table.
- `calculate_offset_values`: Calculates and stores several short integer values into global variables and executes multiple function pointers from a table.
- `call_cleanup_routines`: Executes two function pointers retrieved from a table.
- `check_status_code`: calls a function through a pointer and returns a boolean indicating whether the returned value equals four.
- `return_zero_value_7`: Returns a constant value of zero immediately without performing any other operations.
- `validate_resource_container_header`: Validates a data stream by checking for specific magic number signatures and verifying header field consistency.
- `parse_resource_file_structure`: Iterates through a data stream to identify specific file headers and validate their contents.
- `validate_resource_file_header`: Checks a data block for specific magic numbers and validates that the file size matches the expected length.
- `execute_conditional_recursive_function`: Calls a function via a table and, if the result is non-zero, performs a comparison and potentially calls itself recursively.
- `validate_asset_file_header`: Checks a file for specific magic numbers and verifies that the file size matches the header information.
- `parse_resource_container_stream`: Iterates through a data stream to identify and validate specific file headers, such as those marked with specific magic numbers.
- `parse_file_header`: Reads and validates file headers by checking for specific magic numbers and comparing file sizes.
- `extract_high_word`: Extracts the high word from a value and conditionally calls a function pointer from a table.
- `execute_indirect_function_61`: Calls a function via a pointer retrieved from a structure and returns a value based on the result.
- `execute_indirect_call_5`: Retrieves a function pointer from a memory location offset from a pointer stored in a register and executes it.
- `chain_function_calls`: Calls a routine and, if the result is non-zero, uses that result as an argument to call another routine.
- `invoke_resource_validation`: Calls a routine to verify the integrity of a resource container header.
- `decompress_variable_length_stream`: Expands a compressed data stream into a destination buffer using variable-length byte sequences.
- `decompress_data_stream_2`: Expands a compressed data stream into a destination buffer using variable-length encoding.
- `expand_variable_length_stream`: Decompresses a data stream by reading length-encoded byte sequences and copying the specified number of subsequent bytes to a destination buffer.
- `decompress_data_stream`: Expands a compressed data stream into a destination buffer using variable-length encoding.
- `apply_control_stream_to_buffer`: Iterates through a control stream to modify a destination buffer by performing byte copies, recursive calls, or bitwise masking operations.
- `reconstruct_masked_buffer`: Iterates through a control stream to reconstruct a destination buffer by either copying bytes from a source or performing bitwise AND operations against a mask.
- `decompress_and_mask_stream_2`: Iterates through a control stream to reconstruct a buffer by either copying bytes from a source or performing bitwise AND operations against a mask.
- `decompress_and_mask_stream`: Iterates through a compressed data stream to reconstruct a buffer by either copying bytes from a source or performing bitwise AND operations with a mask.
- `process_structured_table`: Iterates through a table of entries to perform conditional updates.
- `apply_bitmask_stream`: Iterates through a control stream to either copy a block of bytes to a destination or perform a bitwise AND operation between a source buffer and a destination buffer.
- `apply_recursive_bitmask`: Recursively traverses a tree or table structure to perform bitwise AND operations on data pointed to by the entries.
- `process_recursive_bitmask_table`: Iterates through a table of pointers to perform bitwise AND operations on data structures.
- `decompress_data_stream_3`: Expands a compressed data stream into a destination buffer by iterating through a control structure.
- `decompress_bitmask_stream`: Expands a compressed data stream by reading control bytes that dictate either literal byte copies or bitwise AND operations against existing data in a destination buffer.
- `return_zero_value_18`: Takes no arguments and returns the constant 0 immediately, with no other operations.
- `decompress_bitmask_stream_2`: Processes a data stream by either copying literal bytes or performing bitwise AND operations on a destination buffer based on control flags.
- `apply_bitmask_stream_entry`: Processes an individual entry within a compressed data stream by either copying bytes from a source buffer or performing a bitwise AND operation between a source buffer and a destination buffer.
- `serialize_screen_color_data`: Extracts color information from a source and packs it into a 24-bit RGB format.
- `format_color_data_buffer`: Iterates through a sequence of values to extract nibbles and store them as three-byte color components in a buffer.
- `format_color_data_buffer_2`: Extracts color components from a source value and stores them as a sequence of three bytes in a destination buffer.
- `serialize_entity_data`: Iterates through a collection of data elements to pack nibbles and bytes into a buffer.
- `serialize_rgb_color_data`: Converts a sequence of color values into a packed 24-bit RGB format by extracting nibbles and storing them as three consecutive bytes in a destination buffer.
- `convert_palette_to_rgb24_buffer_2`: Iterates through a color palette to convert 4-bit color values into a 24-bit RGB format, storing the results in a destination buffer.
- `convert_palette_to_rgb24_buffer`: Iterates through a color palette to convert 4-bit color values into a 24-bit RGB format, storing the results in a destination buffer.
- `convert_rgb4_to_rgb24`: Converts 4-bit color values into a 24-bit RGB format by bit-shifting and storing the components into a destination buffer.
- `validate_compressed_resource_header`: Verifies the integrity and structure of a compressed data block by checking for specific magic constant signatures and validating length fields.
- `verify_compressed_data_integrity`: Validates a sequence of data by checking specific magic constants and length values against a structure.
- `update_status_flag_5`: Checks a condition and updates a status byte in a structure, then optionally calls a function via a pointer in a table.
- `check_status_four`: Calls a function via a pointer in a table and returns true if the result is four.
- `return_zero_and_restore_a6`: Sets register d0 to zero, restores register a6 from the stack, and returns.
- `check_status_one`: Calls a function via a pointer in a table and returns true if the result is one.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `accumulate_table_values_3`: Copies values from a table into a local stack array and then iteratively adds a value to each entry in that array using a function pointer.
- `accumulate_table_values_iteratively`: Copies data from one table to another before entering a nested loop that calls a function pointer for each entry.
- `process_nested_loops_2`: Iterates through a table, calling another routine and adding a value from a table to a specific memory location.
- `process_nested_loops_extended`: Performs a nested loop that updates a table with values from another routine and manages a secondary counter.
- `update_table_values_2`: Iterates through a table to add values from another table to a specific memory location while managing loop counters.
- `validate_compressed_sequence_integrity`: Iterates through a sequence of values to verify that each element matches a specific condition after being incremented by a base value.
- `expand_compressed_sequence`: Copies a block of data from a source buffer to a destination table based on a count specified in a header.
- `process_table_with_callback`: Iterates through a table, calling another routine for each element and updating a table based on the return value.
- `decompress_run_length_stream`: Iterates through a source byte stream to identify repeating sequences and updates a destination table by adding values to existing entries.
- `process_decompression_stream`: Iterates through a byte stream to identify repeating sequences and updates a destination table by adding values to existing entries.
- `accumulate_weighted_sum_from_table`: Copies a series of 16-bit values from a source buffer into a local array, then performs a nested loop to calculate a running sum by calling another routine with each value and an offset.
- `accumulate_decompressed_values_2`: Copies values from a source buffer to a destination table and returns a running sum of the processed data.
- `accumulate_routine_results`: Iterates through a table, calling another routine for each entry and accumulating the returned values into a sum.
- `accumulate_decompressed_values`: Iterates through a source byte stream to identify repeating sequences, updates a destination table, and returns a running sum of the processed values.
- `copy_memory_with_initial_byte`: Copies a block of memory from a source to a destination, while first setting the destination's initial byte to a value derived from the high byte of a third parameter minus one.
- `copy_memory_block_8`: Copies a sequence of bytes from a source location to a destination location using a counter.
- `store_encoded_values`: Stores two calculated values into a destination buffer and returns a pointer to the memory following the buffer.
- `compress_run_length_stream`: Iterates through a source byte stream to identify repeating sequences.
- `compress_data_stream`: Processes a stream of bytes to perform a run-length encoding style compression into a buffer.
- `decompress_byte_stream`: Analyzes a byte stream using conditional logic and recursion to determine a compressed size and reconstruct data.
- `calculate_compressed_size_3`: Analyzes a byte stream using compression logic to determine the difference between two pointers representing the new size.
- `calculate_compressed_size_2`: Analyzes a byte stream using compression logic to determine the difference between two pointers.
- `calculate_compressed_size`: Analyzes a byte stream using compression logic and returns the difference between two pointers representing the new size.
- `detect_and_encode_run_length`: Iterates through a source buffer to identify sequences of identical bytes and writes the results into a destination buffer.
- `encode_run_length_data`: Analyzes a byte stream to identify repeating sequences and encodes them into a compressed format.
- `calculate_size_difference`: Returns the difference between two values after calling another routine with a pointer offset by a constant.
- `calculate_register_difference`: Computes the difference between a value returned by another routine and the contents of a register.
- `calculate_size_difference_2`: Computes the difference between a value returned by another routine and a provided value.
- `return_zero_19`: This is an empty routine that returns zero to the caller.
- `return_zero_20`: This routine performs no operations and returns zero to its caller.
- `return_zero_stub_5`: The routine takes no arguments and returns the constant 0 immediately, performing no other work.
- `update_minimum_counters`: Copies short values into longword locations, calls another routine twice to update specific counters, and then ensures two global variables maintain minimum threshold values.
- `recursively_clip_coordinates`: Compares a set of input coordinates against various global boundary limits and recursively calls itself with adjusted values to ensure they fall within specified ranges.
- `check_graphics_bounds`: Performs conditional checks on global variables and stack values to determine if a routine should proceed.
- `clip_and_dispatch_graphics`: Checks if a rectangular area is within bounds and calculates intersection dimensions before calling either a specific routine or a sequence of function pointers.
- `calculate_clipping_rect`: Calculates the intersection between a fixed-size bounding box and a target area to determine valid drawing dimensions.
- `update_blt_context_2`: Calculates a range based on bounds and updates a data structure, potentially triggering a function call to handle graphics or memory operations.
- `update_blt_context`: Calculates a value based on a comparison between a global variable and a threshold, then updates a data structure using values from a table and function calls.
- `update_blt_context_structure`: Updates a data structure with values from registers and memory, including a calculated offset from a table.
- `setup_graphics_parameters`: Initializes bitmapped graphics parameters and populates a structure with values from various memory locations and tables.
- `call_blt_routine`: Calls a routine using a pointer retrieved from an offset of a value stored at a specific memory location.
- `call_indirect_pointer_13`: Checks if a parameter is non-zero and then calls a function through a pointer located at an offset from a value stored in a global variable.
- `call_indirect_function_38`: Calls a function through a pointer located at an offset from a value stored in a global variable.
- `execute_graphics_calls`: Calls a routine multiple times with a specific constant and a base address offset.
- `update_accumulator_with_clamping`: Calculates a scaled delta based on a control value and adds it to an accumulator.
- `update_graphics_state_5`: Adjusts a value in a table using a scaled constant and performs conditional function calls to update graphics or state based on the result.
- `update_graphics_state_4`: Updates a value by adding a scaled constant to it and performs conditional bitmapped graphics operations based on the resulting value and several state checks.
- `update_sprite_display_state`: Checks a signed short value to determine if a graphical element should be reset or updated via bitmapped blitting.
- `manage_bitmap_operations`: Performs conditional bitmapped graphics operations and calls functions through pointers based on specific state checks.
- `call_indirect_and_set_flag`: Calls a function through a pointer located at an offset from a global variable and then stores a value of one into another global variable.
- `blit_linked_list_elements`: Iterates through a linked list and calls a routine to perform bitplane graphics transfers for each element using calculated coordinates and dimensions.
- `return_no_op_stub_8`: The routine consists of a single return instruction and performs no work.
- `dispatch_nested_data`: Calls another routine using two values retrieved from a global table.
- `iterate_nested_data_structures`: Executes a nested loop structure that iterates through a range of indices defined by global variables and calls another routine for each combination.
- `loop_routine_calls`: Executes a nested loop that calls another routine based on values retrieved from a table.
- `execute_nested_loop`: Executes a nested loop structure that calls another routine while incrementing counters based on values stored in a global table.
- `dispatch_conditional_tile_effects`: Calculates screen coordinates and offsets based on tile data and bitwise flags to conditionally call a routine from a function table.
- `process_conditional_tile_data`: Evaluates bitwise flags and range checks on a data structure to conditionally execute routine calls from a table.
- `process_conditional_pixels`: Iterates through a data range to perform conditional function calls based on bitwise checks and values retrieved from a table and memory.
- `process_complex_data_structure`: Iterates through data to perform conditional calls to a routine, involving bitwise operations on a value to determine loop bounds and branching logic.
- `process_conditional_pixel_data`: Iterates through data to perform conditional calls to a routine based on bitwise checks and table lookups.
- `process_pixel_data`: Retrieves a value from a table and conditionally calls another routine based on a calculated byte value.
- `process_conditional_pixel`: Checks bit flags and counter thresholds before retrieving a value from a table and conditionally executing a routine.
- `execute_partial_data_processing`: Calls a routine with a value from a table and then performs a conditional check on a byte retrieved from a memory location to call another routine.
- `update_bounding_box`: Copies a value from one memory location to another and then calls another routine.
- `calculate_sprite_bounding_box`: Checks if a given coordinate pair falls within a defined rectangular boundary and, if so, recursively calculates and updates spatial dimensions.
- `update_sprite_frame_indices`: Iterates through a table of frame indices and offsets to update sprite animation states.
- `dispatch_sprite_frames`: Calculates and executes multiple sprite frame updates by iterating through a table of frame indices and offsets.
- `dispatch_composite_sprite_frames`: Calculates and executes multiple sprite frame updates by iterating through a table of frame indices and offsets.
- `dispatch_sprite_animation_frames`: Calculates an index based on sprite data and executes a series of function pointers from a table to handle frame updates.
- `process_data_with_conditional_calls`: Performs a loop to call a routine with data from a table, followed by conditional logic based on bitwise checks and specific value ranges.
- `execute_callback_loop`: Iterates through a table to execute function pointers and performs conditional logic based on bitwise checks and value ranges.
- `dispatch_table_callback`: Executes a function pointer from a table and then performs conditional calls based on a value retrieved from a calculated memory location.
- `execute_conditional_callback_6`: Performs conditional function calls based on bitwise checks and then executes a callback if a value from memory falls within a specific range.
- `dispatch_table_callback_with_range_check`: Executes a function pointer retrieved from a table and then performs conditional logic based on a byte value retrieved from a calculated memory offset.
- `dispatch_conditional_callback_3`: Checks a bitmask and specific value ranges within a register to select and call a function from a table.
- `execute_table_callback`: Calls a routine using a value from a table and then performs conditional logic based on a byte retrieved from memory.
- `search_value_recursively`: Performs a recursive search through a data structure by decrementing a value and comparing it against a value stored at an offset from a pointer.
- `update_byte_bits`: Combines the lower three bits of a byte with the upper five bits of a byte stored in a table at a calculated offset.
- `set_high_bit_2`: Sets the highest bit of a byte located in a table at a calculated offset.
- `clear_high_bit`: Clears the highest bit of a byte located in a table at a calculated offset.
- `execute_graphics_sequence`: Calls a series of four function pointers retrieved from a table.
- `execute_extended_graphics_sequence`: Calls a series of seven function pointers retrieved from a table.
- `call_bitmapped_blt`: Calls a function pointer to perform a bitmapped blit operation using two provided parameters.
- `copy_compressed_string`: Copies data from a source to a destination, expanding run-length encoded sequences where a byte is repeated based on a following count.
- `execute_indirect_calls_6`: Retrieves a pointer from a data structure and executes it, repeating this process for several consecutive pointers stored in memory.
- `remap_buffer_via_lookup_table`: Iterates through a source buffer and uses each word's value as an index into a lookup table to write the resulting values into a destination buffer.
- `map_data_via_lookup`: Iterates through a buffer and uses the values found within to perform lookups in a table, storing the results back into a destination memory area.
- `map_buffer_via_lookup`: Iterates through a buffer and uses the values found within to perform lookups in a table, storing the results back into a destination memory.
- `draw_blt_sprites`: Calculates source and destination coordinates for multiple bitplane graphics transfers.
- `set_color_register`: Stores a value into a global variable and writes a constant to the first color register.
- `perform_file_operation`: Uses file system calls to read or seek data and manages a pointer to a structure based on the result.
- `get_global_error`: Returns the value stored in a specific global variable.
- `load_resource_table`: Attempts to open a file and reads a structured table into memory.
- `free_nested_structures`: Iterates through a multi-level data structure to call a cleanup routine on various pointers and then frees the main structure.
- `process_callback_queue_2`: Iterates through a nested table structure to find entries where a specific byte is zero and a pointer is non-zero, then executes a function via a pointer and clears that pointer.
- `iterate_and_execute_callbacks`: Iterates through a nested table structure and executes a function pointer for each entry that meets specific byte and integer criteria.
- `process_callback_queue`: Iterates through a nested table structure to find entries where a specific byte is zero and a pointer is non-zero.
- `search_entry_list_3`: Searches a list for a matching identifier and then iterates through a secondary list of eight-byte entries to find a specific value.
- `search_entry_list`: Searches a table for a matching identifier and then iterates through a secondary list of eight-byte entries to find a specific word.
- `search_for_short_match`: Searches through a list of 8-byte entries to find one where the first word matches a provided value, returning the pointer to that entry or a constant if not found.
- `search_eight_byte_entries`: Iterates through a list of eight-byte entries to find one where the first word matches a provided value.
- `search_entry_list_2`: Searches through a list of entries to find a match for a specific identifier and then iterates through a secondary list of eight-byte entries to find a matching word.
- `search_nested_list_entry`: Iterates through a primary list to find a matching entry, then searches a nested list pointed to by that entry for a specific value.
- `increment_list_entry`: Searches for a matching entry in a list and, if found, increments a byte within that entry and returns a pointer to a related memory location.
- `increment_and_return_pointer`: Searches a list of eight-byte entries for a match.
- `return_zero_25`: This is an empty routine that returns a value of zero to its caller.
- `increment_entry_counter`: Searches a list of eight-byte entries for a match, increments a byte at a specific offset within the found entry, and returns a pointer to a data field.
- `search_and_increment_nested_list_entry`: Iterates through a primary list to find a matching entry, then performs a secondary search through a nested list pointed to by that entry.
- `decrement_and_check_counter`: Decrements a byte at a specific offset within a structure and, if it reaches zero, calls a routine to clear a pointer.
- `find_table_pointer`: Iterates through a table of entries to find a match for a specific value and returns a pointer from a secondary list.
- `find_matching_data`: Searches a table for a matching identifier and then iterates through a secondary list of eight-byte entries to find a specific word, returning a long value from the found entry.
- `search_for_long_match`: Searches through a list of 8-byte entries to find one where the first word matches a provided value, returning the long integer stored at an offset within that entry.
- `find_long_match`: Iterates through a list of eight-byte entries to find a match for a provided word.
- `search_table_entry`: Iterates through a table of entries to find a match for a specific value and then searches a secondary list of eight-byte entries to return a long integer.
- `execute_table_operation_3`: Searches a table for a matching ID, iterates through a linked list to find a value, and executes a function via a pointer.
- `execute_table_operation`: Locates an entry in a table, searches a linked list of data for a matching value, and then calls a function via a pointer based on the result.
- `execute_file_operation`: Searches a list of entries for a match and then calls a function via a pointer to perform a file operation.
- `store_library_result`: Calls a function via a pointer in a table and stores the returned value into a global variable.
- `execute_file_operation_2`: Searches a list for a matching identifier and calls a function via a pointer; if the result is negative, it calls another function to update a global value.
- `execute_table_operation_2`: Locates an entry in a table, searches a linked list for a matching value, and calls a function via a pointer.
- `set_error_status`: Stores a specific constant value into a global variable and returns zero.
- `recursively_validate_and_update_entry`: A recursive routine that searches for an entry and, upon finding it, conditionally executes a function pointer and performs a secondary search to validate or update a value via another function call.
- `update_table_value_4`: Searches a table for a matching ID and updates a global variable with a value retrieved from a function pointer, or clears it if a condition is met.
- `clear_global_variable`: Sets a specific global variable to zero.
- `return_zero_stub_15`: The routine takes no arguments and returns a constant zero with no other side effects, so it acts as a stub or placeholder.
- `manage_resource_structure`: Retrieves a pointer from a function call and performs several conditional checks and function calls through a table.
- `release_resources`: Checks if a pointer is non-zero and calls two different functions via a table before calling a third function.
- `synchronize_buffer_data`: Iterates through a linked list of data structures to compare identifiers and synchronize memory contents.
- `synchronize_file_buffer_data_2`: Iterates through a linked list of data structures to compare identifiers and synchronize memory contents.
- `synchronize_file_buffer_data`: Iterates through a linked list of data structures to compare identifiers and synchronize memory contents.
- `sync_file_buffer_data`: Iterates through a linked list of data structures to compare identifiers and synchronize memory contents.
- `process_file_data`: Reads data from a file into a memory buffer and performs conditional file operations based on the results.
- `perform_file_operations`: Performs a sequence of file-related operations including seeking and writing, and returns a status flag based on the results.
- `update_structure_counters_2`: Updates fields in a structure and increments counters based on comparisons within a linked list.
- `update_structure_counters`: Updates several fields in a structure and increments counters based on a comparison within a linked list.
- `update_linked_list_counters`: Iterates through a linked list to compare values at specific offsets and updates several counters in a structure based on the results.
- `traverse_linked_list_and_update_counters`: Traverses a linked list comparing values at specific offsets, then updates several global counters and a pointer.
- `traverse_and_increment_counters`: Iterates through a linked list while comparing values at specific offsets between list elements and a reference structure.
- `validate_directory_and_traverse_list`: Checks for the existence of specific files within a directory and, if successful, iterates through a linked list or table to recursively call itself on elements until a non-zero value is encountered.
- `process_recursive_grid_updates`: Searches a grid for a value and performs a series of conditional updates to specific offsets within a memory structure.
- `validate_resource_buffers`: Performs a sequence of conditional checks by calling a routine with various size parameters and offsets retrieved from a global structure.
- `validate_and_load_resource_buffers`: Performs a sequence of conditional checks by calling a routine with various size parameters and offsets retrieved from a global structure.
- `call_library_function_8`: Recursively calls itself if a condition is met and then calls a function through a pointer retrieved from a global structure.
- `recursively_distribute_resource_data`: This routine recursively traverses a data structure to distribute a block of data into multiple disjointed memory locations defined in a table.
- `dispatch_conditional_function`: Compares a value from a structure offset against a constant and calls one of two different functions retrieved from a table.
- `call_indirect_function_54`: Calls a function through a pointer retrieved from a structure offset.
- `dispatch_icon_function`: Checks a value in a structure and calls one of two different functions based on the comparison result.
- `call_library_function_5`: Calls a function through a pointer retrieved from a table.
- `allocate_copy_buffer`: Allocates a memory block, copies data from a source, and manages buffer status flags.
- `allocate_and_copy_data`: Allocates a memory block, copies data from a source to the new location, and calls another routine with the resulting pointer.
- `copy_and_dispatch_icon`: Retrieves a pointer from a routine, copies a block of data to a new location, and calls a function using that pointer.
- `populate_data_structure`: Copies a large sequence of values from various memory locations into a single destination structure.
- `copy_structured_data_block`: This routine performs a large-scale, multi-stage copy of data from a source buffer into various disjointed locations within a global data structure.
- `copy_structured_data`: Calls a function to retrieve data and then performs a loop to distribute the resulting bytes into several separate tables.
- `set_graphics_attributes`: Calls two graphics library functions via a pointer in a global structure.
- `execute_graphics_attribute_call`: Calls a function through a pointer located at an offset within a structure.
- `draw_graphics_elements`: Calls three graphics library functions to perform drawing operations.
- `draw_graphics_with_parameter`: Calls another routine while passing a value derived from a pointer offset and a parameter.
- `draw_multiple_graphics_elements`: Calls five graphics library functions to perform drawing operations.
- `draw_graphics_single_parameter`: Calls another routine while passing a value derived from a pointer offset.
- `execute_graphics_function_sequence`: Retrieves three function pointers from a table located via a global pointer and executes them in succession.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `test_signed_integer_2`: Returns a value of all ones if the input is negative, zero if it is less than one, or one otherwise.
- `test_signed_integer`: Checks if a signed short value is greater than zero and returns the result.
- `return_zero_26`: The routine performs no operations and returns a value of zero.
- `calculate_coordinate_offset`: Calculates differences between two coordinate sets and uses a loop to adjust values based on bounds checking and absolute differences.
- `validate_coordinate_bounds_7`: Checks if coordinate values fall within specific bounds defined by a table using a loop.
- `validate_coordinate_bounds_5`: Checks if coordinate values fall within specific ranges using a loop and conditional logic.
- `validate_coordinate_bounds_3`: Uses nested loops to check if calculated coordinate values fall within specific ranges defined by a global variable.
- `check_coordinate_validity_2`: Performs a loop-based check to determine if coordinate values satisfy specific boundary conditions.
- `validate_coordinate_bounds`: Uses nested loops to increment a counter and check if calculated coordinate values fall within specific ranges defined by a global variable.
- `check_coordinate_bounds`: Performs nested loops to check if calculated coordinate values fall within specific bounds.
- `validate_coordinate_range`: Uses a loop to determine if a set of coordinates satisfies certain boundary conditions.
- `validate_coordinate_range_2`: Performs nested loops to check if coordinate values fall within specific bounds defined in a table.
- `draw_graphics_elements_2`: Calls three graphics library functions to perform drawing operations and returns zero.
- `execute_graphics_function_sequence_2`: Retrieves three function pointers from a table via a global pointer and executes them in succession.
- `execute_graphics_library_calls`: Fetches three function pointers from a table located via a base pointer and executes them in sequence.
- `move_graphics_elements`: Calls two graphics library functions to perform drawing or movement operations.
- `execute_graphics_library_calls_2`: Retrieves a base address from a data structure and executes a sequence of function pointers located at specific offsets within that structure.
- `execute_graphics_sequence_5`: Calls several functions retrieved from a structure, including two calls with a modified parameter derived from an input.
- `execute_conditional_graphics_sequence`: Calls a series of functions through pointers in a structure, with some calls being conditional based on a parameter.
- `execute_graphics_sequence_2`: Calls a series of graphics functions retrieved from a table via an offset from a pointer.
- `perform_rect_operations`: Calls two functions from a table to perform rectangle filling and pen setting.
- `call_other_routine_3`: Calls another routine using a value derived from a register and a constant.
- `perform_recursive_drawing`: Executes a sequence of function calls retrieved from a structure and then calls itself recursively if a local variable is not equal to negative one.
- `execute_recursive_graphics_2`: Calls a sequence of functions from a structure and then calls itself three times recursively.
- `execute_recursive_graphics`: Calls a sequence of functions from a structure and then calls itself recursively.
- `draw_with_concatenated_parameter`: Calls two functions and then conditionally calls a third function using a parameter formed by concatenating a modified byte with another value.
- `draw_graphics_elements_3`: Performs multiple drawing operations and rectangle manipulations based on input parameters.
- `call_drawing_routine`: Calls another routine using an address calculated by adding an offset to a value in a register.
- `execute_extended_drawing`: Performs a long sequence of function calls retrieved from a structure and concludes by calling another routine three times.
- `execute_triple_call_2`: Calls another routine three times in succession.
- `call_graphics_with_offset`: Calls two functions and then calls a third function using a parameter that is an offset added to an input value.
- `render_formatted_text`: Calculates text layout parameters and calls a drawing routine to render formatted text based on a structure's properties.
- `render_recursive_text`: Recursively processes a structure to set parameters and call a drawing routine for formatted text.
- `draw_formatted_text`: Sets parameters for a drawing routine and calls a function to render text at a specific location.
- `render_element_with_attributes`: Determines a render mode based on a byte value from a table and several global flags, then stores calculated offsets and attributes into a local structure before calling a drawing routine.
- `draw_element_zero_mode`: Sets specific layout parameters with a zeroed attribute and calls a drawing routine to render an element.
- `determine_element_render_mode`: Selects a render mode based on a comparison between a byte and a value stored in a memory offset.
- `draw_element_nine_mode`: Sets specific layout parameters with an attribute value of nine and calls a drawing routine to render an element.
- `draw_element_with_attributes`: Sets layout parameters and attributes based on conditional checks before calling a drawing routine for an element.
- `draw_element_recursive`: Recursively processes a structure to set layout parameters and calls a drawing routine for an element.
- `draw_element_conditional_mode`: Sets layout parameters with an attribute value determined by a conditional check and calls a drawing routine to render an element.
- `draw_recursive_elements`: Performs a recursive sequence of drawing operations by retrieving function pointers from a structure and updating local values.
- `process_recursive_drawing`: Performs a recursive sequence of drawing operations by retrieving function pointers from a structure and updating several local values in a table.
- `repeat_routine_twenty_five_times`: Executes another routine in a loop exactly twenty-five times.
- `loop_other_routine`: Executes another routine within a loop that runs twenty-five times.
- `execute_repeated_routine_2`: Executes a specific routine twenty-five times in a loop.
- `execute_repeated_calls_6`: Executes a specific routine twenty-five times within a loop and then executes it one final time.
- `repeat_routine_calls_3`: Executes another routine twenty-six times in total through a loop and a final call.
- `execute_routine_loop`: Executes another routine twenty-five times in a loop and then calls it one final time.
- `loop_other_routine_plus_one`: Executes another routine twenty-five times in a loop and then calls it one additional time.
- `repeat_routine_calls_2`: Calls another routine in a loop for a fixed number of iterations and then calls it one final time.
- `repeat_routine_calls`: Calls another routine repeatedly in a loop for a fixed number of iterations before calling it one final time.
- `execute_repeated_calls_3`: Calls another routine repeatedly in a loop for a fixed number of iterations before calling it one final time.
- `execute_repeated_calls_5`: Executes another routine twenty-six times using a loop and then calls it one final time.
- `render_text_grid_a_2`: Sets graphics parameters and uses a loop to call another routine three times with calculated offsets and data from a table.
- `render_text_grid_a`: Calls another routine three times within a loop, using calculated offsets based on a counter to pass specific coordinates.
- `render_text_grid_b_2`: Sets graphics parameters and uses a loop to call another routine three times using a different set of memory offsets for the data.
- `render_text_grid_b`: Calls another routine three times within a loop, using a different set of calculated coordinate offsets for each iteration.
- `format_car_count_string`: Constructs a formatted string containing a numeric value and a text suffix by populating multiple local buffers through repeated calls to a string formatting routine.
- `draw_formatted_string_set`: Populates a buffer with characters based on an offset and calls drawing routines in loops to render text and symbols.
- `draw_text_loops`: Executes two separate loops to call drawing routines with incrementing offsets and calls a function from a table.
- `draw_text_loop_2`: Calls another routine repeatedly with an incrementing offset calculated from a loop counter.
- `render_conditional_text_data`: Selects a data pointer from a table based on a calculated index and calls a rendering routine, followed by conditional calls to other routines based on a value in a global variable.
- `render_conditional_string_data`: Calculates a string length or index based on structure offsets and conditional flags, then passes a pointer to a data table to a rendering routine.
- `render_conditional_text_2`: Renders text using various drawing routines based on a value checked in a structure.
- `render_conditional_text`: Renders text using various drawing routines based on a value checked in a structure.
- `draw_conditional_text_with_pen`: Selects a drawing routine and calculates a pen value based on a table to render text at a specific location.
- `draw_conditional_text`: Calculates a multiplier based on values in a table and calls a drawing routine with a specific pen and offset.
- `render_conditional_text_5`: Checks if a value falls within a specific range and a limit, then performs conditional updates to global variables and calls other routines based on table data.
- `render_conditional_text_4`: Checks a value against a range and a limit, then renders formatted text if a specific byte in a table is non-zero.
- `render_conditional_text_3`: Checks a value from a routine against a limit and renders formatted text if a specific byte in a table is non-zero.
- `render_formatted_text_block`: Clears specific bytes in memory, calls a routine to render formatted text, and updates several global variables before calling other routines.
- `update_text_selection`: Calculates a new index using a divisor and updates several global state variables and pointers if specific conditions regarding a table and a value are met.
- `update_text_selection_index_2`: Calculates a new index based on a value and a divisor, then updates several global state variables and pointers if specific conditions are met.
- `update_text_selection_index`: Calculates a new index based on a value and a divisor, then updates several global state variables and pointers if specific conditions are met.
- `draw_text_with_offsets`: Calls a routine to draw text using data retrieved from a table via an index, then clears two bytes and calls other routines.
- `process_text_grid_2`: Validates a range for a value and checks a table for a non-zero byte to determine if text processing should occur.
- `process_text_grid`: Validates a range for a value and checks a table for a non-zero byte.
- `update_text_grid_state`: Checks a value from a routine against a limit, calculates an index, and if a byte in a table is non-zero, updates several global variables and calls other routines.
- `process_conditional_text_element`: Validates a range of input parameters and calls a routine to check a value, then conditionally updates global variables and calls various routines based on table data and flag comparisons.
- `process_conditional_element_2`: Evaluates a range and several flags to conditionally call routines and update multiple global variables with values derived from a table index.
- `process_conditional_element`: Calls a routine via a pointer and performs conditional logic based on the return value and various global flags.
- `iterate_graphics_stride`: Calls a routine with a specific parameter and then enters a loop that calls another routine with an incrementing parameter.
- `iterate_with_stride`: Calls another routine in a loop, incrementing the parameter by a fixed amount each iteration until a limit is reached.
- `format_numeric_string_2`: Formats a numeric value into a string by selecting different conversion sequences based on the magnitude of the input.
- `format_numeric_string_by_magnitude`: Compares a value against two thresholds to select a specific sequence of function calls that process and store a formatted string into a global variable.
- `format_and_store_string`: Formats a string using a format string and stores the resulting pointer into a global variable.
- `format_string_data`: Uses a format string to process values through several function calls and returns a pointer to a global variable.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `modify_memory_values_2`: Stores a global variable into a memory location and performs a bitwise OR operation on a byte at another location.
- `check_string_delimiters`: Checks if a string contains a space or a hyphen character at a specific position.
- `scan_string_for_characters`: Iterates through a memory buffer to check for space or hyphen characters while calling another routine.
- `scan_string_for_delimiters`: Iterates through a memory buffer checking for space or hyphen characters while calling another routine.
- `draw_text_with_offset_3`: Uses a pointer and an offset to draw text while performing several graphics operations.
- `draw_text_with_offset_2`: Uses a pointer and an offset to draw text and calls several other routines.
- `draw_text_with_offset`: Calls graphics functions to draw text using a value retrieved from a pointer plus an offset.
- `draw_text_with_pen`: Sets a graphics pen and draws text at a specific location if certain conditions in a structure are met.
- `update_pen_and_draw_text`: Sets a graphics pen color and conditionally draws text using a pointer-based value and an offset.
- `draw_text_with_pointer_2`: Draws text using a pointer and calls several other routines.
- `draw_text_with_pointer`: Calls graphics functions to draw text using a value retrieved from a pointer directly.
- `draw_masked_bitmaps`: Retrieves several values from a table and calls a graphics routine to perform a bitmask blit operation.
- `dispatch_indirect_function_pointer`: Fetches a pointer from a global structure and uses it to call a function located at a specific offset within a secondary table.
- `execute_blt_mask_2`: Calls a routine from a table to perform a bitmapped mask operation.
- `execute_blt_mask`: Calls a single function from a table to perform a bitmapped mask operation.
- `setup_drawing_state`: Calls multiple functions from a table to configure pens, drawing modes, and templates.
- `draw_patterned_graphics`: Calls routines with varying parameters using two separate loops and modulo arithmetic to determine offsets.
- `process_data_in_loops`: Performs a loop with increments of six and a subsequent loop with increments of three to call another routine.
- `execute_nested_loops`: Performs two separate loops that call another routine with varying parameters based on modulo arithmetic.
- `iterate_with_variable_step`: Iterates through a loop, calling another routine with a step value that changes based on a modulo operation of a counter.
- `execute_nested_step_loop`: Performs a nested loop that calls another routine with a varying parameter.
- `execute_recursive_calls`: Calls another routine twice in succession.
- `execute_conditional_calls_3`: Calls another routine multiple times, with additional calls performed if a specific bit in a parameter is set.
- `execute_modulo_loops`: Iterates through a range in steps of six, calling different routines based on the current iteration count modulo five and ten.
- `loop_with_modulo_calls`: Iterates through a loop and calls another routine with different parameters depending on whether a counter is divisible by five or ten.
- `loop_with_conditional_calls`: Iterates through a loop and calls either another routine with a parameter or without one based on modulo checks.
- `execute_nested_modulo_loops`: Performs nested loops that call another routine with different parameters based on whether a counter is divisible by five or ten.
- `iterate_and_call`: Iterates through a memory range in increments of six and calls another routine with an incremented value.
- `increment_and_call_routine`: Calls another routine with an incremented value while iterating through a memory range.
- `copy_and_accumulate_values`: Copies ten words from one memory location to another and then adds values returned by a function to entries in a table.
- `accumulate_table_values`: Iterates through a loop to add values retrieved from a function to entries in a table.
- `copy_and_accumulate_with_offset`: Copies ten words from one memory location to another and then adds a value returned by a function plus a constant offset to entries in a table.
- `accumulate_values_with_offset`: Iterates through a loop to add a value from a function and a stack value to entries in a table.
- `draw_table_values`: Iterates through a table of values and passes each value to another routine for processing.
- `draw_parameterized_graphics`: Iterates through a table of values and passes each value to another routine for drawing.
- `execute_recursive_graphics_4`: A routine that performs recursive graphics operations by calling a sequence of functions retrieved from a structure.
- `execute_recursive_graphics_3`: Iterates through a table to call routines recursively and performs additional function calls if a specific byte value is met.
- `execute_recursive_graphics_loop`: Iterates through a list of function pointers and performs multiple recursive calls based on a specific byte value.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `check_range_bounds`: Performs multiple comparisons to determine if a value falls within specific bounds defined by a table and returns a boolean result.
- `return_zero_and_restore_registers_2`: Sets register d0 to zero, restores d2, d3, a2, and a6 from the stack, and returns.
- `render_text_with_length`: Calculates text length and performs several graphics operations including setting pens and draw modes.
- `draw_text_graphics`: performs several graphics operations including setting a pen, setting a draw mode, and rendering text.
- `return_immediately_stub_4`: The routine consists of a single return instruction and performs no work.
- `search_table_for_pointer`: Iterates through a table of longwords to find a match for a provided value and returns the subsequent longword as a function pointer.
- `modify_memory_values`: negates a byte at a memory location, increments a pointer, and stores a bitwise ORed byte into a table.
- `update_state_logic`: Evaluates a value from a function call and updates multiple global variables and status flags based on several conditional checks.
- `update_status_values`: Checks a global variable against two specific values to set it to a new constant, then sets a different global word to one.
- `set_fixed_values`: Sets a longword to zero and a word to one at specific memory locations.
- `return_zero_27`: The routine is empty and returns zero.
- `validate_and_toggle_range_entry`: Iterates through a table of range definitions to check if two input values fall within specified bounds and if a corresponding status flag is zero.
- `update_conditional_table_2`: Checks if a value falls within ranges defined in a table and, if so, calls two routines and updates a status table.
- `update_conditional_table`: Calls two routines and conditionally clears a word in a table based on a comparison.
- `execute_conditional_logic_7`: Executes two routines via pointers and conditionally clears a word in a table based on whether a register is zero.
- `execute_conditional_logic_5`: Calls two different routines and conditionally clears a word in a table based on whether a register is zero.
- `execute_conditional_logic_4`: Calls two different routines and then clears a word in a table based on whether a register is zero.
- `conditionally_clear_word`: Calls another routine and then stores a zero into a word location, where the specific destination depends on whether a register is zero.
- `clear_pointer_value`: stores a zero into a memory location pointed to by a register.
- `iterate_conditional_table_entries`: Iterates through a table of range definitions to check if a value falls within specified bounds.
- `update_summed_values`: Calculates a sum from multiple values and updates a global table.
- `update_state_values_2`: Calculates a sum from multiple values and updates a global table, triggering routines if the new sum differs from the stored value.
- `update_state_values`: Updates several memory locations with calculated values and executes routines if a specific calculated sum differs from a stored value.
- `update_state_conditionally`: Stores a word to a memory location, executes a routine via a pointer, and conditionally executes another routine if one value is less than another.
- `execute_indirect_function_58`: Fetches a function pointer from an offset within a structure and executes it.
- `call_indirect_routine`: Calls a routine through a pointer stored at a specific offset from a base address.
- `calculate_difference_values`: Computes a difference between values and updates a global table.
- `calculate_difference_update`: Computes a difference between values, updates a table, and executes routines if the resulting calculated value has changed.
- `calculate_difference_and_update`: Computes a value based on the difference between two numbers and updates a table, triggering routines if the result changes.
- `copy_and_call_indirect`: Copies a word from one location to another, executes a routine via a pointer, and conditionally executes a second routine based on a comparison.
- `call_indirect_via_offset_9`: Calculates an address by adding a negative constant to a value in a register and executes the function located at that resulting pointer.
- `execute_indirect_function_55`: Calculates an offset from a value in a register to locate a pointer within a table, then calls the function pointed to by that address.
- `update_calculated_table_values`: Calculates two new values based on input parameters and conditional function calls, then stores them in a table.
- `process_conditional_data_2`: Retrieves a value through a pointer if a condition is met, performs arithmetic, and updates a table with the results.
- `calculate_and_store_values`: Performs arithmetic on several values, stores the results in a table, and conditionally executes routines if a calculated value differs from a stored one.
- `execute_conditional_calls_2`: Stores a word to a global location, calls a routine via a pointer, and then conditionally calls another routine based on a comparison.
- `execute_indirect_function_56`: Calculates an offset from a value in a register to locate a pointer in memory, then calls the function pointed to by that address.
- `validate_entity_bounds_and_update_state`: Checks if an entity's coordinates fall within specific boundary limits and predefined rectangular zones.
- `update_entity_collision_state_3`: Checks if an entity's bounding box overlaps with specific rectangular boundaries or predefined area tables.
- `update_collision_counter_and_bounds`: Checks for spatial overlaps between an entity's bounding box and a set of predefined rectangular areas.
- `process_collision_and_toggle_states_2`: Checks for spatial overlaps between a target entity and a set of bounding boxes within a table.
- `update_entity_collision_state_2`: Checks for spatial overlaps between a target entity and a table of bounding boxes.
- `update_collision_counter_and_state`: Checks for spatial overlaps between a target entity and a table of bounding boxes.
- `update_collision_state_and_animation`: Iterates through a bounding box table to detect spatial overlaps with a target entity.
- `process_collision_animation_cycle`: Iterates through a table of object boundary data to detect spatial overlaps with a coordinate set.
- `update_collision_animation_state_3`: Iterates through a table of object boundary data to check for spatial overlaps with a specific coordinate set.
- `update_collision_animation_state_4`: Iterates through a table of object boundary data to check for spatial overlaps with a specific coordinate set.
- `update_collision_animation_state_2`: Iterates through a table of object boundary data to check for spatial overlaps with a specific coordinate set.
- `check_object_collision_bounds_2`: Iterates through a table of data structures to perform boundary checks against a set of coordinates.
- `update_collision_animation_state`: Increments or decrements a counter in a global variable and calls a routine to update an animation frame.
- `check_object_collision_bounds`: Iterates through a table of data structures to perform boundary checks against a set of coordinates.
- `process_collision_and_ui_updates`: Iterates through a list of entities to check for spatial overlaps within defined boundaries.
- `iterate_and_update_object_bounds`: Iterates through a table of data structures to perform boundary checks against a set of coordinates.
- `process_collision_and_toggle_states`: Iterates through a table of bounding box coordinates to detect spatial overlaps with a target entity.
- `update_collision_state`: Iterates through a table of bounding boxes to check for intersections with a target area.
- `check_collision_and_trigger_callbacks`: Iterates through a table of bounding boxes to detect intersections with a target coordinate pair.
- `process_entity_collision_and_interaction`: Iterates through a table of eight bounding boxes to check if a coordinate pair falls within their boundaries, triggering specific callback routines upon collision.
- `check_entity_collision_bounds`: Tests if a coordinate pair falls within specific rectangular boundaries defined in a table.
- `update_entity_boundary_interaction`: Checks an entity's coordinate ranges against predefined boundary segments and updates a global counter.
- `update_entity_collision_state`: Checks an entity's coordinates against specific boundary ranges and updates a counter in a global table if the entity is outside bounds.
- `validate_entity_collision_bounds`: Checks if an entity's coordinates fall within specific boundary ranges defined in a table.
- `process_entity_spatial_bounds`: Iterates through a list of entities to check if their coordinate bounds fall within a specific calculated range.
- `update_entity_collision_states`: Iterates through a list of entities to check for spatial overlaps and updates their status based on coordinate comparisons.
- `update_entity_positions`: Iterates through a list of entities to calculate and update their coordinate offsets.
- `render_entity_bounding_boxes`: Iterates through a list of entities to draw rectangular bounding boxes or coordinate markers using graphics library calls.
- `check_spatial_bounds_and_update_state_3`: Iterates through a table of three spatial boundary definitions to check if a coordinate pair falls within them, triggering specific callback routines.
- `check_spatial_bounds_and_update_state_2`: Iterates through a table of three spatial boundary definitions to check if a coordinate pair falls within them, triggering specific callback routines.
- `check_spatial_bounds_and_update_state`: Iterates through a table of three spatial boundary definitions to check if a coordinate pair falls within them, triggering specific callback routines if a match is found.
- `process_spatial_collision_events`: Checks if a coordinate pair falls within specific spatial boundaries defined in a table and updates a counter if conditions are met.
- `check_spatial_bounds_intersection`: Tests if a coordinate pair falls within specific spatial boundaries defined in a table.
- `update_scroll_offset_by_bounds_2`: Checks if a coordinate pair falls within specific rectangular boundaries defined in a table.
- `check_spatial_bounds_intersection_2`: Tests if a coordinate pair falls within specific rectangular boundaries defined in a table.
- `adjust_scroll_and_render_entities`: Updates a global scroll counter by increments of nine based on entity coordinate bounds and manages spatial intersection checks.
- `process_spatial_bounds_and_scroll`: Checks entity coordinates against spatial bounds to determine if a scroll offset adjustment is required.
- `check_entity_bounds_and_scroll`: Iterates through a table of spatial bounds to check for entity intersections and adjusts a global scroll counter by increments of nine.
- `validate_entity_bounds_and_scroll`: Checks if an entity's coordinates fall within specific rectangular boundaries and updates a global scroll counter by increments of nine if certain conditions are met.
- `update_scroll_position`: Iterates through a data structure to check coordinate bounds and adjusts a global counter by increments of nine.
- `update_entity_position_and_bounds`: Updates the coordinates of an entity and performs boundary checks against a defined rectangular area.
- `update_scroll_offset_by_bounds`: Iterates through a table of spatial bounds to check for entity intersections and adjusts a global scroll counter by increments of nine.
- `resolve_spatial_collision_overlap`: Checks for intersections between an entity's coordinates and a table of rectangular bounds.

The function list continues in: g1284-recovery-functions-2.md.
