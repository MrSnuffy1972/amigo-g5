# Game 1267 (robosport (1992)(ocean)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01267). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 83 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 101 of 101 named (0 author symbols), 101 of 101 described (14 by rule, 86 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 84 of 84 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1966 of 1966 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 496836 |
| code | 5610 |
| data | 369438 |
| padding | 18996 |
| unused | 10240 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Assign | 1 | 938 | 7 |
| RoboSport (1992)(Ocean)[cr].adf:RoboSport.BIN | 2 | 232 | 8 |
| MathTrans.Library | 1 | 91 | 12 |
| ASL.Library | 2 | 104 | 10 |
| IFF.Library | 1 | 217 | 17 |
| MathIEEESingTrans.Library | 1 | 46 | 7 |
| MathIEEESingBas.Library | 4 | 167 | 16 |
| MathIEEEDoubBas.Library | 1 | 108 | 17 |
| MathIEEEDoubTrans.Library | 1 | 45 | 6 |
| RoboSport (1992)(Ocean)[cr].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- robosport (1992)(ocean)[cr]/RoboSport (1992)(Ocean)[cr].adf:RoboSport.BIN: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Assign

- `parse_assign_command`: Parses a command-line assignment string of the form 'Name:' and validates the device name and options, then prepares internal state for adding, subtracting, deferring or setting paths, returning an error code when the input is invalid.
- `check_volume_mount`: Checks a volume reference stored in a global table and tests whether the volume is mounted.
- `copy_bytes_to_buffer`: Checks the length field of a buffer descriptor and, when the destination is too small, allocates a new buffer through a global routine pointer.
- `process_assignment_queue`: Walks a linked list pointed to by the first argument, invoking a handler stored at a fixed offset from the node and then a second handler in a loop controlled by a counter field.
- `assign_object_to_slot`: Attempts to assign an object identifier to a slot list, checking for duplicates and capacity using a global variable and another routine.
- `copy_name_to_buffer`: It null-terminates the destination and returns zero, or points to a global variable when the source is too long.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by two registers character by character.

### RoboSport (1992)(Ocean)[cr].adf:RoboSport.BIN

- `process_amiga_hunk_relocations`: Parses a data stream containing AmigaDOS hunk types to perform memory allocation and relocation.
- `decrement_and_call_library`: Decrements a value in d1 and, if it is not equal to 1010, performs a series of pointer manipulations and copies four words from a source to a destination before calling a library routine at a negative offset from a6.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_stream_data`: Reads a longword from a source pointed to by a0, performs bitwise operations and calls another routine, then enters a loop that writes bytes to a destination pointed to by a3 based on a count derived from the input data.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_stream`: Performs a loop that shifts and rotates bits from a source pointed to by a0 into a destination, while updating a hardware color register and a pointer in a3.
- `perform_arithmetic_operations`: Moves a word from an offset of a pointer in a2 into d4 and performs a logical OR of d2 into d0.

