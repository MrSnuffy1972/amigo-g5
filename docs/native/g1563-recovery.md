# Game 1563 (monster business (1992)(eclipse)[cr vf][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01563). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 87 raw-word lines |
| Functions named and described | pass, 214 of 214 named (0 author symbols), 214 of 214 described (11 by rule, 203 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 165 of 165 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 130268 bytes, 205 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6912 of 8380 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 146680 |
| code | 3860 |
| data | 204164 |
| padding | 7280 |
| unused | 539136 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| Monster Business (1992)(Eclipse)[cr VF][a2].adf:monster | 1 | 178 | 6 |
| Monster Business (1992)(Eclipse)[cr VF][a2].adf:bootblock | 1 | 10 | 1 |
| Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-5 (raw code file, load address unknown) | 1 | 816 | 9 |
| Monster Business (1992)(Eclipse)[cr VF][a2].adf:mons#unpacked (raw code file, load address unknown) | 1 | 7259 | 196 |

## Cases handled

- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-0: LOB: LOB's File Compressor-packed, unpacked with ancient (26246 -> 60044 bytes)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-1: LOB: LOB's File Compressor-packed, unpacked with ancient (31716 -> 78918 bytes)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-2: LOB: LOB's File Compressor-packed, unpacked with ancient (30692 -> 62802 bytes)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:mons: LOB: LOB's File Compressor-packed, unpacked with ancient (54932 -> 123612 bytes)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-3: LOB: LOB's File Compressor-packed, unpacked with ancient (6708 -> 32000 bytes)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-4: LOB: LOB's File Compressor-packed, unpacked with ancient (12712 -> 32000 bytes)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-5: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf:mons#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- monster business (1992)(eclipse)[cr vf][a2]/Monster Business (1992)(Eclipse)[cr VF][a2].adf: blocks 187-187 (bytes 95744-96256) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_disk_interrupt_and_status` (mons#unpacked (raw code file, load address unknown)): Updates a pointer in a global variable with a value from a register, modifies the interrupt request register, and updates several values in a structure pointed to by a6. Kind: disk hardware/track validation check. Pass path: bpl.b an address.
- `configure_hardware_and_status` (mons#unpacked (raw code file, load address unknown)): Sets several memory locations, calls multiple subroutines, and uses a loop to update status based on a table and bit testing. Kind: disk hardware/track check. Pass path: the branch following the call to sub_B340 that does not lead to a failure state (likely the path where d1 is loaded with specific status codes like an address or an address and then passed to a verification routine).
- `initialize_memory_and_call` (mons#unpacked (raw code file, load address unknown)): Writes several constants to memory, calls multiple subroutines, uses a loop to write a longword to a table, and calls another subroutine using a pointer in a1. Kind: disk hardware/track check. Pass path: bmi.b an address.

## Functions


### type

- `allocate_and_copy_data_block`: Retrieves a pointer from a table, calculates a new size, and calls another routine to allocate memory.
- `move_and_branch`: Moves the contents of register d1 to register d0 and then performs a short branch.

### Monster Business (1992)(Eclipse)[cr VF][a2].adf:monster

- `initialize_hardware_registers`: Sets a pointer to the custom hardware registers in a6, writes 32767 to several offsets of a6, writes 135 to a specific hardware register, and stores a pointer to a local address into a memory location.
- `copy_and_jump`: Sets the status register, initializes several pointers, copies a byte from one location to another in a loop, pushes a value onto the stack, and performs a jump using a pointer from a table.
- `copy_data_block`: Copies a block of bytes from a source pointed to by a1 to a destination at 1024, using a count from a byte in the source, then stores a word from d0 into a global variable and returns via an interrupt return.
- `process_data_blocks_3`: Copies a block of bytes, performs a series of conditional calls to other routines, and uses a loop to move bytes into a structure.
- `move_and_branch_2`: Moves a longword from the address in a0 to d0, copies a byte from d0 to a location pointed to by a5, modifies the status register, and then performs a branch to another routine.
- `process_data_blocks_2`: Performs a series of conditional branches and calls to other routines, including a loop that moves bytes from one location to another using a decrementing counter.

### Monster Business (1992)(Eclipse)[cr VF][a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Monster Business (1992)(Eclipse)[cr VF][a2].adf:BD60-5 (raw code file, load address unknown)

- `execute_branches`: Performs four relative branches to different locations.
- `clears_data_register_zero`: Performs five consecutive logical OR operations with zero on data register zero.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `copy_bytes_to_structure`: Calls a subroutine four times with different PC-relative addresses in a0, then stores four bytes from PC-relative locations into a structure at offsets -1772, -1771, -1770, and -1769 relative to a0.
- `update_sprite_and_audio_data`: Decrements several counters within a structure, updates sprite coordinate lists by reading from a table, and performs fixed-point multiplication to calculate new values.
- `contains_zeroed_data_bytes`: This routine consists of a sequence of bytes that represent the value zero when interpreted as data.
- `contains_null_byte_sequence`: This routine consists of a sequence of instructions that perform a logical OR operation with zero on a data register, effectively acting as a data table of zero bytes.
- `contains_zeroed_data_table`: This routine consists of a sequence of instructions that perform logical OR operations with zero on a data register, effectively acting as a table of zeroed bytes.
- `manage_hardware_and_data`: Clears several audio registers, performs bitwise operations on a value, manipulates a structure by copying several longwords, and executes a complex loop involving table lookups and arithmetic to update various status fields.

### Monster Business (1992)(Eclipse)[cr VF][a2].adf:mons#unpacked (raw code file, load address unknown)

- `initialize_system_and_counters`: Clears several global words and longwords, performs a loop to write zeros to a hardware register area, calls several subroutines, and conditionally calls another routine to update a global word based on a calculation involving a byte and a multiplication.
- `dispatch_event_and_update_state`: Loops until a byte from a source is greater than ten, then performs several subroutine calls, updates a global word by dividing a value from a global word by fifteen, and branches back to the start of the routine.
- `initialize_memory_block`: Writes three longwords to specific memory locations and then copies a block of data from a source to a destination in a loop before jumping to the destination.
- `initialize_hardware_state`: Writes a word to 1066, clears a value at 1062, and enters a loop that clears a byte at 33344 for 15 iterations before branching to another routine.
- `setup_system_state`: Disables interrupts, initializes several custom hardware registers via a6, calls another routine, and performs a long sequence of bitwise OR operations on d0.
- `update_system_state`: Sets several pointers, clears a register, and checks hardware bits to decide whether to call a subroutine, update several longwords, and perform multiple calls to other routines before returning via an interrupt return.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compare_byte_to_self`: Compares a byte loaded from an offset to itself and returns if they are equal, otherwise it returns.
- `write_incrementing_values`: Writes a series of words to a destination pointed to by a0, including a constant and values derived from a1, using a loop that runs 15 times before returning.
- `update_global_byte`: Checks two global bytes and, if certain conditions are met, writes a byte from a table to a global variable at 8178.
- `process_input_and_interrupts`: Reads joystick and mouse status from hardware registers, updates global bytes at 7514 and 7515, and performs complex operations involving DMA, custom hardware, and interrupt management before returning via an interrupt return.
- `initialize_hardware_and_state`: Sets a value in a global variable, writes several longwords to a table, configures hardware registers, and calls three subroutines.
- `execute_conditional_subroutines`: Checks three longwords in a global variable and calls a subroutine via a pointer if the value is non-zero, otherwise it returns.
- `update_status_and_registers`: Compares a byte from a global variable with a value from a table, and if they differ, copies the table data to a global location, modifies two global bytes, and returns via an exception.
- `process_global_state_and_counters`: Performs several subroutine calls, updates a longword at a global address, and enters a loop that increments a global word and a counter while comparing a value from a table to a global word.
- `call_subroutines`: Calls two different subroutines and then returns.
- `run_loop_with_call`: Sets a counter to five, pushes it onto the stack, calls a subroutine, pops the counter, and loops using the dbra instruction until the counter reaches zero.
- `set_global_pointers`: Checks a word in a global variable and, if it is zero, writes several longwords and words from various sources into specific global memory locations.
- `copy_data_to_globals`: Checks a word in a global variable and, if it is zero, writes four values from PC-relative locations into a series of global memory locations.
- `initialize_and_copy_tables`: Clears a memory location in a loop, performs two calls to other routines, copies a word from one location to another, and copies a longword from one location to another.
- `manage_global_state_and_tables`: Executes a large sequence of subroutines and jumps, performs bitwise operations on global bytes, and contains conditional logic that updates global words and longwords based on various tests.
- `decrement_and_call_subroutine`: Decrements a word in a global variable and, if a bitwise test on a derived value passes, calls a subroutine using a pointer from a table.
- `modify_global_values`: Checks a byte in a global variable and, if it matches a specific value, writes two words to global locations and potentially adds a value to another global word based on a comparison.
- `update_global_counters`: Calls a subroutine, writes a longword and two words to global variables, and then decrements a word in a global variable, potentially clearing a longword if the result is not positive.
- `update_counters_and_pointers`: Sets a longword in a global variable, clears and modifies a word in another global variable, and calls a routine to process data using a pointer and a counter.
- `clear_or_copy_memory`: Either clears a word at the address in a0 or copies a longword from a0 to a1, looping based on the value in d0.
- `tiny_moveq`: Small routine of 1 instruction that loads 15 into d2.
- `process_and_store_modified_words`: Loops through a list of words, performs bitwise masking and arithmetic based on bit tests, and stores the resulting words into a destination table.
- `manage_complex_data_structures`: Performs complex operations involving table lookups, loops to clear memory, and conditional data copying into global structures, eventually returning via an exception.
- `increment_and_process_counters`: Checks two global bytes, increments a word in a global variable, and performs conditional logic to write a word to a global variable or call a subroutine and update a global word.
- `process_table_entries`: Checks two values at offset 0 and offset 40 of a table in a0 against bounds in d2 and d3, then if they are within range, calls a routine with d0 and d1 as arguments, increments a global word, and clears the first byte of the table in a loop.
- `process_table_and_structure`: Calls a subroutine, performs arithmetic on values from a structure, and enters a loop using a dbra instruction to call another subroutine while modifying structure members.
- `branch_if_not_negative`: Calls a subroutine, performs a bitwise AND on d0 with three, moves the result to d7, decrements d7 by one, and branches if d7 is positive.
- `update_structure_from_table`: Loads values from a table, performs arithmetic, and calls several subroutines to populate a structure with derived values, including bitwise operations and pointer updates.
- `decrement_and_check_zero`: Checks if the byte at offset 0 of a0 is zero, otherwise it enters a loop that decrements d0 and moves a pointer to a0 until d0 is no longer positive, finally setting a0 to all ones and returning.
- `call_multiple_subroutines`: Calls five different subroutines in sequence and then returns.
- `execute_multiple_calls`: Compares a longword in d0 against a constant, and if it is not less, increments a global longword and a global word before performing four separate calls to a routine with varying d0 and d1 values.
- `execute_conditional_table_update`: Compares a word in d0 against two values, and if they match, stores a longword at a global address, calls a subroutine, and performs a loop that fetches a byte from a table and calls another routine to update data.
- `update_multiple_values`: Checks a word against a constant, and if it matches, performs a series of operations including calling subroutines, using a loop to update values with an incrementing offset, and performing bitwise tests.
- `update_value_from_table`: Compares a word from a global address to a constant, and if they match, updates a global word, performs a loop that calls a subroutine and another routine to update data from a table.
- `update_zero_value_table`: Compares a word to a constant, and if they match, sets d0 to zero, updates a global word, and enters a loop that calls two subroutines to update data from a table.
- `subtract_and_store_loop`: Subtracts a longword from register d0, then enters a loop that adds values from a table to d0 and stores a byte in a destination for a set number of iterations.
- `apply_bitwise_or`: Performs a series of bitwise OR operations on registers d0 and d1 using various immediate byte values.
- `extract_masked_data`: Calculates an offset into a table using d0, then performs a loop that uses bit manipulation on d0 and d1 to select and bitwise AND several words from the table into a6.
- `rotate_and_or_data`: Calculates an offset using d1 and d3, fetches four words from a table at a calculated address, rotates each word by the value in d0, and performs bitwise OR operations to update four consecutive words at a target address, repeating this process in a loop five times.
- `branch_negated_value`: Negates the byte in d0 and branches to another routine.
- `fetch_table_value`: Adds d0 to itself and uses the result as an offset to load a word from a table into d0 before returning.
- `manage_complex_data_flow`: Executes multiple library calls, performs nested loops to copy data, updates various global pointers and words, and manages hardware-related parameters through several subroutines.
- `apply_zero_or`: Performs eight consecutive bitwise OR operations on register d0 using the immediate byte value zero.
- `write_structured_data`: Performs bitwise operations and writes a series of words to a destination pointed to by a0 using a loop.
- `write_structured_data_loop`: Calls subroutines to write incrementing values, perform bitwise OR operations, and write structured data using a loop.
- `write_incrementing_values_2`: Writes a series of words to a destination pointed to by a0, including a constant and values derived from a1, using a loop that runs 15 times, then calls a routine to perform bitwise OR operations on register d0 using the immediate byte value zero.
- `update_pointer_value`: Writes a word to a longword at a specific address, calls another routine three times, and then subtracts a value from that same address before returning.
- `reset_pointer_value`: Clears a longword at a specific address, calls another routine three times, adds a value to that address, and then compares it to a constant before returning.
- `load_pointers_and_word`: Loads two PC-relative longwords into a0 and a1, loads a word from a PC-relative location into d0, and moves the value 31 into d2 before branching to another routine.
- `setup_word_processing_decrement`: Writes a word to a global variable, calls a subroutine three times, and decrements a global word, returning if the result is not positive.
- `setup_word_processing_increment`: Clears a global word, calls a subroutine three times, and increments a global word, returning if the result is less than a specific value.
- `setup_word_processing`: Loads two pointers into a0 and a1, moves a word from a PC-relative address into d0, sets d2 to 15, and branches to another routine that performs bitwise masking and arithmetic on words.
- `call_subroutines_and_modify_word`: Writes 1911 to a longword at a PC-relative offset, calls three subroutines, subtracts 273 from the same longword, and branches if the result is positive.
- `clear_and_compare_word`: Clears a longword at a PC-relative offset, calls three subroutines, adds 273 to that longword, compares it to 2184, and branches if it is less than that value.
- `prepare_data_processing`: Loads two PC-relative pointers into a0 and a1, moves a word from a PC-relative location into d0, and branches to another routine.
- `setup_word_processing_subtraction`: Writes a word to a global variable, calls a subroutine three times, and subtracts a value from a global word, returning if the result is not positive.
- `update_counter_and_branch`: Clears a 16-bit word at a global address, calls three separate routines, adds 273 to that same global address, and then branches to an earlier instruction if the value is less than 2152.
- `execute_dual_word_processing`: Loads two different sets of pointers and word values into registers, then calls another routine twice to perform bitwise masking and arithmetic on words using different loop counts in d2.
- `search_byte_sequence`: Compares bytes in a table pointed to by a0 against d0 in a loop, and if a byte is not the end marker, toggles bit 0 of a global word; otherwise it stores a pointer into a global longword and returns.
- `copy_structured_data`: Copies words from a source table in a0 to specific offsets within a destination structure in a1 using two nested loops.
- `update_global_pointers`: Copies a longword from a global address into d0, stores it into two other global addresses, writes a constant to a global address, writes a word to another global address, and performs three sign-flag updates to global bytes.
- `update_blt_parameters`: Performs complex calculations involving bitwise operations and hardware registers to update Blitter and Copper settings, including a loop that iterates through data structures.
- `process_structured_data`: Calculates a pointer from a word, calls a routine, and then executes three separate loops that use a byte from a table to calculate an offset, call another routine, and iterate using d7.
- `copy_data_to_structure`: Copies four 16-bit words from a source pointer to specific offsets within a structure in a1, then decrements the pointer and loops sixteen times.
- `format_string_data`: Subtracts a value from a pointer, then enters a loop that adds a constant to a value and stores the result into a destination buffer.
- `manipulate_large_data_block`: Performs complex data movement and bitwise operations involving multiple nested loops, register saving, and word-wise transfers between various memory locations and structures.
- `copy_and_clear_tables`: Performs many bitwise OR operations on d0, copies 35 words from a table to another location using a loop, and then clears 91 longwords using a second loop.
- `copy_table_data_2`: Calls a subroutine, then runs a loop 35 times to copy words from a source at a0 to a destination at a1.
- `initialize_buffer_and_save_registers`: Clears a memory block pointed to by a global variable using a loop, stores a pointer to that block in another global variable, saves the current registers to a global location, and calls another routine after performing conditional checks on two global bytes.
- `initialize_system_state`: Clears a longword at an address using a loop, calls several subroutines, updates multiple global longwords and words, and performs conditional loops based on register and memory checks.
- `update_table_and_dispatch_event`: Updates a global pointer and clears a memory block, then checks a byte against a table to determine if it should increment or decrement a counter in a global variable and copy a value from a table.
- `increment_counter_conditionally`: Reads a byte from a table, compares it to a value in d0, and either increments d0 by a constant or sets it to a specific value based on the comparison.
- `load_address_and_jump`: Loads a pointer from a global address into a0, loads a constant into a2, and branches to another routine.
- `initialize_structured_data`: Clears a block of memory using a loop, calls two subroutines, copies 32 words from one location to another using a loop, and performs several conditional checks and writes to various global variables.
- `load_address_and_jump_alt`: Loads a pointer from a global address into a0, loads a different constant into a2, and branches to another routine.
- `calculate_pointer_offset`: Loads a pointer from a global address into a0, loads a word from a relative offset into d0, doubles d0 twice, and uses the result to load a pointer into a2 before branching to another routine.
- `update_system_state_2`: Checks a byte at a global address, clears a memory area using a loop, copies a table of words into a structure using nested loops, and updates a global pointer before calling several subroutines.
- `call_subroutine_and_update_byte`: Calls a subroutine, loads a pointer from a global variable and adds 32000 to it, checks a byte at a global address, and if certain conditions are met, calls another subroutine or writes a modified byte to a location relative to a0.
- `load_data_and_call_subroutines`: Loads a longword into d0, loads a pointer into a0, calls a routine, adjusts a0, loads a pointer into a1, calls another routine, loads a new longword into d0, and repeats the pointer loading and routine calls.
- `convert_ascii_to_byte`: Reads a byte from a source, converts it from an ASCII character to a numeric value, and stores it in a destination buffer within a loop of five iterations.
- `store_registers_and_clear_memory`: Calls two subroutines, writes 10 to a global word, clears a global word, and if a global word is non-zero, saves d0-d7 to a global location, calls another subroutine, and clears a block of memory using a loop.
- `convert_and_store_ascii`: Loads a pointer into a0, performs several operations involving calling a routine and adding 48 to bytes loaded from memory to store them into global variables, then performs a loop that clears a memory area 8 times before returning.
- `process_and_loop_data`: Performs ASCII conversion and stores bytes into global variables, calls a routine with a loop count of 30, and contains a loop that decrements a global word and branches back to the start of the routine if the result is positive.
- `save_and_clear_memory`: Calls two subroutines, saves registers d0-d7 to a global address, calls another subroutine, and if a comparison passes, enters a loop to clear eight bytes of memory at a specific address.
- `process_large_data_sequence`: Performs multiple loops and subroutine calls, including a loop that updates a global longword, a sequence that converts bytes to ASCII by adding 48, and several other conditional data processing blocks.
- `clear_memory_block`: Clears a block of memory starting at an offset from a pointer in a0 using a loop that runs 7999 times.
- `copy_multiple_words`: Loads four words from a table into registers and then stores them at specific offsets within a structure, using nested loops to repeat the process.
- `update_structure_with_ascii`: Loads a pointer from a global address, calls a routine, performs arithmetic on d0, calls another routine, converts three bytes from memory to numeric values by adding 48 and stores them in a destination pointed to by a1, and calls a third routine.
- `load_word_from_table`: Loads a 16-bit word from a table using an index calculated from the sum of a0 and a7 multiplied by eight.
- `calculate_and_copy_offsets`: Calculates two destination addresses by multiplying bytes from a2 by constants and adding them to a0, then calls another routine if a byte from a2 is not negative.
- `copy_structured_data_2`: Calculates a source address using a byte from a table and a bit shift, then copies several 16-bit words from that source to a destination structure in a loop.
- `perform_obfuscated_data_manipulation`: Performs a series of bitwise OR operations, moves bytes between several pointers, and executes various bit tests and moves without any loops or calls.
- `update_global_pointers_2`: Loads a word from a table using a value at a constant offset, performs an exclusive OR on a byte at a global address, and stores the resulting word into two different global locations.
- `manage_global_state`: Writes two words to global memory, performs bitwise OR operations on d0, and executes multiple conditional branches that call subroutines to update various global words and longwords based on arithmetic results.
- `process_conditional_data`: Tests bit 3 of a byte from a memory location, then either calls two subroutines and updates a word at offset 6b9c, or performs arithmetic on d0, compares it to 298, and updates a word at offset 6b8c before potentially calling two other subroutines.
- `test_bit_and_branch`: Tests bit 0 of d7 and branches if it is set, otherwise it returns.
- `check_global_flag`: Tests a word in a global variable and returns either 4 or 2 in d0 depending on whether the value is zero.
- `store_status_flags`: Stores two bytes at specific memory locations, moves a word into a third location, moves a longword into a fourth, and then calls another routine.
- `process_complex_conditional_logic`: Performs extensive conditional logic involving multiple subroutines, bitwise operations on d0-d7, and updates various global words and longwords based on comparisons and bit tests.
- `evaluate_and_update_scaled_values`: Performs multiple comparisons and arithmetic operations on values loaded from memory, calls several routines to fetch data, and updates various global variables based on bit tests and conditional branches.
- `compare_scaled_values`: Compares two values derived from constants and d0 by adding offsets and shifting right by 4, or compares two values derived from d0 and constants by adding d0 to them and shifting right by 4, then updates a global word and two global pointers based on the results.
- `fetch_and_or_values`: Adds 16 to d0, calls another routine, then adds 18 to d0, calls another routine, and finally performs a bitwise OR between d0 and d6 before returning.
- `check_and_jump_condition`: Tests a byte at a global address, branches if zero, calls another routine, compares a word at a global address to 11, and calls a third routine if the comparison matches.
- `calculate_jump_address`: Calculates an address by multiplying a word from a table by a value at an offset of a structure in a5, adds this to a pointer from the same structure, and jumps to the resulting address.
- `calculate_offset_and_jump`: Calculates a value using d0 and constants, checks a bit in a local variable, and if set, calculates an address by multiplying a value from a table by a word in a structure and adding it to a pointer, potentially adding an additional offset before jumping to a target address.
- `update_pointer_and_counter`: Checks a byte in a global variable and, if non-zero, checks if a word at a pointer plus 4 equals 1000; if so, it updates a global pointer, adds a value to a global word, and performs a comparison to decide whether to return or continue.
- `update_global_state`: Performs extensive conditional logic involving bit tests, memory writes to several global locations, and multiple loops that manage data transfers and pointer increments.
- `clear_status_and_flags`: Clears a word at a global address, clears bit 1 of a byte at an offset of a structure in a6, and either clears a global word or performs a conditional branch based on a global byte.
- `decrement_and_search_pointer`: Loops through a list by checking if a word at the pointer in a0 is negative, incrementing the pointer by 4 and decrementing d0 until the word is non-negative or d0 reaches zero, then sets a0 to a constant value.
- `initialize_global_structure`: Checks a word in a global structure and, if zero, writes 9 to that location, updates a second field in the structure with a pointer, and copies a value from a table into a third field.
- `process_table_entry_and_call`: Checks a word in a global structure, and if non-zero, extracts a value from a table, performs arithmetic including a multiplication and several additions, and calls a routine via a pointer in a structure.
- `execute_nested_loops`: Compares a word at a specific memory location to two, then enters a loop that calls a routine using a pointer from a table, followed by a second loop that calculates an offset and calls another routine using a pointer from a table.
- `clamp_value_to_range`: Subtracts a value from a word, compares it against constants, and performs bit shifts and negations to store a result in a global variable.
- `set_jump_target`: Loads a longword into a6 and sets d7 to 99 before branching to another routine.
- `loop_structure_processing`: Runs a loop 20 times that accesses a structure in a6, calls a subroutine via a pointer in a4 if bit 0 of offset 68 is set, performs arithmetic and calls another subroutine if a value at offset 10 is within a specific range, and uses a multiplication to calculate an offset for a jump table call.
- `update_structure_bounds_and_state`: Validates coordinate ranges against a structure in a5 and a6, then updates a global structure with calculated values and calls another routine to process updates.
- `initialize_structure_data`: Saves the stack pointer, calls a routine, populates a structure in a2 with values derived from a0, and calculates a pointer to a table using a bit-shifted value.
- `call_three_subroutines`: Calls three different subroutines and then returns.
- `call_four_subroutines`: Calls four different subroutines and then returns.
- `update_entity_attributes_and_table`: Compares a value from a structure with a constant sum from a table, then performs arithmetic on structure members including bitwise masking and pointer increments.
- `update_complex_values`: Performs extensive arithmetic on structure members in a6, including additions, subtractions, and bitwise operations, while calling several subroutines and using conditional branches to update various words and longwords in the structure.
- `process_structure_data`: Performs conditional logic on a structure, calls a subroutine via a jump table, updates a global longword based on a word value, and uses a loop to call another subroutine.
- `copy_table_data`: Loads a longword from a specific address into a0, calls another routine, and then copies 16 words from the address in a1 to the address in a0 using a loop.
- `calculate_table_offsets`: Performs complex arithmetic and table lookups involving multiple structure members, calls another routine, and uses a loop to update a structure.
- `select_structure_value`: Performs arithmetic on values from a structure in a5 and a6, calls another routine, and then conditionally stores one of three longwords into offset 46 of the structure in a6 based on the results.
- `calculate_conditional_values`: Performs arithmetic on values from a structure, calls a subroutine, and uses bit testing to branch based on the result.
- `update_global_values`: Calculates values using structure members and calls another routine multiple times, performing conditional arithmetic and branching to update various offsets in a structure and global variables.
- `decrement_and_return_2`: Decrements a word at an offset of a6 and returns if the result is not negative, otherwise it returns.
- `process_structure_updates_2`: Loads a pointer from offset 36 of a structure, performs conditional logic involving table lookups and calls to another routine, and updates various structure members based on the results.
- `update_structure_pointer`: Decrements a value in a structure, calls another routine, and performs arithmetic on a longword in d0 before returning.
- `update_pointer_and_value`: Loads a pointer from a structure, increments a byte in a global variable, and writes one of two possible long values to a global variable based on a byte test.
- `initialize_structure_from_table`: Loads a word from a table into a structure, calculates a new pointer, and updates structure members based on a conditional check.
- `increment_and_check_pointer`: Increments a pointer in a register by two, checks if the word at that location is not one thousand, and if so, updates a global pointer with the value at the new address.
- `adjust_pointer_based_on_sign`: Checks a byte in a global variable; if negative, it decrements a pointer and checks if the word at that location is one thousand to potentially update a global pointer, otherwise it increments the pointer and performs a similar check.
- `update_structure_with_table_data_2`: Checks a 16-bit value at offset 0 of a structure in a6 and returns if it equals 20; otherwise, it updates a pointer in the structure with a value from a table and conditionally modifies a 16-bit value at offset 44 based on a test.
- `update_structure_with_table_data`: Sets values in a structure, loads a pointer from a table, and then either calculates new values using table lookups or sets a specific constant in the structure.
- `count_down_to_null`: Tests a byte at a pointer, and if non-zero, enters a loop that decrements a register and increments a pointer until the byte is zero, then returns a null pointer.
- `modify_and_copy_data`: Performs multiple table lookups and arithmetic operations, calls several routines, and uses a loop to copy and modify data.
- `perform_bitwise_operations`: Tests a byte at a specific address, and if not zero, sets the sign flag and calls another routine twice using different pointer values in a1 and a2.
- `bitwise_combine_and_store`: Performs bitwise AND and OR operations on values from a source list using a mask, then stores the results into a destination structure at specific offsets within a loop.
- `modify_value_with_mask`: Subtracts d1 from a constant in d0, and if the result is greater than 0 and less than 15, calls another routine after modifying a value at a specific address; otherwise it performs different arithmetic and calls another routine.
- `bitwise_combine_and_copy`: Performs bitwise AND and OR operations on values from a source list using a mask, then stores the results into both a destination structure and a second destination structure within a loop.
- `process_complex_table_data`: Performs multiple table lookups and arithmetic operations to update several global pointers and bytes, including a loop that writes bytes to a structure based on calculated offsets.
- `fetch_table_value_2`: Calculates an index by shifting and multiplying registers, then fetches a byte from a table at a fixed address using that index.
- `fetch_table_byte`: Adds a constant to d1, calls another routine, and then fetches a byte from a table at an address in a0 using d7 as an offset.
- `process_structure_updates`: Loads a value from a table, performs arithmetic and bitwise operations on registers, and stores the results into a global structure.
- `update_structure_data`: Writes values to a structure at a2, then enters a loop that performs multiple memory-to-memory transfers and swaps of register contents into a table at a1.
- `modify_register_values`: Performs multiple bitwise OR operations on d0, modifies d4 and d7, and uses a conditional branch to perform arithmetic on a0.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_address_offset`: Performs arithmetic on d1 and d3 to calculate an offset, modifies a0, and then conditionally writes values to offset 68 and 70 of a structure at address a fixed address.
- `write_to_structure`: Loads an address into a3, adds d0 to d1, and writes a word from a table at offset d1 from a3 into offset 68 of a structure in a4, then writes 65535 to offset 70 of a4 before branching.
- `write_table_to_structure`: Writes a constant to a structure member, performs arithmetic on d0 and d1, and loads a value from a table into a structure member before branching.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 254 into d2, then merges 0 into (a4)+.
- `set_register_and_memory`: Sets bit 0 of d1 and writes a long value to the address pointed to by a6.
- `update_hardware_registers`: Manipulates bits in a6, performs arithmetic to calculate offsets, and calls several routines to move data from a table into a structure at a2 and update hardware registers at offset 104 and 112 of a4.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 254 into d3, then merges 0 into (a2)+.
- `set_register_and_memory_2`: Sets d3 to a specific value and writes zero to the address pointed to by a2.
- `set_register_and_memory_3`: Sets d1 to a specific value and writes a long value to the address pointed to by a6.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 0 into d2, then merges 0 into (a4)+.
- `update_hardware_registers_2`: Calculates a value based on d1 and d2, performs a loop to copy data from a source to a destination structure at a4, and updates hardware registers at a fixed address based on a value in d1.
- `tiny_ori_ori_4`: Small routine of 2 instructions that merges 254 into d4, then merges 0 into (a0)+.
- `set_register_and_memory_4`: Sets d4 to a specific value and writes zero to the address pointed to by a0.
- `set_register_and_memory_5`: Sets d3 to a specific value and writes a long value to the address pointed to by a2.
- `set_bit_and_update_pointer`: Sets bit 2 of a byte in a register and then stores a 32-bit constant into a memory location pointed to by a register, incrementing that pointer by 4.
- `tiny_ori_ori_5`: Small routine of 2 instructions that merges 0 into d3, then merges 0 into (a2)+.
- `tiny_ori_ori_6`: Small routine of 2 instructions that merges 2 into d2, then merges 0 into (a4)+.
- `process_data_blocks`: Performs several loops that read data from a source, modify it using bitwise operations and rotations, and write it into a structure at a4.
- `extract_encoded_data`: Calls three different routines and uses a loop to check bytes at a pointer in a4, performing arithmetic and bitwise operations on d7 and d6 before returning with a value in d0.
- `search_list_for_value`: Iterates through a list of long words pointed to by a0, comparing them to a constant and performing arithmetic on d7 to track progress until a match or the end of the list at a1 is reached.
- `process_data_buffer`: Decrements a value in d0 and, if not negative, enters a nested loop structure that modifies data at a pointer derived from a0 and d1, eventually comparing a value at a0 with a constant and writing two bytes to d6 and d7 before returning.
- `extract_and_transform_bytes`: Runs a loop based on d1 that reads bytes from a0, masks them, and combines them with values from a table into a single byte which is then stored at a4, followed by a sequence of bitwise operations and memory accesses using a pointer in a3.
- `write_encoded_bytes`: Performs a loop seven times that calculates a value using bit shifts and XOR operations, then writes the resulting byte to a table at a0 and another byte to an offset of 256 from the table base.
- `set_memory_constants`: Calls another routine and then writes two different 16-bit constants to offsets 36 and 158 of a structure in a6.
- `update_status_byte`: Writes the value 127 to the address in a4, compares a word from a memory location to zero, and if not zero, clears bits 2 and 3 of d1 before writing the result to a4.
- `write_multiple_bytes`: Writes the bytes 255, 247, and 255 sequentially to the address pointed to by a4.
- `get_constant_value`: Checks a bit in the address in a5 and returns either 254 or 0 in d0 based on the result of a comparison with a constant.
- `check_and_modify_status`: Checks a bit in a byte at a5, calls another routine in a loop, and then writes a word and a byte to global memory locations.
- `modify_status_word`: Checks a longword at a specific address, calls another routine, and then performs arithmetic on values from PC-relative locations to update a longword at a specific address, potentially entering a loop.
- `modify_status_bits`: Sets and clears specific bits in a byte at the address in a4, sets d0 to 2968, and calls another routine before returning.
- `clear_and_set_bits`: Clears bits 0 and 1 of a byte at the address in a4, sets bit 0 of that same byte, loads 44070 into d0, and calls another routine before returning.
- `decrement_and_return`: Decrements d0 and, if the result is not negative, branches back to the start of the routine before returning.
- `branch_with_value`: Loads 40960 into d1 and branches to another routine.
- `configure_disk_interrupt_and_status`: Updates a pointer in a global variable with a value from a register, modifies the interrupt request register, and updates several values in a structure pointed to by a6.
- `update_bit_field`: Checks specific bits in d1 and a0 within a loop, potentially setting or clearing bit 7 at an offset of 2 in a0, and finally writes a value from d0 to a0 and performs table-based XOR operations.
- `call_with_stack_save`: Pushes the contents of d7 onto the stack, calls another routine twice using d1, and returns.
- `configure_hardware_and_status`: Sets several memory locations, calls multiple subroutines, and uses a loop to update status based on a table and bit testing.
- `execute_sequence_of_calls`: Calls several routines with different values in d1, and uses a loop to call another routine based on a byte from a table.
- `loop_with_stack_save`: Loads 78 into d1, pushes d7 onto the stack, calls another routine, restores d7 from the stack, and loops using d7 until it reaches zero.
- `initialize_memory_and_call`: Writes several constants to memory, calls multiple subroutines, uses a loop to write a longword to a table, and calls another subroutine using a pointer in a1.
- `clear_memory_range`: Calculates a length based on a constant and a pointer, then enters a loop that clears bytes at the pointer address until the length reaches zero.
