# Game 434 (Sim City 2000): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00434). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2712384 top-level bytes in 22 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 187 of 187 named (0 author symbols), 187 of 187 described (3 by rule, 181 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2184 of 2186 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8093 of 8243 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2952 |
| code | 19190 |
| data | 2407330 |
| padding | 3360 |
| unused | 279552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 458 | 12 |
| Delete | 1 | 600 | 3 |
| Join | 1 | 363 | 2 |
| endcli | 2 | 182 | 3 |
| loadwb | 2 | 301 | 5 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer | 6 | 250 | 17 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:decrunchall | 3 | 2015 | 40 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE | 3 | 160 | 5 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1324 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 632 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE#unpacked | 1 | 1946 | 56 |

## Cases handled

- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:sc1: PCompress LH Data (xfd)-packed, unpacked with ancient (753664 -> 6683 bytes)
- sim city 2000 (1993)(electronic arts)[cr psg](disk 2 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 2 of 3)[HD].adf:sc2: high-entropy data file (7.79 bits/byte), classified as packed data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 3 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 3 of 3)[HD].adf:sc3: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address (play-rs-m1500): 1144 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 356 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer: Imploder (xfd) stream found by ancient scan (61704 -> 113740 bytes) but only 0 of 56 sampled windows are in reference RAM: not used
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE: crunched executable decrunched by XFDMaster (CrunchMania Normal (xfd), 18908 -> 50292 bytes), 12 of 13 windows confirmed in reference RAM: recovered as a hunk executable
- sim city 2000 (1993)(electronic arts)[cr psg](disk 2 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 2 of 3)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- sim city 2000 (1993)(electronic arts)[cr psg](disk 3 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 3 of 3)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- sim city 2000 (1993)(electronic arts)[cr psg](disk 1 of 3)[hd]/Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_audio_hardware` (runtime segment an address (play-rs-m1500)): Allocates memory for an audio buffer and configures hardware registers based on detected chipset capabilities. Kind: disk hardware/track/sync check. Pass path: u9__l0_0F0C.
- `copy_and_execute_subroutine` (runtime segment an address (play-rs-m1500)): Copies a block of memory from a source table to a destination, then executes a series of indirect function calls and modifies interrupt and DMA control registers. Kind: self-modifying code/data copy and execution. Pass path: The loop copies a block of data (puVar3 to puVar4) and then executes a subsequent branch/call. A successful check or valid copy allows the execution to proceed to the subsequent function calls (e.g., u9__sub_105A or the indirect calls at th.
- `initialize_hardware_and_copy_memory` (PRESTiGE#unpacked): Opens a library, allocates memory, and performs hardware checks on the custom chips and video registers. Kind: disk hardware/sync check. Pass path: the code continues to the DMACON/CIAB_PRB manipulation and subsequent subroutines after the DSKBYTR/CIAB_PRB checks.
- `copy_memory_and_manage_library` (PRESTiGE#unpacked): Copies a block of 256 words from a table to a destination, then updates hardware registers to enable DMA and interrupts. Kind: loader/protection routine. Pass path: The routine proceeds to execute library calls and memory management (exec_CloseLibrary, exec_FreeMem) after performing hardware register manipulations (ADKCON/DMACON) and a memory copy loop, suggesting it is a specialized loader that valida.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_register_and_branch`: copies a longword from one data register to another and then performs a short branch to another routine.
- `initialize_entity_structure`: Initializes a structure by setting several fields to zero, updating an entity type from a table using an index, and copying data from a source buffer into a destination array via a loop.
- `update_entity_type_and_increment_index`: Fetches a byte from a table using an index stored at offset 0 of a structure, stores it at offset 20, and increments the index at offset 24 by 1.
- `update_structure_and_increment_link_2`: Calculates a new value for a field at offset 16 of a structure in a1 by adding 11 to the value at offset 12, then updates a field at offset 80 by subtracting 1 from the value at offset 20.
- `update_structure_and_increment_link`: Updates several fields in a structure pointed to by a1 using values from a table in a0, including setting a byte to 10 and clearing a longword, then calls another routine and performs a loop that increments a value in a structure field.
- `branch_with_offset`: Adds 1 to a value from a table at an offset of 84 in a1 and branches to another routine.
- `test_structure_bitfield_and_branch`: Tests if a longword at offset 40 of the structure in a1 is non-zero, then branches to another routine or performs an unconditional branch to a third routine.
- `modify_structure_bitfield`: Checks a longword at offset 48 of the structure in a1; if the value is non-zero, it sets the upper 8 bits of that longword and branches to another routine, otherwise it branches to a different location.
- `update_entity_table_index`: Copies a longword from offset 64 of a structure to offset 108 of the same structure, then calls another routine with parameters derived from offsets 48, 52, and 132 of various structures, finally storing the returned value into offset 28 of the structure.
- `initialize_entity_state_and_clear_table`: Calculates an index in a table using offsets from a structure in a1, stores a value from a1 into that table, and calls two other routines.
- `update_entity_state_and_decrement_counter`: Updates several fields in a structure using values returned from other routines, clears a value in a table at an offset calculated from a field in the structure, and decrements a counter in a table if it is not negative.

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
- `process_data_and_call_subroutines`: Performs bitwise operations on a value in a2, calls several subroutines via a pointer in a5 with various arguments, clears a value in a table at an offset determined by a value in a0, decrements a value in a table, and performs an indirect jump.

