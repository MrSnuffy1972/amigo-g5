# Game 1548 (helter skelter (1990)(audiogenic)[cr skr][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01548). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916080 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 241 raw-word lines |
| Functions named and described | pass, 166 of 166 named (11 author symbols), 166 of 166 described (5 by rule, 94 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3121 of 3121 (100.00%) |
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
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:Type | 2 | 363 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3103 | 65 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs.img (raw code file, load address unknown) | 1 | 2747 | 68 |

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
- `wait_for_input`: Waits until a bit in a byte at a pointer plus five becomes set, then loops until a counter at a pointer plus six reaches a global variable.
- `clear_global_variables`: Assigns zero to six consecutive global variables in a row and then returns.
- `clear_more_globals`: Assigns zero to eight consecutive global variables in a row and then returns.
- `multiply_and_add_globals`: Multiplies a 32-bit value by two, adds it to another 32-bit value, and carries the result into a pair of global variables.
- `copy_and_process_blocks`: It copies a block of 16-bit values from a source region into a destination region using a helper routine, looping over several entries and adjusting pointers as it goes.
- `copy_shifted_block`: Reads a pair of nibbles from a lookup table and uses them as a bit mask and shift amount to copy a block of 16-bit values from one region into another.
- `copy_word_to_table`: It copies a 32-bit value from a register into consecutive 32-bit words of a table, decrementing a counter until the whole block is written.
- `mask_and_double_words`: It walks a table of 16-bit words, masking each value and doubling the result in place, repeating until every entry is processed.
- `interleave_and_copy_blocks`: It calls a helper routine, then interleaves four source blocks into four destination blocks in a fixed pattern.
- `split_and_copy_digits`: Splits a global value into two decimal digits, then copies a block of 32-bit words to two destinations and calls a routine with those digits.
- `split_modulo_digits`: Computes the digits of a global variable's value modulo 512, then stores the tens digit in one location and the units digit in another.
- `copy_memory_block`: Copies 8000 consecutive words from one memory region to two other regions of the same length, advancing all three pointers together as it goes.
- `copy_block_to_two_pointers`: It copies a block of 32-bit words from one table to two others, advancing all three pointers each iteration until the counter reaches zero.
- `find_zero_short_sequence`: It advances a pointer by a fixed stride, reading each 16-bit value, until a zero entry is found in the sequence.
- `scatter_xor_bytes`: It combines two 32-bit values with bytes read from a table and scatters the resulting bytes into several offsets of a destination block, repeating for a fixed number of steps.
- `scatter_xor_to_two_blocks`: It combines a 32-bit value with bytes read from a table and scatters the resulting bytes into two destination blocks, repeating for a fixed number of steps.
- `add_block_with_bcd_adjust`: It adds a running carry through a block of bytes, subtracting ten whenever the result exceeds nine, then adjusts a two-digit BCD value and stores it.
- `repeat_scatter_operation`: It invokes a helper routine eight times in a loop, performing the scatter-and-xor operation for each iteration.
- `scatter_bcd_after_delay`: It decrements a global counter on each call and, once the counter reaches zero, derives a BCD-adjusted value from it.
- `scatter_and_xor_bytes`: It calls a helper routine twice, then loops eight times combining a value with table entries and scattering the resulting bytes into two destination blocks, masking each byte as it writes.
- `scatter_xor_two_blocks`: Combines a 32-bit value with bytes read from a table and scatters the resulting bytes into two destination blocks, repeating for a fixed number of iterations.
- `scatter_xor_table_bytes`: Computes a value from a global variable and uses it to index a lookup table, then for each of seven iterations XORs a 32-bit table entry with a fixed mask and scatters its four bytes into two destination blocks at fixed offsets.
- `process_string_bytes`: Walks a null-terminated string one byte at a time, invoking another routine for each character until the terminator is reached.
- `iterate_short_sequence`: Walks a sequence of 16-bit values until a negative entry is found, invoking another routine for each value.
- `set_planar_pixel_bits`: Computes a destination pointer from a base table and a source offset, then clears a block of bytes for a negative count or fills it with 255 for a positive count.
- `copy_byte_block`: Copies a block of bytes from one memory location to another, advancing both pointers by the same amount each iteration.
- `clear_word_table`: Clears a sequence of 16-bit words in a global table by repeatedly storing zero and advancing the pointer until the loop count reaches zero.
- `init_zero_sequence`: Finds the first zero entry in a stride-advancing 16-bit sequence, then stores several fixed values into the located structure.
- `empty_infinite_loop`: Contains only an infinite loop and performs no operations on any data.
- `empty_infinite_loop_2`: Consists solely of an infinite loop and does not touch any data.
- `copy_memory_blocks`: Copies 8000 consecutive words from one memory region to three other regions of the same length, advancing all four pointers together.
- `invoke_scatter_seven_times`: It invokes a scatter-and-xor helper routine seven times in a loop.
- `scatter_transform_counter`: It transforms a global value by swapping its high and low halves, feeds the result to a scatter-and-xor helper eight times, then reverses the decimal digits of a counter until the counter reaches zero.
- `run_scatter_xor_loop`: Loops eight times, each iteration invoking the scatter-and-xor helper routine to process one element.
- `process_value_digit_by_digit_again`: It repeatedly splits a global value into high and low halves, passing each half to a scatter helper while reordering the digits of a counter.
- `store_table_block`: Selects one of two four-word blocks from a table based on a flag and stores the chosen words into a fixed location within a structure.
- `init_zero_sequence_flags`: Finds the first zero entry in a stride-advancing 16-bit sequence, then stores different fixed values into the located structure depending on a flag.
- `build_entries_from_sequence`: Walks a negative-terminated sequence of 16-bit values, building and storing entries into a structure for each one.
- `iterate_null_terminated_block_2`: It calls a helper once, then walks a null-terminated block of memory, passing a running counter and a fixed value to another routine for each byte.
- `iterate_null_terminated_block`: Walks a null-terminated block of memory byte by byte, invoking another routine for each byte while incrementing a counter and advancing the pointer until the terminator is reached.
- `initialize_display_and_process_blocks`: Initializes the custom chip registers and sets up display-related state, then processes a sequence of data blocks by copying, transforming, and scattering their contents across memory.
- `empty_spin_loop`: Spins in a loop that compares an incremented global value against itself and then returns without modifying anything.
- `wait_for_input_and_return`: Waits for a button bit to be set and a counter to reach a global value, then branches on a global flag to return a value from another routine.
- `find_char_in_string`: Walks through a null-terminated string until it finds a matching character, then returns the byte located at a fixed offset from that position.
- `process_string_characters`: It calls a helper three times, then walks a null-terminated string byte by byte, invoking another routine for each character.
- `copy_masked_buffer`: Copies words from a source buffer into a destination buffer at spaced offsets, then applies a computed mask to several locations.
- `mask_buffer_words`: Combines a 16-bit value with several spaced-out words in a large buffer, masking the low and high bytes separately and OR-ing in extra bits at two of the locations.
- `copy_words_to_buffer`: Copies four consecutive words from a fixed source into four spaced-out destinations within a buffer, advancing the source and interleaving the destination pointers each pass.
- `copy_table_blocks`: When a counter is non-zero, reads pairs of words from a source table and copies blocks of words from one address range to another, adjusting the pointers when they exceed a limit.
- `increment_routine_result`: Reads a value from another routine and returns that value incremented by one.
- `split_and_call_digits`: Runs a helper routine twice, splits a global value into two decimal digits, then calls a routine with those digits.
- `call_routine_four_times`: Invokes another routine four times in sequence and then returns.
- `process_string_characters_2`: Runs a helper routine three times, then walks a null-terminated string invoking a character routine for each byte.
- `clamp_value_in_loop`: Loops until a global value overflows, then repeatedly clears a global, branches on an input flag, and clamps a computed value between two constants.
- `store_doubled_values`: It reads two 32-bit values from fixed offsets in a global structure and stores each of them, doubled, into two other fixed offsets in a second structure.
- `iterate_short_sequence_3`: It walks a sequence of 16-bit values until a negative entry is found, invoking another routine for each value.
- `iterate_short_sequence_2`: Walks a negative-terminated sequence of 16-bit values, invoking another routine for each value.
- `copy_shifted_block_2`: Copies a block of 16-bit values from a source region to a destination region, using a lookup table to derive a bit mask and shift amount.
- `copy_shifted_rows`: Loads a source pointer and a destination pointer from a global variable, then loops over four stacked screen rows copying 16-bit words from the source to the destination while masking and shifting each pair of nibbles using a lookup table.
- `initialize_and_store`: Runs a helper routine three times, stores a fixed value and a hardware address into global variables, then calls another routine three times.
- `read_structured_dword`: It returns a 32-bit value read from a fixed offset in a structure passed in a register, acting as a simple accessor for that field.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `init_channel_pointer_table`: It writes a fixed 32-bit value into consecutive slots of a pointer table, then writes a small byte into the following slot, repeating this for four slots.
- `initialize_audio_channels`: It initializes the four audio channels by loading each channel's pointer and length registers from a table of addresses, then enables DMA and interrupts.
- `write_channel_descriptors`: It writes a fixed table of four channel descriptors, each holding a pointer, length and control word, into a channel table.
- `store_into_channel_slot`: It locates the next free slot in a table of channel pointers by scanning for a sentinel value, wrapping around at the end, then stores a pointer and its associated value into that slot.
- `scan_channel_table`: It scans a table of channel pointers looking for a specific sentinel value, advancing through a fixed number of entries before stopping.
