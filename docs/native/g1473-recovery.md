# Game 1473 (boston bomb club (1990)(silmarils)[cr company][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01473). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 258 of 258 named (0 author symbols), 258 of 258 described (14 by rule, 243 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 858 of 858 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 115628 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5941 of 6852 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 369637 |
| code | 27794 |
| data | 213627 |
| padding | 8542 |
| unused | 281600 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:Type | 2 | 117 | 2 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:t.x | 4 | 6704 | 254 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:PRESENT.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:BOBOI.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:BOUM.CO (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op FFFF) A7=an address (A7)=an address, slid to exec.library -an address): 16656 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- boston bomb club (1990)(silmarils)[cr company][a]/Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:PRESENT.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][a]/Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:BOBOI.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][a]/Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:BOUM.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][a]/Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.85)
- 3 never-loaded region(s), 115628 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address using a loop.

### Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:t.x

- `iterate_linked_list_elements`: This routine iterates through a linked list of structures, performing conditional updates to integer and byte fields within each element.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `allocate_memory_and_init_state`: Opens libraries and allocates memory, and on success derives several global pointers and counters from a structure and returns a value.
- `open_keyboard_device`: Uses library calls to find a task and open the keyboard device, then stores several values into a global structure.
- `call_four_routines_in_sequence`: Calls four unnamed routines one after another and then returns.
- `store_mouse_data`: Reads a value from a register and stores it into a global variable.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `allocate_and_initialize_buffers`: Sets up several global pointers based on a base address, allocates memory via a library call, initializes a structure, calls several functions through pointers, and sets up further global pointers and constants.
- `copy_and_set_pointers`: Copies a block of memory and updates several global pointers or values, potentially disabling certain hardware DMA or interrupts.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_value_in_table`: Searches a table of shorts for a specific value and returns the number of steps taken or a negative error code.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `apply_scaled_transform`: Calculates a value using a multiplication and addition, then stores the result as a dword into a table, while also modifying a hardware interrupt enable register.
- `update_pointer_and_string_tables`: Iterates through a table to update pointers with an offset-based calculation and writes a string of characters to a memory buffer.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `call_indirect_function`: Calls a function pointer stored in a memory location and returns the result.
- `wait_for_bit`: Performs a busy-wait loop until a specific bit in a global variable is set.
- `set_interrupt_enable_bit`: Writes a value to the interrupt enable register.
- `update_pointer_table`: Iterates through a list of pointers and updates the target or destination using a linear transformation or offset.
- `write_instruction_sequence`: Writes a short sequence of instructions to a memory location and then executes a series of function calls, including a call to an external function via a pointer retrieved from a global table.
- `execute_sequence`: Executes a sequence of routine calls and updates a global variable with a constant value before calling a function via a pointer.
- `reset_and_recurse_pointer`: Sets a global variable to zero and calls a routine pointer retrieved from a table offset by that global variable.
- `set_system_state`: Calls a function through a pointer, updates a global variable, and calls another function with an argument.
- `increment_counter_and_call_four`: Increments a 32-bit global counter, then calls one routine four times in a countdown loop from three down to zero, followed by three further routine calls.
- `modify_global_value`: Checks a global variable and, if zero, performs conditional logic to update a global variable or modify a memory location based on a bit-shifted or offset value.
- `update_hardware_parameters`: Checks a global variable and, if zero, updates several hardware registers or memory locations with new values based on a lookup or calculation.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `return_immediately_6`: The routine performs no operations and returns immediately to the caller.
- `return_immediately_3`: This routine performs no operations and returns to its caller immediately.
- `return_empty_routine`: An empty routine that performs no operations and returns immediately.
- `process_string_decryption`: Iterates through a list of data structures to identify specific tagged entries, then performs a multi-pass decryption or transformation on a buffer using a lookup table and a running sum or offset.
- `expand_compressed_assets`: Iterates through a linked list of compressed data structures to decompress or expand certain byte sequences into a destination buffer.
- `return_constant`: Returns the constant 3000 in d0 to the caller without any other effect.
- `return_constant_2`: Returns the constant 100 in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant 0 in d0 to the caller without any other effect.
- `replicate_global_byte`: Returns a value formed by duplicating a byte from a global variable.
- `return_immediately_4`: This routine performs no operations and returns immediately to the caller.
- `copy_memory_block_2`: Copies a block of data from a source pointer to a destination pointer, checking for a null terminator.
- `call_indirect_function_2`: Calls another routine and returns the concatenated contents of two registers.
- `call_draw_and_move`: Calls another routine, then calls the graphics Draw and Move library functions through library pointers stored in a global variable.
- `call_indirect_through_table`: Calls another routine, then calls a routine loaded through a pointer read from a table at a fixed offset, and returns the values held in the data registers.
- `empty_routine`: The routine contains no executable instructions and performs no operations.
- `store_byte_and_call_indirect`: Stores a byte from a register into a global variable and calls another routine via a pointer stored in a table.
- `store_register_and_call_function`: Stores the contents of a data register into a global variable and then calls a function pointer retrieved from a table or offset.
- `update_global_with_conditional`: Checks if a register contains a specific value and, if so, assigns a different value to a global variable before calling another routine.
- `iterate_until_value`: Iterates through a routine or table, checking return or comparison values until a specific condition or non-zero value is met.
- `update_global_buffer`: Updates a global variable or a buffer-like structure with new values based on certain character checks and then calls a function pointer.
- `traverse_string_null`: Traverses a null-terminated string or buffer, calling another routine for every non-null character encountered.
- `calculate_scaled_offsets`: Clamps register values and stores the results multiplied by a constant into two global variables.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `update_position_limits`: Updates two global variables by adding or subtracting a value, then clamps them within a certain range, and finally clears certain bits in a third global variable.
- `scale_input_values`: Multiplies two input registers by two and stores the results in two global variables.
- `return_halved_global`: Returns the value of a global variable shifted right by one.
- `compare_and_reset_table`: Iterates through a table to compare values, and if a certain condition is met, updates a global variable or resets a portion of a table to zero.
- `set_status_flags`: Calls a function pointer and then modifies a global variable or sets specific bits within it based on the status of certain flags.
- `update_joystick_status_bits`: Modifies a value based on the state of specific bits within a global variable.
- `construct_input_bitmask`: Constructs a bitmask by evaluating specific bits within a global variable and applying conditional bitwise OR operations.
- `process_joystick_input`: Reads the joystick port and modifies a value based on the state of several bits in a global variable.
- `perform_no_operation`: The routine contains instructions but performs no functional logic or state changes before returning.
- `execute_null_instructions`: executes a sequence of seven instructions that perform no functional logic or state changes before returning.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `call_library_and_map_zero`: Calls a library function through a stored base pointer and replaces a zero result with minus one, returning that value.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `lock_and_unlock_conditionally`: Calls a function through a pointer from a global variable, and if it returns nonzero, calls a second function through another pointer from the same variable with an argument register.
- `open_and_close_file`: Calls a function through a pointer from a global variable, and if a global flag byte is nonzero, calls two further functions through pointers from the same variable and then loops indefinitely.
- `execute_library_function_with_error_check`: Calls a function via a pointer stored in a global variable and returns negative one if the result is zero.
- `open_file_or_fail`: Calls a routine and then invokes a library function through a pointer from a table; if the result is zero it returns minus one, otherwise it returns the result.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_external_function`: Calls a function pointer located at an offset from a value stored in a global variable.
- `write_then_set_flag`: Calls a routine, invokes a library function through a pointer from a table, and then sets a global variable to one.
- `wait_for_disk_removal`: Repeatedly checks conditions and traverses a string until a specific character is encountered.
- `copy_and_clear_planes`: Copies words from five source planes into an interleaved destination buffer and clears the source words, over a fixed grid.
- `copy_from_interleaved`: Copies words from an interleaved source buffer into five destination planes, over a fixed grid.
- `return_empty_subroutine`: This routine performs no operations and returns immediately upon entry.
- `fetch_exec_message`: Calls a library function to retrieve a message and checks specific bits within the message flags.
- `return_uninitialized_stack_value`: The routine returns an undefined value from the stack.
- `execute_recursive_calls_2`: Calls a library function through a pointer, and if the result is zero, performs two consecutive calls to itself before returning.
- `execute_recursive_calls`: This routine performs two consecutive calls to itself before returning.
- `return_zero_2`: This routine is empty and simply returns a zero value to the caller.
- `wait_for_flag_scale`: Repeatedly calls a routine until a global variable is no longer minus one, then computes the difference of two globals, multiplies it by a global word, and divides by one thousand.
- `query_dos_info`: Queries file information from a device and returns a combined result containing a status and a value.
- `update_global_table`: compares two global values and, if they differ, updates a table of five values and swaps two other global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_state_parameters`: Checks a global variable against a stored value and, if they differ, resets a sequence of parameters in a data block before calling another routine multiple times.
- `update_state_parameters_2`: Compares a global variable against a stored value and, if they differ, updates a sequence of five data words with new constants before calling another routine and swapping two global variables.
- `update_state_parameters_3`: Compares a global value against a stored threshold and, if they differ, updates a series of consecutive data words with new constant values before calling another routine multiple times.
- `update_state_parameters_4`: Checks a global variable against a stored value and, if they differ, updates a sequence of five data words with new constants before calling another routine multiple times.
- `swap_parameter_tables`: Swaps sets of values within a table and initializes new parameters if a specific global value changes.
- `swap_and_initialize_params`: Swaps several values in a table or global group and initializes a new set of parameters if a certain condition is met.
- `copy_block_and_toggle_globals`: Copies five longwords from one global buffer to another, executes a loop of other routines, swaps three global pointers, and temporarily sets five consecutive global words to fixed values before restoring them.
- `update_table_values`: Calculates a value from a global variable and adds it to several consecutive entries in a table.
- `initialize_data_structure`: checks a pointer, and if non-zero, calculates several values based on offsets and stores them into a series of global variables and a new data structure.
- `initialize_object_structure`: Calculates various offsets and sizes based on a pointer and stores them into a series of global variables.
- `initialize_object_structure_2`: Calculates various offsets and sizes based on a pointer and stores them into a series of global variables.
- `allocate_block_and_write_header`: Computes a row size from a structure field, derives several sequential offsets and sizes from global values, stores the input pointer in a global, writes a fixed word and two differences into the block header, and stores a size value.
- `insert_into_stack`: calculates a value based on an input, and if a specific global value is greater than a calculated address, stores that value into a table at that address.
- `allocate_memory_block`: Calculates a size based on a register, checks if a global pointer is within bounds, and if so, updates the global pointer and stores the size at the new location.
- `update_stack_pointer`: Calculates a new offset based on a register value and checks it against a global limit.
- `insert_stack_entry`: Calculates a size-aligned offset and attempts to insert a value into a descending memory buffer.
- `update_stack_pointer_2`: Calculates a new offset based on a register value and checks it against a global boundary.
- `pop_stack_pointer`: Calculates a pointer based on a global value and an offset, then updates a global pointer and stores the calculated offset if it is within bounds.
- `return_stack_value`: This routine is empty and simply returns the value currently residing at the top of the stack.
- `free_block_if_top`: If the global allocation pointer equals the block address passed in a register minus a header size, advances the pointer by the size word stored at the block start; returns the passed value.
- `return_zero_value`: This routine performs no operations and returns a zero value to the caller.
- `return_constant_4`: Returns the constant -1 in d0 to the caller without any other effect.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `process_linked_list_elements`: Iterates through a list of structures to update various fields, perform conditional logic, and modify data based on specific byte values.
- `execute_indirect_call`: Performs an indirect call to a destination calculated by adding an offset from a table to a base address.
- `compute_scaled_vector_offset`: Calculates a new position and direction by applying a bit-shifted scale factor to a vector, then updates global destination coordinates.
- `extract_byte_from_structure`: Checks a byte in a structure and returns a value composed of a high word and a byte from a specific offset within that structure.
- `clear_flag_and_traverse_list`: Loops while a 16-bit value read through a pointer is non-zero, calling another routine on each pass that may unlink list nodes.
- `traverse_and_update_linked_list`: Recursively traverses a linked list, updating or replacing node pointers based on a condition checked against a byte in the node.
- `update_linked_list_node`: Updates a node in a linked list or replaces a node with another if certain conditions are met.
- `store_word_pair`: Stores two consecutive words from a memory location pointed to by a register into two global variables.
- `apply_coordinate_offset`: Calculates a difference between two points and adds that difference to a list of coordinate values.
- `offset_list_coordinates`: Adds values from a source or offset table to an existing list and updates a flag in a list.
- `process_object_list_and_copy_data`: Waits for a hardware bit, enables interrupts, updates object bounding boxes and dimensions via a linked list, and copies a sequence of words from a table into global variables until a null pointer is reached.
- `update_sprite_list_and_bounds`: Extracts rectangular dimensions and scaling factors from a data structure to update global bounding boxes and coordinate tables.
- `write_to_table_offset`: Writes several values, including a global variable, to a table or memory area using an offset.
- `relink_node_and_check_bound`: Copies a next-pointer field from one node to another, pushes the node onto a global free-list, and sets a global flag when a table value is at least a global threshold.
- `update_table_and_bounds`: Checks a condition involving a global variable and a memory value to potentially set a flag, then writes several values from registers and globals into a table at a specific offset.
- `initialize_global_bounds`: Sets two global variables to a high value and two other global variables to a low value.
- `expand_global_bounds`: Updates certain global variables with new values if the new values exceed or fall below current bounds.
- `insert_sorted_list_node`: Inserts a node into a linked list ordered by priority fields, comparing a short and two bytes against existing nodes, with a special case for a global flag and a specific node index.
- `process_tiled_collision_bounds`: Iterates through a linked list of data structures to check boundary conditions and update global clipping or visibility rectangles.
- `set_flag_from_bounds`: Compares two global bound values against two other global bounds and stores either one or all-ones into a global variable depending on the comparisons.
- `initialize_buffer_constants`: Copies a sequence of four word-size values from one memory location to another.
- `copy_table_block`: Iterates through a table of 16-bit values, copying them into a set of global variables.
- `clamp_rect_bounds`: Checks if a range or area defined by a pointer-offset table exceeds certain global bounds and clamps those values to a minimum threshold if they are too small.
- `expand_rect_bounds`: Checks if a rectangular area defined by a table and certain global values is within a valid range, then expands the global bounds to include that area.
- `update_clipping_region`: Updates a global clipping or visibility rectangle by comparing a new area against existing bounds, including bitwise masking and calculating an offset or size.
- `execute_null_subroutine`: An empty routine that performs no operations and returns immediately.
- `reset_global_variable`: Sets a global variable to zero through a series of assignments.
- `check_nibble_sum`: Compares the sum of two lower nibbles against a constant value and returns if the sum exceeds that value.
- `check_nibble_difference`: Calculates the difference between two lower nibbles and returns if the result is negative or outside a certain range.
- `reconstruct_nibble_data`: Iterates through a buffer to detect or modify nibble-based data encodings.
- `call_nibble_sum_check`: This routine calls a function that compares the sum of two lower nibbles against a constant value.
- `compare_nibble_values`: Invokes a routine to determine if the difference between two lower nibbles falls within a specific range or is negative.
- `call_sequence_of_three_routines`: Calls three other routines in sequence, passing the result of the first into the second, then calls a third routine and returns.
- `initialize_data_structures`: Initializes a series of global variables and a table with specific values and a loop that populates a table with calculated pointers.
- `process_rle_and_sorted_list`: Copies a string while replacing specific characters, performs a binary search through a sorted table, and calculates several offsets and pointers for a data structure.
- `decode_rle_stream`: Reads a control byte from a global stream; if its high bit is set, it repeats the next byte a number of times determined by the low bits, otherwise it copies that many following byte pairs.
- `fetch_global_pointer_value`: Compares a value to a global variable and returns a value from a memory location pointed to by a global variable.
- `write_byte_with_bounds_check`: Writes a byte to a destination if a calculated address range check passes.
- `shift_memory_block`: Moves bytes within a memory range using a loop that calculates offsets and handles variable step sizes.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `calculate_signed_difference`: Computes the difference between two values and returns the result or a signed version of it.
- `invoke_and_return_pair`: Calls another routine and returns a concatenated 64-bit value formed from two 32-bit registers.
- `search_sorted_table`: Performs a binary search through a table of word-sized values to find a match for a given value.
- `initialize_list_structure`: Iterates through a memory range, writing incrementing offsets and zero-values to a list or buffer.
- `insert_into_sorted_list`: This routine inserts a pointer into a sorted linked list by traversing the list to find the correct position and shifting subsequent elements to maintain order.
- `initialize_and_manage_linked_list_nodes`: Initializes a new data structure by zeroing out a block of memory and setting up several pointer and value fields.
- `execute_conditional_logic_loop`: Performs a complex series of conditional calls to various function pointers, including a loop that retries a library call until a non-negative result is achieved.
- `execute_conditional_function`: Calls a function pointer and subsequently calls a second function pointer if the first call returns a non-zero value.
- `loop_until_positive_result`: Repeatedly calls another routine in a loop until a non-negative value is returned.
- `loop_until_success`: Repeatedly calls a routine and checks its return value, continuing to call another routine if the result is negative until a non-negative value is returned.
- `retry_library_call`: Repeatedly calls a function via a pointer stored in a global variable until it returns a non-negative value.
- `process_external_function_loop`: Calls a function pointer located at an offset from a value stored in a global variable within a loop that continues as long as a negative value is returned.
- `update_channel_countdown_state`: Sets a global byte to one, increments it with wraparound, and then decrements and conditionally clears a global countdown, calling other routines on specific transitions.
- `update_global_and_copy`: Updates a global variable, checks boundary conditions, and copies a sequence of memory blocks from a source to a destination.
- `decrement_counter_and_clear_fields`: Decrements a word counter in a structure addressed by an address register and, while it is nonzero, adds two word fields with clamping and conditionally updates a third field before calling another routine.
- `execute_empty_routines`: Calls several routines that contain no executable instructions.
- `clear_structure_fields`: Through an address register, writes zero to three byte offsets and zero to two word offsets of the pointed-to block.
- `update_clamped_channel_state`: Manages three global counters to modify a global value through addition or subtraction, clamps the result to a maximum, and triggers an interrupt update routine when certain conditions are met.
- `return_to_caller`: Immediately returns to the calling routine without performing any operation on data.
- `return_empty_pair`: This routine is empty and performs no operations.
- `update_global_offsets_on_nonzero`: Calls another routine and, if the returned 16-bit value is non-zero, adds fixed constants to two global 32-bit values before returning the original register pair.
- `return_concatenated_registers`: Calls another routine and returns a 64-bit value composed of two 32-bit registers.
- `update_bitmask_and_shift_data`: Checks certain boundary conditions and, if met, clears specific bits in a global variable and then performs a bitwise-masked bit-shift.
- `blit_masked_sprite_region`: Clips a rectangular region against a global bounding box, then copies masked source words from a table into five destination planes while saving the original words, using shifted bit masks derived from the source data.
- `blit_5plane_tile_column`: Copies a 16-row block of five bitplanes from a source table into five destination pointers, advancing each destination by a stride derived from a global and optionally shifting the source by one word when a global flag is set.
- `tiny_ori_2`: Small routine of 1 instruction that merges 117 into d0.
- `blit_single_row_region`: Sets up the blitter with fixed masks and modulo values, waits for the blitter to finish, then programs the source and destination pointers from computed row and column offsets and issues one blit of a single row region.
- `blit_loop_over_rows`: Configures the blitter once, then loops over a computed number of rows, each iteration programming source and destination pointers advanced by a row stride and issuing a blit, waiting for completion each time.
- `copy_tiled_block_rows`: Copies a block of data across multiple source planes into interleaved destination rows by performing word-by-word transfers.
- `configure_blitter_transfer`: Calculates transfer parameters and updates various blitter hardware registers and collision bounds.
- `update_blitter_parameters`: Checks specific bits in a byte from a pointer, then calculates and stores a value into a global variable based on several input registers and a conditional check.
- `blit_clipped_sprite_with_masking`: Clips a sprite rectangle against global bounds, converts source pixel rows into masked planar bitplane data through a lookup table, and programs the blitter to draw the clipped region, enabling DMA channels.
- `update_collision_bounds`: Calculates and updates intersection boundaries between a moving object and a defined rectangular area.
- `clip_and_prepare_sprite_blit`: Calculates the intersection between a sprite rectangle and the screen boundaries to determine valid drawing coordinates.
- `store_byte_via_pointer`: Stores a byte through a pointer at a fixed offset and then calls another routine.
- `store_byte_through_pointer`: Stores a byte from a register through a pointer at a fixed offset, then calls another routine and returns the stack value.
- `configure_audio_channel`: Enters supervisor state through a library call, then computes a channel bit mask and a clamped volume value and writes volume, period and a sample pointer to the audio hardware registers.
- `clear_channel_flag_if_negative`: Tests a per-channel flag byte and, if its sign bit is set, clears that flag byte, clears a byte through a pointer, and computes a channel bit mask shifted left by seven into a global variable.
- `decrement_global_timer`: Checks a bitmask within a global variable and recursively calls itself if certain bits are set, then writes a constant value to a global variable and updates hardware interrupt registers via a called routine.
- `decrement_channel_counter`: Decrements a per-channel word counter through a pointer and, when it reaches zero, clears a byte through a pointer, clears the per-channel flag byte, and computes a channel bit mask shifted left by seven into a global variable.
- `execute_sequential_routines`: Calls another routine four times in succession.
- `save_registers_then_call`: Calls a neighboring routine and returns the concatenated values of two registers.
- `conditionally_call_interrupt_update`: If a global variable is non-zero, it calls another routine that updates interrupt state from a word in a data register, then returns the value held in the data register.
- `forward_clamped_value_to_globals`: Passes a signed word from a data register to another routine, which clamps it to a maximum of 100 and stores it in several global variables and a hardware volume-related register.
- `compute_scaled_seven_bit_value`: Masks a value in a register to seven bits, multiplies it by eight, subtracts seven, and stores the result in a global variable.
- `pass_through_register`: Calls another routine and returns the value held in a register.
- `clamp_and_store_volume`: Clamps a value in a register to at most one hundred and, when it is non-negative, stores it to a global word and three audio volume registers; otherwise stores it to one global word.
- `update_callback_structure`: Stores several values into a structure, updates a global variable with a shifted value, and conditionally sets routine pointers and a global constant based on a register value.
- `write_short_values`: Stores two short values into a structure through an address register at specific offsets.
- `setup_audio_interrupt_state`: Saves several register values into global variables, calls a routine through a function pointer in a global table, and configures an interrupt handler and control bits.
- `toggle_audio_and_interrupt_state`: Depending on a global byte and the sign of a register value, it saves or clears state bytes, masks interrupt and DMA enable registers, and writes control values to a hardware register.
- `conditionally_set_control_bits`: If a global byte is non-zero, it stores a routine pointer into a global variable and writes two constant words to other globals, then conditionally sets a bit in a structure.
- `dispatch_conditional_calls`: Tests specific bits within a global variable and, based on which bits are set, calls one of several different routine pointers.
- `dispatch_conditional_function_calls_2`: Tests specific bits within a register to determine whether to call one of several function pointers stored in global variables.
- `dispatch_conditional_function_calls`: Tests specific bits within a register to determine whether to call one of two different function pointers stored in global variables.
- `dispatch_conditional_call`: Checks a bit in a register and, if set, calls a function pointer stored in a global variable.
- `dispatch_conditional_subroutine`: Reads two words from a memory location pointed to by a register and performs a bitwise mask.
- `set_handler_pointers`: Stores several values and pointers into a table.
- `update_function_pointers`: Updates a series of function pointers within a data structure and resets specific global variables.
- `configure_callback_table`: Sets a global variable to a specific value and updates a table with a function pointer and a secondary routine address.
- `update_function_pointers_2`: Updates a global variable and modifies a table by storing the address of the current routine and another routine into specific offsets.
- `update_callback_table`: Updates a global variable and writes a function pointer and a secondary routine address into a table structure using offsets from a register.
- `update_function_pointers_3`: Updates a global variable with a constant value and modifies several entries in a table or structure by storing the addresses of two different routines.
- `register_callback_pointers`: Updates a global variable and writes the address of the current routine and another routine into specific offsets within a structure or table pointed to by a register.
- `initialize_timer_and_pointers`: Writes constants to global variables, stores a routine pointer and a value into a structure through an address register, and clears two global variables and a pointer.
- `update_sprite_attributes`: Parses a data stream to update sprite hardware registers and associated metadata tables.
- `parse_audio_command_stream_2`: Iterates through a data stream to parse command bytes that update Amiga audio hardware registers.
- `parse_audio_command_stream`: Iterates through a data stream to parse command bytes that update Amiga audio hardware registers.
- `update_audio_hardware_registers`: This routine parses a data stream to update Amiga audio hardware registers, specifically controlling the frequency and volume parameters of various sound channels.
- `parse_audio_command_stream_3`: Iterates through a data stream to parse command bytes that update Amiga audio hardware registers.
- `update_function_pointer_table`: Increments a global variable and stores a calculated address into a specific offset within a table pointed to by another global variable.
- `parse_audio_command_stream_4`: Iterates through a data stream to parse command bytes that update Amiga audio hardware registers.
- `update_audio_hardware_registers_2`: Parses a command stream to update Amiga audio hardware registers, including volume and period settings.
- `parse_audio_command_stream_5`: Iterates through a data stream to parse command bytes that update Amiga audio hardware registers.
- `update_audio_hardware_registers_3`: Parses a compressed data stream to update Amiga audio hardware registers, including volume, period, and frequency settings.
- `parse_compressed_data_stream`: Iterates through a data stream to decode compressed or indexed values into a destination buffer.
- `parse_audio_command_stream_6`: Iterates through a data stream to parse command bytes that update Amiga audio hardware registers.
- `set_bit0_pointer_and_width`: If bit 0 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 164, stores the pointer plus a field at offset 160, and sets a word at offset 154 to 128.
- `set_bit0_handler_and_mask`: If bit 0 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode1_and_masks`: Stores the value 1 at offset 150 and the value 128 at offsets 154 and 156 of the object.
- `set_bit1_pointer_and_width`: If bit 1 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 180, stores the pointer plus a field at offset 176, and sets a word at offset 154 to 256.
- `set_bit1_handler_and_mask`: If bit 1 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode2_and_masks`: Stores the value 2 at offset 150 and the value 256 at offsets 154 and 156 of the object.
- `set_bit2_pointer_and_width`: If bit 2 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 196, stores the pointer plus a field at offset 192, and sets a word at offset 154 to 512.
- `set_bit2_handler_and_mask`: If bit 2 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode4_and_masks`: Stores the value 4 at offset 150 and the value 512 at offsets 154 and 156 of the object.
- `decrement_counter_set_bit3_mode`: If bit 3 of a byte at offset 3 of the object is set, decrements a global counter and, when it becomes negative, stores the value 8 at offset 150 and 1024 at offset 154.
- `return_immediately_5`: This routine performs no operations and returns immediately to its caller.
- `empty_routine_2`: This routine is empty and contains no executable instructions.
- `empty_routine_3`: This routine contains no executable instructions.
- `execute_single_instruction`: Executes a single instruction at a specific location.
- `execute_single_instruction_2`: Performs a single instruction that involves clearing, moving, or subtracting values.
- `execute_single_instruction_3`: Performs a single instruction that involves clearing or logical OR operations.
- `execute_single_instruction_4`: Performs a single instruction involving move or move address operations.
- `execute_single_instruction_5`: Performs a single instruction that moves data.
- `execute_short_sequence`: Executes a short sequence of seventeen instructions.
- `execute_two_instructions`: Executes two instructions involving address loading and data movement.
- `empty_routine_4`: The routine is empty and contains no executable instructions.
- `empty_routine_5`: The routine is empty and performs no operations.
- `execute_medium_sequence`: Executes a sequence of one hundred and sixty instructions involving address and data movement.
- `empty_routine_6`: This routine contains no valid instructions and performs no operations.
- `empty_instruction_sequence`: The routine contains no valid instructions and performs no operations.
- `empty_routine_7`: The routine contains no executable instructions.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `empty_routine_8`: This routine contains no executable instructions and performs no operations.
- `copy_memory_block_3`: Copies a block of data from one memory location to another using a loop.
- `empty_routine_9`: The routine contains no valid executable instructions.

### Boston Bomb Club (1990)(Silmarils)[cr Company][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `add_offsets_on_nonzero_result`: Calls another routine that returns a 16-bit value and, when that value is non-zero, adds fixed constants to two global 32-bit values.
