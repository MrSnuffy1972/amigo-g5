# Game 2008 (populous (1989)(electronic arts)[cr dcw][docs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02008). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (943584 top-level bytes in 42 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 689 raw-word lines |
| Functions named and described | pass, 746 of 746 named (33 author symbols), 746 of 746 described (217 by rule, 22 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1362 of 1362 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13984 of 13984 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 504914 |
| code | 44796 |
| data | 381099 |
| padding | 10727 |
| unused | 2048 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| SetMap | 13 | 627 | 33 |
| IFFL | 3 | 235 | 3 |
| More | 19 | 2321 | 79 |
| prompt | 2 | 117 | 2 |
| PATTERN | 3 | 223 | 6 |
| type | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr DCW][docs].adf:POPULOUS | 2 | 206 | 7 |
| DCW | 1 | 3 | 1 |
| Populous (1989)(Electronic Arts)[cr DCW][docs].adf:populous.prg | 2 | 206 | 7 |
| icon.library | 10 | 1421 | 110 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 3000 | 181 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr DCW][docs].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 664 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3208 | 221 |
| Populous (1989)(Electronic Arts)[cr DCW][docs].adf:populous.prg#unpacked | 3 | 709 | 20 |

## Cases handled

- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: file popsam (header 114718 bytes, blocks hold 63928): its blocks are judged as raw blocks outside the filesystem
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: file land3 (header 33714 bytes, blocks hold 17080): its blocks are judged as raw blocks outside the filesystem
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf:ICONS_PIC: The linear IFF-ILBM modulo cruncher V2.0 (xfd)-packed, unpacked with ancient (6815 -> 51364 bytes)
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 127 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1066 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf:c/PATTERN: TNM Cruncher 1.1 (xfd) stream found by ancient scan (6816 -> 8420 bytes) but only 0 of 5 sampled windows are in reference RAM: not used
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf:POPULOUS: PowerPacker 2.3 (xfd) stream found by ancient scan (15592 -> 29116 bytes) but only 0 of 11 sampled windows are in reference RAM: not used
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf:populous.prg: crunched executable decrunched by XFDMaster (PowerPacker 2.3 (xfd), 42372 -> 89092 bytes), 41 of 41 windows confirmed in reference RAM: recovered as a hunk executable
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 jump target, 1 address
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 2-10 (bytes 1024-5632) hold ASCII text
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 22-87 (bytes 11264-45056) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.86)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 97-98 (bytes 49664-50688) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.65)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 110-186 (bytes 56320-95744) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.75)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 191-197 (bytes 97792-101376) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.28)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 937-937 (bytes 479744-480256) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.07)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1003-1003 (bytes 513536-514048) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.05)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1089-1089 (bytes 557568-558080) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.00)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1100-1154 (bytes 563200-591360) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.08)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1159-1165 (bytes 593408-596992) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.89)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1210-1210 (bytes 619520-620032) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.03)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1227-1231 (bytes 628224-630784) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.57)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1683-1690 (bytes 861696-865792) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.87)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf: blocks 1694-1759 (bytes 867328-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.66)
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf:POPULOUS: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous (1989)(electronic arts)[cr dcw][docs]/Populous (1989)(Electronic Arts)[cr DCW][docs].adf:populous.prg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `_main` (SetMap): Opens intuition, icon and keymap resources, reads the KEYMAP tool type or argument, lowercases the name, loads the keymap segment and sends the console device IO request. Kind: tooltype/string lookup. Pass path: u0___L57.

## Functions


### SetMap

- `startup`: Saves the stack pointer and DOS command line, opens the DOS library, splits the command buffer into argv strings handling quotes and star escapes, sets up stdin/stdout/stderr, then calls _main and restores the initial stack.
- `_exit`: Moves the exit code to the restored initial stack, closes the DOS library if open, and if launched from Workbench forbids, replies the startup message and unloads, then returns the exit code.
- `waitmsg`: Loads the address of the message port at offset 92 from a4 and calls the exec wait and get-message functions to receive the Workbench startup message, then returns.
- `openDOS`: Loads the address of the dos.library name string into a1, sets d0 to zero, calls the exec library open function via a6, and stores the returned base pointer at absolute address 8, returning after the branch.
- `_Write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `_Input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `_Output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `_LoadSeg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `_cleanup`: Saves d2, then if the device flag is set frees the signal and closes the serial device; if IconBase is set frees the disk object and closes IconBase, closes IntuitionBase, and calls exit with the argument.
- `_failNotice`: Stores two arguments from the stack into two global variables, then calls a routine with a fixed set of arguments including a global address and constants, then calls another routine with a single constant and returns.
- `_main`: Opens intuition, icon and keymap resources, reads the KEYMAP tool type or argument, lowercases the name, loads the keymap segment and sends the console device IO request.
- `_AllocMem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `_FreeMem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `_AddHead`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `_FindName`: Library stub: loads its arguments and the exec library base, calls FindName and returns its result to the caller.
- `_FindTask`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `_AllocSignal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `_FreeSignal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `_PutMsg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `_WaitPort`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `_CloseLibrary`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `_OpenDevice`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `_CloseDevice`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `_DoIO`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `_OpenResource`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `_OpenLibrary`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `_GetDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `_FreeDiskObject`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `_FindToolType`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `_L3_2`: Saves d2, loads the format string argument, pushes the remaining stack arguments and stdout, calls the formatted print routine, then restores d2 and returns.
- `_AutoRequest`: Saves d2-d3 and a2-a3/a6, loads the IntuitionBase pointer into a6, copies seven stack arguments into a0-a3 and d0-d3, jumps to the library offset minus 348, restores registers and returns.
- `__doprnt`: Formats arguments through exec.RawDoFmt into a 140-byte stack buffer using stuffChar, measures the resulting string length, then writes it to the file handle with _Write.
- `stuffChar`: Stores the low byte of d0 at the address in a3 and advances a3 by one, then returns to the caller.

### IFFL

- `branch_to_offset_52`: Unconditionally branches forward by 52 bytes to another location, transferring execution without touching registers or memory.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `load_iff_picture_and_palette`: Opens the IFF file named in the global path, reads its header and chunks, allocates memory, loads the picture data and colour table, and sets up the screen and window for display.

### More

- `parse_command_line_into_argv`: Allocates a task structure, opens the dos library, splits the command line at spaces and quotes into an argument vector, and opens the input file for the game.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `editor_key_dispatch_loop`: Pushes registers, sets the editor state flag at offset 96, then loops reading keys via the input routines and branches on the key code to editor actions until a quit condition is set.
- `handle_editor_commands`: Processes editor commands by checking a condition and calling different routines based on whether the condition is met, with some calls using specific parameters.
- `editor_page_and_file_commands`: Handles editor commands: parses a filename, opens and reads a file into the buffer at offset 4796, formats and displays text, and restores the saved screen pointer at offset 184.
- `initialize_entity_state`: Sets a 32-bit field at offset 60 and a 16-bit field at offset 94 of a structure to specific values, calls another routine with a pointer from offset 52, calls itself recursively if offset 116 is not 4096 and offset 120 is non-zero, and enters a loop that calls another routine while certain conditions regarding offset 60, offset 8 of a pointed-to structure, and offset 94 are met.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field at offset 4 is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable; it also recursively calls itself, calls another routine with a pointer to a stack buffer, and calls a library routine to reschedule.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Calls another routine with zero, then with a parameter, then with a pointer from offset 16 of a structure, a global variable, and a parameter, conditionally calls another routine with the same arguments but without the parameter, calls another routine with two arguments, calls another routine with a constant, calls another routine with a parameter, and returns the result of the call involving two arguments.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `scroll_text_region_by_entity_count`: Allocates a stack frame, saves registers, then repeatedly calls the update routine to scroll a text buffer at offset 4284 of the entity, counting newline and form-feed bytes until the requested line count is reached, and clears the entity's word at offset 94.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_process_buffer_segments`: If a field at offset 120 is non-zero and a field at offset 116 is not 4096, it calls itself recursively with a pointer into the structure and a size from offset 120.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against the current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_string_to_destination`: Copies bytes from the source string at the second stack argument to the destination at the first stack argument, including the terminating null, advancing both pointers.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking itself on various fields and string arguments, conditionally updates fields at offsets 8, 6, and 7, and performs a loop based on a return value.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
- `initialize_object_from_template`: Obtains a template identifier, allocates an object, links its fields to a template and source block, and copies words from a source address into the new object.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `forward_two_args_with_stack_pointer`: Saves d2 and d3, loads two arguments from the stack, pushes a pointer to the caller's stack area plus both arguments, calls a routine, then restores registers and returns.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns certain a value, otherwise returning negative one.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `append_d0_byte_to_a3_alt`: Writes the byte in d0 to the location pointed to by a3, increments a3, and returns; used by a different caller than the similar routine.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `divide_d0_by_d1_via_swap`: Saves d2, moves d1 into d2 and d0 into d1, calls the 32-bit division routine, then restores d2 and returns.
- `shift_and_divide`: Moves data between registers, calls a 32-bit unsigned division routine, and returns the result in a data register.
- `negate_if_nonzero`: Checks if the result from a division operation is non-zero and negates it before returning.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `format_string_via_rawdofmt`: Saves a2-a4 and a6, loads the format string and argument pointer from the stack, points a1 at the stack arguments and a2 at a local format template, then calls exec RawDoFmt through a6.
- `store_byte_to_a3_and_advance`: Writes the low byte of d0 to the address in a3 and increments a3 by one, then returns to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `init_self_referential_struct`: Loads the return address from the stack into a0, stores a0 at (a0), adds four to that word, clears the longword at offset four, stores a0 at offset eight, and returns.

### prompt

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0 and branches forward to another routine, transferring control with d0 set from d1.

### PATTERN

- `pattern_entry_jump_to_main`: Calls the routine at offset eight, then jumps to address zero, so it never returns to its caller.
- `load_and_relocate_hunk_segments`: Allocates memory with exec.AllocMem, copies hunk data with exec.CopyMem, walks hunk type codes, applies relocations, frees memory with exec.FreeMem and returns.
- `apply_relocation_offsets_to_segment`: Reads relocation entries from a3, adds target offsets into the current segment's memory via a2, and advances d6 when d5 is set.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream by reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit, and writes the resulting bytes to a destination buffer.
- `fetch_bit_with_carry_rotate`: Pops a longword from the stack pointer a0, rotates it right through the carry flag after setting the extend bit, and returns the shifted value in d0.
- `shift_bits_into_d2_with_color_flash`: Shifts d0 right one bit at a time, refilling from memory at a0 when empty, and rotates each bit into d2 while writing a0 to COLOR00 and looping d1 times.