### MathTrans.Library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_library_structure`: As library code, it clears a 16-bit word at offset 32 of a structure in a5, then copies the values from a6 and a0 into offsets 34 and 38 of that same structure.
- `increment_counter_and_clear_flag`: As library code, the routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_library_counter`: As library code, it decrements a 16-bit word at offset 32 of a structure in a6 and, if the result is zero and bit 3 of a byte at offset 42 is set, calls another routine.
- `update_library_status`: As library code, it copies a pointer from a structure in a5 to a6, checks a word at offset 32 of that structure, and if non-zero sets bit 3 of a byte at offset 42 of that structure before branching.
- `execute_library_callbacks`: As library code, it calls a routine at an offset from a6 using a pointer from a5, then calls another routine at a different offset from a6 using a pointer derived from a5 and a value at offset 16 of a5.
- `transform_integer_data`: As library code, it performs a series of arithmetic and bitwise operations on d0, including a left shift, an XOR, a right shift, a subtraction, a swap, and a left rotation, before returning.
- `transform_word_bits`: This library code performs a series of bitwise operations on a 16-bit value in d0, including a swap, a rotate left, an eor, an addition, and conditional additions and rotations based on the overflow flag.
- `compare_and_rotate_value`: Compares d0 to the constant 124, and if it matches, adds 133 to d0 and performs a left rotation of 1 bit, otherwise it returns.
- `test_and_return_zero`: As library code, it performs a bitwise AND on d0 with a constant, and if the result is non-zero it sets d0 to zero and returns; otherwise it branches to another routine.
- `shift_conditional_value`: As library code, it compares d0 to a constant, and if it matches it shifts d0 right by 9 bits using d1 as a count before returning; otherwise it sets d0 to zero and returns.
- `manipulate_word_bits`: As library code, it shifts d0 left by 8 bits, fills the upper byte with ones, performs a rotate right through the carry, and sets bit 1 of the status register.

### ASL.Library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clears_byte_and_register`: Clears a byte in a memory location pointed to by a6 using the value in d0, then clears d4.
- `update_callback_data`: This library routine updates several fields in a data structure using values from registers and results from other routines.
- `execute_indirect_call_2`: As library code, it loads a value into d0, calls a routine at an offset of negative 552 from a6, and then jumps to an address popped from the stack if d0 is non-zero.
- `execute_indirect_call`: As library code, it calls a routine located at an offset of negative 108 from the address in a6, then restores d7, a0, and a5 from the stack and returns zero.
- `execute_library_vectors`: Loads several pointers from a structure in a5 into a1 and calls a library routine at a negative offset in a6 for each, finally jumping to a pointer at offset 72 of the structure.
- `transfer_pointer_to_register`: As library code, it copies the value from a0 into d1 and branches to another routine.
- `fill_buffer_loop`: As library code, it copies bytes from a source pointed to by a1 to a destination pointed to by a0 in a loop controlled by the count in d0 until the count reaches zero or a zero byte is encountered.
- `zero_fill_buffer`: Clears bytes in a buffer pointed to by a0 using the count in d0 and returns.
- `copy_c_string`: Advances to the terminating zero of the string at a0, then copies the bytes from a1 onto its end, including the zero.

### IFF.Library

- `open_library_call_vectors_close`: Opens a library through the exec open routine, calls two of its vectors with the result, then closes the exec library and returns the constant ten.
- `init_library_fields_and_list`: Stores the base and argument pointers in the library structure, opens the dos library through a library vector and records the result, then sets up an empty list header inside the structure.
- `update_iff_node_list`: As library code, it calls another routine, and if the zero flag is set, calls a routine at an offset of negative 504 from a6, then uses the result to update a field in a structure and increments a counter at offset 50 of the library base.
- `set_zero_and_branch`: As library code, it sets d0 to zero and branches to another routine.
- `initialize_iff_header_and_open_file`: Checks if a value in d0 is non-zero; if so, it initializes an IFF header by writing a constant to the location pointed to by d0, sets a second value at an offset of 4 from that location by calling a routine, and then attempts to open a file via a library call, returning a specific error code if the file cannot be opened.
- `initialize_iff_header`: This library code stores a constant in the location pointed to by d0, sets a value at an offset of 4 from that location, and calls a routine at an offset of negative 30 from a6.
- `compare_and_branch_d0`: This library code moves d0 to d1, loads a pointer into d2, calls a routine at an offset of negative 48 from a6, and compares the result in d3 to d0, branching based on the outcome.
- `transfer_register_to_d0`: This library code calls another routine, moves the value from a2 into d0, and then branches to another location.
- `open_dos_file`: Library code that calls a routine at offset -30 of a6, and if the result in d0 is zero, sets d5 to 16 and branches to another routine; otherwise it branches to another routine.
- `handle_iff_structure`: Library code that calls a routine at offset -42 of a6, compares the result to d0, and if equal, performs several library calls including one at offset -408 of a6 and another at offset -36 of a6 using a stack-allocated structure.
- `handle_structure_update`: This library code checks if d6 is positive, compares a value on the stack to a constant, and if it matches, calls a routine at an offset of negative 198 from a6 before branching based on the result.
- `process_and_call_io`: This library code stores d3 at the location pointed to by a2, decrements d3, and if a4 is not zero, pushes several values onto the stack and calls a routine at an offset of negative 48 from a6.
- `execute_io_operations`: This library code calls a routine at an offset of negative 42 from a6, compares the result to d3, and if they are not equal, calls another routine at an offset of negative 36 from a6.
- `call_and_restore_registers`: As library code, it calls another routine, moves the value from a2 into d0, and restores several registers from the stack before returning.
- `find_node_by_matching_id`: Walks a linked list from the pointer at offset 46 of the library base and returns the first node whose field at offset 14 equals the value at offset 276 of the referenced structure.
- `clear_field_return_one`: Calls one routine and, when its test result is not zero, clears a 32-bit field at offset 18 of the structure.
- `store_field_return_zero`: Calls one routine, then stores its result into a 32-bit field at offset 18 of the structure when that value is non-zero.

