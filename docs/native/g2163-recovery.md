# Game 2163 (ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02163). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903792 top-level bytes in 30 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 279 of 279 named (0 author symbols), 279 of 279 described (8 by rule, 259 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1955 of 1955 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12450 of 12450 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 100517 |
| code | 39038 |
| data | 501802 |
| padding | 7459 |
| unused | 254976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| setmap | 13 | 653 | 31 |
| Install | 1 | 406 | 4 |
| Delete | 1 | 600 | 3 |
| Type | 2 | 363 | 12 |
| Assign | 1 | 938 | 7 |
| Copy | 2 | 1685 | 5 |
| MakeDir | 1 | 112 | 1 |
| SetMap | 13 | 627 | 33 |
| Format | 2 | 3440 | 38 |
| Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_BLIT | 11 | 250 | 5 |
| d | 1 | 3 | 1 |
| Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_CPU | 10 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:pdy! | 2 | 2022 | 64 |
| Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon_install | 7 | 250 | 17 |
| Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 584 | 21 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-long-keys): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 231 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_BLIT: Imploder (xfd) stream found by ancient scan (212376 -> 392204 bytes) but only 0 of 176 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_CPU: Imploder (xfd) stream found by ancient scan (210424 -> 380604 bytes) but only 0 of 176 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon: Imploder (xfd) stream found by ancient scan (4012 -> 6252 bytes) but only 2 of 4 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon_install: Imploder (xfd) stream found by ancient scan (7876 -> 14284 bytes) but only 0 of 7 sampled windows are in reference RAM: not used
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1993)(thalion)(de)[cr pdy][tr en-partial](disk 01 of 10)(a)/Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `normalize_keymap_name_and_lookup_2` (SetMap): The routine normalizes a keymap name to lower case, looks it up in a table obtained from the system, and prepares a small descriptor structure for the found entry. Kind: keymap name lookup. Pass path: not determined.

## Functions


### setmap

- `initialize_setmap_state`: The routine initializes a global state structure for the setmap module and prepares a command-line argument buffer.
- `restore_map_state`: The routine retrieves a structure via a global variable table, saves the incoming pointer into the structure and runs a series of callbacks stored in the global variable table, updating a field in the structure before each call.
- `call_indirect_table_entry`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `call_indirect_from_table`: The routine calls another routine through a pointer obtained by indexing a table with a fixed negative offset.
- `dispatch_handler_call`: The routine calls a function pointer taken from a global variable and then returns.
- `call_indirect_through_table`: The routine calls another routine indirectly via a pointer taken from a global variable table.
- `propagate_map_updates`: The routine conditionally calls another routine with several global variables and with its own parameter, passing a global variable by reference when a flag is set, and then unconditionally calls the other routine with a fixed global variable and with the incoming parameter.
- `store_parameters_and_call_helpers`: The routine stores its two parameters into two global variables and then invokes another routine with a fixed set of arguments that reference a global variable and constant values, followed by a second call to another routine with a single constant argument.
- `normalize_keymap_name_and_lookup`: The routine normalizes a keymap name string to lower case and looks it up in a table of installed keymaps, returning a handle and size information for the chosen map.
- `call_indirect_table_entry_2`: The routine calls another routine through an indirect pointer obtained by adding a constant offset to a global variable.
- `call_indirect_table_entry_3`: The routine calls another routine through a pointer obtained by indexing a global table with a fixed offset.
- `call_indirect_via_table`: The routine calls another routine indirectly through a pointer obtained by adding a constant offset to a global variable.
- `call_indirect_table_entry_4`: The routine calls another routine through a pointer obtained by indexing a global table with a fixed negative offset.
- `call_indirect_table_entry_5`: The routine calls another routine through a pointer taken from a global variable table.
- `call_indirect_table_entry_6`: The routine calls another routine through a pointer obtained from a global variable table.
- `call_indirect_handler`: The routine calls another routine indirectly through a pointer stored in a global variable.
- `call_indirect_table_entry_7`: The routine calls another routine through an indirect pointer taken from a global variable table.
- `call_indirect_from_table_2`: The routine calls another routine through an indirect pointer obtained by indexing a global table with a fixed offset.
- `call_indirect_from_table_3`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `call_indirect_table_entry_8`: The routine calls another routine through an indirect pointer obtained from a global variable and an offset.
- `call_indirect_table_entry_9`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed negative offset.
- `call_indirect_table_entry_10`: The routine calls another routine through a pointer obtained by indexing a global table with a fixed negative offset.
- `call_indirect_table_entry_11`: The routine calls another routine through a pointer taken from a global variable table.
- `invoke_handler_from_table`: The routine calls another routine obtained by indexing a global variable table with a negative offset.
- `call_indirect_table_entry_12`: The routine calls another routine via an indirect pointer taken from a global variable table.
- `call_indirect_table_entry_13`: The routine calls another routine through a pointer obtained from a global variable table.
- `invoke_indirect_handler`: The routine calls another routine through a pointer obtained from a global variable.
- `call_other_routine_with_stack`: The routine calls another routine, passing a global variable, its own parameter, and the address of an eight-byte stack slot.
- `call_indirect_table_entry_14`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `self_test_and_dispatch`: The routine runs a self test by calling a function through a table in a global variable, then computes a checksum of a 140-byte stack buffer by iterating until a zero byte is found and accumulating a 32-bit value.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Install

