# Game 1553 (llamatron (1991)(llamasoft)(pd)[shell - run llama]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01553). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901568 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 259 of 259 named (0 author symbols), 259 of 259 described (13 by rule, 228 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 12 of 12 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 579766 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5416 of 5416 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 3322 |
| data | 674964 |
| padding | 3122 |
| unused | 220160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ppmore | 2 | 233 | 8 |
| echo | 2 | 117 | 2 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA512 | 1 | 17 | 2 |
| arp.library | 3 | 605 | 68 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA | 1 | 17 | 2 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 121 | 1 |
| ppmore#unpacked | 3 | 4292 | 173 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON1MEG (raw code file, load address unknown) | 1 | 2 | 1 |
| Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON512 (raw code file, load address unknown) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:c/ppmore: crunched executable decrunched by XFDMaster (PowerPacker 3.0 (xfd), 11116 -> 17216 bytes), 9 of 9 windows confirmed in reference RAM: recovered as a hunk executable
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON1MEG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON512: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- llamatron (1991)(llamasoft)(pd)[shell - run llama]/Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:c/ppmore: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 579766 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_and_dispatch_string_data` (ppmore#unpacked): Saves registers, performs several calls with various parameters including values from a5, enters a loop that writes a byte to a structure in a5 and increments a counter, and finally performs an indirect jump using a value from the stack. Kind: checksum/data integrity trap. Pass path: the branch where the loop completes normally without triggering the indirect jump/crash.

## Functions


### ppmore

- `load_executable_hunks`: Loads an AmigaDOS executable from the ppmore file.
- `wait_for_countdown_loop`: Runs a countdown loop on a 16-bit count until it reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_backward_stream_step`: Part of a backwards bit-stream unpacker in ppmore.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `decode_bit_stream_values`: Reads bits from a packed longword stream, shifting by a count held in a data register and refilling from memory when the register empties.
- `subtract_word_then_trap`: Subtracts one register from the word at a pointer and then issues trap six.

### echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_table_then_call_pointer`: When an output pointer is supplied, copies sixteen longwords from a fixed table in the echo program into it, then calls a routine through a pointer.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA512

- `forward_to_unnamed_routine`: The decompiler gave no pseudo-code.
- `open_dos_and_read_file`: Opens the dos library through the exec library, then opens and reads from a file, with dos.library and tron512 referenced in its text.

