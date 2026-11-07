# Game 717 (BostonBombClub v1.2 1502): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00717). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1357081 top-level bytes in 46 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 10 raw-word lines |
| Functions named and described | pass, 469 of 469 named (0 author symbols), 469 of 469 described (20 by rule, 448 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3961 of 3961 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 115628 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8011 of 8922 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 386645 |
| code | 35134 |
| data | 935226 |
| padding | 76 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 1110 | 75 |
| T.X | 4 | 7684 | 330 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 23 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 7 |
| BOBOI.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| BOUM.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| PRESENT.CO (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- BostonBombClub_v1.2_1502/BostonBombClub_v1.2_1502.hdf: hard-disk image (AmigaDOS OFS, 1536 blocks): its filesystem parses and all 39 files in it are byte-identical to the extracted tree BostonBombClub_v1.2_1502_hdf, which is recovered file by file; no block outside the filesystem holds data
- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 54 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f300): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- BostonBombClub_v1.2_1502/BostonBombClub_v1.2_1502_hdf/BostonBombClub/data/BOBOI.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BostonBombClub_v1.2_1502/BostonBombClub_v1.2_1502_hdf/BostonBombClub/data/BOUM.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BostonBombClub_v1.2_1502/BostonBombClub_v1.2_1502_hdf/BostonBombClub/data/PRESENT.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 115628 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_globals_and_call_handlers`: Stores a function pointer and a register value into global variables, then calls two routines through pointers at offsets from that register value and finally jumps indirectly through a global pointer.
- `jump_through_unrecovered_table`: Performs an indirect jump through a computed table entry that could not be resolved statically, transferring control to another routine.
- `jump_through_unrecovered_table_2`: Performs an indirect jump through a computed table entry whose targets could not be resolved, so control transfers to an address taken from that table rather than continuing sequentially.
- `clear_bits_in_global`: Reads a global variable, clears bits 4 through 6, and returns the resulting value.
- `return_stack_argument`: Returns the value passed on the stack to the caller.
- `store_stack_argument_to_global`: Stores a word from the stack into a global variable.
- `return_zero_constant_2`: Returns the constant zero without performing any other operations or modifying data.
- `set_flag_in_struct`: Receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure, then returns.
- `return_immediately_15`: An empty routine that contains only a return instruction and performs no operations.
- `duplicate_structure_with_allocations`: Allocates a new structure, copies a word and a series of pointer/size pairs from the source, allocating memory for each non-null pointer; on allocation failure it frees previously allocated blocks and returns an error flag.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: Contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `store_register_to_global`: Copies the value from a data register into a global variable.
- `store_global_and_jump`: Copies a global variable into another global variable and then performs an indirect jump through an unrecovered jump table.
- `copy_byte_from_offset`: Copies a byte from offset six of the pointer in A0 to an offset from the frame pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: Compares a global variable to a constant value and returns four if they match, otherwise one.
- `initialize_pointer_chain`: Writes a sentinel value into a pointer and stores the address of the next entry into three fields in two structures, clears a word field, and writes the sentinel into the pointed-to entry.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure, only if those pointers are non-zero.
- `check_collision_flags`: XORs a global state word with a value from a linked list node and checks for specific bit flags; if conditions are met, it iterates through the linked list and invokes a callback function for nodes matching certain criteria.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a7, then returns.
- `copy_word_if_valid`: Checks if two pointer arguments are non-null and, if valid, copies a 16-bit value from an offset in the first pointer to an offset in the second pointer.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic, stores the computed byte into a memory location, and returns a combined value derived from another pointer offset.
- `negate_nonzero_flag`: Returns zero if the input value is non-zero, and negative one if the input value is zero.
- `modify_vhp_pos_register`: Sets a bit in a byte at an offset from a pointer via the video hardware position register and clears a byte at another offset.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition, storing the resulting computed value into a global variable.
- `check_status_and_set_code`: Clears a byte at offset 31 of a structure, then based on flags in another structure and a byte at offset 29 either calls a routine, sets the byte to 28, or sets it to 29, and finally calls another routine and returns the byte.
- `negate_word_flip_byte_bit`: Negates a 16-bit word at an offset from a pointer and flips the high bit of a byte in a structure, returning a value based on the previous state of that bit.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location, negates the zero test result, and stores the resulting byte into a location pointed to by a pointer.
- `return_immediately`: The routine consists of a bare return with no data access or calls; it performs no work and simply returns to its caller.
- `return_immediately_2`: The routine consists of a single return instruction and performs no other work; it simply returns to its caller.
- `clear_bit_and_update_counter`: Clears a bit in a global byte, and if that bit was set, sets a flag in a structure, increments a counter, calls another routine, then clears the flag and increments the counter again.
- `return_immediately_16`: An empty routine that contains only a return instruction and performs no operations on any data or registers.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `return_immediately_17`: The routine is empty: it contains only a return instruction and performs no operations on any data or registers.
- `clear_offset_and_save_register`: Stores zero to a longword at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `compare_and_jump_indirect`: Compares a register against a constant and, depending on the result, either calls a routine through a pointer or calls another routine and then jumps indirectly through a pointer.
- `store_handler_address`: Stores the address of another routine into a global variable and returns.
- `copy_string_to_buffer`: Copies a null-terminated string from an input pointer to a newly allocated buffer, prefixing it with its length byte, and then sets up a structure with fields derived from a global table and a library base, storing the buffer address in the structure.
- `initialize_expansion_structures`: Uses exec library calls to allocate and enqueue structures, sets flags and pointers in them, stores global pointers, and opens a library.
- `call_library_and_jump`: Calls a library function through a base pointer and then performs an indirect jump through a pointer obtained from the result.
- `send_message_and_dispatch`: Builds a message structure, sends it to a port, waits for a reply, and then dispatches through a jump table based on a value from the reply.
- `dispatch_on_reply_value_2`: Calls several routines, stores a value from a register into a global variable, then scans a table of short values for a match against a value from a structure and jumps through a table entry.
- `dispatch_on_reply_value_3`: Stores a negative value and a register into fields of a structure, calls three routines through pointers, then scans a table of short values for a match with a value from a reply structure and jumps through a table entry.
- `copy_buffered_stream_data`: Copies a requested number of bytes from a stream position into a caller-supplied buffer, advancing the stream offset and handling a cached buffer window that is refilled when the position falls outside it.
- `flush_audio_buffer`: Copies pending sample bytes into a buffer, growing the write position and high-water mark, allocating a new buffer via a library call when none exists, and flushing when the buffer fills or the position reaches the end.
- `dispatch_on_reply_value`: Calls a function through a pointer, stores two values into a structure, calls two more functions, then reads a value from a reply structure and searches a table of short values for a match, finally jumping through an offset from the table.
- `dispatch_with_error_code`: Calls several routines, conditionally stores a value and an error code into global variables, then scans a table of short values for a match against a value from a structure and jumps through a table entry.
- `conditional_dispatch_with_error`: Calls several routines, conditionally stores a value and an error code into global variables, then scans a table of short values for a match and jumps through a table entry.
- `conditional_call_then_dispatch`: If a field of a structure is non-zero, calls a routine through a pointer, then stores a negative value and a register into fields, calls three routines through pointers, and dispatches through a table based on a reply value.
- `advance_offset_and_dispatch_2`: Adds a value from a register to a field in a structure, checks it against a limit, and either stores the sum or sets an error code; then scans a table of short values for a match against a value from a structure and jumps through a table entry.
- `advance_offset_and_dispatch`: Adds a value from a register to a field in a structure, checks it against a limit, and either stores the sum or sets an error code; then calls two routines, reads a value from a reply structure, searches a table of short values for a match, and jumps through a table entry.
- `set_fields_and_dispatch`: Stores two constants into a structure, calls two functions, reads a value from a reply structure, searches a table of short values for a match, and jumps through an offset from the table.
- `return_immediately_3`: The routine consists of only a return instruction; it performs no data operations and simply returns to its caller.
- `return_immediately_4`: The routine consists of only a return instruction; it performs no operation and returns to its caller.
- `return_immediately_5`: The routine is empty and simply returns to its caller.
- `rotate_string_and_store_length`: The routine rotates a null-terminated byte string one position to the right by shifting each byte forward and writes the original first byte into the last position, then stores the length of the string minus one as a byte just before the string in memory.
- `allocate_and_initialize_structure`: Calls another routine and, if it returns non-zero, allocates a structure via a library call; on failure it calls a library function with a constant and returns zero, otherwise it fills fields of the structure with register values and possibly a size from a second call.
- `free_structure_fields`: If the data register is non-zero, loads a field from the structure, calls a library function on it and on another field, then calls a library function again before returning.
- `build_path_string`: Allocates a buffer via an OS memory call and concatenates a directory portion (up to the last colon or slash) with a filename, inserting a slash separator and collapsing duplicate slashes; returns the buffer or an error code.
- `compare_two_strings_twice`: Calls a string comparison routine twice, returning zero if both comparisons fail, otherwise returning the result of the second comparison; the code is not valid 68000 instructions.
- `compare_two_strings`: Compares the bytes pointed to by two pointers until a zero byte is found, returning early if the bytes differ, otherwise it falls through after the terminating zero.
- `halt_on_bad_instruction`: The routine's code is not valid 68000 instructions, so control flow cannot be recovered; the disassembly stops at the bad instruction data.
- `empty_routine`: Contains only a return instruction and performs no operations.
- `install_interrupt_handlers`: Saves registers to globals, obtains a base pointer via a library call, writes jump instructions and handler addresses into it, calls several routines including library opens and interrupt setup, and conditionally calls further handlers before returning zero.
- `check_and_call_handlers`: Checks two memory locations for a specific value and conditionally calls a handler through a function pointer, then sets a global variable and another function pointer before calling a routine through a global pointer.
- `check_and_call_handlers_2`: Tests two instruction words at fixed offsets from a table indexed by a register, and when either matches a specific opcode it invokes a routine through a function pointer table twice.
- `check_magic_and_call`: Checks two memory locations for a specific constant value, calls a function through a pointer if either matches, then stores constants into global variables and calls another function through a pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_function`: Calls a function through a pointer offset from a global base, likely a library vector.
- `store_constants_to_structure`: Stores several constant values into consecutive fields of a structure pointed to by a register.
- `process_linked_list`: Initializes several global pointers and then iterates over a linked list, performing conditional operations and calling other routines based on flags and counters in each node.

### T.X

- `program_entry_point`: The program entry point; its single instruction transfers control to another routine.
- `empty_stub_routine_2`: This routine is empty: it performs no operations and simply returns.
- `empty_stub_routine_3`: This routine is empty: it contains no instructions and simply returns to its caller.
- `open_libraries_and_init`: Opens three libraries through a library base pointer, stores the resulting bases in globals, and on failure calls a routine with zero; on success it reads a structure field, sets up pointers, calls several initialization routines, and computes memory-related globals.
- `open_keyboard_and_setup`: Calls a function through a pointer, stores pointers and constants into global variables, calls another function, and if it returns non-zero calls a routine and returns that value.
- `call_four_routines`: Calls another routine four times in succession and then returns.
- `copy_joystick_register`: Copies the value of the joystick port 0 data register into a global variable.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `allocate_and_initialize_buffers`: Sets up several global pointers based on a base address, allocates memory via a library call, initializes a structure, calls several functions through pointers, and sets up further global pointers and constants.
- `copy_block_and_reset_pointers`: Conditionally sets two constants, copies a block of words from one global area to another, updates several global pointers based on the copy length, calls a routine, and then calls a library function through a pointer.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `find_value_in_table`: Searches a table of shorts for a specific value and returns the number of steps taken or a negative error code.
- `do_nothing_2`: The routine is empty; it performs no operations and simply returns to its caller.
- `apply_scaled_transform`: Calculates a value using a multiplication and addition, then stores the result as a dword into a table, while also modifying a hardware interrupt enable register.
- `build_pointer_and_string_tables`: Iterates over up to four entries of a table, writing characters into a buffer and storing pointers and values into a table, then calls two routines.
- `return_immediately_6`: The routine is empty: it performs no operations and simply returns to its caller.
- `call_indirect_function`: Calls a function pointer stored in a memory location and returns the result.
- `wait_for_register_bit`: Loops until a specific bit in a hardware register becomes set.
- `enable_interrupts_and_call`: Writes a value to the interrupt enable register and calls another routine.
- `update_pointer_table`: Iterates through a list of pointers and updates the target or destination using a linear transformation or offset.
- `write_instruction_sequence`: Writes a short sequence of instruction bytes to a memory location, calls several routines, then calls an external function through a pointer from a global table and modifies interrupt and peripheral registers.
- `execute_sequence`: Executes a sequence of routine calls and updates a global variable with a constant value before calling a function via a pointer.
- `reset_and_call_pointer`: Sets several global variables to zero, calls a routine pointer retrieved from a table offset by a global variable, stores a value in a global, then calls another routine pointer through the table with the caller's frame pointer.
- `set_system_state`: Calls a function through a pointer, updates a global variable, and calls another function with an argument.
- `increment_byte_and_call_routines`: Increments a global byte, then calls another routine four times in a loop and three more times afterwards, returning the incoming data registers.
- `decrement_counter_and_set_flag`: If a global variable is zero, it checks a per-index byte and, when that byte is nonzero, decrements it; if it reaches zero it sets the global variable and writes a computed value based on the index into a table entry.
- `update_channel_parameters`: If a global variable is zero, it reads a status byte from a structure and, depending on its bits, computes values and writes them to per-index hardware registers and table entries, setting the global variable.
- `empty_handler`: Empty routine that immediately returns.
- `empty_handler_routine`: This routine is empty: it contains only a return and performs no operations on data or hardware.
- `return_immediately_7`: An empty routine that does nothing and returns immediately.
- `empty_do_nothing_routine`: This routine is empty; it performs no operations and simply returns.
- `decrypt_tagged_structures`: If a global flag is set, walks a linked list of structures, and for each entry whose type byte is 1 or 2 and whose flag byte is 1, copies its payload into a 40-byte-wide buffer and then decrypts it in place using a nibble lookup table with a running sum, terminating the result with two zero bytes and decrementing the entry's length field by two.
- `expand_compressed_assets`: Iterates a linked list of compressed data structures, reading a type byte and dimensions, then expands the following byte sequences into four destination buffers using a bit-shifting decode loop and a fixed row stride.
- `return_constant_3000`: Returns the constant value 3000 as a 16-bit result.
- `tiny_move_rts`: Small routine of 2 instructions that copies 100 to d0, then returns.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies 0 to d0, then returns.
- `replicate_global_byte`: Returns a 16-bit value formed by duplicating a byte read from a global variable into both halves.
- `empty_stub_return`: This routine contains no instructions other than returning; it performs no operation and returns immediately to its caller.
- `copy_table_entries`: Iterates over up to four entries of a table of source/destination pointer pairs, copying a block of 16-bit words from each source pointer to its destination pointer, stopping early when a source pointer is null.
- `call_write_pixel`: Calls another routine and then calls the graphics WritePixel function through a library pointer.
- `call_draw_and_move`: Calls another routine and then calls the graphics Draw and Move functions through library pointers.
- `call_indirect_through_table`: Calls another routine, then calls a routine through a pointer stored in a table at a fixed offset, and returns the values held in the data registers.
- `empty_routine_2`: Contains no executable instructions and performs no operations.
- `store_byte_and_call`: Stores a byte from a data register into a global variable and then calls a routine through a pointer fetched from a table offset.
- `store_byte_and_call_table_entry`: Stores the low byte of a data register into a global variable, then calls a function pointer taken from a table at a fixed negative offset.
- `update_global_with_conditional`: If the low word of a data register equals three, substitutes four, stores the resulting byte into a global variable, and then calls a routine through a pointer fetched from a table offset.
- `poll_until_nonzero_then_zero`: Repeatedly calls another routine until it returns a non-zero byte, then repeatedly calls it until it returns a zero byte, returning the first non-zero result.
- `draw_character_and_update_cursor`: Handles a character code: adjusts cursor position globals for carriage return and line feed, otherwise stores the character in a global, calls a graphics routine, copies four longwords from a table into a structure, and advances the cursor.
- `iterate_over_string`: Loops over a null-terminated string pointed to by a register, calling another routine for each non-null character until the terminator is reached, then returns zero.
- `calculate_scaled_offsets`: Clamps the low words of two data registers to maximum values of thirty-nine and twenty-four, then stores each clamped value multiplied by a constant into two global variables.
- `return_immediately_8`: The routine is empty: it returns without touching any data, registers, or hardware.
- `update_position_limits`: Adds a byte-derived delta to two global position variables, clamps them to fixed ranges, and clears bits in a third global variable.
- `scale_input_values`: Multiplies two input registers by two and stores the results in two global variables.
- `return_halved_global`: Returns the value of a global variable shifted right by one.
- `compare_and_reset_table`: Compares two twelve-byte tables byte by byte, and on the first differing byte computes a bit index and either updates a global from a table or clears a portion of the second table to zero.
- `set_status_flags`: Calls a function pointer from a global variable, then sets bits in a global variable based on flags read from a byte.
- `build_status_from_flags`: Reads bits of a global variable and assembles a status byte in the low word of the data register, setting bits for pairs of flags and for the sign bit.
- `build_status_from_shifted_byte`: Reads bits of a global variable and assembles a status byte in the low word of the data register, setting bits for pairs of flags and for a bit derived from a shifted byte.
- `build_joystick_status_byte`: Reads joystick port movement bits from a global variable and assembles a status byte in the low word of the data register, setting bits for up, down, left, right, and fire depending on the port bits and a sign flag.
- `empty_return_2`: Contains no data-processing instructions and simply returns to its caller.
- `return_immediately_18`: The routine contains no data-processing instructions and simply returns to its caller.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `open_file_and_check_handle`: Calls a DOS open function through a library pointer and returns minus one if the returned handle is null, otherwise returns the handle.
- `return_zero_constant`: This routine is empty: it performs no operations and simply returns a constant zero value to its caller.
- `lock_and_unlock`: Calls a function pointer from a global variable and, if it returns nonzero, calls another function pointer with a register argument.
- `open_and_close_file`: Calls a function pointer, and if a global variable is nonzero, calls another function pointer and then loops forever.
- `open_file_or_minus_one`: Calls another routine, then calls a DOS open function through a library pointer; if the returned handle is null it returns minus one, otherwise it returns the handle.
- `open_file_and_return_handle`: Calls a routine, then calls the DOS open function through a library pointer and returns its result, substituting minus one when the result is zero.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_external_function`: Calls a function pointer located at an offset from a value stored in a global variable.
- `write_file_and_set_flag`: Calls a routine, then calls the DOS write function through a library pointer and stores one into a global variable.
- `wait_for_disk_and_keypress`: Loops calling another routine until two global flags indicate readiness, then checks a global byte for specific values; if not matched, calls several routines, waits for a carriage return from a routine, calls more routines, and then loops forever.
- `copy_and_clear_planes`: Copies words from five source planes into an interleaved destination buffer and clears the source words, over a fixed grid.
- `copy_from_interleaved`: Copies words from an interleaved source buffer into five destination planes, over a fixed grid.
- `return_immediately_9`: Empty routine that does nothing and returns to its caller.
- `wait_for_message_with_flags`: Calls a message-retrieval function through a library pointer and, while the returned message's flags lack a specific bit, keeps retrieving messages until one with that bit is found.
- `return_stack_argument_2`: Returns the longword passed on the stack unchanged to the caller.
- `lock_current_directory_or_error`: Calls the DOS current-directory function; if it returns zero, calls two routines (likely error handling), otherwise stores the lock in a global and calls a DOS info function through a library pointer.
- `return_zero`: Returns the constant value zero to its caller.
- `compute_scaled_difference`: Repeatedly calls another routine until a global is not minus one, then computes a difference of two globals, multiplies by a word value, divides by one thousand, subtracts two, clamps to zero, and returns the quotient and remainder.
- `lock_and_info_device`: Calls a DOS lock function through a library pointer; if it fails returns minus one, otherwise stores the lock in a global, calls a DOS info function, and calls a DOS unlock function.
- `swap_display_buffers`: Saves five display-related words, selects a buffer index based on a flag, and if the current index differs, fills the words with fixed values, calls another routine, swaps two index globals, restores the words and calls the routine twice more.
- `swap_display_buffers_alt`: Same buffer-swapping sequence as the neighboring routine, saving and restoring five display words around a conditional index swap and three calls to another routine.
- `swap_display_buffers_variant`: Repeats the conditional display-buffer swap: saves five words, sets fixed values, calls another routine, exchanges two index globals, restores the words and calls the routine twice.
- `swap_display_buffers_copy`: Performs the same conditional buffer-index exchange with saved and restored display words and three calls to another routine.
- `swap_display_buffers_again`: Again saves five display words, conditionally swaps two index globals around fixed-value stores and three calls to another routine, then restores the words.
- `swap_display_buffers_final`: Repeats the conditional display-buffer swap with saved and restored words and three calls to another routine.
- `swap_display_buffers_with_region_reset`: Saves five words from globals, selects an index based on a flag, and if the index differs from a global, resets those words to fixed values, calls a routine, swaps two globals, restores the saved words, and calls two more routines.
- `swap_and_reset_region`: Saves five words from globals, and if a register differs from a global it resets those words to fixed values, calls a routine, swaps two other globals, restores the saved words, and calls two more routines.
- `copy_block_and_toggle_globals`: Calls another routine, copies five longwords from one global buffer to another, then repeatedly calls routines in a loop, swaps two global pointers, temporarily sets five consecutive global words to fixed values, calls another routine, and restores the original values.
- `add_scaled_value_to_table`: Computes a value from a global variable shifted right by three and adds it to five consecutive entries of a table, then calls another routine.
- `allocate_and_init_buffer`: Calls another routine to obtain a pointer; if null it clears a global, otherwise it computes a row stride from a field, stores derived pointers and sizes into globals, writes a header word and dimensions into the allocated block.
- `allocate_and_init_block`: Calls another routine to obtain a block; if it returns null it clears a global variable, otherwise it computes a row stride from a field of the caller's structure, stores pointers to successive sub-blocks spaced by that stride, writes a size word and two differences into the block header, and stores a value of four at the block start.
- `allocate_and_initialize_block`: Calls another routine to obtain a block; if it returns null it clears a global variable, otherwise it computes a row stride from a field of a structure, derives several block offsets from global values, stores the input pointer in a global, writes a header and dimensions into the block, and calls another routine.
- `compute_block_layout_and_store`: Computes a row size from a field of a structure, derives several block offsets from global values, stores the input pointer in a global, writes a four at that pointer, and stores a fixed word and two differences into a structure.
- `allocate_aligned_block`: Rounds the requested size up to a multiple of four plus a header, subtracts it from a global allocation pointer, and if the result stays above a global limit it updates the pointer, stores the size and returns the block after the header, otherwise returns null.
- `allocate_block_from_heap`: Computes a rounded block size from the input value, then carves that block off a global heap pointer by subtracting the size and storing the size at the new block's start; returns a pointer past the size word, or null if the heap would underflow.
- `allocate_aligned_block_from_arena`: Computes a size from the input register, rounds it up to a multiple of four plus a header, and carves it from a global arena pointer if enough space remains; otherwise returns null.
- `allocate_aligned_block_from_arena_2`: Computes a rounded-up block size from the value in a data register, then carves that block from a downward-growing arena pointer held in a global variable.
- `allocate_block_from_heap_2`: Computes a rounded block size from the value in a register, subtracts it from a global heap pointer, and if the result stays above a global limit, stores the size at the new block and returns a pointer past that header; otherwise returns null.
- `allocate_block_from_heap_3`: Computes a rounded block size from the input value, subtracts it from a global heap pointer, and if the result stays above a global limit stores the new pointer and writes the size into the block header.
- `return_stack_argument_3`: Returns the longword passed on the stack unchanged.
- `free_block_if_top`: If the global allocation pointer equals the block address passed in a register minus a header size, advances the pointer by the size word stored at the block start; returns the passed value.
- `call_routine_and_test_range`: Calls another routine to obtain a word, then returns zero if that word lies within a fixed range and all-ones otherwise.
- `save_pointer_and_call_six`: Stores a register into a global variable, calls another routine six times in succession, and returns a word from a register.
- `save_and_reset_hardware`: Copies several hardware and global values into saved variables, then writes to the interrupt, DMA, ADKCON and CIAB registers to disable interrupts and DMA and set control values.
- `clear_bits_and_wait`: Clears two bits in a global variable, then busy-loops a short counter down from one hundred while repeatedly clearing one of those bits, and finally calls two other routines.
- `delay_loop`: Counts a short register down from three thousand to minus one in a busy loop.
- `wait_for_bit_clear`: Loops until a bit in a global variable is clear, polling the CIA port register that holds the fire buttons.
- `poll_fire_button_loop`: While a bit in a global variable is set, clears and sets bits in that variable and calls another routine, polling the CIA port that holds the fire buttons.
- `wait_and_pulse_fire_button`: Waits while a bit in a global variable is set, then for a fixed count clears and sets bits in that variable and calls another routine, followed by a busy delay loop.
- `pulse_fire_button_and_wait`: Calls a routine to wait and pulse a fire button, then repeatedly clears a bit in a global, writes to CIA registers, calls another routine, and polls a CIA register for a bit with a timeout, returning when the bit is set.
- `clear_and_set_stack_words`: Stores zero and a constant into two words at fixed offsets from a frame pointer.
- `wait_for_flag_bit`: Spins in a loop until a specific bit in a global variable becomes set, touching the CIA interrupt control register.
- `mask_global_and_write_stack_words`: Masks a global variable and writes four words derived from global variables with a constant ORed in to stack offsets, also touching the CIA port register.
- `scan_words_for_sync_pattern`: Scans a sequence of words bit by bit looking for a matching pattern, calling another routine when found, and sets a global flag if the scan completes without a match.
- `scan_words_for_pattern`: Scans a sequence of words bit by bit for a matching pattern, storing the index found or clearing a global flag when none is found.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `initialize_globals_and_process_list`: Stores several routine addresses and pointers into globals, calls three routines, then walks a linked list of entries, updating flags and counters and calling routines for entries whose conditions are met.
- `jump_through_indexed_table`: Reads a byte through a pointer, uses it as an index into a table relative to a base register, and jumps to the resulting address.
- `read_three_bytes_advance_pointer`: Reads three consecutive bytes from a pointer, stores the advanced pointer at a stack offset, and returns the three bytes packed together.
- `read_three_bytes`: Reads three consecutive bytes from a pointer and returns them as a single packed value.
- `clear_counter_on_match`: Returns early if a global byte is zero and a value is at least a global bound; otherwise, if the value equals a word at an offset from the frame pointer, clears that word to zero.
- `conditionally_return_pointer_byte`: If a flag register is zero, returns a byte from a pointer; otherwise returns a value from a register.
- `conditionally_return_pointer_target`: If a certain flag is zero, it returns a value from a pointer, otherwise it returns a register value.
- `conditionally_return_three_bytes`: If a flag word is zero, it builds a 32-bit value from three consecutive bytes pointed to by a pointer and returns it; otherwise it returns the incoming data register unchanged.
- `copy_d7_to_d6`: A one-instruction routine that copies the value in data register D7 into data register D6.
- `dispatch_table_handler`: Reads a byte through a pointer, uses it as an index into a table of shorts, and branches on the fetched value, updating two global pointers as it goes.
- `jump_through_table`: Reads a signed word from a pointer and uses it as an offset into a base register to fetch a code pointer, then jumps to it.
- `copy_block_and_dispatch`: Copies a small block of bytes and words from one memory area to another, then calls a table-dispatch routine and a sorted-table search routine.
- `check_string_then_process_list`: If a word pointed to by an address register is non-zero, it scans forward over a null-terminated string and returns; otherwise it calls two routines and then walks a linked list of entries, updating flags and counters and calling routines for entries whose conditions are met.
- `copy_structure_and_link_node`: Sets a flag bit in a byte at an offset from a global base, copies a byte and thirty-two bytes from a source structure into it, then either appends the node to a linked list or moves a free-list entry and initializes fields in the destination structure.
- `store_byte_via_offset`: Stores a byte from a register into a global buffer at an offset given by a signed word from a pointer.
- `update_if_different`: Compares a word from a pointer with a word at a fixed offset from a base register and updates that offset if they differ.
- `modify_bit_flags_and_relink`: Clears one bit and sets another bit in a byte located at an offset from a global variable, then performs linked list insertion and removal operations.
- `append_to_linked_list`: Clears a link field at an offset in a structure, then either sets a global head pointer or walks the linked list to append the new index at the end.
- `remove_from_linked_list`: Walks a linked list starting from a global head pointer and unlinks the node matching a given index, updating the head or the previous node's link.
- `calculate_cross_products`: Computes three signed byte cross products from six bytes in a structure and stores the results in three consecutive bytes.
- `dispatch_script_command_bytes_2`: Reads a command byte through a pointer and branches: for one value it copies a counted block of words into two global buffers and resets a flag; otherwise it loops over a count of entries, adding per-entry offsets to stored coordinate globals and toggling a direction flag before calling another routine.
- `dispatch_script_command_bytes_3`: Reads a command byte through a pointer and branches: for one value it copies a counted block of words into two global buffers and resets a flag, while other values loop over a table of word entries, updating global position/state words and calling another routine for each entry.
- `dispatch_script_command_bytes`: Reads a command byte through a pointer and branches: for one value it copies a counted block of words into two global buffers and resets a flag; otherwise it loops over a count of records, negating and offsetting coordinate words and calling another routine for each, and writes a byte value through a pointer in the remaining path.
- `process_command_table_entries`: Reads a command byte from a table and, for a negative command, copies a counted run of words into two global buffers and resets several global flags; for other negative commands, iterates over packed entries, updating global position and direction fields and calling another routine per entry.
- `compare_table_entry_fields`: Reads a short offset from a table entry, then compares a short and a byte from the pointed-to entry against a stack variable and a global variable, returning without acting on the result.
- `compare_table_entry`: Compares a short from a table entry against a stack variable and a byte from the same entry against a global variable, returning without action if both match.
- `search_linked_list`: Traverses a linked list via short offsets, comparing each node's short and byte fields against a stack variable and a global variable, stopping early when a comparison fails.
- `insert_node_into_list`: Removes the head node from a global free-list, links it into a list at a given position, fills its fields from registers and globals, and inserts it into a per-index list.
- `compute_scaled_vector_offset`: Computes a vector between two structures, scales it by a bit-shifted factor divided by the depth component, and writes the resulting destination coordinates and depth to globals.
- `read_entry_byte_if_positive`: Checks the sign byte of a structure and, if non-negative, returns the byte at offset thirty-six of that structure in the low word of the result.
- `compute_indexed_table_pointer`: When a global flag is clear, computes a pointer from a base pointer, a table offset and a scaled index, then returns the value read from that computed address.
- `calculate_pointer_offset`: Computes a pointer by adding a base pointer, an offset from a table, and a scaled index, then returns a value read from that computed address.
- `loop_until_byte_nonzero`: Repeatedly calls another routine while a byte value is zero, storing that byte in a global variable, then clears the global variable when the byte becomes nonzero.
- `loop_until_short_zero`: Checks a short value on the stack; if nonzero it repeatedly calls another routine until a returned short becomes zero, then clears a global variable.
- `unlink_or_relink_list_node`: Examines a node's flag byte and either marks it and unlinks it from a list, or unlinks it and pushes it onto a global free-list, updating the previous node's next pointer.
- `update_linked_list_node`: Replaces a node in a linked list by updating a next-pointer field, either at the head or after traversing until a matching node is found.
- `copy_two_globals`: Copies two longwords from the caller's frame into two global variables.
- `apply_coordinate_offset`: Computes differences between three coordinate values and a stored base, then for each node in a list adds those differences to its coordinate fields and sets a flag byte if zero.
- `offset_list_coordinates`: Adds offset values to three coordinate fields of a structure, then traverses a linked list setting a flag byte on qualifying nodes and clears a bit in a status byte.
- `wait_and_process_object_list`: Waits for a hardware register bit, enables interrupts, then walks a linked list of object entries copying bounds into globals and relinking or freeing nodes, and copies words from a region into globals until a pointer limit.
- `process_object_list_with_bounds`: Walks a linked list of object entries, reading flag bytes and rectangle fields, copying bounds into globals, and relinking or freeing nodes whose flags match certain bits while invoking helper routines for clipping and rendering.
- `write_values_to_table_offset`: Calls a routine, then writes a word from one offset to another in a table, stores several global values at offsets from a register, writes a byte from a global, and calls two more routines.
- `relink_node_and_check_bound`: Copies a next-pointer field from one node to another, pushes the node onto a global free-list, and sets a global flag when a table value is at least a global threshold.
- `copy_entry_and_set_flag`: Calls another routine, then if a global flag is set and a table value at an offset is at least a global bound, sets that flag to one; calls the routine again, copies a word from one table offset to another, stores several global values at offsets from a register, and calls the routine twice more.
- `initialize_global_bounds`: Sets two global variables to the maximum positive short value and two others to the minimum negative short value.
- `expand_global_bounds`: For a table entry with a non-negative flag, updates four global bound variables with the entry's coordinate values and their sums with table-derived offsets when they exceed or fall below current bounds.
- `insert_sorted_list_node`: Inserts a node into a linked list ordered by priority fields, comparing a short and two bytes against existing nodes, with a special case for a global flag and a specific node index.
- `render_object_list_with_clipping`: Walks a linked list of object entries, skipping entries whose flag byte has bit 6 set, and for each valid entry sets up clip bounds, iterates its child list, and calls the clipping, region-update, and blit routines to draw the objects.
- `set_flag_from_bounds`: Compares two global bound values against two other global bounds and stores either one or all-ones into a global variable depending on the comparisons.
- `store_four_words_and_advance`: Stores four global words into consecutive locations at a global pointer, advances that pointer by four words, and returns or continues based on the new pointer value.
- `copy_words_until_pointer_limit`: Loops over a region, loading four consecutive words into global variables and invoking another routine, until a pointer reaches a global limit.
- `clamp_rect_bounds`: Reads four short fields from a table entry at a pointer plus offset, rejects the entry if any field lies outside global bounds, otherwise clamps the fields to those bounds and stores them into globals, setting a flag.
- `expand_rect_bounds`: Tests a table entry's rectangle against global bounds and a size value, and if it overlaps, stores the union of the entry and the global bounds into globals and sets a flag.
- `update_clipping_region`: Computes a rectangle from a table entry and a size pointer, intersects it with another entry's rectangle, masks the result to a nibble grid, and stores the clipped bounds, width and screen offset into globals.
- `copy_memory_block`: Copies a length-prefixed block from a source pointer to a destination pointer, writing the first bytes then looping over words while decrementing the count.
- `return_immediately_10`: An empty routine that performs no operations and simply returns to its caller.
- `store_byte_to_global`: Calls another routine and then stores a byte from a data register into a global variable.
- `compute_indexed_pointers`: Uses a global byte value shifted left five times as an offset from a base to compute and store two global pointers.
- `reset_global_variable`: Stores all-ones then zero twice into a global variable.
- `check_nibble_sum`: Adds the low nibbles of two input words and returns if the sum exceeds fifteen.
- `check_nibble_difference`: Subtracts the low nibble of one input word from the other and returns if the result is negative or at least sixteen.
- `dispatch_on_state_byte`: Tests a global state byte against several values and returns early for some; otherwise copies a counted block of words through a global pointer, optionally copies a fixed block, and sets several global bytes based on a register value.
- `pack_nibbles_into_words`: Iterates over a 64-entry table of 16-bit words, comparing each word against a second table; when they differ, it compares the low, middle, and high nibbles of both words, recursively resolving nibble differences, then writes a repacked word combining the resulting nibbles back into the second table.
- `copy_block_and_set_state`: Calls a routine, returns early if a global state byte matches certain values, otherwise clears the state, copies a counted block of words through a global pointer, optionally copies more words, and sets several global flags based on a register value.
- `call_nibble_sum_check`: Calls a routine that adds the low nibbles of two words and returns if the sum exceeds fifteen, then returns.
- `call_nibble_difference_check`: Calls a routine that subtracts the low nibble of one input word from the other and returns if the result is negative or at least sixteen.
- `store_command_block_and_ratio`: Stores a word from the stack into a global, calls a routine, stores bytes from a register into globals, stores words into globals, and if a register is non-zero computes a scaled ratio and stores it, then sets a flag and calls a routine.
- `build_and_store_command_block`: Compares a byte against several table entries and, when none match, writes a command block of bytes and words through a pointer, setting the first byte to a command value.
- `call_routines_and_clear_globals`: Calls several other routines and then sets two global variables to zero.
- `copy_register_data`: Calls a small routine that copies the contents of one data register to another.
- `tiny_move`: Small routine of 1 instruction that copies d7 to d6.
- `jump_through_table_2`: Uses a byte pointed to by an address register as an index into a table based at another address register and jumps indirectly through the resulting entry.
- `swap_globals_and_jump_table`: Swaps the values of two global variables, then uses a byte pointed to by an address register as an index into a table based at another address register to jump indirectly through the fetched entry.
- `tiny_move_ext_rts`: Small routine of 3 instructions that copies (a3)+ to d7, then sign-extends d7, then returns.
- `return_empty_routine`: Contains no operations and immediately returns to the caller.
- `return_empty_routine_2`: Contains no executable instructions and performs no operations before returning.
- `return_empty_routine_3`: Performs no operations and returns to the caller.
- `call_tiny_move_then_self`: Calls the tiny routine that copies d7 into d6, then falls into a recursive call to itself, so it does not return normally.
- `call_self_recursively`: The routine calls itself with no arguments and returns; it performs no other work and has no visible termination condition.
- `compare_registers_not_equal`: Calls a small routine that copies one data register to another, then checks whether the two register values differ and returns.
- `compare_registers_equal`: Calls a small routine that copies one data register to another, then checks whether the two register values are equal and returns.
- `compare_registers_and_branch`: Calls a small routine that copies one data register to another, then checks whether the new value is less than the original and returns.
- `compare_registers_and_branch_2`: Calls another routine that copies one data register to another, then compares the two words and returns if the first is not greater than the second.
- `call_tiny_move`: Calls another routine that copies one data register to another and then returns.
- `call_tiny_move_routine`: Calls a small routine that copies the value of one data register into another, then returns.
- `store_linear_hash`: Multiplies the low word of a data register by a constant, adds another constant, and stores the resulting 16-bit value into a global variable.
- `multiply_and_add_constant`: Multiplies the low word of a data register by a constant and adds another constant, returning the result.
- `call_compare_if_flag_clear`: If a global byte is zero, calls another routine that compares and resets a table, then returns.
- `branch_on_global_zero`: Checks whether a global variable is zero and, if so, calls another routine.
- `call_constant_3000`: Calls a routine that returns the constant value 3000 as a 16-bit result.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d7 to -(a4), then returns.
- `jump_through_indexed_table_2`: Uses a byte pointed to by an address register as an index into a table based at another address register and jumps indirectly through the selected entry.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `store_byte_at_offset`: Stores a byte from a data register into memory at an address formed by adding a word pointed to by an address register to a base address in another address register.
- `store_word_at_offset`: Stores a word from a data register into memory at an address formed by adding a word pointed to by an address register to a base address in another address register.
- `write_word_at_offset`: Stores a word from a data register into memory at an address formed by adding a sign-extended byte pointed to by an address register to a base address in another address register.
- `call_three_routines`: Calls one routine, passes its result to a second routine, then calls a third routine and returns.
- `build_path_and_initialize`: Clears globals, copies a string into a buffer, appends another string, then calls routines that lock the current directory, initialize data structures, decode a run-length stream, and build a sorted list of pointers.
- `initialize_data_structures`: Initializes several global variables and fields in a structure with specific values, then loops to populate a table with calculated pointers until a limit is reached.
- `copy_string_and_setup_buffers`: Copies a null-terminated string from an address register into a global buffer, replacing the character after the last dot with a letter chosen by a global flag, then calls several routines and initializes a set of global pointers and counters based on values read from memory.
- `decode_rle_stream`: Reads a control byte from a global stream; if its high bit is set, it repeats the next byte a number of times given by the low bits, otherwise it copies that many following byte pairs.
- `read_byte_with_bounds_retry`: Compares two address registers and, when one is not above the other, calls a retrying routine and reloads a pointer before returning the byte it points to.
- `write_byte_with_bounds_check`: Checks a calculated address against a bound and, if the check passes, stores a byte from a data register at that address.
- `decompress_run_length_encoded_data`: Copies two bytes from a global pointer to globals, then loops over a destination range, calling routines to decode run-length encoded data and writing bytes to the destination until the end is reached.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `compute_signed_byte_difference`: Subtracts one byte register from another and returns the difference, or its negation when the result is negative, calling a retrying routine if an address comparison fails.
- `invoke_and_return_pair`: Calls another routine and returns a concatenated 64-bit value formed from two 32-bit registers.
- `search_sorted_table`: Performs a binary search through a table of word-sized values to find a match for a given value, returning if a match is found or if the search range becomes invalid.
- `initialize_incrementing_list`: Walks a buffer from a global start pointer to a global end pointer, writing a zero longword and an incrementing six-based short offset into each six-byte entry, then clears a short field at the start and sets two global boundary values.
- `insert_and_swap_list_entries`: Maintains a sorted list of pointers by comparing a word at each entry against a value from a pointer argument, shifting entries to insert the new pointer, then increments counters and conditionally invokes other routines.
- `swap_pool_entries_and_init_block`: Swaps two entries in a global table indexed by a register offset, increments a counter and recurses when the counter exceeds a limit, otherwise initializes a new block header from a source structure and clears its fields.
- `allocate_and_link_block`: Allocates a block from a global memory pool, initializes its header fields and pointers, and links it into a free/used list; if allocation fails it compacts the pool by moving blocks and updating list offsets, then retries.
- `retry_open_until_flag_clear`: Repeatedly calls a routine that opens a file and returns a handle, storing the result in a global variable, and loops while the result is negative or while certain flag bits remain set, returning the handle once the flags clear.
- `call_function_pointer`: Calls a function pointer and returns the value it leaves in the data register.
- `retry_call_until_nonnegative`: Repeatedly calls another routine and loops while its returned value is negative, returning once the result is non-negative.
- `retry_open_until_success`: Repeatedly calls a routine that opens a file and returns a handle, storing the result in a global variable, looping while the result is negative and returning the incoming data register otherwise.
- `retry_open_file_until_success`: Repeatedly calls a routine that opens a file and returns a handle, storing the result in a global byte; while the result is negative it calls the routine again, and it returns the last data register value.
- `retry_call_preserving_registers`: Repeatedly calls another routine, looping while the returned value is negative and preserving the address registers across retries, then returns the result.
- `execute_recursive_logic`: Sets a global byte to one, adjusts it with conditional increments and decrements, and recursively calls itself based on the resulting value and on checks of a routine pointer, finally clearing the global byte.
- `update_counter_and_blit`: Checks a global byte, calls a halving routine and blit routines, decrements a counter, and stores new values to global bytes when the counter reaches zero.
- `decrement_and_accumulate_structure_fields`: Decrements a word counter in a structure; if it is nonzero it clamps and accumulates word fields and, when a shifted sum stays non-negative, stores the sum and calls another routine, otherwise it clears several structure fields and calls another routine.
- `execute_invalid_instruction_data`: The routine's bytes are not valid 68000 instructions, so execution cannot proceed past them; the decompiler reports an undecodable instruction rather than any real operation.
- `clear_structure_fields`: Through a pointer passed in a register, stores zero bytes at three byte offsets and zero words at two word offsets, clearing selected fields of a structure.
- `update_timed_counter_and_audio`: Decrements or increments several global words based on their current values, clamps a counter to a maximum, and calls routines that toggle audio, clamp a volume, and compute a scaled value stored in a global variable.
- `empty_return_stub`: The routine contains only a return sequence and performs no operation on data; it simply returns to its caller.
- `empty_handler_2`: Empty routine that immediately returns; it is referenced as a handler pointer in data and called by another routine.
- `advance_counters_if_word_nonzero`: Calls another routine to obtain a word; if the word is nonzero, adds a fixed amount to one global variable and increments another global variable by two, otherwise leaves them unchanged.
- `blit_and_return_register_pair`: Calls a masked sprite blit routine and returns a 64-bit value composed of two data registers.
- `call_blit_routine`: Calls a masked sprite blit routine and returns.
- `blit_masked_sprite_region`: Clips a rectangular region against a global bounding box, then copies masked source words from a table into five destination planes while saving the original words, using shifted bit masks derived from the source data.
- `blit_5plane_tile_column`: Copies a 16-row block of five bitplanes from a source table into five destination pointers, advancing each destination by a stride derived from a global and optionally shifting the source by one word when a global flag is set.
- `tiny_ori_2`: Small routine of 1 instruction that merges 117 into d0.
- `blit_single_row_region`: Sets up the blitter with fixed masks and modulo values, waits for the blitter to finish, then programs the source and destination pointers from computed row and column offsets and issues one blit of a single row region.
- `blit_loop_over_rows`: Configures the blitter once, then loops over a computed number of rows, each iteration programming source and destination pointers advanced by a row stride and issuing a blit, waiting for completion each time.
- `copy_tiled_block_rows`: Computes a destination offset from a horizontal position and a row stride, then copies a block word-by-word across several source planes into interleaved destination rows, advancing each source and destination pointer by a constant stride after each row.
- `program_blitter_rectangle_copy`: Computes blit dimensions from global coordinates, waits on the blitter busy bit, programs blitter pointers, modulos and control registers, and starts a blit.
- `blit_conditional_pointer_setup`: Checks two flag bits in a structure; if neither is set it programs blitter pointers and control from computed offsets and performs a blit, otherwise it only stores a computed pointer value into a global variable.
- `blit_clipped_sprite_with_masking`: Clips a sprite/object rectangle against global bounds, converts source pixel rows into masked planar bitplane data through a lookup table, and programs the blitter (control, modulo, pointers, size) to draw the clipped region, enabling DMA channels.
- `clip_and_blit_sprite_region`: Reads a sprite/object descriptor through a pointer, clips its rectangle against global bounds, computes source and destination offsets, and invokes another routine to draw the clipped region; it also sets mask and offset globals used by the drawing.
- `clip_and_blit_masked_sprite`: Reads a sprite/object descriptor through a pointer, rejects it if its bounds fall outside the global clip window, and computes the clipped rectangle, source offsets, and edge masks.
- `check_and_clear_audio_flag`: If a global byte is zero, it compares a code pointer against four other routines and, when none match, stores a byte through that pointer; then it calls another routine.
- `configure_audio_channel`: Calls a library routine to enter supervisor state, computes a channel bit mask and a clamped volume value, writes the volume and period and pointer to the audio hardware registers, sets a per-channel flag byte and a status byte, then calls a library routine to leave supervisor state.
- `clear_channel_flag_if_negative`: Tests a per-channel flag byte and, if its sign bit is set, clears the flag byte, clears a byte through a pointer, and computes a channel bit mask shifted left by seven.
- `test_interrupt_bits_and_reset`: Reads a global word, recursively calls itself for each of four interrupt bits that are set, then writes a constant to that global word.
- `decrement_channel_counter`: Decrements a per-channel word counter and, when it reaches zero, clears a byte through a pointer, clears the per-channel flag byte, and computes a channel bit mask shifted left by seven.
- `call_four_routines_in_sequence`: Calls another routine four times in succession.
- `call_routine_and_return`: Calls another routine and returns the values left in the data registers to its caller.
- `call_audio_toggle`: Calls a routine that toggles audio and interrupt state and returns the value in the data register.
- `call_volume_clamp`: Calls a routine that clamps a value and stores it to a global word and audio volume registers, returning the data registers.
- `compute_scaled_value`: Takes a value in a register, masks it to seven bits, multiplies by eight and subtracts seven, storing the result in a global variable.
- `call_routine_return_value`: Calls another routine and returns the value it leaves in the data register.
- `clamp_and_store_volume`: Clamps a value in a register to at most one hundred and, when it is non-negative, stores it to a global word and three audio volume registers; otherwise stores it to one global word.
- `install_audio_interrupt_handler`: Stores register values into audio hardware registers, sets up a handler pointer and clears fields in a structure, then programs interrupt and DMA control registers to enable audio interrupts.
- `store_two_words_through_pointer`: Stores two constant words at offsets into the structure pointed to by a register.
- `save_state_and_start_audio`: Saves several registers into globals, calls the supervisor state function, clears a series of audio volume registers, installs a handler pointer into a structure, and restores the user state.
- `toggle_audio_and_interrupt_state`: Depending on a global byte and the sign of a register value, it saves or clears state bytes, masks interrupt and DMA enable registers, and writes control values to a hardware register.
- `restore_audio_and_interrupts`: If a global flag is set, writes values to the DMA and interrupt control registers and sets a bit in a structure field; if another flag is set, sets a bit in a structure field and enables interrupt bits.
- `dispatch_interrupt_bits`: Tests a global word for the interrupt-enable bit and, when set, combines two hardware interrupt registers and jumps indirectly through a table entry for each of four set bits among the tested positions; otherwise returns.
- `dispatch_on_d1_bits_2`: Tests bits in the D1 register and, for each set bit among three tested positions, jumps indirectly through a table entry; otherwise returns the value passed on the stack.
- `dispatch_on_d1_bits`: Tests bits in the D1 register; if the high bit is set it jumps indirectly through one table entry, if the next tested bit is set it jumps indirectly through another table entry, otherwise it returns the value passed on the stack.
- `dispatch_on_mask_bit`: Tests a bit in a register and, when set, jumps indirectly through a table entry; otherwise returns the value passed on the stack.
- `dispatch_on_interrupt_mask`: Combines two words from a structure with a mask and, for each of four set bits, jumps indirectly through a table entry.
- `install_handler_pointer`: Writes constant values to global variables, stores a routine pointer and a zero into fields of a structure pointed to by a register, and stores a routine pointer into a field of another structure.
- `install_handler_and_clear_globals`: Writes fixed byte values into global variables, ORs one of them with a constant, then stores two routine pointers into fields of the structure pointed to by the incoming address register and clears the corresponding globals.
- `install_handler_and_clear_slots`: Writes fixed byte values into global variables, stores a routine pointer into two fields of the structure pointed to by A0, clears the corresponding global pointers, and installs a routine pointer into a field of another structure.
- `install_handler_and_clear_slots_2`: Writes constant bytes into several global variables, stores a routine pointer and a zero into two fields of a structure pointed to by an incoming address register, and installs a routine pointer into a field of another structure.
- `install_handler_and_clear_slots_3`: Writes fixed byte values to global variables, ORs a value into one of them, then stores a routine pointer and a zero into two fields of the structure pointed to by a register, and finally writes a routine pointer into a field of a structure reached through a global pointer.
- `install_handler_and_clear_slots_4`: Writes fixed byte values to global variables, stores a routine pointer and a zero into two fields of the structure pointed to by A0, and installs a routine pointer into a field of another structure.
- `install_handler_and_clear_pointers`: Writes fixed byte values into global variables, stores a routine pointer and a zero into two fields of a structure pointed to by an address register, and installs a routine pointer into a table entry.
- `reset_ciab_and_audio_state`: Writes constants to global words, stores a routine pointer and a value into a structure, and clears two global pointers.
- `write_ciab_timer_and_audio_registers`: Writes control and timer values to the CIAB registers (CRA, TAHI, TALO) and updates audio channel registers, stepping through a byte stream and dispatching on command bytes to load pointers and lengths into tables.
- `parse_audio_command_stream_2`: Reads a byte command stream through a pointer, dispatching on the command byte to update a table of audio channel entries, writing channel pointers, periods, volumes and lengths into the Paula audio registers and setting channel-active flags.
- `parse_audio_command_stream`: Reads a byte command stream through a pointer, dispatching on the command byte to update a table of audio channel entries, writing channel pointers, lengths, periods and volume values into hardware audio registers and setting channel-active flags.
- `parse_and_apply_audio_command_stream`: Reads a byte command stream through a pointer and dispatches on the command byte to update a table of audio channel entries, writing channel pointers, lengths, periods and volumes into the hardware audio registers and setting/clearing channel-active bits in global masks.
- `parse_audio_command_stream_3`: Reads a byte command stream through a pointer, dispatching on the command byte to update a table of audio channel entries, writing channel pointers, periods, volumes and control bits into the table and the audio hardware registers, and advancing the stream pointer.
- `store_pointer_and_advance_global`: The routine's bytes do not decode as valid 68000 instructions, so no behavior can be recovered; it is only referenced as a data pointer.
- `dispatch_audio_command_stream`: Reads bytes from a command stream through a pointer, using each command byte to index a table of channel entries and update audio channel state, and dispatches to a handler routine for extended commands.
- `dispatch_audio_command_stream_2`: Reads bytes from a command stream through a pointer, dispatching on the command byte to update audio channel entries and writing channel period, pointer, and volume registers for the selected channel.
- `parse_audio_command_stream_4`: Reads a byte command stream through a pointer, dispatching on the command byte to update a table of audio channel entries, writing channel pointers, sample data, period and volume values into the channel table and the audio hardware registers, and advancing the stream pointer.
- `play_audio_command_stream`: Reads a byte command stream through a pointer, dispatching on the command byte to update a table of audio channel entries, writing channel pointers and lengths to the audio hardware registers and setting channel enable bits.
- `run_bytecode_script`: Interprets a byte stream through a jump table, writing values into a global table and programming the custom chip registers (pointer, length, and modulo) from table entries.
- `parse_audio_command_stream_5`: Reads a byte command stream through a pointer, dispatching on the command byte to update a table of audio channel entries, writing channel pointers, periods, volumes and lengths to the audio hardware registers and setting channel-active flags.
- `set_bit0_pointer_and_width`: If bit 0 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 164, stores the pointer plus a field at offset 160, and sets a word at offset 154 to 128; then always sets a word at offset 156 to 128.
- `set_bit0_handler_and_mask`: If bit 0 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable; then stores the value 128 at offset 156 of the object.
- `set_mode1_and_masks`: Stores the value 1 at offset 150 and the value 128 at offsets 154 and 156 of the object.
- `set_bit1_pointer_and_width`: If bit 1 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 180, stores the pointer plus a field at offset 176, and sets a word at offset 154 to 256; then always sets a word at offset 156 to 256.
- `set_bit1_handler_and_mask`: If bit 1 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable; then stores the value 256 at offset 156 of the object.
- `set_mode2_and_masks`: Stores the value 2 at offset 150 and the value 256 at offsets 154 and 156 of the object.
- `set_bit2_pointer_and_width`: If bit 2 of a byte at offset 3 of the object is set, computes a width from two fields before the pointer and stores it at offset 196, stores the pointer plus a field at offset 192, and sets a word at offset 154 to 512; then always sets a word at offset 156 to 512.
- `set_bit2_handler_and_mask`: If bit 2 of the byte at offset 3 of the object is set, stores a routine pointer into a global variable; then stores the value 512 at offset 156 of the object.
- `set_mode4_and_masks`: Stores the value 4 at offset 150 and the value 512 at offsets 154 and 156 of the object.
- `decrement_counter_set_bit3_mode`: If bit 3 of a byte at offset 3 of the object is set, decrements a global counter and, when it becomes negative, stores the value 8 at offset 150 and 1024 at offset 154; then always stores 1024 at offset 156.
- `empty_return`: Empty routine that only returns.
- `halt_on_invalid_opcode`: The routine's single instruction is not valid 68000 code, so execution stops here; the decompiler cannot recover any behavior.
- `halt_on_invalid_opcode_2`: The routine consists of a single instruction that the disassembler cannot decode as valid 68000 code, so it does not perform any recoverable operation.
- `halt_on_invalid_opcode_3`: The routine's single instruction is not valid 68000 code, so control flow cannot be decoded; it is reached only as an address taken by another routine and a handler pointer in data.
- `halt_on_bad_instruction_2`: The routine's entry point is referenced as a pointer, but its bytes do not decode as valid 68000 instructions, so control flow cannot be recovered and execution would stop on an illegal opcode.
- `halt_on_invalid_opcode_4`: The routine's single instruction is not valid executable code, so control flow cannot continue past it.
- `halt_on_invalid_opcode_5`: The routine's single instruction is not a valid 68000 opcode, so execution stops here rather than performing any operation.
- `halt_on_invalid_opcode_6`: The routine contains no valid 68000 instructions; the disassembler stops at invalid opcode data, so it does not perform any operation.
- `halt_on_invalid_opcode_7`: The disassembly shows no valid instructions: control flow reaches invalid opcode data, so the routine cannot execute any meaningful operation and effectively stops there.
- `halt_on_invalid_opcode_8`: The routine's two instructions are not valid executable code; the disassembly shows bad instruction data, so control cannot proceed through it.
- `halt_on_bad_instruction_3`: The disassembly at this entry point does not decode as valid 68000 instructions, so the decompiler cannot recover any behavior; the routine's address is taken and stored as a handler pointer in data, but no executable logic is visible.
- `halt_on_invalid_opcode_9`: The disassembly of this routine contains invalid opcode data, so the recovered control flow ends at a bad instruction and no actual behavior can be determined from it.
- `halt_on_bad_instruction_4`: The decompiler could not recover valid instructions at this entry point; the disassembly shows bad instruction data rather than a coherent routine, so no data operations can be described.
- `halt_on_bad_instruction_5`: The routine's code is not valid 68000 instructions, so execution stops at the bad instruction data rather than performing any operation.
- `halt_on_bad_instruction_6`: The routine's control flow reaches an invalid or illegal instruction, so execution stops here; no other operation is performed.
- `halt_on_bad_instruction_7`: The routine's four instructions are not valid 68000 code, so control flow cannot be followed and execution stops at the undecodable data.
- `halt_on_bad_instruction_8`: The routine's code is not valid 68000 instruction data, so execution cannot proceed past it; the decompiler reports a bad instruction and truncated control flow.
- `halt_on_bad_data`: The disassembly for this routine is invalid, so no executable instructions can be recovered; the entry point is referenced as an address and as a handler pointer in data, but the bytes do not decode into a valid instruction sequence.
- `execute_unrecovered_data`: The routine's entry is reached through an address stored in data and its pointer is also held in a data handler slot, but the bytes at its entry do not decode as valid 68000 instructions, so control flow cannot be recovered from this disassembly.
- `jump_to_invalid_opcode`: The routine's entry is reached through an address stored in data and via a handler pointer, but its instruction stream decodes as invalid opcode data, so it does not execute a recoverable sequence of operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `or_byte_with_constant`: Returns the low byte of the data register ORed with a constant value.
- `or_byte_with_constant_2`: Returns the low byte of the data register combined with a constant using a bitwise OR.
- `return_low_byte`: Returns the low byte of the data register unchanged.
- `or_byte_with_75`: Returns the low byte of the data register ORed with the constant value 117.
- `return_byte_in_register`: Returns the byte value passed in the data register unchanged; the routine has no body beyond that return.
- `return_byte_in_register_2`: Returns the byte value passed in the data register unchanged; the routine performs no other operation.
- `return_byte_unchanged`: Returns the byte value passed in the data register unchanged; it performs no other operation and references no data or hardware.
- `return_byte_in_d0`: Returns the byte value passed in the low byte of data register D0 unchanged; the routine performs no other operation.
- `or_byte_with_constant_3`: Returns the low byte of the data register ORed with the constant value 117.
- `return_byte_unchanged_2`: Returns the byte value passed in the data register unchanged, acting as a no-op passthrough.
- `return_byte_in_register_3`: Returns the byte passed in the data register unchanged; the routine has no other effect and performs no memory access.
- `return_byte_in_d0_2`: Returns the byte value passed in the low byte of the data register unchanged; it performs no other work.
- `return_byte_in_d0_3`: Returns the byte value passed in the low byte of data register D0 unchanged; the routine has no other effect.
- `return_data_register_byte`: Returns the byte value passed in the low byte of the data register unchanged.
- `return_input_byte`: Returns the byte value supplied in the low byte of the data register without modification.
- `pass_through_byte`: Returns the low byte of the data register as its result, leaving it unchanged.
- `echo_data_byte`: Returns the byte held in the low byte of the data register unchanged.
- `or_byte_with_mask`: Returns the low byte of the data register combined with the constant value through a bitwise OR.
- `set_bits_in_byte`: Returns the low byte of the data register with the constant bits set via a bitwise OR.
- `apply_or_constant`: Returns the low byte of the data register after ORing it with the fixed constant.
- `return_unchanged_byte`: Returns the byte value from the low byte of the data register without altering it.
- `combine_byte_with_constant`: Returns the low byte of the data register ORed with the constant value.
- `yield_input_byte`: Returns the low byte of the data register as received, with no modification.
- `return_byte_in_d0_4`: Returns the byte value passed in the data register unchanged.
- `return_low_byte_of_d0`: Returns the low byte of the value passed in the data register unchanged; the routine has no other effect.
- `or_byte_with_mask_2`: Returns the byte value passed in the data register combined with a fixed bit mask.
- `single_instruction_stub`: A one-instruction routine entered from outside static flow; its decompilation is unavailable, so only that it executes a single instruction is known.
- `return_byte_in_d0_5`: Returns the byte value passed in the data register unchanged; the routine has no other effect on memory or registers.
- `return_byte_in_data_register`: Returns the byte passed in the low byte of the data register unchanged; the routine is a single instruction that leaves that register value as its result.
- `or_byte_with_immediate`: Returns the low byte of the data register ORed with the immediate value 117.
- `return_byte_in_register_4`: Returns the byte value passed in the data register unchanged; the routine has no other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_absolute_pointer`: Calls through an absolute memory pointer and returns the value left in the data register.
- `call_indirect_through_pointer`: Calls another routine through a pointer stored in memory, then returns the value left in a data register.
- `call_indirect_through_pc`: Calls another routine through an indirect pointer located in the instruction stream, then returns the data register value left by that call.
- `call_indirect_through_pc_pointer`: Calls the routine whose address is stored at a fixed location in the code segment, then returns the data register value left by that call.
- `call_indirect_and_return_d1`: Calls another routine through an indirect pointer, then returns the value left in the second data register.
- `call_indirect_through_pc_2`: Calls another routine through an indirect pointer stored in the instruction stream, then returns the data register value left by that call.
- `call_indirect_through_pc_3`: Calls another routine through an indirect pointer held in the instruction stream, then returns the data register value left by that call.
- `return_zero_in_d0`: Returns zero in the data register without performing any other work.
- `increment_word_at_offset`: Increments a 16-bit value located at a fixed positive offset from a base pointer register in place and then returns.
- `decrement_word_at_offset`: Decrements a 16-bit value located at a fixed positive offset from a base pointer register and then returns a zero value.
- `execute_single_instruction`: A one-instruction routine entered from outside static flow during the reference run.
- `execute_single_instruction_stub`: A one-instruction routine entered from outside the static flow; the listed facts show only that it executes a single instruction and returns, with no calls, text, or register effects documented.
- `execute_single_instruction_stub_2`: A one-instruction routine entered from outside the static flow; the decompiler timed out, so only its single instruction and external entry are known.
- `execute_single_instruction_2`: A one-instruction routine in a gap that decodes as code but was not reached in the reference run.
- `unknown_single_instruction_stub`: A one-instruction routine entered from outside its static flow during execution; the decompiler timed out, so only its instruction count and external entry are known.
- `execute_single_instruction_stub_3`: A one-instruction routine entered from outside the static flow; the decompiler timed out, so its single instruction cannot be characterized beyond that it executes and returns.
- `execute_single_instruction_stub_4`: A one-instruction routine entered from outside the static flow; the listed facts show only a single instruction with no calls, text, or register effects described, so its data operation is not evident from the available information.
- `return_immediately_19`: The routine consists only of a return instruction and performs no work; it does not call the referenced bit-flag routine.
- `return_immediately_20`: The routine contains no operations and simply returns to its caller.
- `return_immediately_21`: The routine is empty: it performs no work and simply returns to its caller.
- `return_immediately_22`: The routine contains only a return instruction and performs no data operations before returning.
- `set_flag_from_bit_2`: Reads a bit from a global variable using an index derived from a byte in another memory location, negates the zero test result, and stores the resulting flag.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_single_instruction_stub_5`: A one-instruction routine entered from outside the static flow; the listed facts show only a single instruction with no calls, no referenced text, and no evident data manipulation, so its purpose is not determinable from the available information.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_interrupt_enable`: Stores a value into a global variable and modifies the interrupt enable bit in the hardware interrupt enable register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_11`: This routine is empty and performs no operations before returning to the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `nop_padding_instruction`: This routine is empty and performs no operations.
- `return_immediately_13`: This routine performs no operations and returns immediately to its caller.
- `return_no_op`: The routine consists of a single instruction that returns immediately, performing no work.
- `return_immediately_12`: This routine is empty and performs no operations before returning to its caller.
- `nop_operation`: The routine is empty and performs no operations.