- `initialize_installer`: The routine is the program entry point for the installer.
- `toggle_bit_and_branch`: Toggles a bit in memory and checks if a value is zero, branching to a specific location if the value equals zero.
- `return_constant_all_ones`: The routine returns a constant value with all bits set.
- `copy_memory_block`: The routine copies a block of bytes from a source pointer to a destination pointer.

### Delete

- `initialize_game_state`: The routine clears a small stack buffer and checks a global variable; if the check fails it returns early.
- `delete_file_entry`: The routine attempts to delete an entry and reports whether it was deleted or not.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by two registers character by character.

### Type

- `initialize_memory_table`: The routine walks a linked list of tables to find the maximum size, then builds a small table of sequential values and copies a block of sixteen words between a temporary buffer and a global variable under certain conditions, returning a status code.
- `copy_table_to_stack`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer in A2 into the memory area pointed to by the first argument on the stack, then calls another routine via a pointer in A5.
- `initialize_structure`: Initializes several fields in a data structure pointed to by a1, clears specific memory locations, and sets up counters and flags based on conditional checks.
- `jump_to_handler`: Performs an unconditional branch to a specific memory address, effectively jumping to a handler routine based on program flow requirements.
- `initialize_global_flag`: Sets a 32-bit value at offset 612 of the structure in a2 to negative one and then branches to address 552.
- `update_memory_location`: Stores negative one into the 32-bit field at offset 532 of the structure in a1 and then branches to address 552.
- `load_and_process_byte`: Reads a byte from memory using an index calculated from two 32-bit values at offsets 220 and 224 of the structure in a1, then calls another routine with a pointer to a4 plus 928.
- `process_byte_value`: Reads a byte from memory using calculated index, compares it against specific values, and branches to different handlers based on the comparison result.
- `process_memory_operations`: Updates counters in a structure, performs conditional checks on memory locations, and calls handler routines based on program flow requirements while managing state variables.
- `initialize_and_execute`: Sets up global flags and executes a series of function calls with specific parameters, updating state variables based on conditional checks.
- `loop_continuation`: Unconditionally branches to address 696 and contains no other instructions.
- `execute_indirect_call`: Loads a long immediate value into a data register, uses another register as a pointer to access a table, calls a function through a pointer in a5, and then jumps to a location pointed by a6.

### Assign

- `parse_assign_command`: The routine parses command line arguments for the Amiga assign utility, validates device names and options, and prepares internal state for adding, subtracting, deferring or setting paths.
- `check_volume_mount`: The routine checks a volume reference stored in a global table and tests whether the volume is mounted.
- `copy_bytes_to_buffer`: The routine copies a number of bytes from a source pointer to a destination buffer managed through a pointer in A1.
- `process_assignment_queue`: The routine walks a linked list pointed to by the first argument, repeatedly invoking a handler via a pointer stored at a fixed offset from a second argument.
- `assign_object_to_slot`: The routine attempts to assign an object identifier to a slot list, checking for duplicates and capacity using a global variable and another routine.
- `copy_name_to_buffer`: The routine copies characters from a source string up to but not including a colon or the terminating null into a destination buffer, stopping early if the length reaches 30 characters, in which case it returns a pointer to a global variable; otherwise it null-terminates the destination and returns null.
- `compare_case_insensitive_strings`: The routine compares two byte strings character by character.

### Copy

- `initialize_copy_operation`: The routine sets up global state and arguments for a file copy operation, validates source and destination arguments obtained from a global variable, and prepares a buffer and control flags before delegating to another routine.
- `call_two_callbacks_with_flag`: The routine calls a callback stored in a table via a pointer in A1, then calls another routine from the same table and returns its result.
- `call_function_table`: The routine receives a count and a pointer to a table of function pointers.
- `check_bitmask_range`: The routine tests whether an input value lies within a numeric range and whether a specific bit in a bitmask is set, returning a flag indicating the result.
- `compare_case_insensitive_strings_2`: The routine compares two byte strings pointed to by two registers, treating characters between backtick and left brace as case-insensitive by mapping them down, and returns the signed difference of the first mismatching bytes or zero if the strings match up to the first null terminator.

### MakeDir

- `set_error_and_cleanup`: The routine initializes an error code and, if a library base is present, it clears a small stack buffer, opens a DOS library context, and iterates over a null-terminated list of strings.

### SetMap

