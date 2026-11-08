# Game 1808 (cruise for a corpse (1991)(u. s. gold)[cr skr](disk 1 of 5)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01808). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (970016 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 661 of 661 named (0 author symbols), 661 of 661 described (14 by rule, 607 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4768 of 4768 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18642 of 18642 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 25144 |
| code | 58328 |
| data | 831846 |
| padding | 5034 |
| unused | 49664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:Skid_Row_Cracked_Cruise | 1 | 57 | 7 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:skidrow | 2 | 24 | 4 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:endcli | 2 | 117 | 2 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:cruise | 2 | 233 | 8 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:pause | 2 | 3 | 1 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:cd | 2 | 117 | 2 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:add21k | 3 | 653 | 17 |
| Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 61 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15742 | 514 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1616 | 41 |

## Cases handled

- cruise for a corpse (1991)(u. s. gold)[cr skr](disk 1 of 5)[a]/Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:d1: high-entropy data file (7.69 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 38 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3767 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 712 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (15 bytes change between runtime segment an address and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- cruise for a corpse (1991)(u. s. gold)[cr skr](disk 1 of 5)[a]/Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:cruise: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:Skid_Row_Cracked_Cruise

## Copy protection found

- `manipulate_color_register` (runtime segment an address (play-long-joy)): Performs bitwise operations involving a global variable and a register to find a specific bit pattern. Kind: checksum/data integrity trap. Pass path: the loop completes normally without triggering a crash or infinite loop, allowing execution to proceed to the next instruction.

## Functions


### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:Skid_Row_Cracked_Cruise

- `copy_entry_block_low_memory`: Entry point that copies a block of 156 words from the program's own image into low memory starting at address 256, then returns.
- `copy_ok_tagged_entry_words`: Checks that the text at the current position reads ok and that a 0 appears at one of two offsets.
- `copy_four_table_entry_words`: Copies four words from a table entry to the destination, replacing the sentinel minus two with minus one, and stores the count of valid copies in the next word.
- `copy_three_table_entry_words`: Writes a minus one word, then copies three table words the same way, replacing the sentinel with minus one, and stores the count of valid copies in the following word.
- `copy_two_table_entry_words`: Writes a minus one word, copies two table words with the same sentinel handling, and stores the count of valid copies in the following word.
- `copy_one_table_entry_word`: Writes a minus one word, copies one table word with the same sentinel handling, and stores the count in the following word.
- `store_sentinel_and_count_word`: Writes a minus one word, clears a global flag and stores the input register value in the following word.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:skidrow

- `call_library_entry`: At program entry it calls one routine, and if the zero flag is clear it calls a second routine and then a function pointer taken from a table.
- `open_dos_library`: Calls the exec OldOpenLibrary entry through a library table pointer and stores the returned handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a destination pointer is not null, copies sixteen longwords from a table at a fixed offset into it, then calls one routine through a stored pointer.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:cruise

- `load_executable_hunks`: Program entry that walks executable hunk records by comparing their type codes, allocates memory for each hunk, copies its data and adds offsets to relocation targets.
- `count_down_delay_loop`: Counts down the D1 register in a loop until it wraps past zero, producing a timed delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_bitstream_block`: Unpacks a block backwards into memory by reading bits from a stream and copying byte runs and literal bytes until the destination reaches its start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bit_buffer_with_extend`: Repeatedly shifts a bit buffer by a count held in a register, carrying bits through the extend flag and refilling from input words when the buffer empties.
- `execute_trap`: Triggers a software interrupt via a trap instruction.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:pause

- `wait_until_flag_bit_clears`: Loops while bit 6 of a global flag word is set, then returns.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:cd

- `allocate_and_copy_entry_block`: Program entry that scans a chain of table entries for the largest value, writes an incrementing sequence of longwords into a newly allocated block, and copies a block of longwords into another area.
- `copy_block_and_call`: Copies a fixed number of words from a memory location to a destination and then calls a function pointer.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:add21k

- `clear_block_and_dispatch_result`: Calls one routine, clears a block of 25 longwords and stores values from a global table into it.
- `open_system_libraries_or_alert`: Opens the dos, graphics and intuition libraries by name, showing an alert if one fails to open.
- `stub_return_only`: Its decompiled body only returns, and it is called by one routine, so it has no observable behaviour.
- `open_dos_streams_into_record`: Allocates a record and fills its fields.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `split_command_line_into_tokens`: Copies a string into a buffer while splitting it into tokens at whitespace, honoring double quotes and doubled quotes, and stores a pointer to each token in an allocated table ending with a null pointer.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_limited_string`: Finds the end of the first string, then appends characters from the second string up to a count held in a register, stopping when the second string ends.
- `open_icon_library_and_release`: Opens the icon library by name and, when that succeeds, calls library entries to obtain values that are stored in fields of a record, then releases the library.
- `copy_string_or_pad`: Copies characters from a source string into a destination up to a count, and once the source ends it fills the rest of the count with zero bytes.
- `copy_string_or_pad_2`: Copies characters from a source to a destination up to a specified count, or fills the destination with null bytes if the count is zero.
- `calculate_string_length`: Advances through a zero-terminated string to its terminator and returns the number of bytes before it.
- `copy_string_simple`: Copies characters from a source string into a destination buffer through the terminating null byte, and returns the destination address.
- `release_list_entries_and_call`: Walks a linked list, calling a function pointer stored in each entry and advancing to the next entry, then calls an optional stored routine.
- `release_locks_and_close_resources`: Cleans up in sequence: when a pointer is set it calls a routine for each entry up to a count, then conditionally releases a lock, restores a stored value and calls several library entries.
- `stub_return_only_2`: A single return in a gap that the decoder reached as code.
- `close_indexed_dos_handle`: Returns minus one and sets an error code when the index is out of range or the entry is empty.

### Cruise for a Corpse (1991)(U. S. Gold)[cr SKR](Disk 1 of 5)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: A two-instruction routine that returns minus one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_bit_mask`: Performs a bitwise OR operation on a byte at a specific offset from a pointer.
- `iterate_word_list`: Iterates through a list of words pointed to by a register until a negative value is encountered.
- `modify_memory_via_pointer`: Performs a bitwise OR operation on a byte located at an offset from a pointer stored in a register.
- `set_memory_bits`: Updates a byte in memory by applying a bitwise OR with a constant value.
- `traverse_word_table`: Loops through a table of short integers until a terminator is found.
- `scan_short_array`: Reads a sequence of short integers from a pointer until a negative value is reached.
- `apply_bit_mask`: Sets specific bits in a byte located at a fixed offset from a base address.
- `update_byte_bits`: Uses a bitwise OR to modify a single byte at a memory location relative to a register.
- `process_word_sequence`: Steps through a list of short values until a negative integer is detected.
- `loop_through_shorts`: Increments a pointer through a list of short integers until a negative value is found.
- `or_memory_byte`: Modifies a byte at a specific offset from a pointer using a bitwise OR operation.
- `modify_byte_via_offset`: Performs a bitwise OR operation on a byte located at a specific offset from a pointer held in a register.
- `modify_byte_via_offset_2`: Updates a byte at a fixed offset from a pointer using a bitwise OR operation.
- `modify_byte_via_offset_3`: Applies a bitwise OR to a byte stored at a constant offset relative to a register value.
- `iterate_through_table`: Traverses a table of short integers until a negative value is encountered.
- `modify_byte_via_offset_4`: Sets specific bits in a byte at a defined offset from a pointer using a bitwise OR.
- `modify_byte_via_offset_5`: Uses a bitwise OR to modify a single byte at a constant offset from a pointer.
- `modify_byte_via_offset_6`: Performs a bitwise OR on a byte at a specific offset relative to a pointer in a register.
- `modify_byte_via_offset_7`: Modifies a byte at a fixed offset from a pointer by applying a bitwise OR operation.
- `iterate_through_table_2`: Loops through a sequence of short values, incrementing a pointer until a value less than zero is found.
- `modify_byte_via_offset_8`: Updates a byte at a specific offset from a pointer using a bitwise OR operation.
- `set_byte_bit`: Performs a bitwise OR operation on a byte at a specific offset from a pointer in a register.
- `set_byte_bit_2`: Modifies a byte by applying a bitwise OR using an offset from a register.
- `iterate_pointer_list`: Traverses a list of short integers using a pointer until a negative value is encountered.
- `iterate_pointer_list_2`: Increments a pointer through a sequence of short values while checking for a termination condition.
- `iterate_pointer_list_3`: Loops through a table of short integers by incrementing a pointer until a negative value is found.
- `set_byte_bit_3`: Updates a byte at an offset from a register using a bitwise OR operation.
- `set_byte_bit_4`: Applies a bitwise OR to a byte located at a fixed offset from a pointer.
- `iterate_pointer_list_4`: Walks through a series of short values via a pointer until a negative value is reached.
- `iterate_pointer_list_5`: Processes a list of short integers by incrementing a pointer until a negative value is detected.
- `set_byte_bit_5`: Uses a bitwise OR to set bits in a byte at a specific offset from a register.
- `modify_byte_via_offset_9`: Performs a bitwise OR operation on a byte located at a fixed offset from a pointer in a register.
- `iterate_through_table_3`: Iterates through a table of short integers using a pointer until a negative value is encountered.
- `apply_bitmask_to_byte`: Updates a byte at a specific offset from a register by applying a bitwise OR mask.
- `set_byte_bits`: Sets specific bits in a byte stored at a constant offset from a pointer.
- `or_byte_with_mask`: Uses a bitwise OR to modify a byte at a calculated memory location.
- `update_offset_byte`: Modifies a single byte at a fixed offset from a register using a bitwise OR operation.
- `bitwise_or_at_offset`: Performs a bitwise OR on a byte at a specific offset from a pointer in a register.
- `traverse_short_list`: Steps through a list of short values via a pointer until a value less than zero is found.
- `scan_table_for_negative`: Reads short integers from a table and increments the pointer until a negative integer is reached.
- `process_short_array`: Loops through an array of short integers, checking each value until a negative terminator is met.
- `modify_memory_byte`: Performs a bitwise OR operation on a single byte located at an offset from a pointer held in a register.
- `iterate_signed_list`: Iterates through a list of signed words until a negative value is encountered.
- `traverse_word_table_2`: Steps through a table of short integers and stops when a value less than zero is found.
- `scan_short_array_2`: Reads a sequence of signed shorts from memory until a negative terminator is reached.
- `process_signed_elements`: Loops through a series of words, incrementing a pointer until a negative integer is detected.
- `find_negative_terminator`: Traverses a memory block of signed shorts to find the first negative entry.
- `read_word_sequence`: Increments a pointer through a list of words while checking for a negative value.
- `adjust_character_case`: Subtracts thirty-two from a byte if it falls within a specific range, effectively converting uppercase to lowercase.
- `call_other_routine`: Calls another routine with a modified pointer and a parameter.
- `find_character_in_string`: Searches for a specific byte within a null-terminated string and returns its location.
- `clear_and_call_functions`: Fills a memory block with zeros using a loop and then calls two different routines based on the results of a comparison and a function return value.
- `initialize_stack_range`: Sets up a range of memory starting from a provided pointer and calls another routine, then clears a byte at a specific offset from the input pointer.
- `concatenate_strings`: Finds the end of a null-terminated string and copies the contents of a second string to that location.
- `calculate_mixed_product`: Performs a series of arithmetic operations involving bit shifts and multiplications on two 32-bit values.
- `populate_data_structure`: Calls several routines to retrieve values and then stores an integer, a byte, and other data into specific offsets of a structure.
- `reset_structure_fields`: Sets specific bytes and longwords to fixed values within a structure and calls two other routines.
- `assign_structure_values_2`: If a pointer is non-zero, it calls a routine to obtain a structure and then assigns a byte, a word, and an integer to specific offsets within it.
- `reset_structure_and_call`: If a pointer is non-zero, it sets three fields in a structure to specific values and calls another routine.
- `execute_recursive_call`: The routine performs a single recursive call to itself before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_return_only_3`: Its decompiled body only returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_initialize_memory`: Clears a block of memory with zeros using a loop and then performs several assignments to global variables and function pointers.
- `initialize_system_state_2`: A large routine that performs system initialization by updating flags, processing resource tables, executing callback sequences, and initializing font and engine states.
- `update_system_flags`: Checks a condition based on a string, then performs a series of calls to update several global status flags and bitwise shifts a value in a global variable.
- `execute_buffered_commands`: Copies a block of data from a global table to a local stack and uses that data to perform several function calls with various parameters.
- `copy_string_and_recurse`: Copies a null-terminated string from a global variable to a local stack buffer and then calls another routine with that buffer.
- `check_memory_and_copy_string`: Calls a function via a pointer to check a condition; if the result is zero, it copies a null-terminated string from a global variable to a local stack buffer and passes that buffer to another routine.
- `dereference_pointer`: Returns the value stored at the location pointed to by the provided parameter.
- `store_split_word`: Splits a 32-bit value into two bytes and stores them in a destination buffer.
- `increment_and_fetch_byte`: Increments a counter at a specific offset within a table and returns a single byte from that table.
- `increment_and_fetch_word`: Increments a counter at a specific offset within a table and returns a 16-bit word from that table.
- `push_word_to_stack`: If a counter is below a specific limit, increments the counter and stores a 16-bit value into a table.
- `pop_word_from_stack`: If a counter is greater than zero, decrements the counter and returns a 16-bit value from a table.
- `process_resource_handle`: Obtains a resource through a function call and attempts to use it with another function, potentially releasing it if the operation fails.
- `process_resource_and_callback`: Obtains a resource through a function call, updates a pointer with the result, and executes a callback function if the resource is non-zero.
- `execute_callback_and_function`: Checks if a parameter is non-zero, executes a function pointer stored at an offset within that parameter, and then calls another function pointer with the parameter.
- `execute_multiple_callbacks`: Checks a parameter for non-zero values at specific offsets to execute a primary callback, a secondary callback from a global table, and a final function call.
- `update_value_and_call`: Stores a 16-bit value into a buffer at a specific offset and then calls a function pointer from a global table.
- `return_offset_value`: Calls another routine and returns a 32-bit value located at an offset from the passed parameter.
- `set_flag_and_call`: Sets a 32-bit value at an offset from a register to either zero or one based on a condition and then calls another routine.
- `clear_and_call`: Sets a 32-bit value at an offset from a register to zero and then calls another routine.
- `handle_error_message`: Checks a flag in a parameter and, if set, calls two functions to process a string related to a disk error.
- `update_structure_fields_3`: Stores several values into a structure at specific offsets, including a shifted parameter and a value returned from another routine.
- `process_resource_and_callback_2`: Obtains two resources through function calls and then iterates four times to process a global table, updating a status array based on the results.
- `execute_loop_callbacks`: Iterates through a table to call routines if a corresponding status value is zero, then calls a function pointer from a table twice.
- `process_structure_data`: Retrieves a pointer from a table and uses it to perform multiple operations involving several values and a parameter.
- `copy_string_to_buffer`: Copies a string from a source pointer to a destination pointer and appends a null terminator.
- `execute_callback_list_2`: Calculates the length of a null-terminated string and uses that value as a counter to execute a series of function pointers from a table.
- `execute_callback_list`: Iterates through a list of function pointers, executing each one, and finally calls a different function and returns its result plus six.
- `compare_disk_strings`: Traverses a linked list to find a matching entry based on specific data values and then compares a null-terminated string from a global variable against a local buffer.
- `initialize_data_structure`: Writes a series of specific sixteen-bit values into a block of memory pointed to by a value retrieved from a global offset.
- `update_data_fields`: Calls another routine and then writes several specific values into a memory block pointed to by a global offset.
- `get_global_value`: Returns a 32-bit value stored at a specific offset from a global base.
- `set_global_value`: Stores a 32-bit value into a specific offset from a global base.
- `interleave_word_data`: Copies thirty-two words from a source to a destination, placing them into every other word position in the destination.
- `populate_coordinate_table`: Copies a block of data from a source table and then populates a destination table with a series of split high and low word values derived from an input parameter.
- `interleave_and_update_data`: Swaps two 32-bit values, copies thirty-two words from a source to a destination using an offset, and executes several function calls based on global status flags.
- `update_system_state`: Calls two functions, copies a four-byte value from one memory location to another, clears two words, and then calls a third function.
- `store_global_value`: Stores the current value of a register into a global variable.
- `call_indirect_via_offset`: Calls a function pointer located at an offset from a global variable.
- `update_list_nodes`: Traverses a linked list and updates bytes within each node based on specific bitwise conditions and a global counter.
- `update_buffer_via_list_traversal`: Traverses a linked list to clear specific bytes in nodes while filtering for specific byte values.
- `process_linked_list_nodes`: Traverses a linked list, clearing specific bytes in each node and updating a global buffer with data from nodes that meet certain bitwise criteria.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `initialize_structure_data`: Calls a routine and then populates several fields in a structure with parameters and a function pointer.
- `update_structure_fields`: Stores a calculated value and a constant into a structure and calls another routine via a pointer.
- `initialize_input_device`: Attempts to open an input device and, if successful, initializes a structure with data and a function pointer.
- `execute_conditional_callbacks`: Checks three different two-byte or four-byte values at specific offsets; if they are non-zero, it calls associated function pointers using the values as arguments.
- `copy_table_data`: Copies data from one memory location to another and populates a table with values from a loop.
- `allocate_and_set_buffers`: Allocates memory for two different sizes using a function, stores the resulting pointers in a structure, and initializes the memory at those locations.
- `process_index_sequence`: Compares two values to determine if an index should increment, searches a table for a matching byte, and stores a resulting value into a table.
- `initialize_font_resource_pointers`: Retrieves several resource handles via a function call and stores them in a structure.
- `execute_system_initialization_sequence`: Executes a sequence of function calls through various pointers and tables to initialize system states or device contexts.
- `execute_system_callbacks`: Calls a sequence of function pointers retrieved from various offsets within a structure.
- `initialize_system_routines`: Executes several function pointers from various tables and updates system pointers.
- `find_maximum_value`: Iterates through a table to find the highest calculated value based on bitwise operations and returns it along with an index.
- `execute_callback_sequence`: Calls a function with several parameters and then calls another function via a pointer.
- `loop_function_calls`: Repeatedly calls a function pointer within loops.
- `compare_and_initialize_data`: Compares a local string against a global variable and, if they match, calls a routine to update a global value.
- `execute_conditional_call`: Checks if a global variable is non-zero and, if so, calls a routine via a pointer found in a table.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `initialize_and_update_engine_state`: Copies several null-terminated strings from a global buffer into a structured memory block and initializes various state flags and counters.
- `initialize_engine_state`: Clears several memory blocks, including a large array and multiple structured tables, and populates a global string.
- `process_structured_data_list`: Checks a bit flag in a table and, if not set, performs several function calls and iterates through a list of structures to call a routine on specific members.
- `process_input_event_queue`: Iterates through a linked list of input structures to update their state and handle button transitions.
- `update_structured_fields`: Calculates several values based on function returns and sign checks, then updates a data structure using multiple calls to a field modification routine.
- `process_pause_logic`: Checks a return value to determine if a specific state is active, then updates a global word and a passed pointer based on the result of a conditional check.
- `update_entity_state_logic`: Manages the lifecycle and state transitions of entities by checking status flags and updating their internal data structures.
- `handle_pause_input`: Checks a value from a routine and performs different actions based on whether it matches specific constants, including calling other routines or toggling a global value.
- `do_nothing`: This routine is empty and performs no operations.
- `process_input_flags`: A twenty-instruction routine that calls another routine to return a byte derived from a global variable by combining bits sixty-four and four using a logical OR.
- `update_table_values`: Compares values from a table against global variables, updates accumulators if they differ, and copies sixteen pairs of words from a table into a destination buffer.
- `manipulate_color_register`: Performs bitwise operations involving a global variable and a register to find a specific bit pattern.
- `return_immediately_4`: The routine is empty and performs no operations before returning.
- `update_interrupt_state`: Modifies global variables and writes to the INTENA and INTENAR registers to control interrupt behavior.
- `disable_interrupts`: Stores a constant value into a global variable that controls the interrupt enable bit of the INTENA register.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `set_hardware_constants`: A short routine that calls another routine to merge zero into a register and writes constant values to the DMACON and INTENA registers.
- `update_dma_and_constants`: Clears a register and calls a routine to store a constant into a global variable while updating DMA and interrupt enable hardware registers.
- `update_dma_and_constants_2`: Clears a register and calls a routine to store a constant into a global variable while updating DMA and interrupt enable hardware registers.
- `configure_dma_and_cop_registers`: This routine clears a register and calls a routine to set global constants and hardware registers.
- `set_hardware_constants_2`: Sets a constant value into a global variable and updates the DMA and interrupt enable hardware registers.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `initialize_system_state`: Updates several global variables and pointers from a table and modifies the interrupt enable register.
- `set_global_constants`: Stores a constant value into a global variable and writes to DMACON and INTENA hardware registers.
- `initialize_system_pointers`: Writes several pointers to a table and sets a global variable to a specific value.
- `update_mouse_position_2`: A 13-instruction routine that modifies the interrupt request register.
- `update_mouse_position`: Reads input from a mouse port, calculates relative movement with wrapping logic, and updates a pointer to a structure containing clamped coordinate values.
- `update_joystick_input_state`: Processes raw input values by applying signed arithmetic with wrapping logic and clamping them within specific ranges.
- `calculate_clamped_values`: Performs arithmetic on values from a table, applies clamping logic, and stores the results into a pointer-based destination.
- `update_pointer_and_clamped_data`: Constructs a new pointer by bit-shifting and masking values from a source structure, then writes calculated values to the memory location pointed to by that new pointer.
- `clear_global_flag`: If a value in a table is non-zero, it stores a constant into a global variable and updates another memory location before clearing that table value.
- `read_input_flags`: The routine returns a byte derived from a global variable, testing bits 64 and 4 and combining the results with a logical OR. It references the CIAA_PRA and POTGOR hardware registers.
- `wait_for_bit_clear`: Loops while a specific bit in a global variable remains set.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `decompress_bitstream_data`: Reads a compressed bitstream using a bit-by-bit reader to reconstruct bytes.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bits_across_words`: The routine shifts a 32-bit value right by one bit repeatedly for a given count, refilling from the next word in a pointer when the value becomes zero.
- `perform_blitter_operations`: Uses blitter functions to draw two rectangular areas at different offsets while managing blitter ownership.
- `initialize_data_structure_2`: Allocates a new memory block and populates it with several values from parameters and a global table.
- `traverse_linked_list_5`: Iterates through a linked list of structures, executing a routine based on a value in the structure and updating a table entry if a specific condition is met.
- `process_entity_linked_list`: Iterates through a linked list of structures to identify specific entities based on matching ID and state flags.
- `process_complex_data_structures`: A 202-instruction routine that performs recursive traversals, rectangular area intersections, and string copying operations.
- `process_joystick_input`: The routine performs a series of operations involving 48 instructions and is invoked by two separate callers.
- `process_input_buffer`: Iterates through a data structure to evaluate input states.
- `process_input_and_coordinates`: A 213-instruction routine that evaluates input states, performs recursive data traversals, and calculates coordinate offsets.
- `recursively_update_structure_2`: Performs a recursive traversal of a data set, calling another routine multiple times with varying offsets and parameters based on conditional checks and table lookups.
- `intersect_rect_sub_regions`: Performs a recursive intersection check between two rectangular areas to identify overlapping sub-regions.
- `recursively_update_structure`: Performs a recursive traversal of a data set, calling another routine multiple times with varying offsets and parameters based on conditional checks.
- `decompose_rectangle_into_tiles`: Calculates the dimensions and offsets required to split a rectangular area into smaller sub-sections or tiles.
- `initialize_quadrant_subdivision`: An entry point that calls a routine to recursively divide a sprite area into smaller quadrants using six parameters, with the final two parameters initialized to zero.
- `update_complex_structure`: Calls two routines and then stores a large number of parameters into a structure at various offsets.
- `copy_parameters_to_structure`: Stores multiple parameters into a structure at specific offsets.
- `manage_blitter_state`: Uses graphics library functions to own, disown, and wait for the blitter.
- `scan_bit_sequence`: Calculates a memory address and checks a bit at a specific position across five consecutive word-aligned locations, returning a bitmask of the results.
- `check_single_bit`: Calculates a memory address and returns a boolean value based on whether a specific bit is set at that location.
- `calculate_coordinate_offsets`: Performs complex arithmetic involving global memory values to calculate and update coordinate-based parameters.
- `update_complex_structure_2`: Calculates several values based on data from a global structure and passes them as arguments to another routine that stores them into a structure.
- `clear_data_structure`: Iterates through a loop to zero out multiple fields within a series of repeating data blocks and sets a final word to one.
- `find_string_in_blocks`: Compares a provided string against a series of data blocks and returns the index of the first matching block.
- `search_string_table_2`: Iterates through a table of fixed-size entries to find a string match.
- `find_string_in_table`: Iterates through a table of strings to find a match for a provided string and returns its index.
- `find_string_index`: Compares a provided string against entries in a table and returns the index of the first matching entry.
- `allocate_command_data`: Searches a command table for a matching string and allocates a new data structure.
- `load_command_data_structure`: Searches a command table for a matching string and allocates a new data structure.
- `clear_table_entry`: Sets a byte, a long, and a word to zero at specific offsets within a table entry.
- `release_object_resources`: Iterates through a structured data table to identify and release associated resources.
- `get_offset_from_block`: Calculates an offset into a table based on a value found within a specific data block.
- `get_offset_from_pointer`: Retrieves a pointer from a data block and calculates an offset into a secondary table using a provided index.
- `add_offset_to_value`: Adds a value from one of several specific offsets within a structure to a base value, depending on a provided index.
- `nop_padding_sequence_2`: This routine is empty and consists of non-functional instructions used as padding within the executable segment.
- `add_offset_to_register`: Adds a value located at a specific offset within a memory structure to the contents of a register and returns the result.
- `get_offset_from_subtable`: Calculates an offset into a subtable by using a pointer and an index retrieved from a data block.
- `resolve_string_hierarchy`: Recursively parses a string to split it into components based on a delimiter.
- `parse_resource_hierarchy`: Recursively traverses a hierarchical data structure to resolve string-based identifiers.
- `iterate_and_populate_entity_attributes`: Iterates through a table of 90 entries, checking if both a pointer and a short value are non-zero before calling a routine to populate attributes from the data.
- `update_subtable_data`: Updates a value in a table or a linked list structure based on an index and a switch case determined by a retrieved offset.
- `reset_table_entry_values_2`: Iterates through a structured table to locate entries matching specific criteria and resets a value within those entries to zero.
- `clear_table_entries`: Iterates through a table to find specific values and resets them to zero if certain conditions are met.
- `reset_table_entry_values`: Iterates through a structured table to locate entries matching specific criteria and resets a value within those entries to zero.
- `count_matching_elements`: Iterates through a list of elements within a data block and counts how many match a specific value.
- `execute_unreachable_instructions_2`: This routine consists of four instructions and is located within a gap in the executable.
- `execute_unreached_code`: This routine contains eleven instructions but is never formally called by the program.
- `execute_unreachable_instructions`: This routine consists of six instructions and is located within a gap in the executable.
- `search_data_table`: Iterates through a table of data structures to find an entry matching a specific short value, then copies a string and compares it against another.
- `search_string_table_3`: Iterates through a table of data structures to find an entry matching a specific short value, then copies a string and compares it against another string.
- `search_and_verify_string_table`: Iterates through a table of data structures to find an entry where a specific short value matches a target.
- `compare_string_data`: Copies a string from one location to another and then performs a comparison between two null-terminated strings.
- `copy_and_compare_strings`: Copies a string to a new location and then compares it against another null-terminated string.
- `initialize_parameter_block`: Sets the first word of a provided pointer to zero and a word at a specific offset to all ones.
- `traverse_and_call_list`: Traverses a linked list and calls a function pointer stored within each node if specific criteria are met.
- `call_other_routine_2`: calls another routine and returns.
- `update_linked_list_node_value`: Traverses a linked list and updates a field in any node where certain criteria or wildcards match.
- `allocate_and_link_node`: Allocates a new data structure, populates it with provided parameters, and inserts it into a linked list.
- `set_list_field_to_max`: Traverses a linked list and sets a specific field to all ones if certain criteria or wildcards match.
- `filter_and_update_list`: Traverses a linked list and updates a specific field in nodes that match provided criteria.
- `return_zero`: This routine is empty and simply returns zero to the caller.
- `traverse_linked_list`: Traverses a linked list and updates a pointer to a new head if a certain condition or terminator is met.
- `traverse_data_hierarchy`: Traverses a linked list of data blocks, checking specific flags and conditions before recursively calling itself or marking a block as processed.
- `process_data_block_hierarchy`: Retrieves data from a block, calculates offsets into various tables, and executes a loop of function calls based on returned values.
- `dispatch_offset_function`: Uses a value from a global variable to select a case in a switch statement, which then executes different sequences of function calls.
- `call_two_functions`: Calls two functions at addresses retrieved from a global table and returns zero.
- `execute_conditional_function`: Retrieves a value from a table and calls a function with a calculated offset or address.
- `execute_conditional_calls_3`: Retrieves two values from a table via function calls and stores them in a global structure.
- `call_offset_function_2`: Calls a function at an address calculated by adding an offset to a global value.
- `process_table_data`: Uses a switch statement to handle different data types.
- `return_zero_11`: An empty routine that returns zero.
- `store_register_value_2`: Stores the contents of a register into a global variable.
- `modify_memory_via_table`: Uses a table to resolve an address and either calls a function or writes a byte to that location.
- `execute_status_check`: Calls two functions via pointers from a table, stores their results in local memory, and performs a conditional call based on the second result.
- `call_and_check_status`: Calls a function using a value retrieved from a table and returns a status code based on whether the result is zero.
- `return_zero_value_2`: The routine performs no operations and returns zero to the caller.
- `calculate_offset_value`: Calculates a value based on several lookups and arithmetic operations, then passes it to another routine.
- `execute_table_function`: Uses a value to index into a table of function pointers and executes the resulting routine if it is valid.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_via_switch`: Calls a routine twice and then executes a switch statement based on a value from a global variable before calling another routine.
- `call_routine_only`: Calls a routine and returns zero.
- `execute_indirect_calls_2`: Calls two indirect functions using offsets from a register and then executes a third indirect function.
- `execute_indirect_function`: Calculates an offset from a register and executes the routine located at the resulting address.
- `update_status_word`: Calls a routine and stores a bitmask in a table based on the returned signed integer value.
- `execute_multiple_routines`: Calls three different routines, passing the result of the first call as an argument to the fourth.
- `update_conditional_value`: Updates a word in a table based on a value returned by a function and a bitmask check.
- `store_word_if_bit_clear`: Stores a word into a table at a specific offset if a bit in a register is zero.
- `store_word_if_bit_set`: Stores a word into a table at a specific offset if a bit in a register is non-zero.
- `store_word_if_second_bit_set`: Stores a word into a table at a specific offset if the second bit of a register is set.
- `store_register_value`: Stores a word-sized register value into a table at a specific offset.
- `set_word_value`: Stores a constant word value into a table at a specific offset and returns one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_callback_table`: Clears a sequence of memory entries to zero and populates a series of consecutive function pointer slots in a table with the address of the routine itself.
- `clear_memory_block`: Sets two consecutive longwords at a memory location to zero.
- `process_linked_list_recursively`: Iterates through a linked list and calls two functions for each node where a specific word is zero, potentially recursing if no non-zero nodes are found.
- `process_linked_list_nodes_2`: Iterates through a linked list and calls two functions for each node where a specific word is zero.
- `check_and_recurse`: Checks if a word is zero and calls itself if the condition is met.
- `insert_linked_node_2`: Traverses a linked list to find an insertion point based on specific field comparisons.
- `insert_linked_node`: Traverses a linked list to find a suitable insertion point.
- `allocate_linked_node`: Allocates a memory block, populates it with data from registers and a table, and inserts it into a linked list.
- `allocate_and_link_structure`: Allocates a memory block, populates it with various data from registers and a table, and inserts it into a linked list.
- `initialize_new_list_node`: Traverses to the end of a linked list, allocates a new block, populates it with parameters, and links it to the list.
- `allocate_and_initialize_structure`: Allocates a new memory block, populates it with several values from registers and the stack, and inserts it into a linked list.
- `process_table_entry`: Checks a value in a table, and if non-zero, performs a sequence of function calls using data from that table entry.
- `filter_and_remove_nodes`: Iterates through a linked list to update certain fields based on a range check, then removes nodes matching a specific value and updates the list head.
- `filter_and_update_list_2`: Iterates through a linked list and updates a word in each node that matches several provided criteria or wildcards.
- `return_from_subroutine_2`: This routine performs a simple return to the calling function.
- `update_list_fields`: Iterates through a linked list and, for nodes matching three specific criteria, updates two word-sized fields in the node.
- `update_list_values`: Iterates through a linked list and updates two word-sized fields in nodes that match specific criteria.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `extract_linked_list_nodes`: Iterates through a linked list to find nodes matching specific criteria and removes them from the original list.
- `reorder_linked_list_3`: Performs complex reordering of a linked list by comparing values and pointers, potentially swapping nodes or updating list head pointers.
- `reorder_linked_list`: Performs complex operations to swap or reorder elements within a linked list structure based on comparisons of values and pointers.
- `reorder_linked_list_2`: Reorders or swaps elements within a linked list based on comparisons of values and pointers.
- `manipulate_linked_list`: performs complex operations to swap or reorder elements within a linked list structure based on comparisons of values and pointers.
- `swap_list_elements`: exchanges two nodes within a linked list by updating their respective pointers and the pointers of their neighbors.
- `store_register_to_memory`: Stores a longword from a register into a memory location offset by four bytes from a pointer.
- `process_entity_state_list`: Iterates through a list of data structures to check specific type flags and status bits.
- `clear_longword_pair`: Writes zero to two consecutive 32-bit longwords at the address passed in the single pointer argument, then returns.
- `search_linked_list`: Iterates through a linked list to find the first node that matches specific range criteria for multiple fields.
- `remove_list_elements_2`: Iterates through a linked list to mark specific nodes for removal based on three criteria.
- `remove_list_elements`: Iterates through a linked list and removes specific nodes by updating pointers and executing indirect function calls.
- `traverse_linked_list_3`: Iterates through a linked list, updating pointers and performing indirect function calls based on values found within the list elements.
- `nop_padding_sequence`: This routine is empty and contains no executable instructions.
- `search_linked_list_3`: Traverses a linked list to find an entry matching specific short integer criteria at fixed offsets.
- `search_linked_list_2`: Traverses a linked list to find an entry that matches specific short integer criteria stored at fixed offsets within the list nodes.
- `return_zero_3`: This routine is empty and simply returns a zero value to the caller.
- `update_list_entry`: Searches a linked list for a node matching two criteria and updates a short integer field in that node if a third criterion is met.
- `instantiate_entity_from_template`: Allocates a new memory block and populates it with data from a template structure, including a string copy and various coordinate or state parameters.
- `spawn_entity_from_template`: Allocates a new memory block and populates it with data copied from a template structure and a string.
- `append_entity_to_linked_list`: Traverses a linked list to find the last element and inserts a newly allocated structure at the end.
- `allocate_and_populate_sprite`: Traverses a linked list to find the end and inserts a new structure at the head.
- `initialize_sprite_structure`: Allocates a new structure from a linked list and populates it with data from a source structure and a lookup table.
- `access_calculated_pointer`: Calculates an offset using a multiplication and addition, then dereferences the resulting address to call another routine.
- `process_conditional_input`: Checks a value in a structure and, if it matches a specific constant, executes two separate subroutines and performs a bitwise operation on a value from a memory location.
- `iterate_linked_list`: Traverses a linked list and calls another routine for each entry that matches specific criteria stored in a structure and a table.
- `nop_placeholder`: The routine is empty and performs no operations.
- `filter_linked_list_nodes`: Iterates through a linked list to mark specific nodes based on matching short integer values and then removes those marked nodes from the list while executing indirect function calls.
- `process_list_callbacks`: Traverses a linked list and executes indirect function calls while updating a pointer in a global structure.
- `process_list_with_callbacks`: Traverses a linked list and executes indirect function calls, optionally updating a pointer stored in a global structure if a specific field is non-zero.
- `return_zero_value`: Returns a constant value of zero immediately without performing any other operations.
- `iterate_and_execute_linked_list`: Traverses a linked list by following pointers and executes a specific function pointer stored in a global table for each node.
- `traverse_and_call`: Iterates through a linked list and executes indirect function calls for each node.
- `clear_dual_longwords`: Writes zero to two consecutive longword memory locations using pointers provided as parameters.
- `zero_two_longwords`: Writes zero to two consecutive longword locations pointed to by a parameter.
- `process_linked_event_list`: Iterates through a linked list of data structures, performing string comparisons against a lookup table to trigger conditional logic.
- `traverse_and_process_linked_data`: Iterates through a linked list of data structures, performing string comparisons against a table and executing conditional logic based on the results.
- `traverse_linked_data_table_2`: Iterates through a linked list of data structures by comparing string identifiers against a lookup table.
- `traverse_linked_data_table`: Iterates through a linked list of data structures by comparing string identifiers against a lookup table.
- `store_byte_at_offset`: Stores a single byte at a specific offset relative to a register value.
- `clip_and_draw_structure`: Checks if coordinates are within bounds, clips them, and then updates a structure with calculated dimensions and calls another routine in a loop.
- `calculate_and_draw`: Calculates coordinate offsets and dimensions, then executes a routine five times in a loop to perform drawing operations.
- `iterate_complex_structure`: Calculates an offset based on multiple parameters and calls another routine five times within a loop, updating a value in each iteration.
- `execute_repeated_drawing`: Checks specific conditions in a structure and a table before calling another routine five times with incrementing parameters.
- `update_drawing_loop`: Checks if specific values in a structure and a table are non-zero, then executes another routine five times with incrementing parameters.
- `update_drawing_parameters`: Calculates parameters from a structure and executes a routine five times in a loop to update drawing state.
- `update_structure_parameters`: Performs calculations using values from a structure and calls another routine five times in a loop with varying parameters.
- `reset_table_entries`: Iterates through a table of two hundred and fifty-seven entries, calling a function pointer if a condition is met and then zeroing out several fields within each entry.
- `populate_table_entry_2`: Calculates a value based on the product of two inputs and stores several unsigned shorts and a processed result into a table entry.
- `populate_table_entry`: Calculates a value based on a product of two inputs, calls a routine to process it, and stores the result along with several unsigned shorts into a table.
- `update_table_entry`: Stores a calculated value and several unsigned shorts into a table at an offset determined by an index.
- `write_table_entry`: Stores several values, including a zeroed integer and two unsigned shorts, into a table at an offset determined by an index.
- `reset_table_entry`: Resets several fields within a table entry to zero or default values and copies a null-terminated string from a global variable into the entry.
- `clear_table_entry_2`: Resets multiple fields in a table entry to zero or default values and copies a null-terminated string from a global variable into the entry.
- `reset_table_string`: Clears multiple fields in a table entry and copies a null-terminated string from a global variable into that entry.
- `reset_and_copy_string`: Clears multiple fields in a table entry and then copies a null-terminated string from a global variable into that entry.
- `return_immediately_3`: This routine performs no operations and returns immediately to its caller.
- `return_no_op_2`: The routine consists of a single return instruction and performs no work.
- `zero_memory_loop`: Enters an infinite loop that repeatedly clears several fields at a specific memory offset.
- `call_list_elements`: Iterates through a list of pointers and calls a routine through each non-zero pointer, then clears a word in a global variable.
- `call_and_clear_list`: Iterates through a list of pointers, calling a function through a pointer if the entry is non-zero, and then sets a word to zero.
- `display_disk_message_and_clear_buffer`: Copies a string to a local buffer and displays it, then clears a structured data table by iterating through its entries and zeroing out specific byte offsets.
- `copy_string_and_close`: If a condition is met, it calls two function pointers, copies a null-terminated string from a global location to a local buffer, and updates a status value.
- `compare_string_against_table`: Reads characters from a source into a buffer and compares the resulting string against a table of entries.
- `search_mismatched_string_table`: Iterates through a table to find an entry where a specific short value does not match a field, then performs a string comparison against a list of strings within that entry.
- `search_matched_string_table`: Iterates through a table to find an entry where a specific short value matches a field, then performs a string comparison against a list of strings within that entry.
- `process_string_input`: Copies a string into a local buffer and enters a loop of conditional routine calls to handle input or display messages based on the string content.
- `perform_file_io`: Performs file seeking and reading operations using system library calls.
- `copy_string_from_source`: Copies a certain number of bytes from a source pointer to a destination pointer and updates a source pointer tracker.
- `fetch_table_entry`: Searches a table for a valid entry and copies data from a source table into a local structure.
- `fetch_table_data`: Retrieves data from a table using an index and copies a string into a local buffer if a valid pointer is found.
- `retrieve_table_data`: Retrieves data from a table using an index, copies a string into a local buffer if a pointer is valid, and handles error cases.
- `process_preloaded_string`: Retrieves data from a table based on a returned index and either performs error handling or copies a string from a pointer into a local buffer.
- `load_data_from_file`: Reads data from a file into a buffer based on values stored in a table and copies the resulting string into a destination.
- `process_string_table`: Iterates through sixty-four entries in a table, checking string lengths and specific conditions to call a function and clear various fields in the table entries.
- `clear_string_table`: Iterates through a table of entries to find non-empty strings, clears them, and resets associated data and pointers.
- `compare_string_table`: Compares an input string against a table of strings and clears the entry if a match is found or specific conditions are met.
- `search_string_table_and_store_metadata`: Iterates through a table of null-terminated strings to find a match for a provided string.
- `find_string_in_table_2`: Searches a table of null-terminated strings for a match against a provided string and stores two values if a match is found.
- `search_command_table`: Iterates through a table of string entries to find a match for a provided input string.
- `multiply_table_values`: Calculates the product of two 16-bit values retrieved from a table based on an index.
- `wait_for_blit`: A routine that calls a graphics library function to wait for a blit operation to complete.
- `map_entity_string_to_data`: Compares an input string against a global string and, upon a match, iterates through a list of sub-entries to populate an entity data table.
- `populate_entity_data_table_2`: Compares a local string against a global string and, if they match, iterates through a list of sub-entries to calculate spatial dimensions and populate a data table with entity properties, including a string copy and type flags.
- `match_and_populate_entity_table`: Compares a local string against a global string and, if they match, iterates through a list of sub-entries to calculate spatial dimensions and populate a structured table with metadata, including type flags and string data.
- `update_table_entries`: Calculates a product of two values and populates a table with several fields, including a string and a status byte based on a specific value.
- `update_data_structures`: Performs arithmetic on structure members, updates status bytes based on specific values, and copies strings from a source to a destination.
- `process_data_list`: Iterates through a list to update status bytes, copy strings, and call other routines while performing arithmetic on structure members.
- `update_data_list`: Iterates through a list of entries to update status bytes, copy strings, and perform recursive updates.
- `sync_entity_string_and_table`: Compares a local string against a global string and, upon a match, calculates spatial dimensions and populates a structured table entry.
- `populate_entity_data_table`: Iterates through a collection of inputs to calculate products and populate a structured table with derived values, dimensions, and string data.
- `update_entity_attributes`: Iterates through a collection of entities to update their properties, including status flags, dimensions, and string data.
- `populate_entity_table`: Iterates through a data structure to populate a table with entity attributes, including status flags, coordinates, and string data.
- `populate_entity_attribute_table_2`: Iterates through a nested data structure to populate an entity attribute table with properties including status flags, short integers, and string data.
- `populate_entity_attribute_table_3`: Iterates through a nested data structure to populate an entity attribute table with properties including status flags, short integers, and strings.
- `populate_entity_table_2`: Iterates through a data structure to populate a table with properties including short integers, status flags, and null-terminated strings.
- `populate_entity_attribute_table`: Iterates through a nested data structure to populate an entity attribute table with properties including status flags, short integers, and null-terminated strings.
- `update_sprite_attribute_table`: Iterates through a data structure to update sprite or object attributes, including copying string data and calculating positional offsets.
- `populate_entity_attribute_table_4`: Iterates through a structured data set to calculate derived values and populate an attribute table.
- `sync_string_to_table_entry`: Compares a string from a local structure against a global string and, upon a match or specific condition, updates a table entry with a new string, a status byte, and an integer value.
- `update_data_structure`: Copies a string from a source to a destination, updates several fields in a table, and returns a concatenated value.
- `compare_and_update_string`: Compares a string at a specific offset with a global string and, if they match, updates a table entry with new data and a copied string.
- `update_data_structure_offset`: Decrements a value at a specific offset, copies a string, updates table fields, and returns a concatenated value.
- `execute_indirect_calls_3`: Calls two function pointers using values derived from registers and a global variable, then returns a specific error code.
- `load_word_from_offset`: Returns a 16-bit value stored at a specific offset from a pointer.
- `update_entity_state_from_string_2`: Compares an input string against a command table to identify a target entity and updates its properties.
- `sync_entity_state_by_string`: Copies a string to a local buffer and compares it against a global variable.
- `update_entity_state_from_string`: Copies a string to a local buffer, compares it against a global variable, and if they match, performs arithmetic calculations to update an entry in a data table.
- `calculate_and_update_table`: Calculates new values based on arithmetic operations and table lookups, then updates a table entry with specific short and byte values based on the contents of local variables.
- `update_table_and_copy_string`: Calculates a value from a table, conditionally calls another routine, updates several fields in a table based on a status byte, and copies a null-terminated string to a destination.
- `update_status_and_copy_string`: Sets a status byte in a table based on a condition, conditionally calls another routine using parameters from a table, and copies a string to a destination.
- `process_conditional_structure`: Sets a field in a table, conditionally calls another routine if a pointer is non-zero, and copies a string to a destination.
- `update_entity_table_entry`: Compares a string against a global variable and, upon a match, calculates new spatial or state parameters to update an entry in a data table.
- `update_table_entry_via_nested_loops_2`: Calculates a value based on the product of two inputs and updates a table entry with several unsigned shorts and a processed result.
- `update_table_entry_via_nested_loops`: Iterates through nested loops to update a table entry with calculated values and status bytes.
- `process_structured_data`: Iterates through a nested loop to update a table with values, copies a null-terminated string, and returns a combined 32-bit value.
- `process_nested_loop`: Iterates through nested loops to update a table with status bytes, copies a string, and returns a combined 32-bit value.
- `execute_nested_updates`: Performs nested loops to update a table entry with calculated values, sets status bytes, and copies a string from a source.
- `process_structured_data_alt`: Performs a nested loop to update a table, copies a string, and returns a value based on a pointer and an index.
- `process_nested_table_loop`: Executes a loop that calls another routine, updates a table entry, and performs nested loops to update status bytes and copy strings into a table.
- `update_node_status_and_copy_string`: Sets a status byte in a data structure based on a condition, then conditionally performs a recursive call using values from that structure.
- `update_node_and_copy_string`: Sets a status byte in a data structure and conditionally performs a recursive call using values from that structure.
- `sync_table_entry_by_string`: Compares a string from a local buffer against a global variable and updates a specific entry in a data table based on the match.
- `update_data_structure_copy`: Copies a string from a source to a destination, updates table fields, and returns a concatenated value.
- `compare_and_update_table`: Compares a string from a pointer against a global variable and, if they match, performs a series of table updates, string copies, and indirect function calls.
- `update_table_entry_and_copy_string`: Decrements a value at a memory location, calls another routine with parameters derived from a table, and resets specific fields within a table entry.
- `execute_indirect_calls_alt`: Executes two function pointers using an offset from a register and a global variable, returning a specific error code.
- `load_word_from_offset_alt`: Returns a 16-bit value from a memory location relative to a pointer.
- `display_disk_message`: Copies a global string to local buffers, performs several function calls, and enters a loop that executes a routine until it returns zero.
- `wait_for_condition`: Checks a value in a table against a target and repeatedly calls another routine until the value matches.
- `manage_object_state`: Checks a condition to reset a value, searches a table for a matching entry, and either updates an existing entry or adds a new one using a function call.
- `update_table_and_execute_pointers`: Increments a counter within a loop and writes multiple values into a table.
- `update_table_and_call`: Increments a counter in a loop, stores several values into a table, and executes two function pointers.
- `update_data_and_call`: Increments a counter in a loop, stores a 32-bit value to a pointer, and executes two function pointers.
- `set_hardware_register_and_notify`: Sets a bitmask in a global variable, writes values to specific hardware register offsets, and executes two function pointers.
- `loop_and_execute_callbacks`: Increments a value in a register until it reaches a limit stored in a memory location, then updates a memory value and executes two function pointers retrieved from a table.
- `write_hardware_registers`: Executes two function pointers and writes two 16-bit values to specific hardware register offsets.
- `call_stored_function`: Calls a function through a pointer stored at a specific offset from a base address.
- `call_indirect_function`: Calls a function through a pointer stored at an offset from a register.
- `transform_string_bytes`: Iterates through a null-terminated string and replaces each byte with a value returned from a function call.
- `process_clamped_data_series`: Iterates through a table of values using nested loops to perform clamped arithmetic and stores the results in a destination buffer.
- `store_constant_word`: Stores the value one into a word at a specific offset from a register.
- `calculate_clamped_sums_3`: Performs multiple signed additions on input parameters and clamps the results between zero and fifteen.
- `calculate_clamped_sums_2`: Performs two signed additions with clamping to a maximum value of fifteen and returns the results combined into a single word.
- `calculate_extended_clamped_sums`: Performs three signed additions with clamping to a maximum value of fifteen and returns the results combined into a single word.
- `calculate_clamped_sums`: Performs two separate signed additions with clamping to a maximum value of fifteen and returns the results combined into a single word.
- `calculate_clamped_sum_2`: Performs signed addition with overflow checks and clamping to a maximum value, returning a 16-bit result.
- `calculate_multiple_clamped_sums`: Performs two separate signed addition operations with overflow checks and clamping, returning a combined 16-bit value.
- `calculate_clamped_sum`: Performs signed addition with overflow and clamping logic on two values and returns a 16-bit result combining the sum with another short.
- `combine_nibbles_with_mask`: Returns a 16-bit value formed by shifting a short, applying a bitwise OR with a constant, and adding another short.
- `combine_nibbles_and_check_sign`: Combines three values into a single word by shifting and bitwise ORing, while performing a signed addition check on two values.
- `combine_nibbles`: Combines two values by shifting one left by four bits and performing a bitwise OR with the other.
- `call_and_concatenate`: Calls a function twice and concatenates the two resulting words into a single 32-bit value.
- `apply_clamped_transformations`: Performs sixteen iterations of clamped arithmetic on a buffer of thirty-two words.
- `execute_conditional_indirect_calls`: Calls several routines through pointers and, if certain conditions are met, performs multiple indirect calls and concatenates results into a 32-bit value.
- `execute_indirect_calls_5`: Calls several routines through pointers stored at various offsets from a base address and performs conditional logic based on a returned value.
- `execute_conditional_insertion`: Executes a function pointer multiple times and, if the returned value is not negative one, calls another routine with the result and two other parameters.
- `call_insertion_routine`: Calls another routine with a pointer to a specific memory location.
- `execute_callback_loop_2`: Calls a function pointer repeatedly to populate a local array, then conditionally calls another routine and iterates through the array to call a second function pointer.
- `execute_callback_loop`: Populates a table by calling a function pointer in a loop, then conditionally executes another function pointer multiple times based on a value from another routine.
- `iterate_linked_nodes`: Calls another routine to obtain a pointer, then executes a function pointer in a loop based on a count.
- `call_routine_in_loop`: Executes a routine repeatedly within a loop, passing a value that increments by two each iteration.
- `execute_multiple_calls`: Calls four different functions stored at specific offsets relative to a register.
- `filter_linked_list`: Calls a function three times to retrieve values and then passes a combined 32-bit value and a pointer to another routine.
- `call_with_arguments`: Calls a function multiple times to retrieve values and then passes those values as arguments to another function.
- `call_with_pointer`: Calls a function using a pointer calculated from a register and an offset.
- `execute_indirect_and_traverse`: Executes two routines through pointers and then calls another routine that traverses a linked list.
- `call_with_concatenated_values`: Calls a function twice to retrieve two words and passes them as a single concatenated value to a third function.
- `process_string_conditionally`: Checks a bit in a global variable, calculates the length of a null-terminated string, and if the length is not one, executes several indirect calls and copies the string to a new location.
- `process_string_length`: Calculates the length of a null-terminated string and calls another routine twice if the string length is not one.
- `get_string_length_and_call`: Determines the length of a null-terminated string and calls a routine using the string as an argument.
- `transform_data_buffer_2`: Iterates through a range of values from a source table, applies clamped arithmetic, and writes the results to a destination buffer.
- `transform_data_buffer`: Iterates through a range of values from a source table, applies clamped arithmetic, and writes the results to a destination buffer.
- `call_offset_function`: Calls a function located at an offset from a value held in a register.
- `call_scaled_offset_routine`: Checks a table using an index to verify a pointer is non-zero, then calls a routine with a value multiplied by one thousand.
- `call_scaled_and_offset`: Calls a routine with a value multiplied by one thousand and then calls another routine via a pointer at a specific offset.
- `validate_table_entry`: Verifies if a table entry at a specific index contains a specific byte and a non-zero pointer before calling another routine.
- `call_offset_with_table_lookup`: Performs a table lookup based on an index to check specific byte and pointer values before calling a routine with a scaled value or an indirect function.
- `call_indirect_function_2`: Calls a function through a pointer located at an offset from a register.
- `set_constant_value`: Stores the value one into a memory location.
- `call_routine_with_string`: Finds the end of a null-terminated string and calls a routine through a pointer if the string length is not one.
- `append_to_list`: Retrieves several values through pointer-based calls and passes them to a routine to append an entry to a list.
- `filter_linked_list_with_constant`: Retrieves values via function calls and passes a 32-bit value containing a constant integer and a pointer to another routine.
- `dispatch_conditional_callback`: Updates a value in a structure and checks a short integer within a nested structure against two specific constants.
- `call_conditional_function`: Checks a value within a structure and, if it matches a specific constant, calls another routine with three arguments.
- `update_list_entry_data`: Searches a linked list for an entry and, if found, updates several fields in that entry using values obtained from multiple function calls.
- `return_one`: Returns the integer value one.
- `get_list_entry_word`: Searches a linked list for an entry and returns a word from that entry, or returns all bits set if no entry is found.
- `return_negative_one`: Returns a value representing all bits set to one.
- `execute_multiple_calls_2`: Calls one routine three times and then calls a second routine with a pointer to a global variable.
- `execute_indirect_calls_4`: Performs several indirect function calls and then passes a retrieved word and a pointer to another routine.
- `store_function_results`: Calls a routine four times and stores each returned word into consecutive memory locations.
- `search_list_wrapper`: Executes several function calls and then calls another routine to search a linked list using a specific memory offset.
- `call_two_routines`: Executes two different function pointers stored in a table.
- `update_word_table`: Calls a routine four times and stores the results into four specific memory locations.
- `update_and_return_short`: Calls a routine and, if the result is not negative one, stores it in a global variable and sets a flag before returning the stored value.
- `store_short_value`: Calls a routine and, if the result is not negative one, stores it in a memory location and sets a flag before returning the stored value.
- `preserve_previous_value`: Returns a previously stored word while updating that same location with a new value if a routine call does not return negative one.
- `call_indirect_subroutine`: Calls a subroutine using a pointer stored at a specific offset from a base address.
- `search_and_copy_string_short`: Calls a function and then searches for a string in a list to copy a 32-byte segment into a buffer.
- `search_and_copy_string_long`: Calls a function and then searches for a string in a list to copy a 20-byte segment into a buffer.
- `modify_structured_field`: Retrieves three values via indirect calls and updates a structure field using a concatenated 32-bit value.
- `execute_indirect_calls`: Calls three different subroutines via pointers stored in a table and passes one return value to a fourth subroutine.
- `check_string_existence`: Retrieves an index, calculates a pointer to a string in a table, and calls another routine if the string is not empty.
- `invoke_entity_constructor`: Calls a routine to initialize an entity by passing a pointer to a data structure, a value from a table, and a concatenated 32-bit value derived from two 16-bit fields in a nested table.
- `filter_nodes_with_offset`: Retrieves two values via indirect calls and, if the second is zero, fetches a short from an offset of a pointer stored in a global variable before passing the results to another routine.
- `filter_nodes_with_offset_2`: Retrieves two values from a function call and, if the second value is zero, fetches a short from an offset of a pointer stored in a global variable before passing the results to another routine.
- `call_with_offset_argument`: Calls a subroutine using a pointer located at a specific offset from a base address, passing an offset value as an argument.
- `update_table_values_2`: Calls a subroutine, stores the result in two consecutive memory locations, and conditionally loads a value from a table if a specific entry is zero.
- `reset_attribute_state`: Sets a global word to zero and, if a global pointer is non-zero, calls a function at that pointer, clears the pointer, and resets a global word to all ones.
- `get_and_store_short`: Calls a subroutine and, if the returned value is non-negative, stores it in a global variable before returning that variable's current value.
- `call_other_routine_3`: Calls another routine and returns zero.
- `store_dual_values`: Calls a subroutine twice, performs a conditional check on the second result, and stores both results into two global variables.
- `copy_conditional_code_2`: Copies a sequence of instructions to a destination address based on a range check of a retrieved value.
- `copy_code_block`: Copies a sequence of instructions from the current location to a destination address until a null terminator is encountered.
- `update_index_if_valid`: Checks if a value returned by a subroutine is within a specific range and points to a non-zero entry in a table, then updates a global variable and a flag.
- `calculate_quotient_remainder`: Calls two subroutines and combines the quotient and remainder of their results into a single 32-bit value.
- `clear_and_copy_code`: Calls two subroutines, copies a block of code to a destination, and then clears two global variables by setting them to zero.
- `clear_global_variables`: Calls a subroutine and sets two global variables to zero.
- `validate_table_index`: Checks if a provided index is within a valid range and verifies that a pointer at the corresponding table location is non-zero.
- `check_table_pointer`: Checks if a pointer stored in a table at a specific index is non-zero.
- `return_zero_4`: Returns the integer value zero.
- `clamp_and_update_value`: Calls a function and uses its return value to update a global variable with a value clamped between zero and sixty-three.
- `compare_strings`: Retrieves a pointer to a string and compares it against a global string to check for equality.
- `return_zero_5`: The routine performs no operations and returns zero.
- `get_long_value`: Returns a long value from a global variable.
- `copy_conditional_byte_array`: Checks a condition via a global short value, then copies a sequence of bytes from a source table to a destination buffer.
- `copy_memory_block`: Copies a sequence of bytes from one memory location to another using a loop controlled by a byte value.
- `update_and_return_byte`: Retrieves a byte from a structure and potentially updates it with a value from a function call if it falls within a specific range.
- `return_zero_2`: Clears the accumulator and returns zero to the caller.
- `write_function_result`: Calls a function twice and, if a global flag is non-zero, stores the first result into a buffer at an offset determined by the second result.
- `update_stored_values`: Calls a function and updates two different global variables with the returned value, while also returning the previous value of one of them.
- `get_word_value`: Returns a word value from a global variable.
- `execute_indirect_calls_alt_2`: Performs multiple indirect function calls using a pointer retrieved from a memory offset and passes a value to another routine.
- `store_multiple_function_calls`: Calls a function three times and stores each result into three consecutive word locations in a global structure.
- `get_nth_list_element_with_increment`: Retrieves the Nth element of a linked list by comparing a value returned from another routine against a counter.
- `find_list_item_incremented`: Traverses a linked list to find the Nth element and returns a value containing its pointer and an incremented word from an offset.
- `traverse_list_and_increment`: Iterates through a linked list for a specified number of steps and returns a value composed of a pointer and an incremented word from the list.
- `traverse_linked_list_4`: Iterates through a linked list for a specified number of steps and returns a value composed of a pointer and an incremented word from the final node.
- `return_zero_6`: This routine is empty and simply returns a value of zero to the caller.
- `get_list_element_with_incremented_value`: Iterates through a linked list to find the Nth element as determined by a value returned from another routine.
- `find_list_item_with_offset`: Traverses a linked list to find the Nth element and returns a value containing its pointer and an incremented word from a different offset.
- `traverse_list_with_offset_2`: Iterates through a linked list for a specified number of steps and returns a value composed of a pointer and a word from an offset within the list.
- `traverse_list_with_offset`: Iterates through a linked list for a specified number of steps and returns a value composed of a pointer and a word from an offset within the final node.
- `return_zero_7`: This routine performs no operations and returns a zero value to the caller.
- `check_global_boolean`: tests if a value stored at a specific offset from a pointer is non-zero.
- `return_zero_8`: This routine is empty and returns zero to the caller.
- `update_two_shorts`: Calls a function and, if the result is non-negative, updates two different global short variables with that result.
- `execute_three_functions`: Calls three different function pointers stored within a global structure.
- `compare_short_values`: Compares two short values stored at offsets from a pointer and returns the result of a comparison.
- `return_one_2`: returns the constant value one.
- `fetch_word_from_offset`: Returns a word stored at a specific offset from a value in a register.
- `update_word_via_call`: Calls a function through a pointer and stores the resulting word into a memory location.
- `execute_table_calls`: Calls a series of function pointers retrieved from a table and conditionally calls another function if a value equals negative one.
- `conditionally_update_word`: Fetches a word from a location, calls a function, and updates the original location if the function result is not negative one.
- `call_and_return_stack`: Calls a function through a pointer and returns a value from the stack.
- `execute_indirect_function_2`: Calculates an address by applying a negative offset to a value in a register and then calls the function located at that destination.
- `execute_indirect_pointer`: Calculates an address by adding a negative offset to a value in a register and executes the routine located at that destination.
- `compare_string_table_2`: Iterates through a table of null-terminated strings to find a match for a provided input string.
- `search_string_table`: Searches through a table of strings to find a match for a given string, returning a combined value containing the index and a result.
- `call_with_argument`: Calls a function through a pointer and passes a calculated memory address as an argument.
- `copy_string_from_table`: Copies a null-terminated string from a table into a destination buffer based on an index, returning one if the string was not empty and zero otherwise.
- `return_zero_9`: This routine performs no operations and returns zero to the caller.
- `return_zero_10`: The routine is empty and simply returns a zero value to the caller.
- `update_data_table_2`: Calls several functions to retrieve values and stores them into a table at an offset determined by a retrieved index.
- `update_data_table`: Calls a function and stores the result along with other register values into a table.
- `reset_hardware_values`: Checks a function result and either fills a series of memory locations with a constant value or updates a table and executes a function if a global variable is zero.
- `set_table_value`: Stores a constant 16-bit value into a table and conditionally calls another routine if a specific global value is zero.
- `process_conditional_table_entry`: Checks a table entry based on a value, calls a function with calculated parameters, and updates a table with results if certain conditions are met.
- `call_twice_and_return`: Calls a function through a pointer twice and returns the value fetched before the calls.
- `call_parameterized_function`: Checks a word in a structure against two constants and calls a function with specific arguments if a match is found.
- `call_if_value_matches`: Checks if a specific word in a structure equals a constant and, if so, calls a function with three arguments.
- `call_function_with_zero`: Checks a word in a structure against two constants and calls a function with a zero as the third argument if a match is found.
- `call_if_value_matches_zero`: Checks if a specific word in a structure equals a constant and, if so, calls a function with a zero as the third argument.
- `copy_array_via_pointer`: Retrieves a count and a series of values from a pointer-based source to populate a local stack array.
- `update_structure_state_2`: Calls multiple routines to gather data, inserts a node into a linked list, and updates structure fields based on specific values.
- `update_structure_state`: Evaluates conditions based on structure fields to execute function pointers, update a structure field, or decrement a value.
- `update_structure_field`: Updates a field within a data structure and conditionally executes a function pointer and decrements a value in a global variable.
- `execute_conditional_calls_2`: Checks a bit in a global byte and, if not set, calls three functions and decrements a value.
- `execute_two_calls`: Calls two functions and then calls a third function using a value from the first call as an argument.
- `execute_indirect_sequence`: Performs a sequence of indirect function calls based on a returned value and a second value retrieved from a function call.
- `call_subroutine`: performs a direct call to another routine.
- `execute_conditional_calls`: Calls another routine and, if the return value is non-zero, calls that same routine twice more.
- `process_string_result`: Calls a function and, if the result is non-zero, passes that result to two subsequent function calls.
- `update_global_pointer`: Calls a function through a pointer and stores the returned word into a global variable.
- `iterate_and_process_buffer`: Checks if a value retrieved from a pointer is within a specific range and then executes a loop that calls a function for each byte in a sequence.
- `loop_and_clear_byte`: Executes a loop of function calls, clears a byte in a structure, calls another routine, and returns a value from a function call.
- `swap_table_entry`: Calls a function to get a value, retrieves an index from a table, and swaps a word at a calculated offset within another table.
- `copy_string_data`: Calls two routines to obtain pointers and then copies characters from the first pointer to the second until a null terminator is found.
- `call_and_pass_arguments`: Calls two routines and passes their return values as arguments to a third function call.
- `execute_conditional_table`: Populates a local array with values from a function call and either executes a function with a constant or copies a sequence of bytes to a stack and executes it.
- `copy_and_execute_table`: Iterates through a loop to store function results into a table, then either executes a function with a constant or copies a block of memory and executes multiple functions via pointers.
- `copy_and_call_functions`: Copies a block of memory from one location to another and then executes several functions via pointers retrieved from a table.
- `conditionally_call_routine`: Calls a routine and then calls another routine using the first result if a specific global word is non-zero.
- `execute_concatenated_calls`: Executes a function pointer twice and passes the results to another routine that concatenates them into a single value.
- `rotate_word_value`: Retrieves a word from a global variable, calls a function to get a new value, stores the new value in the variable, and returns the original word.
- `get_global_word`: Returns a word stored at a specific offset from a global pointer.
- `copy_conditional_code`: Checks if a value is within a specific range and points to a non-zero address, then calls a function and copies a block of code to a new location.
- `update_function_pointer`: Retrieves a value from a memory location, executes a function pointed to by a different location, and stores the function's return value back into the original memory location.
- `update_function_pointer_2`: Retrieves a value from a memory location, executes a function pointed to by a different location, and stores the returned value back into the original memory location.
- `increment_and_fetch_table_value`: Compares two values, and if they differ, fetches a word from a table based on an index and increments that index with carry handling.
- `toggle_global_word`: Checks the return value of a routine and updates a global word to either zero or one based on that result.
- `clear_global_word`: Sets a word at a specific offset from a global pointer to zero.
- `update_stored_value`: Calls a function via a pointer and updates a global variable with the result if it is not negative one.
- `clear_multiple_pointers`: Checks several global pointers, calls the functions they point to, and resets the pointers and associated values to zero or a specific constant.
- `reset_callback_table`: Clears a block of memory and then populates a large table of function pointers with either the current routine or a global variable, depending on a bitwise check of a flag in a structure.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into -(a5).
- `call_indirect_via_pointer`: Checks if a passed pointer is non-zero and if its contents are non-zero before calling a function via an offset from a global pointer.
- `call_indirect_via_offset_2`: Checks if a global variable is non-zero and then calls a function using an offset from a pointer stored in another global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_window_structure`: Initializes a structure with specific values and a magic constant, then calls another routine to populate it with data retrieved from a library.
- `build_file_list_structure`: Iterates through a linked list of memory blocks to count elements and then populates a new structure with pointers and metadata.
- `get_current_directory`: This routine calls a system function to retrieve the path of the current working directory.
- `adjust_character_value`: Subtracts thirty-two from a sixteen-bit value if it falls within a specific range.
- `find_string_occurrence`: Searches for a specific character within a string and returns a pointer to its location.
- `memmove_block`: Copies a block of memory from a source to a destination, handling overlapping memory regions correctly.
- `clear_memory_and_check_status`: Clears a block of memory by looping through a range and setting bytes to zero.
- `return_from_subroutine_3`: This is an empty routine that performs no operations before returning.
- `tokenize_string_into_list`: Parses a string into individual words by handling whitespace and quoted substrings.
- `strncat_limited`: Appends a source string to the end of a destination string up to a specified maximum number of characters.
- `copy_string_with_limit`: Calculates the length of a source string and then copies it to a destination buffer.
- `retrieve_window_attributes`: Attempts to open a window and retrieve its attributes, such as dimensions and position, from a library.
- `format_bounded_string_into_buffer`: Formats a string into a buffer with a specified maximum length and returns the written length as a 16-bit value.
- `copy_string_to_null`: Locates the null terminator of the first string parameter by incrementing a pointer until a zero byte is found and then copies the second string there.
- `copy_string_or_pad_3`: Copies characters from a source string to a destination buffer up to a specified count or until the source ends.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_structure_member`: Retrieves a value from a library call and populates several fields within a structure, including an integer, a byte, and a pointer.
- `update_structure_fields_2`: Modifies specific bytes and words within a structure and executes code via pointers retrieved from a global table.
- `assign_structure_values`: Sets a byte, a word, and an integer within a structure retrieved from a library call.
- `initialize_structure_and_call_pointer`: If the provided parameter is non-zero, the routine writes a byte and two longwords into a structure at specific offsets and then executes a function via a pointer retrieved through multiple levels of indirection from a global variable.
- `initialize_and_call_function`: Stores three values into a structure at specific offsets and then calls a function through a pointer.
- `parse_command_string`: Parses a character buffer to identify and extract command tokens, numeric values, and flags.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: Calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `calculate_string_length_2`: Receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `search_linked_list_nodes_2`: Iterates through a linked list of structures, checking specific bit flags and calling another routine for each entry until a termination condition is met.
- `search_linked_list_nodes`: Iterates through a list of data structures, checking specific bit flags and calling another routine until a termination condition is met.
- `call_read_buffered_string`: This routine calls another routine and then calls a function to read characters from a memory buffer into a destination pointer.
- `read_buffered_string`: Reads characters from a memory buffer into a destination pointer while managing buffer offsets and status flags.
- `initialize_linked_list_node`: Sets specific bitmask values and pointers within a structure, potentially calling another routine to assign a value to a field.
- `remove_linked_list_node`: Traverses a linked list to find a specific node and removes it by updating the preceding pointer to point to the next node in the sequence.
- `search_linked_list_and_assign`: Traverses a linked list to find a matching pointer and then assigns a value to either a global variable or a specific location within a structure before calling a function.
- `assign_value_to_structure`: Assigns a 32-bit value from a pointer to either a global variable or a location within a structure, then calls a function via a pointer.
- `store_and_call_function`: Stores a 32-bit value from a pointer into a structure and then calls a function through a pointer.
- `clear_linked_list_elements`: Iterates through a linked list by following pointers and executes a routine via a function pointer stored in a global table for each node.
- `traverse_linked_list_2`: Iterates through a linked list by following pointers until a null pointer is encountered, then clears a value in a global variable.
- `insert_into_linked_list`: Retrieves a pointer from a function call and inserts a new node into a linked list by updating various pointers.
- `copy_string_simple_2`: Copies characters from a source string to a destination buffer until the null terminator is copied.
- `initialize_node_member`: This routine is called by a function that initializes a linked list node.