### loadwb

- `run_loadwb_with_switches`: Clears a local 16-byte buffer and parses command-line switches from an argument template.
- `initialize_library_data`: Checks a structure for specific bit patterns and then copies a block of instructions to a newly allocated memory area.
- `check_and_set_pointer`: Compares d0 to 8 and 27, clears bit 1 of a byte at an offset of 44 in a6 if the first comparison matches, and if the second comparison matches, loads a pointer into d0 and returns.
- `get_pointer_from_library`: Calls a library function to retrieve a pointer and then conditionally returns a pointer to a global variable or a pointer retrieved from another library call.
- `close_library_and_clear_pointers`: If a value at offset 50 in a6 is 1 and a global pointer is non-zero, it calls a library function to close the library and then clears three consecutive global pointers by writing zero to them.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:installer

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `execute_installer_sequence`: Calls a function pointer in a4 multiple times, manipulates pointers in a1, a0, a6, and a2, calls another routine, and then enters a loop that increments d5 and calls a function pointer in a6 if a comparison with a value at an offset of 20 in a structure fails.
- `process_data_loop`: Doubles a value in d0, branches if negative, calls another routine, and then enters a loop that calls a function pointer in a4, adds a value to a table at an offset determined by d0 and d2, and decrements d1 until it reaches zero.
- `conditionally_call_buffer_copy_routine`: If a zero flag is set, it branches to another location; otherwise, it moves a pointer from a3 to a1, calls copy_four_bytes_from_buffer, and then branches to another location.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_four_bytes_from_buffer`: Compares a pointer in a6 with a pointer in a2 and, if they are not equal, copies four bytes from the source buffer in a2 to the destination buffer in a1 while decrementing a counter in d0; otherwise, it loads a longword from the destination buffer into d0 and returns.
- `expand_variable_length_bitstream`: Reads a control byte from a buffer to determine a bit count, then enters a loop that shifts a value left and conditionally loads new bytes from the buffer to handle carries.
- `process_bitstream_loop`: Sets d7 to 2, adds the byte in d1 to itself, and if the result is not zero, loads a byte from the buffer in a0 into d1 and adds it with carry to d1 and d0, then decrements d7 and loops if d7 is not zero; otherwise it adds 6 to d0, sets d7 to 3, and branches to another routine.
- `initialize_loop_parameters`: Sets d0 to 5, sets d7 to 3, and branches to another routine.
- `initialize_loop_parameters_2`: Sets d0 to 4, sets d7 to 2, and branches to another routine.
- `initialize_loop_parameters_3`: Sets d0 to 3, sets d7 to 1, and branches to another routine.
- `expand_bitstream_variable_length_integer`: Processes a bitstream by shifting the value in d1 left by one bit and conditionally loading a new byte from the buffer in a0 to handle carries.
- `update_bitstream_accumulator`: Adds a byte to a value in register d1 and branches if the result is non-zero; otherwise, it loads a byte from a buffer in a0 into d1, adds it to d1 with the carry flag, adds d2 to d1 with the carry flag, and branches to another routine.
- `decompress_bitstream_data_2`: Reads a bitstream from a source buffer to reconstruct values, using a control byte to determine bit lengths and a table to resolve offsets.
- `decompress_bitstream_data`: Reads a control byte from a source buffer to determine the number of bits to process, then performs bitwise shifts and additions to reconstruct a value from a bitstream.
- `push_registers_and_branch`: Pushes bytes from d0, d1, and d2 onto a stack pointed to by a4, restores several registers from the stack, and branches to another routine.

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
- `process_stream_opcodes`: Iterates through a stream of opcodes to update various counters and structure members, potentially triggering a DMA transfer by calling configure_dma_transfer and writing to a hardware register.
- `decode_string_stream_opcodes`: Decodes a custom-encoded instruction stream by interpreting bit-masked opcodes to perform pointer arithmetic, table lookups, and conditional updates to a destination buffer.
- `process_string_stream`: Iterates through a buffer to decode or decompress a stream of bytes, performing bitwise negation and pointer increments based on certain byte values.
- `move_and_branch_2`: Moves a byte from an offset of a pointer to another offset of the same pointer and then performs a relative branch.
- `decode_string_stream_buffer`: Decodes or decompresses a byte stream by updating pointers and copying data from a source buffer.
- `decode_string_stream_buffer_2`: Decodes a compressed or encoded byte stream by performing bitwise nibble swapping and pointer-based data extraction.
- `dispatch_jump_table`: Calculates an index from a byte at offset 2 of a structure in a5, uses it to jump to a target address from a table, and then executes one of several branches that modify values in a5 or a3 based on the initial index.
- `update_string_stream_control_bytes`: Reads a control byte from a buffer and updates a destination or state table with a modified or decremented value.
- `configure_dma_transfer`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers.
- `load_audio_register_values`: Loads several 32-bit or 16-bit values from a series of memory locations into a block of RAM and sets a global variable to a fixed value, while modifying the low 16-bits of the first audio channel and the interrupt clear register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### Sim City 2000 (1993)(Electronic Arts)[cr PSG](Disk 1 of 3)[HD].adf:PRESTiGE#unpacked

- `allocate_memory_and_initialize_subsystems`: Allocates several blocks of memory using absolute and general allocation methods, then performs a series of initialization calls to various subroutines.
- `update_pointer_table`: Loads a pointer from a table, reads a 16-bit word from the location it points to, and if that word is positive, replaces the pointer in the table with a different address; otherwise, it loads a second 16-bit word from the original location into a register and branches to another routine.
- `update_pointer_and_indices`: Updates a pointer in a table, modifies a bitmask in a global variable using the low nibble of a value, and calculates and stores two 16-bit values into global variables based on a shifted value.
- `fill_symmetric_tables`: Fills four different memory areas starting from a base address in a loop 255 times, writing descending and ascending 16-bit values to create a symmetric pattern of signed integers.
- `fill_symmetric_tables_2`: Calls another routine, updates a global pointer, and then enters a loop that runs 64 times to calculate and store 16-bit values into a table by multiplying a value from a source table with a constant and adding an offset.
- `swap_and_reset_values`: Swaps two longwords at a specific memory location, calculates two new 16-bit values based on the swapped data to store in global variables, and clears two 16-bit words at fixed global locations.
- `update_tile_graphics_buffer`: Iterates through a table of tile indices to calculate memory offsets and performs bitwise operations to update a series of 8-byte graphical data structures.
- `update_bitmask_grid_data`: Reads a value from a table to determine bitwise operations on a grid structure, then sets or clears specific bits at various offsets within multiple memory blocks based on flags in the table value.
- `populate_data_structure`: Checks a bit in a structure at a6, then either initializes several fields in the structure or loops 98 times to copy three words from a table into specific offsets of the structure.
- `generate_arithmetic_lookup_tables`: Performs multiple passes over data to populate lookup tables, involving arithmetic operations such as adding values from two source tables, performing bitwise manipulations, and applying scaling factors using multiplication and bit shifts.
- `generate_arithmetic_tables`: Fills a table with alternating increments of 32, then performs a second pass to populate another table by adding values from two source tables to a base value.
- `update_color_palette_and_shuffle`: Calculates new color values by adding constants to a base value, updates a global color component, and calls clamp_color_components in a loop three times.
- `update_structure_fields`: Calls another routine, then checks bit 6 of a byte at the start of a structure in a6.
- `initialize_system_parameters`: Performs several loops to swap bytes within specific memory locations, copies data between tables, calls two other routines, and if a value at an offset of a structure in a6 meets a specific bitmask condition, it writes several constants to global variables and clears four 16-bit values before storing a pointer in a global variable.
- `initialize_command_list`: Sets several fields in a structure and then sequentially populates it with four different longword values and associated metadata from a series of tables.
- `process_color_data`: Uses a loop to write pairs of words to a table starting at offset 74 of a structure in a0, alternating between values of 10 and -10 with an increment of 20.
- `calculate_offset_parameters`: Iterates through a table to calculate a value based on the difference between two counters, then stores a transformed version of this value and several pointers into a structure.
- `generate_bit_pattern_table`: Calls another routine, then populates two parallel tables by performing a bitwise rotation on a starting value and copying data from a source table.
- `calculate_memory_offsets`: Calculates and stores a series of longword offsets into a table by repeatedly adding a constant value to a starting address.
- `update_linked_list_pointers`: Swaps two longwords in a table, updates a structure with a new value, and then updates two specific word fields in a target structure using bit manipulation.
- `clamp_color_components`: Compares the high, middle, and low nibbles of two 16-bit words and increments or decrements the nibbles of the first word to bring them closer to the second word.
- `reset_and_configure_structure`: Clears a 32-bit word at a global address, calls another routine, stores the address of the current routine into a global variable, and writes 32 to offset 154 and 49200 to offset 152 of the structure in a6 before returning.
- `test_and_update_status`: Checks if bit 5 is set in a 16-bit word at offset 28 of the structure in a6; if set, it writes 32 to offset 154 of that structure, decrements a6 by 2, and calls another routine before returning via an exception return.
- `initialize_hardware_and_copy_memory`: Opens a library, allocates memory, and performs hardware checks on the custom chips and video registers.
- `loop_until_byte_match`: Decrements a counter in d0 and compares a byte at an offset in a6 to itself, looping until the counter reaches zero.
- `check_bitmask_match`: Returns immediately if the bits 8 through 15 of a longword at offset 2 of a6 do not match the value 496.
- `copy_memory_and_manage_library`: Copies a block of 256 words from a table to a destination, then updates hardware registers to enable DMA and interrupts.
- `branch_to_audio_logic`: Performs a short branch to another location.
- `branch_to_target_routine_2`: Performs a short branch to a specific instruction address.
- `branch_to_audio_logic_2`: Performs a short branch to another location.
- `process_audio_sample_table`: Iterates through a table of audio parameters to update a global structure, performing mathematical transformations on sample data and calculating volume or pitch adjustments.
- `check_global_word`: Loads a pointer into a3, tests a word at a negative offset from that pointer, and returns if the word is not zero; otherwise it branches to another location.
- `increment_structure_counter_and_dispatch`: Increments a 16-bit word in a structure pointed to by a3, and if the new value reaches a specific threshold, it uses a jump table to call a subroutine based on a value extracted from a table.
- `update_sprite_attributes_and_loop`: Iterates through a list of structures, performing arithmetic on values at specific offsets and updating a 16-bit value in a structure based on the result.
- `adjust_pointers_and_test_condition`: Adds the contents of two registers to two other registers, decrements a counter register to loop, and then compares a 16-bit value at a constant offset to a register, either branching to another routine or returning.
- `check_metadata_status_and_return`: Tests a 16-bit word at offset 78 of a structure pointed to by a3; if the word is zero, it branches to the next instruction, otherwise it returns.
- `copy_and_update_stream_metadata`: Copies a block of metadata from a source structure to a destination structure and enters a loop that iterates four times.
- `increment_stream_offset_and_branch`: Increments a byte in a global variable at offset 3 from a pointer in a5 and branches to another routine.
- `extract_and_decode_stream_byte`: Checks a byte at offset 45 of a structure; if non-zero, it decrements that byte, loads a pointer from offset 40, and copies a byte from that pointer into the structure at offset 0.
- `store_three_bytes`: Stores two bytes from a source pointed to by a2 into offsets 1 and 2 of a structure in a5, stores the third byte from the source into offset 1 of the structure, then updates offset 40 of the structure with the new pointer value and branches to another routine.
- `copy_conditional_bytes`: Loads a pointer from offset 36 of a structure into a2, stores a byte from that pointer into a structure in a5, and if the result is not negative, stores two more bytes from the pointer into offsets 1 and 2 of the structure; it then updates offset 36 of the structure with the pointer and branches to another routine.
- `update_pointer_and_offset`: Stores a byte from a2 into offset 45 of the structure in a5, loads a byte from a2 into d0, shifts d0 left by 8 bits, loads another byte from a2 into d0, stores a2 into offset 36 of the structure in a5, subtracts d0 from a2, and branches to another routine.
- `decode_and_populate_stream_structure`: Decodes a stream of bytes into a structure by checking for a specific marker, performing bitwise negation, and copying three bytes from a source pointer.
- `decrement_structure_counter_and_branch`: Clears a 16-bit word in a register, checks a 16-bit word at offset 78 of that register, and if non-zero, decrements that word by one and branches to another routine; otherwise it branches to a different routine.
- `update_structure_and_dispatch_jump_table`: Checks a 16-bit word at offset 10 of a structure; if non-zero, it calls another routine, updates a function pointer in a global variable, sets a hardware register to 25, and enters a loop that processes a table of data by performing various bitwise operations, memory copies, and arithmetic updates to the structure at offset 10, before finally performing a jump through a calculated address from a jump table.
- `process_and_copy_data_structures`: Iterates through a list of structures, updating a 16-bit value at offset 8 and copying data from a source table into a destination structure.
- `decrement_and_check`: Decrements a 16-bit value at offset 4 of a structure in a3 and branches to another routine if the result is negative, otherwise it returns.
- `load_offset_table_entries`: Reads a signed byte from a table to determine an index, then uses that index to fetch four word values from a secondary table and stores them as absolute pointers into a structure at offsets -212, -150, -88, and -26 relative to a base pointer in a3.
- `copy_byte_and_branch`: Copies a byte from an offset of 2 in a structure pointed to by a5 to an offset of the same structure and then branches to another routine.
- `fetch_and_populate_structure_data`: Loads a byte from a pointer in a5 to determine a table index, then fetches four 16-bit words from a table and stores them at offsets -212, -150, -88, and -26 of the structure in a3.
- `swap_nibbles_in_structure_byte`: Reads a byte from offset 2 and a byte from offset 22 of a structure in a5; if the byte at offset 2 is non-zero, it swaps its high and low nibbles and merges them with the nibbles of the byte at offset 22, then writes the resulting byte back to offset 22 before returning to another routine.
- `dispatch_jump_table_via_bitmask`: Extracts a nibble from a byte at offset 2 of a structure in a5, uses it as an index into a jump table to call another routine, and otherwise returns immediately.
- `update_byte_values`: Compares a byte from an offset of 2 in a structure to 1, and if not equal, stores that byte at offset 7 of another structure, stores the byte decremented by 1 at offset 9 of that same structure, clears a word at an offset in a structure, and branches to another location.
- `update_structure_and_lookup_table`: Updates a 16-bit value in a structure and performs a bitwise calculation to index into a table, then modifies a byte at offset 23 of the structure and calls another routine.
- `configure_hardware_registers`: Tests a byte at a specific address, then writes a longword from a table to a memory location, updates the DMA controller, sets the interrupt request to 8192, writes 25 to a hardware register, and returns from an interrupt.
- `update_audio_channels`: Tests a byte at a specific address, then copies four sets of longword and word values from a table into the audio channel and length registers, sets the interrupt request to 8192, and returns from an interrupt.
