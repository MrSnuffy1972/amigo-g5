# Game 903 (rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00903). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914800 top-level bytes in 44 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 159 of 159 named (0 author symbols), 159 of 159 described (11 by rule, 145 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1228 of 1228 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3212 of 3260 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 148061 |
| code | 12424 |
| data | 737039 |
| padding | 9084 |
| unused | 8192 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:PDY-RALLY.EXE | 3 | 160 | 5 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:GO.EXE | 2 | 267 | 8 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:CopStart | 1 | 14 | 1 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 179 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2122 | 119 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 499 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:PRES.EXE: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:OPTION.EXE: high-entropy data file (7.64 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/Crash.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/Victory.iff: high-entropy data file (7.90 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/Timeout.iff: high-entropy data file (7.88 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/WorldImage.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/InsideLogo.iff: high-entropy data file (7.82 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/WorldPodio.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/SelectRally.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/OutFuel.iff: high-entropy data file (7.92 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/SinglePodio.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/Mod.Racer: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:Data/practice.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)[a]/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:flagselect.exe: high-entropy data file (7.69 bits/byte), classified as packed data
- runtime segment an address: 122 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 435 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 258 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_for_shutdown_flag` (runtime segment an address): Waits until a shared variable equals minus one and then loops endlessly. Kind: sabotage path / infinite loop trap. Pass path: bra.w an address.

## Functions


### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:PDY-RALLY.EXE

- `initialize_program_entry`: The routine calls another routine multiple times and invokes a function pointer stored in a global variable.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `populate_data_buffer_via_table_lookups`: Reads a count and a source pointer from a table, then performs a series of conditional lookups and data transfers into a destination buffer.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using an index passed in a register, and returns it ANDed with the input word.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:GO.EXE

- `apply_amiga_hunk_relocations`: Parses a list of data structures by comparing values against AmigaDOS hunk type codes, performing memory allocation, and applying relocations or copying data blocks based on the identified hunk types.
- `process_pointer_list`: Decrements a counter in d1 and loops while it is non-zero, performing pointer arithmetic and calling a function pointer from a global variable and a library function.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `extract_bit_stream_2`: Loads a value from a pointer in a0, calls a routine to check a flag, and then enters a loop that calls other routines to perform bitwise operations and store bytes into a memory range starting at a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bit_stream`: Performs a bitwise extraction loop that shifts and rotates values from a memory source based on a variable bit length, updating a register and writing results to a destination buffer.
- `execute_trap_instruction`: Loads a word from an offset of a2, performs a bitwise OR on d0, calls a trap, and performs a bitwise OR on d1.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:CopStart

- `open_and_close_library_probe`: Opens a library by name through the exec OldOpenLibrary call and, on success, stores a pointer to a shared variable in the library's base field before closing it.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_and_setup_interrupts`: Calls a library function to obtain a pointer, stores it in a global variable, and then calls five subroutines to install an interrupt server, write four fixed values, write two value pairs, wait for a shutdown flag, and wait for the blitter before entering an infinite loop.
- `install_interrupt_server_safely`: Writes disable values to the interrupt and DMA control registers, forbids multitasking through exec, then installs an interrupt handler with the exec AddIntServer call.
- `write_four_fixed_values`: Writes four fixed values into shared variables and returns.
- `write_two_value_pairs`: Writes two fixed value pairs into shared variables and returns.
- `wait_for_shutdown_flag`: Waits until a shared variable equals minus one and then loops endlessly.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_value_return_minus_one`: Writes a fixed value into a shared variable and returns a result whose high word is all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_process_task_loops`: Initializes several global variables and a structure with specific values, then enters a loop that manages task execution based on the value of a global variable.
- `load_practice_asset`: Calls a routine with d1 set to a specific constant, calls a routine at the address in a0, sets a global variable to 1, and calls another routine.
- `execute_task_and_set_flag`: Calls manage_task_pointers and then executes a routine via a pointer returned in a0, before setting a global word to 1 and calling another routine.
- `load_out_of_time_asset`: Calls a routine with d1 set to a specific constant, calls a routine at the address in a0, stores the result in a global variable, sets a global variable to 1, and calls another routine.
- `load_damage_asset`: Calls a routine with d1 set to a specific constant, calls a routine at the address in a5, stores the result in a global variable, sets a global variable to 1, and calls another routine.
- `execute_task_and_update_status`: Calls a routine with a constant value, executes a function pointer stored in a register, and updates a global variable with the result before setting a status flag in another global variable.
- `load_winner_single_asset`: Calls a routine with d1 set to a specific constant, loads d0 and d1 from global variables, calls a routine at the address in a5, sets a global variable to 1, and calls another routine.
- `update_task_and_execute_callback`: Stores a longword into a global variable, calls a routine, then executes a function pointer retrieved from a second global variable before setting a global word to one and calling another routine.
- `execute_task_and_set_flag_2`: Calls a routine with a constant value in d1, executes the function returned in a0, sets a 16-bit value in a global variable to 1, and calls another routine before returning.
- `execute_task_pointer`: Loads a library base pointer into a6, calls a routine with an address in d1, then calls a routine at the address in a0, writes 1 to a global variable, and calls another routine before returning.
- `allocate_and_initialize_memory`: Allocates memory using library calls, calls three routines via pointers, writes 20081 to four consecutive words at an offset of 156 from a pointer in a0, calls a routine at a0, writes 1 to a global variable, and calls another routine before returning.
- `execute_task_with_parameter`: Loads a library base pointer into a6, calls a routine with an address in d1, calls a routine at the address in a0, writes 1 to a global variable, and calls another routine before returning.
- `update_and_store_task_data`: Calls a routine with 3386 in d1, clears registers a1 through a6, calls a routine at a0, writes 1 to a global variable, and updates five global longwords using values from a0 and other global variables before calling another routine.
- `execute_task_and_store_result`: Loads a library base pointer into a6, calls a routine with an address in d1, calls a routine at a0, writes 1 to a global variable, and stores the result in a0 into a global variable before calling another routine.
- `update_task_pointers_from_data`: Calls a routine with 3841 in d1, loads six longwords from a table, clears registers a1 through a5, calls a routine at a0, writes 1 to a global variable, and updates five global longwords using the returned values and original table data before calling another routine.
- `calculate_and_call_task`: Calls a routine with 3861 in d1, performs arithmetic to calculate offsets into a table, loads a value from that table, calls a routine at a0, writes 1 to a global variable, and calls another routine before returning.
- `update_task_pointers`: Loads several longwords from a table into registers, calls a routine at a pointer in a0, updates a global variable with the result, and writes several longwords back to the table before looping or returning.
- `process_asset_table`: Clears several registers, calls a subroutine, calculates an offset into a table using a multiplication and a bit shift, and then calls another routine with several global pointers before updating multiple global variables and calling a final subroutine.
- `execute_callback_and_set_flag`: Loads a function pointer from a structure into a register, calls that function, writes the value one to a global variable, and then calls another routine before returning.
- `update_global_accumulators`: Sets a global variable to zero, calls another routine, and then adds the returned value from d1 to two different global variables.
- `calculate_and_store_time_data`: Calculates a value based on a table lookup and several bytes, then divides the result into four bytes stored at a pointer derived from d0.
- `calculate_offset_value`: Calculates a value by accessing a table using two offsets derived from global variables, performs arithmetic on bytes retrieved from the table, and returns the result.
- `configure_blitter_and_execute_task`: Calls two routines to process data, manipulates a global counter, configures hardware registers including the blitter and DMA controller, calls a routine with 3846 in d1, and updates a global variable with the value in d7 before calling another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_table_ranges`: Calls another routine twice with different parameters and different source and destination table addresses.
- `call_library_function_twice`: Calls a routine at an offset from a pointer stored in a memory location twice and then returns.
- `check_hardware_status`: Performs a bitwise AND on a value from a hardware register and compares it to a constant, otherwise it returns.
- `update_table_elements`: Loops through a table, storing the lower 16 bits of a register at offset 6 and the upper 16 bits at offset 2 of each entry, while adding a value to the register in each iteration.
- `manage_library_handle`: Calls a routine via a pointer from a library base, and if the result is non-zero, calls another routine with two addresses in a6 and a1, then negates a word from a structure and calls a third routine before closing the library.
- `calculate_division_result`: If d1 is positive, it doubles a value from a global variable, optionally performs an XOR with a constant, stores the result back to the global variable, and returns the quotient and remainder in d0.
- `shift_list_elements_2`: Checks if a global variable is non-zero, then either calculates an offset into a table and calls another routine or calls another routine with a different table address.
- `shift_list_elements`: Searches a list for a value, and if a condition is met, shifts elements within the list and updates the list head and tail pointers.
- `do_nothing`: Performs an operation that does not change the value of d0.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `clears_data_register_zero_2`: Performs a logical OR operation with zero on data register zero.
- `check_interrupt_and_call`: Saves registers, reads the interrupt enable register, tests bit 5, compares a 16-bit word in a global variable to zero, and if the value is not less than zero, calls another routine before jumping to a fixed address.
- `clear_audio_dma_state`: Clears a table of DMA pointers and resets a set of global variables, then writes to the audio volume registers and the CIA port register and updates the DMACON register to enable DMA channels.
- `process_audio_logic`: Increments a byte at a global address, compares a byte from a table with another byte, and if the comparison fails, clears a byte at a global address, checks a byte at another global address, and calls another routine before branching.
- `branch_to_subroutine_2`: Calls a subroutine and then branches to another location.
- `update_audio_channels`: Updates the four Amiga audio channel launch addresses by calling a parameter update routine for each channel sequentially.
- `update_audio_channels_2`: Extracts a byte from a table using an index, performs bit shifts and additions to calculate a value, clears a 16-bit word in a global variable, and then calls another routine four times to update the four Amiga audio channel launch addresses.
- `update_vehicle_attribute_data`: Loads a structure from a table based on a nibble-extracted index, then updates several fields including a calculated offset and two 16-bit values at offsets 10 and 36 of the structure in a6.
- `update_structure_state_logic_2`: Calculates a new value by adding a constant to a field at offset 4 of a structure, stores it in offsets 10 and 36, and updates a field at offset 14 from a table.
- `update_structure_state_logic`: Searches a table for an entry matching a value from a structure, then updates several fields in that structure and a global variable.
- `call_and_branch`: Calls a subroutine and then branches to another routine.
- `update_structure_state`: Searches a table to find an entry matching a value from a structure, then updates several fields in that structure and a global variable based on a nibble-based dispatch.
- `update_audio_parameters`: Updates audio hardware registers with values from a table, modifies a global counter, and performs conditional logic based on a byte value and a global variable.
- `dispatch_parameter_update`: Reads a command code from offset 2 of a structure in a6 and branches to different subroutines based on the value.
- `copy_word_to_offset`: Copies a 16-bit word from offset 16 of a structure in a6 to offset 6 of a structure in a5.
- `classify_divided_value`: Loads a byte from a constant offset, divides it by 3, swaps the high and low bytes of the result, and branches to different locations based on whether the value is 0, 2, or otherwise.
- `extract_masked_byte`: Reads a byte from offset 3 of a structure in a6, masks it with a constant, and branches to another location.
- `transfer_word_data`: Moves a 16-bit word from an offset of 16 in a structure pointed to by a6 into d2 and branches to another routine.
- `search_table_for_value`: Loops through a table of 37 entries, comparing values to a constant and storing a word from the table into a structure at offset 6 of a pointer derived from d0.
- `store_word_to_offset`: Stores a 16-bit word from register d2 into a structure at offset 6 of the structure pointed to by a5.
- `subtract_and_clamp_word`: Subtracts a value from a word in a structure, clamps the result to a minimum of 289 if it is below that value, and stores the lower 12 bits of the result into a structure at offset 6 of a5.
- `add_and_clamp_word`: Adds a value to a word in a structure, clamps the result to a maximum of 856 if it is above that value, and stores the lower 12 bits of the result into a structure at offset 6 of a5.
- `update_parameter_from_table`: Calculates an offset into a table using a byte from offset 18 of a structure in a6, searches for a 16-bit value matching a masked 16-bit value from offset 0 of the structure, and if found, stores that value at offset 24 of the structure, clears offset 22, and potentially sets bit 0 of offset 22 if the value is less than a value at offset 16.
- `clear_word_at_offset`: Clears a 16-bit word at offset 24 of the structure pointed to by a6.
- `update_byte_and_clear`: If a byte at offset 3 of a structure in a6 is non-zero, it copies that byte to offset 23 of the same structure and clears the byte at offset 3; otherwise it returns immediately.
- `update_structure_value`: Checks a word at offset 36 of a structure; if non-zero, it either adds a byte from offset 39 to offset 24 or subtracts it, then clamps the result to the value at offset 24 and calls another routine to perform a table lookup.
- `calculate_index_and_store`: Subtracts a value from a word, performs a loop to find a value in a table based on a calculated index, and stores the result into a structure.
- `update_nibbles_in_byte`: Checks a byte at offset 3 of a structure in a6; if non-zero, it merges the lower nibble of that byte into the lower nibble of a byte at offset 26 of the structure, and merges the upper nibble of the byte at offset 3 into the upper nibble of the byte at offset 26.
- `calculate_scaled_value`: Processes a byte from offset 27 of a structure in a6 to derive a value, then updates a 16-bit word at offset 6 of a structure in a5 and modifies a byte at offset 27 of the structure in a6.
- `calculate_subtracted_byte`: Tests a byte at offset 27 of a structure in a6 and branches if it is positive; otherwise, it subtracts the value in d0 from 255 and stores the result in d2 before branching to another routine.
- `copy_byte_and_branch`: Copies the byte from register d0 to register d2 and then branches to another routine.
- `calculate_scaled_offset`: Loads a byte from a structure at offset 26 and another from offset 26 of a6, performs a multiplication by the low nibble of the second byte, shifts the result, and adds it to a value from offset 16 of a6 if a sign bit is not set.
- `subtract_and_advance_phase`: Subtracts one input value from another and stores the difference in a global variable.
- `branch_to_update_routine`: Calls a routine and then branches to another location.
- `branch_to_subroutine`: Calls a subroutine and then performs an unconditional jump to another location.
- `update_clamped_parameter`: Updates a byte at offset 28 of a structure in a6 by combining nibbles from a byte at offset 3, then calculates a scaled value based on bytes at offsets 29, 30, and 28 to store a clamped 16-bit result at offset 8 of a structure in a5 and updates a byte at offset 29 of the structure in a6.
- `calculate_clamped_subtraction`: Checks a byte at offset 27 of a structure in a6; if the byte is negative, it subtracts a value in d0 from 255 and stores the result in d2, otherwise it branches to another routine.
- `transfer_byte_and_branch`: Moves a byte from a data register to another data register and then performs an unconditional short branch into another routine.
- `calculate_scaled_offset_variant`: Loads a byte from offset 28 of a6 and another from offset 19 of a6, multiplies the latter by the low nibble of the former, shifts the result right by 6, and adds it to a value from offset 16 of a6 if a sign bit is not set.
- `clamp_and_update_value`: Calculates the difference between two values, clamps the result between zero and sixty-four, stores it in a structure, and updates a byte by merging a shifted nibble from a neighboring byte.
- `adjust_structure_offset`: If a byte at offset 3 of a6 is non-zero, it copies that byte to offset 32 of a6, shifts it left by 7, and if it is less than a word at offset 8 of a6, subtracts it from that word and adds a shifted version to a longword at offset 4 of a6.
- `set_byte_to_one`: Stores the value 1 into a word at offset 8 of the structure pointed to by a6.
- `increment_clamped_byte`: Takes the high nibble of a byte at offset 3 of a6, and if it is non-zero, adds it to a byte at offset 19 of a6, clamping the result to 64, and stores the result in a word at offset 8 of a5.
- `decrement_and_clamp_byte`: The routine reads a byte from an offset in a structure, subtracts the low nibble of another byte from the same structure, writes the result back, and clamps it to zero if it became negative.
- `clear_global_byte_flag`: It stores one less than the value of a byte in a flag, writes zero to a global variable, and finally sets the flag to all ones.
- `clamp_and_store_byte`: Reads a byte from a structure, clamps its value to a maximum of sixty-four, and stores the result in both a byte and a word at different offsets.
- `calculate_and_store_flag`: Takes a byte from offset 3 of a6, extracts its high nibble, multiplies it by 10, adds the low nibble of the original byte, and if the result is not greater than 63, stores it into a longword at offset 18 of a global variable and a byte at offset 22 of a global variable.
- `write_byte_to_memory`: Checks if a byte at offset 3 of a structure in a6 is zero; otherwise, it writes that byte to a global memory location and writes a zero to the byte immediately preceding that location before returning.
- `dispatch_by_nibble`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different routines based on whether the value is 9, 11, 13, 14, 15, or 12, otherwise it branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `manage_library_lifecycle`: Stores two longwords into global variables, calls a subroutine, and then performs a series of library calls to allocate, free, and manipulate memory or task attributes based on the results of previous calls and specific constant checks.
- `close_library_and_load_registers`: Loads a library base into a6, loads a value into a1, calls a routine at an offset from a6, calls another routine via a branch to subroutine, and finally loads two 32-bit values from global variables into d0 and d1 before returning.
- `manage_library_assets`: Calls a routine with a specific constant, increments a global value, calls another routine via a pointer, updates several global variables, and calls a subroutine before branching to another location.
- `decode_variable_length_bitstream`: Reads a bitstream from a source pointer and decodes variable-length values into a destination buffer.
- `extract_bitstream_byte`: Calls another routine, then performs a loop that shifts a longword in d0 and rotates bits into d3, finally storing the resulting byte at the address pointed to by a2.
- `unpack_and_store_bitstream`: Reads a 32-bit word from a source pointer in a0, performs bitwise rotations and shifts to extract bits, and stores the resulting byte into a destination buffer in a2.
- `initialize_bitstream_unpacking`: Sets a counter in register d1 to 2, calls unpack_and_shift_bitstream, adds 7 to the value in register d2, and stores this new value in register d3 before branching to another routine.
- `process_bitstream_chunk`: Sets a counter in register d1 to 8, calls unpack_and_shift_bitstream, moves the resulting value from register d2 to d3, and branches to another routine.
- `shift_and_branch_bitstream`: Shifts a 32-bit value in d0 right by one bit, then calls another routine if the result is non-zero, otherwise it calls another routine if the carry flag is set or performs a jump to a specific location if the carry flag is clear.
- `unpack_and_shift_bitstream`: Shifts a 32-bit value in d0 and uses the carry flag to determine whether to call shift_bits_loop or shift_pointer_value, then performs a loop to copy bytes from a source pointed to by a2 into a destination while decrementing a counter in d3.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `refill_bits_backward_until_count`: Repeatedly refills a bit register from the longword before the source pointer and shifts bits out, returning when a count in d1 runs out.
- `shift_bits_loop`: Decrements a counter in d1, clears d2, shifts d0 left by one, and if d0 is non-zero, performs a rotate left through carry on d2 before looping based on the counter in d1.
- `initialize_and_close_library`: Calls two routines, loads a library base from a global variable, calls a library function with d1 and d2, and if the result is non-zero, stores it in a global variable and calls a third routine before branching to another location.
- `fill_zero_data`: Contains a series of instructions that perform a logical OR with zero on a data register, effectively acting as a data table.
- `manage_task_pointers`: Stores a longword into a global variable, calls a routine, then uses a library call to check a pointer, potentially repeating the library call in a loop, and finally stores a value from a0 into a global variable and calls another routine.
- `copy_memory_backward`: Copies a sequence of bytes from a table to a destination by reading from a pointer and writing to the preceding byte in a loop.
- `get_task_attribute`: Calls a library routine to find a task, reads a longword from an offset within that task, stores it in a global variable, and then sets the value at that same offset within the task to all ones.
- `set_task_pointer`: Calls a routine using a pointer from a library base, stores the result in a global variable, and then writes that value to offset 184 of a structure pointed to by a1.
- `initialize_pointer_tables`: Calls two different routines to process data at two different memory locations, then copies a longword from offset 50 of a structure to a new location and updates that same offset with the new address.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `accumulate_and_store_words`: Iterates through a loop to store the lower and upper words of a 32-bit register into a structure at specific offsets, while adding a value to the register in each iteration.
- `configure_blt_hardware`: Searches for a specific character in a string, calculates an address based on a value derived from that character, and then updates several Blitter hardware registers and DMA registers.
- `wait_multiple_raster_lines`: Calls another routine to wait for a raster line and repeats this process fifty times in a loop.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `unpack_and_load_asset_data`: Allocates memory and checks for specific magic constants in a data stream, including values representing chip, fast, and pubb tags.
- `process_data_table`: Calls a routine with several parameters, checks if a value at a pointer matches a constant, and if so, enters a loop that performs additions on a table of values using a counter and a secondary table.
- `execute_library_calls`: Calls three different library functions using offsets from a pointer in d0, then stores the result of the last call into a global variable and returns with a pointer shifted left by 2.
- `perform_zero_or`: Performs a logical OR with zero on a data register.
- `clear_memory_blocks`: Clears nine consecutive longwords starting from a specific memory location and then clears four additional longwords at a different location.
- `loop_unpack_bitstream_data`: Loads several longwords from memory into a structure pointed to by a0 and a5, calls another routine, and then enters a loop that decrements d0 and increments a4 until d0 reaches zero.
- `unpack_variable_bit_stream`: Reads a bitstream from a source buffer and unpacks it into a destination buffer based on a length and bit-width specified in a control structure.
- `fetch_shifted_bit_word`: Fetches a longword from the address pointed to by a0, rotates it right by one bit, and returns.
- `shift_and_loop_bits`: Decrements d1, clears d2, and enters a loop that shifts d0 right and rotates d2 left until d1 reaches zero, otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_manage_libraries`: Opens two libraries and stores their base addresses in global variables, then enters a loop that calls another routine up to seventy-five times while checking a bit in a hardware register.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `setup_split_longword`: Loads a longword from a global variable into a0, sets a pointer in a1 to a global variable, sets d1 to 28, and calls a routine to split and store the longword.
- `store_split_long_values`: Splits a 32-bit value from a register into two 16-bit words, storing them at offsets 2 and 6 of a structure pointed to by a register, adds a value from another register to the original 32-bit value, and repeats this process through a loop.
- `copy_word_loop`: Copies a word from the source pointed to by a0 to offset 2 of the destination pointed to by a1, increments both pointers by their respective sizes, and repeats this process for a count in d7.
- `adjust_values_in_loop`: Iterates through a loop to compare high, middle, and low nibbles of values from two source pointers and updates a 16-bit value in a table at offset 2 of a structure in a2 accordingly.
- `process_data_blocks`: Traverses a linked list of structures to find a specific signature, then copies bytes from a source to a destination based on a count and a sign bit, updating a global counter and a structure member.
- `pack_nibbles_into_words`: Traverses a linked list of structures to find a specific signature, then packs nibbles from three consecutive bytes into a 16-bit word and stores it in a table at a global address.
- `probe_and_load_library`: Attempts to open a library and allocate memory, then iterates through a list of structures to find a specific task or library base.
- `unpack_bitstream_to_buffer`: Reads a bitstream from a source pointer and unpacks bits into a destination buffer based on a count and bit-width specified in a structure.
- `fetch_shifted_bit_word_loop`: Fetches a longword from the address pointed to by a0, rotates it right by one bit, rotates d2 left by one bit, and loops d1 times using a decrement and branch instruction before returning.
- `decrement_and_loop`: Decrements d1, clears d2, and performs a logical shift right on d0; if d0 is zero, it returns, otherwise it performs a rotate left through the carry flag into d2 and enters a loop controlled by d1.
- `do_nothing_2`: Performs four identical logical OR operations on d0 with zero.
- `copy_bytes_decrementing`: Copies a byte from a source pointed to by a0 to an offset of -8 from that same pointer, then increments a0 and decrements a counter in d1, looping until the counter is no longer positive.
- `find_task_and_set_priority`: Calls a routine via a pointer in a6, stores the result in a global variable, then uses the returned pointer to read a value from offset 184, stores that value in another global variable, and writes -1 to offset 184 of the returned structure.
- `find_task_and_store`: Calls a routine to load a library base into a6, calls another routine to find a task, and stores the result at offset 184 of a global variable.
- `load_library_base`: Loads the value from a global address into a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
