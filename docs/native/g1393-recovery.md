# Game 1393 (combat air patrol (1992)(psygnosis)(m5)(disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01393). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 55 of 55 named (0 author symbols), 55 of 55 described (4 by rule, 50 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 196 of 196 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1336 of 1336 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 806302 |
| code | 4570 |
| data | 85521 |
| padding | 3191 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:Echo | 2 | 117 | 2 |
| Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:CAPINTRO | 3 | 1201 | 52 |
| Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |

## Cracktros (kept, not protection)

- Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:CAPINTRO

## Copy protection found

- none found

## Functions


### Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `transfer_and_branch`: Moves a longword from register d1 to d0 and then branches to another location.

### Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:CAPINTRO

- `initialize_intro_environment`: Initializes the display registers, retrieves a process handle, and allocates memory for the introduction sequence.
- `initialize_display_registers`: Initializes custom video registers and writes a set of fixed constants into global memory to set up the display.
- `init_intro_process`: Retrieves a process handle from the DOS library and, when it is valid, initializes a set of global structures, zeroes a block of custom video registers, and returns a flag indicating success.
- `process_linked_list`: Checks a byte at a specific address, calls another routine if it is non-zero, and then updates a table with values and a pointer derived from a register.
- `tiny_tst_rte`: Small routine of 2 instructions that tests (a7)+, then returns from the exception.
- `negate_and_invert_data`: Negates d6, loads a word from a constant into d0, performs a bitwise NOT on a byte at an offset in a table indexed by d3 using a1, negates d6 again, loads a word from a constant into d1, performs a bitwise NOT on a byte at an offset in a table indexed by d3 using a3, swaps the upper and lower bytes of d4, and loads a word from a constant into d0.
- `open_and_load_files`: Opens several files using DOS library calls, performs memory allocation and data manipulation involving bitwise operations and pointer arithmetic, and manages a loop that updates a table based on values read from a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_bitwise_or`: Performs a bitwise OR operation on register d0 with zero.
- `clear_d0_register`: Performs a logical OR operation on the least significant byte of register d0 with zero.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clear_data_register_zero_2`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `clears_data_register_zero`: Performs a bitwise OR operation on data register zero with zero.
- `clears_data_register_zero_2`: Performs a bitwise OR operation on data register zero using a zero constant.
- `clears_data_register_d0`: Performs a bitwise OR operation on data register d0 using a zero constant.
- `execute_scripted_sequence`: Executes a series of scripted operations by repeatedly setting a global callback pointer and a counter, then calling a dispatcher routine.
- `loop_until_global_variable_reaches_target`: Sets a counter to two and calls loop_until_global_reached, repeating the process until the loop finishes and then returns.
- `loop_and_decode_script_bytes`: Loop a fixed number of times, decoding a byte from a stream each iteration and returning early when a negative counter is detected.
- `loop_and_decode_script_bytes_in_steps`: Iterate a fixed number of times, decoding a byte from a stream on each pass and returning early when successive decremented counters go negative.
- `loop_compare_and_clear_word`: Executes a routine three times, calling a routine that compares a word at a specific offset to a register and clears it if the register is not greater, then returns.
- `decode_script_byte`: Reads a byte from a stream and dispatches based on its value, setting flags and calling helper routines.
- `shift_running_total`: Copies four longwords from one location to another, shifting them one position down the chain, and then stores the value from register a3 into the final location.
- `decode_compressed_stream`: Decodes a compressed byte stream, following a jump table to expand run-length markers, and returns the first non-zero byte found or a default value.
- `parse_variable_length_data`: Reads two bytes from a memory pointer, uses the first byte to calculate a count and an offset, and then performs a jump to a destination calculated from the second byte and the offset.
- `fill_memory_with_byte`: Fills a block of memory with the byte from d1 using nested loops, then performs a jump to an address calculated from a register.
- `copy_byte_blocks`: Copies forty bytes from a source at a3 to a destination at a0 using two nested loops, then returns.
- `invoke_script_data_processor`: Invoke a helper routine that manages counters, copies data from a global table, swaps two global words, and calls a routine through a pointer, then return a constant zero.
- `test_bit_and_process_script_data`: Test a bit in a global variable and, when set, clear that bit and call a helper routine that manages counters and copies data from a global table.
- `process_script_data`: Manages counters, copies data from a global table, swaps two global words, and invokes a routine through a pointer.
- `copy_audio_channel_state`: Reads interrupt enable bits and copies the corresponding audio channel length and pointer registers from the custom area into a global table.
- `initialize_structure_fields`: Initializes a structure pointed to by a register by clearing its first word, setting a following byte to a mask, and storing a value at a fixed offset.
- `update_audio_and_script_entry`: Updates the Amiga audio registers with values from a table and iterates through a script entry table, decrementing a counter in each entry and executing a jump to a location determined by a signed byte value when the counter reaches zero.
- `update_script_entry_data`: Calculates a new value by adding an offset from a table to a field in a structure, stores this value in the structure and a secondary structure, and loads two words from a pointer located at offset 46 of the primary structure into the secondary structure.
- `process_script_entry_loop`: Iterates through a table of entries, loading bytes from a source pointer into a structure and updating various fields including a counter and a calculated value.
- `update_callback_and_flag`: Stores the address of the calling routine into a global variable and writes the value fifteen into another global variable before returning.
- `set_script_handler_and_counter`: Stores a routine address into a global pointer and sets a 16-bit global value to fifteen.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `prepare_jump_target`: Loads a local address into a0, clears d5, and branches to another location.
- `update_global_values`: Loops 32 times, reading a word from a table and comparing its nibbles to a global word to adjust that global word by adding or subtracting values.
- `adjust_word_value_2`: Increments a 16-bit value in d2 by one, copies d1 to d3 and d2 to d4, and then compares the high nibbles of d3 and d4 to decide whether to subtract sixteen from d2 or branch.
- `adjust_word_value`: Adds sixteen to a 16-bit value in d2, clears d3 and d4, and then either subtracts two hundred fifty-six from d2 or branches based on a comparison of d3 and d4.
- `increment_and_store_words`: Adds two hundred fifty-six to a 16-bit value in d2, stores the result at the address in a1, increments a1 by four, adds the value in d5 to a0, and loops based on the d0 counter until it returns.
- `clear_table_words`: Clears each of twenty consecutive words in a table by writing zero to them in a loop.
- `store_offseted_values`: Stores a base value together with offsets of 8000, 16000, 24000, and 32000 into a table, writing both the high and low words of each result.
- `compare_and_clear_word`: Compares a word at a specific offset to a register and, if the register is not greater, clears that word.
- `wait_for_counter_zero`: Spins until a global counter reaches zero, waiting for a delay or event to elapse.
- `loop_until_global_reached`: Loops until a global variable reaches a value held in an unaffiliated register, then clears that global variable to zero.
- `infinite_loop_halt`: The routine executes an unconditional infinite loop, causing the processor to stall indefinitely.
- `check_flag_then_call_routine`: Checks a global variable for two conditions, and if both hold it sets that variable to one and invokes another routine through a pointer.
- `load_table_pointers`: Loads two longword addresses from a table into a2 and a0, then branches to another routine.
- `copy_table_words`: Copies ten consecutive four-byte words from one global table to another, repeating the block transfer until a counter reaches negative one.
- `reverse_copy_table`: Copies a large block of data from one table to another in reverse order by processing the data in chunks, swapping the beginning of the source with the end of the destination.

### Combat Air Patrol (1992)(Psygnosis)(M5)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
