# Game 543 (Nova 9): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00543). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703424 top-level bytes in 88 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 21 of 21 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 435 of 435 named (0 author symbols), 435 of 435 described (20 by rule, 413 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1040 of 1040 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 68064 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9411 of 9425 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1078843 |
| code | 31748 |
| data | 1541895 |
| padding | 21754 |
| unused | 29184 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:nx | 6 | 45 | 2 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install | 10 | 45 | 2 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0 | 10 | 45 | 2 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more | 6 | 45 | 2 |
| run | 2 | 117 | 2 |
| resident | 2 | 117 | 2 |
| sp | 35 | 962 | 54 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim | 12 | 45 | 2 |
| SKID_ROW | 2 | 267 | 8 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title | 17 | 45 | 2 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Nova | 3 | 317 | 13 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover | 10 | 45 | 2 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl | 12 | 45 | 2 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro | 21 | 2802 | 128 |
| Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:amigasnd.drv | 3 | 1820 | 112 |
| Nova 9 (1991)(Dynamix)(Disk 3 of 3).adf:story | 11 | 2652 | 98 |
| Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| ACTORS5.TBC (raw code file, load address unknown) | 1 | 0 | 0 |
| std.TBC (raw code file, load address unknown) | 1 | 0 | 0 |
| ACTORS6.TBC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:nova9.sng: high-entropy data file (7.74 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:tbc/ACTORS5.TBC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:tbc/std.TBC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:tbc/ACTORS6.TBC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- nova 9 (1991)(dynamix)(disk 3 of 3)/Nova 9 (1991)(Dynamix)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 1 of 3)/Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- nova 9 (1991)(dynamix)(disk 2 of 3)/Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 never-loaded region(s), 68064 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro

## Copy protection found

- none found

## Functions


### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:nx

- `initialize_aga_patch`: Executes a loop of function calls and checks a structure value against a specific bitmask to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Install

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_register_until_negative_one`: Decrements a 16-bit register value by one in a loop until the value reaches negative one.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sp0

- `verify_and_swap_memory_constant`: Executes a loop of function calls and checks a value at a calculated offset from a global pointer.
- `decrement_d6_until_negative_one_2`: Decrements the value in register D6 until it reaches negative one and then returns.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:more

- `patch_aga_hardware`: Checks a specific bitmask against a value in a memory structure and, if it matches, updates a global variable and clears a memory location.
- `decrement_register_until_negative_one_3`: Decrements a 16-bit value stored in a register until it reaches negative one and then returns.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block_3`: Copies sixteen words from a source location to a destination location and then calls another routine.

### resident

- `initialize_resident_memory_structure`: Traverses a linked list to find the maximum value in a sequence and then populates a memory structure with a pattern of incrementing values.
- `copy_memory_block_and_call`: Copies sixteen bytes from a memory location to a destination pointed to by the stack and then executes a function via a pointer.

### sp

- `parse_and_store_command_tokens`: Parses a string into tokens by iterating through characters and handling specific control characters and whitespace.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `update_task_status`: Retrieves a task pointer, modifies a value at an offset within that task's structure using a specific constant, and calls an indirect routine.
- `self_calling_guard`: Calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `manage_resource_nodes`: Iterates through a linked list of nodes to find specific entries based on string matching and bitwise conditions.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: The routine calls another routine through a pointer stored in a global variable.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `sum_and_store_inverted`: Iterates through a table of words to calculate a sum, stores the bitwise NOT of that sum in a global variable, and then performs an indirect call.
- `patch_resource_function`: Checks a global variable and calls a routine through a pointer.
- `check_flag_and_copy_word`: The routine calls another routine and tests its return value.
- `dispatch_conditional_pointer_call`: Compares a value against a global variable and a constant, then executes another routine via a pointer if the conditions are met.
- `dispatch_conditional_call`: Checks if a register contains a specific value and, if not, calls another routine via a pointer stored in a global variable.
- `update_interrupt_handler`: Updates a global variable and writes a value to the INTENA register with the interrupt enable bit set, then performs an indirect call to a routine retrieved from a table via an offset.
- `call_indirect_via_offset`: Calls a routine using a pointer retrieved from a global variable with a specific offset.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `execute_empty_routine_3`: This routine is empty and performs no operations.
- `return_input_value_2`: An empty routine that simply returns the value provided in a register.
- `execute_recursive_calls`: Performs a sequence of repeated calls to another routine before returning a value.
- `return_input_value`: The routine receives a value in D0 and returns it unchanged.
- `dispatch_conditional_call_2`: Checks a value in a global variable against a threshold and calls a function through a pointer if the condition is met.
- `copy_data_via_loop`: Copies a sequence of words from one memory location to another using a loop.
- `execute_loop_and_mask_bit`: Executes a routine multiple times through a loop and returns a value from a pointer with the highest bit set.
- `get_masked_value`: Calls an indirect routine and returns a value from a pointer with the highest bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `reset_global_variable`: Sets a global variable to zero and returns its previous value.
- `execute_conditional_logic`: Calls a routine with a value from a pointer, and if the result is non-zero, performs two subsequent routine calls using that same value.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker`: Iterates through an array of words starting from an input pointer.
- `calculate_offset_pointer`: Calculates a new pointer by reading a 16-bit value from an input pointer and applying a bit-shifted offset.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `traverse_linked_list`: Traverses a linked list and executes a function pointer stored in a global variable if certain bitwise conditions are met by the list elements.
- `nop_routine`: The routine is empty and performs no operations.
- `return_from_subroutine_2`: The routine performs no operations and immediately returns to its caller.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:sim

- `execute_initialization_sequence`: Performs a series of function calls and checks a value in memory against a constant to update a global variable.
- `decrement_d6_to_minus_one`: Decrements the value in a 16-bit register until it reaches negative one.

### SKID_ROW

- `parse_amiga_hunk_data`: Iterates through a data stream to identify and process specific hunk type codes.
- `decrement_and_loop`: Decrements a value in a loop until it reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_2`: Performs complex memory manipulation including conditional loops, byte copying, and pointer arithmetic based on bitwise checks.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Processes a stream of bits by performing bitwise shifts and rotations based on a counter and a data source.
- `modify_memory_via_pointer`: Increments a byte at a location pointed to by a register and negates a short integer at an offset from another register.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:title

