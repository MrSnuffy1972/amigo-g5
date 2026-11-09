# Game 869 (amazing spider-man, the (1990)(empire software)[cr cls]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00869). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 305 of 305 named (0 author symbols), 305 of 305 described (11 by rule, 247 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 915 of 915 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7655 of 7680 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 279894 |
| code | 26788 |
| data | 251849 |
| padding | 5245 |
| unused | 337408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:spiderintro | 1 | 89 | 3 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:ReadMe | 1 | 47 | 1 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Exit | 2 | 117 | 2 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Boot | 1 | 43 | 1 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Comic | 5 | 848 | 58 |
| Music | 2 | 396 | 8 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Loader | 1 | 68 | 1 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Spidey | 8 | 6061 | 229 |
| Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- amazing spider-man, the (1990)(empire software)[cr cls]/Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:spiderintro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:spiderintro

## Copy protection found

- none found

## Functions


### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:spiderintro

- `unpack_bitstream_data`: Reads a longword from a table, performs a series of bitwise shifts and conditional calls to another routine, and stores the resulting byte into a destination buffer pointed to by a pointer in a register.
- `process_bit_stream_2`: Loads a longword from a pointer in a0, stores its byte value into a location pointed to by a5, and returns the longword shifted right by one bit.
- `process_bit_stream`: Performs a bitwise right shift on a value in a loop, calling another routine when the value reaches zero.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:ReadMe

- `manage_library_resources`: Calls a routine to find a task, then checks a field at offset 172 of a structure; if zero, it calls another routine to update a global variable, performs several library calls to manipulate memory or messages, and finally calls two more routines before returning.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Exit

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_and_branch`: Moves the contents of d1 to d0 and then branches to another routine.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Boot

- `initialize_system_state`: Calls a routine to find a task, then if successful, calls several other routines to manage memory and library state, potentially copying a value from one structure to another, and finally returns a value.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Comic

- `jump_to_address`: Performs an unconditional jump to a long address stored at the start of the routine.
- `initialize_resource_structures_2`: Checks if a register contains a specific constant, then stores two pointers in a table, calls two routines via a library base, and performs several conditional calls to other routines using pointers from a global variable and a structure.
- `release_system_resources`: Calls another routine, then checks a global variable and, if non-zero, calls a routine via a pointer from that variable.
- `initialize_and_manage_resource_structures`: Allocates memory for a structure, sets a field at offset 184 to a maximum value, and populates several global pointers by calling routines that return memory addresses.
- `cleanup_system_resources`: Tests a byte in a global variable, and if non-zero, calls a routine using a pointer from a global variable, clears that byte, and calls two other routines; it then checks a global pointer and, if non-zero, calls a routine via a pointer in a structure, and finally checks two other global pointers to call routines via pointers if they are non-zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_global_counters`: Checks a global word, calls two routines, and increments two consecutive global words by one before returning zero.
- `clear_and_test_global_word`: Clears a 16-bit word in a global variable and returns immediately if the value is zero; otherwise, it enters an infinite loop.
- `allocate_memory_conditionally`: Calls a routine to allocate memory of size 65538 and returns the result, otherwise it jumps to another location.
- `allocate_and_copy_data`: Calls a routine via a library function, copies a sequence of words from a source structure to a destination, calls a routine with various sizes, and updates several global pointers and values.
- `rebuild_structure_list`: Calls a routine with three different size parameters, and if a global word is non-zero, it calls a routine to allocate memory, copies a sequence of words, and updates two global pointers.
- `execute_conditional_jump`: If d1 is not zero, it calls a routine via an indirect jump through a library function; otherwise, it returns.
- `swap_global_pointers`: Swaps the values of two global variables, sets a byte at a global variable to zero, and sets a word at another global variable to one.
- `update_structure_list`: If a global word is non-zero, it clears it and enters a loop to swap and store values from a global longword into a series of word-sized entries in a structure.
- `copy_word_sequence`: Copies 16 words from a source pointed to by a0 to a destination starting at a global variable, where each word is also written to an offset of 32 from the destination pointer, using a loop.
- `set_handler_data`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `manage_file_buffer`: Uses a table of function pointers to perform operations, including checking a value, seeking, and reading, while conditionally calling another routine.
- `process_conditional_call`: If a2 is not zero, it calls a routine and returns a value of negative one; otherwise, it calls another routine and returns a value of negative one.
- `call_two_subroutines`: Calls two subroutines and then returns.
- `execute_sequence_of_routines`: Writes the constant 3822 to an offset of a structure pointed to by a0, then calls four different routines in sequence.
- `copy_compressed_data_blocks_3`: Pushes three values onto the stack and calls a routine via a pointer.
- `process_data_sequence`: Initializes a counter to 4 and loads a pointer from a table, then enters a loop that calls two routines, performs several comparisons on a value in d0, calls a third routine, and calls a fourth routine via a pointer before decrementing the counter.
- `apply_masked_updates`: Saves d6 to the stack, calls another routine, calculates an address by adding 40 to a global pointer, sets d3 to 15, calls another routine, sets d3 to 240, calls another routine, enters a loop that iterates 18 times, calls several routines with different values in d3, and finally calls another routine before restoring d6 and returning.
- `copy_and_mask_data`: Copies words from a source structure to a destination structure while applying a bitwise OR with d3 to specific offsets, then executes a loop to perform similar operations, calls another routine, and branches to another routine.
- `copy_and_mask_bytes`: Copies bytes and words from a source to a destination while applying a bitwise OR with d3 to specific offsets, then executes a loop to perform similar operations, calls another routine, and branches to another routine.
- `copy_and_mask_bytes_limited`: Copies bytes from a source to a destination while applying a bitwise OR with d3 to specific offsets, then executes a loop to perform similar operations, calls another routine, and branches to another routine.
- `apply_bitwise_or_to_sequence`: Enters a loop that copies words from a source to a destination and performs a bitwise OR with d3 on specific offsets of the destination structure.
- `call_and_branch`: Calls a routine, loads the value two into d7, and branches to another routine.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `update_32_bit_accumulator`: Adds a 16-bit value from a memory location to a 32-bit value stored across two memory locations, then updates those locations with the result.
- `loop_and_test_condition`: Runs a loop using a counter in d7, calling two different routines and checking a condition based on the zero and negative flags before returning.
- `toggle_status_flags`: Reads two bytes from a memory location, swaps them, and then uses bit testing to return various longword constants or clear a global word.
- `load_source_pointer`: Loads a long value from a global variable into a0.
- `copy_data_block`: Copies 2000 words from the source pointed to by a0 to the destination pointed to by a1 using a loop.
- `test_d0_bit`: Performs a bitwise OR on d0 with zero and then tests bit 0 of d0.
- `clear_d0_multiple`: Performs four consecutive bitwise OR operations on d0 with the value 255 and then with zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_d0`: Performs a bitwise OR operation on d0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clears_data_register_zero_2`: Performs a bitwise OR operation on register d0 with zero.
- `test_and_call_routines`: Calls two different routines and checks a byte at a global address; if the byte is not negative, it calls a third routine before returning.
- `fetch_and_reset_global_word`: Checks if a global word is set to a specific value; if not, it calls another routine, processes the result by swapping bytes and combining it with the global word, and then resets the global word to a specific value and clears a following word before returning.
- `set_global_constants`: Stores the constant 65535 in one global word and zero in another global word before returning.
- `initialize_resource_structures`: Calls two routines to populate global pointers, and if the second pointer is non-zero, calls a routine via a pointer in a global variable; it then repeats this process for a second set of pointers, and if the resulting pointer is non-zero, calls a routine via a pointer in a global variable and writes a pointer and a word to that structure before calling another routine via a pointer in a global variable.
- `process_object_cleanup`: Checks two global pointers; if they are non-zero and their associated structures have non-zero fields, it calls two routines via a pointer in a global variable, calls two other routines, and then performs similar checks and calls for a second global pointer.
- `compare_and_clear_byte`: Compares a byte at offset 4 of a structure in a0 to the constant two; if they are not equal, it checks a value at a PC-relative address and returns zero if that value is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies a0 to d0, then returns.
- `filter_and_store_structure_data`: Checks if a byte in d0 is one and if a byte at offset 6 of the structure in a0 is positive; if bit 6 of the word at offset 8 is clear and a global variable is zero, it stores the byte from offset 6 and the word from offset 8 into two consecutive global words and returns zero, otherwise it returns the original value in d0.
- `store_values_by_type`: Compares a byte in d1 against two constants, and if neither matches, stores d1 into a global word and d0 into another global word before returning.
- `set_view_and_return`: The routine writes a constant to a global variable, then calls another routine via a table of function pointers, and returns the value received in register A0.
- `set_structure_pointers`: Clears a 16-bit word at a global variable, then stores the value zero in two different offsets of 50 and 54 of a structure pointed to by a global variable.
- `calculate_offset_address`: Adds two 16-bit values from a structure in a0 to d0 and d1, multiplies d1 by a 16-bit value from a pointer in a1, and returns a pointer calculated by adding the results to a base address.
- `copy_compressed_data_blocks`: Copies data from a source buffer to a destination buffer using a nested loop structure that handles variable-length data segments.
- `update_button_status`: Reads a 16-bit word from offset 12 of a structure in a0, performs bitwise transformations on it, checks bit 7 of the CIAA Port A register, and sets various bits in a byte before storing that byte and its bitwise OR with itself into two consecutive global byte locations.
- `clears_data_register_zero_3`: Performs a logical OR operation on data register zero using a zero constant.
- `allocate_and_initialize_structure`: Calls several functions to allocate memory and signals, then populates a structure with data from a global variable and other sources.
- `free_message_port`: Validates a pointer, invokes cleanup routines, sets specific bytes to zero or maximum values, and frees associated memory and signals.

