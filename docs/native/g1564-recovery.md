# Game 1564 (monster business (1992)(eclipse)[cr vf][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01564). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 87 raw-word lines |
| Functions named and described | pass, 214 of 214 named (0 author symbols), 214 of 214 described (11 by rule, 203 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 165 of 165 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 130268 bytes, 205 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6912 of 8380 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 146448 |
| code | 3860 |
| data | 203628 |
| padding | 7024 |
| unused | 540160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| Monster Business (1992)(Eclipse)[cr VF][a].adf:monster | 1 | 178 | 6 |
| Monster Business (1992)(Eclipse)[cr VF][a].adf:bootblock | 1 | 10 | 1 |
| Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-5 (raw code file, load address unknown) | 1 | 816 | 9 |
| Monster Business (1992)(Eclipse)[cr VF][a].adf:mons#unpacked (raw code file, load address unknown) | 1 | 7259 | 196 |

## Cases handled

- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-0: LOB: LOB's File Compressor-packed, unpacked with ancient (26246 -> 60044 bytes)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-1: LOB: LOB's File Compressor-packed, unpacked with ancient (31716 -> 78918 bytes)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-2: LOB: LOB's File Compressor-packed, unpacked with ancient (30692 -> 62802 bytes)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:mons: LOB: LOB's File Compressor-packed, unpacked with ancient (54932 -> 123612 bytes)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-3: LOB: LOB's File Compressor-packed, unpacked with ancient (6708 -> 32000 bytes)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-4: LOB: LOB's File Compressor-packed, unpacked with ancient (12712 -> 32000 bytes)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-5: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf:mons#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- monster business (1992)(eclipse)[cr vf][a]/Monster Business (1992)(Eclipse)[cr VF][a].adf: blocks 187-187 (bytes 95744-96256) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_interrupts` (mons#unpacked (raw code file, load address unknown)): Sets several hardware registers including CIAB_ICR and DSKLEN, performs conditional logic involving a status flag, and uses rte to return from an interrupt context. Kind: disk hardware/track check. Pass path: bpl.b an address.
- `configure_hardware_and_sequence` (mons#unpacked (raw code file, load address unknown)): Sets several memory locations, calls multiple subroutines, and performs a loop that decrements d1 to call another routine, while also interacting with hardware registers. Kind: disk hardware/track check. Pass path: the branch following the bsr.w an address calls (likely the path where d1 is loaded with specific values like #an address or #an address and then passed to a subsequent routine).
- `execute_hardware_sequence` (mons#unpacked (raw code file, load address unknown)): Writes several values to global memory, calls multiple subroutines, and enters a loop to copy data from a table to a destination, followed by further conditional subroutine calls. Kind: disk hardware/track check. Pass path: bmi.b an address.

## Functions


### type

- `process_complex_data_structure`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 to d0 and then branches to another routine.

### Monster Business (1992)(Eclipse)[cr VF][a].adf:monster

- `initialize_hardware_registers`: Sets a pointer to the hardware registers in a6, writes 32767 to several hardware register offsets, writes 135 to a hardware register, and stores a pointer in a hardware register.
- `copy_and_jump`: Disables interrupts, sets up several pointers, copies a byte from a source to a destination in a loop, pushes a value onto the stack, and performs a jump to an address stored in a register.
- `copy_data_block`: Loops over an input byte stream with run-length style decoding into a buffer of 1024 bytes.
- `process_data_blocks_3`: Copies bytes from one memory location to another, then performs a series of conditional checks and calls to other routines while iterating through a loop.
- `transfer_and_branch`: Moves a longword from the address in a0 to d0, moves the byte in d7 to a hardware register, modifies the carry flag, performs a rotate through the left on d0, and returns; otherwise it branches to another location.
- `process_data_blocks_2`: Performs a series of conditional branches and calls to other routines, including a loop that decrements a value in d1 and uses d2 to perform arithmetic before returning.

### Monster Business (1992)(Eclipse)[cr VF][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Monster Business (1992)(Eclipse)[cr VF][a].adf:BD60-5 (raw code file, load address unknown)

- `execute_branch_sequence`: Performs four consecutive branches to different locations.
- `clears_data_byte`: Performs five consecutive logical OR operations with zero on a single byte in register d0.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `copy_data_to_structure`: Calls a subroutine four times with different addresses in a0, then stores four bytes from relative offsets into a structure at offsets -1772, -1771, -1770, and -1769.
- `update_internal_state`: Checks values in a structure pointed to by a0, performs multiple loops that decrement counters and update various offsets in the structure, and calls another routine using a pointer from a table.
- `contains_zeroed_byte_sequence`: This routine consists of a sequence of instructions that perform a logical OR operation with zero on a data register, effectively acting as a data table of zeroed bytes.
- `contains_zeroed_data_table`: This routine consists of a sequence of operations that perform a logical OR of zero onto a data register, which results in no change to the register state; it functions as a static table of zeroed bytes.
- `contains_zeroed_data_bytes`: This routine consists of a sequence of bytes that, when interpreted as instructions, perform logical OR operations with zero on a data register.
- `manage_hardware_state`: Modifies data at memory locations via pointers and updates audio volume, CIAA, and DMA control registers.

### Monster Business (1992)(Eclipse)[cr VF][a].adf:mons#unpacked (raw code file, load address unknown)

- `initialize_system_and_counters`: Clears several memory locations, performs a loop to write zeros to a hardware register range, calls multiple subroutines, and conditionally calls another routine to update a global word based on a multiplication result.
- `decrement_and_branch_on_byte`: Repeats by branching to itself until a byte from a relative offset is zero, decremented, or greater than ten; otherwise it performs a division, calls several subroutines, and branches to another routine.
- `initialize_memory_block`: Writes three longwords to specific memory locations, then enters a loop to copy 20 words from a source to a destination starting at a specific address before jumping to that destination.
- `initialize_hardware_state`: Writes a word to a memory location, clears a value at another location, and enters a loop that clears a byte at a destination pointed to by a0 for 15 iterations before branching.
- `setup_system_parameters`: Disables interrupts, writes several values to hardware registers, calls another routine, and then performs a long sequence of bitwise OR operations on d0.
- `update_system_state`: Updates several memory locations, checks hardware register bits, calls multiple subroutines, and performs arithmetic on registers before returning via an interrupt return instruction.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `compare_byte_to_self`: Loads a byte from an offset of the program counter into d0, compares it to itself, and returns if they are equal; otherwise it returns.
- `write_incrementing_values`: Writes a sequence of words to a destination pointed to by a0, where the values are derived from a constant and a byte from a1, and performs a loop for 15 iterations.
- `update_global_byte`: A routine consisting of fourteen instructions.
- `process_joystick_input`: A large routine that interacts with CIAA ports for joystick and mouse data, DMA controllers, and various custom hardware registers and interrupt settings.
- `configure_hardware_and_interrupt_state`: Interacts with custom hardware registers, DMA, and CIAA ports while calling a mid-length routine.
- `execute_conditional_subroutines`: A routine consisting of twenty-nine instructions.
- `update_status_and_memory`: A routine consisting of twenty-two instructions that is accessed via a relative address.
- `process_conditional_updates`: Calls several subroutines and library functions, clears a word in a global variable, and performs conditional updates to global variables and a table based on comparisons and a loop that repeats until a specific condition is met.
- `call_two_subroutines`: Calls two different subroutines and then returns.
- `call_subroutine_with_loop`: Calls a subroutine with a value of 5 passed in d7 and then executes a loop that runs 5 times using the dbra instruction.
- `set_multiple_memory_values`: A routine consisting of sixteen instructions.
- `copy_table_values`: A routine consisting of twelve instructions.
- `clear_and_copy_tables`: Clears a memory location in a loop, calls two subroutines, performs two jumps to external addresses, and copies values from one memory location to another using a loop.
- `dispatch_entity_updates`: Executes a large number of subroutines, including hardware register modifications and pointer-based memory storage.
- `decrement_and_call_subroutine`: A routine consisting of seventeen instructions.
- `modify_global_parameters`: Checks a global byte and, if it matches a specific value, updates several other global words with new values based on conditional checks.
- `update_global_counters`: Loads a subroutine address into a0, calls a subroutine, writes a longword to a global variable, and modifies two other global variables based on conditional branches.
- `update_counters`: Provides a relative address, loads a value into a register, and executes a nineteen-instruction routine.
- `clear_memory_in_loop`: Clears a word at the address in a0 and then executes a loop that decrements d0 until it reaches zero.
- `tiny_moveq`: Small routine of 1 instruction that loads 15 into d2.
- `process_and_store_masked_values`: Iterates through a table, subtracting a value from each entry and then adding specific increments to each entry until certain bits are cleared before storing the result in a new table.
- `update_entity_table`: Performs complex operations involving table lookups, loops to clear memory, and conditional data copying into global structures, eventually returning via an exception.
- `increment_and_check_flags`: This routine executes six instructions before calling another routine.
- `process_structure_data`: Iterates through a table of structures, checks if specific words within the structure fall within a range defined by other values, and if so, calls a routine and clears a word in the structure.
- `process_indexed_data`: Executes a sequence of ninety-four instructions including a ten-instruction routine.
- `check_and_return`: Calls a subroutine, performs a bitwise AND on d0, and returns if the resulting value in d7 is not positive.
- `process_entity_list`: Scans a table for a zero entry, and if found, populates a structure with values derived from several helper calls and table lookups.
- `decrement_and_loop`: Checks if the byte at offset 0 of a0 is zero; if not, it enters a loop that decrements d0 and moves a pointer to offset 26 of a0 until d0 is no longer positive, otherwise it sets a0 to all ones and returns.
- `call_sequence_of_subroutines`: Calls four subroutines in sequence and then returns.
- `execute_multiple_calls`: Compares two table constants and, when the first is at least the second, adds twenty thousand and one to the state and calls helpers.
- `process_conditional_table_update`: Executes a sequence of subroutines including one accessed via a relative address calculation.
- `execute_parameterized_updates`: Compares d0 against a constant, and if equal, performs multiple operations including calling subroutines, using a loop to add values to d2, and performing bitwise operations and table lookups.
- `update_structure_with_table_data`: Performs a series of subroutine calls including one accessed via a relative address.
- `update_zero_value_table`: This routine executes a sequence of three distinct subroutines.
- `subtract_and_store_loop`: Subtracts a longword from d0, enters a loop that runs seven times to add values from a table to d0 and store a byte in a destination, and then returns.
- `apply_bitwise_or`: Performs a series of bitwise OR operations on d0 and d1 using various immediate byte values.
- `extract_table_values`: Computes a shifted mask from an input register and writes a masked word into a table.
- `update_data_via_rotation`: This routine consists of forty instructions and calls another routine.
- `negate_and_branch`: Negates the low byte of d0 and branches to another routine.
- `fetch_offset_value`: Adds d0 to itself and uses the result as an offset to fetch a 16-bit word from a table into d0 before returning.
- `manage_complex_state`: Performs a large sequence of operations including copying table segments via loops, calling multiple subroutines, updating global pointers, and managing various counters and status flags.
- `apply_zero_or`: Performs eight consecutive bitwise OR operations on d0 using the immediate byte value zero.
- `write_sequence_to_memory`: A routine that performs bitwise operations and writes a sequence of values to a destination pointed to by a0.
- `write_sequence_to_memory_2`: An address-taken routine that calls other subroutines to perform bitwise operations and write sequences of values to a destination pointed to by a0.
- `write_incrementing_values_2`: Writes a sequence of words to a destination pointed to by a0, where the values are derived from a constant and a byte from a1, then calls another routine and performs bitwise OR operations on d0 using zero.
- `set_and_decrement_global`: Writes a word to a global variable, calls another routine three times, decrements that global variable, and returns if the result is positive.
- `clear_and_increment_global`: Clears a global variable, calls another routine three times, increments the global variable, and returns if the value is less than 888.
- `initialize_address_registers`: Loads two relative addresses into a0 and a1, moves a word from a relative offset into d0, and sets d2 to thirty-one before branching.
- `modify_and_call_subroutines`: Writes a word to a global variable, calls three subroutines, subtracts a value from that global variable, and returns if the result is not negative.
- `clear_and_check_variable`: Clears a global variable, calls three subroutines, adds a value to that global variable, and returns if the result is less than a constant.
- `setup_masked_values`: Loads two addresses into a0 and a1, sets d0 from a word at a relative offset, sets d2 to fifteen, and calls another routine.
- `modify_and_call_subroutines_2`: Writes a word to a relative offset, calls three subroutines, subtracts a word from the same relative offset, and returns if the result is not positive.
- `clear_and_compare_offset`: Clears a word at a relative offset, calls three subroutines, adds a word to that offset, and returns if the value at that offset is less than 888.
- `setup_masked_parameters`: Loads two addresses into a0 and a1, moves a word from a relative offset into d0, sets d2 to thirty-one, and branches to another routine.
- `modify_and_verify_global_word`: Writes a word to a global variable, calls compare_byte_to_self and process_multiple_masked_sequences three times, subtracts a constant from the same global variable, and returns if the result is non-negative; otherwise it returns.
- `increment_and_check_counter`: Clears a 16-bit word in a global variable, calls three different routines, increments that variable by 273, and returns if the variable is less than 2152.
- `process_multiple_masked_sequences`: Sets up two different sets of addresses and counters in a0, a1, and d2, then calls another routine twice.
- `search_and_update_pointer`: Compares the input byte with a table byte and, when they match and the next entry is not the end marker, advances the table pointer.
- `copy_table_segments`: The routine performs a sequence of operations involving a pointer to write data to a memory location.
- `update_global_pointers`: Copies a longword from a global variable into two other global variables, writes a constant to a global longword, writes a word to a global word, and performs three sign flag updates to global bytes.
- `update_blt_parameters`: Copies four 16-bit values from a source table into a structure pointed to by a6 at specific offsets, then enters a nested loop to perform complex calculations involving bitwise operations and hardware register updates for the Blitter.
- `manipulate_memory_via_pointers_2`: Calculates a value from a word, uses it to index a table to find a pointer in a0, and then performs four separate loops that call another routine to process data from a0 and a table into a destination pointed to by a2.
- `copy_structure_fields`: Copies four 16-bit words from a source pointed to by a5 into a destination pointed to by a1 at offsets 0, 44, 88, and 132, then decrements a counter in a loop.
- `increment_byte_table`: Calculates a value by subtracting a source from a constant, then enters a loop to add 48 to that value and store the result in a destination pointed to by a4.
- `copy_and_transform_data`: Performs one hundred and one instructions and calls another routine via a relative address.
- `copy_and_clear_memory`: Performs a long sequence of bitwise OR operations on d0, copies 35 longwords from a source to a destination via a loop, and then clears 91 longwords via another loop.
- `copy_memory_block`: Calls a subroutine, then enters a loop that copies 35 longwords from a source at a0 to a destination at a1 using a counter in d0.
- `initialize_system_state`: This routine calls four different subroutines of varying lengths.
- `initialize_and_loop_structures`: Performs multiple nested subroutine calls and writes a value to memory via a pointer.
- `update_menu_selection_and_buffer`: Writes a byte from a register into a memory location pointed to by another register and performs several subroutine calls.
- `compare_and_modify_byte`: Iterates through a null-terminated string to find a specific character and returns a value based on its position.
- `setup_pointer_and_offset`: Loads a pointer from a global variable into a0, loads an offset into a2, and branches to another routine.
- `initialize_hardware_and_buffers`: Executes a sequence of instructions including a relative address call and multiple subroutines of varying lengths.
- `load_pointer_and_branch`: Loads a longword from a global variable into a0, loads the address of the next instruction into a2, and branches to another routine.
- `calculate_dynamic_offset`: Loads a pointer from a global variable into a0, calculates an offset by doubling a value from a table, and branches to another routine using a pointer derived from that offset.
- `initialize_data_structures`: Executes a series of routines including one that provides a relative address and another that performs thirteen instructions.
- `update_offset_and_branch`: Calls a subroutine, loads a pointer from a global variable and adds 32000 to it, and then either branches to another routine or modifies a byte at an offset of 19 from a relative address before branching.
- `load_data_and_call_subroutines`: Loads a longword into d0, loads a pointer into a0, calls a routine, adjusts a0, loads a pointer into a1, calls another routine, loads a new longword into d0, loads a pointer into a0, calls a routine, adjusts a0, and loads a pointer into a1.
- `convert_ascii_to_byte`: Converts six digit values from a source buffer into ASCII characters, replacing leading zeros with spaces, and writes them through a destination pointer.
- `clear_and_save_registers`: Calls two subroutines, writes values to global variables, and if a specific global word is zero, saves d0-d7 to a global location and executes a loop that clears eight bytes at a relative offset.
- `process_ascii_and_offsets`: This routine consists of thirty-six instructions and performs a relative address calculation, a six-instruction routine call, and calls two different subroutines.
- `convert_and_store_ascii_digits`: This routine consists of forty-two instructions and calls a four-instruction routine, a six-instruction routine via a relative address calculation, and two different subroutines.
- `update_status_and_clear_memory`: Provides a relative address and modifies a custom hardware register.
- `perform_multi_stage_data_processing`: This routine invokes several short subroutines, including one that executes thirteen instructions and another that calls multiple subroutines.
- `clear_memory_block`: Clears a block of memory at an offset of 1280 from a pointer retrieved from a global address using a loop that runs 7999 times.
- `populate_data_structure`: Copies 16-bit words from a source buffer into four destination planes spaced twenty words apart, over eight groups of twenty steps.
- `process_entity_data`: This routine consists of thirty-five instructions and modifies a custom hardware register while calling a six-instruction routine, a single instruction, and two different subroutines.
- `load_word_with_offset`: Loads a 16-bit word from a memory location calculated by adding the product of a7 and 8 to a0 into d0.
- `calculate_and_copy_offsets`: Executes twenty instructions and calls two different subroutines.
- `copy_offset_data_block`: Copies sixteen rows of four interleaved bitplane words from a source block chosen by a table lookup, where the destination advances eighty words per row.
- `manipulate_memory_via_pointers`: Loads three bytes from incrementing pointers in a3, a5, and a7, then performs a series of byte-level transfers and bit tests between registers and memory locations pointed to by a0, a1, a2, a4, and a6.
- `update_pointer_and_xor`: Loads a word from a table using a value at an offset, performs an XOR on a byte at a global address, and stores the resulting word into two different global locations.
- `update_scaled_data_and_dispatch`: This routine consists of ninety-four instructions and calls several sequences of varying lengths.
- `update_scaled_values`: Tests bit 3 of a byte from memory, then either calls two subroutines and updates a word in a global variable and a longword in another global variable, or performs a negation and arithmetic on a word in a global variable before calling other subroutines.
- `branch_on_bit_test`: Tests bit 0 of d7 and branches to another location if the bit is set; otherwise it returns.
- `check_and_shift_value`: Tests a word in a global variable; if it is not zero, it shifts the value in d0 right by one and returns, otherwise it returns immediately.
- `set_global_flags`: Stores bytes at two specific locations, moves a word into a third location, moves a longword into a fourth, and calls another routine before returning.
- `manage_scaled_data`: Performs extensive logic involving multiple subroutines, bitwise operations, and conditional branches to update various global words and longwords based on comparisons and bit tests.
- `manage_scaled_values`: Performs multiple comparisons and updates to global variables using scaled values, calls several routines to calculate and store results, and contains branches that can restart the routine or exit based on various bit tests and comparisons.
- `compare_scaled_values`: Performs two different comparisons by adding constants and right-shifting values by four bits; if the comparisons match, it stores a pointer and updates a global word and longword, otherwise it returns -1 or 0.
- `calculate_bitwise_or`: This routine consists of twenty-one instructions and calls a ten-instruction routine.
- `branch_on_global_byte`: Tests a byte at a global address and branches if zero, otherwise calls another routine, compares a global word to eleven, and calls a third routine if the comparison matches.
- `calculate_offset_and_jump`: Calculates a new address by multiplying a word from a table by a value at an offset from a structure, adds it to a pointer, and jumps to the address stored at a specific location.
- `calculate_complex_offset`: Calculates an address using a multiplication and conditional additions based on a bit test and a global word, then jumps to a location via a pointer.
- `update_table_entry`: When a global flag is set, reads the current table entry and reloads the pointer when its marker is 1000.
- `initialize_and_update_structures`: Performs one hundred and sixty instructions and calls three different routines.
- `clear_and_set_flags`: Clears a word at a global address, clears bit 1 of a byte at an offset in a structure, and either clears a global word or performs a conditional branch based on a global byte.
- `decrement_and_search_pointer`: Loops through a list by checking if a word pointed to by a0 is negative; if positive, it increments the pointer and decrements d0, repeating until d0 reaches zero or a negative value is found, then sets a0 to a constant.
- `initialize_global_structure`: Checks a word in a global structure; if it is zero, it sets it to nine and updates a pointer in the structure with a value from another table, then loads a value from that table into a global longword.
- `process_table_and_call`: While a state word is nonzero, advances a 16-bit entry pointer by one word and clears the state on a marker of 1000.
- `execute_nested_loops`: When a counter exceeds one, invokes a stored routine for each of several indexed slots.
- `clamp_and_store_value`: Sets a global to a quarter of the distance by which a position lies outside a band from sixty to eighty.
- `set_loop_parameters`: Loads a longword into a6, sets d7 to 99, and branches to another routine.
- `process_entity_list_2`: Iterates through a list of 20 items using a loop, performing pointer arithmetic and calling a subroutine via a pointer based on bit 0 of a byte at offset 104 of a structure in a6.
- `process_complex_structure`: This routine consists of one hundred and forty-five instructions and modifies a custom hardware register while calling a six-instruction routine and a twenty-nine-instruction routine.
- `populate_new_structure`: Calls another routine and populates a memory structure with specific offsets using values derived from a pointer, constants, and global variables.
- `execute_sequence_of_subroutines`: Calls three different subroutines in sequence and then returns.
- `execute_four_subroutines`: Calls four different subroutines in sequence and then returns.
- `process_entity_state_transitions`: Performs multiple hardware register modifications and various subroutine calls.
- `modify_structure_state`: Performs complex arithmetic on values within a structure in a6, including conditional updates to a pointer at offset 46 and various word values, while calling multiple subroutines.
- `update_entity_state_2`: Modifies values in a structure pointed to by a6, calls a subroutine via a pointer, and enters a loop to process a list of data while performing various conditional updates to a global address and a structure.
- `copy_longword_in_loop`: Loads a longword from a memory address into a destination address using a loop that runs 21 times.
- `update_game_state_structure`: This routine consists of one hundred and sixty-two instructions and calls a seven-instruction routine and a ten-instruction routine.
- `select_pointer_by_logic`: Performs arithmetic on values from a structure in a6 and a5, calls another routine, and then conditionally stores one of several constant longwords into offset 46 of the structure in a6 based on the results and a byte at offset 67.
- `calculate_state_offsets`: Performs arithmetic on values from a structure in a6 and a5, calls a subroutine, and uses the result to perform conditional branches based on bit tests.
- `update_structure_and_list`: Performs a series of subroutine calls including a forty-two instruction routine and two ten-instruction routines.
- `decrement_and_branch`: Decrements a word at an offset of 44 from a6 and branches to another routine if the result is not negative; otherwise it returns.
- `validate_entity_state_parameters`: Checks specific bytes and words within an entity structure against values in lookup tables to determine a state transition.
- `update_entity_state`: Modifies a pointer at offset 24 of a6 and a value at offset 10 of a6, and calls another routine based on the value of a byte at offset 67 of a6.
- `update_structure_fields`: Copies a pointer from a structure offset to a4, increments a byte at an offset in a6, and then writes one of two constant longwords to an offset in a6 based on a byte test.
- `initialize_data_structure`: Sets a word from a source at offset ece4 to offset 0 of a6, calculates a new pointer for offset 2 of a6, and updates offsets 6 and 10 of a6.
- `increment_and_check_pointer`: Increments a pointer in a0 by two, checks if the word at that pointer is not 1000, and if so, updates a pointer in a6 with the value at offset 2 of the new pointer.
- `adjust_pointer_based_on_sign`: Moves a pointer at offset 24 of a structure forward or backward by two based on the sign of a value at offset 56, following a link if the 16-bit word at the new location is one thousand.
- `update_entity_parameters_and_state`: Checks a value at offset 0 of a6 and returns immediately if it equals 20; otherwise, it updates a pointer at offset 32 of a6, modifies a value at offset 44 of a6, and performs calculations involving a table lookup and arithmetic on a value from offset 10 of a6.
- `calculate_and_set_parameters`: Sets a record's type to five and its link to a fixed address, then sets its position from a table entry plus an offset from another record, and sets its state value to either four or negative four thousand nine thousanders based on a flag.
- `decrement_until_zero`: Tests a byte at a0; if non-zero, it enters a loop that decrements d0 and moves a pointer through a table until d0 reaches zero, then returns a pointer of all ones.
- `resolve_sprite_attributes`: Calculates memory offsets and updates a global state variable based on a recursive call.
- `apply_bitmask_logic`: Tests a byte at offset 35321, and if non-zero, it loads three pointers from a table and calls another routine twice using different register combinations.
- `apply_bitmask_to_array`: Merges sixteen rows of four 16-bit words into a destination by ORing each source word with a mask selected from a record, advancing the destination pointer by eighty words per step.
- `calculate_mask_offset`: Subtracts d1 from a value in d0, and if the result is between 0 and 15, it calls another routine with a modified d7 and updates a value at offset 35312 before branching to that routine again.
- `copy_masked_data_to_two_buffers`: Merges a specified number of rows of four 16-bit words under a mask, writing each resulting word to two separate destination blocks.
- `resolve_sprite_attribute_table`: Performs multiple table lookups and arithmetic operations to update several global pointers and bytes, including a loop that writes bytes to a structure based on calculated offsets.
- `fetch_table_value`: Calculates an index by adding the results of arithmetic shifts and a multiplication to a value in d1, then fetches a byte from a table at a specific address using that index and returns it in d7.
- `fetch_indexed_byte`: Adds a constant to d1, calls another routine, and then fetches a byte from a table at offset 43972 using d7 as an index into a0.
- `process_entity_data_list`: Modifies a custom hardware register and executes a sequence of one hundred and four instructions.
- `update_structure_and_copy_data`: Writes values to a structure at a2, then enters a loop that performs multiple memory-to-memory transfers and swaps of register contents into a table at a1.
- `modify_d0_and_branch`: Performs multiple bitwise OR operations on d0, manipulates d4 and d7, and branches based on the sign of d4.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_address_and_set_hardware`: Calculates an offset based on d1, performs arithmetic to determine a memory address, and conditionally writes values to hardware registers at a fixed address or calls other routines.
- `write_register_values`: Loads a pointer into a3, adds d0 to d1, writes a word from a table at offset (a3, d1) to offset 44 of a structure in a4, and then branches to another routine.
- `load_graphics_parameters`: Writes a word to offset 44 of a4, loads a value from a table into d1 using d0 as an offset, and then branches to another routine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 254 into d2, then merges 0 into (a4)+.
- `set_register_and_memory`: Sets bit 0 of d1 and writes a longword to the address pointed to by a6.
- `manipulate_graphics_parameters`: Performs arithmetic on d4 and d7, updates several longwords in memory, and uses a loop to call another routine while modifying hardware registers and structure fields.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 254 into d3, then merges 0 into (a2)+.
- `set_register_and_memory_2`: Sets d3 to a specific value and writes a longword of zero to the address pointed to by a2.
- `set_register_and_memory_3`: Sets bit 1 of d1 and writes a longword to the address pointed to by a6.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 0 into d2, then merges 0 into (a4)+.
- `configure_graphics_registers`: Calculates offsets and values to write several longwords to a destination pointed to by a2, and updates hardware registers at the a fixed address base address based on a value in d1.
- `tiny_ori_ori_4`: Small routine of 2 instructions that merges 254 into d4, then merges 0 into (a0)+.
- `set_register_and_memory_4`: Sets d4 to a specific value and writes a longword to the address pointed to by a0.
- `set_register_and_memory_5`: Sets d3 to zero and writes a longword to the address pointed to by a2.
- `set_register_and_memory_6`: Sets bit 2 of d1 and writes a longword to the address pointed to by a6.
- `tiny_ori_ori_5`: Small routine of 2 instructions that merges 0 into d3, then merges 0 into (a2)+.
- `tiny_ori_ori_6`: Small routine of 2 instructions that merges 2 into d2, then merges 0 into (a4)+.
- `process_data_blocks`: Writes a run of blitter entries, advancing an address by forty bytes per entry and waiting for the busy flag before each one.
- `search_and_extract_data`: Executes a sequence of three distinct subroutines in succession.
- `search_list_for_value`: Iterates through a list of longwords pointed to by a0, comparing each to a constant and performing arithmetic on d7 to track progress until a match is found or the end of the list is reached.
- `process_data_block`: Performs a series of left bitwise shifts on a block of memory, propagating carry bits through the data.
- `extract_bitfield_data`: A twenty-three instruction routine invoked by a single caller.
- `write_table_values`: Executes sixteen instructions to process data.
- `set_hardware_constants`: Calls another routine and then writes two different 16-bit values to offsets 24 and 9e of a structure in a6.
- `update_status_byte`: Writes the value 127 to a4, checks a word from a memory location, and if it is not zero, clears bits 2 and 3 of d1 before writing the result to a4.
- `write_multiple_bytes`: Writes the bytes 255, 247, and 255 sequentially to the address pointed to by a4.
- `check_register_bit`: Checks bit 5 of the address in a5 and returns either 254 or 0 in d0 based on the result and a comparison with a constant.
- `modify_bit_flags_2`: Checks bit 4 of a byte at a5, calls another routine if the bit is set, and then writes a word and a byte to specific memory locations before returning.
- `modify_global_and_loop`: Checks a word at a global address, calls another routine, and then enters a loop that decrements d1 to perform repeated calls to another routine.
- `modify_bit_flags`: Sets and clears bits 1, 0, and 0 of a byte at the address in a4, sets d0 to 3000, and calls another routine before returning.
- `reset_hardware_bits`: Clears bits 0 and 1 of a byte at the address in a4, sets bit 0 of the same byte, loads 68440 into d0, and calls another routine before returning.
- `decrement_and_return`: Decrements d0 and returns immediately.
- `load_constant_and_branch`: Loads the value 16384 into d1 and performs a branch to another location.
- `configure_hardware_interrupts`: Performs forty-two instructions while accessing the interrupt request, disk length, and serial port interrupt clear registers.
- `modify_bit_and_table`: Builds a 16-bit word from the bits of one input byte, interleaving each bit with a bit taken from the previous value, and stores it in the buffer.
- `call_subroutine_twice`: Pushes d7 onto the stack, moves d6 to d1, calls another routine, restores d1 from the stack, calls another routine, and returns.
- `configure_hardware_and_sequence`: This forty-three instruction routine accesses CIAA, CIAB, and custom hardware registers while calling multiple small subroutines.
- `execute_sequence_of_calls`: Calls several different routines with various values in d1, and then enters a loop that calls another routine using d0 as a counter.
- `loop_decrement_and_call`: Moves 78 into d1, pushes d7 onto the stack, calls another routine, restores d7 from the stack, and loops using d7 as a counter until it reaches zero.
- `execute_hardware_sequence`: This forty-nine instruction routine accesses CIAA, CIAB, and custom hardware registers and executes several distinct subroutines.
- `clear_memory_range`: Enters a loop that clears bytes starting at a specific address for a length determined by the difference between two constants.