- `execute_title_initialization`: Executes a loop of function calls and performs a memory check against a constant to update a global variable.
- `decrement_d6_to_negative_one`: Decrements the value in a 16-bit data register until it reaches negative one.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:Nova

- `initialize_game_environment`: Opens required system libraries and initializes several data structures and global pointers.
- `decompress_bitstream_data`: Processes a nested data stream using bit-masking and control codes to expand compressed bytes into a destination buffer.
- `unpack_compressed_data_stream`: Iterates through a nested data structure to decompress and expand a byte stream into a destination buffer.
- `decrement_and_process_table`: Decrements a value at a pointer and, if it reaches zero, performs bitwise transformations on a table before calling a function pointer.
- `process_data_table_2`: Decrements a value at a pointer and, if zero, iterates through a table to perform bitwise comparisons and updates values before calling a function pointer.
- `search_and_call_function_2`: Searches through a byte table for a matching value and then calls a function pointer stored at an offset from a pointer.
- `search_and_call_function`: Performs a bitwise search through a table to find a matching value and then calls a function pointer stored at a specific offset.
- `check_available_memory`: Calls a library function to check available memory and executes two other routines if the returned value is below a threshold.
- `trigger_memory_error_dialog`: Sets a value in a global variable and executes two functions via pointers retrieved from a global variable.
- `execute_library_calls`: Calls two functions using pointers retrieved from a global variable.
- `copy_memory_block`: Copies ten words of data from a source memory location to a destination memory location.
- `calculate_offset_address`: Calculates a value using multiplication and addition involving three input parameters and a global constant.
- `copy_ten_words`: Copies ten words of data from a source memory location to a destination memory location using a loop.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:gameover

- `execute_sequence_and_swap_constant`: Executes a series of function calls and a loop containing function calls, then checks a value at an offset from a pointer derived from a global variable; if that value matches a specific constant, it swaps the value with a stored constant and clears the original source.
- `decrement_d6_until_negative_one`: Decrements the value in a data register until it reaches negative one and then returns.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:ttl

- `check_memory_constant`: Performs a series of function calls and checks a memory location for a specific constant to swap a value.
- `decrement_register_until_negative_one_2`: Decrements a 16-bit register value by one repeatedly until the value reaches negative one.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:intro

