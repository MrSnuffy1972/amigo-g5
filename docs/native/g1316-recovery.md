# Game 1316 (helter skelter (1990)(audiogenic)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01316). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (916080 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 241 raw-word lines |
| Functions named and described | pass, 166 of 166 named (11 author symbols), 166 of 166 described (5 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3051 of 3051 (100.00%) |
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
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:Type | 2 | 363 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3103 | 65 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs.img (raw code file, load address unknown) | 1 | 2747 | 68 |

## Cases handled

- 2715 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 2715 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- helter skelter (1990)(audiogenic)[cr skr]/Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs.img: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 address
- 1 never-loaded region(s), 326122 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `increment_and_store_loop` (runtime segment an address): Continuously increments a short integer value starting at offset 384 of a global structure and stores the result back to the same location, creating an infinite loop that never exits. Kind: checksum/data integrity trap. Pass path: not determined.
- `process_input_data` (runtime segment an address): Processes input data by calling multiple routines based on a global variable's state, and returns a value from one of those calls when the variable is non-zero. Kind: password/manual lookup. Pass path: the branch where DAT_0000089b is non-zero.
- `configure_display_and_interrupt_registers` (hs.img (raw code file, load address unknown)): Writes display, copper, DMA, interrupt and port control registers while calling many routines, with part of the setup using a character value. Kind: disk hardware/track check. Pass path: the branch where register/state comparison matches or certain hardware flags (like an address or an address or an address) are cleared or set as expected, specifically avoiding the branch to an address/the conditional skip or the specific exit/failure branches, l.

## Functions


### Helter Skelter (1990)(Audiogenic)[cr SKR].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a destination pointer is non-null, copies sixteen longwords from a structure addressed through the caller's frame register into it, then calls a routine through a pointer.
- `update_entity_state_and_process`: Increments an index in a structure, clears several fields, and performs a series of validation and attribute updates by calling a routine that checks entity states against various tables.
- `branch_to_offset`: Performs a short branch to a relative offset.
- `set_and_store_negative_one`: Loads negative one into register d1, stores it at offset 584 of the structure in a2, and branches to another routine.
- `set_and_store_negative_one_offset`: Loads negative one into register d1, stores it at offset 52 of the structure in a1, and branches to another routine.
- `calculate_offset_and_call`: Calculates an offset by adding a value from offset 220 of the structure in a1 to a shifted value from offset 224 of the structure in a1, fetches a byte from a0 at that offset, loads a pointer from offset 304 of the structure in a2 into a3, sets d1 to a shifted version of d1, sets d0 to 240, loads a pointer from offset 328 of the structure in a2 into a4, calls the routine at a5, and branches to another routine.
- `dispatch_by_byte_value`: Calculates an offset using values from offset 220 and 224 of the structure in a1, fetches a byte from a0 at that offset, loads a pointer from offset 300 of the structure in a2 into a4, calls the routine at a5, and then branches to another routine, a routine that sets d1 to 78, or a routine that sets d1 to 72 based on the value in d1.
- `validate_and_update_entity_state`: Increments a counter at offset 224 of a structure in a1 and checks a byte from a table to determine if an entity is valid.
- `initialize_and_call_multiple`: Sets d1 to 1 and stores it at offset 616 of the structure in a2, then performs a series of calls using pointers from the structure in a2 and stores results in the structure in a1 before branching to another routine.
- `branch_to_another_routine`: Performs a short branch to a location within another routine.
- `call_and_jump_indirect`: Loads the constant negative forty into register d0, loads a pointer from offset 336 of the structure in a2 into a4, calls the address in a5, and jumps to the address in a6.

### Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs

- `start`: Allocates two memory buffers through the system library, clears the colour and display registers, builds a copper list and then loads and fades in a picture.
- `copy_trampoline_to_handler`: Copies a short block of longwords from a labelled data block into a handler location, then calls the routine now stored there.
- `trans1`: The decompiler produced no pseudo-code for this four-instruction block, which the caller copies to another location.
- `loadfile`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.
- `dopiccy`: Loads a file, fades the screen down, decompresses the picture, then runs sixteen vertical blank waits, each followed by a fade step.
- `dopiccy1`: Decompresses the picture, then runs sixteen vertical blank waits, each followed by a fade step.
- `waitabit`: Calls a wait routine in a counted loop to pass 150 vertical blank periods, then returns.
- `decompress`: Scans chunk headers for a colour map and a body block, converts the sixteen colour map entries into palette words, then unpacks the body into 200 rows using a signed run-length scheme.
- `fadedown`: Executes a routine that calls two other routines a fixed number of times.
- `dofade`: The routine reads 32 palette entries from chip memory and writes them to the Amiga color registers.
- `wvbl`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `makecoplist`: Writes four pairs of bitplane pointer moves into the copper list, advancing the pointer by 40 bytes per pair, then writes the end-of-list terminator.

### Helter Skelter (1990)(Audiogenic)[cr SKR].adf:bootblock

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
- `copy_memory_blocks`: Calls another routine and then performs two large block memory copies to different destination addresses.
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
- `execute_infinite_loop_2`: The routine performs a series of operations that result in an infinite loop, preventing further execution.
- `copy_and_transform_memory_blocks`: Copies data from a source table to a destination while applying specific offsets and performs multiple memory block transfers.
- `setup_loop_iteration_one`: Loads a pointer to an offset of a base address into a0, loads a word from a global variable into a3, and branches to another routine.
- `setup_loop_iteration_two`: Loads a pointer to an offset of a base address into a0, moves a word from a global variable into d0, and branches to another routine.
- `setup_loop_iteration_three`: Loads a pointer to an offset of a base address into a0, loads a word from a global variable into a3, and branches to another routine.
- `setup_loop_iteration_four`: Loads a pointer to an offset of a base address into a0, moves a word from a global variable into d0, and branches to another routine.
- `decrement_and_clear`: Decrements a counter in a global structure and clears multiple memory locations with zero or specific values when the counter reaches zero.
- `update_animation_data`: Decrements a global counter and processes animation data from a buffer when the counter reaches zero, updating values in a structure.
- `update_position_and_collision`: Processes position coordinates and performs collision detection with boundaries, updating position and velocity values based on comparison results.
- `load_and_adjust_pointer`: Loads a value from memory, adjusts an address based on offset values, and copies data from a calculated location to another memory area.
- `frag_movea_lea_bra`: Short code fragment that copies a fixed memory address to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `compare_coordinate_ranges`: Compares coordinate values against ranges defined in memory, returning different status codes based on whether the comparisons fall within specified bounds.
- `scale_and_update_value`: Scales a value by dividing and multiplying with constants, updates a memory location with the result, and sets a flag to indicate completion.
- `update_position_and_velocity`: Updates position and velocity components based on input conditions and global state, then adjusts related counters and flags accordingly.
- `process_collision_data`: Processes collision-related data by loading word values from memory locations, performing arithmetic operations on them, and applying lookup table transformations based on the results.
- `check_pointer_range`: Increments a pointer by 36 and returns if it does not match a specific constant value; otherwise it continues.
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
- `update_audio_buffer_list`: Iterates through a bitmask-controlled sequence of pointers to copy data into audio hardware registers.
- `setup_data_structures`: Populates two different tables with specific values and pointers, and modifies several global variables.
- `update_table_entry`: Searches a table for a specific pointer and, if not found, updates a table entry with a value and its preceding word.
- `initialize_function_pointer_table`: Populates a table with alternating entries of a routine address and a constant value.
- `wait_for_pointer`: Enters a loop that waits until a specific pointer is found within a table.
- `update_list_with_mask`: Iterates through a table and copies entries to a destination, performing bitwise shifts or arithmetic on data based on a threshold.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_undecoded_loop_block`: The decompiler produced no pseudo-code for this four-instruction block, which is entered at a loop head during the reference run.

### Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs.img (raw code file, load address unknown)

- `initialise_tables_and_registers`: The pseudo-code is truncated, but its visible part clears a 200-row offset table, fills a long table with stepped values, writes colour and display registers and calls several routines.
- `poll_flag_then_counter_threshold`: Waits for a flag bit in a structure addressed through the caller's frame register, then waits until a counter byte in that structure reaches the value held in a global, returning early in one branch when the threshold is met.
- `clear_six_global_values`: Sets six globals to zero in sequence and returns.
- `clear_eight_global_values`: Sets eight globals to zero in sequence and returns.
- `double_long_pair_into_globals`: Doubles a 32-bit value formed from a pair of input longs, propagating carries between the words, and stores the results in two global longs.
- `copy_rows_then_process_records`: Copies runs of words from one bitplane region into four interleaved bitplanes, wrapping at the buffer end.
- `shift_bitplane_span_with_masks`: Shifts a span of bitplane words by a bit offset taken from a table, combines them with masked neighbours and writes the shifted pixels into four bitplanes, wrapping at the buffer end.
- `fill_long_words_with_value`: Fills 8000 longwords starting at a pointer with a single 32-bit value.
- `double_sixteen_masked_words`: Masks each of sixteen words with a fixed bit pattern and doubles it in place.
- `copy_source_into_four_bitplanes`: Calls a routine, then copies a source buffer word by word into four bitplane regions.
- `format_offset_as_decimal_digits`: Calls a routine, derives a row and column from a global offset by dividing and taking remainders, converts the results into decimal digits and passes them to another routine.
- `calculate_pointer_mangling`: Calculates a value based on a global variable and calls a routine with a modified version of that value.
- `duplicate_bitplane_to_two_planes`: Copies 8000 longwords from one bitplane buffer into two destination buffers in parallel.
- `copy_longs_to_two_destinations`: Copies a block of longwords from a source into two destination buffers in parallel, with the count taken from a register.
- `find_first_free_record_slot`: Walks the eighteen-word record table from a fixed starting point until it reaches a record whose first word is zero, and leaves a pointer to that record.
- `blit_byte_pattern_to_planes`: Combines two longword pattern values from a table with a mask derived from a second table entry, then writes the resulting bytes into four bitplanes over eight rows, advancing 40 bytes per row.
- `write_masked_pattern_to_planes`: Blends a table longword with a stored pattern through a mask and writes its four bytes into four bitplane rows over eight rows, 40 bytes apart.
- `add_decimal_digits_with_carry`: Adds a digit string held in a buffer with decimal carry, subtracting ten on overflow.
- `call_routine_eight_times`: Calls a routine eight times in a counted loop and returns.
- `tick_bcd_counter_draw_digit`: Decrements a global counter and returns while it is nonzero.
- `draw_digit_pattern_to_planes`: Calls two routines, then combines a fixed mask word with a table pattern selected by the low nibble of a global and writes the bytes into two bitplane regions over eight rows.
- `draw_digit_pattern_at_offset`: Calls three routines, then writes a table-selected pattern into two bitplane regions over eight rows, at a different screen offset from its neighbours.
- `draw_row_digit_pattern`: Derives a value from a global position, passes it through two routine calls, then writes a table-selected digit pattern into two bitplane regions over eight rows.
- `call_routine_per_string_char`: Walks a zero-terminated byte string, calling a routine for each character until reaching zero, then returns zero.
- `call_routine_per_nonnegative_word`: Walks a list of words, calling a routine for each entry while the entry is non-negative.
- `fill_bitplane_bit_pattern`: Writes solid bytes into a block of rows in one bitplane, then sets or clears single bits across four bitplanes according to a column pattern, with the choice depending on the sign of a row count.
- `unpack_run_length_picture_rows`: Unpacks a run-length encoded picture into bitplane data over 200 rows, copying literal bytes or repeating a byte by count and advancing the output pointer between row groups.
- `clear_record_first_words`: Sets the first word of each of sixteen eighteen-word records in a global table to zero.
- `initialise_record_defaults`: Calls a routine, then fills in a record through its pointer: sets one field to twelve, stores two longword fields as fixed constants and sets a flag field to one.
- `loop_forever_stub`: Contains only an infinite loop with no work, so the routine never returns.
- `execute_infinite_loop`: Performs an infinite loop that prevents further execution.
- `copy_table_block_to_planes`: Selects a table block using a routine's result modulo eight, copies 320 longwords from it into bitplane regions at fixed offsets, then clears several globals and fills a record list from a table.
- `execute_eight_times`: Calls another routine a fixed number of times using a loop.
- `emit_decimal_places_of_global`: Splits a 16-bit global into decimal places by repeated division by ten, calling a routine for each place.
- `execute_routine_eight_times`: Executes another routine or a block of code in a loop that runs exactly eight times.
- `iterate_and_transform_value`: calculates a new value by performing a division and remainder-based bit-swap and calls another routine with that value, while decrementing a counter via a division-based update.
- `select_table_long_into_record`: Sets a small status word in a record, then selects one of two longwords from a table based on a flag in the record and the sign of another field, and stores it into the record.
- `fill_descriptor_records_from_globals`: When a global is zero, calls a routine and fills a descriptor structure with fixed constants and values read from globals.
- `build_record_list_from_entries`: Walks a list of words until one is negative, filling a record for each entry with a fixed type value, an index from the entry's high nibble and a position computed from its low byte and a second word, then calls routines to finish each record.
- `draw_string_from_fixed_origin`: Calls a routine once, then walks a zero-terminated string at a fixed address, calling a routine for each character with a column value that increases from a fixed starting value.
- `draw_string_from_register_origin`: Walks the same zero-terminated string at a fixed address, calling a routine per character with a value taken from a register that increases by one for each character.
- `configure_display_and_interrupt_registers`: Writes display, copper, DMA, interrupt and port control registers while calling many routines, with part of the setup using a character value.
- `spin_until_zero_offset`: Loops with no body until adding a register value to a global leaves the global unchanged, which happens only when the register value is zero, then returns.
- `call_optional_routine_return_result`: Calls two routines, then calls a third only when a flag global is nonzero, returning that routine's result; otherwise it returns the register value unchanged.
- `lookup_byte_in_table_offset`: Scans a list of bytes for one matching the low byte of a register.
- `call_setup_per_string_char`: Calls three routines, then calls another routine for each character of a zero-terminated string until it reaches zero.
- `copy_plane_slice_mask_bits`: Copies a four-plane slice selected by a table entry into a fixed staging buffer, then computes a bit mask from a low nibble and applies it to the screen planes by clearing and setting words.
- `apply_mask_set_to_planes`: Clears the bits that a mask longword selects in four bitplane words, then sets bits from a second longword, for a pair of word positions across the planes.
- `restore_staged_plane_slice`: Copies groups of longwords from a fixed staging buffer back into three bitplane regions at fixed plane offsets, stepping forward for each group.
- `copy_table_runs_between_planes`: Walks a table of run descriptors, copying each run of words from one bitplane buffer into another and writing four plane strides at once, wrapping at the end of the buffer.
- `call_routine_return_incremented`: Calls a routine and returns its result plus one.
- `emit_decimal_digits_with_offset`: Calls two routines, derives a value from a register and a global rounded to a multiple of four plus four, splits it into decimal digits and passes them to a routine with a constant argument.
- `call_routine_four_times`: Calls a routine four times in sequence and returns.
- `call_routine_once`: Calls one routine and returns.
- `poll_and_clamp_two_coordinates`: Spins on a global, then loops: when a flag is set it calls a routine and returns a packed value.
- `fill_table_rows_into_planes`: Reads two masked values from globals into a structure, then copies a table of words into bitplane memory over 168 rows, writing all-ones words into three further planes at fixed offsets.
- `call_routine_per_nonnegative_word_2`: Walks a list of words starting at an offset from a global, calling a routine for each entry while the entry is non-negative.
- `call_routine_per_word_record`: Walks a list of three-word records starting at a global pointer, calling a routine for each record while its first word is non-negative.
- `apply_bitplane_shift_and_mask`: Iterates through a table of bit-shift offsets and bitplane data to update a range of memory buffers.
- `apply_bitplane_shift_to_buffer`: Iterates through a table of bit-shift offsets to apply bitwise or-masks and right-shifts to a destination buffer.
- `reset_table_pointer_and_call`: Calls three routines, sets a global to zero and another to the start address of the record table, then calls three more routines and returns.
- `read_long_from_frame_offset`: Returns a longword read from a fixed offset in a structure addressed through the caller's frame register.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `initialise_four_word_pairs`: Writes four pairs of fixed constant words into a table, then sets a global to a constant.
- `initialize_audio_and_interrupts`: Checks if a global variable matches a specific value, then if not, performs a delay loop and writes new length and low-address values to the four audio channels and updates the DMA and interrupt control registers.
- `initialise_channel_descriptors`: Sets a frame-structure field to fifteen, fills three sets of four descriptor longwords with fixed constants and stores three constant words into the frame structure.
- `store_entry_in_ring_slot`: Looks for an entry marked with a fixed value in a four-entry table.
- `wait_until_slots_marked`: Spins until each of four two-longword table entries holds a fixed marker value, checking them in turn, then returns.
