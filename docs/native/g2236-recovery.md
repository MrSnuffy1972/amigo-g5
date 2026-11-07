# Game 2236 (wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02236). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 27 of 27 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 279 of 279 named (0 author symbols), 279 of 279 described (40 by rule, 62 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1639 of 1639 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8571 of 8580 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16316 |
| code | 25864 |
| data | 844137 |
| padding | 14291 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Setmap | 13 | 653 | 31 |
| Else | 2 | 117 | 2 |
| isdosdev | 7 | 45 | 2 |
| Type | 1 | 426 | 2 |
| SetPatch | 24 | 833 | 45 |
| addbuffers | 2 | 117 | 2 |
| multidie | 7 | 45 | 2 |
| wipeout | 7 | 45 | 2 |
| remove | 7 | 45 | 2 |
| EndCli | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| lab | 1 | 2 | 1 |
| Cls | 2 | 20 | 1 |
| FastMemFirst | 7 | 207 | 6 |
| CD | 2 | 117 | 2 |
| mouse | 2 | 4 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| memavail | 7 | 45 | 2 |
| setpatchV38.31 | 2 | 893 | 76 |
| Version | 11 | 574 | 18 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Type | 2 | 117 | 2 |
| d | 1 | 13 | 1 |
| gb | 1 | 4 | 1 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:InstallWingCommander | 2 | 3734 | 67 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing | 8 | 45 | 2 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:bootblock | 1 | 126 | 2 |

## Cases handled

- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PLANET0.PKT: high-entropy data file (7.70 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:ENDGAME.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:FONTS.PKT: high-entropy data file (7.88 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP03.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP01.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME2.PKT: high-entropy data file (7.94 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:DEMO.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME0.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP02.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:DEMOSAMP.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME3.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP00.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME1.PKT: high-entropy data file (7.91 bits/byte), classified as packed data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/multidie: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Setmap

- `run_setmap_with_parsed_arguments`: Program entry for the Setmap tool: it allocates a message structure, opens the dos library, and parses its command text into an argument list, handling quoted strings and star escapes for newline and escape characters.
- `release_program_resources`: Releases a program's resources: it closes its file, closes the library it opened, replies to its message port, frees the memory block, and returns the saved result value.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `apply_setmap_library_updates`: Conditionally calls library routines on stored global values, then calls another routine with the incoming parameter.
- `store_parameters_and_call_routines`: Stores its two parameters in global variables, then calls another routine with a fixed set of global references and constants, followed by a second call with a constant.
- `load_keymap_from_arguments`: Loads a keyboard keymap chosen by the arguments or the KEYMAP tool type: it lowercases the name, looks it up among the installed keymaps, and reports missing arguments, missing libraries or an unopenable keymap file.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_with_stack_slot_pointer`: Calls another routine with a global value, the incoming parameter, and the address of an eight-byte stack slot.
- `call_indirect_table_entry`: Calls a function whose address is read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_forward_length`: Invokes a function pointer stored in a global variable, then measures the null-terminated text in a 140-byte stack buffer and passes the buffer and its length to another routine when the length is nonzero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_routine`: Copies sixteen longwords from a fixed offset in the base structure into a caller-supplied buffer when one is given, then calls a routine through an address register.

### isdosdev

- `clear_matching_patch_signature`: Runs a fixed sequence of library calls and a short loop, frees memory through the library base, then checks a structure field and, when a signature matches, saves and clears a word.
- `decrement_until_minus_one`: Decrements a 16-bit local counter until it reaches minus one, then returns.

### Type

- `run_type_command_with_options`: Program entry for the Type command: it parses the FROM, TO, OPT, HEX and NUMBER arguments, rejects conflicting HEX and NUMBER options, and then prints each named file in the requested display format.
- `format_file_dump_lines`: Reads a file in blocks of sixteen bytes and prints each line with an offset column, a grid of byte values, and a text column, or as decimal numbers when requested.

### SetPatch

- `run_setpatch_with_parsed_arguments`: Program entry for the SetPatch tool: it allocates a message structure, opens the dos library, and parses its command text into an argument list, handling quoted strings and star escapes.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_guarded_routine_sequence`: Calls one routine, then when a global flag is set calls another routine with the incoming parameter and passes that result to a third routine.
- `install_setpatch_patches`: Applies the SetPatch patches: it checks for the r option and the usage question mark, then patches exception vectors, alert and allocation entries, layers, trackdisk and user state, reporting each patch that was applied or skipped.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: Calls a routine through a pointer stored in a global structure and returns the value that call produces.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Forwards control to the library function stored at offset minus 198 from the global base pointer and returns, with no other work.
- `call_library_vector_294`: Forwards control to the library function stored at offset minus 294 from the global base pointer and returns, with no other work.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_414`: Forwards control to the library function stored at offset minus 414 from the global base pointer and returns, with no other work.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Forwards control to the library function stored at offset minus 552 from the global base pointer and returns, with no other work.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the unable to be met, computes a checksum over a short array in the structure and stores its complement, then returns.
- `update_checksum_then_jump`: Writes the one's complement of the 16-bit sum of a 22-word field in the structure into a trailing word.
- `open_disk_resource_and_patch`: Opens the disk resource through a library call and, if it is present and its 16-bit field is below 36, updates a counter in the structure and stores two library results in globals.
- `check_flag_and_copy_word`: Calls the routine pointed to by a stored handler slot and, when it returns nonzero, copies the 32-bit field at a fixed offset in the global structure into a global word.
- `jump_to_pending_handler`: Compares a stored word with a field of the global structure and returns unless they differ and the stored word is not minus one.
- `check_pending_handler`: Compares a stored word with a field of a global structure and executes another routine if the word is not minus one and if they differ.
- `initialize_signature_and_dispatch`: Sets a flag word to 16384.
- `jump_through_dispatch_pointer`: Jumps through a routine pointer at offset 136 from a global structure pointer, with no other work.
- `initialize_patch_function_table`: Stores a computed address in a global and fills a 16-entry table at a low fixed address with successive addresses of a routine, advancing two bytes per entry.
- `call_routine_sixteen_times`: Calls the same routine sixteen times in succession and returns the value produced by the final call.
- `return_input_value_2`: Returns the 32-bit value passed in the data register unchanged.
- `call_routine_fifteen_times`: Calls the same routine fifteen times in succession and returns the value produced by the final call.
- `return_input_value`: Returns the 32-bit value passed in the data register unchanged, performing no other operations.
- `call_setfunction_if_small`: Returns without action when a 16-bit count field in the global structure exceeds 35.
- `allocate_and_copy_pointer_table`: Allocates a block through a library call, then copies a table of 32-bit pointer and value pairs from the source structure into it.
- `free_table_and_return_flag`: Repeatedly calls the second library vector, which releases a block, once for each remaining entry counter value and once more.
- `free_entry_and_return_flag`: Calls the second library vector, which releases a block, then returns the word at the stored pointer with its top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `check_field_then_dispatch`: Passes a field of the structure to a checking routine.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker_word`: Advances a pointer through 16-bit words until the high byte of the current word equals 97, then returns a pointer to that word.
- `decode_instruction_displacement`: Reads a displacement from the instruction word at the pointer, using its low byte when that byte is nonzero and otherwise the following word.
- `compute_word_displacement_target`: Reads a 16-bit value from the word two bytes past the pointer, shifts it right by one, and sign-extends it.
- `walk_and_store_three_pointers`: Calls a helper routine in a chain of six steps, each call advancing from the pointer returned by the previous one.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_60`: Calls the library routine at offset minus 60 from a global base pointer with no arguments and returns.
- `call_routine_with_stack_slot`: Forwards its two parameters to a second routine together with the address of a fixed stack slot, then returns.
- `format_text_and_forward`: Calls a library routine that formats text into a 140-byte stack buffer, then measures the length of the resulting null-terminated string.
- `store_byte_at_pointer`: Writes the low byte of the data register to the memory location addressed by the address register, then returns.

### addbuffers

- `obtain_block_and_install_patch`: Walks a linked chain of entries to find the largest recorded size, starting from 149, then obtains a table block through library calls and fills it with a counting pattern.
- `copy_block_and_call_vector`: If the pointer argument is non-null, copies sixteen 32-bit words from a fixed offset in the base structure into the pointed-to location.

### multidie

- `release_blocks_and_swap_entry`: Calls helper routines and a library vector, then runs a three-iteration loop that calls a memory-release library vector.
- `decrement_until_minus_one_3`: Counts a 16-bit local value down until it equals minus one, then returns.

### wipeout

- `reset_install_state_at_entry_2`: Calls routines and a library function in a loop, clears a word at a fixed location, and swaps a memory word with a global variable if certain conditions or constants match.
- `decrement_until_minus_one_5`: Decrements a local 16-bit variable until it reaches minus one and then returns.

### remove

- `reset_install_state_at_entry_3`: Performs a loop that calls routines and a library function, clears a word, and swaps a memory word with a global variable if a specific constant is matched.
- `decrement_until_negative_one`: Decrements a local or temporary register value until it reaches negative one and then returns.

### EndCli

- `update_map_argument_list`: Iterates through a linked list to find the maximum length, then populates a table with an incrementing sequence and copies argument data or routine pointers into a target buffer.
- `copy_stack_data`: copies a fixed number of words from a source pointer to a destination pointer if the destination pointer is not null.

### Echo

- `populate_argument_list_buffer`: Iterates through a linked list or table to find a maximum value, then populates a buffer with a sequence of incrementing or fixed-value longwords.
- `copy_memory_block`: Copies a fixed number of dwords from a source pointer to a destination pointer if a certain pointer is not null.

### lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Cls

- `open_dos_library_and_check`: Opens dos.library through the library vector at offset minus 552, calls two further vectors, and if the second returns a negative value calls a third vector and returns its result.

### FastMemFirst

- `parse_command_line_arguments`: Obtains a command-line buffer and, when a command line is present, copies it into a null-terminated string and splits it into arguments stored in a linked list.
- `find_and_copy_fastmem_block`: Walks a linked list of memory blocks to identify a fast-memory block with a specific type tag and a chip-memory block with a different type tag, then executes a routine on the fast block and another routine with the identified block pointers.
- `call_library_vector_132`: Calls the library routine at offset minus 132 from the global base pointer and returns.
- `call_library_vector_138`: Calls the library routine at offset minus 138 from the global base pointer and returns.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### CD

- `populate_argument_table`: Iterates through a linked list or pointer table to find a maximum value, then populates a structured table or array with a sequence of incrementing values and copies data from a source buffer to a destination buffer.
- `copy_stack_frame_data`: copies a fixed block of sixteen bytes from a memory location offset from a pointer into the current stack frame, then calls another routine.

### mouse

- `wait_while_flag_bit_set`: Loops while bit 64 of a global flag word is set, then returns zero.

### EndIf

- `return_zero_status`: Returns zero immediately without other work.

### If

- `initialize_argument_list_and_copy_data`: Calculates the maximum length of a linked list, initializes a message structure with a new argument list, and copies a block of data or a routine pointer into a destination buffer using a loop.
- `copy_memory_block_2`: Copies a fixed number of words from a source pointer to a destination pointer, provided a certain check passes.

### memavail

- `reset_install_state_at_entry`: Performs an initial call, then executes a loop that calls a library function and decrements a counter, and finally checks a memory constant to swap a word with a global variable and clear it.
- `decrement_until_minus_one_4`: Decrements a local variable until it reaches negative one and then returns.

### setpatchV38.31

- `parse_switches_and_apply_patches`: Parses the QUIET, NOCACHE and REVERSE switches, checks the Kickstart version requirement, and applies a list of patches through library calls while printing status text.
- `check_vector_then_copy_stub`: Calls a library vector and, when the result is nonzero and bit 8 of a flag byte in the global structure is set, allocates a memory block, copies a 50-word stub routine into it, and returns one.
- `copy_stub_if_flagged`: If bit 8 of a flag byte in the global structure is set, allocates a memory block, copies a 50-word stub routine into it, runs library calls on it, and returns one.
- `copy_stub_and_run_patch`: Allocates a memory block and, if that succeeds, copies a 50-word stub routine into it and runs library calls on it.
- `probe_memory_and_update_control`: If a byte in low memory is not 127, it probes a high memory address by reading back a few words, derives a small mask from a register value, and updates a control byte in low memory with that mask.
- `release_and_clear_pointer_slots`: If a pointer field in the structure is set, it clears the field and releases the block through a library vector.
- `update_message_structure`: Checks certain field values in a message structure and, if conditions are met, copies data, clears a flag, and calls a library routine.
- `dispatch_message_and_reply_alt`: Performs the same message handling as its neighbouring routine, entered one instruction later, and otherwise passes control to a shared routine.
- `reply_message_when_type_four`: If the message's byte value is 4, copies stored values into the message, clears flag bytes, and replies through the ReplyMsg library vector.
- `branch_on_empty_message`: Checks if a message field is zero and, if so, copies data, clears a flag, and calls a library routine.
- `call_two_routines_return_zero`: Calls two routines in sequence with no arguments and returns zero.
- `set_flag_around_library_calls`: Looks up an object through a library vector and, when the object is found and its status byte is negative, runs a sequence of library calls with that status byte set to one.
- `call_one_routine_only`: A two-instruction routine that makes a single call to another routine and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_single_routine_and_return`: Calls one routine with no arguments and returns.
- `store_library_result_globals`: Calls a library vector and, when the result is nonzero, stores the result, a field read from it, and the base pointer into three globals.
- `range_check_and_dispatch`: If the argument lies within a stored range, calls a library vector and, if it returns nonzero, calls a routine and another vector.
- `jump_through_saved_handler`: Tail-jumps through a saved routine pointer with no other work.
- `check_upper_bound_and_dispatch`: If the argument is at or below a stored upper bound, calls a library vector and, if it returns nonzero, calls a routine and another vector.
- `store_offset_by_variant`: Selects one of two fixed offsets depending on whether a 16-bit field of the input structure equals 44, then calls a library vector.
- `clear_matching_table_entry`: Looks up a table entry by the argument index.
- `set_pattern_on_match`: Calculates an offset and, if a pointer in a table matches a specific routine, sets two adjacent memory locations to a constant value.
- `patch_table_and_branch`: checks if a table entry matches a certain routine and, if so, overwrites two adjacent table entries with a constant before branching to another routine.
- `execute_stored_routine`: executes a routine stored at a specific memory location.
- `run_vectors_and_set_flag`: Calls three library vectors in sequence, sets a global flag word to 64, and returns one.
- `store_argument_and_return_one`: Stores the register argument in a global word and returns one.
- `patch_table_with_pattern`: Calls a routine to get a value, updates a table, and if a specific constant is detected, fills a memory range with a pattern.
- `update_table_and_clear_memory`: Updates a table entry with a return value from another routine and, if a specific constant value is detected, performs a block-fill of a memory range with a repeating pattern using a pointer ors derived from a secondary table.
- `initialize_memory_buffer`: Checks if a value returned from a routine matches a specific constant and if a corresponding table entry is non-zero, then populates a memory range with a repeating pattern using a decrementing counter.
- `update_table_and_fill_memory`: compares a value from another routine against a table, updates the table if they differ, and fills a memory block with a pattern if the value matches a specific constant.
- `add_library_result_twice`: Calls a library vector and, when the result is nonzero, adds that result to a stored word twice.
- `copy_field_on_matching_event`: If the stack argument equals 96994 and the second argument equals 21264, and bit 3 or bit 4 of a status field is set, copies a longword field to another field.
- `copy_field_on_event_code`: If the second argument equals 21264 and bit 3 or bit 4 of a status field is set, copies a longword field to another field.
- `clear_bit_and_call_vector`: Selects a target pointer from a library vector when bit 0 of a flag byte in the global structure is set, and otherwise a fixed low-memory address.
- `search_node_list_locked`: Searches a linked list of nodes for one whose 16-bit field matches the argument, while multitasking is suspended through library vectors.
- `search_linked_list_node`: Calls a library routine, then traverses a linked list of nodes, checking a field in each node, before calling another library routine and returning a status.
- `check_and_traverse_list`: Checks if a register contains a non-zero value, then executes a routine, traverses a linked list, and executes another routine.
- `traverse_list_with_checks`: executes a routine, then traverses a linked list of nodes checking a certain field before executing another routine.
- `traverse_linked_list_with_side_effects`: Calls a routine via a pointer-offset, then iterates through a linked list of structures, terminating if a certain field matches a value, and calls another routine via a pointer-offset before returning a status check.
- `traverse_list_and_execute`: executes a routine, then iterates through a linked list of structures until a certain field matches a value, and executes another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `traverse_linked_list`: executes a routine via a pointer, iterates through a linked list of structures until a null pointer or a specific termination condition is met, and executes a final routine via another pointer.
- `traverse_list_and_return_constant`: executes a routine, traverses a linked list, and returns a constant value if a certain condition is met.
- `check_and_traverse_list_with_error`: Checks if a register contains a specific negative value, then executes a routine, traverses a linked list, and executes another routine.
- `traverse_list_with_error_return`: checks if a register contains a specific value, executes a routine, and then traverses a linked list, returning a specific error value or the register value.
- `search_node_list_locked_2`: Searches a linked list of nodes for one whose 16-bit field matches the argument, while multitasking is suspended through library vectors.
- `execute_and_traverse_list`: Executes a routine via a pointer, traverses a linked list of structures while checking a specific field for a termination value, and then executes a second routine.
- `search_node_list_or_param`: Searches a linked list of nodes for one whose 16-bit field matches the argument, while multitasking is suspended through library vectors.
- `release_lock_and_return_value`: Calls a library vector that resumes multitasking, then returns the stack argument when the pointer argument is nonzero and the third parameter otherwise.
- `return_parameter`: Returns the input parameter unchanged.
- `search_word_list_store_pointer`: Chooses a pointer slot from a fixed low address, or from a library vector when bit 0 of a flag byte in the global structure is set.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `jump_unless_marker_word`: If the word at a stored address is not the constant 24832, it tail-jumps through a stored routine pointer.
- `branch_on_marker_word`: checks if a word at a certain address matches a specific value and, if not, executes a routine via a pointer.
- `forward_to_next_routine`: Calls one further routine and returns with no result, adding no other work.
- `allocate_vec_and_copy_string`: Measures a null-terminated argument string, allocates a buffer with exec.AllocVec, and copies the string into the buffer while recording its length in the buffer.
- `insert_length_prefixed_string`: Measures a null-terminated string, moves the existing buffer contents to make room, copies the bytes backward into the buffer and writes the length as a prefix.
- `shift_bytes_and_store_length`: Moves a block of bytes one position backward for the count held in a register, stores a fill byte before the block, terminates the string at its length plus one, and calls a library vector.
- `return_void`: this routine is empty and performs no operations before returning.
- `call_library_vector_306`: Calls the library routine at offset minus 306 from the global base pointer and returns its register result pair.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `decrement_count_and_pop_node`: Increments a counter byte in the global structure and decrements a count in the entry.
- `alert_or_decrement_count`: If the condition flag is negative, calls an alert library routine and a second library routine.
- `alert_and_init_semaphore`: Calls the alert library routine and initializes a semaphore through a library vector, then calls another library vector and returns.
- `process_flagged_list_nodes`: Removes nodes from an entry's list, calling a library vector for each node whose flag bit is set and relinking the nodes as it goes.
- `process_flagged_list_nodes_2`: Increments a counter, calls a library routine, and removes nodes from a list if a specific bit is set, relinking the remaining nodes.
- `clear_status_bit_and_release`: Increments a counter byte in the global structure and a count field of the entry.
- `clear_status_bit_and_release_2`: If a pointer field in the entry is set, clears a status bit, calls two library vectors and resumes multitasking.
- `check_stack_bounds_and_call`: Calls a stored routine when the stack address falls outside a range recorded in the global structure or when a stored flag equals minus one.
- `check_stack_bounds_and_call_2`: Calls a stored routine when the stack address falls outside a range recorded in the global structure or when a stored flag equals minus one.
- `check_stack_bounds_and_call_3`: Checks if a pointer or a value in a table falls outside a certain range or equals a specific value, then calls a routine.
- `call_handler_when_stack_outside`: calls the routine held in a global pointer when the stack pointer lies outside a range recorded in a structure, or when a field of another structure equals minus one.
- `store_two_values_return_one`: Stores the address-register argument and the base pointer into two globals and returns one.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `call_single_routine_and_return_2`: Calls one other routine and returns.
- `call_single_routine_and_return_3`: Calls one other routine after a short sequence of instructions and returns.

### Version

- `parse_current_directory_string`: Retrieves the current directory path and parses it into a list of tokens, handling quoted strings and converting certain escaped characters into newlines or escape codes.
- `cleanup_task_record_and_reply`: Finds the current task and stores a value in its per-task record.
- `print_library_version_report`: Parses a library or device name with optional version, revision and unit arguments, opens it, and prints its version with the Kickstart and Workbench versions.
- `call_routine_with_fixed_argument`: Calls the first routine with the constant 128 and then calls the second routine with its second argument.
- `parse_signed_decimal_string`: Parses an optional plus or minus sign followed by decimal digits from a string and returns the signed integer value.
- `compare_null_terminated_strings`: Compares two null-terminated byte strings and returns minus one when the first is less than the second, one when it is greater, and zero when they match.
- `measure_string_length`: Returns the number of bytes before the terminating zero byte of a string argument.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_function_pointer`: invokes a function pointer stored in a global variable.
- `recurse_with_stack_slot`: calls another routine passing two parameters and a pointer to a specific location on the stack.
- `invoke_library_function`: invokes a function pointer stored in a global variable via a library base.
- `call_library_vector_210`: Forwards control to the library function stored at offset minus 210 from the global base pointer and returns, with no other work.
- `invoke_library_function_offset`: invokes a function pointer stored in a global variable with a negative offset.
- `call_library_vector_444`: Forwards control to the library function stored at offset minus 444 from the global base pointer and returns, with no other work.
- `call_library_vector_450`: Forwards control to the library function stored at offset minus 450 from the global base pointer and returns, with no other work.
- `invoke_library_function_offset_large`: invokes a function pointer stored in a global variable with a large negative offset.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_register_to_memory`: stores the contents of a register to a memory location pointed to by another register.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Type

- `build_argument_list_buffer`: Calculates the maximum length of a linked list of arguments, then initializes a buffer or structure with a sequence of incrementing or fixed values, copies certain data blocks, and appends a pointer to a routine or a list terminator.
- `copy_data_loop`: Copies a fixed number of words from a source pointer to a destination pointer.

### d

- `stub_bad_instruction_data`: Stub made of data rather than code: the decompiled output reports bad instruction data, so no routine behaviour is recorded.

### gb

- `halt_on_bad_instruction`: The routine contains invalid or bad instruction data.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:InstallWingCommander

- `open_dos_and_setup_installer`: Program entry that opens dos.library, duplicates a lock on the current directory, allocates working memory and waits for messages through library calls.
- `release_task_record_and_reply`: Stores a parameter in a slot of a per-task record and, when a count field exceeds 35 and a pointer is set, decrements the count and calls a library vector.
- `run_delayed_call_sequence`: Checks a stack-limit field against the current stack address, then runs a long sequence of calls including a dos.Delay wait, and returns an integer status.
- `query_volume_info_with_lock`: Locks a path, queries disk information through the dos.Info library call and releases the lock, using a memory block obtained and freed through exec library calls.
- `scan_fixed_size_entry_table`: Walks a table of 16-byte entries referenced by the global structure, testing a byte at offset 14 of each entry, and makes one call with the result.
- `process_named_entry_with_checks`: Checks a stack-limit field, then processes a named entry through five routine calls and returns an integer status.
- `copy_file_with_protection`: Opens a file through dos calls, examines and reads its contents into memory obtained from exec, writes the data to a destination file and sets its protection bits.
- `process_path_string_with_calls`: Processes a path string argument through six routine calls and returns a status value.
- `create_directory_path`: Walks a path string, examining each component with dos.Examine and creating missing directories with dos.CreateDir, using locks and temporary memory obtained from exec.
- `return_immediately`: The routine executes a return instruction immediately upon entry, performing no other operations despite the instruction count suggesting additional code may be present or filtered.
- `return_immediately_2`: The routine executes a return instruction immediately upon entry, performing no other operations.
- `free_linked_lists_in_structure`: Frees every node of two linked lists held in the global structure by calling the FreeMem library vector for each node, clears the list heads, and returns zero.
- `split_argument_fields`: Splits a text argument into fields at slash characters, honouring backslash escapes, and records the start offset of each field in an output array terminated by minus one.
- `build_path_string_from_parts`: Copies a name into a 63-character stack buffer, appends further components with separator handling, and calls one routine with the assembled string.
- `append_path_separator_if_needed`: Appends a separator character from a global table to the end of a string when its last character is not a slash, colon or backslash, then calls one routine.
- `copy_string_return_end`: Copies bytes from a source string to a destination until a zero byte is copied, and returns the destination pointer at that zero byte.
- `format_decimal_digits`: Repeatedly divides a number by calling a routine and writes each remainder as an ASCII digit, then reverses the digits into the output string.
- `format_octal_digits`: Converts an unsigned value into octal digits by repeatedly taking its lowest three bits, then copies the digits in reverse order into the output string with a terminating zero.
- `format_hex_digits`: Converts an unsigned value into hexadecimal digits using a 16-character table, then copies them in reverse order into the output string with a terminating zero.
- `parse_signed_integer`: The routine reads a signed decimal integer from a zero-terminated string pointed to by a register, handling an optional leading plus or minus sign.
- `open_buffer_or_set_error`: If bit 3 of a flag byte in the record is clear, calls a routine to obtain a buffer and stores it in two fields.
- `convert_string_to_uppercase`: Converts each character of a zero-terminated string to upper case when that character's entry in a global attribute table has bit 1 set.
- `write_file_with_error_code`: Checks a file handle's flag byte and, when the handle is writable, writes data through the dos.Write library call.
- `check_handle_and_set_error`: Calls two routines to look up a file handle.
- `read_file_with_error_code`: Looks up a file handle and, when it has the required flag bit, reads data through the dos.Read library call.
- `seek_file_position`: Looks up a file handle and seeks through the dos.Seek library call, recording an error code from dos.IoErr when the handle is missing or the seek fails.
- `close_file_handle`: Looks up a file handle and returns minus one when it is missing or has no open flag.
- `open_file_with_protection`: Locks and opens a named file through dos calls, sets its protection flags and unlocks it.
- `multiply_32bit_values_wide`: Multiplies two 32-bit register values and returns a 64-bit product, built from 16-bit partial products with special handling when the high halves are zero.
- `multiply_32bit_values_low`: Returns the low 32 bits of the product of two 32-bit register values, built from 16-bit partial products.
- `return_empty_subroutine`: This routine is empty and performs no operations before returning.
- `close_file_list_entries`: Sets a status flag to one, then walks a linked list of file entries.
- `open_nil_or_redirect_io`: Examines the current task and opens an input or output stream through dos.Open, falling back to the NIL: device name, then sets the standard input and output with dos.Input and dos.Output.
- `copy_string_up_to_79`: Copies a string of at most 79 characters from a structure field into a local 81-byte buffer and returns the length copied.
- `print_break_message_and_open`: Copies up to 79 characters from a structure field into a local buffer, opens intuition.library through exec, and writes a break message with dos.Write.
- `get_list_entry_by_index`: Returns the entry at a given index of a linked list in the global structure, walking forward from the head.
- `poll_break_signal_bits`: Reads the task's signal state through the SetSignal library vector and keeps the bits selected by the mask 12288.
- `find_byte_in_buffer`: Searches a buffer of a given length for a specific byte value, returning a pointer to its first occurrence or null when the buffer does not contain it.
- `append_string_to_buffer`: Copies a counted number of bytes from a source string to the end of a destination string, then returns.
- `concatenate_zero_terminated_strings`: Finds the terminating zero byte of the first string, copies the second string including its terminator after it, and returns the first string's pointer.
- `copy_counted_bytes`: Copies a counted number of bytes from a source pointer to a destination pointer and returns when the count is exhausted.
- `copy_bytes_forward_if_below`: When the count is positive, copies that many bytes from source to destination in forward order if the destination address lies below the source; otherwise it returns.
- `release_matching_memory_block`: Returns immediately for a null pointer.
- `release_matching_memory_block_2`: Performs the same record search as its neighbouring routine, releasing the matching block through the FreeMem library vector.
- `start_block_allocation_or_fail`: Records error code 22 and returns null when its size argument is zero.
- `allocate_memory_block_record`: Allocates a memory block through the exec.AllocMem library call for a requested size and returns a pointer to it.
- `run_callbacks_and_scan_table`: Calls each routine stored in a linked list of callbacks in the global structure, then walks a fixed table of records testing their flag bits.
- `append_byte_to_buffer`: Increments a counter in the buffer header, stores the byte at the next position, and when the count exceeds 511 calls one routine and clears a second field.
- `return_status_from_flag_mask`: Calls one routine with the argument and a stack slot, then returns minus one when bits 4 or 5 of a status field are set and zero otherwise.
- `read_characters_until_terminator`: Reads input characters one at a time, taking them from a buffer when a flag bit is set and otherwise through a routine call.
- `process_flagged_entry_status`: When the argument matches the main entry and a flag bit is set, calls a routine and classifies its returned value as positive, zero or negative, updating a status byte to match.
- `check_entry_flags_or_error`: Returns minus one and records error code 9 when a field of the argument structure is zero.
- `calculate_file_offset`: Calculates a file offset by counting newline characters in a buffer and adjusting the result based on specific flags and a length field.
- `reset_entry_and_call_routines`: Calls one routine when a flag bit is set, optionally calls another when the mode argument equals one, then copies a stored field into another field of the structure and calls a further routine.
- `check_entry_flags_for_name`: Returns zero when either count argument is zero.
- `write_bytes_to_stream_node`: Copies bytes from a source pointer into the node's current output buffer one at a time, up to the node's remaining count, and calls a helper after each newline when a newline flag is set.
- `read_bytes_from_stream_node`: Copies bytes from the node's current buffer position into a caller buffer, fetching a fresh chunk through a helper whenever the node's remaining count runs out.
- `update_file_node_state`: With no node argument, walks the linked list of nodes from a fixed head and records an error when a helper called on an active node reports failure.
- `close_stream_node`: Returns minus one if the node is missing or has no state set.
- `close_stream_node_via_register`: Performs the same close sequence as the stack-argument variant: it checks the node for presence and state, flushes pending output through a helper when flagged, clears the state word, calls a second helper, and returns zero or minus one.
- `parse_open_mode_string`: Reads a two-character mode string whose first character is a, r or w, with optional plus and b markers, and turns it into access flags.
- `find_or_allocate_free_node`: Searches the linked list of nodes from a fixed head for one whose active field is zero, allocating a new node through a helper when none is free.
- `format_one_printf_conversion`: Parses one printf-style conversion specifier, reading its flags, width, precision and length modifier, then fetches the argument and formats it as a number, string or character with padding.
- `emit_format_string_text`: Walks a format string, passing ordinary characters and escaped percent signs to an output routine, and handing each percent-prefixed conversion to the specifier parser.
- `delete_file_record_error`: Calls the DOS library's file-delete routine; when the call fails it stores the DOS error code and the result of a second library call.
- `stub_return_immediately`: The routine contains no operations and returns at once, so it is an empty stub with no effect on data, hardware or the caller.
- `forward_call_to_helper`: Calls one other routine and then returns, passing no data through and changing nothing itself.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing

- `release_memory_and_patch_word`: Program entry code that calls a few helpers, then runs three memory-free library calls in a loop.
- `decrement_until_minus_one_2`: Decrements a 16-bit counter in a loop until it passes zero and reaches minus one, then returns.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `walk_length_prefixed_entries`: Walks a table of variable-length entries, advancing by the size stored at each entry's third byte plus three, and calls two library routines per entry.
