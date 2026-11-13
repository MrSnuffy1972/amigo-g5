# Game 331 (seconds out (1988)(tynesoft)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00331). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (915392 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 259 of 259 named (0 author symbols), 259 of 259 described (23 by rule, 236 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2033 of 2033 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 33947 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3902 of 3902 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 497785 |
| code | 18622 |
| data | 109139 |
| padding | 7734 |
| unused | 282112 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Seconds Out (1988)(TyneSoft)[a2].adf:.prg | 1 | 3017 | 182 |
| Seconds Out (1988)(TyneSoft)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 875 | 76 |
| Seconds Out (1988)(TyneSoft)[a2].adf:WIN.PI1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Seconds Out (1988)(TyneSoft)[a2].adf:.tos (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- seconds out (1988)(tynesoft)[a2]/Seconds Out (1988)(TyneSoft)[a2].adf:crowd: high-entropy data file (7.60 bits/byte), classified as packed data
- 21 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- seconds out (1988)(tynesoft)[a2]/Seconds Out (1988)(TyneSoft)[a2].adf:WIN.PI1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- seconds out (1988)(tynesoft)[a2]/Seconds Out (1988)(TyneSoft)[a2].adf:.tos: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 call target
- 2 never-loaded region(s), 33947 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_dma_registers_and_set_audio` (.prg): Clears four 16-bit words at 16-byte intervals starting from an offset in the custom chip register block using a loop, sets the audio volume to 255, sets the audio frequency to 15, and calls another routine. Kind: DMA/Audio register reset and hardware initialization. Pass path: u0__sub_471C() continues execution.
- `clear_dma_and_call` (runtime segment an address): Clears three 8-byte blocks starting at an offset of 54 from the custom chip base address in a loop, sets the custom chip address and DMA control registers, and then calls another routine. Kind: disk hardware/sync check. Pass path: u2__sub_33B4() completes normally.

## Functions


### Seconds Out (1988)(TyneSoft)[a2].adf:.prg

- `initialize_stack_and_call_subroutines`: Copies the stack pointer to a global variable, calls reset_and_reinitialize_game_state, calls execute_initialization_sequence, and then returns.
- `copy_and_expand_data`: Copies 4000 words from a source to four different destination tables, then copies 8000 words from the first destination table to a new location pointed to by the stack.
- `process_linked_list`: Clears a global word, loads a pointer from a table using that word as an index, and if the pointer is not negative, loads four values from the structure at that pointer and calls another routine.
- `reset_and_reinitialize_game_state`: Clears several global variables and counters, performs multiple bitblock transfers to various memory locations, and executes a loop that updates color registers and data tables by iterating through specific memory addresses.
- `reset_game_state_and_graphics`: Clears numerous global words and counters, performs several bitblock transfers to specific memory locations, and executes a loop that updates state flags and counters based on the status of various global variables.
- `manage_state_cycle_and_tables`: Updates several global counters and state flags, manages a multi-step cycle by decrementing values from a table, and resets various global variables and counters when a specific threshold is reached.
- `execute_handler_jump`: Pushes a byte to the stack, tests a global byte, and if non-zero, jumps to another routine; otherwise, it clears a word at offset 01CA, calls two routines, and performs conditional jumps based on global word values.
- `branch_to_self`: Performs a short branch to itself.
- `update_color_registers`: Loops sixteen times to read a word from a table, shifts it left by one bit, and writes the result to both a hardware register and a memory location.
- `clear_color_registers`: Clears 16 words starting at the address in a1, which points to the hardware color registers, and returns.
- `update_status_bytes`: Shifts d1 right by eight bits, stores the original byte in d2, and performs several conditional checks and memory writes involving d1, d2, and various global variables.
- `compare_and_store_bytes`: Compares a word from a global variable with another global variable, and if the first is larger, shifts d1 right by eight bits and stores the resulting bytes into two different global variables.
- `test_byte`: Tests a byte at a specific memory location and then returns.
- `fetch_table_entry`: Uses bits from d0 to select a word from a table at offset 1980, stores that word in a global variable, and calls another routine.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (or.b) and then falls through into the code that follows, with no return of its own.
- `clears_data_register_zero`: Performs a bitwise OR operation on data register zero with zero.
- `select_and_store_table_entry`: Calculates an index by masking and shifting a value, fetches a longword from a table using that index, stores it in a global variable, and calls another routine.
- `increment_and_toggle_counters`: Increments a byte at a global address, sets a word at another global address to 65535, and if a third global word is not zero, increments a fourth global word and either calls a routine or clears that word and calls another routine based on its value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_blt_transfer_one`: Loads a pointer into a0, adds an offset to a pointer from a global variable in a1, and calls another routine.
- `prepare_blt_source_address`: Loads a pointer from a global variable, adds 2400 to it, and passes this modified address along with a pointer to another global variable and the value 70 to another routine.
- `copy_data_to_tables`: Copies words from a source table to two destination tables at a fixed address and a fixed address in a loop until a pointer reaches a fixed address.
- `copy_structured_data`: Copies words from a source table to two destination tables, performing specific word copies at offsets 8000, 16000, and 24000 within a loop that runs 14 times.
- `set_jump_parameter`: Stores 105 in a global word at offset 10048 of a structure in d4 and jumps to another routine.
- `manage_audio_and_state_logic`: Sets d4 to 102, d5 to 150, and stores 5 in a global long at offset 440 of a structure in a6, performs arithmetic on a global word and a global pointer, calls another routine, and if certain global conditions are met, updates several global words and calls another routine.
- `update_audio_state`: Sets d5 to 100 and d0 to 30, stores 2 in a global word, increments a global word, and if a value at an offset of a global pointer is not 65535, it sets d0 to 65535, d4 to 4, d3 to 0, and d2 to 0 before calling another routine; otherwise it calls another routine and returns.
- `decrement_and_conditional_branch_logic`: Decrements a global word and, if it reaches zero, calls another routine and proceeds to check a series of global flags and counters.
- `update_state_and_execute_callback`: Updates a global word by adding or subtracting a value based on range checks, manages several counters and state flags, and conditionally calls other routines or executes a function pointer retrieved from a table via a scaled index.
- `process_conditional_jump`: Loads d4 from a global pointer, loads d5 from another global pointer, adds a third global word to d5, clears that global word, loads d0 and d6 from a table, stores 2 in a global word, and jumps to another routine.
- `initialize_data_structure`: Loads a pointer from a table into a0, then writes several values to specific memory locations, including a word from a table offset 54 into a global variable.
- `initialize_data_structure_2`: Loads a pointer from a table into a0, then writes several values to specific memory locations, including the value 4 into a global variable and a word from a table offset 130 into another.
- `initialize_data_structure_3`: Loads a pointer from a table into a0, then writes several values to specific memory locations, including the value 2 into a global variable and a word from a table offset 128 into another.
- `set_control_values`: Clears a word in a global variable and writes three specific constant words to other global variables before returning.
- `write_multiple_values`: Writes the values 1, 13, 2, and 105 to four consecutive global word locations and returns.
- `set_offset_one`: Loads a pointer into a register and writes the value 1 to an offset of 30 from that pointer before returning.
- `set_offset_two`: Loads a pointer into a register and writes the value 2 to an offset of 30 from that pointer before returning.
- `set_offset_three`: Loads a pointer into a register and writes the value 3 to an offset of 30 from that pointer before returning.
- `reset_and_set_nine`: Clears two global word locations and writes the value 9 to a third global word location before returning.
- `reset_and_set_ten_one`: Clears one global word location, writes the value 10 to a second, and writes the value 1 to a third before returning.
- `set_one_nine_zero`: Writes the value 1 to a global word location, writes the value 9 to another, and clears a third before returning.
- `set_one_ten_one`: Writes the value 1 to a global word location, writes the value 10 to another, and writes the value 1 to a third before returning.
- `update_table_index`: Increments two global counters, resets one if it reaches two, and updates a table entry using a value from a second global counter.
- `calculate_offset_and_jump`: Calculates a value in d5 by shifting and adding to a word from offset 20FE, loads words from offsets E54 and E6E into d4 and d0, sets a global word at offset 1B8 to 4, and calls another routine before jumping to another location.
- `manage_game_state_logic`: Updates various global counters and state flags, manages a 6-step cycle by incrementing a counter and calling another routine, and performs conditional bitwise operations on structure members based on status bits.
- `update_control_flags`: Checks bit 3 of a global word and modifies a word at offset 0E54 of a structure in d0 by subtracting or adding 3 based on its value and other bit tests, then conditionally clears or sets words at offsets 0E6E and 2DCA.
- `manage_data_loop`: Loads d0 from a global word, clears d6, stores 4 in a global word, adds a global word to d5, and clears that global word; it then enters a loop that checks if two global words are zero and if a global word is less than 9, performing various stores and jumps based on these conditions, including loading a value from a table using a shifted d7 into d5.
- `loop_and_store_byte`: Writes a byte from register d0 to a global variable, calls another routine, and loops if the decremented value in d0 is greater than 44; otherwise it writes 255 to the global variable and stores 70000 in a global long word.
- `loop_and_store_long`: Writes 44 to a global variable, stores 68300 in a global long word, calls another routine, and loops if the incremented value in d0 is less than 244; otherwise it writes 255 to the global variable and stores 68300 in a global long word.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `load_and_update_data`: Calls a routine, then calls two other routines, stores two bytes derived from a long value into a table, and enters a loop that decrements a global word and calls another routine when it reaches zero.
- `update_color_palette_table`: Checks a global flag and, if zero, calls two routines before performing a loop that copies 16 words from a source table to a destination table, while left-shifting each word by 1 and writing it to the hardware color registers.
- `loop_pattern_copy`: Runs a loop five times, copying a byte from a table at offset d0 to a destination and calling another routine with the byte masked to 8 bits.
- `copy_pattern_bytes`: Copies bytes from a source table to two destination tables in a loop, using offsets to copy data at four different 8000-byte intervals.
- `execute_bitblock_transfers`: Calls a routine twice to perform bitblock transfers using the address in a0, the address in a1, and the value 30 in d0.
- `update_table_values`: Runs a loop four times that adds the value in d0 to a byte at an offset in a table, checks if the result is greater than 9, and either jumps to another routine or subtracts 10 from the byte and sets d0 to 1.
- `copy_data_blocks`: Calls another routine twice with different parameters and offsets in d6 and d5 to copy data from a table into memory.
- `loop_offset_copy`: Runs a loop that copies a byte from a table at offset d0 to a destination, subtracts 65 from the byte, and calls another routine if the byte is not zero.
- `copy_structured_data_3`: Calculates an offset based on d0 and copies 16-bit words from a source table to two destination tables using a loop.
- `copy_structured_blocks`: Copies several 16-bit words from a source table to two destination tables at offsets 0, 8000, 16000, and 24000, repeating this process based on values in d0 and d2.
- `set_processor_status`: Sets bit 7 and bit 6 of the status register and then returns.
- `move_byte_to_address`: Moves a byte from an offset of 4664 of the structure in a2 to a longword location.
- `process_data_segments_2`: Calculates and stores a pointer in a global variable, calculates and stores another pointer in a global variable, calls a routine, and then enters a nested loop that copies words between two tables at offsets of 8000, 16000, and 24000 while decrementing counters.
- `process_data_segments`: Runs a loop four times that calls another routine to process data from two tables, incrementing the table pointers by 7968 each iteration.
- `process_bit_shifts`: Runs a nested loop that performs bitwise shifts and logical operations on data from a source table and stores the results in a destination table.
- `execute_conditional_jump`: Uses d0 to index into a table to find a pointer in a2, then uses d6 to find a pointer in a1; if a1 is not a fixed address, it compares a value derived from d5 and d2 against a threshold and jumps to another routine if the condition is met.
- `calculate_parameter_bounds`: Checks a global word and, if it meets certain criteria, fetches values from a table and two other tables to perform arithmetic comparisons and updates several global variables and a global word.
- `update_state_and_call_subroutines`: Increments a global word, clears two global words, and performs arithmetic on a global word using values from a table in a0.
- `increment_and_call_routine`: Compares a global word to 9, increments it if it is less than 9, and if it is still less than 9, calls a routine before jumping to another location.
- `set_audio_parameters`: Writes 960 to a memory location, calls another routine, and then writes 1 to a global variable before returning.
- `copy_table_entries`: Runs a loop that copies 16-bit words from a source table, determined by an offset in d0, to a destination table.
- `check_and_clear_word`: Tests a word at a specific memory location; if it is negative, it jumps to another location, otherwise it clears the word and jumps to another location.
- `fill_pattern_blocks`: Clears or fills specific 16-bit word offsets in a table with either zero, a calculated value, or 65535, then calls another routine.
- `copy_memory_blocks`: Copies several longwords from the source at a0 to the destination at a1, swaps a1 and a2, and repeats this process in a loop controlled by d7 and d0 before returning.
- `fill_pattern_blocks_alt`: Clears or fills specific 16-bit word offsets in a table with either 65535, a calculated bit-shifted value, or zero, then calls another routine.
- `reset_global_parameters`: Clears several global words, sets a global word to 40, and copies a word from an offset of a table into a global variable.
- `update_game_state_and_audio_parameters`: Checks a global flag and, if set, updates a score or counter by adding a value from a pointer, resets several global words, increments a counter, and fetches a new value from a table.
- `decrement_counters_and_jump`: Decrements three different byte and word counters in a table and performs conditional jumps or writes specific values to those counters based on whether they are negative.
- `loop_pattern_copy_short`: Runs a loop four times, copying a byte from a table at offset d0 to a destination and calling another routine with the byte masked to 8 bits.
- `execute_table_sequence`: Calls another routine, sets a global word to 1, clears two global words, uses a global word to index a table and stores a value in a global word, calls another routine, clears a global word, and enters a loop that increments a global word and calls another routine if it is less than 5, eventually performing a long decrement loop.
- `update_structured_tables`: Sets a base address in d7, then runs nested loops that call another routine to process data and eventually writes 65535 to several specific memory locations.
- `bitwise_and_tables`: Performs a bitwise AND operation between words at specific offsets in two tables and repeats this process sixteen times in a loop.
- `clear_global_variables`: Clears several 16-bit global variables and sets two specific global variables to 110 and 102 respectively.
- `clear_and_set_words`: Clears a 16-bit word at a global address and sets a 16-bit word at the following address to one.
- `update_global_counter_loop`: Clears a global word and, if not zero, calls a routine, adds a value from d0 to a global word, and enters a loop that either calls a routine or resets a global word and jumps back to the start of this routine.
- `copy_table_data_2`: Loads a pointer from a global variable into a0 and jumps to the address stored at that pointer.
- `load_address_to_a0`: Loads a specific address into register a0.
- `copy_table_data`: Copies several 32-bit words and 16-bit words from a source table to a destination table, repeating the process twenty-eight times in a loop.
- `update_counter_and_table`: Increments a global word, uses the result to fetch a word from a table via a scaled index in d0, and writes it to another global variable; if the counter reaches 4, it clears a global word and writes two other values before returning, otherwise it returns.
- `update_game_state_parameters`: Checks several global conditions and range constraints, then updates a global word with a value from a table, modifies a global counter, and conditionally calls other routines or sets specific global values based on the state of various memory locations and structure offsets.
- `clear_global_word`: Clears a 16-bit word at a global address and then returns.
- `select_parameter_offset`: Checks several global conditions and compares a value from a table against specific constants to determine a new value for register d0 and a secondary value for register d7.
- `load_parameter_values`: Loads a pointer into a0, checks a word at an offset of 110 from that pointer, and branches based on the result.
- `evaluate_parameter_range`: Calculates a range using a table and a global word, then compares it against a value in d1.
- `update_counters_and_tables`: Increments a global word, clears a global word, sets two global words to 65535, subtracts a value from a table in a0 from a global word, calls another routine, and if a global word is greater than 8, calls two routines; it also checks if a global word is 18 to call another routine and performs a table lookup via a global pointer.
- `decrement_and_call_routine`: Checks a global word and, if it is not zero, decrements it and calls a routine.
- `initialize_audio_state`: Checks a word at a specific location; if it is not zero, it writes -1 to that location, writes 768 to a memory location, calls another routine, and writes 1 to a global variable.
- `cycle_table_entries`: Checks a global word, and if zero, loads d6 from a global word, sets d0 to 21, loads d4 and d5 from a table using an index from a global word, calls another routine, increments that index, and if it reaches 12, clears it; it also handles a case where it clears several global words and calls another routine.
- `clear_conditional_globals`: Tests a word in a global variable; if it is not zero, it returns, otherwise it clears three global words and sets the first one to 65535 before returning.
- `update_state_and_counters`: Manages a series of state counters and global variables by incrementing or resetting them based on specific conditions, updating values from a table, and calling other routines to modify global memory.
- `toggle_state_and_decrement`: Sets a global word to 5, decrements a global word if it is greater than 3, and then checks a bit in a value derived from a global variable to either toggle a bit in a global word or jump to a routine via a pointer.
- `increment_global_counters`: Increments a global word, sets a global word to 65535, and jumps to another routine.
- `copy_structured_data_2`: Copies data from one memory area to another using nested loops, updating pointers in a5 and a6 by 36 decimal after each inner loop iteration.
- `loop_structured_copy`: Calls another routine, then enters a loop that calculates an offset, calls another routine, and increments d7 until it reaches 99.
- `update_loop_parameters`: Calls several routines to initialize registers, calculates an offset for a table in a6 using a loop based on d7 and d6, clears a global word, and then calls another routine before potentially performing more register operations and returning.
- `decrement_and_loop`: Decrements register d0 in a loop until it reaches zero and then returns.
- `load_address_and_call`: Loads a pointer into a6, calls another routine, and then loads a different pointer into a6.
- `clear_memory_blocks`: Clears four blocks of memory by looping through a nested structure where each block is cleared using a 16-bit word increment.
- `setup_jump_tables`: Stores the value 70000 in a global word, calls two routines, stores 78000 in the same global word, calls two routines, and then performs a conditional jump via a pointer.
- `setup_conditional_jump_table`: Sets d0 to 27, clears d6, and stores 2 in a global long at offset 440 of a structure in a6, then jumps to another routine.
- `setup_conditional_jump_table_alt`: Sets d0 to 28, clears d6, and stores 2 in a global long at offset 440 of a structure in a6, then jumps to another routine.
- `copy_multiple_tables`: Updates several global bytes, calls a routine, sets up three different source and destination pairs to call a routine with, and finally performs a loop that copies words from a source table to a destination table.
- `execute_multiple_jumps`: Calls another routine, clears d6, sets d0 to 31, d4 to 105, and d5 to 161, stores 2 in a global long at offset 440 of a structure in a6, then calls another routine; it then loads a word from a global pointer into d6, sets d0 to 32, d4 to 202, and d5 to 160, stores 5 in a global long at offset 440 of a structure in a6, and jumps to another routine.
- `clear_multiple_tables`: Sets a global long value, then enters a nested loop that clears 8 words at a time across four different tables, repeating this process 21 times.
- `clear_and_call_handler`: Calls a routine via a pointer, clears register d0, calls another routine, and clears two global words.
- `process_audio_trigger`: Loads a word from offset 14 into d0, calls another routine, clears a word at offset 68A, and if a byte at offset 45DC is non-zero, calls another routine.
- `process_audio_table_loop`: Loads a word from a table into d1, scales it, and uses it to index into a table at a0 to load a value into a global variable; if the value is not 65535, it loops, otherwise it clears a global word, sets d0 to 65535, calls another routine, and returns.
- `fetch_and_store_table_data`: Calculates an index by shifting and masking d0, fetches a long word from a table using that index, performs an XOR on d0, fetches another long word from the table, and stores the results in various global locations before checking a global word and potentially returning.
- `clear_memory_block`: Loads a pointer from a global variable and enters a loop that clears 8 consecutive long words, repeating this process 1000 times.
- `process_bitmask_table`: Performs bitwise operations on data from a source table to update a destination table using a mask derived from the input, then increments a global counter and loops based on d6 and a global word.
- `clear_and_jump`: Clears a 16-bit word in a global variable and jumps to the address stored at another global variable.
- `copy_scattered_bytes`: Copies five bytes from a source address calculated from a global variable and d0 to a destination address in a1, specifically targeting offsets of zero, 8000, 16000, and 24000 within a loop of six iterations.
- `load_pointers`: Loads a long value from a memory address into a6, moves it to a4, and loads another long value into a5.
- `dispatch_scattered_data`: Reads a byte from a table, compares it against two constants, and if neither matches, calls a routine with a pointer in a1 before jumping to a new address.
- `jump_with_offset_a4`: Copies d6 to a6 and jumps to an address stored in a pointer, or alternatively adds 280 to a4, copies a4 to a6, and jumps to an address stored in a pointer.
- `load_pointer_registers`: Loads a global pointer into a register and copies that pointer to another register, then loads a second global address into a third register.
- `dispatch_byte_command`: Loads a byte from a pointer in a5, and based on its value, either jumps to another routine, performs an arithmetic operation on the byte and jumps, or branches to different locations.
- `execute_jump_routine`: Sets a1 to the value in a6, calls a routine, and then jumps to an address calculated by adding one to d6.
- `jump_with_offset_a4_large`: Copies d6 to a6 and jumps to an address stored in a pointer, or alternatively adds 320 to a4, copies a4 to a6, and jumps to an address stored in a pointer.
- `compare_byte_sequences`: Compares up to six bytes from a table at a0 with bytes from a destination at a1, then either clears or sets a 16-bit value in a global variable based on the comparison result.
- `tiny_move`: Small routine of 1 instruction that copies 7 to d7.
- `update_byte_sequence`: Calculates an offset into a table based on a value in d7, calls another routine, and then enters a loop that copies five bytes from a source to a destination at specific intervals until d7 reaches zero, or otherwise copies five bytes from a source to a destination and returns.
- `check_threshold_conditions`: Compares a global word against several constant values and performs range checks using a global word and a constant.
- `manage_index_pointer`: Checks a word at a global address; if non-zero, it branches elsewhere; otherwise, it increments a global word, loads a value from a table into d1 using a value from that table, and either clears a global word or updates a global word and clears another global word based on the sign of d1.
- `handle_indexed_table_sequence`: Loads a word from a global pointer into d0; if it is 5 or 6, it updates the global pointer and returns; otherwise, it decrements d0, uses it to index into a table to load d4 and d5, sets d0 to 34, sets d6 to the global pointer, increments the global pointer, stores 2 in a global long at offset 440 of a structure in a6, and jumps to another routine.
- `clear_and_load_hardware_data`: Clears a long at a memory address, sets a global pointer, and loads bytes from a long into two global locations; it then sets d0 to 40, clears d6, sets d4 to 95, and d5 to 80, stores 2 in a global long at offset 440 of a structure in a6, and calls another routine before performing a loop that checks a bit in a global variable and calls another routine.
- `clear_and_update_tables`: Calls another routine, clears several words at offsets 0, 8000, 16000, and 24000 from two different memory areas using a loop, then enters a second loop that updates a global word and calls another routine based on bitwise checks.
- `clear_and_copy_blocks`: Clears several longwords at specific offsets within a structure pointed to by a global pointer, then enters a nested loop that copies words from a source table to a destination table, repeating for a set number of iterations defined by d0 and d1.
- `update_indexed_structure_values`: Sets d0 to 35, d4 to 169, and d5 to 129, stores 2 in a global long at offset 440 of a structure in a6, and calls another routine; it then calculates an index from a global word, loads a value from a table into d4, and if the value is negative, clears several globals and performs two more jumps to another routine.
- `setup_blt_transfer`: Loads a pointer into a0, loads and offsets a pointer in a1, sets d0 to 30 decimal, and calls a routine.
- `set_d6_constant`: Loads the word value 31999 into register d6.
- `copy_string_with_limit`: Copies bytes from a source pointer in a0 to a destination pointer in a1, decrementing a counter in d6, and if a null byte is encountered, it enters a loop that clears bytes and decrements a counter in d0 before calling another routine or jumping back to itself.
- `filter_and_store_words`: Iterates through a list of words in memory until a terminator is found, and if a word matches a specific bitmask, stores a corresponding word from another location into a table at an offset determined by the lower nibble of the matched word.
- `open_libraries_sequentially`: Calls a function via a pointer in a6 twice, storing the result in a global variable if the return value is non-zero, using a value from a1 as an argument.
- `execute_initialization_sequence`: Calls several routines in sequence, and if the first call does not set the zero flag, it calls three more routines, stores a long value from a global variable into another global variable, and calls three additional routines before returning.
- `setup_interrupt_vectors`: Stores three global pointers into specific memory locations, clears a bit in a custom chip register, sets several custom chip registers, and copies nine words from a source table to a destination table using a loop.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `check_and_clear_long`: Reads a byte from a global variable into d0, and if that byte is not zero, clears a long value at another global variable location before returning.
- `read_keyboard_and_update`: Saves registers, reads a byte from the keyboard serial register, calls a routine, writes 76 decimal to the keyboard serial register, restores registers, and calls another routine.
- `update_hardware_interrupts`: Sets a bit in a hardware register, decrements a long global variable, and if that variable is not positive or a second global variable is not zero, sets two specific hardware registers; finally, it restores registers and returns from an interrupt.
- `update_display_bitplanes`: Checks bit 5 of the Amiga custom chip status register; if set, it increments a global counter, calculates four subsequent bitplane pointer addresses by adding 8000 to a base value, stores these pointers in a structure, and calls three separate routines.
- `tiny_or`: Small routine of 1 instruction that merges d0 into d0.
- `conditional_return`: Clears a word in a global variable and, if that word is not zero, otherwise it returns.
- `set_video_registers`: Writes specific values to five hardware registers and stores a long value into a global variable.
- `store_register_values`: Stores a word from d0 into a table at an offset determined by the lower nibble of d1, and also stores the value of d0 shifted left by one into a hardware register at an offset determined by d1.
- `wait_for_bit`: Tests bit 6 of a byte at an offset of 2 from a pointer in a6 and loops until that bit is cleared.
- `process_joystick_input`: Reads a byte from a CIAA register and a word from a hardware register to set various bits in d1, then stores d1 at a location pointed to by a value in d0.
- `update_table_byte`: Rotates d0, performs an XOR, and uses the result to either move a byte from a table to a global address or move a byte from one table location to another based on a bit test.
- `clear_table_entry`: Clears a long global variable, masks d0 with a bitmask, and clears a byte in a table at an offset determined by the masked d0 value.
- `save_and_call_subroutines`: Stores four values from registers a6, a0, d1, and d0 into global variables, calls a subroutine, and if the result in d0 is non-zero, calls two more subroutines before returning.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `clear_dma_registers_and_set_audio`: Clears four 16-bit words at 16-byte intervals starting from an offset in the custom chip register block using a loop, sets the audio volume to 255, sets the audio frequency to 15, and calls another routine.
- `store_and_call_sequences`: Loads several values from memory, stores them into a destination table at a6, and calls a routine five times, updating the values in d7 and d1 between calls.
- `execute_conditional_callback`: Performs arithmetic on d1 and d2, updates several global words based on a comparison with a value at a global offset, calls a routine, and then calls a function pointer retrieved from a table.
- `setup_blt_parameters`: Initializes several hardware registers for a bitblock transfer, including color and size registers, and then enters a loop that performs the transfer for a specified number of iterations using values from a0 and a1.
- `configure_blt_registers`: Calls a subroutine, increments two pointers by 8000, and stores these pointers, a constant, and a value from d3 into specific offsets of a structure pointed to by a6.
- `update_structure_fields`: Stores several values from registers and a global variable into a structure pointed to by a6, then calculates a return value using bit shifts and logical OR on d3.
- `set_structure_constants`: Stores specific constant values and register contents into a structure pointed to by a6, then calculates a value in d3 using a left shift and logical OR with d2 before returning.
- `initialize_complex_structure`: Calls three different routines, performs bitwise operations on d3 and d6, and stores the results into a structure at a6 at offsets 66 and 64, then calls three more routines.
- `initialize_structure_pointers`: Calls another routine, adds 8000 to a0 and a1, stores a1 at offset 76 of a structure in a6, stores an address at offset 80 of the structure, stores a0 at offset 84 and offset 72 of the structure, then calls another routine and returns.
- `update_structure_pointers_2`: Clears a global word, sets a0 to an address decimal, stores d3 at offset 88 of a structure in a6, calls a routine, increments a global counter, and if the counter does not match a global word, updates pointers in a4 and a5 before jumping to a new address.
- `update_structure_pointers`: Copies the contents of a4 and a5 into offsets 76 and 72 of a structure in a6, then sets d7 to the value of d5 and returns.
- `update_structure_data`: Calls several subroutines, stores a word from d0 into offset 66 of a structure in a6, stores a word from d0 with bitwise ORed value 4042 into offset 64 of a6, and stores a word from d3 into offset 88 of a6 before calling another routine three times.
- `initialize_structure_values`: Sets several fields in a structure pointed to by a6, including three word values from d1, three word values from a constant, and several cleared words, then calls another routine and updates three fields in the structure before returning.
- `calculate_address_offset`: Calculates an address by performing arithmetic on d1 and d0, then adds a value from a global variable to that address and returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (or.b) and then falls through into the code that follows, with no return of its own.
- `update_global_counter`: Checks a global word, then either adds or subtracts an address from a global long based on its sign and value, and writes the result to a hardware register.
- `configure_audio_hardware`: Checks d0 and a global word, then writes several values to hardware registers and uses values from tables to update audio parameters and a global word before returning.
- `tiny_subi`: Small routine of 1 instruction that subtracts 0 from d0.
- `modify_d0_and_memory`: Subtracts 64 from register d0, performs a bitwise AND on d0 with 64, performs a bitwise AND on the byte at the address pointed to by a0 with 64, and subtracts 64 from d0.

### Seconds Out (1988)(TyneSoft)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_table_entry`: Jumps to an address at a fixed address.l and then returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_blocks_loop`: Loops up to six times, reading a byte from a table at a0, masking it, and calling a subroutine with a pointer in a4.
- `copy_data_blocks_2`: Copies bytes from a source table to two destination tables in a loop that runs seven times, incrementing the pointers by 40 decimal each iteration.
- `perform_bitblock_transfers`: Calls another routine twice, each time using different source and destination addresses and a constant value in d0.
- `increment_table_byte`: Sets d1 to 4, adds the value of d0 to a byte at an offset of 4 from a global address, and then branches to an earlier instruction if the byte is not greater than 9, otherwise it jumps to another routine.
- `decrement_and_jump`: Subtracts 10 from a byte at a pointer in a0, sets d0 to 1, decrements d1, and then either branches to a previous instruction or jumps to another routine.
- `copy_nested_data_blocks`: Performs a triple-nested loop that copies words between memory locations, updates several global pointers and counters, and calls another routine.
- `process_bit_streams_loop`: Loops four times, copying a word from a global variable to a new location, calling a subroutine, and incrementing two pointers by 4960 decimal each iteration.
- `process_bit_streams`: Performs nested loops to shift bits from a source word into a destination word, updating several global counters and pointers.
- `execute_table_entry`: Calculates addresses from a table using d0 and d6, compares a value at a calculated address to a constant, and if not equal, performs arithmetic to determine a jump target and calls another routine.
- `copy_table_entries_2`: Copies 134 words from a table located via a pointer in a global table to another global table, using a loop.
- `branch_on_sign`: Tests a longword at a specific address and either branches to another routine or jumps to a different routine based on the sign bit.
- `clear_and_fill_memory_blocks`: Clears a global word, then loops to clear blocks of memory at offsets from a pointer, optionally fills blocks with a value derived from a bit shift, and calls a subroutine.
- `swap_and_copy_blocks`: Copies several longwords from a source to a destination, exchanges two registers, and repeats this process in a loop three times.
- `branch_on_sign_and_skip`: Tests a longword at a specific address and either branches to another routine or performs a branch to skip the next instruction.
- `fill_and_clear_memory_blocks`: Clears a global word, then performs a loop to fill memory blocks with 65535, followed by conditional logic to fill blocks with 32768 or 0, and finally calls a subroutine.
- `initialize_global_values`: Clears several global words and sets specific global words to constant values or values loaded from a table.
- `decrement_and_jump_2`: Decrements a byte in a global variable and jumps to another address if the result is not negative, otherwise it returns.
- `decrement_global_byte`: Writes 9 to a global byte, decrements another global byte, and then branches to an earlier instruction if the result is not negative, otherwise it jumps to another routine.
- `decrement_and_jump_byte`: Stores 5 in a byte at a specific address, decrements a byte at another address, and then either branches to a previous instruction or jumps to another routine.
- `copy_data_blocks_3`: Sets a global word to all ones, clears two global words, and then enters a loop that runs four times, copying data from a table to two destination tables using a subroutine.
- `reset_parameters_and_copy`: Sets three global words to specific values, increments a fourth global word, and then copies a word from a table to another global variable based on a shifted version of that word.
- `reset_global_parameters_2`: Clears one global word, sets another to 101, and sets a third to 2.
- `clear_global_word_2`: Clears a 16-bit global variable and returns.
- `manage_counters_and_jump`: Increments a global word and performs conditional checks to clear or modify other global words, then loads several values into d4, d5, d6, and d0 before jumping to another routine.
- `increment_and_jump`: Increments a global word by 1, sets another global word to 65535, and jumps to another routine.
- `copy_memory_blocks_2`: Performs a nested loop to copy five 16-bit words from one memory location to another, incrementing the source and destination pointers by 36 bytes after each inner loop.
- `copy_structured_data_4`: Copies six bytes from a source address calculated from a global variable and a byte in d0 to a destination address in a1, repeating this process six times.
- `dispatch_table_command`: Loads a byte from a table at a location pointed to by a register, compares it against two constants, and if neither matches, subtracts 41 from it, calls another routine, and jumps to an address stored in a register plus one.
- `jump_via_register`: Moves the value of a register to a pointer and jumps to the address contained in that pointer.
- `jump_via_offset_pointer`: Adds a constant to a register and then jumps to the address stored in that register.
- `modify_and_jump_offset`: Loads a byte from a table at a location pointed to by a register, compares it against several constants, and if it is less than 10, adds 28 to it, otherwise it jumps to a specific address.
- `copy_and_jump`: Subtracts 41 from a byte in d0, copies six bytes from a source address calculated from a global variable and d0 to a destination in a1, and then jumps to an address stored in a6 plus one.
- `jump_via_register_direct`: Moves the value of a register to a pointer and jumps to the address contained in that pointer.
- `jump_via_offset_pointer_alt`: Adds a constant to a register and then jumps to the address stored in that register.
- `compare_table_bytes`: Compares bytes from two different locations in a loop that runs up to five times, and clears a global word if the loop completes.
- `set_global_word`: Sets a 16-bit value in a global variable to all ones and returns.
- `copy_bytes_loop`: Calculates an offset for a destination address based on a value in d7, calls another routine, and then enters a loop that copies five bytes from the destination to an address offset by 11 if a global word is non-zero.
- `copy_five_bytes`: Copies five consecutive bytes from the address in a0 to the address in a1 plus an offset of 11.
- `update_table_index_2`: Checks a global pointer, and if not zero, loads a word from a table using an index from a global variable, increments that index, and if the loaded word is not negative, clears a global variable and updates a pointer.
- `execute_conditional_table_logic`: Compares a global word against two constants, and if neither matches, decrements a global word, calculates a table index, loads two words from that table, and then calls another routine via a jump.
- `increment_global_word`: Increments a 16-bit value in a global variable by one and returns.
- `clear_global_word_3`: Clears a 16-bit value in a global variable and returns.
- `process_table_entries`: Calls another routine to calculate an address, then uses a value from a global variable to index into a table, performs arithmetic on that index, and stores the result back in a global variable before calling another routine three times with different parameters.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `open_libraries`: Attempts to open two libraries using a library function; if successful, it stores the resulting pointers in two different global variables.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (movea.w) and then falls through into the code that follows, with no return of its own.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `check_and_clear_byte`: Checks a byte in a global variable; if it is non-zero, it clears a 32-bit global variable and returns.
- `read_keyboard_and_reset`: Reads a byte from the keyboard serial register into d0, calls another routine, writes 4c to the keyboard serial register, and then calls another routine before returning.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `push_global_long`: Pushes a 32-bit value from a global variable onto the stack and returns.
- `wait_for_bit_2`: Enters a loop that repeatedly tests bit 14 of a byte at an offset of 2 from the address in a6 until that bit is cleared.
- `update_joystick_status`: Reads the CIAA port and custom register to check specific bits, then updates a global variable with a bitmask based on the joystick direction and button state.
- `modify_and_store_byte`: Performs bitwise operations on a register, stores the result in a global byte, and then either stores a byte from a table into another global byte or stores a zero into a table location.
- `clear_table_entry_2`: Clears a long global variable, masks d0 with bit 7, and clears a byte in a table at an offset determined by the masked d0 value, then returns.
- `store_and_call_subroutines`: Stores a6, a0, d1, and d0 into global variables, calls a subroutine, and then calls two other subroutines based on the value of d0 before restoring a6 and clearing d0.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `clear_dma_and_call`: Clears three 8-byte blocks starting at an offset of 54 from the custom chip base address in a loop, sets the custom chip address and DMA control registers, and then calls another routine.
- `process_data_and_execute`: Copies several blocks of data from a table to a new location while calling a subroutine, then performs arithmetic and bitwise operations to calculate a value and calls a subroutine via a pointer.
- `setup_blt_and_loop`: Configures several hardware registers for a bitblock transfer, then enters a loop that runs three times to perform the transfer using addresses from two registers.
- `update_structure_pointers_3`: Calls another routine, then stores three calculated longwords and one word into a structure pointed to by a6 at offsets 4c, 50, 54, 48, and 58.
- `initialize_structure_data`: Writes several values from a pointer in d0 and registers a1, d1, and a0 into a structure at a6, then returns a value calculated from a word in d0.
- `set_structure_fields`: Writes constant values and register contents into a structure at a6, then returns a value derived from d1 and d2.
- `initialize_and_update_structure`: Saves registers to the stack, calls four different routines, performs bitwise operations on d3, and stores the results into a structure at offsets 64 and 66 before calling another routine three times and returning.
- `setup_structure_pointers`: Calls another routine, then stores three calculated addresses into a structure at offsets 48, 54, and 4c, and stores the constant an address at offset 50, before calling another routine and returning.
- `update_structure_loop`: Clears a word in a global variable, then enters a loop that stores a word from d3 into a structure at offset 88 of a6, calls another routine, increments a global counter, and updates two longwords in the structure at offsets 4c and 50 using values from a4 and a5.
- `clear_and_loop`: Clears a longword at a specific address, stores a6 at offset 4c and a5 at offset 50 of a structure, then enters a loop that decrements d7 using d5 until it reaches zero.
- `initialize_structure_fields`: Calls several routines to initialize data, then writes a word from d0 to offset 66 of a structure, writes a word derived from d0 with bitwise ORed with 4042 to offset 64, writes d3 to offset 88, and calls another routine three times.
- `initialize_structure_data_2`: Sets multiple values in a structure at offsets 72 through 102, performs bitwise operations on d3 and d2, calls another routine, and then updates several structure offsets with new values.
- `calculate_address_offset_2`: Performs arithmetic on d1 and d0 to calculate a value in a0, using bitwise operations and shifts.
