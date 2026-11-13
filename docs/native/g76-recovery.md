# Game 76 (Barbarian v1.0 Palace 1784): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00076). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (497120 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 512 of 512 named (0 author symbols), 512 of 512 described (120 by rule, 187 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5548 of 5561 (99.77%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8976 of 8989 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 458426 |
| code | 29620 |
| data | 8897 |
| padding | 177 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| BarbarianPalace.Slave | 1 | 1077 | 78 |
| main | 1 | 19 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 100 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 121 | 46 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 79 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 246 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6904 | 231 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 51 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 372 | 16 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 26 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 79 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-mixed): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-mixed): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 118 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4331 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 314 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 470 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### BarbarianPalace.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `invoke_structure_callbacks`: Saves its own address and structure pointer in globals, then calls two function pointers stored in that structure.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in a pointer.
- `execute_indirect_jump_2`: Executes a jump to a location determined by a pointer.
- `return_masked_global_value`: Reads a 32-bit global variable and returns it with bits four to six cleared.
- `return_top_of_stack`: Returns the 32-bit value at the top of the stack without changing anything.
- `store_stack_word_to_global`: Copies the 16-bit value passed on the stack into a global variable and returns.
- `return_zero_constant_2`: The routine immediately returns zero without performing any operations or modifying data.
- `set_bit_one_in_struct`: Sets bit one in the byte at offset fourteen of the structure whose pointer is in A1, then returns.
- `return_to_caller`: Returns immediately without performing any work.
- `store_register_to_global`: Copies the 32-bit value in data register D0 into a global variable and returns.
- `copy_global_then_indirect_call`: Copies a stored global value into another global variable, then makes an indirect call the decompiler could not resolve.
- `copy_struct_byte_to_memory`: Copies the byte at offset six of the structure in A0 into a memory location 2048 bytes above the address held in A6.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: Returns four when a global 32-bit variable equals a specific constant, and one otherwise.
- `initialize_pointer_chain`: Writes a sentinel value into a new entry and stores the address of the following entry into three pointer fields across two structures.
- `copy_nested_pointers`: Copies a 32-bit value reached through each of two pointers in the source structure into fields of the destination structure, skipping any pointer that is zero.
- `convert_nonzero_to_minus_one`: Returns minus one, truncated to a 16-bit result, when the value in data register D0 is non-zero, and zero when it is zero.
- `set_bit_six_clear_byte`: Sets bit six in a byte at offset 3584 from the address register A0, then clears a byte at offset 3072 from the same base.
- `enter_infinite_loop`: Enters an infinite loop by repeatedly checking a global variable and then executing an unconditional jump.
- `combine_table_values_into_field`: Combines the first three values of a global table into one number: the first is multiplied by 1440, the second is added, the sum is multiplied by 60, and the third, read as a signed 16-bit value and divided by 50, is added.
- `dispatch_status_by_flags`: Clears a status byte at offset 31 of the structure in A1, then sets it to 29 when a flag bit in the structure at offset 64 is set.
- `negate_word_flip_top_bit`: Negates the 16-bit value at offset 29547 in the structure at A1 and toggles the top bit of the byte at offset 65 in the structure at A3.
- `store_inverted_bit_test`: Tests the bit selected by the byte at offset 67 of the structure at A3 within a global byte, and stores 255 at offset 32 of the structure at A1 when that bit is clear, or 0 when it is set.
- `return_from_call`: The routine is empty and performs no operations before returning.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `clear_bit_and_increment_counter`: Clears the bit selected by the byte at offset 67 of the structure at A3 in a global byte.
- `return_to_caller_2`: Returns immediately without performing any operation on data.
- `do_nothing_stub`: Performs no work and returns, leaving memory and registers unchanged.
- `return_to_caller_3`: Returns immediately without performing any operation on data.
- `clear_field_and_save_register`: Stores a zero 32-bit value at offset twelve of the structure in A1, then saves the A4 register value into a global variable and returns.
- `compare_and_dispatch_indirect`: Compares the A4 register with a saved global value.
- `store_own_address_in_ram`: Writes this routine's own address into a fixed RAM location and returns.
- `allocate_and_link_library_nodes`: Allocates two memory blocks and links them together after opening a library through exec's OldOpenLibrary.
- `invoke_library_function_pointer`: Calls a library function at offset minus 96 from the library base in A6, then makes an indirect call through the function pointer at offset 22 of the returned structure.
- `dispatch_message_and_execute`: Finds a task, opens a library, waits for a port, and retrieves a message to update a structure and perform an indirect jump based on a table.
- `dispatch_message_and_execute_2`: Updates several bitfields in various memory locations, sends a message to a port, waits for a port, retrieves a message, and performs an indirect call based on a value found in a table.
- `store_result_then_dispatch`: Calls helper routines and stores the returned value into a structure, using its low part at offset sixteen when the high part is zero, and stores a second register at offset four.
- `store_minus_one_then_dispatch`: Calls two helper routines, stores minus one at offset twelve and a second register at offset four of a structure, then searches a table and makes an indirect call to the matching routine.
- `store_status_then_dispatch`: Calls helper routines and, when the first returned high part is non-zero and a check fails, calls a function pointer and stores minus one as the status.
- `store_fixed_code_then_dispatch`: Calls a helper, then stores the code 216 at offset sixteen and zero at offset twelve of a structure, with a second register at offset four.
- `dispatch_message_and_execute_3`: Calls another routine, retrieves a pointer from a structure in d7, calls a library routine, and if the result is non-zero, copies a block of memory from a source to a destination, performs several memory moves, and enters a loop that branches back to itself based on a value in d0.
- `load_and_call_pointer`: Loads a pointer from the address in a1, calls the routine at that address, and then branches to another location.
- `branch_with_inverted_byte`: Sets d0 to zero, sets d1 to 34, inverts the bits of d1, and branches to another location.
- `process_and_dispatch_message`: Calls two routines, and if the second returns a non-zero value, enters a loop to copy 57 bytes from a source to a destination, then calls another routine twice and branches to another routine.
- `copy_memory_loop`: Clears the stack, calls a routine using a pointer in a2, and then enters a loop that copies bytes from one memory location to another until a byte is zero.
- `execute_multiple_subroutines`: Calls a subroutine, then calls two more subroutines using addresses derived from a pointer in a1, and finally branches to another location.
- `call_other_routine`: Calls another routine and then branches to another location.
- `fill_memory_block`: Clears a memory block starting at the address in a0 and fills it with several longword constants and a value from a3 before branching to another location.
- `process_string_and_request`: Calls another routine, calculates a length based on a string in a structure, copies bytes to the stack, and calls allocate_and_record_request if a condition is met.
- `clear_registers_and_branch`: Sets d0 and d1 to zero and branches to another location.
- `copy_buffered_bytes_to_destination`: Copies up to the requested number of bytes from a 10240-byte buffer in a structure into a destination.
- `write_bytes_into_buffered_block`: Copies the requested number of bytes from a source into a 10240-byte buffer in a structure.
- `flush_then_store_and_dispatch`: Calls a flush function pointer from a second structure, stores the requested length at offset twelve and a second register at offset four, then performs the same table search and indirect call as its neighbour.
- `store_status_with_optional_field`: Calls helper routines and stores either the returned value at offset sixteen with status zero, or a value at offset thirty-six of a second structure with status minus one.
- `execute_indirect_calls`: This routine performs several indirect function calls and iterates through a table until a matching value is found to execute a final indirect call.
- `retry_helper_after_library_call`: Calls helpers, and when the first result has a non-zero high part, calls a function pointer and a library function, then retries the helpers.
- `flush_if_buffered_then_dispatch`: Calls a function pointer from a second structure when a counter at offset 256 of a buffer structure is non-zero, then stores minus one at offset twelve and a second register at offset four.
- `compute_bounded_position_offset`: Computes a new position from an offset and a mode held in the structure.
- `advance_position_with_bounds_check`: Adds a value to a position in the structure in A0 and, when the result passes the stored limit, stores minus one with the code 219 instead.
- `store_fixed_error_then_dispatch`: Stores the code 219 at offset sixteen and minus one at offset twelve of a structure, with a second register at offset four.
- `return_from_subroutine_2`: This routine performs a single return instruction to exit the current function.
- `return_from_subroutine_3`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine_4`: executes a single instruction to return control to the calling routine.
- `shift_string_and_store_count`: Moves the bytes of a zero-terminated string one place along toward its end, then writes the number of shifted bytes minus one into the first byte position.
- `allocate_and_record_request`: Returns zero when its first helper reports nothing to do.
- `process_request_data`: Checks a register, retrieves a value from a structure, and calls a function via a library offset.
- `build_joined_path_string`: Combines a directory part taken from a stored name with the name passed in D1, resolving slashes and colons, and writes the result into a newly allocated block.
- `call_routines_return_zero`: Calls two routines in turn and then returns zero.
- `compare_two_strings`: Compares two zero-terminated byte strings addressed by A0 and A1, returning as soon as a byte differs and falling through once the terminating zero matches.
- `return_from_subroutine_5`: This routine is empty and performs no operations.
- `check_hunk_code_and_run`: Opens a library through exec and calls library functions, then compares a 16-bit value with a fixed AmigaDOS hunk type code.
- `update_pointer_and_fill_data`: Increments a value from a structure, stores it in a local variable, conditionally writes a constant to two offsets in that structure, and calls another routine.
- `set_low_bits_advance_pointer`: Adds four to the value at the pointer in A2 and sets the low two bits of three consecutive bytes at the pointer in A4.
- `forward_to_neighbour_routine`: Calls a single neighbouring routine and returns, passing control straight through with no other work.
- `return_data_register_value`: Returns the 32-bit value in data register D0 without modifying it.
- `xor_running_word_table`: Applies a running XOR across 16-bit words of a table block, stepping through a fixed-length region in fixed increments.
- `return_to_caller_4`: Returns without performing any operation.

### main

- `xor_decode_own_block`: Runs a running XOR over 16-bit words starting at its own address, which changes the routine's own memory.
- `tiny_neg`: Small routine of 1 instruction that negates (a6)+.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `or_low_byte_with_constant`: Returns the low byte of data register D0 combined with the constant 117 by bitwise OR.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `or_register_bit`: Performs a bitwise OR operation on the contents of a data register with a constant value.
- `return_low_byte_value`: Returns the low byte of data register D0 without modifying it.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `apply_bit_mask`: Returns a byte resulting from a bitwise OR between a register and a specific bit pattern.
- `return_register_byte`: Returns the byte currently held in a data register.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `pass_data_register`: Passes the value of a data register through to the caller.
- `transfer_register_value`: Returns the value contained within a data register.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_constant_bits`: Combines a byte from a data register with a fixed bitmask using an OR operation.
- `output_register_content`: Returns the byte value stored in a data register.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `copy_register_byte`: Returns the byte value from a data register without modification.
- `mask_data_register`: Applies a bitwise OR to a data register using a constant value.
- `return_byte_value`: Returns the byte value held in a data register.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_register_value`: Returns the byte value currently held in a data register.
- `return_register_value_2`: This routine returns the current value held in a data register.
- `return_uninitialized_register`: The routine returns the value currently held in a data register without performing any operations.
- `return_uninitialized_value`: The routine returns an undefined byte value from a register without performing any operations.
- `return_uninitialized_register_2`: The routine returns the current value of a data register without performing any operations.
- `return_undefined_value`: This routine is empty and returns an uninitialized value from a register.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_undefined_byte`: This routine is empty and returns an undefined byte value.
- `return_modified_register`: Returns the byte value from a data register after performing a bitwise OR operation with a constant.
- `bitwise_or_register_value`: Performs a bitwise OR operation between the contents of a data register and a constant value, returning the result.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `apply_bitmask_to_register`: Performs a bitwise OR operation on the contents of a data register with a constant value and returns the result.
- `return_byte`: Returns a byte value from a register.
- `return_modified_byte`: Returns a byte value from a register after performing a bitwise OR operation with a constant.
- `nop_padding_byte`: This routine is empty and consists of a single byte of data that does not perform any operations.
- `return_uninitialized_value_2`: The routine returns an undefined byte value without performing any operations.
- `return_uninitialized_value_3`: The routine returns an uninitialized byte from a register.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_uninitialized_value_4`: This routine returns an uninitialized value from a register to the caller.
- `return_undefined_value_2`: The routine is empty and returns an uninitialized value from a register.
- `return_empty_register`: This routine returns the current value held in a data register without performing any operations.
- `bitwise_or_register_value_2`: Performs a bitwise OR operation between the contents of a data register and a constant value, then returns the result.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_undefined_byte_2`: This routine is empty and returns an uninitialized byte value.
- `nop_padding_byte_2`: This routine is empty and consists of a single byte that does not perform any operations.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_data_register`: Passes the contents of a data register back to the caller.
- `return_data_register_2`: Returns the byte value from a data register.
- `return_bit_masked_byte`: Returns a byte from a data register with specific bits set via a logical OR operation.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_data_register_3`: Returns the value from a data register.
- `return_bit_masked_byte_2`: Returns a byte from a data register after applying a bitwise OR with a constant.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecoded_single_instruction`: The decompiler timed out, so only the listing is known: a single instruction that was executed in the reference run.
- `execute_indirect_jump_3`: Clears bit 0 of register d1 and performs an indirect jump using the address stored at the location pointed to by a0.
- `return_uninitialized_value_5`: This routine returns the value currently held in a data register without performing any operations.
- `return_bit_masked_byte_3`: Returns a byte from a data register with a constant bitmask applied.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_data_register_4`: Returns the byte contents of a data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_ram_vector_return_value`: Calls through a function pointer stored at a fixed RAM address, then returns the value left in data register D1.
- `call_increment_routine`: Calls another routine that increments a 16-bit value at a specific offset from a base pointer.
- `load_constant_and_branch`: Loads the value 232 into register d0 and branches to another location.
- `load_constant_and_branch_2`: Loads the value 233 into register d0 and branches to another location.
- `load_constant_and_branch_3`: Loads the value 234 into register d0 and branches to another location.
- `load_constant_and_branch_4`: Loads the value 235 into register d0 and branches to another location.
- `load_constant_and_branch_5`: Loads the value 236 into register d0 and branches to another location.
- `increment_offset_word`: Increments a 16-bit value located at an offset of 32 bytes from the pointer held in A6.
- `increment_offset_word_2`: Adds one to a 16-bit integer stored at a specific offset from the A6 base pointer.
- `increment_offset_word_3`: Increments the 16-bit value found at offset 32 relative to the address in A6.
- `increment_offset_word_4`: Performs a 16-bit increment on the data at offset 32 from the A6 pointer.
- `increment_offset_word_5`: Increments a word at a 32-byte offset from the base address in A6.
- `increment_offset_word_6`: Adds one to the 16-bit value at the offset of 32 from the A6 register.
- `increment_offset_word_7`: Increments the 16-bit value located 32 bytes past the pointer in A6.
- `load_constant_and_branch_6`: Loads the value 242 into register d0 and branches to another location.
- `load_constant_and_branch_7`: Loads the value 243 into register d0 and branches to another location.
- `load_negative_value_and_branch`: Loads the value negative 24 into register d0 and branches to another routine.
- `increment_offset_word_8`: Increments a 16-bit value at an offset of 32 from the A6 base pointer.
- `increment_offset_word_9`: Adds one to the 16-bit word at offset 32 from the address in A6.
- `increment_offset_word_10`: Increments the 16-bit value stored at offset 32 from the A6 base pointer.
- `increment_offset_word_11`: Calls a routine that adds one to a 16-bit value located at an offset of 32 from the base pointer in A6.
- `load_negative_value_and_branch_2`: Loads the value negative 25 into register d0 and branches to another routine.
- `load_negative_value_and_branch_3`: Loads the value negative 24 into register d0 and branches to another routine.
- `increment_offset_word_12`: Increments a 16-bit word stored at a 32-byte offset from the address held in A6.
- `increment_offset_word_13`: Performs a 16-bit increment on a value at a specific offset from the A6 pointer.
- `increment_offset_word_14`: Executes a call to add one to a 16-bit integer at offset 32 relative to A6.
- `increment_offset_word_15`: Adds one to the 16-bit value found at offset 32 from the base pointer in A6.
- `increment_offset_word_16`: Increments a 16-bit word at a fixed offset from the pointer in A6.
- `increment_offset_word_17`: Calls another routine to increment a 16-bit value at offset 32 from the A6 base pointer.
- `return_zero_constant`: Returns the constant zero without doing anything else.
- `branch_to_another_routine`: Performs a no-operation and then branches to another routine.
- `decrement_word_at_offset`: Subtracts one from the 16-bit value at offset 32 from the base pointer in A6, then returns zero.
- `increment_word_at_offset`: Adds one to the 16-bit value at offset 32 from the base pointer in A6, then returns.
- `execute_single_instruction`: A routine consisting of a single instruction.
- `execute_single_instruction_2`: Contains one instruction and is entered from outside the static flow.
- `execute_single_instruction_3`: A one-instruction routine that is executed during a reference run.
- `execute_single_instruction_4`: This routine consists of a single instruction and is entered from outside the static flow of the program.
- `return_from_subroutine_6`: The routine contains a single instruction that returns control to the caller.
- `return_from_subroutine_8`: This routine performs a standard return from a subroutine.
- `return_from_subroutine_9`: This routine performs a standard return to the calling code.
- `return_from_subroutine_10`: This routine performs a single return instruction to exit the current execution flow.
- `return_from_subroutine_11`: performs a standard subroutine return to the caller.
- `copy_stack_to_register`: This routine copies a value from the stack pointer to a register and then returns.
- `setup_stack_and_branch`: Decrements the stack pointer by eight, pushes a longword constant onto the stack, and branches to another location.
- `push_literal_and_setup_frame_then_return`: Decrements the stack pointer by eight, pushes a literal long, branches to a shared tail, then stores a5 in the frame, loads a new a5 from pc-relative code, stores it and returns.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `forbid_reply_and_call_hooks`: Records the address of a stack variable, looks up the current task through a library call, and runs library calls including Forbid and ReplyMsg.
- `store_result_then_release_fields`: Calls a library routine and stores its result in a structure field, returning it when it is non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `xor_table_and_install_hook`: XORs pairs of words from a table into a buffer at offset 69360 until the index reaches 3416, then clears a block of 250 words at address an address and installs a small handler routine into a linked list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_ram_block_then_calls`: Zeroes a fixed RAM block of words with a counted loop, then makes library calls including FindTask, Forbid, Permit, AllocEntry, FreeEntry and CloseLibrary, along with six calls to neighbouring routines.
- `find_task_and_run_hooks`: Finds the current task through a library call and stores a pointer derived from it in a RAM location.
- `release_structure_fields_by_flags`: Calls an optional function pointer, then releases several structure fields through library calls, each only when its field is non-zero.
- `release_structure_fields_by_flags_2`: This routine calls an optional function pointer, stores a returned value into a global variable, and then conditionally calls several other function pointers based on whether specific structure fields are non-zero.
- `call_two_library_vectors`: Calls two library function pointers read from fixed offsets of the base register in A6, one after the other, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bclr.b) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_and_objects`: Initializes a global structure by populating several fields with values from library calls and table lookups.
- `run_state_machine_loop`: Runs a loop that dispatches on a state value held in a structure field, calling helper routines and a library function in its branches, and repeats until the state changes.
- `execute_state_machine_callbacks`: Evaluates a state variable to execute specific logic branches, including iterating through a table of function pointers and clearing a sequence of values.
- `process_state_machine_logic`: Executes a state machine based on a value stored in a global structure.
- `process_state_machine_logic_2`: Iterates through a state machine based on a value stored in a global structure.
- `reset_state_machine_and_jump`: Clears object link tables, sets a state value to 3 in a structure via a4, and branches to another routine.
- `branch_to_middle_of_routine`: Performs a short branch to a specific offset within another routine.
- `jump_to_state_handler`: Performs a short branch to another location in the executable.
- `count_then_set_state`: Increments a counter field in a structure until it passes eight, then sets a state field to one and calls a helper twice.
- `process_state_machine_logic_3`: Evaluates a state variable to execute a sequence of conditional function calls and table lookups.
- `jump_to_state_handler_2`: Performs a short branch to another routine, which checks a bit in a structure and potentially updates a state value to 5 before entering a loop that processes record handlers, updates slot pairs, and manages object lists and library vectors.
- `process_object_state_and_dispatch`: Checks if an object slot is active and, if not, evaluates a specific flag to move an object to new coordinates, update its entry byte, and remove it from a list.
- `branch_to_handler`: Performs an unconditional short branch to another location.
- `decrement_and_branch_countdown`: Decrements a longword at offset 5876 of the structure in a4 if it is non-zero, otherwise it sets a value at offset 5978 of the same structure to 5, then branches to another routine.
- `process_state_machine_logic_4`: Evaluates a state variable to execute specific branches of logic, including iterating through a table of function pointers and clearing a sequence of values in a memory array.
- `dispatch_and_update_state`: Pushes a pointer from a global variable onto the stack, calls another routine, and adjusts the stack pointer.
- `process_object_state_machine`: Iterates through a state machine based on a status field, executing specific logic branches for different integer values.
- `handle_object_removal_and_teleportation`: Checks if an object slot is inactive and has a specific flag set; if so, it updates an object field with a value from a global variable, stores a flag, removes the object from a list, updates its coordinates, and calls a library vector and a helper routine.
- `handle_object_state_transition`: Checks a byte at offset 15 of an object structure for a specific value, and if it matches, updates a state field to 12, calls store_entry_byte_flag, and sets a flag at offset 46 of the main structure to 1 before continuing the state machine dispatch loop.
- `dispatch_subroutine_and_branch`: Calls a subroutine and then performs an unconditional branch to another location.
- `decrement_and_branch`: Tests a longword at offset 5876 of a structure in a4; if it is not zero, it decrements that longword by 1 and branches to another routine, otherwise it branches to a different location.
- `set_structure_values_and_branch`: Stores the value 5 into a longword at an offset of 10584 from a6, calls a subroutine, writes 1 to an offset of 152 from a4, and then performs an unconditional branch.
- `dispatch_state_machine_actions`: Clears object link tables and sets a state value to 3 in a structure via a4, then branches to specific subroutines based on the value of a state variable.
- `update_object_state_and_cleanup`: Executes a state machine dispatch loop and, depending on a state value, performs object deactivation, list removal, and a series of function calls via pointers stored in a structure.
- `call_helper_per_table_entry`: Loops over seventeen entries of a table held at a fixed offset in a structure, calling a helper with each entry.
- `accumulate_table_entries_totals`: Initialises five local counters, then loops over up to seventeen table entries, accumulating values and calling helpers.
- `store_entry_pointer_and_call`: Stores a table entry pointer into a slot chosen by a loop index, calls a helper with that entry and saves the helper's result in the entry.
- `copy_table_field_into_entry`: Stores a table entry pointer into a slot chosen by a loop index, then copies a field from a second table into the entry at offset 32.
- `populate_entity_attribute_tables`: Iterates through a list of pointers to populate several structured tables based on values returned by other routines.
- `iterate_and_populate_data_structures`: Iterates through a table of pointers to copy data into indexed slots within a larger structure.
- `link_table_entry_to_slot`: Copies a table entry selected by a loop index into a fixed structure slot, then stores a pointer to a second structure field at offset 40 of that entry.
- `store_table_value_by_index`: Stores a table entry into a slot chosen by an index held in the structure, then makes two library calls and two neighbouring routine calls.
- `call_helper_per_table_entry_2`: Calls a neighbouring routine, then calls a helper with each of the seventeen table entries in turn and returns.
- `finish_entry_or_clear_state`: Calls a helper with its argument and, when that succeeds, calls it again for entries whose state at offset 32 is five.
- `clear_entry_link_fields`: Loops over seventeen table entries and, for each non-null entry whose state at offset 32 is not five, clears a 32-bit field reached through its pointer chain.
- `accumulate_positive_entry_states`: Loops over seventeen table entries, adding the positive state values at offset 32 into four local counters, and calls two neighbouring routines.
- `set_entry_word_and_call`: Stores a table word into offset 22 of an entry selected by a loop index, then calls a helper with that entry and a value from a second table.
- `call_helper_with_entry_constants`: Calls a helper with a table entry, a value from a second table, the constant 120 and zeros, then reads the entry's first field and calls a neighbouring routine.
- `dispatch_entry_state_loop`: Runs a loop that switches on an index, storing a table word into an entry and calling a helper in one case, and repeats until the index changes.
- `call_helper_for_indexed_entry`: Calls a helper with a table entry, two values from a third table and a computed index, then increments a loop index held in the structure.
- `process_event_table_dispatch`: Iterates through a table of event descriptors and executes different subroutines based on a type field.
- `call_helper_with_optional_library`: When a word field at offset 156 is non-zero, calls a helper with a table entry and a value from a second table, then makes a library call and calls further neighbouring routines.
- `store_library_result_in_field`: Calls a helper with a structure offset and stores the result of a library call in a 32-bit field.
- `run_counted_library_loop`: Makes three library calls, then loops for the count stored at offset 1260 of the structure, calling further routines on each pass.
- `call_four_library_functions`: Calls four library functions in turn, reads a field through a pointer chain, then calls two neighbouring routines with that data.
- `store_library_result_or_default`: Calls a library function and stores its result in a field.
- `call_helper_in_three_passes`: Calls a helper with a fixed set of offsets on each of three passes, passing its argument as the first value.
- `fill_buffer_from_indexed_pointer`: Looks up a pointer from a table by index and writes bytes through it into a buffer, using three loop values, then calls a neighbouring routine.
- `run_library_pair_loop`: Loops twice, making several library calls on each pass, and calls a neighbouring routine at the end.
- `decrement_field_and_test_bit`: Decrements a counter at offset 36 of a structure, passes the lowest bit of the new value to a helper, then calls two further routines and returns a flag from the result.
- `compare_linked_fields_then_call`: Checks a 16-bit field of one linked structure against a field of another, and calls up to six neighbouring routines depending on the comparison.
- `adjust_linked_field_and_call`: Calls a helper, adjusts a 16-bit field of a linked structure by seven when the result allows it, and calls up to eight neighbouring routines.
- `reset_countdown_and_call`: When a counter in the structure is below one, calls two neighbouring routines with fixed arguments and resets the counter to seven.
- `reset_countdown_and_call_2`: Calls two neighbouring routines with fixed arguments, then sets the counter in the structure to seven.
- `write_five_decimal_digits`: Writes a number into a buffer as decimal digits, counting down from 10000 and emitting one ASCII character per step through a helper, then calls two neighbouring routines.
- `call_two_helpers_with_buffer`: Calls a helper with the first field of a structure and a buffer pointer, then calls a second helper with fixed values including 64 and 20.
- `select_masked_index_and_call`: When a structure field equals minus one, takes a value from a helper, masks it to its low four bits, passes it to another helper, and calls a library function.
- `initialize_structure_defaults`: Sets several fields of the structure to default values, copying 16-bit values from fixed offsets of the same structure into others.
- `clear_object_link_tables`: Iterates through a table of seventeen entries to clear specific 32-bit fields in non-null entries that do not meet a specific state condition.
- `advance_state_machine_tick`: In state 2 with a positive counter it moves ten units from that counter into a stored value and clears the timer.
- `choose_status_code_and_count`: Sets the countdown to 10 and chooses status code 42 or 43 by comparing an object field with a table entry.
- `tick_state3_status_counter`: Sets a countdown of 10.
- `set_status_code_eleven`: Sets a countdown of 11 and calls one of two unnamed routines depending on whether a field of the argument object is zero.
- `forward_single_unnamed_call`: Calls one unnamed routine and returns.
- `clear_state_counter_and_flag`: Clears a state-dependent counter, one field when the state is 1 and another otherwise, calls an unnamed routine, and sets a completion flag to 1.
- `clear_pending_value_and_flag`: Clears a stored value, calls an unnamed routine, and sets a completion flag to 1.
- `run_frame_display_update`: Updates several object pointers, waits until an unnamed routine reports success, then calls library functions at offsets -270, -252 and -150 to set up two regions sized 320 by 200 and the display.
- `resolve_round_end_countdown`: Calls two unnamed routines for each of two objects, then in state 2 either sets result code 9 and ends the phase when the countdown reaches zero, or copies two object fields into stored values and reloads the countdown to 450.
- `process_object_proximity_events`: Compares the position fields of two objects, and when the first is at or above the second it runs a fixed sequence of unnamed display and object calls, adds 10000 to a stored counter and sets a timer to 50.
- `run_object_event_sequence`: If a counter field of the caller's object is zero it calls one unnamed routine.
- `invoke_unnamed_object_routine`: Calls one unnamed routine with an object pointer taken from the game-state block and returns.
- `allocate_linked_object_nodes`: Allocates an object record through a library call at offset -396.
- `build_positioned_object_record`: Allocates three linked records, sets fixed values 200 and 316 and type 4 in the first, and calls library function at offset -120.
- `verify_bitmap_row_transfer`: Opens a resource through an unnamed routine, transfers each row listed in a table and checks that each transferred length equals width times height.
- `check_transfer_length_match`: Opens a handle through an unnamed routine and performs one transfer of the requested length.
- `run_transfer_and_compare_length`: Runs one transfer using three values from the caller's frame, stores the result there, releases the handle, and returns whether the result equals the expected length.
- `release_handle_return_one`: Calls library function at offset -36 on the handle from the caller's frame and returns one.
- `read_blocks_to_offset_buffer`: Opens a handle and reads 512-byte blocks into the caller's buffer at increasing offsets until a read returns zero, then releases the handle and returns the total count.
- `read_blocks_until_exhausted`: Opens a handle and reads 512-byte blocks into a local stack buffer until a read returns nothing, then releases the handle and returns the total byte count.
- `read_matching_length_header`: Opens a handle and reads a 350-byte header into a local buffer.
- `create_timed_object_pair`: Allocates an object record and its sub-record, sets default values of 127 and 1, and calls library function at offset -444 with the caller's value.
- `advance_round_robin_slot`: Advances a round-robin index in the caller's table, wrapping to zero at the table length.
- `queue_indexed_object_entry`: Sets an object record from a table entry indexed by the caller's count, using type 3, flag 17 and values from two tables.
- `deactivate_indexed_entry`: If the record is marked active, it calls an unnamed routine on its handle.
- `store_entry_byte_flag`: Calls an unnamed routine on the record and, on success, stores the given byte in one flag field and clears the adjacent flag field.
- `initialize_record_flag_bytes`: Calls an unnamed routine on the record and, on success, stores the given byte in one field, clears six following flag bytes and returns zero.
- `set_object_coordinates_and_refresh`: Sets the object's two coordinate fields and marks it active.
- `remove_object_from_list`: Releases an object: for type 5 with a valid handle it calls library function at offset -414, otherwise it sets bit 1024 in a flag word and calls an unnamed routine.
- `match_entry_against_table`: Calls an unnamed routine with a table entry indexed by the caller's index, and when that succeeds calls another unnamed routine on the first field and returns one if it equals the stored value, otherwise zero.
- `relink_active_objects_in_order`: Links the active entries of a seventeen-slot table into forward and reverse chains by comparing their key values with an unnamed routine.
- `link_three_objects_in_cycle`: If three object pointers are set and a third check passes, it links the three objects into a cycle through their link fields and returns the check result.
- `link_three_slots_into_cycle`: If three slot pointers are set and a third check passes, it links the three objects into a cycle through their link fields and returns the check result.
- `dispatch_handler_by_record_code`: Calls library function at offset -372 to get a record, and if it is set searches a four-entry table keyed by the record's code and calls the matching unnamed handler.
- `decrement_counter_then_forward`: Decrements a stored counter when it is positive and the mode equals one, then calls an unnamed routine with a value from the caller's frame.
- `perform_logical_or_and_branch`: Performs a bitwise OR on register d0 with itself and then branches to another routine.
- `clear_d1_and_branch`: Clears the least significant byte of d1 and branches to another routine.
- `process_command_and_call`: Subtracts zero from d0, performs a bitwise OR on d2 with zero, loads a pointer from a structure in a5, moves a word from offset 24 of that pointer into d0, pushes d0 onto the stack, calls another routine, increments the stack pointer, and branches to another routine.
- `clear_word_and_branch`: Clears a 16-bit word at offset 186 of a structure in a4 and then branches to another routine.
- `set_flag_then_library_forward`: Sets the same flag word to 1 and calls an unnamed routine, which in turn calls library function at offset -378 for the value in the caller's frame.
- `dispatch_command_code_switch`: Dispatches a command code below 128.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `set_sprite_frames_by_state`: Sets frame indices on two sprite records according to the state value, which is 1, 2 or 3, then calls an unnamed routine with fixed values 108 and 199 and a state-dependent pointer.
- `refresh_grid_cells_when_flagged`: When its refresh flag is set, loops over a 32 by 15 grid, reads a value per cell from a table, combines it with two unnamed routine results and a library function at offset -582 masked to the low four bits, and passes each cell to an unnamed routine.
- `fill_grid_cells_set_flag`: Loops over a 32 by 15 grid, computes each cell value from a library function at offset -582 and two unnamed routine results, decrements nonzero components, and passes each cell to an unnamed routine.
- `update_sprite_pair_and_redraw`: When an unnamed routine succeeds, it updates the position fields of two sprite records and toggles a flag, calls library functions at offsets -270, -192, -378, -390 and -150, redraws six entries in a loop, then clears a stored value and calls an unnamed routine with 300.
- `read_up_to_three_characters`: Reads up to three character codes from an unnamed input routine into a buffer, stopping early at code 13 and cycling a value from 1 to 15 between reads.
- `initialize_seven_slot_records`: Fills seven record slots with a base pointer, a value from a table and a code byte from a table, resets a stored value to zero, and calls an unnamed routine with zero.
- `sort_and_pick_active_slot`: Clears the status byte of six slots, stores the given value in the head record, bubble-sorts the six slots by their key field, and passes the first slot with nonzero status to an unnamed routine.
- `fill_buffer_with_spaces`: Writes the value 32, the space character, into the given number of bytes starting at the buffer.
- `render_slot_text_fields`: Renders one slot by calling three unnamed routines: the first with the slot's pointer and a width of 3, the second with the slot's first field into a five-byte buffer, and the third with that buffer, length 5 and the slot's code byte.
- `swap_redraw_frame_pointers`: Exchanges two saved pointers with new values, calls library functions at offsets -378, -390 and -60, then calls an unnamed routine with bounds 250 and 70 and a flag of 32768.
- `redraw_panel_and_restore_pointers`: Exchanges two saved pointers, calls library functions at offsets -378, -390 and -60, then calls an unnamed routine with bounds 250 and 100 and another unnamed routine.
- `read_printable_key_event`: Fetches an input event from an unnamed routine.
- `process_helper_table_entry`: Processes a helper entry based on a type field, performing tile strip drawing, frame scaling, or coordinate updates.
- `draw_twelve_tile_strip`: Draws twelve tiles by calling an unnamed routine for each index, using a stored position pair offset by a constant and an index-scaled tile value.
- `copy_record_header_fields`: Copies two 16-bit values from the caller's record header into the record it points to, one into the field at offset 24 and one into the field at offset 22.
- `update_record_animation_frame`: If a flag field of the record is set, it computes a frame pointer from an unnamed routine result, sets the record's size field to a scaled value and calls library function at offset -126.
- `update_frame_and_check_change`: If the record's flag is set, it computes a frame pointer from an unnamed routine result, copies a record field into the size field, and calls an unnamed routine.
- `copy_header_state_field`: Copies a 16-bit value from the record's header into its state field.
- `process_slot_and_store_offsets`: Calls a system library routine with argument 21, clears a 32-bit counter at offset -4 of the structure in a5, and returns immediately if that counter is at least 12.
- `update_record_frame_scaled`: If the record's flag is set, it computes a frame pointer from an unnamed routine result, sets the size field to a scaled value and calls library function at offset -126.
- `copy_structure_fields`: Copies two 16-bit values from the structure pointed to by the pointer stored at offset 8 of the a5 structure into the fields at offsets 24 and 22 of the structure pointed to by the pointer stored at offset 0 of that same pointer, then returns.
- `test_object_flag_bit_1024`: Returns one when bit 1024 is set in the flags word of the object pointed to by the argument, otherwise zero.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `check_coordinate_passed_by_direction`: Compares the coordinate at offset 8 of two records.
- `step_record_and_check_blocker`: Tries to step a record toward a target: when its status bit 32 is clear and no pending flag is set, it calls an unnamed routine to move it.
- `finish_pending_step_marker`: Decrements a pending counter and, when it reaches zero, calls an unnamed routine on two objects with the code 36 unless their type is already 36.
- `is_slot_active`: Returns one when the pointer is non-null and the record's active field is nonzero, otherwise zero.
- `run_script_bytecode_loop`: Interprets a byte-coded animation script for one record.
- `enter_script_loop_at_top`: Entry point into the byte-coded script interpreter that shares its opcode table and end-of-data checks, entered at the top of the loop.
- `enter_script_loop_after_stop`: Alternate entry into the byte-coded script interpreter with the same opcode table, which resets the current offset before its loop.
- `enter_script_loop_from_jump`: Alternate entry into the byte-coded script interpreter with the same opcode table, used when a script jump is taken.
- `branch_to_script_advance`: Branches unconditionally to another routine and returns without executing any further instructions.
- `advance_script_bytecode_2`: Jumps to another routine that increments a per-object script byte and advances the round-robin slot by one, two, or three based on the object's current script position.
- `advance_script_bytecode`: Increments a per-object script byte and, based on the object's current script position, advances the round-robin slot for one, two, or three script entries, then reads the next script byte and applies its opcode: skipping, ending, removing the object, or modifying its position and flags.
- `dispatch_record_by_mode`: Calls one of two unnamed routines on the record depending on whether its mode field equals 1.
- `advance_record_coordinate_clamped`: Moves a record's coordinate at offset 8 by a per-direction step from a table, subtracting it when the mode is 1 and adding it otherwise.
- `stub_undecompiled_routine`: The decompiler produced no pseudo-code for this routine.
- `stub_undecompiled_single_call`: The decompiler produced no pseudo-code for this one-instruction routine.
- `branch_to_another_routine_2`: Performs a short branch to another routine.
- `jump_to_stub_undecompiled_three_calls`: Jump stub: passes control straight on to stub_undecompiled_three_calls.
- `branch_to_offset`: Performs a short relative branch to a specific instruction located elsewhere in the code.
- `jump_to_stub_undecompiled_three_calls_2`: Jump stub: passes control straight on to stub_undecompiled_three_calls.
- `branch_to_slot_update`: Performs a short branch to the routine that updates round robin slots.
- `store_word_and_branch`: Stores the value one into offset negative 2 of a structure in a5 and then branches to another routine.
- `call_and_branch`: Pushes a pointer from offset 8 of a structure in a5 onto the stack, calls another routine, increments the stack pointer, stores the value one into offset negative 2 of a structure in a5, and branches to another routine.
- `update_round_robin_slots`: Performs two sequences of calculating an offset using a division routine, adding it to a base address in a4, and calling a routine to advance a slot, then increments a byte at offset 23 of a structure in a5 and branches to another routine.
- `stub_undecompiled_three_calls`: The decompiler produced no pseudo-code for this routine.
- `handle_table_matched_event`: Looks up a code in a 12-byte per-slot table and, when it matches, plays the event.
- `refresh_active_slot_pairs`: Loops over the 17 object slots and, for each active slot, calls two unnamed routines on it.
- `resolve_pair_slot_interaction`: For the two tracked slots, when a slot is not flagged with bit 32 and is active, it calls an unnamed routine with the slot and its partner.
- `remap_record_code_by_kind`: For an active record, selects a frame table entry according to its kind: kind 2 uses one table, kinds 1 and 3 use a combined flag word, others use zero.
- `lookup_indexed_tile_byte`: Returns a byte looked up through two index tables keyed by the two byte arguments, then a third table keyed by a value from an unnamed routine.
- `compute_coordinate_distance`: Returns the absolute difference between the 16-bit coordinates at offset 8 of two records.
- `clear_flagged_tracked_slots`: For each of two tracked slots, if status bit 64 is set it calls an unnamed routine on the slot.
- `read_offset_table_byte`: Returns a byte from a fixed table at an index computed by an unnamed routine plus a fixed base offset.
- `apply_matching_code_actions`: Looks up a sequence for the record's code in a 128-byte-per-entry table.
- `clear_port_register_top_bits`: Calls two library functions at offsets -132 and -138 around a masked update that clears the top two bits of a fixed hardware byte register.
- `remap_byte_rows_via_table`: Repeats 88 times: copies ten bytes from the argument buffer, mapping each through a 256-entry lookup table into a scratch area in reverse order, then copies the scratch area back over the buffer.
- `build_bit_reverse_table`: Fills a 256-entry table so that each index holds the byte with its eight bits reversed.
- `move_and_branch`: Moves the status register into d0, moves a longword from the address pointed to by a0 to that same address, and then branches to another routine.
- `call_indirect_table_entry`: Calls one function through a pointer read from a global variable at offset -348 and returns, with no other operations.
- `lookup_slot_or_error`: Clears a field of the caller's state, then returns the slot address for an index inside the valid range when that slot is non-null.
- `open_library_init_display`: Opens the operating system library through the exec open-library call on first use and stores its base.
- `flush_dirty_records_list`: Walks a linked list of records and, for each record with bit 2 set and bit 4 clear whose two counters differ, calls an unnamed routine with the difference.
- `lookup_slot_and_forward`: Looks up a record by the first argument and returns minus one if none is found.
- `update_node_pointer`: Adds 12 to a longword at offset 8 of a structure in a5, calls another routine, stores the result at offset -4 of the structure in a5, and then branches to another routine or clears d0 and branches to a third location based on whether the stored value is zero.
- `update_linked_list_node_2`: Calculates a new address by adding 12 to a value from offset 8 of a structure in a5, stores this at offset 8 of a structure pointed to by offset -4 of a5, calls another routine with a pointer from a4, and if a value at offset 10228 of a4 is zero, stores offset -4 of a5 into that location.
- `negate_zero_return`: Calls another routine with an argument and returns that value, or returns negative one if the result was zero.
- `initialize_and_process_nodes`: Calls another routine, clears several offsets in a4, and if a longword at offset 10224 of a4 is not zero, it runs a main loop and calls a division routine before attempting to process nodes in a loop.
- `call_push_and_branch`: Pushes d7 and a2 onto the stack, calls another routine, and then returns.
- `free_linked_list_nodes`: Iterates through a linked list starting from a pointer at offset 10960 of a4, calling a routine to free each node using its size from offset 8, and finally clears offsets 10992 and 10996 of the structure in a4.
- `clear_global_pointers`: Subtracts the value in a0 from itself, stores the resulting zero into offsets 10996 and 10992 of a structure in a4, and unlinks a5.
- `update_linked_list_node`: Copies a value from a source pointer into a destination structure, clears the destination's head word, links the destination after the source's tail, and points the source's tail at the destination.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `call_triple_parameter`: Pushes a longword from offset -12 of a5 and a longword from offset 8 of a5 onto the stack, calls another routine using three identical arguments, and returns.
- `check_and_branch`: Checks if the longword at offset -12 of a5 is greater than zero; if it is not, it sets d0 to negative one and branches.
- `allocate_and_insert_node`: Calculates a size based on d7, updates a global counter at offset 10244 of a4, and attempts to insert a new node into a linked list structure.
- `update_linked_list`: If a2 equals d2, it copies a pointer from a2 to a location at offset -12 of a5, adds d7 to a longword at offset 4 of that location, and updates a pointer at offset -8 of a5.
- `compare_and_branch`: Compares a longword in a1 with a longword in a0; if a1 is not less than a0, it subtracts d7 from a longword at offset 10244 of a4 and branches.
- `compare_and_subtract`: Compares a longword in a1 with a longword in a0; if they are equal, it tests a longword at the address in a2, and if that value is not zero and is greater than or equal to d2, it subtracts d7 from a longword at offset 10244 of a structure in a4 before branching to another routine, otherwise it branches to a different location.
- `modify_structure_pointers`: Adds d7 to offset 4 of a2, compares the value at a2 with d2, and if they are equal, adds the word at offset 4 of a0 to offset 4 of a2 and moves the pointer at a0 into a2.
- `store_and_branch`: Stores a2 into offset -8 of a5, stores the value from offset -20 of a5 into offset -24 of a5, updates a2 with the value at a2, and branches to another routine.
- `reset_structure_pointers`: Moves a pointer from offset negative 12 of a structure in a5 to offset negative 8 of the same structure, clears the word at the new pointer's location, calculates the difference between the pointer and zero, stores that difference into offset negative 12 of a structure in a5 and into offset 4 of that same structure, and then moves that difference into d0.
- `append_string_with_limit`: Appends up to a caller-supplied limit of characters from the source string to the end of the destination string, terminates the result with a null byte, and returns the destination pointer.
- `forward_record_with_reset`: Looks up a record by its first argument and returns minus one when it is missing.
- `parse_command_line_arguments`: Splits the command-line string into up to 32 arguments, treating spaces, tabs and newlines as separators and double-quoted text as one argument.
- `run_init_and_main_loop`: Allocates records and resources in order, opening libraries and checking each step, and exits early when a step fails.
- `divide_32bit_with_sign_handling`: Divides a 32-bit value by another 32-bit value.
- `divide_32bit_with_remainder`: Divides a 32-bit value by a 32-bit divisor using normalised 16-bit partial divisions and returns the quotient and remainder packed into one 64-bit result.
- `multiply_32bit_words`: Returns the low 32 bits of the product of two 32-bit values, built from the two cross products of their 16-bit halves shifted into the high word plus the low-word product.
- `release_unflagged_slots_then_forward`: Loops over the slot table and, for each non-empty slot without bit 4 set, calls an unnamed routine on its second entry.
- `poll_break_signal_bits`: Reads the task signal bits and tests bits 12 and 13.
- `call_poll_break_signal_bits`: Call stub: calls poll_break_signal_bits, then returns to its own caller.
- `open_library_and_load_block`: Opens the operating system library on first use, copies a stored value into another field, and calls an unnamed routine with buffers and the values 250 and 60.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `check_io_status_record_error`: Clears a stored status and calls library function at offset -48.
- `seek_with_origin_mode`: Calls library function at offset -66 to get the current position, storing error code 22 when it fails.
- `close_handle_and_return_zero`: Clears a stored flag through an unnamed routine when it is set, calls library function at offset -36 to close the handle and returns zero.
- `next_random_scaled_value`: Steps a 32-bit shift-register pseudo-random generator once per bit of the argument, reseeding it with a fixed feedback constant when the carry is set or the state is zero.
- `transform_input_value`: Doubles the input parameter and conditionally XORs the result with a constant if a carry occurs or the result is zero.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `init_self_linked_header`: Writes the block's own address plus four into its first word, clears the second word and points the third word back at the block.
- `allocate_indexed_record_block`: Obtains a slot index from an unnamed routine and, when valid, allocates a 34-byte block through a library call with flags 65537.
- `release_record_chain`: Recursively releases a linked chain of records, processing the linked record first when its link field is set.
- `call_library_function_30`: Calls the library function at offset -30 through the library base held in a global variable and returns, with no other operations.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_198`: Calls the library function at offset -198 through the library base held in a global variable and returns, with no other operations.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_indirect_via_global_30`: Calls a function pointer read from a global variable at offset -30 and returns, with no other operations.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `stub_empty_return_routine`: Contains only a return instruction, so calling it does nothing.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `perform_no_operation`: This routine is empty and performs no actions.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `return_from_subroutine_7`: The routine performs no operations and returns immediately to its caller.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_18`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_19`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_20`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_21`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `set_interrupt_request`: Writes the value 32767 to the interrupt request register and returns immediately via an exception return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_system_handles`: Computes the current task pointer and stores it into a global variable, then opens a system library and stores its base pointer into another global variable, returning zero on failure.
- `push_return_and_return`: Pushes a 32-bit return address onto the stack and returns immediately, so it discards its caller and never resumes execution.
- `init_handles_and_loop_check`: Calls two system library routines to obtain handles, storing each and returning immediately if either is zero.
- `open_two_libraries_storing_bases`: Opens a library by name and stores the returned base pointer into a global variable at offset 1760, then returns zero on failure.
- `check_pointers_and_transform_input`: Loads a global pointer and returns the input unchanged if it is zero, otherwise loads a second global pointer and returns the input if that is also zero.
- `dispatch_hunk_type_and_update`: Compares an input value against several AmigaDOS hunk type codes and branches accordingly, updating structure fields and calling other routines for specific cases, while for other cases it XORs two bytes in memory and returns the input value.
- `return_byte_if_zero`: Calls another routine and returns its result in d0 when the value is zero, otherwise performs a sequence of no-op OR operations on d0 and returns the unchanged byte.
- `allocate_and_fill_block`: Calls another routine to obtain a memory block, writes a marker byte and a size word into it, then loops filling pairs of fields with a tag and a shifted size while decrementing a counter.
- `store_four_values_and_diff`: Calls another routine four times, storing each returned 32-bit value into consecutive fields of a global structure, then stores the incremented difference of two of those values into a following field and returns.
- `compute_checksum_and_adjust`: Computes a checksum by XORing a constant into an accumulator, then iterates over a table of 32-bit words, adding each non-zero value and rotating the result through carry for eight bits while decrementing a counter.
- `load_flag_and_call_check`: Loads a 32-bit value from a global variable at offset 2cfc, stores the constant 4 in a register, calls another routine, then loads a value from the address in that global variable into a register and returns it.
- `compare_caller_value_and_call`: Tests a caller-supplied value and, if non-zero, calls a function pointer and compares its result to that value.
- `allocate_memory_then_call`: Allocates memory by shifting the input size left by two bits and requesting 65537 bytes from the allocator.
- `tiny_lsl`: Small routine of 1 instruction that shifts d0 left by 2.
- `free_memory_conditionally`: Tests two inputs and, if both are non-zero, calls a function pointer before returning.
- `compute_crc_checksum`: Computes a CRC-style checksum over a byte buffer of the given length, updating a 16-bit accumulator one byte at a time through an inner bit loop.
