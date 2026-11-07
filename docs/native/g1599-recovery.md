# Game 1599 (pga tour golf (1991)(electronic arts)[cr atx][h tension]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01599). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (958304 top-level bytes in 63 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 559 of 559 named (0 author symbols), 559 of 559 described (86 by rule, 388 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2285 of 2285 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 31016 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10000 of 10150 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 116302 |
| code | 33432 |
| data | 712279 |
| padding | 14883 |
| unused | 81408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:GOLF | 2 | 267 | 8 |
| Run | 2 | 117 | 2 |
| type3 | 1 | 343 | 24 |
| pp | 2 | 267 | 8 |
| nclick | 2 | 267 | 8 |
| eighty | 1 | 449 | 16 |
| PPLoadSeg | 2 | 314 | 13 |
| powerpacker.library | 2 | 1755 | 129 |
| diskfont.library | 15 | 1040 | 75 |
| PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3046 | 129 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 129 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 499 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1397 | 124 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:FXSMALL (raw code file, load address unknown) | 1 | 239 | 3 |

## Cases handled

- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TR4.EGA: high-entropy data file (7.86 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:SOUTHWIN.CRS: high-entropy data file (7.85 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TR3.EGA: high-entropy data file (7.81 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:EAGLE.PAK: high-entropy data file (7.62 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:AVENEL.PAK: high-entropy data file (7.66 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TR1.EGA: high-entropy data file (7.85 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TITLE.EGA: high-entropy data file (7.77 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:FXLARGE: high-entropy data file (7.87 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:EAGLE.CRS: high-entropy data file (7.79 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:FLYMUS: high-entropy data file (7.86 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:GR3.EGA: high-entropy data file (7.66 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:AVENEL.CRS: high-entropy data file (7.80 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:SAWGRASS.CRS: high-entropy data file (7.84 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:GMUSIC: high-entropy data file (7.85 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TR2.EGA: high-entropy data file (7.83 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TPCSCOTT.CRS: high-entropy data file (7.82 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:TENT.EGA: high-entropy data file (7.64 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:GR2.EGA: high-entropy data file (7.67 bits/byte), classified as packed data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:PGAWEST.CRS: high-entropy data file (7.76 bits/byte), classified as packed data
- reference run ext ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 27)
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1240 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 63 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 520 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:FXSMALL: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf: blocks 883-885 (bytes 452096-453632) outside the filesystem hold data, not code (opcode density 0.010, entropy 4.18)
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:GOLF: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:C/pp: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- pga tour golf (1991)(electronic arts)[cr atx][h tension]/PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:C/nclick: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 31016 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:GOLF

- `parse_amigados_hunks`: Parses a memory block containing AmigaDOS hunk type codes to perform relocation and memory allocation.
- `decrement_value_loop`: Decrements a value in a register until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `manipulate_memory_blocks`: This routine performs complex conditional loops to move or copy bytes between memory locations based on bitwise flags and specific byte values.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `perform_bit_rotation_2`: Performs complex bitwise shifts and rotations on data from a table and updates a hardware color register.
- `execute_trap`: Triggers a software interrupt trap.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_and_call`: This routine copies a block of data from a memory location to a stack and then calls another routine.

### type3

- `parse_command_string`: Copies a string into a global buffer, trims whitespace, and searches for a specific character flag to trigger a sequence of nested function calls.
- `execute_nested_callbacks`: This routine executes a series of function calls via pointers, checking return values at each step to determine whether to continue or return an error code.
- `execute_empty_loop`: Runs an empty loop for a specified number of iterations based on a global variable.
- `nop_padding`: This routine is empty and performs no operations.
- `call_if_string_not_empty`: Calculates the length of a null-terminated string and calls a function via a pointer in a global variable if the length is non-zero.
- `call_function_via_pointer`: Calls a function using a pointer stored at an offset from a global variable.
- `call_dos_library_function`: Calls a function from a library using an offset from a value in a register and stores the result in a global variable.
- `call_offset_function_2`: Checks a global pointer and calls a function at a specific offset, storing the result in another global variable.
- `call_alternative_offset`: Checks a global pointer and calls a function at a different offset, storing the result in a global variable.
- `call_conditional_function`: Calls a function at a specific offset only if two global variables are non-zero.
- `call_offset_function`: Calls a function using an offset from a global variable and stores the result in another global variable.
- `call_offset_function_short`: Calls a function using an offset from a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `execute_indirect_function`: Loads a function pointer from a location offset from a global variable and executes it.
- `return_global_value`: Returns the value stored in a global variable.
- `iterate_and_call_functions`: Iterates backward through a list of pointers starting from a global variable and calls a function at a fixed offset from a value in a register for each entry.
- `invoke_error_callback`: Checks if a null-terminated string is non-empty and, if so, executes a function through a pointer stored in a global variable.
- `execute_function_pointers`: Fetches a pointer from a global variable, calls the function at that address, and if the result is non-zero, calls two more functions via offsets from the original pointer.
- `calculate_list_length`: Iterates through a list of longwords until a specific sentinel value is found, returning the total byte count incremented by four for each element.
- `copy_and_decrement_list`: This routine copies blocks of memory from one location to another based on lengths derived from a list, and then enters a loop that decrements a global value.
- `update_hardware_pointer`: Iterates through a loop that modifies a global variable and interacts with a hardware video pointer register.
- `interpolate_table_values`: Iterates through a list of shorts and performs bitwise interpolation on specific byte segments of values within a certain range using a multiplier and a global mask.
- `return_without_action`: A routine the decompiler reduced to a plain return, so it has no effect that the evidence shows.
- `compare_byte_threshold`: Compares an unspecified byte against a constant value and returns.

### pp

- `relocate_amigados_hunks`: Parses a memory block to identify and process specific hunk types, such as executable or relocation data.
- `decrement_value_to_zero`: Decrements a 16-bit integer value until it reaches zero using a loop.
- `null_routine`: This routine is empty and performs no operations before returning.
- `memmove_block_data`: Copies a block of memory from a source address to a destination address using a loop.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `perform_bitwise_circular_rotation`: Processes a sequence of bits by performing bitwise shifts and circular rotations on data retrieved from a memory table.
- `rotate_and_update_color`: Performs bitwise rotations and shifts on data extracted from a table and updates a hardware color register.
- `trigger_trap`: Executes a processor trap instruction.

### nclick

- `relocate_hunk_data`: Parses a data structure containing hunk type codes to identify and relocate memory blocks.
- `decrement_value_to_zero_2`: Repeatedly subtracts one from a 16-bit register until the value becomes zero.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `memmove_block_data_2`: Copies a block of memory from a source address to a destination address using a loop.
- `test_bit_shift`: Checks if a bitwise right shift of one value by a masked amount results in zero.
- `perform_bit_rotation`: Performs a series of bitwise rotations and shifts on data extracted from a source pointer and a bit count.
- `rotate_and_update_color_2`: Performs bitwise rotations and shifts on data from a table and a register, then updates a hardware color register with the resulting bit values.
- `execute_trap_2`: This routine contains a single trap instruction.

### eighty

- `execute_callback_chain`: Checks a value at an offset within a structure pointed to by a global variable and calls a function retrieved from that structure if the value is non-zero.
- `parse_command_string_2`: Copies a sequence of bytes and performs bitwise manipulations to compare the resulting values against specific four-byte constants.
- `execute_nested_callbacks_2`: Checks a value at an offset within a structure pointed to by a global variable and, if non-zero, sequentially calls functions retrieved from other offsets in that same structure.
- `execute_conditional_callbacks`: Checks specific offsets within a structure to determine if various function pointers should be called.
- `traverse_and_allocate_list_node`: Iterates through a linked list to find a node containing a specific four-byte signature.
- `traverse_linked_list`: Traverses a linked list by following pointers until a specific value is found, then executes a function pointer from a global variable.
- `traverse_linked_list_2`: Iterates through a linked list of pointers, checking for a specific four-byte constant and a flag, while potentially calling a function through a pointer stored in a global variable.
- `copy_structured_data`: Copies several fields from a source structure into a destination buffer using multiple function calls.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `filter_linked_list_nodes`: Iterates through a linked list to identify nodes containing specific two-byte signatures.
- `swap_linked_list_nodes`: Iterates through a sequence of pointers to swap adjacent elements in a list.
- `manage_resource_state`: Checks a resource status and either updates a specific field in a structure or executes a function via a global pointer.
- `manage_object_state_and_metadata`: Updates an object's status flags and metadata by comparing specific values within a structure.
- `manage_resource_state_2`: Updates a structure with a specific short value and a pointer if certain conditions are met.
- `manage_object_state_and_metadata_2`: Updates an object's status flags and metadata by comparing specific memory values and executing function pointers stored within a structure.
- `manage_resource_state_3`: Processes a structure by comparing its internal values and potentially updating a status flag and a pointer.

### PPLoadSeg

- `load_program_segment`: Checks a global variable and interacts with library functions to manage memory or library states.
- `apply_hunk_relocations_2`: Iterates through a memory segment to identify and apply relocation data for specific AmigaDOS hunk types.
- `process_amigados_hunks_3`: Iterates through a memory segment to identify and process AmigaDOS hunk types, specifically handling executable loading and relocation data.
- `process_amigados_hunks_2`: Iterates through a memory segment to identify and process AmigaDOS hunk types, specifically handling executable loading and relocation data.
- `parse_amigados_hunks_3`: Iterates through a memory segment to identify and process AmigaDOS hunk types, specifically handling executable loading and relocation data.
- `apply_hunk_relocations`: Iterates through a memory segment to identify and process AmigaDOS hunk types.
- `parse_amigados_hunks_2`: Iterates through a memory segment to identify and process AmigaDOS executable hunk types.
- `process_amigados_hunks`: Parses and applies AmigaDOS executable hunk types to a memory segment.
- `apply_relocation_hunks_2`: Iterates through a data stream to identify and process relocation markers.
- `process_data_segments`: Iterates through a nested data structure to add values to memory locations pointed to by a table.
- `decrement_counter`: Decrements a short value in a register until it reaches zero.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a1, then returns.
- `parse_compressed_data`: Traverses a data stream, checking for specific markers to perform additions or further nested iterations on memory contents.

### powerpacker.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `set_library_base`: As library code, it stores an argument in a global variable, writes a library base into a table entry, and calls several other routines via a table to store returned values in global variables.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_and_validate_refcount`: Decrements a reference counter in a data structure and, if the counter reaches zero and a specific status bit is set, performs a library identity check and executes a sequence of cleanup routines.
- `validate_and_initialize_library`: Checks a status flag in a data structure and, if not set, verifies the library's identity by comparing a function pointer to itself.
- `verify_library_identity`: As library code, it checks if a function pointer matches itself and, if so, calls a sequence of routines to return a value from a global variable.
- `initialize_library_state`: The routine calls several library functions via a table and then returns a value read from a global variable.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_resource_parser`: The library routine calls another routine to parse a resource header and then returns.
- `decompress_bitstream_variable_length_2`: Performs bit-level decompression of a data stream using variable-length codes and a lookup table.
- `decompress_bitstream`: This routine performs bitstream decompression by reading bits from a source buffer and using them as indices into a lookup table to reconstruct data.
- `decompress_bitstream_data`: Performs bitstream decompression by reading a compressed input buffer and expanding it into a destination buffer using a lookup table and bitwise shifts.
- `decompress_bitstream_data_2`: Performs bit-level decompression of a data stream using a lookup table and bit-shifting operations.
- `decompress_bitstream_data_3`: This routine performs bitstream decompression by reading a compressed input buffer and expanding it into a destination buffer using a lookup table.
- `decompress_bitstream_data_4`: Performs bit-level decompression on a data stream using a lookup table and variable-length bit sequences.
- `decompress_bitstream_data_5`: Performs bit-level decompression on a data stream using a lookup table and bit-shifting operations.
- `decompress_bitstream_data_6`: This library routine performs bitstream decompression by reading a sequence of bits and expanding them into a destination buffer.
- `decompress_bitstream_data_7`: Decompresses a bitstream using a variable-length coding scheme.
- `decode_bitstream_run_length`: Decodes a compressed bitstream by reading individual bits across word boundaries to determine run lengths and data values.
- `unpack_bitstream_data`: Reads a compressed bitstream by extracting individual bits from memory, often spanning multiple words, to reconstruct data.
- `unpack_bitstream_data_2`: Extracts data from a compressed bitstream by reading individual bits across word boundaries.
- `decompress_bitstream_variable_length`: Performs bit-level decompression of a data stream using variable-length codes and a lookup table.
- `decompress_bitstream_data_8`: Reads a bitstream from a source buffer to reconstruct data by performing bitwise shifts and logical operations.
- `decompress_bitstream_data_9`: Expands a compressed bitstream into a destination buffer by reading variable-length bit sequences and performing bitwise shifts and logical operations.
- `decompress_bitstream_data_10`: Decompresses a bit-packed data stream into a destination buffer using variable-length bit manipulation and look-up tables.
- `extract_bitstream_run_length`: Reads a sequence of bits from a source buffer to determine a run length and then performs a memory copy operation.
- `decompress_bitstream_data_11`: Performs bitstream decompression by reading a sequence of bits to determine the length and type of data to be expanded.
- `decompress_bitstream_data_12`: Performs bitstream decompression by reading a sequence of bits to determine the length and type of data chunks.
- `decompress_bitstream_data_13`: Performs bitstream decompression by reading a compressed input buffer and expanding it into a destination memory area.
- `execute_indirect_jump`: The routine calculates an offset into a table based on a value from a pointer and performs an indirect jump to the resulting address.
- `decompress_bitstream_data_14`: Performs bitstream decompression by reading encoded bits and expanding them into a destination buffer using a lookup table.
- `compute_cyclic_checksum`: Iterates through a null-terminated string to calculate a checksum using a bitwise rotation and addition algorithm.
- `compute_checksum`: This routine computes a checksum over a zero-terminated byte stream pointed to by a0.
- `xor_decrypt_block`: The routine XORs a block of memory in place using a 32-bit key supplied in a register.
- `apply_dos_relocations_3`: Iterates through a sequence of memory hunk structures to identify and apply AmigaDOS relocation and execution type codes.
- `process_relocation_hunks`: Parses a data stream to identify and apply relocation patches by comparing hunk type codes.
- `apply_relocation_hunks`: This routine parses a data stream to identify and apply relocation patches.
- `process_dos_hunks`: Iterates through a sequence of memory hunk structures to identify and process specific AmigaDOS relocation and execution types.
- `apply_dos_relocations_2`: Iterates through a list of AmigaDOS hunk types to perform memory relocations.
- `process_dos_relocation_hunks`: Iterates through a data structure of AmigaDOS hunk types to perform memory relocations.
- `apply_relocation_hunks_3`: Iterates through a data structure to identify and process relocation hunk types.
- `decrement_word_to_zero`: Decrements a 16-bit integer stored in a register until it reaches zero.
- `return_from_subroutine`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `parse_dos_hunks`: Iterates through a data structure containing AmigaDOS hunk type codes to perform memory relocations and executable loading.
- `apply_dos_relocations`: Parses a data structure containing AmigaDOS hunk types to perform memory relocations.
- `construct_magic_signature`: Populates a memory buffer with a specific four-byte magic signature and a value retrieved from a local table.
- `set_powerpacker_header_2`: As library code, this routine selects a magic value based on a condition and copies a value from a table into a header structure.
- `set_powerpacker_header`: The library routine writes a magic value to a header structure pointed to by a pointer, copies a value from a table indexed by a register into the header, and then calls another routine via a global variable.
- `open_library_via_pointer`: As library code, this routine attempts to open a library using a function pointer from a global variable and returns a 64-bit value.
- `open_library_wrapper`: As library code, this routine attempts to open a library via a function pointer from a global variable and returns a 64-bit value.
- `execute_conditional_function_pointer`: This routine calls a function pointer retrieved from a global variable and, if the result is non-zero, returns the contents of the data registers.
- `handle_library_opening`: The library routine attempts to open a library via a function pointer from a global variable.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_powerpacker_decompress`: The routine calls a function pointer taken from a global variable and then returns its argument unchanged.
- `get_powerpacker_table_entry`: The routine takes the value in D0, inverts it, clamps the result to a maximum of 7, and returns the 32-bit word from a global table at that index.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `check_powerpacker_header`: The routine is empty and performs no operations.
- `parse_resource_header`: Copies a null-terminated string into a local buffer and validates a four-byte signature against several known constants.
- `validate_and_update_header`: Checks a data structure against specific magic number signatures and validates internal consistency.
- `unpack_resource_header`: Identifies a resource type by checking a four-byte signature and performs type-specific validation and length adjustments.
- `process_resource_header`: Identifies the resource type by checking a four-byte signature and performs type-specific validation and length adjustments.
- `unpack_resource_data`: As library code, this routine validates a resource header, performs memory allocation, and populates a data structure with a specific four-byte signature and a pointer.
- `validate_resource_header`: As library code, this routine checks a resource header for a specific signature and performs conditional cleanup of resources if validation fails.
- `cleanup_resource_on_error`: As library code, this routine performs conditional cleanup by calling function pointers from a table and a global variable after checking file status.
- `load_resource_data`: As library code, this routine validates a resource signature and populates a structure with a pointer to the data and a bit-shifted length value.
- `validate_and_store_resource`: As library code, this routine checks a constant value and performs conditional cleanup via function pointers from a table and a global variable.
- `cleanup_resource_handler`: As library code, this routine performs conditional cleanup by calling function pointers from a table and a global variable.
- `close_and_cleanup_resource`: This library routine performs file operations and conditional cleanup by calling function pointers from a table and a global function pointer.
- `close_file_and_free_memory`: The routine calls a global function pointer and a function pointer taken from a table, then returns a value from the stack.
- `process_state_logic`: The routine checks a value at an offset from a pointer, updates various fields in a table based on conditional checks, and returns either a pointer to a global variable or zero.
- `verify_checksum_and_increment_counter`: As library code, this routine compares a checksum result against a stored value, increments a counter in a structure, and sets a flag if a specific condition is met.
- `verify_checksum_status`: As library code, this routine increments a counter and compares a checksum result against a stored value to set a flag in a structure.
- `verify_checksum_and_set_flag`: This library routine compares a return value from another routine against a value in a table.
- `set_powerpacker_flag`: The routine stores the constant 1 into a global variable accessed via the A5 register and returns zero.
- `trigger_powerpacker_decompress`: This library routine receives a value in a register and, if it is non-zero, calls another routine three times in succession before returning.
- `initialize_decompression_context_2`: As library code, this routine populates a table with configuration values, bit-shift constants, and calculated buffer sizes based on input parameters.
- `setup_decompression_context`: As library code, this routine populates a table with configuration values, bit-shift constants, and calculated buffer sizes based on an input index.
- `initialize_decompression_config`: Populates a configuration structure with fixed values, buffer sizes, and scaling factors.
- `initialize_decompression_context_variable`: As library code, this routine populates a structure with configuration values, including a bit-shifted value, and calls another routine multiple times to initialize memory.
- `initialize_decompression_context_large`: As library code, this routine populates a structure with configuration values and calls another routine multiple times to initialize memory using larger buffer sizes.
- `initialize_decompression_context`: As library code, this routine populates a structure with configuration values and calls another routine multiple times to initialize memory.
- `return_immediately_2`: This library routine contains four instructions and performs no observable work before returning to its caller.
- `set_ride_stats`: This library routine stores two values from registers into a structure, stores their sum into a third field, clears a fourth field, and calls another routine.
- `set_ride_stats_2`: This library routine stores two values from registers into a structure, stores their sum into a third field, stores a value from an unused register into a fourth field, and calls another routine.
- `reset_stream_buffers`: Clears two memory buffers by zeroing out their contents based on lengths stored in a control structure.
- `update_stream_metadata`: Updates a control structure with new source and destination pointers, calculates buffer lengths, and adjusts internal offsets.
- `validate_compressed_stream_integrity`: Checks the integrity of a compressed data stream by verifying bitstream markers and ensuring the data length remains within expected bounds.
- `scan_compressed_bitstream_markers`: Scans a compressed data stream to locate specific bit patterns or markers.
- `search_compressed_bitstream_markers`: Scans a compressed data stream to locate specific bit patterns or markers.
- `reset_bitstream_buffers`: Iterates through memory buffers to set their contents to zero.
- `find_bitstream_marker`: Scans a compressed data stream to locate a specific bit pattern or marker.
- `clear_bitstream_buffers_2`: Iterates through memory buffers and sets their contents to zero.
- `search_compressed_bitstream`: Scans through a compressed data stream to locate specific bit patterns or markers.
- `clear_bitstream_buffers`: Iterates through memory buffers to set their contents to zero.
- `extract_bitstream_data`: Extracts a variable number of bits from a bitstream buffer and stores the resulting value into a destination memory location.
- `manage_bitstream_buffer`: Maintains a bitstream buffer by tracking bit offsets and performing bitwise shifts to pack or unpack data.
- `clear_memory_buffers`: Iterates through one or two memory buffers to set their contents to zero.
- `reverse_bits_and_pack_data`: Performs bitwise reversal on a value and packs the result into a destination buffer.
- `clear_and_bit_reverse_buffers`: Clears two memory buffers by iterating through them and setting their contents to zero.
- `bit_reverse_and_store`: Performs a bitwise reversal of a value and stores the result into a destination buffer.
- `decompress_bitstream_data_15`: Processes a compressed bitstream by reading bits from a source and expanding them into a destination buffer.
- `reset_bitstream_buffer`: Clears a destination buffer by iterating through a memory range and setting values to zero.
- `update_bitstream_buffer`: Manages the state of a bitstream by shifting bits and writing them into a destination buffer.
- `pack_bit_stream_buffer`: Processes a bitstream by shifting bits and packing them into a destination memory buffer.
- `flush_bit_stream_buffer`: This routine flushes remaining bits from a bitstream buffer into a destination memory location.
- `process_bit_stream_buffer`: This routine processes a bitstream by performing bitwise shifts and logical operations to extract or pack data.
- `calculate_offset_pointer_2`: As library code, this routine calculates a pointer by adding an offset to a value derived from two bytes and a structure member.
- `get_pointer_with_bounds_check`: As library code, this routine performs a bounds check on a pointer and returns either a calculated address or a value from a structure.
- `calculate_offset_pointer`: As library code, this routine selects a value from one of two offsets within a structure and returns a pointer calculated by adding a modified value to a base address.
- `subtract_memory_value`: The routine subtracts a value at an offset from a pointer from a value in a register and returns the result.
- `write_tile_mapping`: The routine iterates over a block of tile indices and writes mapping entries into two tables.
- `update_sprite_offsets`: Iterates over a byte range and updates two tables of 16-bit offsets for each referenced entry.
- `update_sprite_offsets_single`: The routine iterates over a byte range and for each byte that lies before a limit pointer it reads a byte, uses it as an index into a table of pointers, computes an offset from a stored pointer to the current byte, and if the offset is valid writes a derived value into an output table.
- `calculate_loop_iterations`: This library routine performs arithmetic operations on a value to determine a loop count.
- `run_powerpacker_decompress`: This library routine calls a function to write to an output buffer and then calls a function to check for buffer overflow.
- `shift_and_check_buffer_limit`: This library routine performs bitwise right shifts on a value and checks if a pointer increment exceeds a specified limit.
- `expand_buffer_limit_4`: This library code checks if a buffer limit has been reached and conditionally reallocates memory to expand the buffer.
- `expand_buffer_limit_2`: As library code, this routine checks if a bit-shifted value exceeds a limit and conditionally reallocates a buffer if the limit is reached.
- `process_bit_stream`: As library code, this routine performs bitwise shifts and logical operations on data in a loop before potentially updating a value in a structure and calling another routine.
- `process_bit_stream_2`: As library code, this routine performs bitwise shifts and logical operations on a bit stream and updates a structure or increments a value.
- `manage_buffer_expansion`: This library code stores a value into a memory location and conditionally reallocates a buffer if a limit is reached.
- `expand_buffer_limit_3`: As library code, this routine checks if a buffer limit is reached and conditionally reallocates memory to expand the buffer.
- `expand_buffer_and_check_limit`: As library code, this routine checks a limit, potentially increments a value in a structure, and returns a pointer from a table.
- `expand_buffer_limit`: As library code, this routine checks a limit and increments a value in a structure, potentially returning a pointer from a table.
- `increment_and_check_limit`: As library code, this routine increments a value at an offset within a structure and returns a pointer from a table if a comparison with a register fails.
- `return_stack_value`: The routine is empty and simply returns the value currently on the stack.
- `allocate_and_initialize_block`: As library code, this routine calls a memory allocation routine and, if successful, stores a value into the first element of the allocated block and returns a pointer to the next element.
- `set_stack_value`: The routine copies a 32-bit value taken from the stack into the memory location pointed to by the first argument in D0 and returns a pointer advanced by one word.
- `free_powerpacker_library`: The routine checks a pointer passed in a1.

### diskfont.library

- `update_font_handle`: As library code, this routine calls a function via a table and, if successful, stores a value into a structure and calls another function.
- `check_and_set_font_handle`: As library code, this routine calls a routine via a table entry using the library base.
- `cleanup_library_resources`: Attempts to release resources by calling functions via offsets from a base pointer.
- `unload_font_resources`: This library code iterates through a linked list to remove entries and calls a routine to unload segments from memory.
- `update_font_state`: As library code, this routine modifies a byte in a structure and updates pointers within a linked list or table.
- `set_high_bit`: As library code, this routine sets the most significant bit of a byte at a fixed offset from a pointer.
- `return_from_handler`: This library routine is an empty function that performs no operations before returning.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_dispatch_event`: As library code, this routine decrements a value in a structure and, if it reaches zero and a specific bit is set, calls a function pointer stored in that structure.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `realign_glyph_buffer_data_2`: Iterates through a linked list of font resource blocks to shift character data within a buffer and update corresponding glyph metadata.
- `populate_font_string_buffer`: Iterates through a linked list of font data structures to calculate string lengths and copy character data into a destination buffer.
- `process_linked_font_nodes`: Iterates through a linked list of font data structures, copying character strings into a destination buffer.
- `realign_glyph_buffer_data`: Iterates through font resource blocks to shift character data within a buffer and update glyph metadata.
- `parse_font_resource_blocks`: Iterates through a font resource structure to extract glyph metadata and bitmap data.
- `populate_font_glyph_table`: Iterates through parsed font resource blocks to extract glyph data and metadata.
- `parse_font_resource_block`: Reads and parses structured data blocks from a disk-based font resource.
- `extract_glyph_metadata`: Iterates through font resource blocks to extract glyph metadata and bitmap data.
- `return_zero_stub`: Library code that returns zero and performs no operations on registers, memory, or hardware.
- `calculate_bit_field_difference`: This library routine calculates a weighted value based on the difference between two short integers and a bitwise comparison of two byte sequences.
- `compare_bit_fields`: This library routine performs a bitwise comparison between two sets of bytes and subtracts values from a running total based on a table lookup.
- `search_glyph_index_table`: Performs a bitwise search through a table of values to locate a specific index.
- `search_font_glyph_table`: Iterates through a table of values using a bitwise search pattern.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_glyph_metadata`: Iterates through a font glyph table to find a specific character based on width and bitwise properties.
- `optimize_font_glyph_spacing`: Iterates through a font table to find the optimal glyph spacing by comparing character widths and adjusting metadata.
- `update_font_glyph_metadata`: Iterates through a font table to find a specific glyph and updates its internal metadata, including bitmasking and index increments.
- `find_max_glyph_index_2`: Iterates through a font entry table to identify the entry containing the largest index value.
- `search_and_update_glyph_table`: Iterates through a table of font entries to find the entry with the largest index value.
- `find_max_glyph_index`: Iterates through a table of font entries to find the index of the entry containing the largest value.
- `search_font_glyph_table_2`: Iterates through a table of font data to find a specific glyph based on a comparison of values.
- `parse_disk_font_file`: Reads a font file from disk, parses its header and character entries, and builds an in-memory table of font data.
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
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `find_string_terminator`: This library routine increments a pointer until it reaches a null byte and then calls another routine.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_null_terminated_bytes_2`: This library code increments a counter while advancing a pointer through a string until a zero byte is encountered.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_strings`: This library routine compares two null-terminated strings lexicographically and returns a result indicating their relative order.
- `compare_adjacent_string_bytes`: This library code compares adjacent bytes in a string and returns one if the first byte is less than the second, or a specific constant if they are equal or the string is empty.
- `compare_string_bytes`: This library routine compares two strings byte by byte and returns a value based on their lexicographical order.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_byte_in_string`: This library routine searches for the first occurrence of a specific byte within a null-terminated string.
- `find_first_matching_byte`: This library code searches for the first occurrence of a specific byte within a null-terminated string and returns a pointer to it.
- `search_for_byte`: This library routine iterates through a null-terminated string to find the first occurrence of a specific byte.
- `search_string_backwards_3`: This library routine finds the last occurrence of a specific byte by searching backwards from the end of a null-terminated string.
- `search_string_backwards`: This library code searches backwards from a starting pointer for the first occurrence of a specific byte.
- `search_string_backwards_2`: This library routine searches for a specific byte by decrementing a pointer until the byte is found or a limit is reached.
- `find_last_delimiter`: This library routine searches for the last occurrence of specific characters within a string.
- `increment_returned_value`: This library routine returns the value passed in d0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: Acts as a dispatcher that calls an external function pointer stored in data and then returns.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_another_routine`: As library code, this routine acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `combine_indexed_table_value`: Multiplies the input parameter by two to use as an index into a global table, retrieves a word from that table, and returns a 32-bit value composed of the high bits of the input parameter and the retrieved word.
- `put_check`: Stores a specific byte value into a text pointer table and sets a corresponding word in an option state table to one.
- `remove_check`: Sets a byte in a global text table to a space character and clears a corresponding word in a global state table based on an index passed in the parameter.
- `toggle_option_check`: Uses an input parameter as an index into a global table to retrieve a word, then flips a bit in a byte at a calculated address and calls routines to manage checkmark indicators.
- `call_remove_check`: Invokes the routine responsible for removing a checkmark indicator.
- `load_setup_options`: Calls a routine to check a value, then iterates through a loop to validate or update a fixed number of entries in a global table or state.
- `validate_and_write_config`: Checks a string or pointer and conditionally calls another routine to write a certain number of bytes to a destination.
- `write_option_states`: Writes twelve bytes from the global option states table to the configuration file defined by the filename pointer.
- `write_pixel`: Calculates the memory address for a pixel based on a coordinate and a lookup table, then writes a bit mask to four consecutive memory locations separated by fixed offsets.
- `check_byte_bit`: Loops until a specific bit in a byte at a certain offset from a pointer is cleared.
- `draw_box`: Iterates through a range defined by two parameters, calling a drawing routine at each step regardless of a global variable's value.
- `execute_recursive_line_drawing`: performs a recursive sequence of calls to a routine that draws vertical lines, using a loop that increments a counter and a second loop that iterates through a range of values.
- `draw_multiple_vertical_lines`: Iterates through a range to draw vertical lines and then executes a second loop to draw additional lines if a certain condition is met.
- `draw_vertical_line`: Increments a value stored via a pointer and calls a routine twice per iteration until a limit stored on the stack is exceeded.
- `draw_concentric_rect_borders`: Iterates a specified number of times to draw concentric rectangular borders by calling routines that draw vertical and horizontal lines with decreasing or increasing offsets to create a multi-pixel wide frame effect.
- `path_attach`: Checks a global flag and, if set, copies a string from a pointer into a global buffer and updates the pointer to reference a specific path name variable.
- `read_graf_header`: Opens a file and reads a header containing count and size fields, swapping bytes to correct endianness.
- `call_two_routines_in_order`: Calls two routines in order with no arguments and returns.
- `error_exit`: Calls a routine to return from an error state and then invokes a message display routine.
- `validate_string_range`: Checks if a string length or range or a certain value matches a target value and returns a boolean or error state.
- `validate_path_string`: Checks a global flag and attempts to attach a path string, returning a boolean or a constant based on whether a certain value matches a parameter.
- `call_and_return_constant`: Calls another routine using a value loaded from a memory location relative to a register and returns a constant.
- `dereference_and_call`: Loads a value from a memory location relative to a register and calls another routine using that value as an argument.
- `dereference_and_return`: Loads a value from a memory location relative to a register and calls another routine before returning a constant.
- `check_string_validity`: Checks a string or pointer and calls other routines or error handlers based on the result.
- `toggle_global_state`: Compares a global variable to a parameter and sets that global variable to a new constant value or toggles it.
- `toggle_global_flag_2`: Compares a global variable against a memory location and sets it to a new boolean state or a specific constant.
- `toggle_global_flag`: Compares a global variable against a memory location and sets it to a new boolean state.
- `toggle_global_variable`: Sets a global variable to a new boolean state and returns a constant based on whether the previous value matched a memory location.
- `reset_global_variable`: Sets a global variable to zero and returns a constant.
- `set_curr_font`: Checks if the input parameter is zero and stores the resulting boolean state into a global variable controlling font selection.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `swap_adjacent_bytes`: The routine iterates through a memory buffer, swapping each pair of adjacent bytes.
- `swap_longs`: Iterates through a block of memory, swapping the byte order of each 32-bit long word to convert between big-endian and little-endian formats.
- `page_flip`: Calls a hardware swap routine to change the active display buffer and toggles a global variable by XORing it with one to track the current page state.
- `empty_routine`: the routine is empty and performs no operations.
- `cleanup_exit`: Sets a music flag to zero and calls routines to terminate sprite, sound, and fly-by effects.
- `memory_error_exit`: Performs global cleanup, prints an error message string from a fixed offset, and terminates the program execution.
- `memory_error_exit_2`: Performs global cleanup, prints an error message string from a fixed offset, and terminates the program execution.
- `proshop_palette_update`: Passes a pointer to a global palette table to a color switching routine to update display colors.
- `proshop_palette_update_alt`: Passes a pointer to a different global palette table to a color switching routine to update display colors.
- `set_drawing_color`: Sets the active drawing pen to a provided value, updates a global variable with this color index, and enables a pattern using that color value for both components.
- `check_range_overlap`: Compares a global variable against a range and calls a routine if the global variable falls within certain bounds.
- `draw_line`: Moves the drawing cursor to the start coordinates provided in the first parameter and draws a line to the end coordinates provided in the second parameter.
- `draw_two_vertical_lines`: Calls another routine twice to draw vertical lines at the same x-coordinate but at two different y-coordinates provided via parameters.
- `resolve_character_font_index`: Determines a font or character index or pattern based on a character or a global flag and updates a global variable.
- `render_string_with_highlight_check`: Calculates the length of a string and iterates through it to render each character.
- `font_color`: Sets the active pen color to the value passed in the lower word of the first parameter and updates a global variable with this new color value.
- `b_print_font_at`: Calculates the length of a provided string, sets the drawing mode via a global variable, moves the cursor to a position derived from input parameters with a vertical offset, and renders the text.
- `b_print_font_centered`: Calculates the horizontal offset required to center a string based on its length and an input coordinate, configures the graphics driver, moves the cursor to the calculated position, and renders the text.
- `s_print_font_at`: Passes two parameters to a string printing routine and returns.
- `s_print_font_centered`: Calculates the horizontal offset required to center a string by halving its length and adjusting for odd character counts.
- `print_string_and_frame_values`: Passes a string and certain frame-based values to a string printing routine and another routine.
- `forward_frame_values_call`: Calls another routine with a word and a longword read from the frame.
- `print_font_at`: Checks a global flag to determine the active font style, then calls either a big font printing routine or a small font printing routine.
- `extract_nibbles_to_table_2`: Iterates through a source buffer to extract and store high or low nibbles into a destination table based on certain conditions.
- `extract_nibbles_to_table`: Iterates through a source buffer to extract and store high or low nibbles into a destination table if certain conditions or non-zero values are met.
- `extract_nibbles_to_buffer`: Iterates through a buffer to extract high or low nibbles and writes them to a destination table if certain conditions or non-zero values are met.
- `draw_pattern_from_buffer`: Iterates through a buffer and calls a routine to draw or set pixels based on high and low nibbles of a byte.
- `apply_pattern_to_buffer`: Iterates through a buffer to extract nibbles from bytes and writes them to a destination via another routine while checking certain bounds.
- `draw_pattern_with_loop`: Iterates through a buffer and calls a routine using a counter or index if certain bit-masking conditions are met.
- `apply_nibbles_to_buffer_2`: Iterates through a buffer, extracting high and low nibbles from bytes and writing them to a destination via another routine.
- `apply_nibbles_to_buffer_loop`: Uses nested loops to extract nibbles from a source buffer and write them to a destination buffer, finally storing a value in a global variable.
- `apply_nibble_to_buffer`: Iterates through a buffer, splitting a byte into nibbles and calling another routine to apply those nibbles to a destination.
- `apply_nibbles_to_buffer`: Iterates through a buffer, splitting bytes into nibbles and writing them to a destination via another routine while checking a boundary or limit.
- `store_object_pointers`: Stores a value retrieved from a pointer-based lookup into four consecutive fields of a structure.
- `free_object_planes`: Iterates through four pairs of pointers within a structure, calling a raster freeing routine for each non-null pair and zeroing the pointers upon completion.
- `free_background`: Iterates twelve times to call a routine that frees object planes, using a base address and an incrementing offset.
- `update_render_buffer_bounds`: Calculates a target buffer offset and copies or updates coordinate and size data into a temporary buffer or record.
- `copy_record_compute_byte_width`: Copies position and size fields from a source record into the current record, computes a width in bytes by dividing the x value by eight and rounding up when there is a remainder, and makes two unnamed calls.
- `menu_init`: Sets eight consecutive global variables to specific constant values.
- `store_record_to_slot`: Copies several fields from a global variable or a pointer-based source into a calculated slot in a table, then calls another routine with a calculated index and several parameters.
- `store_record_into_slot`: Copies fields from a record passed in the frame into a forty-byte slot selected by an index, then calls two unnamed routines with that slot.
- `initialize_amiga_hardware`: Calls routines to initialize hardware and page flipping, then populates a table with values derived from a global variable.
- `init_sprite_resource`: Requests a sprite handle and checks for failure, terminating the program and printing an error if the allocation fails.
- `end_sprite`: Checks a global variable for a non-zero value and calls a sprite freeing routine with a constant argument if true, then sets the global variable to zero.
- `init_shadow_sprite`: Attempts to acquire a sprite resource using a specific identifier and updates a global flag or terminates if unsuccessful.
- `end_shadow_sprite`: Checks a global variable for a non-zero value and calls a routine to free sprite index 3 if true, then sets the global variable to zero.
- `init_ball_sprite`: Requests a sprite resource from the system and assigns it to a global variable, terminating with an error if allocation fails.
- `end_ball_sprite`: Checks a global variable for a non-zero value and calls a routine to free sprite resource 4 if true.
- `pass_shared_word_to_helper`: Passes a shared word to a helper and returns.
- `execute_conditional_music_routine`: Calls a function pointer stored in a global variable and returns the combined values of two data registers, potentially executing it multiple times based on a global flag.
- `update_palette_from_table`: Iterates through a table of sixteen sixteen-bit color entries, extracting the red, green, and blue nibbles from each word.
- `mouse_position`: Reads two 16-bit values from a global variable structure and stores them into the memory locations pointed to by the two input parameters.
- `mouse_and_keyboard`: Retrieves and replies to an input message from a global pointer.
- `init_hw`: Initializes the Amiga environment by opening the diskfont, dos, graphics, and intuition libraries.
- `display_open_setup`: Allocates and initializes bitmaps and plane pointers, opens a display screen, configures the color palette, and registers an interrupt server.
- `hdswap`: The routine toggles a global index to alternate between two bitmap pointers.
- `hidden_scr`: Sets a global flag to indicate the screen is hidden.
- `hidden_scr_off`: Sets a hidden screen flag to zero and updates multiple global pointers by dereferencing a table of bitmap structures.
- `term`: Disables interrupts and closes the graphics display if active.
- `close_display_resources`: Conditionally disables a vertical blank interrupt, loads a view, waits for two vertical blanks, and frees copper lists.
- `enable_interrupts`: Writes a value to the interrupt enable register to enable interrupts.
- `set_title_box_colors`: Loads six color values from a palette table indexed by the input parameter into a global variable and a specific box text color variable.
- `set_title_box_colors_2`: Loads six color values from a palette table indexed by the input parameter into a global variable and a specific box text color variable.
- `please_wait1`: Sets a global variable to zero, then to a specific non-zero value, and finally to another small integer.
- `please_wait2`: Writes three specific constant values sequentially to a global variable.
- `default_box_colors`: Calls a routine to set title box colors regardless of a global variable's state, then calls another routine to set standard box colors.
- `read_sound_size_value`: Reads and returns the contents of a global variable.
- `sound_end`: Queries a routine to determine the sound configuration and selects a corresponding memory size.
- `fly_by_end`: Checks two global variables for non-zero values and frees the associated memory blocks using a memory deallocation routine, then sets the variables to zero.
- `init_music_buffer`: Allocates a block of memory for a music buffer and stores the pointer in a global variable, terminating with an error if allocation fails.
- `music_code`: Calls a function pointer stored in a global variable and returns the combined values of the two data registers from that call.
- `draw_hline`: Renders a horizontal line on the screen by writing to a global frame buffer pointer.
- `enable_dma_channel`: Sets a bit in the DMA controller register to enable a DMA channel.
- `transform_3d_coordinates`: Performs 3D coordinate transformation calculations for rendering graphical elements.
- `initialize_global_data`: Sets several global flags, copies a block of data from a fixed memory address to a global variable, and updates several global variables.
- `load_pointer_value`: Checks a global variable and loads a value from a fixed memory address into two different global variables.
- `copy_global_pointer`: Loads a global variable into a register and stores that value through a pointer passed in a register.
- `configure_pattern_colors`: Configures color registers for four distinct display patterns based on bit flags in the input parameter.
- `check_line_bounds`: Compares a global variable against a range and calls a routine to draw a horizontal line if the condition is met.
- `sort_attributes_and_set_color`: Sets the active drawing pen to a provided value, updates a global variable with that color, and sorts a linked list of attribute pointers using an insertion sort.
- `sort_attribute_values_and_update_table`: Iterates through a linked list of attribute pointers to extract and sort associated values using an insertion sort.
- `sort_and_update_attribute_array`: Iterates through a linked list or pointer-based array to extract and sort attribute values using an insertion-sort-style mechanism.
- `reorder_array_elements`: Shifts elements within an array and performs a comparison-based reordering or insertion.
- `update_and_sort_element_list`: Updates a table of values by adding a new value to an existing entry and then performs a bubble-sort-style swap or deletion/shift operation on a list of pointers or indices.
- `update_array_elements_and_swap`: Swaps or updates values in an array based on a comparison, then performs a conditional deletion or shift of elements within a list or table if a specific value is matched.
- `shift_and_reinsert_element`: Shifts elements in an array or list to make room for a new value and then performs a comparison-based reordering or insertion.
- `sort_and_update_attribute_array_2`: Iterates through a linked list to extract attribute values and performs an insertion sort.
- `reset_polygon_data`: Zeros out a block of 100 longwords in memory starting at a fixed address and updates a global variable.
- `fill_buffer_with_zeros`: Calculates a memory address based on an input parameter and fills a portion of that memory buffer with zeros.
- `fill_word_array_loop`: Writes a 16-bit value from a register into a memory buffer pointed to by another register, incrementing the pointer and a counter in each iteration.
- `run_startup_sequence`: Runs a fixed startup sequence by calling several initialization and setup routines, including Amiga system init, graphics init, sprite init, menu init, and a wait for blitter, then performs cleanup and exits.
- `free_object_raster`: The routine frees a raster associated with an object structure by checking a flag and calling another routine to release memory.
- `free_all_raster_objects`: Iterates through a fixed number of entries in a global table and calls an object freeing routine for each entry using data retrieved from the table.
- `free_objects_in_range`: Iterates through a range of indices retrieved from a global table and calls another routine for each index, passing a pointer to an object structure and two values from that structure.
- `free_object`: Calculates a memory offset by combining the input parameter with an index from a global table and calls a routine to release the object at that address.
- `execute_conditional_cleanup`: Checks a parameter, potentially modifies it, and calls a routine that may terminate the program or perform further calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: This routine calls another routine.
- `call_recursive_routine`: This routine calls another routine and then returns.
- `initialize_sprite_buffer`: initializes a sequence of nineteen sprite objects by iterating through a loop and calling a routine with a calculated offset and a global pointer.
- `release_planar_graphics`: A wrapper routine that calls a resource management function to deallocate memory used by certain planar graphics assets.
- `title_screen_start`: Sets a global variable, calls a routine with specific arguments, and executes a loop that calls another routine eight times.
- `gr3_mode_switch`: Sets a specific entry in a global table of blit object modes to 1.
- `gr4_mode_switch`: Loads the value 2 into a word-sized entry in a global table.
- `please_wait_message`: Calls a routine multiple times, including one call with zeroed arguments, to perform a sequence of operations.
- `initialize_game_state`: Sets a global variable and calls several routines to initialize sprite buffers, switch graphics modes, and display a wait message.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_ball_physics_state`: Updates a global state variable and certain physics-related parameters or flags based on the current ball position and velocity.
- `convert_32_to_16_signed`: Extracts the upper 16 bits from three 32-bit global variables and performs a signed rounding conversion by incrementing the result if the sign bit of the lower 16 bits is set.
- `copy_global_to_self`: Performs three self-assignments to a global variable.
- `check_cup_success`: Calculates the magnitude of ball position and velocity vectors to determine if the ball is close enough and slow enough to enter the hole.
- `clear_global_timer_flag`: Sets a specific global variable to zero.
- `read_sound_size_value_2`: Reads and returns the contents of a global variable.
- `update_table_entries`: Recursively updates or clears multiple entries in a table or buffer based on the value of a global variable.
- `queue_add_update`: Increments a global tail index and a certain global element count within a circular buffer of fixed size.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `message_window`: Initializes a graphical box and frame, sets the current font and colors, and prints centered text using string length calculations.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `do_nothing_routine_2`: The routine is empty and performs no operations before returning.
- `execute_indirect_jump_2`: Performs an indirect jump to an unrecovered address.
- `seek_file_and_output`: Performs a file seek operation and passes the result to a write and output wrapper while checking a global memory location.
- `seek_file_with_error_check`: Performs a file seek operation and stores an error value in a global variable if the seek fails.
- `seek_file_position`: Calls a routine that wraps a file seek operation using a pointer passed in a register.
- `update_table_entry`: Iterates through a table to find an empty entry and updates it with a value and a short integer, while performing various file or library-related checks.
- `process_io_and_output_result`: This routine handles input or error-related system calls and passes the resulting value to an output wrapper.
- `initialize_program_environment`: Clears a block of memory using a pointer offset from a register and checks a global variable flag to decide whether to call a cleanup routine or proceed to the main program entry point.
- `do_nothing_routine`: the routine is empty and performs no operations before returning.
- `initialize_environment`: Allocates memory for a global structure, parses arguments to set a directory, and redirects standard input and output streams.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_tokens`: Parses a source string into individual tokens, handling quoted strings and skipping whitespace or control characters.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches a certain limit.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_window_environment`: Uses library functions to open a window and assign a value or pointer to a global variable, then clears that global variable if a certain condition is met.
- `signed_divide_longwords`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `apply_signed_word_swap`: Swaps the high and low words of a value and applies a sign-based negation to the result.
- `swap_and_rotate_words_2`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtract.
- `swap_and_rotate_words`: Swaps the high and low words of a value, then repeatedly doubles a pair of words while propagating carry and performing conditional subtractions fifteen times.
- `putchar`: Invokes the _aputc routine to handle character output.
- `execute_empty_routine`: This routine calls another routine that performs no operations and immediately returns.
- `return_immediately_3`: the routine performs no operations and immediately returns to its caller.
- `write_byte_to_incremented_pointer`: This routine increments a pointer and then copies a byte from a source address to the new pointer location.
- `increment_and_set_byte`: Increments a pointer, then copies a byte from a source address to the location pointed to by the incremented pointer.
- `update_line_buffer_and_set_flags`: Updates a memory buffer or pointer-based range by writing a byte value and updating a control structure with new start/end pointers and status flags.
- `draw_line_and_update_state`: Attempts to draw a line while checking a bitmask in a global structure and passes the result to a drawing wrapper.
- `update_buffer_and_reset_state`: Checks a condition in a global structure to either reset a pointer and a status flag or advance a pointer-based buffer.
- `update_pointer_and_set_flag_2`: Updates a pointer and a status flag, and copies a byte from a source address to a new pointer location.
- `update_pointer_and_set_flag`: Updates a pointer and a value at an offset, modifies a bit in a byte, and then copies a byte from a source address to the new pointer location.
- `update_parameter_status`: Updates a status field and a pointer or value within a parameter structure based on the return value of another routine.
- `modify_buffer_status`: Sets a value in a buffer, modifies a bit in a byte, and copies a value from a pointer-based location.
- `copy_pointer_to_table`: Stores a pointer to a global variable in a table and copies a value from another memory location into a new pointer location.
- `copy_data_to_pointer`: Copies two 32-bit values from a source address to a destination address and returns a pointer to the modified data.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `check_interactive_status_2`: Checks if a value from a pointer-based table is within a certain range and updates a global variable with a status or a boolean result.
- `check_interactive_status`: Checks if the system is in an interactive state based on a value retrieved from a pointer-based table.
- `return_zero`: The routine is empty and returns a zero value to another routine.
- `draw_line_with_output`: Attempts to draw a line while checking a bitmask in a global structure and passes the result to a write and output wrapper.
- `draw_line_with_error_check`: Checks a bitmask in a global structure and attempts to draw a line, storing an error code in a global variable if the drawing fails.
- `execute_draw_with_error_check`: Calls a routine to draw a line and checks if the result is an error, storing an error value in a global variable if necessary.
- `return_no_op`: The routine is a two-instruction stub consisting only of a return, so it performs no work when called.
- `chain_result_to_output_2`: Passes the result of a function call to a write wrapper followed by an output wrapper and checks a global memory location.
- `exit_program`: Checks a global pointer variable; if it is non-null, calls the function it points to.
- `cleanup_system_resources`: Iterates through tables to close entries, calls various library functions, and executes several stored function pointers to release resources.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_clear_pointer`: Checks if a pointer or index is within a valid range and if a target value is non-zero, then potentially calls a close routine and clears the target value.
- `call_indirect_from_table`: Performs an indirect call to an address retrieved from a table located via a global pointer.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_2`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `execute_delete_file`: Calls a DOS library function to delete a file.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err_2`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock_2`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_open_file`: Calls a DOS library function to open a file.
- `call_open_2`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_parent_dir_2`: Wrapper that calls the dos library function ParentDir and returns its result to the caller.
- `call_read_2`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `execute_seek_file`: Calls a DOS library function to seek within a file.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_unlock_file`: Calls a DOS library function to unlock a file.
- `call_un_lock_2`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_write_file`: Calls a DOS library function to write to a file.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `execute_library_function`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `execute_indirect_library_call`: Loads a function pointer from a library-relative offset and executes it.
- `execute_library_function_2`: Calls a function from a library using an offset relative to a library base.
- `execute_library_function_3`: Calls a library function using a vector offset from a global library base.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_indirect_library_function`: Loads a function pointer from a table located at an offset from a global pointer and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_indirect_library_function_2`: loads a function pointer from a table or structure and executes it via a library-style indirect call.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_library_function`: loads a function pointer from a table or structure and executes it via an indirect call using a library-relative offset.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_function`: Retrieves a library base from a global variable and executes a function at a negative offset.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_function_2`: Accesses a library base via a global variable and calls a function using a large negative offset.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_function_3`: Uses a library base from a global variable to call another routine at a specific offset.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_426`: Library stub: loads a library base from a global variable and calls the library function at offset minus 426 of that base, returning its result to the caller.
- `call_library_function_4`: Performs an indirect call to a library function using a base address from a global variable.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_function_5`: Loads a library base from a global variable and jumps to a function at a calculated offset.
- `call_library_function_6`: Executes a library function by calculating an offset from a library base stored in a global variable.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_function_7`: Retrieves a function pointer from a library base and calls it.
- `execute_multiple_routines`: Executes a sequence of several other routines and a specific function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_from_indexed_table`: Loads a 32-bit value from a table using a scaled index and stores it at a location relative to a pointer.

### PGA Tour Golf (1991)(Electronic Arts)[cr ATX][h Tension].adf:FXSMALL (raw code file, load address unknown)

- `update_dma_transfer_descriptors`: Iterates through a table of structures to calculate and write source/destination addresses and transfer counts into DMA controller registers.
- `configure_dma_transfer_table`: Iterates through a table-driven structure to calculate and write source or destination addresses and transfer counts into DMA controller registers.
- `configure_dma_transfer_parameters`: Calculates a memory offset using a value from a register and writes a routine pointer, a value, and several bytes to a table entry, then modifies the DMA controller register.
