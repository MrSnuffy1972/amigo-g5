# Game 1549 (helter skelter (1990)(audiogenic)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01549). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916080 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 242 raw-word lines |
| Functions named and described | pass, 196 of 196 named (11 author symbols), 196 of 196 described (16 by rule, 167 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3135 of 3135 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 326122 bytes, 98 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6554 of 6575 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28817 |
| code | 25964 |
| data | 344442 |
| padding | 1785 |
| unused | 515072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:Type | 2 | 434 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3103 | 65 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img (raw code file, load address unknown) | 1 | 3650 | 98 |

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
- `reset_global_variable` (hs.img (raw code file, load address unknown)): Sets a global variable to a specific constant value and calls another routine. Kind: disk hardware check. Pass path: The routine executes normally and continues with the game code flow; the check passes if CIAB_PRB value matches expected hardware signature..
- `frag_move_rts` (hs.img (raw code file, load address unknown)): Short code fragment that copies 255 to a fixed memory address, then copies 135 to a fixed memory address, then copies 255 to a fixed memory address, then returns. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.
- `frag_move_rts_2` (hs.img (raw code file, load address unknown)): Short code fragment that copies 119 to a fixed memory address, then returns. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.
- `write_port_bits` (hs.img (raw code file, load address unknown)): Writes specific byte values to a hardware port address, sets bit 0 of a register, and performs a loop that counts down a counter while writing values to the same port address. Kind: disk hardware check. Pass path: branch taken when CIAB_PRB read value matches expected hardware signature.

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
- `process_sprite_list`: Iterates through a list of pointers, performing memory copies from source addresses to destination addresses based on values stored in the list.
- `blit_transformed_sprites`: Iterates through a list of sprite definitions to perform bitwise scaling and color manipulation, copying transformed pixel data from a source table into destination buffers using bitwise rotations and masks.
- `fill_memory_block`: Copies a 32-bit value from a register to an array of 8000 elements, incrementing the destination pointer after each write operation.
- `process_word_array`: Processes an array of 16 words by applying a bitwise AND operation with the value 1911 and then doubling the result, storing back to the same array locations.
- `copy_memory_blocks_2`: Calls another routine and then performs two large block memory copies to different destination addresses.
- `copy_memory_segments`: Copies data between three memory regions, writing 1999 elements from each of three source addresses to corresponding destinations in two other memory areas.
- `find_zero_word`: Traverses a sequence of words starting at a fixed address until it finds a zero word, advancing the pointer through the sequence with each iteration.
- `update_counter_and_process`: Decrements a counter and if it reaches zero, processes data from a buffer by reading values and updating pointers in a structure.
- `apply_xor_mask_to_memory`: Performs bitwise XOR operations between values from global tables and source data, then writes the results to multiple memory locations with different offsets.
- `apply_conditional_xor_to_memory`: Calculates a value using a bitwise XOR and a conditional mask, then writes the resulting 32-bit word to two different memory destinations.
- `process_bcd_value`: Performs BCD arithmetic on a value and updates a memory location based on a comparison.
- `repeat_routine_eight_times`: Executes another routine eight times in a loop.
- `update_table_entries`: Processes a global variable through bitwise operations and conditional checks, updates table entries, and performs bit manipulation on memory locations before returning.
- `apply_bitwise_operations`: Performs bitwise AND and XOR operations on data using values from a global table, then stores the results to multiple memory locations with specific offsets.
- `process_and_store_data_2`: Processes data using bitwise AND and XOR operations with values from a global table, then stores the results to multiple memory locations with specific offsets.
- `compute_and_process`: Computes a value through arithmetic operations and division, then applies bitwise operations using a global table before storing results to memory locations.
- `process_byte_sequence`: Processes a sequence of bytes from a buffer, skipping zero values, and calls another routine for each non-zero byte while managing register state.
- `compare_and_set_flags`: Compares several word values against thresholds and sets processor flags based on comparison results, returning one of the input values depending on which condition is met.
- `process_relocation_list`: Iterates through a list of pointers ending with a negative value, calling another routine for each entry in the list.
- `update_tile_flags`: Processes tile data based on input parameters to modify bit flags in memory locations, with conditional logic determining which bits are set or cleared.
- `copy_data_with_expansion`: Performs a nested loop to move and expand data between memory locations based on signed byte values.
- `clear_memory_region`: Clears a 16-bit word value in a memory region by writing zero to each location, iterating through a table with a fixed stride.
- `initialize_data_structure`: Writes a series of constant values and pointers into a memory location, including a word, two longwords, and another word.
- `enter_infinite_loop`: Executes an infinite loop that prevents further code execution.
- `execute_infinite_loop`: The routine performs a series of operations that result in an infinite loop, preventing further execution.
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
- `initialize_conditional_structures`: Checks a global variable and populates one of two different data structures with specific constant values and offsets.
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
- `wait_for_input_condition`: Polls a bit in a memory location while comparing a global variable against another value.
- `clear_global_variable`: Sets a global variable to zero multiple times.
- `clear_global_variable_2`: sets a global variable to zero.
- `update_calculated_values_2`: Performs arithmetic operations using values from a table and stores the results into a global variable.
- `process_sprite_list_2`: Iterates through a list of pointers, performing memory copies from source addresses to destination addresses based on values stored in the list.
- `blit_transformed_sprites_2`: Copies sprite data from a source table to a destination buffer using bitwise shifts and masks to perform scaling or sub-pixel positioning.
- `fill_memory_with_register`: Fills a block of memory with the contents of a register using a pointer.
- `modify_memory_with_mask`: Iterates through a list of memory locations, masking certain bits and multiplying the result by two.
- `copy_memory_blocks_3`: Calls another routine and then performs two large block memory copies to different destination addresses.
- `calculate_digit_mapping`: Calculates a value based on a global variable, performs a digit-swapping or masking operation, and calls another routine with a constructed parameter.
- `transform_index_and_swap_digits`: Calculates an offset-based index, performs a digit-swapping bitwise transformation on a value derived from that index, and passes the result to another routine.
- `copy_memory_blocks`: Copies a block of data from a source table to two different destination tables.
- `copy_memory_triple_variable`: Copies data from a source memory location to two separate destination locations for a specified number of iterations.
- `find_null_terminator`: Iterates through a table until a null or zero-valued word is encountered.
- `update_counter_and_process_2`: Decrements a word counter at offset 26 of the structure in a5 and if it reaches zero, reads data from a buffer pointer at offset 22 of the structure and updates the buffer pointer and counter in the structure.
- `apply_xor_mask_to_memory_2`: Performs a bitwise XOR operation between values from a global table and a source table, then writes the results to two different memory destinations.
- `apply_conditional_xor_to_memory_2`: Calculates a value using a bitwise XOR and a conditional mask, then writes the resulting 32-bit word to two different memory destinations.
- `process_bcd_value_2`: Performs BCD arithmetic on a value and updates a memory location based on a comparison.
- `repeat_routine_eight_times_2`: Executes another routine eight times in a loop.
- `apply_masked_data`: Decrements a global variable and, if certain conditions are met, writes masked or modified data to a range of memory buffers.
- `apply_masked_buffer_update`: Calls another routine twice and then performs a loop that applies a bitwise mask to data, storing the result into two separate large memory buffers at specific offsets.
- `write_xor_buffer_v_p_ptr`: Iterates through a source pointer, applies a bitwise XOR mask, and writes the resulting bytes to a primary and secondary buffer.
- `write_xor_buffer_derived_ptr`: Calculates a source pointer based on a global variable and a mathematical offset, then writes XORed bytes to a primary and secondary buffer.
- `iterate_null_terminated_string`: Iterates through a sequence of bytes until a null terminator is encountered, calling another routine for each byte.
- `compare_and_set_flags_2`: Compares several word values against thresholds and sets processor flags based on comparison results, returning one of the input values depending on which condition is met.
- `traverse_pointer_list`: Iterates through a list of pointers until a negative or terminal value is encountered.
- `patch_hunk_relocation_data`: Iterates through a range of memory defined by a hunk-type structure to apply bitmask-based bit-setting or bit-clearing to a set of destination buffers.
- `copy_data_with_expansion_2`: Performs a nested loop to move and expand data between memory locations based on signed byte values.
- `clear_interleaved_table_values`: clears a sequence of words to zero, skipping a fixed number of bytes between each write.
- `initialize_struct_members`: Writes a series or constant values to a memory structure pointed to by a register.
- `enter_infinite_loop_2`: Executes an infinite loop that prevents further code execution.
- `empty_loop_routine_2`: the routine executes an infinite loop without performing any operations or modifying any registers.
- `initialize_and_copy_buffer_tables`: Initializes a block of memory with zeroed or constant values and copies data between several memory-mapped or or large buffer structures.
- `run_repeated_routine`: Executes another routine a fixed number of times using a loop.
- `process_digits_loop`: Iteratively calculates a value based on a global variable and calls another routine while processing or splitting digits.
- `execute_repeated_routine`: executes another routine a fixed number of times.
- `iteratively_transform_and_pass_data`: Iteratively applies a mathematical transformation involving division and remainder to a global variable and passes the result to another routine.
- `update_pointer_value`: Selects a word from one of two memory locations based on a condition and stores it in a global variable.
- `frag_movea_lea_bra`: Short code fragment that copies a fixed memory address to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `validate_and_process_input`: Reads input values from a buffer, performs comparisons with thresholds, updates pointers in a structure, and returns a validation result based on comparison outcomes.
- `process_and_validate_input`: Processes input values from a buffer, performs arithmetic operations and comparisons with thresholds, updates pointers in a structure, and returns a validation result based on the processing outcome.
- `update_and_process_positions`: Updates position values in a structure by negating longword values at specific offsets, increments a counter, and calls other routines to process and validate position data based on conditional checks.
- `update_position_values_2`: Reads word values from a structure at offsets 2 and 6, performs arithmetic operations with data from memory locations, updates the structure values, and compares them with input values to return a result.
- `process_and_insert_elements`: Processes word values to calculate offsets into memory tables, checks bits in memory locations, and inserts elements into lists with specific flags and values based on conditions.
- `calculate_and_update_coordinates`: Processes word values to calculate offsets into memory tables, checks bits in memory locations, and updates coordinate values in a structure with results from multiplication operations.
- `initialize_conditional_structures_2`: Checks a global variable and populates one of two different data structures with specific constant values and offsets.
- `build_lookup_table_2`: Iterates through a list of values to populate a table with calculated offsets and data derived from the list entries.
- `apply_to_string_data`: Iterates through a null-terminated string and calls another routine with an incrementing value for each character found.
- `process_string_characters_2`: Iterates through a null-terminated string, calling another routine for each character while incrementing a counter, and returns immediately after processing all characters.
- `configure_display_and_copy_buffers`: Configures display registers and copies or initializes memory buffers or tables.
- `find_base_address`: Loops through memory until the pointer or index returns to the start of a global variable.
- `execute_recursive_routine_calls`: Executes a routine twice and then, if a certain global variable is non-zero, executes it two additional times, returning the last result in a register.
- `search_string_for_value`: Searches through a string for a terminator and returns a byte from an offset relative to that terminator if found.
- `traverse_string_and_call`: Traverses a null-terminated string and calls another routine for every byte encountered.
- `update_mask_tables`: Copies blocks of data from a source pointer to a destination, then applies a bitwise mask to several global memory locations.
- `apply_bitmask_to_table`: Applies bitwise AND operations to specific word entries in a large table using values from registers.
- `expand_and_copy_data`: Copies and expands a small block of data from one memory location to a larger, or staggered, destination area.
- `transfer_memory_blocks`: Iterates through a list of descriptors to copy blocks of data from one memory range to another.
- `increment_returned_value`: calls another routine and returns the result incremented by one.
- `transform_and_call_digit`: Constructs a value using a register and a global variable, performs a digit-swapping or masking operation, and calls another routine with a constructed parameter.
- `recursively_call_routine`: This routine calls itself repeatedly before returning.
- `update_decoding_table_2`: Calculates a bitwise value and stores the individual bytes of that value into a large memory buffer and a secondary offset buffer, while also traversing a null-terminated string and calling another routine.
- `clamp_global_variable`: Iteratively checks a global variable, clears it, and then applies a clamping or saturation logic to a value before storing it back to a global variable.
- `fill_pattern_buffer`: Calculates two values from a global variable and stores them in a table, then performs a nested loop to fill a large memory area with a repeating pattern of words.
- `traverse_pointer_list_2`: Iterates through a list of pointers in memory until a negative or terminal pointer value is reached.
- `iterate_pointer_list`: Iterates through a list of pointers in increments of three, calling another routine for each entry until a negative value is found.
- `blit_bitmapped_sprite_layer`: Iterates through a table of sprite or tile-like data to apply bitwise masks and ORs to a destination bitmapped buffer.
- `blit_sprite_to_bitmapped_layer`: Iterates through a source list to apply bitwise OR and AND operations, masking or shifting sprite pixel data into a destination bitmapped buffer.
- `reset_global_variable`: Sets a global variable to a specific constant value and calls another routine.
- `frag_move_rts`: Short code fragment that copies 255 to a fixed memory address, then copies 135 to a fixed memory address, then copies 255 to a fixed memory address, then returns.
- `frag_move_btst_beq_moveq_bsr_dbra_clr_r`: Short code fragment that copies 255 to d7, then tests bit 4 of a fixed memory address, then branches if the last result was zero, then loads 1 into d0, then calls another routine, then loops back while counting down d7, then clears a fixed memory address, then returns.
- `adjust_and_compare_values`: Takes a word value from a memory location, compares it with another value, adjusts the first value based on the comparison result, and performs conditional operations based on the sign of the difference.
- `frag_move_rts_2`: Short code fragment that copies 119 to a fixed memory address, then returns.
- `frag_moveq_bra`: Short code fragment that loads 118 into d1, then jumps to another point in the code.
- `write_port_bits`: Writes specific byte values to a hardware port address, sets bit 0 of a register, and performs a loop that counts down a counter while writing values to the same port address.
- `frag_move_dbra_rts`: Short code fragment that copies 2400 to d3, then loops back while counting down d3, then returns.
- `initialize_and_setup_data`: Initializes various data fields including memory locations and structure offsets, performs comparisons with thresholds, and calls other routines to set up and process data structures based on conditional logic.
- `frag_addq_tst_bne_dbra_moveq_rts`: Short code fragment that adds 8 to a1, then tests a fixed memory address, then branches if the last result was not zero, then loops back while counting down d0, then loads 0 into d0, then returns.
- `frag_subq_bmi_move_bsr_bra`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was negative, then copies a fixed memory address to the stack, then calls another routine, then copies the top of the stack to d0, then calls another routine, then jumps to another point in the code.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `validate_and_compare_words`: Loads specific word values into registers, compares word values from memory with these registers, and branches based on comparison results to handle different validation cases.
- `frag_lea_moveq_bra`: Short code fragment that loads the address of a location in the code into a2, then loads 15 into d0, then jumps to another point in the code.
- `compare_and_adjust_pointer`: Compares word values from memory locations, adjusts a pointer based on comparison results, and loops through a sequence of operations until a counter reaches zero.
- `combine_and_shift_words`: Combines two word values from memory with a shift operation, performs bitwise OR on them, stores the result in memory, and loops through a counter while performing these operations.
- `initialize_and_process_structure`: Initializes various fields of a structure with specific values, performs bitwise operations on data from memory locations, and calls multiple routines to process and set up different parts of the structure.
- `initialize_structure_values`: Initializes various fields of a structure pointed to by a6 with specific values including word and longword constants, and sets up pointers and flags based on bit tests in the structure.
- `frag_movem_move_lsr_bsr_rts`: Short code fragment that saves registers on the stack, then copies d0 to d3, then shifts d0 right by 1, then calls another routine, then copies d3 to d0, then calls another routine (2 times in a row), then restores saved registers from the stack, then returns.
- `modify_and_store_bits`: Processes a longword value through bitwise operations, checks a bit in memory, clears a specific bit if the condition is met, and stores the modified value back to memory.
- `frag_move_lsr_subq_eor_dbra_andi_rts`: Short code fragment that copies d2 to the stack, then shifts d1 right by 2, then copies the memory at a0 (advancing a0) to d0, then subtracts 2 from d1, then copies the memory at a0 (advancing a0) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with an address, then copies the top of the stack to d2, then returns.
- `frag_move_btst_bne_bset_bra_bclr_rts`: Short code fragment that copies the memory at a0 to d0, then tests bit 6 of d0, then branches if the last result was not zero, then tests bit 0 of the memory at an offset from a0, then branches if the last result was not zero, then sets bit 7 of d0, then jumps to another point in the code, then clears bit 7 of d0, then copies d0 to the memory at a0, then returns.
- `read_memory_indirect`: Returns a 32-bit value read from a memory location calculated by adding an offset to a register.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `set_memory_pattern`: Writes a specific sequence of bytes into a memory range and updates a global variable.
- `set_audio_registers`: Checks a global variable and, if it does not match a specific value, performs a loop to decrement a counter and writes several values to audio-related hardware registers and certain memory locations.
- `setup_data_structures_2`: Writes a series of specific 16-bit and 32-bit values into two different memory locations using a loop.
- `insert_into_list`: Searches a memory range for a specific marker and, if not found, inserts a 32-bit value and its preceding word into the list.
- `wait_for_marker`: Enters a loop that repeatedly checks a memory location until a specific 16-bit value is encountered.