### type

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_d1_to_d0_and_jump`: Moves d1 into d0 and branches to another routine, passing the value along as the return argument.

### Disk-Validator

- `disk_validator_entry_checksum_and_setup`: Program entry that seeds a checksum in d0, computes a pointer from a4, stores it in a1, and calls library routines through a5 to validate disk data.

### Populous (1989)(Electronic Arts)[cr DCW][docs].adf:POPULOUS

- `hunk_loader_relocate_and_launch`: Program entry that walks AmigaDOS hunk tables, allocates memory with AllocMem, relocates hunks, frees memory with FreeMem, and jumps to the loaded program.
- `count_skip_and_set_flag`: Tests d5; if zero it returns, otherwise increments d6, sets d5 to one and returns to the caller.
- `walk_pointer_list_by_index`: Walks a linked list of pointers from the a5 frame, decrementing d1 and following each pointer doubled, returning the resulting pointer in a1.
- `decode_run_length_backwards`: Reads two little-endian words from the stack, combines their bytes into a bit offset and a bit count, and sets d7 to a space character minus the offset.
- `return_one_in_d0`: Loads the immediate value 1 into data register d0 and then falls through into the following routine.
- `shift_bits_into_d1_with_space_fill`: Loops d0 times, shifting d5 left through the extend bit into d1 and reloading d5 from memory whenever the d7 byte counter wraps to a space.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### DCW

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Populous (1989)(Electronic Arts)[cr DCW][docs].adf:populous.prg

- `relocate_hunks_and_jump_to_entry`: Allocates memory for AmigaDOS hunks, copies and relocates their sections using the library vectors, then jumps to the loaded program entry point.
- `set_d5_flag_and_increment_d6`: Tests d5; if it is zero it returns, otherwise it increments d6 and sets d5 to one before returning.
- `walk_linked_list_by_index`: Follows a linked list from the pointer at a5, stepping through nodes by index in d1, and returns the resulting node pointer in a1.
- `decode_run_length_backwards_2`: Reads two little-endian words from the stack, combines their bytes into a bit offset and a bit count, and sets d7 to a space character minus the offset.
- `set_d0_to_one`: Loads the immediate value one into d0 and falls through into the following bit-reading routine.
- `read_bits_from_backward_stream`: Extracts d0 bits from the backward bitstream in d5 and a0, refilling d5 from memory every 32 bits and returning the bits in d1.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### icon.library

- `return_minus_one_library_stub`: Loads minus one into d0 and returns immediately, giving callers a fixed failure value from this library entry point.
- `increment_counter_and_clear_flag_bit`: Adds one to the word counter at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, then returns that base pointer in d0.
- `decrement_counter_and_call_when_zero`: Sets d0 to zero, decrements the word counter at offset 32 of the a6 library base, and if it reaches zero with bit 3 of the byte at offset 14 set, calls another routine before returning.
- `close_icon_library_and_free_node`: Checks the word at offset 32 of the library base; if zero, unlinks the node at offsets 0 and 4, calls the library close vector at offset 34 with d0 set from offsets 16 and 18, and returns the saved value.
- `set_library_open_flag_return_zero`: Sets bit 3 of the byte at offset 14 of the library base and returns zero in d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_for_icon_base`: Stores a0 and a6 into the new library base at offsets 42 and 34, opens dos.library via the exec vector at offset -408, saves the result at offset 38, and calls Alert with code an address if it fails.
- `copy_a2_offsets_to_library_globals`: Copies the longwords at offsets 34 and 38 from a2 into the library globals at 402 and 406, returns a2 in d0, then pops a2 from the stack and returns.
- `push_a0_a6_call_library_vector`: Pushes a0 then a6 onto the stack, calls the library vector at address zero, then adds eight to the stack pointer and returns.
- `push_a0_a1_a6_call_vector_a8`: Saves a0 and a1 plus a6 on the stack, calls the library vector at offset a8, restores the stack by adding twelve and returns.
- `push_a0_a2_a6_call_vector_124`: Saves a0, a1 and a2 plus a6 on the stack, calls the library vector at offset 124, adjusts the stack by sixteen and returns.
- `icon_library_call_3fe_with_a0_a1`: Saves a0, a1 and a6 on the stack, calls the library routine at offset 3fe, then discards the saved registers and returns.
- `icon_library_call_656_with_a0`: Pushes a0 and a6 onto the stack, calls the library routine at offset 656, adds eight to the stack pointer and returns.
- `icon_library_call_692_with_a0`: Pushes a0 and a6 onto the stack, calls the library routine at offset 692, adds eight to the stack pointer and returns.
- `icon_library_call_a6_via_jsr`: Pushes a6 onto the stack, calls the library routine at the fixed vector, then discards the pushed longword and returns to the caller.
- `icon_library_save_a0_a2_call`: Saves a0 through a2 and a6 on the stack, calls the library routine at the fixed vector, then releases the saved registers and returns.
- `icon_library_save_a0_call`: Pushes a0 and a6 onto the stack, calls the library routine at the fixed vector, then discards both saved longwords and returns to the caller.
- `icon_lib_save_a0_a6_call_helper`: Pushes a0 and a6 onto the stack, calls the library helper at offset 2044, then discards both saved registers and returns.
- `icon_lib_save_a0_a1_call_helper`: Pushes a0 and a1 onto the stack, calls the library helper at offset 2072, then adds eight to the stack pointer and returns.
- `icon_lib_save_a0_a1_call_other_helper`: Pushes a0 and a1 onto the stack, calls the library helper at offset 2148, then adds eight to the stack pointer and returns.
- `call_library_entry_with_saved_regs`: Saves a0 and a1 on the stack, calls the library routine at absolute address 2262, then discards the saved registers and returns to the caller.
- `set_library_field_at_offset_94`: Opens the library base from the caller's a6, calls its offset minus 294 vector to get a structure, stores the second argument at offset 148 of that structure, and returns.
- `allocate_and_init_icon_node`: Calls two library routines to allocate an icon node for the given arguments, then copies 11 longs and several fields from a stack buffer into the node and links it into its parent list, returning the node or zero.
- `copy_stack_fields_into_icon_node`: Copies 11 longs from the stack frame into the icon node at a2, then stores individual byte and long fields and links the node into its parent at offset 424, returning the node in d0.
- `return_zero_and_restore_registers`: Sets d0 to zero as the return value, restores d2 through d4 and a2 from the stack frame, releases the a6 frame and returns to the caller.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `open_icon_stream_and_read_header`: Opens a stream on the passed name, reads the icon header, checks the magic word and version, then reads the icon's image and tool data through the stream callbacks, returning zero on failure.
- `read_icon_images_and_tool_data`: Allocates and reads the icon's image and tool data blocks through the stream callbacks, storing the resulting pointers in the icon structure and returning zero if any read fails.
- `icon_library_load_icon_arrays`: Calls library helpers to allocate and fill the icon data arrays at offsets 18, 22, 50, 54 and 70 of the icon structure, returning 1 on success or 0 on failure.
- `icon_library_fill_icon_array_loop`: Loops over the icon array at a4, calling helpers to allocate and store each entry, then optionally allocates the array at offset 70 and returns 1 or 0.
- `icon_library_return_zero_flag`: Sets d4 to zero and branches to the shared exit path, returning a failure flag to the caller.
- `restore_d2_d5_a2_a5_and_return_d4`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the a6 frame, releases the frame and returns to the caller.
- `open_icon_library_and_allocate_icon_data`: Saves registers, opens the icon library, allocates a 260-byte buffer, and on failure reports error 103 with the first argument before returning zero.
- `write_icon_info_chunks_to_file`: Writes icon header and info chunks through the a4 callback, handling image, palette and tool data fields of the icon structure at a2.
- `draw_icon_two_images_and_list`: Pushes fields at offsets 22 and 26 of the icon structure, calls the draw helper for each, then draws the optional image at offset 50 and walks the list at offset 54, returning d4.
- `advance_icon_list_and_draw`: Steps a3 through the null-terminated list at offset 54 of the icon structure, adding four to the stack counter, then calls the draw callback and restarts the list walk.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `push_a2_load_caller_arg_into_a0`: Pushes a2 onto the stack, loads the longword at offset twelve from the stack into a0, sets a2 to two bytes past that address, then branches to another routine.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `forward_icon_call_with_offset_arg`: Loads the first stack argument into d0 and the second into a0, pushes the address eighty-six bytes past a0 plus d0, calls a nearby routine, then cleans the stack and returns.
- `allocate_and_init_icon_node_2`: Saves d2-d3/a2, allocates a 65536-byte block via library call, then initializes fields at offsets 140 and 142 of the new block and returns the block pointer in d0 or zero on failure.
- `finalize_icon_node_after_init`: Pushes the a8 base and d2 plus the node field at offset 140, calls the init routine, cleans 16 bytes of stack, and returns the node pointer in d0 if successful.
- `return_zero_and_restore_regs`: Sets d0 to zero as the return value, restores d2-d3 and a2 from the stack, and returns to the caller.
- `icon_add_icon_to_list`: Pops d2 and a2, decrements the count word at a2, and if the count reaches zero calls the list grow routine; then stores the two arguments into the icon entry array and returns 1 or 0.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `icon_allocate_and_insert_icon`: Allocates a 94-byte block, initializes its fields, inserts it into the icon list via the add routine, then calls the draw and free routines, returning the block pointer or zero.
- `branch_to_icon_library_entry`: Branches unconditionally to another point in the icon library, transferring execution without touching registers or memory.
- `call_library_vector_and_return_zero`: Pushes a small constant and d4, calls a library vector, clears d4, restores the stack, returns zero in d0 and restores d2-d4/a2.
- `call_icon_routine_with_offset_argument`: Saves d2, loads two stack arguments, adds seventy-eight to the second, pushes both and calls a nearby icon library routine, then restores d2.
- `call_hook_with_arg_and_save_result`: Saves d2-d4/a2-a3, loads a2 and d2 from the caller's stack arguments, pushes d2 and calls the function pointer at address zero, then stores the result in d3 and a2 in d4 before adjusting the stack.
- `scan_string_for_equals_after_hook`: Saves d3, d2 and a3, calls the function at offset 72, and if it returns zero advances a3 by d3 and checks whether the next byte is an equals sign, returning that pointer or branching away.
- `fetch_next_pointer_or_return_zero`: Loads the longword pointed to by a2 into a3 and d0, advances a2, and if the value is zero returns zero after restoring d2-d4/a2-a3, otherwise branches to the caller's continuation.
- `call_second_arg_with_first_arg`: Saves d2-d5 on the stack, loads two arguments from the stack into d2 and d3, pushes d3 and calls a library routine, stores the result in d5, pops the argument and branches onward.
- `allocate_icon_with_size_argument`: Pushes the constant 124 and d2, calls a library allocation routine, stores the returned pointer in d4, pops both arguments, and if non-null returns the difference between the pointer and d2.
- `verify_icon_handle_and_size`: Calls a library routine with d2, compares the result to d5, then calls another routine with d5, d3 and d2, returning 1 on success or 0 on failure.
- `copy_d4_to_d2_plus_one_or_zero`: Tests d4; if zero branches away, otherwise copies d4 into d2 and increments d2 by one, then branches to the shared exit.
- `zero_d2_d0_and_restore_registers`: Clears d2, tests it (always zero so no branch), sets d0 to zero, pops d2 through d5 from the stack and returns.
- `parse_icon_copy_string_into_buffer`: Saves registers, clears byte 30 of a2, calls library string routines to parse the copy text at a3 into a2, and returns a2.
- `parse_icon_number_skip_five`: Advances a4 by five, pushes three arguments and calls the library routine at offset 72, then tests d0 and skips twelve bytes of stack; on failure sets d3 to one and advances a4 by three before branching away.
- `set_d3_zero_and_branch`: Sets d3 to zero and branches forward to another part of the library code, returning no value itself.
- `parse_decimal_number_from_stream`: Reads characters through a4, accumulating a base-ten number in d3 via repeated multiply by ten and a digit conversion call, then optionally formats and stores the result through library calls before returning a2.
- `ascii_digit_to_value`: Reads the longword argument from the stack, checks it lies between character 48 and 57, and returns the value minus 48, or minus one if outside that range.
- `return_minus_one`: Loads minus one into d0 and returns immediately, giving callers a failure result.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `allocate_icon_node_and_link`: Allocates a 96-byte node with tag 65536, stores value 10 in its word at offset 14 and in the caller's word, then links the node into the caller's list and returns 1 on success.
- `return_zero_restore_d2_a2`: Sets d0 to zero as the return value, restores d2 and a2 from the stack and returns to the caller.
- `compare_three_args_via_call`: Pushes the three arguments, calls the routine at offset 48, compares the result with the third argument, cleans the stack and returns 1 if equal else 0.
- `return_d1_in_d0`: Copies the longword in d1 into d0 and returns to the caller, providing d1 as the result.
- `compare_three_args_via_library_call`: Pushes the three longword arguments from the stack, calls the library routine at offset 76, moves the result to d1, compares it with the saved fourth argument, cleans the stack and returns zero or the result.
- `return_d1_in_d0_alt_entry`: Copies the longword in d1 into d0 and returns immediately, serving as an alternate entry point that yields d1 as the result.
- `signed_32bit_divide_d1_by_d2`: Compares d2 with 65535; if larger, performs a 32-bit restoring division of d1 by d2 using shift and subtract over 16 steps, otherwise divides the two 16-bit halves with divu and returns quotient in d0 and remainder in d1.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_d0_by_d1_32bit_2`: Multiplies the 32-bit values in d0 and d1 using partial 16-bit products, combines the cross terms and low product, and returns the 32-bit result in d0 while preserving d2.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `shift_and_divide_2`: Moves data between registers, calls a 32-bit unsigned division routine, and returns the result in a data register.
- `negate_result_on_flag`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `format_icon_string_via_rawdofmt`: Saves a2-a4/a6, loads format string and argument pointers from the stack, fetches exec base from address 4 and calls RawDoFmt at offset -522, then restores registers and returns.
- `store_byte_to_a3_and_advance_2`: Writes the low byte of d0 to the memory pointed to by a3 and increments a3, then returns; used as a put-char callback.
- `strlen_up_to_255_bytes`: Scans the string at the stack argument, counting bytes until a zero or 255 bytes, and returns the negated count in d0.
- `find_char_in_string_or_return_pointer`: Loads a string pointer and a character from the stack, scans bytes until a match or terminator, and returns the matching position or zero in d0.
- `return_zero_after_branch`: Sets d0 to zero and branches to the shared return path, providing a null result to the caller.
- `copy_string_until_terminator`: Copies bytes from the source pointer in a1 to the destination pointer in a0 until a zero byte is copied, then returns.
- `skip_leading_nul_byte`: Loads two pointers from the stack into a0 and a1, tests the byte at a0 and increments a0, and if the byte was zero steps a0 back one byte before branching onward.
- `compare_strings_up_to_length`: Loads two pointers from the stack into a0 and a1 and a length from the stack into d0, then compares bytes from a1 and a0 while decrementing the count, returning zero on a full match.
- `return_one_or_minus_one_by_flags`: Sets d0 to one, branches if the condition flags indicate greater, otherwise sets d0 to minus one, then returns to the caller.
- `scan_string_until_nul_or_count`: Loads a0 and a1 from the stack, takes a count from the stack into d0, then advances a0 past bytes until a zero byte or the count runs out, returning via the caller's flow.
- `decrement_a0_and_call_byte_copy`: Subtracts one from a0 and branches to the byte copy routine, then returns to its caller.
- `copy_bytes_until_nul_or_count`: Copies bytes from a1 to a0 while decrementing d0, stopping at a zero byte or when the count is exhausted, then pads with a zero byte.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_icon_header_self_pointers`: Loads the caller's first argument from the stack into a0, stores a0 at its own start, adds four to that word, clears the next longword and stores a0 at offset eight.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `load_icon_args_and_branch`: Saves d2-d3/a2 on the stack, loads the three stack arguments into d2, d3 and a2, then branches into another library routine.
- `walk_icon_list_validate_entries`: Walks a linked list of icon nodes in a2, calling the validation routine on each node and its data, advancing via the next pointer at offset 12, returning 1 if all pass or 0 on failure.
- `return_zero_restore_d2_d3_a2`: Sets d0 to zero as the return value, restores d2, d3 and a2 from the stack and returns to the caller.
- `allocate_and_fill_icon_structure`: Allocates a 20-byte icon block and a second block, computes scaled width and height from the passed structure, stores the second block pointer at offset 10 and returns the first block or -1 on failure.
- `icon_library_return_minus_one`: Sets d0 to minus one as the return value, restores d2 through d5 and a2 from the stack, then returns to the caller.
- `icon_library_save_regs_load_three_args`: Pushes d2 through d6 and a2 onto the stack, loads three longword arguments from the caller frame into d2, d3 and d4, clears d6, then branches to another library routine.
- `icon_library_call_with_stack_args`: Pushes a longword immediate, a low-memory pointer, d3 and d2 onto the stack, calls a library routine, stores the result in the caller frame, discards the arguments and branches based on the result.
- `icon_alloc_two_buffers_chain`: Walks a linked list of icon records, allocating a buffer for each via two library calls and storing the result at offset 8, returning -1 on failure or the last record pointer.
- `icon_return_minus_one_restore_regs`: Sets d0 to minus one as the failure return value, pops d2 through d6 and a2 from the stack, and returns to the caller.