- `initialize_system_hardware_and_resources`: Configures the Amiga bitplane, audio, and DMA hardware while initializing resource interfaces and interrupt vectors.
- `call_and_increment_result`: Calls a function via a pointer from a provided base address and adds eight to the returned value if it is non-zero.
- `conditionally_call_function`: Calls a function via a pointer from a provided base address if a specific register is non-zero.
- `call_recursive_routines`: The routine calls another routine three times in succession.
- `configure_blt_hardware`: Sets various hardware registers for bitplane transfers and iterates through a loop to populate a table with memory addresses.
- `execute_nested_calls`: Performs a series of conditional calls to another routine based on its return value.
- `loop_and_call_routine`: Executes a loop that calls another routine multiple times, with an additional call performed after the loop completes.
- `execute_dispatch_table_callbacks`: Iterates through a table of data, performing various conditional checks and calling multiple subroutines based on the values found.
- `wait_for_hardware_status_2`: Repeatedly calls another routine and polls a global variable until specific bits match a constant value.
- `process_input_queue`: Repeatedly removes a word from a table and executes a function pointer if the value matches a specific address.
- `call_audio_and_accumulation_routines`: Calls a routine to update hardware audio registers and then calls a routine to accumulate table values into a destination buffer.
- `call_audio_and_accumulation_routines_2`: Calls a routine to update hardware audio registers and then calls a routine to accumulate values from a table into a destination buffer.
- `call_subroutines_recursively`: The routine calls itself twice before returning.
- `update_audio_registers`: Iterates through a table to find a zero byte, increments it, and writes several values to hardware audio registers and DMA/interrupt control registers.
- `wait_for_zero_byte`: Enters a loop that continues as long as the byte at the location specified by a register is non-zero.
- `set_interrupt_and_call_functions`: Updates several global variables and calls a function via a pointer three times in a loop.
- `configure_interrupt_and_dma`: Sets an interrupt vector and modifies the interrupt enable and DMA control registers.
- `decrement_and_halt_80`: Decrements a byte through a pointer and enters an infinite loop if the value reaches zero, otherwise sets a global variable to a specific value.
- `decrement_byte_and_halt_on_zero`: Decrements a byte through a pointer and checks if the result is zero.
- `decrement_and_check_zero`: Decrements a byte through a pointer and checks if the result is zero.
- `decrement_and_halt`: Decrements a byte through a pointer and enters an infinite loop if the resulting value is zero, while writing to a hardware register.
- `accumulate_table_values`: Reads values from a table using offsets derived from a source buffer and stores the accumulated sums into a destination buffer.
- `parse_data_header_and_recurse`: Validates a memory block for a specific magic signature and non-zero length before storing header fields into global variables.
- `write_words_to_table`: Waits for a specific bitmask in a global variable and then writes two words to a table using an index derived from a register.
- `initialize_resource_interface`: Checks if a global variable is zero, then retrieves a library interface from a pointer in a global variable to execute a sequence of functions.
- `validate_data_structure`: Calls another routine and checks a byte from an offset in the returned pointer.
- `execute_callback_chain_2`: Executes a function via a pointer from a global variable and performs conditional function calls based on the result.
- `copy_data_block_2`: Checks if a pointer is non-zero and then copies a sequence of longwords from that pointer into a global variable.
- `store_longword_to_table`: Waits for a condition in a global variable and then stores a 32-bit value into a table as two separate words.
- `release_resource_interface`: Checks if a global variable is non-zero and, if so, executes two functions through offsets of that variable before clearing the global variable.
- `copy_string_and_search_pattern`: Copies a null-terminated string into a global buffer and then scans a memory region for a specific sequence of two words.
- `pack_bitfield_into_word`: Constructs a 32-bit value by performing bitwise shifts and logical operations on three input registers and stores the result at a pointer address.
- `initialize_system_resources`: Performs a series of complex library calls and hardware register manipulations involving stack operations and conditional branching.
- `find_linked_list_element`: Traverses a linked list of longwords by following offsets until a specific constant is matched.
- `pop_word_from_queue`: Removes a word from a table and shifts the remaining elements forward if the table is not empty.
- `rotate_color_components`: Iterates through a source color table and applies a modular arithmetic rotation to each individual color component.
- `disable_interrupt_handler`: Checks if a global flag is set, then clears that flag, sets a second global variable to a maximum value, and executes a routine via a function pointer.
- `wait_for_hardware_status`: Sets a global variable to a specific value based on a register, loops until a bitmask condition is met, and then clears three memory locations and the global variable.
- `wait_and_set_blitter_config`: Polls a bit in a global variable before writing several configuration values to memory locations used for hardware setup.
- `update_dma_pointers`: Calculates a series of memory addresses based on an offset and a loop counter, then stores these resulting values into a sequence of global variables.
- `copy_and_obfuscate_data`: Copies a block of data into global variables and performs a bitwise transformation on a sequence of values.
- `populate_graphics_buffer`: Allocates a destination buffer and performs a nested loop to copy large blocks of data from source tables.
- `copy_masked_data`: Iterates through a source buffer and copies bytes to a destination only if a bitwise comparison with a mask table succeeds.
- `write_table_values`: Waits for a condition in a global variable to be met, then writes two specific words into a table.
- `write_long_to_table`: Waits for a condition in a global variable to be met, then stores a 32-bit value into a table as two words.
- `modify_memory_word`: Performs bitwise manipulations on input values and stores the resulting 32-bit value at a pointer address.
- `reset_hardware_registers`: Sets a global variable based on a register value, waits for a condition, and then clears several memory locations.
- `copy_string_and_search`: Copies a null-terminated string to a global variable and then searches a memory area for a specific pattern of words.
- `sync_object_data_table`: Checks a condition against a global variable before copying a sequence of longwords from a memory location into a global table and updating a global pointer.
- `rotate_color_palette_data`: Copies a source table of color data to a destination buffer while applying a modular arithmetic rotation to each color component.
- `apply_color_palette_rotation`: Copies a source table of color data to a destination buffer while applying a modular arithmetic rotation to each color component.
- `update_scrolling_parameters`: Calculates new horizontal and vertical scrolling offsets based on a global variable and clamps them within defined minimum and maximum bounds.
- `update_object_position_and_state`: Calculates new coordinate values for an object based on velocity and previous positions, applying clamping logic to keep values within defined bounds.
- `update_mouse_cursor_position`: Calculates new cursor coordinates by computing the difference between current and previous input values, applies boundary clamping, and updates a data structure.
- `update_object_state`: Updates a structure of values using arithmetic operations and conditional bounds checking, modifies several global variables, and executes a callback.
- `update_object_state_and_execute_callback`: Updates a structure of values using arithmetic operations and conditional bounds checking, modifies several global variables, and executes a function pointer stored in a table.
- `update_state_and_callback`: Updates a table with calculated values, modifies several global variables, and executes a function pointer from a table.
- `update_state_and_call_callback`: Calculates values based on a table, updates several global variables, and executes a function pointer from a table.
- `copy_data_with_mask_check`: Iterates through a source buffer and copies bytes to a destination only if a bitwise comparison with a mask table succeeds, potentially calling another routine during the process.
- `copy_and_search_data`: Copies a block of seventeen bytes from one location to another and then performs a conditional search through a table to find a matching byte.
- `copy_data_with_mask`: Copies a sequence of bytes from a source buffer to a destination buffer, conditionally performing the copy based on a bitwise comparison between a mask table and a control byte array.
- `manage_library_and_interrupts`: Interacts with system libraries to manage resources and manipulates hardware interrupt registers.
- `configure_bitplane_and_copper_data`: Calculates bitplane dimensions and color palette parameters based on a bitmask, then populates a table with interleaved copper instruction data.
- `execute_callback_chain`: Executes a function pointer from a global structure, potentially calling another function if the result is non-zero, and updates a global pointer from a specific offset.
- `execute_callback_pointers_and_reset`: Checks several global variables for non-zero values, executes functions located at an offset from a global pointer, and resets both the global variables and several other global variables to zero.
- `initialize_display_parameters`: Calculates display dimensions, bitplane counts, and color palette settings based on a bitmask retrieved from a function call.
- `swap_and_copy_pointers`: Swaps two global variables and copies several consecutive longwords from a memory location to a destination buffer.
- `expand_and_copy_data_block`: Swaps two global variables and then performs a series of arithmetic operations on a value to generate a sequence of data.
- `copy_data_block`: Swaps two values in a global memory location and then performs a series of word-sized transfers from a source table to a destination buffer on the stack.
- `update_dma_buffer_pointers`: Calculates a series of memory addresses based on an offset and a loop counter, then stores these resulting values into a sequence of global memory locations.
- `update_vertical_position`: Performs a loop to calculate a series of words based on an input value and stores them in a table, while interacting with a hardware register.
- `return_zero`: Returns a constant value of zero.
- `validate_and_process_data_header`: Checks a memory block for a specific magic signature and non-zero length before performing a series of recursive calls and data assignments.
- `unpack_compressed_data_stream_2`: Iterates through a table of offsets to locate data blocks and performs a custom decompression routine.
- `traverse_and_decrement_linked_list`: Iterates through a linked list by decrementing values at pointer locations and following next-node offsets.
- `initialize_interrupt_server`: Sets a global variable to a specific value and calls a function using a pointer from a global variable.
- `remove_interrupt_server`: Clears a global variable and sets another to a specific value before calling a function via a global pointer.
- `populate_coordinate_table`: Calculates a series of values based on input register data and populates a table in memory using a decrementing counter.
- `clear_data_register_bits`: Performs a bitwise OR operation on a data register with zero to clear its bits.
- `perform_blitter_transfer`: Calculates bitwise values and memory addresses to perform a series of hardware blitter operations using a loop.
- `setup_blitter_registers`: Writes several values to hardware registers to prepare for a blitter operation.
- `validate_structure_element`: Checks a byte within a data structure retrieved from another routine.
- `traverse_offset_list_2`: Iterates through a list of memory locations by adding an offset found within each element until a match is found.
- `find_linked_list_element_2`: Traverses a linked list of longwords by following pointers stored within the nodes.
- `update_dma_buffers`: Calculates a sequence of memory addresses and stores them into a series of global variables.
- `obfuscate_data_block`: Copies a sequence of longwords into global variables and applies a bitwise transformation to a data block.
- `search_linked_list`: Traverses a linked list of longwords, checking for a specific constant value, and returns a pointer to the element following the match.
- `traverse_offset_list`: Iterates through a list of structures using an offset stored within each element to find a matching longword.
- `copy_and_obfuscate_data_block`: Copies a sequence of longwords from a pointer into a series of global variables, performs a bitwise transformation on a data block using a loop, and then overwrites several global variables with the previously stored values.
- `validate_status_byte`: Calls another routine to check a value at an offset and performs conditional loops and returns based on that byte's content.
- `process_data_structure`: Traverses a data structure and performs conditional operations based on values found within the elements.
- `expand_data_blocks`: Processes a multi-dimensional data structure by copying and expanding bytes based on sign-extended values and length counters.
- `copy_data_blocks`: Performs a nested loop to copy blocks of data from a source to a destination based on predefined dimensions.
- `copy_data_sequence`: Checks a condition against a global variable before copying a sequence of longwords from a memory location into a global table.
- `calculate_divisor_limit`: Performs arithmetic comparisons on input values and executes a loop to find a specific divisor based on global constants.
- `assign_global_pointers`: Copies values from two global variables into two other global variables.
- `copy_graphics_buffer_blocks`: Allocates memory and performs a nested loop to copy large blocks of data from source tables to a destination buffer.
- `generate_sequence_table`: Calculates a series of values using arithmetic operations and stores the resulting sequence into several global variables.
- `calculate_sequence_values`: Computes a sequence of values through iterative addition and stores the results into a series of global memory locations.
- `clip_and_update_blitter_parameters`: Clips a coordinate pair against a bounding box and updates Blitter control registers and memory pointers based on the resulting offsets.
- `configure_blt_line_drawing`: Calculates and writes parameters to the Blitter registers to perform line drawing or shape filling.
- `configure_blitter_parameters`: Waits for a bit in a global variable to clear before writing several values to memory locations used for hardware configuration.
- `apply_bitmask_to_memory_2`: Performs bitwise AND and OR operations on a memory buffer using a bitmask.
- `apply_bitmask_to_memory`: Performs bitwise AND and OR operations on a memory buffer using a bitmask.
- `extract_bitfield_value`: Checks if a set of coordinates is within a specific range before calculating a memory offset.
- `return_constant_value`: Returns a constant value of negative one.
- `calculate_line_raster_parameters`: Calculates slope, step increments, and coordinate offsets for line drawing.
- `generate_line_raster_parameters`: Calculates and stores incremental step values, slope constants, and coordinate offsets for drawing a line segment.
- `calculate_line_segment_parameters`: Calculates step increments, slope-related constants, and coordinate offsets for a line segment.
- `generate_line_segment_buffer`: Calculates the step increments and pixel data for a line segment based on start and end coordinates.
- `rasterize_line_segment_buffer`: Calculates the length and step increments for a line segment and populates a memory buffer with interpolated pixel data and pattern information.
- `render_3d_line_segment`: Calculates pixel coordinates for a line segment using linear interpolation and writes color and pattern data into a memory buffer.
- `calculate_and_render_line`: Performs linear interpolation to determine pixel coordinates and then writes color and pattern data into a memory buffer.
- `rasterize_transformed_polygon`: Calculates vertex positions via linear interpolation and populates a buffer with scanline data.
- `calculate_polygon_vertices`: Computes the intersection points of line segments using linear interpolation to determine the vertices of a transformed shape.
- `render_scaled_bitplane_graphics`: Calculates scaling parameters and performs bitplane-aware blitter transfers to render transformed graphics.
- `apply_bitmask`: Calls another routine to perform bitwise operations on memory.
- `setup_blt_parameters`: Calculates transfer parameters and configures bitplane transfer hardware registers within a loop.
- `configure_blt_transfer`: Calculates parameters and updates several hardware registers to perform a Blitter transfer.
- `process_keyboard_input`: Reads from a serial register and stores processed bit-manipulated data into a table.
- `open_resource_and_initialize`: Opens a resource and uses a pointer from a global variable to call several functions through a retrieved library interface.
- `close_resource_and_clear`: Calls two functions via a pointer retrieved from a global variable and then clears that global variable.
- `fetch_byte_from_table`: Returns a word composed of a high byte and a byte fetched from a table using an index.
- `pop_queue_element`: Removes a word from a table and shifts the remaining elements forward.
- `setup_blitter_parameters`: Calculates bitwise values and writes to hardware registers to prepare a Blitter operation.
- `clear_data_register_byte`: Performs a bitwise OR operation on a byte of a data register with zero.
- `execute_blitter_operation`: Sets multiple Blitter mode and control registers to perform a hardware graphics transfer.
- `initialize_blitter_registers`: Writes specific values to Blitter registers to prepare for a transfer.

