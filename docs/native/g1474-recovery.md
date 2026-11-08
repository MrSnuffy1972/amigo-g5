# Game 1474 (boston bomb club (1990)(silmarils)[cr company][m jive]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01474). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921360 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 40 raw-word lines |
| Functions named and described | pass, 412 of 412 named (0 author symbols), 412 of 412 described (23 by rule, 382 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2005 of 2005 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 115628 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8668 of 9579 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 377385 |
| code | 39194 |
| data | 263253 |
| padding | 9080 |
| unused | 232448 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:tsb | 1 | 26 | 1 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:Type | 2 | 117 | 2 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:t.x | 4 | 7104 | 296 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:bootblock | 1 | 193 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2083 | 100 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 44 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:PRESENT.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:BOBOI.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:BOUM.CO (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 673 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- boston bomb club (1990)(silmarils)[cr company][m jive]/Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:PRESENT.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][m jive]/Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:BOBOI.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][m jive]/Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:BOUM.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- boston bomb club (1990)(silmarils)[cr company][m jive]/Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.85)
- 3 never-loaded region(s), 115628 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:tsb

- `copy_memory_blocks`: Copies blocks of data from one memory location to another using a loop that handles variable byte lengths.

### Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_buffer`: Copies sixteen words from a memory location offset by a pointer into a destination buffer and then calls another routine.

### Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:t.x