### translator.library

- `translator_open_library_init`: Opens the translator library with exec AddLibrary using the resident name and version, stores the exec base and argument in the new library node, then calls MakeLibrary and returns minus one on success or zero on failure.
- `library_open_increment_and_clear_flag`: Tests the word at offset 32 of the library base in a6; if zero, increments it and clears bit 5 of the byte at offset 14, then returns the base in d0.
- `library_close_decrement_and_release`: Returns zero in d0, decrements the word at offset 32 of the library base in a6, and if it reaches zero with bit 5 of offset 14 set, saves a1 and calls the release routine.
- `library_open_set_flag_and_chain`: Saves d3, a3 and a6, copies a6 to a3, loads the library base from absolute address 4, and if the word at offset 32 is nonzero sets bit 5 of offset 14 before branching on.
- `unlink_node_and_call_library`: Removes the node from its doubly linked list by rewiring the surrounding links, then calls a function through the Amiga library base and returns a value stored in the node.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `parse_translator_format_string`: Saves registers, fills a stack buffer with spaces, sets up a frame, then scans the input string for hash and space characters, calling helper routines to process each field and returning a status in d0.
- `copy_d0_word_to_d1_and_branch`: Copies the low word of d0 into d1 and branches unconditionally to another routine, transferring control without returning here.
- `close_translator_stream_and_return_status`: Calls a library routine with d1 zeroed, writes a zero byte just before the returned a1 pointer, computes d0 as a1 minus the value at offset 8 of a5, restores registers and returns d1 sign-extended.
- `walk_translator_list_until_match`: Loads the list head from offset 0 of a5 into a6, returns if null, otherwise advances to the next node, calls two library routines, and branches based on the resulting condition codes.
- `scan_translator_table_from_offset`: Loads a pointer from offset 4 of a5, dereferences it into a6, points a6 at a fixed table offset, calls two library routines, then tests d7 and branches on its sign.
- `parse_three_escape_sequences`: Calls three library subroutines in sequence, saving a2 and a3 on the stack, branching to the shared exit on any nonzero result, then restores a2 and returns.
- `pop_regs_and_check_escape_char`: Pops a3 and a2 from the stack, reads a byte from a6, and branches to the escape handler if it is a backslash or backtick, otherwise continues to the shared exit.
- `returns_without_data_access`: The decompiled body consists only of a return and shows no reads or writes to any global variable, table or structure in the library.
- `match_bracketed_tag_text`: Skips forward to an opening bracket character, then compares the characters that follow against a reference string until a closing bracket.
- `backward_compare_translation_bytes`: Walks d6 backwards through the a6 table, looks up each byte in the a4 table, and compares it with bytes read backwards from a2 until a mismatch or negative index.
- `set_condition_codes_and_return`: Loads the condition code register with the immediate value four and returns immediately to the caller.
- `set_d7_one_and_test_condition`: Loads 1 into d7, calls the shared condition-test routine, branches back into the caller's loop if the result is nonzero, otherwise clears the condition codes and returns.
- `match_translation_string_against_table`: Reads characters from the table at a6 indexed by d6, skips entries flagged in the a4 table, compares each against the string at the stack argument, and returns when a mismatch or terminator is found.
- `set_d7_minus_one_and_test_condition`: Loads minus one into d7, calls the shared condition-test routine, branches back into the caller's loop if the result is nonzero, otherwise clears the condition codes and returns.
- `scan_back_for_escape_or_backtick`: Copies d6 to d7, decrements then increments d7, reads the byte at a6 plus d7 into d0, and returns when that byte is neither backslash nor backtick, subtracting d6 from d7 first.
- `check_translated_offset_against_limit`: Computes a1 minus the longword at offset 8 of a5 plus d7 into d0, compares it with d2, and if below, masks d6 to a word, adds it to a6, pushes d7 and branches onward.
- `copy_counted_bytes_with_flag`: Copies a counted run of bytes from the source to the destination buffer, then sets the continuation flag bit and clears it again if the next source byte is a backtick.
- `truncate_text_at_break`: Walks backwards from the current position through a text buffer until it reaches a break character, then writes a zero byte there to end the string.
- `dispatch_char_via_table_index`: Scans the 11-byte character list at a3 for d5, computes the index, loads the matching code pointer from the table at a3 and jumps to it.
- `test_d7_and_copy_byte_backward`: Tests d7 and branches if negative, otherwise copies the byte before a2 into d1 and decrements a2, then branches onward.
- `fetch_translation_word_indexed_by_byte`: Reads the next byte from the a2 stream, doubles it into d4, and returns the word from the a4 table at that doubled offset in d4.
- `poll_translation_status_bit_six`: Calls the shared byte-fetch routine, then tests bit six of the returned d4 value and returns with the condition codes set accordingly.
- `wait_for_translation_bit_five_clear`: Repeatedly calls the byte-fetch routine and tests bit five of d4, looping while it is set, then sets the carry flag and returns once clear.
- `check_d4_bits_3_and_5_after_call`: Calls a library routine, then tests bits 3 and 5 of d4 and returns, branching based on those flag tests.
- `scan_input_for_49_or_45_after_calls`: Calls a library routine, tests bit 5 of d4, calls again, compares d1 against 49 and 69, and sets condition codes before returning.
- `set_condition_codes_and_return_2`: Loads the condition code register with the value 8 and returns immediately to the caller.
- `parse_english_escape_sequence`: Calls the character fetch routine repeatedly and compares d1 against the letters E, S, D, R, L, Y, N and G, testing bit 7 of d4 to branch to shared exit paths in the translator library.
- `match_ng_and_optional_s_suffix`: Calls the character fetch routine and checks d1 for N then G, tests bit 7 of d4, then optionally matches S, setting the condition codes to 4 before returning.
- `set_condition_codes_to_eight`: Loads the immediate value 8 into the condition code register and returns immediately to the caller, an empty routine apart from the flag change.
- `test_d4_bit4_after_shared_call`: Calls the shared routine at offset 958, then tests bit 4 of d4 and returns with the zero flag set from that test.
- `test_d4_bit2_after_shared_call`: Calls the shared routine at offset 958, then tests bit 2 of d4 and returns with the zero flag set from that test.
- `test_d4_bit5_after_shared_call`: Calls the shared routine at offset 958, then tests bit 5 of d4 and returns with the zero flag set from that test.
- `translator_test_bit8_after_call`: Calls the shared library routine at offset 3be, then tests bit 8 of d4 and returns with the result in the zero flag.
- `translator_loop_test_bit5_add_a2`: Calls the library routine at offset 3be, tests bit 5 of d4, loops while set, adds d7 to a2 and sets the condition codes to 8 before returning.
- `translator_test_bit1_after_call`: Calls the shared library routine at offset 3be, then tests bit 1 of d4 and returns with the zero flag reflecting that bit.
- `call_helper_then_test_d4_bit1`: Calls the shared helper at offset 958, tests bit 1 of d4, and if clear adds d7 to a2 and sets the condition codes to 8 before returning.
- `call_helper_then_test_d4_bit7`: Calls the shared helper at offset 958, tests bit 7 of d4, and if clear sets the condition codes to 8 and returns to the caller.
- `set_condition_codes_and_return_3`: Sets the condition code register to 4 and returns immediately, touching no other registers or memory.
- `translate_input_to_uppercase_buffer`: Copies bytes from the a0 input pointer into the a5 buffer at offset 22, converting lowercase to uppercase, mapping control and delete characters to space, collapsing double hash into one, and returning a status in d1.
- `backtrack_buffer_and_emit_space`: Steps a0 back twice, writes a space just before the a2 buffer pointer, decrements d3 twice, and returns to the caller.
- `return_error_code_minus_three`: Loads the value minus three into d1 as an error return code and returns immediately to the caller.
- `write_two_spaces_to_a2`: Writes two space characters to the memory pointed to by a2, advancing a2 by two bytes, then returns to the caller.
- `insert_byte_after_escape_scan`: Scans bytes from the buffer pointer at offset 16 of a5 up to a1, skipping bytes flagged in the a4 table, then searches a 15-entry table for a match and shifts bytes to insert a value.
- `shift_bytes_and_store_escape`: Shifts bytes from a1 down one position toward a0, writes a byte value at the shifted slot, advances a1, restores saved registers and stores a1 at offset 16 of a5.

