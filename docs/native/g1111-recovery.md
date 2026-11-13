# Game 1111 (sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01111). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (920410 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 27 raw-word lines |
| Functions named and described | pass, 254 of 254 named (0 author symbols), 254 of 254 described (9 by rule, 242 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3920 of 3924 (99.90%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10544 of 10544 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5192 |
| code | 26870 |
| data | 860331 |
| padding | 2929 |
| unused | 25088 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| Delete | 1 | 600 | 3 |
| Join | 1 | 363 | 2 |
| endcli | 2 | 182 | 3 |
| loadwb | 2 | 301 | 5 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer | 6 | 250 | 17 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:decrunchall | 3 | 2015 | 40 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE | 3 | 160 | 5 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1324 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 632 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1315 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 623 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 64 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 225 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE#unpacked | 1 | 1946 | 56 |

## Cases handled

- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:sc1: PCompress LH Data (xfd)-packed, unpacked with ancient (753664 -> 6683 bytes)
- runtime segment an address (play-rs-m1500): 1144 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 356 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 1101 executed PCs whose RAM was overwritten before any snapshot match its bytes 524400-528672: recovered as a unit run at an address
- RAM snapshot an address: 402 executed PCs whose RAM was overwritten before any snapshot match its bytes 558272-562784: recovered as a unit run at an address
- 77 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 8 first-run image unit(s) in 1 version group(s); 1162 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer: Imploder (xfd) stream found by ancient scan (61704 -> 113740 bytes) but only 0 of 56 sampled windows are in reference RAM: not used
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE: crunched executable decrunched by XFDMaster (CrunchMania Normal (xfd), 18908 -> 50292 bytes), 12 of 13 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 branch displacement
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[a][hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_hardware` (runtime segment an address (play-rs-m1500)): Allocates memory for an audio buffer and configures hardware registers based on detected chipset capabilities. Kind: disk hardware/track check. Pass path: the branch leading to u9__l0_0F0C (the main initialization logic).
- `copy_and_execute_functions` (runtime segment an address (play-rs-m1500)): Copies a block of data from a table to a destination and then executes several functions via pointers stored in memory and global variables. Kind: loader with integrated decryption/unpacking and integrity checks. Pass path: The loop copies data from a source to a destination (likely a decrypted buffer), followed by a series of indirect function calls that validate the environment or state. A successful path continues through the sequence of indirect calls (the.
- `initialize_audio_hardware_2` (runtime segment an address (from file RAM snapshot an address +an address)): Allocates a memory buffer and identifies the chipset by checking Denise and custom register values. Kind: disk hardware/timing check. Pass path: The routine proceeds to initialize hardware (DMACON, ADKCON) and enters a loop/subroutine (u12__sub_0FCA) that manipulates CIAB_PRB (likely checking disk controller status/interrupts) after verifying the Denise ID and specific hardware regi.
- `update_custom_registers_and_free_memory` (runtime segment an address (from file RAM snapshot an address +an address)): Updates custom hardware registers by reading from the Amiga hardware registers, performs a 256-iteration memory copy from a table to a destination, and then closes a library and frees 1024 bytes of memory. Kind: disk hardware/register validation and memory cleanup. Pass path: The routine proceeds to clear custom registers (DMACON/ADKCON), free the loaded buffer via exec.FreeMem, and release the library via exec.CloseLibrary. A failure in the hardware/register state (ADKCON/DMACON) would likely occur before this .
- `initialize_audio_hardware_3` (PRESTiGE#unpacked): Opens a library, allocates a 1024-byte memory buffer, and detects the chipset type by checking Denise and custom chip registers. Kind: disk hardware/sync check. Pass path: The routine checks the status of the disk controller (ADKCON/DSKBYTR) and the CIAB_PRB (Chip Interrupt Abort/Status) register. It performs a loop waiting for a specific interrupt bit (`CUSTOM_INTREQR`) to be set, which is characteristic of .
- `reconfigure_hardware_and_manage_library` (PRESTiGE#unpacked): Disables interrupts and DMA, copies a 256-word block of data from a table to a destination, and then performs a series of library management operations including switching to supervisor mode, updating hardware control registers, and closing a library before freeing the copied memory block. Kind: loader/hardware reconfiguration trap. Pass path: The routine proceeds to reconfigure custom hardware registers (ADKCON, DMACON, COP1LCH) and then attempts to call functions from a library pointer. If the hardware state (ADKCON/DMACON) or the library structure is not exactly as expected du.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: Copies the contents of one data register to another and then performs a short relative branch to another location.
- `initialize_entity_state_structure`: Initializes a structure by setting several fields to zero, fetching a byte from a table using a value from the structure to store at offset 32, and updating a pointer at offset 36 from a table.
- `update_entity_state_and_call_handler_2_2`: Fetches a byte from a table using a value from a structure, stores it at offset 20 of that structure, and increments a value at offset 24.
- `update_entity_state_and_call_handler_2`: Calculates a new value based on a field in a structure, stores it at offset 22, and updates a secondary field at offset 32.
- `update_entity_state_and_call_handler`: Updates several fields in a structure pointed to by a1 using values from a table in a0, including setting a byte to 10 and clearing a longword at offset 12.
- `branch_with_offset`: Adds 1 to the value at offset 84 of a1 and branches to another routine.
- `test_status_bit_and_branch`: Tests a longword at offset 40 of the structure in a1; if non-zero, it branches to another routine, otherwise it branches to a third routine.
- `modify_status_flag_and_branch`: Checks a bitmask at offset 48 of a structure in a1; if the bitwise AND of the mask and a constant is non-zero, it performs a bitwise OR to set the upper 8 bits of the value at offset 48 and branches to another routine, otherwise it compares the value at offset 48 to a constant and branches to another routine.
- `update_entity_index_and_state`: Copies a longword from offset 64 to offset 108 of a structure, calculates a shifted index from a value at offset 1136, and calls another routine with several parameters including a constant 800 and a value from offset 52.
- `initialize_entity_structure_and_update`: Calculates an index into a table using values from a structure, stores a value at that index, and clears several fields in a target structure.
- `update_entity_state_and_decrement_counter`: Updates a structure by storing a value from a register into offset 200, then performs several conditional calls to other routines.

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
- `update_internal_state`: Performs several calls to another routine using various offsets from a structure in a2 and a1, modifies a value in a table at an offset calculated from a0, and ends with an indirect jump.

### loadwb

- `run_loadwb_with_switches`: Clears a 16-byte local buffer and parses command line switches including DEBUG, DELAY, CLEANUP, and NEWPATH. It then calls a routine to load the workbench catalog and returns a status code.
- `load_workbench_catalog`: Checks for a specific catalog version and bitmask in a structure, then allocates memory to copy a block of code and populate it with data retrieved from library functions.
- `check_and_set_pointer`: Compares d0 to 8 and 27, clears bit 1 of a byte at offset 44 of a6 if the first comparison matches, and otherwise returns the address of a label in d0 if the second comparison matches.
- `get_pointer_from_library`: Calls a library function to retrieve a pointer and conditionally modifies it based on a global variable and a constant value.
- `close_library_and_clear_pointers`: If a value at offset 50 in a6 is 1 and a global pointer is non-zero, it calls a library function to close the library and then clears three consecutive global pointers by writing zero to them.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:installer

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `execute_installer_sequence`: Performs several calls to a function at a pointer in a4 while incrementing d5 and d6, then calls another routine, and if d5 is 2, compares a value from a table at a pointer in a6 to 37 and calls a function at an offset of negative 636 from that pointer before returning.
- `process_data_loop`: Checks if a doubled value in d0 is negative; if so, it enters a loop that calls another routine, calls a function at a pointer in a4, and adds a value from a1 to a table at a pointer in a3 using an index derived from d0 and d2; otherwise, it calls another routine and branches.
- `conditionally_call_subroutine_with`: If a zero flag is set, it branches to a target offset; otherwise, it moves a pointer from a3 to a1, calls another routine, and then branches to a target offset.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_from_source`: Compares a pointer in a2 with a pointer in a6 and, if they are not equal, copies four bytes from the source pointed to by a2 to the destination pointed to by a1 using a loop that decrements a counter in d0.
- `decode_variable_length_bitstream`: Reads a control byte from a table to determine a bit-length and then performs a series of shifts and additions to reconstruct a value from a bitstream in a0.
- `process_bitstream_loop`: Decrements a counter in d7 and performs arithmetic on d1 and d0 using values from a byte at the address in a0, repeating the loop until d7 reaches zero before jumping to another routine.
- `initialize_registers_and_jump`: Sets d0 to 5, sets d7 to 3, and jumps to another routine.
- `initialize_registers_and_jump_2`: Sets d0 to 4, sets d7 to 2, and jumps to another routine.
- `initialize_registers_and_jump_3`: Sets d0 to 3, sets d7 to 1, and jumps to another routine.
- `extract_variable_length_bitstream`: Reads a variable number of bits from a bitstream in a0, using a control byte to determine bit lengths and a lookup table to handle bit-shifting and data accumulation.
- `process_bitstream_control_byte`: Increments a byte in register d1, and if not zero, branches to another routine; otherwise, it loads a byte from the address in a0 into d1, adds it to d1 with the carry flag, adds d2 to d2 with the carry flag, and branches to another routine.
- `decode_variable_bitstream_data`: Decodes a variable-length bitstream into words using a control byte to determine bit lengths, then performs a series of memory copies and data expansions based on values retrieved from a table.
- `decode_and_copy_compressed_data`: Reads a control byte to determine a bit-length and uses a loop to decode a variable-length bitstream into a word, then copies the resulting data from a source buffer to a destination buffer using a loop.
- `push_registers_and_branch`: Pushes bytes from d0, d1, and d2 onto a stack pointed to by a4, restores several registers from the stack, and branches to another routine.

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
- `update_interpolated_animation_stream`: Updates animation frame counters and playback offsets within a data structure, performs bitwise masking on control bytes, and calls configure_dma_transfer to initiate a DMA transfer after updating a hardware register.
- `decode_interpolated_animation_stream`: Iterates through a compressed data stream to update animation frame parameters and coordinate values.
- `process_text_stream`: Iterates through a data structure to decode and advance a stream of characters.
- `branch_to_offset`: Moves a byte from an offset of a pointer to another offset of the same pointer and then performs a short branch to an address held in a register.
- `update_text_stream_state`: Manages the state and pointer progression of a character stream by decoding control bytes and updating buffer offsets.
- `decode_text_stream_metadata`: Processes a character stream by extracting control nibbles to update font indices, sprite offsets, and coordinate data.
- `dispatch_text_stream_command`: Performs an indirect jump to a routine selected by a value in a table, then executes several branches that either update a counter in a structure, copy a block of data, or modify values at specific offsets in a structure based on a byte from a structure.
- `update_control_byte_pair`: Reads a byte from a structure and updates two specific offsets within a target buffer based on its value.
- `configure_dma_transfer`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers.
- `load_audio_register_values`: Loads several 32-bit or 16-bit values from a series of memory locations into a block of RAM and sets a global variable to a fixed value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `allocate_and_initialize_engine_resources`: Allocates several memory blocks using absolute and standard memory allocation, then enters a loop that waits for a specific bit in a hardware register and performs multiple calls to various subroutines, including coordinate transformations and data table updates, before freeing the allocated memory and returning.
- `update_pointer_from_table`: Fetches a word from a table pointed to by a global variable and, if the value is negative, loads a new pointer from a different table location and updates the global variable with it before branching to another routine.
- `update_pointer_and_calculate_offsets`: Increments a global pointer, potentially resetting it to a fixed address if the current value is negative, and updates a global variable by masking and combining bits from the retrieved value.
- `fill_symmetric_tables`: Runs a loop 255 times to populate four different memory areas with a sequence of values that decrease and increase symmetrically.
- `scale_source_table`: Calls another routine, stores a value in a global variable, and then iterates through a loop 64 times to scale values from a source table and store the results into a destination table in memory.
- `swap_and_distribute_modified_value`: Swaps the contents of two global variables and then distributes the original value, modified by adding 18430, into four separate 16-bit locations across two different global structures.
- `update_tile_graphics_buffer`: Iterates through a list of tile indices, calculates memory offsets for tile data, and performs bitwise operations to expand single-byte tile definitions into 8-bit planar graphics data within a destination buffer.
- `update_dual_bitmask_tables`: Calculates indices for two adjacent elements in a data structure and updates multiple bitmasks at specific offsets within those elements.
- `copy_data_to_structure`: Copies 98 words from a table pointed to by a0 into a structure in a6, updating specific offsets with values from d0, d1, and d3 during the loop.
- `generate_coordinate_tables_2`: Populates a series of tables using nested loops to perform coordinate transformations, involving arithmetic shifts, bitwise rotations, and multiplications by 112.
- `generate_coordinate_tables`: Fills a table with incrementing values in a nested loop and then populates another table by adding values from two source tables to a third.
- `update_interpolated_timer_and_shuffle`: Calculates new 32-bit values by adding constants to a value retrieved from a pointer, then updates a global variable and performs a loop that calls interpolate_16bit_fixed_point three times.
- `update_status_flags_2`: Waits for bit 6 of a byte at offset 0 of a structure in a6 to clear, then writes a longword from a global variable to offset 82, writes 24618 to offset 86, and performs several subsequent checks and writes of 24618 to offset 86 while waiting for bit 6 to clear at various offsets.
- `initialize_and_update_data_tables`: Performs several loops to swap and distribute values into global memory locations, copies data from a table to two other locations, calls two subroutines, and then checks a bitmask on a value from the stack; if the condition is met, it updates several global variables and pointers with specific constants and clears multiple 16-bit values.
- `populate_hardware_config_table`: Writes several sets of configuration data into a structure pointed to by a6 and updates several pointers within that structure to point to elements of a data table.
- `interpolate_and_copy_data`: Loops to update two 16-bit values at a structure offset by incrementing or decrementing them based on a counter, then performs a loop that calls another routine to interpolate values from one table into another table at a structure offset.
- `calculate_coordinate_deltas`: Iterates through a list of values, calculating the difference between consecutive entries and storing the result along with modified pointer values into a structure pointed to by a6.
- `initialize_data_tables_and_generate_bit`: Calls another routine, then populates several memory structures by performing bitwise rotations to create a sequence of bit patterns, copies data from a table into two parallel buffers, and fills a third buffer with a series of structured byte sequences based on a loop.
- `populate_offset_table`: Calculates a series of memory addresses by adding incremental offsets to a base value retrieved from a global variable and stores these resulting pointers into a destination table, then stores a final modified address at the end of the table before returning.
- `rotate_pointers_and_update_structure`: Rotates three 32-bit values in a circular buffer and updates a structure pointed to by a6 with new pointer data and specific 16-bit values.
- `interpolate_16bit_fixed_point`: Takes two 16-bit values and performs a step-wise convergence by incrementing or decrementing their constituent nibbles (bits 8-15, 4-7, and 0-3) independently to move the first value toward the second.
- `reset_status_and_set_hardware_registers`: Clears a 16-bit global variable, calls another routine, and then writes the values 32 and 49184 to offsets 156 and 154 of a structure pointed to by a6.
- `update_flag_on_bit_set`: Checks if bit 5 is set in a 16-bit word at offset 28 of a structure, and if so, writes the value 32 to offset 154 of that structure before calling another routine.
- `initialize_audio_hardware_2`: Allocates a memory buffer and identifies the chipset by checking Denise and custom register values.
- `compare_stack_byte`: Compares a byte at an offset in a6 with itself and returns immediately if they are equal, otherwise it loops based on a counter in d0.
- `wait_for_bitmask_2`: Enters a loop that waits until a specific bit pattern is present in a memory location pointed to by an offset of a pointer in a6, otherwise it returns.
- `update_custom_registers_and_free_memory`: Updates custom hardware registers by reading from the Amiga hardware registers, performs a 256-iteration memory copy from a table to a destination, and then closes a library and frees 1024 bytes of memory.
- `perform_arithmetic_and_logic`: Divides a word in d0 by d1, performs bitwise OR and AND operations on d0, and then performs a bitwise OR between d0 and the byte at the address pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset_4`: Performs a short branch to an instruction at a relative offset.
- `process_audio_and_hardware_state`: Updates a global counter and, if a condition is met, performs a series of data manipulations including bitwise operations on a structure, copying data from a table into a RAM block, and configuring hardware registers for audio volume, DMA, and interrupts.
- `reset_input_state_and_update_pointer`: Clears several 16-bit values at specific offsets within a structure pointed to by a6, modifies the second bit of the CIAA port A data register, and updates a global pointer using a value loaded from a table.
- `dispatch_data_stream_updates`: Iterates through a list of structures using a jump table based on a high nibble in a byte at offset 2, performing various arithmetic updates to counters, pointer dereferences, and data transfers between the structures and a global table.
- `decrement_structure_counter`: Checks a 16-bit value in a register; if non-zero, it extracts a value from offset 2 of a structure, subtracts it from the 16-bit value at offset 12 of that structure, and if the result is negative, clears the value at offset 12 before storing the new value at offset 8 of another structure, otherwise it returns.
- `update_data_stream_counters`: Iterates through a list of structures using a loop, performing a switch-based dispatch on a type field to update various counters, perform bitwise operations on values, or fetch data from a table into the destination structure.
- `process_data_stream_and_update_pointers_2`: Iterates through a list of structures using a loop, performing a switch-based dispatch on a type field to update various counters, perform bitwise arithmetic on bytes, and modify values in a destination structure.
- `process_data_stream_and_update_pointers`: Iterates through a list of structures, performing bitwise operations and conditional logic on bytes to update various pointers and counters.
- `copy_byte_and_branch`: Copies a byte from an offset of 2 relative to a pointer in a5 to an offset of 13 relative to the same pointer, then performs a short branch to the address stored in a0.
- `update_structure_pointers_from_stream`: Reads a byte from a stream to determine a base address and an index, then uses the index to fetch four 16-bit values from a table to update four specific pointer offsets within a structure.
- `swap_nibbles_in_structure_field`: Reads a byte from offset 2 of a structure, extracts its low and high nibbles, and swaps them into offset 22 of the same structure if certain conditions are met.
- `execute_jump_table`: Extracts a byte from a location offset by 2 from a5, masks it, shifts it right by 3, and uses the result to index into a table to perform an indirect jump.
- `update_structure_counters_and_copy_data`: Processes a list of structures by masking a byte from a pointer to determine a count, then either decrements a counter in a structure or performs a block copy of 12 bytes from a table into a destination structure.
- `update_counter_conditionally`: Masks a byte from offset 2 of the structure in a5 with 15, subtracts the result from a 16-bit word at offset 12 of the structure in a5, and if the result is not negative, clears the 16-bit word at offset 12 and branches to another routine.
- `store_masked_byte`: Masks a byte from offset 2 of the structure in a5 with 15, stores the result as a 16-bit word at offset 26 of the structure in a5, and branches to another routine.
- `store_masked_byte_a3`: Masks a byte from offset 2 of the structure in a5 with 15, stores the result as a 16-bit word at offset 78 of the structure in a3, and branches to another routine.
- `dispatch_data_stream_elements`: Iterates through a list of structures, using a type field to update specific bytes and counters in a target structure.
- `configure_dma_and_interrupt`: Sets a destination address and a transfer count in memory, configures the DMA controller to enable a channel, sets an interrupt request, and updates a control register before returning from an interrupt.
- `load_audio_register_values_2`: Loads several 32-bit or 16-bit values from a series of memory locations into a block of RAM and sets a global variable to a fixed value, while modifying the low 16-bits of the first audio channel and the interrupt clear register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_multiple_values`: Stores a 32-bit value from d0 into a global variable at an offset of 10146, then performs six bitwise OR operations on d0 with zero, and repeats this process for another global variable at an offset of 10150.
- `call_library_and_clear_d0`: Calls a routine via a pointer in a6 and then performs four bitwise OR operations on d0 with zero.
- `call_library_and_clear_d0_repeatedly`: Calls a routine via a pointer in a6 and then performs forty-one bitwise OR operations on d0 with zero.
- `compare_and_branch`: Compares a word in a global variable to 24 and branches if the result is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_sequence_and_clear_globals`: Calls another routine, sets a global variable to 46472, performs two assignments to other global variables, and then returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (cmpi.w) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_clear`: If the carry flag is set, it clears a 16-bit word at the address in a0 and increments a long value at a specific memory location, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_store`: Copies a 16-bit word from the address in a0 to d0, stores a 32-bit value from a1 to the address in a2 and increments a2 by 6, then stores the value in d4 to a long value at a specific memory location.
- `mask_and_fill`: Masks the lower 4 bits of d0 and stores the result in a long value at a specific memory location, then performs 50 consecutive bitwise OR operations on d0 with zero.
- `write_and_update`: Clears a 16-bit word at the address in a3, stores d5 and d0 at the address in a3, compares a3 to a constant, performs 57 bitwise OR operations on d0 with zero, then copies a word from a0 to a2, copies a word from a0 to a specific memory location, stores a0 in a1, and stores d0 in a specific memory location.
- `update_global_from_d1`: Stores the upper 16 bits of d1 at the address in a0, decrements a global variable, and if that variable is negative, enters a loop that performs bitwise OR operations on a value until it matches a constant, then stores a value at a specific memory location and returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_loop_and_store`: Sets a global variable to zero, enters a loop that decrements a value until it reaches negative one, performs bitwise OR operations on a value until it matches a constant, then stores a value at a specific memory location and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_and_store`: Calls a routine at a specific address and stores the constant 5166 at an offset of 126 from a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_and_set`: Copies the word in d0 to the address in a0 three times, clears a long value at a specific memory location, and stores the constant 24 at a specific memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_or_rts`: Small routine of 2 instructions that merges d2 into d0, then returns.
- `clear_and_call`: Clears a long value at a specific memory location, loads a specific address into a0, performs two bitwise OR operations on d0 with zero, calls a routine, increments a6 by 2, and stores the constant 20078 at a specific memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_rte`: Small routine of 1 instruction that returns from the exception.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[a][HD].adf:PRESTiGE#unpacked

- `allocate_and_process_system_buffers`: Allocates several memory blocks using absolute and general memory allocation, then enters a loop that waits for a specific bit pattern in a linked list node before calling several subroutines and checking a bit in the CIAA_PRA register.
- `update_pointer_and_calculate_offsets_2`: Loads a word from a pointer stored in a global variable, and if that word is non-negative, it updates the global variable with a new address from a table; otherwise, it loads a different word from a second table and updates the global variable with that address.
- `update_pointer_and_values_2`: Retrieves a word from a pointer in a1, and if it is negative, updates the pointer to a new address and retrieves a new word, then updates a global variable using bitwise operations, calculates a new value based on a constant and a bit-shifted version of the word, and stores the results in several global variables.
- `populate_lookup_tables_2`: Fills four different memory areas with values derived from a decreasing arithmetic sequence using a loop that runs 255 times.
- `scale_source_table_2`: Calls another routine, updates a global variable with a new address, and then runs a loop 64 times to scale values from a source table using multiplication and bit shifts, storing the results into a destination table.
- `swap_and_store_values_2`: Swaps the contents of two global variables and stores the resulting values and their high words into several other global locations, then clears two 16-bit words at a global address.
- `render_glyph_bitmaps_2`: Iterates through a string of characters to fetch corresponding glyph data from a font table and performs bitwise expansion to convert compressed font data into a larger bitmap format in a destination buffer.
- `synchronize_bitmask_layers`: Calculates indices for multiple bitmask data structures using a value from a global variable and then sets or clears specific bits at several fixed offsets within those structures based on state flags.
- `populate_structure_from_table`: Initializes several fields in a structure located at a6 with constant values and then iterates through a table in a0 to write triplets of words into specific offsets of that structure.
- `generate_coordinate_tables_3`: Populates multiple memory buffers with calculated values using nested loops, involving bitwise masking, bit-shifting, and arithmetic offsets.
- `populate_and_sum_grid_table`: Populates a memory area with a pattern of incrementing values using nested loops, then performs a second pass of nested loops to calculate and store sums of values from the previously generated table into a new memory area.
- `update_interpolated_values_and_shuffle`: Calculates new 16-bit values by adding constants to a base value and updating global variables via a loop that calls interpolate_16bit_fixed_point.
- `update_status_flags_3`: Calls another routine, then enters a loop to wait for bit 6 of a byte at the start of a structure in a6 to clear.
- `initialize_interpolation_tables_2`: Populates several global tables with values derived from a source table and constant values.
- `populate_parameter_table`: Writes several sets of configuration data to a structure and updates pointers to a table.
- `interpolate_table_values_2`: Calculates a sequence of 16-bit values using a loop and stores them into a table starting at offset 74 of a structure in a0, then copies two values from another memory location into a destination table starting at offset 58 of a structure in a2 by calling another routine.
- `process_coordinate_list_2`: Iterates through a list of 16-bit values, calculating the difference between the current value and a running accumulator.
- `initialize_audio_hardware_and_generate`: Writes values to a memory-mapped structure and waits for a bit in a status byte to clear, then generates a bit-shifted sequence of 80 words in a buffer and populates two tables with structured byte patterns and calculated values.
- `populate_offset_table_2`: Loads a base address from a global variable and calculates a series of incremental offsets using several constant values, storing each resulting address into a sequence of memory locations before storing a final modified address and a value from another global variable.
- `rotate_data_table_and_update_structure`: Rotates three 32-bit values within a table and updates a structure pointed to by a6 with new pointer data and modified 16-bit values.
- `interpolate_nibble_components`: Takes two 16-bit values and incrementally adjusts each of their three constituent components—the high byte, the middle nibble, and the low nibble—to move them one step closer to the values in the second register.
- `reset_and_update_status_flag`: Clears a 32-bit global variable, calls another routine, and then writes the words 32 and 49184 to offsets 154 and 152 of the structure pointed to by a6.
- `check_status_flag_2`: Checks if bit 5 is set in a 16-bit word at offset 28 of a structure, and if so, writes the value 32 to offset 154 of that structure and calls another routine before returning.
- `initialize_audio_hardware_3`: Opens a library, allocates a 1024-byte memory buffer, and detects the chipset type by checking Denise and custom chip registers.
- `loop_until_byte_match`: Decrements a counter in d0 until it reaches negative one, while repeatedly comparing a byte at offset 4 of a structure in a6 to itself; if the byte matches, it exits the loop and returns.
- `wait_for_bitmask_3`: Enters a loop that waits until a specific bit pattern is present in a memory location pointed to by an offset of two from a value on the stack, otherwise it returns.
- `reconfigure_hardware_and_manage_library`: Disables interrupts and DMA, copies a 256-word block of data from a table to a destination, and then performs a series of library management operations including switching to supervisor mode, updating hardware control registers, and closing a library before freeing the copied memory block.
- `branch_to_audio_processing`: Performs a short branch to a location within the routine that processes audio sample tables.
- `jump_to_target_routine`: Performs a short branch to another routine.
- `branch_to_audio_processing_2`: Performs a short branch to a location within another routine.
- `process_audio_sample_table`: Iterates through a table of audio parameters to update a global structure, performing mathematical transformations on sample data and calculating volume or pitch adjustments.
- `test_word_value`: Loads a pointer into a3 based on a relative offset and checks if the word at the negative offset of 4718 from that pointer is non-zero; otherwise it returns.
- `increment_and_dispatch_animation_frame`: Increments a 16-bit value at a pointer in a3 and compares it to a constant; if the value reaches the limit, it resets the value to zero and executes a jump table based on a masked value from a data structure.
- `process_animation_frame_data`: Iterates through a data block to update animation properties, including calculating frame differences and clamping a value at 64.
- `adjust_pointers_and_loop_block`: Adds two values to two different pointers, then executes a loop based on a counter in a register; if a comparison with a constant fails, it returns.
- `check_bitstream_block_status`: Tests a 16-bit word at offset 78 of a structure pointed to by a3; if the word is zero, it returns, otherwise it returns.
- `expand_compressed_bitstream_block`: Iterates through a loop four times to copy data from a source structure into a destination structure, then processes a byte at offset 3 of the source structure to either decrement a counter, branch, or perform bitwise operations and pointer increments to extract data from a bitstream.
- `increment_and_branch`: Increments a byte at offset 3 of the structure pointed to by a5 and then performs a short branch to another location.
- `pop_signed_byte_from_stream`: Checks a byte at offset 69 of a structure; if non-zero, it decrements that byte and copies the next byte from a pointer stored at offset 64 to offset 0 of the structure.
- `copy_three_bytes`: Copies two bytes from a source pointed to by a2 to offsets 1 and 2 of a structure in a5, stores the updated pointer in a5 at offset 40, and branches to another routine.
- `copy_conditional_bytes`: Loads a pointer from a5 at offset 36 into a2, copies a byte from that pointer to a5, and if the byte is not negative, copies two more bytes from the pointer to offsets 1 and 2 of the structure in a5 before updating the pointer at offset 36 and branching to another routine.
- `extract_and_shift_stream_data`: Moves a byte from a pointer in a2 to an offset of 45 in a structure pointed to by a5, loads a byte from a2 into d0, shifts d0 left by 8 bits, loads another byte from a2 into d0, stores the updated pointer from a2 into offset 36 of the structure in a5, subtracts d0 from a2, and branches to another routine.
- `decompress_bitstream_data`: Reads a byte from a stream and writes it to a structure, applying a bitwise NOT if the byte matches a specific marker.
- `decrement_and_branch_entity_counter`: Clears a 16-bit word in a register, tests a 16-bit value at offset 78 of that register, and if non-zero, decrements that value by one and branches to another routine; otherwise it returns.
- `update_entity_state_via_bitstream`: Checks a 16-bit word at offset 10 of a structure; if non-zero, it calls another routine, updates a function pointer in a global variable, writes the value 25 to a hardware register, and enters a loop that processes a bitstream table to update various fields in the structure based on a command byte.
- `process_entity_list_and_update_state`: Iterates through a list of structures, updating an index and copying data from a table into a destination structure.
- `decrement_and_check_value`: Decrements a 16-bit value at offset 4 of a structure in a3 and branches to another routine if the result is negative, otherwise it returns.
- `load_offset_table_entries`: Reads a signed byte from a table to determine an index, then uses that index to fetch four word values from a secondary table and stores them as absolute pointers into a structure at offsets -212, -150, -88, and -26 relative to a base pointer in a3.
- `copy_byte_and_branch_2`: Copies a byte from an offset of 2 in a structure pointed to by a5 into a destination at an offset of the same structure, then performs a short branch.
- `initialize_entity_structure_from_table`: Loads an index from a byte at offset 2 of a structure, sets a 16-bit counter to 64 at offset 4, and fetches four 16-bit values from a table using the index to populate offsets 212, 150, 88, and 26 of the destination structure before calling process_entity_list_and_update_state.
- `swap_nibbles_in_structure_field_2`: Reads a byte from offset 2 of a structure, then swaps its high and low nibbles with the nibbles of a byte at offset 22 of the same structure and writes the result back to offset 22.
- `dispatch_entity_update_subroutine`: Reads a byte from offset 2 of a structure in a5, extracts bits 3 through 6 to index into a jump table, and calls a subroutine via that jump table.
- `update_status_byte`: Reads a byte from offset 2 of a structure in a5, compares it to 1, and if not equal, stores that byte at offset 7 of a structure in a3, decrements the byte, stores the result at offset 9 of the structure in a3, clears a word at offset 16 of the structure in a3, and branches to another routine.
- `update_entity_state_and_index`: Updates a status byte and a counter in a structure pointed to by a3, then enters a loop that modifies values in a structure pointed to by a5 based on bitwise operations and table lookups, eventually updating a destination structure pointed to by a4 and potentially calling another routine.
- `configure_dma_transfer_2`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers, then clears an interrupt request and returns via an exception return.
- `load_audio_register_values_3`: Loads four sets of 32-bit and 16-bit values from a table into the four audio channel registers and sets a bit in the interrupt clear register before returning.
