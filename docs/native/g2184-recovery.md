# Game 2184 (megaball (1991)(e. mackey)(sw)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02184). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 102 of 102 named (0 author symbols), 102 of 102 described (26 by rule, 62 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 213 of 213 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1502 of 1502 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 285320 |
| code | 4490 |
| data | 181756 |
| padding | 3058 |
| unused | 426496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 8 | 1 | 2 | 1 |
| iff.library | 1 | 217 | 17 |
| diskfont.library | 15 | 1040 | 76 |
| MegaBall (1991)(E. Mackey)(SW)[a].adf:load | 7 | 233 | 7 |
| MegaBall (1991)(E. Mackey)(SW)[a].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### 8

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### iff.library

- `open_library_call_vectors_close`: Opens a library through the exec open routine, calls two of its vectors with the result, then closes the exec library and returns the constant ten.
- `init_library_fields_and_list`: Stores the base and argument pointers in the library structure, opens the dos library through a library vector and records the result, then sets up an empty list header inside the structure.
- `update_iff_node`: As library code, it calls another routine, and if the result is not zero, it copies a 32-bit value from a structure to a field at offset 14 of another structure, calls another routine, increments a 16-bit value at offset 32 of a structure, and clears bit 3 of a byte at offset 14 of that same structure.
- `branch_to_offset`: As library code, it sets d0 to 0 and branches to another location within the library.
- `write_iff_header_2`: Checks if a value in d0 is non-zero, then calls a routine at offset 34 of the structure in a6 to write an IFF header by passing a pointer from a0 and a constant value of 128 in d0.
- `write_iff_header`: Library code that copies a pointer from a0 to a2, writes a 24-bit constant to the address in a2, calls another routine using a pointer from a5, and then either branches or writes a value to offset 4 of the address in a2 and calls another routine.
- `validate_iff_form`: As library code, it moves d0 to d1, loads a constant address into d2, calls another routine, and then branches based on a comparison between d0 and d3.
- `transfer_register_and_branch`: As library code, it calls another routine, moves the value from a2 into d0, and branches to another location.
- `process_iff_chunk`: Copies a pointer from a library structure to a6, calls a routine at an offset from a6, and if the return value in d0 is not zero, moves d0 to d4 and branches to another location; otherwise, it sets d5 to 16 and branches to another location.
- `parse_iff_chunk_header`: Checks for a specific four-character identifier in a memory buffer and, if found, uses a pointer from a library structure to call a routine that processes the chunk data.
- `execute_library_call`: Library code that checks a value in d6 and compares a stack value against a constant; if conditions are met, it adds a value from the stack to d3, calls a routine using a pointer from a5, and then branches based on the result in d0.
- `push_data_and_call_library`: As library code, it stores d3 at the address in a2, decrements d3, and if a4 is not zero, pushes several values onto the stack and calls a routine from a6.
- `execute_library_sequence`: As library code, it calls a routine using a pointer from a5, compares the result with d3, calls another routine from a5, calls a DOS routine using a pointer from a5, and finally calls another routine before branching.
- `call_and_restore_registers`: As library code, it calls another routine, moves the value from a2 into d0, and restores several registers from the stack before returning.
- `find_node_by_matching_id`: Walks a linked list from the pointer at offset 46 of the library base and returns the first node whose field at offset 14 equals the value at offset 276 of the referenced structure.
- `clear_field_return_one`: As library code, it calls another routine and, when its test result is not zero, clears a 32-bit field at offset 18 of the structure in a0, then returns 1.
- `store_field_return_zero`: As library code, it calls another routine, then stores the value from d0 into a 32-bit field at offset 18 of the structure in a0 when that value is non-zero, and returns 0.

### diskfont.library

