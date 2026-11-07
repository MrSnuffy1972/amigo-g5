# Game 39 (BoulderDashConstructionKit v1.0 0464): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00039). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (967258 top-level bytes in 41 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 382 of 382 named (0 author symbols), 382 of 382 described (10 by rule, 365 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1209 of 1209 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7370 of 7370 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 343798 |
| code | 32164 |
| data | 591222 |
| padding | 74 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 983 | 70 |
| kit | 2 | 6273 | 255 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 67 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |

## Cases handled

- BoulderDashConstructionKit_v1.0_0464/BoulderDashConstructionKit_v1.0_0464.hdf: hard-disk image (AmigaDOS OFS, 1120 blocks): its filesystem parses and all 34 files in it are byte-identical to the extracted tree BoulderDashConstructionKit_v1.0_0464_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 52 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `go_sound` (kit): Fills a sound channel record with a sample pointer, two word parameters, a cleared status word, a flag set to one and a second pointer. Kind: disk hardware/track check. Pass path: the code continues execution after the 'rts' instruction in the caller, typically following the logic where the return value in D0 indicates success or failure.
- `initialize_audio_hardware` (kit): Waits for several global flags to clear, performs a countdown loop, and then writes multiple values to audio control and DMA registers. Kind: disk hardware/track check. Pass path: the code continues to the next instruction after the bsr.w an address call.
- `wait_for_bit_clear` (kit): Enters a loop that waits for a specific bit in a global variable to be cleared, decrementing a counter three times. Kind: disk hardware/sync check. Pass path: the loop completes and execution continues to the instruction after the loop (e.g., bra.w an address or cmp.w #an address, d0).
- `initialize_and_wait` (kit): Stores a constant value into a global variable and then enters a loop that waits for a bit in a global variable to clear. Kind: disk hardware/sync check. Pass path: the loop continues and eventually exits to the instructions following the call (lea.l an address.l, a0).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_calls`: The routine stores the current address and a register value into global variables before performing three indirect calls using offsets from a pointer.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a register.
- `execute_indirect_jump_2`: Executes a jump to a location determined by a register value.
- `set_register_and_return_offset`: If a register is zero, it copies the value of another register into it, then returns that register's value decremented by eight.
- `mask_global_variable`: Returns a global variable after applying a bitwise mask.
- `clear_interrupt_request`: Clears a specific bit in the interrupt request register.
- `store_interrupt_request`: Stores a value from the stack into a global variable.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `nop_operation`: This is an empty routine that performs no operations before returning.
- `store_register_to_global`: The routine copies the value from D0 into a global variable.
- `jump_via_global_value`: Copies a global variable to another location and then performs an indirect jump.
- `copy_byte_between_pointers`: Copies a single byte from an offset of one pointer to an offset of another pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: The routine compares a global variable to a specific constant value.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `negate_nonzero_flag`: The routine receives a value in a register and returns zero if the value is non-zero, and returns a negative one if the value is zero.
- `manipulate_interrupt_state`: Performs bitwise rotations on a register value and conditionally updates a global variable and stack values.
- `toggle_interrupt_state`: This routine modifies the interrupt enable register and updates a global variable using a value retrieved from the stack.
- `toggle_interrupt_enable`: The routine writes to the interrupt enable register and conditionally updates a global variable with a value from the stack.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `update_status_byte`: The routine checks bits in a structure and updates a byte at a specific offset in another structure based on those values and a bitmask.
- `negate_short_toggle_bit`: Negates a short value at a specific offset in a structure and toggles the high bit of a byte at an offset in another structure.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: The routine performs no operations and immediately returns to its caller.
- `toggle_bit_and_increment`: The routine clears a bit in a global byte based on a mask, modifies a flag in a structure, and increments a value at an offset in that structure.
- `return_immediately_4`: This routine is empty and performs an immediate return to the caller.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `return_immediately_3`: This routine is empty and performs an immediate return to its caller.
- `clear_offset_and_save_register`: Stores zero to a word-sized location at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `execute_indirect_call_3`: The routine compares a value against a global and then performs an indirect call through a pointer stored in a register.
- `perform_indirect_call`: Loads the address of another routine into a memory location and returns.
- `copy_string_to_buffer`: The routine copies a null-terminated string from an input pointer to a newly allocated buffer, prefixing it with its length byte, and then sets up a structure with fields derived from a global table and a library base, storing the buffer address in the structure.
- `initialize_expansion_structures`: Uses memory allocation and queueing functions to build descriptor structures, sets flags, and stores global pointers into RAM.
- `call_indirect_functions`: Calls a function pointer retrieved from an offset of a value returned by another routine, then calls a second function pointer at a fixed offset from that result.
- `dispatch_via_message`: Constructs a message structure, sends it to a task, and then executes a function pointer found by searching a table for a matching value.
- `dispatch_message_structure`: Populates a structure with values derived from a routine return, executes several function pointers, and searches a table to call another routine.
- `dispatch_message_callback`: Constructs a message, executes several function pointers, and searches a table to call a specific routine.
- `copy_circular_buffer_segment`: This routine manages data movement within a circular buffer by calculating offsets and performing memory copies.
- `shift_memory_buffer`: Increments a pointer and copies a block of data from a source buffer to a destination buffer while managing wrap-around logic.
- `dispatch_message_via_table`: Searches a table for a matching value and executes a function pointer found at an offset from a global variable.
- `dispatch_conditional_message_2`: Sets structure fields based on a return value and executes a sequence of function pointers and a table-based call.
- `dispatch_conditional_message`: Sets structure fields based on a routine return value and executes a sequence of function pointers and a table-based call.
- `dispatch_conditional_logic`: Evaluates a return value to set structure fields and executes a sequence of function pointers and a table-based call.
- `execute_callback_and_dispatch`: Checks a value in a structure, executes a callback, and then performs a table-based dispatch via a message.
- `update_message_offset`: Calculates a new offset based on a conditional check of a structure field and dispatches a message via a table search.
- `dispatch_message_and_execute`: Constructs a message structure with a value and a constant, then executes a function pointer found by searching a table.
- `dispatch_message_with_constants`: Writes specific constant values to a structure and then executes a function pointer found by searching a table.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately`: This routine performs no operations and returns to its caller immediately.
- `return_void`: This routine performs no operations and immediately returns to its caller.
- `rotate_string_and_store_length`: Rotates a null-terminated byte string one position to the right by shifting each byte forward and writes the original first byte into the last position, then stores the length of the string minus one as a byte just before the string in memory.
- `process_file_path_result`: Checks a return value and either populates a structure with several fields or calls a routine with a constant.
- `execute_callback_function`: Checks a register value and, if non-zero, calls a function pointer using values retrieved from a structure.
- `concatenate_file_paths`: Combines a directory path and a filename into a single string by handling separators and null terminators.
- `perform_nested_string_comparisons`: Performs a series of nested string comparisons and returns a result based on their outcome.
- `compare_two_strings`: The routine compares the bytes pointed to by two pointers until a zero byte is found, returning early if the bytes differ, otherwise it falls through after the terminating zero.
- `empty_routine`: This routine is empty and performs no operations.
- `return_from_subroutine_4`: This routine performs a standard return from a subroutine.
- `modify_system_strings`: Checks for specific string patterns in memory and replaces them with different character sequences.
- `compare_case_insensitive`: The routine compares a null-terminated string with a null-terminated sequence in a global table using case-insensitive matching for specific character ranges.
- `compare_case_insensitive_strings`: Calculates the length of a null-terminated string and compares it against a global string using case-insensitive matching for specific character ranges.
- `compare_case_insensitive_strings_2`: Compares two null-terminated strings by iterating through them and performing a case-insensitive character match.
- `execute_indirect_call_4`: Performs an indirect call using a pointer stored in a structure.
- `execute_indirect_call`: This routine performs an indirect call through a pointer stored in a table.
- `return_from_subroutine_5`: This routine performs a return from a subroutine by restoring the program counter.

### kit

- `dispatch_subroutine_table_2`: Iterates through a table of function pointers and executes each subroutine in sequence.
- `perform_no_operation`: Consists of fifteen instructions but performs no functional operations or data manipulations before returning.
- `apply_masks_to_four_words`: Applies a clear mask and then a set mask to four words spaced 8000 bytes apart, using the data register values.
- `check_flag_and_return`: Tests a global flag and a bit of a register value, but both branches return immediately without making any change.
- `check_pointer_and_return`: Compares a global pointer with zero and returns in either case, making no change.
- `calculate_division_loop`: Performs a mathematical loop involving division and bit manipulation to update global variables and memory.
- `initialize_hardware_state`: Performs a loop to initialize hardware registers and global variables, then waits for a specific bit in a global variable to change.
- `initialize_audio_hardware_2`: Populates a table with values through a loop and initializes several global variables and hardware registers including audio and DMA controllers.
- `go_sound`: Fills a sound channel record with a sample pointer, two word parameters, a cleared status word, a flag set to one and a second pointer.
- `trigger_sound_playback`: This routine calls a sound routine by passing a sample pointer and two word-sized parameters.
- `trigger_sound_playback_2`: Calls a routine to populate a sound channel record with a sample pointer, volume parameters, and status flags.
- `update_global_value_mathematically`: Performs a mathematical calculation involving addition, multiplication, and bitwise operations on a global variable.
- `increment_global_with_mask`: Adds a value to a global variable and applies a bitwise AND mask to keep it within a specific range.
- `call_sound_routine`: This routine acts as a wrapper that immediately calls another routine.
- `perform_complex_math`: Performs arithmetic operations on two global variables and stores the results back into them before calling another routine indirectly.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d0.
- `execute_indirect_call_2`: Performs an indirect jump to a target address retrieved from a pointer stored in a data table.
- `invoke_sound_system`: This routine invokes the sound system and then returns without performing any other action.
- `trigger_sound_only`: Calls a routine to configure a sound channel and then returns.
- `bop_sound_record`: Executes a routine to write a 32-bit pointer and two 16-bit values into a record while zeroing counters.
- `invoke_sound_system_2`: Calls a routine to trigger sound playback and then returns.
- `trigger_sound_playback_3`: Calls a routine to populate a sound channel record with a sample pointer, volume parameters, and status flags.
- `trigger_sound_playback_4`: Calls a routine to populate a sound channel record with a sample pointer and specific parameters.
- `bury_sound_effect`: Plays a sound effect by calling a routine that configures a sound channel.
- `update_global_value`: Performs a mathematical calculation involving a constant and a global variable, then stores the result back into that variable.
- `call_sound_routine_2`: Calls a routine that populates a sound channel record with a sample pointer, volume parameters, and status flags.
- `call_sound_routine_3`: Calls a routine that populates a sound channel record with a sample pointer and several parameters.
- `initialize_audio_hardware`: Waits for several global flags to clear, performs a countdown loop, and then writes multiple values to audio control and DMA registers.
- `update_audio_period`: Modifies a global value using a bitwise mask and then updates two hardware period registers.
- `reset_audio_settings`: Writes zero and three to specific global variables and updates audio volume and DMA control registers.
- `call_chain_while_zero_flag`: Calls one routine, then calls up to three more in turn, each only when the previous step left the zero flag set, and touches the hardware audio channel pointers.
- `select_first_positive_entry`: If the record's field is zero, searches four global words for the first positive one.
- `update_active_entry`: Checks if a value matches one of several global identifiers and updates flags or pointers in a record accordingly.
- `manage_audio_and_state`: Updates several global counters and calls a chain of routines that interact with audio hardware and linked lists.
- `manage_linked_list`: Manipulates a linked list structure by updating pointers and modifying values within a control block, including enabling a DMA channel via a hardware register.
- `call_copy_data_block`: Calls a routine that copies multiple words from a global table to a destination buffer and performs a bitwise shift on a value stored in RAM.
- `copy_data_block`: Copies multiple words from a global table to a destination buffer and performs a bitwise shift on a value stored in a RAM location.
- `concatenate_registers`: Combines two 32-bit values from registers into a single 64-bit return value.
- `pulse_keyboard_serial_line`: Clears bit six of a serial control byte, waits in a loop, and then writes a value back to the same byte.
- `modify_keyboard_bit`: Calculates an index and bitmask to set or clear a specific bit within a byte at a memory location derived from the keyboard serial register.
- `clear_bit_in_byte`: Clears a specific bit within a byte at a given memory location using a bitmask.
- `return_register`: Returns the value held in a 32-bit register.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_cia_register`: Writes a specific bit pattern to a CIA hardware register.
- `execute_file_operations`: Performs a sequence of operations involving opening, reading, and closing files through system calls.
- `check_button_status`: Checks a bit in the joystick/mouse fire register and stores a result in a global variable.
- `read_joystick_movement`: Reads the joystick port movement register and performs bitwise operations to update two global variables.
- `dispatch_subroutine_table`: Iterates through a table of function pointers and executes them in sequence.
- `update_status_flags`: Performs a series of conditional checks and updates multiple global variables based on the results of several routine calls.
- `validate_hunk_type_codes`: Compares a value against specific AmigaDOS hunk type constants used for executable loading or relocation.
- `invoke_active_entry_update`: Calls a routine that verifies a value against a set of global identifiers to update record flags or pointers.
- `fill_memory_with_spaces`: Fills a block of memory with a repeating pattern of space characters using a loop.
- `load_and_format_file_data`: Opens and reads a file to copy its contents into memory, then appends a specific file extension based on certain global flags.
- `process_text_buffer`: Iterates through a data block to transform characters and fills a destination buffer with modified bytes and spaces.
- `wait_for_bit_clear`: Enters a loop that waits for a specific bit in a global variable to be cleared, decrementing a counter three times.
- `parse_data_stream`: Iterates through a buffer, performing sparse data copies and reordering elements based on specific values encountered in the stream.
- `copy_sparse_data`: Copies specific bytes from various offsets in a source buffer to corresponding offsets in a destination buffer.
- `reorder_buffer_elements`: Transfers bytes between specific offsets of two buffers in a non-linear, reordered sequence.
- `initialize_data_table`: Sets multiple global variables to zero, a specific constant, or various small integer values.
- `set_handler_pointers`: Stores several function addresses and constant values into a series of global variables.
- `set_handler_pointer_variant`: Stores a specific global variable with an added bitmask into a global variable, followed by other constant assignments.
- `set_handler_pointer_dynamic`: Stores a value from a global variable into another global variable, followed by other constant assignments.
- `set_handler_pointer_self`: Stores its own address into a global variable, followed by other constant assignments.
- `set_handler_pointer_alt`: Stores a specific address into a global variable, followed by other constant assignments.
- `initialize_data_structures`: Populates a series of global variables and memory locations with specific constant values and sets multiple consecutive memory entries to a maximum unsigned value.
- `initialize_hardware_and_handlers`: Sets various hardware registers for display and DMA, populates multiple global variables with constant values, and assigns several handler pointers to global variables.
- `set_handler_values`: Assigns a constant value to several global variables and performs bitwise OR operations on others.
- `enter_infinite_loop`: Executes an infinite loop that prevents further code execution.
- `iterate_string_records`: Iterates through a list of records by calling another routine repeatedly until a termination condition is met.
- `parse_and_insert_string_record`: Scans a memory buffer for a period delimiter to extract a string, validates the string against specific character patterns, and inserts the processed result into a sorted linked list.
- `update_offset_pointers`: Adds values from a pointer to existing global variables and updates a pointer based on a conditional check.
- `poll_and_set_status_bytes`: Initializes five consecutive global bytes to zero and then repeatedly calls another routine to check for a boolean return value.
- `modify_memory_value`: Updates a value at a memory location based on comparisons with specific constants.
- `iterate_value_ranges`: Uses nested loops to increment or decrement global variables until they match specific target values.
- `perform_nested_blitter_copies`: Configures blitter registers and performs a nested loop of block transfers using a specified count.
- `copy_data_blocks`: Copies bytes from one memory location to another at many specific offsets.
- `copy_memory_block`: Copies a block of 8000 words from a source memory location to a destination memory location using a loop.
- `load_offset_values`: Loads three words from a table into global variables using offsets derived from a long value.
- `convert_value_to_ascii`: Adjusts a value using BCD logic and stores the resulting nibbles as ASCII characters in memory.
- `convert_to_ascii`: Splits a 16-bit value into two nibbles and stores them as ASCII characters in a memory location.
- `convert_to_ascii_loop`: Iterates through a list of pointers, calling a routine to convert values to ASCII characters at each step.
- `initialize_pointer_list`: Stores the addresses of three global variables into a table and sets a fourth global variable to zero.
- `swap_global_variables`: Swaps the values of two adjacent global variables.
- `increment_global_value`: Adds a constant value to a global variable.
- `copy_data_blocks_2`: Performs multiple overlapping memory copy operations from a global variable to itself using loops.
- `calculate_offset_address`: Calculates an address based on a global variable and bit-shifted values from registers, then stores it in a global variable.
- `execute_repeated_blitter_transfers`: Performs a series of repeated block transfers within a loop that continues until a specific value is reached.
- `load_hunk_data`: Parses input to determine a source address and copies blocks of data, optionally inverting the bits, into a destination buffer.
- `calculate_memory_offsets`: Performs mathematical calculations involving division and multiplication to update several global variables and memory offsets.
- `execute_division_loop`: Invokes a mathematical routine that performs division and bitwise operations to update memory and global variables.
- `load_and_calculate_offsets`: Loads hunk data and performs mathematical calculations to update global variables and memory offsets.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_state_variables`: Stores a word, a zero, and a long value into three global variables and sets another global variable to one.
- `load_offset_data`: A sixteen-instruction routine that loads data from a calculated pointer into global variables and decrements a counter in a loop.
- `execute_looping_subroutines`: Repeatedly calls a routine while a global variable is non-zero, followed by a single call to that routine, a write to a global variable, and another call to the routine before entering an infinite loop.
- `scan_table_for_zero`: Iterates through a table of shorts and calls another routine whenever a zero value is encountered.
- `scan_table_and_blit`: Iterates through a table of shorts and calls another routine for every non-zero value encountered.
- `execute_blitter_transfers`: Sets blitter parameters and performs three consecutive block transfers using hardware registers.
- `execute_blitter_transfers_alt`: Performs three repeated block transfers by writing to hardware registers and waiting for a specific bit in a global variable.
- `initialize_data_table_2`: Initializes a memory region by filling it with specific repeating patterns of bytes.
- `identify_amigados_hunk_type`: Compares a value against AmigaDOS hunk type codes to distinguish between executable loading segments and relocation segments.
- `decrement_global_variable_loop`: Decrements a global variable by one in a loop until it reaches a specific value, provided the initial value is not one.
- `parse_amigados_hunk_types`: Iterates through loaded file data to identify and process AmigaDOS hunk type codes, such as executable or relocation segments.
- `compress_data_stream`: Compresses a sequence of values into a run-length encoded format, using a high bit to indicate repeated values.
- `expand_and_apply_colors`: Processes a compressed data stream to expand values and writes them to both a global table and the hardware color registers.
- `set_constant_flags`: Assigns specific constant values to several global variables and registers.
- `initialize_and_wait`: Stores a constant value into a global variable and then enters a loop that waits for a bit in a global variable to clear.
- `setup_blitter_and_copy`: Configures blitter parameters and then enters an infinite loop after performing a setup routine.
- `setup_blitter_operation`: Configures blitter control, mode, and size registers before calling another routine and writing two words to memory.
- `setup_blitter_transfer`: Configures blitter registers with specific values and waits for the blitter to complete its operation.
- `store_register_values_to_memory`: Stores a value from a data register into a memory location pointed to by an address register, then calls another routine to perform a similar operation with a different register and address.
- `store_calculated_difference`: Calculates a value based on the comparison of two 32-bit registers and stores the result into multiple memory locations via pointers.
- `calculate_difference_word`: Calculates a value based on the difference between two 32-bit registers and stores the result in multiple memory locations.
- `calculate_and_store_word`: Calculates a 32-bit value by comparing halves of two registers and stores the result in multiple memory locations.
- `store_concatenated_register`: Concatenates two 16-bit halves of a register into a 32-bit value and stores it in a memory location.
- `store_calculated_word`: Calculates a 16-bit value based on comparisons of register halves and stores the result as a 32-bit word into a memory location.
- `store_shifted_register`: Shifts a 32-bit value and stores the result into a memory location pointed to by a global variable.
- `store_concatenated_values_partial`: Stores a 32-bit value into a memory location after a concatenation operation.
- `swap_and_update_tables`: Swaps two values in a table and populates several global variables with incrementing offsets based on a single value.
- `update_global_offsets_4`: Adjusts multiple global variables and memory locations based on comparisons between various global values.
- `update_global_counters`: Sets several global constants and increments a global variable before performing conditional routine calls.
- `update_global_offsets_3`: Clears several global variables and adjusts multiple other global offsets based on their relative values.
- `update_global_offsets_2`: Decrements two global variables, conditionally adjusts two others based on their comparison, and increments a fifth global.
- `update_global_offsets`: Decrements two global variables, sets two other globals to specific constants, and increments a third global.
- `store_concatenated_value_single`: Stores a single 32-bit value into a memory location following a concatenation.
- `initialize_hardware_state_2`: Calls several subroutines to interact with hardware registers and sets multiple global variables to specific values.
- `populate_address_table`: Stores a series of word pairs into a table, where each pair consists of a value and a constant increment.
- `update_offset_table`: Writes specific word values and constant increments into a table of global variables.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 4 into d2, then merges 0 into d6.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 2 into d0, then merges 0 into d4.
- `calculate_interpolated_offsets`: Uses several counters and indices to add values from a table to global variables, updating the counters cyclically.
- `update_checksum_data`: Performs arithmetic operations on two values and stores the results back into global variables.
- `update_pointer_table`: Updates a table of pointers with calculated offsets and resets several global variables when a counter reaches seven.
- `update_color_and_table_pointer`: Updates a hardware color register by copying data from a global variable and performs complex pointer arithmetic to traverse and update nested data tables.
- `traverse_nested_data_tables`: Iterates through a hierarchy of tables by fetching pointers and using them as indices for subsequent lookups.
- `fetch_table_entry_and_update_counters`: Fetches a pointer from a table using an index and an offset, then performs a series of conditional decrements and increments on multiple global counters based on several status flags.
- `decrement_and_store_pointer`: Decrements a global variable with a floor of zero, increments another if a condition is met, and stores a value into a memory location using a pointer.
- `increment_and_store_pointer`: Increments one global variable, decrements another, and stores a value into a memory location using a pointer.
- `update_nested_counters`: Decrements or increments various global counters through a series of nested conditional checks and stores a 32-bit value at a pointer.
- `decrement_and_store_pointer_alt`: Decrements one global variable, decrements another, and stores a value into a memory location using a pointer.
- `update_counters_and_pointer`: Adjusts several global counters based on conditional checks and stores a 32-bit value into a memory location with a bit set.
- `update_state_and_counters`: Increments or decrements several global variables based on a conditional check and then calls another routine.
- `conditionally_decrement_and_store`: Conditionally decrements global variables based on their current values and stores a value into a memory location using a pointer.
- `update_pointer_and_counters`: Decrements several global variables and stores a 32-bit value into a memory location with a bit set.
- `process_joystick_and_blitter`: A twelve-instruction routine that manipulates interrupt registers and updates global variables using data from a table and hardware registers.
- `load_joystick_data`: Cycles through a table of data by incrementing an offset and loading values from the table into several global variables.
- `initialize_joystick_data`: Clears a byte and stores several values derived from a register into global variables.
- `copy_memory_blocks`: Copies data from a source table to a destination, interleaving words from the source with words from a distant offset in the same source.
- `decrement_global_if_nonzero`: The routine checks a global variable and, if it is non-zero, subtracts one from it.
- `execute_blitter_draw`: Sets blitter control and size registers and waits for the hardware to finish the operation.
- `update_global_bitmask`: Calls another routine, then updates a global variable with a constant value and applies a bitwise mask to set a specific bit while clearing others.
- `perform_blitter_loop_large`: Executes a loop three times that configures blitter registers and waits for a specific bit in a global variable to clear.
- `perform_blitter_loop_small`: Executes a loop three times that configures blitter registers with different address increments and waits for a hardware status bit.
- `configure_blitter_transfer`: Sets up the blitter hardware registers with specific control, mode, depth, and size parameters before initiating a transfer.
- `update_entity_attributes`: Iterates through a list of entities to update their properties by copying specific data fields from a source table based on an index.
- `clear_conditional_pointer`: Checks a byte at an offset in a structure and, if it matches specific values, clears a word at the base pointer and stores the original value with high bits set in a related offset.
- `clear_and_flag_pointer`: Clears a word at a pointer location and stores the original value with the high two bits set at an offset within that same structure.
- `flag_conditional_offsets`: Checks if specific offsets in a structure are zero and, if so, clears a word and sets high bits in a related location based on a value match.
- `conditionally_flag_pointer`: Checks if two specific offsets in a structure are zero, then clears the first offset and stores the original value with the high two bits set in the second offset.
- `initialize_level_data_structures`: Checks several global flags to determine if level data structures need initialization, then iteratively calls a subroutine to verify conditions.
- `update_record_flags`: Checks a value at a pointer and conditionally clears it, sets high bits in adjacent memory locations, or updates a global variable based on specific constant matches.
- `modify_record_state_flags`: Checks a value at a pointer and conditionally clears it or sets high bits in adjacent memory locations based on specific constant matches.
- `update_structure_flags_2`: Checks a value at a pointer and conditionally modifies it or sets high bits in adjacent memory locations based on specific constants.
- `write_constant_to_offset`: Writes a specific constant value to a memory location at a fixed offset from a provided pointer.
- `update_structure_flags`: Clears a value at a pointer and conditionally sets a new value with the high bit set at a specific offset based on the original content.
- `process_pointer_flags_2`: Masks a value at a pointer, checks a specific offset for certain constants, and conditionally clears a value while storing it with high bits set.
- `process_pointer_flags`: Evaluates a value against several constants and conditionally clears a word at a pointer while storing the original value with high bits set in an adjacent offset.
- `reset_and_flag_structure`: Clears a global variable, then clears a word at a pointer location and stores the original value with the high bit set at an offset within that structure.
- `process_data_blocks`: A 54-instruction routine that updates an active entry, stores bytes, performs a blitter transfer, copies sixteen 20-byte blocks, and rotates data elements in a table.
- `invoke_data_handler`: This three-instruction routine is called via a pointer located in a data hunk to execute a handler function.
- `process_data_hunk`: An eight-instruction routine called via a handler pointer in data that invokes several subroutines to manage data blocks, memory ranges, and global states.
- `invoke_data_handlers`: Calls a routine to manage data blocks and then calls another routine to reset state based on a conditional flag and a pointer comparison.
- `process_data_hunk_2`: A twenty-instruction routine called via a handler pointer in data that invokes a routine to manage data blocks and memory ranges.
- `process_hunk_data`: A five-instruction routine that calls a data management routine and an unnamed routine.
- `execute_unnamed_routine`: An unnamed routine that calls another unnamed routine.
- `invoke_data_management_subroutines`: Calls two separate routines to update a record based on global identifiers and to manage data blocks or memory ranges.
- `execute_data_handler_2`: A six-instruction routine that invokes several subroutines to manage data blocks and update record flags.
- `execute_data_handler`: A two-instruction routine invoked via a handler pointer in a data structure.
- `execute_conditional_recursion_2`: Invokes a routine to manage data blocks, updates structured data via bitwise operations, and performs recursive calls based on structure contents.
- `dispatch_handler_pointer`: This four-instruction routine is invoked via a handler pointer located within a data block.
- `invoke_conditional_recursion`: A two-instruction routine that calls another routine to manage data blocks and perform recursive calls.
- `compare_and_return`: Compares a value to four and returns.
- `nop_padding_routine`: The routine is empty and contains no executable instructions.
- `select_and_copy_data_bytes`: Checks a global byte value to determine which set of three bytes to copy into a sequence of global variables, then copies a word from a global variable into a final global byte.
- `update_global_state_buffer`: Checks a condition to determine which set of values to copy into a group of global variables, then stores a final value into a global variable via a pointer.
- `increment_control_variable`: Increments a global variable, ensuring it does not match its current value by adding one if they are equal.
- `return_from_subroutine_6`: The routine is empty and performs no operations before returning.
- `update_structured_data`: Checks a signed value in a structure and performs bitwise operations to update or clear words at various pointer locations based on specific conditions.
- `update_flagged_word`: Updates a word at a pointer with a bitwise combination of registers and sets the high bit, or clears the word if a condition is met.
- `set_flagged_word`: Clears a word at a pointer and sets the high bit of a value at another location.
- `combine_and_flag_bits`: Combines bits from two registers and stores the result with the high bit set at a pointer location.
- `clear_word_bits`: Iterates through a block of memory and clears the most significant bit of each word.
- `invoke_data_handler_2`: This four-instruction routine executes a subroutine via a pointer retrieved from a data structure.
- `replace_bytes_in_range`: Iterates backward through a memory range, replacing bytes equal to a specific value with a new value if certain global flags are set.
- `scan_and_replace_bytes`: Iterates backward through a memory range, replacing bytes equal to a specific value with a constant if a global flag is zero.
- `update_value_conditionally`: Checks if a value at a pointer matches specific constants and updates a global variable or the pointer's target accordingly.
- `iterate_and_update_records`: Repeatedly calls two subroutines to validate specific identifiers and conditionally update values or pointers within a data structure.
- `update_pointer_data`: Modifies a value through a pointer and updates several global variables.
- `execute_conditional_recursion`: Checks if specific offsets within a pointer-referenced structure contain a certain byte, and if so, performs ten recursive calls to itself.
- `execute_repeated_calls`: Performs a sequence of ten repeated calls to another routine.
- `execute_indirect_calls_2`: This routine performs a series of repeated, direct calls to the same subroutine.
- `reset_global_state`: Checks several global variables and, if they meet specific criteria, resets them to predefined values.
- `validate_and_reset_state`: Checks if the low byte of a value at a pointer matches specific constants.
- `reset_global_state_2`: Checks several global flags and, if they are not set, performs a series of nested calls and updates multiple global variables with specific constants.
- `reset_state_on_condition`: Checks if a global flag is set before verifying if a value at a pointer matches specific constants.
- `update_active_entry_2`: Checks if the low byte of a value at a pointer matches specific constants and, if so, updates several global variables.
- `execute_empty_sequence`: This routine performs a series of self-referential calls that result in an infinite loop or immediate stack exhaustion.
- `return_from_subroutine_7`: This routine is an empty function that performs no operations and simply returns to the caller.
- `store_two_bytes`: Stores two consecutive bytes from registers into a memory location.
- `store_bytes_and_transfer_data`: Stores two consecutive bytes from registers into a memory location and performs a recursive blitter transfer.
- `adjust_bcd_value`: Adds a byte to a global variable, performs multiple BCD adjustments, and conditionally increments a global value while resetting another.
- `increment_bcd_value`: Increments a global variable using BCD adjustments and conditionally updates other global variables and pointers.
- `process_bcd_increment`: Increments a global variable with BCD adjustments and conditionally executes other routines based on global flags.
- `wait_for_condition`: Performs a nested loop that waits for a global variable to reach a specific value.
- `dispatch_handler_routine`: Invokes two subroutines via pointers retrieved from data.
- `dispatch_handler_pointers`: A four-instruction routine that invokes two subroutines via pointers retrieved from data.
- `decrement_and_check_counter`: Decrements a global variable and resets another global variable to zero if the counter reaches zero, provided a third variable is non-zero and non-negative.
- `execute_subroutine_wrapper`: A wrapper routine that calls another subroutine.
- `decrement_global_if_nonzero_2`: The routine checks a global variable and, if it is non-zero, subtracts one from it.
- `decrement_bcd_counter`: Decrements a global variable using BCD adjustments and conditionally updates a global flag if the value reaches zero.
- `decrement_bcd_counter_3`: Decrements a global variable using BCD adjustments and conditionally updates other values or flags.
- `decrement_bcd_counter_2`: Decrements a global variable using BCD adjustments and sets a global flag if the value reaches zero.
- `decrement_register_to_zero`: Initializes a short integer to its maximum positive value and decrements it in a loop until it reaches negative one.
- `manage_blitter_sequence`: Performs a complex sequence of conditional checks, blitter transfers, memory segment copies, and loops based on various global flags.
- `clear_global_on_condition`: Clears a global byte if a condition returned by another routine is true.
- `format_and_copy_strings`: Constructs a string from various global values and then iterates through a null-terminated list to perform memory transfers.
- `update_animation_frame_counters`: Manages state transitions and frame counters for various animation sequences by incrementing or decrementing specific global counters and adjusting memory offsets.
- `call_blitter_loop_routine`: Calls a routine that executes a three-iteration loop to configure blitter registers and waits for a bit in a global variable to clear.
- `execute_recursive_blitter_transfer`: Sets a global flag and a second global variable before performing multiple recursive calls to another routine that manages blitter configuration and memory segment copying.
- `run_blitter_loop`: Repeatedly calls another routine in a loop until it returns zero.
- `copy_memory_segments_2`: Transfers several discrete chunks of data from a source table to a destination table based on a null-terminated list.
- `fill_memory_pattern`: Fills a series of memory blocks with either all zeros or all ones based on bits in a source word.
- `copy_memory_blocks_2`: Copies specific non-contiguous segments of data from one memory location to another.
- `copy_memory_segments`: Transfers several discrete chunks of data from a source table to a destination table.
- `populate_data_pairs`: Stores four pairs of word values into consecutive global memory locations, where each pair consists of a specific value followed by a constant value.
- `initialize_data_tables`: Sets a series of non-contiguous global memory locations to specific constant values.
- `process_grid_data`: Iterates through a two-dimensional array to transform specific values into new constants based on conditional checks.
- `execute_empty_recursive_calls`: The routine performs a series of self-referential calls to itself before returning.
- `assign_routine_pointer`: A two-instruction routine that stores the address of itself into a global variable.
- `initialize_constant_table`: Writes a sequence of eight word values into a series of global memory addresses.
- `setup_dma_transfer`: Copies sixteen words from a global variable to a hardware register area and configures several DMA-related parameters.
- `update_color_register`: A two-instruction routine that updates a hardware color register by copying data from a global variable.
- `configure_blitter_dma`: Sets hardware registers for a blitter operation and performs a sixteen-word memory copy to a hardware destination.
- `execute_state_loop`: Performs conditional routine calls and updates several global variables, including a loop that polls memory locations until they reach zero.
- `manage_bcd_counter`: Updates a two-byte value using BCD adjustment logic based on comparisons of global variables and status flags.
- `execute_blitter_command`: Configures blitter registers, performs a memory transfer, and waits for the hardware to complete the operation.
- `copy_memory_blocks_3`: Copies sixteen 20-byte blocks from a source table to a destination table using a loop.
- `rotate_data_elements`: Performs a circular shift of data elements within a large table across four iterations.
- `update_player_status_variable`: Sets a global variable based on a bitwise OR operation and a conditional check, then calls another routine.
- `process_string_data`: Iterates through a buffer to write character values based on conditional checks and a loop termination condition.
- `fill_memory_with_constant`: Writes the value 32 to five consecutive memory locations starting from a global variable.
- `process_player_string`: Updates two global values, performs multiple routine calls, and iterates through a null-terminated string while conditionally calling another routine.
- `invert_color_palette`: Inverts sixteen 16-bit values from a table and writes them to the hardware color registers if a flag is not set.
- `apply_color_palette`: Writes sixteen 16-bit values from a table to the hardware color registers when a counter reaches zero.
- `wait_for_audio_completion`: Polls four memory locations until they are zero, performs a delay loop, and then updates several global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_modified_byte`: Returns a byte after performing a bitwise OR with 117.
- `apply_bitmask_to_register`: Performs a bitwise OR operation between the contents of a data register and a constant value, returning the result.
- `return_byte`: Returns the input byte unchanged.
- `bitwise_or_constant`: Performs a bitwise OR operation between a byte from a register and a constant value, then returns the result.
- `return_undefined_byte`: This routine is empty and returns an undefined byte.
- `return_undefined_byte_2`: This routine is empty and returns an undefined byte value.
- `return_undefined_byte_6`: Returns an undefined byte value.
- `return_uninitialized_register`: This routine is empty and returns the current value of a data register without performing any operations.
- `return_modified_byte_2`: Returns a byte value after performing a bitwise OR operation with a constant.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that does not perform any operations.
- `return_uninitialized_register_2`: This routine returns the current value of a data register without performing any operations.
- `modify_bitfield_and_byte`: Clears a specific bit in a byte using a bitmask derived from a register and sets specific bits in another byte via a pointer.
- `return_undefined_byte_3`: This routine is empty and returns an uninitialized byte value.
- `return_undefined_byte_4`: This routine is empty and returns an uninitialized byte.
- `return_register_value`: Returns the byte value currently held in a data register.
- `nop_padding_byte_2`: This routine is an empty instruction used as padding between executable code blocks.
- `return_uninitialized_byte`: This routine returns an uninitialized byte value to its caller.
- `apply_bitmask_to_register_2`: Performs a bitwise OR operation between a byte value in a data register and a constant bitmask.
- `apply_bitmask_to_register_3`: Performs a bitwise OR operation on a byte value from a register with a constant mask.
- `apply_bitmask_to_register_4`: Performs a bitwise OR operation on the contents of a data register with a constant value and returns the result.
- `return_undefined_byte_5`: This is an empty routine that returns an uninitialized byte value.
- `nop_padding_byte_3`: This routine is empty and consists of a single byte of data that does not perform any operations.
- `unknown_single_instruction_routine`: A routine consisting of a single instruction with no available pseudo-code.
- `return_uninitialized_value`: This routine returns an uninitialized value from a register to the caller.
- `or_byte_with_constant`: The routine performs a bitwise OR between the contents of a register and a constant value and returns the result.
- `return_register_value_2`: The routine returns the byte value currently held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_decrement_routine`: Calls a routine that decrements a 16-bit value at a fixed offset from a base pointer.
- `decrement_word_at_offset_2`: Decrements a 16-bit value located at a fixed positive offset from a base pointer.
- `decrement_memory_word`: Calls a routine that decrements a 16-bit value located at a fixed offset from a base pointer register.
- `execute_indirect_call_5`: Calls a routine through a pointer stored at a specific memory location.
- `decrement_memory_word_2`: Calls a routine to decrease a sixteen-bit value at a memory location relative to a base pointer.
- `decrement_memory_word_3`: Calls a routine to decrement a sixteen-bit value located at a fixed offset from a base pointer register.
- `decrement_memory_word_4`: Calls a routine that decrements a sixteen-bit value located at a fixed offset from a base pointer register.
- `decrement_word_and_return_zero`: Decrements a 16-bit value located at a fixed offset from a base pointer and returns zero.
- `increment_word_at_offset`: The routine increments a 16-bit value located at a fixed positive offset from a base pointer register in place and then returns.
- `decrement_word_at_offset`: The routine decrements a 16-bit value located at a fixed positive offset from a base pointer register and then returns a zero value.
- `empty_routine_2`: This routine contains a single instruction and no available pseudo-code.
- `nop_padding_instruction`: This routine consists of a single instruction that performs no operation.
- `execute_single_instruction`: This routine executes exactly one instruction.
- `execute_single_instruction_2`: This routine performs exactly one instruction and then returns.
- `execute_single_instruction_3`: This routine performs a single instruction and then returns.
- `copy_stack_data`: Copies a value from the stack into a register.
- `transfer_stack_value`: Moves a value from the stack to a register.
- `load_stack_to_register`: Loads a value from the stack into a register.
- `move_stack_to_register`: Copies data from the stack into a register.
- `copy_stack_to_register`: This routine copies a value from the stack pointer or an offset or a pointer to a register and then returns.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_single_instruction_4`: This routine consists of a single instruction and is entered from outside the static code flow during execution.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub`: The routine consists of a single instruction that returns to its caller without performing any work, touching no registers, memory, library calls, or hardware.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning.
- `perform_null_operation`: This routine performs no actions and returns immediately to the caller.
- `return_immediately_no_op`: The routine consists of a single instruction that returns to its caller without touching registers, memory, or hardware.
- `return_no_op`: The routine consists of a single return instruction and performs no work: it reads no registers or memory, makes no library calls, touches no hardware registers, and has no referenced text.
- `return_no_op_2`: The routine consists of a single instruction that returns immediately, performing no work.
- `return_no_op_3`: The routine consists of a single return instruction and performs no work: it loads no registers, calls no library functions, touches no hardware registers, and has no referenced text.
