# Game 1323 (fa-18 interceptor (1988)(electronic arts)[h j. fabre][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01323). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (913332 top-level bytes in 79 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 43 of 43 units identical (bytes and relocations), 201 raw-word lines |
| Functions named and described | pass, 498 of 498 named (0 author symbols), 498 of 498 described (140 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2225 of 2250 (98.89%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10178 of 10297 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 336676 |
| code | 34636 |
| data | 231233 |
| padding | 16387 |
| unused | 294400 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Prompt | 2 | 117 | 2 |
| binddrivers | 11 | 659 | 39 |
| EndCLI | 2 | 117 | 2 |
| rename | 2 | 117 | 2 |
| newcli | 2 | 117 | 2 |
| LoadWb | 11 | 362 | 22 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 475 | 31 |
| F-18 Interceptor | 189 | 252 | 6 |
| dk | 1 | 3 | 1 |
| is | 1 | 3 | 1 |
| d | 1 | 3 | 1 |
| e | 1 | 3 | 1 |
| f | 1 | 3 | 1 |
| cdn | 1 | 3 | 1 |
| usa0 | 1 | 3 | 1 |
| i | 1 | 3 | 1 |
| usa2 | 1 | 3 | 1 |
| n | 1 | 3 | 1 |
| s | 1 | 3 | 1 |
| gb | 1 | 3 | 1 |
| icon.library | 15 | 1460 | 109 |
| info.library | 25 | 2845 | 181 |
| version.library | 2 | 41 | 7 |
| FA-18 Interceptor (1988)(Electronic Arts)[h J. Fabre][HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 283 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 367 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 295 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 482 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 99 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 58 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 60 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 334 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 189 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 198 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 82 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 252 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 65 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 197 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 3 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 58 64 KB pages each with >= 29491 distinct PCs executed, 981783 of its 1002621 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 58 64 KB pages each with >= 29491 distinct PCs executed, 981783 of its 1002621 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 58 64 KB pages each with >= 29491 distinct PCs executed, 981783 of its 1002621 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 58 64 KB pages each with >= 29491 distinct PCs executed, 981783 of its 1002621 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- 135 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 134 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1850 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 14 first-run image unit(s) in 1 version group(s); 3528 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- fa-18 interceptor (1988)(electronic arts)[h j. fabre][hd]/FA-18 Interceptor (1988)(Electronic Arts)[h J. Fabre][HD].adf:F18/F-18 Interceptor: Imploder (xfd) stream found by ancient scan (149064 -> 328660 bytes) but only 0 of 135 sampled windows are in reference RAM: not used
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-lgpwide)): both forms recovered; writer: not identified statically (indirect store)
- fa-18 interceptor (1988)(electronic arts)[h j. fabre][hd]/FA-18 Interceptor (1988)(Electronic Arts)[h J. Fabre][HD].adf:F18/F-18 Interceptor: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Prompt

- `build_prompt_string_table`: Walks a linked list at offset 356 of a2 to find the largest id, allocates a string table via library calls, fills it with incrementing longword pointers, copies 16 longs from offset 536 of a2, and returns zero or minus one.
- `return_d1_in_d0_tail_branch`: Moves d1 into d0 as the return value and branches into the middle of another routine; it contains no loop of its own.

### binddrivers

- `bind_drivers_from_cli_args`: Saves registers, opens the current directory, splits the command line into whitespace-separated argument pointers, opens the named driver file, allocates memory, stores the loaded driver pointer, and returns zero or an error code.
- `open_and_close_driver_library`: Loads the library name at offset 92 from a4 via the a6 library vector, then closes that same library through the next vector and returns.
- `open_dos_library_store_handle`: Clears the global at address 8, loads the dos.library name into a1, calls the a6 open vector with d0 set to 31, stores the returned handle at address 8 and returns.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `bind_expansion_drivers_main`: Sets up a stack frame, opens expansion and icon libraries, allocates a 256-byte buffer, opens a driver file, loops over its entries binding each driver, then closes everything and returns.
- `find_end_marker_in_stack_frame`: Reads a size from the caller's stack frame, scans memory for the end marker word with a matching self-pointer, and returns that address or zero.
- `walk_string_list_until_null`: Repeatedly finds the next string in a list using a terminator byte, accumulates results through a helper, and returns the accumulated value when the list ends.
- `split_and_merge_string_segments`: Locates a separator in a string, splits it into two parts, validates both, and merges them through a helper, returning zero on failure.
- `find_free_driver_node`: Walks the driver list via offset 48, and for each node whose flags byte has bit 1 set, checks with the second routine whether the given node is already linked; if not, stores the given pointer at offset 48 and returns the new head.
- `list_contains_node`: Follows the next-node pointer at offset 48 from the first argument, returning 1 if the second argument is reached and 0 if the chain ends at null.
- `check_all_bytes_are_digits`: Scans the buffer at a0 for the given count, returning 1 if the count is zero or every byte is between character zero and nine, otherwise 0.
- `parse_decimal_string_to_long`: Reads consecutive ASCII digit bytes from the pointer argument, accumulating them into d0 as a decimal value, stopping at the first non-digit or null byte and returning the result in d0.
- `bounded_string_compare_with_length`: Calls the string compare routine with the given pointers, then checks whether the compared length exceeds the supplied limit, returning zero if it does and the comparison result otherwise.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `compare_null_terminated_strings`: Compares bytes from a0 and a1 until a mismatch or null terminator, returning zero when equal, minus one when a0 is less, and one when greater in d0.
- `load_first_stack_argument_to_a0`: Loads the longword at offset four from the stack pointer into address register a0, exposing the caller's first argument.
- `measure_string_length_inverted`: Scans bytes at a0 until a zero byte or 256 bytes, then returns the bitwise complement of the remaining counter in d0.
- `find_character_in_string`: Reads a string pointer and a target byte from the stack, scans for that byte, and returns a pointer to it or zero if the terminator is reached first.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination one at a time until a zero byte is copied, then returns.
- `find_common_prefix_offset`: Measures the lengths of two strings, compares them, and if the first is not shorter, checks the tail of the first against the second, returning the offset or zero.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### EndCLI

- `endcli_scan_and_launch_program`: Walks a resident module list to find the highest priority, allocates memory, builds a command line, copies a sixteen-longword structure, and calls library vectors through a5.
- `copy_d1_to_d0_then_branch`: Moves the longword in d1 into d0 and then branches unconditionally to another part of the same routine.

### rename

- `rename_allocate_and_copy_name`: Calls library routines through a5 to find the largest name length, allocate a block, fill it with name entries, copy sixteen longwords into the structure at offset 536, and return zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and then branches forward to another routine, so d0 carries d1 as the value passed on.

### newcli

- `allocate_and_init_cli_structure`: Finds the highest CLI number in the task list, allocates a CLI structure, fills its command buffer with token pointers, copies the 16-entry table, and returns zero or minus one.
- `return_d1_in_d0_then_jump`: Moves d1 into d0 as the return value and branches to the code at offset 260, which continues the caller's cleanup.

### LoadWb

- `loadwb_startup_parse_args`: Saves registers, reads the current directory and task, splits the command line into argument pointers, opens the startup file, and runs the main program.
- `close_library_and_reply_msg`: Saves the argument, switches to the caller's stack, closes the library at global pointer if set, then if the message pointer is set calls Forbid and ReplyMsg on it, returning the argument.
- `open_and_close_dos_library`: Loads the address of the name string at offset 92 from a4 into a0 and calls the exec open-library vector, then calls the close-library vector with the same name pointer, returning.
- `open_dos_library_store_result`: Clears the global longword at offset 8, loads the dos.library name into a1, sets d0 to 31 and calls the exec open-library vector, storing the result and returning it.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input_2`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output_2`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `load_workbench_tool_and_run`: Opens the workbench library, checks the resident tag, loads the tool file, allocates memory, calls the tool entry and frees the loaded segments before returning.
- `compare_two_strings_lexically`: Compares the two null-terminated strings passed on the stack byte by byte and returns zero when equal, minus one or one for ordering.
- `load_segment_list_into_memory`: Walks a linked list of segment sizes, allocates memory for each, loads the segment data and links the blocks, returning the first allocated block.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_vector_with_second_arg`: Saves d2, loads the second stack argument into d2, pushes a pointer to the third argument and that value, then calls the function pointer at absolute address 20 and restores the stack and d2.
- `format_string_via_rawdofmt`: Saves registers, calls exec RawDoFmt with a format string into a 140-byte stack buffer, measures the resulting string length, and passes it with the buffer to another routine.
- `store_d0_byte_to_a3`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3, then returns to the caller.

### Port-Handler

- `port_handler_init_and_dispatch`: Initializes a port structure from a0 and a1, clears message slots, calls library vectors through a5, and dispatches on message types to update port fields.

### Disk-Validator

- `disk_validator_block_scan`: Scans disk bitmap blocks using a0 and a2 fields, calls DOS library vectors via a5, and marks or reports used and free blocks during validation.

### Ram-Handler

- `ram_handler_copy_mem_thunk`: Saves a1 and a6, moves d1 and d2 into a0 and a1, calls exec CopyMem through the library vector, restores registers and jumps to a6.
- `loads_long_pairs_and_branches`: Reads two longwords from offsets 196 and 200 of the structure in a1 into d2 and d1, loads constants into d3 and d0, and branches into a shared code path.
- `loads_long_words_and_branches_2`: Reads two long words from offsets 196 and 200 of the structure in a1 into d2 and d1, sets d3 and d0 to constants, and branches into a shared section of code.
- `loads_long_words_and_branches`: Loads two long words from offsets 196 and 200 of the structure in a1 into d2 and d1, sets d3 and d0 to constants, and branches into shared code elsewhere.
- `calls_handler_twice_then_branches`: Loads a long from offset 200 of a1 into d1 and sets d0 to 232, then calls through a5 with a4 offset by 1832.
- `calls_handler_with_offset_pointer`: Loads a long from offset 200 of a1 into d1 and sets d0 to 232, then calls through a5 with a4 offset by 1892.
- `passes_zero_and_long_args`: Clears d3 and loads longs from offsets 196 and 200 of a1 into d2 and d1.
- `branches_to_shared_tail_for_a1`: Executes fifteen instructions and then branches forward to a later point in another routine.
- `branches_to_shared_tail_from_a1`: Executes fifteen instructions and then branches forward to a later point in another routine, sharing a tail with a neighboring routine.
- `dispatches_to_shared_tail`: Contains no body of its own beyond a single unconditional word branch that transfers execution to a shared tail routine elsewhere in the code.
- `runs_long_sequence_then_branches`: Executes a sequence of 29 instructions of mixed move, shift, and register-based load operations, then transfers execution to a shared tail routine through an unconditional branch.
- `calls_indirect_twice_then_branches`: Loads a long from offset 200 of the structure in a1 into d1 and an offset-dependent value into d0, calls the routine pointed to by a5 twice with different values, stores a long from offset 620 of the structure in a2 into d3, and then branches to the shared tail.
- `copies_long_pair_via_indirect_call`: Loads a long from offset 200 of the structure in a1 into d1, copies it into d2, loads a second long from offset 220 into d1, and calls through a5 with a parameter from offset 196 of the structure in a2 before branching to the shared tail.
- `moves_longs_with_indirect_calls`: Moves a pair of longs from structure a1 into d1 with immediate constants in d0, passes a pointer from offset 140 of a2 into a4, calls through a5, and ends with an unconditional branch to the shared tail.
- `loads_table_entry_and_calls`: Loads a long from offset 200 of a1 into d1, sets d0 to a constant, loads a pointer into a4 from a table indexed by the long at offset 196 of a2, and calls through a5, then branches to the shared tail.
- `indexes_table_and_calls_through`: Scales a long from offset 200 of a1 by four, reads a table entry from a0 at that index, passes it with a constant in d0 to a call through a5, then branches to the shared tail.
- `copies_long_then_branches_out`: Copies a long from offset 200 of the structure in a1 into d1 and an immediate value into d0, calls through a5 with a pointer loaded from offset 196 of a2, and branches to the shared tail.
- `reads_indexed_table_then_calls`: Loads a long from offset 200 of a1, shifts it left by two, reads a longword from a0 at that index plus offset 20 into d1, adds a fixed offset to a4, sets d2 to minus one, calls through a5, and branches to the shared tail.
- `loads_record_field_then_calls_out`: Reads a longword at offset 200 of a1, shifts it left by two, and fetches a longword from a0 at that index plus offset 20 into d1, then stores d1 at offset 204 of a1 and copies the value at offset 200 into offset 220.
- `sets_register_then_calls_through`: Sets d2 to minus two, loads a long from offset 204 of a1 into d1, adds a fixed offset to a4, calls through a5, copies a long from offset 620 of a2 into d3, copies d1 into d2, and branches to the shared tail after a second indirect call.
- `branches_to_shared_tail_4`: Contains a single unconditional word branch that transfers execution to a shared location elsewhere in the code, with no data movement of its own.
- `branches_to_shared_tail_variant`: Performs a sequence of data movement and then transfers execution to a shared location elsewhere in the code through an unconditional branch.
- `loads_parameters_and_calls_twice`: Loads a longword from the structure in a2 and copies a longword from the structure in a1 into d1 and then into d2, sets d0 to 232 and 216 in two passes, and calls through the register a5 twice, once with a4 set from a fixed offset and once with a4 taken from the structure in a2.
- `frag_move_lea_jsr_bra`: Short code fragment that copies the memory at an offset from a1 to d1, then copies 216 to d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `sets_filler_and_calls_indirect`: Sets d2 to all ones, copies a longword from the structure in a1 into d1, sets d0 to 216, loads a4 from the structure in a2 at offset 196, and calls through the register a5.
- `sets_constants_and_calls_indirect`: Sets d3 to 209, clears d2, copies a longword from the structure in a1 into d1, sets d0 to 216, loads a4 from the structure in a2 at offset 196, and calls through the register a5.
- `dispatches_indirect_routines_with`: Loads long words from offsets 196 and 200 of a1 into d2 and d1, sets d3 and d0 to constants, and branches into shared code elsewhere.
- `dispatches_through_shared_tail`: Loads a longword from offset 200 of the structure in a1 into d1 and sets d0 to 232, then calls through a5 with a4 offset by 1832.
- `calls_indirect_routine_twice`: Loads a longword from offset 200 of the structure in a1 into d1 and an offset-dependent value into d0, then calls the routine pointed to by a5 twice.
- `dispatches_record_fields_through_pointer`: Loads longwords from the structures in a1 and a2 at fixed offsets, copies them into d1 and d2, sets d0 to constants 232 and 216, and calls through a5 with pointers taken from the structure in a2, with the call pattern repeated for several field pairs.
- `frag_bra`: Short code fragment that jumps to another point in the code.

### F-18 Interceptor

- `relocate_rom_tag_and_free_memory`: Saves all registers, calls a helper to locate a ROM tag, frees memory via exec.FreeMem, then swaps a matching resident structure pointer with the tag and clears the original.
- `scan_rom_tag_chain_for_entry`: Uses d6 as a countdown to walk the ROM tag list via a1, doubling each pointer, then returns a1 advanced past the found entry.
- `decode_bitstream_data`: Decodes a bitstream from a buffer pointed to by a0, writing decoded bytes to a destination buffer pointed to by a3.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `decode_bitstream`: Decodes a bitstream from a buffer pointed to by a0, writing decoded bytes to a destination buffer pointed to by a3.
- `frag_or_ori_move_movem_bra`: Short code fragment that merges d0 into d0, then merges 109 into d0, then copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### dk

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### is

- `keymap_data_table`: These four instructions are actually keymap data bytes, not executable code; the ori and subi words encode table entries for the is keymap.

### d

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### e

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### f

- `keymap_f_data_table`: The bytes are keymap data for the F key file, not executable code; the decoded instructions are meaningless immediate operations on d0 and a6.

### cdn

- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### usa0

- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### i

- `keymap_i_data_table`: The bytes are keymap data for the I key file, not executable code; the decoded instructions are meaningless immediate operations on d0 and a6.

### usa2

- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### n

- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### s

- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### gb

- `keymap_gb_data_table`: The bytes are keymap data for the GB key file, not executable code; the decoded instructions are meaningless immediate operations on d0 and a6.

### icon.library

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `increment_open_count_clear_flag`: Increments the 16-bit open counter at offset 32 of the library base in a6, clears bit 3 of the flags byte at offset 14, and returns the library base in d0.
- `decrement_open_count_maybe_close`: Returns zero in d0, decrements the 16-bit open counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of the flags byte set, calls the close routine.
- `close_library_if_unused`: Checks the 16-bit counter at offset 32 of the a6 library base; if zero, calls the library's close vector via a6 and unlinks the node, otherwise sets bit 3 of the flags byte and returns zero.
- `set_icon_library_open_flag`: Sets bit 3 of the byte at offset 14 from the library base in a6, then returns zero in d0.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `icon_open_allocate_and_store_base`: Saves a0 and a6 into the new icon base in a2, calls the library open vector with a name, stores the result, and alerts on failure.
- `icon_open_copy_base_and_result`: Copies the saved library base and open result from offsets 34 and 38 of a2 into globals, returns a2 in d0 and pops a2.
- `push_a0_a6_call_library_vector`: Pushes a0 then a6 onto the stack, calls the routine at address zero, discards the two saved registers by adding eight to a7, and returns.
- `icon_lib_call_aa_save_a0_a1_a6`: Saves a0, a1 and a6 on the stack, calls the library routine at offset aa, then discards the saved registers and returns.
- `icon_lib_call_124_save_a0_a2_a6`: Saves a0, a1, a2 and a6 on the stack, calls the library routine at offset 124, then discards the saved registers and returns.
- `icon_lib_call_3fe_save_a0_a1_a6`: Saves a0, a1 and a6 on the stack, calls the library routine at offset 3fe, then discards the saved registers and returns.
- `icon_lib_push_a0_a6_call_helper`: Pushes a0 and a6 onto the stack, calls the library helper at offset 666, then discards both saved registers and returns.
- `icon_lib_push_a0_a6_call_second_helper`: Pushes a0 and a6 onto the stack, calls the library helper at offset 698, then removes the two saved registers and returns.
- `icon_lib_push_a6_call_third_helper`: Pushes a6 onto the stack, calls the library helper at offset 6b2, then discards the saved register and returns.
- `icon_lib_save_regs_call_72e`: Pushes a0 through a2 and a6 onto the stack, calls the library routine at offset 1838, then discards the saved registers and returns.
- `icon_lib_call_782_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 1922, adds eight to the stack pointer and returns.
- `icon_lib_call_820_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 2080, adds eight to the stack pointer and returns.
- `icon_library_call_842_with_a0_a1`: Saves registers a0 and a1 on the stack, calls the library routine at offset 842, then discards the saved registers and returns.
- `icon_library_call_88a_with_a0_a1`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 88a, cleans the stack by adding eight and returns.
- `icon_library_call_8fe_with_a0_a1`: Stores a0 and a1 on the stack, jumps to the library routine at offset 8fe, restores the stack pointer and returns to its caller.
- `set_icon_library_field_at_94`: Calls the library's open routine through a6, stores the second argument into offset 148 of the returned structure, and returns.
- `load_icon_from_file_into_structure`: Opens a file, reads icon data into a stack buffer, copies 44 bytes and several fields into the icon structure, and links it via offset 424.
- `copy_icon_params_to_new_node`: Copies 44 bytes from the library base offset into a new icon node, then stores several longword fields and links the node into its parent at offset 424.
- `return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, releases the a5 frame and returns.
- `copy_icon_to_stack_and_dispatch`: Copies 44 bytes and several fields from the icon node into a stack buffer, then calls the drawing routine with the two arguments and the buffer.
- `icon_open_validate_and_load`: Library entry that builds a stack buffer, calls three helper routines to open and validate an icon file, then branches to the load path or returns zero on failure.
- `icon_load_header_and_allocate`: Reads the icon header through a4, checks the magic word and version, clears the a3 fields, allocates the image buffer and loads the two bitplane pointers.
- `icon_library_open_icon_images`: Library code that calls the open routine for two icon images, stores the results in the icon structure, then opens and links further image and mask resources, returning success or failure.
- `icon_library_load_four_icons`: Calls library hooks to allocate and fill four icon entries at a3, storing each result, then optionally loads a fifth into offset 70 of a2 and returns success in d0.
- `set_d6_zero_and_branch`: Loads zero into d6 and branches to a nearby continuation, marking failure for the icon loading routine.
- `restore_registers_and_return`: Pops d2-d6 and a2-a5 from the stack, releases the a6 frame and returns to the caller.
- `icon_open_alloc_and_parse`: Library routine that allocates a 260-byte buffer, opens the icon info structure, parses its fields and returns zero on success or failure.
- `icon_parse_info_after_open`: Library routine that saves the icon info pointer, opens the info structure, checks flags and parses name, tool types and default tool fields.
- `icon_parse_tooltypes_and_default`: Library routine that parses the icon's tool type list and default tool string, writing lengths and strings through the caller's callback.
- `scan_icon_list_and_redraw`: Advances a4 by four bytes and increments the counter at -8(a6) until a zero longword is found, then calls the callback in a3 and redraws the icon list.
- `measure_and_draw_icon_entries`: Loops over icon entries, measures each string, draws it via the a3 callback, then handles the extra entry at offset 70(a2) and restores registers.
- `draw_icon_text_at_offset`: Loads a string pointer from the stack, adds two to it, calls the text measuring routine, and if non-zero draws it via the drawing routine.
- `pop_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns to the caller, restoring the saved register pair.
- `forward_call_with_offset_8c`: Saves d2, loads the first argument and a pointer from the stack, then calls another library routine passing the pointer plus 140 bytes and the argument, then restores d2 and returns.
- `allocate_and_initialize_icon_object`: Saves d2-d4 and a2, allocates 65536 bytes of memory, initializes fields at offsets 140 and 142 of the new block, and frees the block if initialization fails, returning the block pointer or zero.
- `open_icon_by_name_via_library`: Pushes a8, a2, an offset from a2 and d2 onto the stack, calls the library open routine at offset 1838, cleans up 16 bytes, and returns a2 in d0 if the call succeeded, otherwise branches to the failure path.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack, and returns to the caller.
- `decrement_icon_refcount_and_store_data`: Saves d2-d3/a2, decrements the word count at a2, and if it reaches zero calls the library routine with a2 and d3; on success stores two longwords at offsets indexed by the count in the structure pointed to by a2.
- `decrement_icon_free_slot_store_pair`: Decrements the word count at (a2), then stores d2 and the caller's stack argument into the two longwords of the indexed slot in the structure pointed to by 10(a2), returning 1.
- `allocate_and_init_icon_structure`: Allocates 94 bytes of memory, initializes the new icon structure, links it with the caller's two arguments, and returns its pointer or zero on failure.
- `return_allocated_pointer_in_d0`: Moves the allocated structure pointer from d6 into d0 and branches forward to the common exit path.
- `icon_lib_return_zero_after_jsr`: Pushes a small immediate and d6 onto the stack, calls a library routine through a jump table, then returns zero in d0 after restoring d2-d6 and a2.
- `icon_lib_call_with_offset_4e`: Saves d2-d4, loads two arguments from the stack, adds 78 to the second argument, pushes both and calls a library routine before restoring registers.
- `icon_lib_find_equals_in_name_list`: Iterates a null-terminated pointer list, calls a string compare routine for each entry, and returns a pointer just past the first '=' character found in the matching name.
- `return_zero_pop_d2_d3_a2_a3`: Sets d0 to zero as the return value, restores d2, d3, a2 and a3 from the stack, then returns to the caller.
- `find_matching_string_in_list`: Saves d2-d6, calls a length routine on the second argument, then scans the first argument list for an entry whose length matches and whose comparison succeeds, returning one on a match and zero otherwise.
- `verify_next_list_entry_match`: Calls a length routine on d2, compares the result with d5, and if equal calls a comparison routine with d5, d3 and d2, returning one on success and continuing otherwise.
- `store_d4_plus_one_to_d0_d2`: Tests d4; if nonzero, copies d4 to d0, increments d0 by one, copies d0 to d2, then branches to shared exit code.
- `zero_d2_then_branch_to_exit`: Sets d2 to zero with a quick move and branches to the shared exit code, used when d4 was zero.
- `return_zero_restore_d2_d6`: Sets d0 to zero as the return value, restores registers d2 through d6 from the stack and returns to the caller.
- `parse_icon_header_and_append_suffix`: Copies a 30-byte header from the second argument into the first, clears the last byte, then parses a numeric suffix and appends it, returning the first argument.
- `append_three_byte_suffix_and_test`: Advances a2 by five, calls the string append routine with a three-byte suffix, tests the result, and on failure sets d5 to one and advances a2 by three.
- `set_d5_zero_and_branch`: Sets d5 to zero and branches forward into the shared continuation code, marking a successful suffix parse.
- `parse_decimal_string_to_long_2`: Walks the character string at a2, converting each digit via the digit routine and accumulating a base-ten value in d5, then formats and stores the result through library calls before returning it in d0.
- `convert_ascii_digit_to_value`: Loads the character from the stack argument into d1, checks it lies between the ASCII codes for zero and nine, and returns its numeric value in d0 or minus one otherwise.
- `return_minus_one_constant`: Loads minus one into d0 and returns immediately, serving as the failure result for the digit conversion.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `icon_alloc_and_init_pair`: Saves d2-d5, reads four stack arguments, allocates a structure via library call, then initializes it with a second call and frees it on failure, returning the handle or zero.
- `icon_alloc_and_set_dimensions`: Saves d2-d3/a2, allocates a 96-byte block, stores the value 10 at offset 14 and at the caller's word pointer, then initializes the second word and returns success flag.
- `icon_alloc_failure_return_zero`: Loads zero into d0 as the failure return value, restores d2-d3 and a2 from the stack, and returns to the caller.
- `icon_lookup_verify_result`: Saves d2-d3, loads three arguments from the stack, calls the icon lookup routine, then returns the result only if it equals the fourth argument, otherwise zero.
- `return_lookup_result_in_d0`: Moves d1 into d0 as the return value, restores d2-d3 from the stack and returns to the caller.
- `icon_lookup_verify_second`: Saves d2-d3, loads three stack arguments, calls the second icon lookup routine, and returns its result only when it matches the fourth argument, else zero.
- `return_d1_in_d0_restore_d2_d3`: Copies the longword in d1 to d0 as the return value, then pops d2 and d3 from the stack and returns to the caller.
- `unsigned_32bit_divide_d1_by_d2`: Divides the longword in d1 by d2, using a shift-and-subtract loop when d2 exceeds 65535 and word divisions otherwise, returning quotient in d0 and remainder in d1.
- `shift_subtract_divide_d1_by_d2`: Divides the longword in d1 by d2 with a sixteen-step shift-and-subtract loop, leaving the quotient in d1 and remainder in d0.
- `mul32x32_high_low_to_64bit`: Multiplies the 32-bit values in d0 and d1 using 16-bit partial products, combines them into a 64-bit result in d0, and restores d2 from the stack.
- `shifts_and_forwards_three_arguments`: Saves d2, moves d1 into d2, d0 into d1, calls the math operations routine, then restores d2 and returns, which is library code in the icon library.
- `forwards_arguments_and_returns_d1`: Saves d2, shifts d0 and d1 into d1 and d2, calls the math operations routine, copies d1 into d0, restores d2 and returns, as library code in the icon library.
- `dispatches_math_operations_in_library`: Calls the math operations routine as part of the icon library's internal arithmetic handling, using the register arguments it is given.
- `performs_library_math_via_call`: Calls the math operations routine from within the icon library, passing through the register arguments and processing the returned values in the library code.
- `format_string_via_rawdofmt_2`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, calls exec RawDoFmt through the library base at absolute address 4, then restores registers and returns.
- `store_byte_to_a3_and_advance`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3 by one, then returns to the caller.
- `strlen_bounded_255`: Loads the string pointer from the stack, scans up to 256 bytes for a zero terminator, and returns the complemented count in d0 as the string length.
- `strchr_find_char_or_null`: Reads a string pointer and a character from the stack, scans bytes until the character matches or a zero byte ends the string, returning the match address or zero in d0.
- `return_zero_string_result`: Sets d0 to zero and branches into the shared exit path of the string search routine, returning a null result to the caller.
- `copy_string_until_nul_2`: Loads two pointers from the stack, copies bytes from the source to the destination incrementing both, and returns after copying the terminating null byte.
- `skip_string_to_terminator`: Loads a pointer from the stack, advances it past bytes until a null byte is found, steps back one, then branches to shared code.
- `compare_strings_up_to_length`: Loads two pointers and a length from the stack, compares bytes one by one, returns zero when equal through the null terminator or a mismatch is found.
- `return_one_or_minus_one_flag`: Loads 1 into d0, branches conditionally past the next instruction, otherwise loads -1 into d0, then returns to the caller.
- `scan_string_until_nul_or_count`: Loads a0 and a1 from the stack, loads a count from the stack into d0, tests bytes at a0 incrementing it, decrements d0 and branches based on the result.
- `decrement_a0_call_helper_return`: Subtracts one from a0, calls a nearby library routine, then returns to the caller.
- `copy_bytes_or_pad_with_zero`: Copies bytes from the address in a1 to the address in a0 until a zero byte is copied or d0 bytes are exhausted, then pads the remainder with zero bytes.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_self_referential_list_header`: Takes a pointer argument from the stack, stores its own address plus four at the start, clears the next field and points the third field back at itself.
- `draw_icon_with_scaled_offsets`: Saves registers, checks the icon pointer, calls a rectangle fill and two scaling routines, then draws the icon data using computed offsets and returns a status.
- `walk_icon_list_validate_entries`: Saves d2-d5/a2, reads three arguments from the stack, then walks a linked list of icon nodes calling a validation routine on each node and its data, returning 1 on success or 0 on failure.
- `return_success_flag_one`: Loads the value 1 into d0 and branches into the shared epilogue that restores registers and returns, signalling success to the caller.
- `return_failure_flag_zero`: Sets d0 to zero as the failure return value, restores d2-d5 and a2 from the stack, and returns to the caller.
- `load_icon_image_and_mask`: Allocates two memory blocks via library calls, computes a rounded even byte width from the icon header, scales two dimensions, stores the image pointer at offset 10 of the first block and returns that block or minus one.
- `return_minus_one_restore_registers`: Sets d0 to minus one as the failure return value, pops d2 through d6 and a2 from the stack and returns to the caller.
- `load_icon_list_chain`: Allocates paired memory blocks for each icon in a list, links them through offset 12, computes a size from a byte at offset 7, stores the image pointer at offset 8 and returns the first block or minus one.
- `icon_alloc_and_link_node`: Calls library allocation routines to create a node, stores the returned pointer at offset 8, copies a stack argument to offset 12, and follows the next pointer until a null link is found.
- `return_a3_as_d0_then_branch`: Moves the a3 register into d0 as the return value and branches forward into the shared epilogue that restores registers.
- `return_minus_one_restore_registers_2`: Sets d0 to minus one as an error return, restores d2 through d7 and a2 through a3 from the stack, then returns to the caller.

### info.library

- `illegal_instruction_trap_stub`: Executes an illegal instruction word, which triggers the 68000 illegal instruction exception; it is a trap stub, not a returning routine.
- `store_library_base_and_data_pointer`: Saves a6 to absolute address 184 and d0 to 188, copies d0 into a1, stores a0 at offset 34 of that pointer and at absolute 192, then returns.
- `call_absolute_zero_with_a6_saved`: Pushes a6 onto the stack, calls the routine at absolute address zero, discards the pushed longword by adding four to a7, then returns.
- `info_library_call_via_a6_offset`: Pushes a6 onto the stack, calls the library routine at offset 120 through a6, then discards the pushed longword and returns.
- `info_library_call_via_a6_offset_18`: Pushes a6 onto the stack, calls the library routine at offset 18 through a6, then discards the pushed longword; it falls through without returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `info_library_call_with_a1_and_a6`: Pushes a1 and a6 onto the stack, calls the library routine at offset 1416 through a6, then discards both longwords and returns.
- `clear_bit3_and_increment_count`: Loads the caller's argument pointer from the stack, clears bit 3 of the byte at offset 14, increments the word at offset 32, and returns the pointer in d0.
- `check_count_and_set_flag`: Saves d2 and a2, loads the argument pointer, calls a subroutine, and if the word at offset 32 is nonzero sets bit 3 of the byte at offset 14 before calling another subroutine and returning zero in d0.
- `save_d2_call_clear_and_restore`: Saves d2 and a2, calls a subroutine, computes offsets from the words at offsets 16 and 18 of a2, calls two more subroutines with those arguments, then restores d2 and a2 and returns the saved d2 value in d0.
- `decrement_library_open_count`: Loads the library base from the stack, decrements its open count word, and if it reaches zero with the expunge flag set, calls the expunge routine and returns.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `close_library_and_free_resources`: Saves d2/a2, then conditionally calls cleanup routines for the passed pointers and several global resource pointers, releasing each non-null one before restoring registers and returning.
- `allocate_and_init_library_base`: Saves a2-a4, allocates a 812-byte memory block via a library call, stores the pointer in a global, and returns zero if allocation failed.
- `init_info_library_vectors_and_tables`: Copies the caller's structure pointer into a4, stores pointers to offsets within that structure into library globals, calls a copy routine twice, then copies ten-longword tables into those structures and returns 1.
- `open_dos_and_icon_libraries`: Saves a2-a3, calls the open-library routine for dos.library storing the base in a global, and on failure pushes error code 3 before branching to the error path.
- `open_icon_library_store_base`: Pushes a longword zero and the library name, calls the open-library routine via a3, stores the returned base in a global, cleans the stack and branches on failure.
- `open_intuition_library_store_base`: Pushes a longword and the library name pointer, calls the open-library vector in a3, stores the returned base at offset 72 in the library structure, then cleans the stack and branches on the result.
- `open_graphics_library_store_base`: Pushes a longword and the name pointer at offset 44 of d4, calls the open-library vector in a3, stores the returned base at offset 76 of d4, and on failure calls the vector in a2 with arguments 5, 0, 0.
- `return_one_restore_a2_a3`: Sets d0 to 1 as the return value, restores registers a2 and a3 from the stack, and returns to the caller.
- `check_library_open_flag`: Pushes a2 onto the stack, loads the absolute longword at address 88 into a1, tests that longword and branches away if it is nonzero, otherwise branches to another routine.
- `check_device_open_count_seven`: Loads the longword pointed to by a1 into a0, compares the byte at offset 61 of that structure with 7 and branches away if unequal, otherwise branches elsewhere.
- `fetch_structure_field_a4_or_zero`: Loads the pointer at a1 into a0, reads the longword at offset 56 into a2, then tests the longword at offset 164 of the structure and returns it in d0 if nonzero, else branches away.
- `get_node_field_via_owner`: Loads the pointer at (a1) into a0, checks the byte at offset 61 of that structure, and if it equals one, follows the pointer at offset 76 and returns the longword at offset 442 in d0.
- `copy_a2_or_follow_offset_76`: Moves a2 into d0; if a2 is zero it branches away, otherwise it loads the pointer at offset 76 of a2 into a0 and branches onward.
- `return_zero_and_pop_a2`: Sets d0 to zero as the return value, pops a2 from the stack and returns to the caller.
- `open_info_disk_and_store_result`: Sets up a stack frame, calls a routine to open the disk.info file, stores the returned handle at offset 152 of the allocated structure, then checks a flag byte and branches to pass a filename or a stored pointer.
- `read_info_file_into_buffer`: Copies a filename into a local buffer, calls the file open and read routines, clears a global word, and if reading succeeded and a flag is set, loads a value from the opened file structure into d0.
- `allocate_info_buffers_and_load`: Allocates two memory blocks, optionally reads the info file into them, computes a size from a stored value, allocates a third buffer, copies data into it and returns success or failure.
- `open_info_library_file`: Pushes a 65536-byte size, a pointer to address 260, and d2 onto the stack, calls the library open routine, stores the returned handle in (a4), then cleans the stack and branches based on the result.
- `check_info_library_open_flag`: Loads d4 into a0, dereferences it into a2, tests byte at offset 61 of a2, and if set pushes address 96 and d3, calls the library routine, cleans the stack and branches on the result.
- `close_info_library_handle`: Pushes the handle stored at (a4) and d3 onto the stack, calls the library close routine, cleans the stack, and branches depending on the returned status.
- `compute_info_buffer_size_and_allocate`: Calls library helpers to read a size from the d4 structure, clamps it to at least 256, allocates a buffer via the memory helper and stores the pointer at offset 96, returning nonzero on success.
- `finish_info_setup_and_return_one`: Calls a helper with the buffer at offset 96, copies fields from the d4 and a4 structures into offsets 104 and 112, restores registers, and returns 1.
- `initialize_info_library_state`: Clears several library globals, stores the second argument, calls the setup routine, then opens resources and fills the global state, returning early on failure.
- `check_library_open_result`: Pushes a pointer to a local variable, calls a library function, tests the returned d0, discards the argument, and on failure pushes a4 and two arguments before branching to shared error code.
- `allocate_library_structure`: Calls a library function, stores the returned d0 in a4, compares a4 against zero, and branches to the failure path when the allocation returned null.
- `allocate_buffer_and_store_result`: Pushes a size argument, calls a library allocation function, stores the returned d0 at the address in a3, discards the argument, and on failure pushes a4 and two arguments before branching to shared error code.
- `check_info_library_entry_and_dispatch`: Pushes a3 and a4 field 50, calls a library check, and if it succeeds calls the a4 setup routine and a6-relative routine, then tests the global flag and branches to the a2 callback with 1 or 0.
- `call_pc_relative_and_test_d0`: Calls the routine at a program-counter-relative offset, moves the returned d0 into d2, and branches based on whether d0 is nonzero.
- `push_d2_call_test_and_invoke_a2`: Saves d2, calls a pc-relative routine with an a6-relative argument, and if it returns nonzero pushes a4 and arguments then calls through a2 and cleans the stack.
- `run_info_library_cleanup_sequence`: Tests the word flag at the library global, optionally calls a d2-saving routine, then calls four a4 routines and finally invokes the a2 callback with zero and an a6-relative pointer before restoring registers.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `info_library_open_build_request`: Builds a request structure on the stack with size, version and pointer fields, calls the open routine, and on success passes the handle and a2 to a second routine before returning the handle in d0.
- `info_library_dispatch_by_offset`: Loads a2 and d1 from the stack, reads a word from the structure at offset 38, and jumps through a table indexed by that word if it is at most 16.
- `return_one_pop_a2_a3`: Sets d0 to 1 as the return value, pops a2 and a3 from the stack, and returns to the caller.
- `dispatch_io_request_by_command_word`: Saves a2, loads the library base and request pointers, reads the command word at offset 38, and jumps through a table for commands 6 to 9.
- `store_d1_to_library_offset_and_call`: Loads the library base from absolute address 100, stores d1 at offset 112, pushes the argument at offset 44 of a1, calls a subroutine, then pops a2 and returns.
- `info_library_open_and_validate`: Sets up a stack frame, calls library open and message routines with the passed info structure, copies a field when the state byte is 3 or 4, then validates and returns a status.
- `info_library_reopen_and_parse`: Calls the open routine on the info structure, allocates a new handle, stores it in the structure, formats a name and parses it, returning a status.
- `info_library_check_state_and_push_disk`: Loads the info structure pointer, compares its state byte at offset 61 with 1, and if not equal pushes the value 30 before branching onward.
- `compare_info_entries_via_d2_and_library`: Loads the structure pointed to by a3, pushes its field at offset 64 and a stack buffer, calls the function in d2, then compares the structure against the buffer via a library call and branches on the result.
- `check_entry_flag_then_compare_field`: Loads the structure from a3, tests byte at offset 61 for one, and if not set pushes a global long and the structure's offset-64 field before calling a library comparison and branching on d0.
- `compare_a4_field_with_a3_field`: Loads the structure from a4, pushes its offset-112 field, then loads the structure from a3 and pushes its offset-64 field before calling a comparison routine and branching on d0.
- `check_two_library_vectors`: Pushes two string pointers and two object fields onto the stack, calls two library routines through a3 and a4, tests the second result and returns zero or one in d0.
- `return_one_restore_registers`: Sets d0 to one as the return value, restores d2, d3 and a2 through a4 from the a6 frame, releases the frame and returns.
- `compare_call_result_to_minus_one`: Saves d2, loads the argument from the stack, calls a library vector with minus one and the argument, then returns zero in d1 if the result equals minus one, else leaves d1 as minus one.
- `return_one_after_library_call`: Saves d2, calls a library routine, sets d1 to 1, discards the pushed argument, moves d1 into d0 as the return value and restores d2.
- `draw_info_library_panel_graphics`: Saves a2-a3, loads the library base from the stack, then repeatedly pushes coordinate and color words and calls the draw and text routines to render the info panel.
- `draw_info_panel_corner_boxes`: Saves d2/a2, loads the library base, then pushes four sets of coordinate words and calls the draw routine to render corner boxes of the info panel.
- `draw_seven_info_rectangles`: Library routine saves d2/a2, loads the caller's argument into d2, then calls the drawing routine seven times with different coordinate and color words pushed on the stack, finally releasing the stack and returning.
- `draw_nine_info_rectangles`: Library routine saves d2/a2, loads the caller's argument into d2, then calls the drawing routine nine times with different coordinate and color words pushed on the stack, finally releasing the stack and returning.
- `draw_nine_info_rectangles_alt`: Library routine saves d2/a2, loads the caller's argument into d2, then calls the drawing routine nine times with a second set of coordinate and color words pushed on the stack, finally releasing the stack and returning.
- `check_library_open_flag_2`: Saves a2-a3, loads the caller argument into a2 and a constant into a3, then reads the library base from absolute address 88 and tests byte 61 against 4, branching to a shared tail with error code 131 if not equal.
- `compare_node_and_report_error`: Loads error code 154 into d1, compares the node pointer at offset 234 with its own first field, and if unequal pushes many arguments and calls the error reporter before jumping to the shared exit.
- `report_error_and_release_node`: Copies d1 to d0, adds 8, pushes arguments and calls the error formatter, then calls the release routine with the node at offset 62 of a2, restores a2-a3 and returns.
- `allocate_info_handle_and_build_list`: Saves d2/a2-a4, calls the allocator with size 65536 and type 14, stores the returned handle at offset 234 of the library base, then returns zero or the handle.
- `link_info_handle_into_list`: Uses a3 to reach the library base, writes the handle's self-pointer and back-pointer fields, clears offset 242, and branches to the list walker if the global list at offset 92 is non-empty.
- `allocate_info_handle_via_allocator`: Pushes size 65536, type 14 and d2, calls the allocator, moves the result to a2, cleans the stack and branches depending on whether the handle is null.
- `info_lib_scan_and_install_entries`: Calls library routines to allocate a node, stores the result at offset 10 of a2, then walks the a4 pointer list calling install routines until a null entry, finally copying the head node pointer into offset 238 of a2.
- `info_lib_advance_or_flush_node`: Compares the current node pointer with the saved one and its next field; if equal it calls cleanup routines with fields at offsets 46 and 62 of a2, otherwise it advances the saved node pointer and calls the node handler.
- `info_lib_remove_current_node`: Calls a routine with the node's offset 10 field and a global, calls another with a global and a2, then unlinks the current node by copying its next pointer into offset 238 of a1 and calls the local routine.
- `info_lib_advance_node_or_flush`: Pops a2-a3, loads the caller's argument and the library base, then either flushes the current node via three library calls or advances the node pointer and calls the node handler.
- `info_lib_pop_node_and_advance`: Pushes the library globals and the current node's field, calls the node handler and a library routine, then advances the node pointer and calls the local dispatcher before restoring a2-a3.
- `info_lib_insert_new_node`: Saves d2-d3/a2-a3, sets a flag in the library base, optionally flushes the current node, allocates two nodes, links them, installs the new node and calls the local dispatcher.
- `info_lib_advance_list_and_notify`: Saves d2/a2-a3, sets field 242 of the library base to 1, then compares list pointers at offsets 234 and 238 and calls library routines to advance and notify, returning a value in d2.
- `info_lib_relink_list_after_removal`: Loads the list node at offset 238, calls a library routine to unlink it, then if the list at 234 is exhausted copies its head into 238 and calls the local routine at offset zero.
- `info_lib_teardown_and_release`: Calls library routines to release the object at offset 46, then invokes the routine at offset 62 with a null and the a2 pointer, restores the stack and pops d2/a2-a3 before returning.
- `check_info_list_empty_branch`: Saves registers d2-d5 and a2-a5, loads a4 with the library base, d2 with a size, reads the list head into a2 and tests the longword at offset 242; branches away if nonzero.
- `compare_list_head_and_copy_entries`: Reads the list head via a4, compares its first node pointer with the node plus four, and if unequal pushes a global and a node field then calls a subroutine before branching on.
- `clear_list_pointer_and_branch`: Moves d2 into a0, loads the pointer at a0 into a2, clears the longword at offset 92 of a2, then branches to another routine.
- `load_info_list_head_and_branch`: Sets d4 to 1, loads the pointer at a4 into a2, loads the longword at offset 234 of that structure into a0, then branches to shared library code.
- `count_info_entries_and_allocate_table`: Walks the linked list in d3 counting entries into d4, allocates d4 times four bytes, stores the pointer at offset 92 of the a2 structure, and returns success or zero.
- `free_info_table_and_reload_list`: Pushes d4, frees the table at offset 92 of the a2 structure along with the structure at offset 140, clears d4, reloads the list head from a4 and branches on.
- `allocate_info_buffer_and_copy_node`: Calls library allocator with size from node offset 10, stores returned pointer in table indexed by d4, then copies node data into it and advances to the next node.
- `copy_info_node_and_advance_list`: Copies node data into the allocated buffer at table index d4, increments d4, walks to the next list node and returns one when the list ends.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_info_display_by_mode`: Reads a mode byte from library global data, bounds-checks it between one and six, then jumps through a table to draw the matching info screen layout.
- `setup_info_parameters`: Prepares multiple parameter values on the stack for various function calls, performs several arithmetic operations, and then branches to different locations based on comparisons within the same library.
- `frag_moveq_cmp_bne_bra`: Short code fragment that loads 81 into d0, then compares d0 with d1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_moveq_cmp_bne_bra_2`: Short code fragment that loads 82 into d0, then compares d0 with d1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_movea_bra`: Short code fragment that copies 96 to a0, then jumps to another point in the code.
- `frag_movea_bra_2`: Short code fragment that copies 106 to a0, then jumps to another point in the code.
- `frag_movea_bra_3`: Short code fragment that copies 118 to a0, then jumps to another point in the code.
- `frag_movea_move_pea_bra`: Short code fragment that copies 130 to a0, then copies a0 to the stack, then pushes the address of a fixed memory address onto the stack (2 times in a row), then jumps to another point in the code.
- `register_info_entries`: Prepares multiple parameter values on the stack, calls several routines including one that registers information table entries, and then branches to another routine in the same library.
- `initialize_info_entries`: Initializes various information table entries by calling multiple routines with different parameters, then cleans up the stack and branches to a final location.
- `setup_info_tables`: Configures information tables and palette entries by calling several initialization routines with varying parameters, then performs stack cleanup and branches to a final location.
- `prepare_and_call`: Pushes two word values onto the stack, moves a register value onto the stack, calls a routine through a register, adjusts the stack pointer, and then branches to a specific location.
- `call_address_in_a3`: Pushes a word value onto the stack, saves registers d2 through a5, calls a routine whose address is in a3, restores saved registers, then returns.
- `allocate_four_rasters_and_dispatch`: Calls the allocator four times to obtain bitmaps for two 44-byte and two 36-byte rasters, stores the results in library globals, and returns zero if any allocation failed.
- `allocate_second_raster_set`: Calls the allocator four more times for 44-byte and 116-byte rasters, stores results in globals and a4, and returns zero if any allocation failed.
- `dispatch_info_library_state_index`: Loads the library base from absolute address 88, reads byte at offset 61 into d0, and jumps through a word table indexed by that value when it is between 1 and 5.
- `restore_registers_and_unlink_frame`: Restores d2, d3 and a2 through a4 from the stack frame at offset -40 from a6, releases the a6 frame and returns to the caller.
- `check_library_flag_and_clear_byte`: Saves d2, loads the caller argument from the stack, tests a longword flag in the library base, and if clear loads another base and clears a byte there.
- `check_info_library_availability`: Pushes a library field, a string and a global onto the stack, calls two library routines, and returns zero if the second call returns minus one, otherwise its result.
- `set_ports_and_check_four_devices`: If a status byte is not four, writes four word values into device structures, then calls a routine on each of four pointers and returns one only if all succeed.
- `check_second_device_pointer`: Pushes minus one, the device pointer at a global and d2, calls the routine in a2, and branches based on comparing its result with minus one.
- `check_negative_one_after_library_call`: Pushes -1 and the longword at absolute address 328 onto the stack, saves d2, calls the routine in a2, compares d0 with -1, cleans the stack and branches to the success path if equal.
- `call_a2_with_negative_one_and_flag`: Pushes minus one, the global at offset 332, and d2, calls the a2 callback, compares the result with minus one, cleans the stack, and returns zero in d1 on match or minus one otherwise.
- `return_one_and_restore_d2_a2`: Sets d1 and d0 to 1, restores d2 and a2 from the stack and returns to the caller.
- `poll_info_library_device_loop`: Saves d2-d4/a2, reads a byte from the library device structure, shifts it into d3, then repeatedly calls the device open and read routines until a read returns zero, returning zero.
- `return_zero_and_restore_registers_2`: Sets d0 to zero as the return value, restores d2-d4 and a2 from the stack, and returns to the caller.
- `compare_device_field_to_four`: Saves d2-d3/a2, loads a pointer and value from the stack, reads two fields from the pointed structure, compares one field against four and branches accordingly.
- `check_d1_against_64_then_branch`: Loads 64 into d0, compares d1 with it, and branches to one of two other code locations depending on whether they differ.
- `check_d1_against_512_then_branch`: Compares d1 with the immediate value 512 and branches to one of two other code locations depending on whether they differ.
- `save_d2_a2_call_a6_then_branch`: Pushes d2 and a2 onto the stack, calls the routine whose address is in a6, discards the two saved longs, then branches onward.
- `push_a2_call_d4_return_zero`: Pushes a2 onto the stack, calls the routine at offset d4, sets d0 to zero and branches onward without restoring a2.
- `call_be_and_d4_return_one`: Pushes d3, calls the routine at offset be, discards the argument, pushes a2, calls the routine at offset d4, sets d0 to one, restores d2-d3/a2 and returns.
- `info_library_cleanup_and_notify`: Saves a2, loads the caller argument, calls four library routines, checks a global flag byte, conditionally calls another, then posts a message and notification before restoring a2 and returning.
- `format_info_string_with_flags`: Saves d2, loads the caller's argument, reads a 16-bit field from the library base at offset 112, masks it with 15 and with 1, then pushes the masked values plus a format string and calls the formatting routine before restoring d2.
- `build_info_display_arguments`: Sets up a stack frame and local buffer, stores fixed byte and word constants, then selects two argument sets depending on whether the third parameter is zero and calls the display routine.
- `render_info_text_variants`: Formats a string into a local buffer, and when the mode byte equals three, pushes computed offsets and coordinates and calls the text drawing routines twice before restoring registers and returning.
- `return_stack_arg_in_d0_clear_d1_a1`: Loads the longword at offset four from the stack into d0, sets d1 to zero, copies d1 into a1, then branches away to another routine.
- `scan_string_from_a1_offset_by_d0`: Increments a1, forms a0 as a1 plus d0, tests the byte at a0 and repeats until a null byte, then returns a1 in d0.
- `copy_null_terminated_string`: Loads destination from stack offset four and source from offset eight, copies bytes from source to destination incrementing both pointers until a null byte is copied, then returns.
- `load_copy_args_and_branch_to_copy`: Loads source pointer from stack offset four into a1, destination pointer from offset eight into a0, byte count from offset twelve into d0, sets d1 to zero and branches into the copy loop.
- `copy_bytes_until_count_reached`: Copies bytes from a1 to a0 incrementing both pointers and d1, looping while d1 is less than d0, then returns to the caller.
- `load_two_pointers_and_branch_to_copy`: Loads a pointer from stack offset four into a0 and another from offset eight into a1, then branches into the shared byte copy code.
- `compare_strings_until_null_or_diff`: Advances a0 and a1 byte by byte, stopping at a null terminator or a differing byte, then returns in d0 whether the final bytes are equal.
- `strlen_count_bytes_until_null`: Reads the pointer argument from the stack, scans bytes from that address until a null byte, and returns the count including the terminator in d0.
- `format_and_emit_info_strings`: Saves d2-d4/a2, calls the local formatter, then passes the result and adjusted arguments to three library output routines using the object at offset 50 of a2.
- `info_library_format_string_call`: Builds a formatted string in a local buffer using a global format and the fifth argument, then calls the library formatting routine with four saved arguments and returns.
- `info_library_load_indexed_byte`: Saves d2-d3/a2, loads a byte from the address indexed by the first argument times four, sign-extends it into d2, sets d1 to one and branches onward.
- `info_library_copy_bytes_terminate`: Copies bytes from the a2-based source into the d0-based destination starting at index d1, increments d1 until it exceeds d2, writes a zero terminator, restores registers and returns.
- `init_info_window_and_open`: Builds two info structures from stack arguments, stores window and screen pointers, calls setup helpers, then opens the window via the library and returns success or failure in d1.
- `close_info_window_and_return_true`: Pushes zero and the window pointer from offset 62 of a3, calls the close routine, then returns 1 in d0 after restoring d2 and a2-a3.
- `draw_info_window_text_lines`: Calls three text output helpers on the window at offset 50 of a2, passing coordinates adjusted by subtracting 10, then releases the stack and returns.
- `raw_do_fmt_via_exec_vector`: Saves a2-a4 and a6, loads format string and argument pointer from the stack, loads the RawDoFmt vector from exec base at address 4, calls it through a6, then restores registers and returns.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3 and advances a3 by one byte, then returns to its caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_72_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.

### version.library

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `store_a6_and_a0_into_library_base`: Copies d0 into a1, then stores a6 at offset 34 and a0 at offset 38 of that structure, and returns.
- `increment_refcount_clear_flag_return_self`: Adds one to the 16-bit counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 42, then returns the base pointer in d0.
- `decrement_refcount_release_when_zero`: Returns zero in d0, decrements the 16-bit counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of offset 42 set, calls the release routine before returning.
- `open_library_set_busy_flag`: Saves d2 and a5-a6 on the stack, builds a frame, loads the library base from offset 34 into a6, and if the counter at offset 32 is nonzero sets bit 3 of offset 42 and returns zero in d0.
- `library_close_and_free_segment`: Saves the value at offset 38 of the library base, calls the library close and free-segment vectors through a6, then returns the saved value in d0 after restoring d2 and a5-a6.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.

### FA-18 Interceptor (1988)(Electronic Arts)[h J. Fabre][HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `fill_with_zeros`: Sets the data register to zero thirteen times consecutively, effectively filling a memory region with zero bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_task_context`: Saves processor registers and memory pointers to a global task context structure, then calls a routine through a function pointer located at offset 4 of another structure.
- `process_graphics_data_4`: Reads a global variable, calls another routine with it as argument, and if a condition is met, calls additional routines using offsets from the returned value.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.w) and then falls through into the code that follows, with no return of its own.
- `frag_jsr`: Short code fragment that calls another routine.
- `frag_lea_moveq_jsr_ori`: Short code fragment that loads the address of a fixed memory address into a1, then loads 0 into d0, then calls another routine, then merges 0 into d0.
- `frag_beq_rts`: Short code fragment that branches if the last result was zero, then returns.
- `process_graphics_data`: Processes graphics-related data by iterating through a loop that modifies values based on calculations, then performs a final graphics operation using accumulated results.
- `process_graphics_data_2`: Processes graphics data by reading dimensions from a structure, performing calculations on them, and storing results in memory locations.
- `process_graphics_data_3`: Processes graphics-related data by iterating through a loop that modifies values based on calculations, then performs a final graphics operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_collision_check`: Updates collision detection parameters by comparing values from a structure with input parameters and performing bitwise operations on byte values.
- `process_object_collision`: Calculates object movement deltas and performs collision checks using a loop that updates position variables based on input parameters, then returns zero when the loop counter reaches zero.
- `handle_object_movement`: Manages object movement by calculating position differences, performing collision checks, and updating object state variables in a loop until completion.
- `handle_positive_input`: Handles positive input values by making two function calls with specific parameters.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `initialize_structure_data`: Initializes a data structure with zero values at specific offsets, performs calculations on input parameters, and stores results in memory locations.
- `check_pointer_validity`: Checks if a pointer in memory is valid by testing the value at the address pointed to.
- `initialize_and_check_loop`: Initializes a local variable to zero and checks if a value at a specific offset is less than or equal to zero.
- `manage_loop_iterations`: Manages loop iterations by checking conditions, updating counters, and performing conditional jumps based on comparison results between memory values.
- `update_collision_data`: Increments a value at offset 12 of a structure and decrements another value at offset -12, then performs a collision check with specific parameters and returns a calculated result.
- `calculate_collision_offset`: Calculates a collision offset by adding a constant to a pointer value, performs a collision check with specific parameters, and returns a calculated result.
- `perform_collision_check`: Performs a collision check by comparing values from a structure with input parameters.
- `validate_parameter`: Calls another routine with a parameter, compares the returned value to a specific constant, and exits immediately if they do not match, otherwise continues execution.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_resources`: Initializes system resources by calling two functions with specific parameters.
- `initialize_sequence`: Sets up a series of sequential operations by calling a routine with pairs of parameters, starting with a value of 13 and incrementing by one for each call, while using string pointers from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `un_pack_row`: It copies bytes from a source buffer to a destination buffer while interpreting two run-length encodings.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_global_table_entries`: The routine reads two 16-bit values from a global variable table indexed by the parameter.
- `read_global_table_entry`: The routine reads a 16-bit value from a global variable table using an index derived from its parameter and stores the result back into a global variable.
- `update_interrupt_handler`: Reads a value from a table using a short integer index, calls a routine through a pointer in the table, and modifies a byte pointed to by a global variable.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movem_move_lea_movea_adda_jsr_ori`: Short code fragment that saves registers on the stack, then copies 5 to d0, then loads the address of a fixed memory address into a1, then copies a fixed memory address to a6, then copies a6 to a0, then adds an address to a0, then calls the routine whose address is in a0, then merges 0 into d0, then returns.
- `frag_moveq_move_jsr_subq_bge_rts`: Short code fragment that loads 3 into d0, then copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then subtracts 1 from d0, then branches if the comparison was greater or equal, then returns.
- `execute_routine_four_times`: Executes another routine four times using a decrementing loop counter.
- `frag_jsr_moveq_move_subq_bge_rts`: Short code fragment that calls another routine, then loads 3 into d0, then copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then subtracts 1 from d0, then branches if the comparison was greater or equal, then calls another routine, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_components`: Initializes game components by checking flags and calling functions to allocate memory, set up structures, and perform other initialization tasks.
- `allocate_and_examine_memory`: It allocates a block of memory, locks a range, reads a value at an offset, unlocks, frees the memory, and returns that value.
- `frag_link_tst_bne_ori`: Short code fragment that creates a stack frame in a6, then tests a fixed memory address, then branches if the last result was not zero, then merges 0 into d0, then merges 46 into d0.
- `allocate_and_initialize_memory`: Allocates memory blocks, locks ranges, reads values from offsets, unlocks and frees memory, then initializes various structure fields with specific constants.
- `copy_transform_matrix`: Copies transformation matrix data from one location to another, setting a specific bit in the process, and returns a success indicator.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_input_character`: Validates input character values by checking a global variable, performing conditional comparisons, and calling another routine with specific parameters.
- `process_input_data`: Checks input character values and calls a function with a global variable, then stores the result in a structure at offset 17 and sets multiple fields to zero.
- `update_object_state`: Tests a bit flag in a structure, conditionally calls another routine with data from the structure, then clears several fields in the structure and calls a function with a byte value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_buffer_from_source`: Iterates through a 2D grid of data, copying bytes from a source to a destination buffer while decrementing counters and calling a function for each element.
- `frag_link_tst_beq_ori`: Short code fragment that creates a stack frame in a6, then tests a fixed memory address, then branches if the last result was zero, then merges 0 into d0, then merges 174 into d0.
- `validate_and_fetch_data`: Compares a value with a global variable, calls a function if the condition is met, then fetches a byte from a memory location and returns it as a pointer.
- `set_flag_and_update_value`: Sets a specific bit in a memory location, updates a value in a structure based on parameter conditions, and calls another function with fixed parameters.
- `process_and_modify_data`: Calls another routine with parameters, checks the result, and if it's not zero, calls a function with data from a structure and modifies a byte in that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_process`: Calls a function with a parameter, processes the returned value, clears a byte in the result, and returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_and_return_value`: Checks several bit conditions in a structure, calls a function with parameters from the structure, updates fields based on the result, and returns a processed byte value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_modify_structure`: Calls a function with a global variable, stores the result in a structure, sets a field to another global variable, clears specific bits in a byte, and initializes several fields to zero.
- `calculate_and_store_value`: Adds a constant to a value from the stack, pushes the result onto the stack, calls a function with that value, stores the result in memory, and returns immediately.
- `update_list_node`: Calculates a new value by adding twelve to a value at an offset of eight from a pointer, stores this in a structure, and calls another routine with a global variable and a pointer.
- `initialize_structure`: Initializes a data structure by copying a pointer from one location to another and setting both the copied pointer and a zero value at specific offsets within the structure.
- `frag_link_move_bsr_addq_unlk_rts`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to d0, then copies d0 to the stack, then copies d0 to the memory at an offset from a6, then calls another routine, then adds 4 to a7, then removes the stack frame held in a6, then returns.
- `frag_link_tst_bgt_ori`: Short code fragment that creates a stack frame in a6, then tests the memory at an offset from a6, then branches if the comparison was greater, then merges 0 into d0, then merges 46 into d0.
- `manage_memory_operations`: Manages memory operations by performing calculations on values from the stack, calling functions with calculated parameters, and storing results in memory locations.
- `frag_link_move_bsr_addq_unlk_rts_2`: Short code fragment that creates a stack frame in a6, then copies the memory at an offset from a6 to d0, then copies d0 to the stack, then copies the memory at an offset from a6 to the stack, then copies d0 to the memory at an offset from a6, then calls another routine, then adds 8 to a7, then removes the stack frame held in a6, then returns.
- `update_linked_list`: Updates a linked list node by calculating a new value based on parameters, updating global variables, and either modifying existing nodes or creating new ones in the list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes`: Copies a specified number of bytes from one memory location to another using a loop that decrements a counter until zero.
- `process_bitfield`: Processes a bitfield value through a series of bitwise operations and shifts, updating internal state variables based on comparison results.
- `multiply_signed`: Performs multiplication of two signed integers with special handling for negative values and overflow conditions, returning the result in a specific register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movem_movea_jsr_ori_rts`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0, then returns.
- `frag_move_movea_jsr`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a7 to d1, then copies a fixed memory address to a6, then calls another routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movem_movea_jsr_ori_rts_2`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0, then returns.
- `frag_movem_movea_jsr_ori_rts_3`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0, then returns.
- `frag_movem_movea_jsr_ori_rts_4`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0, then returns.
- `frag_move_movea_jsr_2`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a7 to d1, then copies a fixed memory address to a6, then calls another routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movem_movea_jsr_ori_rts_5`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0, then returns.
- `setup_graphics_context`: Initializes various graphics system components by calling multiple library functions, sets up stack variables, and prepares data structures for rendering operations.
- `setup_display_buffer`: Configures display buffer parameters by calling several library functions, setting up stack variables with specific values, and preparing data structures for screen rendering.
- `frag_move_movea_jsr_ori`: Short code fragment that copies a6 to the stack, then copies a fixed memory address to a6, then copies the memory at an offset from a7 to d0, then calls another routine, then merges 117 into d0, then merges 0 into d0 (5 times in a row).
- `initialize_graphics_system`: Initializes various graphics system components by calling multiple library functions, sets up stack variables, and prepares data structures.
- `setup_graphics_parameters`: Initializes several byte values in a structure at fixed offsets, calls a library function to configure display buffer parameters, stores the result, and conditionally calls another routine based on a register value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movem_movea_jsr_ori_rts_6`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then merges 0 into d0, then returns.
- `frag_move_movea_jsr_3`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a7 to d0, then copies a fixed memory address to a6, then calls another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