### Nova 9 (1991)(Dynamix)(Disk 2 of 3).adf:amigasnd.drv

- `execute_indirect_call`: Performs an indirect function call using a pointer derived from a global variable and a register value.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `check_status_match`: Searches through a table for a matching pair of bytes and sets or clears bytes in a structure based on the result.
- `set_status_bytes`: Sets one byte to one and another byte to zero at specific offsets within a structure.
- `update_status_flags`: Compares two bytes against values in two different tables and sets or clears bytes in a global structure based on the result.
- `clear_matching_entries`: Compares two byte values against a sequence of four entries in a table and stores a zero at a specific offset if both bytes match.
- `search_and_clear_byte_2`: Iterates through a sequence of four entries to find a match for two byte values and stores a zero at a specific offset if a match is found.
- `search_and_clear_byte`: Compares two byte values against a sequence in two separate memory locations.
- `update_audio_command_state`: Processes a command byte to either update a byte in a table, store a value at a calculated offset, or iterate through a four-entry sequence to find a matching pair.
- `process_audio_command`: Evaluates a command byte to either store a byte in a table at a calculated offset or search a sequence of four entries for a matching pair.
- `scan_audio_table`: Updates a byte in a table based on a specific value or scans a sequence of four entries to find a match where a secondary byte is not negative.
- `conditionally_scan_table`: Checks if a byte matches a value in a table while ensuring a secondary byte at a specific offset is not negative.
- `search_audio_data`: Compares a character against a sequence of bytes in a global structure and updates a local variable if a match is found.
- `store_byte_at_offset`: Stores a byte into a memory location calculated from a base address, an offset, and a word value.
- `update_audio_register`: Calculates a value based on input registers and stores it in a memory location, then iterates through a four-byte sequence to check for matching values.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `get_and_set_byte`: Returns a byte from a memory location and, if the input byte is not negative one, updates that location with the input byte.
- `get_and_set_word`: Returns a word from a memory location and, if the input word is not negative thirty-two thousand seven hundred sixty-eight, updates that location with the input word.
- `update_byte_via_pointer`: Reads a byte from a memory location and conditionally overwrites it with a value from a register if that register does not contain a specific negative value.
- `update_index_table_entry`: Iterates through a data table to find a matching byte and updates a value at a calculated offset within a structure.
- `update_index_table`: Iterates through a table to find a matching byte and updates a value at a calculated offset within a structure.
- `compare_and_recursively_update_audio`: Compares a byte value against a four-byte sequence in a table and calculates a new value based on the number of matches.
- `call_routine_twice`: The routine calls another routine twice in succession and then returns.
- `process_byte_stream_2`: Counts occurrences of a specific byte in a buffer and subtracts that count from values in another memory area.
- `subtract_and_store_byte`: Subtracts two values and stores the result in a memory location pointed to by a register.
- `process_byte_stream`: Iterates through a memory buffer, skipping zero bytes and subtracting values from a register until a condition is met, then writes the remainder back to the buffer.
- `modify_audio_channel`: Searches for a specific byte value in a structure, updates it, and increments a value at a calculated memory offset.
- `update_sample_envelope_values`: Compares a byte value against a table and either decrements the stored value or resets it to zero.
- `update_audio_sample_indices_2`: Iterates through a small table to match specific byte values and updates corresponding entries in a data structure.
- `update_audio_sample_indices`: Iterates through a small table to match specific byte values and updates corresponding entries in a data structure.
- `process_audio_indices`: Uses a loop to traverse a table and update bytes in a structure based on matching values.
- `execute_empty_routine`: This routine consists of ten instructions but performs no observable operation, as it does not modify any registers, memory, or hardware state before returning.
- `update_status_table`: Stores a byte at a calculated offset and, if that byte is zero, iterates through a table to update specific flags based on a matching index.
- `initialize_audio_sample_event`: Clears several fields in a global structure associated with a specific index and searches a sample table for a matching time range.
- `register_audio_event`: Searches a table for a specific value, then populates a global structure with various data points including pointers and calculated offsets.
- `configure_audio_parameter`: Writes multiple values to a structure based on an index and updates a specific byte in a table after calling another routine.
- `call_audio_reset`: Calls a routine that clears specific memory locations and resets the audio hardware registers.
- `clear_audio_channel_data`: Resets several memory locations and offsets within a global structure to zero.
- `select_audio_parameter_and_write`: Determines a 16-bit value based on a stack-based condition and combines it with a 24-bit value to be returned.
- `return_stack_value`: Returns a 24-bit value formed by combining a constant with a 4-byte field taken from a 32-bit value located on the stack at a fixed offset.
- `update_audio_voice_counters`: Iterates through a sequence of four entries and increments specific values in a table based on the contents of a structure.
- `process_audio_envelope_transitions`: Manages the state machine for an audio channel's volume envelope by incrementing or decrementing counters and updating volume levels using lookup tables.
- `update_audio_envelope_state`: Updates an audio channel's volume and envelope state by incrementing or decrementing a counter and applying values from lookup tables.
- `add_audio_data_value`: Adds a value from a table to a byte in a structure and clamps the result to a maximum value.
- `process_audio_envelope`: Modifies a byte in a structure using values from lookup tables and updates a status byte based on conditional checks.
- `subtract_audio_data_value`: Subtracts a value from a table from a byte in a structure and clamps the result to a minimum value.
- `transform_audio_sample_stream`: Iterates through a data stream to perform byte-wise subtraction and replacement using values retrieved from lookup tables.
- `decrement_audio_envelope`: Decrements a byte in a structure and performs a table lookup to update another value if a threshold is reached.
- `modify_audio_data`: Subtracts a value from a byte in a structure using an offset from another table and calls itself recursively if a condition is met.
- `compute_volume_presence_flag`: Calculates a product of three bytes retrieved from tables and a scaled short value, then stores a boolean flag in a global variable based on whether the result is non-zero and a specific condition is met.
- `calculate_volume_scaling_3`: Calculates a value based on the product of multiple inputs and stores the result in a global variable.
- `calculate_volume_scaling_2`: Calculates a scaled volume value by multiplying multiple inputs and dividing by a constant, then stores the result in a memory location.
- `update_volume_scaling_byte`: Calculates a scaled volume value by multiplying two inputs and dividing by a constant, then stores the result in a memory location.
- `calculate_volume_scaling`: Calculates a value based on the product of two inputs and stores the result in a global variable.
- `populate_audio_sequence_2`: Populates a structure and a secondary table with data extracted from a pointer-based sequence.
- `populate_audio_sequence`: Populates a structure and a secondary table with data extracted from a pointer-based sequence.
- `initialize_audio_sequence`: Sets a flag and populates a structure with data from a pointer-based sequence before calling another routine.
- `reset_audio_hardware_state`: Calls a routine to reset the audio hardware and returns a value from a memory location via a pointer.
- `check_handler_table`: Compares a value against a table of four integers and calls another routine if a match is found.
- `recursive_call`: The routine contains no instructions and immediately returns.
- `update_channel_state`: Searches a table for a matching integer, then writes a byte and a word to specific offsets in a structure.
- `extract_and_clear_byte`: Writes a byte to a global structure, updates a word at a calculated offset, and retrieves a byte from a table before clearing it.
- `clear_and_return_byte`: Sets a word at a fixed offset to one, retrieves a byte from a table using an index, clears that byte in the table, and returns the byte shifted left by sixteen bits.
- `fetch_audio_parameter`: Retrieves a value from one of several tables based on a mode and index provided in a register.
- `fetch_audio_table_value`: Returns a byte from one of several tables based on a mode value and an index.
- `select_sequence_data`: Selects a byte from one of several tables based on a mode value and an index.
- `select_sequence_data_offset`: Selects a byte from one of several tables based on a mode value and an index.
- `fetch_sequence_index`: Retrieves a value from a table based on a byte index or searches for a specific sequence in a table.
- `search_sequence_index`: Searches a table for a specific four-byte sequence or retrieves a value from a table using a byte as an index.
- `find_sequence_index`: Searches a table for a sequence of four matching bytes and returns a value derived from the index found.
- `return_constant_value_2`: Returns a constant 16-bit value.
- `update_buffer_pointers`: Stores a value in one memory location and adds a base address to four other memory locations.
- `get_concatenated_data`: Concatenates two 16-bit values from a table into a single 32-bit result.
- `initialize_audio_structures`: Allocates memory and populates a series of offsets with a base address, then initializes several interrupt handler structures with specific values and function pointers.
- `initialize_audio_device`: Opens the audio device and initializes a series of interrupt handler structures with specific parameters and function pointers.
- `initialize_audio_interrupt_handlers`: Initializes a table of structures with specific values and function pointers, then writes to a hardware register to enable interrupts.
- `release_audio_resources`: Performs cleanup operations including calling functions via pointers and interacting with system-level resources.
- `reset_audio_hardware`: Clears specific memory locations and resets the audio hardware register.
- `update_audio_registers_2`: Loads a byte and a word from a table into specific hardware registers and clears a third register.
- `execute_conditional_delay_loop`: Checks a value in a table and, if zero, executes a loop that decrements a counter derived from multiple memory lookups and arithmetic offsets.
- `delay_loop_execution`: Executes a loop that decrements a counter based on values retrieved from multiple tables.
- `return_immediately_3`: This routine performs no operations and returns control to the caller.
- `perform_no_operation`: This routine executes a series of instructions that result in no functional change to the processor state before returning to the caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning to the caller.
- `write_audio_register`: Writes a 16-bit value to a hardware register.
- `write_audio_register_alt`: Writes a 16-bit value to the audio hardware register.
- `decrement_and_update_audio`: Decrements a byte in a table and, if it reaches a specific value, updates hardware registers with values from a table.
- `update_sample_buffer`: Copies a large block of sample data from a source buffer to a destination buffer based on an index and offset.
- `update_audio_hardware`: updates a hardware register with a value from a table and modifies a memory location based on a calculated index.
- `store_table_word`: Stores a 16-bit word from a table into a memory location.
- `update_voice_sample_buffer`: Copies a large block of sample data from a source buffer to a destination buffer based on an index.
- `update_audio_hardware_alt`: updates a hardware register with a value from a table and clears a flag in a memory location.
- `store_table_word_2`: Stores a 16-bit word from a table into a memory location via a pointer.
- `validate_and_set_global_byte`: Checks the values of two consecutive global variables and recursively calls itself if specific conditions are met, otherwise sets a global variable to a constant value.
- `set_sample_flag`: Checks a byte via a pointer and then stores a constant value into a global variable.
- `set_sample_playback_flag`: Checks a byte via a pointer and then stores a constant value into a global variable.
- `set_interrupt_request`: Calls another routine and then sets a bit in a hardware register.
- `reset_audio_state_flag`: Evaluates a series of global variables through conditional checks and either recurses or sets a global variable to a constant value.
- `update_audio_state_flag`: Sets a global variable to a constant value and conditionally writes a byte to a memory location pointed to by a register.
- `update_audio_sample_buffer`: Copies a block of sample data to a destination buffer and sets a flag in a global variable to signal an audio interrupt request.
- `set_audio_interrupt_flag_high`: The routine calls a helper to copy sound samples and then sets a different bit in a global audio state variable to signal an interrupt request.
- `set_audio_flag_low`: Checks a series of global variables and writes a constant value to a global variable.
- `update_audio_sample_status`: Checks two memory locations for null values and writes a byte to one of them if it is non-zero.
- `update_audio_sample_buffer_2`: Updates a sample buffer by copying data and then triggers an audio interrupt request.
- `set_audio_interrupt_request`: The routine writes a constant to the INTREQ register and then calls another routine.
- `update_audio_state_flag_2`: Checks a global variable and performs recursive calls based on its value before setting a global variable to a constant value.
- `reset_interrupt_flag`: Checks a byte via a pointer and then writes a constant value to a global variable.
- `update_audio_sample_state`: Checks a byte via a pointer and then writes a constant value to a global variable.
- `reset_audio_interrupt_flag`: The routine clears a hardware interrupt request flag by writing a constant to a global variable that maps to the INTREQ register after calling another routine.
- `write_hardware_constants`: Writes specific constant values to two hardware registers.
- `modify_data_table`: Iterates through a table of pointers to find specific entries and performs arithmetic on a range of bytes within a memory block.
- `process_data_table`: iterates through a table to find specific entries and performs arithmetic on bytes within a memory range.

