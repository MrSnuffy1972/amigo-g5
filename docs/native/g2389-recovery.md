# Game 2389 (Zeewolf 2 - Wild Justice): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02389). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935216 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 34 raw-word lines |
| Functions named and described | pass, 237 of 237 named (0 author symbols), 237 of 237 described (13 by rule, 224 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1064 of 1064 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 67072 bytes, 11 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4012 of 4316 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28966 |
| code | 20322 |
| data | 885398 |
| padding | 18 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zeewolf 2 - Wild Justice.adf:bootblock | 1 | 55 | 5 |
| logged disk load from Zeewolf 2 - Wild Justice.adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2418 | 172 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 141 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 970 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 57 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Zeewolf 2 - Wild Justice.adf bytes 425472-438272 (code never loaded by a reference run) | 1 | 0 | 0 |
| Zeewolf 2 - Wild Justice.adf bytes 438784-440320 (code never loaded by a reference run) | 1 | 200 | 7 |
| Zeewolf 2 - Wild Justice.adf bytes 443392-443904 (code never loaded by a reference run) | 1 | 0 | 0 |
| Zeewolf 2 - Wild Justice.adf bytes 444928-495104 (code never loaded by a reference run) | 1 | 85 | 1 |
| Zeewolf 2 - Wild Justice.adf bytes 503808-504832 (code never loaded by a reference run) | 1 | 160 | 2 |
| Zeewolf 2 - Wild Justice.adf bytes 882176-883200 (code never loaded by a reference run) | 1 | 99 | 1 |

## Cases handled

- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: the boot block reads 6656 bytes from disk offset 890368 to an address (found statically); recovered as a unit
- runtime segment an address: 553 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 107 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 350 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 35 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: bytes 425472-438272 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: bytes 438784-440320 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: bytes 443392-443904 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: bytes 444928-495104 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: bytes 503808-504832 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: bytes 882176-883200 look like code no run loaded: recovered position-independently (load address unknown)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: 7 logged loads cover 73728 bytes of the image
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: 32 track-loaded blocks found in recovered runtime segments and classified through them
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 2-150 (bytes 1024-77312) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.55)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 152-829 (bytes 77824-424960) outside the filesystem hold data, not code (opcode density 0.032, entropy 5.80)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 862-862 (bytes 441344-441856) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.58)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 970-972 (bytes 496640-498176) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.79)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 974-981 (bytes 498688-502784) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.28)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 994-1720 (bytes 508928-881152) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.59)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 1732-1738 (bytes 886784-890368) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.10)
- Zeewolf 2 - Wild Justice/Zeewolf 2 - Wild Justice.adf: blocks 1752-1759 (bytes 897024-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)
- 3 never-loaded region(s), 63488 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Zeewolf 2 - Wild Justice.adf:bootblock

- `init_library_and_jump`: Retrieves the exec library base and calls AllocMem to allocate memory, storing the result in a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `do_nothing`: It is a single ori.b #0, d0 instruction that does nothing and returns immediately.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `store_constants_and_call`: It stores three 32-bit constants at offsets 36, 40 and 44 of the structure in a1, then calls the routine at offset -200 of a6 and the routine at the address in the constant an address, and returns.

### logged disk load from Zeewolf 2 - Wild Justice.adf boot-block load

