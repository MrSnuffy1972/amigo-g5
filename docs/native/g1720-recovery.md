# Game 1720 (venus the flytrap (1990)(gremlin)[cr untouchables][h hangman]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01720). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1832352 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 271 of 271 named (0 author symbols), 271 of 271 described (42 by rule, 219 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2322 of 2322 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5674 of 5674 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 117973 |
| code | 18410 |
| data | 1284463 |
| padding | 3954 |
| unused | 407552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:Type | 2 | 117 | 2 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:Venus | 2 | 83 | 3 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:cls | 3 | 19 | 1 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:mouse | 2 | 3 | 1 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:stripes | 3 | 228 | 7 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:unt | 2 | 207 | 8 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:border | 3 | 228 | 7 |
| Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 739 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 493 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1765 | 107 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1765 | 86 |

## Cases handled

- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 2 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1746 non-empty blocks: matched against track-loaded segments
- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:00: high-entropy data file (7.87 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:01: high-entropy data file (7.64 bits/byte), classified as packed data
- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:02: high-entropy data file (7.62 bits/byte), classified as packed data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 147 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 389 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 427 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 643 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 2 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 2 of 2).adf: blocks 534-844 (bytes 273408-432640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.24)
- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:Venus: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- venus the flytrap (1990)(gremlin)[cr untouchables][h hangman](disk 1 of 2)/Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:unt: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 packed track region(s), 734720 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `convert_and_compare_hex_nibbles` (runtime segment an address): Converts a byte into two ASCII characters representing its high and low nibbles and stores them in global variables. Kind: manual/code-wheel/color-card lookup. Pass path: the branch where sVar5 == -1 (the loop completes successfully, leading to u9__sub_1D80 and return).

## Functions


### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:Venus

- `unpack_compressed_data`: Decompresses a bitstream into a destination memory buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:cls

- `initialize_library_resources`: Accesses a global variable and calls several library functions using offsets from a library base pointer.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:mouse

- `wait_for_disk_status`: Polls a bit in a global variable until it is cleared.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:stripes

- `initialize_system_memory`: The routine clears a range of hardware registers and copies a block of data from one memory location to another.
- `reconstruct_amiga_hunks`: Parses a custom data structure by identifying specific AmigaDOS hunk type codes to perform memory relocation and data patching.
- `return_immediately`: The routine performs no operations and returns to its caller immediately.
- `process_bitstream_data`: Performs complex bit manipulation and data movement based on the contents of a bitstream.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that uses a 32-bit value from four bytes before the source pointer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitstream_store_pointer`: Repeats a shift of the bit buffer right by one place for a count in D1, storing the current pointer's low word into a global on each pass.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:unt

- `reconstruct_amiga_hunk_table`: Parses a custom data structure to reconstruct an AmigaDOS-style hunk table by processing relocation and memory allocation codes.
- `stub_empty_return_routine`: This routine is empty and contains only a return instruction.
- `decrement_register_to_zero`: Decrements a register in a loop until it reaches zero.
- `tiny_ori`: Small routine of 1 instruction that merges 228 into d0.
- `unpack_compressed_stream_backwards`: Unpacks a compressed byte stream from run-length codes, copying literal runs and back-references from the end of the buffer toward its start.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `check_bounds_and_trap`: Performs a comparison and potentially triggers a software trap.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:border

- `initialize_hardware_and_copy_data`: Clears a block of memory at a hardware register address and then copies a block of data from that register to a specific memory location.
- `reconstruct_amiga_hunks_2`: Parses a custom data stream by identifying specific AmigaDOS hunk type codes to allocate memory and relocate data.
- `return_from_subroutine_5`: The routine is empty and performs no operations before returning to its caller.
- `decode_bitstream_sequence`: Manipulates a bitstream to extract values and write data to a destination pointer.
- `unpack_bitstream_with_pointer_shift`: Decompresses a packed bitstream by reading bits through a shifting routine that fetches a 32-bit value from the four bytes preceding the source pointer.
- `shift_pointer_value_2`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bitstream_store_pointer_2`: Repeats a shift of the bit buffer right by one place for a count in a register, storing the current pointer's low word into a global on each pass.

