# Game 738 (ChaosStrikesBack v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00738). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6348744 top-level bytes in 103 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 41 of 41 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 507 of 507 named (0 author symbols), 507 of 507 described (47 by rule, 455 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 377470 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8618 of 8618 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1145744 |
| code | 27900 |
| data | 4186529 |
| padding | 28571 |
| unused | 960000 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetPatch | 24 | 833 | 45 |
| EndCLI | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| ENDIF | 1 | 2 | 1 |
| IF | 2 | 117 | 2 |
| Version | 11 | 574 | 18 |
| Disk-Validator | 1 | 386 | 1 |
| Disk.1:BJELoad_R | 3 | 2062 | 177 |
| SetPatch | 24 | 833 | 45 |
| EndCLI | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| ENDIF | 1 | 2 | 1 |
| IF | 2 | 117 | 2 |
| Version | 11 | 574 | 18 |
| Disk-Validator | 1 | 386 | 1 |
| Disk.2:BJELoad_R | 3 | 2062 | 177 |
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 10 | 1 |
| Disk.2:bootblock | 1 | 10 | 1 |
| Disk.1:KAOS.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:APPB.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:SWSH.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:CNFG.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:GRF1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:APPA.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:VDEO.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:ANIM.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:MEM1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.1:USIO.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:APPB.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:SWSH.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:CNFG.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:FIO1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:CEDT.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:GRF1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:APPA.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:VDEO.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:HINT.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:ANIM.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:MEM1.FTL (raw code file, load address unknown) | 1 | 0 | 0 |
| Disk.2:USIO.FTL (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:Dungeon.DAT: high-entropy data file (7.65 bits/byte), classified as packed data
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:DungeonF.DAT: high-entropy data file (7.63 bits/byte), classified as packed data
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:DungeonG.DAT: high-entropy data file (7.62 bits/byte), classified as packed data
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:CEDT.FTL: high-entropy data file (7.70 bits/byte), classified as packed data
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:HINT.FTL: high-entropy data file (7.79 bits/byte), classified as packed data
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1.hdf: hard-disk image (AmigaDOS OFS, 6848 blocks): its filesystem parses and all 16 files in it are byte-identical to the extracted tree ChaosStrikesBack_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:KAOS.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:APPB.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:SWSH.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:CNFG.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:GRF1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:APPA.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:VDEO.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:ANIM.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:MEM1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1:USIO.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:APPB.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:SWSH.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:CNFG.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:FIO1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:CEDT.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:GRF1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:APPA.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:VDEO.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:HINT.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:ANIM.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:MEM1.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.2:USIO.FTL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.1: blocks 11-21 (bytes 5632-11264) hold ASCII text
- ChaosStrikesBack_v1.1/ChaosStrikesBack_v1.1_hdf/ChaosStrikesBack/data/Disk.3: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 22 never-loaded region(s), 377470 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### SetPatch

- `initialize_setpatch_environment`: Retrieves system information and parses a command-line string into a structured format.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: Installs a SetPatch handler and performs a series of conditional patches and library initializations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
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
- `calculate_checksum_and_jump`: Iterates through a list of words to calculate a cumulative sum, stores the bitwise complement of this sum in a memory location, and then performs an indirect call.
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `call_if_not_negative_one_2`: Checks if a global variable matches a specific value or negative one, and if not, calls a routine via a pointer stored in another global variable.
- `call_if_not_negative_one`: Checks if a register contains negative one and, if not, calls a routine via a global pointer.
- `update_interrupt_status`: Updates a global variable and modifies the interrupt enable bit in the INTENA hardware register before performing an indirect call.
- `execute_indirect_call`: Performs an indirect call to an address calculated by adding an offset to a value stored in a memory location.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `return_register_value`: A single instruction routine that returns the value currently held in a register.
- `pass_input_register`: Returns the value contained in a data register without modification.
- `execute_multiple_calls`: Calls another routine fifteen times in succession and returns the value from the final call.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `check_size_and_call`: Compares a value at an offset within a global variable against a constant and calls an indirect routine if the value is small.
- `copy_and_verify_list`: Copies elements from one list to another while calling a routine for each element, then performs a cleanup loop if the process fails.
- `call_and_return_bitmask_2`: Calls an indirect routine multiple times and returns the bitwise OR of a value from a pointer and a constant bitmask.
- `call_and_return_bitmask`: Calls an indirect routine and returns the bitwise OR of a value from a pointer and a constant bitmask.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `execute_conditional_callbacks`: Calls a routine using a pointer from a structure and, if the result is non-zero, executes two additional indirect calls using other pointers from that same structure.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `calculate_relative_offset`: Calculates a new pointer by reading a 16-bit value from the input pointer and applying a bit-shifted offset.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_and_call`: Copies fifteen words from a source offset to a destination pointer and then calls another routine.

### FastMemFirst

- `initialize_fast_memory_and_parse_args_2`: Locates a fast memory block and parses the operating system command line into a linked list of null-terminated argument strings.
- `find_and_copy_fastmem_block_2`: Iterates through a linked list of memory blocks to identify the first fast-memory block with a specific type tag and the first chip-memory block with a different type tag.
- `call_table_function`: Calls a function pointer located at a specific offset within a global table.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_table_function_2`: Calls a function pointer located at a different offset within a global table.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### ENDIF

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### IF

- `build_dynamic_data_structure`: Iterates through a linked list to find a maximum value and then populates a structured table with calculated offsets and incremental values.
- `copy_structure_and_execute`: Copies sixteen bytes from a memory location offset from a pointer into a stack-based destination before calling a function pointer.

### Version

- `parse_command_line_string`: Iterates through a string to extract space-separated arguments, handling quoted substrings and converting specific character sequences into newline or escape characters.
- `dispatch_message_event`: Updates a message structure with a new value and executes conditional function calls from a table based on the contents of the message fields.
- `check_library_version_2`: Parses command-line arguments to identify a target library or device name and optional version/revision numbers.
- `dispatch_message_and_finalize`: Updates a message structure with a new value and executes conditional function calls from a table based on the message field, then calls a library function to finalize the operation.
- `parse_integer_string_2`: Converts a null-terminated string into a signed integer by processing an optional sign character followed by a sequence of numeric digits.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte.
- `get_string_length`: Iterates through a null-terminated character sequence to determine the number of bytes preceding the null terminator.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch`: Calls a function pointer located at an offset from a global variable.
- `forward_with_stack_slot`: Takes two 32-bit parameters and calls the routine itself again, passing the same two values plus a pointer to a fixed offset within the current stack frame, then returns.
- `call_library_function_2`: Calls a function from a library using an offset from a global variable.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_function_13`: Fetches a function pointer from an offset within a global variable and executes it.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_14`: Fetches a function pointer from an offset within a global structure and jumps to it.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer from a global variable and computes a rolling checksum over a stack buffer.
- `store_byte`: Stores a byte from a register into a memory location.

### Disk-Validator

- `format_display_string_2`: Converts a numeric value into a character representation and stores it in a buffer.

### Disk.1:BJELoad_R

- `clear_and_dispatch_callback`: Clears a small block of memory with zeros and then executes a conditional function call based on a bitmask check in a global structure.
- `execute_recursive_stack_operation`: Performs a recursive call to itself while passing a constant value and a stack-allocated buffer.
- `initialize_data_structure_2`: Populates a structure with several constant values and pointers, then updates a linked list or table entry based on the result of a function call.
- `update_structure_pointer`: Updates a pointer within a structure and conditionally copies data between offsets based on the results of indirect calls.
- `transfer_structure_data`: Copies data between offsets of two structures and conditionally updates a field based on the result of another routine.
- `process_argument_list`: Initializes a local structure with a constant value and performs an indirect call using the structure and a parameter.
- `execute_empty_call`: Performs an indirect call and then returns zero.
- `process_conditional_data_2`: Evaluates multiple conditions and performs a block memory copy into a structure if specific criteria are met.
- `copy_data_to_structure`: Performs conditional indirect calls and copies a block of bytes from one location to another within a structure.
- `execute_conditional_calls`: Checks two different conditions and executes corresponding routines if the values are non-zero.
- `decrement_and_branch`: Decrements a short value at an offset within a structure and, if it reaches zero and a specific bit is set, executes multiple routines using the structure pointer.
- `set_status_bit_2`: Sets a specific bit in a byte located at an offset within a provided structure.
- `traverse_nested_pointers`: Uses a local variable to store a pointer retrieved from an indirect call and conditionally executes further routines based on the result.
- `traverse_linked_list_recursively`: Recursively traverses a linked list by following pointers stored at a specific offset.
- `validate_resource_header_and_map_indices`: Verifies a specific resource header signature and then iterates through a list of resource entries to ensure exactly one instance of each required type is present.
- `validate_and_map_resource_indices_3`: Verifies a resource structure header and recursively processes its child elements to identify and store specific dependency indices.
- `validate_and_map_resource_indices`: Iterates through a resource table to identify and store the indices of specific dependency types while verifying their validity.
- `resolve_resource_dependency_indices_2`: Iterates through a resource table to identify and store indices of specific dependency types while verifying their validity.
- `validate_resource_dependency_tree_4`: Iterates through a resource table to identify and store indices of specific dependency types while verifying their validity.
- `verify_resource_dependency_tree`: Iterates through a resource table to identify and store indices of specific dependency types while verifying their validity.
- `validate_resource_dependency_tree_2`: Iterates through a resource table to identify and store indices of specific dependency types while verifying their validity.
- `verify_resource_dependency_chain_2`: Iterates through a resource table to verify specific status flags and ensure required dependencies are present.
- `process_resource_allocation_chain_2`: Iterates through a table of resource entries to calculate and allocate memory segments for a linked structure.
- `allocate_resource_buffer_segments`: Calculates and allocates memory segments for a resource structure by iterating through a table and verifying entry flags.
- `clear_linked_structure_field_2`: Checks several pointers within a structure and clears a specific nested pointer if it is non-zero.
- `validate_resource_table_entries`: Iterates through a table of entries to verify specific type flags and ensure required indices are assigned.
- `traverse_resource_dependency_tree_2`: Recursively traverses a hierarchical structure of resource dependencies to validate integrity and calculate memory offsets.
- `resolve_resource_dependency_chain_2`: Recursively validates and links a sequence of resource dependencies by checking pointer integrity and calculating memory offsets.
- `clear_linked_structure_field_alt`: Checks several pointers within a structure and clears a specific nested pointer if it is non-zero, returning a different constant value.
- `validate_resource_dependencies`: Iterates through a table of entries to verify specific requirements and flags.
- `traverse_resource_dependency_tree_3`: Recursively traverses a linked structure of resource dependencies to validate and resolve their loading order.
- `resolve_resource_dependency_chain_3`: Recursively traverses a linked structure to resolve and validate resource dependencies.
- `traverse_linked_structure_3`: Recursively traverses a linked structure by checking and following pointers stored at specific offsets within the structure.
- `validate_resource_table_entries_2`: Iterates through a table to identify and verify specific entry types based on status flags.
- `validate_resource_table_entries_3`: Iterates through a table of entries to verify specific flags and ensure required resource identifiers are present.
- `validate_resource_dependency_chain`: Iterates through a resource table to verify specific status flags and ensure required dependencies are present.
- `resolve_resource_dependency_tree_3`: Iterates through a resource table to identify specific dependency types and updates linked indices within a structure.
- `rebuild_resource_node_hierarchy_2`: Recursively traverses a nested data structure to validate child element sizes and rebuilds a linked list of pointers to these elements.
- `reconstruct_resource_node_hierarchy`: Recursively traverses a nested data structure to validate child element sizes and rebuilds a linked list of pointers to these elements.
- `rebuild_resource_tree_node`: Recursively traverses a nested data structure to validate sizes and checksums, then reconstructs a linked list of pointers to child elements.
- `flatten_resource_hierarchy_4`: Recursively traverses a nested data structure to validate its integrity via checksums and sizes.
- `flatten_resource_hierarchy_2`: Recursively validates a nested data structure by comparing checksums and sizes before copying its elements into a linear array.
- `rebuild_resource_hierarchy`: Validates a nested data structure through recursive checksum and size comparisons before flattening it into a linear array.
- `reconstruct_resource_hierarchy_2`: Validates a nested data structure by comparing checksums or size fields and then flattens the hierarchy into a linear array.
- `unpack_resource_node_hierarchy`: Recursively traverses a nested data structure to validate child elements and extract their properties.
- `parse_and_link_resource_data`: Recursively traverses a hierarchical data structure to validate and extract nested elements.
- `reconstruct_and_link_data_structure`: Copies a sequence of words from a source buffer into a destination buffer while handling zero-terminated padding and updating a linked structure.
- `unpack_and_reconstruct_data_structure`: Copies a sequence of words from a source buffer into a destination buffer while handling zero-terminated padding and updating linked structure fields.
- `unpack_and_resolve_tree_nodes`: Copies a sequence of words from a source buffer into a destination buffer while handling zero-terminated padding.
- `unpack_and_validate_data_structure`: This routine performs a nested data unpacking operation by copying sequences of words from a source buffer into a destination buffer based on length descriptors.
- `reconstruct_linked_tree_nodes`: Iteratively updates pointer offsets within a data structure before performing a recursive depth-first traversal to validate node integrity.
- `validate_linked_structure_2_2`: Recursively traverses a linked data structure to perform integrity checks and update node fields.
- `validate_linked_structure_2`: Recursively traverses a linked data structure to perform validation checks and updates various fields and sums within the nodes.
- `traverse_linked_structure_5`: Recursively traverses a linked data structure, performing validation checks and updating various fields and sums within the nodes.
- `traverse_and_update_linked_list`: Recursively traverses a linked data structure, validating node integrity by comparing values and updating field offsets and sums within the nodes.
- `validate_and_update_structure`: Compares a calculated sum against a value in a structure and, if they match, updates several fields within that structure.
- `update_linked_structure_2`: Updates several fields in a structure pointed to by a register and performs recursive calls based on values in that structure.
- `process_conditional_pointer`: Checks a value at an offset within a structure and, if non-zero, calls another routine before clearing that value.
- `initialize_pointer_list`: Sets a pointer to point to the next element in a list, clears the following element, and stores the original pointer address.
- `search_pointer_chain`: Iterates through a chain of pointers until it finds a node where a specific offset matches a provided value.
- `traverse_pointer_chain`: Iterates through a chain of pointers until a null pointer is encountered or a specific value match is found.
- `execute_indirect_call_5`: Performs an indirect call to another routine and returns zero.
- `insert_node_into_list_2`: Swaps pointers to insert one structure into another, updating a link field within the structure.
- `link_two_structures`: Sets the first element of a structure to point to another structure and updates a link field in that second structure.
- `execute_sequence_of_calls`: Executes a sequence of three routine calls, including one with a specific constant argument.
- `process_global_data_loop`: Initializes several global variables and enters a loop that repeatedly calls another routine until a counter reaches a limit.
- `call_multiple_offsets`: Calls three different routines using the provided parameter and a version of the parameter with an offset applied.
- `traverse_and_recurse_list`: Iterates through a linked list of pointers, performing recursive calls on elements that meet a specific condition.
- `verify_data_signature`: Compares specific values at offsets within a provided pointer to determine if they match a constant pattern.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `accumulate_weighted_byte_sums`: Calculates a sum by iterating through two memory buffers, multiplying each byte by its relative index and adding the product to an accumulator.
- `calculate_weighted_sum_2`: Iterates through a memory range, multiplying each byte by its relative index and adding the result to an accumulator.
- `sum_word_array`: Calculates the sum of a series of words stored in a memory buffer.
- `return_no_op`: The routine body consists only of a return instruction, so it performs no work and simply returns to its caller.
- `sum_byte_array_2`: Calculates the sum of a series of bytes stored in a memory buffer.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `update_node_pointer`: Retrieves a pointer from a structure and, if successful, updates a field in that structure with a new value.
- `update_pointer_table`: Increments a pointer in a table and updates a value at a location pointed to by that table entry.
- `execute_conditional_recursion`: Checks if a parameter is non-zero and, if so, performs a recursive call using values derived from that parameter.
- `store_result_if_nonzero`: Calls a routine with a specific constant and, if the result is non-zero, stores that result in a provided pointer.
- `store_pointer_to_table`: Stores a value from a table into a location pointed to by another table entry.
- `execute_conditional_routine`: Calls a routine, then conditionally calls another routine if a parameter is non-zero, before calling a final routine.
- `execute_routine_with_constant`: Calls a routine with two parameters and a third constant parameter, surrounded by other routine calls.
- `compare_routine_result_2`: Calls a routine with swapped parameters and returns a boolean based on whether the result matches a provided value.
- `execute_and_return_zero`: Calls another routine and returns zero.
- `process_string_and_output_2`: Calculates a length, then uses that length to call two different routines with a parameter and the length.
- `handle_conditional_input`: Calls a routine and, if the result is non-zero, performs a second call with that result as an argument before returning a value.
- `execute_indirect_call_6`: Performs an indirect call to a routine address retrieved from a table and then calls another routine.
- `evaluate_conditional_input`: Checks the return value of a routine to determine whether to execute a secondary routine with specific arguments or return a zero value.
- `execute_indirect_call_7`: Performs an indirect call to a routine address retrieved from a table.
- `enter_infinite_loop`: Performs an indirect jump to an address stored in a jump table and then enters an infinite loop.
- `execute_callback_chain`: Calls a function pointer using three arguments and performs multiple indirect jumps and calls using a table.
- `execute_callback_2`: Calls a function pointer using three passed arguments in a specific order.
- `perform_indirect_jumps`: Executes an indirect jump to an address stored in a jump table followed by an indirect call to an address retrieved from a table.
- `return_from_subroutine_4`: The routine is empty and performs no operations before returning.
- `call_indirect_pointer_2`: Calculates an address using an offset from a pointer and executes the code at that location.
- `execute_jump_table_entry_2`: Performs an indirect jump to an address stored in a jump table.
- `execute_indirect_jump_2`: Performs an indirect call to an address retrieved from a table.
- `call_offset_pointer`: Executes a function by dereferencing a pointer located at a fixed offset from a global variable.
- `call_table_offset_40_2`: Calls a routine located at an offset from a global pointer.
- `execute_indirect_pointer`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_function_15`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_function_16`: Retrieves a function pointer from a table using an offset and executes it.
- `execute_indirect_pointer_2`: Fetches a function pointer from a memory location calculated by applying an offset to a global variable and executes it.
- `execute_indirect_pointer_3`: Fetches a pointer from a calculated offset within a global data structure and jumps to that address.
- `execute_indirect_function_17`: Fetches a function pointer from a table using an offset and executes it.
- `execute_indirect_pointer_4`: Fetches a function pointer from a location calculated by adding an offset to a global variable and executes it.
- `execute_indirect_function_18`: Loads a function pointer from a table and executes it.
- `execute_indirect_function_19`: Fetches a function pointer from a table using an offset and executes it.
- `call_indirect_handler`: Calls a function pointer stored at a specific offset within a global table.
- `call_indirect_handler_2`: Executes a subroutine located at a fixed offset from a global pointer.
- `call_indirect_handler_3`: Performs an indirect call to a routine found via an offset in a global variable.
- `call_indirect_handler_4`: Branches to a function address retrieved from a global table using a negative offset.
- `call_indirect_handler_5`: Calls a subroutine by dereferencing a pointer at a specific offset from a global base.
- `call_indirect_handler_6`: Uses a pointer from a global table to jump to another routine.
- `call_indirect_handler_7`: Executes a routine located at a specific offset relative to a global variable.
- `call_indirect_handler_8`: Performs an indirect jump to a routine based on a large negative offset from a global pointer.
- `call_indirect_handler_9`: Calls a function address stored at a specific offset within a global data structure.
- `call_indirect_handler_10`: Branches to a subroutine address retrieved from a global table using a negative offset.
- `process_complex_data`: Performs multiple indirect calls and updates various fields in a structure, including setting specific multi-byte constants and managing nested pointers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_and_load_directory_contents`: Compares a string against a list of entries and iterates through a linked list of pointers to populate a new data structure.
- `retrieve_directory_info`: Retrieves the current directory and calls a library function via a global pointer and offset.
- `find_string_end`: Iterates through a null-terminated byte string to find the address of the terminating zero byte.
- `compute_string_length`: Receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `copy_memory_block_2`: Moves a block of memory from a source to a destination.
- `return_from_subroutine_5`: This routine performs a return from a subroutine.
- `parse_quoted_string_tokens`: Parses a source string into individual tokens, handling whitespace and quoted substrings.
- `copy_string_limited`: Copies a null-terminated string from a source to a destination up to a specified maximum length.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1, and writes a null terminator at the next destination byte if the counter expires.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `initialize_window_data`: Opens a file and retrieves data from a nested structure to populate several fields in a provided structure.
- `call_indirect_function_14`: Checks if a value at a specific offset in a global structure is non-zero and, if so, executes the function pointed to by that value.
- `execute_cleanup_callback_2`: Checks a global variable and, if non-zero, executes the function pointed to by that value.
- `cleanup_resources`: Iterates through a table to close files, unlocks resources, and executes several function pointers stored in a data structure.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle_2`: Validates an index against a table, checks a flag in the table entry, calls a routine to close the associated file, and clears the table entry.
- `execute_dos_function`: Calls a function located at an offset within a pointer stored in a global structure.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `get_current_directory_2`: Calls a function located at an offset within a pointer stored in a global structure to retrieve the current directory.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `perform_dos_input`: Calls a function located at an offset within a pointer stored in a global structure to perform an input operation.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open_wrapper`: Performs an indirect call to a function located at an offset within a table pointed to by a value in memory.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_wrapper`: Executes an indirect call to a function using a pointer retrieved from a table via an offset.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read_wrapper`: Triggers an indirect call to a function by accessing a pointer through a table and an offset.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_wrapper`: Invokes a function indirectly by calculating an address from a table and an offset.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_wait_for_char`: Wrapper that calls the dos library function WaitForChar and returns its result to the caller.
- `call_write_wrapper`: Calls a function indirectly using a pointer stored at a specific offset within a table.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_offset_one`: Performs an indirect call to a function located at a specific offset within a pointer retrieved from a data structure.
- `execute_library_function`: Performs an indirect call to a function at a specific offset within a library table.
- `call_library_offset_two`: Performs an indirect call to a function located at a different specific offset within a pointer retrieved from a data structure.
- `execute_library_function_2`: Calls a function indirectly by accessing a pointer through a library table and an offset.
- `execute_library_function_3`: Executes an indirect call to a function located at a specific offset in a library table.
- `execute_library_function_4`: Invokes a function indirectly via a pointer found at an offset in a library table.
- `call_library_offset_three`: Performs an indirect call to a function located at a third specific offset within a pointer retrieved from a data structure.
- `execute_library_function_5`: Performs an indirect call to a function using an offset into a library table.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_offset_four`: Performs an indirect call to a function located at a fourth specific offset within a pointer retrieved from a data structure.
- `execute_indirect_call_3`: Performs an indirect call to a function pointer retrieved from a nested structure.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a value in a register and executes it.

### SetPatch

- `initialize_setpatch_environment_2`: Initializes the patching environment by retrieving system information and parsing command-line arguments or configuration strings.
- `update_and_cleanup_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `self_calling_guard_2`: Calls itself unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler_2`: Installs a SetPatch handler and performs a series of conditional patches and library initializations.
- `execute_indirect_function_20`: Fetches a function pointer from a table and executes it.
- `call_indirect_via_table_2`: The routine calls another routine through a pointer stored in a global variable and returns the value produced by that indirect call.
- `execute_indirect_function_21`: Fetches a function pointer from an offset within a global data structure and jumps to it.
- `execute_indirect_function_22`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_23`: Fetches a function pointer from an offset within a global data structure and executes it.
- `call_indirect_handler_one`: Performs an indirect call to an address stored at a specific offset within a global structure.
- `call_indirect_handler_two`: Executes a routine by calling a pointer located at a different offset within a global structure.
- `call_indirect_handler_three`: Calls a function pointer retrieved from a global structure using a specific offset.
- `call_indirect_handler_four`: Retrieves a pointer from a global structure and performs an indirect call to it.
- `call_indirect_handler_five`: Uses an offset from a global structure to locate and call an indirect function pointer.
- `call_indirect_handler_six`: Performs an indirect call to a routine located via an offset in a global structure.
- `allocate_and_patch_memory_2`: Checks specific fields within a global structure and, if criteria are met, allocates a memory block and copies a small code stub into it.
- `calculate_checksum_and_call`: Iterates through a list of words to calculate a cumulative sum, stores the bitwise NOT of that sum in a memory location, and then performs an indirect call.
- `disable_interrupts_and_patch_2`: Checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word_2`: Calls another routine and tests its return value.
- `call_conditional_handler`: Compares a global variable against a value from a structure and against negative one, then calls a routine via a global pointer if the conditions are met.
- `call_conditional_handler_2`: Checks if a register contains negative one and, if not, calls a routine through a pointer stored in a global variable.
- `update_interrupt_enable`: Checks a global variable against a constant, updates it and two other memory locations, and performs an indirect call based on the new value while modifying the interrupt enable bit in the INTENA register.
- `execute_indirect_call_4`: Performs an indirect call to a function pointer retrieved from a table.
- `initialize_patch_function_table_2`: Calculates an offset based on a global variable and fills a sixteen-entry table with successive addresses of the routine, incrementing the pointer each iteration.
- `return_empty_register`: The routine performs no operations and returns the value currently held in a register.
- `return_input_value_3`: An empty routine that returns the value provided in a register unchanged.
- `pass_register_value`: Returns the value contained in a data register unchanged.
- `return_input_value_2`: The routine receives a value in D0 and returns it unchanged.
- `execute_conditional_callback`: Checks a value in a global variable against a constant and, if it is within range, executes a function pointer stored in a global variable.
- `copy_list_elements`: Copies a sequence of values from one memory location to another by calling an indirect routine for each element.
- `loop_and_mask_pointer`: Iterates through a series of function calls using a loop and returns a value from a pointer bitwise ORed with a constant.
- `fetch_and_modify_bitmask`: Executes an indirect function call and returns a value from a pointer with the highest bit set.
- `perform_indirect_call`: Performs an indirect call to an address stored at an offset from a value in a register.
- `return_immediately_2`: The routine executes four instructions and returns to the caller without performing any observable operations on registers or memory.
- `execute_conditional_callbacks_2`: Checks a value at an offset from a pointer and, if non-zero, executes two subsequent function calls using that value.
- `return_zero_value_2`: The routine is empty and returns a value of zero to the caller.
- `find_marker_word`: Iterates through an array of words using a pointer until it finds a word whose high byte matches a specific constant.
- `calculate_word_offset`: Reads a 16-bit value from two bytes after an input pointer and returns a new pointer adjusted by a bit-shifted version of that value.
- `calculate_offset_pointer`: Reads a 16-bit signed value from an offset of two bytes from a pointer, scales it by two, and returns the resulting address offset by two bytes from the original pointer.
- `populate_patch_pointer_table`: Iterates through a data array to find specific markers and calculates relative offsets to resolve absolute addresses.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_pointer`: The routine calls another routine through a pointer stored at a fixed offset from a global variable.
- `invoke_recursive_stack_pointer`: Calls another routine while passing two parameters and a pointer to a specific location on the stack.
- `invoke_and_checksum_stack`: Invokes a function pointer stored in a global variable and computes a rolling checksum over the local stack buffer until a null terminator is reached.
- `store_byte_at_pointer`: The routine stores a byte from a register through a pointer.

### EndCLI

- `parse_and_initialize_environment`: Iterates through a linked list to find a maximum value and then populates a structured data block with calculated offsets and values.
- `copy_environment_block`: Copies a block of sixteen bytes from a memory location to a destination pointed to by a stack value.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: Locates a fast memory block and retrieves the command line arguments from the operating system.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the chip block.
- `invoke_library_function`: Invokes a function pointer located at a specific offset within a global structure.
- `execute_indirect_function`: Fetches a function pointer from a memory location using an offset and executes it.
- `execute_indirect_function_2`: Fetches a function pointer from a location offset from a global variable and executes it.
- `execute_indirect_function_3`: Fetches a function pointer from a location relative to a global variable and executes it.

### ENDIF

- `return_zero`: This routine is empty and simply returns a value of zero to the caller.

### IF

- `build_dynamic_jump_table`: Iterates through a linked list to find a maximum value and then populates a structured table with calculated offsets and data.
- `copy_data_and_execute_callback`: Copies sixteen bytes from a memory location to a destination pointed to by a stack pointer, then calls a function through a pointer.

### Version

- `parse_command_line_arguments`: Retrieves the command line string and parses it into individual tokens.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `process_message_reply_2`: Calls a routine to handle message processing by updating message fields and calling various functions based on flags, then calls a library vector to finalize the operation.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating until the null byte is found.
- `call_reschedule_wrapper`: Calls a function pointer located at an offset within a global table.
- `execute_indirect_function_4`: Fetches a function pointer from a table and executes it.
- `invoke_verify_with_stack_pointer`: Calls a verification routine passing two parameters and a pointer to a specific stack location.
- `call_library_function_wrapper`: Calls a function pointer retrieved from an offset within a global table.
- `execute_indirect_function_5`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_6`: Fetches a function pointer from a location calculated by an offset from a global variable and executes it.
- `execute_indirect_function_7`: Fetches a function pointer from an offset within a global structure and executes it.
- `execute_indirect_function_8`: Fetches a function pointer from a location relative to a global variable and executes it.
- `execute_indirect_function_9`: Fetches a function pointer from an offset within a global structure and executes it.
- `verify_global_function_and_checksum`: Invokes a function pointer stored in a global variable and calculates a rolling checksum over a stack buffer until a null terminator is reached.
- `store_byte_via_pointer`: Stores a byte from a register into a memory location pointed to by another register.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and writes it into a buffer.

### Disk.2:BJELoad_R

- `clear_and_execute_callback`: Clears a block of memory by zeroing out a sequence of words and then executes a function pointer from a global structure based on a bitmask check.
- `execute_nested_calls`: Performs a sequence of calls to other routines, including one that passes a large constant and a stack buffer.
- `initialize_data_structure`: Populates a structure with several constant values and pointers, then updates a table entry with the address of the structure.
- `update_structure_pointers`: Updates a pointer within a structure based on the value of another field and conditionally copies data between structure offsets.
- `transfer_structure_fields`: Copies data between offsets of two structures and performs an indirect call based on a value from the first structure.
- `execute_conditional_logic`: Initializes a local buffer with a constant value and performs an indirect call using that buffer and a parameter.
- `execute_recursive_call_2`: Performs a direct call to itself before returning zero.
- `process_conditional_data`: Performs a series of conditional indirect calls and, if certain criteria are met, copies a block of bytes into a structure and updates several fields.
- `process_memory_block`: Checks conditions using indirect calls and, if a specific condition is met, copies a block of data from one location to another.
- `execute_conditional_indirect_calls`: Performs indirect calls using values found at specific offsets from a pointer if certain conditions are met.
- `decrement_and_branch_on_zero`: Decrements a 16-bit value at an offset within a structure and calls three different routines if the value reaches zero and a specific bit is set.
- `set_status_bit`: Sets a specific bit in a byte located at an offset from the provided pointer.
- `traverse_linked_list_recursively_2`: Attempts to process a structure and, if certain conditions are met, recursively calls itself with a pointer retrieved from that structure.
- `traverse_linked_structure`: Recursively calls routines using pointers found within a structure and returns a status value.
- `parse_resource_manifest`: Validates a resource header by checking specific magic constants and field values before iterating through a list of resource entries.
- `validate_and_map_resource_indices_2`: Verifies a resource structure header and recursively processes a linked list of dependency entries.
- `validate_and_map_resource_dependencies`: Iterates through a resource table to identify and store the indices of specific dependency types while verifying their validity.
- `resolve_resource_dependency_indices`: Iterates through a resource table to identify and store the indices of specific dependency types while verifying their validity.
- `validate_resource_dependency_indices`: Iterates through a resource table to identify and store the indices of specific dependency types while verifying their validity.
- `validate_resource_dependency_tree_3`: Iterates through a resource table to identify and store indices of specific dependency types while verifying their validity.
- `validate_resource_dependency_tree`: Iterates through a resource table to identify and store indices of specific dependency types while verifying their validity.
- `verify_resource_dependency_chain`: Iterates through a resource table to verify specific status flags and ensure required dependencies are present.
- `process_resource_allocation_chain`: Iterates through a table of resource entries to calculate and allocate memory segments for a linked structure.
- `allocate_resource_memory_block`: Calculates required memory sizes based on table entries and performs a series of nested calls to allocate and initialize memory segments.
- `clear_linked_structure_field`: Recursively traverses a structure through several pointer offsets and sets a specific field to zero if it is non-zero.
- `resolve_resource_dependency_tree_4`: Iterates through a resource table to identify specific dependency types and updates linked indices within a structure.
- `traverse_resource_dependency_tree`: Recursively traverses a hierarchical structure of resource dependencies to validate integrity and calculate memory offsets.
- `resolve_resource_dependency_chain`: Recursively validates and links resource dependencies by traversing a table of entries and updating nested pointers.
- `traverse_and_clear_linked_list`: Recursively traverses a nested structure through multiple pointer offsets.
- `resolve_resource_dependency_tree_5`: Iterates through a resource table to identify specific dependency types and updates linked indices within a structure.
- `process_resource_dependency_tree`: Recursively traverses a linked structure of resource dependencies to validate and update memory offsets and pointers.
- `resolve_resource_dependency_tree`: Recursively traverses a linked structure of resource entries to validate their existence and calculate memory offsets.
- `traverse_linked_structure_2`: Recursively visits nodes in a linked structure by following pointers stored at specific offsets.
- `resolve_resource_dependency_tree_6`: Iterates through a resource table to identify and link specific dependency types by updating indices within a structure.
- `resolve_resource_dependency_links`: Iterates through a resource table to identify specific dependency types and updates linked indices within a structure.
- `resolve_resource_dependency_chain_4`: Iterates through a resource table to validate specific flags and resolve dependencies by calculating memory offsets and allocating contiguous segments.
- `resolve_resource_dependency_tree_2`: Iterates through a resource table to identify and link specific dependency types while validating their status flags.
- `rebuild_resource_node_hierarchy`: Recursively traverses a nested data structure to validate child element sizes and rebuilds a linked list of pointers to these elements.
- `reconstruct_resource_tree_node`: Recursively traverses a nested data structure to validate sizes and checksums, then reconstructs a linked list of pointers to child elements.
- `rebuild_resource_tree_nodes`: Recursively traverses a nested data structure to validate child node bounds and rebuilds a flattened list of pointers.
- `flatten_resource_hierarchy_3`: Recursively traverses a nested data structure to validate its integrity and then copies its elements into a linear array.
- `flatten_resource_hierarchy`: Validates a nested data structure through recursive checksum and size comparisons before flattening it into a linear array.
- `rebuild_resource_index_table`: Validates a nested data structure through recursive size and checksum checks before flattening it into a linear array.
- `reconstruct_resource_hierarchy`: Validates a nested data structure by checking checksums and element counts before copying properties into a new memory buffer.
- `resolve_resource_hierarchy`: Recursively validates and traverses a nested data structure to extract and link resource elements.
- `resolve_and_copy_linked_data`: Recursively traverses a nested data structure to validate specific values and then copies a sequence of words from a source buffer into a destination buffer.
- `reconstruct_and_link_data_structures`: Copies a sequence of words from a source buffer into a destination buffer while handling zero-terminated padding and updating linked structures.
- `unpack_and_reconstruct_structure`: Copies a sequence of words from a source buffer into a destination buffer while handling zero-terminated padding and updating linked structure fields.
- `unpack_and_validate_tree_structure`: Copies word sequences from a source buffer into a destination buffer while handling zero-terminated padding.
- `unpack_tree_node_data`: Recursively traverses and unpacks a hierarchical data structure by copying word-aligned sequences from a source buffer into a destination buffer.
- `validate_linked_structure_3`: Recursively traverses a linked data structure to perform integrity checks and update node fields.
- `validate_and_relink_tree_nodes`: Recursively traverses a tree-like data structure to validate node integrity and checksums.
- `validate_linked_structure`: Recursively traverses a linked data structure to perform validation checks and updates various fields and sums within the nodes.
- `traverse_linked_structure_4`: Recursively traverses a linked data structure while performing conditional updates to various fields and integer sums within the nodes.
- `update_linked_structure_3`: Recursively traverses and updates fields within a linked structure based on conditional checks and calculated sums.
- `verify_and_update_structure`: Compares a calculated sum against a stored value in a structure and, if they match, updates several fields in a destination structure using data from the source.
- `update_linked_structure`: Updates several fields within a structure pointed to by a value retrieved from a memory offset and performs recursive calls.
- `clear_structure_field`: Checks a value at an offset within a structure, calls another routine with that value, and then clears the field to zero.
- `initialize_pointer_table`: Sets a pointer to point to the next element in a sequence, clears the following element, and stores the original pointer address.
- `traverse_pointer_chain_2`: Follows a chain of pointers until a null pointer is encountered or a value at a specific offset matches the provided parameter.
- `traverse_pointer_chain_3`: Iterates through a linked list of pointers until a specific value is found at a given offset or a null pointer is encountered.
- `execute_recursive_call_3`: Performs a direct call to itself before returning zero.
- `insert_node_into_list`: Updates a pointer to point to a new structure and modifies the internal links of the involved structures to form a chain.
- `link_data_elements`: Stores a pointer into a location pointed to by another pointer and updates a secondary link within that structure.
- `execute_fixed_calls`: Performs a series of indirect calls, including one with a specific constant value.
- `process_data_structure`: Initializes several global values and then iterates through a loop that calls routines based on the results of a primary function call.
- `dispatch_parameterized_calls`: Executes three separate routines using the provided parameter and a modified version of that parameter.
- `search_linked_list`: Traverses a linked list starting from a global variable, performing actions when a specific field in the structure equals zero.
- `compare_memory_block`: Compares three specific words and a longword within a memory block to return a boolean result.
- `return_zero_value`: This routine is empty and simply returns a zero value to the caller.
- `calculate_weighted_sum_3`: Computes a sum by multiplying bytes in two different memory ranges by their relative indices and accumulating the results.
- `calculate_weighted_sum`: Iterates through a memory range, multiplying each byte by its index plus one and accumulating the result into a sum.
- `sum_short_array`: Iterates through an array of sixteen-bit integers and returns the total sum of the elements.
- `return_from_subroutine`: The routine performs a single return instruction to exit the current execution flow.
- `sum_byte_array`: Iterates through a memory range of bytes and returns the total sum of the values.
- `return_from_subroutine_2`: This routine performs a single return instruction to exit the current execution flow.
- `update_pointer_link`: Retrieves a pointer from a location offset from the input parameter and, if not null, updates the value at that pointer to a new address.
- `increment_and_store_pointer`: Increments a pointer by four bytes and stores a new calculated value into the memory location previously pointed to.
- `execute_recursive_call`: If the provided parameter is non-zero, it performs an indirect call using a value retrieved from an offset of that parameter.
- `store_retrieved_value`: Calls a routine with a specific constant and, if the result is non-zero, stores that result into a location pointed to by the second parameter.
- `assign_pointer_to_structure`: Stores a value from a specific offset of a structure into a pointer located at another offset within that same structure.
- `execute_jump_table_entries`: Performs indirect jumps and calls using addresses retrieved from a jump table and a DOS function handler.
- `execute_indirect_calls`: Executes an indirect call to a function retrieved from a table and invokes a seek wrapper.
- `compare_routine_result`: Calls another routine using three parameters and returns a boolean indicating whether the returned value does not match the second parameter.
- `execute_recursive_call_4`: Performs a direct call to itself before returning zero.
- `process_string_and_output`: Calculates the length of a null-terminated string and uses that value to perform output and write operations via table-based pointers.
- `get_input_byte`: Calls a routine to retrieve a value and, if successful, performs a secondary operation with that value before returning it.
- `call_recursive_routine`: Calls another routine and returns zero.
- `read_character_to_stack`: Calls a routine to obtain an integer and, if non-zero, uses that integer to perform a read operation into a local stack variable.
- `execute_recursive_call_5`: Performs a direct call to itself before returning zero.
- `handle_fatal_error`: Displays a fatal loader error message and enters an infinite loop.
- `execute_callback_with_arguments`: Calls a function pointer using multiple arguments retrieved from a table and a pointer.
- `execute_callback`: Calls a function pointer using three passed arguments.
- `execute_indirect_jumps`: Performs indirect jumps and calls using addresses retrieved from a table.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning.
- `call_indirect_pointer`: Loads a function pointer from an offset within a table and executes it.
- `execute_jump_table_entry`: Performs an indirect call to an address stored in a jump table.
- `execute_indirect_jump`: Loads a function pointer from a table and transfers control to that address.
- `call_table_offset_3c`: Executes a subroutine located at an offset from a pointer stored in a global variable.
- `call_table_offset_40`: Performs an indirect call to a function pointer retrieved from a global table.
- `call_table_offset_44`: Calls a routine by fetching a pointer from a specific offset within a global data structure.
- `call_table_offset_48`: Triggers a subroutine through a pointer found in a global variable at a fixed offset.
- `call_table_offset_4c`: Redirects execution to another routine using a pointer stored in a global table.
- `call_table_offset_50`: Invokes a function by dereferencing a pointer at a specific offset from a global variable.
- `call_table_offset_58`: Executes a routine via an indirect jump using a pointer from a global table.
- `call_table_offset_5c`: Calls a subroutine using a pointer located at a specific offset within a global data structure.
- `call_table_offset_54`: Performs an indirect call to a routine retrieved from a global variable offset.
- `call_table_offset_10`: Executes a function pointer found at a specific offset from a global variable.
- `call_indirect_offset_18`: Calls a routine via a pointer located at an offset from a global variable.
- `call_indirect_offset_1c`: Executes a function by dereferencing a pointer stored at a specific offset from a global variable.
- `call_indirect_offset_14`: Performs an indirect call to a subroutine using an address retrieved from a global table.
- `call_indirect_offset_2c`: Calls another routine using a pointer found at a specific offset from a global variable.
- `call_indirect_offset_30`: Dereferences a pointer at a fixed offset from a global variable and jumps to it.
- `call_indirect_offset_34`: Executes a subroutine by calling an address stored in a global variable at a specific offset.
- `call_indirect_offset_38`: Performs an indirect jump to a routine located via a pointer in a global table.
- `call_indirect_offset_20`: Calls a routine using a pointer retrieved from a global variable at a specific offset.
- `call_indirect_offset_64`: Executes a function by dereferencing a pointer at a large offset from a global variable.
- `call_indirect_offset_68`: Calls a subroutine using an address stored at an offset from a global variable.
- `execute_indirect_function_10`: Fetches a function pointer from a memory location using an offset and executes it.
- `process_data_structure_2`: Manipulates a complex data structure by calculating offsets, writing a specific four-byte constant, and performing multiple indirect calls and recursive operations.
- `update_resource_metadata`: Compares a string against a stored value and, if they match, traverses a linked list to calculate a count and populate a new data structure with the list elements.
- `parse_directory_structure`: Iterates through a directory list to compare string names and build a linked list or table of file information.
- `query_directory_info`: Retrieves the current directory and calls functions through a library vector.
- `calculate_string_length_2`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `calculate_string_length_3`: Iterates through a null-terminated byte string to determine its length.
- `copy_memory_block`: Copies a block of memory from a source to a destination.
- `nop_placeholder`: This routine is empty and performs no operations.
- `parse_quoted_string_list`: Parses a buffer of text into an array of individual strings by identifying space-separated tokens and handling quoted substrings.
- `copy_string_with_limit_3`: Copies a null-terminated string to a destination buffer while enforcing a maximum length.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `initialize_window_structure`: Opens a file and populates a structure with values retrieved from nested pointers and library calls.
- `call_indirect_function`: The routine checks a global pointer and, if it is not zero, calls the function pointed to by that address.
- `execute_cleanup_callback`: Checks a global pointer and executes the function it points to if it is non-zero.
- `cleanup_system_resources`: Iterates through tables to close file handles, unlock memory, and execute various cleanup callbacks stored in a global structure.
- `nop_padding_sequence`: This routine is empty and consists only of instructions that perform no operation.
- `close_file_handle`: The routine validates an index against a table, checks a flag, calls a function to close a file, and clears the entry in the table.
- `execute_indirect_call_2`: The routine performs an indirect call to a function located at an offset from a pointer stored in a global variable.
- `execute_indirect_function_11`: Retrieves a function pointer from a nested structure within a memory offset and executes it.
- `create_process`: Calls a function from a table to create a new process.
- `get_current_directory`: Calls a wrapper function to retrieve the current directory.
- `get_current_directory_direct`: Calls a function from a table to obtain the current directory.
- `delay_execution`: Calls a function from a table to perform a delay.
- `duplicate_lock`: Calls a function from a table to duplicate a file lock.
- `get_input_wrapper`: Calls a wrapper function to retrieve input.
- `get_input_direct`: Calls a function from a table to request input.
- `open_file_wrapper`: Calls a wrapper function to open a file.
- `open_file_direct`: Calls a function from a table to open a file.
- `get_output_wrapper`: Calls a wrapper function to retrieve output.
- `call_dos_output`: Performs an indirect call to a function located at an offset within a structure pointed to by a global variable.
- `call_dos_read_wrapper`: Performs an indirect call to a wrapper function that executes a read operation.
- `call_dos_read`: Performs an indirect call to a function that executes a read operation from a file.
- `call_dos_seek_wrapper`: Performs an indirect call to a wrapper function that executes a seek operation.
- `call_dos_seek`: Performs an indirect call to a function that executes a seek operation on a file.
- `call_dos_unlock`: Performs an indirect call to a function that unlocks a file.
- `call_dos_wait_for_char`: Performs an indirect call to a function that waits for a character input.
- `call_dos_write_wrapper`: Performs an indirect call to a wrapper function that executes a write operation.
- `call_dos_write`: Performs an indirect call to a function that executes a write operation to a file.
- `call_library_function`: Performs an indirect call to a function at a specific offset within a library structure.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_library_function_via_offset`: Performs an indirect call to a function located at a specific offset within a library structure retrieved from a global pointer.
- `call_indirect_function_3`: Executes a function located at an offset within a table pointed to by a global variable.
- `call_library_function_3`: Performs an indirect call to a function by accessing a pointer through a library table and an offset.
- `call_indirect_function_4`: Calls a subroutine by dereferencing a pointer found at a specific offset in a table.
- `call_indirect_function_5`: Uses a pointer from a table to jump to another routine.
- `call_indirect_function_6`: Performs an indirect jump to a function address stored in a table.
- `call_library_function_4`: Executes a function via an indirect call using an offset into a library table.
- `call_indirect_function_7`: Retrieves a function address from a table and calls it.
- `call_indirect_function_8`: Calls a routine through a pointer located at a specific offset in a data structure.
- `execute_nested_function_pointer`: Retrieves a function pointer by traversing two levels of offsets from a base address stored in a register and executes it.
- `call_indirect_function_9`: Executes a function by following a pointer through a table and an offset.
- `call_indirect_function_10`: Performs an indirect call to a routine address stored in a table.
- `call_indirect_function_11`: Calls a function address retrieved from a table via an offset.
- `call_indirect_function_12`: Calls a function pointer retrieved from a table using an offset.
- `call_indirect_function_13`: Calls a function pointer retrieved from a table using a different offset.
- `execute_indirect_function_12`: Retrieves a function pointer from a nested offset within a structure pointed to by a register and executes it.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested offset within a structure pointed to by a value in a register and executes it.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Disk.2:bootblock

- `check_library_status`: Calls a function from a library and returns zero if the result is non-zero, otherwise returns all bits set.