- `store_high_byte_and_rotate`: Reads a long from the stack into d0, extracts its high byte into d7, stores that byte as a word at the address in a5, rotates d0 right through the carry flag, and returns the shifted value.
- `shift_dispatch_and_copy`: Repeatedly shifts a value right while decrementing a counter, calling a routine when the shifted value reaches zero, then dispatches based on the resulting counter by calling another routine with different arguments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_and_run_pointer_table`: Copies three routine pointers from a table referenced by a5 into three global variables, then repeatedly loads the next pointer from that table and calls it until the pointer is zero.
- `jump_into_routine`: Loads a pointer from a global variable into a0, then jumps unconditionally into the middle of another routine without returning.
- `wrap_and_clamp_counter_offset`: Adds a signed input byte to a stored counter, and when the result is negative it resets the counter to the next table entry and returns zero, when it falls outside the current range it clears the counter and returns zero, otherwise it stores the new value and returns negative one.
- `tiny_ori_2`: Small routine of 1 instruction that merges 252 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 150 into d0.
- `process_control_characters`: Copies a table of routine pointers into global variables, then repeatedly calls the next pointer in the table until it is zero.
- `handle_control_characters`: Calls two routines, then compares a byte in d0 and returns when it is carriage return or zero.
- `branch_to_other_routine`: Consists of a single unconditional branch to another routine and does nothing else.
- `handle_byte_and_branch`: Compares a register with eight and returns otherwise, then decrements a global word and calls three helper routines before branching into another routine.
- `handle_byte_and_decrement`: Compares a register with four and returns to a shared routine when it differs, then tests a global word and calls a routine when it is nonzero, decrements that global, calls another routine, and jumps to a shared routine.
- `handle_byte_value_two`: Compares the byte in d0 with two, and when it matches it calls three different routines in sequence, each followed by a conditional branch.
- `conditional_call_by_index`: Tests whether a register equals sixteen and branches otherwise, then makes three conditional calls to other routines based on the results of the previous calls.
- `jump_via_indirect_table`: Jumps to the routine located at the address stored in the word at offset 234 of the structure in a6.
- `call_write_space`: Calls a routine that writes a space byte, restores the caller's registers, and returns.
- `shift_byte_into_string`: Swaps a byte from d1 into a string at an index derived from d0, shifting the original byte into d1, and stops when it reaches a zero byte, otherwise writing a terminating zero and returning.
- `shift_string_bytes_down`: Copies each byte from the following location into the current one until a zero byte is found, then returns with d0 unchanged.
- `store_underscore_at_index`: Writes the byte value 95 into the string location selected by d0 as an index, then returns.
- `store_space_at_index`: Writes the byte value 32 into the string location selected by d0 as an index, then returns.
- `test_byte_at_index`: Tests whether the byte at the string location selected by d0 is nonzero, then returns without modifying any data.
- `index_byte_via_pointer`: Reads a 16-bit index from a global variable at offset 20 and uses it to index into a byte array pointed to by a global pointer at offset 12, then returns the original index value in the register.
- `noop_d0_register`: It executes a single ori.b #0, d0 instruction, which leaves the contents of register d0 unchanged and performs no data movement.
- `write_zero_byte`: It writes a zero byte to the memory location pointed to by a0, then returns.
- `store_zero_byte`: It is a single instruction that writes a zero byte to the memory location pointed to by a0.
- `lookup_and_call`: It doubles the byte in d0 to form an index, looks up a pointer in a table, saves a5, calls the routine at address 56764, restores a5, and returns.
- `store_routine_address`: Loads the address of a routine into a register, stores that address into a global variable, and returns.
- `store_routine_pointer`: Loads the address of a routine into a0, stores that address into a global pointer at offset 6, and returns.
- `increment_and_negate_global`: Loads a global word, and if its sign bit is set it returns without changing anything.
- `increment_and_negate_globals`: Increments a 16-bit global variable at offset 20680 from a register value, negates that register, then increments a second global at offset 20700 and negates it when the first reaches 300.
- `swap_two_globals`: Increments a 16-bit value in a global variable at offset 20680, negates the register it came from, then swaps two 32-bit values held in global variables at offsets 38360 and 38364 before restoring registers and returning.
- `copy_bitmap_regions`: Calls a blitter wait routine, then copies four fixed regions from a source structure at offset 12 of a base pointer to a destination at a fixed address, repeating the copy in a loop that decrements a counter.
- `copy_string_return_byte`: Checks a global byte for zero and negativity, copies a string from a source buffer into a fixed destination buffer until a terminator is stored, then reads a byte from that buffer and returns it.
- `toggle_bit_call_string_check`: Toggles a bit in the byte at the address in a0 using a register, swaps two registers, decrements a counter, copies a word between buffers, loads a value from a table indexed by another register, and calls a routine that sets a flag when a string is empty, clearing a register when the flag is set, otherwise it returns the flag register.
- `compare_byte_jump_handler`: Compares a byte at the address in a0 with the constant 101 and, when they differ, loads a table address into a0 and jumps to a shared handler, otherwise it returns.
- `save_stack_check_word`: Loads a structure pointer and a word from global variables, saves the current stack pointer into a global variable, then branches to the string routine when the word is nonzero and otherwise returns immediately.
- `check_byte_value_101`: Compares the byte at the address in a0 with the decimal value 101 and, if they differ, calls a routine that sets a flag when a byte at offset 128 of the structure in a1 is not reached, then loads the address of a table into a0 and jumps to the routine at decimal 5422.
- `decrement_loop_copy_table`: Saves the current stack pointer into a global variable, decrements a global word counter by one, and enters a loop that pushes register a1, calls the copy routine, restores a1, and repeats while the counter stays non-negative.
- `decrement_counter_store_byte`: Decrements a global word counter and, while it stays non-negative, calls a routine storing its result byte into a buffer, advancing that pointer, then stores the pointer and clears a register before returning.
- `save_stack_pointer_decrement`: Saves the stack pointer into a global variable, decrements a global word counter by four, and returns when the counter goes negative, otherwise it falls through to the following routine.
- `store_long_and_advance_pointer`: Stores a 32-bit value through a pointer, advances that pointer by one word, clears the high byte of a register, and returns.
- `save_registers_check_string`: Saves the stack and register a1 into global variables, pushes a1 on the stack, calls a routine that sets the zero flag when a string is empty, and returns when that flag is set, otherwise it restores a1.
- `copy_table_bytes_loop`: Pushes register a1, calls a routine that sets the zero flag when a string is empty, and when set copies sixteen bytes from a table into a structure eight times in a loop, stores the resulting pointer and the constant 128 in globals, then restores a1 and clears a register.
- `save_registers_store_table`: Saves registers a1 and the stack pointer into global variables, pushes a1 on the stack, calls a routine that sets the zero flag when a string is empty, stores the table address and the constant 128 in globals, then restores a1 and clears a register.
- `fill_buffer_with_constant`: Saves the stack pointer and loads a buffer pointer, and while a global counter is non-negative fills the buffer with the constant 26 in a loop, then calls a routine and a helper before clearing a register and returning.
- `store_table_address_flag`: Calls a routine that sets the zero flag when a byte at offset 128 of the structure in a1 is not reached, saves the address of a table in a global word at offset 12, stores the constant 128 in the following word, and returns the flag register.
- `do_nothing_2`: Contains a single instruction that is a zero word decoded as an operation changing nothing, so it does nothing.
- `store_byte_via_pointer`: It writes the low byte of the 16-bit value in d0 to the memory location pointed to by a6, then returns.
- `do_nothing_3`: Contains two consecutive instructions that are zero words decoded as operations changing nothing, so it does nothing.
- `write_zero_byte_2`: It is a single instruction that writes a zero byte to the memory location pointed to by d0, then returns.
- `write_zero_byte_3`: It is a single instruction that writes a zero byte through a pointer, then returns.
- `store_constants_in_globals`: Stores the decimal value an address into a 16-bit global variable, then stores the same value into another 16-bit global variable, then stores the decimal value an address into a third 16-bit global variable, then stores the decimal value an address into a fourth 16-bit global variable, and finally returns.
- `store_constants_globals`: Writes the constant an address into a global variable and into three other global variables, then returns.
- `tiny_ori_4`: Small routine of 1 instruction that merges 224 into d5.
- `find_null_in_string`: Walks a null-terminated string looking for its first zero byte, and when found writes the byte-swapped distance from the start into a slot derived from a table index, then returns a 64-bit value with the high byte set to 255.
- `process_directory_table`: Clears a 128-byte table in a global variable, then repeatedly calls another routine to read a byte and, if it is less than 16, processes 16-bit values from a second table by rotating them and setting a byte in the table when the value is greater than 435.
- `toggle_parallel_port_bit`: Loads a byte from a global variable, calls a helper routine, and uses the result to set and clear a single bit in a parallel port register, toggling it to the opposite state.
- `toggle_port_bit_delay`: Reads a byte from a global variable, calls a helper routine, adds two to that value masked to seven bits, sets and clears the corresponding bit in a CIAB port at offset 256, then calls another helper with a delay value before returning.
- `clear_and_store_result`: Clears a global byte, calls a helper routine, and stores the returned byte into another global variable before returning.
- `check_status_code_range`: Loads a pointer to a text string, then returns immediately unless register holds zero or a value greater than three, in which case it branches to a shared handler.
- `init_ports_check_disk`: Sets two global words at offsets 1E36 and 21C0 to 65535 and clears two global bytes at offsets 1E38 and 2166, then polls the parallel interface A port and returns 255 when bit 2 is set.
- `poll_joystick_after_calls`: Calls two helper routines with a value of two, then reads a bit from the joystick port and returns.
- `check_disk_protect_bit`: Tests bit 3 of the CIA parallel interface A port and, when that bit is clear, branches to a shared error handler using a loaded pointer.
- `check_drive_ready_bits`: Tests bit 5 of the CIA parallel interface A port, then bit 2, and when the second bit is clear branches to a shared error handler, otherwise clears a register and returns.
- `handle_interrupt_vector_setup`: Saves the current interrupt vector, then calls four helper routines and stores a byte from a structure into a global variable.
- `copy_matching_string_2_2_2`: Initializes a global variable to 127 and sets a 16-bit counter to 3, then scans a table of strings for one matching the input, copying the matching string to a destination and returning the result.
- `match_and_copy_string`: Loops through a table of strings, comparing each against an input and copying the first match into a destination buffer, then calls check_drive_ready_bits to test the parallel port and copy_matching_string to handle the matched string.
- `init_ports_and_match_string`: Initializes two global words at offsets 30 and 8608 to 65535 and clears two global bytes at offsets 3030 and 8550, then scans a table of strings for one matching the input, copying the matched string into a destination buffer and calling a string-handling routine.
- `call_helpers_increment_counter`: Saves the stack pointer into a global variable, calls three helper routines in sequence, then repeatedly increments a byte counter and returns 255 when it reaches four.
- `restore_stack_and_return_error`: Restores the stack pointer from a global variable, loads 255 into a register, and returns.
- `compare_and_increment_byte`: Reads a byte from the structure in a1 and compares it against a byte at offset 15, storing the incremented value at offset 12 and clearing the byte at offset 32 when the first byte is at least 128.
- `check_flag_increment_structure`: Saves register a0, reads a byte from the structure in a1, and branches to a shared error handler when that byte is 128 or greater.
- `write_zero_through_pointer`: This routine contains only a single instruction that writes a zero byte to the memory location pointed to by register d0, then returns.
- `copy_matching_string_2_2`: Initializes a global variable to the value 127 and sets a 16-bit counter to 3, then scans a table of strings for one matching the input, copying the matched string into a destination buffer and calling the string-handling routine.
- `store_and_copy_string`: Sets a global variable to 255 and calls a routine that scans a table of strings for a match, copying the matched string into a destination buffer.
- `copy_matching_string_3`: Clears a 20-byte region starting 13 bytes past the pointer in a1, then scans a table of strings for one matching the input string, copying the matched string into a destination buffer and calling the string-handling routine.
- `copy_matching_string_2`: Scans a table of strings for one matching the input, then copies the matched string into a destination buffer and calls the string-handling routine, storing the saved registers when the handler is active.
- `copy_matching_string`: Scans a table of strings for one matching the input, then copies the matched string into a destination buffer and calls the string-handling routine, otherwise it copies a fixed string and returns.
- `noop_d0_eight_times`: Performs eight sequential no-op byte OR operations on register d0, each of which leaves the register unchanged.
- `return_255`: Loads the stack pointer from a global variable, returns the byte value 255 in a register, and returns.
- `lookup_byte_low_nibble`: Reads a nibble from a byte at offset 32 of the structure in a1 and uses it as an index into a table at offset 17, then stores the low four bits of the original byte into a byte at offset 32 of the same structure.
- `shift_and_add_pointer`: Saves several registers, calls another routine, then shifts the returned value left by seven bits and adds it to a pointer in a0 before restoring the registers and returning.
- `process_byte_table_lookup`: Reads a byte from the structure in a1 at offset 17 and stores copies into offsets 17, 18, and 19 of the same structure, then uses the high nibble of that byte as an index into a table starting at offset 16 to fetch a byte, shifts the low byte at offset 15 left by eight bits, and stores the fetched byte there, and finally masks the original byte with fifteen.
- `copy_table_and_set_flag`: Saves several registers, calls another routine, adds a shifted value to a pointer, copies four longwords from a source table to a destination table in a loop that decrements a counter, sets a global byte to 255, then restores the registers and returns.
- `compare_and_loop_routines`: Compares a signed word input with a stored counter and returns immediately when they differ.
- `init_disk_check_empty_string`: Clears two bytes of the structure in a1, reads its first byte, and returns 255 when that byte is zero.
- `call_helpers_clear_register`: Saves the stack pointer into a global variable, calls four helper routines in sequence, clears a register, and returns.
- `loop_display_on_flag_unset`: Returns zero when a flag byte is set, returns the value 255 when a word exceeds 435, otherwise loops the display-configuration routine three times while incrementing a value, then clears the flag byte and returns.
- `copy_table_on_divisor_remainder`: Divides a signed byte in d0 by eleven, calls the display-configuration routine with the quotient, and when the division leaves a remainder of ten it copies thirty-two longs from the source pointer in a0 to the destination pointer, then returns the original byte value.
- `advance_pointer_on_divisor_remainder`: Divides a signed byte in d0 by eleven and calls the display-configuration routine with the quotient, then when the division leaves a remainder of ten it advances the pointer in a0 by five12 bytes and returns it, otherwise returning the value from the display-configuration routine.
- `set_high_bits_d5`: Sets bits 5, 6 and 7 of register d5 and then returns.
- `copy_table_after_display_setup`: Sets bits in two registers, calls the display-configuration routine with a divided value, then copies thirty-two longs from the source pointer in a0 to the destination pointer and returns the modified register values.
- `call_display_on_divisor_quotient`: Divides a signed byte in d0 by eleven, calls the display-configuration routine with the quotient, and returns when the division leaves a nonzero remainder, otherwise falling through into the following routine.
- `compare_and_configure_display`: Compares a word in d0 with a global and, when different, stores the new value and calls three helper routines.
- `copy_table_for_display`: Compares a word in d0 with a global and, when different, stores the new value and calls three helper routines.
- `configure_custom_registers`: Checks a global byte and returns when it is nonzero, otherwise sets a custom register to a mask, calls a helper routine, lowers the mask, and branches into another routine.
- `fill_structure_loop`: Sets a bit in a global table based on a register value, then runs a loop of ten iterations that fills a structure with a rotating pattern, records bytes from the register and a counter, and advances through the table, calling several helper routines to transform and store 32-bit values.
- `fill_array_rotating_value`: Decrements a 16-bit counter in d6, writes a rotating value into successive words of the array pointed to by a1, and repeats until the counter reaches zero before returning.
- `fill_array_with_constant`: Writes the constant 43690 into successive words of the array pointed to by a1, decrementing a counter in d6 until it reaches zero before returning.
- `xor_array_and_combine`: XORs together all words of the array pointed to by a0 using a counter in d6, then combines the result with a saved value in d1, shifts it right, masks it, and returns the value in d0.
- `transform_word_buffers`: Runs two dbra loops that transform 32-bit words copied from two buffers, interleaving bits from the source with a carry bit, then writes a value to a custom register and returns.
- `init_bits_if_unset`: Reads a 16-bit value from a global variable, indexes a bit table using its low three bits, and returns without doing anything if the corresponding bit is already set.
- `interleave_odd_bits`: If the difference of the two 16-bit values in d7 and d6 is non-negative, it interleaves the odd bits of the two 32-bit words read from the table at a6 and the buffer at a0 into the destination at a1, then returns; otherwise it stores the 32-bit value at a3 into a global variable and clears two other global variables.
- `pack_bits_into_bytes`: Rotates bits out of successive words of the array pointed to by a0 into a carry chain, packs the extracted bits into bytes written to the array pointed to by a1, repeating for each source word before returning.
- `compare_loop_values`: It compares the 16-bit value at the address in a0 with the constant 17545, and if they are not equal it decrements the counter in d7 and branches back to the comparison; otherwise it jumps to the routine at 25ce.
- `empty_routine`: This routine is empty and does nothing.
- `configure_custom_and_wait_flags`: Writes control words to the custom graphics and interrupt registers, waits for a flag bit in a CIA port to be set, stores a global address into a custom register, then waits for another flag bit before writing a final control word and returning.
- `wait_port_bit_clear`: Reads a global byte and, when it is zero, sets a bit in a CIAB port, then polls a joystick port until a bit clears while toggling another bit between polls.
- `store_zero_byte_2`: It writes a zero byte to the byte pointed to by the address in a0, then returns.
- `handle_port_and_return`: Returns 255 when a global flag is nonzero or a register exceeds 160, otherwise saves a register, calls a helper routine on its shifted value, restores the register, sets or clears a bit in a custom register depending on the least significant bit, and returns the value from the helper routine.
- `update_and_pulse_port`: Compares a passed word with a stored global word and returns when they are equal.
- `clear_port_bit_delay`: Adds three to the byte in d0 masked to seven bits, clears bit 7 and then the bit selected by that value in a CIAB port at offset 256, calls a helper with a delay value, and returns.
- `set_cia_port_b_bit`: Takes the low three bits of the value in d0, adds 3, then sets that bit in the CIA port B register at offset 256 and also sets bit 7 of the same register before returning.
- `call_and_return`: Pushes three registers on the stack, calls a helper routine, then restores the registers and returns the original value carried in d0.
- `call_loop_configure`: It calls the routine at 2926, then decrements the counter in d1 and branches back to that call while the counter is nonzero, otherwise it returns.
- `configure_ciab_pra`: Reads the byte at offset 224 of the CIAB port in a4, clears its two least significant bits, sets bit 3, and writes the result back.
- `reset_ciab_parallel_port`: Reads a byte from the CIAB parallel interface port A, clears its low six bits, sets bit 3, and writes the result back, then resets the CIA timer to 127 and clears the CIA interrupt request flags at the two other ports.
- `clear_memory_block`: It loads a pointer from a global variable at offset 18504, adds 22576 to it, then clears 300 consecutive 32-bit words (1200 bytes) in that block using a loop that repeats 100 times before returning.
- `zeroed_data_area`: This routine is a zero-filled data area that other code refers to by address, containing no instructions of its own.
- `wait_blitter_and_configure`: Waits until bit 6 of the custom DMA control register at offset 2 clears, then decrements a global byte counter, resetting it to zero when it underflows.
- `advance_pointer_and_setup`: Advances a global pointer by 8000 bytes, stores it at offset 84 of a global table and at offset 84 of the structure in a6, writes the word 4233 at offset 88 of that structure, and sets a status byte at offset 54 of the global variable to 8.
- `advance_pointer_and_setup_9`: Advances the same global pointer by 8000 bytes, stores it at offset 84 of a global table and at offset 84 of the structure in a6, writes the word 4233 at offset 88 of that structure, and sets a status byte at offset 54 of the global variable to 9.
- `advance_pointer_and_setup_6`: Advances the same global pointer by 8000 bytes, stores it at offset 84 of a global table and at offset 84 of the structure in a6, writes the word 4233 at offset 88 of that structure, and sets a status byte at offset 54 of the global variable to 6.
- `advance_pointer_and_setup_10`: Advances the same global pointer by 8000 bytes, stores it at offset 84 of a global table and at offset 84 of the structure in a6, sets the size word at offset 64 to 511, writes the word 25609 at offset 88 of that structure, and sets a status byte at offset 54 of the global variable to 10.
- `advance_pointer_and_setup_0`: Advances the same global pointer by 8000 bytes, stores it at offset 84 of a global table and at offset 84 of the structure in a6, writes the word 4233 at offset 88 of that structure, clears the size word at offset 102, and sets a status byte at offset 54 of the global variable to 0.
- `configure_blitter_bitmap`: Waits for the blitter to finish, then sets up a bitmap transfer by computing a source address from a global pointer, clearing a blitter register, and configuring the transfer size and format before returning.
- `configure_amiga_blitter`: Computes a pointer by adding 32000 to the value read from the global variable at offset 38336, stores that pointer into the BlitRegister at offset 84 of the custom chip registers, writes 511 to the BltCon0 register at offset 64, writes 25620 to the BltSize register at offset 88, clears the byte at offset 33336, and returns.
- `configure_blitter_bitmap_offset`: Waits for the blitter to finish, then sets up a bitmap transfer using a source address derived from a global pointer, clearing a blitter register, and configuring the transfer size and format before returning.
- `configure_blitter_bitmap_2`: Computes the address of a 5120-byte bitmap region by adding 48000 to a base pointer stored in a global variable, programs the blitter with that address and a size of 512, and sets a flag byte to 1, then returns.
- `setup_dispatch_entry`: Clears the upper bits of a byte at offset 5 of a structure using a mask, then stores the address of this routine into three consecutive global pointers.
- `build_dispatch_table`: Swaps three global pointers in a ring, then records a clamped value plus an offset into bytes at offsets 15 and 17 of a structure in a1 and another clamped value plus an offset into a byte at offset 22.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `configure_blitter_bitmap_source`: Waits for the blitter to finish, then sets up a bitmap transfer using a source address derived from a global pointer, configuring the transfer size and format before returning.
- `store_pointer_and_call_helpers`: Loads the address of a routine into a0, stores that address into a global pointer at offset 6, calls two helper routines in sequence, and returns.
- `read_joy_port_split`: Reads the joy port data, splits it into high and low bytes, stores each byte to a global variable, and sets two other global variables to 256 before returning.
- `clear_two_counters`: Clears the 16-bit value at offset 32 of the structure in a6 and the 16-bit value at offset 36 of the same structure, then returns.
- `wait_for_flag_and_dma`: Spins in a loop until a byte in a global variable becomes zero, then spins again until bit 6 of the custom DMA control register at offset 2 clears, and finally returns.
- `check_cop_and_dispatch`: Saves a6, checks bit 5 of a custom register and returns if set, then checks bit 6 and returns if set, then loads a global address, advances it by four, stores it at offset 128 of the custom chip, calls the dispatch routine, and jumps to a shared routine.
- `dispatch_blitter_and_update_state`: Waits for the blitter to finish, then dispatches to a routine selected by a global counter and sets a custom register, or otherwise updates pointer and palette state and copies eight words into a table before calling a handler.
- `return_from_exception`: Pushes a return address onto the stack and executes a return-from-exception, restoring processor state and returning to supervisor mode.
- `store_index_ten`: Loads the constants 10, 3, and 1 into registers d0, d1, and d2, then calls another routine to store the byte in d0 at the table index derived from d1, and returns.
- `store_index_thirtythree`: Loads the constants 33, 2, and 1 into registers d0, d1, and d2, then calls another routine to store the byte in d0 at the table index derived from d1, and returns.
- `call_and_decrement_counter`: Invokes the routine whose address is held in a global variable, decrements a 16-bit counter in another global variable, and if that counter reaches zero it sets a third global byte to 255 before returning.
- `store_byte_at_index`: Multiplies the byte in d1 by four and stores the byte in d0 into the table at the offset computed from that multiple, then returns.
- `pack_and_write_palette`: Reads a word from a global table and splits its nibbles, dividing each by 16 to build a packed value that it writes into 32 consecutive words of the color palette register block at offset 384 of the custom chip, repeating the outer setup until a counter reaches 1.
- `decrement_and_copy_palette`: Decrements a 16-bit counter in a global variable unless it is zero or ten, then copies sixteen consecutive words from a table into the color palette registers.
- `copy_word_to_globals`: Copies the 16-bit word at offset 38 of the structure in a0 into a global word and into the offset-86 fields of three different structures, then returns.
- `clear_table_and_branch`: Loads an address into a register, clears the word it points to, loops fifteen times decrementing a counter while advancing the pointer, then jumps to a shared routine.
- `copy_tables_and_pulse`: Copies fifteen 32-bit words from one global table to another, then copies the word at offset 56 of that table into a second global and stores the value at offset 4 of a third global there, then copies thirty 16-bit words from the first table into a custom register bank, and finally stores a global word into a custom register.
- `apply_bitmask_from_table`: Loops over a byte string, and for each non-zero byte applies an 8-bit mask derived from a lookup table to clear or set bits in a block of memory, repeating the mask operation eight times per byte, until it reaches the zero terminator, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `check_and_set_display_pointer`: Saves a0, tests bit 0 of a custom register and returns when it is set.
- `apply_bitmask_table_2`: Computes a table index from two bytes in a0 and stores a pointer derived from it into a global variable, then clears a second global variable.
- `apply_bitmask_table`: Reads a byte from a table indexed by a value in d0, then sets or clears the corresponding bits in five 8000-byte regions of a global variable using the byte and its complement, advancing the table pointer until it hits a zero byte or a byte equal to 27, at which point it returns.
- `process_byte_dispatch`: Returns immediately when a register is zero, otherwise saves a0, calls a helper routine, advances a pointer, loads a byte and returns when it is zero, calls another routine when it equals twenty-seven, otherwise builds an index from the byte, calls a routine via a table, adjusts a pointer, restores a0, and returns.
- `setup_pointer_and_branch`: Clears the pointer in a3 to zero, loads the address of a table into a4, and branches into another routine.
- `clear_table_bits_from_byte_2`: Reads a byte from the buffer pointed to by a3, derives a bit pattern from it, and clears the corresponding bits in 15 bytes of a table.
- `clear_register_and_load_table`: Clears the 16-bit register a3, loads the address of a table into a4, then branches to another routine.
- `setup_pointer_and_table`: Clears the 32-bit pointer in a3 and loads the address of a table into a4, then branches to another routine.
- `apply_inverted_byte_masks_2`: Calls a helper routine seven times in a row, then reads a byte from the address in a3, inverts it, and ANDs the inverted value into the bytes at offsets -8000, 0, 8000 and 16000 of the structure in a2, and finally ORs the original byte into the value at offset 24000.
- `setup_pointer_and_branch_2`: Loads the value 65464 into the pointer in a3, loads the address of a table into a4, and branches into another routine.
- `resolve_indexed_pointer`: Checks bit 0 of a global flag and stores the source pointer and a3 value into two globals when it is set, otherwise it computes an index from two consecutive bytes read from the address in a0, adds it to a base table and an offset of 8000, and stores the resulting address into a global.
- `index_table_by_byte`: Reads a byte from the location in a2, subtracts 65 from it, multiplies the result by eight, and loads a 32-bit word from a table indexed by that value into d5 before returning.
- `lookup_and_call_byte`: Saves a6, loads a byte from the address in a0, subtracts sixty-five, triples the result to form an index, looks up a pointer in a table, calls the routine at that address, restores a6, and returns.
- `process_byte_via_pointer`: ORs several constant bit patterns into the byte in d0, saves a1 and a3 on the stack, loads a3 with a fixed address, calls a helper routine, adds that address to a2, restores the registers, loads d0 with the next byte from the buffer in a0, and returns.
- `compute_offset_and_return_byte`: Reads two consecutive bytes from the address in a0, combines them into an index by multiplying the second byte by 40 and adding the first, adds that index to a stored base address, stores the result into a global, and returns the third byte from the address in a0.
- `set_flag_and_return_zero`: Sets bit 1 of a global byte, loads zero into d0, and returns.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 177 into d4, then merges 0 into d0.
- `zero_word_data`: Contains a single instruction that ORs 0 into d0, and is most likely a zero-filled data area referenced by address rather than executed.
- `set_and_clear_bits_in_block`: ORs 27 into the byte in d0, then clears bit 15 of the byte at offset -8000 of the structure in a2 while setting bits in the bytes at offsets 0, 8000, 16000 and 24000, advances a2 by 40 bytes, and returns.
- `apply_complement_byte_mask`: Reads a byte from the table pointed to by a3, stores it in a1, and writes the bitwise complement of that byte to the structures at offsets -8000 and 0 while writing the original byte to the structures at offsets 8000 and 16000, then advances a3 by 40 bytes and returns.
- `clear_table_bits_from_byte`: Reads a byte from the buffer pointed to by a3 and computes a bit pattern from it, then clears the corresponding bits in 15 bytes of a table pointed to by a2.
- `read_byte_from_a3`: Reads a byte from the address in a3 and returns it in d0 without modifying the pointer or any other state.
- `apply_inverted_byte_masks`: Reads a byte from the address in a3, inverts it, and ANDs the result into the bytes at offsets -7968, -8000, 8000 and 16000 of the structure in a2, then ORs the original byte into the byte at offset 24000.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_from_jump_table`: Clears bit 0 of the flag byte at 42048, reads a byte from a0 and scales it by four to index a jump table, then calls the routine at that entry.
- `call_then_branch`: Calls the routine at the first target and then branches to the second target without returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_routine_list`: Reads a pointer from a5, then repeatedly calls each routine in the null-terminated list it points to, passing the following pointer as an argument, until a null entry is reached.
- `set_values_and_call`: Stores the values 198 and 199 into the two consecutive words at 36370, then calls the routine at 31714.
- `set_values_and_call_alt`: Stores the values 80 and 199 into the two consecutive words at 36370, then calls the routine at 31714.
- `save_register_and_call`: Saves register a5 on the stack, calls a helper routine, restores a5 from the stack, and returns.
- `invoke_blitter_channel_setup`: Saves register a5 on the stack, calls configure_blitter_channel, restores a5 from the stack, and returns.
- `call_with_offset_pointer`: Loads a pointer from a5, advances it by four, and calls the routine at a fixed address.
- `configure_and_run_blitter`: Calls the routine at 32592, then configures the blitter by storing two source pointers, the values 2544 and 6400, and the mask value 64000 into the custom registers, calls the blitter routine again, and finally stores 1024 into the DMA control register.
- `configure_blitter_channel`: Programs a Blitter channel by loading its source and destination pointers into the custom registers, setting its size and control words, then triggers the Blitter twice via the driver routine and finally clears the channel.
- `restore_interrupts`: Enables interrupts by writing to the interrupt enable register, then restores the previous a5 register and returns the value passed in d0.
- `jump_and_return`: Jumps to another routine, then restores the a0 register and d1 register from the stack before returning.
- `loop_forever`: Executes an infinite loop that never exits and never returns to the caller.
- `reset_globals_and_loop_check`: Writes the value 255 into a global byte, calls a helper routine, then repeatedly calls another routine in a loop that continues while a flag remains set, loads 255 into d0, and returns.
- `check_and_clear_globals`: Clears three global variables when the not-flag is set, then checks them in sequence and returns a distinct bitmask depending on which are zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_other_routine`: Unconditionally jumps to another routine and does not return.
- `call_empty_stub`: Jumps to another routine that returns immediately, so this routine does nothing and returns to the caller.
- `manage_structured_counters`: Decrements a global counter and, when it reaches zero, resets it to six and returns.
- `init_entity_struct`: Initializes a per-entity structure from two index values, sets up its movement vectors, and enables the corresponding DMA channel and interrupt bit in the custom chips, otherwise it returns without changes.
- `set_state_to_six`: Sets a global variable to six and returns.
- `advance_structured_counters`: Decrements a global counter and, when it reaches zero, resets it to six and returns.
- `clear_and_set_globals`: Clears a 16-bit value at the address in a0, sets a global variable to one, then sets it to 255.
- `copy_table_and_set_global_2`: Disables DMA, then copies sixty-four longs from a global table to a table pointed to by another global when a flag is cleared.
- `tiny_ori_5`: Small routine of 1 instruction that merges 96 into d6.
- `process_global_table_entries`: Walks a small table pointed to by a global variable, calling an unnamed routine for each of the first four entries that is not minus one.
- `test_table_byte_return`: Loads the address of a small table into a register and tests a byte there, then returns immediately when that byte is zero.
- `process_command_loop_dispatch`: Increments a 16-bit counter in a global variable and returns without changes when it matches a stored value, otherwise it decrements a second counter and returns.
- `clamp_byte_counter_value`: Decrements a byte at offset 1 of a structure by one and stores the result back, then clamps a 16-bit value at offset 14 to the range 0 to 64 and writes that clamped value through a second pointer, otherwise it leaves the value unchanged.
- `clamp_and_process_command_loop`: Clamps a 16-bit value in a structure to a range defined by a counter and a step, storing the result through a second pointer, then iterates over a table of commands using two index pointers and dispatches to a shared handler for each command.
- `clamp_value_to_113`: Subtracts a byte read from offset 2 of the structure in a5 from the 16-bit value at offset 12, then clamps the result to a maximum of 113 by storing 113 when it exceeds that limit.
- `store_value_and_branch`: Stores the 16-bit value from the variable at offset c in a5 into the 16-bit value at offset 6 in a4, then jumps to another routine.
- `advance_raster_index_loop`: Decrements a counter in a global variable and returns when it is zero, otherwise runs a four-iteration loop that advances a table pointer, copies data through it, and indexes a jump table using a masked value from the structure.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `manage_raster_and_table_indices`: Checks a global flag and returns without effect when it is clear, otherwise clears a second flag, stores a global word into a structure, and repeatedly decrements a counter until it underflows, then loads four indices from a table and stores them into a structure.
- `copy_structure_byte`: Copies a byte from the structure field at offset 2 into the field at offset 15, then jumps into another routine.
- `advance_raster_index_loop_2`: Computes an index from a byte in a global variable, stores it along with a flag value, then advances a table pointer and loads a sequence of 16-bit values into registers and into fields of a structure, before jumping to another routine.
- `copy_table_and_set_global_2_2`: Clears bit 4 of the byte at offset 11 of the structure in a5, then loops over a table in a global variable decrementing a counter each iteration and copying a long from a source table to a destination table until the counter reaches zero.
- `process_buffer_byte_dispatch`: Reads a byte from a buffer and, depending on its value, resets a global variable to a fixed constant or stores incremented and decremented copies of the byte into two fields of a structure plus a flag, then jumps to a shared exit routine.
- `clamp_and_process_command_loop_2`: Clamps a 16-bit value in a structure to a range defined by a counter and a step, storing the result through a second pointer, then iterates.
- `decrement_and_copy_register_state`: Decrements a global counter and, if non-zero, updates a set of limit-check variables using a bit-masked value from that counter.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_6`: Small routine of 1 instruction that merges 6 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_vertical_blank_and_call`: Waits for the vertical blank flag in the custom chip to clear, then stores a constant long at offset 128 of that chip.
- `store_self_reference_global`: Loads the address of the following instruction into a register, stores it into a global variable at offset 108, and returns.
- `increment_counters`: Increments a 16-bit global variable, wrapping it to negative when it reaches 128.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_addresses`: Stores the address from a0 into a global variable, then stores the addresses of two other global variables into the following global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.

### Zeewolf 2 - Wild Justice.adf bytes 438784-440320 (code never loaded by a reference run)

- `load_and_clear_pointer`: Loads a pointer from a global variable, clears a word at that address, then stores a pointer from another global into a third global and returns.
- `combine_byte_table_sums`: Accumulates the sum of three byte tables indexed by a counter, divides that sum by the counter, stores the result into a word table, and repeats for successive word entries until the index reaches fifteen.
- `compute_and_combine_words`: Calls a helper routine, adds three passed words together, divides the sum by five, combines the quotient with the shifted sum into a single word, branches to a shared routine, and then contains several unused instructions that clear a custom register, fill a table, and store values into globals.
- `compute_average_and_fill_table`: Calls a helper routine, adds the values in three registers plus two, divides the sum by three, shifts the original sum right by two, combines both results into d0, and returns.
- `average_three_registers`: Calls a helper routine, adds the values in three registers plus two, divides the sum by three, and stores the quotient.
- `sum_and_divide_by_three`: Calls a helper routine, adds the values in d1, d2 and d3, divides the sum by three, and combines the quotient shifted left four bits with the result shifted right two bits before returning.
- `isolate_nibbles`: It splits the 16-bit value in d0 into its high and low nibbles, clears the nibble it does not need, and returns the original value unchanged.

### Zeewolf 2 - Wild Justice.adf bytes 444928-495104 (code never loaded by a reference run)

- `expand_byte_pattern`: It reads a byte from the table in a3, inverts it, and writes the original and inverted values to five spaced locations in the structure in a2, then advances the pointer and returns.

### Zeewolf 2 - Wild Justice.adf bytes 503808-504832 (code never loaded by a reference run)

- `copy_table_and_clear_fields`: Copies sixty-four words from one structure to another, then clears three runs of twenty words and initializes several counter and pointer fields of a structure before returning.
- `copy_table_and_set_global`: It disables DMA and interrupts, then, when a flag is clear, copies 64 words from a global variable to another structure and sets that structure based on four sentinel checks before restoring interrupts and DMA.

### Zeewolf 2 - Wild Justice.adf bytes 882176-883200 (code never loaded by a reference run)

- `initialize_struct_from_pointers`: It clears three 21-word blocks, then reads pointers and offsets from a structure in a0, computes indexed addresses into the structure in a3, initializes several fields, sets a global flag, and returns.