- `update_object_list_state`: Iterates through a linked list of objects to update their internal counters and status flags.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `allocate_memory_and_init_state`: Sets a global variable to one, opens a library and allocates memory through library calls, and on failure displays an out-of-memory text and returns zero.
- `initialize_keyboard_device`: Opens the keyboard device and populates a global structure with a task pointer and specific configuration values.
- `call_four_routines_in_sequence`: Calls four unnamed routines one after another and then returns.
- `store_mouse_position`: Stores the value of a global variable into a memory location.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `allocate_and_initialize_buffers`: Sets up several global pointers based on a base address, allocates memory via a library call, initializes a structure, calls several functions through pointers, and sets up further global pointers and constants.
- `copy_and_set_pointers`: Copies a block of memory and updates several global pointers or values, potentially disabling certain hardware DMA or interrupts.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_value_in_table`: Searches a table of shorts for a specific value and returns the number of steps taken or a negative error code.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `apply_scaled_transform`: Calculates a value using a multiplication and addition, then stores the result as a dword into a table, while also modifying a hardware interrupt enable register.
- `update_pointer_and_string_tables`: Iterates through a table to update pointers with an offset-based calculation and writes a string of characters to a memory buffer.
- `return_immediately_3`: This routine is empty and performs no operations before returning.
- `call_indirect_function`: Calls a function pointer stored in a memory location and returns the result.
- `wait_for_bit`: Polls a global variable until a specific bit is set.
- `set_interrupt_enable_bit`: Writes a value to a global variable and modifies the interrupt enable bit in the INTENA register.
- `update_pointer_table`: Iterates through a list of pointers and updates the target or destination using a linear transformation or offset.
- `write_instruction_sequence`: Writes a short sequence of instructions to a memory location and then executes a series of function calls, including a call to an external function via a pointer retrieved from a global table.
- `execute_sequence`: Executes a sequence of routine calls and updates a global variable with a constant value before calling a function via a pointer.
- `reset_and_recurse_pointer`: Sets a global variable to zero and calls a routine pointer retrieved from a table offset by that global variable.
- `set_system_state`: Calls a function through a pointer, updates a global variable, and calls another function with an argument.
- `increment_counter_and_call_four_2`: Increments a 32-bit global counter, then calls one routine four times in a countdown loop from three down to zero, followed by three further routine calls.
- `decrement_channel_countdown`: Decrements a byte in a table indexed by a register and updates a global variable when the byte reaches zero.
- `update_channel_countdown_flags`: Indexes a per-channel record by a register value, and when a countdown byte in that record reaches zero it sets bits in a global variable and conditionally writes a value derived from the channel bit.
- `update_entity_state_2`: Compares a value against a table and updates entries with new data, including hardware register values and calculated offsets.
- `update_entity_state`: Compares a value against a table and updates entries with new data, including hardware register values and calculated offsets.
- `update_entity_state_table`: Compares a byte value against a table and updates entries with new data, including a calculated value, a short integer, and specific hardware register values.
- `update_indexed_data_structure`: Compares a byte from a table with a register and updates several values in a global structure and hardware registers based on the result.
- `write_paired_registers_indexed`: Writes two 16-bit values from registers to a location offset by a register value, one into a global-variable-based address and one into a second address with the same offset.
- `return_no_op_handler`: The routine consists of a single return instruction and performs no work, so it does nothing when entered.
- `return_immediately_4`: This routine performs no operations and returns immediately to its caller.
- `return_immediately_5`: This routine performs no operations and returns immediately to the caller.
- `execute_null_callback`: An empty routine that performs no operations.
- `process_string_decryption`: Iterates through a list of data structures to identify specific tagged entries, then performs a multi-pass decryption or transformation on a buffer using a lookup table and a running sum or offset.
- `expand_compressed_assets`: Iterates through a linked list of compressed data structures to decompress or expand certain byte sequences into a destination buffer.
- `return_constant`: Returns the constant 3000 in d0 to the caller without any other effect.
- `return_constant_2`: Returns the constant 100 in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant 0 in d0 to the caller without any other effect.
- `replicate_global_byte`: Returns a value formed by duplicating a byte from a global variable.
- `return_from_subroutine_3`: The routine is empty and performs no operations before returning to the caller.
- `copy_memory_block`: Copies a block of data from a source pointer to a destination pointer, checking for a null terminator.
- `call_indirect_function_2`: Calls another routine and then executes a single indirect function call via a pointer before returning the concatenated contents of two registers.
- `call_draw_and_move`: Calls another routine, then calls the graphics Draw and Move library functions through library pointers stored in a global variable.
- `call_indirect_through_table`: Calls another routine, then calls a routine loaded through a pointer read from a table at a fixed offset, and returns the values held in the data registers.
- `return_from_subroutine`: The routine performs no operations and immediately returns to the caller.
- `store_byte_and_call_indirect`: Stores a byte from a register into a global variable and calls another routine via a pointer retrieved from a table.
- `store_register_and_call_function`: Stores the contents of a data register into a global variable and then calls a function pointer retrieved from a table or offset.
- `update_global_with_conditional`: Checks if a register contains a specific value and, if so, assigns a different value to a global variable before calling another routine.
- `iterate_until_value`: Iterates through a routine or table, checking return or comparison values until a specific condition or non-zero value is met.
- `update_global_buffer`: Updates a global variable or a buffer-like structure with new values based on certain character checks and then calls a function pointer.
- `traverse_string_null`: Traverses a null-terminated string or buffer, calling another routine for every non-null character encountered.
- `calculate_scaled_offsets`: Clamps register values and stores the results multiplied by a constant into two global variables.
- `nop_padding`: This routine is empty and contains no executable instructions.
- `update_position_limits`: Updates two global variables by adding or subtracting a value, then clamps them within a certain range, and finally clears certain bits in a third global variable.
- `scale_input_values`: Multiplies two input registers by two and stores the results in two global variables.
- `return_halved_global`: Returns the value of a global variable shifted right by one.
- `compare_and_reset_table`: Iterates through a table to compare values, and if a certain condition is met, updates a global variable or resets a portion of a table to zero.
- `set_status_flags`: Calls a function pointer and then modifies a global variable or sets specific bits within it based on the status of certain flags.
- `update_joystick_state_flags`: Constructs a bitmask representing the current state of joystick buttons and directions by performing bitwise logic on a global variable and a register value.
- `process_mouse_input`: Reads bits from a mouse port register and performs bitwise logic to construct a return value based on the state of a global variable.
- `process_joystick_input`: Reads bits from a joystick register and performs bitwise logic to construct a return value based on the state of a global variable.
- `perform_no_operation`: The routine executes a sequence of instructions but performs no functional changes to registers or memory before returning.
- `perform_no_operation_2`: The routine performs a series of no-operation instructions.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `call_library_and_map_zero`: Calls a library function through a stored base pointer and replaces a zero result with minus one, returning that value.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `lock_and_unlock_conditionally`: Calls a function through a pointer from a global variable, and if it returns nonzero, calls a second function through another pointer from the same variable with an argument register.
- `open_and_close_file`: Calls a function through a pointer from a global variable, and if a global flag byte is nonzero, calls two further functions through pointers from the same variable and then loops indefinitely.
- `call_and_map_zero`: Calls a routine through a pointer and replaces a zero return value with minus one.
- `open_file_or_fail`: Calls a routine and then invokes a library function through a pointer from a table; if the result is zero it returns minus one, otherwise it returns the result.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_external_function`: Calls a function pointer located at an offset from a value stored in a global variable.
- `write_then_set_flag`: Calls a routine, invokes a library function through a pointer from a table, and then sets a global variable to one.
- `wait_for_disk_input`: Enters a loop checking specific global variables and executes a series of routines until a carriage return character is encountered.
- `copy_and_clear_planes`: Copies words from five source planes into an interleaved destination buffer and clears the source words, over a fixed grid.
- `copy_from_interleaved`: Copies words from an interleaved source buffer into five destination planes, over a fixed grid.
- `nop_stub_routine`: An empty routine that performs no operations and returns immediately.
- `fetch_exec_message`: Calls a library function to retrieve a message and enters a loop to check specific bits within the message structure.
- `return_uninitialized_stack_value`: This routine returns an uninitialized value from the stack.
- `lock_current_directory`: Calls a routine to retrieve a directory and then calls another routine to lock it if the result is non-zero.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.
- `wait_for_flag_scale`: Repeatedly calls a routine until a global variable is no longer minus one, then computes the difference of two globals, multiplies it by a global word, and divides by one thousand.
- `get_dos_info`: Calls a library function to retrieve file information and stores the result in a global variable.
- `update_global_table`: compares two global values and, if they differ, updates a table of five values and swaps two other global variables.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_state_parameters`: Checks a global variable against a stored value and, if they differ, updates a series of consecutive data words with new constant values before calling another routine and swapping two global variables.
- `update_parameter_table`: Compares a global value against a stored variable and, if they differ, resets a sequence of data values in a table before calling another routine three times while swapping two global variables.
- `update_parameter_table_2`: Compares a global variable against a stored value and, if they differ, updates a series of related data entries with new constants before calling another routine multiple times.
- `update_state_parameters_2`: Checks if a global value matches a specific threshold and, if not, resets a block of data to predefined values before calling another routine and swapping two global variables.
- `swap_parameter_tables`: Compares two values and, if they differ, swaps a set of values in a table with another set and reinitializes them.
- `swap_and_initialize_params`: Swaps several values in a table or global group and initializes a new set of parameters if a certain condition is met.
- `copy_block_and_toggle_globals`: Calls another routine, copies five longwords from one global buffer to another, then repeatedly calls routines in a loop, swaps two global pointers, temporarily sets five consecutive global words to fixed values, calls another routine, and restores the original values.
- `update_table_values`: Calculates a value from a global variable and adds it to several consecutive entries in a table.
- `initialize_data_structure`: calculates several values based on a pointer and a memory offset, then stores them into a series of global variables and a newly initialized structure.
- `initialize_object_header`: Sets a header value to four and populates a structure with calculated offsets and size parameters.
- `allocate_and_setup_header`: Allocates a block of memory, writes a header to it, and calculates several sequential offsets and sizes based on the allocated pointer and other global values.
- `allocate_block_and_write_header`: Computes a row size from a structure field, derives several sequential offsets and sizes from global values, stores the input pointer in a global, writes a fixed word and two differences into the block header, and stores a size value.
- `manage_stack_pointer`: calculates a pointer based on a register and a global variable, then stores that pointer into a table if it falls within a specific range.
- `update_allocation_pointer`: Calculates a new pointer based on a register value and a global variable, then updates the global variable if the new pointer is greater than the current value.
- `insert_stack_element`: Calculates a new position in a descending stack or buffer based on a register value and updates a global pointer.
- `insert_stack_element_2`: Calculates a memory offset based on a register value and checks it against a global boundary.
- `manage_dynamic_stack_pointer`: Calculates a memory offset based on a register value and checks it against a global boundary.
- `calculate_stack_offset`: Calculates a value based on a register, subtracts it from a global variable to create a pointer, and stores the result in that memory location if a condition is met.
- `return_stack_value`: Returns the value currently on the stack.
- `free_block_if_top`: If the global allocation pointer equals the block address passed in a register minus a header size, advances the pointer by the size word stored at the block start; returns the passed value.
- `return_zero_3`: This routine is empty and simply returns a zero value to the caller.
- `return_constant_4`: Returns the constant -1 in d0 to the caller without any other effect.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `return_empty_pair`: This routine is empty and performs no operations.
- `add_offsets_on_nonzero_result_2`: Calls another routine that returns a 16-bit value and, when that value is non-zero, adds fixed constants to two global 32-bit values.
- `return_concatenated_registers`: Calls another routine and returns a 64-bit value composed of two 32-bit registers.
- `update_bitmask_and_shift_data`: Checks certain boundary conditions and, if met, clears specific bits in a global variable and then performs a bitwise-masked bit-shift.
- `blit_masked_sprite_region`: Clips a rectangular region against a global bounding box, then copies masked source words from a table into five destination planes while saving the original words, using shifted bit masks derived from the source data.
- `blit_5plane_tile_column`: Copies a 16-row block of five bitplanes from a source table into five destination pointers, advancing each destination by a stride derived from a global and optionally shifting the source by one word when a global flag is set.
- `tiny_ori_2`: Small routine of 1 instruction that merges 117 into d0.
- `blit_single_row_region`: Sets up the blitter with fixed masks and modulo values, waits for the blitter to finish, then programs the source and destination pointers from computed row and column offsets and issues one blit of a single row region.
- `blit_loop_over_rows`: Configures the blitter once, then loops over a computed number of rows, each iteration programming source and destination pointers advanced by a row stride and issuing a blit, waiting for completion each time.
- `copy_tiled_block_rows`: Computes a destination offset from a horizontal position and a row stride, then copies a block word-by-word across several source planes into interleaved destination rows, advancing each source and destination pointer by a constant stride after each row.
- `configure_blitter_transfer_parameters`: Calculates source and destination dimensions and offsets based on table values and coordinate bounds.
- `update_blitter_parameters`: Checks specific bits in a byte from a table and then calculates and stores a value into a global variable based on the result of that check.
- `blit_clipped_sprite_with_masking`: Clips a sprite rectangle against global bounds, converts source pixel rows into masked planar bitplane data through a lookup table, and programs the blitter to draw the clipped region, enabling DMA channels.
- `update_clipping_bounds`: Compares input coordinates against a set of global boundary limits and updates them if the input values fall outside the current range.
- `clip_and_calculate_sprite_blit`: Calculates the intersection between a sprite rectangle and the screen boundaries to determine valid drawing dimensions.
- `update_pointer_via_conditional`: Checks if a global variable is zero, then conditionally stores that variable into a memory location pointed to by an address register.
- `store_byte_through_pointer`: Stores a byte from a global variable through an address register at a fixed offset, then calls another routine and returns the stack value.
- `configure_audio_channel`: Enters supervisor state through a library call, then computes a channel bit mask and a clamped volume value and writes volume, period and a sample pointer to the audio hardware registers.
- `clear_channel_flag_if_negative`: Tests a per-channel flag byte and, if its sign bit is set, clears that flag byte, clears a byte through a pointer, and computes a channel bit mask shifted left by seven into a global variable.
- `decrement_global_timer`: Checks a bitmask within a global variable and recursively calls itself if certain bits are set.
- `decrement_channel_counter`: Decrements a per-channel word counter through a pointer and, when it reaches zero, clears a byte through a pointer, clears the per-channel flag byte, and computes a channel bit mask shifted left by seven into a global variable.
- `execute_sequential_routines`: Calls another routine four times in succession.
- `save_registers_then_call`: Passes through a call to another routine and returns the register pair.
- `conditionally_toggle_interrupt_state`: Checks a global variable and, if non-zero, calls a routine to toggle audio and interrupt states based on a register value.
- `forward_to_clamp_and_store`: Passes a signed word from a data register to another routine that clamps the value and stores it in multiple locations.
- `compute_scaled_seven_bit_value`: Masks a value in a register to seven bits, multiplies it by eight, subtracts seven, and stores the result in a global variable.
- `conditionally_set_control_bits_2`: If a global byte is non-zero, it stores a handler pointer into a global variable and writes two constant words to global variables.
- `clamp_and_store_word`: Clamps a signed value between a minimum and a maximum range, then stores the resulting word into a global variable and another location.
- `write_word_to_offset`: Stores a word from a register into a memory location at a fixed offset from a pointer.
- `update_audio_and_interrupt_state`: Updates several global variables and a table with values derived from registers, while configuring audio hardware registers and enabling interrupts and DMA via hardware control registers.
- `write_memory_offsets`: Stores two constant words into memory locations at fixed offsets from a pointer.
- `setup_audio_interrupt_state`: Saves four register values into global variables, calls a routine through a function pointer held in a global table, and clears a block of global bytes.
- `toggle_audio_and_interrupt_state`: Depending on a global byte and the sign of a register value, it saves or clears state bytes, masks interrupt and DMA enable registers, and writes control values to a hardware register.
- `conditionally_set_control_bits`: If a global byte is non-zero, it stores a handler pointer into a global variable and writes two constant words to global variables.
- `dispatch_state_bitmask_2`: Tests specific bits within a global variable and calls different subroutines based on which bit is set.
- `dispatch_bitmask_conditional_calls_2`: Tests specific bits within a register and calls one of several different subroutines based on which bit is set.
- `dispatch_bitmask_conditional_calls`: Tests specific bits within a register and calls one of two different subroutines depending on which bit is set.
- `test_bit_and_call`: Tests a specific bit within a register and calls another routine if the bit is set.
- `dispatch_state_bitmask`: tests multiple bits within a word located at an offset from a pointer and branches to one of several subroutines based on which specific bit is set.
- `setup_callback_structure`: Writes several values to a structure, including setting a pointer to itself and another routine into specific offsets.
- `initialize_function_pointer_table`: Sets a global variable to a specific value and populates a table with function addresses and zeroed values.
- `register_callback_pointers`: Updates a global variable and populates a structure or table with two function pointers.
- `update_function_pointers`: Updates a global variable with a constant value and modifies a table of function pointers by storing the address of the current routine and another routine into specific offsets.
- `register_function_pointers`: Updates a global variable with a constant value and populates a structure or table with the addresses of two different routines.
- `initialize_global_values`: Sets a global variable to a specific value and performs a bitwise OR operation.
- `register_callback_pointers_2`: Updates a global variable and writes the address of this routine and another routine into a table or structure via offsets from a pointer held in a register.
- `initialize_timer_and_pointers`: Writes constants to global variables, stores a routine pointer and a value into a structure through an address register, and clears two global variables and a pointer.
- `parse_sprite_command_data`: Parses a command stream to update sprite attributes, including position, frame index, and visibility.
- `interpret_bytecode_stream`: Reads a stream of variable-length instructions and executes them by performing operations such as updating hardware registers, modifying global tables, and manipulating memory pointers.
- `execute_bytecode_instruction`: Interprets a stream of variable-length instructions by reading from a pointer and updating hardware registers.
- `parse_custom_bytecode`: Interprets a stream of variable-length instructions by reading from a pointer and updating hardware registers.
- `interpret_bytecode_stream_2`: Parses a stream of variable-length instructions by reading opcodes and operands from a pointer.
- `update_global_pointer`: Increments a global variable by four and stores a calculated address into a location offset from another global variable.
- `interpret_bytecode_stream_3`: Reads a stream of variable-length instructions and updates a set of virtual registers and memory-mapped I/O values.
- `update_audio_hardware_via_bytecode`: Interprets a custom bytecode stream to update Amiga audio hardware registers and manage internal sound parameter tables.
- `interpret_bytecode_stream_4`: Reads and decodes a stream of variable-length instructions from a memory pointer.
- `configure_audio_sample_parameters`: Parses a bytecode stream to update audio hardware registers, specifically setting the sample period and frequency based on values extracted from a data table.
- `parse_compressed_asset_stream`: Iterates through a data stream to decode compressed information by reading control bytes and updating hardware registers.
- `decode_and_dispatch_bytecode`: Parses a stream of variable-length data using a switch-based jump table to perform specific operations.
- `set_bit0_pointer_and_width`: If bit 0 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 164, stores the pointer plus a field at offset 160, and sets a word at offset 154 to 128.
- `set_bit0_handler_and_mask`: If bit 0 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode1_and_masks`: Stores the value 1 at offset 150 and the value 128 at offsets 154 and 156 of the object.
- `set_bit1_pointer_and_width`: If bit 1 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 180, stores the pointer plus a field at offset 176, and sets a word at offset 154 to 256.
- `set_bit1_handler_and_mask`: If bit 1 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode2_and_masks`: Stores the value 2 at offset 150 and the value 256 at offsets 154 and 156 of the object.
- `set_bit2_pointer_and_width`: If bit 2 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 196, stores the pointer plus a field at offset 192, and sets a word at offset 154 to 512.
- `set_bit2_handler_and_mask`: If bit 2 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable.
- `set_mode4_and_masks`: Stores the value 4 at offset 150 and the value 512 at offsets 154 and 156 of the object.
- `decrement_counter_set_bit3_mode`: If bit 3 of a byte at offset 3 of the object is set and a global counter reaches a negative value, stores the value 8 at offset 150 and 1024 at offset 154.
- `return_no_operation`: The routine consists of a single return instruction with no other operations, so it performs no work when called.
- `halt_execution`: The routine contains no valid instructions and terminates control flow immediately upon entry.
- `empty_routine`: This routine contains no executable instructions.
- `empty_routine_2`: This routine contains no executable instructions and performs no operations.
- `empty_routine_3`: This routine contains no valid instructions and performs no operations.
- `clear_memory_block_2`: clears a block of memory by writing zeros to a location specified by a pointer.
- `null_routine`: The routine is empty and contains no executable instructions.
- `return_audio_interrupt_handler`: This routine is empty.
- `process_object_list`: Iterates through a list of objects, updating bounds, managing a linked list, and copying word pairs into global variables.
- `execute_indirect_call`: Performs an indirect call using a table lookup based on a value from a register and a pointer.
- `read_three_bytes_advance_pointer`: Reads three consecutive bytes from a pointer, stores the pointer advanced by three at a stack offset, and returns the three bytes packed together.
- `read_three_bytes`: Reads three consecutive bytes from a pointer and returns them as a single packed value.
- `check_and_clear_value`: Compares a value to a memory location and, if a global variable is zero and certain conditions are met, clears that memory location.
- `conditionally_return_three_bytes`: Checks if a flag is zero; if so, it constructs a 32-bit value by concatenating three bytes from a pointer and returns it.
- `copy_register_data`: Copies the contents of one data register to another via a single-instruction routine.
- `swap_linked_list_pointers`: Swaps two adjacent pointers within a linked list structure.
- `execute_indirect_jump`: Performs an indirect call to a function address calculated by adding an offset from a table to a base pointer.
- `copy_record_fields_and_dispatch`: Clears three global offset words and, if a flag byte is zero, copies a fixed set of record fields between structures while adding global offsets to some.
- `iterate_and_update_data_table`: Iterates through a linked list of data structures, performing conditional updates to specific fields based on byte comparisons and string length checks.
- `update_structure_data`: Modifies a byte at an offset from a global variable, copies a block of data using a loop, and updates several fields in a structure using values from another structure.
- `store_byte_via_offset`: Stores a byte from a source pointer into a global buffer at an offset given by a signed word read from the same pointer.
- `update_if_different`: Compares a value from a pointer to a value at a calculated offset and updates that offset if they differ.
- `modify_bit_flags`: Clears a specific bit and sets another bit in a byte located at an offset from a global variable, while also performing linked list insertion and removal operations.
- `insert_into_linked_list`: Inserts a new value into a linked list or updates a pointer within a structure if the list is empty or certain conditions are met.
- `remove_from_linked_list`: Traverses a linked list to find a specific value and updates a pointer or replaces a node if a match is found.
- `calculate_cross_products`: Performs a series of signed byte multiplications and subtractions, storing the results in a structure.
- `compute_scaled_vector_offset`: Calculates a new position and direction by applying a bit-shifted scale factor to a vector, then updates global destination coordinates.
- `extract_byte_conditionally`: Checks a byte at an offset from a pointer and returns either the original register values or a combination of a register and a byte from the pointer.
- `clear_flag_and_return`: Loops while a 16-bit value read through a pointer is non-zero, calling another routine on each pass that may unlink list nodes.
- `traverse_and_update_linked_list`: Recursively traverses a linked list, updating or replacing node pointers based on a condition checked against a byte in the node.
- `update_linked_list_node`: Updates a node in a linked list or replaces a node with another if certain conditions are met.
- `copy_word_pair`: Copies two consecutive words from a memory location pointed to by a register into two global variables.
- `apply_coordinate_offset`: Calculates a difference between two points and adds that difference to a list of coordinate values.
- `offset_list_coordinates`: Adds values from a source or offset table to an existing list and updates a flag in a list.
- `process_object_list_and_copy_data`: Waits for a bit to be set, enables interrupts, updates object bounds and manages a linked list of object entries, and copies a block of words into global variables until a pointer limit is reached.
- `update_sprite_table_and_bounds`: Updates a sprite or object table by extracting coordinate and dimension data from a source structure and writing it to a destination list.
- `write_to_table_offset`: Writes several values, including a global variable, to a table or memory area using an offset.
- `relink_node_and_check_bound`: Copies a next-pointer field from one node to another, pushes the node onto a global free-list, and sets a global flag when a table value is at or exceeds a global threshold.
- `update_table_entry`: Checks a global flag and a table value before writing several fields, including a word, a longword, and a byte, into a structure at a specific offset.
- `initialize_global_bounds`: Sets four global variables to specific high and low values.
- `expand_global_bounds`: Updates four global variables with new values if they exceed current bounds based on data from a table.
- `insert_sorted_list_node`: Inserts a node into a linked list ordered by priority fields, comparing a short and two bytes against existing nodes, with a special case for a certain global flag and a specific node index.
- `process_tiled_graphics_list`: Iterates through a table of graphic data to perform clipped blitting of tiles and sprites.
- `set_flag_from_bounds`: Compares two global bound values against two other global bounds and stores either one or all-ones into a global variable depending on the comparisons.
- `initialize_buffer_constants`: Copies a sequence of four word-size values from one memory location to another.
- `copy_table_block`: Iterates through a table of 16-bit values, copying them into a set of global variables.
- `clamp_rect_bounds`: Checks if a range or area defined by a pointer-offset table exceeds certain global bounds and clamps those values to a minimum threshold if they are too small.
- `expand_rect_bounds`: Checks if a rectangular area defined by a table and certain global values is within a valid range, then expands the global bounds to include that area.
- `update_clipping_region`: Updates a global clipping or visibility rectangle by comparing a new area against existing bounds, including bitwise masking and calculating an offset or size.
- `copy_memory_block_2`: Copies a block of memory from a source pointer to a destination pointer, decrementing a counter and updating a pointer for each word or byte transferred.
- `return_immediately_7`: An empty routine that performs no operations and returns to the caller.
- `update_global_pointers`: Calculates a new address by adding a shifted version of a global variable to itself and stores the result into two global variables.
- `reset_global_variable`: Sets a global variable to zero through a series of assignments.
- `check_nibble_sum`: Compares the sum of two lower nibbles against a constant value and returns if the sum exceeds that value.
- `compare_low_nibbles_conditionally`: Subtracts the low nibble of one data register from the low nibble of another, returning early if the result is negative or at least sixteen and otherwise continuing through the remaining logic.
- `reconstruct_nibble_data`: Iterates through a table of words to detect mismatches between two memory locations.
- `copy_and_update_data`: Checks a global variable against several constants, copies a block of memory from one location to another, and conditionally updates a global variable based on a register value.
- `call_nibble_sum_check`: This routine calls a procedure that compares the sum of two lower nibbles against a constant value.
- `compare_low_nibbles_conditionally_2`: Subtracts the low nibble of one data register from another and returns.
- `copy_register_data_2`: Copies the contents of one register to another register.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `execute_indirect_call_2`: Performs an indirect call to a location determined by adding an offset from a table to a base address.
- `swap_table_entries`: Swaps two consecutive words in a table and performs an indirect call to a location determined by an offset from a base address.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine`: This routine performs no operations and immediately returns to the caller.
- `execute_empty_call`: This routine is empty and performs no operations.
- `compare_registers_and_branch`: Copies a value from one register to another and then checks if the new value is less than the original value, exiting if the condition is met.
- `compare_registers_and_branch_2`: Compares two or more words and branches if the value in one register is not greater than the value in another.
- `branch_on_global_zero`: Checks if a global variable is zero and calls another routine if it is.
- `return_constant_value`: Returns a fixed integer value in a data register to the caller.
- `tiny_move_rts`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `perform_indirect_call`: This routine executes an indirect call to a destination address calculated by adding an offset from a table to a base address.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `store_byte_at_offset`: Stores a byte from a register into a memory location calculated from a pointer and an offset.
- `store_word_at_offset`: Stores a 16-bit value from a register into a memory location calculated from a pointer and an offset.
- `reset_local_variables`: Resets a global variable to zero and processes a string or table.
- `build_path_and_initialize`: Clears globals, copies a string into a buffer, appends another string, then calls routines that lock the current directory, initialize data structures, decode a run-length stream, and build a sorted list of pointers.
- `initialize_data_structures`: Initializes a series of global variables and a table with specific values and a loop that populates a table with calculated pointers.
- `process_complex_data_structure`: Performs string manipulation, RLE decoding, binary searching, and linked list insertion while updating various global state variables.
- `decode_rle_stream`: Reads a control byte from a global stream to determine whether to repeat a single byte or copy byte pairs.
- `fetch_string_character`: Compares two values and returns a byte from a memory location pointed to by a table.
- `write_byte_with_bounds_check`: Writes a byte to a destination if a calculated address range check passes.
- `transform_text_string`: Iterates through a string of characters and performs byte-level manipulations including character substitution and length-based shifts.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `calculate_byte_difference`: Calculates the difference between two bytes and returns the result or its negation.
- `invoke_and_return_pair`: Calls another routine and returns a concatenated 64-bit value formed from two 32-bit registers.
- `search_sorted_table`: Performs a binary search through a table of word-sized values to find a match for a given value.
- `initialize_list_structure`: Iterates through a memory range, writing incrementing offsets and zero-values to a list or buffer.
- `insert_into_linked_list_2`: Inserts a data structure into a sorted linked list by traversing a table of pointers and shifting existing entries to accommodate the new element.
- `initialize_object_from_list`: Pops an element from a linked list and initializes a new data structure by copying values from a source list and zeroing out specific memory fields.
- `allocate_and_initialize_object`: Calculates an offset within a memory pool to initialize a new data structure, zeroing out specific fields and setting up pointers.
- `loop_until_negative_return_2`: Repeatedly calls a routine through a pointer and checks the return value, updating a global variable with the result.
- `execute_conditional_function`: Calls a function pointer and subsequently calls a second function pointer if the first call returns a non-zero value.
- `loop_and_fetch_result`: Repeatedly calls another routine in a loop until it returns a non-negative value.
- `loop_file_open`: Repeatedly calls a routine that attempts to open a file, continuing to call another routine if the result is negative until a non-negative value is returned.
- `loop_until_negative_return`: Repeatedly calls a routine through a pointer and checks the return value.
- `process_text_string`: Iterates through a string of characters starting from a pointer provided in a data structure.
- `update_channel_countdown_state`: Sets a global byte to one, increments it with wraparound, and then decrements and conditionally clears a global countdown, calling other routines on specific transitions.
- `update_global_and_copy`: Updates a global variable, checks boundary conditions, and copies a sequence of memory blocks from a source to a destination.
- `decrement_counter_and_clear_fields`: Decrements a word counter in a structure and, while non-zero, adds two word fields with clamping and conditionally updates a third field before calling another routine; otherwise, it zeroes several structure fields and calls another routine.
- `call_subroutines_sequentially`: Calls several subroutines in succession before returning.
- `clear_structure_fields`: Through an address register, writes zero to three byte offsets and zero to two word offsets of the pointed-to block.
- `update_clamped_channel_state`: Checks three global counters to either add a value to a global variable and clamp it to a maximum, or decrement specific counters.
- `return_to_caller`: Immediately returns to the calling routine without performing any operation on data.
- `nop_routine`: The routine is empty and performs no operations.
- `return_from_subroutine_2`: The routine is empty and consists of a single return instruction.
- `empty_routine_4`: The routine contains no executable instructions and performs no operations.
- `execute_empty_handler`: This routine contains no valid instructions and terminates execution immediately upon entry.
- `execute_object_callback`: Invokes a function pointer stored within an object's data structure.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `perform_no_operation_3`: The routine contains no executable instructions.
- `empty_routine_5`: The routine contains no valid instructions and performs no operations.
- `execute_invalid_instruction`: The routine contains no valid executable instructions and terminates immediately upon entry.
- `halt_execution_2`: The routine is empty and contains no valid instructions.
- `execute_object_handler`: Invokes a function pointer retrieved from an object's data structure during a list traversal.
- `update_object_state`: Iterates through a list of data structures to decrement specific byte values and manage object lifecycle flags.

