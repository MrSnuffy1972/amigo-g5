# Game 1073 (ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01073). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1147184 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 322 of 322 named (0 author symbols), 322 of 322 described (38 by rule, 283 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 82889 of 82890 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 100278 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 89200 of 90349 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20974 |
| code | 280006 |
| data | 814433 |
| padding | 9243 |
| unused | 22528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:T.X | 5 | 8373 | 317 |
| Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:bootblock | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81935 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:RUNNER.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:RAZ.FIC (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:RAZDEM.FIC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:ZOLTAR.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:IO.CO: high-entropy data file (7.61 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:FLY5.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:FORET.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:GERDEP.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:EAFF.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:MAIN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:FFORET.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:MESSAGE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:SPACE.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:GESORT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:PARAM.CO: high-entropy data file (7.65 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:FRISE.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:EATT.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:FEE5.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:TEXTIN.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:CHAOS5.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:LOGO.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:SCOMB.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:SFORET.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:SOURIS.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:INTRO5AM.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:OBJET.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:DEADAM.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:INTRO4AM.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run ref slid through memory (an address: 4 64 KB pages each with >= 29491 distinct PCs executed, 201754 of its 286750 PCs), a runaway, not game code; its slide PCs are excluded from executed game code and a fresh valid extended run (no slide, re-executes every other canonical PC) stands in for it
- reference run ext ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 17)
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 81920 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:RUNNER.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:RAZ.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (demo) (1994)(silmarils)/Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:RAZDEM.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 4 never-loaded region(s), 100278 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `save_and_reset_hardware` (T.X): Copies several hardware register values into saved global variables, then writes constants to several hardware registers and resets certain globals. Kind: hardware state reset/sanitization. Pass path: sub_0_2A3A continues execution after the bsr calls.
- `clear_bits_and_wait` (T.X): Clears bits 7 and 3 of a hardware register, busy-loops a 100-iteration counter while repeatedly clearing bit 7 of that same register, and finally calls two other routines. Kind: disk hardware/timing check. Pass path: execution continues to subsequent bsr calls.
- `toggle_bit_loop` (T.X): Checks bit 4 of a hardware register and, if set, enters a loop that toggles bit 1 and bit 0 of a hardware register while calling another routine. Kind: disk hardware/timing check. Pass path: the loop terminates when the bit condition is no longer met, allowing execution to proceed to the instructions following the call site.
- `loop_with_delay` (T.X): Runs a loop 79 times where it clears bits 1 and 0 of a hardware register and sets bit 0 before calling another routine. Kind: disk hardware/sync check loop. Pass path: the loop completes and the code proceeds to the delay loop.
- `pulse_serial_port_until_ready` (T.X): Calls a routine, clears bit 2 of a global variable at a fixed address, writes a word to offset 150 of a structure in a6, loads a longword from a global pointer into offset 32 of a structure in a6, clears offset 36 of that structure, calls another routine, and then polls a status register in a loop for a specific count or until a bit in a global variable changes. Kind: disk hardware/timing check. Pass path: the loop completes without the bit being set (iVar1 reaches 0).
- `latch_status_into_four_words` (T.X): Clears bit 3 of a global status byte, sets bit 7, and writes the result ORed with the top bit into four 16-bit slots of the local frame. Kind: disk hardware status latching. Pass path: d0 = 0 (success/no error).

## Functions


### Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:T.X

- `jump_to_address_2`: Performs a long jump to the address stored in the long word at the specified location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_system_environment`: Retrieves a library base pointer from a global variable and calls a function at a negative offset from it.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `initialize_system_resources`: Performs multiple library calls to allocate memory and check availability, updates several global pointers with results, and executes a sequence of function pointers and subroutines to set up system state.
- `initialize_keyboard_device`: Calls two routines via pointers from a global variable, stores the result of the first in a global variable, then populates a structure at offset 14 with a pointer, a word, and a pointer, before calling a third routine and returning if the result is zero.
- `call_four_routines_in_sequence`: Calls four unnamed routines one after another and then returns.
- `store_mouse_position`: Copies a 16-bit value from the mouse port to a global variable and returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_relocation_tables`: Initializes several global pointers, allocates memory via a library, and sets up relocation or header-related pointer offsets.
- `copy_and_set_pointers`: Copies a block of memory from a source to a destination, updates several global pointers with calculated offsets, and calls two different routines while modifying hardware registers.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_value_in_table`: Searches a table of shorts for a specific value and returns the number of steps taken or a negative error code.
- `branch_if_negative_or_zero`: Tests a 16-bit value in a register and branches to another routine if the value is negative or zero; otherwise it returns.
- `apply_scaled_transform`: Calculates a value using a multiplication and addition, then stores the result as a dword into a table, while also modifying a hardware interrupt enable register.
- `update_pointer_and_string_tables`: Iterates through a table to update pointers with an offset-based calculation and writes a string of characters to a memory buffer.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_indirect_function`: Calls a function pointer stored in a memory location and returns the result.
- `wait_for_bit_set`: Loops until bit 4 of a hardware register is set.
- `wait_for_flag`: Loops until the lowest bit of a certain hardware register is set.
- `set_interrupt_enable_bit`: Calls two routines and writes sixteen to the INTENA hardware register.
- `update_pointer_table`: Iterates through a list of pointers and updates the target or destination using a linear transformation or offset.
- `write_instruction_sequence`: Sets bit 1 of a hardware register, writes two bytes to a memory location pointed to by a value from a table, and calls a routine with a pointer from a structure.
- `execute_sequence`: Calls two routines, writes the values 15 and 1200 to the hardware registers at a fixed address and a fixed address, calls a function via a pointer from a structure in a6, updates a global variable with the result, and then calls another function via a pointer from a structure in a6.
- `reset_and_recurse_pointer`: Clears three bytes in a global variable, calls a routine pointer retrieved from a table offset by a global variable, stores the result in a global variable, updates a global pointer with a specific address, and calls another routine using a pointer from a table.
- `set_system_state`: Calls a function through a pointer, updates a global variable, and calls another function with an argument.
- `increment_and_recurse`: Increments a 32-bit global variable and then performs a series of recursive or repeated calls to another routine.
- `set_intena_value`: Writes the value 32 to the INTENA register and returns.
- `modify_global_value`: Checks a global variable and, if non-zero, performs conditional logic to update a global variable or modify a memory location based on a bit-shifted or offset value.
- `update_hardware_parameters`: Checks a global variable and, if non-zero, updates several hardware registers or memory locations with new values based on a lookup or calculation.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `load_address_to_a0`: Loads the address 61548 into register a0 and returns.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `process_string_decryption`: Iterates through a list of data structures to identify specific tagged entries, then performs a multi-pass decryption or transformation on a buffer using a lookup table and a running sum or offset.
- `expand_compressed_assets`: Iterates through a linked list of data structures to decompress or expand byte sequences into four destination buffers using a custom expansion logic that involves bit-shifting and index-based lookups.
- `return_constant`: Returns the constant 3000 in d0 to the caller without any other effect.
- `return_constant_2`: Returns the constant 100 in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant 0 in d0 to the caller without any other effect.
- `replicate_global_byte`: Returns a value formed by duplicating a byte from a global variable.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `copy_memory_block`: Copies a block of data from a source pointer to a destination pointer, checking for a null terminator.
- `call_indirect_function_2`: Calls another routine and returns the concatenated contents of two registers via a single indirect function call through a pointer at offset 256 of a structure in a6.
- `call_draw_and_move`: Calls another routine, then calls the graphics Draw and Move library functions through library pointers stored in a global variable.
- `call_indirect_through_table`: Calls another routine, then calls a routine loaded through a pointer read from a table at a fixed offset, and returns the values held in the data registers.
- `compare_and_clamp_values`: Compares four 16-bit values against specific constants and clears them if they exceed certain thresholds.
- `store_byte_and_call`: Stores a byte from a register into a global variable and calls another routine.
- `store_register_and_call_function`: Stores the contents of a data register into a global variable and then calls a function pointer retrieved from a table or offset.
- `update_global_with_conditional`: Checks if a register contains a specific value and, if so, assigns a different value to a global variable before calling another routine.
- `iterate_until_value`: Iterates through a routine or table, checking return or comparison values until a specific condition or non-zero value is met.
- `update_global_buffer`: Updates a global variable or a buffer-like structure with new values based on certain character checks and then calls a function pointer.
- `traverse_string_null`: Traverses a null-terminated string or buffer, calling another routine for every non-null character encountered.
- `calculate_scaled_offsets`: Clamps register values and stores the results multiplied by a constant into two global variables.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `update_position_limits`: Updates two global variables by adding or subtracting a value, then clamps them within a certain range, and finally clears certain bits in a third global variable.
- `scale_input_values`: Multiplies two input registers by two and stores the results in two global variables.
- `get_scaled_value`: Returns a global variable bit-shifted right by one.
- `compare_and_reset_table`: Iterates through a table to compare values, and if a certain condition is met, updates a global variable or resets a portion of a table to zero.
- `set_status_flags`: Calls a function pointer and then sets specific bits in a 16-bit value at a global variable based on the status of bits in another global variable.
- `check_joystick_bits_2`: Reads a byte from the CIAA port into register d2 and branches based on the state of register d0.
- `shift_joystick_data`: Reads a word from the joystick data register into d1, shifts the byte in d2 left by one bit, and branches to another location.
- `check_joystick_bits`: Reads a word from the joystick register at a fixed address into d1, then sets bit 2 or bit 0 of d0 based on the state of bit 1 or bit 0 of d1.
- `set_bits_from_flags`: Tests various bits in d1 and sets bit 2 or bit 0 of d0 based on the results of those tests.
- `set_bit_conditionally`: Tests bit 8 of d1 and, if set, checks if d2 is negative before setting bit 7 of d0.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `execute_permitted_function`: Checks if a global variable is greater than one, and if so, calls a function pointer located at an offset from that variable.
- `execute_indirect_function`: Checks if a global variable exceeds one, then executes a function pointer located at an offset relative to that variable.
- `execute_conditional_function`: Executes a function pointer from a table and ensures a return value is not zero.
- `clear_d0_register`: Clears the d0 register and returns.
- `execute_two_functions`: Calls a function pointer and, if the return value is non-zero, calls a second function pointer.
- `execute_indirect_function_loop`: Executes a function pointer and, if a global variable is non-zero, executes another function pointer before entering an infinite loop.
- `jump_to_address`: Jumps to the address stored in a long word.
- `execute_conditional_pointer`: Calls a routine and then calls a function pointer from a table if a certain value is not zero, returning a signed integer.
- `call_indirect_function_3`: Calls a function pointer located at an offset from a global variable.
- `call_external_function`: Calls a function pointer located at an offset from a value stored in a global variable.
- `ensure_minimum_value`: Checks if a global variable is less than two, sets it to one if so, and then calls a routine using a table-based function pointer.
- `process_disk_status`: Checks a byte at an offset in a structure, and if it is not greater than one, performs a sequence of function calls involving string traversals and memory operations, eventually checking a value against 304 before calling another routine.
- `move_and_clear_data`: Iteratively copies data from one set of pointers to a destination pointer while clearing the source memory locations to zero.
- `copy_memory_block_2`: Copies a block of memory from a source address to a destination address using a nested loop structure.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `process_message_queue`: Repeatedly calls a function pointer from a table to retrieve messages and processes them if specific bits are set in the returned data.
- `restore_registers_and_return`: Restores registers from the stack and returns.
- `execute_indirect_callback`: Calls a routine, then uses a pointer from a structure to call a function at a negative offset, and if the return value is not zero, stores it in a global variable and calls another function at a different negative offset from the same structure.
- `call_subroutines_and_branch`: Loads an address into a0, calls a routine at 5614, calls a routine at 5322, and then branches to 7604.
- `compare_and_set_d0`: Compares a byte in d0 with a value from a table, and if they are not equal and d0 is not negative, sets d0 to 65535.
- `execute_conditional_logic`: If d0 is not greater than 1, calls another routine and stores the result in a global variable; otherwise, it selects a pointer from a table based on d0, calls a routine using that pointer, and if the result is non-zero, stores it in a global variable and calls another routine using a pointer from that same global variable.
- `load_byte_to_d0`: Calls another routine and then loads a byte from a global variable into d0.
- `execute_multiple_functions`: Calls a function pointer, then calls that same function again with a register, then calls a different function pointer, and finally assigns a return value to a global variable.
- `perform_logical_or`: Performs a logical OR operation on the d0 register with zero twice.
- `contains_zero_data`: This routine consists of a series of instructions that perform a logical OR on the d0 register with zero and does nothing else.
- `calculate_scaled_value`: If d0 is less than 2, calls two routines and then enters a loop that calls two routines while a global variable is not negative; otherwise, it calls two routines, performs a calculation involving subtraction and multiplication using values from a table, and if the result is greater than a constant, performs a division and subtraction before calling another routine.
- `return_constant_4`: Returns the constant 100 in d0 to the caller without any other effect.
- `execute_two_functions_with_storage`: Calls a function pointer based on a value in d0, and if the result is non-zero, calls two more function pointers and stores the result in a global variable.
- `clear_d0_twice`: Performs an OR operation with zero on register d0 twice.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_d0_once`: Performs an OR operation with zero on register d0.
- `clear_d0_twice_alt`: Performs two consecutive OR operations with zero on register d0.
- `clear_d0_double`: Executes two OR operations with zero on register d0.
- `compare_and_branch`: Clears d0, saves registers to the stack, loads a longword from a table into d0, and compares it with a longword at a specific address; if the values are equal, it branches to 9492, otherwise it branches to 9356.
- `swap_and_initialize_params`: If d0 does not match a global variable, it pushes five values from a table onto the stack, clears the first two, writes three specific constants to the remaining three, calls another routine, restores the original values from the stack, swaps two global variables, and calls two other routines.
- `update_buffer_indices_recursively`: Recursively updates a pointer-based table and adjusts buffer start and end indices based on a certain size or offset.
- `update_table_values`: Calculates a value from a global variable and adds it to several consecutive entries in a table, then calls another routine.
- `clear_d0_five_times`: Performs five consecutive OR operations with zero on register d0.
- `clear_d0_single`: Performs a single OR operation with zero on register d0.
- `initialize_blit_parameters`: Sets several consecutive global bytes to a fixed block of values and calls a routine to configure a destination address and hardware registers.
- `allocate_and_initialize_table`: Calculates a size based on a word at offset 24 of a structure in a2, performs a multiplication using values from two global variables, calls a routine with the result, and if the result is zero, clears a byte at a global address; otherwise, it continues to a branch.
- `allocate_block_and_write_header`: Computes a row size from a structure field, derives several sequential offsets and sizes from global values, stores the input pointer in a global, writes a fixed word and two differences into the block header, and stores a size value.
- `clear_d0_minimal`: Performs one OR operation with zero on register d0.
- `clears_data_register_zero`: Performs a logical OR operation on the least significant byte of data register zero with zero.
- `clears_data_register_zero_2`: Performs a logical OR operation on the lowest byte of data register zero using a zero constant.
- `clears_data_register_zero_3`: Performs a logical OR operation on the least significant byte of data register zero using a zero constant.
- `perform_no_operation`: Performs a bitwise OR operation on register d0 with zero.
- `update_pointer_table_3`: Calculates a value based on d0, compares it to a global long word, and if it is not less than or equal, updates a global pointer with the new value, stores the value in the new pointer location, and iterates through a table to find a zero entry to store the pointer and its successor.
- `store_and_branch`: Subtracts 8 from a pointer in a0, stores the value of d0 at that pointer and its successor, then branches to another routine.
- `update_linked_list_node`: Clears two 32-bit words at the start of a structure pointed to by a register, then checks if a global pointer matches the address of the structure minus eight; if it matches, the global pointer is updated to the address stored within that structure.
- `compare_and_clear_d0`: Calls another routine, then compares the returned value in d0 against two thresholds.
- `return_constant_5`: Returns the constant -1 in d0 to the caller without any other effect.
- `save_pointer_and_call_routines`: Stores a longword from a0 into a global variable and calls six different routines in sequence before returning.
- `save_and_reset_hardware`: Copies several hardware register values into saved global variables, then writes constants to several hardware registers and resets certain globals.
- `clear_bits_and_wait`: Clears bits 7 and 3 of a hardware register, busy-loops a 100-iteration counter while repeatedly clearing bit 7 of that same register, and finally calls two other routines.
- `run_decrement_loop`: Decrements a counter from 3000 down to zero using a loop.
- `wait_for_button_release`: Loops until bit 5 of a CIA A register is cleared.
- `toggle_bit_loop`: Checks bit 4 of a hardware register and, if set, enters a loop that toggles bit 1 and bit 0 of a hardware register while calling another routine.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `poll_hardware_bit`: Checks bit 4 of the CIAA_PRA register and, if set, calls another routine and then loops back to the start of this routine.
- `loop_with_delay`: Runs a loop 79 times where it clears bits 1 and 0 of a hardware register and sets bit 0 before calling another routine.
- `run_delay_loop`: Decrements a counter from 15000 down to zero using a loop.
- `pulse_serial_port_until_ready`: Calls a routine, clears bit 2 of a global variable at a fixed address, writes a word to offset 150 of a structure in a6, loads a longword from a global pointer into offset 32 of a structure in a6, clears offset 36 of that structure, calls another routine, and then polls a status register in a loop for a specific count or until a bit in a global variable changes.
- `clear_and_set_memory`: Clears a 16-bit word at an offset of 36 from a pointer in a6 and writes the value 16 to an offset of 150 from the same pointer.
- `wait_for_status_bit_set`: Spins until bit four of a global status byte is set, where the byte mirrors the CIA B interrupt control status.
- `latch_status_into_four_words`: Clears bit 3 of a global status byte, sets bit 7, and writes the result ORed with the top bit into four 16-bit slots of the local frame.
- `search_bit_pattern`: Iterates through a table of longwords, performing a bitwise shift and comparison on each, and if a specific pattern is found, enters a loop calling another routine while a global byte is non-zero; otherwise, it sets a global byte to one.
- `loop_while_global_nonzero`: Calls a routine in a loop while the byte at a global address is non-zero.
- `search_and_write_word`: Iterates through a list of words pointed to by a0, shifting each word left by one bit in a loop, and if a word's high byte matches a constant, it writes a counter value to a location in a4 and clears a byte in a global variable; otherwise, it clears a byte in a global variable and returns.
- `write_word_conditionally`: Compares a0 to a constant and, if not equal, stores a word from d5 at the address in a4 and writes a byte to a global variable.
- `dispatch_conditional_branch`: Saves registers to the stack, tests the word in d0, and if it is positive branches to a location, otherwise it calls a subroutine and then performs a branch.
- `check_and_dispatch_type`: Loads a pointer from an offset of 4 in a structure in a6 into a5, compares the word at the offset of 4 in a5 to 21332, and if they are equal, calls another routine and branches to a different location; otherwise it branches to another location.
- `dispatch_data_structure_type`: Checks a global flag and calls another routine, then branches based on a value in d0.
- `restore_registers_and_return_2`: Restores registers from the stack, sets the d0 register to one, and returns.
- `update_pointer_table_2`: Copies a word from a table, updates a pointer in a table with a new address, and clears a word in another table.
- `initialize_memory_blocks`: Zeros out two large memory blocks, copies a specific amount of data from one location to another, and updates several global pointers.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `copy_word_block`: Copies a sequence of 32 words from a source buffer pointed to by a5 plus an offset of 6 to a destination buffer starting at a specific memory location, then branches to another routine.
- `initialize_global_pointers`: Stores a sequence of values from a series of global variables into a block of memory starting at a pointer.
- `update_data_structure`: Increments a word at offset 8 of a structure if it does not match the word at offset 10, then enters a loop that processes bytes from a source, performing sign extension and conditional memory copies based on the byte value.
- `copy_bytes_in_loop`: Decrements a counter, copies a byte from a source to a destination, and loops if the counter is not zero.
- `copy_bytes_with_negation`: Negates a longword, decrements it, copies a byte from a source to a destination, and loops if the counter is not zero.
- `swap_and_call_routine`: Swaps two longwords stored at specific locations and calls another routine before branching to a different location.
- `populate_pointer_sequence`: Calculates a series of values by adding a constant to an initial value and stores the high and low words of these values into a memory block.
- `test_status_bytes`: Tests two bytes at specific locations and branches based on their values; otherwise it returns.
- `identify_audio_channel`: Compares a pointer against four specific addresses and sets register d3 to a corresponding value, otherwise it branches to another routine.
- `configure_audio_channel`: Stores a byte from a global variable through an address register at a fixed offset, then calls another routine and returns.
- `set_hardware_registers`: Updates hardware registers and a global variable, while using a library call and modifying a table or register or bitmask.
- `update_bitmask_and_enable_dma`: Checks a condition using a global variable, clears a flag in a table, and sets a bitmask in a global variable while updating hardware registers.
- `decrement_global_timer`: Checks a bitmask within a global variable and recursively calls itself if certain bits are set.
- `update_audio_hardware`: Decrements a value in a table, and if it reaches zero, clears several fields in a structure, updates hardware registers, and sets a bitmask.
- `set_register_from_table`: Writes a word from a table to a hardware register, or writes zero if a global variable is zero.
- `execute_sequential_routines`: Calls another routine four times in succession with varying word values in a data register and different addresses in an address register.
- `return_concatenated_registers_2`: Calls another routine and returns a 64-bit value formed by concatenating the contents of two registers.
- `reset_global_variable`: Sets a global variable to a constant value if a certain condition is met, otherwise calls another routine.
- `cast_global_to_unsigned`: Performs a redundant type cast or assignment of a global variable to itself.
- `scale_global_variable`: Multiplies a global variable by a constant and right-shifts the result.
- `clear_audio_volumes`: Tests a byte at an offset of a global variable, and if non-zero enters a routine that saves registers, tests another byte, calls a subroutine, restores registers, clears d0, and writes zero to four audio volume hardware registers at offsets 168, 184, 200, and 216 from the base address in a0.
- `do_nothing`: Performs an operation that results in no change to the d0 register.
- `clears_data_register_zero_6`: Performs a logical OR operation on register d0 using zero as the operand.
- `clear_d0_register_3`: Clears the least significant byte of register d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 117 into d0.
- `reset_audio_and_memory`: Zeroes a table of longwords, updates several global variables, and clears audio volume registers and other hardware-related memory locations.
- `silence_audio_channels_and_reset`: Zeroes the volume registers of all four audio channels, writes a fixed value into one global, and clears a longword table pointer and a flag byte.
- `update_channel_volumes_from_envelope`: Decrements a timer and derives a volume scale from a ratio or a second timer, then writes scaled volume levels to audio hardware registers for certain channels.
- `update_audio_parameters`: Calls a subroutine, tests a byte at an offset of a global variable, and if non-zero enters a routine that manipulates a structure in a0, performs several calls to subroutines with audio hardware register addresses in a5, modifies hardware registers and a global table, and iterates through a table using a counter until a condition is met.
- `set_audio_registers`: Stores a specific value and a memory address into a global variable.
- `load_audio_channel_registers`: For each audio channel whose flag is zero, copies a saved pointer and length into that channel's location and length registers.
- `update_dma_control_parameters_2`: Retrieves a pointer from a table using a value derived from two bytes in a structure, then updates several fields in that structure including a calculated value at offset 8, a value at offset 10, and a value at offset 14 based on arithmetic performed on data within the pointed-to structure.
- `update_dma_control_parameters`: Calculates a value based on the difference between two words in a structure, stores it at offset 14, and performs a multiplication to update a 16-bit value at offset 8 of another structure.
- `write_dma_control`: Writes a word from an offset of a structure into the DMA control register.
- `update_status_flag`: Masks a word from a structure, stores it at offset 22, and compares it with a word at offset 16, potentially setting a byte at offset 20 based on the comparison result.
- `clear_word_at_offset`: Clears a 16-bit word at offset 22 of the structure pointed to by a6 and returns.
- `update_byte_field`: If a byte at offset 3 of a structure in a4 is non-zero, it copies that byte to offset 21 and sets offset 3 to zero; otherwise it returns.
- `update_structure_counter`: Compares a word at offset 22 of a structure in a4 with a word at offset 16 of the same structure; if the value at offset 16 is less than the value at offset 22, it adds a byte from offset 21 to offset 16, or if offset 20 is non-zero, it subtracts that byte from offset 16.
- `update_structure_field`: Compares a word at offset 16 of a structure in a4 with a word at offset 10 of the same structure, and if the value at offset 16 is less than the value at offset 10, it copies the value from offset 16 to offset 10, clears the word at offset 16, and then checks a byte at offset -2 of the structure to decide whether to move a word from offset 6 of a5 to another location.
- `store_byte_conditionally`: Moves a byte from offset 3 of a structure in a4 into register d0, and if d0 is not zero, stores it into offset 24 of the same structure.
- `calculate_scaled_period`: Calculates a value by fetching a byte from a table using a shifted and masked byte from offset 25 of a4, multiplying it by a masked byte from offset 24 of a4, and then adding or subtracting this result from a word at offset 16 of a4 based on the sign of the byte at offset 25.
- `set_period_from_difference`: Writes the difference between two supplied words to the period register when its flag word is zero, then advances the phase field.
- `fetch_table_value`: Loads a byte from a table using an offset derived from a byte at a specific offset, and then branches based on whether that byte is zero or two.
- `walk_period_table_to_register`: Walks a table until a value is not greater than the period, then writes an entry selected by a parameter nibble to the period register when the flag word is unable to be written.
- `write_base_period_if_unflagged`: Writes the base period to the period register when the channel's flag word is zero.
- `branch_on_audio_command`: Tests a byte at offset 2 of a structure in a4 and returns if the byte is zero, otherwise it branches to another routine.
- `dispatch_audio_parameter_updates`: Reads a command byte from offset 2 of a structure in a4 and branches to specific subroutines based on its lower nibble.
- `lower_period_with_floor`: Subtracts a parameter byte from the channel's period, clamps the result to a floor of 113, and writes it to the period register when the flag word is zero.
- `raise_period_with_ceiling`: Adds a parameter byte to the channel's period, clamps the result to a ceiling of 856, and writes it to the period register when the flag word is zero.
- `call_subroutine_and_branch`: Calls a routine and then branches to another location.
- `adjust_volume_level`: Calls another routine, then either adds a value to offset 19 of a structure and performs a clamped multiplication to update offset 8 of a structure in a5, or subtracts a value from offset 19 and performs a similar calculation.
- `decrease_channel_volume_level`: Subtracts a low-nibble parameter from a volume level, clamps the result at zero, and writes a scaled volume to a global variable if a flag is zero.
- `dispatch_command_byte`: Reads a byte from offset 2 of a structure, masks it, and branches to different routines based on its value; otherwise it returns.
- `update_status_bit_from_input`: Reads the least significant bit of a byte from a memory location and sets bit one of a global variable to match it, while preserving all other bits in the global variable.
- `set_byte_at_offset`: Writes the value one to a byte at offset 499 of a structure pointed to by a6.
- `set_flag_and_countdown_byte`: Sets a flag byte in a channel record to one and stores one less than a parameter byte in a companion countdown byte.
- `set_channel_volume_clamped`: Clamps a parameter byte to a maximum of 64, stores it as the channel's level, and writes a scaled volume to a global variable when the flag word is zero.
- `set_scaled_level_byte`: Caps a parameter byte at 31, multiplies it by a global scale, shifts the product right by five bits, and stores the result in a global byte.
- `jump_to_indirect_address`: Jumps to the address stored at the location specified by the long word at the provided pointer.
- `process_event_list`: Initializes several global pointers and bytes, calls three routines, and then iterates through a linked list to update structure members and perform conditional routine calls based on byte comparisons.
- `execute_jump_table`: Pushes an address onto the stack, reads a byte from a pointer in a3, doubles it, and uses the result to index into a table in a2 to perform an indirect jump.
- `process_input_character`: Checks a global state byte and, if non-zero, evaluates the character in register d0 against specific ASCII values.
- `iterate_string_characters`: Loads a pointer from a table into a0, calls a routine to advance the pointer, and then iterates through the string, calling a routine to process each character or integer-string representation until a null terminator is encountered.
- `convert_integer_to_string`: Converts a signed integer into a sequence of characters, including a negative sign if necessary, and stores the result in a buffer.
- `clear_long_word`: Clears a long word at a specific memory location.
- `process_and_update_linked_list`: Traverses a linked list by reading word-sized elements from a source buffer, updating several global offset variables by adding or subtracting values from the buffer data, and recursively calls itself to process subsequent nodes.
- `compare_table_entry_2`: Copies a word from d1 to d2, fetches a word from a table using d1 as an index, and branches if the result is zero.
- `compare_table_entry`: Compares a value from a table against a global variable and checks if another table entry matches a specific value.
- `search_linked_list`: Traverses a linked list to check if any node contains a value equal to a global variable and a specific short value.
- `insert_into_list_structure`: Swaps or inserts a value into a linked list structure using a global index, while updating pointers and clearing certain memory offsets.
- `calculate_scaled_delta_values`: Checks a bit in a structure, then calculates the difference between two sets of three 16-bit values, scales them using a bit-shift amount and a divisor, and stores the resulting values and a pointer into several global variables.
- `calculate_weighted_sum`: Checks a byte at offset 1 of a structure, then performs a series of multiplications and additions using values from d1, d2, and d3 with bytes from the structure to compute a result in d4 and d5.
- `test_global_byte`: Tests a byte in a global variable and branches to another location if it is non-zero.
- `traverse_indexed_table`: Calculates an offset into a table using a value from a structure and a comparison, then returns a pointer to a specific entry.
- `clear_flag_and_traverse_list`: Loads a pointer from a global variable into a1, then loops while a 16-bit value read from an offset of 24 bytes from the stack frame is non-zero, calling traverse_and_update_linked_list on each pass; if the value is zero, it clears a global byte and returns.
- `traverse_and_update_linked_list`: Recursively traverses a linked list, updating or replacing node pointers based on a condition checked against a byte in the node.
- `update_linked_list_node_2`: Updates a node in a linked list or replaces a node with another if certain conditions are met.
- `copy_memory_to_globals`: Copies several sequential words from a memory location into a series of global variables.
- `update_entity_coordinates_and_wrap`: Wraps three 16-bit values at offsets 12, 16, and 20 of a structure within a range of 360, then iterates through a linked list to update a state byte and apply coordinate displacements to each node.
- `update_view_bounds`: Checks if a flag or certain conditions are met, then updates several global or pointer-referenced boundary or position values by adding a value from a pointer-referenced table and setting a flag.
- `update_list_values`: Adds a value to certain memory offsets and traverses a linked list, setting a byte to a constant value if certain conditions are met.
- `update_entity_state_and_linked_list`: Updates a global pointer from a source address, increments a counter, and manages a state machine by modifying bytes in a global structure.
- `update_node_properties_and_bounds`: Traverses a linked list of structures to update their spatial properties, including coordinate offsets, dimensions, and padding, while conditionally calling other routines to handle clipping, coordinate clamping, and blitter window updates.
- `insert_node_into_table`: Calls two other routines, then writes a 16-bit word from a source offset to a destination offset, followed by several 32-bit, 16-bit, and 8-bit values from a table into a structure in a1.
- `copy_table_entry`: Calls another routine, copies a 16-bit value from one location to another, and updates a global variable with a previously held value.
- `update_node_properties`: Calls another routine, then compares a value at an offset of a structure in a1 with d4, and if they match, checks a byte at offset 0 of the structure.
- `initialize_global_bounds`: Sets two global variables to a high value and two other global variables to a low value.
- `expand_global_bounds`: Checks a range of values from a table and expands the minimum or maximum allowed global boundary variables if the new values exceed the current bounds.
- `update_linked_list`: Traverses a linked list and replaces a node or updates a pointer to a new node if certain comparison conditions or certain value checks are met.
- `process_tiled_object_list`: Iterates through a linked list of structures from a global pointer, checking bit flags and coordinate bounds for each entry.
- `initialize_buffer_constants`: Copies a sequence of four word-size values from one memory location to another and calls a routine if a pointer exceeds a specific value.
- `copy_table_block`: Iterates through a table of 16-bit values, copying them into a set of global variables, and calls another routine during each iteration.
- `clamp_and_set_view_rect`: Checks if a certain area or object overlaps with current boundaries, then clamps the coordinates to a certain range and stores the new dimensions or bounds into global variables.
- `update_bounds_with_padding`: Verifies if an object or area or even certain bounds or padding, and if so, updates global boundary variables with or without padding.
- `recalculate_view_parameters`: Updates several global boundary variables based on a provided object or area, and calculates a new offset or index for a certain memory-mapped or calculated value.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_table_offsets`: Calculates two memory addresses by scaling a byte from a global variable and adding it to two different base addresses, then stores these addresses in global variables.
- `initialize_and_copy_data`: Sets a byte at a global address to 255, clears two consecutive bytes in a global variable, and then calls another routine to perform a loop that copies words from a source pointed to by a1 to a destination pointed to by a2.
- `convert_color_values`: Iterates through a list of values, calculating new components based on a table of multipliers and bitwise operations, then stores the result in a destination buffer.
- `check_nibble_sum`: Compares the sum of two lower nibbles against a constant value and returns if the sum exceeds that value.
- `check_nibble_difference`: Calculates the difference between two lower nibbles and returns if the result is negative or outside a certain range.
- `reconstruct_nibble_data`: Iterates through two buffers of 64 words, comparing each word; if a mismatch is found, it compares individual nibbles and calls itself recursively to resolve differences before packing the corrected nibbles into the destination word.
- `check_nibble_sum_2`: Loads the value 1 into register d0 and calls another routine.
- `prepare_nibble_comparison`: Sets a 16-bit constant of one in a data register and calls check_nibble_difference, then returns.
- `execute_jump_table_entry`: Loads a byte from a pointer in a3, uses it as an index into a table pointed to by a5, and jumps to the resulting address.
- `update_system_state`: Performs multiple nested routine calls, clears several global variables and bytes, and updates a global pointer with a value derived from a calculation involving a multiplication and addition.
- `initialize_data_structure`: Initializes a series of global variables and populates a linked list or table with incrementing or constant values.
- `process_string_and_update_state`: Copies a string from a source pointer to a destination buffer, replacing the first period character with a specific byte if found.
- `update_global_byte`: Sets a global variable to 1, checks if its high nibble is 160, and if so, performs conditional calls and loops that modify memory via pointer-based checks and decrement counters.
- `execute_external_loop_2`: Compares a long word in a3 with a4, and if a4 is greater than or equal to a3, it loads a pointer from a table into a1, sets d1 to 7936, calls another routine, and loops back to reload the pointer from the table into a3.
- `write_byte_with_bounds_check`: Writes a byte to a destination if a calculated address range check passes.
- `expand_and_copy_byte_sequences`: Loads a data table into memory and iterates through a destination buffer to write bytes based on values read from the table.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `process_external_loop`: Saves registers to the stack, loads a pointer from a table into a1, sets d1 to 7936, and calls another routine before reloading a0 from the table and looping back to the start.
- `transfer_registers_via_branches`: Performs two short branches to other routines before restoring registers and returning.
- `search_sorted_table`: Performs a binary search through a table of word-sized values to find a match for a given value.
- `initialize_list_structure`: Iterates through a memory range, writing incrementing offsets and zero-values to a list or buffer.
- `manage_event_list_and_dispatch_2`: Updates a linked list of event structures by shifting elements to remove an entry, increments a counter in a global variable, and calls several subroutines based on a status byte.
- `manage_event_list_and_dispatch`: Traverses a linked list of event structures, clearing flags and updating pointers, then performs memory shifts to remove elements and calls dispatch_event_data to execute commands based on the event data.
- `dispatch_event_data_2`: Stores a pointer from a0 into a structure, clears a byte at an offset in that structure, and writes a word from d0 into another offset.
- `execute_conditional_function_2`: Calls a function pointer from a global variable and, if the return value is non-zero, calls a second function pointer.
- `execute_looping_pointer`: Loads a long word from a global variable, calls the routine at that address, and if the result is negative, calls another routine with the value 14 and loops back to the start.
- `store_extended_pointer`: Extends the sign of d0, calls a routine, stores the result in a global variable, and if the result is negative, calls another routine with the value 8 and loops back to the start.
- `execute_indirect_call_and_store_result`: Calls a routine via a pointer, stores the returned 32-bit value into a global variable, and then enters a loop that repeatedly calls another routine until a negative result is returned.
- `execute_external_loop`: Loads a long word from a global variable, calls a routine using a pointer from a table, and if the result is negative, calls another routine with the value 13 and loops back to the start.
- `update_global_and_branch`: Increments a 32-bit value and a byte in a structure, then conditionally calls a routine or resets a byte based on a value.
- `update_global_and_copy`: Checks a byte in a structure, calls another routine, and if a byte in d2 is non-zero, stores it in a structure and decrements a counter in a structure in a loop.
- `decrement_and_clamp_summed_values`: Decrements a 16-bit value at offset 4 of a structure, then calculates the sum of values at offsets 6 and 8; if the sum is positive, it clamps the result to 32767, updates offset 6, and adds a 16-bit value from offset 12 to offset 10 before calling another routine; otherwise, it clears offsets 0, 2, 6, and 10, sets offset 1 to 128, and calls another routine.
- `reset_multiple_memory_blocks`: Calls another routine four times, each time passing a different address loaded into register a3.
- `reset_memory_block`: Clears or initializes a specific block of memory by writing zero or a specific constant value to multiple offsets from a pointer.
- `dispatch_event_data`: Stores four values into a structure, swaps two global pointers, and executes a sequence of subroutines based on a command ID. It performs conditional logic and string-based lookups depending on the value of the command ID.
- `execute_multiple_subroutines`: Extends a word from a table into a longword, calls a subroutine, saves registers, calls another subroutine with a constant pointer in a0, restores registers, extends a word from a different table offset into a longword, calls a subroutine, loads a pointer from a table into a6, and jumps to another routine.
- `initialize_and_jump`: Performs four calls to other routines using values from a structure in a6 and specific pointers, writes three words to a global table, and then calls two more routines before jumping to an indirect address.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `call_and_test_d0`: Saves registers to the stack, loads a pointer from a table into a0, calls a subroutine, tests the word in d0, and returns if d0 is non-zero, otherwise it restores registers and returns.
- `increment_global_pointer`: Adds a large constant to a global long value and restores registers.
- `return_concatenated_registers`: Loads a pointer from a global variable, performs several word-sized moves from memory into registers, and calls another routine.
- `clip_and_blit_masked_tile`: Loads a pointer from a global variable, sets four registers to specific constant values, and calls another routine.
- `update_bitmask_and_shift_data`: Checks boundary conditions against several values, clears specific bits in a global structure, and performs a bit-slicing operation on a block of data by masking and shifting words across multiple destination buffers in a loop.
- `copy_memory_blocks`: Copies a sequence of 2-byte blocks from a source address to a destination address calculated using a table, where the number of blocks and the source offset are determined by values in a structure.
- `perform_logical_or_2`: Performs a bitwise logical OR on register d0 with zero.
- `setup_blt_params`: Calculates destination addresses and configures Blitter hardware registers based on values in a table, while waiting for a bit in a global variable to clear.
- `update_blt_attributes`: Iteratively updates a global or a pointer-based address or a value by adding or subtracting an offset or a certain constant, while waiting for a certain bit in a global variable to clear.
- `copy_block_tiled_pattern`: Checks if a value derived from two global variables is within a specific range, otherwise it calls another routine.
- `execute_blitter_clipping_and_transfer`: Calculates drawing boundaries and source/destination coordinates based on a structure, configures the blitter registers with calculated widths, heights, and address offsets, and then enables the DMA controller to perform the transfer.
- `configure_blt_transfer`: Checks bits in a structure, configures Blitter registers with values from a register and a table, and updates a pointer in a table.
- `calculate_blt_window_bounds`: Calculates the visible clipping or drawing boundaries by comparing certain pointer-based structures against limits and then populates a table or buffer with visibility or masking data using a loop that writes or copies values from an indexed source or table.
- `clears_data_register_zero_4`: Performs a logical OR operation on the lowest byte of data register zero with zero.
- `clip_and_update_blitter_window`: Calculates the intersection between a source rectangle and a clipping boundary, updates a global window coordinate structure with the resulting dimensions, and branches to different routines based on a type byte found in the source structure.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `zero_data_block`: This area contains a single instruction that performs a logical OR operation with zero on a data register.
- `clear_d0_register_2`: Performs a logical OR operation on the least significant byte of the d0 register using a zero constant.
- `clears_data_register_zero_5`: Sets the least significant byte of data register zero to zero.
- `clears_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `clear_data_register_zero_2`: Performs a bitwise OR operation on data register zero with zero.
- `initialize_engine_pointers`: Saves registers to the stack, tests a longword at a global address, and if non-zero branches, otherwise it calls a subroutine, stores the result in several global pointers, calculates two additional pointers by adding constants to the first, sets a byte in a global variable, clears a byte at an offset of a1, and calls another subroutine.
- `update_geometry_and_handle_allocations`: Extracts and stores width and height values from a structure into a series of global variables, potentially initializing a new memory block via a call if a global pointer is null.
- `update_structure_field_and_branch`: Copies a longword from offset 36 of the structure pointed to by a1 to offset 4 of the same structure, writes the constant 32772 to offset 40 of the structure, and branches into another routine.
- `clear_word_and_restore_registers`: Clears a 16-bit word at offset 40 of the structure pointed to by a1, restores d0 through d7 and a0 through a6 from the stack, and returns.
- `reset_global_state`: Saves registers, sets d0 to a constant, calls a subroutine, tests a global longword, and if non-zero calls another subroutine, clears the global longword and a global byte, then restores registers and returns.
- `parse_data_structure_table`: Iterates through a table of data structures, checking for specific marker values at fixed offsets.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Ishar 3 - The Seven Gates of Infinity (demo) (1994)(Silmarils).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stack_push_d0_with_ori_pattern_data`: Repeatedly pushes d0 onto the stack via a1 and ORs the immediate value 50 into d0; the long run of identical pairs indicates data decoded as code, not an executable routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_and_modify_memory`: Branches if the result of a comparison is positive, otherwise it writes a byte to a location pointed to by a3 and increments the pointer, then performs a test and branch based on the status register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_d2_and_branch`: Adds four to register d2 and performs a relative branch to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_audio_volumes`: Checks two global bytes; if both are non-zero, it calls another routine, otherwise it clears four 16-bit audio volume registers at offset 168, 184, 200, and 216 of the hardware base address.