- `branch_to_font_handler`: Branches to an offset of 80 relative to the program counter.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: As library code, it calls two different routines via the library base and, if both return non-zero, updates a structure by setting a pointer to a new location, incrementing that pointer by 4, clearing a 32-bit field at offset 14, and setting a pointer at offset 8.
- `execute_library_calls`: As library code, it calls two different routines via the library base and performs arithmetic to calculate a value based on offsets 16 and 18 of a structure.
- `remove_linked_list_node`: As library code, it checks a 16-bit value at offset 84 of a structure; if it is not greater than zero, it unlinks a node from a list, calls another routine, and then checks if a node is the last in a list to perform further unlinking and decrement a 16-bit counter at offset 40 of a structure.
- `manage_linked_list_node`: As library code, it checks two 16-bit values at offsets 38 and 40 of a structure; if both conditions are met, it calls two routines, performs arithmetic on a structure, calls a third routine, and swaps two pointers in a linked list; otherwise, it sets bit 7 of a byte at offset 14 of the structure and returns 0.
- `set_bit_and_clear_d0`: Sets bit 7 of a byte at offset 14 of the structure in a6 and clears d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_jump_conditionally`: Decrements a word at offset 38 of the structure in a6, and if it is not zero, checks bit 7 of a byte at offset 14 of the structure in a6 and jumps to an offset of negative 18 relative to a6 if that bit is set.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_font_metadata_and_copy_strings`: Iterates through a linked list of font entries, calculating string lengths to determine offsets within a buffer.
- `load_string_pointer`: As library code, it loads a pointer from an offset of a4 into a1 and sets d4 to 1 before branching to another routine.
- `count_null_terminated_string`: Counts the number of bytes in a null-terminated string by iterating through memory starting at a pointer in a1 and increments a counter in d4 for each byte.
- `copy_font_data_and_parse_structure`: Copies bytes from a source buffer to a destination buffer in a loop, then populates a font entry structure with data from a header, including size, flags, and an offset.
- `load_and_parse_font_data`: Opens a file using a pointer from a structure, reads a null-terminated string, and copies its contents into a memory buffer.
- `count_null_terminated_string_2`: Increments a counter in d4 while iterating through a null-terminated string in a1.
- `copy_font_data_from_disk`: Copies a sequence of bytes from a source buffer to a destination buffer using a loop, then calls a routine to read data from a disk and checks if the read operation was successful and if a specific value at a negative offset of a stack pointer equals 61440.
- `parse_font_header_and_data`: Reads a file to parse a font structure, populating a list of font entries with attributes such as size and flags.
- `return_zero_stub`: Clears register d0 and restores registers from the stack before returning.
- `calculate_font_offset`: Library code that checks a 16-bit value at offset 256 of a structure in a1; if non-zero, it subtracts a 16-bit value at offset 4 of a structure in a0 from that value and either adds a shifted version of the result to 32767 or returns 32767.
- `calculate_font_index_offset`: As library code, it performs bitwise operations on bytes from two structures to calculate a value, then uses a bitwise AND to decide whether to subtract a value from a table at a specific address.
- `subtract_font_indices`: As library code, it subtracts a longword from a table at a specific address using an index derived from shifting a 16-bit value in d4 left by two bits, then performs bitwise checks against bytes at offsets 259 and 258 of a structure in a1.
- `iterate_font_character_indices`: Shifts a 16-bit value in d4 left by two bits, uses the result as an index into a table at address 72 to subtract a value from d2, increments d4, and loops if d4 is less than 7.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `initialize_font_resource`: Copies two longwords from a source pointer into a local stack buffer, calls another routine to obtain a pointer, and then populates a structure with values from offsets 20, 22, and 23 of that pointer.
- `load_font_data_loop`: Calls another routine and stores the returned value into a structure at offset negative twelve relative to a6, then branches based on the zero flag or performs an unconditional branch.
- `update_font_glyph_metadata`: Searches through a memory block for a glyph with the highest value based on a comparison routine, then updates its metadata, increments a counter in a global variable, and modifies a bit in a status byte at offset 85 of a structure.
- `clear_registers_and_branch`: Clears registers d4 and d3, sets d2 to 255, and performs an unconditional branch to another location.
- `search_and_update_font_resource`: Iterates through a list of font indices to find the largest available index, then unloads the selected font and updates its metadata, including setting a flag at offset 85 and incrementing a counter at offset 132 of a structure.
- `unload_and_clear_memory`: As library code, it pushes d4 onto the stack, calls a routine to unload a segment, and then clears a word at offset 256 of a memory block calculated by multiplying d2 by 260 and adding d5.
- `manage_font_resource`: Calculates a memory offset based on a provided index and base address, clears a word at a specific offset within that memory block, and performs a series of conditional cleanup operations including unlocking, freeing memory, and closing a font if certain conditions are met.
- `load_and_parse_font_table`: Allocates memory and reads a font definition from a source pointer, then iterates through a list of font entries to populate a new table with character data and attributes.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_pointers_and_branch`: Loads a pointer from offset 4 of the stack into a0, loads a longword from offset 8 of the stack into d0, and branches to an offset of 28 relative to the program counter.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `load_pointer_and_branch`: Loads a pointer from offset 4 of the stack into a0, clears d0, and branches to an offset of 54 relative to the program counter.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `compare_memory_buffers`: As library code, it loads two pointers from the stack and branches to another routine.
- `skip_non_zero_byte`: As library code, it tests a byte at the pointer in a0 and increments the pointer; otherwise it returns.
- `compare_and_branch_bytes`: As library code, it compares a byte from a pointer in a0 with a byte from a pointer in a1, and if they are equal, it compares a byte from a pointer in a1 with a byte from a pointer in a0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_pointer_and_branch`: As library code, it loads a pointer from the stack into a0, loads a byte from the stack into d0, and branches to another routine.
- `compare_and_load_pointer`: As library code, it compares d1 with d0; if they are not equal, it loads the value from a0 into d0 and branches to another routine.
- `increment_and_test_byte`: As library code, it increments the pointer in a0, compares the byte at the new address with d1, and if the byte is non-zero, sets d0 to zero and returns.
- `process_pointer_with_boundary`: As library code, it saves registers to the stack, loads a value from the stack into d3 and a byte into d2, calls another routine, and then modifies a0 using d3 before branching.
- `compare_byte_and_load`: As library code, it compares the byte at the pointer in a0 with d2; if they are not equal, it loads the value from a0 into d0 and branches to another routine.
- `check_pointer_boundary`: As library code, it decrements the pointer in a0, compares it to d3, and if the pointer is less than d3, sets d0 to zero and restores d2 and d3 from the stack before returning.
- `search_boundary_values`: Library code that attempts to call another routine twice with different constant arguments, returning the incremented result of a call if either attempt returns a non-zero value.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### MegaBall (1991)(E. Mackey)(SW)[a].adf:load

- `free_blocks_and_clear_field`: Runs a short loop that releases memory blocks through the system FreeMem entry, then zeroes a longword field in the caller's structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `decompress_and_accumulate_data`: It calls a routine multiple times using a pointer in a4, performs arithmetic on a value in d5, and uses a loop to add values from a table into a destination buffer based on results from another routine.
- `load_address_and_set_d0`: Loads a 32-bit address into a1 from a location relative to the program counter and sets d0 to 2.
- `decompress_variable_length_data`: Decompresses a stream of data using a variable-length encoding scheme where bytes are expanded based on bit-shifts and repeated values.
- `push_registers_to_stack`: It stores the bytes from d0, d1, and d2 onto the stack at the location pointed to by a4, then restores d0 through d3, d5 through d6, and a3 through a4 from the stack before branching to another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### MegaBall (1991)(E. Mackey)(SW)[a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
