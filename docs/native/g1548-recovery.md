# Game 1548 (helter skelter (1990)(audiogenic)[cr skr][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01548). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901808 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 214 raw-word lines |
| Functions named and described | pass, 101 of 101 named (11 author symbols), 101 of 101 described (3 by rule, 85 by the local model) |
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
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:Type | 2 | 363 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 143 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs.img (raw code file, load address unknown) | 1 | 2747 | 68 |

## Cases handled

- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- helter skelter (1990)(audiogenic)[cr skr][a2]/Helter Skelter (1990)(Audiogenic)[cr SKR][a2].adf:hs.img: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 326122 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

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