### Nova 9 (1991)(Dynamix)(Disk 3 of 3).adf:story

- `initialize_resource_loading_sequence`: Checks a magic constant in a data stream and initiates a deeply nested series of conditional calls to load, validate, and decompress animation and graphics resources.
- `execute_conditional_routine_chain`: Calls another routine and sets a global variable to one if the return value is negative, then calls that same routine a second time.
- `poll_hardware_input`: Repeatedly calls another routine to check values from a hardware register and performs table operations based on the results.
- `process_input_loop`: Iterates through a loop checking a hardware register and compares a value against a global variable until a specific condition is met.
- `load_address_from_table`: calculates an offset from a local variable to index into a table and stores the resulting value into a global variable.
- `execute_empty_routine_2`: The routine is empty and performs no operations before returning.
- `copy_table_data_2`: Copies a block of data from a source table to two different destination memory locations.
- `copy_table_data`: Copies a block of data from a source table to two different destination memory locations.
- `increment_global_byte`: Increments a global byte by one.
- `increment_global_word`: Increments a 16-bit global variable by one.
- `loop_until_null`: Repeatedly calls another routine until it returns a zero value.
- `swap_boundary_values`: Assigns the value of one global variable to another, calls a routine, and then repeats the process using a different global variable.
- `process_magic_header`: Checks a memory pointer for a specific magic constant and updates several global variables and status flags through a conditional chain.
- `configure_video_display_parameters_2`: Calculates screen dimensions, bitplane counts, and memory requirements based on a bitmask.
- `initialize_resource_pointer`: Sets a global variable to zero and attempts to populate it by calling a function through a pointer stored in a global variable.
- `check_magic_constant`: compares a value at a pointer to a specific constant and calls a function pointer if they match.
- `decompress_structured_data_stream`: Parses a control-coded data stream to perform variable-length decoding.
- `validate_and_reset_resource_pointers`: Checks a global pointer through an indirect call and conditionally executes further indirect calls based on the result.
- `clear_and_call_handler`: checks if a global variable is non-zero, sets it to zero, and executes a function pointer from a global structure.
- `copy_data_block_3`: Copies a sequence of longwords from a source pointer into a global variable while waiting for a specific bitmask condition.
- `initialize_global_state`: sets a global variable to negative one, decrements other global variables, and executes a function pointer from a global structure.
- `release_resource_handle`: Checks if a global pointer is non-zero and, if so, executes two functions via that pointer before clearing the pointer.
- `cleanup_graphics_resources`: Releases allocated memory and system resources by invoking function pointers from a global structure.
- `update_clipping_rectangle`: Adjusts a rectangular boundary by comparing input coordinates against predefined limits and recalculating dimensions.
- `pop_table_element_2`: decrements a global counter and shifts elements in a table to remove the first entry.
- `reset_interrupt_state`: Clears a global variable and sets another to a specific value before calling a routine via a pointer in a global variable.
- `update_dma_parameters`: swaps two global variables and copies a series of calculated longwords into a sequence of global variables.
- `validate_resource_type`: Checks various global values and function return codes to determine if a resource matches specific signature constants.
- `perform_conditional_copy`: Copies bytes from a source to a destination while checking against a global table and potentially calling another routine if a bitwise match is found.
- `sync_display_parameters`: Synchronizes a series of longwords from a source pointer into a global variable while waiting for a specific bitmask condition.
- `compare_and_copy_data`: Compares bytes from a source table against a bitmask stored in a global variable.
- `process_conditional_copy`: Copies bytes from a source to a destination while performing bitwise comparisons against a global table and potentially calling another routine via a jump table.
- `copy_and_process_data`: Copies three words from one memory location to another and then enters a loop that performs bitwise comparisons and conditional byte transfers.
- `decompress_custom_data_stream`: Parses a structured data stream containing control codes to perform variable-length decoding.
- `copy_memory_block_2`: Calls another routine twice to obtain two memory addresses and then copies ten words from the first address to the second address.
- `calculate_offset_address_2`: Performs arithmetic on input registers to calculate and return a specific memory address.
- `copy_ten_words_2`: Obtains two memory addresses from another routine and copies ten words from the first address to the second.
- `manage_library_resources`: Interacts with graphics and intuition libraries to configure display parameters and clean up allocated resources while manipulating interrupt enable registers.
- `configure_video_display_parameters`: Calculates display dimensions, bitplane counts, and color palette values based on a bitmask.
- `cleanup_library_resources`: Executes a sequence of function calls to check global pointers, frees associated memory, and clears those pointers while interacting with intuition.library and hardware registers.
- `clear_memory_pointers`: Checks several global pointers for non-zero values, calls a function to free the associated memory, and then clears those pointers and several other global variables.
- `initialize_video_buffer_structures`: Calculates display dimensions and bitplane counts based on a bitmask, then populates a table with memory addresses for video buffers.
- `swap_and_copy_pointers_2`: Swaps two values in a table and then copies a series of longwords from a code location into several global variables.
- `swap_and_copy_data`: Swaps two values in a table and then copies a series of calculated longwords into a set of global variables.
- `copy_indexed_data_block`: Swaps two values in a memory location and then performs a series of arithmetic operations to expand a data sequence.
- `modify_dma_control`: Swaps two values in a table and then copies a series of calculated longwords into a set of global variables while touching the DMA control register.
- `copy_calculated_longwords`: Calculates a series of longwords based on an input and stores them into a set of global variables while updating the video port register.
- `update_mouse_input_state`: Reads mouse movement from a hardware port and updates clamped coordinate values within a data structure.
- `update_clamped_parameters`: Clamps values against a table, performs arithmetic to derive new values, and stores the results into a series of memory locations.
- `process_clamped_values`: Clamps input values against a table, performs arithmetic to derive new values, and executes a callback function via a pointer in a table.
- `clamp_value_and_execute_callback`: Clamps a value between two bounds stored in a table and performs arithmetic to store the result and a derived byte into a memory location.
- `update_table_and_execute_callback`: Updates a table entry with a clamped value and performs arithmetic to store derived bytes into a memory location.
- `initialize_interrupt_handler`: Sets several global variables to zero or decremented values and executes a function pointer from a global structure.
- `remove_interrupt_handler`: Resets a global variable to zero and executes a function pointer from a global structure.
- `clip_and_fill_rectangle`: Clips a rectangular area against defined boundaries and uses the blitter to perform a patterned area fill.
- `update_blitter_parameters_2`: Calculates blitter and DMA control values based on global variables and updates hardware registers.
- `update_blitter_parameters`: Calculates values based on several global variables and updates multiple hardware registers including the blitter and DMA control.
- `apply_bitmask_to_buffer`: Performs bitwise operations on a memory buffer using a bitmask derived from a global variable.
- `test_bit_pattern`: Checks a bitmask within a table at a calculated offset.
- `return_all_ones`: The routine returns a value consisting of all bits set to one.
- `execute_blitter_area_fills`: Configures and triggers the Amiga Blitter to perform area fills or block transfers using calculated dimensions and bitmask patterns.
- `process_keyboard_input_2`: Reads a byte from the keyboard serial register, updates a global variable, and stores a bit-manipulated value into a table if a condition is met.
- `open_resource_handler`: Attempts to open a resource and, if successful, performs several operations using the returned pointer before returning a failure code.
- `close_resource_handler`: Checks if a resource pointer is valid and, if so, performs two operations using that pointer before clearing it.
- `get_concatenated_byte`: Returns a 16-bit value formed by concatenating a bit-shifted version of a register with a byte from a table.
- `pop_table_element`: Decrements a global counter and shifts elements within a table to remove the first entry, returning the removed value.
- `update_graphics_parameters`: Calculates and stores multiple values into a series of memory locations based on input registers and global variables.
- `apply_zero_mask`: Uses a bitwise OR instruction to ensure a data register contains zero.
- `configure_blitter_transfer`: Sets up Blitter hardware registers and memory locations using values derived from input registers and a loop.
- `setup_blitter_state`: Writes specific values to memory locations and Blitter registers to prepare for a hardware transfer.
- `validate_resource_header`: Checks a memory block for specific magic signatures and validates its structure using library function calls.
- `execute_conditional_callback`: Checks if a value at a pointer matches a specific four-byte constant and, if so, calls a function pointer stored at a fixed offset from a global variable.
- `copy_with_conditional_branch`: Copies a block of bytes from a source to a destination while checking each byte against a global table.
- `process_complex_conditional_chain`: Checks a magic constant, updates several global variables, and executes a sequence of conditional routines based on various status checks.
- `expand_nested_data_streams`: Iterates through a structured data format to expand and copy byte sequences into a destination buffer.
- `expand_nested_data_stream`: Iterates through a structured data format to copy byte sequences into a destination buffer.
- `decompress_nested_data_streams`: Traverses a structured data format to expand and copy byte sequences into a destination buffer using nested loops and length descriptors.
- `decrement_and_check_counter_2`: Decrements a value at a pointer offset and traverses a linked list if the value reaches zero.
- `decrement_and_check_counter`: Decrements a value at an offset from a pointer and calls another routine if the value reaches zero.
- `initialize_interrupt_server_2`: Sets a global variable to a specific value and calls another routine if a certain condition is met.
- `remove_interrupt_server_2`: Resets a global variable and calls another routine if a certain condition is met.
- `clear_data_register_byte_2`: Performs a bitwise OR operation on a byte of a data register using zero.
- `null_operation_padding`: This routine consists of a single instruction that performs a bitwise OR operation on a data register with zero, effectively acting as a no-operation.
- `configure_blitter_transfer_2`: Calculates several values and uses them to populate Blitter registers and memory locations through a loop.
- `setup_blitter_state_2`: Writes specific values to Blitter registers and memory locations based on global variables.
- `evaluate_conditional_chain`: Recursively calls another routine and checks a byte at a specific offset from the returned pointer to determine control flow.
- `traverse_data_chain_2`: Walks through a linked list of data elements by following offsets until a target value is found or the end is reached.
- `search_linked_list_2`: Traverses a linked list of structures identified by a specific four-byte constant to find an entry matching a given value.
- `update_dma_parameters_2`: Swaps two global values and populates a series of memory locations with calculated longwords based on a loop.
- `sync_global_state_and_transform_table`: Copies a sequence of longwords from a pointer into a series of global variables, performs a bitwise nibble-swapping transformation on a data block, and waits for a specific bitmask condition in a global variable before updating that variable with the previously stored values.
- `search_linked_list_3`: Iterates through a list of data blocks marked by a constant signature to locate a specific entry based on a provided value.
- `traverse_data_chain`: Walks through a chain of data elements by using an offset stored within each element to find a matching value.
- `update_data_structures`: Copies a series of longwords from a pointer to several global variables, modifies a sequence of values using bitwise operations, and waits for a bitmask condition in a global variable before updating a global pointer.
- `validate_status_byte_2`: Calls another routine to retrieve a pointer, checks a byte at a specific offset, and performs conditional logic based on that byte and a global flag.
- `evaluate_conditional_data_chain`: Recursively calls another routine and checks a byte at a specific offset from the returned pointer.
- `process_nested_buffers`: Performs complex data manipulation on nested buffers, involving sign-based branching and byte copying across multiple loops.
- `copy_multidimensional_data`: Copies blocks of data using nested loops to move bytes from a source to a destination based on calculated dimensions.
- `synchronize_data_block`: Waits for a specific bitmask condition in a global variable before copying a series of longwords from a source pointer to that global variable.

### Nova 9 (1991)(Dynamix)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning.