### info.library

- `illegal_instruction_trap_stub`: Executes the illegal instruction word 4afc, which triggers the 68000 illegal instruction exception and never returns normally.
- `store_a6_d0_and_a0_globals`: Saves a6 and d0 into absolute long globals, copies d0 into a1, stores a0 at offset 34 of that pointer and into another global, then returns.
- `push_a6_call_vector_pop_return`: Pushes a6 onto the stack, calls the routine whose address is stored at absolute address zero, discards the pushed longword and returns.
- `info_library_call_via_a6_offset_120`: Pushes a6 onto the stack, calls the library routine at offset 120 relative to a6, then discards the pushed longword and returns.
- `info_library_call_via_a6_offset_18`: Pushes a6 onto the stack, calls the library routine at offset 18 relative to a6, then discards the pushed longword and falls through to the next routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `info_library_call_a1_and_a6_offset_1506`: Pushes a1 and a6 onto the stack, calls the library routine at offset 1506 relative to a6, discards both saved longwords and returns.
- `clear_flag_and_increment_count`: Loads the caller's structure pointer from the stack, clears bit 3 of the byte at offset 14, increments the word at offset 32, and returns the pointer in d0.
- `check_count_and_set_flag`: Saves d2 and a2, loads the structure pointer, calls a subroutine, and if the word at offset 32 is nonzero sets bit 3 of the byte at offset 14 before calling another subroutine and returning zero in d0.
- `call_library_functions_with_offset_args`: This library routine retrieves a library base from a global variable and calls two library functions using offsets and calculated arguments.
- `decrement_refcount_and_release`: Loads the object pointer from the stack, decrements its word reference count at offset 32, and if it reaches zero and bit 3 of its flags is set, calls the release routine with the object.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `recursively_call_pointers`: The routine checks several global pointers and arguments, then calls itself recursively if a pointer is non-zero.
- `allocate_and_init_library_state`: Saves a2-a4, allocates a 944-byte block with the given flags, stores it in a global, and returns zero if allocation failed, otherwise proceeds to initialize the library state.
- `initialize_info_library_struct`: Initializes a structure or table by calculating and storing several offsets or pointers derived from a base address.
- `open_info_library_resources`: Calls a library open function with a name pointer, stores the result in a global, and on failure branches to an error path with code 3.
- `open_icon_library_and_store_base`: Pushes a small count and a name pointer, calls the open routine in a2, stores the returned base in a global, and branches on success.
- `open_intuition_library_or_fail`: Pushes a longword and the library name pointer, calls the open-library vector in a2, stores the returned base at offset 72, then cleans the stack and branches on the result.
- `open_graphics_library_store_base`: Pushes the graphics library name and calls the open vector in a2, stores the base at offset 76, and on failure calls the vector in a3 with 5 and two zero arguments before returning zero.
- `return_one_and_pop_a2_a3`: Sets d0 to 1 as the return value, restores registers a2 and a3 from the stack, and returns to the caller.
- `check_library_base_flag_and_branch`: Pushes a2 onto the stack, loads the absolute address 88 into a1, tests the longword there and branches to another routine if it is nonzero, otherwise branches elsewhere.
- `check_node_type_seven_and_branch`: Loads the pointer stored at a1 into a0, compares the byte at offset 61 of that structure with 7, and branches to another routine if equal, otherwise elsewhere.
- `load_node_field_and_return_value`: Loads the pointer at a1 into a0, reads a2 from offset 56, tests the longword at offset 164 and if zero branches away, else loads that field into d0 and branches.
- `get_node_field_from_a1_pointer`: Loads the pointer at (a1) into a0, checks byte at offset 61 of that structure, and if it equals 1 loads a0 from offset 76 then d0 from offset 442, then branches onward.
- `return_a2_or_follow_offset_76`: Moves a2 into d0; if a2 is zero branches to the shared return, otherwise loads a0 from offset 76 of a2 and branches back into the caller's flow.
- `return_zero_and_pop_a2`: Sets d0 to zero as the return value, pops a2 from the stack and returns to the caller.
- `open_info_disk_and_store_result`: Opens the disk.info file via the DOS library, stores the returned handle at offset 152 of the info structure, then checks a flag byte and branches to the open path.
- `check_library_status_and_set_flag`: Checks a boolean flag and a function return value to decide whether to update a global flag and retrieve a value from a nested pointer.
- `allocate_info_buffers_and_load`: Allocates two memory blocks of size 65536, stores their pointers in globals, then loads the info file into the allocated buffers and returns success or failure.
- `push_alloc_args_and_store_result`: Pushes immediate 65536, a low-memory pointer and d3 onto the stack, calls a library routine, stores the returned d0 into (a4), then pops the arguments and branches based on the result.
- `check_node_flag_and_call_cleanup`: Loads a pointer from d4 into a1, reads the node at (a1) into a0, tests byte at offset 61, and if set pushes a low-memory long and d2 before calling a routine and popping the stack.
- `test_d2_and_call_release`: Tests d2 and returns if zero, otherwise pushes the value at (a4) and d2, calls a library routine, pops the arguments and branches on the returned d0.
- `open_info_library_and_allocate_buffer`: Calls library open helpers with offsets from a3 and a4, increments a counter at offset 100, clamps it to 256, allocates a buffer via a library call and stores the pointer at offset 96, returning nonzero on success.
- `copy_library_strings_and_data`: This library routine copies a string using a library function and then copies certain dwords or even dwords from one structure to another.
- `initialize_info_library_globals`: Clears several global longs, stores the argument at a global, calls an initialization routine and branches to error handling if it returns zero.
- `open_library_and_store_base`: Pushes a version number, a name pointer and a library base pointer, calls the open routine, stores the returned base at offset 100, then pops the arguments and branches if the result is nonzero.
- `close_library_and_branch_on_error`: Pushes the library base pointer, calls the close routine, tests the returned value, pops the argument and branches to an error path if the result is nonzero.
- `call_vector_and_save_result_in_a3`: Calls the routine at address zero, moves the returned value into a3 and d2, then branches depending on whether the result is zero.
- `allocate_68_byte_block_store_in_a4`: Pushes the immediate value 68 and calls a library allocation routine, stores the returned pointer at (a4), pops the argument, and branches onward if the result is nonzero.
- `check_info_and_call_callback`: Pushes (a4) and field 50 of a3, calls a comparison routine, and depending on the result calls further library routines and invokes the a2 callback with 0 or 1.
- `query_value_then_call_with_result`: Calls a routine at pc offset 418, saves d0 in d2, and if nonzero pushes d2 and a stack slot to call another routine, branching based on its result.
- `validate_library_parameters`: As part of library code, this routine checks a global variable, calls a library function via a global base, and executes a parameter-validated routine.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `info_dispatch_by_node_type`: Loads a node pointer and value from the stack, reads a word from the node, and jumps through a table indexed by that word when it is within range.
- `return_one_pop_a2_a3`: Sets d0 to 1 as the return value, pops a2 and a3 from the stack, then returns to the caller.
- `toggle_flag_bits_by_message_code`: Reads a word from offset 38 of a2, maps codes 6 to 9 and 17 to 19 to bit masks, XORs the mask into a longword at offset 112 of the library base, then calls a routine with a1's offset 44.
- `frag_ori_moveq_bra`: Short code fragment that merges 32 into the memory at a4 (advancing a4), then merges 40 into the memory just below a4 (moving a4 down), then merges 56 into a fixed memory address (2 times in a row), then merges 44 into a fixed memory address, then merges 8 into an indexed entry from a4, then loads 4 into d0, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 2 into d0, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 1 into d0, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 16 into d0, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 32 into d0, then jumps to another point in the code.
- `xor_store_and_call_template`: Library code that exclusive-ORs d0 with d1, stores the result at offset 112 of a global structure, and calls another routine with a longword read from offset 44 of the structure in a1, then returns.
- `info_library_open_and_validate`: Library routine that opens the info device, configures it, checks the node type byte, allocates and stores a buffer pointer, and returns a success flag in d2.
- `check_node_type_and_select_disk`: Loads the node pointer from a3, compares its type byte at offset 61 with one, and branches to push either the disk string or a fallback argument.
- `process_string_and_attributes`: This library routine copies a string, calls a library function, and performs several conditional checks involving pointer offsets and string copies, potentially updating a local variable with a value from a structure.
- `open_four_info_tags_with_fallback`: Saves d2-d3/a2, calls the tag-opening routine with the caller's tag list and three -1 tags, and returns 1 only if all four opens succeed, otherwise 0.
- `open_second_info_tag_after_first`: Pushes -1 and the second global tag value with d2, calls the tag-opening routine through a2, and branches to the shared exit if the result is not -1.
- `open_third_info_tag_after_second`: Pushes -1 and the third global tag value with d2, calls the tag-opening routine through a2, and branches to the shared exit if the result is not -1.
- `call_a2_and_check_ff_result`: Pushes -1, the longword at address 320 and d2 onto the stack, calls the routine in a2, then compares d0 with -1, discards 12 stack bytes and returns zero if equal.
- `call_114_and_return_one`: Pushes d2, calls the routine at address 276, sets d0 to 1, adds 4 to the stack pointer, restores d2-d3/a2 and returns.
- `initialize_info_library_tables`: Loads a pointer from offset 80 of the input structure, calls a routine with it, then repeatedly calls another routine many times with the input pointer paired with different constant arguments, passing a global variable to some of those calls, and returns.
- `execute_list_of_library_calls`: This library routine calls another routine multiple times with a fixed first parameter and varying other constant arguments.
- `initialize_display_palette_entries`: Loads a pointer from the input structure into d2, then calls another routine seven times passing that pointer with a fixed set of integer constants that vary each time, zeroing the stack for some of the calls, and returns.
- `register_info_table_entries`: Loads a pointer from the input structure into d2, then calls another routine ten times passing that pointer with a fixed set of integer constants that vary each time, zeroing the stack for some of the calls, and returns.
- `set_info_table_entries`: Loads a pointer from the input structure into d2, then calls another routine ten times passing that pointer with a fixed set of six integer constants that vary each time, zeroing the stack for some of the calls, and returns.
- `info_library_check_requester_flag`: Saves d2/a2-a3, loads two stack arguments into a2 and d2, points a3 at a small constant, reads the base pointer from absolute address 88 and tests byte 61 of it against 4, branching to set d1 to 131.
- `info_library_compare_and_report_error`: Loads a3's target, compares the pointer at offset 234 plus 4 with the pointer stored there, and if unequal pushes error parameters and calls the report routine before branching onward.
- `execute_multiple_library_calls`: Library code that calls a library function or routine multiple times using a pointer from a structure.
- `allocate_info_list_node`: Saves registers, reads the argument from the stack, calls an allocation routine for 14 bytes, stores the result at offset 234 of the a2 structure and returns zero if allocation failed.
- `link_info_nodes_into_list`: Reads the info structure through a3, advances its node pointer by four bytes, stores that pointer at offset eight of the node, clears the field at offset 242, then walks the list at offset 92 of the global structure and links each entry, returning the first node pointer.
- `open_info_library_stream`: Pushes a 65536-byte buffer size, a small word argument and d2, calls the library open routine, stores the returned handle in a2 and d3, then cleans the stack and branches onward.
- `iterate_info_stream_entries`: Opens a stream with a 65536-byte buffer, stores the handle at offset 10 of a2, then loops over entries in a4 calling read and dispatch routines until a zero entry, finally copying a pointer to offset 238 of the library base.
- `check_info_stream_position_and_dispatch`: Loads the library base from a global, compares the stream's current and saved positions, and if they match or the entry is at the start, calls the flush and close routines with fields from a2.
- `update_pointer_from_structure`: Library code that calls a routine with a pointer from a structure, then updates that structure member with a value from another pointer.
- `check_info_list_bounds_and_dispatch`: Compares the list head's offset-eight field and offset-four pointer against the tail at offset 238; if either matches it calls the notify and message routines with fields from a2, otherwise it advances the node and dispatches.
- `update_pointer_dereference`: Library code that calls a routine and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `modify_global_state_and_execute`: Library code that sets a global variable, checks a condition, calls several library functions, and updates a global pointer.
- `remove_current_info_node`: Library code that marks the info list busy, flushes the current node, unlinks the current node from the list, frees it, and advances the current pointer to the next node.
- `free_current_info_node_advance`: Library code that reads the current node pointer, frees that node, then advances the current pointer to the next list entry and flushes the list.
- `info_library_close_and_release_object`: Calls library functions to close a resource, pushes fields at offsets 46 and 62 of a2 as arguments, cleans 16 bytes of stack, restores d2/a2-a3 and returns.
- `info_library_scan_node_list_for_flag`: Saves d2-d6/a2-a5, loads constant 100 into d3 and 88 into a4, reads a pointer from address 100, tests a longword at offset 242 of that node and branches based on it.
- `compare_node_pointer_and_clear_list`: Loads the node pointer from a3, computes the address four bytes past its offset 234 field into d0, compares it with the stored pointer, and on mismatch clears the field at offset 92 of the structure pointed to by a4.
- `load_info_node_and_jump_common`: Pushes the library base from absolute address 272, sets a5 to d3, loads a2 from (a5), fetches a0 from offset 238 of a2, pushes a0's offset 10, calls the library jump table at 22, sets d4 to 1, reloads a3 from offset 234 of the new a0, pops 8 bytes and branches to shared code.
- `count_nodes_alloc_index_table`: Walks the a3 linked list incrementing d4 until a null pointer, shifts d4 left twice, allocates that many bytes with tag 65536 via the library call, stores the result at offset 92 of a2, and branches to the fill routine or returns zero on failure.
- `free_index_table_and_reset_list`: Pushes d4, loads a2 from (a4), pushes the table pointer at offset 92, pushes a4 plus 140, calls the library free routine, clears d4, reloads a3 from offset 234 of the node, pops 12 bytes and branches to the refill loop.
- `allocate_info_buffer_and_copy_entry`: Calls library routines to allocate a buffer of size from the entry, stores the pointer in the table indexed by d4, copies the entry data into it, and advances to the next entry.
- `copy_entry_and_advance_to_next`: Copies the current entry's data into the allocated buffer, increments d4, walks the entry list via a3, and returns 1 when the list end is reached or jumps back to the allocation routine.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_info_display_by_mode`: Saves registers, loads the mode byte at offset 61 of the info structure, and jumps through a table to draw the matching info screen layout.
- `draw_info_windows`: Displays multiple text strings and rectangles using various library functions, with calls to draw_nine_rectangles_third_set and other drawing routines.
- `frag_movea_bra`: Short code fragment that copies 96 to a0, then jumps to another point in the code.
- `frag_movea_bra_2`: Short code fragment that copies 106 to a0, then jumps to another point in the code.
- `frag_movea_bra_3`: Short code fragment that copies 118 to a0, then jumps to another point in the code.
- `frag_movea_move_pea_bra`: Short code fragment that copies 130 to a0, then copies a0 to the stack, then pushes the address of a fixed memory address onto the stack (2 times in a row), then jumps to another point in the code.
- `show_tool_information`: Displays tool-related information including comments and drawer details by calling several drawing routines and pushing text strings to the stack.
- `display_memory_stats`: Shows memory usage statistics including block counts and sizes, calling multiple drawing functions and accessing global data structures.
- `show_project_details`: Displays project information including default tool settings and memory details by calling various drawing routines and accessing global variables.
- `display_tool_information`: Displays tool-related information including comments and drawer details by calling several drawing routines and pushing text strings to the display.
- `frag_pea_move_jsr_lea_movem_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies a2 to the stack, then calls the routine whose address is in a3, then loads the address of the memory at an offset from a7 into a7, then restores saved registers from the stack, then returns.
- `allocate_four_brush_bitmaps`: Calls the library allocator four times to create bitmaps of sizes 44, 36, and two from offset 100, storing results in globals and returning zero if any allocation fails.
- `allocate_second_bitmap_set`: Calls the allocator four more times for sizes 44, 36, and two of 116, storing results in globals and a4, then branches to the shared dispatch if all succeed.
- `dispatch_info_state_index`: Loads the library base from absolute address 88, reads byte at offset 61 into d0, and jumps through a table indexed by that value when it is between 1 and 5.
- `restore_registers_unlink_and_return`: Restores d2, d3, a2, a3 and a4 from the stack frame at offset -44 from a6, releases the a6 frame and returns to the caller.
- `check_flag_and_clear_byte`: Saves d2 and d3, loads the argument from the stack into d2, tests a longword at offset 104 of the library base, and clears a byte at the address in absolute 352 when the flag is zero.
- `open_info_stream_with_name`: Pushes a name string and a stream handle onto the stack, calls the open routine, then calls the read routine with a buffer and returns zero if it fails or the result otherwise.
- `restore_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns to the caller, restoring the saved register set.
- `set_channel_modes_and_open_four`: Checks a status byte, writes mode words to four channel structures, then opens each channel and returns one only if all four opens succeed.
- `call_a2_with_ff_and_148_vector`: Pushes -1, the longword at absolute address 148 and d2 onto the stack, calls the routine in a2, then compares d0 with -1 and branches based on the result.
- `call_a2_with_ff_and_154_vector`: Pushes -1, the longword at absolute address 154 and d2 onto the stack, calls the routine in a2, then compares d0 with -1 and branches based on the result.
- `call_a2_with_ff_and_158_vector`: Pushes -1, the longword at absolute address 158 and d2 onto the stack, calls the routine in a2, compares d0 with -1, and returns zero in d0 when they differ.
- `return_one_restore_d2_d3_a2`: Sets d0 to 1 as the return value, restores d2, d3 and a2 from the stack, then returns to the caller.
- `poll_signal_bit_and_dispatch`: Reads a signal bit number from the library base, loops calling two library routines to wait on and handle that signal, and returns zero on failure.
- `return_zero_restore_d2_d4_a2`: Sets d0 to zero as the return value, restores d2, d3, d4 and a2 from the stack, then returns to the caller.
- `dispatch_info_node_by_type`: Saves d2-d3/a2, loads node pointer and value from the stack, reads the node type at offset 20 and branches to type-specific handling for values 4, 64 or 512, otherwise falls through to a default call.
- `call_node_callback_with_saved_regs`: Pushes d2 and a2 onto the stack, calls the function pointer at address a4, then discards the two saved registers and branches onward.
- `call_second_callback_return_zero`: Pushes a2, calls the function pointer at address d4, sets d0 to zero as the return value and branches onward.
- `call_two_library_hooks_return_one`: Pushes d3 and a2, calls the library routines at offsets b8 and d4 with a2 as argument, sets d0 to 1, restores d2-d3/a2 and returns.
- `execute_multiple_callbacks`: Library code that calls another routine or a global function pointer multiple times based on a condition checked against a global variable, then calls another routine with a specific pointer and constant, and finally calls another routine with a certain pointer and a constant.
- `setup_string_template`: Library code calculates a bitmask from a value in a table and calls a string-formatting routine multiple times with different bit-masked arguments and a global pointer.
- `info_draw_two_text_lines`: Sets up a stack frame, saves d2-d6, copies a four-byte attribute string, then calls the text drawing routine twice with different pen numbers and coordinates.
- `format_string_arguments`: prepares a pointer-based argument list or a temporary buffer for a string-formatting routine that uses a pattern or template string and multiple string arguments.
- `info_load_d0_and_clear_d1_a1`: Loads the longword at offset four from the stack into d0, sets d1 to zero, clears a1, then branches forward past the following code.
- `skip_to_nul_after_offset`: Increments a1, then tests the byte at a1 plus d0 until a zero byte is found, and returns the resulting a1 pointer in d0.
- `copy_string_until_nul`: Loads destination from stack offset four and source from offset eight, then copies bytes from a1 to a0 including the terminating zero.
- `load_args_and_branch_to_copy`: Loads a0 and a1 from the stack arguments, sets d0 from the third argument and d1 to zero, then branches to the string copy code.
- `copy_bytes_until_count_reached`: Copies bytes from the address in a0 to the address in a1, incrementing both pointers and d1 until d1 reaches the count in d0, then returns.
- `load_two_stack_args_and_branch`: Loads the first two longword arguments from the stack into a0 and a1, then branches forward into another routine's code.
- `compare_strings`: Library code that compares two null-terminated strings and returns true if they are equal.
- `strlen_return_length_plus_one`: Reads the byte at the address passed on the stack, counting bytes until a zero terminator, and returns the count plus one in d0.
- `execute_library_operations`: Library code that calls several other library functions, including a routine that initializes a structure array and routines that use conditional logic or call library vectors.
- `call_string_formatter`: Library code that calls another routine using a pointer to a global variable and a local buffer.
- `load_byte_and_set_loop_bounds`: Saves d2 and a2-a3, loads a0 and d0 from the stack arguments, computes a3 as a0 shifted left by two, reads a signed byte from a3 into d1, and sets d2 and a2 to one before branching away.
- `copy_bytes_until_limit_and_terminate`: Copies bytes from a2 plus a3 into a2 minus one plus d0, incrementing a2 until it exceeds d1, then writes a zero byte at d1 plus d0 and restores registers.
- `initialize_info_structure_and_open`: Fills two caller-supplied structures with the stack arguments and fixed defaults, calls two setup routines, then invokes the open routine and returns whether it succeeded.
- `call_a3_offset_3e_and_return_one`: Pushes a3, calls the function pointer stored at offset 62 in the a3 structure, then returns 1 in d0 after restoring d2-d3/a2-a3.
- `execute_library_calls`: Library code that calls a library function or another routine multiple times using a pointer retrieved from a structure member.
- `format_string_via_rawdofmt_2`: Loads the format string and argument pointer from the stack, calls exec RawDoFmt through a6, then restores a2-a4/a6 and returns.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.