### MathIEEESingTrans.Library

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_math_structure`: Clears a word at offset 32 of a structure in a5, then stores the library base in a6 at offset 34 and a pointer from a0 at offset 38 of that structure, before checking bits in a byte at offset 329 of a6.
- `increment_counter_and_clear_flag_2`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_and_conditionally_call_handler`: Decrements a 16-bit value at offset 32 of a structure and, if the result is zero and bit 3 of a byte at offset 42 is set, calls another routine before returning.
- `set_library_flag`: Saves registers to the stack, moves a pointer from a structure into a register, and if a word at offset 32 of that structure is non-zero, sets bit 3 of a byte at offset 42 of the same structure before returning.
- `process_math_data`: Calls a library routine using a pointer from offset 38 of a5, then calculates a new offset by subtracting a word from offset 16 and adding a word from offset 18 to a pointer in a1, before calling another library routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### MathIEEESingBas.Library

- `return_constant_3`: Returns the constant -1 in d0 to the caller without any other effect.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `initialize_library_context_2`: Library code that tests bit 4 of a byte at offset 297 of a structure in a6, calls another routine via a pointer at offset -498 of a6, and if the result is non-zero, loads a long value from offset 16 of the returned pointer into d2 before loading a new pointer into a0.
- `initialize_library_context`: Library code that calls a routine at offset -84 of a6, populates a structure in a2 with various pointers and values, and potentially calls a routine at an offset found in a structure in d3 before calling another routine at offset -396 of a6.
- `raise_ieee_exception_code`: Stores two into a 16-bit field of the structure at A0, then, depending on the low byte of the exception code in D7, deliberately divides by zero or raises the overflow trap.
- `check_threshold_value`: This library code compares a 16-bit value in d7 to the constant 7168 and branches to another routine if it is greater than or equal to that value, otherwise it returns.
- `compare_exception_code_2`: This library code compares a 16-bit value in d7 to the constant 7680 and branches to another routine if it is greater than or equal to that value, otherwise it branches to another location.
- `compare_exception_code`: As library code, it compares a 16-bit value in d7 to the constant 23552 and branches to another routine if it is greater than or equal to that value.
- `branch_on_exception_code`: Compares a 16-bit value in d7 to a constant, and branches to another routine if the value is greater than or equal to that constant; otherwise it branches to another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `set_structure_field`: Library code that writes 16384 to offset 10 of a structure pointed to by a0, then enters a loop that tests bits and compares the word at a0 against constants, potentially calling another routine and storing the result in a pointer at offset 8 of a1.
- `update_pointer_value`: Library code that compares d7 to two constants, stores d0 at the pointer in a1, and then enters a loop that checks a word at a0, potentially writing 25600 to offset 10 of a0 and calling another routine based on bit tests and value comparisons of the word at a0.
- `load_and_test_pointer`: Compares a 16-bit value in d7 to two different constants, and if neither matches, loads a long value from a pointer in a1 into d0 and tests a word at a0; if the word is negative, it returns after popping d7 from the stack, otherwise it returns after popping d7 from the stack.
- `test_and_branch`: Library code that stores d7 on the stack, writes 0 to offset 6 of a structure in a0, and branches to another routine if the high byte of the word at offset 6 of a0 is non-zero.
- `process_threshold_value`: Library code that checks if the word at a0 is negative, otherwise writes 36864 to offset 10 of a0 and enters a loop that tests bits and compares the word at a0 against constants, potentially calling another routine.
- `store_constant_and_test`: Compares a 16-bit value in d7 to two different constants, and if neither matches, stores the constant 64 at a pointer in a1 and tests a word at a0; if the word is negative, it clears d0 and returns after popping d7 from the stack, otherwise it returns after popping d7 from the stack.