- `parse_command_line`: The routine reads a null-terminated string from registers, trims trailing spaces, and parses the command line into arguments, handling quoted strings and *N/*E escapes, storing each argument pointer in a table and terminating the list with a null pointer.
- `restore_map_state_2`: The routine saves the incoming pointer into a global slot, then conditionally invokes two callback pointers taken from a global table when two global flags are non-zero, updating a global status word before and after each call, and finally returns the saved pointer.
- `call_two_function_pointers`: The routine calls two function pointers stored in a table referenced by a register, invoking each in turn and then returning.
- `check_and_run_callbacks`: The routine calls a function through a table entry, returns early if the result is non-zero, then sets up a local variable and invokes two more functions via a global table depending on flags.
- `invoke_indirect_handler_2`: The routine calls another routine through a pointer stored in a global variable.
- `invoke_indirect_handler_3`: The routine calls another routine through a pointer stored in a global variable.
- `call_indirect_from_table_4`: The routine calls another routine through an indirect pointer obtained from a global variable table.
- `call_indirect_table_entry_15`: The routine calls another routine through a pointer obtained by indexing a global variable table.
- `propagate_global_updates`: The routine conditionally calls another routine with several global variables and with its own parameter.
- `set_map_handler_pointer`: The routine stores its first two arguments into two global variables and then calls another routine with a fixed set of arguments that includes two references to a global variable, followed by a call to another routine with a single constant argument.
- `normalize_keymap_name_and_lookup_2`: The routine normalizes a keymap name to lower case, looks it up in a table obtained from the system, and prepares a small descriptor structure for the found entry.
- `indirect_call_via_table`: The routine performs an indirect call through a pointer derived from a global variable.
- `call_indirect_table_entry_16`: The routine calls another routine through a pointer taken from a global variable table.
- `invoke_table_entry`: The routine calls another routine through a pointer stored in a global variable table.
- `amiga_os_library_call_exec_findname`: The routine calls another routine taken from a table.
- `invoke_table_entry_2`: The routine calls another routine through a pointer taken from a global variable.
- `invoke_callback_from_table`: The routine calls another routine obtained by indexing a global table with an offset.
- `invoke_callback_from_table_2`: The routine calls another routine obtained by indexing a global table with a fixed offset.
- `invoke_callback`: The routine calls another routine obtained by indexing a global variable table with a fixed offset and then returns.
- `call_indirect_table_entry_17`: The routine loads an address from a global variable table, subtracts a constant offset, and calls the routine found there via an indirect jump.
- `call_indirect_table_entry_18`: The routine calls another routine through a pointer obtained by indexing a global variable table.
- `call_handler_from_table`: It calls another routine obtained by indexing a global variable table with an offset.
- `call_indirect_via_table_2`: The routine loads a global variable, adds a constant offset to obtain a pointer to a table entry, dereferences it to obtain a function pointer and calls another routine through that pointer.
- `invoke_indirect_through_table`: The routine fetches a function pointer from a global variable table by applying a fixed negative offset and then calls the retrieved routine indirectly.
- `call_handler_from_table_2`: The routine fetches a handler address from a global table via an offset and calls the handler.
- `invoke_callback_from_table_3`: The routine calls another routine through a pointer obtained by indexing a global table.
- `call_indirect_table_entry_19`: The routine calls another routine through a pointer taken from a table stored in a global variable.
- `invoke_callback_2`: The routine calls another routine obtained by indexing a global table with a fixed offset and then returns.
- `call_handler_from_table_3`: The routine calls another routine through a pointer obtained from a global variable table.
- `call_other_routine_with_stack_2`: The routine calls another routine, passing a global variable, the incoming parameter, and the address of an eight-byte stack slot.
- `call_indirect_table_entry_20`: The routine calls another routine through a pointer obtained from a global variable table.
- `compute_checksum_and_call`: The routine initializes a 32-bit accumulator to all ones and walks a 140-byte stack buffer byte by byte, folding each byte into the accumulator with a decrementing short counter.
- `store_byte_via_pointer`: The routine stores a byte from a register into the memory location pointed to by another register.

### Format

- `initialize_game_system`: The routine initializes a set of global state variables and opens several system resources, then checks for the presence of required libraries and loads a catalog file.
- `parse_and_validate_format_string`: The routine terminates two format strings at the first colon or null, clears a flag byte, calls another routine through a table indexed by the first word of a control block, stores the returned pointer, and if present dispatches a parser on two string buffers using a global variable and a table entry.
- `parse_and_validate_format_string_2`: The routine parses two colon-terminated fields inside a structure pointed to by a0, replaces the colons with null terminators, clears a byte at offset 35, calls several indirect methods via the vtable at a0, stores a result at offset 250 and, if non-zero, invokes another routine with the second field and a global variable.
- `search_table_for_match`: The routine initializes a result flag to zero and calls a library routine via an indirect pointer derived from a global variable.
- `check_format_state`: The routine checks a format state flag in a structure and, if clear, calls several function pointers from the structure to initialize fields, then calls another routine and updates the flag.
- `follow_linked_list_until_zero_or_callback`: The routine reads a pointer from a table entry and walks a singly linked list of pointers, stopping when the head is zero or when a callback stored via an offset from a structure returns zero.
- `iterate_linked_list_until_zero`: The routine walks a singly linked list via a pointer in A2, calling a function obtained from a table at an offset in A3 each iteration.
- `find_first_matching_entry`: The routine walks a linked list via a pointer stored in a global variable, dereferencing it repeatedly until the pointer becomes null or a callback stored in a table returns non-zero.
- `initialize_directory_cache_entry`: The routine initializes fields of a directory cache structure, storing a pointer to a global variable, clearing flags, copying a value from a source structure, and conditionally setting a pointer or calling another routine to obtain an identifier.
- `call_indirect_via_table_3`: The routine reads a pointer from a global variable, adds a constant offset to it, dereferences the result to obtain a function pointer, and calls that function.
- `execute_format_callbacks`: The routine calls two callbacks from a table pointed to by a global variable, then scans a zero-terminated string in memory, and finally calls a third callback from the same table.
- `call_indirect_via_table_4`: The routine reads a pointer from a global variable, adds a constant offset to it, and calls the routine found at that address.
- `invoke_callback_via_table`: The routine reads a function pointer from a table stored at an offset from the first argument and calls another routine with the second argument and the first argument.
- `copy_strings_to_buffers`: The routine copies two null-terminated strings from a structure pointed to by a0 into two local buffers, choosing the source for the first buffer based on a flag byte in the structure.
- `handle_dialogue_input`: The routine processes a dialogue structure pointed to by a0, invoking several library functions to manage the display and input, and loops waiting for a key or mouse event.
- `increment_pointer_and_store`: The routine receives a pointer in A0 and writes back a value derived from the pointer itself.
- `process_input_table`: The routine drives a table of function pointers stored in a global variable, repeatedly invoking its entries and copying null-terminated strings from the table into a buffer at a fixed offset in a structure pointed to by A0.
- `initialize_format_handler`: The routine sets up a format handler structure, initializes a directory cache entry, invokes callbacks via a table, processes an input table and parses two colon-terminated fields inside a structure pointed to by a0, replacing the colons with null terminators and clearing a byte.
- `reset_format_state`: The routine conditionally calls several other routines through function pointers stored in a structure pointed to by a0, then clears a set of fields in that structure and calls one more routine via a function pointer.
- `close_icon_window`: The routine closes an icon window by invoking functions through a pointer table and a global variable table.
- `invoke_callback_from_table_4`: The routine reads a pointer from a global variable, subtracts a constant from it, and calls the routine stored at the resulting address.
- `process_format_flag`: The routine checks a flag in the parameter block and then calls another routine.
- `check_table_entry`: The routine calls another routine three times and then invokes a function pointer taken from a table stored in memory.
- `parse_format_block`: The routine reads a format block from memory, selects a field depending on a flag byte, calls a helper to fill a temporary buffer, then uses a function pointer from a table to process the selected field and optionally a second field.
- `compute_scaled_value_with_conditional`: The routine reads several fields from a structure pointed to by a pointer, computes a difference between two integer fields, and selects a scaling factor based on the size of a 32-bit value, using library calls and helper routines for multiplication and normalization.
- `check_flag_and_recurse`: The routine checks a short value at a fixed offset from a pointer in A0.
- `check_and_process_format`: The routine checks a flag in a structure and, if set, calls a routine via a function pointer table, reads a string returned by another routine, scans it to its terminator, calls another routine via the table, then calls another routine and returns its result.
- `update_screen_region`: The routine computes a screen region based on input flags and a table entry, then writes a sequence of 32-bit values into a buffer pointed to by a global pointer, using the region bounds stored in the structure.
- `read_format_string_and_dispatch`: The routine reads a format string from a structure and copies it to a stack buffer before calling another routine, then dispatches based on a code returned by another routine, invoking further routines and returning specific codes.
- `find_entry_in_table`: The routine scans a global table of 32-bit entries in steps of eight bytes, comparing each entry to a value in D0 and, on a match, loads a result from the paired entry in the table.
- `call_indirect_via_table_5`: The routine calls another routine through a pointer obtained by indexing a global variable.
- `store_byte_via_pointer_2`: The routine stores a single byte from a register into memory via a pointer.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, then fills the remaining space up to a count with zero bytes.
- `copy_string_limited`: Copies a null-terminated string from a source pointer to a destination buffer, limited by a length in D0 or the source length.
- `copy_c_string`: The routine copies a null-terminated byte string from a source pointer to a destination pointer.
- `multiply_32bit_integers`: The routine takes two 32-bit values and returns their 32-bit product using 16-bit halves.
- `normalize_fixed_point_modulo`: The routine normalizes a fixed-point divisor and computes a modulo-like reduction of a dividend using shifts and arithmetic, returning a rotated fixed-point result.
- `normalize_and_divide_16bit`: The routine takes two 32-bit inputs and performs a division-like normalization of the first value by a 16-bit divisor derived from the second value, handling a special case when the high word of the second input is zero.

### Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_BLIT

- `initialize_blitter_state`: The routine calls another routine several times and then clears a word in memory.
- `decrement_until_minus_one`: The routine contains an empty loop that decrements an unused short value until it reaches -1, then returns.
- `perform_iterative_memory_updates`: Executes a loop seven times that performs indirect function calls and updates memory values by adding an integer to an existing value at a calculated offset.
- `clear_d0_register`: Sets the d0 register to zero by calling a small routine that performs a logical OR operation with zero.
- `copy_memory_block_optimized`: Copies a block of memory from a source to a destination using an optimized loop.

### d

- `reset_keymaps`: The routine is empty.

### Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:AM2_CPU

- `initialize_game_state_2`: The routine calls another routine several times and runs a small loop that invokes a function pointer taken from a global variable.
- `decrement_register_until_minus_one`: The routine contains an empty loop that decrements an unused short register until it reaches -1, then returns.
- `initialize_and_process_loop`: Initializes loop counters and processes data through multiple library calls while handling conditional branches based on loop iterations.
- `update_array_with_loop`: Checks if a value in d0 is negative, otherwise it calls another routine and, if d0 is not zero, enters a loop that adds a value from a pointer in a3 to a table and decrements a counter until it reaches zero.
- `process_loop_with_branch`: Checks a condition and branches if true, loads a pointer from another register into a1, calls another routine with a relative offset, and then branches unconditionally to a label.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `expand_bitstream_sequence_2`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting exp.
- `expand_bitstream_loop`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_constants_and_branch`: Sets specific values in two data registers and then branches unconditionally to another location.
- `set_constants_and_branch_2`: Sets specific values in two data registers and then branches unconditionally to another location.
- `initialize_loop_counters`: Sets the loop counter in register d0 to 3 and another counter in register d7 to 1, then branches to a routine that handles bitstream expansion.
- `expand_bitstream_sequence`: Decodes a bitstream-encoded sequence by checking if certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or or compressed data to a destination.
- `modify_d1_and_branch`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `expand_and_copy_bitstream_2`: Reads a bitstream-encoded sequence where certain bits determine the number of bytes to skip or copy, and writes the resulting expanded or decompressed byte sequence to a destination buffer.
- `expand_and_copy_bitstream`: Reads a bitstream or compressed-style bit-count-based sequence, expanding or skipping data based on a bit-mask, and copies the resulting bytes from a source pointer to a destination pointer while decrementing a counter in d0.
- `store_bytes_and_return`: Stores byte values from data registers into memory locations pointed to by an address register, then returns from the routine.

### Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:pdy!

- `initialize_global_tables`: The routine clears a large block of a global variable, then fills several tables in that global variable with pointer pairs and small constants, and calls another routine twice.
- `copy_words_loop`: Copies words from one memory location to another in a loop that decrements a counter until it reaches zero.
- `initialize_audio_and_input_state`: The routine validates a header pointer and copies fields from a table into a global structure, builds a pointer table from offsets, computes derived values, fills a buffer with a descending character pattern and zeros, clears several fields in the global structure, initializes a few global flags and writes to audio volume registers, CIA ports and interrupt control registers.
- `reset_audio_and_dma_state`: The routine clears four audio volume registers and three custom chip registers, then writes a constant to a fifth custom register.
- `process_audio_channel_data`: The routine updates four sets of audio channel state variables by copying values from RAM, invoking a table-driven callback for each set, and storing results back to RAM. It also decodes packed channel parameter bytes into period, control and volume fields, selects a callback via an index table, and programs the Amiga audio hardware registers and CIAB control register.
- `configure_dma_and_ciab`: The routine writes values to a global variable and to a table entry and then updates a pointer in the table.
- `set_audio_channels_and_flags`: The routine copies values from a table in memory to the four audio channel registers and to the custom chip registers, then writes a constant to a global variable and stores a pointer into a table entry.
- `process_entity_data`: Loads and processes entity data from a table based on index values in registers, updating structure fields with loaded values and handling conditional logic for different data paths.
- `unreached_routine_with_calls`: The routine contains 21 instructions and is marked as unreached because it lies in a gap decode as code.
- `decode_unreached_subroutine`: The routine is an unreached 26-instruction block that calls two other routines and is called by one unnamed routine.
- `decode_gap_routine`: The routine consists of eleven instructions and is marked as unreached.
- `unreached_routine_with_calls_2`: The routine is unreached and contains 30 instructions.
- `unreached_routine_with_calls_3`: The routine is marked as unreached by the disassembler and contains eleven instructions.
- `unreached_gap_decode_routine`: The routine is eight instructions long and is marked as unreached because it lies in a gap that decodes as code.
- `execute_external_routine`: The routine is 42 instructions long and is entered from outside the static flow.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `handle_input_byte`: The routine reads a byte from a pointer and uses its bits to select a case, then performs small manipulations on memory pointed to by several registers, such as setting or clearing bits in a global variable, adjusting short values with clamping, storing nibbles into fields, and copying or updating values between buffers.
- `clamp_short_field`: The routine reads a byte from an offset within a structure pointed to by a4, masks the low nibble, adds it to a short at another offset in the same structure, clamps the sum to a maximum, and writes the result back to that short field.
- `check_joy_fire_and_clear_flag`: The routine tests a bit in a byte pointed to by an uninitialized pointer and, if set, sets a bit in a global variable; otherwise it clears that bit in the global variable.
- `clear_global_flag`: The routine clears a single bit in a global variable by ANDing it with a mask, then returns.
- `clear_word_if_conditions_met`: Clears a 16-bit word at a specific offset in a structure if a byte at a different offset does not equal one and another value shifted right is non-zero.
- `copy_word_if_flag_set`: The routine checks a byte at an offset from a pointer.
- `decode_and_store_nibbles`: The routine checks a flag byte in a structure and, if the flag is not set, conditionally clears a word field based on a table pointer and then extracts the high and low nibbles from a byte field in the structure, storing each non-zero nibble into two separate byte fields.
- `decode_and_store_nibbles_extended`: Checks a flag byte in a structure and, if the flag is not set, clears a word field, copies a word from one offset to another, and then extracts the high and low nibbles from a byte field in the structure, storing each non-zero nibble into two separate byte fields.
- `clamp_byte_field`: The routine reads a byte from an offset within a structure pointed to by a register, limits its value to a maximum, and writes the clamped value back to another offset in the same structure.
- `copy_byte_with_default`: The routine reads a byte from an address derived from a pointer plus an offset, substitutes a non-zero default if the byte is zero, and writes the resulting byte to an address derived from another pointer with an offset.
- `update_structure_counters`: Stores a byte from a register through a pointer into a word field of a structure and increments two short counters in the same structure.
- `update_structure_and_pointer`: Reads a byte from a pointer plus an offset, subtracts 1, and if the result is negative sets it to 128; it then stores this byte at an offset, calculates a new long value using a table and stores it, sets a word to 64, and increments two short counters in a structure.
- `copy_state_entries`: The routine copies four word-sized values from four source globals to four destination globals, invoking another routine and a table-driven callback before each copy, then stores the copied values into fields of an unspecified structure pointed to by an uninitialized pointer, and finally writes a pointer to another routine into a global table entry and sets a global variable to a constant.
- `dummy`: This routine does not contain any instructions; it is empty.
- `process_global_table_entry`: The routine indexes a global variable table with a byte from an input pointer and extracts fields from the entry.
- `update_timer_and_pointer`: The routine updates a byte field by adding an offset from a global table using an index stored in the structure, then checks the result for overflow.
- `update_character_ability_index`: The routine increments a short counter stored in a structure and, when the counter matches an input value, copies a word from a register into two locations derived from two pointers, updates a pointer to a table entry selected by the counter, and propagates fields from that entry into the structure.
- `update_ability_index_partial`: Increments a short counter in a structure and, if it matches a value derived from a byte at offset 5, copies a word from a register into two locations derived from two pointers.
- `increment_and_clamp_counters`: Increments a 16-bit value at offset 18 of a structure and, if it reaches or exceeds a value derived from a byte at offset 5, clears the 16-bit values at offsets 12 and 54.
- `select_entry_from_table`: The routine checks a flag byte and a lookup in a global variable and then indexes a table built from two global lookups.
- `copy_word_from_offset_to_offset`: The routine calls another routine twice and then copies a 16-bit value from an offset within a structure pointed to by an uninitialized pointer to another offset within the same structure.
- `ambermoon_de`: A small routine that reads a byte and two words from a structure pointed to by an argument, compares and possibly updates a word field and a related field, then copies the word to another location if a flag in the structure is clear.
- `update_party_member_stat`: The routine updates a two-byte field in a party member structure using a value derived from two inputs and a table lookup, and writes the same value to two other fields in the same structure.
- `update_nibble_adjusted_counter`: The routine reads a byte from an offset in a structure pointed to by a register, extracts its high and low nibbles, and adjusts a 16-bit counter at another offset by subtracting the low nibble or adding the high nibble depending on the high nibble.
- `add_byte_step_capped_at_ceiling`: Adds an unsigned byte from a record to a 16-bit field, limits the sum to a fixed ceiling, and writes the value to the record and to the output.
- `decrement_a4_word_clamp_and_copy`: Subtracts the byte at offset 5 of a4 from the word at offset 2, clamps it to a minimum of 108, copies it to offset 6 of a3, and returns.
- `calculate_adjusted_value`: Loads byte values from specific offsets in a structure, performs bit manipulation and arithmetic operations, then stores the calculated result back to memory.
- `load_and_test_condition`: Loads a word value from memory, performs bit shifting and masking operations, then tests a condition before proceeding with further execution.
- `shift_and_compare`: Shifts a byte value left by three bits, tests a condition in memory, and performs conditional subtraction to set a result register.
- `copy_and_branch`: Copies the value in data register d2 to data register d3 and then unconditionally branches to address 16776.
- `calculate_and_update_value`: Loads a pointer to a table, reads a byte from that table using an index, multiplies it with a word value, shifts the result right by seven bits, adds it to a word value from another location, and stores the result in a global variable.
- `subtract_and_update_a4_byte`: Subtracts d3 from d2, copies the result to offset 6 of a3, and adds the value in d1 shifted left by 2 to the byte at offset 19 of a4.
- `calculate_scaled_value`: Computes a scaled value by loading data from memory locations, performing bitwise operations and arithmetic calculations, then stores the result in a destination register.
- `shift_and_conditional_subtract`: Shifts a byte value left by three bits, tests a memory condition, and performs conditional subtraction to compute a final result value.
- `transfer_and_continue`: Copies the value in data register d2 to data register d3 and then unconditionally branches to address 16894.
- `clamp_and_store_result`: Loads a pointer to a table, reads a byte from that table using an index, multiplies it with a word value, shifts the result right by six bits, adds or subtracts it from a word value depending on a condition, clamps the result between zero and sixty-four, and stores the final value in a global variable.
- `subtract_clamp_and_update_a4`: Subtracts d3 from d2, clamps the result to a minimum of 0, stores it in offset 54 of a4, and adds the value in d1 shifted left by 2 to the byte at offset 19 of a4.
- `align_color_components`: The routine steps through two parallel word tables and adjusts the destination word so its high, middle and low nibbles converge toward the source word.
- `increment_counter_and_signal`: The routine increments a global counter and then calls another routine.
- `fill_and_copy_struct_table`: The routine fills a table of structures with constant byte values from RAM, then copies a second table of structures over that table.
- `copy_font_glyphs`: The routine copies a series of glyphs from a source table into a destination buffer using an index driven by a global pointer.
- `scale_short_table`: The routine multiplies each 16-bit entry of a global short table by ten, wrapping around the table via a circular pointer, then multiplies each entry by four until a sentinel value is encountered.
- `clear_party_inventory`: The routine clears a large block of long words belonging to a global variable, zeroing the memory region starting at an offset from the base.
- `initialize_message_struct`: The routine waits until a flag byte in the structure is cleared, then writes constant values into several fields of the structure, copies the address of a global variable into two integer fields, and stores a zero and a constant word.
- `rotate_and_fill_table`: The routine rotates three 32-bit values stored in a global variable and then writes the rotated value repeatedly into a table of 32-bit entries.
- `rotate_3x3_matrix`: The routine updates three global 16-bit offsets by wrapping them, then iterates three times applying a fixed 3x3 matrix multiplication with 16.16 fixed-point arithmetic to a three-element vector stored in a global variable, and writes the transformed values back to the same global storage.
- `clamp_table_values`: The routine waits for a flag in a structure, initializes several fields, then walks a short-indexed table in a global variable.
- `set_tile_window_parameters`: The routine computes a size and offset from two input values and a global variable, then writes several short and int fields into a structure pointed to by a pointer.

### Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon

- `initialize_game_state_3`: The routine calls another routine several times and then runs a small loop that invokes a function pointer taken from a global variable.
- `decrement_until_minus_one_2`: The routine contains an empty loop that repeatedly decrements an uninitialized short in D6 until it equals -1, then returns.
- `execute_with_cache_clear`: Executes a sequence of operations with loop counters, performs memory access and calculations, and calls a cache clearing function when specific conditions are met.
- `conditional_accumulate_loop`: Performs a conditional loop that accumulates values from a table, stores results in memory, and continues until a counter reaches zero.
- `conditional_copy_loop`: Checks a condition and branches if true, loads a pointer from another register into a1, calls another routine with a relative offset, and then branches unconditionally to a label.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_loop_2`: Compares a pointer in a2 with a pointer in a6 and, if not equal, copies four bytes from a source pointer in a2 to a destination pointer in a1 while decrementing d0 by two in a loop, otherwise it returns.
- `decode_bitstream_loop`: Processes bits from a source buffer to decode values and copy bytes to a destination, managing counters and pointers while performing bit manipulation operations.
- `expand_bitstream_loop_2`: Sets d7 to 2, and if d1 doubled is not zero, it decrements a0, adds d1 doubled and d0 to d0, and loops using d7; otherwise, it increments d0 by 6, sets d7 to 3, and branches.
- `set_constants_and_branch_3`: Loads immediate values into two registers and performs an unconditional branch to another location in the code.
- `load_and_execute`: Loads immediate values into registers and executes a branch instruction to a specific memory address.
- `initialize_and_branch`: Sets up initial register values and performs an unconditional branch to another code location.
- `decode_bitstream`: Reads bits from a buffer and decodes them into values, managing pointers and counters while performing bit manipulation operations.
- `modify_d1_and_branch_2`: Multiplies d1 by two and branches if the result is non-zero, otherwise it decrements a pointer in a0, adds d1 to d1 with carry, adds d2 to d2 with carry, and jumps to another routine.
- `decode_bits_and_copy_2`: Reads bits from a source buffer and copies decoded data to a destination buffer while managing counters and pointers.
- `decode_bits_and_copy`: Decodes bits from a source buffer and copies decoded data to a destination buffer while managing counters and pointers.
- `copy_four_bytes_loop_3`: Copies a byte from d0 to the address pointed to by a4, decrements a4, copies a byte from d1 to the address pointed to by a4, decrements a4, copies a word from d2 to the address pointed to by a4, decrements a4, and then returns.

### Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:Ambermoon_install

- `initialize_installer_state`: The routine runs a small loop that calls another routine several times and then clears a word in memory.
- `decrement_until_minus_one_3`: The routine contains an empty loop that decrements an uninitialized short in D6 until it equals -1, then returns.
- `modify_table_elements_2`: Clears d5, increments d6, calls a routine, increments a pointer in a1, calls another routine, sets a6 to that pointer, and then enters a loop that increments d5 and checks if it equals 3, calling a routine if a certain condition is met.
- `conditional_accumulate_loop_2`: Processes data through a conditional loop structure, accumulating values and performing memory operations based on control flow conditions.
- `conditional_copy_and_branch`: Tests a condition and branches to a specific location if the condition is met, otherwise continues with normal execution flow.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes`: Copies four bytes from a source buffer to a destination, decrementing the source pointer after each byte copy, and continues until a counter reaches zero.
- `decode_bitstream_loop_2`: Processes bits from a source buffer to decode values and copy bytes to a destination buffer, managing pointers and counters while performing bit manipulation operations.
- `bitstream_decode_loop`: Decodes bits from a buffer through a loop structure, accumulating values in registers while managing bit positions and counters.
- `set_decode_params`: Sets the bitstream decoding parameters by loading immediate values into data registers d0 and d7, preparing for a call to the decode_bitstream routine.
- `set_decode_params_2`: Configures bitstream decoding parameters with different immediate values in data registers d0 and d7 compared to the first routine, then branches to the decode_bitstream routine.
- `set_decode_params_3`: Loads specific immediate values into data registers d0 and d7 for bitstream decoding configuration, followed by a branch to the decode_bitstream routine.
- `decode_bitstream_2`: Processes bits from a source buffer to decode data and manage pointers, with conditional branching based on bit values and table lookups.
- `shift_and_accumulate`: Shifts a byte value left by one bit position and accumulates the result into another register.
- `decode_bitstream_2_2`: Reads bits from a buffer, accumulates them into a word, and performs bit manipulation and table lookups to decode a stream of data.
- `decode_bitstream_loop_3`: Processes bits from a source buffer to decode values and copy bytes to a destination, managing counters and pointers while performing bit manipulation and conditional copying.
- `copy_four_bytes_loop_4`: Copies a byte from d0 to the address pointed to by a4, decrements a4, copies a byte from d1 to the address pointed to by a4, decrements a4, copies a word from d2 to the address pointed to by a4, decrements a4, and then returns.

### Ambermoon (1993)(Thalion)(De)[cr PDY][tr En-partial](Disk 01 of 10)(A).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: This routine executes five instructions and performs a call to an Amiga OS library function.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_system_resources`: Performs a complex sequence of tasks including checking task status, managing memory, polling hardware, and handling message locks and library pointers.
- `execute_nested_library_calls`: Retrieves a pointer from a global variable and performs a series of conditional calls to functions within that object, potentially releasing a lock or jumping to a new location.
- `examine_locked_object_field`: Locks an object through the second library and examines it.
- `dispatch_table_entry_by_index`: Looks up an index through a library call; if the index is zero, it releases a lock and returns minus one, otherwise it calls a routine from a table and releases the lock.
- `execute_indirect_library_calls`: Calls a function via a pointer stored in a global variable and, if the result is non-zero, updates that global variable and performs three subsequent indirect calls before calling another function from the updated base.
- `execute_conditional_library_calls`: Calls a function via a pointer in a global variable and, if the result is non-zero, updates the global variable and performs a series of indirect calls, including a conditional call based on a returned value.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from a global variable offset and executes it.
- `search_terminated_16bit_table`: Iterates through a table of 16-bit values until it encounters a negative one marker or a value matching the data register.
- `measure_string_length`: Counts the characters of a null-terminated string addressed by the address register and returns the count in the data register.
- `wait_for_hardware_status`: Sets a global variable to zero and enters an infinite loop if a specific bit in a memory location is set, while polling a hardware register.
- `set_interrupt_request`: Stores a constant value into a global variable that is used to modify the interrupt request register.
- `probe_floppy_drive_entry`: When a state cell is zero, it attempts a lock through the second library and records whether a returned status equals a fixed code.
- `wait_for_message`: Checks a value in a global structure and, if zero, waits for a message and updates a global pointer.
- `release_message_lock`: If a global variable is non-zero, it calls a routine to release a lock and then calls another routine to reply to a message.
- `open_library_pointer`: Calls a function through a pointer in a global variable and stores the result in that global variable.
- `close_library_pointer`: If a global variable is non-zero, it calls a function through a pointer within that global variable.
- `store_current_task_pointer`: Calls the task lookup entry of the system library and stores the returned task pointer in a global.
- `store_adjusted_char_then_call`: If a global variable is zero, it adds sixty-four to a character from a pointer and stores it in a fixed byte cell before calling another routine; otherwise, it simply calls that routine.
- `copy_null_terminated_string`: Copies characters from a source pointer to a destination pointer one at a time until a zero byte is encountered.
- `set_flag_if_field_below`: Sets a global flag to minus one when a 16-bit field of a record, reduced by thirty-six, is negative, and to zero otherwise.