### version.library

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `store_a6_and_a0_into_library_struct`: Copies d0 into a1, then stores a6 at offset 34 and a0 at offset 38 of that structure before returning.
- `increment_counter_and_clear_flag`: The library routine increments a 16-bit value at an offset of 32 from a base pointer and clears bit 3 of a byte at an offset of 42 from the same base pointer, then returns.
- `decrement_open_count_and_close`: Sets d0 to zero, decrements the word at offset 32 of the library base a6, and if it reaches zero with bit 3 of byte at offset 42 set, calls the close routine before returning.
- `open_library_increment_and_set_flag`: Saves d2/a5-a6, builds a stack frame, loads a6 from offset 34 of a5, and if the word at offset 32 of a5 is nonzero sets bit 3 of byte at offset 42 and returns zero in d0.
- `get_library_value`: Reads a 32-bit value from offset 60 of the structure pointed to by register a5 into register d2, then calls two library functions via pointers at offsets -252 and -210 of register a6, then returns that value.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.

### Populous (1989)(Electronic Arts)[cr DCW][docs].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `relocate_three_jump_tables`: Adds the base offset from the first longword to entries of three tables at a3, a4 and a2, then jumps to the relocated code at a4.
- `initialize_library_system`: Initializes the library system by setting up global pointers, opening libraries, and handling supervisor mode execution when needed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `allocate_screen_and_audio_buffers`: Calls library allocation routines to reserve several memory blocks, stores the returned pointers in a4-relative globals, then initializes display and sound and returns.
- `call_audio_play_sound_38`: Opens a stack frame, loads the audio base pointer from a4-relative storage, pushes the longword at offset 38 and calls the audio library routine, then discards the argument and returns.
- `init_sound_and_interrupt_setup`: Calls several audio library routines, configures eight sound channels with volume and period values, enables the interrupt bit in INTENA, then calls two local setup routines before returning.
- `set_sound_state_399_and_call`: Stores the value 399 into a4-relative sound state word, then branches to a local routine before releasing the stack frame and returning.
- `init_petersint_io_and_open_port5`: Sets up a two-byte IO request block with a code pointer and buffer, copies two words from the open device structure, calls the setup routine, then opens port 5 with a buffer.
- `close_port5_and_send_command9`: Closes the port 5 IO request with buffer at the given offset, then sends command 9 through the library call and returns.
- `open_serial_ports_11_and_0`: Builds two IO request blocks with code pointers and buffers, opening serial unit 11 and then unit 0 via the library call.
- `enable_vertical_blank_interrupt`: Opens an a5 stack frame, writes the value 2049 to the custom chip interrupt enable register to enable vertical blank interrupts, then releases the frame and returns.
- `close_two_library_bases`: Calls a setup routine, then closes the two library bases stored in a4-relative globals by passing each to the close-library vector, cleaning the stack after each call.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `open_graphics_and_intuition_libraries`: Opens graphics and intuition libraries, stores their bases in a4-relative globals, exits on failure, and saves the low bit of the graphics library flags word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_game_state_and_copy_tables`: Calls several library routines, copies two eleven-entry tables into working buffers, sets display and sound parameters, then initialises two pointer tables and copies the first pair of entries.
- `load_and_scale_background_pictures`: Opens picture files, allocates memory, loads and scales the images into buffers, then frees the file handles and returns zero on success or minus one on failure.
- `match_pointer_and_update_slot`: Scans three pointer slots for a given value, calls two routines with the matching slot's data, and stores a new value into a global if one is supplied.
- `swap_two_global_pointers_and_call`: Swaps the two longwords at offsets -26212 and -26216 from a4, then calls a subroutine with both values pushed and a -1 word, restoring the stack.
- `update_tile_position`: Calculates and stores tile coordinates based on input parameters, then updates a counter in a global table and makes several calls to another routine with specific values from the table.
- `handle_file_operations`: Sets up stack frame, performs file system operations using library calls, processes directory entries, validates character ranges, transforms characters to uppercase, and manages dispatch table calls.
- `frag_movea_tst_bgt_addq_move_clr_bra`: Short code fragment that copies the memory at an offset from a4 to a0, then tests the memory at an offset from a0, then branches if the comparison was greater, then copies the memory at an offset from a4 to a0, then adds 8 to a0, then copies a0 to the memory at an offset from a5, then clears the memory at an offset from a5, then jumps to another point in the code.
- `validate_character_range`: Checks if a character byte is within the range 97 to 122, and returns immediately if not; otherwise it subtracts 32 from the character value.
- `process_character_buffer`: Copies character data from one memory location to another while updating indices, clears memory locations under certain conditions, and handles character validation and transformation before calling dispatch functions.
- `poll_mouse_and_buttons_update_state`: Reads JOY0DAT and POTGOR to update mouse position and button state, clamps coordinates, and triggers game actions through several subroutine calls.
- `compare_short_difference_2`: The routine computes the absolute difference of two shorts from global memory and stores it into a word pointed to by a register, then returns a bool indicating whether an input value is less than or equal to that difference.
- `compare_short_difference`: The routine computes the difference of two shorts from global memory and stores it into a word pointed to by a register, then returns a boolean indicating whether a short from a register is less than or equal to the stored difference.
- `frag_moveq_bra_5`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `validate_game_data_2`: Reads and validates various data segments from memory using a series of checks, storing cumulative results in a local variable.
- `validate_game_data`: Reads and validates various data segments from memory using a series of checks, storing cumulative results in a local variable.
- `frag_link_clr_bra`: Short code fragment that creates a stack frame in a5, then clears the memory at an offset from a5 (2 times in a row), then jumps to another point in the code.
- `accumulate_values`: Reads word values from a table using an index, adds them to an accumulator, and increments the index until it reaches a limit.
- `initialize_game_context`: Initializes game graphics context by setting up memory regions with specific values, then configures a global variable based on a conditional check.
- `initialize_graphics_context`: Initializes graphics context by setting up multiple memory regions with specific values, then configures a global variable based on a condition derived from a memory check.
- `setup_graphics_context`: Clears a global variable, makes several library calls with specific parameters, copies values between memory locations, and performs initialization for graphics rendering operations.
- `initialize_table_processing`: Initializes a counter in memory and processes table entries by checking their validity, incrementing a counter for valid entries, and clearing specific memory locations based on the counter value.
- `process_table_entries`: Iterates through table entries, checks if each entry is valid, calls a function for valid entries, and increments a counter until reaching a maximum value.
- `frag_moveq_muls_addq_move_bra`: Short code fragment that loads 0 into d0, then multiplies d0 by 3 (signed), then adds 1 to d0, then copies d0 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_move_jsr_addq_unlk_rts`: Short code fragment that copies the memory at an offset from a5 to the stack, then calls another routine, then adds 4 to a7, then removes the stack frame held in a5, then returns.
- `read_keyboard_serial_2`: The routine reads the keyboard serial register, checks its low bit and, if set, stores the low byte into a global variable and computes an index from the high byte to fetch a byte from a table.
- `map_input_to_table`: The routine checks the low bit of an input value and, if set, stores the low byte into a global variable and computes an index from the high part of the input and the negated global variable.
- `handle_input_and_update`: Processes mouse and keyboard input data, updates global variables with computed values, handles serial input, and manages various game state flags.
- `build_copper_list_for_display`: Writes a Copper display list into the buffer at a0 using the passed bitplane pointers, palette and sprite data, then enables Copper DMA via DMACON.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `set_copper_list_and_wait_vblank`: Reads the return address from the stack, adds four, writes it to the copper list pointer high register, then polls the interrupt request register until the vertical blank bit is set.
- `acknowledge_vblank_interrupt`: Writes 32 to INTREQ to clear the vertical blank interrupt, then polls INTREQR bit 5 until that interrupt request flag becomes set again.
- `copy_data_with_bounds_check`: Copies word values from one buffer to another while checking for a specific boundary condition, then fills the remaining space with a fixed value.
- `called_twenty_one_instructions`: No further behavior can be described from the available information because the decompiler timed out.
- `set_object_position_and_velocity`: Indexes a table and an object record by the given index, stores the supplied position and velocity longwords into both, or clears them and copies the object base address when the value is zero.
- `copy_struct_fields_loop`: The routine computes source and destination pointers from parameters, then loops a number of times copying several fields from the source structure to the destination structure at fixed offsets, advancing both pointers each iteration.
- `update_byte_buffer_from_params`: The routine indexes a table with a parameter to obtain a byte buffer pointer, then writes several bytes into that buffer derived from the two parameters and a global variable, including conditional flag bits.
- `process_character_stream`: Processes a character stream by iterating through bytes, performing masked OR operations on buffer data using lookup tables.
- `apply_masked_bytes`: The routine iterates over a table indexed by a byte from d0 and performs masked OR operations on bytes in a buffer using a global variable as a base, then scans a byte list from a pointer until a zero byte is found, handling a special case for value 10.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `configure_hardware_registers`: The routine writes several constants to a global variable using arithmetic involving a global variable, then returns a value derived from that global variable and an input register.
- `clear_two_globals`: The routine stores zero into two global variables and then returns.
- `update_handler_state`: The routine writes a global variable through a pointer derived from a parameter, increments a short counter in that parameter with wrap at 120, stores a constant into a global variable, and returns zero.
- `frag_move_rts`: Short code fragment that copies 0 to the memory at an offset from a1, then copies 1 to a fixed memory address, then copies 0 to d0, then returns.
- `handle_keyboard_input`: Handles keyboard input by checking serial register flags and processing bytes from a stream with conditional branches based on input values.
- `process_character_stream_2`: The routine loops calling a bounds-check routine and a character stream processor, updating a global flag and manipulating bytes and shorts from two pointers based on a counter, returning a combined value when a condition is met.
- `frag_adda_subi_bne_move_tst_movem_unlk`: Short code fragment that adds 1 to a0, then subtracts 1 from d1, then branches if the last result was not zero, then copies 0 to d0, then tests d0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `wait_for_keyboard_byte`: Waits for keyboard input by decrementing a counter, checking serial register flags, and processing bytes from a stream with conditional branches.
- `wait_for_serial_byte`: The routine loops calling a bounds-check routine and waiting for a condition on a global flag, then processes a byte stream using pointers and a counter, updating a global flag and a short field, and returns a combined value when a condition is met.
- `frag_movem_move_subi_lea_bra`: Short code fragment that saves registers on the stack, then copies 1332 to d1, then subtracts 1 from d1, then loads the address of a fixed memory address into a0, then copies an address to d0, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_move_subi_lea_nop_movem_rts`: Short code fragment that copies 0 to a fixed memory address, then copies 1332 to d1, then subtracts 1 from d1, then loads the address of a fixed memory address into a0, then pauses for one no-operation step (4 times in a row), then restores saved registers from the stack, then returns.
- `update_best_score_for_level`: Validates a level and slot index, compares the stored score against the current best, updates the best-score table and triggers score display routines when a new record is set.
- `fill_animation_record_from_source`: Copies a source record's word, byte and offset fields into an animation structure, sets frame count and delay, computes a pointer from a table plus offset, and returns whether that pointer is nonzero.
- `advance_script_pointer_and_play_voice`: Reads four bytes from the current script pointer, subtracts 65 from each and passes them with voice numbers 0 to 3 to a sound routine, then advances the script index and pointer.
- `free_four_audio_channels_and_buffers`: Calls a release routine for each of four audio channel pointers, then frees the sample buffer, the second buffer and the third buffer via the library free call and clears their pointers.
- `open_four_audio_channels`: Opens up to four audio channels with allocation flags, stores each channel pointer, sets the channel enable bit, attaches a sample buffer and sets the repeat count, returning success or failure.
- `release_four_slot_objects`: Loops over four pointer slots in the a4-relative table, calls cleanup routines on each object's fields, frees the object via a library call and clears the slot.
- `load_level_data_from_file`: Opens a file, reads header, tile and object tables into allocated memory, sets up pointers and counters in the a4 data area, then closes the file.
- `frag_link_move_bra`: Short code fragment that creates a stack frame in a5, then copies the memory at an offset from a5 to the memory at an offset from a5, then jumps to another point in the code.
- `iterate_handler_table`: The routine loops from a start index up to a limit stored in a structure, calling another routine for each table entry indexed by the current counter.
- `dispatch_handler_by_index`: The routine calls another routine with a table entry selected by an index derived from the parameter.
- `check_char_and_return_code`: The routine examines two short values from its parameter, masks and adds them, checks a byte at a computed offset in a global table, and returns a 24-bit value formed by a small code and a value taken from the stack.
- `frag_move_bra`: Short code fragment that copies 1 to d0, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 2 to d0, then jumps to another point in the code.
- `frag_move_movem_unlk_tst_rts`: Short code fragment that copies 3 to d0, then restores saved registers from the stack, then removes the stack frame held in a5, then tests d0, then returns.
- `parse_and_accumulate_values_2`: The routine walks a table of short values, invoking another routine for each entry and adjusting an accumulator based on the result.
- `parse_accumulate_values`: Parses table values and accumulates results by comparing byte values against conditions and updating global flags based on comparison outcomes.
- `parse_table_and_accumulate_2_2`: The routine walks a table of short values, calling another routine for each entry and adjusting an accumulator based on the result.
- `parse_table_and_accumulate_2`: The routine walks a table of short values, calling another routine for each entry and adjusting an accumulator based on the result.
- `frag_tst_beq_cmp_ble_bgt_movem_unlk_clr`: Short code fragment that tests d5, then branches if the last result was zero, then compares d1 with 32, then branches if the comparison was less or equal, then compares d1 with 44, then branches if the comparison was greater, then restores saved registers from the stack, then removes the stack frame held in a5, then clears d0, then returns.
- `parse_and_accumulate_values`: The routine loops over a table of short values, calling another routine for each entry and adjusting an accumulator based on the result.
- `frag_link_movem_move_bra`: Short code fragment that creates a stack frame in a5, then saves registers on the stack, then copies the memory at an offset from a5 to d0, then jumps to another point in the code.
- `frag_link_movem_move_ext_bra`: Short code fragment that creates a stack frame in a5, then saves registers on the stack, then copies the memory at an offset from a5 to d0, then sign-extends d0, then jumps to another point in the code.
- `format_integer_to_string`: The routine writes a sign and decimal digits of a signed integer through a pointer parameter using a temporary stack buffer and returns an 8-byte value taken from the stack.
- `write_bytes_and_return_stack_2`: The routine writes a constant byte and a zero byte through a pointer parameter and returns an 8-byte value taken from the stack.
- `frag_move_movem_unlk_rts`: Short code fragment that copies 48 to the memory at a0 (advancing a0), then copies 0 to the memory at a0, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `decode_three_bytes_to_word`: The routine builds a 24-bit value from the first three bytes of a parameter buffer using bit masks and shifts, then returns it concatenated with a register value.
- `encode_data_bytes`: Encodes a 24-bit value into three bytes stored at consecutive memory locations using bit shifting and masking operations.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data, not executable code.
- `store_constant_to_abs_24`: Branches away to another routine, then stores the immediate value into absolute memory address 36 and returns.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `parse_ascii_signed_integer_2_2`: The routine skips leading spaces and tabs, optionally handles a sign, then accumulates decimal digits from a pointer using a table lookup, optionally negates the result, and returns the parsed short value.
- `parse_ascii_signed_integer_2`: The routine skips an optional plus sign, accumulates decimal digits from a pointer using a table lookup, optionally negates the result, and returns the short value.
- `frag_cmpi_bne_addq_moveq_bra`: Short code fragment that compares the memory at a2 with 43, then branches if the last result was not zero, then adds 1 to a2, then loads 0 into d4, then jumps to another point in the code.
- `convert_ascii_to_signed`: The routine converts an ASCII digit string pointed to by a pointer into a numeric value by accumulating digits.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `frag_move_bra_3`: Short code fragment that copies 32767 to d0, then jumps to another point in the code.
- `compare_byte_strings`: The routine compares two byte strings for up to a given length, returning all ones, one, or zero depending on the ordering of the first differing bytes.
- `frag_bls_moveq_rts`: Short code fragment that branches if the comparison was lower or same (unsigned), then loads 1 into d0, then returns.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `save_regs_and_jump_via_stack`: Pops the return address into a0, pushes d2-d3 and a4 onto the stack, pushes a0, then jumps to a fixed target.
- `restore_regs_and_jump_return`: Pops the return address into a0, restores d2-d3 and a4 from the stack, then jumps to that saved address.
- `load_a4_global_base_pointer`: Loads the fixed global data base address into a4 and returns immediately, giving callers the shared a4-relative data pointer.
- `allocate_and_init_manx_runtime`: Allocates a runtime block, writes the MANX signature, sets up stack and heap fields, opens the runtime library and calls the exit handler.
- `copy_d0_to_d5`: Copies the longword in d0 into d5 and then falls through into the following routine.
- `parse_config_text_into_argv`: Allocates a buffer, copies a source string, splits it into whitespace-separated tokens honoring double quotes, stores pointers in an allocated array and returns the token count in a global.
- `load_7fff_into_d0_and_branch`: Moves d0 to itself, loads the immediate value 32767 into d0, then branches into the following routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_with_zeros`: Copies bytes from the source to the destination until a null or the count is reached, then fills the remaining count bytes with zeros and returns the destination pointer in d0.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `load_window_icon_and_attach_to_object`: Opens the WINDOW library, fetches an icon via icon.library, stores the icon data and two sub-objects into fields of the object at a2, then closes the library.
- `signed_divide_longwords`: Performs a signed division by calling an unnamed unsigned routine and negating the result when the operand signs differ.
- `signed_divide_with_sign_restore`: Saves d1 and d4, negates d0 and d1 if negative while tracking the sign in d4, calls the divide routine, moves the quotient into d0 and branches back to the shared sign-fixup code.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `frag_move_bsr_tst_rts`: Short code fragment that copies d1 to the stack, then calls another routine, then copies the top of the stack to d1, then tests d0, then returns.
- `unsigned_32bit_divide_by_d1`: Saves d2-d3, divides the 32-bit value in d0 by d1 using a 16-step shift-subtract loop when the high word of d1 is nonzero, otherwise uses two divu instructions, returning quotient in d0 and remainder in d1.
- `unsigned_32bit_divide_shift_subtract`: Saves d2-d3, performs a 16-iteration shift-subtract division of d0 by d1, leaving quotient in d0 and remainder in d1, then restores registers and returns; trailing bytes are a jump and dos.Close call.
- `dispatch_message_loop_2`: The routine checks a structure field and repeatedly calls another routine a number of times derived from the structure, then invokes several.
- `dispatch_message_loop`: The routine checks a structure field and repeatedly calls another routine a number of times derived from the structure, then invokes several other routines based on fields in the structure.
- `process_message_dispatch`: The routine repeatedly calls another routine while a counter is less than a value from a structure, then invokes several other routines based on fields in the structure and performs library calls via vectors.
- `dispatch_message_handlers`: The routine checks two fields in a structure pointed to by an argument and calls another routine with values taken from those fields, or calls other routines with different arguments depending on a flag.
- `call_library_vectors_and_return`: The routine calls another routine, then calls a library function via a vector at offset minus 132 and another via a vector at offset minus 378 using a value from a global variable, and returns a value from a global variable.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `close_handle_if_active_2`: The routine checks a flag byte in a structure and calls another routine with the structure pointer if the flag is clear, clears the structure.
- `frag_btst_bne_move_jsr_addq_clr_moveq_b`: Short code fragment that tests bit 7 of the memory at an offset from a2, then branches if the last result was not zero, then copies the memory at a2 to the stack, then calls another routine, then adds 4 to a7, then clears the memory at a2, then loads 0 into d0, then jumps to another point in the code (2 times in a row).
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_ex_next`: Wrapper that calls the dos library function ExNext and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `jump_to_divide_entry_point`: A single unconditional jump that transfers execution to the nearby division routine entry, reached only as a gap decoded as code.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `dos_seek_read_output_thunks`: Loads the DOS library base from a4 offset and jumps through three library vectors for Seek, Read and Output, restoring d1-d3 from the stack between calls.
- `jump_to_dos_vector_thunk`: A single jump instruction that transfers control to the next instruction, which then dispatches through a DOS library vector at a4 offset.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `frag_jmp_3`: Short code fragment that jumps to another point in the code.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `dos_library_call_thunks`: Loads a1 from the stack and the library base from a4, then jumps through two library vectors, restoring d0 and a1 from the stack between them.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `jump_through_a6_library_offsets`: Loads the return address from the stack into a1, fetches the library base from offset 13548 of a4 into a6, then jumps through two different negative offsets of that library vector table.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_init_message_node`: Allocates a message port with signal flag and size 34, fills its fields from the arguments, opens a reply port, and returns the new node pointer or zero on failure.
- `check_flag_and_invoke_callback`: Checks a flag in the passed structure and recurses into itself when it is set, then writes a byte and a word to other fields and finally invokes a stored callback with a constant argument.
- `call_library_vector_354_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `jump_exec_library_offset_330`: Loads the longword at offset 4 of the stack into d0, fetches the library base from a4 offset 13548 into a6, then jumps to the library vector at offset -330.
- `jump_exec_library_offset_198`: Loads two longwords from the stack into d0 and d1, fetches the library base from a4 offset 13548 into a6, then jumps to the library vector at offset -198.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `jump_to_next_instruction`: An empty routine that jumps to the immediately following instruction, effectively returning without touching any registers or memory.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `init_self_referential_struct_2`: Loads the first stack argument into a0, stores a0 at (a0), adds four to that longword, clears the longword at offset four, stores a0 at offset eight and returns.
- `jump_library_offset_with_args`: Loads the first stack argument into a0, moves the next two arguments into d0/a1 and d1, then jumps through the library vector at offset minus four hundred forty-four in a6.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `jump_library_vector_minus_78`: Loads the first stack argument into a0, fetches the library base from a4 offset and jumps through the vector at offset minus seventy-eight in a6.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_runtime_entry_c226a2`: Performs a single jump to the runtime entry point at the start of the loaded segment, transferring control without touching registers or memory.
- `jump_to_runtime_entry_c23c56`: Performs a single jump to a runtime entry point in the loaded segment, transferring control without modifying registers or memory.
- `jump_to_runtime_entry_c23d6c`: Performs a single jump to a runtime entry point in the loaded segment, passing control onward without altering registers or memory.
- `jump_to_sound_channel_lookup`: Jumps to the routine at the fixed target address, which searches three sound entries for a matching id and stores a value in the audio state.
- `jump_to_swap_two_audio_words`: Jumps to the fixed target address, which swaps two longwords in the audio state area and passes them to another routine.
- `frag_jmp_4`: Short code fragment that jumps to another point in the code.
- `jump_to_allocate_eight_buffers`: Jumps to the fixed target address, which allocates eight buffers of size 36866 and stores their pointers in the audio state area.
- `jump_to_clear_eight_sound_slots`: Jumps to code that loops over eight entries in an A4-relative table and calls a routine with each entry and value 144, clearing sound slots.
- `jump_to_mouse_pointer_update`: Jumps to code that updates mouse pointer position globals, clamps them to screen bounds, and handles button state changes.
- `jump_to_store_pointer_coords_set_delay`: Jumps to code that stores the low and high bytes of a global into two pointer coordinate words and sets a delay value of 10.
- `jump_to_shared_return_path`: Jumps to a shared code location, transferring execution there without touching registers or memory itself.
- `frag_jmp_5`: Short code fragment that jumps to another point in the code.
- `frag_jmp_6`: Short code fragment that jumps to another point in the code.
- `frag_jmp_7`: Short code fragment that jumps to another point in the code.
- `frag_jmp_8`: Short code fragment that jumps to another point in the code.
- `jump_to_free_four_audio_channels_and_b`: Jump stub: passes control straight on to free_four_audio_channels_and_buffers.
- `jump_to_release_four_slot_objects`: Jump stub: passes control straight on to release_four_slot_objects.
- `frag_jmp_9`: Short code fragment that jumps to another point in the code.
- `frag_jmp_10`: Short code fragment that jumps to another point in the code.
- `frag_jmp_11`: Short code fragment that jumps to another point in the code.
- `frag_jmp_12`: Short code fragment that jumps to another point in the code.
- `frag_jmp_13`: Short code fragment that jumps to another point in the code.
- `jump_to_external_entry_point`: Jumps to an external code location reached from outside static flow, transferring execution there without touching registers or memory itself.
- `jump_to_unreached_target`: Jumps to a code location in an unreached gap, transferring execution there without touching registers or memory itself.
- `frag_jmp_14`: Short code fragment that jumps to another point in the code.
- `frag_jmp_15`: Short code fragment that jumps to another point in the code.
- `frag_jmp_16`: Short code fragment that jumps to another point in the code.
- `frag_jmp_17`: Short code fragment that jumps to another point in the code.
- `jump_to_restore_regs_and_jump_return`: Jump stub: passes control straight on to restore_regs_and_jump_return.
- `jump_to_load_7fff_into_d0_and_branch`: Jump stub: passes control straight on to load_7fff_into_d0_and_branch.
- `jump_to_multiply_16bit_halves_to_32bit`: Jump stub: passes control straight on to multiply_16bit_halves_to_32bit.
- `jump_to_signed_multiply_with_sign_rest`: Jump stub: passes control straight on to signed_multiply_with_sign_restore.
- `frag_jmp_18`: Short code fragment that jumps to another point in the code.
- `jump_via_a4_offset_table_entry`: Loads a pointer from the table at offset 13552 of a4 and jumps to the routine at minus 36 bytes from that pointer.
- `jump_via_a4_table_minus_198`: Loads a pointer from the table at offset 13552 of a4 and jumps to the routine at minus 198 bytes from that pointer.
- `jump_via_a4_table_minus_30`: Loads a pointer from the table at offset 13552 of a4 and jumps to the routine at minus 30 bytes from that pointer.
- `jump_via_a4_offset_34f0_minus_2a`: Loads the pointer at offset 13552 from the a4 base, then jumps to the code pointer stored at minus 42 from that table entry.
- `jump_via_a4_offset_34f0_minus_42`: Loads the pointer at offset 13552 from the a4 base, then jumps to the code pointer stored at minus 66 from that table entry.
- `jump_via_a4_offset_34ec_minus_a8`: Loads the pointer at offset 13548 from the a4 base, then jumps to the code pointer stored at minus 168 from that table entry.
- `jump_via_a4_offset_table_entry_2`: Loads a pointer from the table at offset 13548 of a4 and jumps to the routine at offset -414 within it.
- `jump_via_a4_table_minus_c6`: Loads a pointer from the table at offset 13548 of a4 and jumps to the routine at offset -198 within it.
- `jump_via_a4_table_minus_126`: Loads a pointer from the table at offset 13548 of a4 and jumps to the routine at offset -294 within it.
- `jump_via_a4_offset_table_entry_3`: Loads a pointer from the table at offset 13548 of a4 and jumps to the entry at minus 210 bytes from it.
- `jump_via_a4_table_minus_552`: Loads a pointer from the table at offset 13548 of a4 and jumps to the entry at minus 552 bytes from it.
- `jump_via_a4_table_minus_174`: Loads a pointer from the table at offset 13548 of a4 and jumps to the entry at minus 174 bytes from it.
- `frag_jmp_19`: Short code fragment that jumps to another point in the code.
- `jump_via_a4_dispatch_table`: Loads a function pointer from a table indexed through the a4-relative pointer at offset minus 26276 and jumps to it, transferring control to the dispatched routine.