### Venus the Flytrap (1990)(Gremlin)[cr Untouchables][h Hangman](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_blocks`: Clears a block of memory at a hardware address and then copies a block of data from another location to a new destination.
- `copy_and_initialize_memory`: Clears a block of memory at a specific hardware address and then copies a block of data from that memory to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_global_state`: Checks a bit in a global variable and sets it to one if active, then overwrites the same variable with a constant value.
- `store_constant_value`: Stores a constant value into a global variable.
- `initialize_system_subsystems`: Configures hardware registers, initializes memory blocks, and performs data decompression.
- `decrement_packed_nibble_counters`: Walks a block of 32 words, unpacking each into high-byte and two nibble fields, decrementing every non-zero field by one and repacking the word, and repeats the sweep until all fields are zero.
- `skip_execution_sequence`: This routine performs no operations and allows the processor to proceed directly to the subsequent code block.
- `decrement_table_values`: Iterates through a table of words, decrementing the high and low nibbles of each word until a condition is met.
- `copy_table_data`: Checks a bit in a global variable and, if set, copies data from a table into multiple destination buffers using a loop.
- `update_global_status`: Checks a bit in a global variable and updates that variable and another global with specific constant values.
- `execute_infinite_loop`: Executes an empty loop that never terminates, so control never returns to the caller.
- `update_state_and_callback`: Sets a bit in a global variable, overwrites it with a constant, and stores both a function pointer and a constant value into global memory locations.
- `update_hardware_and_status`: Sets a bit in a global variable, updates that variable and another with specific constant values, and stores a function pointer into a global variable.
- `decompress_bitstream_data`: Searches for a specific magic signature in memory before performing a bit-shifting decompression algorithm.
- `shift_bitstream_process_data`: Shifts a value left by one bit repeatedly, reloading the value from a pointer and adjusting the pointer when the value becomes zero.
- `toggle_interrupt_enable`: Modifies the interrupt enable bit in the INTENA register and updates the CIAA_PRA register.
- `save_register_context`: Stores a series of registers into a table in memory and performs conditional logic based on bits in a global variable.
- `convert_and_compare_hex_nibbles`: Converts a byte into two ASCII characters representing its high and low nibbles and stores them in global variables.
- `store_constant_to_memory`: Stores two specific values into a memory location relative to a pointer.
- `assign_routine_result`: Calls another routine and stores its return value into a global variable.
- `interleave_bit_data`: This routine performs bitwise interleaving of two data sources by masking bits and shifting them to create a single stream.
- `decrement_and_loop_2`: decrements a value from two hundred fifty-five down to zero in a loop.
- `set_bit_and_loop`: Sets a specific bit in a byte through a pointer and repeats the operation in a loop while a bit in another byte remains set.
- `modify_byte_and_loop`: Clears the lowest bit of a byte through a pointer, sets the lowest bit, and enters a loop that continues while a specific bit in another byte is set.
- `decrement_and_loop`: Decrements a value in a loop while performing bitwise operations on a byte and adding a value to a memory location.
- `write_and_wait_for_bit`: Stores a word into two separate memory locations and then waits in a loop until a specific bit in a third memory location is cleared.
- `manipulate_bits_and_loop`: Performs a series of bitwise operations to modify a byte through a pointer and then enters a loop that continues while a specific bit in another byte is set.
- `write_and_wait_status`: Performs bitwise operations on a byte and writes values to three memory locations before waiting for a specific bit to clear in a memory location.
- `return_from_subroutine_3`: The routine performs a return from a subroutine.
- `return_from_subroutine_4`: The routine is empty and performs no operations before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_initialization_sequence`: Executes a long sequence of initialization tasks including memory allocation, pattern filling, and table setup.
- `wait_disk_status`: Polls a bit in the CIAA Port A data register until it is cleared.
- `allocate_memory_blocks`: Allocates two memory blocks using a system library and stores the resulting pointers in global variables.
- `fill_memory_pattern`: Writes a repeating pattern of four words into a memory area using a loop.
- `initialize_data_structure`: Populates a large block of memory with a series of specific constant values.
- `copy_memory_block_2`: Copies sixteen words from one location in memory to another.
- `setup_data_table`: Writes several constant values and a global variable into a structure pointed to by a register.
- `update_graphics_state`: Uses graphics library functions to retrieve a value and update a structure with it.
- `initialize_subsystems`: Sets a global variable to a specific value and prepares hardware subsystems by clearing registers and copying memory.
- `initialize_hardware_subsystems`: Clears a range of hardware registers and copies a block of memory to initialize system subsystems.
- `configure_control_block`: Waits for a bit in a memory byte to clear, then initializes a structure with several constant values and a pointer to a global variable.
- `update_sprite_attributes`: Iterates through a data table to update a series of sprite or object attribute structures.
- `calculate_audio_table_offsets`: Iterates through a data table to calculate and store a series of values into a global array.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a sixth global variable.
- `update_audio_channel_parameters`: Modifies audio hardware registers by adjusting volume or pitch values based on specific mode flags.
- `update_audio_parameters`: Modifies audio length or pitch values in a structure based on a mode flag and updates hardware audio registers and DMA settings.
- `unknown_routine`: This routine is too short or complex for the decompiler to provide a description, but it interacts with the CIAA and DMA controllers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_library_functions`: Clears a block of memory, updates several global values, and conditionally executes library functions based on system state.
- `initialise_border_state`: Clears eight longwords of state and stores a value returned by a call.
- `check_and_call_library_hooks`: Checks several global pointers and values to conditionally execute a series of library function calls and an optional hook routine.
- `call_present_resource_routines`: checks several global pointers and status bytes to conditionally execute a series of library function calls and indirect routines.
- `setup_console_device_request`: Calls library vector functions and routines in sequence, referencing the console device text.
- `initialize_border_structure`: Sets pointers and flag bits within a structure using a library vector call and a text reference to Border.
- `copy_border_data`: Copies longwords from one memory location to another based on offsets from provided pointers.
- `call_border_routines_with_coordinates`: Computes four offsets from stored structure fields and calls routines with those coordinates.
- `delay_after_coordinate_calls`: Calls routines with coordinates computed from stored structure fields, then makes a delay call through the DOS library and calls a library vector function.
- `store_parameter_and_call_routine`: Stores its first parameter in a stored field, then calls a routine with the second parameter and a stack address.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `initialize_resource_structure`: Initializes a data structure by calling several library functions, performing arithmetic on memory offsets, and writing specific values to a table.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_array_2`: Parses a null-terminated string into individual tokens by skipping whitespace and handling quoted substrings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `copy_string_with_limit_3`: Copies bytes from a source pointer to a destination pointer, stopping when a null terminator is reached or a maximum length is exceeded.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `load_window_resource`: Opens a file and retrieves a resource from a library to populate a data structure with specific offsets and values.
- `store_parameter_to_global`: Stores a passed parameter into a global variable and then clears the first byte of that same location.
- `call_and_terminate_buffer`: stores its first parameter as a pointer, calls a routine with the second parameter and a stack address, then writes a zero byte at the stored pointer and returns the call's result.
- `increment_pointer_and_store_byte`: Increments a global pointer and stores a byte from the first parameter at the address previously held by that pointer.
- `execute_callback_sequence`: Calls two subroutines in succession using a provided pointer and a stack reference.
- `concatenate_return_values`: A 48-instruction routine that calls another routine and returns a 64-bit value composed of the original contents of a data register and the value left in that register by the called routine.
- `parse_format_string`: Parses a string containing format specifiers to extract and convert values from a provided data list.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `call_two_subroutines`: Executes two different subroutines in succession and then returns.
- `call_subroutines`: This routine calls two different subroutines in succession.
- `nop_placeholder`: This routine is empty and performs no operations.
- `increment_and_store_byte_2`: A fifteen-instruction routine that is called by routines that increment a pointer and store a byte from a table.
- `increment_and_store_byte`: Increments a pointer stored in a global variable and stores a byte from a table at the original pointer location.
- `apply_routine_to_record_table`: Steps through a fixed table of 22-byte records from its first entry up to its end, calling another routine once for each record's address.
- `clear_structure_recursively`: Recursively processes a structure based on bit flags and clears its contents to zero.
- `clear_structure_and_recurse`: Recursively traverses a structure to perform operations based on bit flags and then clears the structure's fields to zero.
- `write_byte_to_buffer`: Writes a byte to a buffer location and updates the buffer's internal state and pointers.
- `manage_circular_buffer_state`: Checks buffer status flags and pointer values to determine if a buffer reset, a pointer update, or a data write is required.
- `process_circular_buffer`: Manages a data buffer by checking status flags and updating head/tail pointers.
- `update_buffer_status`: Manages a buffer by updating pointers, indices, and status flags based on conditional checks and data writes.
- `update_buffer_state`: Updates a structure by modifying a pointer, an index, and a status flag, then stores a byte from a table into the pointed-to memory.
- `search_table_for_null_2`: Iterates through a table of 22-byte entries to find a null byte at a specific offset, then clears the entry and returns its pointer.
- `search_table_for_null`: Iterates through a table of 22-byte entries until a null byte is found in a specific offset, then clears the entry and returns it.
- `clear_data_structure`: Sets three consecutive longwords at a specific memory location to zero.
- `update_parameter_status`: Updates a structure's fields with specific values and pointers based on the result of another routine call.
- `update_status_flags`: sets specific bits in a byte at an offset from a pointer and conditionally sets another bit based on the result of a routine call.
- `free_linked_list`: Iterates through a singly linked list and releases each node until an empty link is encountered, then clears a global pointer.
- `free_list_nodes_from_pointer`: Releases each node of a singly linked list starting at the node in the address register and stopping after the node whose link is empty, using each node's stored size plus eight, then clears a global head pointer.
- `push_list_node_2`: Allocates a new node via a library call and inserts it into a linked list by updating pointers.
- `push_list_node`: Creates a new list entry by copying data from a global pointer and another source into a new node, then updates the global pointer to point to this node.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: traverses a linked list to find a matching node and removes it by updating the preceding node's pointer.
- `search_linked_list`: The routine traverses a linked list to find a matching node and then copies a value to a global variable or another location.
- `set_global_pointer_2`: Copies a value from one location to another or to a global variable and then calls another routine.
- `set_global_pointer`: Copies a value from a register into a global variable and then calls another routine.
- `check_table_interactivity`: The routine validates an index against a table and calls another routine to determine if a specific entry is interactive.
- `check_is_interactive`: Calls a routine with a value retrieved from a table and returns a boolean result.
- `return_zero`: An empty routine that returns zero.
- `handle_delete_file_error`: Calls a routine and, if it returns zero, calls another routine to store a result in a global variable before returning a specific value.
- `return_zero_2`: The routine performs no operations and returns zero to the caller.
- `perform_conditional_write`: A routine consisting of nineteen instructions that is called by several routines including those that perform error-checked writes.
- `perform_write_with_error_check_2`: The routine checks a bitmask, attempts a write operation, and stores a result in a global variable if the operation fails.
- `perform_write_with_error_check`: Calls a routine with three arguments and, if it returns negative one, calls another routine to store a result in a global variable.
- `return_void`: An empty routine that returns.
- `call_library_with_check`: Calls a library function and performs a conditional check on a global value before potentially calling another routine.
- `execute_fixed_hook`: calls a routine and returns zero.
- `write_text_and_execute_hook`: Writes a four-byte text fragment from a fixed memory location to an output handle and then executes an optional callback routine.
- `run_optional_hook_then_finish`: Calls a global hook pointer when it is non-zero, then calls another routine and returns.
- `release_all_resource_tables`: Releases the program's resource tables by iterating through a table to free entries, running an optional global hook, and freeing various saved buffers or single buffers, potentially making a supervisor-mode library call.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `traverse_and_clear_linked_list`: Validates an index against a table's bounds and checks if the target entry is non-zero.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function`: Calls a library function using an offset derived from a pointer stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_function`: Loads a function pointer from a memory location calculated using an offset from a global variable and executes it.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Retrieves a function pointer from a nested memory structure and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a table located via an offset from a global variable and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `store_byte_from_offset`: Stores a single byte from an offset relative to a pointer into a stack location.
- `copy_byte_from_offset`: Attempts to store a single byte from an offset relative to a pointer into a stack location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_check_status`: Clears a sequence of four words in a structure and executes a function through a pointer if a specific bit is set in a global variable.
- `parse_and_render_text_attributes`: Parses a sequence of character data to extract formatting parameters such as font size, spacing, and color.
- `fill_memory_with_byte`: Fills a specified range of memory with a single byte value using a loop.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `initialize_window_structure`: Allocates memory, populates a structure with specific values, and performs several indirect function calls to manage data and state.
- `tokenize_string_into_array`: Parses a string by identifying whitespace and quoted substrings, then copies the resulting tokens into a new array of pointers.
- `tokenize_string_into_array_3`: Parses a null-terminated string into individual tokens by splitting on whitespace or handling quoted substrings.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `copy_string_with_null_padding`: Copies bytes from a source pointer to a destination pointer for a specified count, terminating early if a null byte is encountered.
- `append_string_to_destination`: Finds the end of a destination string and copies a source string to that location up to a maximum length.
- `insert_string_at_end`: Finds the end of a destination string and writes a source string into that position for a limited count, ensuring a terminator is present.
- `copy_string_to_buffer_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `process_string_data`: Iterates through a pointer to a sequence of bytes, calling another routine for each byte until a null terminator is reached or a failure occurs.
- `load_window_data`: Opens a file and uses its contents to populate several fields within a structure, including a value retrieved from a table via a signed division.
- `signed_divide_longwords`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `perform_signed_division_2`: Performs a signed division by calling an unsigned division routine and adjusting the sign of the quotient.
- `call_and_return_concatenated_2`: Calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call_2`: The routine calls another routine.
- `divide_32bit_integers`: Performs division on two 32-bit integers provided in registers, using a bit-shifting loop for large divisors or standard arithmetic for smaller ones.
- `compute_string_length_2`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `subtract_and_call_routine`: Subtracts a constant value from an uninitialized register and passes the result as an argument to another routine.
- `check_condition_and_clear`: Calls another routine and, if a specific value and a bit flag are met, calls another routine with a sentinel value.
- `insert_into_buffer`: Checks if a current index is less than a limit, then stores a byte into a buffer and increments the index, or calls another routine if the limit is reached.
- `traverse_node_tree`: Recursively traverses a linked data structure based on bitwise flags within a node, performing operations on child nodes before zeroing out.
- `traverse_and_clear_node_tree`: Recursively traverses a linked data structure based on bitwise flags within a node, performing operations on child nodes before zeroing out the current node's contents.
- `process_node_flags`: Checks flags within a structure to conditionally call other routines and then clears the structure's contents.
- `modify_linked_list_node`: Recursively traverses a linked data structure to locate a specific node and update its value or modify the list's pointers.
- `search_table_entry`: Iterates through a table of entries to find one where a specific byte is zero, then clears three consecutive longwords at that location.
- `search_and_clear_table_entry`: Iterates through a table to find an entry where a specific byte is zero, then clears three consecutive longwords at that location.
- `clear_three_longwords`: Sets three consecutive longwords starting at a pointer to zero.
- `update_structure_state`: Validates an index against a table and updates a structure with status flags and pointers based on the result.
- `traverse_linked_list`: Iterates through a linked list of pointers, executing a routine at each node and clearing a global pointer when finished.
- `traverse_and_execute_linked_list`: Iterates through a linked list of pointers, executing a function at each node via an indirect call.
- `insert_list_node`: Assigns a routine to a global pointer and inserts a new node into a linked list using provided parameters.
- `forward_list_insertion`: Acts as a thin wrapper that passes its argument directly to a linked list insertion helper.
- `search_and_move_node`: Traverses a linked list to find a specific node and moves it to a new position or updates a pointer.
- `find_list_node`: Traverses a linked list to find a specific node, then stores its contents into a global variable or a destination pointer.
- `store_pointer_and_call_2`: Stores a value from a pointer into either a global variable or a provided destination before calling another routine.
- `store_pointer_and_call`: Stores a value from a pointer into a global variable and then calls another routine.
- `validate_index_and_check_value`: Checks if an index is within bounds and if a value at a calculated offset in a table is non-zero, returning a boolean result.
- `check_routine_result`: Calls a routine and, if the result is zero, fetches another value to store in a global variable and returns a specific constant.
- `return_zero_3`: An empty routine that simply returns zero.
- `access_table_element`: Validates an index against a table and a size limit, then performs an indirect call or updates a status variable.
- `execute_conditional_indirect`: Performs a conditional check on a return value and executes an indirect call if certain criteria are met.
- `execute_callback_chain`: Calls a routine using a pointer from a global variable, then executes a subsequent routine with a provided value.
- `call_callback_and_routine`: Executes a function through a global pointer if it is non-zero, then calls another routine with a provided parameter.
- `execute_system_callbacks`: Checks several global pointers and hardware status bits to conditionally execute various indirect function calls and loops.
- `nop_sequence`: This routine consists of three instructions that perform no operation and return immediately.
- `validate_and_clear_pointer`: Checks if a provided index is within the bounds of a table and if the pointed-to value is non-zero.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_dos_close`: Performs an indirect call to a function located via an offset in a table.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested table structure and executes it.
- `execute_indirect_function_pointer_4`: Fetches a function pointer from a nested structure within a table and executes it.
- `execute_indirect_function_pointer_5`: Performs an indirect call to a function address retrieved by traversing a pointer offset from a base address stored in a register.
- `execute_indirect_function_pointer_6`: Performs an indirect call to a function located by traversing two offsets from a base address stored in a register.
- `check_interactivity`: Calls a function to determine if the system is in an interactive mode.
- `open_file`: Calls a function to open a file.
- `output_data`: Calls a function to output data.
- `write_data`: Calls a function to write data.
- `call_library_offset_6c`: Calls a function located at a specific offset within a library structure.
- `execute_indirect_call_8`: Performs an indirect call to a function address calculated by applying an offset to a pointer retrieved from a table.
- `execute_indirect_function_3`: Calculates an address by applying two offsets to a pointer stored within a memory location and then calls the resulting function.
- `execute_indirect_call_9`: Calls a function through a pointer located at a specific offset within a table.
- `execute_indirect_function_pointer_7`: Calculates an address by applying two offsets to a value retrieved from a pointer located at a fixed offset from a register and then jumps to that address.
- `execute_indirect_function_pointer_8`: Performs a double-dereference of a pointer stored within a memory offset relative to a base address to call a function.
- `execute_indirect_call_10`: Executes a function by jumping to an address found via an offset from a table entry.
- `execute_indirect_function_pointer_9`: Calculates an address by applying two offsets to a value stored at a location relative to a register and jumps to that address.
- `execute_indirect_function_pointer_10`: Retrieves a function pointer from a nested offset within a table and executes it.
- `execute_indirect_call`: Performs an indirect function call by dereferencing a pointer located at an offset within a structure.
- `execute_indirect_call_11`: Triggers a function call using a pointer retrieved from a table using a large offset.
- `execute_indirect_call_2`: Calls a function through a pointer retrieved from a table using an offset from a base address.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `execute_indirect_call_3`: Executes a function via an indirect pointer stored in a memory structure.
- `execute_indirect_call_4`: Performs an indirect call by accessing a function pointer through a calculated offset.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `execute_indirect_call_5`: Calls a subroutine using a pointer found at a specific offset from a base address.
- `execute_indirect_call_6`: Executes a function by dereferencing a pointer within a memory structure.
- `execute_indirect_call_7`: Performs an indirect call through a pointer located at a specific offset from a base address.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
