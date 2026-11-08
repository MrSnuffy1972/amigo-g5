# Game 135 (3D World Tennis): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00135). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 74 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 211 of 211 named (0 author symbols), 211 of 211 described (13 by rule, 194 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 455 of 455 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4473 of 4505 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 233168 |
| code | 16474 |
| data | 569989 |
| padding | 16465 |
| unused | 65024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| test | 1 | 9 | 1 |
| EndIf | 1 | 2 | 1 |
| if | 1 | 239 | 5 |
| 3D World Tennis.adf:C.D.I. | 2 | 267 | 8 |
| 3D World Tennis.adf:test | 1 | 49 | 1 |
| tennis | 3 | 3690 | 189 |
| 3D World Tennis.adf:if | 1 | 239 | 5 |
| 3D World Tennis.adf:bootblock | 1 | 10 | 1 |

## Cases handled

- 3D World Tennis/3D World Tennis.adf:gfx/intrbase: high-entropy data file (7.70 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/cityphilad: high-entropy data file (7.74 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/allenicons: high-entropy data file (7.62 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/strptourna: high-entropy data file (7.62 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/gameover: high-entropy data file (7.72 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citymoskow: high-entropy data file (7.69 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citynewyor: high-entropy data file (7.71 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/strpoffice: high-entropy data file (7.61 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/selemain: high-entropy data file (7.71 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/intrsimul: high-entropy data file (7.61 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/seleicons: high-entropy data file (7.71 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citymelbou: high-entropy data file (7.70 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citysydney: high-entropy data file (7.74 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/cityrome: high-entropy data file (7.71 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/selefaces: high-entropy data file (7.78 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/tabletourn: high-entropy data file (7.80 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/wmapicons: high-entropy data file (7.61 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/endparta: high-entropy data file (7.69 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/seleallena: high-entropy data file (7.75 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/allen: high-entropy data file (7.73 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/intrlogo: high-entropy data file (7.67 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/winparta: high-entropy data file (7.73 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/winpartb: high-entropy data file (7.63 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citytokyo: high-entropy data file (7.61 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citylondon: high-entropy data file (7.74 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citystockh: high-entropy data file (7.73 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/intrflags: high-entropy data file (7.65 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/citymonaco: high-entropy data file (7.74 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:gfx/wmapbase: high-entropy data file (7.72 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/snboxe: high-entropy data file (7.74 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/terz min: high-entropy data file (7.70 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/single guitar: high-entropy data file (7.69 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/public: high-entropy data file (7.73 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/popped sc: high-entropy data file (7.86 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/terza min 2: high-entropy data file (7.78 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/key: high-entropy data file (7.72 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/tappeto: high-entropy data file (7.79 bits/byte), classified as packed data
- 3D World Tennis/3D World Tennis.adf:snd/tastieraaaaa: high-entropy data file (7.63 bits/byte), classified as packed data
- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 3D World Tennis/3D World Tennis.adf:C.D.I.: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `evaluate_script_condition` (if): Checks for the existence of a required library and then parses a script string to evaluate a conditional expression involving comparison operators such as equal, greater than, or less than. Kind: manual/code-wheel/colour-card lookup. Pass path: iVar3 >= 0 (specifically the branch leading to u2__l0_0174 or the logic where iVar3 is evaluated against 0/1).
- `parse_script_conditional_logic` (if): Checks for the existence of a required library and, if present, parses a script string to evaluate conditional expressions using comparison operators such as equal, greater than, or greater than or equal to. Kind: checksum or integrity trap. Pass path: u6__l0_0174.

## Functions


### test

- `check_available_memory`: Calls a library function to check available memory and returns 5 if the result is less than an address, otherwise it returns 0.

### EndIf

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### if

- `evaluate_script_condition`: Checks for the existence of a required library and then parses a script string to evaluate a conditional expression involving comparison operators such as equal, greater than, or less than.
- `parse_script_commands`: Parses a string to identify command keywords such as IF, ELSE, ENDIF, EQ, GT, GE, and others, while performing numeric conversions and managing a string pointer.
- `get_byte_conditionally`: If d6 equals d5 and a routine call returns a value less than 1, it returns 255; otherwise, it returns a byte from a table at offset d5 of a5.
- `update_structure_field`: Stores the value from d5 into a structure at offset 16 of a table indexed by d7, calls a library function with a pointer from offset 36 of the same structure, and then stores the returned value into offset 20 of the structure and clears offset 16.
- `manage_string_pointer`: Checks if a pointer in a2 points to a string starting with a specific character, calls a library function to advance the pointer, and if the result is zero, calls two other library functions.

### 3D World Tennis.adf:C.D.I.

- `parse_amiga_hunk_data`: Iterates through a data structure containing AmigaDOS hunk types, performing memory allocation and copying data blocks while applying relocations.
- `process_list_and_blit`: Decrements a counter in d1 and, if non-zero, performs calculations involving a pointer in a1 and a library call, eventually writing a value to a global variable and calling another library function.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `extract_bits_and_copy_2`: Uses bitwise shifts and a loop to extract data from a source and write it to a destination, while checking a flag and comparing pointers.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_and_copy`: Performs bitwise shifts and rotations on d5 using d4, then enters a loop that processes bytes from a table and writes them to a destination pointed to by a3.
- `execute_trap_instruction`: Moves a word from a table at offset 176 of a2 into d4, performs an OR operation on d1 with the constant 248, calls a trap, and then performs an OR operation on d1 with d0.

### 3D World Tennis.adf:test

- `initialize_system_resources`: Checks available memory, calls library functions to load two pointers into global variables, modifies a byte at an offset within a structure, and enters a loop that calls library functions three times while incrementing a global byte.

### tennis

- `initialize_hardware_and_tasks`: Sets hardware register values, calls several routines to initialize structures and lists, and executes functions from a table using pointers retrieved from a task structure.
- `clear_registers`: Clears all eight data registers and all eight address registers by setting them to zero.
- `modify_task_data`: Retrieves a task pointer and updates a specific offset within that task with a new value, specifically setting the value at offset 184 to an address.
- `restore_task_data`: Retrieves a task pointer and restores a previously saved value to a specific offset within that task using a value stored at offset 288 of the program.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `setup_module_state`: Clears a byte at a global address, calls another routine, sets a global word to 1, calls another routine, and if a global longword is non-zero, populates three global longwords before calling another routine.
- `update_list_pointers`: Iterates through a list of structures, calls a library function using values from offsets 4 and 8 of each structure, and if the return value is not 0, stores it at offset 0 and moves to the next structure.
- `accumulate_list_values`: Iterates through a list of structures, adds the value at offset 8 to the value at offset 4, and stores the result at offset 0 of the current structure.
- `free_list_memory`: Iterates through a list of structures and calls a library function to free memory using the pointer at offset 0 and the size at offset 4 of each structure.
- `initialize_structures`: Copies a pointer from a global variable to a destination address, then clears a 32-bit value and sets three 16-bit values at offsets 8, 10, and 12 of that destination.
- `execute_function_table`: Checks a 16-bit value in a global variable; if it is not zero, it decrements the value, multiplies it by 6, and calls a subroutine from a table using the result as an index.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `initialize_system_state`: Writes zero to two custom chip registers, calls several routines with specific parameters, and returns if a value from a global variable is not negative.
- `initialize_hardware_and_input`: Writes to custom chip registers, calls several routines to process data and update input states, and returns if a global word is less than zero.
- `initialize_system_and_parameters`: Calls several routines to initialize data, sets hardware registers, and performs a loop that continues as long as a global word is less than zero.
- `initialize_game_state`: Calls several routines with specific parameters, writes three words to a global structure, and calculates a value from a routine return to update two global words.
- `adjust_global_counters`: Sets a global word to negative one and conditionally modifies a global word at offset 3B0A based on the value of d0, then potentially updates a global word at offset 3B08.
- `decrement_and_call_routines`: Decrements a word in a global variable and, if it reaches zero, sets another global variable to zero, otherwise it sets it to two, then calls three different routines.
- `scale_and_call_subroutine`: Loads a word from a global variable, then performs four successive cycles of setting d2 to a specific constant, calling another routine, and arithmetic right-shifting d0 by three bits.
- `setup_blitter_parameters`: Masks d0, shifts it left by 5, and calls another routine with a0 and a1 pointing to specific memory locations and several constant values in d3 through d6.
- `process_data_blocks`: Calls several routines to process data using various pointers and constants, including multiple calls to a routine that uses a nested loop to copy word blocks, and finally calls a routine to process a block with a specific count.
- `process_parameter_blocks`: Calls another routine four times with incrementing d3 values and specific register constants, then calls a final routine using a pointer from a global variable.
- `update_state_and_call_subroutines`: Writes 65535 to a global word, compares a value in d1 with a global word and updates it if they differ, then calls a routine and conditionally updates a global word and calls two more routines if the result is between 0 and 3.
- `execute_conditional_blitter_sequence`: Checks if a byte at the VHPOSR register is 100, then performs a sequence of four calls to another routine with various constant parameters, and finally performs a fifth call if a word from a global variable is between 0 and 3 after a multiplication and addition.
- `update_input_state`: Calls three different routines in a loop until a specific flag is set by the third routine.
- `initialize_system_parameters`: Calls several routines to initialize data using various pointers and constants, sets three global longwords, and updates a global word with 100 before calling another routine.
- `update_state_based_on_d0`: Copies a global word to another, sets a global word to 65535, and then performs a series of conditional checks on d0 to call different routines or update a global word.
- `check_and_call_routine_2`: Calls a routine, compares a global word to zero, and if it is not zero, calls another routine before returning.
- `execute_parameter_calls`: Saves registers to the stack, loads two addresses into a0 and a1, calls two different routines, and restores the registers before returning.
- `process_data_buffer`: Calls several routines to process data from a buffer, using values from a structure to calculate offsets and calling a routine multiple times with varying parameters.
- `compare_and_branch`: Loads a pointer from a global variable, compares a byte at offset 29 of that pointer with d0, and branches to another routine if they are equal; otherwise, it loads a pointer from offset 34 of the same structure and branches back to an earlier instruction in this routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `perform_logical_or`: Performs a bitwise OR operation on d0 with zero.
- `clear_d0_register`: Performs a logical OR operation on the data register zero with zero.
- `setup_and_process_tables`: Calls several routines to initialize data from tables, sets a global word to 4f, and calls another routine five times with incrementing d0 values before calling two more routines.
- `execute_conditional_logic_sequence`: Sets a global word to 65535, calls another routine, and if the result is 1, performs two nested loops calling another routine, then conditionally calls other routines based on the value of d0 and updates a global word and a global longword.
- `tiny_move_move`: Small routine of 2 instructions that copies -(a0) to d0, then copies d0 to -(a2).
- `process_table_entry_and_blit`: Calls another routine, then calculates an address by shifting a byte from offset 23 of a structure in a0, loads four words from that address, and calls another routine with a0 and a1 pointing to specific memory locations.
- `format_and_process_buffer`: Calls a routine to get a pointer, then performs a series of operations involving multiplying values by 64, shifting a global word, and calling routines to process data at specific offsets.
- `handle_conditional_blit`: Performs calculations using d0 and a global word, calls a routine, and then conditionally executes a sequence of operations involving a loop and bit testing of a global word.
- `perform_dual_blit_operations`: Calls another routine with d0 set to 0 and d2 set to 117, then calls another routine with d0 set to 16 and d2 set to 135 after adding 30 to d1.
- `set_bit_conditionally`: Subtracts 20 from d0, calculates a value from a word at a global pointer, and if the bit 0 of that value is not set, sets bit d0 of a byte at offset 5 of a structure in a0.
- `clear_bit_at_offset`: Clears a bit in a byte at offset 5 of a structure in a0 using the index in d0, calls another routine, and then returns.
- `update_table_with_calculated_byte`: Subtracts 10 from d0, calculates a value by subtracting 61 from a word at a global pointer and shifting it left by 1, stores this value at a table offset determined by d0, calls two other routines, and then stores the same value at the same table offset before returning.
- `sum_table_bytes`: Iterates through five bytes starting at a global pointer, adding them to a 16-bit register, subtracts a value from a global variable from that sum, and if the result is greater than zero, subtracts it from d1 before returning.
- `compare_and_branch_3`: Compares d1 to a constant, and if it is not that constant, compares a long value at a global variable to a constant, and branches to another routine if they are not equal.
- `dispatch_indexed_blitter_calls`: If a word in d1 is between 1 and 3, it stores d1 in a global variable and performs one of up to three calls to another routine using different constant values in d0 and d2 based on the value of d1.
- `process_conditional_buffer_data`: Decrements a word in a global variable if it is non-zero, calls another routine, compares a byte at offset 19 of a structure in a0 to 65, and if not equal, calls another routine to calculate a value from d0 and a byte at offset 19 of a0, stores the result in a global longword, and sets bit 2 of a global word.
- `sum_hardware_registers`: Adds the bytes from the CIAA timer, CIAA timer low, and the video beam position registers into d0 and returns.
- `fill_data_table`: Calculates an offset using a longword in d0, calls another routine, and then enters a loop that writes incrementing bytes to a global table at specific offsets.
- `load_save_data`: Disables interrupts by writing to the custom chip, calls a routine via a pointer, updates several global pointers, and if a global longword is non-zero, calculates an offset using a multiplication and calls a routine twice with different parameters.
- `disable_interrupts_and_process`: Calls two routines with specific register constants, calls a function via a pointer from a stack location, and then writes values to the hardware custom chip registers.
- `clears_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `clear_data_register_zero`: Sets the least significant byte of data register zero to zero.
- `perform_multiple_logical_ors`: Performs seven consecutive bitwise OR operations on d0 with zero.
- `decompose_value_into_table`: Calculates a series of values by repeatedly right-shifting and subtracting from a value at an offset of 54 in a structure, stores these values in a table by calling itself recursively, and finally stores two results of a division by 96 into the same table.
- `modify_d0_bits`: Sets bit 6 of d0, then performs a bit test on d0, and finally sets all bits in d0.
- `reset_system_state`: Calls several routines with varying parameters and then clears several global variables and a word at the routine's own address.
- `check_and_call_routine`: Writes a 16-bit value to a global variable and, if certain conditions are met regarding d0 and a global variable, calls another routine and updates the global variable.
- `manipulate_d0_register_bits`: Clears the d0 register, sets bit 6, tests bit 6, and then sets all bits in d0 to one.
- `clear_table_bits`: Loops 64 times to clear the two least significant bits of every second byte in a table starting at a global address.
- `decrement_global_word`: Writes 1 to a global word, compares a value from a global word to zero, and if it is greater than zero, decrements that global word.
- `increment_and_branch`: Increments a word at offset 6 of a global variable and branches to another routine.
- `clear_word`: Writes zero to a word at offset 6 of a global variable.
- `adjust_offset_and_call`: Checks a bit in a hardware register, calculates an offset using a value from a structure and a global word, and calls another routine twice with different offsets based on comparisons with global words.
- `adjust_offset_values`: Loads values from a structure pointed to by a0, subtracts a global word from d3 to get d2, and if d2 is negative, modifies d5, d1, and d3 using d2 and the global value before branching to another routine.
- `calculate_and_blit_offset_data`: Subtracts a value from a global variable from the sum of d3 and d5, and if the result is not negative, calculates a new address using a value from a0 and a pointer from a1 to call another routine.
- `compare_and_decrement_global`: Compares a word from a global variable with another global variable and, if the first is greater, calls another routine.
- `update_counter_conditional`: Subtracts a value derived from a global variable from another global variable, then if a comparison with a third global variable passes, increments that third variable and updates the first variable.
- `check_and_decrement_counter`: Compares a word from a global variable to zero and, if it is greater, calls another routine.
- `decrement_counter_conditional`: Adds a value derived from a global variable to another global variable, then if a comparison with a third global variable fails, decrements that third variable and updates the first variable.
- `conditional_block_processing`: Compares two global words; if they are not equal, it calls one routine with specific constants and then calls another routine using a pointer retrieved from a table.
- `check_bounds_and_bit`: Iterates through a list of data, checking if values fall within a range defined by two global words, and calls another routine to test a bit.
- `check_ciaa_bits`: Tests bit 6 of the CIAA PRA register and, if set, tests bit 7 of the same register before returning.
- `wait_for_ciaa_bits`: Enters a loop that waits until both bit 6 and bit 7 of the CIAA PRA register are set.
- `store_word_split`: Stores the lower 16 bits of d0 at offset 6 of a structure in a1 and the upper 16 bits of d0 at offset 2 of the same structure.
- `configure_hardware_and_loop`: Sets three hardware registers to specific values, clears bit 0 of a fourth register, and enters a loop that decrements d0 and checks bit 0 of a fifth register until d0 reaches negative 1, finally clearing bit 0 of the fourth register before returning.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `setup_blitter_transfer`: Calculates an offset, calls another routine, and configures several hardware registers for a blitter operation.
- `update_function_pointer`: Stores a word from a global variable into a longword at a global address and then stores the address 5170 into a global longword.
- `copy_long_word`: Copies a long word from a memory location to a destination at offset 6c.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clear_data_register_zero_2`: Sets the lowest byte of data register zero to zero.
- `clear_d0_register_2`: Clears the least significant byte of the d0 register.
- `clears_data_register_d0_2`: Sets the least significant byte of data register d0 to zero.
- `clear_data_register_zero_3`: Performs a bitwise OR operation on data register zero with zero.
- `clear_data_register_zero_4`: Clears the lowest byte of data register zero.
- `zero_d0_register`: Sets the low byte of the d0 register to zero.
- `perform_multiple_logical_ors_2`: Performs a bitwise OR operation on d0 with zero eight times.
- `open_and_read_file`: Calls a routine to initialize, then uses a library function to open a file, stores the result in a global variable, and if successful, calls another library function to read data and a third to close the file before calling another routine and returning.
- `open_and_write_file`: Calls a routine to initialize, then uses a library function to open a file, stores the result in a global variable, and if successful, calls another library function to write data and a third to close the file before calling another routine and returning.
- `update_joystick_state`: Reads the CIAA_PRA register and the JOY1DAT register, performs bitwise manipulations to combine the values, and stores the result in a global variable.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, clears that register, stores the byte in a global variable, and updates a global word based on whether the byte matches specific characters.
- `parse_and_decrypt_file_header`: Updates a global pointer, calls another routine, and then checks a global pointer against two specific long constants to decide whether to call one of three different routines.
- `copy_memory_block`: Copies a block of bytes from a source address to a destination address by decrementing pointers and moving bytes in a loop.
- `decrypt_data_block`: Calculates a value based on a sequence of bytes from a table, and if this value matches a word at an offset from a pointer, performs an XOR operation on a block of longwords.
- `extract_bitstream_data_2`: Calculates a pointer from d0, then enters a loop that processes bytes from a table using bitwise shifts and writes them to a destination, storing a calculated offset in a global longword.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_registers`: Subtracts the value of register d4 from register d0.
- `extract_bitstream_data`: Iterates through a bitstream by shifting a longword and extracting bits based on a variable bit-length, then uses the extracted values to index into a table and decrement a pointer in a3.
- `fill_aligned_memory`: Aligns two pointers to 32-byte boundaries and then copies a block of eight longwords from a global table to the destination address in a loop.
- `copy_word_array`: Copies an array of words from a source to a destination using a loop controlled by a counter in register d7.
- `calculate_blt_parameters_and_draw`: Calculates bitplane masks and Blitter parameters based on coordinate scaling and bitwise logic, then writes these values to a structure in a6 and calls another routine to perform the Blitter operation.
- `poll_memory_bit`: Polls a specific bit within a byte at an offset from a pointer until that bit is cleared.
- `wait_for_flag_toggle`: Spins until bit 0 of the custom chip register is cleared, then spins until bit 0 is set again before returning.
- `calculate_and_store_structure_values`: Performs several arithmetic operations involving d0, d1, d2, d3, and d6, and uses conditional branches to set d7 before returning.
- `load_constant_and_swap`: Loads the value 1 into d7, swaps d0 and d1, then branches to another routine.
- `compare_and_branch_d1`: Negates d1, compares d1 to d0, and if d1 is greater than or equal to d0, it branches to another routine; otherwise it loads 25 into d7 and branches to another routine.
- `load_constant_and_swap_2`: Loads the value 5 into d7, swaps d0 and d1, then branches to another routine.
- `compare_and_branch_d0`: Negates d0, tests d1, and if d1 is less than zero, it branches to another routine; otherwise it compares d0 to d1 and if d0 is greater than or equal to d1, it branches to another routine; otherwise it loads 21 into d7 and branches to another routine.
- `load_constant_and_swap_3`: Loads the value 9 into d7, swaps d0 and d1, then branches to another routine.
- `compare_and_branch_2`: Negates d1, compares d1 to d0, and if d1 is less than d0, it loads 29 into d7 and branches to another routine; otherwise it branches to another routine.
- `update_structure_fields`: Calculates several values based on registers and a structure in a6, then stores them into offsets 50, 52, 58, 64, 72, 84, 98, and 100 of that structure.
- `update_mouse_position`: Reads a 16-bit value from the mouse port, splits it into two bytes, updates a structure at a0 with these bytes, performs arithmetic using values from that structure and other memory locations, calls another routine, and finally calls a third routine.
- `write_packed_bytes`: Performs arithmetic on registers d0 and d1, then writes the resulting bytes to offsets 0, 1, 2, and 3 of a structure pointed to by a1.
- `decrement_global_counter`: Decrements a global variable and, if it becomes negative, updates another global variable with a value from a pointer and conditionally modifies d2 or d3 based on bits in d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 80 into d0.
- `parse_and_process_bytes`: Reads three bytes from a buffer, performs bitwise operations to update two global words, and then enters a loop to process a sequence of bytes based on specific character values and global table entries.
- `skip_null_terminated_strings`: Uses a value from a global variable as a counter to skip through a series of null-terminated strings pointed to by a0.
- `update_global_and_call`: Calculates a value in d0 by subtracting 32 from d3 and shifting it left by 4, sets d6 based on bit 3 of d2, updates a global variable by adding a word from a structure in a2, and calls another routine.
- `copy_and_encode_buffer`: Copies bytes from a buffer to a destination until a zero byte is found, then performs bitwise shifts and masks on registers to update a global byte and a global word before calling another routine.
- `format_numeric_data`: Saves registers, calls two routines, and then calls a third routine using d0 and d1 as arguments before restoring the registers and returning.
- `convert_to_ascii_digits`: Divides d0 by d1, then uses the quotient and remainder to write ASCII digit characters into a memory area starting at a calculated address.
- `loop_through_memory`: Saves registers, then loops through a memory area using d7 as a counter, calling another routine for every byte that matches the value 48, and finally restores registers and returns.
- `shift_memory_block`: Copies bytes from one location to a preceding location in a loop controlled by d5.
- `process_data_stream`: The routine contains a series of zeroing operations, a call to a function pointer, and a loop that processes data based on a global variable and a pointer.
- `perform_bitwise_or`: Performs a bitwise OR of zero with the contents of register d0.
- `clear_and_process_buffer`: Clears a range of bytes in a buffer starting from an offset determined by a value in a register, calls a routine via a pointer in a1, then checks a global variable for specific characters to either clear more bytes in the buffer or store a byte from a global variable into the buffer and call a routine.
- `loop_and_store_byte`: Compares d1 to d5, and if d5 is less than d1, it loads a pointer into a4, calls another routine, and then enters a loop that checks a global variable and a byte at d2 to swap d2, store it at offset d5 of the buffer in a0, increment d5, and call a routine until d5 reaches d1.
- `conditional_return`: Checks if d5 is zero; if it is not zero, it restores registers from the stack and returns; otherwise it returns immediately.
- `loop_pointer_increment`: Compares d0 to a byte at a4, and if d0 is not equal and the byte at offset 1 of a4 is not zero, it increments a4 by 3 and branches back to the start of the routine; otherwise it returns.
- `swap_bytes_from_pointer`: Reads a byte from offset 1 of a structure in a4, swaps the bytes of d2, and then overwrites d2 with a byte from offset 2 of the same structure.
- `manage_state_and_music`: Compares a pointer against zero and a global variable, then conditionally calls routines to update a global variable, a pointer, and several other global values.
- `decrement_signed_word`: Checks if a word at a global address is less than or equal to zero, and if so, decrements that word by one before returning.
- `loop_through_data_entries`: Sets several global variables, calls another routine twice, and then enters a loop that runs 30 times, calling another routine and updating a pointer based on a value read from a structure in a0.
- `copy_bytes_loop`: Copies 21 bytes from the source pointed to by a0 to the destination pointed to by a1 using a loop.
- `check_interrupt_and_bit`: Checks if bit 5 of a word from a hardware register is set and if bit 1 of a global word is set, then calls another routine if both conditions are met.
- `initialize_memory_tables`: Loads a pointer from a table, iterates through a range of memory to clear bytes and update a linked list, and performs several conditional operations involving bitwise OR on a hardware register and clearing various global bytes.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_branch_on_byte`: Increments a global byte, compares it to another byte, and then either calls a routine and branches or calls a routine twice.
- `update_audio_channels_and_entity_state`: Updates four audio channel registers by calling another routine with specific pointers, then calculates a value based on a byte from a table and updates a global word with the result.
- `update_entity_state_and_audio`: Updates an entity structure by fetching data from a table based on a nibble-extracted index, calculates new position and state values, and modifies audio hardware registers and DMA control.
- `dispatch_structure_parameter_index`: Calls another routine, then extracts a nibble from a word at offset 2 of the structure in a6 to branch to various subroutines based on the value.
- `update_parameter_value_from_structure`: Extracts a 16-bit word from offset 16 of a structure in a6 and stores it at offset 6 of a structure in a5 before returning.
- `update_structure_values_2`: Calculates an offset into a table using a byte from a structure in a6, compares a value from that table with a word in a6, and then updates offset 24 of the structure with the table value, clears offset 22, and sets offset 22 to 1 if the new value is less than the value at offset 16.
- `update_structure_and_lookup`: Checks a value at offset 24 of a structure, modifies offset 16 based on offset 23 and offset 22, performs a lookup in a table using a value from offset 18 and offset 31, and updates offset 26 of the structure using bits from offset 3.
- `process_structure_parameters`: Performs complex calculations on values from a structure in a6, including bitwise shifts, multiplications, and conditional additions or subtractions, and updates various offsets in a6 and a5 based on the results.
- `update_structure_state_from_input`: Reads a byte from offset 2 of a structure, masks the lower nibble, and branches to various subroutines based on the value.
- `extract_nibble_to_offset`: Extracts the lower nibble from a byte at offset 3 of a structure in a6 and stores the result into offset 18 of the same structure before returning.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `contains_zero_word`: Contains a single instruction that performs a bitwise OR on register d0 with zero.
- `clears_data_register_zero_2`: Performs a bitwise OR operation on data register zero using a zero constant.
- `contains_zero_words`: Contains a sequence of 33 instructions that each perform a bitwise OR on register d0 with zero.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `calculate_and_swap_ranges`: Calculates a value using a division and multiplication, calls another routine, and then conditionally calls two other routines or performs a loop to swap word blocks between two memory locations.
- `branch_on_condition`: Calls another routine, then branches to another routine if a condition is met, otherwise it branches to a third routine.
- `test_bit_and_branch`: Loads the address at offset 16 of the structure in a1 into a4, tests bit 16 of d2, and branches to another routine if the bit is clear, otherwise it calls another routine and branches based on the result.
- `branch_and_subtract`: Calls another routine, and if d0 is not greater than or equal to zero, it subtracts a1 from d0 and branches to another routine.
- `swap_and_split_ranges`: Swaps a range of words between two memory locations using a loop, then calls another routine twice to process two potentially overlapping memory segments.
- `copy_bytes_from_offset`: Copies a sequence of bytes from a source address calculated by adding a word offset to a0 to a destination address starting at a fixed memory location, using a loop controlled by d2.
- `compare_byte_sequences`: Compares two sequences of bytes from two different memory locations and returns if all corresponding bytes are equal, using a loop controlled by d2.
- `contains_zero_words_2`: Contains a sequence of 10 instructions that each perform a bitwise OR on register d0 with zero.
- `update_pointer_and_call`: Stores a pointer from a0 into a global variable, copies a value from one global variable to another, and calls another routine if a global variable is non-zero, then calls a final routine if another global variable is non-zero.
- `execute_conditional_subroutines`: Saves registers to the stack, clears d0, calls three consecutive subroutines, and if d0 is not negative, calls three more subroutines and stores the value in a4 into a global variable at offset 68800, finally restoring the registers and returning.
- `fill_memory_with_loop`: Fills a memory location with a byte value in a loop that runs 79 times.
- `verify_and_store_pointer`: Checks if the longword at a0 matches a specific constant; if it does, it stores the longword at offset 4 of a0 into a global variable, otherwise it returns -1.
- `parse_header_data`: Searches for a specific four-byte constant in memory, then extracts and stores several values from the subsequent data into global variables, including a word that is bit-shifted right by 3.
- `parse_and_reconstruct_words`: Searches for a constant in a memory block pointed to by a0, then iterates through subsequent bytes to reconstruct words by shifting and ORing bits into a destination table pointed to by a3.
- `copy_block_with_sign_extension`: Searches for a constant in a memory block, calls another routine, and then iterates through a block of data, either copying bytes directly or negating and copying them based on their sign and a specific value.
- `calculate_and_copy_block`: Calculates a size based on global word values and multiplications, then calls another routine to copy a block of memory to a destination address derived from a4.
- `clear_d0_register_3`: Sets the least significant byte of the d0 register to zero.
- `clear_data_register_zero_5`: Sets the low byte of data register zero to zero.
- `zeroes_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `clear_data_register_zero_6`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `copy_nested_word_blocks`: Performs a nested loop to copy word blocks from a source list to a destination list, where the number of blocks and the size of each block are determined by global variables.
- `perform_nested_copy`: Executes a nested loop that copies bytes from one memory location to another based on values stored in registers.
- `delay_and_call_routines`: Multiplies d5 by 1000 and uses the result to control a nested loop that calls two other routines multiple times.
- `update_color_values`: Iterates through two lists of words, compares their nibbles, and updates the destination list with modified values based on the comparison results.
- `initialize_globals_and_call`: Sets a global longword to zero and another global longword to one, then calls three different routines.
- `execute_multiple_setup_sequences`: Performs several sequences of setting global variables, calling a routine to update pointers, and calling other routines to process data from various structures and tables.
- `update_structured_data`: Writes a longword to a destination list by splitting it into two words at specific offsets, then updates a second destination list with a calculated value and a modified byte.
- `update_structure_values`: Copies a pointer from a6 to offset 0 of a structure at a5, sets offset 4 to zero, and then copies two 16-bit values from a global table into offsets 8 and 10 of the structure, finally storing a byte from a global table into offset 12.
- `update_structure_fields_2`: Sets a global longword to zero, updates a global longword based on the lowest bit of a register, calls another routine, and then populates a structure at a pointer in a1 with values from several global variables and a byte from a global variable.
- `setup_dma_and_loop`: Calculates values using multiplication and bit shifts, configures DMA controller registers, calls two routines, and then uses two loops to add calculated offsets to pointers in a1 and a2.
- `write_data_structure`: Waits for a specific bit to clear and then writes several values into a memory structure at offset 64 of a6, including a constant, two values from a2, and three 16-bit values from d2.
- `write_cleared_structure`: Calculates a value using a 16-bit multiplication and bit shifts, waits for a specific bit to clear, and then writes several values into a memory structure at offset 64 of a6, including a constant, two values from a2, and two zeroed 16-bit values.
- `prepare_blitter_parameters`: Saves registers to the stack, calculates several pointers and values from structures in a0 and a1, and calls another routine before returning.

### 3D World Tennis.adf:if

- `parse_script_conditional_logic`: Checks for the existence of a required library and, if present, parses a script string to evaluate conditional expressions using comparison operators such as equal, greater than, or greater than or equal to.
- `parse_script_string`: Parses a string to identify command keywords and processes numeric values or character data.
- `get_byte_conditionally_2`: If d5 equals d6 and a routine call returns a value less than 1, it returns 255; otherwise, it returns a byte from a table at offset d5 of a5.
- `call_library_function`: Stores a value from d5 into a structure at offset 16 of a table indexed by d7, calls a library function with a pointer from offset 36 of t, and stores the result in a structure at offset 20.
- `process_if_command`: Checks if a pointer in a2 points to a string starting with a specific character, calls another routine to convert the string to a long value, stores that value in the location pointed to by a2, and then calls another routine if the value is not zero.

### 3D World Tennis.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