### arp.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 39 into d0, then returns.
- `collect_library_entries_until_null`: Library code from arp.library.
- `initialize_list_header_node`: Library code from arp.library that initialises a list node.
- `setup_library_node_from_result`: Library code from arp.library.
- `poll_until_flag_then_count`: Library code from arp.library that repeats a library call at offset -462 until the zero flag is set, then increments a 16-bit counter in the structure.
- `clear_slot_then_call_library`: Library code from arp.library.
- `repeat_library_call_until_zero`: Repeatedly calls the routine at a fixed displacement from a6 until the zero flag is set, then decrements the 16-bit counter at offset 32 of a6 and returns zero in d0.
- `cleanup_library_and_memory`: As library code, it checks a 16-bit value at offset 32 of a structure in a6, sets bit 3 of a byte at offset 14 of a6, and if certain conditions are met, calls routines to free memory, close multiple libraries, remove a node, and free the structure itself.
- `call_library_function_98`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 98.
- `call_library_function_92`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 92.
- `call_library_function_86`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 86.
- `call_library_function_80`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 80.
- `call_library_function_62`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 62.
- `call_library_function_56`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 56.
- `call_library_function_50`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 50.
- `call_library_function_44`: As library code, it loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus 44.
- `call_library_offset_38`: As library code, it loads the value 38 into d0 and branches to another location.
- `call_library_via_offset`: Loads a library base from offset 48 of the structure in a6 and calls the library function at the displacement -128 plus the word in d0, preserving a6 on the stack.
- `call_library_offset_246`: As library code, it loads the value 246 into d0 and branches to another location.
- `call_library_offset_240`: As library code, it loads the value 240 into d0 and branches to another location.
- `call_library_offset_234`: As library code, it loads the value 234 into d0 and branches to another location.
- `call_library_offset_228`: As library code, it loads the value 228 into d0 and branches to another location.
- `call_library_offset_222`: As library code, it loads the value 222 into d0 and branches to another location.
- `call_library_offset_216`: As library code, it loads the value 216 into d0 and branches to another location.
- `call_library_offset_210`: As library code, it loads the value 210 into d0 and branches to another location.
- `call_library_offset_204`: As library code, it loads the value 204 into d0 and branches to another location.
- `allocate_block_with_link_word`: Library code from arp.library.
- `free_memory_via_pointer`: If a1 is non-zero, reads the longword stored just before the pointer in a1 as a size and passes the pointer and size to the memory-freeing function through the exec base.
- `execute_raw_formatted_output`: As library code, it calls a function through a vector at minus 60 from a6 and, when that returns non-zero, passes the string in a0 and the argument pointer in a1 to the exec RawDoFmt library function with a stack buffer of 80 bytes.
- `update_string_buffer`: Stores the byte in d0 at the index held in a structure through a3, advances that index as a 16-bit value, and when the byte is zero or the index passes 79 with a non-zero first byte, calls a function through another structure and adds its result to a running total, then clears the index.
- `format_then_measure_string`: Calls the exec RawDoFmt function with the arguments already in registers, then scans the byte string at the address in d0 counting bytes until the null terminator and returns the inverted count in d0.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `allocate_msg_port`: As library code, it calls a routine to allocate a signal, then calls another routine to initialize a structure, and if successful, sets several fields in a message port structure before returning it.
- `initialize_library_node`: Restores registers from the stack, stores a byte at offset 8 of the structure in a1, copies a longword from offset 4 of a pointer in a6 to offset 16 of the structure in a1, pushes a1 to the stack, and calls another routine if a longword at offset 10 of the structure in a1 is not zero.
- `initialize_linked_list_node`: Sets the longword at offset 20 of the structure in a1 to point to itself and then adds 4 to it, zeroes the longword at offset 24, and stores the adjusted pointer at offset 28; it then restores the saved registers and returns.
- `remove_message_port`: If the structure in a1 is not zero, it checks if a longword at offset 16 of the structure in a1 is not zero, calls another routine, loads a byte from offset 15 of the structure in a1 into d0, calls another routine, and performs an indirect jump.
- `copy_structure_data`: Stores a3 on the stack, sets a3 to offset 20 of the structure in a1, stores a3 at offset 10 of the structure in a1, saves a1 and a2 to the memory pointed to by a3, stores d0 at offset 8 of the structure in a3, and enters a loop that clears memory at the location pointed to by a3 for 7 iterations if d0 is not zero.
- `copy_longword_block`: Copies longwords one at a time from the source pointed to by a0 to the destination pointed to by a3, looping over the count held in d1, then restores a3 from the stack and returns.
- `dispatch_message_and_task`: As library code, it calls a routine to put a message, then calls another routine to retrieve a pointer from a task structure, and finally calls a routine at an address stored in a register.
- `execute_library_operations`: Calls two different routines using a2 as a pointer in a0, compares the resulting longword in d0 with a7, and if they are not equal, performs an indirect jump using a value from a6.
- `copy_structure_long_to_slot`: Loads the longword at offset 32 of the structure in a1 as a return value in d0, then stores the longword at offset 36 of that structure into offset 148 of the structure in a4, and restores the saved registers before returning.
- `conditionally_push_address_register`: If the zero flag is clear, pushes a1 onto the stack, loads a1 with a PC-relative address, and returns; otherwise it returns with no other effect.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `compare_and_concatenate_strings`: Checks if a byte at a pointer in a1 is non-zero; if so, it compares a byte from a pointer in a0 with a byte from a pointer in a1, and if they match, it continues a loop to check subsequent bytes.
- `skip_byte_if_nonzero`: As library code, it checks a byte at a pointer in a0; if it is non-zero, it returns, otherwise it checks a byte at a pointer in a1, increments that pointer, and branches back to the check if the byte is non-zero.
- `copy_bytes_loop`: Copies bytes from a source in a1 to a destination in a2 in a loop, and then performs a final byte copy when the first loop terminates.
- `append_string_to_buffer`: Calculates the length of a new string and compares it against the capacity of a buffer stored in a structure.
- `clear_stack_value_and_call`: As library code, it stores zero in a 32-bit stack slot, calls another routine, and returns that stack value to its caller.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies a6 to d0, then returns.
- `loop_until_nonzero_flag`: Calls a function through a pointer at a displacement of -372 from the library base in a6 and loops if the zero flag is not set.
- `process_library_result_and_update`: As library code, it calls a library function at offset minus 486 from a6, and if the result is zero it calls another library function at offset minus 198 from a6, then stores the results into a structure at a1 and returns.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `process_library_result_with_pointer`: As library code, it calls a library function at offset minus 486 from a6, and if the result is non-zero it calls another library function at offset minus 344 from a6, then stores the first result in a structure at a1 and writes the word 5 at offset minus 2 of that structure before returning.
- `call_library_and_update_structure`: As library code, it calls a library function at offset minus 486 from a6, and if the result is non-zero it calls another library function at offset minus 30 from a6, then stores the result in a structure at a1 and writes the word 2 at offset minus 2 of that structure before returning.
- `call_library_subroutine`: As library code, it calls a subroutine at an offset from a6, then branches to another location based on the result, otherwise it pushes a pointer from a1 to the stack and calls another library subroutine before branching.
- `update_structure_from_register`: As library code, it stores the value in d0 into a structure pointed to by a1, and if the result is non-zero it writes the word 1 at offset minus 2 of that structure before returning.
- `allocate_block_or_return_zero`: As library code, it saves d0, d1 and a0 on the stack, calls a library function with a size of 16 in d0, and copies the result to a1.
- `call_two_functions_and_relink_list`: As library code, it calls two library functions at offsets minus 462 and minus 468 from a6, and if the second result is non-zero it calls a third function at offset minus 342, then swaps the forward and back pointers of list entries to insert the returned node, and returns the adjusted pointer with the d0 value.
- `allocate_structure_and_set_pointers`: As library code, it loads the size 410 into d0, calls a library function at offset minus 384 from a6, and if the result is non-zero it writes two pointers at offsets 8 and 4 of that structure, a byte 1 at offset 17, and the word 14 at offset minus 2, then returns.
- `save_pointer_and_branch`: As library code, it saves d0 and a2 on the stack, moves the value from a0 to a2, and branches to another location.
- `unlink_and_return_list_node`: As library code, it calls a library function through the library base to obtain a node pointer, and if that pointer is not null it increments the nesting counter in the exec library base, relinks the node's neighbours to each other, calls another routine, then calls a second library function through the library base.
- `call_library_via_pointer_offset`: As library code, it loads a pointer from an offset of a1, checks if the value at that pointer is non-zero, and if so calls a library function at offset minus 480 from a6 using a pointer derived from that value.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `traverse_pointer_list`: As library code, it saves a2 to the stack, loads a value from an offset of 12 from a2 into a2, and branches to another routine.
- `traverse_pointer_list_cleanup`: As library code, it saves a2, moves a1 into a2 and the value in d0 into a2, calls another routine with the structure in a1 as its argument, then loads the longword that a2 points to into d0 and repeats while that value is non-zero.
- `unlink_and_dispatch_node`: As library code, it compares a byte at offset 9 of a1 to 255 and branches if they are not equal.
- `clear_and_branch_on_bit`: As library code, it clears a long value at an offset of a2, then checks bit 7 of a byte at offset 8 of a1, and if that bit is set it calls a library function at offset minus 348 from a6.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:LLAMA

