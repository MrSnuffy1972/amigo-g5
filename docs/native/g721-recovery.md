# Game 721 (Pong v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00721). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (83546 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 337 of 337 named (0 author symbols), 337 of 337 described (10 by rule, 326 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1846 of 1846 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2794 of 2794 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4531 |
| code | 12044 |
| data | 66968 |
| padding | 3 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Pong | 3 | 1614 | 47 |
| game.slave | 1 | 908 | 71 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 265 | 212 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |

## Cases handled

- Pong_v1.0/Pong_v1.0.hdf: hard-disk image (AmigaDOS OFS, 64 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Pong_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 201 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Pong

- `program_entry_and_message_loop`: Program entry: calls a library routine to find a task, checks a field of the result, and if nonzero runs the main loop; otherwise calls library routines to wait for and reply to messages.
- `run_main_game_loop`: Calls setup routines, then loops forever: clears globals, waits on a hardware register bit, delays, waits for the bit to clear, sets up display values, calls several routines, and waits for a global byte to reach a specific value.
- `fill_three_regions_with_bytes`: Fills a first block of two hundred bytes with 255, then for two hundred forty-six entries writes 1 and 128 at offsets 39 and 40 of each 40-byte stride, and finally fills another two hundred bytes with 255.
- `fill_three_regions_with_ff`: Fills a first block of two hundred bytes with 255, then for two hundred forty-six entries writes 255 at offset 39 of each 40-byte stride, and finally fills another two hundred bytes with 255.
- `fill_forty_two_longs`: Writes the value a fixed address into forty-two consecutive longwords through the pointer passed in A1.
- `fill_nine_longs`: Writes the value a fixed address into nine consecutive longwords through the pointer passed in A1.
- `clear_forty_two_longs`: Writes zero into forty-two consecutive longwords through the pointer passed in A1.
- `fill_two_long_runs`: Writes zero to the first longword, then fills seventy-one longwords with a fixed address, then fills another seventy-one longwords starting further along with a fixed address.
- `update_game_state_from_input`: Tests several global bytes and bits to decide branches, then calls a series of routines that read joystick and mouse registers, adjust position and counter globals, clamp values, and update state bytes.
- `select_sound_by_position`: Compares a global position value against several offset ranges around two other globals and stores a small index into a global, then clears a flag, subtracts a global from another, and recursively calls itself before restoring the flag.
- `empty_stub_routine`: This routine contains no instructions and immediately returns; it performs no operation on any data.
- `advance_counter_with_wrap_2`: Adds a global increment to a global counter; if the counter is below a threshold it stores the sum, otherwise it clears a global, calls another routine, and subtracts the increment.
- `advance_counter_with_wrap`: Adds a global increment to a global counter; if the counter is below a threshold it stores the sum, otherwise it clears a global, calls another routine, and subtracts the increment from the counter.
- `update_vertical_position_by_mode`: Depending on a global mode byte, moves a vertical position global up or down by fixed steps (smaller step when a modifier bit is set), copies it from another global, or, in the default case, applies the change in a global input byte to both a size global and the position global, clamping each within bounds.
- `clamp_value_and_restore_registers`: When a global flag equals one, clamps a global value to a minimum or to one of two upper limits depending on the flag, then calls another routine and restores two saved registers.
- `branch_on_global_byte`: Tests a global byte against one and calls another routine in both branches, then stores two register values into globals.
- `update_potgo_from_state`: If a global byte equals two, sets it to one and stores two shifted copies of its bytes into two globals; otherwise returns without changes.
- `update_position_from_input`: Reads joystick and mouse registers and a global mode byte, then adjusts a position global by fixed increments or clamps it within bounds depending on the mode.
- `clamp_global_word_by_mode`: Reads a global mode byte and, when it equals one, clamps a global word to a minimum of 49 or to one of two upper limits depending on the mode value; for other mode values it skips the clamping.
- `branch_on_mode_two_or_three`: Checks a global byte for two or three and calls another routine in each case, then stores two register values into globals.
- `call_and_store_registers`: Calls another routine and then stores two register values into globals.
- `return_immediately`: The routine contains no operations and simply returns to its caller.
- `decrement_counter_with_reset`: If a global byte is nonzero, calls two routines and clears it when it equals one, otherwise decrements it.
- `decrement_counter_with_call`: If a global byte is nonzero, clears it and calls another routine when it equals one, then decrements it.
- `decrement_global_with_recursion`: If a global byte is nonzero, it checks whether the value equals one; when it does, it clears the byte and recursively calls itself, then decrements the byte by one.
- `decrement_state_counter`: If a global byte is nonzero, decrements it unless it equals one, in which case it clears it and sets another global byte to three.
- `set_audio_channel_periods`: Writes audio channel pointers, lengths, periods and volumes for both channels, then enables audio DMA and sets a small value.
- `set_audio_channel_periods_alt`: Writes audio channel pointers, lengths, periods and volumes for both channels with a different period value, then enables audio DMA and sets a small value.
- `set_audio_channel_periods_third`: Writes audio channel pointers, lengths, periods and volumes for both channels with a third period value, then enables audio DMA and sets a small value.
- `reset_globals_and_conditionally_call`: Calls another routine, clears three globals, calls another routine if a global byte is not three, then clears another global.
- `copy_block_repeatedly`: Copies a sequence of longwords from one memory area to another, repeating each value at several offsets within each destination block for five iterations.
- `call_routines_and_set_global`: Calls three routines in sequence and then stores the value one hundred into a global.
- `initialize_display_registers`: Calls several routines, then writes values to display and DMA hardware registers and stores a split register value into a structure.
- `store_split_word`: Stores the high and low halves of a register value into two short fields at offsets in a destination structure.
- `close_graphics_library`: Calls a library close routine, updates a global pointer from a returned structure, calls another library routine, clears a global variable and restores a saved value.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `return_input_registers`: Calls another routine and returns the incoming data registers unchanged.
- `allocate_and_initialize_message_port`: Calls a library routine to obtain a value, calls another to allocate memory, stores the returned pointer in a global, initializes fields of the allocated structure (a pointer, two bytes, and a code pointer), then calls another library routine.
- `remove_interrupt_server`: Calls a library routine to remove an interrupt server and then writes a small value to a global variable.
- `dispatch_keyboard_command`: Reads a command byte and dispatches to bit-set/clear operations on a global variable, color-register cycling, or setup routines based on the command value; also reads the CIAA serial data register and tests nibbles of the result.
- `call_routine_nine_times`: Calls another routine nine times in succession, then stores the value two hundred into a global variable and returns the incoming register value.
- `store_word_by_mode_4`: Reads a global mode byte and, depending on whether it equals one through five, stores an incoming word into one or two of several global words.
- `call_sequence_and_set_counter`: Calls another routine nine times in succession, then stores the value two hundred into a global variable and returns the incoming register values.
- `decrement_or_reset_counter`: If a global variable is nonzero, either calls another routine five times and clears the variable when it equals one, or decrements the variable by one.
- `decompress_run_length_data`: Decodes a run-length encoded byte stream: literal bytes are copied directly, while bytes with the high bit set indicate a repeat count for the following byte, until a memory limit is reached.
- `clear_block_and_store_pairs`: Clears a large block of longwords to zero, calls another routine, then stores two incoming register words into five successive pairs of global word variables.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_offsets_and_jump_table`: Stores a routine address and a register value into globals, calls three routines through offsets from a pointer in a register, then performs an indirect jump through a global pointer.
- `open_library_and_dispatch`: Opens a library, obtains a base from it, and if that base is nonzero calls a routine through it with an adjusted argument, then closes the library and jumps through a stored pointer; otherwise returns zero.
- `assign_global_and_return`: Copies a global variable into a local variable and stores a self reference into the global variable, then returns.
- `modify_cia_register_and_return_input`: Sets and then clears a bit in a global variable (touching the CIA control register), calls another routine, and returns the input register value unchanged.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `jump_through_global_pointer_2`: Loads a global variable and performs an indirect jump through an unrecovered jump table; the decompiler's apparent infinite loop is an artefact of the unrecovered control flow, not real behaviour.
- `patch_jump_table_entries`: Computes a value from a global variable plus a constant and stores it into two fields of an unrecovered jump table before performing an indirect jump through that table.
- `jump_through_table`: Performs an indirect jump through an unrecovered jump table.
- `mask_global_word`: Returns the value of a global variable with the bits corresponding to mask 143 cleared.
- `store_stack_word_to_global`: Stores a word taken from the stack into a global variable.
- `return_zero_constant`: Immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: Receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure, then returns.
- `return_immediately_14`: This routine is empty: it performs no operations and simply returns to its caller.
- `duplicate_structure_with_allocations`: Allocates a new structure, copies a word count and then copies pairs of fields from the source structure, allocating memory for each non-zero second field; on allocation failure it frees the previously allocated blocks and returns an error-tagged value.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node whose string field matches the provided string, or null if none matches.
- `store_register_to_global`: Copies the value from a data register into a global variable and returns.
- `jump_through_global_pointer`: Loads a global variable and performs an indirect jump through an unrecovered jump table.
- `copy_byte_to_global_offset`: Copies a byte from an offset in the source pointer to a fixed offset in a global structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_global_constant`: Compares a global variable to a specific constant value and returns 4 if they match, otherwise 1.
- `initialize_pointer_chain`: Writes a sentinel value into a pointer, stores the address of the next entry into three fields across two structures, clears a word field, and writes the sentinel into the pointed-to entry.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure, only when those pointers are non-zero.
- `check_collision_flags`: XORs a global state word with a value from a linked list node and checks for specific bit flags; if conditions are met, it iterates through the linked list and invokes a callback for nodes matching certain criteria.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a7, then returns.
- `copy_word_if_valid`: Checks that two pointer arguments are non-null and, if so, copies a 16-bit value from an offset in the first pointer to an offset in the second pointer.
- `combine_and_store_byte`: Calls a function pointer from a global structure, combines its result with two register bytes and a byte at an offset in a structure using bitwise logic, stores the result back at that offset, and returns it ORed with a byte from another structure offset.
- `clear_four_words`: Writes the value -1 to four consecutive 32-bit words ending at an offset in a structure pointed to by a register.
- `calculate_table_offset`: Reads three values from a global table, divides the third by fifty, multiplies the first by a constant and adds the second, multiplies that sum by sixty, adds the division result, and stores the computed value into a global variable.
- `select_disk_status_byte`: Reads a byte from a structure, clears a byte at an offset in another structure, and sets that byte to a status value based on flags and a table lookup, calling other routines conditionally.
- `negate_short_toggle_bit`: Negates a short value at an offset in a structure and toggles the high bit of a byte at an offset in another structure, returning a non-zero value if the original high bit was clear and zero otherwise.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location, negates the zero test result, and stores the resulting byte into a location pointed to by a pointer.
- `return_immediately_2`: The routine consists of three instructions that perform no data operation and simply return to the caller.
- `return_immediately_3`: The routine consists of a bare return with no data access or calls; it performs no operation and simply returns to its caller.
- `toggle_flag_and_increment`: Tests and clears a bit in a global byte, sets a flag bit in a structure, and increments a counter at an offset in that structure, calling another routine when the flag was previously clear.
- `set_bit_in_byte_array`: Disables the exec kernel, calls another routine to obtain an index, stores a byte from a register through a pointer offset by that index, sets a bit in a byte pointed to by a pointer using the index, then calls another routine and enables the exec kernel.
- `return_from_routine`: Performs a tail branch to exit the routine.
- `return_immediately_empty`: An empty routine that performs no operations and returns directly to its caller.
- `clear_offset_and_save_register`: Stores zero to a longword at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `dispatch_via_jump_table`: Compares a register against a global value and, depending on the result, performs an indirect jump through a table or calls a function pointer from a library base offset.
- `dispatch_or_recurse_on_register`: Compares a register against a global value and, depending on the result, either performs an indirect jump through a table or calls a function pointer; otherwise it recurses into itself and then jumps through another function pointer.
- `store_pc_relative_address`: Stores the address of this routine into a global variable and returns.
- `halt_on_bad_instruction`: The disassembly shows no valid 68000 instructions at this entry point; the decompiler treats the bytes as bad instruction data and truncates control flow.
- `open_expansion_library_and_setup_messages`: Allocates memory, opens the expansion library, and builds message structures by calling library functions and storing pointers and fields into the allocated blocks and global variables.
- `call_dos_library_function`: Calls a function through a pointer obtained from the dos.library base, then calls another function through a pointer offset from the returned value.
- `open_dos_library_and_dispatch`: Calls exec library functions to find a task, get messages, open dos.library, put messages, and wait on a port, then scans a table and performs an indirect jump based on a matched value.
- `dispatch_table_jump`: Calls several routines, then scans a table of short entries comparing each against a value read from a structure until a match is found, and jumps to the code offset stored in the following table entry.
- `set_error_and_dispatch`: Calls two routines, stores an error code into a structure field, calls three library functions, and scans a table to dispatch through an offset.
- `copy_buffered_bytes_to_output`: Copies a bounded number of bytes from a source buffer into a destination buffer, first from a direct source pointer and then, if a second buffer pointer is set, from that buffer starting at an offset derived from a stored position.
- `copy_buffer_with_retry`: Copies bytes from a source pointer into a destination buffer in chunks, tracking a running offset and maximum, and retrying through an allocation routine when the destination chunk is exhausted.
- `dispatch_by_task_id`: Calls a function through a pointer, copies a value into a structure field, calls two more functions, then scans a table of word entries comparing each against a field of a structure obtained from a function result and jumps to an offset from the matching entry.
- `dispatch_via_library_table_short`: Calls a routine and, if it returns nonzero, calls an allocation routine and stores the result in a structure field, then calls three library functions and scans a table to dispatch through an offset.
- `dispatch_via_library_table`: Calls a routine and, if it returns nonzero, calls an allocation routine and stores the result in a structure field, then calls three library functions and scans a table of short values to find a match with a field of the library base, finally jumping through an offset from the table.
- `dispatch_via_library_table_2`: Calls a routine and, if it returns nonzero, calls an allocation routine and stores the result in a structure field, then calls three library routines and scans a table to dispatch through an indirect jump.
- `free_structure_fields_2`: If the incoming register is nonzero, reads a field from the structure, calls a library function with it and another field, and if the first call returns nonzero, calls a free routine.
- `advance_offset_with_limit_2`: Adds a register value to a field of a structure, checks it against a limit field, and either stores the new value or sets an error code, then calls three library routines and dispatches through a table.
- `advance_offset_with_limit`: Adds a register value to a field of a structure, checks it against a limit field, and either stores the new value or sets an error code, then calls three library functions and scans a table to dispatch through an offset.
- `set_fields_and_dispatch`: Stores two immediate values into structure fields, copies a register value into another field, calls three functions, then scans a table of word entries comparing each against a field of a structure obtained from a function result and jumps to an offset from the matching entry.
- `return_without_action`: The routine contains only a return sequence and performs no data operations; it exits immediately to its caller.
- `return_immediately_4`: The routine contains only a return instruction and performs no operation on its data before returning to its caller.
- `return_immediately_5`: The routine consists of three instructions that perform no data operations and simply return to the caller.
- `rotate_string_and_store_length`: Rotates a null-terminated byte string one position to the right by shifting each byte forward and writes the original first byte into the last position, then stores the length of the string minus one as a byte just before the string in memory.
- `allocate_and_initialize_structure`: Calls a routine that returns a value; if nonzero, calls an allocation routine and, on success, optionally calls another routine and stores returned values and register values into fields of the allocated structure, calling an error routine on failure.
- `free_structure_fields`: If the incoming register is nonzero, reads a field from the structure, calls a library function with it and another field, and if the first field is nonzero calls the library function again, then calls it a third time and returns.
- `build_path_from_strings`: Computes the length of a first string and scans a second string for a colon, then allocates memory and copies the first string, a slash, and the portion of the second string after the colon into it, returning the new buffer and a status code.
- `compare_two_strings_twice`: Calls a string comparison routine twice, returning a value only if both comparisons succeed.
- `compare_two_strings`: Compares the bytes pointed to by two pointers until a zero byte is found, returning early if the bytes differ, otherwise it falls through after the terminating zero.
- `empty_return`: Performs no operations and simply returns to the caller.
- `invalid_instruction_stub`: The routine's single instruction is not valid 68000 code, so it cannot execute any operation; the decompiler reports bad instruction data and truncated control flow.
- `invalid_instruction_stub_2`: The routine's single instruction is not valid 68000 code, so it cannot execute as a normal routine; the disassembly shows only bad instruction data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_invalid_opcode`: The routine consists of a single instruction that the disassembler cannot decode as valid 68000 code, so it does not perform any recoverable operation.
- `execute_invalid_instruction`: The routine consists of a single instruction that the disassembler cannot decode as valid 68000 code, so control flow stops there; no data is read or written.
- `execute_invalid_opcode_2`: The routine's single instruction is not a valid 68000 opcode, so the processor takes an illegal-instruction exception instead of executing any operation.
- `abort_on_undecodable_word`: Holds a single undecodable instruction word that terminates control flow immediately.
- `trap_on_bad_instruction`: Is a one-instruction stub whose single word is invalid, so execution stops rather than continuing.
- `return_empty_stub_2`: An empty routine that performs no operations and immediately returns to its caller.
- `halt_on_invalid_opcode`: The routine contains no valid instructions; decoding stops at an invalid opcode, so it performs no operation and cannot be executed.
- `execute_invalid_opcode_3`: The routine's single instruction is not valid 68000 code, so execution stops at that point; the decompiler cannot recover any further behavior.
- `halt_on_invalid_opcode_2`: The single instruction is not valid 68000 code, so the processor would take an illegal-instruction exception rather than execute a routine.
- `execute_invalid_opcode_4`: The routine's single instruction is not a valid 68000 opcode, so the processor traps rather than performing any defined operation.
- `halt_on_invalid_opcode_3`: The single instruction is not a valid 68000 opcode, so execution cannot continue; this is a gap that merely decodes as code rather than a real routine.
- `execute_invalid_opcode_5`: The routine's single instruction is not valid 68000 code, so execution stops at this point; the decompiler cannot recover any further behavior.
- `decode_invalid_instruction`: The bytes at this location do not decode as a valid 68000 instruction, so the routine cannot execute any operation; it is an unreached gap that the disassembler treats as code.
- `halt_on_invalid_opcode_4`: The routine contains no valid instructions; execution stops on an invalid opcode, so it performs no data operation and returns nowhere.
- `execute_invalid_instruction_2`: The routine consists of a single instruction that is not valid 68000 opcode data, so execution cannot continue past it; the decompiler treats it as a bad instruction and truncates control flow.
- `return_without_operation`: This routine is empty: it performs no operations and immediately returns to its caller.
- `return_no_op`: The routine contains a single instruction that returns to its caller, performing no operations.
- `return_immediately_12`: An empty routine that performs no work and returns to its caller.
- `execute_invalid_opcode_6`: The routine's single instruction is not valid 68000 opcode data, so control flow cannot continue past it; the decompiler truncates here.
- `execute_invalid_opcode_7`: The routine's single instruction is not a valid 68000 opcode, so execution stops at this point rather than performing any operation.
- `execute_invalid_opcode_8`: The routine's single instruction is not a valid 68000 opcode, so execution stops at this point rather than performing any defined operation.
- `return_immediately_empty_2`: This routine is empty: it performs no operations and returns directly to its caller.
- `execute_invalid_opcode_9`: The routine consists of a single instruction that the disassembler cannot decode as a valid 68000 opcode, so it does not perform any recoverable operation.
- `return_immediately_15`: The routine performs no operations other than returning to the caller; its instructions decode to a no-op sequence and an immediate return.
- `halt_on_invalid_opcode_5`: The routine contains no valid instructions; execution stops at an invalid opcode, so it performs no data operation and returns no value.
- `invalid_instruction_gap`: This location does not decode to a valid instruction; the bytes here are not executable code and control flow cannot continue through them.
- `execute_invalid_instruction_3`: The routine contains a single instruction that is not valid 68000 opcode data, so execution stops there rather than performing any operation.
- `execute_invalid_opcode_10`: The routine consists of a single instruction that is not a valid 68000 opcode, so it cannot execute as normal code and control flow is truncated at that point.
- `halt_on_bad_instruction_2`: Consists of one instruction that is not valid code, causing execution to stop.
- `stop_at_undecodable_word`: Holds a single undecodable instruction word that terminates execution when reached.
- `halt_unreached_gap`: A one-instruction gap that decodes as invalid code and is never reached during execution.
- `execute_invalid_opcode_11`: The routine's single instruction is not a valid 68000 opcode, so the processor traps rather than performing any operation; the decompiler cannot recover further control flow.
- `stop_on_bad_opcode`: A single bad instruction word that stops the processor when control reaches it.
- `halt_at_invalid_word`: One instruction that fails to decode, ending execution at this point.
- `halt_on_invalid_opcode_6`: The routine contains no valid instructions; execution stops at an invalid opcode, so it performs no operation and returns no value.
- `decode_invalid_instruction_2`: The single instruction is not valid 68000 code, so the routine cannot execute and control flow stops at that point.
- `halt_on_bad_instruction_10`: The routine contains no valid instructions; execution stops on an illegal opcode, so it performs no data operation and returns nothing.
- `halt_on_invalid_opcode_7`: The routine contains no valid instructions; decoding stops at an invalid opcode, so it does not execute any meaningful operation.
- `execute_invalid_instruction_4`: The routine's single instruction is not valid 68000 opcode data, so the processor would take an illegal-instruction exception rather than perform any defined operation.
- `halt_on_invalid_opcode_8`: The routine contains no valid instructions; execution stops on bad instruction data, so it performs no operation and returns nothing.
- `halt_on_invalid_opcode_9`: The routine contains no valid 68000 instructions; execution stops at an invalid opcode, so the decompiler cannot recover any behavior.
- `execute_invalid_opcode_12`: The routine's single instruction is not a valid 68000 opcode, so the processor traps it as an illegal instruction rather than performing any data operation.
- `execute_invalid_opcode_13`: The routine's single instruction is not a valid 68000 opcode, so it cannot execute as normal code; the decompiler treats it as bad instruction data and truncates control flow.
- `decode_invalid_instruction_3`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot be decoded as executable code.
- `execute_invalid_instruction_5`: The routine's single instruction is not valid 68000 code, so control flow cannot be decoded; no data is read or written.
- `execute_invalid_opcode_14`: The routine's single instruction is not valid 68000 code, so the processor takes an illegal-instruction exception rather than performing any operation.
- `execute_invalid_opcode_15`: The routine's single instruction is not valid 68000 opcode data, so control flow cannot be decoded past it.
- `halt_on_invalid_opcode_10`: The routine contains a single instruction that the disassembler cannot decode as valid 68000 code, so execution stops there; no data is read or written.
- `stop_at_invalid_opcode`: Consists of one instruction that is not valid code and stops the processor.
- `abort_on_bad_instruction`: Holds a single undecodable instruction that terminates execution.
- `halt_on_invalid_opcode_11`: The routine contains no valid instructions; execution stops on an invalid opcode, so it performs no data operation and returns nothing.
- `decode_invalid_instruction_4`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot produce a routine body here.
- `execute_invalid_opcode_16`: The routine's single instruction is not a valid 68000 opcode, so the processor traps as an illegal instruction rather than performing any data operation.
- `decode_invalid_instruction_5`: The bytes at this location do not form a valid 68000 instruction, so the disassembly stops here; no operation is performed.
- `return_immediately_empty_stub`: This routine is empty: it performs no operations and simply returns to its caller.
- `decode_invalid_instruction_6`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot continue and control flow is truncated here.
- `halt_on_invalid_opcode_49`: The routine contains no valid 68000 instructions; execution stops on undecodable data, so it performs no operation and returns nothing.
- `execute_invalid_instruction_6`: The routine's single instruction is not valid 68000 opcode data, so execution cannot proceed past it; the decompiler truncates control flow here.
- `halt_on_invalid_opcode_12`: Contains a single instruction that decodes as invalid data, so execution stops here without performing any operation.
- `halt_on_invalid_opcode_13`: Consists of one instruction that is not valid code; control flow terminates immediately with no data effect.
- `execute_invalid_instruction_7`: The routine contains a single instruction that the disassembler cannot decode as valid 68000 code, so it does not perform any recoverable operation.
- `halt_on_invalid_opcode_14`: Holds a single undecodable instruction; entering it stops execution without touching registers or memory.
- `halt_on_invalid_opcode_15`: The routine contains no valid instructions; execution stops on an invalid opcode, so it performs no data or register operations.
- `halt_on_invalid_opcode_16`: The bytes at this location do not decode as a valid instruction, so the processor would take an illegal-instruction exception rather than execute a routine.
- `halt_on_invalid_opcode_17`: A single bad opcode that terminates execution immediately when reached, with no side effects.
- `execute_invalid_instruction_8`: The routine's single instruction is not valid 68000 code, so execution cannot continue past it; the decompiler truncates control flow here.
- `decode_invalid_instruction_7`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot decode a routine here; it is a gap in the code rather than executable logic.
- `halt_on_invalid_opcode_18`: One undecodable instruction that stops execution on entry and leaves all state unchanged.
- `halt_on_invalid_opcode_19`: The routine contains no valid 68000 instructions; execution stops at an invalid opcode, so it performs no operation on data.
- `invalid_instruction_gap_2`: This location does not contain a valid instruction; it is an unreached gap that the disassembler cannot decode as code, so no routine behavior can be described.
- `halt_on_invalid_opcode_20`: The routine contains a single instruction that the disassembler cannot decode as valid code, so execution stops at that point.
- `execute_invalid_instruction_9`: The routine's single instruction is not valid 68000 code, so execution cannot continue past it; the decompiler truncates control flow here.
- `execute_invalid_opcode_17`: The routine's single instruction is not a valid 68000 opcode, so it cannot be decoded as executable code and produces no defined data operation.
- `execute_invalid_instruction_10`: The routine's single instruction is not valid 68000 opcode data, so control flow cannot be decoded and execution would trap rather than perform any operation.
- `halt_on_invalid_opcode_21`: The routine contains no valid instructions; execution stops at invalid opcode data, so it performs no operation and returns nowhere.
- `execute_invalid_opcode_18`: The routine's single instruction is not valid 68000 code, so control flow cannot continue past it; the decompiler reports bad instruction data and truncates the flow.
- `decode_invalid_instruction_8`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot recover a routine here; it is a gap that decodes as bad instruction data rather than executable code.
- `halt_on_invalid_opcode_22`: The routine contains no valid instructions; execution stops on an illegal opcode, so it performs no operation and returns nothing.
- `halt_on_bad_data`: Contains a single instruction that decodes as invalid data, so execution stops immediately without performing any operation.
- `stop_at_invalid_opcode_2`: Consists of one undecodable instruction; control flow is truncated at this point and no data is processed.
- `execute_invalid_opcode_19`: The routine consists of a single instruction that the disassembler cannot decode as a valid 68000 opcode, so it does not perform any defined operation.
- `execute_invalid_opcode_20`: The routine consists of a single instruction that is not a valid 68000 opcode, so execution stops there; the decompiler cannot recover any further control flow.
- `execute_invalid_opcode_21`: The routine's single instruction is not a valid 68000 opcode, so the processor takes an illegal-instruction exception rather than executing any defined operation.
- `halt_on_invalid_opcode_23`: The single instruction is not a valid 68000 opcode, so the routine cannot execute any operation; it is an unreachable gap that merely decodes as code.
- `execute_invalid_opcode_22`: The routine's single instruction is not a valid 68000 opcode, so the processor takes an illegal-instruction exception rather than performing any data operation.
- `execute_invalid_opcode_23`: The routine consists of a single instruction that is not a valid 68000 opcode, so execution stops at that point rather than performing any operation.
- `decode_invalid_instruction_9`: The single instruction at this location does not decode as valid 68000 code, so no operation is performed and control cannot continue through it.
- `halt_at_entry_point`: One undecodable instruction that stops execution as soon as it is reached, with no observable effect.
- `invalid_instruction_gap_3`: This location does not contain a valid instruction; the bytes decode as an illegal opcode, so control cannot proceed through it as code.
- `execute_invalid_opcode_24`: The routine's single instruction is not valid 68000 code, so the processor traps on an illegal instruction rather than performing any operation.
- `execute_invalid_opcode_25`: The routine's single instruction is not valid 68000 code, so the processor traps rather than performing any operation.
- `execute_invalid_opcode_26`: The routine consists of a single instruction that is not a valid 68000 opcode, so execution stops at that point rather than performing any data operation.
- `decode_invalid_instruction_10`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot proceed and control flow is truncated.
- `execute_invalid_opcode_27`: The routine consists of a single instruction that is not a valid 68000 opcode, so the processor takes an illegal-instruction exception rather than performing any data operation.
- `execute_invalid_opcode_28`: The routine's single instruction is not a valid 68000 opcode, so the processor would take an illegal-instruction exception rather than perform any data operation.
- `decode_invalid_opcode_halt`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot proceed and control flow stops here.
- `execute_invalid_instruction_11`: The routine's single instruction is not valid 68000 opcode data, so execution stops at this point rather than performing any operation.
- `execute_invalid_instruction_12`: The routine's single instruction is not valid 68000 code, so execution cannot proceed past it; the decompiler truncates control flow here.
- `halt_on_invalid_opcode_24`: The single instruction is not a valid 68000 opcode, so the processor would trap rather than execute a routine; it is an unreached gap that the disassembler decoded as code.
- `halt_on_bad_instruction_3`: Consists of one instruction that is not valid code, so control flow stops there.
- `halt_on_bad_instruction_4`: Holds a lone instruction that fails to decode, terminating execution.
- `execute_invalid_instruction_13`: The routine's single instruction is not valid 68000 opcode data, so execution stops at that point rather than performing any operation.
- `invalid_instruction_gap_4`: This location does not decode to a valid instruction; the disassembly treats the bytes as an illegal opcode, so no routine behavior can be recovered here.
- `halt_on_bad_instruction_5`: Comprises one bad instruction that stops control flow.
- `halt_on_invalid_opcode_25`: The bytes at this location do not decode as a valid instruction, so execution would stop here; the routine performs no operation.
- `halt_on_bad_instruction_6`: Has one instruction that is not valid code, ending execution.
- `halt_on_bad_instruction_7`: Contains a lone undecodable instruction that terminates the flow.
- `decode_invalid_instruction_11`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot continue and control flow is truncated.
- `halt_on_bad_instruction_8`: Contains a single instruction that decodes as invalid data, so control flow stops here without performing any operation.
- `execute_invalid_instruction_14`: The routine contains a single instruction that is not valid 68000 opcode data, so execution stops there; the disassembly cannot recover any further behavior.
- `abort_on_undecodable_data`: Holds a single undecodable instruction that terminates execution when reached.
- `execute_invalid_opcode_29`: The routine's single instruction is not a valid 68000 opcode, so the decompiler cannot recover any behavior; execution reaches an illegal instruction rather than performing a defined operation.
- `halt_on_invalid_opcode_26`: The single instruction is not a valid 68000 opcode, so the processor would trap rather than execute it.
- `halt_on_invalid_opcode_27`: The routine contains a single instruction that is not valid 68000 opcode data, so execution stops immediately without performing any operation.
- `unreached_invalid_opcode_gap`: Is a lone undecodable instruction in an unreached gap that decodes as code.
- `stop_on_bad_instruction_entry`: Is a single bad instruction that stops execution when entered externally.
- `halt_on_invalid_opcode_28`: The single instruction is not a valid 68000 opcode, so execution cannot continue past this point; it is a decoding gap rather than a real routine.
- `execute_invalid_instruction_15`: The routine consists of a single instruction word that the disassembler cannot decode as a valid 68000 opcode, so execution would trap as an illegal instruction rather than perform any data operation.
- `halt_on_bad_data_2`: Consists of a single instruction that decodes as invalid data, so execution stops immediately without performing any operation.
- `stop_at_invalid_opcode_3`: Contains one instruction that is not valid code; control flow is truncated at this point and nothing is executed.
- `execute_invalid_instruction_16`: The routine contains a single instruction that is not valid 68000 opcode data, so execution cannot continue past it; the decompiler truncates control flow here.
- `decode_invalid_instruction_12`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot be decoded as code; this is a gap that was mistakenly treated as a routine.
- `execute_invalid_opcode_halt`: The routine's single instruction is not valid 68000 code, so the processor traps on an illegal instruction rather than performing any defined operation.
- `fail_on_bad_opcode`: A single undecodable instruction that truncates control flow on entry and leaves registers and memory unchanged.
- `halt_without_effect`: Consists of one bad-data instruction; entering it stops execution immediately with no side effects.
- `unreached_bad_data_stub`: An unreached one-instruction gap that decodes as invalid data and is never executed.
- `execute_invalid_opcode_30`: The routine's single instruction is not a valid 68000 opcode, so the processor traps it as an illegal instruction rather than executing any meaningful operation.
- `invalid_instruction_gap_5`: This location does not decode to a valid instruction; it is a gap in the code that Ghidra could not disassemble, so no routine behavior is present.
- `halt_on_bad_instruction_9`: The routine contains no valid 68000 instruction; execution stops on invalid opcode data, so it performs no operation and returns nothing.
- `invalid_instruction_gap_6`: This location is not a real routine; it falls in a gap that the disassembler attempted to decode as code, and the bytes do not form a valid instruction, so no operation is performed.
- `abort_on_undecodable_data_2`: Holds one undecodable instruction word, causing control flow to terminate immediately with no side effects.
- `halt_on_invalid_opcode_29`: The routine contains no valid instructions; execution stops on bad instruction data, so it performs no operation and returns to no caller.
- `fail_on_invalid_encoding`: Contains only an instruction that cannot be decoded, so the routine ends at once without touching registers or memory.
- `stop_on_bad_opcode_2`: Is one non-instruction word that aborts execution; no computation or memory access occurs.
- `halt_at_undecodable_byte`: Consists of a single byte pattern that fails to decode as an instruction, terminating the routine immediately.
- `execute_invalid_opcode_31`: The routine consists of a single instruction that the disassembler cannot decode as valid 68000 code, so control flow stops at this point.
- `halt_on_invalid_opcode_30`: The routine contains a single instruction that the disassembler cannot decode as valid 68000 code, so control flow stops there.
- `execute_invalid_instruction_17`: The routine's single instruction is not valid 68000 opcode data, so execution stops at this point; the decompiler cannot recover any further control flow.
- `execute_invalid_instruction_18`: The routine's single instruction is not valid 68000 opcode data, so control flow cannot continue past it; the decompiler truncates here.
- `halt_on_invalid_opcode_31`: Consists of one undecodable instruction that stops execution at once; it is not reached by the static flow.
- `invalid_instruction_stub_3`: The routine's single instruction is not valid 68000 opcode data, so execution cannot proceed through it; the decompiler treats it as a bad instruction rather than a real operation.
- `halt_on_invalid_opcode_32`: Holds a single bad instruction that terminates execution immediately and touches no registers or memory.
- `decode_invalid_instruction_13`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot be decoded as code.
- `halt_on_invalid_opcode_33`: One invalid opcode that aborts execution on entry, performing no computation.
- `halt_on_invalid_opcode_34`: A single undecodable instruction that stops the processor immediately without side effects.
- `execute_invalid_opcode_32`: The routine consists of a single instruction that is not a valid 68000 opcode, so execution stops at this point rather than performing any operation.
- `halt_on_invalid_opcode_35`: A one-instruction routine decoding as invalid data that terminates execution at once.
- `decode_invalid_instruction_14`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot produce executable code here.
- `halt_on_invalid_opcode_36`: The routine consists of a single instruction that the disassembler cannot decode as valid 68000 code, so execution stops there rather than performing any operation.
- `execute_invalid_opcode_33`: The routine consists of a single instruction that is not a valid 68000 opcode, so the processor traps rather than performing any defined operation.
- `invalid_instruction_gap_7`: This location does not decode to a valid instruction; it is an unreached gap in the code that the disassembler cannot interpret, so no routine behavior can be described.
- `execute_invalid_instruction_19`: The routine's single instruction is not valid 68000 opcode data, so control flow cannot continue past it; the decompiler truncates the flow at this point.
- `fail_on_invalid_encoding_2`: Contains only one instruction that fails to decode, stopping the processor at entry.
- `invalid_instruction_gap_8`: This location does not decode to a valid instruction; it is an unreached gap in the code that the disassembler cannot interpret as executable code.
- `execute_invalid_instruction_20`: The routine's single instruction is not valid 68000 code, so control flow cannot be decoded and execution would trap rather than perform any operation.
- `abort_on_bad_instruction_2`: Contains a single undecodable instruction that terminates execution on entry.
- `halt_on_invalid_opcode_37`: The bytes at this location do not decode as a valid instruction, so execution would stop here.
- `trap_on_invalid_code`: Holds a single instruction that is not valid machine code, so execution stops immediately.
- `execute_single_instruction`: Performs a single instruction execution.
- `halt_on_invalid_opcode_38`: Contains a single instruction that decodes as invalid data, so execution stops here.
- `invalid_instruction_stub_4`: The bytes at this location do not decode as a valid instruction, so the routine cannot execute; it is a gap in the code that the disassembler treats as an illegal opcode.
- `halt_on_invalid_opcode_39`: A single instruction that fails to decode as a valid opcode, terminating control flow.
- `halt_on_invalid_opcode_40`: The routine contains no valid instructions; execution stops on an illegal opcode, so control flow cannot continue past this point.
- `halt_on_invalid_opcode_41`: Contains a single undecodable instruction that stops execution when reached.
- `decode_invalid_instruction_15`: The single instruction at this location is not valid 68000 code, so the disassembly cannot be decoded as a routine.
- `halt_on_invalid_opcode_42`: A single bad instruction that terminates control flow on entry.
- `halt_on_invalid_opcode_43`: The routine contains no valid 68000 instructions; execution stops at an illegal opcode, so it performs no data processing and returns to no caller.
- `invalid_instruction_gap_9`: The bytes at this location do not decode as a valid instruction, so no routine behavior can be recovered; it is an unreached gap in the code.
- `execute_invalid_instruction_21`: The routine contains a single instruction that is not valid 68000 opcode data, so execution stops there; the disassembly cannot be interpreted as a meaningful operation.
- `decode_invalid_instruction_16`: The bytes at this location do not decode as a valid instruction, so control flow cannot continue here.
- `execute_invalid_opcode_34`: The routine's single instruction is not a valid 68000 opcode, so the processor traps it as an illegal instruction rather than executing any defined operation.
- `decode_invalid_instruction_17`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot produce a routine body here.
- `store_byte_via_pointer`: Stores a single byte from a data register into a memory location pointed to by another register.
- `trap_on_bad_encoding`: Is a single bad instruction encoding that stops control flow immediately.
- `decode_invalid_instruction_18`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot produce a routine body here.
- `update_pointer_offset`: Increments a value stored in a memory location by a constant amount.
- `store_byte_via_pointer_2`: stores a single byte from a data register into a memory location pointed to by another register.
- `execute_invalid_opcode_35`: The routine's single instruction is not valid 68000 code, so execution stops there; the decompiler cannot recover any further behavior.
- `stop_on_illegal_opcode`: Consists solely of an illegal instruction that terminates the flow.
- `decode_invalid_instruction_19`: The bytes at this location do not form a valid 68000 instruction, so the routine cannot execute; it is a decoding gap rather than real code.
- `execute_invalid_opcode_36`: The routine consists of a single instruction that is not a valid 68000 opcode, so execution stops there; the decompiler cannot recover any further behavior.
- `return_from_subroutine_2`: The routine performs a single return instruction.
- `execute_invalid_instruction_22`: The routine contains a single instruction that the disassembler cannot decode as valid 68000 code, so control flow stops at this point.
- `invalid_instruction_gap_10`: This location is not a real routine; its single word does not decode as a valid 68000 instruction, so execution cannot continue here.
- `halt_on_bad_data_3`: The routine contains no valid instructions; execution stops on illegal instruction data, so it performs no operation and returns nothing.
- `halt_on_invalid_opcode_44`: The single instruction is not a valid 68000 opcode, so the processor would trap rather than execute any operation; the routine performs no work.
- `execute_invalid_opcode_37`: The routine's single instruction is not a valid 68000 opcode, so the processor traps rather than executing any defined operation.
- `halt_on_invalid_opcode_45`: The routine contains no valid instructions; execution stops on an illegal opcode, so it performs no data operation and never returns.
- `decode_invalid_instruction_20`: The bytes at this location do not form a valid 68000 instruction, so the disassembly cannot decode a routine here.
- `execute_invalid_instruction_23`: The routine consists of a single instruction that is not valid 68000 code, so control flow cannot continue past it.
- `execute_invalid_instruction_24`: The routine contains a single instruction that the disassembler cannot decode as valid 68000 code, so control flow stops at that point.
- `decode_invalid_instruction_21`: The single instruction at this location is not valid 68000 code, so control flow cannot continue past it.
- `trap_on_bad_data`: Is a single bad instruction that terminates flow as undecodable data.
- `halt_on_invalid_opcode_46`: The routine's single instruction is not valid 68000 code, so execution stops there; it performs no data operation.
- `decode_invalid_instruction_22`: The single instruction at this location is not valid 68000 code, so the disassembler cannot decode it as a routine.
- `execute_invalid_instruction_25`: The routine's single instruction is not valid 68000 code, so execution cannot continue past it; the decompiler reports bad instruction data and truncates control flow here.
- `call_increment_word`: Calls a routine that increments a 16-bit value at an offset from a base pointer.
- `call_indirect_via_pc`: Calls a routine through an indirect pointer fetched from program memory, then returns the value left in a data register.
- `call_indirect_through_pc`: Calls the routine whose address is held in memory through the program counter, then returns the value left in a data register.
- `call_indirect_through_pc_2`: Calls a routine through an indirect pointer held in the program counter, then returns the value left in a data register.
- `decrement_word_at_offset`: Decrements the 16-bit value located at offset 32 from the base pointer in register A6 and returns zero.
- `increment_word_at_offset`: The routine increments a 16-bit value located at offset 32 from a base pointer stored in register A6.
- `halt_on_invalid_opcode_47`: The routine contains no valid instructions; execution stops at an invalid opcode, so it performs no data operation.
- `halt_on_invalid_opcode_48`: The routine contains no valid instructions; execution stops on an invalid opcode, so it performs no data operation.
- `execute_invalid_instruction_26`: The routine's single instruction is not a valid 68000 opcode, so control flow cannot be decoded and the routine performs no recoverable operation.
- `execute_invalid_opcode_38`: The routine consists of a single instruction that is not a valid 68000 opcode, so execution traps rather than performing any operation.
- `return_empty_stub`: The routine is empty: it performs no operations and simply returns to its caller.
- `return_immediately_13`: The routine performs no operations other than returning to the caller; its three instructions decode to a no-op sequence and an immediate return.
- `empty_return_routine`: The routine performs no operations other than returning to the caller; its eight instructions decode to a no-op sequence and an immediate return, so it is effectively empty.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.
- `call_indirect_chain`: The routine calls a function, loads a global variable, then calls two functions through pointers derived from that variable; if the second call returns nonzero, it calls a function with an adjusted parameter and the returned value, then calls another function and returns its result, otherwise returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_void`: The routine is empty and performs no operations before returning.
- `return_immediately_6`: The routine is empty and performs no operations before returning to the caller.
- `do_nothing`: the routine is empty and performs no operations before returning.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `perform_no_operation`: the routine is empty and performs no action.
- `empty_routine`: This routine is empty and performs no action.
