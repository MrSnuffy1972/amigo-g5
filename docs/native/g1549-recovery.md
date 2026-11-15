# Game 1549 (helter skelter (1990)(audiogenic)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01549). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916080 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 241 raw-word lines |
| Functions named and described | pass, 166 of 166 named (11 author symbols), 166 of 166 described (3 by rule, 150 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3135 of 3135 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 326122 bytes, 68 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6554 of 6575 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28817 |
| code | 22754 |
| data | 347657 |
| padding | 1780 |
| unused | 515072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:Type | 2 | 363 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3103 | 65 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img (raw code file, load address unknown) | 1 | 2747 | 68 |

## Cases handled

- 2799 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 2799 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- helter skelter (1990)(audiogenic)[cr skr][a]/Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 address
- 1 never-loaded region(s), 326122 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `increment_and_store_loop` (runtime segment an address): Continuously increments a short integer value starting at offset 384 of a global structure and stores the result back to the same location, creating an infinite loop that never exits. Kind: disk hardware check. Pass path: branch taken when ADKCON and CIAB_PRB values match expected hardware state.
- `process_input_data` (runtime segment an address): Processes input data by calling multiple routines based on a global variable's state, and returns a value from one of those calls when the variable is non-zero. Kind: checksum trap. Pass path: The function returns the result of `u3__sub_121C()` when `DAT_0000089b` is non-zero, indicating a successful checksum validation..
- `configure_display_and_copy_buffers` (hs.img (raw code file, load address unknown)): Configures display registers and copies or initializes memory buffers or tables. Kind: disk/track/hardware check (ADKCON/CIAB_PRB). Pass path: an address.w == d0.

## Functions


### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.
- `update_entity_state_and_dispatch_events`: Increments an index in a structure, clears several fields in a global state structure, and calls a dispatcher multiple times to update entity properties, handle conditional logic based on table lookups, and potentially iterate through a list of sub-elements while updating the global state.
- `process_event_dispatch_loop`: Iterates through a table of pointers, incrementing a counter at offset 56 of each structure and comparing it against a value fetched from a table in a1.
- `set_global_constant`: Sets d1 to negative 256, stores this value at an offset of 612 in a2, and branches to another routine.
- `set_offset_constant`: Sets d1 to negative 256, stores this value at an offset of 516 in a1, and branches to another routine.
- `fetch_byte_and_call`: Calculates an index using values from a1 and a0, fetches a byte from a0 at that index, loads a pointer from an offset of 304 in a4 into a3, sets d0 to 240, loads a pointer from an offset of 360 in a2 into a4, calls the function at a5, and branches to another routine.
- `fetch_byte_and_dispatch_event`: Calculates an index from a table in a1, fetches a byte from a0 at that index, and calls a routine via a pointer in a4.
- `process_event_dispatch_loop_2`: Increments a counter at offset 224 of a structure and iterates through a table of pointers, calling a function for each entry until the counter reaches a value fetched from a table.
- `execute_conditional_function_sequence`: Sets a value of 1 in a structure at offset 616, then calls a function via a pointer in a structure, stores the result in a structure at offset 208, and enters a loop that continues calling functions and checking results until a specific error value or a zero result is encountered, eventually storing the value 5 in a structure at offset 632 before returning via an indirect jump.
- `branch_short`: Performs a short relative branch to another routine.
- `call_indirect_function`: Loads the value 232 into d0, loads a pointer from an offset of 336 in a2 into a4, calls the function at the address in a5, and then jumps to the address in a6.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs

- `start`: Program entry that opens the DOS library and allocates memory, clears colour and interrupt-related registers, sets display window and bitplane registers and builds the copper list.
- `copy_trampoline_and_jump`: Copies three longwords from the transfer block into a global area, then jumps to that area as code.
- `trans1`: A four-instruction block that one unnamed caller reaches by a pc-relative address.
- `loadfile`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.
- `dopiccy`: Loads a file, fades the display down, decompresses the picture and then runs sixteen vertical-blank waits, each followed by a fade step.
- `dopiccy1`: Decompresses picture data and then runs sixteen vertical-blank waits, each followed by a fade step.
- `waitabit`: Waits for one hundred and fifty vertical blanks by calling the vertical-blank wait routine in a countdown loop.
- `decompress`: Walks a chunked picture file, reads the palette chunk into a sixteen-entry palette and then run-length decodes the body into the image buffer row by row.
- `fadedown`: The routine waits for scanlines and updates the palette fade repeatedly.
- `dofade`: The routine reads 32 palette entries from chip memory and writes them to the Amiga color registers.
- `wvbl`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `makecoplist`: Writes a copper list of four bitplane-pointer pairs spaced forty bytes apart and terminates it with end-of-list words.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Contains a single instruction that unconditionally branches to a fixed address offset, effectively jumping to another routine in the program.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, processes it through bit manipulation, and updates various global variables based on specific byte values.
- `compare_with_threshold`: Compares a global variable with data from a table and performs conditional operations based on the comparison results, using flags and loop controls to determine execution flow.
- `update_joystick_positions`: Reads joystick and mouse input data, calculates position differences, updates global variables with movement values, and sets flags based on button states and input conditions.
- `clear_global_words`: Clears multiple word-sized global variables by setting them to zero using explicit store instructions.
- `clear_global_bytes`: Clears multiple byte-sized global variables by setting them to zero using explicit store instructions.
- `update_calculated_values`: Performs arithmetic operations on values from a table, processes the results through addition and bit manipulation, and stores the updated values back into global variables.
- `transfer_sprite_data`: Copies sprite data between memory locations, applies bitwise operations to transform color and shape information, and updates pointers for subsequent processing.
- `transform_sprite_data`: Processes sprite definition data by applying bitwise operations to manipulate color and shape information during transfer, using shift and mask techniques for scaling or sub-pixel positioning.
- `fill_memory_block`: Copies a 32-bit value from a register to an array of 8000 elements, incrementing the destination pointer after each write operation.
- `process_word_array`: Processes an array of 16 words by applying a bitwise AND operation with the value 1911 and then doubling the result, storing back to the same array locations.
- `copy_word_arrays`: Copies 16-bit words from a source array to multiple destination arrays in sequence, then transfers 32-bit longwords from one location to another using a loop with a counter.
- `copy_memory_segments`: Copies data between three memory regions, writing 1999 elements from each of three source addresses to corresponding destinations in two other memory areas.
- `find_zero_word`: Traverses a sequence of words starting at a fixed address until it finds a zero word, advancing the pointer through the sequence with each iteration.
- `update_counter_and_process`: Decrements a counter and if it reaches zero, processes data from a buffer by reading values and updating pointers in a structure.
- `apply_xor_mask_to_memory`: Performs bitwise XOR operations between values from global tables and source data, then writes the results to multiple memory locations with different offsets.
- `process_and_store_data`: Processes data using bitwise AND and XOR operations with values from a global table, then stores the results to multiple memory locations with various offsets.
- `process_bcd_arithmetic`: Performs BCD arithmetic on a value by adding bytes with carry, updating memory locations, and calling routines to process byte data based on comparison results.
- `process_byte_data`: Processes a byte value through a series of operations including bitwise AND, division, and stack manipulation before calling another routine multiple times in a loop.
- `update_table_entries`: Processes a global variable through bitwise operations and conditional checks, updates table entries, and performs bit manipulation on memory locations before returning.
- `apply_bitwise_operations`: Performs bitwise AND and XOR operations on data using values from a global table, then stores the results to multiple memory locations with specific offsets.
- `process_and_store_data_2`: Processes data using bitwise AND and XOR operations with values from a global table, then stores the results to multiple memory locations with specific offsets.
- `compute_and_process`: Computes a value through arithmetic operations and division, then applies bitwise operations using a global table before storing results to memory locations.
- `process_byte_sequence`: Processes a sequence of bytes from a buffer, skipping zero values, and calls another routine for each non-zero byte while managing register state.
- `compare_and_set_flags`: Compares several word values against thresholds and sets processor flags based on comparison results, returning one of the input values depending on which condition is met.
- `process_relocation_list`: Iterates through a list of pointers ending with a negative value, calling another routine for each entry in the list.
- `update_tile_flags`: Processes tile data based on input parameters to modify bit flags in memory locations, with conditional logic determining which bits are set or cleared.
- `process_data_block`: Reads bytes from a source buffer and writes them to a destination, with conditional copying and subtraction operations based on the sign of read values.
- `clear_memory_region`: Clears a 16-bit word value in a memory region by writing zero to each location, iterating through a table with a fixed stride.
- `initialize_data_structure`: Writes a series of constant values and pointers into a memory location, including a word, two longwords, and another word.
- `calculate_and_store_values`: Processes a series of values from memory, applies bit operations and arithmetic transformations, then stores the results in two separate locations.
- `transform_and_store_data`: Reads data from memory locations, performs bitwise operations and arithmetic calculations on values, then stores transformed results to two destinations.
- `copy_and_transform_data`: Copies data between memory regions using calculated offsets, transforms values through bitwise operations, and stores results to multiple locations while initializing several global variables.
- `jump_to_address`: Loads specific addresses into registers and performs a jump to another location without any additional processing or return.
- `load_pointer_offset`: Loads a memory address from a fixed location into register a0, then adjusts that address by adding a constant offset, and finally branches to another routine without modifying any data.
- `setup_and_branch`: Initializes a memory address from a fixed location into register a0, then adjusts that address by adding another constant offset, loads a word value from a specific memory location into register a3, and finally branches to another routine without modifying any data.
- `fetch_and_jump`: Loads a memory address from a fixed location into register a0, then adjusts that address by adding a constant offset, copies a word value from a specific memory location into register d0, and finally branches to another routine without modifying any data.
- `decrement_and_clear`: Decrements a counter in a global structure and clears multiple memory locations with zero or specific values when the counter reaches zero.
- `update_animation_data`: Decrements a global counter and processes animation data from a buffer when the counter reaches zero, updating values in a structure.
- `update_position_and_collision`: Processes position coordinates and performs collision detection with boundaries, updating position and velocity values based on comparison results.
- `load_and_adjust_pointer`: Loads a value from memory, adjusts an address based on offset values, and copies data from a calculated location to another memory area.
- `load_and_adjust_pointer_2`: Loads a pointer from memory, adjusts it by adding a fixed offset, then branches to another routine at a specific location.
- `compare_coordinate_ranges`: Compares coordinate values against ranges defined in memory, returning different status codes based on whether the comparisons fall within specified bounds.
- `scale_and_update_value`: Scales a value by dividing and multiplying with constants, updates a memory location with the result, and sets a flag to indicate completion.
- `update_position_and_velocity`: Updates position and velocity components based on input conditions and global state, then adjusts related counters and flags accordingly.
- `process_collision_data`: Processes collision-related data by loading word values from memory locations, performing arithmetic operations on them, and applying lookup table transformations based on the results.
- `update_pointer_and_check`: Updates a pointer by loading a value from a global variable, increments it by 36, and compares the result with the address an address.
- `invert_position_components_2`: Inverts the sign of three signed values located at specific offsets in a data structure, then increments a counter until it reaches twelve.
- `invert_position_components`: Inverts the sign of three signed values located at specific offsets in a data structure, then increments a counter until it reaches twelve, performing this sequence multiple times before clearing a global variable and returning.
- `update_position_values`: Updates position-related values in memory by performing arithmetic operations on data from global variables and lookup tables.
- `increment_global_counter`: Increments a counter in a global variable while traversing a sequence of integers until it encounters a negative value, then returns the incremented counter value.
- `update_position_vector`: Processes a bit sequence to determine a range, calculates vector components using multiplication and accumulation, and updates position data based on table lookups.
- `initialize_structures`: Initializes two different data structures based on a global variable's value, populating them with specific constant values and offsets, and clears certain flags in hardware registers.
- `build_lookup_table`: Iterates through a sequence of values to populate a table with calculated offsets and data derived from the list entries, including processing word values and adjusting memory pointers based on input data.
- `reset_counter_if_match`: Checks if a specific value in a structure matches a constant, and if so, clears a counter field in that same structure before returning.
- `load_string_data`: Loads string processing parameters into registers and calls a routine to process characters from a global table, then updates the table pointer and repeats the process.
- `process_string_characters`: Iterates through a null-terminated string, calling another routine for each character while incrementing a counter, and returns immediately after processing all characters.
- `update_collision_flags`: Updates collision detection flags based on object positions and thresholds, then processes table entries and adjusts BCD values in a memory region.
- `update_data_structure`: Updates a data structure by copying values from another structure, performing arithmetic operations on memory locations, and setting specific flags based on comparison results before returning.
- `initialize_game_state`: Initializes game state variables and clears specific memory locations before proceeding with further setup.
- `increment_and_store_loop`: Continuously increments a short integer value starting at offset 384 of a global structure and stores the result back to the same location, creating an infinite loop that never exits.
- `wait_for_memory_match`: Enters a loop that continues until the sum of a global variable and a register value equals the global variable itself.
- `process_input_data`: Processes input data by calling multiple routines based on a global variable's state, and returns a value from one of those calls when the variable is non-zero.
- `search_string_for_byte`: Iterates through a memory buffer looking for a specific byte and returns a value from an offset if found.
- `initialize_rendering`: Sets up rendering parameters by calling another routine with varying offset values, then processes a byte sequence until a zero terminator is encountered.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_tables`: Initializes several memory tables with pointers and values, while configuring audio hardware, DMA, and interrupts.
- `update_audio_buffer_list`: Iterates through a bitmask-controlled sequence of pointers to copy data into audio hardware registers.
- `setup_data_structures`: Populates two different tables with specific values and pointers, and modifies several global variables.
- `update_table_entry`: Searches a table for a specific pointer and, if not found, updates a table entry with a value and its preceding word.
- `initialize_function_pointer_table`: Populates a table with alternating entries of a routine address and a constant value, then assigns that routine's address to a global variable.
- `wait_for_pointer`: Enters a loop that waits until a specific pointer is found within a table.
- `update_list_with_mask`: Iterates through a table and copies entries to a destination, performing bitwise shifts or arithmetic on data based on a threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: The routine is empty and contains no instructions.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img (raw code file, load address unknown)

- `initialize_memory_tables`: Initializes a memory-resident table with incrementing or constant values and clears several global or memory-based flags and pointers.
- `wait_for_condition_and_limit`: Waits for a specific bit in a memory location to become set or checks a global variable against a value in a table, returning if a certain condition is met.
- `clear_global_variable`: Sets a global variable to zero multiple times.
- `clear_global_variable_2`: sets a global variable to zero.
- `update_global_with_math`: Performs arithmetic or bitwise operations using values from a table and stores the result into a global variable.
- `process_graphics_list`: Iterates through a list of pointers, using a lookup table to calculate source and destination addresses, then copies memory blocks between them.
- `update_bitmapped_graphics_layer`: Calculates a destination pointer and bit-shift amount based on a lookup table, then iterates through a source list to perform bitwise OR and masking operations on a target buffer.
- `fill_memory_with_register`: Fills a block of memory with the contents of a register using a pointer.
- `modify_memory_with_mask`: Iterates through a list of memory locations, masking certain bits and multiplying the result by two.
- `expand_memory_block`: Calls another routine and then copies a block of memory from a source pointer to several different destination pointers, followed by a second loop that copies a block or clears a range.
- `calculate_digit_mapping`: Calculates a value based on a global variable, performs a digit-swapping or masking operation, and calls another routine with a constructed parameter.
- `transform_index_and_swap_digits`: Calculates an offset-based index, performs a digit-swapping bitwise transformation on a value derived from that index, and passes the result to another routine.
- `copy_memory_blocks`: Copies a block of data from a source table to two different destination tables.
- `sync_memory_buffers`: Copies a block of data from one source pointer to two different destination pointers.
- `find_null_terminator`: Iterates through a table until a null or zero-valued word is encountered.
- `update_decoding_table`: Calculates a bitwise value and stores the individual bytes of that value into a large memory buffer and a secondary offset buffer.
- `apply_xor_mask_to_buffer`: Applies a bitwise XOR mask and stores the resulting bytes into a primary buffer and a secondary offset buffer.
- `update_value_with_carry`: Performs arithmetic or bitwise operations on a byte or a pointer, involving a carry or a checksum-like adjustment, and updates a certain memory location.
- `repeat_apply_mask`: Executes another routine a fixed number of times in a loop.
- `apply_masked_data`: Decrements a global variable and, if certain conditions are met, writes masked or modified data to a range of memory buffers.
- `apply_masked_buffer_update`: Calls another routine twice and then performs a loop that applies a bitwise mask to data, storing the result into two separate large memory buffers at specific offsets.
- `write_xor_buffer_v_p_ptr`: Iterates through a source pointer, applies a bitwise XOR mask, and writes the resulting bytes to a primary and secondary buffer.
- `write_xor_buffer_derived_ptr`: Calculates a source pointer based on a global variable and a mathematical offset, then writes XORed bytes to a primary and secondary buffer.
- `traverse_string_buffer`: Iterates through a pointer until a null byte is encountered, calling another routine for each byte.
- `traverse_pointer_list`: Iterates through a list of pointers until a negative or terminal value is encountered.
- `patch_hunk_relocation_data`: Iterates through a range of memory defined by a hunk-type structure to apply bitmask-based bit-setting or bit-clearing to a set of destination buffers.
- `decode_string_and_copy`: Decodes a specialized string format by reading or copying bytes based on certain values and writes the result to a destination buffer.
- `clear_interleaved_table_values`: clears a sequence of words to zero, skipping a fixed number of bytes between each write.
- `initialize_struct_members`: Writes a series or constant values to a memory structure pointed to by a register.
- `empty_loop_routine`: The routine is empty and consists only of an infinite loop.
- `empty_loop_routine_2`: the routine executes an infinite loop without performing any operations or modifying any registers.
- `initialize_and_copy_buffer_tables`: Initializes a block of memory with zeroed or constant values and copies data between several memory-mapped or or large buffer structures.
- `run_repeated_routine`: Executes another routine a fixed number of times using a loop.
- `process_digits_loop`: Iteratively calculates a value based on a global variable and calls another routine while processing or splitting digits.
- `execute_repeated_routine`: executes another routine a fixed number of times.
- `iteratively_transform_and_pass_data`: Iteratively applies a mathematical transformation involving division and remainder to a global variable and passes the result to another routine.
- `select_and_store_pointer_value`: updates a flag in a global structure and selects a word from a table or an offset-displaced table based on a conditional check, then stores either the first or second word of the selected location into a target pointer or value.
- `initialize_conditional_struct`: Checks a global variable and writes a set of constant values or values from a pointer-offset to a memory structure.
- `process_pointer_table`: Iterates through a table of pointers, and for each, populates a structure with a calculated pointer and a flag.
- `apply_to_string_data`: Iterates through a null-terminated string and calls another routine with an incrementing value for each character found.
- `traverse_null_terminated_string`: Iterates through a string starting at a fixed address until a null terminator is found.
- `configure_display_and_copy_buffers`: Configures display registers and copies or initializes memory buffers or tables.
- `find_base_address`: Loops through memory until the pointer or index returns to the start of a global variable.
- `execute_recursive_routine_calls`: Executes a routine twice and then, if a certain global variable is non-zero, executes it two additional times, returning the last result in a register.
- `search_string_for_value`: Searches through a string for a terminator and returns a byte from an offset relative to that terminator if found.
- `traverse_string_and_call`: Traverses a null-terminated string and calls another routine for every byte encountered.
- `update_mask_tables`: Copies blocks of data from a source pointer to a destination, then applies a bitwise mask to several global memory locations.
- `mask_table_values`: Performs a bitwise AND between a register and several specific memory locations, while also ORing a value into certain table entries.
- `expand_and_copy_data`: Copies and expands a small block of data from one memory location to a larger, or staggered, destination area.
- `copy_memory_block`: Iteratively copies large blocks of data from a source address to a destination address, where the source address is determined by a value in a table.
- `increment_returned_value`: calls another routine and returns the result incremented by one.
- `transform_and_call_digit`: Constructs a value using a register and a global variable, performs a digit-swapping or masking operation, and calls another routine with a constructed parameter.
- `recursively_call_routine`: This routine calls itself repeatedly before returning.
- `update_decoding_table_2`: Calculates a bitwise value and stores the individual bytes of that value into a large memory buffer and a secondary offset buffer, while also traversing a null-terminated string and calling another routine.
- `clamp_global_variable`: Iteratively checks a global variable, clears it, and then applies a clamping or saturation logic to a value before storing it back to a global variable.
- `copy_and_expand_buffer`: Updates a global variable using a bitmask, then copies a sequence of words from a source table to a destination buffer, repeating the copy or clearing certain memory offsets.
- `traverse_pointer_list_2`: Iterates through a list of pointers in memory until a negative or terminal pointer value is reached.
- `traverse_pointer_triplet_list`: Iterates through a list of pointers, advancing by a fixed offset, and calls another routine.
- `blit_bitmapped_sprite_layer`: Iterates through a table of sprite or tile-like data to apply bitwise masks and ORs to a destination bitmapped buffer.
- `blit_sprite_to_bitmapped_layer`: Iterates through a source list to apply bitwise OR and AND operations, masking or shifting sprite pixel data into a destination bitmapped buffer.
- `reset_global_variable`: Sets a global variable to a specific constant value and calls another routine.
- `read_memory_indirect`: Returns a 32-bit value read from a memory location calculated by adding an offset to a register.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `set_memory_pattern`: Writes a specific sequence of bytes into a memory range and updates a global variable.
- `set_audio_registers`: Checks a global variable and, if it does not match a specific value, performs a loop to decrement a counter and writes several values to audio-related hardware registers and certain memory locations.
- `initialize_data_pattern`: Writes a specific pattern of 32-bit or 16-bit values to a range of memory starting from an offset relative to a register.
- `find_and_insert_value`: Searches a list of memory locations for a specific 32-bit pattern and, if found, replaces a value or inserts a new value from a register.
- `search_for_specific_word`: Iterates through a memory-resident table of 32-bit words, checking if any entry matches a specific constant value, and continues for a fixed number of iterations or until the value is found.