### Populous (1989)(Electronic Arts)[cr DCW][docs].adf:populous.prg#unpacked

- `initialize_table_entries`: Processes a series of table entries by calculating offsets and adding values to memory locations, with multiple loops iterating through different data ranges.
- `process_data_structure`: Handles data processing within a structured format, performing calculations on indices and copying data between memory regions based on computed offsets.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `frag_move_andi_lsr_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 255, then copies d0 to a fixed memory address, then copies a fixed memory address to d0, then copies 8 to d1, then shifts d0 right by d1, then copies d0 to a fixed memory address, then copies 0 to d0, then copies 10 to a fixed memory address, then returns.
- `initialize_dma_control_block_2`: The routine writes a fixed sequence of long words to a block pointed to by a0, then fills a short table with a repeating pattern derived from parameters.
- `initialize_dma_control_block`: The routine writes a fixed sequence of long words to a block pointed to by a0, then fills a short table with a repeating pattern derived from a1 and d1, and finally writes sentinel values and stores a constant into a global variable.
- `write_dma_control`: The routine writes to DMACON with bit 15 set to enable DMA channels and then returns.
- `start_copper_list_2`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `wait_for_copper_interrupt`: The routine stores a parameter-derived value into a global variable and then loops, recombining the parameter and the global variable and masking the result, until the global variable has bit five set.
- `acknowledge_vblank_interrupt_2`: Writes 32 to INTREQ to clear the vertical blank interrupt, then polls INTREQR bit 5 until that interrupt request flag becomes set again.
- `copy_data_with_bounds_check_2`: Searches for a specific word value in a buffer, then copies thirty-one words from a second buffer into the location following that value.
- `initialize_data_structure`: Initializes a data structure by copying 16 longwords from a source table to a destination, with specific offsets and a loop counter.
- `initialize_struct_from_args_2`: The routine is 26 instructions long, is entered via a call and a pointer in data, is called by one unnamed routine, and calls a routine that clears two words in a structure pointed to by a1 and copies the high word of an argument and the low word of the same argument.
- `initialize_struct_from_args`: The routine clears two words in a structure pointed to by a1, then copies the high word of an argument and the low word of the same argument into four locations of a structure pointed to by a0.
- `populate_data_block`: Copies data from a table into a series of memory blocks with specific offsets and a loop that processes 16 iterations.
- `update_byte_buffer`: Indexes a table with a parameter to obtain a byte buffer pointer, then writes several bytes into that buffer derived from the two parameters and a global variable, including conditional flag bits.
- `process_byte_sequence`: Processes a sequence of bytes by calculating offsets into a lookup table, applying bitmask operations to memory blocks, and handling special cases like line feeds.
- `apply_bitmask_operations`: Applies bitmask operations on data blocks by reading from a lookup table, performing AND/OR operations with source data, and updating multiple memory locations in sequence.
- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