- `open_file_and_initialize`: Opens the DOS library via the exec library and reads data from a specific file.
- `open_dos_and_read_file_2`: Opens the dos library, then opens and reads a file whose text is tron1meg.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_then_call_library_vectors`: Calls exec.Wait and then a series of library vectors at offsets from -30 to -222.

### ppmore#unpacked

- `jump_to_address`: Performs an absolute long jump to the address stored in the first word of the current instruction.
- `allocate_system_block`: Allocates a memory block of 65536 bytes, stores a value and two pointers into that block, calls another routine, and then performs a cleanup using the released resources before freeing the block.
- `release_system_resources`: Writes two specific values to a memory location pointed to by a value retrieved from a global structure and then executes a function pointer from a memory location.
- `execute_jump_table_entry`: It iterates through a table of entries, checking a byte at an offset of a4, and if the byte is zero it uses the value in d0 to jump to an address found at an offset of a0.
- `copy_string_with_limit`: It decrements the stack pointer, calls a subroutine, and then enters a loop that copies bytes from a source buffer to a destination buffer as long as the count is less than 256, the source byte is not zero, and the count is less than a value at an offset of a5.
- `clear_byte_at_offset`: Compares a 16-bit value at offset 2 of the stack to 256; if it is not greater, it clears a byte at the address stored in a pointer from the stack at offset 12 using an index from the stack at offset 2, then jumps to an address from the stack.
- `initialize_and_execute_dispatch`: It clears several bytes and words in a structure at an offset of a5, performs multiple memory moves using d0-d4, calls three library functions using pointers from a6, and finally performs an indirect jump using a pointer from a5.
- `jump_to_stack_address`: Increments the stack pointer by 6, loads an address from the stack, increments the stack pointer by 4, and jumps to that address.
- `call_library_and_jump`: Loads a pointer from the stack, calls a library function using a pointer from a table at offset 19800 of a5, and then jumps to an address from the stack after updating the stack pointer.
- `unpack_and_allocate_resources`: Copies a block of memory using a loop, then allocates multiple memory blocks of varying sizes by calling system allocation routines.
- `parse_and_load_resource`: Parses a file path string to extract a filename between specific delimiters and attempts to load the resulting resource.
- `update_structure_and_jump`: Updates a 16-bit value in a structure at offset 19690 of a5, potentially calls a routine to allocate memory, updates a pointer at offset 19692 of a5, and finally performs an indirect jump through a pointer from the stack.
- `free_linked_list_items`: It iterates through a linked list and frees each memory block until the list is empty by calling a subroutine for each node and updating the pointer at an offset of a5.
- `execute_library_calls`: Calls three library functions using pointers from a table at offset 19784 and 19700 of a5, and if a byte at offset 4 of the stack is non-zero, calls a fourth library function using a pointer from a table at offset 19700 of a5 at a specific offset, then jumps to an address from the stack.
- `reset_state_and_call`: Calls a function from a table twice and then clears two word-sized values in different memory locations.
- `configure_and_render_bitmap`: Calculates bitmap dimensions and bitplane requirements based on source data, then performs a loop to decompress and populate pixel data into a planar format.
- `clear_flagged_callbacks`: Checks two different status flags and, if non-zero, executes a corresponding function and resets the flag to zero.
- `process_file_buffer`: Compares two longwords in a structure, calls a library function using a pointer from the structure, and then either clears a longword and a byte or enters a loop to copy bytes from a table into a structure while incrementing two counters and finally jumps to an address from the stack.
- `build_status_string`: Constructs a character buffer by evaluating bitwise flags and numeric values from a global structure.
- `calculate_and_dispatch_buffer_offsets`: Calculates a byte offset by scaling a value from a structure in a5 and calls another routine with pointers derived from a structure in a7.
- `process_and_dispatch_buffers`: It calculates several offsets based on values in a5, calls a subroutine, and enters a loop that moves data between pointers stored in a structure, incrementing those pointers and decrementing a counter until it reaches zero, before performing an indirect jump.
- `dispatch_jump_table`: Copies a sequence of data from a table to the stack, calls a subroutine, updates a value in a structure in a5, and then iterates through a table to modify bytes in a stack-based buffer before performing an indirect jump using a pointer from the stack.
- `toggle_byte_and_wait`: Sets a byte to a specific value, executes an indirect function, and then resets that byte to zero based on certain conditions.
- `update_and_dispatch_pointer`: Calls a subroutine, and if the return value is non-zero, updates a pointer and a counter in a structure in a5, calculates a new address using a value from a structure in a4, and if a condition is met, calls another subroutine before performing an indirect jump.
- `advance_linked_buffer_queue`: Advances a linked list of buffers by updating a pointer in a structure and decrementing a counter.
- `decrement_and_call_subroutine`: Decrements a word at offset 13910 of a structure in a5, calls a subroutine using a pointer from a structure in a4, and if a byte on the stack is non-zero, it pushes a byte from a structure in a5 to the stack, calls another subroutine, and then calls a third subroutine using values from a structure in a5.
- `process_and_dispatch_buffers_incremented`: It calculates offsets using an incremented value from a5, calls a subroutine, and enters a loop that decrements a counter and moves data between pointers, then performs an indirect jump using a pointer from the stack.
- `update_buffer_and_call_subroutine`: Checks a word at offset 13910 of a structure in a5, updates a value at offset 10 of a pointer from a5, decrements the value at offset 13910 of a5, and if a pointer at offset 28 of a5 is non-zero, calls another routine with several parameters.
- `execute_data_copy_loop`: Calls a subroutine to perform a calculation, then enters a loop that decrements a counter in a table and copies multiple longwords from one memory area to another.
- `format_numeric_string`: Decrements a value on the stack, calls a routine, and if the result is within a specific range, enters a loop that calls a routine twice to convert values into characters and store them in a table, or fills a table with spaces.
- `jump_via_stack`: Increments the stack pointer by 6, loads an address from the stack, sets the stack pointer to an offset of 12 from the current stack pointer, and jumps to that address.
- `clear_and_jump_to_pointer`: It clears two long words at offsets of a5, checks a value on the stack, and if non-zero, increments a word at an offset of a5, calculates a value from a pointer at an offset of a4, adds it to a long word at an offset of a5, and jumps to an address from the stack.
- `jump_to_stack_pointer`: Loads an address from the stack, increments the stack pointer by 4, and jumps to that address.
- `advance_pointer_in_loop`: Calls a subroutine, then enters a loop that compares a stack value to a value at offset 1590 of a5 and updates a pointer in a5 from a structure if the pointer is non-zero.
- `loop_through_linked_list`: Compares a word from the stack with a value at offset 1590 of a5, and if it is less, enters a loop that updates a pointer from the stack and increments a word on the stack.
- `iterate_and_call_subroutine`: Stores a value from the stack into offset 32 of a5, calls three subroutines, and then enters a loop that calls another routine while incrementing a value on the stack, provided a value at offset 1590 of a5 is greater than the stack value.
- `set_byte_to_zero`: Sets a byte at offset 18032 of a5 to zero and returns.
- `wait_on_flag`: Checks if a longword at offset 20 of a structure in a5 is non-zero; if so, it sets a byte at offset 18461 in a5 to one, calls a routine using a pointer from offset 18456 in a5, sets that same byte to zero, and then performs an indirect jump using a pointer from the stack.
- `copy_and_update_table`: Copies a longword and then a sequence of 67 longwords and one word from a source to a destination, updates a pointer in a structure, and enters a loop that copies bytes from a source table into a structure based on a value from a stack-based table.
- `copy_data_from_table`: Calculates a value by subtracting a byte from 17, stores it at offset 260 of a structure, and then enters a loop that copies bytes from a table into that structure until a specific byte is encountered.
- `update_pointers_and_jump`: Copies bytes from a source structure into a destination structure at offset 260 using a loop controlled by a byte from the stack, then updates two potential pointer locations at offsets 392 and 396 of a structure in a4 based on the address of that structure, and finally performs an indirect jump using a pointer from the stack.
- `assign_structure_pointers`: If a pointer at offset 392 of a structure in a4 is non-zero, it stores a longword from offset 518 of the structure in a4 into offset 4 of the pointed-to structure; if a pointer at offset 396 is non-zero, it stores the longword from offset 518 of the structure in a4 into the location pointed to by that pointer, then performs an indirect jump using a pointer from the stack.
- `update_pointer_and_dispatch_2`: Checks a byte at offset 410 of a structure in a4, calls a routine using a pointer from a5, and if a longword at offset 4 of a structure in a4 is non-zero, copies that longword to offset 32 of a5 before branching.
- `update_pointer_and_dispatch`: Copies a longword from offset 4 of a structure in a4 to offset 32 of a5, calls another routine, and if a byte at offset 522 of a stack pointer is non-zero, it pushes the stack pointer, calls another routine, and performs an indirect jump using a pointer from the stack.
- `update_structure_field`: Clears a word at offset 12 of a structure in a4, loads a word from offset 8 of a structure in a4 into d7, compares d7 with a word at offset 16216 of a5, and if d7 is less than that value, stores a word from offset 6 of a structure in a3 into a structure in a3 before branching.
- `compare_and_update_string`: Compares a character from a table against a value in a structure, increments two fields in that structure, and calls another routine if a specific byte is non-zero before performing an indirect jump.
- `increment_structure_field`: Increments a 16-bit word at offset 12 of a structure pointed to by a value from the stack, then branches to another routine.
- `perform_indirect_jump`: Clears d0 and performs an indirect jump using a pointer retrieved from the stack.
- `process_and_dispatch_string_data`: Saves registers, performs several calls with various parameters including values from a5, enters a loop that writes a byte to a structure in a5 and increments a counter, and finally performs an indirect jump using a value from the stack.
- `update_and_wait_for_task`: Updates a value in a structure, populates a local data block with several constants and pointers, and calls a routine to initiate a task.
- `initialize_data_table_entries`: Calls another routine repeatedly with various pointers from a table, then calculates a value based on a word at an offset from a structure in a5 and uses it as a loop counter to call another routine and decrement that counter until it reaches zero or less.
- `process_messages_in_loop`: Calls two routines using a pointer from a table, stores a longword in a global variable, and if bit 2 of offset 34 of a structure in a4 is set and offset 36 is not less than 128, calls another routine using a pointer from a global variable and repeats the routine in a loop.
- `reply_message_and_dispatch`: Loads a pointer from offset 17996 of a5 into a1, calls a routine using a pointer from a6, copies a longword from offset 28 of a5 to offset 32 of a5, calls another routine, and performs an indirect jump using a pointer from the stack.
- `iterate_linked_list`: Calls a subroutine, copies a longword from offset 40 of a structure in a5 to offset 32 of a5, and enters a loop that updates the longword at offset 32 of a5 with the value from offset 4 of that same structure, provided a counter at offset 12 of a structure in a4 is greater than 1 and the target pointer is non-zero.
- `call_and_dispatch`: Calls another routine and then performs an indirect jump using a pointer from the stack.
- `check_status_and_branch`: Stores a byte from offset 66 of a pointer from the stack into offset 18065 of a5, and if a longword at offset 20 of a5 is non-zero or if a longword at offset 32 of a5 is non-zero, it branches to another routine; otherwise, it checks a byte at offset 5 of a pointer from the stack and branches to another routine if it is non-zero, or sets d0 to negative one and branches to another routine.
- `store_byte_and_branch`: Loads a pointer from offset 4 of the stack into a4, stores a byte from offset 5 of that pointer into a4, and branches to another routine.
- `update_status_and_call_subroutine`: Checks a byte at offset 4 of a pointer from the stack, checks a byte at offset 18032 of a5, and if certain conditions are met, calls another routine and updates the byte at offset 18032 of a5.
- `set_byte_and_jump`: Stores a byte from a pointer in a stack parameter into a global variable at offset 4670 of a structure in a5, then performs an indirect jump using a pointer from the stack.
- `update_status_flags`: Copies a byte from offset 66 of a structure to offset 18111 of another structure, then checks a byte at offset 4 of a structure to conditionally perform a bitwise sequence operation on a byte at offset 4670 of the second structure.
- `set_status_and_jump`: Stores a byte from offset 4670 of a structure in a5, sets the byte at offset 4 of a pointer from the stack to the status flag, and performs an indirect jump using a pointer from the stack.
- `wait_for_message_and_process_queue`: Waits for a message on a port, retrieves it, and copies thirteen words from the message structure into a local buffer.
- `set_status_byte`: Sets a byte at offset 17896 of a structure in a5 and branches to another routine.
- `update_status_and_branch`: Moves a byte from offset 17897 of a structure in a5 to the stack, calls another routine, loads a pointer from offset 28 of a5 into a4, tests the longword at offset 4 of the pointer, sets bit 7 of the byte at offset 17896 of the structure in a5, and branches to another routine.
- `process_message_queue`: Waits for a message on a port, retrieves it, and copies thirteen words from the message structure into a local buffer.
- `process_message_and_update_status`: Copies thirteen words from a structure in a5 to the stack, calls a library routine with a pointer from that structure, and updates several status bytes in a structure based on the results of bitwise tests on the message content.
- `process_event_status_and_dispatch`: Checks a 16-bit value at offset 56 of the stack and, if it equals 15, performs several bitwise tests on the stack to update status bytes in a structure pointed to by a5.
- `test_and_call_routine`: Tests bit 5 of a byte at offset 40 of the stack, and if it is not set, calls another routine before branching to another location; otherwise it branches to another location.
- `validate_and_call_subroutine`: Checks bit 2 of a stack value and verifies if a 16-bit stack value is between 0 and 127; if the value equals 64, it calls another routine and branches to a new location based on the result, otherwise it returns.
- `compare_and_call_routine`: Compares a 16-bit value in d7 to 65 and, if not equal, branches to another routine; otherwise, it pushes the stack pointer to the stack and calls another routine before branching to a third location.
- `validate_command_id_and_status`: Compares a 16-bit value in d7 against specific constants and, if a match is found, checks several bytes on the stack and in a global structure.
- `store_status_bytes`: Stores bytes from offsets 4670 and 4671 of a structure and a byte from offset 4 of the stack into a destination, then branches to another routine.
- `check_and_decrement_counter`: Checks a byte at offset 67 of the stack, compares a word at offset 38 of the stack to 31, and if it matches, decrements a word from offset 1590 of a5 and stores it on the stack.
- `decrement_and_branch_conditional`: Writes a word to the stack, checks a byte at offset 42 of the stack, calls another routine, decrements a word at offset 12 of the stack, and branches based on the results.
- `filter_input_command_id`: Compares a 16-bit value in d7 against several constants and, if it matches specific values, performs conditional checks on stack data and global variables to update status bytes or decrement counters before calling other routines.
- `write_status_bytes_and_branch`: Stores two consecutive bytes into a global variable via a pointer in a5, updates the carry flag based on the last byte stored, and branches to another routine.
- `check_and_decrement_counter_alternate`: Checks a byte at offset 67 of the stack, compares a word at offset 38 of the stack to 63, and if it matches, decrements a word from offset 1590 of a5 and stores it on the stack.
- `decrement_and_call_buffer_queue`: Decrements a 16-bit value on the stack, checks a byte at offset 66 from the stack pointer, calls advance_linked_buffer_queue, and then branches based on the result of a status flag check and the stack value.
- `check_and_set_status_flag`: Compares the word in d7 to 68 and, if equal, clears a byte on the stack, calls a subroutine, and sets the sign flag of a byte at offset 1776 of a structure in a5; otherwise, if d7 is 67, it branches to another routine, or it branches to another routine if d7 is neither 68 nor 67.
- `update_and_call_subroutine`: Compares the word in d7 to 61 and, if not equal, branches to another routine; otherwise, it moves a longword from offset 24 of a structure in a5 to offset 32 of the same structure, calls a subroutine, and then branches to another routine.
- `compare_and_call_subroutine_2`: Compares the word in d7 to 29 and, if not equal, branches to another routine; otherwise, it pushes the current stack pointer to the stack, calls a subroutine, and then branches to another routine.
- `update_linked_list_node`: Checks if a value in d7 is between 80 and 89, calculates an index by subtracting 80, and uses it to retrieve a pointer from a table at offset 19066 of a5.
- `write_structure_member`: Shifts a 16-bit word from the stack left by two bits, uses the result as an offset to write a 32-bit long word from a structure into another structure, and then branches to another routine.
- `compare_and_call_subroutine`: Compares a word in d7 to 95, and if not equal, branches to another routine; otherwise, it pushes the current stack pointer to the stack, calls another routine, and branches to another routine.
- `evaluate_register_conditions`: Compares a word in d7 to 69, and if equal, branches to another routine; otherwise, it compares d7 to 46, and if not equal, branches to another routine; if equal, it tests a byte at offset 67 of the stack, and if not zero, branches to another routine, otherwise it branches to another routine.
- `validate_and_update_pointer`: Checks if a stack value is less than 64, uses it as an index into a table to retrieve a byte, and if that byte equals 116, updates a pointer in a structure and calls another routine.
- `branch_on_byte_match`: Compares a byte in register d7 to 98, and if they are not equal, branches to another routine; otherwise, it pushes the current stack pointer to the stack, calls another routine, and branches to another routine.
- `convert_string_to_uppercase`: Checks if a byte in a register matches specific values, then prepares a task structure by calling another routine, clears a field in a global structure, and iterates through a list of characters to convert them to uppercase by subtracting 32 if they fall within the lowercase range.
- `increment_index_on_condition`: Loads a word from a stack location into a register, tests a byte at an offset determined by that word, and if the byte is zero, increments the word at the stack location before branching to another routine; otherwise, it returns.
- `pop_linked_list_node_2`: Updates a pointer in a structure by reading the value from the address stored at offset 19106 of that structure, then decrements a 16-bit counter at offset 12 of a stack frame if the new pointer does not match the pointer at offset 32 of the structure.
- `pop_linked_list_node`: Compares a longword at offset 32 of a structure in a5 with a value at offset 19106 of the same structure; if they match and the value is zero, it replaces the pointer at offset 32 with the pointer from offset 28, then calls another routine and updates the pointer at offset 32 with the value pointed to by the new pointer.
- `traverse_linked_list_node`: Follows a pointer at offset 32 of a structure in a5 to a new structure, then attempts to update that pointer with the value found at offset 4 of the new structure.
- `clear_pointer_and_branch`: Clears a longword at offset 32 of a structure in a5, pushes the current stack pointer onto the stack, calls another routine, and then branches to another routine.
- `calculate_total_length_and_signal_task`: Checks if a character is 'w' and verifies several conditions in a structure, then iterates through two linked lists to sum their lengths into structure offsets.
- `update_global_value_from_structure`: Loads a pointer from an offset of a structure in a5 into a4, moves a longword from the address pointed to by the stack pointer into the stack, and if that longword is non-zero, adds a sign-extended 16-bit word from offset 8 of a structure in a4 to a value at offset 12258 of a structure in a5 before branching back to an earlier instruction in this routine.
- `decrement_message_count_and_update`: Decrements a 16-bit value in a structure and, if the value remains greater than one and the pointer is non-zero, adds a 32-bit value from a message to a global accumulator before popping the stack and branching.
- `signal_task_and_wait_for_message`: Subtracts a value from a longword in a structure, and if the result is positive, prepares a stack frame to call two subroutines, signals a task using a pointer and a bitmask from the structure, and then enters a loop that waits for messages from a port and processes them based on flags in the structure.
- `process_task_command_character`: Checks if a byte in d7 equals 111; if so, it performs several checks on stack and structure offsets, copies data from a source buffer to a destination buffer via multiple calls to another routine, sets a byte in a structure, and signals a task.
- `handle_task_command_signal`: Checks if a byte in register d7 equals 108, then calls two subroutines, performs library calls using pointers from a table in a5, and enters a loop that waits for a task signal based on a bitmask and a flag in a structure.
- `process_task_list_and_signal`: Checks if a byte in register d7 equals 103, then pushes parameters onto the stack to call a subroutine.
- `branch_to_subroutine`: Calls a subroutine and then performs a branch to another location.
- `check_status_and_call_handler`: Compares the byte in register d7 to 104; if it matches, it pushes the current stack pointer to the stack, calls another routine, and then branches to another location, otherwise it branches to another location.
- `compare_status_byte`: Compares the byte in d7 to the decimal values 113 and 120, and branches to another location if either match; otherwise it branches to another location.
- `check_port_and_signal_task`: Checks if a byte in d7 is 115 and if two specific bytes in the structure pointed to by a5 are zero; if so, it attempts to find a message port, and if the port is not found, it signals a task and waits on a message.
- `signal_and_wait`: Stores a byte at offset 465e of a structure in a5, calls a routine to signal a task using a pointer and a value from the structure, and then enters an infinite loop by calling a wait routine.
- `parse_file_path_and_allocate`: Parses a file path string to separate the directory from the filename by searching for path delimiters, then copies the directory and filename into separate buffers within a structure and attempts to allocate memory for file operations.
- `assign_global_to_pointer`: Stores the address of a global variable into a pointer location, calls another routine, and then re-assigns the global address.
- `initialize_system_subsystems`: Increments a 16-bit counter in a global structure and, if it is the first call, opens several system libraries, allocates signals, and populates a large configuration structure with pointers and status flags.
- `shutdown_system_subsystems`: Decrements a 16-bit counter in a global structure and, if it reaches zero, performs a cascading cleanup of system resources including removing tasks, freeing signals, closing libraries, performing I/O operations, closing devices, and removing ports.
- `initialize_library_on_first_call`: Increments a global counter and, if it is the first call, opens a library and initializes a structure with the returned pointer, then jumps to a location in a structure if the pointer is not zero.
- `decrement_and_close_library`: Decrements a word at offset 19192 of a5 and, if it is not zero, branches; otherwise, it checks a longword at offset 19188 of a5, and if it is not zero, loads it into a1, loads a longword from offset 4 into a6, calls a routine at an offset from a6, and jumps to a longword address.
- `calculate_signed_multiplication_result`: Calculates the sign of two 32-bit values in d0 and d1, negates them if they are negative, swaps their high words, and performs a 32-bit multiplication using the high words and the low words if certain conditions are met.
- `multiply_and_negate`: Performs two signed word multiplications using values in d0 and d1, then optionally negates the result in d0 based on the value of d3 before returning.
- `divide_signed_long`: Calculates the signed division of two 32-bit values in d0 and d1, using a loop for large divisors and a word-based method for smaller ones, then returns the result.
- `perform_fixed_point_division`: Executes a loop that performs bitwise shifts and subtractions to divide a value by a divisor using a fixed number of iterations.
- `allocate_and_add_node`: Allocates memory for a node, sets its type, adds it to a list, and performs an indirect jump using a pointer from the stack.
- `remove_and_free_node`: If a pointer is not zero, removes a node from a list, frees the memory associated with it, clears the pointer, and performs an indirect jump using a pointer from the stack.
- `set_value_and_call`: Stores a constant value into a memory location and then calls another routine.
- `store_function_pointer`: Loads a global structure pointer into a4 and derives a second pointer from offset 114, then a third from offset 2e.
- `initialize_task_structure`: Increments a counter in a global variable and, if it equals one, initializes a structure with several values and pointers, including a function pointer and a stack address, then optionally waits for a message port and retrieves a message.
- `free_and_cleanup_resources`: Decrements a counter in a global variable and, if it reaches zero, iterates through a linked list to free memory, updates several fields in a structure, and calls other routines to forbid and reply to a message.
- `initialize_intuition_library`: Increments a word at offset 19788 of a5 and, if it is not one, branches; otherwise, it calls two routines, opens a library with a specific ID, stores the resulting pointer in offset 19784 of a5, and if the pointer is not zero, clears a longword at offset 19716 of a5, loads a pointer from offset 19708 of a5 into a0, and jumps to it.
- `decrement_and_cleanup`: Decrements a word at offset 19820 of a5 and, if it is not zero, branches; otherwise, it checks a longword at offset 19816 of a5, and if it is not zero, loads it into a1, loads a longword from offset 4 into a6, calls a routine at an offset from a6, calls another routine, and jumps to a longword address.
- `increment_task_counter`: Increments a specific word in memory and calls another routine if the value reaches one.
- `decrement_and_jump`: Decrements a word at offset 19828 of a5 and, if it is not zero, branches; otherwise, it jumps to a longword address.
- `manage_dos_library_lifecycle`: Increments a 16-bit counter in a structure and, if it reaches one, opens a library and stores the pointer; otherwise, it decrements the counter and, if it reaches zero, frees resources and calls another routine.
- `call_copy_memory_block`: Calls another routine and preserves d1 through d7 and a2 through a6 on the stack.
- `copy_memory_block`: Moves data between memory locations using a series of loops and conditional checks to manage byte and word transfers.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `decrement_byte_cycle_counter`: Decrements a byte counter in d7 for each iteration of a loop controlled by d0, resetting it to a space character whenever it reaches zero, and returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (movep.w, nop) and then falls through into the code that follows, with no return of its own.
- `signal_task_via_list`: Traverses a linked list starting at the pointer in a1, checking if a byte at offset 9 is 16 and a word at offset 6 is 69; if so, it clears the word at offset 6, signals a task using a bitmask derived from a byte at offset 0, and sets the byte at offset 6 to 0.
- `pop_and_return`: Loads a pointer from the address in a0 into a0, then pops a long value into d0 and a long value into a3 before returning.
- `initialize_and_parse_buffer`: Clears two memory buffers by zeroing out longwords in a loop based on a count and offset provided in a structure, then parses a sequence of bytes from a source to perform indirect calls to other routines.
- `calculate_offset_from_byte`: Subtracts 17 from d0, extends the result to a word, and performs a series of arithmetic shifts and additions to compute a value in d4 before branching back.
- `decrement_byte_and_branch_on_non_zero`: Decrements a byte in register d0 and branches to another routine if the result is non-zero; otherwise, it loads a longword from a global variable into register a5 and branches to another routine.
- `decrement_counter_and_load_pointer_4`: Decrements a byte in register d0 and, if non-zero, branches to another routine; otherwise, it loads a longword from a global variable into a5 and branches to another routine.
- `decrement_byte_and_load_pointer_4`: Decrements a byte in register d0 and, if non-zero, branches to another routine; otherwise, it loads a longword from a global variable into a5 and branches to another routine.
- `decrement_byte_and_branch_to_pointer_load`: Decrements a byte in register d0 and, if non-zero, branches to another routine; otherwise, it loads a longword from a global variable into register a5 and branches to another routine.
- `decrement_byte_and_load_pointer_3`: Decrements a byte in a register and, if the result is non-zero, branches to another routine; otherwise, it loads a longword from a global variable into a register and branches to another routine.
- `decrement_counter_and_load_pointer_3`: Decrements a byte in register d0 and, if it is not zero, branches to another routine; otherwise, it loads a longword from a global variable into register a5 and branches to another routine.
- `decrement_byte_and_load_pointer_2`: Decrements a byte in register d0 and, if it is not zero, branches to another routine; otherwise, it loads a longword from a global variable into register a5 and branches to another routine.
- `decrement_counter_and_load_pointer_2`: Decrements a byte in a register and, if it is not zero, branches to another routine; otherwise, it loads a longword from a global variable into a register and branches to another routine.
- `decrement_and_load_pointer_2`: Decrements a byte in register d0 and, if it is not zero, branches to another routine; otherwise, it loads a longword from a memory location into a5 and branches to another routine.
- `decrement_and_load_pointer`: Decrements a byte in register d0 and, if it is not zero, branches to another routine; otherwise, it loads a longword from a memory location into a5 and branches to another routine.
- `decrement_byte_and_load_pointer`: Decrements a byte in register d0 and, if it is not zero, branches to another routine; otherwise, it loads a longword from a memory location into register a5 and branches to another routine.
- `decrement_counter_and_load_pointer`: Decrements a byte in register d0 and, if it is not zero, branches to another routine; otherwise, it loads a longword from a memory location into register a5 and branches to another routine.
- `decrement_and_load_a5_alt`: Decrements a byte in d0 and, if it is not zero, branches; otherwise, it loads a longword into a5 and branches.
- `decrement_and_load_a5`: Decrements a byte in d0 and, if it is not zero, branches; otherwise, it loads a longword into a5 and branches.
- `decrement_and_branch`: Decrements a byte in d0 and, if it is not zero, branches to an earlier instruction; otherwise it loads a constant into a5 and branches to another routine.
- `branch_to_buffer_fill`: Loads a constant into a5 and branches to another routine.
- `calculate_jump_address`: Modifies a value in d0 through arithmetic and bitwise operations, uses it as an offset to calculate an address relative to a2, and performs an indirect jump using a pointer in a5.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `scatter_bytes_with_stride`: Copies eight bytes from a source list in a6 to a destination buffer in a1, inserting a stride of d2 bytes between each write, then applies a bitwise AND mask from d7 to the final byte written and branches back to a previous instruction.
- `scatter_bytes_with_mask`: Reads seven bytes from a source list pointed to by a6 and writes them to a destination pointed to by a4 at intervals defined by d2, then performs a bitwise AND on the final written byte with d7 before returning via a branch.
- `distribute_and_mask_bytes`: Copies eight bytes from a source pointer into two destination buffers at specific intervals determined by a word in d2, then applies a bitwise AND mask from d7 to the final byte written to both buffers before branching.
- `scatter_bytes_to_parallel_buffers`: Copies eight bytes from a source pointer into two destination buffers at intervals defined by a word offset, then performs a bitwise OR on the final byte written to the second buffer before returning via a branch.
- `fill_parallel_memory_buffers_2`: Sets a byte to 255 at the current pointer locations in two buffers, then increments both pointers by 1 and repeats the process for eight iterations.
- `blit_interleaved_pixel_data`: Copies eight bytes from two source buffers into two destination buffers using an interleaved stride defined by a register, then applies a bitwise OR to a byte in the first destination buffer and a bitwise AND to a byte in the second destination buffer before branching to another routine.
- `unroll_parallel_buffer_write`: Writes the value 255 to a destination buffer at an offset and simultaneously copies eight bytes from a source buffer to a second destination buffer at the same offset, applying a bitwise AND mask to the final byte before branching.
- `scatter_bytes_to_parallel_buffers_2`: Reads eight consecutive bytes from a source pointer in a3 and writes them to a destination pointer in a4, applying a constant offset in d2 between each write, then performs a bitwise OR on the final byte with d6 before returning via a branch.
- `blit_interleaved_bytes`: Copies eight bytes from a source buffer into two destination buffers at interleaved offsets determined by a stride value, then applies a bitwise AND to the final byte of the first destination and a bitwise OR to the final byte of the second destination before branching to another routine.
- `fill_interleaved_memory_buffers`: Writes the value 255 to eight staggered memory locations by incrementing a pointer by a specified word offset after each write, then returns.
- `write_interleaved_bytes`: Reads eight bytes from a source pointer in a6 and writes them to two destination buffers in a1 and a4 using a stride defined by a word in d2.
- `copy_and_mask_parallel_buffers`: Copies eight bytes from a source pointer into two destination buffers at offsets determined by a value in d2, then applies a bitwise OR operation to the final byte in both buffers using a value from d6 before branching to another routine.
- `write_interleaved_bytes_2`: Writes the value 255 to a destination buffer at specific intervals defined by a register, while simultaneously copying eight bytes from a source buffer to a second destination buffer and applying a bitwise OR operation to the final byte.
- `write_interleaved_bytes_3`: Copies eight bytes from a source pointer into two destination buffers at staggered intervals determined by a word in a register, then performs a bitwise OR on the final byte written to the first buffer before branching to another routine.
- `fill_parallel_memory_buffers`: Sets a byte to 255 at the current pointer locations in two buffers, then increments both pointers by 1 and repeats the process for eight iterations using an offset in d2, before branching to another routine.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `expand_compressed_bitplanes`: Expands a compressed bitmap stream into eight planar bitplanes by shifting and OR-ing each source byte, advancing the source pointer by the value in d7 each step, then inverting the 1536 bytes following the destination buffer.
- `copy_memory_block_2`: Copies a block of data from a source pointer in a1 to a destination pointer in a3 using a loop.
- `copy_memory_block_reverse`: Copies a block of data from a source pointer in a3 to a destination pointer in a1 using a loop.
- `calculate_combined_values`: Multiplies two 16-bit values by a scaled version of a third value to calculate two new addresses and returns a 32-bit result composed of the modified values.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON1MEG (raw code file, load address unknown)

- `execute_initial_instructions`: This routine consists of two instructions at the start of a raw code region.

### Llamatron (1991)(Llamasoft)(PD)[Shell - run llama].adf:TRON512 (raw code file, load address unknown)

- `return_from_subroutine`: The routine consists of a single instruction that returns control to a caller.
