# Game 434 (Sim City 2000): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00434). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2712384 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 103 of 103 named (0 author symbols), 103 of 103 described (2 by rule, 98 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2184 of 2186 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5535 of 5537 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3254 |
| code | 16890 |
| data | 2409336 |
| padding | 3352 |
| unused | 279552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Delete | 1 | 600 | 3 |
| Join | 1 | 363 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 2 | 283 | 4 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer | 6 | 45 | 2 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:decrunchall | 3 | 2015 | 40 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE | 3 | 160 | 5 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1324 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 501 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:sc1: high-entropy data file (7.99 bits/byte), classified as packed data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 2 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 2 of 3)[HD].adf:sc2: high-entropy data file (7.79 bits/byte), classified as packed data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 3 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 3 of 3)[HD].adf:sc3: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address (play-rs-m1500): 1144 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 356 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sim city 2000 (1993)(electronic arts)[cr psg](disk 2 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 2 of 3)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- sim city 2000 (1993)(electronic arts)[cr psg](disk 3 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 3 of 3)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_hardware` (runtime segment an address (play-rs-m1500)): Allocates memory for an audio buffer and configures hardware registers based on detected chipset capabilities. Kind: disk hardware/track/sync check. Pass path: u9__l0_0F0C.
- `copy_and_execute_subroutine` (runtime segment an address (play-rs-m1500)): Copies a block of memory from a source table to a destination, then executes a series of indirect function calls and modifies interrupt and DMA control registers. Kind: self-modifying code/data copy and execution. Pass path: The loop copies a block of data (puVar3 to puVar4) and then executes a subsequent branch/call. A successful check or valid copy allows the execution to proceed to the subsequent function calls (e.g., u9__sub_105A or the indirect calls at th.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: copies a longword from one data register to another and then performs a short branch to another routine.

### Delete

- `parse_device_string_list`: Parses a list of colon-separated strings to identify device names and copies them into a new table.
- `delete_file_entry`: Attempts to delete a file entry and reports whether the operation succeeded or failed.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by two registers character by character.

### Join

- `initialize_file_copy_operation`: Initializes a file-related operation by checking library status, clearing a local buffer, and attempting to open or manipulate file handles or directory structures.
- `validate_range_and_size`: Calls several library functions to check if a value or range is valid and returns a specific error code if certain conditions or size checks fail.

### endcli

- `find_max_and_initialize_buffer`: Iterates through a linked list to find the maximum value in a field, then initializes a buffer or table with a header and a sequence of incrementing values.
- `move_and_branch`: Moves a longword from one register to another and then performs a short branch to a fixed offset.

### loadwb

- `run_loadwb_with_switches`: Clears a local 16-byte buffer and parses command-line switches from an argument template.
- `initialize_library_data`: Checks a structure for specific bit patterns and then copies a block of instructions to a newly allocated memory area.
- `check_register_value`: Checks if a register contains a specific value and clears a bit in a global variable or returns a pointer to a constant or a nearby address.
- `get_pointer_from_library`: Calls a library function to retrieve a pointer and then conditionally returns a pointer to a global variable or a pointer retrieved from another library call.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:decrunchall

- `allocate_and_initialize_string_buffer`: Calculates a required buffer size based on a pointer or a length field, then allocates memory and initializes a string or buffer with a null-terminated or length-prefixed sequence.
- `execute_callback_chain`: Updates a pointer in a memory block, checks a condition involving a global variable, and executes a sequence of function pointers retrieved from a global table or other memory locations.
- `parse_string_table_and_load_file`: Iterates through a null-terminated string table, performing a series of function calls or error-handling routines based on string content, and subsequently copies a string or buffer into a calculated memory location before calling a routine to finalize a file-related operation.
- `initialize_global_parameters`: Sets several global variables to zero or to values passed via parameters and calls a routine to allocate or initialize memory or data structures.
- `iterate_and_execute_function_list`: Increments a pointer through a list of function pointers and executes each non-null function if a certain flag or an initial condition is met.
- `call_indirect_handler_return_constant`: The routine calls an indirect handler through a pointer stored in a global variable, passing the address of a global variable, then returns a constant value.
- `invoke_global_callback`: The routine calls a function pointer stored in a global table, passing the address of a global variable as an argument, and then returns.
- `traverse_linked_list_and_execute`: Iterates through a linked list of pointers, executing a function pointer stored in a global variable for every node in the list, and clears the head pointer or list head upon completion.
- `copy_string_to_buffer`: Copies bytes from a source pointer in a register to a destination pointer in a register until a null byte is encountered, returning the original destination pointer.
- `initialize_file_system_structures`: Initializes a set of pointers and status flags within a data structure, including the setup of linked list nodes and string buffer pointers.
- `move_and_load_pointer`: Moves a value from a register to another register and loads a new address into a register from the stack.
- `allocate_memory_block`: Checks a flag in a structure and calls a memory allocation routine to assign a pointer to a memory block.
- `iterate_linked_lists`: Iterates through two different linked lists, performing conditional calls to another routine or a library call based on the contents of the nodes.
- `traverse_buffer_and_set_status`: Traverses a linked list while checking certain bits in a buffer, updating a global status or a field in a structure with a return value or a constant.
- `test_and_update_stack`: Tests a bit in a register and performs a series of move operations using values or addresses from the stack.
- `validate_library_status`: Recursively calls itself and checks the return value to either update a global status or update a field in a structure using a value from a library call.
- `execute_recursive_call`: Performs a single recursive call to itself.
- `parse_string_tokens`: Parses a string or quoted string, handling escape sequences and replacing certain characters, while counting tokens and allocating a buffer via a library call.
- `conditionally_free_memory`: Checks if two global pointers are non-null and, if so, calls a library function to free memory.
- `recursively_validate_task_pointer`: Checks a pointer or task-related structure for a non-zero value and recursively calls itself if certain conditions or return values are met.
- `traverse_list_with_bounds_check`: Iterates through a linked list while decrementing a counter, updating a global variable with a status code based on whether the list or counter or a null pointer is reached.
- `update_task_pointer`: Checks a pointer or task-related structure and, if certain conditions are met, updates a function pointer and recursively calls itself.
- `find_byte_in_buffer`: Searches a memory buffer for a specific byte value.
- `update_linked_list_node_bounds`: Iterates through a linked list of memory blocks to find a block containing a specific pointer, then updates the block's size and potentially modifies the list or calls a function if certain size or pointer conditions are met.
- `allocate_and_partition_memory_block`: This routine searches a linked list of memory blocks to find or create a free segment that can accommodate a requested size.
- `load_stack_pointer_to_register`: Loads a longword from a location pointed to by the stack pointer into a register.
- `traverse_function_list`: Iterates through a linked list of function pointers and executes each one before performing a recursive call.
- `process_string_input`: Iterates through a null-terminated string, updating a character buffer and managing its length and pointer.
- `update_input_buffer`: Updates a character buffer by storing a byte from a register and managing buffer pointers and length counters.
- `process_ride_object`: Walks a linked list via a global pointer to check for certain flags, or if a non-zero argument is provided, clears fields in a structure and conditionally updates flags or counters.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `call_subroutine_and_return`: Calls another routine and then returns.
- `initialize_global_state`: Assigns parameters to global variables and calls several subroutines to allocate or free memory or initialize data structures.
- `branch_to_offset_2`: Performs a relative branch to a specific offset in the instruction stream.
- `initialize_lookup_tables`: Initializes memory buffers with a sequence of incrementing or decrementing values and populates a table-based lookup structure using nested loops and conditional checks.
- `fill_memory_with_nested_loops`: Uses nested loops to write certain byte values to a memory area using a pointer.
- `update_counter_and_pointer`: Compares two global variables and, if equal, decrements a global counter by a certain amount or a fixed value.
- `call_function_and_clear_memory`: Calls a function pointer stored in a global variable and then clears a memory location via a pointer.
- `calculate_crc_table`: Calculates a table of values using a bit-shifting loop and stores the results in a global table.
- `update_checksum_with_table`: Updates a global variable by performing a bitwise operation using a value from a table and a memory pointer.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE

- `initialize_program_entry`: The routine calls another routine multiple times and invokes a function pointer stored in a global variable.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `update_global_array_from_table`: Iterates through a table of pointers and counts to copy or modify values in a global array.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using an index passed in a register, and returns it ANDed with the input word.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_registers_to_constants`: Loads a constant value into a register and clears another register.
- `execute_library_calls`: This routine executes two library calls and is entered via an absolute move instruction.
- `traverse_and_process_linked_list`: Iterates through a linked list of memory blocks, performing conditional recursive calls and status checks.
- `fetch_and_update_data`: Loads a pointer from a global location, then uses that pointer to read a word and a byte, potentially updating a global pointer with a new address and a word value.
- `update_pointer_and_values`: Increments a global pointer and updates several global variables based on the value retrieved from the previous pointer location.
- `populate_lookup_tables`: Fills four different memory areas with values derived from a decreasing arithmetic sequence.
- `scale_source_table`: Calls another routine and then iterates through a loop to scale values from a source table.
- `swap_and_store_values`: Swaps the contents of two global variables and stores the new value into several other global locations.
- `render_glyph_bitmaps`: Iterates through a string of characters to fetch corresponding glyph data from a font table and performs bitwise expansion to convert compressed font data into a larger bitmap format in a destination buffer.
- `update_bitmask_tables`: Updates multiple bitmask tables by setting or clearing specific bits based on a state flag and a calculated index.
- `update_structure_data`: Iterates through a table of values and writes specific elements to a structure in memory.
- `generate_lookup_tables`: Populates multiple memory buffers with calculated values using nested loops, involving bitwise masking, bit-shifting, and arithmetic offsets.
- `initialize_grid_table`: Populates a memory area with a pattern of incrementing values in a nested loop structure.
- `interpolate_and_shuffle_table`: Calculates interpolated coordinate values through a loop and updates a set of global variables.
- `update_status_flags`: Waits for a specific bit in a memory location to clear before writing a series of bytes and a longword to a data structure.
- `initialize_interpolation_tables`: Populates several global tables with values derived from a source table and constant values.
- `configure_hardware_parameters`: Writes several sets of configuration data to a structure and updates pointers to a table.
- `interpolate_table_values`: Calculates a sequence of values using a loop and stores them into a table, then copies values from another memory location into a destination table.
- `process_coordinate_list`: Calculates differences between values in a list and writes the results to a structure.
- `initialize_audio_registers_and_bitmask`: Writes a series of values to a memory-mapped structure and waits for specific status bits to clear.
- `set_pointer_table`: Stores several calculated offsets from a global variable into a series of memory locations.
- `rotate_buffer_pointers`: Shifts values between three memory locations and updates a structure with new pointer data.
- `interpolate_coordinates`: Compares two sets of values and increments or decrements them to move them closer together.
- `reset_and_update_status_flag`: Clears a global variable, calls another routine, and then writes a specific word value to a memory location pointed to by a register.
- `check_status_flag`: Checks a bit in a structure and writes a value to a different offset if the bit is set.
- `initialize_audio_hardware`: Allocates memory for an audio buffer and configures hardware registers based on detected chipset capabilities.
- `decrement_counter`: Decrements a value passed in a register until it reaches negative one.
- `wait_for_bitmask`: Enters a loop that waits until a specific bit pattern is present in a memory location.
- `copy_and_execute_subroutine`: Copies a block of memory from a source table to a destination, then executes a series of indirect function calls and modifies interrupt and DMA control registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a short branch to a relative offset.
- `branch_to_target_routine`: Performs a short branch to a location within another routine.
- `reset_global_state`: Clears several memory locations or global variables to zero and modifies a global variable using a bitwise mask.
- `decode_string_stream_opcodes`: Decodes a custom-encoded instruction stream by interpreting bit-masked opcodes to perform pointer arithmetic, table lookups, and conditional updates to a destination buffer.
- `process_string_stream`: Iterates through a buffer to decode or decompress a stream of bytes, performing bitwise negation and pointer increments based on certain byte values.
- `move_and_branch_2`: Moves a byte from an offset of a pointer to another offset of the same pointer and then performs a relative branch.
- `decode_string_stream_buffer`: Decodes or decompresses a byte stream by updating pointers and copying data from a source buffer.
- `decode_string_stream_buffer_2`: Decodes a compressed or encoded byte stream by performing bitwise nibble swapping and pointer-based data extraction.
- `update_string_stream_control_bytes`: Reads a control byte from a buffer and updates a destination or state table with a modified or decremented value.
- `update_interpolated_coordinate_table`: Updates a table of coordinate or offset values by performing bitwise shifts and arithmetic or lookups based on a control byte.
- `configure_dma_transfer`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers.
- `load_audio_register_values`: Loads several 32-bit or 16-bit values from a series of memory locations into a block of RAM and sets a global variable to a fixed value, while modifying the low 16-bits of the first audio channel and the interrupt clear register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
