# Game 582 (Sim Life): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00582). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2707728 top-level bytes in 165 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 29 of 29 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 291 of 291 named (0 author symbols), 291 of 291 described (49 by rule, 225 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1389 of 1389 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 535431 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9318 of 9564 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 976771 |
| code | 29542 |
| data | 1439764 |
| padding | 31763 |
| unused | 229888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| install | 1 | 433 | 6 |
| stack | 2 | 117 | 2 |
| setpatch | 35 | 962 | 54 |
| assign | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| copy | 2 | 117 | 2 |
| install3 | 3 | 117 | 2 |
| ram-handler | 2 | 10 | 1 |
| Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:installit | 5 | 4443 | 71 |
| f | 1 | 4 | 1 |
| ramdrive.device | 3 | 345 | 17 |
| 13 | 1 | 2 | 1 |
| 7 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| 11 | 1 | 111 | 2 |
| 12 | 1 | 112 | 2 |
| 16 | 1 | 2 | 1 |
| 18 | 1 | 2 | 1 |
| 6 | 1 | 112 | 2 |
| 8 | 1 | 2 | 1 |
| 9 | 1 | 2 | 1 |
| asl.library | 2 | 31 | 3 |
| diskfont.library | 15 | 1040 | 76 |
| Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:simintro.exe | 2 | 267 | 8 |
| Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 283 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 800 | 12 |
| Sim Life (1993)(Mindscape)[cr FLT](Disk 2 of 3).adf:res5 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- sim life (1993)(mindscape)[cr flt](disk 1 of 3)/Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:simv13: high-entropy data file (7.71 bits/byte), classified as packed data
- sim life (1993)(mindscape)[cr flt](disk 1 of 3)/Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:simv20: high-entropy data file (7.77 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 237 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 268 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sim life (1993)(mindscape)[cr flt](disk 2 of 3)/Sim Life (1993)(Mindscape)[cr FLT](Disk 2 of 3).adf:res5: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- sim life (1993)(mindscape)[cr flt](disk 2 of 3)/Sim Life (1993)(Mindscape)[cr FLT](Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- sim life (1993)(mindscape)[cr flt](disk 3 of 3)/Sim Life (1993)(Mindscape)[cr FLT](Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 1 never-loaded region(s), 535431 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:simintro.exe

## Copy protection found

- `perform_io_operation` (installit): Calculates a sum from a table of values and executes an I/O operation using a function pointer from a global structure. Kind: checksum/integrity check. Pass path: the routine continues execution normally after the loop and subsequent assignments.
- `execute_io_operations` (installit): Executes a function pointer from a global structure, modifies two values at specific offsets within a second structure, and then executes another function pointer from the global structure. Kind: track/sector verification check. Pass path: the routine continues to the next instruction (the second I/O call) after the first call returns, implying the first call is a 'seek/read' and the second is a 'verify/status' or 'load' operation where the return value is checked implicitly .

## Functions


### install

- `install_bootblock_to_disk`: Validates a device name for floppy or hard disk formats and attempts to write a bootblock to the specified volume.
- `call_library_function_and_dereference`: Sets a bitmask in a register, calls a library function via a pointer, and if the result is non-zero, dereferences a pointer found at an offset from the result.
- `clear_register_and_branch`: Sets the d0 register to a negative value and performs a relative branch.
- `modify_status_byte`: Calls three different functions via pointers and performs a bitwise OR operation on a byte at a specific offset from a returned pointer.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `copy_memory_block`: Copies a specified number of bytes from a source pointer to a destination pointer.

### stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_value`: Moves a longword from one register to another and then performs a short branch to another location.

### setpatch

- `patch_system_environment`: Initializes system structures and parses a string-based configuration list into a structured array of pointers.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `update_task_status`: Retrieves a task pointer, modifies a value at an offset within that task's structure using a specific constant, and calls an indirect routine.
- `self_calling_guard`: Calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
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
- `call_indirect_via_table`: Calls another routine through a pointer stored in a global variable and returns the value produced by that indirect call.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `calculate_checksum_and_call`: Iterates through a table to calculate a sum, stores the bitwise NOT of that sum into a global variable, and calls a routine from a jump table.
- `apply_patch_with_interrupt_control`: Checks a global variable and calls a routine through a table.
- `check_flag_and_copy_word`: Calls another routine and tests its return value.
- `dispatch_conditional_jump`: Compares a long integer from a relative memory location with a value in a table and performs an indirect jump through a pointer if they are equal.
- `compare_and_branch`: Compares two long integers and branches if they are equal.
- `set_interrupt_and_call`: Sets a global variable, modifies a value in memory, and calls a routine using an offset from a register while potentially affecting the INTENA register.
- `call_offset_pointer`: Calls a routine using a pointer located at an offset from a global variable.
- `initialize_patch_function_table`: Calculates a computed offset to store in a global variable and fills a sixteen-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `clear_data_register`: Performs a bitwise OR operation on a data register with zero.
- `execute_branch_to_offset`: Performs a single brief subroutine call to a specific offset.
- `execute_multiple_calls`: Repeatedly calls another routine multiple times before returning a value.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `call_indirect_handler`: Checks a value in a global variable against a constant and, if it is within range, calls a function pointer stored in another global variable.
- `copy_structure_elements`: Copies elements from one structure to another by iterating through a list of values.
- `call_routine_via_table`: Decrements a register, loads a pointer from a table, and calls a routine using an offset derived from a register.
- `calculate_offset_and_call`: Calculates a value based on a field in a structure, calls a routine using a pointer from a table, and sets a bit in d0 before branching.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `reset_global_variable`: Sets a global variable to zero and returns its previous value.
- `execute_conditional_callbacks`: Calls a routine using a pointer from a global table and checks the result.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `calculate_word_offset`: Calculates an offset from a pointer based on the value of the word at that pointer and returns a new pointer.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `traverse_linked_list_2`: Performs a bitwise XOR between two values and, if certain bits are set, traverses a linked list and calls a function pointer if specific bits are present in the list nodes.
- `nop_routine`: The routine is empty and performs no operations.
- `return_from_subroutine`: This routine performs no operations and immediately returns to its caller.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### assign

- `find_max_and_populate_buffer`: Traverses a linked list to identify the maximum value in a specific field and then initializes a buffer with a sequence of incrementing values.
- `move_and_branch`: Moves a long value from one register to another and then performs a short relative branch.

### endcli

- `find_max_and_populate_buffer_2`: Traverses a linked list to identify the maximum value in a specific field and then allocates or prepares a buffer based on that value.
- `transfer_register_and_branch`: Copies the contents of one data register to another and then performs a short relative branch to another location.

### copy

- `reallocate_and_copy_buffer`: Traverses a linked list to find a maximum value, then reallocates a buffer and performs a series of memory copies to update a data structure.
- `transfer_register_and_branch_2`: Moves a longword from one data register to another and then performs a short relative branch to another location.

### install3

- `find_max_and_populate_buffer_3`: Traverses a linked list to identify the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `transfer_register_and_branch_3`: Copies a longword from one data register to another and then performs a short relative branch to another location.

### ram-handler

- `execute_indirect_call`: Loads a pointer from a global variable and executes the routine located at that address.

### Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:installit

- `initialize_environment_and_parse_args`: Sets up a local stack frame and clears a memory block before parsing command line arguments.
- `execute_callback_chain`: Updates a pointer with a passed value and executes a series of conditional function calls through pointers stored in a global structure.
- `validate_executable_hunks`: Calculates the total size of an executable by iterating through its hunk list and compares the result against a size threshold.
- `process_data_list`: Iterates through a list of structures to perform conditional operations and memory allocations.
- `process_installation_metadata_list`: Iterates through a table of structures to evaluate file status and conditional flags.
- `initialize_message_structure`: Populates a structure with various values and pointers, then executes three different routines through pointers stored in a global variable.
- `concatenate_memory_values`: Concatenates a global variable with a register value by calling a memory copy routine.
- `execute_indirect_calls`: Performs a series of calls to other routines and updates a global variable before executing a function through a pointer.
- `return_without_effect`: The routine has 8 instructions in the binary, but the decompiler reduces them to an immediate return, meaning the body performs no observable work beyond returning to its caller.
- `return_without_effect_2`: The routine consists of eight instructions that perform no observable work and result in an immediate return to the caller.
- `calculate_string_length`: Calculates the length of a null-terminated string by iterating through its characters.
- `parse_format_string_2`: Parses a string for format specifiers such as decimal, pointer, or string.
- `traverse_and_free_list`: Iterates through a linked list to execute a function for each node, then traverses a second linked list to update a pointer and potentially call another routine before finishing.
- `append_path_delimiters`: Modifies a string by inserting specific characters such as colons, slashes, or dots at certain positions based on provided parameters.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer in A1 to a destination pointer in A0 until a zero byte is encountered, writing each byte to the destination and advancing both pointers.
- `format_signed_integer`: Writes a minus sign through a pointer when the input value is negative, then repeatedly calls another routine to obtain digits and stores ASCII characters through pointers, and finally reverses the digit bytes in a stack buffer before terminating the string with a null byte.
- `parse_signed_integer`: The routine reads a signed decimal integer from a zero-terminated string pointed to by a register, handling an optional leading plus or minus sign.
- `calculate_complex_sum`: This routine calls another routine and performs a series of additions and multiplications involving values from a table and a global variable.
- `initialize_memory_header`: Checks a bit in a byte, then if not set, calls another routine to set several fields in a structure including a pointer and a size value.
- `map_dos_error`: The routine calls a function via a pointer in a global structure and uses the result to assign a specific error code to a global variable based on the returned value.
- `populate_configuration_structure`: Copies data from a source structure into a destination buffer, including bitwise flag manipulation and string copying.
- `execute_conditional_callback`: Calls a subroutine via a pointer in a global structure if a previous function returns a non-zero value.
- `set_protection_status`: The routine calls a function via a pointer in a global structure and updates a global variable and another value based on whether the call succeeded.
- `process_file_status_logic`: Evaluates bits within a structure to perform conditional branching, including iterating through a list of function pointers and updating a global variable with a status code or error result.
- `test_bit_and_load_registers`: Tests a specific bit in a register and then loads three values from the stack into registers.
- `process_string_data`: Processes a string by stripping carriage return characters and updating global status values.
- `handle_file_status`: Checks the result of a subroutine and updates global variables with specific status codes.
- `evaluate_buffer_state`: Evaluates a memory buffer and updates its contents based on bitwise checks.
- `append_string_to_linked_list`: Appends a string to a new node within a linked list and updates the list's tail and count.
- `calculate_fixed_point_division`: Performs signed division on two 32-bit values using bit shifts and normalization to return a fixed-point result.
- `normalize_fixed_point`: The routine takes two 32-bit inputs and normalizes a fixed-point value by scaling the numerator with a variable shift derived from the denominator’s magnitude, performing a division with remainder and a borrow correction, then returns a 64-bit result composed of a signed quotient and a scaled remainder.
- `handle_game_event`: The routine is 21 instructions long but ends with an immediate return, performing no observable work on data or registers.
- `execute_callback_chain_2`: This routine iterates through a linked list of pointers and executes specific functions based on bitwise checks of the data.
- `parse_command_line_arguments`: Parses a string of input to extract individual arguments, handling quoted strings and expanding escape sequences like newlines and carriage returns.
- `move_register_to_stack`: The routine moves the contents of one register to another and then loads a value from the stack into a register.
- `process_string_and_callbacks`: Copies a string into a local buffer, performs conditional checks using various pointers, and executes a chain of callback functions.
- `traverse_linked_list`: The routine iterates through a linked list a specified number of times, updating a global variable with a status code based on whether the list is exhausted or the count is invalid.
- `manage_string_callback_state`: Copies a string from a pointer into a local stack buffer and executes a sequence of function pointers.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `append_string_to_buffer`: Finds the end of a destination string and copies bytes from a source string to that location until a null terminator is reached.
- `copy_c_string`: Locates the null terminator of a destination string and then copies all bytes from a source string, including the terminator, to that position.
- `copy_bytes_with_termination`: Copies bytes from a source to a destination until a null terminator is found, then fills the remaining specified count with null bytes.
- `update_linked_list_node`: Traverses a linked list to locate a specific memory block based on a provided pointer.
- `allocate_memory_block`: Searches a linked list of memory segments to find a block large enough to satisfy a requested size, splitting the block if it is larger than needed.
- `execute_callback_chains`: Iterates through multiple linked lists to execute function pointers based on bitwise checks of the data.
- `increment_buffer_indices`: Increments two values stored in a structure and copies a byte to an offset within that structure.
- `check_status_bits`: Calls another routine and then checks specific bits in a structure to return either zero or a specific constant.
- `store_byte_and_update_state`: Examines status bytes in a structure and branches on their bits to set flags, reset counters, or return early.
- `read_file_stream`: Reads bytes from a source into a destination buffer by checking status flags and decrementing a counter.
- `process_file_node_stream`: Iterates through a linked list or buffer associated with a file node structure, decrementing counters and updating status flags based on the results of subsequent routine calls.
- `copy_data_to_buffer`: Copies characters from a source to a destination buffer while checking structure flags and handling newlines.
- `update_file_node_state`: Checks flags in a file node structure, iterates through a linked list to verify conditions, updates internal counters and status bytes based on helper return values, and returns a status code.
- `parse_command_string`: Parses a string to identify a command and its associated numeric arguments.
- `parse_format_string`: This routine scans a string for percent signs and executes a function pointer when a conversion specifier is identified.
- `delete_file_and_check_error`: Attempts to delete a file and, if the operation fails, retrieves an error value and stores it in a global variable.
- `release_system_resources`: Releases a signal, a memory block, and a message port while updating specific bytes in a structure.
- `allocate_memory_structure`: Allocates a memory block and initializes its fields with a provided value and a constant byte.
- `load_stack_parameters`: Loads two longwords from the stack into registers A0 and D0.
- `allocate_message_port`: Reserves a signal bit and allocates a memory block for a message port, filling in the owning task, port type, signal number and the supplied pointer.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `invoke_memory_copy`: Calls a routine that performs a memory block copy and returns a concatenated value consisting of a global variable and a register contents.
- `copy_memory_block_2`: Performs a complex memory copy operation involving loops that decrement registers and move data between memory locations.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_registers_in_loop`: Performs a loop that decrements two registers until they reach a specific value.
- `execute_io_sequence`: Opens a library and a disk device to perform a series of input and output operations before returning zero.
- `open_dos_library`: Opens a library and stores the resulting base address into two global variables.
- `close_library_and_return`: Calls a function pointer from a global structure and returns zero.
- `clear_data_word`: Performs a logical OR operation on a register with zero.
- `open_trackdisk_device`: Opens a device and uses the resulting identifier to perform subsequent operations involving a global variable.
- `perform_io_operation`: Calculates a sum from a table of values and executes an I/O operation using a function pointer from a global structure.
- `execute_io_operations`: Executes a function pointer from a global structure, modifies two values at specific offsets within a second structure, and then executes another function pointer from the global structure.

### f

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### ramdrive.device

- `return_error_code`: Returns a value representing a failure or error state.
- `parse_device_configuration`: Iterates through a list of device parameters to extract configuration strings and numeric values.
- `initialize_ramdrive_device_structure`: Copies a configuration block into a global buffer and calculates required memory sizes for device operations.
- `allocate_and_initialize_device_buffers`: Calculates a memory size based on several values and attempts to allocate a block of memory.
- `initialize_ramdrive_device`: Initializes a device structure by storing pointers to internal buffers and control fields, and links it into a global list.
- `call_internal_functions`: Moves a pointer to a1, calls a routine using a constant in d0, moves a different pointer to a1, calls another routine using a value from a table, and branches.
- `traverse_linked_list_and_interrupt`: Traverses a linked list to find a specific value and then calls a routine via a pointer.
- `modify_interrupt_state`: Performs arithmetic on a value in d3, calls a routine using a pointer derived from that value, and conditionally enables interrupts by writing to the INTENA register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_offset`: Clears a register and performs an unconditional branch to a specific offset.
- `store_word_at_offset`: Stores a 32-bit value into a memory location calculated by adding an offset to a pointer.
- `process_device_message`: Executes a function pointer from a global structure, looks up a value in a table based on a field in a message structure, stores the result back in the message, and conditionally calls another function pointer before a final cleanup routine.
- `return_constant_value_2`: Loads the value negative three into a register and returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `modify_string_and_execute_callback`: Decrements a value in a structure, manipulates a linked list, iterates through a string to replace a null terminator with a colon, and calls a function pointer.
- `traverse_string_pointer`: Moves a value from a pointer into a new location and enters a loop that checks for a null terminator in a string.
- `terminate_string_with_colon`: Iterates through a string until a null terminator is found, replaces it with a colon, and executes a function pointer.

### 13

- `return_constant_value_3`: Returns a 32-bit value of all ones.

### 7

- `return_maximum_value`: The routine returns a value consisting of all bits set to one.

### 8

- `return_all_ones`: Returns a value consisting of all bits set to one.

### 11

- `return_maximum_value_2`: This routine returns a value consisting of all bits set to one.
- `return_all_ones_2`: The routine returns a value consisting entirely of set bits.

### 12

- `return_error_constant`: returns a value consisting of all bits set to one.
- `process_font_data`: Performs a series of operations on data provided through pointers and registers.

### 16

- `return_negative_one`: Returns a value of all bits set to one.

### 18

- `return_maximum_value_3`: This routine returns a 32-bit value consisting of all bits set to one.

### 6

- `return_maximum_value_4`: Returns a value representing the maximum possible unsigned 32-bit integer.
- `execute_other_routine`: Performs a series of operations and returns a value of all bits set to one.

### 8

- `return_maximum_value_5`: The routine returns a value where all bits are set to one.

### 9

- `return_negative_one_2`: Returns a value of negative one.

### asl.library

- `return_negative_one_library`: As library code, this routine returns a value of negative one.
- `execute_conditional_library_calls`: As library code, this routine stores a value in a memory location and then performs a series of conditional calls to another routine based on the zero status of a register.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.

### diskfont.library

- `call_library_function_and_store_result`: As library code, this routine calls a function via a pointer in a table and, if the result is non-zero, stores a value at an offset from that result before calling another function.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: As library code, this routine calls two functions via pointers and, if successful, initializes a structure by setting a pointer and updating its members.
- `release_library_resources`: As library code, this routine calls two functions via pointers, one of which takes a parameter.
- `traverse_and_cleanup_list`: As library code, this routine iterates through a linked list to remove elements and, if certain conditions are met, performs a series of library calls and updates a list head.
- `manage_library_state`: As library code, this routine either updates a bit in a structure and returns, or performs a series of library calls and manipulates a linked list to store a value.
- `set_bit_and_clear_register`: As library code, this routine sets a specific bit in a memory location and clears a register.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_check_counter`: Decrements a 16-bit value at a memory location pointed to by a register and branches if the result is non-zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_font_resource_stream_2`: Iterates through a linked list of font resource entries to extract character metadata and string data.
- `prepare_jump_context`: Loads a pointer from a location relative to a register into another register and sets a small constant in a data register before branching.
- `parse_font_data_stream`: Iterates through a linked list of data structures to extract and copy character strings into a destination buffer.
- `reconstruct_font_glyph_stream`: Iterates through a font resource stream to extract and reassemble character metadata and string data.
- `parse_font_resource_stream`: Iterates through a font resource stream to extract character metadata and string data.
- `populate_font_glyph_table`: Iterates through a font resource stream to identify character metadata and populates a structured table with glyph offsets, character codes, and attribute flags.
- `parse_font_resource_data`: Iterates through a data stream to identify specific font resource headers and extracts character metadata into a structured table.
- `parse_font_glyph_metadata`: Iterates through a font resource stream to extract character metadata and string data.
- `return_zero_stub`: As part of library code, this routine returns zero and performs no operations on registers, memory, or hardware.
- `calculate_font_offset_3`: As library code, it performs arithmetic based on values in two structures and iteratively subtracts values from a table based on bitwise comparisons of bytes.
- `calculate_font_offset_2`: As library code, this routine iterates through a loop to subtract values from a register using an index into a table based on bitwise comparisons of byte pairs.
- `update_font_index`: This library routine performs a loop that uses a bitwise mask and a counter to subtract values from a register using an index into a table.
- `update_font_index_and_offset`: Shifts a value in a register, uses it as an index into a table to subtract a value from a register, and increments a counter within a loop that terminates after seven iterations.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_glyph_metadata_2`: Searches a font table for a specific entry and updates its internal properties, such as character counts and bitmask flags, based on a comparison loop.
- `search_font_table_entry`: Iterates through a font table by calling another routine to compare entries and updates a value in a global variable based on the result.
- `update_font_glyph_metadata`: Iterates through a table of font data to find a specific glyph based on a comparison loop and updates its associated metadata and properties.
- `clear_registers_and_branch`: Clears two word-sized registers, loads a constant byte into a third register, and performs a short branch.
- `update_font_metadata`: Iterates through a font data table to find the entry with the highest value, then modifies specific fields within the font structure, such as clearing words or incrementing counters, based on conditional checks of the font's properties.
- `calculate_font_offset`: This library routine calls another routine, performs a multiplication, and clears a word at a calculated offset in memory.
- `search_font_table`: Iterates through a table of font data to find a specific entry based on a comparison loop.
- `parse_disk_font_data`: Iterates through a data structure loaded from disk to populate a font table.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_pointers_and_branch`: As library code, this routine loads two values from the stack into registers and then performs a short branch.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `initialize_pointer_and_branch`: As library code, this routine loads a value from the stack into a register, clears another register, and performs a short branch.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_null_terminated_strings`: Compares two null-terminated strings and returns a value indicating their relative order.
- `test_and_branch_library`: As library code, this routine tests a byte at a pointer, branches if non-zero, and otherwise clears a register and branches.
- `compare_and_mask_byte`: Compares a byte from a pointer to a value in a register and performs conditional moves or bitwise masking based on the comparison result.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `load_and_branch_library`: As library code, this routine loads a pointer and a byte from the stack and performs an unconditional branch.
- `compare_and_move_pointer`: This library routine compares two bytes and, if they are not equal, moves a pointer from a register into another register before branching.
- `increment_and_compare`: This library routine increments a pointer, loads a byte from the new location, and branches based on the result.
- `process_library_pointer`: As library code, this routine saves registers, calls a subroutine, and then performs arithmetic on a pointer before branching.
- `compare_byte_at_pointer`: This library routine compares a byte at a memory location pointed to by a register with another value and, if they are not equal, moves that pointer into another register before branching.
- `compare_pointer_range`: This library routine decrements a pointer, compares it against a value in a register, and restores registers before returning.
- `search_and_increment_value`: As library code, it calls another routine twice with different constant arguments and increments the input value if either call returns a non-zero result.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:simintro.exe

- `parse_amigados_hunks`: Parses a memory buffer containing AmigaDOS hunk types to handle executable loading and relocation.
- `decrement_and_loop`: This routine decrements a value in a register in a loop until it reaches zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_copy_buffer`: Shifts data within a memory buffer by checking a flag and performing conditional byte-wise copies.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: This routine performs bitwise manipulations and shifts on data using a loop controlled by a counter.
- `perform_trap_and_combine_results`: This routine moves a value from memory, performs a bitwise operation, executes a trap, and then performs a bitwise OR between two registers.

### Sim Life (1993)(Mindscape)[cr FLT](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_runtime_environment`: Performs a complex sequence of hardware register initializations, recursive calls, and table population.
- `set_bit_in_register`: Sets a specific bit in a register using a bitmask.
- `search_null_terminated_string`: Iterates through a memory location until it finds a null byte and a specific terminator byte.
- `execute_recursive_calls`: This routine repeatedly calls itself in a loop until a specific value is reached.
- `update_table_with_accumulated_values`: Stores a 32-bit value split into two 16-bit parts into a table and increments the value in a loop.
- `fill_table_with_incrementing_values`: Populates a table with incrementing 16-bit values while copying data from another source.
- `initialize_data_tables`: This routine fills a table with incrementing values and then populates a subsequent section of memory with specific constant words.
- `parse_bytecode_stream`: Parses a stream of command bytes to perform conditional data copying.
- `clear_register_bits`: Performs a logical OR operation with zero on a register.
- `update_memory_pointer`: Decrements a global counter and updates a pair of memory locations with a value determined by a loop and a bitwise inversion of a flag.
- `pack_bits_into_record`: Distributes bits from two input words into specific positions within a memory record.
- `init_record_defaults_after_wait`: Waits until a status bit clears, then sets three longwords of a record to fixed defaults and stores a combination of two input words into the record.
- `clear_data_register_byte`: Performs a logical OR operation on a byte of a data register with zero.
- `clear_data_byte`: performs a logical OR operation on a data register with zero.
- `traverse_memory_list`: Iterates through a list of words in memory, checking specific bit conditions and comparing values before calling another routine and updating a pointer.
- `transform_nibbles_in_loop`: Performs bitwise manipulations and nibble rotations on two words through a fixed number of iterations.
- `rotate_bits`: Performs a bitwise rotation on a 16-bit value by shifting it left by twelve bits and right by four bits, combining the results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_dma_state`: Clears a table of DMA pointers and resets a set of global variables, then writes to the audio volume registers and the CIA port register and updates the DMACON register to enable DMA channels.
- `update_global_pointer_and_branch`: Increments a global pointer, compares a byte from memory to a constant, and conditionally clears a global variable before executing one of two subroutines based on a test of another global variable.
- `update_audio_sample_parameters_2`: Updates audio properties by parsing a command packet to modify sample offsets and lengths.
- `update_audio_sample_parameters`: Parses a command packet to update audio properties by calculating sample offsets and lengths from a lookup table.
- `update_clamped_value_by_nibble`: Extracts a nibble from a byte to select one of four operations: returning a value, subtracting a masked amount with a lower clamp, adding a masked amount with an upper clamp, or performing a complex update involving a table search and a secondary limit check.
- `copy_word_between_structures`: Copies a two-byte word from an offset within one memory structure to a different offset within another memory structure.
- `search_and_update_table`: Iterates through a table to find a value that meets a specific condition.
- `update_and_search_table_value`: This routine modifies a value in a structure by adding or subtracting a byte based on a flag, clamping it to a specific limit.
- `calculate_scaled_offset`: Performs bitwise extraction and arithmetic scaling on data from a structure to update a destination value and a local byte field.
- `process_input_command_packet`: Parses a command byte by extracting a nibble to determine an action type and using the remaining bits to update various state variables in a structure.
- `enable_dma_channel`: This routine modifies a byte in a structure and writes to a hardware register to enable DMA channels.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
