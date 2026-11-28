# Game 1548 (helter skelter (1990)(audiogenic)[cr skr][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01548). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916080 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 242 raw-word lines |
| Functions named and described | pass, 196 of 196 named (11 author symbols), 196 of 196 described (18 by rule, 111 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3121 of 3121 (100.00%) |
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
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:Type | 2 | 434 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3103 | 65 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs.img (raw code file, load address unknown) | 1 | 3650 | 98 |

## Cases handled

- 2785 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 2785 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- helter skelter (1990)(audiogenic)[cr skr][a2]/Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs.img: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 address
- 1 never-loaded region(s), 326122 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `increment_and_store_loop` (runtime segment an address): Continuously increments a short integer value starting at offset 384 of a global structure and stores the result back to the same location, creating an infinite loop that never exits. Kind: checksum/data integrity trap. Pass path: the loop must complete without memory corruption or illegal instruction triggers, typically leading to the next stage of the loader.
- `process_input_data` (runtime segment an address): Processes input data by calling multiple routines based on a global variable's state, and returns a value from one of those calls when the variable is non-zero. Kind: password/manual lookup. Pass path: the branch where DAT_0000089b is non-zero.
- `initialize_and_store` (hs.img (raw code file, load address unknown)): Runs a helper routine three times, stores a fixed value and a hardware address into global variables, then calls another routine three times. Kind: disk hardware check. Pass path: CIAB_PRB value is valid and matches expected pattern.
- `frag_move_rts` (hs.img (raw code file, load address unknown)): Short code fragment that copies 255 to a fixed memory address, then copies 135 to a fixed memory address, then copies 255 to a fixed memory address, then returns. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.
- `frag_move_rts_2` (hs.img (raw code file, load address unknown)): Short code fragment that copies 119 to a fixed memory address, then returns. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.
- `set_port_bits_and_loop` (hs.img (raw code file, load address unknown)): Sets specific bits in a hardware register through memory addresses, writes byte values to memory locations, calls another routine, and loops back while decrementing a counter before writing a final value to memory. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.

## Functions


### Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: Copies sixteen longwords from a fixed offset in a table into a caller-supplied buffer when that buffer pointer is set, then calls through a routine pointer.
- `update_entity_state_and_process`: Increments an index in a structure, clears several fields, and performs a series of validation and attribute updates by calling a routine that checks entity states against various tables.
- `branch_to_offset_short`: Branches to an offset of 508.
- `set_negative_one_and_branch`: Sets d1 to negative one, stores d1 at offset 612 of the structure in a2, and branches to another routine.
- `set_negative_one_and_branch_a1`: Sets d1 to negative one, stores d1 at offset 532 of the structure in a1, and branches to another routine.
- `calculate_offset_and_call`: Calculates an offset by adding a value from offset 220 of the structure in a1 to a shifted value from offset 224 of the structure in a1, fetches a byte from a0 at that offset, loads a pointer from offset 304 of the structure in a2 into a3, sets d1 to a shifted version of d1, sets d0 to 240, loads a pointer from offset 328 of the structure in a2 into a4, calls the routine at a5, and branches to another routine.
- `dispatch_by_byte_value`: Calculates an offset using values from offset 220 and 224 of the structure in a1, fetches a byte from a0 at that offset, loads a pointer from offset 300 of the structure in a2 into a4, calls the routine at a5, and then branches to another routine, a routine that sets d1 to 78, or a routine that sets d1 to 72 based on the value in d1.
- `validate_and_update_entity_state`: Increments a counter at offset 224 of a structure in a1 and checks a byte from a table to determine if an entity is valid.
- `initialize_and_call_multiple`: Sets d1 to 1 and stores it at offset 616 of the structure in a2, then performs a series of calls using pointers from the structure in a2 and stores results in the structure in a1 before branching to another routine.
- `branch_to_offset`: Branches to an offset of 792.
- `call_indirect_routine`: Loads the value 232 into d0, loads a pointer from offset 336 of the structure in a2 into a4, calls the routine at the address in a5, and then jumps to the address in a6.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs

- `start`: Program entry of fifty-five instructions that allocates and loads memory, clears the color registers, sets the display window and bitplane registers, then runs the picture load, fade, copper list and wait routines in sequence.
- `copy_transfer_code_then_call`: Copies two longwords from the transfer routine into a global area and then calls the copied code through that area.
- `trans1`: Four-instruction routine whose address is taken by a PC-relative load and called by one unnamed routine.
- `loadfile`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.
- `dopiccy`: Loads a file, runs the fade-down routine, and decompresses the picture.
- `dopiccy1`: Decompresses the picture, then runs sixteen steps, each waiting for vertical blank and calling the fade step.
- `waitabit`: Waits for vertical blank in a counted loop of about 150 steps by calling a wait routine on each pass.
- `decompress`: Walks a chunked picture file looking for a body chunk, converting a color map chunk of sixteen entries from byte triplets into palette words, then decodes the run-length compressed body into the picture buffer.
- `fadedown`: The routine waits for scanlines and updates the palette fade repeatedly.
- `dofade`: The routine reads 32 palette entries from chip memory and writes them to the Amiga color registers.
- `wvbl`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `makecoplist`: Fills in pairs of copper move instructions for a series of bitplane pointer registers at a fixed stride, ends the list with wait and end markers, and stores the list address in a global variable.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, processes it through bit manipulation, and updates various global variables based on specific byte values.
- `compare_with_threshold`: Compares a global variable with data from a table and performs conditional operations based on the comparison results, using flags and loop controls to determine execution flow.
- `update_joystick_positions`: Reads joystick and mouse input data, calculates position differences, updates global variables with movement values, and sets flags based on button states and input conditions.
- `clear_global_word`: Sets a global variable to zero using multiple instructions.
- `clear_global_long`: Sets a global variable to zero using a sequence of instructions.
- `update_calculated_values`: Performs arithmetic operations using values from a table and stores the results into a global variable.
- `process_sprite_list`: Iterates through a list of pointers, performing memory copies from source addresses to destination addresses based on values stored in the list.
- `blit_transformed_sprites`: Copies sprite data from a source table to a destination buffer using bitwise shifts and masks to perform scaling or sub-pixel positioning.
- `fill_memory_block`: Copies a 32-bit value from a register to an array of 8000 elements, incrementing the destination pointer after each write operation.
- `transform_memory_values`: Iterates through a memory block, masking each word with a specific bitmask and multiplying the result by two.
- `copy_memory_blocks_2`: Calls another routine and then performs two large block memory copies to different destination addresses.
- `copy_memory_segments`: Copies data between three memory regions, writing 1999 elements from each of three source addresses to corresponding destinations in two other memory areas.
- `find_zero_word`: Traverses a sequence of words starting at a fixed address until it finds a zero word, advancing the pointer through the sequence with each iteration.
- `update_counter_and_process`: Decrements a counter and if it reaches zero, processes data from a buffer by reading values and updating pointers in a structure.
- `apply_xor_mask_to_memory`: Performs a bitwise XOR operation between values from a global table and a source table, then writes the results to two different memory destinations.
- `apply_conditional_xor_to_memory`: Calculates a value using a bitwise XOR and a conditional mask, then writes the resulting 32-bit word to two different memory destinations.
- `process_bcd_value`: Performs BCD arithmetic on a value and updates a memory location based on a comparison.
- `repeat_routine_eight_times`: Executes another routine eight times in a loop.
- `update_display_data`: Decrements a global variable and performs bitwise operations to write data to two separate memory buffers.
- `apply_xor_mask_to_memory_2`: Calls two routines and then performs a loop that applies a bitwise XOR operation to data and writes the results to two separate memory destinations.
- `apply_dual_destination_xor_transform`: Iterates through a table to perform bitwise XOR operations on data using a mask and writes the resulting 32-bit words to two separate memory locations.
- `apply_masked_xor_to_dual_buffers`: Calculates a bitwise XOR between a global value and a mask, then distributes the resulting 32-bit word across four specific offsets in two separate memory buffers.
- `process_byte_sequence`: Processes a sequence of bytes from a buffer, skipping zero values, and calls another routine for each non-zero byte while managing register state.
- `compare_and_set_flags`: Compares several word values against thresholds and sets processor flags based on comparison results, returning one of the input values depending on which condition is met.
- `process_relocation_list`: Iterates through a list of pointers ending with a negative value, calling another routine for each entry.
- `update_tile_flags`: Processes tile data based on input parameters to modify bit flags in memory locations, with conditional logic determining which bits are set or cleared.
- `copy_data_with_expansion`: Performs a nested loop to move and expand data between memory locations based on signed byte values.
- `clear_memory_table`: Fills a series of word-sized entries in a table with zero.
- `initialize_data_structure`: Writes a series of constant values and pointers into a memory location.
- `enter_infinite_loop`: Executes an infinite loop that prevents further code execution.
- `execute_infinite_loop`: The routine performs a series of operations that result in an infinite loop, preventing further execution.
- `copy_and_transform_memory_blocks`: Copies data from a source table to a destination while applying specific offsets and performs multiple memory block transfers.
- `setup_eight_repeats`: Loads a pointer to an offset of a global structure into a register, loads a word from a global variable into another register, and branches to another routine.
- `branch_to_offset_logic`: Loads a pointer to an offset of a global structure into a register, moves a word from a global variable into a register, and branches to another routine.
- `setup_eight_repeats_alt`: Loads a pointer to an offset of a global structure into a register, loads a word from a global variable into another register, and branches to another routine.
- `branch_to_offset_logic_alt`: Loads a pointer to an offset of a global structure into a register, moves a word from a global variable into a register, and branches to another routine.
- `decrement_and_clear`: Decrements a counter in a global structure and clears multiple memory locations with zero or specific values when the counter reaches zero.
- `update_animation_data`: Decrements a global counter and processes animation data from a buffer when the counter reaches zero, updating values in a structure.
- `update_position_and_collision`: Processes position coordinates and performs collision detection with boundaries, updating position and velocity values based on comparison results.
- `load_and_adjust_pointer`: Loads a value from memory, adjusts an address based on offset values, and copies data from a calculated location to another memory area.
- `frag_movea_lea_bra`: Short code fragment that copies a fixed memory address to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `compare_coordinate_ranges`: Compares coordinate values against ranges defined in memory, returning different status codes based on whether the comparisons fall within specified bounds.
- `scale_and_update_value`: Scales a value by dividing and multiplying with constants, updates a memory location with the result, and sets a flag to indicate completion.
- `update_position_and_velocity`: Updates position and velocity components based on input conditions and global state, then adjusts related counters and flags accordingly.
- `process_collision_data`: Processes collision-related data by loading word values from memory locations, performing arithmetic operations on them, and applying lookup table transformations based on the results.
- `check_pointer_range`: Increments a pointer by an offset and returns if the pointer does not match a specific constant value.
- `invert_position_components_2`: Inverts the sign of three signed values located at specific offsets in a data structure, then increments a counter until it reaches twelve.
- `invert_position_components`: Inverts the sign of three signed values located at specific offsets in a data structure, then increments a counter until it reaches twelve, performing this sequence multiple times before clearing a global variable and returning.
- `update_position_values`: Updates position-related values in memory by performing arithmetic operations on data from global variables and lookup tables.
- `increment_global_counter`: Increments a counter in a global variable while traversing a sequence of integers until it encounters a negative value, then returns the incremented counter value.
- `update_position_vector`: Processes a bit sequence to determine a range, calculates vector components using multiplication and accumulation, and updates position data based on table lookups.
- `initialize_conditional_structures`: Checks a global variable and populates one of two different data structures with specific constant values and offsets.
- `build_lookup_table`: Iterates through a list of values to populate a table with calculated offsets and data derived from the list entries.
- `reset_counter_if_match`: Calls another routine, checks if a specific value in a structure matches a constant, and if so, clears a 16-bit counter field in that same structure before returning.
- `process_string_buffer`: Iterates through a null-terminated string at a fixed address and calls another routine for each character.
- `process_string_characters_3`: Iterates through a null-terminated string, calling another routine for each character while incrementing a counter, and returns immediately after processing all characters.
- `update_collision_flags`: Updates collision detection flags based on object positions and thresholds, then processes table entries and adjusts BCD values in a memory region.
- `update_data_structure`: Updates a data structure by copying values from another structure, performing arithmetic operations on memory locations, and setting specific flags based on comparison results before returning.
- `initialize_game_state`: Initializes game state variables and clears specific memory locations before proceeding with further setup.
- `increment_and_store_loop`: Continuously increments a short integer value starting at offset 384 of a global structure and stores the result back to the same location, creating an infinite loop that never exits.
- `wait_for_pointer_match`: Enters a loop that continues until a specific memory address matches a global variable.
- `process_input_data`: Processes input data by calling multiple routines based on a global variable's state, and returns a value from one of those calls when the variable is non-zero.
- `search_string_for_byte`: Iterates through a memory buffer looking for a specific byte and returns a value from an offset if found.
- `iterate_string_sequence`: Iterates through a null-terminated byte sequence and calls another routine for each byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_tables`: Initializes several memory tables with pointers and values, while configuring audio hardware, DMA, and interrupts.
- `traverse_linked_list_3`: Traverses a structure of pointers and copies data to a destination, while interacting with audio and interrupt hardware registers.
- `setup_data_structures`: Populates two different tables with specific values and pointers, and modifies several global variables.
- `update_table_entry`: Searches a table for a specific pointer and, if not found, updates a table entry with a value and its preceding word.
- `initialize_function_pointer_table`: Populates a table with alternating entries of a routine address and a constant value.
- `wait_for_pointer`: Enters a loop that waits until a specific pointer is found within a table.
- `update_list_with_mask`: Iterates through a table and copies entries to a destination, performing bitwise shifts or arithmetic on data based on a threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: Reads a byte from a global variable and branches based on its sign, but the body is unreachable because the preceding instruction always jumps away.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs.img (raw code file, load address unknown)

- `initialize_display_and_process_blocks_2`: Initializes the custom chip registers and sets up display-related state, then processes a sequence of data blocks by copying, transforming, and storing their contents.
- `wait_for_input_condition`: Polls a bit in a memory location while comparing a global variable against another value.
- `clear_global_variables`: Assigns zero to six consecutive global variables in a row and then returns.
- `clear_more_globals`: Assigns zero to eight consecutive global variables in a row and then returns.
- `update_calculated_values_2`: Performs arithmetic operations using values from a table and stores the results into a global variable.
- `process_sprite_list_2`: Iterates through a list of pointers, performing memory copies from source addresses to destination addresses based on values stored in the list.
- `blit_transformed_sprites_2`: Copies sprite data from a source table to a destination buffer using bitwise shifts and masks to perform scaling or sub-pixel positioning.
- `copy_word_to_table`: It copies a 32-bit value from a register into consecutive 32-bit words of a table, decrementing a counter until the whole block is written.
- `mask_and_double_words`: It walks a table of 16-bit words, masking each value and doubling the result in place, repeating until every entry is processed.
- `copy_memory_blocks_3`: Calls another routine and then performs two large block memory copies to different destination addresses.
- `split_and_copy_digits`: Splits a global value into two decimal digits, then copies a block of 32-bit words to two destinations and calls a routine with those digits.
- `split_modulo_digits`: Computes the digits of a global variable's value modulo 512, then stores the tens digit in one location and the units digit in another.
- `copy_memory_block`: Copies 8000 consecutive words from one memory region to two other regions of the same length, advancing all three pointers together as it goes.
- `copy_memory_triple_variable`: Copies data from a source memory location to two separate destination locations for a specified number of iterations.
- `find_null_terminator`: Iterates through a table until a null or zero-valued word is encountered.
- `update_counter_and_process_2`: Decrements a word counter at offset 26 of the structure in a5 and if it reaches zero, reads data from a buffer pointer at offset 22 of the structure, updates the buffer pointer, and stores a processed value at offset 18 of the structure.
- `apply_xor_mask_to_memory_3`: Performs a bitwise XOR operation between values from a global table and a source table, then writes the results to two different memory destinations.
- `apply_conditional_xor_to_memory_2`: Calculates a value using a bitwise XOR and a conditional mask, then writes the resulting 32-bit word to two different memory destinations.
- `process_bcd_value_2`: Performs BCD arithmetic on a value and updates a memory location based on a comparison.
- `repeat_routine_eight_times_2`: Executes another routine eight times in a loop.
- `scatter_bcd_after_delay`: It decrements a global counter on each call and, once the counter reaches zero, derives a BCD-adjusted value from it.
- `scatter_and_xor_bytes`: It calls a helper routine twice, then loops eight times combining a value with table entries and scattering the resulting bytes into two destination blocks, masking each byte as it writes.
- `scatter_xor_two_blocks`: Combines a 32-bit value with bytes read from a table and scatters the resulting bytes into two destination blocks, repeating for a fixed number of iterations.
- `scatter_xor_table_bytes`: Computes a value from a global variable and uses it to index a lookup table, then for each of seven iterations XORs a 32-bit table entry with a fixed mask and scatters its four bytes into two destination blocks at fixed offsets.
- `process_string_bytes`: Walks a null-terminated string one byte at a time, invoking another routine for each character until the terminator is reached.
- `compare_and_set_flags_2`: Compares several word values against thresholds and sets processor flags based on comparison results, returning one of the input values depending on which condition is met.
- `iterate_short_sequence`: Walks a sequence of 16-bit values until a negative entry is found, invoking another routine for each value.
- `set_planar_pixel_bits`: Computes a destination pointer from a base table and a source offset, then clears a block of bytes for a negative count or fills it with 255 for a positive count.
- `copy_data_with_expansion_2`: Performs a nested loop to move and expand data between memory locations based on signed byte values.
- `clear_word_table`: Clears a sequence of 16-bit words in a global table by repeatedly storing zero and advancing the pointer until the loop count reaches zero.
- `init_zero_sequence`: Finds the first zero entry in a stride-advancing 16-bit sequence, then stores several fixed values into the located structure.
- `enter_infinite_loop_2`: Executes an infinite loop that prevents further code execution.
- `empty_infinite_loop_2`: Consists solely of an infinite loop and does not touch any data.
- `copy_memory_blocks`: Copies 8000 consecutive words from one memory region to three other regions of the same length, advancing all four pointers together.
- `invoke_scatter_seven_times`: It invokes a scatter-and-xor helper routine seven times in a loop.
- `scatter_transform_counter`: It transforms a global value by swapping its high and low halves, feeds the result to a scatter-and-xor helper eight times, then reverses the decimal digits of a counter until the counter reaches zero.
- `run_scatter_xor_loop`: Loops eight times, each iteration invoking the scatter-and-xor helper routine to process one element.
- `process_value_digit_by_digit_again`: It repeatedly splits a global value into high and low halves, passing each half to a scatter helper while reordering the digits of a counter.
- `update_pointer_value`: Selects a word from one of two memory locations based on a condition and stores it in a global variable.
- `frag_movea_lea_bra_2`: Short code fragment that copies a fixed memory address to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `validate_and_process_input`: Processes input values from a buffer by reading word values, performing comparisons with thresholds, updating pointers in a structure, and returning a validation result based on comparison outcomes.
- `validate_and_update_input`: Validates input values from a buffer by reading word values, performing comparisons with thresholds, updates pointers in a structure, and processes data through multiple arithmetic operations before returning.
- `toggle_position_signs`: Negates three longword values located at offsets 10, 14, and 2412 relative to a5, then increments a counter in d7 and branches based on its value.
- `update_position_values_2`: Reads position data from memory locations, performs arithmetic operations with shift and add operations, updates values in a structure at offsets 2 and 6, and compares the results with input values to determine if they match.
- `update_and_insert_list`: Updates position values in a structure, performs bit tests and operations on memory locations, calls another routine to insert data into a list, and modifies values based on input comparisons.
- `update_position_and_multiply`: Updates position values in a structure by performing bit tests and operations on memory locations, then calls another routine to insert data into a list, followed by multiplication and addition operations on register values.
- `initialize_conditional_structures_2`: Checks a global variable and populates one of two different data structures with specific constant values and offsets.
- `build_lookup_table_2`: Iterates through a list of values to populate a table with calculated offsets and data derived from the list entries.
- `iterate_null_terminated_block_2`: It calls a helper once, then walks a null-terminated block of memory, passing a running counter and a fixed value to another routine for each byte.
- `iterate_null_terminated_block`: Walks a null-terminated block of memory byte by byte, invoking another routine for each byte while incrementing a counter and advancing the pointer until the terminator is reached.
- `initialize_display_and_process_blocks`: Initializes the custom chip registers and sets up display-related state, then processes a sequence of data blocks by copying, transforming, and scattering their contents across memory.
- `empty_spin_loop`: Spins in a loop that compares an incremented global value against itself and then returns without modifying anything.
- `wait_for_input_and_return`: Waits for a button bit to be set and a counter to reach a global value, then branches on a global flag to return a value from another routine.
- `find_char_in_string`: Walks through a null-terminated string until it finds a matching character, then returns the byte located at a fixed offset from that position.
- `process_string_characters`: It calls a helper three times, then walks a null-terminated string byte by byte, invoking another routine for each character.
- `copy_masked_buffer`: Copies words from a source buffer into a destination buffer at spaced offsets, then applies a computed mask to several locations.
- `apply_bitmask_to_table`: Applies bitwise AND operations to specific word entries in a large table using values from registers.
- `copy_words_to_buffer`: Copies four consecutive words from a fixed source into four spaced-out destinations within a buffer, advancing the source and interleaving the destination pointers each pass.
- `transfer_memory_blocks`: Iterates through a list of descriptors to copy blocks of data from one memory range to another.
- `increment_routine_result`: Reads a value from another routine and returns that value incremented by one.
- `split_and_call_digits`: Runs a helper routine twice, splits a global value into two decimal digits, then calls a routine with those digits.
- `call_routine_four_times`: Invokes another routine four times in sequence and then returns.
- `process_string_characters_2`: Runs a helper routine three times, then walks a null-terminated string invoking a character routine for each byte.
- `clamp_value_in_loop`: Loops until a global value overflows, then repeatedly clears a global, branches on an input flag, and clamps a computed value between two constants.
- `fill_pattern_buffer`: Calculates two values from a global variable and stores them in a table, then performs a nested loop to fill a large memory area with a repeating pattern of words.
- `iterate_short_sequence_3`: It walks a sequence of 16-bit values until a negative entry is found, invoking another routine for each value.
- `iterate_short_sequence_2`: Walks a negative-terminated sequence of 16-bit values, invoking another routine for each value.
- `copy_shifted_block_2`: Copies a block of 16-bit values from a source region to a destination region, using a lookup table to derive a bit mask and shift amount.
- `copy_shifted_rows`: Loads a source pointer and a destination pointer from a global variable, then loops over four stacked screen rows copying 16-bit words from the source to the destination while masking and shifting each pair of nibbles using a lookup table.
- `initialize_and_store`: Runs a helper routine three times, stores a fixed value and a hardware address into global variables, then calls another routine three times.
- `frag_move_rts`: Short code fragment that copies 255 to a fixed memory address, then copies 135 to a fixed memory address, then copies 255 to a fixed memory address, then returns.
- `frag_move_btst_beq_moveq_bsr_dbra_clr_r`: Short code fragment that copies 255 to d7, then tests bit 4 of a fixed memory address, then branches if the last result was zero, then loads 1 into d0, then calls another routine, then loops back while counting down d7, then clears a fixed memory address, then returns.
- `update_and_compare_values`: Reads a word value from memory, compares it with a register value, updates the memory value if they don't match, and performs conditional operations based on the comparison result.
- `frag_move_rts_2`: Short code fragment that copies 119 to a fixed memory address, then returns.
- `frag_moveq_bra`: Short code fragment that loads 118 into d1, then jumps to another point in the code.
- `set_port_bits_and_loop`: Sets specific bits in a hardware register through memory addresses, writes byte values to memory locations, calls another routine, and loops back while decrementing a counter before writing a final value to memory.
- `frag_move_dbra_rts`: Short code fragment that copies 2400 to d3, then loops back while counting down d3, then returns.
- `set_flags_and_process_data`: Sets flags in memory locations, initializes data structures with specific values, processes input through multiple routine calls, and performs conditional operations based on comparisons with memory values.
- `frag_addq_tst_bne_dbra_moveq_rts`: Short code fragment that adds 8 to a1, then tests a fixed memory address, then branches if the last result was not zero, then loops back while counting down d0, then loads 0 into d0, then returns.
- `frag_subq_bmi_move_bsr_bra`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was negative, then copies a fixed memory address to the stack, then calls another routine, then copies the top of the stack to d0, then calls another routine, then jumps to another point in the code.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `validate_and_compare_words`: Loads word values from memory locations, compares them with specific constants, and branches to different code paths based on the comparison results, ultimately returning a value in a register.
- `frag_lea_moveq_bra`: Short code fragment that loads the address of a location in the code into a2, then loads 15 into d0, then jumps to another point in the code.
- `compare_and_shift_values`: Compares word values from memory locations, performs conditional subtraction and shifts based on comparison results, then branches to different code paths depending on the outcome.
- `bitwise_combine_values`: Performs bitwise operations on word values from memory locations, combines them using shift and or operations, stores the result in memory, and loops back while decrementing a counter.
- `initialize_and_process_structure`: Initializes various fields of a structure with specific values, copies pointers from memory locations, performs bitwise operations on word values, and calls multiple routines to process data.
- `initialize_structure_values`: Initializes various fields of a structure pointed to by a6 with specific values including word and longword constants, copies pointers from input registers, and sets conditional values based on bit tests.
- `frag_movem_move_lsr_bsr_rts`: Short code fragment that saves registers on the stack, then copies d0 to d3, then shifts d0 right by 1, then calls another routine, then copies d3 to d0, then calls another routine (2 times in a row), then restores saved registers from the stack, then returns.
- `bit_manipulate_and_store`: Performs bitwise operations on a longword value, manipulates bits in the result, checks a specific bit in memory, clears a bit if the condition is met, and stores the final value to memory.
- `frag_move_lsr_subq_eor_dbra_andi_rts`: Short code fragment that copies d2 to the stack, then shifts d1 right by 2, then copies the memory at a0 (advancing a0) to d0, then subtracts 2 from d1, then copies the memory at a0 (advancing a0) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with an address, then copies the top of the stack to d2, then returns.
- `frag_move_btst_bne_bset_bra_bclr_rts`: Short code fragment that copies the memory at a0 to d0, then tests bit 6 of d0, then branches if the last result was not zero, then tests bit 0 of the memory at an offset from a0, then branches if the last result was not zero, then sets bit 7 of d0, then jumps to another point in the code, then clears bit 7 of d0, then copies d0 to the memory at a0, then returns.
- `read_structured_dword`: It returns a 32-bit value read from a fixed offset in a structure passed in a register, acting as a simple accessor for that field.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `init_channel_pointer_table`: It writes a fixed 32-bit value into consecutive slots of a pointer table, then writes a small byte into the following slot, repeating this for four slots.
- `set_audio_registers`: Checks a global variable and, if it does not match a specific value, performs a loop to decrement a counter and writes several values to audio-related hardware registers and certain memory locations.
- `setup_data_structures_2`: Writes a series of specific 16-bit and 32-bit values into two different memory locations using a loop.
- `insert_into_list`: Searches a memory range for a specific marker and, if not found, inserts a 32-bit value and its preceding word into the list.
- `wait_for_marker`: Enters a loop that repeatedly checks a memory location until a specific 16-bit value is encountered.
