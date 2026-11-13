# Game 1316 (helter skelter (1990)(audiogenic)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01316). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901808 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 214 raw-word lines |
| Functions named and described | pass, 101 of 101 named (11 author symbols), 101 of 101 described (3 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 336 of 336 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 326122 bytes, 68 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3451 of 3472 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 26237 |
| code | 12052 |
| data | 346673 |
| padding | 1774 |
| unused | 515072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:Type | 2 | 363 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs.img (raw code file, load address unknown) | 1 | 2747 | 68 |

## Cases handled

- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- helter skelter (1990)(audiogenic)[cr skr]/Helter Skelter (1990)(Audiogenic)[cr SKR].adf:hs.img: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 326122 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

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
