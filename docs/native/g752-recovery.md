# Game 752 (Explorer): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00752). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 293 of 293 named (0 author symbols), 293 of 293 described (14 by rule, 275 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 212 of 212 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4550 of 4564 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 115208 |
| code | 16112 |
| data | 442218 |
| padding | 2974 |
| unused | 324608 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| type | 2 | 156 | 3 |
| BorderOff | 1 | 138 | 11 |
| Dir | 2 | 117 | 2 |
| Explorer.adf:Explorer | 5 | 3791 | 266 |
| Explorer.adf:CrEmKi | 3 | 235 | 8 |
| Explorer.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- Explorer/Explorer.adf:CrEmKi: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_and_jump`: Copies the byte in register d1 into register d0 and then jumps forward by a small offset into another routine, joining its flow without returning.

### type

- `read_command_line_file`: Opens dos.library and intuition.library, then reads a command line, trims trailing spaces and a trailing slash, and opens the resulting file.
- `process_struct_with_library_calls`: Saves a pointer on the stack, clears a longword at offset 32 of the structure in a0, then repeatedly reads a value at offset 62 and calls a library routine until that value is zero or negative.
- `execute_dos_write_operation`: Loads a library base pointer into register a6, places an argument into register d1, clears register d0, and calls the DOS write function via the library dispatch vector before returning.

### BorderOff

- `open_libraries_in_sequence`: Opens three libraries in sequence, storing each returned base pointer into a global string area, and returns zero if any open fails.
- `call_library_routines_in_sequence`: Saves a library base pointer in a6 and calls a library routine, then moves the returned value into d1.
- `perform_dos_write_operation`: Loads a library base pointer into a6, places an argument into d1, clears d0, and calls the DOS write function via a library call.
- `compare_string_against_two_globals_2`: Loads a global pointer and stores it into a global string area, then copies an adjacent pointer into the following location.
- `compare_string_against_two_globals`: Compares the string pointed to by register a1 against a global string of length 8, returning zero when they match, and otherwise compares against a second global string of length 10, returning negative one if neither matches.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_d0_noop`: This routine is empty, containing only a single instruction that modifies register d0 in place, and is referenced by other code by address.
- `copy_byte_via_pointers`: Reads a byte from the memory location pointed to by a0 and stores it into the byte at offset 1 of the memory location pointed to by a1.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `open_library_and_store_result`: Pushes register a0 onto the stack, clears register d0, loads the exec library base into register a6, calls the OpenLibrary entry point, restores a0, stores the returned pointer through the saved address, and returns.

### Dir

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes a small header and fills a buffer with sequential words.
- `transfer_d1_to_d0`: Copies the 32-bit value from register d1 into register d0 and then branches to the routine that finds the maximum value and prepares the buffer.

### Explorer.adf:Explorer