### MathIEEEDoubBas.Library

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_structure_fields`: Clears a 16-bit word at offset 32 of the structure in A5, copies the contents of A6 to offset 34, copies the contents of A0 to offset 38, and returns if certain bits in a byte at offset 329 of A6 are not set.
- `decrement_and_check_reference_count`: Decrements a 16-bit word at offset 32 of the structure in a6 and, if the result is zero and bit 3 of the byte at offset 42 is set, calls update_library_status; otherwise it returns.
- `set_library_callback_flag`: Copies a pointer from a structure into a register, checks a word at offset 32 of that structure, and if it is zero, sets bit 3 of a byte at offset 42 of the structure before returning; otherwise it returns.
- `execute_library_callbacks_2`: Copies a long value from offset 38 of the structure in A5 to D2, calls a routine at an offset from A6, then calls another routine at a different offset from A6 using a modified pointer, and returns D2.
- `compare_double_values`: Library code that compares four long values in d0, d1, d2, and d3, returning zero if specific conditions regarding their signs and equality are met, or returning the result of adding d2 to itself if certain non-zero checks pass.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `return_negative_one`: Loads the constant negative one into D0 and returns.
- `compare_long_values`: Compares d0 to d2, and if d0 is negative, branches to another routine; if d0 is greater than d2, branches to another routine; otherwise it compares d1 to d3 and branches based on the carry and not-equal flags, and if those conditions are not met, it clears d0 and returns.
- `check_long_equality_and_double_zero`: Compares three longwords in d1, d2, and d3 for equality; if they are all zero, it doubles the value in d0 and otherwise returns negative one via another routine or zero.
- `compare_long_values_extended`: Compares d0 to d2 and branches based on the sign and magnitude, then compares d3 to d1 and branches based on the carry and sign; if the conditions are met, it clears d0 and returns.
- `test_and_increment_d0`: Adds the value in d0 to itself and branches based on the carry, overflow, or negative flags; if d1 is zero, it returns, otherwise it sets d0 to one and returns.
- `branch_on_d1_zero`: Tests the long value in D1 and, if it is zero, returns; otherwise it loads the constant negative one into D0 and returns.
- `return_negative_one_if_nonzero`: If d1 is zero, it returns; otherwise it loads the constant negative one into d0 and returns.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_highest_bit`: Clears bit 31 of the long value in D0 and returns.
- `toggle_highest_bit`: Toggles bit 31 of the long value in D0 and returns.

### MathIEEEDoubTrans.Library

- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `update_structure_fields`: Clears a word at offset 32 of a structure pointed to by d0, then copies the contents of a6 and a0 into offsets 34 and 38 of that same structure, provided specific bits in a status byte at offset 300 of a6 are set; otherwise it returns with d0 cleared.
- `increment_short_and_clear_bit_3`: Increments a 16-bit value at offset 32 from a base pointer and clears bit 3 of a byte at offset 42 from the same base pointer before returning.
- `decrement_library_counter_2`: Decrements a 16-bit word at offset 32 of the structure in a6 and, if the result is zero and bit 3 of the byte at offset 42 is set, calls another routine.
- `update_structure_status_bit`: Copies a pointer from a register to another register, checks a word at offset 32 of the structure, and if it is not zero, sets bit 3 of a byte at offset 42 of the structure before returning.
- `execute_library_callback_with_offset`: Fetches a value from an offset of 38 in a structure, calls a function via a pointer at offset 252 in the library base, calculates a new pointer by subtracting a 16-bit value at offset 16 from the original pointer and adding a 16-bit value at offset 18, then calls a second function via a pointer at offset 210 in the library base.

### RoboSport (1992)(Ocean)[cr].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