### Music

- `branch_to_audio_initialization`: Performs a short branch into another routine.
- `initialize_audio_server`: Calls a routine to write negative one to a memory location, sets a6 to a value from the stack, loads a constant into a1, sets d0 to 5, and calls a library function via a6.
- `write_byte_to_memory`: Writes the byte value negative one to a memory location.
- `update_music_playback_state_2`: Increments a byte in a structure and, if it reaches a limit, performs several operations including calling a routine four times to update hardware registers and copying values from a table to various memory locations.
- `process_music_command_data`: Reads a command structure from a pointer, uses a nibble from the command to index into a table to update playback position, volume, and channel data, and then processes specific command bytes to manipulate hardware registers, toggle CIAA port A, or update playback state.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to the low twelve bits, stores the result into an offset of the same structure and clears a byte at another offset.
- `update_music_playback_state`: Processes a command byte from a structure in a6 to manipulate a playback position and volume, or to toggle a bit in the CIAA port A data register.
- `process_audio_data`: Iterates through a data buffer to calculate values, writes them to a destination table, and clears several hardware audio registers and CIAA registers.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Loader

- `load_system_libraries`: Opens several libraries, calls functions via pointers obtained from those libraries, and handles memory allocation and error checking.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:Spidey

- `jump_to_address_2`: Performs a long jump to the address stored in the first byte of the program.
- `initialize_system_and_call_handlers`: Checks d0 for a specific constant, stores pointers from a0 and a1 into global variables, and calls several functions via pointers retrieved from a library and global structures.
- `dispatch_library_messages`: Calls a routine, then checks a global pointer; if non-zero, it calls a routine via a pointer in a global variable, checks a second global pointer, and if non-zero, calls two more routines via pointers in a global variable.
- `initialize_audio_and_hardware_structures`: Calls several routines to populate global pointers, sets a byte from a library call into a global word, and performs pointer arithmetic to update a table of function pointers.
- `dispatch_device_messages_2`: Checks a byte at a global address and, if non-zero, calls a routine to clear it and then calls two other routines; it also checks two other global pointers to conditionally call routines and library functions.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `increment_global_counters_2`: If a word in a global variable is non-zero, it calls three different routines, then increments two consecutive words in a global variable and returns.
- `clear_and_loop_on_nonzero_word`: Clears a 16-bit word in a global variable and returns immediately if the value is zero; otherwise it enters an infinite loop.
- `allocate_and_call_function`: Loads a constant into d1, calls a function via a pointer at an offset from a global variable, and either jumps to a fixed address or returns based on the result in d0.
- `copy_and_initialize_data`: Calls a routine via a pointer from a global variable, copies 137 words from a source structure to a destination table using a loop, calls another routine three times with different constant values, and updates three global variables with the results of those calls.
- `update_structure_pointers`: Calls a routine twice with different constant values, and if a specific global word is non-zero, it calls a routine, copies 137 words from a table to a structure, and updates two pointers in a structure at offsets 50 and 54.
- `call_function_via_pointer`: If a provided pointer is not zero, it calls a function at an offset from a global variable via a jump.
- `swap_and_reset_globals`: Swaps the values of two global pointers, clears a byte at a global variable, and sets a word at another global variable to one.
- `process_data_and_update_pointers`: If a word in a global variable is non-zero, it clears that word, enters a loop that runs four times to write swapped halves of a longword from a global variable into a table at a specific address with an incrementing offset, and finally updates two pointers at offsets 50 and 54 of a structure pointed to by another global variable.
- `copy_interleaved_word_sequence`: Copies 16 words from a source pointed to by a0 to a destination starting at a global variable, while simultaneously writing each word to an offset of 32 from that same destination pointer using a loop.
- `set_handler_data_2`: The routine checks a pointer argument and, if non-zero, calls a function obtained from a global variable table.
- `reset_object_flags_2`: The routine clears a byte at an offset in a structure pointed to by a parameter and sets two 32-bit fields to all ones, then calls another routine via an indirect pointer from a global variable and returns its result, or returns a constant if the parameter is zero.
- `manage_file_position`: Uses library functions to seek, read, or manipulate file positions based on comparisons of input values and current file state.
- `call_state_verification_function`: Saves registers to the stack, calls a routine through a pointer from a table, compares the result with a register, and then either branches or sets a 32-bit flag to zero before returning.
- `verify_state_and_return_flag`: The routine calls another routine through a pointer stored in a table, compares its result with a register value, sets a 32-bit flag to all ones if they differ and to zero otherwise, calls another routine through a pointer stored in a table, and returns a 64-bit value formed from the flag and the incoming stack value.
- `execute_sequence_of_subroutines`: Calls three subroutines in sequence and then performs a long jump to another location.
- `initialize_memory_buffers`: Copies several blocks of data to various destinations using a routine that performs structured copies, then executes a loop that performs two structured copies for each of eight iterations, and finally performs a loop that performs two structured copies for each of sixteen iterations.
- `copy_charset_data`: Copies a block of data to a destination and then executes a loop that performs two structured copies for each of four iterations.
- `dispatch_message`: Calls another routine, tests d0, and if d0 is less than zero, branches to another routine; otherwise it sets a0 to zero and returns.
- `copy_bytes_with_stride`: Performs a loop 128 times, copying a byte from a source at a0 to a destination at a1, then incrementing a0 by 6 and a1 by 1 in each iteration.
- `copy_structured_data_nested`: Executes a nested loop where the outer loop runs 7 times and the inner loop runs 167 times, copying four 32-bit words from a source structure to a destination structure with specific offsets.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_and_update_entity_state`: Clears a series of memory blocks, initializes several global pointers and counters, and performs a sequence of data copies and bitwise operations to update entity attributes and status.
- `update_global_pointer`: Loads a longword from a global variable, performs a bitwise rotation and an addition, and stores the result back into another global variable.
- `execute_conditional_calls`: Extends d0 and branches if it is negative; otherwise it calls four routines in sequence and returns.
- `process_input_command_sequence`: Processes a character input by comparing it against specific values; it updates a global word with a value from a memory location, toggles a bit in a global byte, or executes a block copy of 2000 words from one memory location to another based on the input character.
- `copy_memory_blocks`: Swaps d0, tests a 16-bit value in a global variable, and if it is positive, checks if d0 equals 95; if so, it sets a global word to 191, calls another routine, loads two pointers from global variables into a0 and a1, and enters a loop to copy 1999 words from a0 to a1.
- `update_counters_based_on_thresholds`: Runs a loop based on a value from a global variable, performing various arithmetic operations on several global 16-bit and 8-bit counters depending on comparisons between a byte from a structure and specific thresholds.
- `clears_data_register_zero_4`: Performs a logical OR operation on register d0 with zero.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with a zero constant.
- `process_object_collision_and_state`: Iterates through a list of objects to check for collisions against a spatial boundary table and updates object status bits or triggers a routine call based on collision results.
- `copy_and_update_data_blocks`: Decrements a byte at a global address and calls a routine; it then performs a loop to copy words from a source to a destination, and if a certain condition is met, it performs a second loop to copy larger blocks of data using a multiplier.
- `initialize_data_pointers`: Stores two 16-bit values into global memory locations, updates a pointer by adding 992 to it, loads a new address into a0, sets d0 to 192, and branches to another routine.
- `process_sprite_position_and_update_state`: Checks a status byte and a bit in a global variable, then retrieves a value from a table to call another routine.
- `clear_d0_register`: Performs a logical OR operation on the d0 register with zero to ensure its value is zeroed.
- `test_status_and_call_handler`: Performs a logical OR on a register, tests bit 1 of a global variable, and if that bit is set, calls another routine with a value of 9 in d0; otherwise it returns.
- `initialize_and_process_status`: Sets a 32-bit value and a 16-bit value to all ones, clears bit 7 of a global variable, and if bit 1 of that variable is set, loads a byte from a pointer, calls two other routines, and returns; otherwise, it performs complex arithmetic and memory access operations involving division, bit shifts, and conditional loops to update memory based on calculated offsets.
- `update_parameter_state`: Loads two addresses into a1 and a2, calls another routine, stores d4 and d5 into the locations pointed to by a1, calls another routine, loads a pointer from a global variable into a0, compares d0 to 3, performs a table lookup in d0 if it is not less than 3, calls another routine, loads a pointer from a global variable into a0, calls another routine, and stores d7 into a global variable.
- `calculate_summed_values`: Calculates two values by summing several 16-bit words from different global locations and adding 10 to each, then returns them in d4 and d5.
- `calculate_vector_intersection`: Calculates intersection coordinates by performing arithmetic on two vectors derived from global pointers and input registers.
- `process_object_update_loop`: Clears a series of bytes in a table and iterates through a list of structures, updating 16-bit values at offsets 40, 42, and 44 based on results from other routines.
- `process_structure_list`: Iterates through a list of structures pointed to by a value at offset 46 of a structure in a6, calling another routine and updating values in the structure in a6 based on a table and input registers until a bit is cleared in a byte at offset 12 of a pointer.
- `calculate_blitter_draw_parameters`: Calculates the width and height of a rectangular area by adding offsets from a structure in a6 to values stored at offsets 18 and 20, then passes these dimensions and a pointer to a data structure to calculate_blitter_area_and_draw.
- `update_structure_state`: Checks a global variable and a bit in a structure in a5 to determine whether to clear a byte in a6 or perform a series of operations involving fetching nibbles from a table, calculating differences between values at offsets 30, 32, 34, and 36 of the structure in a6, and running a loop that calls another routine and updates values in a6.
- `process_collision_data`: Iterates through a table, performing bitwise rotations and comparisons on values from the table against bytes loaded from a0, and increments a byte in the table if a condition is met.
- `update_entity_position_and_attributes`: Calculates movement deltas for two 16-bit values based on scaled offsets and iterates through a table of structures to update their coordinate and attribute fields.
- `parse_and_populate_entity_structures`: Iterates through a source data table to populate a list of structures, calculating coordinate offsets and bit-packed attributes from the input bytes.
- `copy_nested_data_blocks`: Performs a triple-nested loop to copy bytes from two source tables into four destination buffers, incrementing pointers by specific offsets in each loop iteration.
- `copy_data_blocks_loop`: Copies two blocks of six bytes each from a source table to a destination table using a loop that runs seven times.
- `iterate_and_call_nested_data_blocks`: Iterates through a table of pointers using a count from a global variable, calls copy_nested_data_blocks_2 if the byte at the current pointer is non-negative, and increments the pointer by 210 bytes each iteration.
- `update_entity_attributes`: Iterates through a table to calculate and store new byte values into a destination buffer based on bitwise operations and arithmetic on values from a source table and a global structure.
- `update_entity_positions_and_bounds`: Iterates through a table of structures to update entity coordinates and bounding boxes by reading scaled offsets from a table and adding them to existing values.
- `update_sprite_collision_bounds`: Calculates and updates sprite bounding box coordinates by reading dimensions from a structure in a2 and writing them to a destination buffer in a5.
- `toggle_structure_bits`: Calls two routines to toggle bit 7 and bit 3 of a byte at offset 12 of a structure in a2, then returns.
- `calculate_offset_value`: Calculates a 16-bit value by performing arithmetic and bit shifts on two 16-bit words from a table and adds the result to a pointer in a0.
- `copy_nested_data_blocks_2`: Calls another routine, then enters a nested loop that copies bytes from a source structure at offset 18 of a2 to a destination pointed to by a3, using d0 and d1 as loop counters.
- `toggle_bit_seven`: Checks bit 7 of a byte at offset 12 of a structure in a2 and toggles it by shifting and XORing the value.
- `toggle_bit_three`: Checks bit 3 of a byte at offset 12 of a structure in a2 and toggles it by shifting and XORing the value.
- `setup_blitter_parameters`: Saves registers, stores a 32-bit value from offset 14 of a2 into a global variable, and calls a routine to configure the blitter using values from a1 and a2.
- `update_sprite_data_2`: Calculates an index and bitmask from a source table, conditionally modifies a byte in a target buffer, and then writes a large block of data.
- `write_entity_state_record`: Calculates an index and bitmask from a source table, conditionally modifies a byte in a target buffer, and then writes a large block of data including pointers and bytes to a destination buffer.
- `modify_bit_buffer`: Clears a bit in a structure at a6, calls another routine, and then enters a loop that reads bytes from a2 to perform bitwise operations on a target buffer pointed to by a table, occasionally calling another routine or updating a value at an offset of 984 from a0.
- `update_buffer_from_table`: Checks a condition in a structure, modifies a byte in a target buffer based on a bit test, and then copies seven 4-byte sequences from a table into a destination buffer.
- `update_sprite_data`: Calculates memory offsets based on input parameters and performs nested loops to copy blocks of four bytes from a source table into a destination buffer.
- `process_bit_table`: Loads a word from a table into d7, enters a loop that iterates d7 times, loads a pointer from a table into a6, loads a word from offset 4 of that pointer into d6, tests bit d6 in the memory at a6, and if the bit is set, calls another routine using a pointer from offset 18 of the table.
- `decompress_data_stream`: Pushes a0 to the stack, loads a pointer into a1, calls a routine to decompress data, and returns.
- `load_pointers_to_registers`: Loads a 16-bit word from a global variable into d0 and sets a0 and a1 to specific constant addresses.
- `decompress_custom_stream`: Reads a compressed data stream from a table pointed to by a pointer in a1 and writes the decompressed bytes to a buffer in a0.
- `parse_input_stream_and_update_structure`: Checks a global variable and a bit in a structure to decide whether to update a 32-bit value in a global variable using a byte from offset 13 of the structure in a6.
- `process_input_stream_and_update_structure`: Reads bytes from a buffer in a0, calling negate_registers_by_bit to adjust values and update_clamped_velocity_and_position to modify a structure in a6.
- `update_structure_values`: Calculates and stores two 16-bit values into a structure at offsets 30, 32, 34, and 36 based on values from a table and input registers, then returns.
- `store_and_scan_buffer`: Stores a pointer from a0 and three values from a6 into a structure, then loops through the memory pointed to by a0 until it encounters a byte with the value negative 128.
- `update_structure_fields`: Reads a byte from a0, uses it to index a table to update a 32-bit value at offset 14 of a6, and conditionally updates a 16-bit value at offset 2 and an 8-bit value at offset 4 of a6 based on bits in a byte at offset 27 of a6.
- `reset_structure_fields`: Checks bit 0 of offset 27 in a6; if clear, it clears offsets 22 and 1 of a6, then enters a loop that calls another routine and updates offsets 18 and 1a of a6 using values from offsets 36 and 38 of a6.
- `update_structure_fields_2`: Copies a byte from a pointer in a0 to offset 27 of a6, calls two other routines, updates offset 24 of a6 by adding the value at offset 54 of a6 shifted left by 1, updates offset 26 of a6 by adding the value at offset 56 of a6 shifted left by 1, and clears offsets 40, 42, 54, and 56 of a6.
- `load_three_values`: Loads a 16-bit word from a0 into d2, then loads a byte from a0 into d3 and another byte from a0 into d5.
- `fetch_table_byte`: Selects one of four tables based on the signs and values of d2 and d3, then returns a byte from that table using an offset calculated from d2 and d3.
- `update_clamped_velocity_and_position`: Calculates clamped velocity values and updates a structure at offset 1 of the structure in a6, then performs a loop to adjust position values by calling another routine and updating offsets 34, 35, 36, and 37 based on the results.
- `negate_if_negative`: Checks the signs of d0 and d1, and if either is negative, negates that register before returning.
- `clear_d5_from_a6`: Loads a 16-bit word from offset 2 of a6 into d4 and then clears d5 by loading a byte from offset 4 of a6 into it.
- `increment_and_shift_values`: Increments d4 and d5 by 10 in a loop until d5 reaches 210, then performs an arithmetic shift right by 3 on both registers and returns.
- `negate_registers_by_bit`: Checks bits 0, 1, and 2 of d1 to determine whether to negate d3, d2, or both, then returns.
- `extract_combined_nibble`: Pushes a0 to the stack, performs a bitwise operation on a value from a byte at offset 7 of a6 and the result of a call to another routine, then returns the combined value in d0.
- `call_pointer_update_handler`: Pushes a0 to the stack, loads a pointer into a0, calls another routine, and returns.
- `invoke_pointer_update_handler`: Pushes the current value of a0 onto the stack, loads a constant address into a0, calls extract_nibble_from_table, restores the original value of a0, and returns.
- `extract_nibble_from_table`: Calculates an index using d0 and d1, fetches a byte from a table at that index, and returns either the high or low nibble of that byte based on a bit in d1.
- `check_bounds_and_call_handler`: Checks if a6 is not a specific address and if bit 7 of offset 12 of a pointer at offset 46 of a6 is set, then verifies if four values from a6 fall within a specific range defined by a table, and if so, adds a value from a6 to a global variable and calls another routine.
- `update_bounding_box_center`: Calculates the midpoint of two coordinate ranges provided in a structure, checks if this midpoint falls within specific bounds relative to a reference point, and if so, updates a global minimum distance value and several global variables with the new midpoint and the structure pointer.
- `process_coordinate_list`: Loads a word from a pointer in d5, sets a global variable to 32767, and enters a loop that checks a bit in a structure at offset 46 of a5, calls another routine, and increments a5 by 438 until a counter reaches negative one; if the global variable was changed, it updates a pointer in a2 and modifies several global variables and structure fields.
- `perform_logical_or`: Performs a bitwise logical OR on d0 with zero.
- `clear_data_register_zero_2`: Sets the lowest byte of data register zero to zero.
- `process_coordinate_table_loops`: Checks the sign of two 16-bit values in d2 and d3; if positive, it calculates a loop count by subtracting two scaled values from a structure in a6, then executes a loop that calls fetch_table_byte and potentially calls toggle_bit_seven or toggle_bit_three while incrementing a value at offset 58 of the structure in a6.
- `copy_data_blocks`: Copies blocks of four bytes from one source to a destination using nested loops, where the source address is determined by either a value from a table or a value from a global variable, and the destination is updated through a series of offsets.
- `update_sprite_attribute_table`: Iterates through a table of bytes, modifies a nibble in each byte to increment a value, and uses that value as an index to copy blocks of 8 bytes from a source buffer into a destination table.
- `update_sprite_palette_data`: Reads data from a source pointer and writes it to a destination pointer in a loop of 20 iterations.
- `branch_and_call_handler`: Sets bits 22, 26, and 30 of d0, branches to another routine, and calls a routine through a pointer.
- `decompress_variable_length_data_blocks`: Decompresses data by reading bytes from a source buffer and writing them to a destination buffer, using a control byte to determine if the next segment is a literal sequence or a copy from a previously written block.
- `manage_dynamic_list_elements`: Clears two global bytes and then processes a list of elements by shifting bytes within a table, inserting new values, or deleting existing ones based on a comparison of a byte value against several constants.
- `process_address_offset`: Calls another routine, then calculates a destination address by adding a word from d0 to a pointer loaded from a global variable, and calls two more routines before branching to another routine.
- `set_byte_at_address`: Stores the byte value 255 at the address stored in a long word at a global variable location.
- `clear_d0_register_5`: Sets d0 to zero and branches to another routine.
- `clear_byte_at_address`: Clears the byte at the address stored in a long word at a global variable location and returns.
- `subtract_from_two_globals`: Subtracts 8 from two different 16-bit values stored at two global variable locations and returns.
- `add_to_two_globals`: Adds 8 to two different 16-bit values stored at two global variable locations and returns.
- `calculate_and_store_offsets`: Takes a byte from a global variable, sign-extends it, multiplies it by 8, adds 64, and stores the result in one 16-bit global variable and the result plus 7 in another 16-bit global variable.
- `copy_structured_data_blocks`: Calculates an offset based on a global variable, then enters a loop to copy blocks of four bytes from two source tables into a destination structure, while also clearing specific bytes in the destination structure during certain iterations.
- `convert_and_load_data`: Calls another routine, loads a long word from a global variable into d0, calls a routine at a pointer stored in a global variable, and sets d3 to 23.
- `copy_multiple_data_entries`: Uses a byte from a source list to calculate an offset into a table, then copies eight bytes from that table location into a destination structure, repeating this for a number of entries specified by a second source value.
- `convert_and_copy_data`: Calculates four decimal digits from a value in d0, stores them into a global table, and then calls another routine to copy eight bytes from eight different table locations into a destination structure using a loop.
- `execute_data_processing_loop`: Calls three routines, loads a pointer from a global variable and adds a constant to it, then enters a loop that runs 16 times, calling a routine via a pointer and another routine twice within the loop before returning.
- `update_dynamic_list_and_rebuild_buffer`: Iterates through a list to calculate a new size and updates several global word values, then if a condition is met, copies data from one table to another and builds a new buffer by inserting a specific byte at the end.
- `prepare_blitter_parameter_table`: Fetches a byte from a global variable, performs bitwise masking and subtraction, and uses the result to select a jump target from a table to call a routine with three specific pointers in registers a0, a1, and a2.
- `copy_multiple_entries`: Calculates a destination address based on a global pointer, a value from a global variable, and an offset, then enters a loop that calls another routine to copy eight bytes from a source list into a destination structure.
- `process_user_input_loop`: Calls several routines, then enters a loop that compares a byte in d0 against specific character values; if the byte matches certain characters, it sets a word at an offset of a global variable to all ones and clears a word at another global variable, otherwise it branches to another routine or repeats the loop.
- `tiny_moveq`: Small routine of 1 instruction that loads 19 into d3.
- `copy_conditional_data_blocks`: Iterates through a list of bytes; if a byte is non-zero, it uses that byte to calculate a table offset and copies eight bytes into a destination structure, then increments the destination pointer by 1952.
- `execute_sequential_calls`: Calls three different routines in sequence, loads a value from a global variable into d4 with an offset, and moves the resulting address into a2.
- `execute_sequential_copies`: Adjusts a pointer in a2 and calls another routine five times with varying offsets to copy eight bytes from a source list into a destination structure.
- `copy_single_data_block`: Calculates an offset from a value in a source list, then copies eight bytes from that table location into a destination structure using a loop that runs eight times.
- `add_to_32_bit_accumulator`: Adds a 32-bit value from two memory locations to a 32-bit value stored in two other memory locations, then updates the destination locations with the result and returns the high 16 bits of the sum.
- `set_d7_register`: Sets d7 to zero and branches to another routine.
- `loop_and_test_flags`: Calls a routine, then calls another routine and branches if the result is less than or equal to zero; otherwise it enters a loop that decrements d7 and repeats the calls until d7 reaches negative one.
- `evaluate_status_byte`: Moves a byte from a global variable to a second global variable, performs bitwise checks on the value, and returns one of several possible 32-bit values in d0 while updating a global word.
- `update_blitter_parameter_structure`: Calls a routine to process blitter data, then updates a structure pointed to by a6 with calculated bitwise values, coordinate offsets, and flags.
- `call_copy_compressed_data`: Pushes two 16-bit values, a 32-bit zero, and two 32-bit pointers onto the stack before calling a routine to copy compressed data blocks.
- `clear_memory_block`: Loads a pointer from a global variable and uses a loop to write zero into each 32-bit word at that location 4095 times before returning.
- `load_pointers`: Loads two global variables into registers a0 and a1.
- `copy_memory_block`: Copies 1999 32-bit words from the source pointed to by a0 to the destination pointed to by a1 using a loop.
- `test_register_bit`: Performs a bitwise OR on register d0 with zero and then tests bit 0 of register d0.
- `set_register_bits`: Performs four consecutive bitwise OR operations on register d0 with the values 255 and 0.
- `perform_bitwise_or`: Performs a bitwise OR on register d0 with zero.
- `clear_data_register_zero_3`: Sets the least significant byte of data register zero to zero.
- `clear_data_register_zero_4`: Performs a bitwise OR operation on data register zero with zero.
- `clears_data_byte`: Performs a logical OR operation on the least significant byte of register d0 with zero.
- `clear_d0_register_2`: Sets the least significant byte of the d0 register to zero.
- `execute_conditional_tasks`: Saves registers to the stack, calls another routine, calls another routine, tests a byte in a global variable, and based on the result, calls one or two other routines before restoring registers and returning.
- `update_global_status_word`: Checks a global variable, calls a library routine, and if the result is one, constructs a 32-bit value from a shifted byte and a global variable before storing it and resetting the global variable.
- `initialize_global_values`: Sets a 16-bit global variable to all ones and clears a 16-bit global variable before returning.
- `initialize_device_structures`: Calls several routines to populate global pointers, then uses a library call to check a condition; if successful, it performs further library calls and writes a word and a long value into a structure pointed to by a register before jumping to another routine.
- `dispatch_device_messages`: Checks two global pointers; if their respective structures have non-zero values at offset 20, it performs library calls and calls other routines using pointers from those structures.
- `test_byte_and_return`: Compares a byte at offset 4 of the structure in a0 to two, and if it matches, checks a global word before returning zero or otherwise returning d0.
- `copy_pointer_to_d0`: Copies a long value from a pointer in a0 to d0 and returns.
- `check_and_store_structure_data`: Checks if d0 is one and if a byte at offset 6 of the structure in a0 is non-negative, then checks bit 6 of a word at offset 8 and a global word before storing values into a global table.
- `update_global_values_conditionally`: Compares d1 against two constants and, if they match, stores d1 and d0 into two global word locations before checking a global word and potentially returning.
- `set_view_and_return_2`: The routine writes a constant to a global variable, then calls another routine via a table of function pointers, and returns the value received in register A0.
- `process_cheat_confirmation_input`: Clears a 16-bit word in a global variable and sets two pointers in a structure to zero, then enters a routine that processes character input to handle a confirmation prompt.
- `reset_and_retry_sequence`: Calls two routines, loads a pointer into a0, calls a third routine, sets a global variable to negative one, and then enters a loop that calls two routines and branches back to a previous instruction if a zero flag is not set, otherwise it returns.
- `calculate_table_offset`: Calls several routines, then subtracts 49 from d0; if the result is between 0 and 4 inclusive, it shifts the value left by 6, adds it to a constant, and stores the resulting long value into a global variable.
- `initialize_data_buffer`: Stores a pointer into a global variable, clears a byte at a global variable, and fills a 23-byte buffer starting at a specific address with the value 32 using a loop.
- `update_state_and_process_list`: Updates several global bytes and words, then enters a loop that compares a masked value against a global variable.
- `find_non_space_character`: Skips spaces in a buffer pointed to by a0 using a loop, then compares a byte from a1 with a character from the buffer to determine if it falls within the range of lowercase letters, returning a modified byte or a specific byte based on comparisons.
- `calculate_scaled_offset`: Adds a 16-bit value from offset 4 of a structure in a0 to d0, then calculates a new address by multiplying a 16-bit value from offset 6 of a pointer at offset 0 of a0 with a 16-bit value from offset 6 of a0, adding this product and a bit-shifted version of d0 to a0.
- `copy_compressed_data_blocks_2`: Copies bytes from a source buffer to a destination buffer using a nested loop structure that handles variable-length data segments.
- `dispatch_message_port`: Checks if a pointer in a register is non-null and if a value at offset 10 of that pointer is non-zero, then calls a routine from a table and sets bytes at offsets 8 and 14 to 255.
- `calculate_blitter_area_and_draw`: Calculates bitplane offsets and Blitter parameters based on a structure's dimensions and coordinates, then configures the Blitter hardware registers to perform a block transfer.
- `interpolate_parameter_tables`: Compares two 16-bit coordinate pairs against bounds provided in a table to determine their state, then performs linear interpolation or clamping on the values in the second table based on the relationship between the two pairs.
- `process_blitter_parameter_table`: Calculates differences between values in two tables and uses the results to perform arithmetic and bitwise operations, eventually populating a structure pointed to by a6 with several calculated 16-bit and 32-bit values and calling other routines.
- `call_function_table_loop`: The routine repeatedly calls a function pointer taken from a table in A1, decrementing a counter in D3 until it reaches -1, then returns the value originally on the stack.
- `call_function_table_loop_2`: The routine repeatedly calls a function taken from a table via A1, decrementing a counter in D3 until it reaches -1, then returns the value originally passed on the stack.
- `execute_blitter_loop`: Executes a loop that calls a routine via a pointer, performs arithmetic on values from a structure, and populates a structure at a6 with several calculated values before calling another routine multiple times.
- `mask_bytes_in_pattern`: The routine repeatedly masks four bytes spaced by a constant offset using a byte mask, writing the masked results back through a pointer.
- `apply_byte_mask_pattern`: The routine writes a byte from a register into a memory location pointed to by a pointer, OR-ing it with the existing byte, then repeatedly applies a byte mask from a register with AND to three successive locations spaced by a word offset, advancing the pointer by that offset each iteration.
- `mask_and_or_pattern`: The routine iterates a fixed number of times, using a pointer and a stride to access a block of memory.
- `apply_byte_mask_pattern_2`: The routine writes a byte from a register to a memory location pointed to by a pointer, ORs it with the existing byte, then repeats the OR with the same byte at an offset, then ANDs two more locations with a mask byte, advancing the pointer by a stride each time in a loop that runs a fixed number of times.
- `mask_bytes_in_pattern_2`: The routine repeatedly masks bytes at a base address and at offsets using a mask byte and an OR byte, writing the results back through a pointer.
- `apply_mask_pattern_to_bytes`: The routine writes to a byte pointed to by a pointer using a byte from a register and a byte from another register, repeating the operation with a fixed byte offset four times per iteration.
- `mask_and_fill_pattern`: The routine iterates a fixed number of times, using a byte mask from a register and a byte value from a register to AND and OR bytes in a memory area accessed through a pointer and a stride.
- `fill_masked_bytes`: A routine that iterates a fixed number of times, OR-ing a byte from a register into three consecutive memory locations pointed to by a pointer and a stride, then AND-ing a second byte from a register into a fourth location, advancing the pointer by the stride each iteration and decrementing a counter until it reaches -1.
- `mask_and_fill_pattern_2`: The routine repeatedly masks three consecutive bytes at an address derived from a base pointer and a stride using a mask byte, then ORs a fill byte into the next byte in the sequence.
- `apply_masks_to_buffer`: The routine writes to a memory buffer using a pointer and a stride.
- `apply_masked_pattern_to_buffer`: The routine iterates a fixed number of times, masking a byte at a pointer with a byte from a register and setting another byte at an offset with a byte from a register using bitwise OR, repeating the pattern with a stride.
- `apply_bitmask_to_memory`: Iterates through memory locations at fixed intervals, applying bitwise OR and AND operations with specific byte values to modify the data.
- `mask_and_fill_bytes`: The routine repeatedly masks two bytes at an offset from a base pointer using a byte from a register and then ORs the same byte into the next two bytes at the same offset, decrementing a counter until it reaches -1.
- `apply_byte_mask_pattern_3`: The routine writes a byte from a register into a memory location pointed to by a pointer, OR-ing it with the existing byte, then uses a second byte from a register to AND a byte at an offset, and repeats OR operations at further offsets using a decrementing counter until the counter reaches zero.
- `fill_masked_bytes_2`: The routine repeatedly ANDs a byte from a register with bytes in a memory area pointed to by a pointer, then ORs a byte from another register with three subsequent bytes spaced by a word offset, updating the pointer and a counter until the counter reaches -1.
- `xor_fill_pattern`: The routine repeatedly ORs a byte from a register into four consecutive bytes of a buffer, stepping through the buffer by an offset held in a register, and repeats the operation a number of times counted down in another register.
- `mask_words_in_table`: The routine masks four consecutive 16-bit words in a table with a value in a register, using an offset and a pointer derived from the table base.
- `apply_mask_to_table_entries`: The routine ORs a 16-bit value from a register into a global variable and then applies a bitwise AND mask from a register to three consecutive 16-bit entries in a table addressed via a pointer offset by a register value.
- `mask_and_or_word_pair`: The routine masks a word at a pointer with an input word, then ORs a second word at an offset derived from a short offset and the first pointer with another input word, and finally masks two further words at offsets derived from the updated pointer with the input word.
- `apply_bitmask_to_table_entries`: The routine ORs a 16-bit mask from a register into a word pointed to by a pointer, then uses an offset from a register to index a table of words and ORs the same mask into that entry.
- `mask_and_or_word_pair_2`: The routine masks two words pointed to by a pointer derived from an input word and a base pointer, using a bitwise AND with an input word, then ORs a word from a register into a third word.
- `apply_mask_to_word_table`: The routine reads a 16-bit value from a register and a 16-bit mask from a register, then ORs the mask into a word pointed to by a pointer in a register, and performs a series of indexed AND and OR operations on words addressed relative to a base offset in a register using the current pointer value.
- `mask_and_or_word_table`: The routine masks one word at a pointer with a value from D0 and then ORs a value from D4 into two words located by adding an offset from D2 to the pointer and to the result, and finally masks a further word reached by adding D2 again with the D0 value.
- `or_data_table_via_pointer_chain`: The routine uses a pointer held in a global variable as a base and an offset from a register to index a table of 16-bit words.
- `mask_and_or_word_table_2`: The routine masks two words in a table pointed to by a global pointer using a word from D0, with an offset from D2, then ORs a word from D4 into a further offset word in the same table.
- `mask_and_or_word_pair_3`: The routine reads a word from a global variable, ORs it with a word pointed to by another global pointer, then uses an offset from a register to index into that pointer to mask two words with an input word, and finally ORs a word from a global variable into a word located at an offset derived from the previous pointer.
- `mask_and_or_word_pair_4`: The routine masks a word at a pointer with an input word, then ORs a word at an offset computed from an index and the pointer with another input word.
- `other_routine`: This routine modifies memory indirectly via a series of pointer arithmetic operations using a global variable.
- `mask_and_or_word_pair_5`: The routine masks two words pointed to by a pointer and an offset pointer with a word from a register, then ORs a word from a register into two further words derived by adding an offset to the previous pointers.
- `mask_and_or_word_pair_6`: The routine reads a word from a global variable and ORs it with a pointer value, then uses the result as an offset to mask a word at a computed address with an input word, and finally ORs a mask word into two further words derived from the masked result.
- `mask_and_or_two_words`: The routine masks a word at a pointer with a value from a register and then ORs a word from a register into two words located at offsets derived from another register and the pointer, and then ORs the same register value into a third word at a further offset derived from the previous result.
- `set_bits_in_table`: The routine sets bits in a table by OR-ing a mask from a register with four words selected via pointer arithmetic using an offset from a register.
- `call_blitter_function`: Loads a pointer from a global variable into a6, saves registers to the stack, calls a routine at an offset from a6, restores registers, and sets a6 to the base address of the custom hardware registers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_disown_blitter`: Library stub: loads its arguments and the graphics library base, calls DisownBlitter and returns its result to the caller.
- `apply_bitmask_rotation`: Calculates a bitmask and a rotation amount based on values in a structure, then enters a loop that performs bitwise operations and bit rotations to update a destination buffer using data from a source buffer.
- `process_blitter_data`: Reads values from a structure, performs calculations involving bitwise operations and shifts, and writes multiple values to a structure, including several updates within loops that check a bit in a byte at offset 2.
- `update_blitter_parameters`: Calculates new coordinates and parameters based on values from a structure, then performs a loop that updates a structure at a6 with several values while checking a bit in a byte at offset 2.
- `act_as_data_table`: This routine consists of a sequence of null bytes that function as a data table.
- `call_object_handler`: Checks if a 16-bit value is non-zero, then uses it as an index into a table to fetch a pointer and calls another routine with that pointer.
- `update_object_list`: Iterates through a list of structures starting at a global address, incrementing a byte at offset 1 until it equals 1, then populates the structure with data from a provided pointer and a table based on a value at offset 2.
- `iterate_and_match_table_entries`: Runs a loop three times that checks if a 16-bit value at a pointer in a table matches a parameter and if the lower 4 bits of the following byte are non-zero; if both conditions are met, it clears that byte and calls another routine.
- `parse_object_data_stream`: Iterates through a data stream using a switch-based opcode parser to update various fields within a structure.
- `update_timer_and_counter_structures`: Decrements two separate counters and, if they reach zero, resets them using values from a structure and performs bounds checking and clamping on associated 16-bit and 8-bit values.
- `map_value_into_range`: The routine obtains a short value from another routine and maps it into the interval defined by two short fields of its parameter.
- `perform_32bit_addition`: Performs a 32-bit addition using two 16-bit values stored at a memory location and updates two global variables with the result.
- `set_flag_on_lowercase`: The routine checks a byte at an offset from a pointer in A1.
- `update_button_status_2`: Reads a 16-bit word from offset 12 of a structure in a0, performs bitwise transformations, checks bit 7 of the CIAA Port A register, and stores a byte and an ORed byte into two consecutive global memory locations.
- `clears_data_byte_2`: Performs a logical OR operation with zero on a byte in register d0.
- `perform_multiple_logical_ors`: Performs four consecutive logical OR operations on register d0 with zero.
- `clears_data_register_d0`: Performs four consecutive logical OR operations with zero on the data register d0.
- `clears_data_byte_3`: Performs four consecutive logical OR operations with zero on the data register zero.
- `zeroes_data_bytes`: Performs a logical OR operation with zero on the data register four times in succession.
- `zero_d0_register`: Performs four consecutive logical OR operations with zero on the d0 register.
- `clear_d0_register_3`: Performs four consecutive logical OR operations on the d0 register using a zero byte constant.
- `clear_d0_register_4`: Clears the d0 register four times using logical OR operations with zero.
- `test_bits`: Tests bit 0 of register d2 and bit 0 of register d4.
- `allocate_and_initialize_audio_structures`: Sets a bit in a CIA register and enters a loop that allocates multiple memory structures by calling library functions, populates them with data from a table, and links them together in a linked list.
- `iterate_and_call_handlers`: Runs a loop four times, checking if a pointer at an offset from a base address is non-zero to call various routines.
- `set_structure_fields`: Uses a parameter to index into a table, then writes the constant eleven to offset 28 and zero to offset 30 of the resulting structure, before calling a routine via a pointer at offset 20.
- `unknown_routine_0_0_c18`: It calls another routine to initialize enemy structures, then calls another routine to initialize an object chain.
- `set_object_fields_and_trigger_callback`: The routine indexes into a table using an offset from the first argument, loads a pointer from the table, writes a constant word and a zero byte to fixed offsets in the pointed-to structure, copies a word from the first argument and a word from the second argument to other fixed offsets in the same structure, then calls a function pointer stored in the structure with a fixed negative offset applied.
- `allocate_and_initialize_structure_2`: Calls several functions to allocate memory and signals, then populates a structure with data from a global variable and other sources.
- `destroy_message_port`: Checks if a pointer is non-null and if an offset of 10 contains a non-zero value, then calls a routine via a table, sets bytes at offsets 8 and 32 to 255, and calls two subsequent routines to free memory and signals.

### Amazing Spider-Man, The (1990)(Empire Software)[cr CLS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
