# Game 1111 (sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01111). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910144 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
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
| data | 862005 |
| padding | 2907 |
| unused | 25088 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Delete | 1 | 600 | 3 |
| Join | 1 | 363 | 2 |
| endcli | 2 | 117 | 2 |
| loadwb | 2 | 283 | 4 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer | 6 | 45 | 2 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:decrunchall | 3 | 2015 | 40 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE | 3 | 160 | 5 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1324 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 501 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:sc1: high-entropy data file (7.99 bits/byte), classified as packed data
- runtime segment an address (play-rs-m1500): 1144 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 356 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_hardware` (runtime segment an address (play-rs-m1500)): Allocates memory for an audio buffer and configures hardware registers based on detected chipset capabilities. Kind: disk hardware/track check. Pass path: the branch leading to u9__l0_0F0C (the main initialization logic).
- `copy_and_execute_functions` (runtime segment an address (play-rs-m1500)): Copies a block of data from a table to a destination and then executes several functions via pointers stored in memory and global variables. Kind: loader with integrated decryption/unpacking and integrity checks. Pass path: The loop copies data from a source to a destination (likely a decrypted buffer), followed by a series of indirect function calls that validate the environment or state. A successful path continues through the sequence of indirect calls (the.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Copies the contents of one data register to another and then performs a short relative branch to another location.

### Delete

- `delete_file_via_dos_command`: Constructs a DOS command string to delete a file, including parameters for all files, quiet mode, and force, while checking if the target is a device to prevent illegal deletion attempts.
- `delete_file_entry`: Attempts to delete a file entry by clearing specific flags and updating size and type fields in the entry structure.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by two registers character by character.

### Join

- `manage_file_system_operations`: Performs file system tasks including checking for file existence, removing destination files, and managing file handles through a series of conditional checks and error handling.
- `validate_range_loop`: Performs a loop that compares a parameter against values retrieved from a structure, returning specific error codes if conditions are not met.

### endcli

- `find_max_and_populate_buffer`: Traverses a linked list to determine the maximum value in a specific field and then populates a memory buffer with a header and a sequence of incrementing values.
- `move_and_branch`: Moves the contents of one register to another and then performs a short relative branch.

### loadwb

- `run_loadwb_with_switches`: Clears a 16-byte local buffer and parses command line switches including DEBUG, DELAY, CLEANUP, and NEWPATH. It then calls a routine to load the workbench catalog and returns a status code.
- `load_workbench_catalog`: Checks for a specific catalog version and bitmask in a structure, then allocates memory to copy a block of code and populate it with data retrieved from library functions.
- `test_and_clear_bit`: Checks a value in a register against two constants and clears a specific bit in a memory location if the value equals eight.
- `get_pointer_from_library`: Calls a library function to retrieve a pointer and conditionally modifies it based on a global variable and a constant value.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:decrunchall

- `initialize_memory_and_load_config`: Clears a memory block and calculates stack or buffer offsets based on a structure's size.
- `execute_callback_chain`: Updates a memory location with a passed parameter and executes a series of conditional function calls retrieved from a global table and various memory offsets.
- `parse_config_file_entries`: Iterates through a null-terminated list of strings to process configuration parameters.
- `initialize_system_parameters`: Sets several global variables using passed parameters and then calls a sequence of functions via pointers stored in a global structure.
- `execute_chained_callbacks`: Iterates through a list of pointers, incrementing a pointer by four each step and calling the function pointed to until a null pointer is encountered.
- `call_indirect_handler_return_constant`: The routine calls an indirect handler through a pointer stored in a global variable, passing the address of a global variable, then returns a constant value.
- `invoke_global_callback`: The routine calls a function pointer stored in a global table, passing the address of a global variable as argument.
- `traverse_and_free_list`: Iterates through a linked list of pointers and calls a memory freeing routine for each element.
- `copy_string_to_buffer`: Copies bytes from a source pointer in A1 to a destination pointer in A0 until a zero byte is encountered, writing each byte to the destination and advancing both pointers.
- `initialize_file_system_context`: Initializes a data structure by setting up several linked lists and pointers, including path strings and memory management metadata.
- `move_and_load_pointer`: Moves the contents of register D0 to D5 and loads a value from a stack offset into A0.
- `initialize_memory_block`: Checks a bit flag in a structure and, if not set, calls a memory allocation routine to populate several fields within that structure.
- `process_linked_lists`: Iterates through two different linked lists, conditionally calling functions based on bitwise checks of the data within each node.
- `process_status_and_traverse_list`: Evaluates bitmask flags in a returned structure to determine whether to update a global variable, call a function via a pointer in a table, or enter a loop that traverses a linked list while performing arithmetic on a value.
- `test_bit_and_rearrange_stack`: Tests a bit in a register and then performs a sequence of moves that reorders three values from the stack, effectively replacing one stack parameter with another.
- `handle_io_error_status`: Updates a global variable with a constant if a function returns zero, otherwise it checks a pointer and conditionally calls a function based on a parameter value.
- `execute_callback_chain_2`: This routine updates a memory location with a provided parameter and then executes a sequence of conditional function calls retrieved from a global table.
- `parse_config_string_tokens`: Parses a configuration string by skipping whitespace and handling quoted substrings.
- `conditionally_free_memory`: Checks if two specific memory addresses contain non-zero values, and if so, calls a memory freeing routine.
- `process_callback_logic`: Evaluates conditions based on values retrieved from a pointer and executes a sequence of function calls.
- `traverse_linked_list_with_bounds`: Validates a count against a limit and then traverses a linked list a specified number of times.
- `update_callback_state`: Checks a bitmask in a global variable and, if a specific pointer is null, performs a series of conditional function calls and updates a pointer to another routine.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `unlink_memory_block`: Traverses a linked list of memory blocks to find a block containing a specific address, then removes that block from the list and updates the list pointers.
- `allocate_memory_block`: Searches a linked list of memory segments to find a block that can accommodate a requested size, potentially splitting an existing block into two smaller pieces.
- `move_stack_to_register`: Moves a long value from the stack into a data register.
- `traverse_callback_list`: Iterates through a linked list of pointers and executes the function located at the second word of each entry.
- `parse_input_string`: Iterates through a null-terminated string and copies characters into a destination buffer managed by a global structure.
- `process_text_input_buffer`: Processes a byte from a register using a structure pointed to by a register.
- `process_ride_object`: Walks a linked list via a global pointer when the first argument is zero and recursively processes each node, returning a non-zero flag if any call returns non-zero.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `call_parse_config_file_entries`: Calls another routine and then returns.
- `initialize_system_state`: Assigns parameters to global variables and performs a series of checks on returned values from a function.
- `branch_to_address`: Performs a short relative branch to a specific address.
- `initialize_lookup_tables`: Initializes several memory buffers with arithmetic sequences and pattern-filled data.
- `fill_memory_with_pattern`: Fills a memory area with repeating byte patterns using nested loops.
- `update_buffer_state`: Compares two global values and, if they are equal, performs arithmetic on a third global value and calls another routine.
- `call_function_pointer`: Calls a function pointer stored in a global variable and then clears a memory location via a pointer.
- `generate_crc_table`: Calculates and stores a table of 256 values using a bitwise CRC algorithm.
- `process_crc_data`: Updates a running value by performing bitwise operations and table lookups based on input data.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE

- `initialize_program_entry`: The routine calls another routine multiple times and invokes a function pointer stored in a global variable.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `update_entity_attribute_tables`: Iterates through a list of active entities to update their properties by applying values from a lookup table.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using an index passed in a register, and returns it ANDed with the input word.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_registers_with_constants`: Loads a constant value into a data register and clears another data register.
- `call_library_functions`: Calls two library functions via an absolute move.
- `traverse_and_process_linked_list`: Iterates through a linked list of memory blocks, performing conditional recursive calls and status checks.
- `pop_and_conditionally_replace_pointer`: Retrieves a word from the location pointed to by a global pointer and checks its sign.
- `update_pointer_and_values`: Increments a global pointer and updates several global variables based on the value retrieved from the previous pointer location.
- `populate_lookup_tables`: Fills four different memory areas with values derived from a decreasing arithmetic sequence.
- `scale_lookup_table`: Calls another routine and then iterates through a loop to scale values from a source table.
- `swap_and_store_values`: Swaps the contents of two global variables and stores the new value into several other global locations.
- `render_glyph_bitmaps`: Iterates through a string of characters to fetch corresponding glyph data from a font table.
- `update_bitmask_tables`: Updates multiple bitmask tables by setting or clearing specific bits based on a state flag and a calculated index.
- `update_structure_data`: Iterates through a table of values and writes specific elements to a structure in memory.
- `generate_lookup_tables`: Computes and populates multiple tables used for coordinate or color transformations through nested loops.
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
- `update_status_flag_and_pointer`: Clears a global variable, calls another routine, and then writes a specific word value to a memory location pointed to by a register.
- `check_status_flag`: Checks a bit in a structure and writes a value to a different offset if the bit is set.
- `initialize_audio_hardware`: Allocates memory for an audio buffer and configures hardware registers based on detected chipset capabilities.
- `decrement_counter`: Decrements a value passed in a register until it reaches negative one.
- `wait_for_bitmask`: Enters a loop that waits until a specific bit pattern is present in a memory location.
- `copy_and_execute_functions`: Copies a block of data from a table to a destination and then executes several functions via pointers stored in memory and global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset_2`: Performs a short relative branch to a specific instruction offset.
- `branch_to_offset_3`: Performs a short relative branch to a specific instruction offset within another routine.
- `initialize_data_structure`: Clears several fields within a data structure pointed to by a register and updates a global variable using a bitwise operation.
- `decode_interpolated_animation_stream`: Iterates through a compressed data stream to update animation frame parameters and coordinate values.
- `process_text_stream`: Iterates through a data structure to decode and advance a stream of characters.
- `branch_to_offset`: Moves a byte from an offset of a pointer to another offset of the same pointer and then performs a short branch to an address held in a register.
- `update_text_stream_state`: Manages the state and pointer progression of a character stream by decoding control bytes and updating buffer offsets.
- `decode_text_stream_metadata`: Processes a character stream by extracting control nibbles to update font indices, sprite offsets, and coordinate data.
- `update_control_byte_pair`: Reads a byte from a structure and updates two specific offsets within a target buffer based on its value.
- `process_interpolated_animation_frames`: Iterates through a data structure to update coordinate values and animation states based on a switch-case logic.
- `configure_dma_transfer`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers.
- `load_audio_register_values`: Loads several 32-bit or 16-bit values from a series of memory locations into a block of RAM and sets a global variable to a fixed value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
