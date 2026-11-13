# Game 559 (Quest For Glory 2 - Trial By Fire): code recovery summary

## Functions (part 3 of 3)

### Quest for Glory 2 - Trial by Fire (1992)(Sierra)[cr SKR](Disk 1 of 8).adf:amigasnd.drv
(continued)

- `update_audio_channel_buffer`: Copies a sequence of 16-bit words from a source buffer to a destination buffer while updating status bytes and volume/period parameters in a control structure.
- `update_audio_parameters`: Sets a byte at an offset of 1612 from a pointer in a4, copies a word from a source in a3 to a destination in a0 based on a value in a2, and programs audio hardware registers using a4 and d2.
- `program_audio_channel_alt`: Sets a bit in a global structure, copies a 16-bit value from a source table to a destination buffer, and programs the Amiga audio hardware with period and volume values, but writes a different constant to a structure offset.
- `set_audio_channel_parameters`: The routine programs the Amiga audio hardware for a single channel.
- `reset_audio_byte_if_ff`: Saves four registers to the stack, loads two addresses relative to the program counter into a0 and a1, and if the byte at the address in a0 is 255, it sets d2 to zero and calls another routine; otherwise it returns.
- `write_audio_byte_if_nonzero`: Compares a byte at the address in a1 to 0; if it is not 0, it writes the value 255 to the address in a0 and branches to another routine.
- `check_and_copy_audio_zero`: Sets register d2 to 0, checks if the byte at the address in a0 is 0, and if it is not 0, calls another routine.
- `set_audio_interrupt_flag`: Calls another routine, sets bit 7 of the interrupt request register, and restores d2, a4, a1, and a0 from the stack before returning.
- `set_audio_byte_to_one`: Saves four registers to the stack, loads two addresses relative to the program counter into a0 and a1, and if the byte at the address in a0 is 255, it sets d2 to one and calls another routine; otherwise it returns.
- `update_audio_status_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value two hundred fifty-five to the address in a0 and then branches to another routine, otherwise it returns.
- `check_and_copy_audio_one`: Sets register d2 to 1, checks if the byte at the address in a0 is 0, and if it is not 0, calls another routine.
- `set_audio_interrupt_flag_high`: Calls another routine, writes 256 to the interrupt request register, and restores d2, a4, a1, and a0 from the stack before returning.
- `push_registers_and_check_byte`: Pushes a0, a1, a4, and d2 onto the stack, loads two pointers from a table, checks if the byte at the first pointer is 255, and if it is not 255, calls another routine.
- `check_and_initialize_audio_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value 255 to the address in a0 and branches to another routine, otherwise it returns.
- `check_and_copy_audio_two`: Sets register d2 to 2, checks if the byte at the address in a0 is 0, and if it is not 0, calls another routine.
- `set_audio_interrupt_request`: Calls another routine, writes 512 to the interrupt request register, and restores d2, a4, a1, and a0 from the stack before returning.
- `set_audio_byte_to_three`: Saves four registers to the stack, loads two addresses relative to the program counter into a0 and a1, and if the byte at the address in a0 is 255, it sets d2 to three and calls another routine; otherwise it returns.
- `test_and_set_audio_byte`: Compares a byte at the address in a1 to zero; if it is not zero, it writes the value 255 to the address in a0 and then branches to another routine, otherwise it returns.
- `check_and_copy_audio_three`: Sets register d2 to 3, checks if the byte at the address in a0 is 0, and if it is not 0, calls another routine.
- `reset_audio_interrupt_flag`: Calls another routine, writes 1024 to the interrupt request register, and restores d2, a4, a1, and a0 from the stack before returning.
- `decrement_and_update_counters`: Checks a byte at an offset from a table in a4; if it is not the end marker, it decrements a second byte at a different offset in the same table and updates several values in a global variable structure using data from the table, then increments two 16-bit counters in the table before returning.
- `update_hardware_registers_and_counters`: Writes values from a table in a4 to the hardware registers at a fixed address, a fixed address, and a fixed address, increments two 16-bit counters in a4, and calls two other routines within a loop that continues until a counter in d2 reaches four.
- `update_palette_transition_state_2`: Checks a status byte in a global structure and, if certain conditions are met, updates a state byte and a counter byte using values from a table.
- `update_palette_transition_state`: Calculates an index from a register, checks a status byte in a structure, and either sets a state byte to 3 or performs a multi-stage transition by decrementing a counter, updating a palette index from a table, and adjusting a secondary counter toward a threshold.
- `increment_palette_index`: Copies a byte from a table into a destination buffer, adds a value from another table to a counter, and if the counter is below a threshold, sets a byte to one in a global structure.
- `decrement_buffer_byte_2`: If a byte at an offset in a structure pointed to by a6 is one, it checks if a byte at an offset in a0 is zero; if it is, it subtracts one from that byte in a0 and branches to another routine.
- `decrement_palette_index`: Copies a byte from a table into a destination buffer, subtracts a value from another table from a counter, and if the counter is above a threshold, sets a byte to two in a global structure.
- `decrement_buffer_byte`: If a byte at an offset in a structure pointed to by a6 is two, it copies a byte from a1 at an offset to a destination buffer at an offset in a2, then returns.
- `decrement_buffer_value`: If a byte at an offset in a global structure is three, and a byte in a destination buffer is not zero, it decrements that byte.
- `apply_palette_byte_and_decrement_counter`: Reads a byte from an offset in a structure pointed to by a register, uses it as an index into two tables in a global structure, stores a byte from the first table into a destination buffer and subtracts a byte from the second table from a counter byte, then calls another routine if the counter dropped below one.
- `calculate_audio_volume_2`: Pushes several registers onto the stack, loads a byte from a table at an offset in a4 using an index from a structure at an offset in a4, loads another byte from a different table in a4, and if the first byte is not zero, it performs calculations before writing a 16-bit value to a hardware register.
- `check_byte_and_branch_2`: If the value in d4 is not zero, it branches to another routine; otherwise it returns immediately.
- `compute_scaled_volume_2`: Calculates a volume value by multiplying two bytes, dividing by sixty-three, scaling it by a value from a global structure, and writing the result to a hardware register.
- `compute_scaled_volume`: Calculates a volume value by multiplying two bytes, dividing by sixty-three, scaling it by a value from a global structure, and writing the result to an Amiga audio register if a flag is set.
- `set_audio_channel_volume_2`: The routine computes a volume value from two input bytes, scales it, forces a minimum of one, clears it if a flag byte in a global variable is zero, and writes the result to the left volume register of an audio channel selected by a base value.
- `search_audio_channel_table`: Iterates through a four-entry table to find a byte matching a value in a register, then performs several writes to a structure and hardware registers if a match is found.
- `configure_audio_channel_2`: Writes a value to offset 18 of a structure in a3, clears several entries in a table using an index in d2, and then loads data from a source.
- `configure_audio_channel`: Writes a value to offset 18 of a structure in a3, clears several entries in a table using an index in d2, and then loads data from a source table into a structure pointed to by a4.
- `clear_audio_channel`: Clears an audio channel by writing zero to a hardware audio register selected by a channel index, calls another routine, then marks a byte in a table as active.
- `search_audio_index`: Pushes registers to the stack and compares a value in a register against a table, incrementing a counter if they do not match.
- `shift_and_reset_audio_channel`: Shifts the value in d2 right by 2, calls reset_audio_channel_state, and then pops a longword into a1 and a word into d2 before returning.
- `search_audio_index_alt`: Pushes registers to the stack and compares a value in a register against a table, incrementing a counter if they do not match, otherwise it sets a register to one.
- `clear_flag_and_return_byte_2`: Writes a constant to a word at an offset from a pointer in a1, then reads a byte from a table indexed by d2 and base a4, clears a bit, and loads a pointer from a table into a1.
- `clear_flag_and_return_byte`: The routine writes a constant to a word at an offset from a pointer in a1, then reads a byte from a table indexed by d2 and base a4, clears that byte in the table, and returns the byte shifted left by sixteen bits.
- `write_hardware_constants`: Writes specific constant values to two hardware registers.
- `fetch_table_byte_6`: Swaps the value in d2, compares it to 192, and if not equal, jumps to another location; otherwise it swaps d2 back, calculates an offset from a table in a4 using a shifted d2, loads a word from that table into d0, and jumps to another routine.
- `fetch_table_byte_5`: Compares a word in d2 to 192, and if equal, swaps d2 and loads a byte from a table at offset 1240 of a4 using the modified d2 as an index into d0, then jumps to another routine.
- `fetch_table_byte_4`: Compares d2 to 176, and if equal, swaps d1, compares the swapped d2 to 7, and if equal, loads a byte from a table at offset 1288 of a4 using d0 masked with 255.
- `fetch_table_byte_2`: If d2 is not 40, it returns; otherwise it loads an address from a table at an offset in a4, masks d0 with 255, and moves a byte from that address into d0 before branching to another routine.
- `fetch_table_value`: If d2 is one, it loads the address of a table into a0, masks d0 with a constant, and moves a byte from that table into d0 before branching to another routine.
- `fetch_table_byte`: Compares registers against constants and, if they match, fetches a byte from a table using a masked value in a register.
- `search_and_return_index`: Performs a loop to find a matching byte in a table, increments a counter, and returns a value derived from that counter.
- `return_constant_16bit_value_and_pop_stack`: Loads a constant 16-bit value of all ones into a register, pops two longwords from the stack into registers, and returns.
- `initialize_audio_hardware`: Calls a routine using a pointer from a structure in a6, stores the result in offsets 1204 and 1540 of a structure in a4, and then writes a series of eight incrementing values starting from the result into offsets 1544, 1548, 1552, 1556, 1560, 1564, and 1568 of the same structure.
- `initialize_audio_device`: Moves a value from a0 to an offset in a4, increments that offset by 4, and then calls a routine using a pointer from a structure in a6, branching based on the result.
- `initialize_audio_interrupt_handlers`: Clears a block of memory, sets up or updates four audio channel structures with a common pointer and handler, and then writes a value to the certain hardware register to disable interrupts.
- `return_constant_7`: The routine returns a constant 64-bit value and performs no other operations.

### Quest for Glory 2 - Trial by Fire (1992)(Sierra)[cr SKR](Disk 1 of 8).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_unspecified_logic`: The routine performs a large number of repeated moves and logical operations on data in a0 and d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `return_constant_9`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_entry`: Calls a routine via a pointer in a5, then uses the result to index into a table and compares a value at the calculated offset with a constant.