- `jump_to_main_routine`: Contains a single unconditional branch that jumps to the routine at offset 15686, so it only transfers control and then returns.
- `scan_buffer_find_max`: Copies a byte from a table at offset 250 of the buffer in a0 into d1 while decrementing a counter, and when the byte exceeds the running maximum it stores the maximum, a derived pointer, and an array of pointers into global variables, then sets bits in a status byte and returns.
- `disable_dma_channels`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `add_byte_to_counter`: Reads a byte at offset 107 of a structure in a6 and adds the byte at offset 106 to it, then returns without storing the result when the sum is negative.
- `clamp_and_increment_counter`: Clamps a byte value to a maximum of 64, stores it and a zero byte into a structure, increments a counter and compares it against a previous value.
- `branch_through_helper_routine`: Calls a helper routine via a back-branch, loads a table address into a register, and then branches forward into another routine, so it never returns on its own.
- `set_diwstop_and_dispatch_nibble`: Sets the DIWSTOP hardware register to a value derived from a global variable, then reads a byte at offset 46 of the structure in a6 and dispatches to a handler based on its low nibble, decrementing the byte each time.
- `setup_and_dispatch_actions`: Clears a global word, loads a table address and sets up several register pairs, then calls a helper routine four times and sets bit 15 of the global word.
- `dispatch_structure_action_from_nibble`: Computes a byte-indexed offset from a global value and a table entry, loads a 32-bit value from a routine table and a 16-bit value from another table into the structure in a6, clamps a byte against a global value into the structure in a5, then dispatches based on the low nibble of a structure byte, calling another routine for the values 3, 5, and 9.
- `dispatch_nibble_action_2_2`: Reads the low nibble of the byte at offset 2 of the structure in a6, subtracts 9, and dispatches to a handler based on the result.
- `process_structure_dma_control`: Masks the low 12 bits of the 16-bit value at offset 0 of the structure in a6 and stores it at offset 16, then compares the value at offset 2 with 3792.
- `update_timer_and_counters`: Sets bit 15 of a global word, calls a helper routine twice, copies eight values from a table into the timer registers, then decrements a counter at offset 10 and a byte at offset 8 of the structure in a4, and if the counter stays positive it stores a shifted value and advances a byte at offset 2.
- `dispatch_nibble_action_clamp`: Reads a byte at offset 46 of the structure in a6 and dispatches to a handler based on its low nibble, decrementing the byte each time.
- `copy_word_between_structures`: Copies a 16-bit value from offset 16 of the structure in register a6 into offset 6 of the structure in register a5 and then returns.
- `divide_global_byte_by_three`: Reads a byte from a global variable, divides it by three, and returns the quotient or zero depending on the result.
- `copy_word_between_structures_2`: Copies the 16-bit word located at offset 16 of the structure pointed to by a6 into the 16-bit word at offset 6 of the structure pointed to by a5, then returns.
- `lookup_indexed_table`: Reads a byte at offset 3 of the structure in a6, masks its low nibble, doubles it, and uses it to index a table in a0, then compares a word at offset 16 of the structure against the table entries in a loop of eight iterations and returns when a match is found.
- `copy_indexed_word_into_structure`: Copies a 16-bit value from a table in register a0 indexed by register d0, using an offset of negative 2, into offset 6 of the structure in register a5 and then returns.
- `decrement_and_clamp_value`: If a flag byte at offset 1 of the structure in a4 is clear it masks the low nibble of a byte at offset 3, subtracts it from a word at offset 16, and if the result is below 113 it sets the low bits to 113 and stores the value at offset 6 of the structure in a5, otherwise it returns.
- `increment_and_clamp_value`: If a flag byte at offset 1 of the structure in a4 is clear it masks the low nibble of a byte at offset 3, adds it to a word at offset 16, and if the result stays below 856 it sets the low bits to 856 and stores the value at offset 6 of the structure in a5, otherwise it returns.
- `lookup_and_store_clamped_value`: Searches a table of 16-bit values for one matching the low twelve bits of the word at offset 0 of the structure in a6, then stores the found table entry at offset 24 and clears offset 22.
- `clear_structure_counter`: Clears the 16-bit value at offset 24 of the structure in a6 to zero and then returns.
- `copy_and_clear_structure_byte`: Copies the byte at offset 3 of the structure in a6 to offset 23, then clears the source byte at offset 3.
- `adjust_and_lookup_counter`: If a counter at offset 24 of the structure in a6 is zero it returns, otherwise it adds or subtracts a byte at offset 23 from a counter at offset 16 depending on a flag at offset 22, swaps the two counters when needed, and then searches a table for the first entry greater than the counter.
- `update_and_lookup_table`: Subtracts an input value from a counter at offset 16 of the structure, then transfers a second counter from offset 24 when the result does not go negative.
- `merge_nibbles_into_byte`: Reads a byte at offset 3 of a structure and returns immediately when it is zero, otherwise merging the low nibble into the byte at offset 26 when nonzero and merging the high nibble into that same byte when nonzero before storing the result.
- `select_scaled_value`: Reads the byte at offset 27 of the structure in a6 and, if it is positive, returns without changes.
- `negate_and_branch_conditionally`: Tests a byte at offset 27 of the structure in a6, and if it is positive branches away, otherwise stores negative one into a data register, subtracts a second data register from it, and branches into another routine.
- `copy_register_then_branch`: Copies the byte in d0 into d2, then jumps into the middle of another routine.
- `scale_and_add_phase`: Reads a byte from a table indexed by d0 in a4, multiplies it by the low nibble of a byte at offset 26 of the structure in a6, shifts the product right, then adds it to a word at offset 16 unless a flag at offset 27 is negative, and stores the result.
- `subtract_and_advance_phase`: Subtracts one input value from another and stores the difference at offset 6 of a5.
- `call_then_jump_forward`: Calls a routine, then jumps into the middle of another routine.
- `branch_into_two_routines`: Branches into another routine and then branches into a second routine, performing no instructions of its own.
- `merge_and_store_nibbles`: Combines the low nibbles of the byte at offset 3 with the byte at offset 28 of the structure in a6, then stores the merged result back at offset 28.
- `invert_byte_to_d2`: Reads a byte from the structure in a6 at offset 28, inverts it and stores the result into d2, then returns.
- `copy_byte_to_d2`: Copies the byte in d0 into d2 and then branches to another routine.
- `scale_and_add_value`: Reads a byte from a table indexed by d0 in a4, multiplies it by the low nibble of a byte at offset 28 of the structure in a6, shifts the product right, then adds it to a byte at offset 19 unless a flag at offset 29 is negative, and stores the result.
- `clamp_and_update_value`: Calculates the difference between two values, clamps it between zero and sixty-four, and stores the result at offset 8 of a5.
- `set_flag_and_adjust`: Reads a byte at offset 3 of the structure in a6, and if nonzero stores it at offset 32, shifts it left, subtracts it from a word at offset 8, and adds the doubled value to a long at offset 4 of the structure in a6, otherwise it returns.
- `set_structure_flag_one`: Stores the value one into the 16-bit field at offset 8 of the structure in a6, then returns.
- `clamp_byte_value`: Reads the high nibble of a byte at offset 3 of the structure in a6, and if nonzero adds it to a byte at offset 19, caps the result at 64, clamps it against a value at offset 2 of a global word, and stores the final value at offset 9 of the structure in a5, otherwise it subtracts the low nibble and clamps at zero.
- `clamp_and_cap_byte`: Subtracts the low nibble of a byte at offset 3 of the structure from a value at offset 19, clamps the result at zero, and stores it back.
- `decrement_and_store_byte`: Reads a byte at offset 3 of the structure in a6, decrements it by one, and stores the result at offset 1072 of a global variable, then clears the byte at offset 1073 and sets the byte at offset 1074 before returning.
- `cap_byte_at_sixty_four`: Caps the byte at offset 3 of the structure at sixty-four and stores it at offset 19.
- `sum_nibbles_and_set_flag`: Adds the high and low nibbles of the byte at offset 3 of the structure in a6, and if the sum is at least 63 sets a global flag, otherwise stores the sum and sets the flag.
- `copy_byte_conditional_clear`: Copies the byte at offset 3 of the structure in a6 into d0, and if that byte is zero it clears a global byte at offset 2625 and stores d0 into the global byte at offset 2624, then returns.
- `dispatch_nibble_action_2`: Reads the low nibble of the byte at offset 2 of the structure in a6, subtracts 9, and dispatches to a handler based on the result (0 through 5), otherwise calls a routine that copies a word from offset 16 of the structure in a6 to offset 6 of the structure in a5.
- `dispatch_nibble_action`: Reads the high nibble of the byte at offset 3 of the structure in a6 and dispatches to a handler based on its value, decrementing that byte each time.
- `merge_bit_to_port_a`: Extracts bit 0 of a byte at offset 3 of the structure, doubles it, and merges it into bit 0 of the CIA port A register while clearing bit 1.
- `merge_low_nibble_byte`: Clears the low nibble of a byte at offset 31 of the structure, then merges in the low nibble of a byte at offset 3.
- `clear_and_merge_low_nibble`: Clears the low nibble of a byte at offset 30 of the structure, then merges in the low nibble of a byte at offset 3.
- `store_low_nibble_to_offset`: Reads a byte at offset 3 from the pointer in a6, keeps only its low nibble, and stores that nibble into the byte at offset 18 of the same structure, then returns.
- `decrement_byte_and_store`: Checks a global flag and returns if set, otherwise decrements the byte at offset 22 of the structure in a6 and, when it reaches zero, stores a byte and sets a flag.
- `store_byte_and_branch`: Writes the byte in d0 to offset 22 of the structure in a6, then jumps into the middle of another routine.
- `shift_global_and_store_byte`: Reads a 16-bit word from a global variable, shifts it right by two bits, and stores the result as a byte at offset 33 of the structure in a6, then returns.
- `merge_nibble_to_offset`: Reads a byte at offset 3 from the pointer in a6, shifts it left by four bits, clears the low nibble of the byte at offset 30 of the same structure, and ORs the shifted value into that byte, then returns.
- `conditional_dma_write_and_call`: Pushes d1 onto the stack, then, only when the low nibble of the byte at offset 3 is nonzero, the referenced global is nonzero or the word at offset 0 is zero, and that global is divisible by the nibble, it writes a word to DMACON, sets up two 32-bit values in a5, calls another routine twice with the high bit of the offset 20 word toggled on, and finally restores d1 from the stack before returning.
- `mask_nibble_then_branch`: Checks a global flag and returns if set, otherwise isolates the low nibble of the byte at offset 3 of the structure in a6 and jumps into another routine.
- `test_status_and_mask_nibble`: Tests a status byte and returns if it is nonzero, otherwise reads a byte at offset 3 of the structure in a6, keeps its low nibble, and jumps into the middle of another routine.
- `compare_nibble_and_clear`: Reads a byte at offset 3 of the structure in a6, keeps its low nibble, compares it against a global byte, and returns if they differ, otherwise clears a byte at offset 13 and a word at offset 8 of the structure in a5.
- `check_nibble_and_push`: Reads a byte at offset 3 of the structure in a6, keeps its low nibble, and returns if it is zero or a global byte is zero, otherwise pushes d1 onto the stack and jumps into the middle of another routine.
- `increment_and_store_byte`: Tests a status byte and returns if it is nonzero, otherwise reads a byte at offset 3 of the structure in a6, keeps its low nibble, returns if another status byte is nonzero, increments the value, and stores it at offset 1077 of a global variable.
- `merge_low_nibbles`: Tests a status byte and returns if it is nonzero, otherwise reads a byte at offset 3 of the structure in a6, keeps its low nibble, shifts it left by four bits, and combines it with the low nibble of a byte at offset 31 of the structure in a6.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine`: The routine contains an infinite loop with no operations.
- `detect_and_copy_buffer`: Detects a magic marker at the start of the buffer in a0 and, if present, copies a block of bytes from one location to another while shifting pointers by the sizes stored in the header, then returns.
- `empty_data_word`: It is empty, containing only a single instruction that does nothing and returns immediately.
- `empty_data_area`: It contains no instructions, performs no operations, and returns immediately.
- `tiny_addi`: Small routine of 1 instruction that adds 0 to d0.
- `empty_routine`: It is an empty routine that does nothing.
- `store_zero_byte`: It is a single instruction that writes a zero byte to the memory location pointed to by d0, then returns.
- `store_zero_byte_2`: It is a single instruction that writes a zero byte to the memory location pointed to by the register pair d0/a0, then returns.
- `decode_rle_block`: Verifies the four-byte magic at the start of the buffer in a0, then copies a block described by that header from the source in a1 into the destination in a0, and if the trailing data has a non-negative length it decodes the RLE stream and stores the resulting bytes back into the destination.
- `process_byte_table`: Reads a 16-bit value from a global variable, subtracts 273 from it, and if nonzero processes a table of 128 bytes by copying and masking four-byte groups into another global variable, then returns the running index.
- `copy_table_words`: Reads a 16-bit value from a global variable, masks off bit 6, indexes a table of 32 words, and copies each word into two other global variables spaced four bytes apart, then returns.
- `store_table_pointers`: Reads a 16-bit value from a global variable, masks off bit 6, and stores two 32-bit pointers from a table into two other global variables, then returns.
- `tiny_move`: Small routine of 1 instruction that copies (a1) to -(a0).
- `rotate_global_value`: Reads a 16-bit value from a global variable, rotates it right by three bits after adding 37448, stores the result back, and returns.
- `clean_nibble_groups_loop`: Walks sixteen four-word slots in a structure, subtracting one from each nibble group of the word at offset 2 and storing the cleaned value back, then calls a helper routine, repeating the whole pass sixteen times.
- `store_min_nibble_groups`: Compares each nibble group of two structures at offset 2, keeping the smaller of the incremented value and the stored value for each group, then writes the result back, repeating for sixteen slots and sixteen outer passes.
- `process_table_counters`: Subtracts 273 from a 16-bit counter in a global variable, then for each of 16 consecutive 32-bit words starting at offset 4 in a table it reads the word at offset 2, computes the bitwise AND of that value with 15 and of the counter with 15, adds them, clamps the sum to 15 if it exceeds 15, ORs the result into the value at offset 2, and advances by 4 bytes until the counter reaches 128.
- `store_pointer_and_return_word`: Stores a global pointer at offset 128 of the structure in a6, then returns the word at offset 136.
- `copy_table_words_pair`: Copies 32 consecutive 16-bit words from a source table, whose starting address is offset by a value taken from a global variable with bit 6 set, into two separate destination tables, then returns.
- `call_helper_clear_bit_branch`: Loads a constant into a register, calls a helper routine, then loads a global value, clears a bit in it, and branches away without returning.
- `toggle_bit_and_branch`: Reads a 16-bit global value, toggles bit 2 of it, and jumps into the middle of another routine.
- `call_store_and_fill`: Calls a routine that stores a pair of table values, then calls another routine that fills a buffer, and returns.
- `store_table_pair`: Takes the low byte of a 16-bit value in d0 and uses it as a 2-byte offset into a table of 32-bit values, then stores the 32-bit word at that offset into a global variable and the 32-bit word at the offset XORed with 4 into the following global variable, then returns.
- `copy_word_pair_buffer`: Reads a 32-bit value from a global variable, stores its two halves into a buffer, advances the pointer by eight bytes, repeats this four times while incrementing the source by 40 each time, then returns.
- `swap_global_pointer`: Stores the address in a0 at offset 8 of a global structure, then points the global structure at that address and returns.
- `find_matching_list_node`: Walks a linked list of structures pointed to by a global variable, comparing the two input words and a pointer field at offset 12 of each node, and following the next-pointer at offset 8 until a match is found or the list ends.
- `locate_list_pointer`: Walks a linked list of structures pointed to by a global variable, following the next-pointer at offset 8 until it reaches the pointer in the input register or the end of the list, then returns.
- `advance_to_list_end`: Walks a linked list of structures pointed to by a global variable, following the next-pointer at offset 8 until it reaches the end of the list, then returns.
- `unlink_list_node`: Removes a node from a linked list by unlinking it: if it is the head it updates the global pointer, otherwise it re-links the previous node to the next node, then returns.
- `process_list_nodes`: Loops over a chain of structures starting from the input register, calling a helper routine for each node and advancing to the next node via the pointer at offset 8 until the pointer is zero, then returns.
- `clear_structure_fields`: Reads a pointer from a register, and if nonzero clears the two 32-bit fields at offsets 32 and 36 of the structure it points to, otherwise returns.
- `handle_flagged_structure`: Reads a flag from a structure and, depending on whether it is negative or positive, clears its high bit and adjusts a counter, then initializes a block of fields in the structure in a6 and clears the flag, before returning.
- `process_flagged_structures`: Loads the base address from a global variable and, if it is set, calls a helper routine in a loop while the value at offset 8 of the structure is nonzero.
- `scale_and_add_values`: Reads two 16-bit values from a structure, scales the second by 200, adds the shifted first value, and returns the combined 32-bit result.
- `compute_and_store_viewport`: Reads a structure pointed to by a0 and, depending on whether bits 12 to 15 of its first word are zero, stores scaled position and size values into a second structure pointed to by a6, then returns.
- `compute_scaled_offsets`: Computes scaled offsets from a source table in a0 and a value in d0, then calls a helper routine four times while storing the derived values into fields of the structure in a6.
- `match_masked_flag_value`: Reads a 32-bit value from offset 4 of the structure in a6, shifts it right by eight bits, masks the low nine bits, and compares the result against a value pushed on the stack, looping until they match before calling the flag routine and returning the pushed value.
- `compare_shifted_flag_value`: Reads a 32-bit value from offset 4 of the structure in a6, shifts it right by eight bits, masks the low nine bits, and compares the result against a value pushed on the stack, looping until it is less than that value before calling the flag routine and returning the pushed value.
- `wait_flag_bit_clear`: Writes the word 33792 to the field at offset 150 in the structure in a6, then spins in a loop reading the byte at offset 2 until bit 6 of that byte is clear, and finally stores the word 1024 at offset 150 before returning.
- `call_three_routines`: Calls three helper routines in sequence and falls through into the next routine without a return.
- `load_and_branch_global`: Loads a 32-bit value from a global variable into d0, then branches to another routine to continue its execution.
- `load_other_global_branch`: Loads a different 32-bit value from a global variable into d0, then branches to another routine to continue its execution.
- `run_blitter_loop`: Initializes a blitter channel in the custom registers, sets a counter in d1 to four, and loops calling a helper routine four times before returning.
- `setup_blitter_channels`: Loads a pointer from a global variable and calls two helper routines in sequence.
- `run_blitter_setup`: Sets up a blitter channel in the custom registers using a value from a global variable, then loops calling a helper routine four times before returning.
- `init_blitter_param_loop`: Initializes the custom registers by loading their base address, then runs nested loops over a table, computing scaled offsets and calling a helper routine to store blitter parameters for each entry whose mask bit is set.
- `store_scaled_offsets_when_flag`: Reads a word from a source table and a flag word from another table, and when the flag bit is set stores a scaled offset and two indices into a structure, then calls two helper routines while advancing through sixteen by sixteen blocks.
- `scan_pairs_store_slots`: Scans pairs of words in a table until it finds a zero entry, saving each non-zero word to a global slot, then inspects a global value and branches away if it matches a pattern.
- `store_blitter_parameters`: Calls a helper routine once, then stores a series of constants and two incoming values into fields of the structure in a6 before returning.
- `call_add_branch_global`: Calls a helper routine, adds a global value to d6, loads a constant into d4, then branches to another routine to continue its execution.
- `compute_scaled_blitter_offset`: Converts a byte into a scaled offset using multiplication, combines it with a global value, then branches to a helper routine to store the computed blitter parameters.
- `compute_split_blitter_offset`: Converts a byte into a scaled offset using multiplication, adds global values to the registers, calls a helper routine to split and store the value, then adds the offset to another global value.
- `store_split_value`: Calls a helper routine once, then stores a value split across two halves into adjacent fields of the structure in a6 along with constants before returning.
- `decode_and_adjust_pointer`: Copies a block from a source buffer into a destination buffer via a helper routine, then loads a pointer from a table, adds an offset to it, pushes it on the stack, and stores a constant into d0.
- `hold_zero_word`: Holds a single zero word that other code references by address, performing no operation on it.
- `store_zero_byte_3`: It writes a zero byte to the memory location pointed to by a0.
- `sync_button_state`: Clears three adjacent global words, then reads a byte from a structure field and sets or clears those words based on individual bits of that byte before returning.
- `invoke_pointer_if_valid`: Loads a pointer from a global variable and returns immediately if it is null, otherwise copies it to a0, calls a routine, and returns if the pointed-to value is zero.
- `call_indirect_if_valid`: Loads a pointer from the field at offset 48 in the structure in a0, and if that pointer is not null it jumps to it and returns; otherwise it returns immediately.
- `test_combined_flag_return`: Loads a value and a flag word from a structure, combines them, and returns immediately if the combined value is zero otherwise falling through to another routine.
- `branch_to_unlink`: Branches to another routine to continue its execution.
- `decrement_and_update_handler`: Decrements the word at offset 46 of the structure in a0 and, if it reaches 23, increments a global counter when the value is divisible by four.
- `increment_if_divisible_by_four`: Checks whether the low two bits of the value in d7 are clear, and if so increments the 16-bit value stored at offset 40 of the global variable table before returning.
- `increment_status_counter`: Reads a status word at offset 40, increments it, and compares it against the value at offset 44, returning early if they match; otherwise it toggles a flag bit at offset 52 and either resets the counter or decrements the value at offset 46, finally setting a global constant.
- `decrement_status_word`: Tests a word in a structure and, if it is nonzero, decrements it before returning.
- `store_status_word`: Stores the word 69 into the 32-bit value at offset 15964 of the global variable table, then returns.
- `decrement_counters_if_active`: Decrements the 16-bit counter at offset 2 of the structure in a0, and if the counter at offset 46 is nonzero it also decrements that counter, otherwise it returns.
- `increment_and_adjust_counter`: Increments the 16-bit counter at offset 2 of the structure in a0, and if the counter at offset 46 is nonzero it decrements that counter, otherwise it returns.
- `decrement_counter_if_active`: Decrements the 16-bit counter at offset 0 of the structure in a0, and if the counter at offset 46 is nonzero it also decrements that counter, otherwise it returns.
- `increment_and_adjust_counter_zero`: Increments the 16-bit counter at offset 0 of the structure in a0, and if the counter at offset 46 is nonzero it decrements that counter, otherwise it returns.
- `tick_and_wrap_counter`: Increments the 16-bit counter at offset 52 in the structure in a0 and masks it to one bit, and when that counter reaches zero it increments the value at offset 40, clamps it to the limit at offset 44, and stores the result back before returning.
- `increment_and_wrap_counter`: Increments a counter in a structure, masks it to two bits, and jumps to another routine when the masked value becomes zero.
- `index_and_store_table`: Indexes a 32-word table using a shifted value derived from a structure field at offset 46, stores the looked-up word at offset 40, and increments that field, then returns.
- `advance_and_store_pointer`: Manages a 16-bit counter at offset 34 of the structure in a0 by decrementing it when nonzero or incrementing it when zero, then adjusts a value at offset 40 and a byte at offset 34, and calls a helper routine to compute a value when the counter is initially zero.
- `initialize_blitter_table`: Stores two handler pointers into a structure, sets three control words, and calls a helper routine to compute a value.
- `call_helpers_adjust_stack`: Saves registers, calls two helper routines, and if a value read from the stack is zero it adjusts the stack and calls a helper routine before returning otherwise it performs more stack adjustments and helper calls.
- `increment_and_store_pointer`: Increments and masks the word at offset 34 of the structure in a0, branching away if the result is nonzero.
- `compute_and_update_indices`: Splits the low seven bits of the word at offset 26 in the structure in a0 into two indices, looks up two words in a global table, computes two derived values from them, stores them at offsets 0 and 2, and updates the index field at offset 26, then increments a counter at offset 64 and resets it to zero if it exceeds the limit at offset 68.
- `rotate_global_value_2`: Reads a word at offset 46 of the structure in a0, rotates it right by three bits after adding 37448, stores the result back, and returns.
- `adjust_and_split_blitter_offset`: Adjusts a value read from a table by subtracting an offset, then divides it by 40 to derive a scaled offset and remainder.
- `compare_and_branch_offset`: Swaps the nibbles of a data register and compares the result against thirty-eight.
- `check_and_branch_value`: Compares a data register against the value 5656 and, if they differ, branches into a helper routine.
- `split_and_store_blitter_offset`: Splits a byte read from a global table into a scaled offset and remainder, stores the remainder into a table at offset 34 and the scaled offset into two other tables, then branches to a helper routine when the remainder is non-zero and the scaled offset is within range.
- `process_grid_cell`: Computes the difference between a grid index and a stride value, then divides the result by 40 to locate a cell.
- `check_grid_stride_value`: Compares the value in d2 with 38 and, if it is not below that threshold, jumps to another routine; otherwise it stores 4095 into a 16-bit global variable at offset 34 and continues into that routine.
- `compare_and_check_d2`: Compares a 16-bit value in d7 with the constant 65529 and skips the next test when they differ, then checks whether d2 is zero and branches away when it is, otherwise falling through to the instruction at the start.
- `process_grid_cells`: Walks a grid of cells by adding a stride to a base index, scaling it and dividing by 40, marks each visited cell in three tables, and when a neighbour cell is free calls two helper routines to process it; if no cell is free it writes 4095 to a global variable and updates a structure.
- `store_flag_and_branch`: Stores the constant 4095 into a 16-bit global variable at offset 34, then branches into another routine that uses the value previously read from that global variable to update several fields of a structure.
- `scan_table_for_marker`: Walks entries of a table in a3 starting at an offset derived from d0, advancing by two each step and wrapping past the limit at 559, until it finds an entry whose byte at offset 35700 equals 17 or the offset matches the original, then compares the offset with the original and returns.
- `search_table_for_match`: Divides the value in d0 by 40 and tests several offsets around the result against the word 230 in a table at offset 1280, setting d0 to -1 if none of the compared values match, otherwise leaving d0 unchanged before returning.
- `initialize_list_node`: Initializes a linked-list node by copying two 16-bit values from the structure in a0 into global variables, sets up pointers to three global tables, stores a global index into those tables, and calls several helper routines to process the node.
- `process_indexed_table_entry`: Reads a 16-bit value from a global variable, stores two bytes from tables indexed by it, masks a third byte to 8 bits, and calls a helper routine.
- `combine_and_store_offsets`: Adds 8 to both input words, combines a masked high-nibble result and a shifted low-nibble result arithmetically, and stores the 16-bit outcome at offset 34 of the structure in a0, then returns.
- `empty_data_area_2`: It is a single zero word that decodes as ori.b #0, d0, so it is an empty data area that other code refers to by address.
- `read_table_store_word`: Masks the 16-bit input value in d0 with 65534 and reads the word at that offset from a table starting at 1280 into d7, then stores d7 into the word at 9044 and returns.
- `remap_index_store_byte`: Reads a word at offset d0 from a table at 640 into d7, uses that value as an index into a byte table at 36612, masks the result with 255, and stores it as a word at offset d0 in a table at 1280 before returning.
- `update_handler_pointer`: Decrements a global counter and, based on a command byte, updates a handler routine pointer in a structure, copies structure fields into a new structure, or resets a flag counter, then returns.
- `select_and_store_routine`: Compares four global flags in sequence and loads a matching table address into a register, then stores a routine address at offset 48 of the structure in a0 before returning.
- `scan_table_find_match`: Walks a table of words at 1280 in steps of 2 starting from d0, resetting to zero and stopping once it has looped back, until it finds a word equal to 64, then stores the low 5 bits of the index shifted left by 3 and the index divided by 40 shifted left by 4 into two words of a table at a fixed address.
- `clamp_and_select_values`: Reads the 16-bit word at offset 40 of the structure in a0 and, if it is less than 28, loads a code address and a flag word from a data area, then returns immediately when the flag is zero.
- `decrement_counters_conditional`: Loads a code address and a flag word from a data area, and returns immediately when the flag is zero.
- `select_constants_conditional`: Compares a register against the value 2 and, when it differs, loads the constants 0 and 49 into two registers.
- `store_conditional_table_values`: Loads two values into d5 and d6, using 0 and 48 instead of 12 and 50 when a global variable at the referenced address is zero, then stores d5 at offset 52, a byte from a table indexed by d6 at offset 40, and the constant 3 at offset 46 of the structure in a0 before returning.
- `decrement_counters_if_active_3`: Decrements the 16-bit counter at offset 46 of the structure in a0 and, if it was nonzero, also decrements the counter at offset 2, otherwise it returns.
- `increment_offset_42_counter`: Reads the 16-bit value at offset 42 of the structure in a0, adds 8 to it, clamps the result to a maximum of 200, then stores the value back and returns.
- `decrement_counters_if_active_2`: Decrements a 16-bit counter at offset 2 of the structure in a0 and, if the counter at offset 46 is nonzero, decrements that counter too; if the counter at offset 2 is greater than 15 it calls two helper routines, adjusts a value in d0, and stores a byte from a table at offset 40 of the structure, otherwise it increments two adjacent words at offsets 0 and 2, masks them, and clears the counter at offset 46.
- `increment_mask_struct_fields`: Adds 8 to each of two adjacent words at offsets 0 and 2 of the structure in a0, masks them with 496 and 240 respectively, clears the word at offset 46, and sets the word at 10148 to 65535 before returning.
- `increment_offset_40_counter`: Reads the 16-bit value at offset 40 of the structure in a0, adds 8 to it, clamps the result to a maximum of 304, then stores the value back and returns.
- `decrement_offset_40_counter`: Reads the 16-bit value at offset 40 of the structure in a0, subtracts 15 from it, and if the result is negative it returns without modifying anything.
- `decrement_offset_40_by_two`: Reads the 16-bit value at offset 40 of the structure in a0, subtracts 2 from it, and if the result is negative it returns without modifying anything.
- `tiny_moveq`: Small routine of 1 instruction that loads 255 into d1.
- `split_words_into_tables`: Writes the incoming word to a global byte at offset 29BE, then loops 288 times copying each 16-bit entry from a global table into two parallel tables, storing the low byte in one and the high byte in the other.
- `holds_zero_word`: Contains a single zero word that decodes as ori.b #0, d0, leaving the data area unchanged.
- `process_byte_table_loop`: Clears two adjacent global words and then loops over a 288-byte table, masking each entry to a byte and storing it into a structure.
- `load_zero_and_branch`: Loads the constant 0 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `load_one_and_branch`: Loads the constant 1 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `load_two_and_branch`: Loads the constant 2 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `load_three_and_branch`: Loads the constant 3 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `load_four_and_branch`: Loads the constant 4 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `load_five_and_branch`: Loads the constant 5 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `load_six_and_branch`: Loads the constant 6 into a register, then branches into another routine that selects a constant pair from a flag and stores it into a structure.
- `store_constants_flag`: Selects one of two constant pairs based on a flag, stores them at offsets 52 and 40 of the structure in a0, sets offset 46 to 3, and returns.
- `load_data_address`: It loads the address of the data at a fixed address into a1 and does nothing else.
- `advance_and_copy_bytes`: Reads a 16-bit value from a global variable and, while it is less than 48, adds 54 to a 32-bit value stored in another global variable and increments the 16-bit variable by one, then copies 55 bytes from the buffer pointed to by a1 to the buffer pointed to by that 32-bit value using a loop that counts down from 53, and finally returns.
- `copy_bytes_loop`: It copies bytes from one memory region to another in a loop controlled by a decrementing register, then returns.
- `call_and_clear_globals`: Calls a routine using a pointer read from a global variable, then clears three adjacent global variables and stores the address of a global variable into a fourth one before returning.
- `empty_data_area_3`: It is two consecutive zero words that decode as ori.b #0, d0, so it is an empty data area that other code refers to by address.
- `decrement_flag_when_set`: Copies a 16-bit flag from a global variable into d7 and returns immediately when that flag is zero, otherwise loading the constant 50, decrementing it, storing it back to the global variable, and returning when the result is not zero.
- `decrement_counter_loop`: Reads a flag word from a data area and returns immediately when it is zero.
- `init_three_structures`: Initializes three structures by storing a shared code address at offset 48, copies two data words into their offset 40 fields, and sets their offset 46 counters to two, then returns unless the incoming flag is nonzero, in which case it links the first structure into a global list and branches away.
- `decrement_struct_counter`: Tests the word at offset 46 of the structure in a0 and returns without changing it if it is zero, otherwise it decrements that word and returns.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `init_structure_and_global`: Stores the value 50 into a global word and initializes a structure by writing 2 into offset 0 and 0 into offsets 2 and 4, then calls a routine and returns.
- `clear_structure_five_words`: Clears five consecutive words of a structure starting at offset 0, then calls a routine and returns.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d1, then merges 0 into d0.
- `mark_zeroed_data`: It contains two consecutive zero-word instructions that decode as ori.b #0, d0, which is the standard encoding for a zero-filled data area rather than executable logic.
- `shift_and_compare_loop`: Loops eight times, pushing a structure word onto the stack and shifting the input register right by four bits each iteration.
- `carry_over_decimal_digit`: Adds the low nibble of d0 to a 16-bit value at a1 plus d1 and returns if the sum is less than ten.
- `init_five_structures`: Initializes five structures by storing a shared code address at offset 48, copies five data words into their offset 40 fields, and sets their offset 46 counters to two, then returns unless the incoming flag is nonzero, in which case it links the first structure into a global list and branches away.
- `set_global_and_branch`: Stores the value 5 into a global word, then branches into another routine that stores the global into a structure and appends a node to a linked list when a register is nonzero.
- `zero_fill_data_area`: Contains a single ori.b #0, d0 instruction that is part of a zero-filled data area referenced by address.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 1 into d0.
- `increment_and_clamp_values`: Adds the input register to a structure word and resets it to 0 when the sum exceeds 9, then increments a global counter and clamps it to 9 when it exceeds 9, before branching into another routine.
- `decrement_and_clamp_values`: Decrements a structure word and resets it to 9 when it underflows, then decrements a global counter and resets it to 0 when it underflows, before branching into another routine.
- `init_two_structures`: Initializes two structures by copying two data words into their offset 40 fields, storing a shared code address at offset 48, and setting their offset 46 counters to two, then returns unless the incoming flag is nonzero, in which case it links the first structure into a global list and branches away.
- `load_addresses_and_branch`: Loads two fixed addresses into registers, calls a routine, pushes a register onto the stack, then branches into another routine that walks a linked list and initializes two structures.
- `empty_routine_2`: It is an empty routine that does nothing and returns immediately.
- `set_flag_and_branch`: Stores the constant 1 into a 32-bit global variable at offset 2F7A, then branches into another routine without returning.
- `increment_and_clamp_bytes`: Increments a byte at offset 2 of a two-word structure and resets it to zero when it exceeds 9, then increments a second byte at offset 0 and clamps both bytes to 9 when they exceed 9, and finally branches into another routine.
- `decrement_global_counter`: Loads a 16-bit value from a global variable, decrements it by one, stores the result back, and returns.
- `run_sequence_when_flag_set`: Reads a 16-bit value from a global table and, when it equals 64, calls three helper routines in sequence, looping the second call until it returns 64 and the third call until it returns nonzero, then calling two more helper routines before returning.
- `split_and_update_indices`: Splits the low seven bits of a word at offset 26 of the structure in a0 into two indices, looks up two words in a global table, computes two results by multiplying and shifting, stores them at offsets 0 and 2, updates the split field and a second field, then returns.
- `zero_fill_data_area_2`: Contains two ori.b #0, d0 instructions that are part of a zero-filled data area referenced by address.
- `process_table_flags`: Loops through eight 16-bit entries of a table, and for each non-zero entry whose high bit is set it stores a power-of-two flag in a status word at offset 150 of the structure in a6 and marks the entry, while for each non-zero entry whose high bit is clear it stores a pointer to a global variable at offset 160 of the structure in a6, sets the word at offset 164, and clears the entry; otherwise it returns.
- `update_scaled_offsets`: Adds 5120 to the two 32-bit values at offsets 12 and 16 of the structure in a0, then restores them from the stack.
- `initialize_structure_fields`: Initializes a structure pointed to by a0 by storing zero words at offsets 40, 40, 52 and 46, then stores a table address at offset 12 and a zero byte at a global address.
- `compare_flag_and_clear_bit`: Compares a word at offset 3e5c of a global variable with the decimal value 81 and returns without modifying anything if they differ, otherwise clears bit 0 of a word at offset 32da of a global variable.
- `check_flag_then_call`: Tests a word at offset 32da of a global variable and returns immediately if it is nonzero, otherwise calls a routine, jumps to another routine, and returns.
- `store_byte_via_pointer`: Writes the low byte of the 32-bit value in d0 to the memory location pointed to by a1, then returns.
- `call_routine_via_bsr`: Calls a routine at offset 2F7E via a subroutine call and never returns on its own.
- `init_and_run_game_handlers`: Initializes a game structure by clearing its fields, loading a handler routine pointer and a table address, then repeatedly reads a command byte and dispatches to the corresponding handler until the command is 32.
- `copy_table_bytes`: Reads a 16-bit global variable and returns if it is zero.
- `hold_zero_word_2`: It is a single zero word that other code references by address, performing no operation on it.
- `search_byte_table`: Compares d6 against bytes at a1 plus d7 starting at offset 15 and counting down.
- `compute_transformed_viewport_values`: Swaps a global code pointer, then computes five transformed values from two stack words and stores each into offset 40 of a structure at successive offsets, incrementing that structure's base word between each store, then restores the global pointer and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_helpers_for_pushed_values`: Pushes the 16-bit value in d0 onto the stack, calls a helper routine, then loops calling that helper once more for each of the values it pushed before returning.
- `push_and_transfer_words`: Pushes the 32-bit value at a6 onto the stack, loads a 16-bit value from the stack into d3, stores d0 onto a2, and ends with an ori.b #0, d0 instruction that is part of a data area.
- `shift_and_verify_bytes`: Reads a 16-bit value from a global table and returns immediately when it is zero, otherwise shifting four bytes within a table, storing the value at the top, comparing the shifted bytes against a second table and returning early if any differ, then calling two helper routines and storing the constant 4095 into a global variable.
- `load_words_from_globals`: Loads a word from the global variable at offset 47 into d0 and a word from the global variable at offset 48 into d1, then returns.
- `compute_scaled_offset`: Multiplies d0 by ten, adds d1, subtracts one, sign-extends the result into d0, and returns.
- `decode_rle_from_buffer`: Sets a0 to point at the buffer at address 9004 and points a1 at a source table advanced by 2000 bytes, then calls the RLE decoder routine.
- `run_blitter_setup_pair`: Loads a global word into a data register and calls the blitter setup routine, then loads a second global word into the same register and calls the blitter setup routine again, then returns.
- `setup_structure_and_calculate_pointer`: Calls a series of helper routines to set up a structure, then masks a value with 7 and stores it at offset 46, clears the words at offsets 2 and 52, zeroes two global variables, stores 15 at offset 2, and finally computes a pointer from a negative offset and returns the value 65366.
- `init_and_advance_table_index`: When a0 holds a specific global address it calls a helper twice and stores a pointer and counter fields into the structure, then computes scaled offsets from a table using a global index and writes them into the structure.
- `add_to_struct_counter`: Reads a 16-bit counter at offset 52 of the structure in a0 and adds a value stored at offset 2 of the same structure to it, then stores the result back at offset 52.
- `decrement_and_check_counter`: When the incoming flag equals two, adds a structure word at offset 2 to its counter at offset 52, adjusts that counter toward a limit, and if it reaches the limit and the pointer matches a global entry it decrements an offset 46 counter and resets the offset 52 counter to thirty-four, otherwise it decrements the counter and returns.
- `decrement_table_counter`: Reads a counter word at offset 52 of the structure in a0, decrements it, and stores the result back.
- `accumulate_bytes_in_loop`: Decrements a register, accumulates a byte from a pointer into a data register in a loop that repeats until the register reaches zero, then compares the accumulated value against a second register and returns if they differ.
- `call_via_stack_push`: Pushes the address of a routine onto the stack and returns to it, effectively calling that routine.
- `set_flag_byte_to_255`: Writes the byte value 255 into the global variable at offset 4223 and then returns.
- `wait_for_flag_to_clear`: Tests the global variable at offset 4223 and returns immediately when its value is zero, otherwise looping back to re-test it.
- `initialize_pair_of_flags`: Stores the byte value 64 into the global variable at offset 4222 and stores 0 into the global variable at offset 4223, then returns.
- `clear_custom_registers`: Stores the constant 65535 into a 16-bit custom register field, calls three helper routines in sequence, then clears sixteen 16-bit fields at offsets 0 through 56 of a custom register block by writing zero to each pair of adjacent registers.
- `call_process_and_helpers`: Calls a helper routine that processes flagged structures, then calls another helper routine, and returns.
- `call_three_helpers`: Invokes three helper routines in sequence and then returns.
- `compare_and_copy_tables`: Compares two parallel tables of five words and, when any word in the first table is smaller, copies the second table over the first and returns, otherwise copying the second table over the first, initializing five structures with shared data, and finally branching into another routine.
- `dispatch_and_copy_table`: Compares a global word against the constants 5 and 4 and branches away when either matches, otherwise loads a table base, copies one pointer into a structure, and loops four times storing a second pointer into consecutive slots before returning.
- `check_value_and_toggle_bit`: Calls a routine, then if the value at offset 15964 equals 80 it toggles bit 7 of the byte at offset 75454 and calls another routine.
- `zero_filled_data_area`: Contains a single zero word that decodes as ori.b #0, d0, indicating a zero-filled data area referenced by address rather than executed code.
- `init_and_run_game_handlers_2`: Initializes a game structure by clearing its fields, loading a handler routine pointer and a table address, then repeatedly reads a command and dispatches it until a counter reaches zero.
- `run_frame_setup_loop`: Initializes a global counter and a byte flag, then repeatedly calls setup and helper routines while a second counter counts down.
- `set_palette_index_loop`: Sets bit 7 of the color palette register at custom offset 0 to index 200, then loops 200 times calling the palette handler routine before returning.
- `write_zero_byte`: It writes a zero byte to the memory location pointed to by the register in a0, then returns.
- `resolve_library_and_call_handlers`: Resolves a library base from a global variable, calls a library function through its vector table, stores the returned pointer, and if it is non-zero saves a value from that structure and calls three other routines before returning.
- `verify_and_decode_rle_block`: Verifies a four-byte magic value at the start of the buffer in a0, then copies a block described by that header from the source in a1 into the destination using decode_rle_block.
- `setup_custom_and_store_pointer`: Calls a routine to find the maximum byte in a table, calls another routine to initialize a pair of flags, stores the address of a third routine into a global variable at offset 108, sets the custom register base pointer in a6, and returns.
- `store_pointer_and_call_routine`: Loads a pointer from the global variable at offset 15840 into a0, stores that pointer into the global variable at offset 108, calls the routine at offset 136, sets a6 to the custom register base, and then returns.
- `save_registers_and_call`: Pushes registers d0-d7 and a0-a6 onto the stack, calls another routine, then restores those registers and jumps to an invalid address.
- `call_library_functions`: Sets a6 to an exec library base via a global variable, calls a library function through its vector table, then switches a6 to a graphics library base and jumps into another library function.
- `read_keyboard_bit_and_store`: Reads the keyboard serial register and, if bit 0 is set, clears that register, inverts the value, shifts it right by one, and stores the result into the global variable at offset 15964, otherwise it stores 0 there.
- `tiny_ori_2`: Small routine of 1 instruction that merges 16 into d0.

### Explorer.adf:CrEmKi

- `parse_and_decompress_chunk`: Parses a chunked data structure identified by the magic number an address, decompresses its RLE-style payload by copying run-length byte sequences into a destination buffer, and then calls the figure-building routines for three regions.
- `extract_flag_bit_from_table`: Loops over a table of word pairs, comparing the masked word with the following word while shifting a mask register, and stops when they match.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks a word in a register with a mask register and stores the result.
- `process_and_adjust_registers`: Performs a series of arithmetic and rotate operations on registers d0, d1, d6 and d7, reading and decrementing two bytes from the address in a3, and then returns.
- `fill_figure_table`: Reads a count, fills a stack buffer with values from a helper routine, then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `process_linked_list_entries`: Checks whether the value at the address in a4 equals 10000, and if so walks a linked list clearing entries, iterates a table via a self-referential pointer, and accumulates computed addresses into a structure, otherwise it returns without modifying anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Explorer.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