### Boston Bomb Club (1990)(Silmarils)[cr Company][m JIVE].adf:bootblock

- `initialize_system_environment`: Clears a large block of memory and initializes various system structures and parameters.
- `update_parameter_structures`: Updates multiple integer values within a structure and configures a DMA transfer by writing to memory and the DMA control register.
- `update_parameter_structure`: Stores an integer value into a specific offset within a structure pointed to by a parameter.
- `initialize_boot_parameters`: stores a pointer to a global variable and two integer values into a structure before calling another routine.
- `configure_dma_transfer`: Stores several values into memory and enables a DMA channel by writing to the DMACON register.
- `calculate_checksum_and_store`: Calculates a one's complement sum of a sequence of words and stores the result in a memory location.
- `store_inverted_checksum_of_block`: sums 24 words of a block starting at a fixed offset and stores the bitwise inverse of the sum in the following word.
- `copy_modified_data_block`: checks for a specific signature, modifies a sequence of words by changing a value of five to six, and copies the resulting data to a new location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_flag_and_call`: Polls a global variable until a specific bit is cleared, then performs several assignments and calls eight other routines.
- `wait_for_flag_and_call_routines`: Polls a global variable until a specific bit is cleared, then performs several assignments to a global variable and calls eight other routines in turn.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `execute_abs_move`: Performs an absolute move instruction.
- `loop_on_flag`: Repeats a loop while a specific bit in a global variable is set, then performs several assignments to a global variable and calls another routine multiple times.
- `process_table_entries`: Iterates through a table to search for a specific character in a string and subsequently executes a routine using a value fetched from a table.
- `process_table_data`: Iterates through a table to update a pointer and then executes a routine using a value fetched from a second table.
- `iterate_and_call_table`: Iterates through a list of values to update a global variable and eventually calls a routine using a value from a table.
- `iterate_and_call_routine`: Iterates through a list of values, storing each into a global variable until a negative value is encountered, then calls a routine using a value from the list as an index.
- `run_record_script_dispatch`: Runs through a record table, calling a handler for each entry from a dispatch table until a terminator stops it.
- `check_fire_button_flag`: Checks a shared flag bit that reflects the fire-button and disk status and, when the bit is clear, spins while it is set.
- `initialise_sprite_record_table`: Writes a fixed value into one field of a hundred and fifty-seven twenty-four-byte records and clears or sets several other fields in each one.
- `run_blitter_operation`: Waits for the blitter to be idle, programs its control, modulo, destination pointer and size registers, then waits for it to finish.
- `return_from_subroutine_4`: This routine performs no operations and immediately returns to its caller.
- `copy_table_words_into_records`: Copies a column of words from a source table, chosen by the caller's index and a fixed row size, into one field of consecutive twenty-four-byte records.
- `copy_table_words_to_slot`: Copies a column of words from a source table, chosen by the caller's index and a fixed row size, into a second field of consecutive twenty-four-byte records.
- `set_bitplane_pointers_and_modulo`: Writes two bitplane address pointers stepping by forty bytes per row into a list, sets a fixed width value in each of a hundred and fifty-seven records, and stores three addresses for the display.
- `initialize_state_table_and_dispatch`: Initializes a table of data structures with specific word values through a loop.
- `return_immediately_6`: The routine is empty and performs no operations before returning to its caller.
- `increment_shared_pointer_and_copy`: Increments a global pointer by two bytes and copies words from a source buffer into a destination table while wrapping the source.
- `increment_shared_pointer_and_copy_alt`: Increments a global pointer by two bytes and performs a word copy from a source buffer into a destination table.
- `copy_buffer_with_wrap`: Copies words from a source buffer to a destination table, wrapping the source pointer if it reaches a specific boundary, and increments a global pointer.
- `advance_word_pointer_by_two`: Advances a shared word pointer by two bytes and returns; the comparison that follows has no effect.
- `copy_words_into_mirrored_records`: Copies words from a source buffer into one field of records stepping forward and into another field stepping backward, wrapping the source at its end.
- `copy_data_to_buffer`: Copies words from a source to a destination at specific offsets and updates a global index using a masked value.
- `advance_pointers_by_offsets`: Advances two different pointers by values stored in a table, provided the pointers do not equal specific constant values.
- `increment_pointers_by_table`: Increments two pointers by adding values retrieved from a table.
- `write_record_pair_and_advance`: If the shared pointer has not reached the end, writes the same value into two fields of the record it points to and moves on to the next record.
- `write_record_pair_and_retreat`: If the shared pointer has not reached the start, writes a negative value into two fields of the record it points to and moves back one record.
- `increment_counter_until_limit`: Increments a shared counter by two and calls itself recursively until the counter reaches a specific limit.
- `step_counter_down_by_two`: Subtracts two from a shared counter while it is not two, and calls another routine after each step.
- `reset_record_fields_and_compute`: Sets two fields of each of a hundred and fifty-seven records to a fixed negative value, then computes a division-based value from shared state.
- `update_accumulated_values`: Performs arithmetic on global variables, including a conditional sign-halving and clamping operation.
- `clamp_and_recalculate_global_values`: Calls a routine, clamps a global variable to a maximum limit if exceeded, and performs arithmetic operations involving sign-halving and additions on two global variables.
- `increment_global_counter`: Increments a global variable by one.
- `increment_global_variable`: Increments a global 32-bit integer by one.
- `increment_global_variable_2`: Increments a global 32-bit integer by one.
- `execute_blitter_sequence`: Increments a global variable and calls another routine three times.
- `run_blitter_sequence`: Calls another routine three times in succession.
- `increment_counter_and_call_four`: Increments a shared counter and then calls four other routines in turn.
- `set_blitter_masks_then_run`: Waits for the blitter to be idle, sets its first-word and last-word masks and modulo values, then calls a routine three times.
- `update_table_data`: Updates a table of values based on a global index and performs a series of memory writes to a specific range.
- `update_table_data_2`: Calculates a value based on a global variable and populates a table with specific word patterns and calculated addresses.
- `step_pointers_by_table_offsets_2`: Advances two pointers by adding offsets retrieved from a table.
- `step_pointers_by_table_offsets`: Advances a shared pointer by the offsets stored at it, unless it has reached one of two end values.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `reset_shared_counter_and_constants`: Stores two fixed values into fixed memory addresses and clears a shared counter.
- `call_eight_routines`: Calls eight other routines in turn and returns.
- `clear_byte_range`: Zeroes a run of bytes starting at the address in the address register, for the count given in the data register plus one.
- `initialise_state_and_draw_records`: Initialises a large set of shared fields and table values, then calls sixteen routines that fill and update the display records.
- `copy_and_merge_data`: Copies a block of data from one location to another and then performs a bitwise OR operation on specific word pairs in a loop.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_shared_address_and_mode`: Stores a fixed address into shared state, sets a mode value of two, and returns.
- `wrap_angle_pair_modulo`: Adds two input deltas to a pair of stored 16-bit values and wraps each one into the range of a fixed period of seven hundred and twenty, adding the period back when the sign bit is set.
- `copy_nested_data_structures`: Uses a table of indices to copy multiple data blocks from a source to a destination buffer, or zeroes the buffer if conditions are not met.
- `clear_word_buffer`: Zeroes a buffer of twenty-five thousand two hundred and eighty words, one word at a time, starting at the address in the address register.
- `copy_word_data_blocks`: Copies several word-sized data blocks from a source to a destination based on a table of indices, or zeroes the buffer if conditions are not met.
- `copy_byte_data_blocks`: Copies several byte-sized data blocks from a source to a destination based on a table of indices, or zeroes the buffer if conditions are not met.
- `copy_structured_data_to_buffer`: Copies multiple structured data blocks from a source to a destination buffer using a table of indices.
- `copy_word_structured_data`: Copies multiple word-based structured data blocks from a source to a destination buffer using a table of indices.
- `copy_structured_data_from_table`: Copies multiple fields from a source table into a destination buffer using an index derived from a byte value.
- `fill_buffer_from_table_or_zero`: Fills a destination area with data from a table using nested loops, or zeroes out a large block of memory if certain conditions are not met.
- `clear_memory_block`: Fills a block of memory with zeros using a counter.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `open_library_and_store_field`: Calls a library-opening routine through the system library, stores a field from the returned structure into a global, then clears another global and sets a third to a constant.
- `enable_vertical_blank_interrupt`: Stores an interrupt-related address in a global variable and writes a value to the interrupt enable register to enable interrupts.
- `disable_all_interrupt_sources`: Sets the top bit of a shared word, then writes the mask that, with the top bit clear, clears every interrupt enable bit.
- `dispatch_interrupt_request_state`: Checks a shared value through several branches and calls up to nine routines according to the result, touching the interrupt request register on the way.
- `clear_data_register_d0`: Clears the least significant byte of data register d0.
- `decrement_column_stride_bytes`: Walks a fixed grid of 13 byte cells at a stride of 40 bytes per row, subtracting an increasing value (1, then 2) from each cell, with the start position stepped by 44 bytes between passes.
- `tiny_ori_3`: Small routine of 1 instruction that merges 68 into d0.
- `decrement_bitplane_column_bytes`: Walks a 13-step pattern through a fixed-stride byte grid, subtracting a rising counter (1 then 2) from each byte, so every row of the 12-entry column is darkened by a step that grows per pass.
- `subtract_graded_bytes_in_strided_grid`: Walks a 2-row, 12-column grid of byte cells in memory, stepping 40 bytes between cells in a row and 44 bytes between rows, and subtracts an increasing value (1 for the first row, 2 for the second) from each cell.
- `subtract_graded_byte_columns_counted`: Subtracts an increasing value from each of a counted number of byte columns, where each column is twelve bytes long and spaced forty bytes apart.
- `write_sprite_control_pairs`: Clears a shared word and writes a pair of control words into each of two table entries spaced eight bytes apart.
- `initialize_display_parameters`: Writes specific word patterns to a memory block and updates several global variables based on a conditional check.
- `copy_strided_word_block`: Copies a block of twelve rows of seventeen words, reading the source at a stride of three words and writing them consecutively into the destination.
- `manage_linked_list_buffer`: Decrements a counter and, if it reaches zero, performs a series of operations on a linked list or buffer structure.
- `pop_linked_list_element`: Decrements a counter and retrieves the next element from a linked list structure.
- `manage_linked_list_buffer_2`: Decrements a counter and performs conditional operations on a linked list or buffer based on a control value.
- `manage_data_stream_buffer`: Decrements a counter and, if zero, performs a block transfer or data manipulation based on a mode flag.
- `advance_linked_digit_list`: Decrements a counter in a list header and, upon underflow, reloads it from a stored value and follows a link to the next entry.
- `call_two_routines_pass_through`: Calls two other routines and returns with its register arguments unchanged.
- `call_single_routine_pass_through`: Calls one other routine and returns with its register arguments unchanged.
- `scale_and_store_data`: Performs a bitwise scaling operation on a series of words from a source table and stores the results in a destination table.
- `advance_record_state_machine`: Advances a per-object state machine held in a record by selecting branches based on state fields, updating counters and positions, and performing block memory copies or pointer updates.
- `fill_record_block_when_idle`: When a record flag is zero, calls a routine and fills a block of records with a fixed negative value, then continues with the rest of the record update.
- `read_next_stream_byte_pair`: Reads the next byte from a stream pointer held in a record.
- `read_next_stream_word_pair`: Reads the next word from a stream pointer held in a record.
- `parse_stream_command_byte`: Reads bytes from a stream, stopping on a byte in the range thirty-two to ninety.
- `reset_stream_pointer`: Copies a value from one memory location to another and returns a constant.
- `tiny_ori_4`: Small routine of 1 instruction that merges 90 into d2.
- `copy_indexed_table_data`: This routine performs a block transfer of data by fetching values from two separate tables using indexed offsets and storing the result into a destination buffer.
- `perform_blitter_transfer`: Updates several global variables and uses a blitter routine to transfer data between memory locations using specific control and size parameters.
- `dispatch_indexed_function_3`: Iterates through a table to find a matching byte value and then executes a function pointer from a corresponding table.
- `dispatch_indexed_function_2`: Iterates through a table to find a matching value and then executes a function pointer from another table.
- `dispatch_indexed_function`: Iterates through a table to find a matching value and then executes a function pointer from another table.
- `dispatch_indexed_function_call`: Iterates through a list to find a matching value and then executes a function pointer from a table based on that match.
- `blit_text_glyph`: Waits for the blitter to be idle, then programs its registers to copy one glyph from a font table, indexed by the character code minus the first printable code.
- `clear_sprite_table_entries`: Writes a fixed control value and zero into two blocks of nineteen entries spaced six bytes apart, clearing a sprite table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_table`: Fills a table with a repeating pattern of a constant value and an incrementing sequence.
- `write_word_to_table`: Iterates through a list of pointers and writes a 32-bit value, split into two 16-bit parts, to the locations they point to.
- `configure_blt_hardware`: Writes values to several Blitter registers and waits for a specific bit in a global variable to clear.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub`: The routine consists of a single return instruction and performs no work, no register or hardware access, and no library calls.
- `add_offsets_on_nonzero_result`: Calls another routine and, if the returned 16-bit value is non-zero, adds fixed constants to two global 32-bit values.
