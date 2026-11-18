# Game 1262 (red storm rising v2 (1990)(microprose)[cr skr][h lsd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01262). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (928516 top-level bytes in 64 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 39 of 39 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 1259 of 1259 named (0 author symbols), 1259 of 1259 described (390 by rule, 137 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5370 of 5370 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 97680 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 46326 of 46378 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 115811 |
| code | 93242 |
| data | 702717 |
| padding | 15210 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm3.fnt | 1 | 2 | 1 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:more | 19 | 2321 | 79 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm1.fnt | 1 | 2 | 1 |
| run | 2 | 458 | 12 |
| format | 2 | 267 | 8 |
| ask | 2 | 183 | 5 |
| merge | 2 | 435 | 7 |
| resident | 2 | 171 | 4 |
| assign | 2 | 435 | 7 |
| LSD1992 | 2 | 267 | 8 |
| EndCLI | 2 | 182 | 3 |
| newcli | 2 | 388 | 9 |
| echo | 2 | 241 | 10 |
| lab | 1 | 2 | 1 |
| skip | 2 | 117 | 2 |
| execute | 2 | 117 | 2 |
| copy | 2 | 267 | 8 |
| makedir | 2 | 211 | 4 |
| endif | 1 | 2 | 1 |
| if | 2 | 413 | 15 |
| iconx | 12 | 806 | 37 |
| LoadWB | 12 | 537 | 30 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStart | 2 | 267 | 8 |
| disk-validator | 1 | 386 | 1 |
| SkidRow | 2 | 24 | 4 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Title | 2 | 6944 | 231 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm | 2 | 267 | 8 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm2.fnt | 1 | 2 | 1 |
| icon.library | 10 | 1421 | 110 |
| info.library | 25 | 3001 | 181 |
| diskfont.library | 15 | 1044 | 79 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Storm | 2 | 267 | 8 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Red | 2 | 267 | 8 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4051 | 161 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3045 | 1 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm#unpacked | 3 | 4794 | 202 |
| Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:sounds (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:ships: high-entropy data file (7.86 bits/byte), classified as packed data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:osg: high-entropy data file (7.91 bits/byte), classified as packed data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:pics: high-entropy data file (7.94 bits/byte), classified as packed data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:opening: high-entropy data file (7.92 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 137 64 KB pages each with >= 29491 distinct PCs executed, 2239956 of its 2242320 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 126 64 KB pages each with >= 29491 distinct PCs executed, 2060755 of its 2063112 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 126 64 KB pages each with >= 29491 distinct PCs executed, 2060755 of its 2063112 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 126 64 KB pages each with >= 29491 distinct PCs executed, 2060755 of its 2063112 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 126 64 KB pages each with >= 29491 distinct PCs executed, 2060755 of its 2063112 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 1334 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3045 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:c/format: PowerPacker 4.0 (xfd) stream found by ancient scan (7164 -> 13280 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:c/LSD1992: PowerPacker 4.0 (xfd) stream found by ancient scan (20660 -> 33188 bytes) but only 0 of 15 sampled windows are in reference RAM: not used
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:c/copy: PowerPacker 4.0 (xfd) stream found by ancient scan (6068 -> 9848 bytes) but only 0 of 5 sampled windows are in reference RAM: not used
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStart: PowerPacker 4.0 (xfd) stream found by ancient scan (52840 -> 83512 bytes) but only 4 of 37 sampled windows are in reference RAM: not used
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 12388 -> 29948 bytes), 10 of 11 windows confirmed in reference RAM: recovered as a hunk executable
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Storm: PowerPacker 4.0 (xfd) stream found by ancient scan (58024 -> 111208 bytes) but only 8 of 52 sampled windows are in reference RAM: not used
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Red: PowerPacker 4.0 (xfd) stream found by ancient scan (77816 -> 125232 bytes) but only 7 of 56 sampled windows are in reference RAM: not used
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:sounds: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:c/format: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:c/LSD1992: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:c/copy: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStart: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Storm: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- red storm rising v2 (1990)(microprose)[cr skr][h lsd]/Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Red: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 97680 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- SLIDEOK (fix-r4, 19 Nov 2026): the reference run slide was redone and slid again; its PCs are excluded and the 5370 clean executed game-code addresses pass proof 4, so the slide no longer blocks R (it still needs the G0 harness fix): reference run failed (Karl 4 Nov 2026): run ref slid through memory (an address: 63 64 KB pages each with >= 29491 distinct PCs executed, 1028563 of its 1034000 PCs), a runaway, not game code; its slide PCs are excluded from executed game code and the game needs a fresh reference run

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_message_2` (more): A complex routine that performs multiple tasks including calling library functions, processing entity lists, handling search loops, drawing text and windows, parsing signed decimals, and performing 32-bit integer multiplication. Kind: code-wheel lookup. Pass path: call sites and instructions right after each call: [].

## Functions


### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm3.fnt

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:more

- `parse_and_load_config_file`: Allocates memory for a structure and calls a routine to load file data, then parses the loaded buffer as a string to extract tokens, handling quoted strings and converting specific character sequences into newline or escape characters.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_and_initialize_file_data`: Parses a file header to populate a structure with metadata and then loads associated data into memory.
- `iterate_table_with_function`: Iterates through a table of four values, passing a pointer from a structure and each value from the table to another routine.
- `validate_input_buffer`: Calls another routine twice with arguments derived from an input pointer and then scans a range of bytes to check if they fall within certain allowed ranges.
- `process_entity_command_dispatch`: Sets a 32-bit field at offset 60 of a structure to 1 and enters a loop that calls another routine until it returns a non-zero value.
- `conditionally_dispatch_and_update`: Reads a 32-bit value at offset 60 of the structure in a2 and, when it is greater than zero, calls the flag-based dispatch routine and then the entity update routine.
- `process_message_2`: A complex routine that performs multiple tasks including calling library functions, processing entity lists, handling search loops, drawing text and windows, parsing signed decimals, and performing 32-bit integer multiplication.
- `initialize_entity_state`: Sets a 32-bit field at offset 60 and a 16-bit field at offset 94 of a structure to specific values, calls another routine with a pointer from offset 52, calls itself recursively if offset 116 is not 4096 and offset 120 is non-zero, and enters a loop that calls another routine while certain conditions regarding offset 60, offset 8 of a pointed-to structure, and offset 94 are met.
- `update_entity_and_increment_counter`: Calls another routine twice with arguments derived from the input pointer, stores the second result into a field of the pointed-to structure, and if a flag field at offset 4 is zero it increments a short located at an offset from a pointer stored in the structure.
- `parse_buffer_content`: Uses another routine to check a buffer, updates a size field, and scans for a newline or form-feed character while potentially modifying a flag or string content.
- `process_entity_list`: Determines a loop limit based on fields in a passed structure and iterates to call another routine using a pointer to a table and a global variable; it also recursively calls itself, calls another routine with a pointer to a stack buffer, and calls a library routine to reschedule.
- `handle_search_loop`: Uses a flag in an argument block to select a global variable and repeatedly calls another routine, updating a counter and looping while a search result is zero.
- `initialize_handler`: Calls another routine with zero, then with a parameter, then with a pointer from offset 16 of a structure, a global variable, and a parameter, conditionally calls another routine with the same arguments but without the parameter, calls another routine with two arguments, calls another routine with a constant, calls another routine with a parameter, and returns the result of the call involving two arguments.
- `check_flag_and_call`: Calls another routine with two arguments, then checks a short value at an offset from a pointer and conditionally calls another routine with one argument.
- `draw_scroll_window_bounds`: The routine calls a drawing routine using a pointer and a certain value, then calls another routine using a value or offset from a pointer-based structure.
- `draw_text_window_range`: The routine calls a drawing routine with a fixed or zeroed argument, then calls another routine with a decremented value derived from a pointer-based structure.
- `draw_text_line`: Uses a handler pointer to call drawing primitives while iterating over a text buffer, tracking line count and stopping if a certain character or flag is encountered.
- `dispatch_recursive_pointer`: The routine calls another routine with a single argument, then calls another routine using a pointer from a structure and a global variable.
- `set_flag_and_forward`: Checks a field in a passed structure, optionally sets a flag in a remote structure, and calls another routine using a global variable.
- `dispatch_by_flag`: Selects two pointers based on a flag in a parameter block and calls another routine using either a global variable or a table entry.
- `dispatch_handler_by_flag`: Reads a flag from a parameter block and calls another routine with either three references to a global variable or a value from a table entry.
- `clamp_negative_result`: The routine calls another routine twice with the same first argument and different third arguments, then clamps a negative return value to zero before returning it.
- `consume_from_buffer`: Checks a size field in a structure and, if a certain condition or partial way is met, updates several offset or remaining size fields and calls another routine with a pointer derived from the structure.
- `recursively_process_buffer_segments`: If a field at offset 120 is non-zero and a field at offset 116 is not 4096, it calls itself recursively with a pointer into the structure and a size from offset 120.
- `update_scroll_window`: Calculates a target position based on provided offsets and compares it against the current window bounds.
- `read_line_from_buffer`: Repeatedly calls another routine to read bytes from a source buffer into a destination buffer, stopping at a newline or a fixed limit, and appends a null terminator.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `convert_string_to_uppercase`: Iterates through a null-terminated string and converts any lowercase ASCII letters to uppercase by subtracting a constant.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_library_function_with_mask`: Checks if a pointer in a structure or table is non-null, then calls a library function using a base address from a global variable and a provided mask.
- `process_message`: Recursively processes a structure by invoking itself on various fields and string arguments, conditionally updates fields at offsets 8, 6, and 7, and performs a loop based on a return value.
- `build_message_buffer`: The routine obtains a pointer from another routine and then calls another routine with that pointer, a constant, a stack-allocated buffer containing the input parameter, and a size of one.
- `copy_entity_data_2`: The routine obtains a handle from another routine and, if valid, reads a value from that handle, passes it to another routine together with a small stack buffer, and copies two fields from the handle into the caller-provided structure.
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
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `extract_byte_from_call`: Calls another routine with a pointer to a small stack buffer and a constant, returning a byte from that buffer if the call returns certain a value, otherwise returning negative one.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `modular_inverse_32bit`: The routine performs a 32-bit modular reduction or a 16-bit modulo operation depending on the size of an input value, using repeated doubling and subtraction, and returns a combined 32-bit result.
- `multiply_32_bit_integers`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `call_helper_return_register`: Saves d2 on the stack, copies d1 to d2 and d0 to d1, calls the modular computation routine, then returns the value left in d1 as d0 after restoring d2.
- `negate_result_on_flag`: Calls the modular computation routine and then negates its 32-bit result when the 16-bit value left in a0 is nonzero, returning the result in d0.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `allocate_message_entry`: Attempts to allocate a message buffer, and if successful, populates several fields of that buffer with parameters and a value from another routine before dispatching it.
- `delete_port`: Removes the port from a global list if a specific field is non-zero, sets status fields to indicate deletion, frees the associated signal bit, and releases the memory block.
- `compare_strings_with_wrap`: Compares two null-terminated strings byte by byte, skipping the check if the first string contains a newline character.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm1.fnt

- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `copy_record_and_clear_slot_2`: Copies a block of bytes from the buffer in a0 into a record at a computed offset, writes a marker byte of 10 after the copied data, and then records the slot value at a fixed offset of the record.
- `copy_record_and_clear_slot`: Copies a block of bytes from a source buffer in a0 into a record at a computed offset, writes a marker byte of 10 after the copied data, and initialises a table entry to 4 with fields copied from a second table entry.
- `fill_slot_entry_record`: Stores a byte value of 10 through a computed pointer derived from the structure in a1, writes the constant 4 to a table entry indexed by a longword at offset 52 of that structure, and copies two longwords from the table entry indexed by the value at offset 56 into the following slots.
- `initialises_slot_table_entries`: Stores a byte value of 10 through a computed pointer from the structure in a1, then writes a fixed value of 4 and copies two longwords from the slot table in a0 into the next slot, clearing a further longword and loading a value from offset 68 of the structure.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `clamp_flag_and_dispatch`: Sets d1 to 128 and tests it against the longword at offset 48 of the structure in a1; if the masked result is zero it branches, otherwise it sets the upper 24 bits of that field.
- `update_slot_and_dispatch_handlers`: Copies the longword at offset 64 of the structure in a1 into offset 108 of the same structure, then calls a routine through a5 and stores its result at offset 28.
- `reset_slot_and_call_handlers`: Writes the longword at offset 28 of the structure in a1 into an indexed slot in the table at a0, then makes several indirect calls through a5, zeroes fields 4 to 7 of the structure and sets fields 23 to 31 from neighbouring fields and a table pointer, and conditionally clears a further table slot and decrements a counter.
- `clear_indexed_slot_and_decrement`: Calls the routine in a5 twice, stores d1 and the longword at a1 into the structure in a1, conditionally calls the routine in a5 when a flag is zero, clears a table entry indexed by fields in a1, decrements a counter in a table when it is nonzero and not minus one, calls the routine in a5 twice more, and transfers control through a jump table.

### format

- `process_hunk_headers_5`: Reads and processes Amiga hunk headers from memory, allocating memory blocks for each hunk and copying data into them.
- `loop_until_zero`: Repeatedly decrements a word value in a register until it reaches zero, performing no other operations during the loop iterations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `parse_and_shift_bits`: Loads a long value from memory, tests if the loaded value is zero and branches if so, calls another routine to process bits, performs bit shifting operations on data values, updates a pointer, calls another routine multiple times with different parameters, and returns when conditions are met.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_accumulate`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution based on shift values.
- `load_and_execute_trap`: Loads a word from memory at a calculated offset, performs a bitwise OR operation with a constant, executes a trap instruction, and combines results from multiple registers.

### ask

- `find_max_and_prepare_buffer_9`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initializes_pointers_then_dispatches`: Stores a shifted pointer into the word at offset 0 of the structure in a1 and another into the longword at offset 208, then clears the longword at offset 372.
- `conditionally_stores_five_and_calls`: Tests a longword in a table indexed by a value at offset 328 of the structure in a1; when it is nonzero, it repeatedly calls through the pointer in a5 until d1 is zero, then stores 5 at offset 588 of that structure if a second table entry is nonzero.
- `stores_flag_and_calls_three`: Tests a longword in a table indexed by a value at offset 328 of the structure in a1 and, when it is nonzero, stores 5 at offset 588 of that structure, then makes three indirect calls through a5 and a jump-table address, and returns.

### merge

- `find_max_and_prepare_buffer_10`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_3`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_structure_fields_3`: Sets multiple fields in a data structure including pointers and counters, clears specific status fields, and performs conditional operations based on pointer table lookups.
- `scan_and_shift_pointer_table`: Checks a status field in a structure and, when it is clear, loads a counter-indexed entry from a pointer table and repeatedly copies longword pointer slots toward the next index through a helper routine, stopping when a slot is zero.
- `process_indexed_entry_chain`: Repeatedly loads a pointer-table entry indexed by a word counter held in a structure, copies pointer slots within a table at 4-byte stride, and conditionally sets a field to 20 or clears a flag, looping back through a jump-table call until the chain terminates.
- `shift_pointer_slots_in_loop`: Repeatedly copies longword pointer slots within a table, moving the entry at the index held in a counter field into the slot at the next index, and loops while the copied entries are non-zero.
- `rotates_pointer_slots_and_calls`: Copies a longword from one slot of a pointer table in a1 into another slot indexed from offset 492, stores 1 and the moved value into neighbouring slots, clears the source field, then makes several indirect calls through a5 that test the flags at offsets 484, 488, 492 and 500 and clear longword fields at offsets 476 and 492 when a result is nonzero, ending with an indirect jump.

### resident

- `find_max_and_prepare_buffer_11`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_4`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `stores_flags_and_chains_calls`: Stores a computed pointer at offset 4 of the structure in a1, then stores results of calls through a5 into several fields of that structure at offsets 236, 240, 244 and 276.
- `frag_moveq_jmp`: Short code fragment that loads 0 into d1, then jumps to the address in a6.

### assign

- `find_max_and_prepare_buffer_12`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_5`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `scan_pointer_table_entries`: Clears bits in a longword at unaff_A4 and stores several values into the structure at A1 (a shifted pointer, counters set to 0 and -1, and a field set to 20).
- `scan_and_shift_pointer_table_2`: Checks a status field in a structure and, when it is clear, loads a counter-indexed entry from a pointer table and repeatedly copies longword pointer slots toward the next index through a helper routine, stopping when a slot is zero.
- `shift_pointer_slots`: Copies longword pointer values between slots in a table, shifting entries based on index fields and updating related counters.
- `rotate_pointer_slots_and_calls`: Copies a longword from one slot of a pointer table into another slot indexed by offset 492, stores 1 and the moved value into neighboring slots, then calls a routine based on the moved value.
- `reassign_slot_entries`: Reorganizes slot entries by moving data between arrays based on index values, clears source indices, and performs multiple conditional operations with indexed array accesses.

### LSD1992

- `process_hunk_headers_6`: Reads and processes AmigaDOS hunk headers from a memory block, allocating memory for each hunk and copying data into the allocated blocks.
- `initialize_structure`: Allocates memory for a structure, loads file data into it, and parses the loaded buffer as a string to extract tokens.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `parse_and_shift_bits_copy`: Loads a long value from memory, tests if the loaded value is zero and branches if so, calls another routine to process bits, performs bit shifting operations on data values, updates a pointer, calls another routine multiple times with different parameters, and returns when conditions are met.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `rotate_bits_right`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution.
- `load_and_modify_data`: Loads a 16-bit value from a global variable at offset 1988, stores it in a register, modifies another global variable with an immediate byte value, and then triggers a system trap.

### EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_6`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `sets_status_and_stores_counters`: ORs the word at a2 with 29188, computes a shifted value from a1 plus d1 and stores it through a1, stores longwords into fields at offsets 48 and 52 of a1 and a0, resets two words at offsets 244 and 246 of a2 to 65535, decrements a counter in a table entry when it is positive, makes several indirect calls through a5, and ends with an indirect jump.

### newcli

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_7`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `copy_table_entries_with_offsets`: Stores a longword from the table in a0 into a structure in a1 at an offset derived from a count field, then loops over a range of indices copying longwords within the table at offsets taken from the structure, and calls a helper routine several times along the way.
- `update_slot_table_entry`: Reads a longword from offset 288 of the structure in a1 and stores it into a table in a0 at an index taken from offset 264, then decrements a counter in the same table when it is positive.
- `scan_slots_then_dispatch`: Stores a value derived from a constant plus a register into a slot of a table indexed by the structure in a1, then scans a loop over the table from index 1 upward, recording the first empty index at offset 276 of the structure in a1.
- `frag_moveq_add_bra_2`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `conditionally_store_slot_and_dispatch`: Tests the longword at offset 276 of the structure in a1; when it is zero it makes three indirect calls and otherwise raises the value at offset 264 to at least 800, copies offset 284 into offset 324, and stores the value in d1 into a table in a0 at a slot indexed by the fields at offsets 276 and 280, then performs further indirect calls and writes several fields of the structure.
- `stores_table_entries_and_calls`: Stores the longword at offset 260 of the structure in a1 into a table in a0 at a slot computed from two fields of a1, and writes zero or other values into fields at offsets 328 through 340 of a1 depending on the result in d1.
- `stores_queue_entries_and_calls`: Stores a longword from offset 260 of a1 into a table of slots in a0 indexed by two fields, copies register d1 into fields at offsets 332 and 336 and clears fields at offsets 324 and 328 of a1, computes and stores an address offset from a2, clears a table slot when a field is nonzero, decrements a table entry that is not -1, makes several indirect calls through a5, and ends with an indirect jump.

### echo

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_8`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `update_window_and_dispatch_callbacks`: Stores a shifted value from D1 into a table entry and then performs several indirect calls through A5.
- `compute_clamped_window_and_dispatch`: When the longword indexed from the pointer table in a1 is nonzero, it calls through a5 to store a result at offset 328 of the structure in a1, then derives a clamped window from the pointer-table entries and stores start, end and current values at offsets 328, 332 and 336 before a loop writes successive values to offset 344 and calls through a5 each pass.
- `frag_move_bra_9`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_slot_window_and_dispatch`: Sets a flag longword at offset 208 of the structure in a1 to 1 and computes a clamped window value from the longwords at offsets 216 and 212 of that structure, storing it at offsets 212 and 220.
- `frag_move_bra_10`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `copy_longword_and_loop_calls`: Copies the longword at offset 216 of the structure in a1 into offset 212 and then into offset 220, and reads the longword at offset 208 as a loop counter that repeatedly calls an indirect routine (through a5) and advances it while it does not exceed the value at offset 220.
- `conditionally_calls_indirect_twice`: Reads the longword at a1 as an index, shifts it left by 2, and tests the table entry in a0 at that index; when it is nonzero, the routine branches elsewhere, otherwise it loads 10 into d1 and 352 into d0, takes the pointer at offset 224 of a2 into a4, makes an indirect call through a5, and branches to a later point.
- `conditionally_calls_twice`: When the longword at offset 584 of the structure in a2 is nonzero, it makes two indirect calls through a5 and makes a third indirect call if d1 is nonzero, then ends with an indirect jump.

### lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### skip

- `find_max_and_prepare_buffer_5`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra_11`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### execute

- `find_max_and_prepare_buffer_6`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame.
- `frag_move_bra_12`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### copy

- `process_hunk_headers`: Reads and processes AmigaDOS hunk type codes from memory, allocates memory blocks based on the hunk data, and copies relocation information into the allocated blocks.
- `decrement_and_check`: Decrements a 16-bit value in a register until it reaches zero, then returns.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `decode_and_copy_data`: Decodes compressed data from a source buffer and copies it to a destination, managing pointer adjustments and conditional copying logic.
- `check_shifted_flag_3`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `rotate_and_accumulate`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution based on shift values.
- `load_and_process_data`: Loads a word from a memory location, performs a bitwise OR operation with an immediate value, triggers a system trap, and combines two registers through a bitwise OR operation.

### makedir

- `find_max_and_prepare_buffer_7`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, copies sixteen words between areas, installs a function pointer, and returns success or failure.
- `frag_move_bra_13`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_structure_fields_4_2`: Sets bits in a word at offset minus two from a4 and stores a shifted address value into the first longword of the structure in a1, then clears two longwords at offsets 136 and 140.
- `initialize_structure_fields_4`: Calls the routine in a5, stores d1 at offset 216 of a1, and when d1 is nonzero calls that routine three more times; it then calls the routine again, stores longwords at offsets 216 and 212 of a1, tests the field at offset 216 and stores 10 at offset 208 when it is zero, calls the routine twice more, and transfers control through a jump table.

### endif

- `tiny_eor_rts_2`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### if

- `find_max_and_prepare_buffer_8`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, copies sixteen words between areas, installs a function pointer, and returns success or failure.
- `frag_move_bra_14`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `init_structure_then_poll_input`: Writes the longword at offset 0 of the structure in a1 divided by four back into that same offset, clears the longwords at offsets 208, 216, 220, and 224 of that structure, and calls the input poll routine.
- `dispatch_if_table_entry_set`: Reads the longword at offset 0 of the structure in a1, scales it by four and uses it to index the table at offset 8 of a0; if that entry is nonzero and the longword at offset 224 of the structure in a2 is not less than 10, it sets d1 to -1 and branches to the common exit at another routine.
- `test_table_entry_then_dispatch`: Reads the longword at offset 0 of the structure in a1, scales it by four and tests the table entry at offset 12 of a0 indexed by that value.
- `evaluate_slot_flags_and_dispatch`: Reads several longword fields of the structure in a1, indexed by the value stored at its offset 0, and copies a nonzero entry from the table in a0 into the structure's word-sized slots at offsets 216 to 228.
- `evaluate_slot_flags_and_poll`: Tests several table entries and flags in the structure in a1 to choose a result value, stores it at offset 328 (decimal 328) of the same structure, and then, when that result is zero, sets a counter at offset 600 of the structure in a2 to 10 and loops while polling input into the structure, storing the returned values at offsets 360, 368, 368 and 364 and combining two of them into the flag word at offset 364.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `test_table_entry_then_compare`: Reads the longword at offset 0 of the structure in a1, scales it by four and tests the table entry at offset 24 of a0 indexed by that value.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `dispatch_on_longword_value`: Compares the longword at offset 356 of the structure in a1 against 1 and branches to one of two other routines if it equals 1 or is nonzero; otherwise it loads -1 into d1 and branches to another routine.
- `poll_input_and_store_state`: Tests a bit-flag word through a pointer and, when clear, clears a longword at offset 208 of the structure in a1 and stores 10 at offset 600 of the structure in a2, then calls the poll-input routine in a loop that stores its results into fields at offsets 360, 364, 368, 372, 376 and 380 of the structure in a1 while the offset-600 value is neither 10 nor -1 and the flag at offset 364 is zero.
- `poll_input_into_structure`: Clears a longword at offset 208 of the structure in a1 and sets a value at offset 600 of the structure in a2 to 10.
- `dispatches_queued_input_events`: Repeatedly calls through an indirect routine pointer, storing returned 32-bit values into two structures and a global variable, and tests bit-set flags to decide whether to finish with a final indirect call.
- `poll_input_until_nonstatus`: Repeatedly calls a routine through a5 and stores its returned longword at offset 600 of the structure in a2, looping while that value is 10 or -1 and running a status-handling sequence that stores values into the structures in a1 and a4; it returns through an indirect jump when the value is -1 or the status flag at offset 364 of the structure in a4 is set.

### iconx

- `parse_command_string`: Parses a command string by scanning for whitespace or quotes, performing character substitutions for specific escape sequences, and populating a structure with the resulting tokens.
- `iconx_cleanup_close_and_free`: Finds the current task, restores the saved stack pointer, closes the script file and dos.library, replies any pending message, frees the memory block and returns the exit code.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start by checking a parameter and a byte in a structure, then proceeds to allocate memory, parse decimal values, and perform various library calls including icon and exec functions.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a buffer, walks a linked list of entries, copies entries with name bytes above 31 into the buffer using various helper routines, performs string replacements, and returns a value.
- `concatenate_strings_and_call_helper`: Copies a null-terminated string into a destination buffer, appends a second null-terminated string to that buffer, and then calls another routine with the buffer pointer and the total length of the combined strings.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to another routine.
- `parse_signed_decimal_string_to_long`: Reads an optional plus or minus sign then decimal digits from the string, accumulating the value in d0 by repeated multiply-by-ten, negates if signed, and returns the long in d0.
- `call_supervisor_2`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr_2`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule_2`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule_2`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch_2`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception_2`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable_2`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry_2`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_string_via_rawdofmt_then_output`: Calls exec RawDoFmt with a format template and arguments into a 140-byte stack buffer, measures the resulting string length and passes it to the output routine.
- `tiny_move_rts_3`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry_2`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_4`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### LoadWB

- `parse_command_string_2`: Parses a command string by scanning for whitespace or quotes, performing character substitutions for specific escape sequences, and populating a structure with the resulting tokens.
- `iconx_cleanup_close_and_free_2`: Finds the current task, restores the saved stack pointer, closes the script file and dos.library, replies any pending message, frees the memory block and returns the exit code.
- `initialize_workbench_data`: Performs complex data initialization involving string comparisons, linked list construction, and multiple library function calls to manage system resources and memory.
- `compare_null_terminated_strings`: Compares two null-terminated strings byte by byte and returns minus one when the first is lower, one when it is higher, and zero when they match.
- `build_linked_list_from_table`: Walks a table of pointer and value pairs, allocating a fixed-size node for each entry through a memory allocation call and storing the looked-up value in it.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_dispatch_3`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_reschedule_3`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_other_routine_with_stack`: Calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `format_string_via_rawdofmt_then_output_2`: Calls exec RawDoFmt with a format template and arguments into a 140-byte stack buffer, measures the resulting string length and passes it to the output routine.
- `tiny_move_rts_5`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compare_version_string_prefix`: Compares a2 against the all-ones value, and when it differs, saves a0 and a1, checks the byte at offset 20 of a0 against 50, then compares bytes from a1 against a0 with a post-incrementing string comparison, branching back and forth through a short loop until the bytes differ or match.
- `fill_memory_with_bytes`: Writes the byte value 46 to offset 17 of the structure in a2, then writes the byte value 32 to offsets 18, 19, 20, and 21 of that same structure before branching.
- `compare_memory_blocks`: Compares a block of memory pointed to by a0 with a block of memory pointed to by a location relative to the program counter, returning if they are not equal or if a null terminator is reached.
- `frag_lea_movem_move_rts`: Short code fragment that loads the address of a location in the code into a2, then restores saved registers from the stack, then copies a location in the code to the stack, then returns.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStart

- `process_hunk_headers_2`: Processes Amiga hunk headers from a memory block, allocates memory for each hunk, and copies data into the allocated blocks.
- `decrement_counter`: Decrements a counter in register d1 until it reaches zero, using a loop that branches back to an earlier instruction when the counter is not zero.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `decode_and_copy_data_2`: Decodes compressed data from a source buffer and copies it to a destination, managing pointer adjustments and conditional copying logic.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `rotate_and_accumulate_2`: Rotates bits from a memory buffer into a global variable using a shift count from another global, accumulates results through bitwise operations, and handles bit rotation with carry propagation.
- `write_port_word`: Writes a word value to a memory-mapped port address and performs a bitwise OR operation with another register before executing a trap instruction.

### disk-validator

- `update_disk_validation_status`: Calculates a character representation of a value from a structure, writes it to a buffer, and performs a series of bitwise operations and calls to update a bitmask and status flags within several structures.

### SkidRow

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library and stores the resulting library handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Title

- `initialize_and_parse_environment`: Clears a memory block using a loop, calls a routine to initialize a library, and then parses a string by splitting it into an array of pointers based on whitespace or quoted segments.
- `release_system_resources`: Executes several function pointers and handles library-related cleanup tasks based on the state of global variables.
- `execute_sequence_of_calls`: Performs a series of subroutine calls with varying parameters and returns a short value.
- `initialize_system_structures`: Executes routines via pointers from a global structure and copies data between memory locations.
- `update_structure_field_2`: Executes several routines and then stores a word from a parameter into a specific offset within a structure pointed to by a global variable.
- `copy_fixed_block`: Copies thirty-six bytes from a source pointer to a destination pointer using word-sized increments in a loop.
- `copy_structure_data`: Copies a large block of data from a source buffer into various offsets of a destination structure.
- `copy_word_data`: Copies three words from a source location to offsets 12, 14, and 160 of a destination structure.
- `execute_callback_chain_7`: Recursively executes a sequence of function calls based on a conditional check and iterates through a loop to perform further function calls and memory comparisons.
- `process_data_buffer_loop`: Iterates through a data buffer, performing calculations and storing results into a structured array.
- `clear_table_entries`: Iterates through a nested table to check for non-zero values, calls a function pointer, and clears those values to zero.
- `traverse_linked_list_2`: Traverses a linked structure and updates various pointers and counters based on conditional checks and function return values.
- `traverse_linked_list`: Checks a value at an offset from a pointer and, if it is non-zero, calls itself recursively using a pointer found at another offset before clearing that pointer.
- `initialize_structured_data`: Clears several word-sized fields in a structure, copies data from a table into the structure, and performs a loop to copy a block of memory if a parameter is not negative one.
- `check_table_value`: Calls two other routines and checks a value in a table at an offset determined by a parameter.
- `update_sprite_table`: Reads data from a table to update a list of entries with new coordinates and attributes using a blitter operation.
- `initialize_data_blocks`: Sets specific word values in two consecutive blocks of a table using a loop.
- `process_blitter_queue`: Iterates through a list of parameters to perform multiple blitter operations and then resets a counter in a table.
- `process_resource_table`: Iterates through a table to open files and perform blitter transfers based on entries and associated data structures.
- `update_entity_states`: Iterates through a table of entities to update their positional coordinates based on velocity vectors and handles lifecycle counters.
- `execute_recursive_calls_2`: Performs a recursive call if a parameter is non-zero and then executes two subsequent routines.
- `rotate_array_elements`: Decrements a counter and, upon reaching zero, performs a left or right shift of elements within an array based on a flag.
- `perform_blitter_block_transfer`: Calculates clipping boundaries for a memory-to-memory area transfer and configures the Blitter hardware registers with source, destination, and size parameters.
- `perform_blt_operation`: Calculates clipping dimensions and configures the Blitter hardware registers to transfer data from a source to a destination.
- `blit_clipped_bitplane_rectangle`: Clips a source rectangle against a destination window, then waits for the blitter to go idle, programs its channels, masks and modulos, and runs a blit for each bitplane pointer in the source header, toggling the DMA enable bits around the operation.
- `saves_dmacon_flag_and_disables`: Reads the DMA control register into d0 and stores its low byte at a frame offset of -25 from a5, waits while the blitter busy bit is set, writes 1024 to disable DMA, then calls a library routine through the base pointer at offset 9424 in a4 and restores saved registers.
- `draw_masked_blitter_plane`: Waits for the blitter to be free, then for each bit plane of a source image it selects a solid fill word from bit tests on a flag byte and programs the blitter to write that plane into a clipped rectangle of the destination bitmap, with first and last word masks taken from a table indexed by the low four bits of the x offset.
- `stores_blitter_plane_and_size`: Clears the blitter control word, computes a longword from a table indexed by d1 scaled by four and adds a signed word from a stack-relative location, writing the result to the blitter destination pointer, then writes a word from another stack-relative location to the blitter size register, increments a word in a stack-relative location, and branches to an earlier point of the surrounding code.
- `read_dmacon_and_enable_dma_alt`: Reads the DMA control register into d0 and stores its low byte at offset negative 23 of a5, waits while bit 6 of that register is set, writes 400 hexadecimal to the DMA control register with bit 15 set, and calls the library routine at offset negative 462 through the pointer at offset 4312 of a4.
- `initialize_sprite_data_structures`: Allocates memory for a primary buffer and populates four sets of structured data blocks with offset values.
- `dispatch_switch_cases`: Evaluates a parameter to execute different branches, which may include iterating through a table of pointers to call other routines or calling function pointers from a global table.
- `update_table_entry`: Copies a short value and two other values from parameters into a table entry based on an index, then calls another routine using the entry pointer.
- `process_complex_entry`: Checks a global value, updates a table entry with specific parameters, calls another routine, and then updates a second table entry with different values before calling a function from a global pointer.
- `update_table_entries`: Iterates four times to copy short values into specific offsets of a table and executes a function via a pointer in each entry.
- `frag_move_rts`: Short code fragment that copies 65535 to the memory at an offset from a4, then returns.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a4, then returns.
- `sets_state_and_resets_tables`: Compares a 16-bit word in a global variable with the low word of the parameter and, when they differ, stores the parameter there; if the parameter is zero it stores 17 at a negative offset from a4, otherwise it clears three words in global variables, calls a table-entry update routine three times, and stores 16 at that same offset.
- `set_flag_if_sequence_matches`: Stores 17 as a 16-bit word at offset -25186 from a4.
- `update_derived_position_words`: Compares the 16-bit word at offset 4106 from a4 with the low word of the value passed in, and when they differ it stores that value there.
- `check_table_status`: Compares a global value against a specific address and updates a short value in a table if certain conditions are met.
- `process_decrementing_counter`: Decrements a value in a structure and either calls another routine, executes a jump table, or updates several fields including a clamped value.
- `process_conditional_updates`: Increments a value in a table and performs multiple conditional updates and function calls based on the state of a global variable.
- `reset_and_call_multiple`: Saves a word from a structure, clears five word-sized offsets within that structure, calls another routine three times with a specific parameter, and restores the original word.
- `initialize_table_entry`: Clears multiple fields and sets specific values within an entry of a table based on an index.
- `reset_and_call_repeatedly`: Sets a word at a specific offset within a structure to zero and calls another routine four times, with three of those calls using a specific parameter.
- `frag_move_clr_bsr_addq_rts`: Short code fragment that copies 65535 to the memory at an offset from a4, then clears the stack, then calls another routine, then adds 2 to a7, then returns.
- `initialize_multiple_entries`: Calls another routine four times with different offsets.
- `initialize_multiple_entries_2`: Calls another routine four times with different offsets.
- `initialize_multiple_entries_short`: Calls another routine three times with different offsets.
- `initialize_duplicate_entries`: Calls another routine twice using the same offset.
- `initialize_table_entry_wrapper`: Acts as a wrapper that calls another routine twice with a calculated offset, passing a value derived from a register to the called routine.
- `recursively_call_self`: This routine calls itself twice using an offset calculated from a register value.
- `initialize_table_entry_wrapper_2`: Acts as a wrapper that calls another routine twice with a calculated offset, passing a value derived from a register to the called routine.
- `update_coordinate_bounds`: Calculates and stores two sets of coordinate pairs into global memory based on an input value.
- `check_and_initialize_entry`: Tests a word at an offset of a4, and if non-zero, compares a pointer at offset 5f8 of a4 with a calculated address before calling another routine with a specific offset and incrementing the stack pointer.
- `set_status_on_match`: Compares an input value against two specific memory locations and writes a constant to a global variable if a match is found.
- `check_and_initialize_entry_2`: Tests a word at an offset of a4, and if non-zero, compares a pointer at offset 5f8 of a4 with a calculated address before calling another routine with a specific offset and incrementing the stack pointer.
- `set_status_on_match_2`: Compares an input value against two specific memory locations and writes a constant to a global variable if a match is found.
- `calculate_offset_values`: Computes two derived values from an input and stores them in global memory locations.
- `calculate_offset_address`: Subtracts a constant value from a value in a register and passes the result as an argument to another routine.
- `initialize_table_entry_twice`: Calls another routine twice with a calculated offset to initialize table entries.
- `calculate_offset_address_2`: Subtracts a constant value from a value in a register and passes the result as an argument to another routine.
- `offset_calculate_and_return`: Calculates a value by subtracting a constant from a register and returns it.
- `initialize_table_data`: Calls a routine to initialize an entry within a table.
- `offset_calculate_and_return_2`: Calculates a value by subtracting a constant from a register and returns it.
- `offset_calculate_and_jump`: Calculates a new address by adding a negative constant to a value in a register and jumps to that location.
- `calculate_offset_address_3`: Subtracts a constant value from a register and passes the result as an argument to another routine.
- `initialize_table_entry_8`: Tests a word at an offset in a structure and, if it is zero, calls another routine with a pointer to a stack-allocated word.
- `frag_moveq_move_pea_jsr_addq_rts`: Short code fragment that loads 2 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a4 onto the stack, then calls another routine, then adds 6 to a7, then returns.
- `check_and_initialize_entry_2_2`: Checks if a specific word at an offset is zero before initializing a table entry.
- `check_and_initialize_entry_2_3`: Checks if a specific word at an offset is zero before initializing a table entry.
- `frag_tst_bne_moveq_move_pea_jsr_addq_rt`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was not zero, then loads 2 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a4 onto the stack, then calls another routine, then adds 6 to a7, then returns.
- `initialize_table_data_2`: Calls a routine to initialize an entry within a table.
- `frag_moveq_move_pea_jsr_addq_rts_2`: Short code fragment that loads 1 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a4 onto the stack, then calls another routine, then adds 6 to a7, then returns.
- `frag_moveq_move_pea_jsr_addq_rts_3`: Short code fragment that loads 1 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a4 onto the stack, then calls another routine, then adds 6 to a7, then returns.
- `frag_moveq_move_pea_jsr_addq_rts_4`: Short code fragment that loads 1 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a4 onto the stack, then calls another routine, then adds 6 to a7, then returns.
- `frag_moveq_move_pea_jsr_addq_rts_5`: Short code fragment that loads 2 into d0, then copies d0 to the stack, then pushes the address of the memory at an offset from a4 onto the stack, then calls another routine, then adds 6 to a7, then returns.
- `dispatch_callback_index`: Checks a parameter value and, if it falls within a specific range, saves a global word, calls a routine from a table using an offset derived from the parameter, and restores the word.
- `process_structure_entry`: Uses a parameter to index into a table, calls several indirect functions, and stores a returned value into a structure.
- `set_boundary_values`: Sets a word at a specific offset to zero and, if another word at a nearby offset is zero, calls another routine and sets both offsets to a maximum value.
- `process_callback_loop`: Executes a routine, checks a global value, and then runs a loop twenty times to call another routine.
- `clear_table_pointer`: Checks if a value in a table is non-zero, calls a routine via a global pointer, and then clears that table entry.
- `execute_six_initializations_2`: Iterates six times, pushing a word from a table into the stack and calling another routine.
- `execute_seven_initializations`: Runs another routine seven times in a loop.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_four_resources`: Loops four times from zero, calling the routine that initializes a resource structure on each pass.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_twelve_resources`: Loops twelve times from zero, calling the routine that initializes a resource structure on each pass.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `execute_six_initializations`: Iterates six times, pushing a word from a table into the stack and calling another routine.
- `execute_seven_initializations_2`: Runs another routine seven times in a loop.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_four_resources_2`: Loops four times from zero, calling the routine that initializes a resource structure on each pass.
- `function_epilogue_5`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_twelve_resources_2`: Loops twelve times from zero, calling the routine that initializes a resource structure on each pass.
- `function_epilogue_6`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `initialize_resource_structure`: Performs conditional logic based on global flags to execute indirect function calls and populate a structure with data from a pointer or a global variable.
- `initialize_system_pointers`: Calls several functions via pointers stored in a global variable and a structure, then stores the results into a table and a global variable.
- `loads_segments_and_allocates_buffers`: Reads a flag bit from a structure to set a global counter to 10, then calls two library routines to get a value and store it in a global variable, and it loads two named executable segments and stores their handles and computed sizes in globals.
- `frag_addq_bra`: Short code fragment that adds 1 to d6, then jumps to another point in the code.
- `load_segments_and_allocate_buffers`: Loads three program segments by name from the dos library and stores their handles in global variables, calling a failure handler through a function pointer when a load returns zero.
- `releases_resources_and_clears_slots`: Closes a device whose pointer is stored in a global variable, then runs a set of cleanup calls and frees two memory blocks of 31500 and 16000 bytes when their pointers are nonzero.
- `frag_addq_bra_2`: Short code fragment that adds 1 to d6, then jumps to another point in the code.
- `release_loaded_segments_and_tables`: When a global value is nonzero, it updates a table entry, copies data through another routine, and calls a function through a pointer taken from a global structure.
- `function_epilogue_7`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `close_libraries_and_allocate`: If a global flag at offset -22874 from the base register is nonzero, it calls a function pointer stored in a library pointer at offset 4316, then closes the libraries whose pointers are stored at offsets 4320, 4316 and 4312 when each is nonzero.
- `call_indirect_function`: Calls a function using a pointer located at an offset from a value stored in a table.
- `update_table_entry_2`: Stores a parameter into a global variable and copies a longword from a table into another memory location based on an index.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a4 to d0, then returns.
- `frag_move_rts_3`: Short code fragment that copies the memory at an offset from a4 to d0, then returns.
- `fetch_table_longword`: Returns a longword from a table using an index provided in a parameter.
- `fetch_table_longword_2`: Returns a longword from a table using an index provided in a parameter.
- `copy_data_and_call_functions`: Updates a global variable, copies a longword to a destination, performs a block memory copy of thirty-two bytes if a condition is met, and calls two functions via pointers.
- `call_indirect_function_alt`: Calls a function using a pointer located at a specific offset from a value stored in a table.
- `prepare_and_call_dos_open`: Extracts six parameters from a structure, sign-extends them to 32-bit values, pushes them onto the stack along with two constants, and calls another routine.
- `execute_dual_calls`: Calls two functions sequentially using pointers retrieved from a table.
- `loops_three_times_and_updates_entries`: Runs a counted loop that increments a stack-relative word until it reaches 3, then calls a table-entry update routine, calls a routine through a function pointer read from a global variable, calls the table-entry update routine again, and returns.
- `copy_word_and_update_entries`: Copies a 16-bit value from a global variable at offset -23540 from a4 into the word before the location addressed by a5, then calls another routine to update a table entry.
- `call_input_seek_and_return`: Loads a pointer from offset 4312 from the base register, calls a function at offset -66 through it, then calls the function at offset -54 and keeps its 32-bit result.
- `frag_move_bsr_addq_rts`: Short code fragment that copies the memory at an offset from a4 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_move_bsr_addq_rts_2`: Short code fragment that copies a fixed memory address to the stack, then calls another routine, then adds 4 to a7, then returns.
- `initialize_bit_pattern_search`: Clears a byte at a stack offset, calls a DOS routine to get a pointer, and if a comparison with a value at an offset in a structure fails, calls another DOS routine and then calls a routine to check for bit patterns in a loop.
- `search_bit_patterns`: Clears a byte at an offset from a structure, performs several initialization steps, and then enters a loop to check for specific bit patterns within a table.
- `frag_tst_bne_addq_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then adds 1 to d7, then jumps to another point in the code.
- `store_byte_to_global`: Reads a byte from a memory location and stores it into a global variable.
- `process_message_input`: Iterates through a message queue to extract values and updates a global variable based on specific numeric codes and bit patterns.
- `frag_bsr_tst_bne_clr_rts`: Short code fragment that calls another routine, then tests d0, then branches if the last result was not zero, then clears the memory at an offset from a4, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `read_joystick_direction_code`: Reads the joystick movement register and masks it to two bits per axis, mapping each of eight bit patterns to a direction value 1 through 10 that is kept in d4.
- `frag_moveq_bra`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 2 into d5, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 4 into d5, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 8 into d5, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 5 into d5, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 9 into d5, then jumps to another point in the code.
- `frag_moveq_bra_7`: Short code fragment that loads 6 into d5, then jumps to another point in the code.
- `clear_word_and_select_code`: Sets d5 to 10 and clears the word at offset 4596 from the base register.
- `fill_six_row_pointer_table`: Walks a sequence of records from a pointer in a7 frame data, advancing by a computed size per record for the count given in the first parameter.
- `build_frame_pointer_table`: Reads two bytes through a pointer loaded from a stack slot, stores their product and the second byte in stack slots, then fills six pointer entries in a stack table with offsets into that data.
- `shift_word_and_call_dos`: Shifts the word at offset -6 from a5 left by 3 and stores it back, then calls a DOS wrapper routine with a buffer address at a5 minus 48, zeros, a longword from offset 4336 from a4, the registers d6 and d5, the shifted word, the word at offset -8 from a5, and the constant 224.
- `update_parameter_value`: Calls a function pointer from a table and updates a global variable with a provided parameter if it does not match a specific value.
- `calculate_data_offsets`: Calculates a difference between two values in a table and passes the result along with a pointer and an offset to another routine.
- `execute_indirect_function_call_2`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `execute_indirect_function_call`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `copy_data_block_4`: Sets a flag in a global variable and copies thirty-two bytes from a source pointer to a destination table.
- `frag_move_movea_jsr_rts`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a4 to a6, then calls another routine, then copies the top of the stack to a6, then returns.
- `call_different_offset_function`: Calls a function pointer located at a different specific offset within a structure.
- `concatenate_strings_to_buffer`: Calculates the length of strings and appends them to a local buffer before executing an indirect function call.
- `compare_string_lengths`: Calls a routine to process a pointer from a3, calls another routine using a pointer from a6 to perform string operations, and calls a third routine via a pointer from a6 if the results of two previous calls match.
- `filter_and_update_list`: Calls a routine to update a field in a structure and then calls a routine to filter entries in a linked list.
- `composite_sprite_buffer_build`: Calculates memory offsets for a set of six data blocks and performs a bitwise OR operation to merge them into a single destination buffer.
- `calculate_buffer_offsets`: Compares a value against a global variable and calculates memory offsets to pass to another routine.
- `lock_path_and_unlock`: Clears a longword, then calls the DOS Lock function on a path string at offset -22382 from a4 with mode -2.
- `execute_file_callback`: Calls a function through a pointer and handles a return value to potentially execute another routine.
- `lock_and_process_file`: Attempts to lock a file using a system call, and if successful, processes the locked file by calling another routine, then unlocks the file.
- `process_file_message_queue`: Copies a block of data from a memory source to a local stack buffer and executes a series of indirect function calls via a structure pointer.
- `process_file_data_2`: Performs recursive indirect function calls and copies data from a source buffer to a destination buffer based on file operations.
- `execute_two_functions`: Calls two different function pointers located at specific offsets within a structure pointed to by a global variable.
- `check_and_execute_callback`: Checks if a global variable is non-zero; if so, executes a function pointer from a global table, otherwise clears a global flag.
- `fetch_next_table_entry`: Checks if a global pointer is within bounds relative to another global variable; if so, it calls another routine and updates the pointer.
- `initialize_memory_buffers`: Writes a pattern of zeros and decrementing bytes into two memory regions and sets a global variable to a constant value.
- `update_bit_buffer`: Manipulates a bitstream by shifting values, updating a global counter, and copying bytes from a source buffer to a destination based on a table of indices.
- `update_dynamic_table_entry`: Copies a byte value between memory locations, updates a word-sized entry in a separate table, and conditionally increments a counter and updates a value before potentially calling another routine.
- `copy_bit_buffer_data`: Loads several values from a structure, then enters a loop to copy bytes from a source to a destination, updating a counter and a pointer along the way.
- `frag_bsr_tst_bmi_move_bra`: Short code fragment that calls another routine, then tests d1, then branches if the last result was negative, then copies d1 to d4, then jumps to another point in the code.
- `frag_cmpa_beq_move_subq_bra`: Short code fragment that compares a1 with a5, then branches if the last result was zero, then copies the memory just below a1 (moving a1 down) to d6, then subtracts 2 from d6, then jumps to another point in the code.
- `update_bit_buffer_and_jump`: Calls a routine, decrements a value in d6 twice, copies a byte from a source to a destination in a loop, and then stores several values into a structure before performing an indirect jump.
- `extract_nibbles_from_buffer`: Loads several values from a structure in a1, compares a byte from a pointer in a1 to ninety, and if not equal, moves it to d4.
- `frag_bsr_cmpi_beq_move_bra`: Short code fragment that calls another routine, then compares d1 with 144, then branches if the last result was zero, then copies d1 to d4, then jumps to another point in the code.
- `check_pointer_and_decrement`: Compares two pointers, and if they are not equal, moves a byte from the pointer in a1 to d1 and decrements d6 if d1 is not zero.
- `frag_move_bra_15`: Short code fragment that copies 144 to d4, then jumps to another point in the code.
- `update_bit_buffer_nibbles`: Calls a routine, checks if d1 is non-zero, and if so, enters a loop that shifts a byte right by four bits and stores the nibbles into a destination buffer before storing several values into a structure and performing an indirect jump.
- `unpack_four_values`: Unpacks bits from a source buffer into four 16-bit words stored in a table, updates the table pointers, and then branches to another routine.
- `extract_bits_to_five_words`: Iterates through a table to extract bits from bytes into five 16-bit words, updates a list of pointers, and calls another routine if a global word is non-zero.
- `unpack_bitstream_to_pointers_2`: Iterates through a bitstream to extract bits into six 16-bit values, stores these values in a list of pointers, and then increments those pointers by a constant offset before calling another routine or returning.
- `reset_and_load_status_flags`: Clears a 80-longword block of memory, then reads a value from the bsr-called routine and derives a status flag and mode/count variables from its bits, with a branch chain selecting one of four modes.
- `process_data_stream`: Calls another routine, then checks a byte from a structure in a0 against a global word; if it is greater than or equal, it copies a block of longwords from a0 to a global table, updates several global variables, and calls another routine via a pointer in a global variable.
- `frag_movea_jmp`: Short code fragment that copies a fixed memory address to a0, then jumps to the address in a0.
- `decrement_and_branch_2`: Decrements a word in a global variable and branches if it is not zero; otherwise it returns.
- `dispatch_function_pointer_3`: Saves registers, retrieves a value from a pointer at an offset from a5, calls a routine via a pointer from a table, and returns zero if the result is zero.
- `update_asset_buffer_2`: Sets three global variables using values from d0 and a calculated address, calls another routine, and then executes a function via a pointer retrieved from a memory location.
- `set_global_range_params_2`: Clears a global longword, stores a value from a structure into a second global longword, stores the sum of that value and a second value from the structure into a third global longword, and then calls another routine.
- `copy_table_from_address`: Copies a sequence of 32-bit words from a fixed memory address into a destination buffer.
- `monitor_structure_changes`: Increments a value at a pointer, updates global variables if specific word values in a structure change, and decrements a byte until it reaches zero.
- `setup_interrupt_handler`: Sets up a structure with specific values and a function pointer, then calls a routine via a pointer from a global variable.
- `execute_conditional_callback`: Checks a byte at a specific offset in a structure; if non-zero, it calls a function pointer retrieved from a global variable and then clears that byte.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `tokenize_string_input`: Splits a string into an array of pointers by using whitespace as delimiters or by grouping characters contained within double quotes.
- `transform_string_case`: Iterates through a null-terminated string and subtracts thirty-two from characters if a specific bit is set in a global lookup table.
- `calculate_string_length`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `copy_string_to_buffer_2`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `frag_clr_move_bra`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `reverse_engineer`: Repeatedly calls another routine, stores a byte from a register through a pointer, and then reverses a 12-byte buffer in place before terminating it with a null byte.
- `frag_clr_move_bra_2`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `convert_number_to_ascii`: The routine converts an integer into ASCII decimal digits using a temporary stack buffer and copies them in reverse order to a buffer.
- `reverse_nibbles_from_table`: The routine expands a packed nibble value using a global lookup table and writes the resulting bytes in reverse order to a destination.
- `parse_signed_integer_4`: Converts an ASCII string representing a signed decimal integer into a 16-bit value, handling optional leading plus or minus signs.
- `append_byte_to_list`: Increments a counter in a structure and stores a byte into a buffer pointed to by another field in that structure, then advances that pointer.
- `format_string_buffer`: Clears a value in a structure, stores a parameter into a field, calls a string parsing routine, and then clears that field again.
- `get_table_offset`: Calls another routine to retrieve a value, then uses that value to look up an address in a table, returning a default value if certain conditions are not met.
- `manage_sorted_list`: Searches for and manages entries within a sorted linked list, potentially allocating new nodes.
- `update_list_field`: Recursively traverses a linked structure and updates a field using a value derived from a pointer.
- `filter_list_entries`: Iterates through a linked list of structures and calls another routine for each entry that meets specific bitwise and value criteria.
- `execute_io_operation`: Checks a global pointer, calls a function via a pointer in a structure, and handles errors by writing to a global variable and updating a status field.
- `seek_file_position`: Performs a file seek operation and returns a calculated offset based on the provided parameters and the result of the seek.
- `initialize_structure_fields`: It writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets.
- `cleanup_display_context_2`: It conditionally invokes a callback when a structure field is nonzero, then writes reset values at two fixed offsets.
- `close_file_resource_2`: Checks a global variable and calls a function pointer to close a file or resource.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `normalize_signed_division`: Handles the signs of two 32-bit registers and calls another routine to perform a normalized fixed-point division.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.
- `check_pointer_and_branch`: Loads a word from an offset of a pointer in a3 and branches if it is not zero; otherwise it returns.
- `allocate_and_initialize_structure`: Calls another routine to allocate a memory block and, if successful, stores the value five at offset 8, zero at offset 9, and the contents of a register at offset 14 of the returned structure.
- `initialize_and_link_object`: Allocates a structure and populates its fields with provided arguments, a value from a function call, and a system-derived value.
- `get_table_entry`: Checks if an index is within valid bounds and if the corresponding entry in a table is non-zero, returning the address of the entry or a default value.
- `free_linked_list_nodes`: Iterates through a linked list and calls a routine to free each node until a null pointer is reached, then clears the head and tail pointers in a global structure.
- `insert_into_linked_lists`: The routine allocates memory for a new node and inserts it into three different linked lists tracked by a global structure.
- `retrieve_table_value`: Retrieves a value from a routine, checks a bit in the result, and performs a table lookup or returns a default value.
- `parse_format_specifier`: Parses a format specifier string to extract flags, width, precision, length, and conversion type, then retrieves arguments from a pointer and calls helper routines to write formatted output.
- `scan_format_string`: The routine walks a string looking for percent signs, skips escaped percent signs, and for each format specifier calls parse_format_specifier to process it and then calls a function pointer.
- `dispose_object_list`: Iterates through a list of objects, calling a routine on each object that meets specific bitwise criteria, and finally calls another routine with a provided parameter.
- `insert_into_sorted_list`: The routine inserts a new memory block into a sorted linked list, updating the list pointers and a global counter accordingly.
- `copy_memory_block_2`: The routine copies a block of bytes from one memory area to another using a count argument, handling overlapping source and destination by copying backwards when the source address is at or before the destination address, otherwise copying forwards.
- `traverse_and_update_list`: Checks a global pointer to call another routine, then recursively traverses a structure to update a field with a value derived from a pointer.
- `dispose_object_list_2`: Calls a routine via a pointer and, if the result meets specific bitwise criteria and a pointer is non-zero, clears that pointer and calls itself recursively.
- `frag_bsr_rts_2`: Short code fragment that calls another routine, then returns.
- `call_dos_function`: The routine calls a function pointer located at an offset within a structure pointed to by a global variable.
- `open_dos_file`: The routine calls a function pointer at a specific offset within a structure to perform a file opening operation.
- `copy_null_terminated_string_4`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm

- `process_hunk_headers_3`: Reads and processes Amiga DOS hunk headers from memory, allocating memory blocks for each hunk and copying data into them.
- `decrement_counter_2`: Decrements a counter in register d1 until it reaches zero, using a loop that branches back to an earlier instruction when the counter is not zero.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `decode_bitstream`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution.
- `frag_lsr_beq_rts_move_roxr_2`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `rotate_and_accumulate_3`: Rotates bits from a memory buffer into a global value using a shift count from another global variable, accumulating results through multiple iterations.
- `load_and_or`: Loads a 16-bit value from a global variable at offset 3205, stores it in a register, performs a bitwise OR operation on another global variable with the value 252, and then stores the result in a different global variable.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm2.fnt

- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 255 into d0, then returns.

### icon.library

- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code that increments a 16-bit value at a specific offset from a base pointer and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_refcount`: The library code decrements a reference counter in a global structure.
- `update_library_state`: As library code, it checks a 16-bit value at offset 32 of a6; if zero, it manipulates a linked list and calls another routine via a pointer, otherwise it sets bit 3 of a byte at offset 14 of the structure in a6 and returns.
- `frag_bset_moveq_rts`: Short code fragment that sets bit 3 of the memory at an offset from a6, then loads 0 into d0, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `handle_library_alert`: Copies a longword from a0 to offset 42 of a structure in a2, copies a6 to offset 34 of that structure, calls a routine at an offset from a6, stores the result in offset 38 of the structure, and if the result is zero, calls a routine at an offset from a global variable.
- `copy_two_pointers_from_table`: Library code that copies two 32-bit values from an address derived from a register into two global variables.
- `frag_move_jsr_addq_rts`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_move_jsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls copy_icon_data_to_stack, then loads the address of the memory at an offset from a7 into a7, then returns.
- `invoke_icon_structure_update`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `frag_movem_move_jsr_lea_rts_2`: Short code fragment that saves registers on the stack, then copies a6 to the stack, then calls copy_icon_data_to_stack, then loads the address of the memory at an offset from a7 into a7, then returns.
- `frag_move_jsr_addq_rts_2`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_move_jsr_addq_rts_3`: Short code fragment that copies a0 to the stack, then copies a6 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_move_jsr_addq_rts_4`: Short code fragment that copies a6 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `invoke_icon_structure_update_2`: This library routine calls a function to update various fields within an icon structure or its associated linked lists.
- `push_args_and_call_init`: Pushes a0 and a6 onto the stack, calls a library initialization routine, removes the two pushed longwords, and returns.
- `icon_library_call_vector_at_7fc`: Pushes a0 and a6 on the stack, calls the library vector at offset 2044, then discards both saved registers and returns to the caller.
- `frag_movem_jsr_addq_rts`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_jsr_addq_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movem_jsr_addq_rts_3`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `update_data_field`: Library code that calls another routine via a function pointer table using a passed parameter, then stores a second parameter into a memory location at an offset from the returned pointer.
- `initialize_icon_structure`: As library code, it saves registers, calls a routine with a pointer from a6, and if the result is non-zero, it sets a2 to the result, pushes two arguments onto the stack, and calls another routine; if that routine returns a non-zero value, it calls a third routine and branches to a final instruction.
- `copy_icon_data`: Copies ten longwords from a source at a negative offset of a6 to a destination at a positive offset of a2 using a loop, then writes several bytes and longwords from a6 to a2 and updates a pointer at offset 424 of a structure in a0.
- `frag_moveq_movem_unlk_rts`: Short code fragment that loads 0 into d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `copy_icon_data_to_stack`: This library routine copies ten longwords from an offset within a structure pointed to by the third argument into a local array on the stack, then copies several additional fields from the same structure into other stack slots and finally calls another routine with the first two arguments and a pointer to a stack-resident header.
- `initialize_icon_metadata`: Allocates memory for icon metadata and populates a structure with image pointers and associated data by calling several routines.
- `update_icon_image_and_metadata`: Checks if a specific icon type is requested, then updates the icon image pointer and associated metadata by calling several library routines.
- `propagate_icon_changes_2`: As library code, this routine calls a routine to update two different icon pointers in a structure in a4, and if successful, checks various flags to potentially call a routine with a pointer from a2 and then calls another routine to update a third icon pointer in the same structure.
- `iterate_icon_callbacks`: As library code, it enters a loop that calls a routine using parameters from the stack, stores the result in a pointer in a4, and continues iterating while a counter at offset -4 of a6 is greater than four; it also performs conditional checks on a value at offset 70 of a2 and calls another routine.
- `frag_moveq_bra_8`: Short code fragment that loads 0 into d4, then jumps to another point in the code.
- `frag_move_movem_unlk_rts`: Short code fragment that copies d4 to d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `process_icon_metadata_callbacks`: Checks for the existence of various icon metadata fields and callback pointers within a structure, then executes them sequentially.
- `update_icon_metadata_and_callbacks`: Saves a value from offset 26 of a structure, performs several conditional calls to a function pointer using values from offsets 42, 66, 22, and 26, and iterates through a null-terminated table of pointers at offset 54 to execute a callback for each entry.
- `iterate_icon_tables`: As library code, it calls two routines using values from the stack, then checks a pointer in a structure to call a third routine, updates a counter in a global variable, and finally iterates through a table of pointers calling a fourth routine.
- `iterate_icon_table`: Increments a pointer in a3 and a value at an offset from a6 in a loop until the pointer in a3 is null, then calls a routine using values from the stack, and if the result is non-zero, updates a3 from a structure in a2 and branches.
- `iterate_icon_table_entries`: Library code that walks a null-terminated table of pointers, calling a routine for each entry, updating a counter, and invoking a callback while potentially processing a secondary entry.
- `frag_move_movea_lea_bra`: Short code fragment that copies a2 to the stack, then copies the memory at an offset from a7 to a0, then loads the address of the memory at an offset from a0 into a2, then jumps to another point in the code.
- `iterate_linked_list_nodes`: The routine traverses a linked list structure by following pointers stored at an offset from the current node.
- `increment_param_and_recurse`: Library code that calls another routine with the same first argument and the second argument increased by a constant, then returns.
- `process_icon_structure`: As library code, it saves registers, calls a routine with a constant, and if the result is non-zero, calls another routine with a pointer from the result structure and a pointer from a2, then potentially calls a third routine with a pointer from the result structure and a pointer from a2.
- `decrement_icon_counter_2`: As library code, it pushes a pointer from a2 onto the stack, calls another routine with a pointer at offset 140 of a2, and if the result is non-zero, moves a2 into d0.
- `function_epilogue_8`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `decrement_icon_counter`: Decrements a 16-bit word through a pointer in a2 and, if the result is not greater than zero, calls another routine with values from the stack.
- `decrement_counter_and_store_pair`: The routine decrements a short counter through a pointer, then uses the counter value as an index into a table pointed to by a global variable.
- `process_icon_metadata`: As library code, it saves registers, calls a routine with a constant, and if the result is non-zero, it calculates a new address in a2, calls a second routine, and if that returns zero, it calls a third routine; if the third routine returns non-zero, it calls a fourth routine and branches to a final instruction.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `call_library_routine_and_return_zero`: Library code that calls another routine and then returns zero.
- `perform_incremental_recursion`: Library code that performs a recursive call with an incremented parameter until the callee stops the recursion.
- `load_and_call_pointer`: Library code that loads a longword from a pointer in a2 into a3 and d0, then calls a function pointer from the stack and performs a branch based on the result in d0.
- `match_byte_pattern`: As library code, it saves registers d2, d3, and a3 to the stack, calls a routine, and if the result is non-zero, checks if a byte at the pointer in a3 matches a specific constant.
- `load_pointer_and_return`: As library code, this routine loads a longword from a pointer in a2 into a3, moves it to d0, and if d0 is not zero, returns; otherwise it clears d0 and restores registers from the stack before returning.
- `invoke_checksum_handler`: Library code that saves registers, loads two values from the stack, calls a function pointer, moves the result to d5, and branches to another routine.
- `verify_counter_and_subtract`: As library code, this routine pushes a longword onto the stack, calls a routine, and if the result in d0 is not zero, calculates the difference between d0 and a value in d2 and branches; otherwise it moves d0 to d4 and branches.
- `verify_and_increment_counter`: As library code, it calls a routine, compares the result to d5, and if they match, calls another routine with d5, d3, and d2, then sets d2 to 1 if the second call returns zero.
- `frag_tst_beq_move_addq_bra`: Short code fragment that tests d4, then branches if the last result was zero, then copies d4 to d2, then adds 1 to d2, then jumps to another point in the code.
- `return_zero_if_not_zero`: Library code that checks if register d2 is non-zero, otherwise it clears d0 and restores d2 through d5 from the stack before returning.
- `process_table_data`: Library code that clears a byte at offset 30 of a structure, calls a routine with a pointer and a constant, and if the result is non-zero, calls another routine and then a third routine before branching.
- `call_and_branch_conditional`: Library code that pushes two values onto the stack, calls a routine, and then either sets d3 to one and branches or adjusts a4 and branches based on the result in d0.
- `frag_moveq_bra_9`: Short code fragment that loads 0 into d3, then jumps to another point in the code.
- `calculate_and_process_table`: As library code, it performs arithmetic on d3 and d2 to calculate a value, enters a loop that calls a routine for each byte in a table, and if a condition is met, calls two more routines using values from the stack and registers.
- `compare_and_subtract`: Compares a value from the stack to the constant 48 and 57, then either sets d0 to a specific constant or calculates the difference between the stack value and 48.
- `tiny_moveq_rts_5`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: The library routine forwards a set of four parameters to another routine, checks the returned value, and if non-zero calls the same routine again with a modified argument list.
- `process_icon_data`: Saves registers, calls a routine with a constant, and if the result is non-zero, writes 10 to offset 14 of the result structure and offset 14 of a structure in a2, then calls another routine using a pointer from a2.
- `function_epilogue_9`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `schedule_library_task`: Loads d0 and d1 from the stack, pushes two values from the stack onto the stack, calls a routine, moves the result to d1, compares d1 to a value from the stack, and otherwise returns zero.
- `tiny_move_rts_6`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `schedule_library_task_2`: Loads d0 and d1 from the stack, pushes two values from the stack onto the stack, calls a routine, moves the result to d1, compares d1 to a value from the stack, and otherwise returns zero.
- `tiny_move_rts_7`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `divide_and_swap`: Compares d2 to 65535; if d2 is greater than 65535, it performs a series of swaps and unsigned divisions using d1 and d2, otherwise it performs unsigned divisions and swaps on the components of d1 and d2 before returning.
- `rotate_32bit_value`: The library code rotates a 32-bit value left by one bit per iteration for sixteen iterations, using a carry flag to move bits between the high and low halves.
- `multiply_32_bit_integers_2`: The library code computes the 32-bit product of two 32-bit registers using 16-bit partial products.
- `forward_call_to_helper_2`: Calls one helper and returns, with no other work.
- `call_helper_return_register_2`: Saves d2 on the stack, copies d1 to d2 and d0 to d1, calls the modular computation routine, then returns the value left in d1 as d0 after restoring d2.
- `negate_result_on_flag_2`: Calls a helper for a value and negates that value when the flag the helper leaves in an address register is nonzero.
- `call_and_negate_on_flag_2`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `invoke_table_entry_3`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_8`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `compute_string_checksum`: The library code iterates through a null-terminated byte string, updating a 32-bit accumulator with a decrementing 1s-complement-style pattern for each character.
- `compare_byte_with_register`: As library code, it loads a pointer from offset 4 of the stack into a0, loads d0 from offset 8 of the stack, compares the byte at a0 with d0, and if they are equal, decrements the pointer in a0 and returns it in d0.
- `frag_moveq_bra_10`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `copy_c_string`: The library code copies bytes from a source pointer to a destination pointer until a zero byte is encountered, advancing both pointers each iteration.
- `frag_movem_tst_bne_subq_bra`: Short code fragment that copies several registers to or from memory, then tests the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from a0, then jumps to another point in the code.
- `check_conditional_match`: As library code, it saves registers a0 and a1, decrements d0 from offset 12 of the stack, and if d0 is not less than zero, compares a byte from a1 with a byte from a0, returning zero if a match is found and the byte at a1 is zero.
- `frag_moveq_bgt_rts`: Short code fragment that loads 1 into d0, then branches if the comparison was greater, then loads 255 into d0, then returns.
- `test_and_decrement_byte`: As library code, it saves registers a0 and a1, loads d0 from offset 12 of the stack, tests a byte at the pointer in a0, and if the byte is non-zero, decrements d0.
- `frag_subq_bsr_rts`: Short code fragment that subtracts 1 from a0, then calls another routine, then returns.
- `copy_bytes_with_counters`: Decrements d0 and loops while it is not less than zero, copying a byte from the address in a1 to the address in a0 and incrementing both pointers, then performs a second loop that decrements d0 again and clears a byte at a0 if d0 is not less than or equal to zero.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Library code that validates a parameter, calls another routine to check a return value, and if successful, performs further calls using a value from a pointer.
- `frag_movem_move_movea_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d2, then copies the memory at an offset from a7 to d3, then copies the memory at an offset from a7 to a2, then jumps to another point in the code.
- `traverse_icon_list`: Calls another routine repeatedly, using a value from a byte at offset 7 of a structure in a2 to update a pointer at offset 12 of that structure, until the pointer is zero.
- `function_epilogue_10`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `calculate_icon_dimensions`: Performs several 32-bit multiplications and calls another routine to update a structure at offset 10 of a pointer, using values from the stack and a structure at offset 36.
- `function_epilogue_11`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `save_registers_and_branch`: Saves registers d2 through d6 and a2 to the stack, loads three values from the stack into d2, d3, and d4, and branches to another routine.
- `process_icon_task`: Pushes several values onto the stack, calls another routine, and then either branches to another location or modifies d6 based on a value at offset 28 of the stack.
- `propagate_icon_changes`: As library code, it stores a value from offset 40 of the stack into offset 12 of the structure in a2, calls another routine, and if the result is non-zero, performs a series of calculations involving a byte from offset 7 of a structure in a0, calls another routine, and potentially calls a third routine based on the result of a comparison.
- `function_epilogue_12`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.

### info.library

- `fall_through_entry_6`: Entry point part-way through straight-line code: runs one instruction (illegal) and then falls through into the code that follows, with no return of its own.
- `store_library_state`: Stores the a6 register and the d0 register into two global variables, writes the a0 pointer into a table field at offset 34 of the structure pointed to by d0, and then stores that same pointer into another global variable.
- `frag_move_jsr_addq_rts_6`: Short code fragment that copies a6 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_move_jsr_addq_rts_5`: Short code fragment that copies a6 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_move_jsr_addq`: Short code fragment that copies a6 to the stack, then calls another routine, then adds 4 to a7.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `info_library_call_5e2_with_saved_regs`: Pushes a1 and a6 onto the stack, calls the library routine at offset 5e2, then discards both saved registers and returns.
- `clear_flag_and_increment_counter_2`: Clears bit 3 of the byte at offset 14 of the structure whose pointer is passed on the stack, adds 1 to the 16-bit word at offset 32 of that structure, and returns the pointer in d0.
- `check_and_modify_structure`: This library routine calls another routine, checks a word at offset 32 of a structure in a2, and if it is not less than or equal to zero, sets bit 3 of a byte at offset 14 of that structure before calling another routine and returning.
- `call_library_functions_with_offset_args`: This library routine retrieves a library base from a global variable and calls two library functions using offsets and calculated arguments.
- `decrement_and_branch_on_bit`: As library code, it loads a pointer from offset 4 of the stack into a0, decrements a word at offset 32 of the structure in a0, and if the result is non-zero, tests bit 3 of a byte at offset 14 of the structure; if the bit is clear, it calls another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `recursively_call_pointers`: The routine checks several global pointers and arguments, then calls itself recursively if a pointer is non-zero.
- `initialize_library_internal_structures`: Allocates a memory block via another routine and populates a series of internal pointers and data tables by copying values from a global variable into various offsets of the newly allocated structure.
- `initialize_info_library_struct`: Calculates several offsets from a base address and stores them into a series of internal library pointers, then populates various structure members by copying sequences of values from a global table using multiple loops.
- `execute_library_function_and_store_result`: As library code, it saves registers, calls another routine with a pointer from a stack-allocated location, stores the result in a global variable at offset 84, and then branches to another routine if the stack pointer is not zero.
- `execute_conditional_call_2`: This library code pushes a longword onto the stack, calls a routine using a pointer from a register, stores the result in a global variable, and then branches to another routine or a different location based on the status of the stack pointer.
- `execute_conditional_call`: This library code pushes a longword onto the stack, calls a routine using a pointer from a2, stores the result in a global variable at offset 72, and branches if the result is not zero.
- `execute_conditional_library_call`: Calls a routine with a pointer from a global variable, stores the result in another global variable, and if the result is zero, calls a second routine with a constant argument.
- `function_epilogue_13`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `frag_move_movea_tst_bne_bra`: Short code fragment that copies a2 to the stack, then copies 88 to a1, then tests the memory at a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_movea_cmpi_bne_bra`: Short code fragment that copies the memory at a1 to a0, then compares the memory at an offset from a0 with 7, then branches if the last result was not zero, then jumps to another point in the code.
- `branch_on_structure_field`: Loads a pointer from a structure, checks a longword at offset 68 of that structure, and branches to another routine based on the result.
- `load_offset_pointer`: This library code loads a pointer from a1 into a0, compares a byte at offset 61 of that structure to one, and if they are not equal, loads a pointer from offset 76 of that structure into a0 and then loads a longword from offset 442 of that new address into d0 before branching.
- `frag_move_beq_movea_bra`: Short code fragment that copies a2 to d0, then branches if the last result was zero, then copies the memory at an offset from a2 to a0, then jumps to another point in the code.
- `function_epilogue_14`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `update_structure_field`: Calls another routine, stores the returned value at offset 152 of a structure pointed to by the result, and then checks a byte at offset 61 of a structure pointed to by a global variable to decide whether to push a global address onto the stack.
- `check_library_status_and_set_flag`: This library code checks a boolean flag and a function return value to decide whether to update a global flag and retrieve a value from a nested pointer.
- `update_info_structure`: Library code that loads values from the stack into d3 and d2, calls a routine with a pointer and a constant, and stores the result in a global variable at offset 60.
- `call_library_function_and_store`: As library code, this routine pushes a longword onto the stack, pushes a pointer, pushes d3, calls a routine, stores the result in a structure in a4, and branches based on whether d0 is non-zero.
- `process_icon_list`: Loads a pointer from a global variable and enters a loop that checks if a byte at offset 61 of the pointed-to structure is 1; if so, it pushes two values onto the stack and calls another routine, repeating this process until the condition is no longer met.
- `update_icon_data_2`: This library code tests a register, and if non-zero, pushes two values onto the stack, calls a routine, and then loops or branches based on the result.
- `update_icon_data`: This library code calls a routine to copy a string, calls another routine to get a length, stores a value in a global variable at offset 160 of a structure, and then conditionally calls another routine to update a different global variable.
- `copy_library_strings_and_data`: This library routine copies a string using a library function and then copies certain dwords or even dwords from one structure to another.
- `reset_and_dispatch_library_callback`: Clears several fields in a global structure and stores a value from a library parameter into a specific offset of that structure, then calls another routine and returns based on the result.
- `execute_callback_and_store_result`: Pushes a constant and two pointers onto the stack to call another routine, then stores the returned value into a long at offset 100 of a global variable.
- `initialize_library_internal_structures_2`: Calls a routine using a pointer from an offset of a6, and if the return value is non-zero, clears a long value on the stack; otherwise, it pushes two parameters and branches to another location.
- `call_and_store_result_2`: This library code calls a routine via a pointer, stores the result in a register, and then branches to another location.
- `call_routine_and_update_pointer`: It pushes a longword onto the stack, calls a routine at an address from a pointer, stores the result in a location pointed to by a4, and branches to another routine if the stack pointer is not zero.
- `execute_conditional_callback_6`: Calls a routine using a pointer from offset 50 of a structure and a pointer from a4, then conditionally calls another routine and a function pointer based on the result and a global variable.
- `validate_and_update_info`: Library code that calls a routine, moves the result to d2, and if d2 is non-zero, pushes it to the stack to call another routine before branching.
- `validate_library_parameters`: As library code, this routine checks a 16-bit value and a longword, then performs a series of calls to different routines using values from a3 and various stack arguments, before restoring registers and returning.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_366_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `invoke_library_functions`: Library code that calls two different library functions using a base address from a global variable and a pointer passed via a stack-allocated structure.
- `dispatch_via_jump_table`: Checks a word at offset 38 of a structure, and if it is between 17 and 16 inclusive, it uses it as an index into a jump table to jump to another routine.
- `function_epilogue_15`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `apply_bitmask_to_global`: Library code that loads a value from a global structure, performs a lookup based on a word from a structure, and jumps to a destination determined by a table.
- `set_bits_and_call_template`: Sets bits 5 and 3 in a byte at the address in a4, sets bits 3, 4 and 5 in a global byte, sets bit 3 of a byte indexed by d6 plus 22, stores the exclusive-or of d1 with 4 into a global structure at offset 112, and calls a setup routine with a value from a1.
- `frag_moveq_bra_11`: Short code fragment that loads 2 into d0, then jumps to another point in the code.
- `frag_moveq_bra_12`: Short code fragment that loads 1 into d0, then jumps to another point in the code.
- `frag_moveq_bra_13`: Short code fragment that loads 16 into d0, then jumps to another point in the code.
- `frag_moveq_bra_14`: Short code fragment that loads 32 into d0, then jumps to another point in the code.
- `xor_store_and_call_template`: Library code that exclusive-ORs d0 with d1, stores the result at offset 112 of a global structure, and calls another routine with a longword read from offset 44 of the structure in a1, then returns.
- `validate_and_update_info_structure`: Checks a type byte at offset 61 of a structure and updates a pointer at offset 160 if the type is 3 or 4.
- `frag_movea_cmpi_bne_pea_bra`: Short code fragment that copies the memory at a3 to a0, then compares the memory at an offset from a0 with 1, then branches if the last result was not zero, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `process_string_and_attributes`: This library routine copies a string, calls a library function, and performs several conditional checks involving pointer offsets and string copies, potentially updating a local variable with a value from a structure.
- `verify_multiple_pointers`: This library code calls a routine with a pointer and a constant value of negative 255, then checks if the result is not negative 1 to decide whether to continue or return.
- `compare_status_and_branch`: As library code, it pushes the value negative 255 onto the stack twice and pushes a pointer from a memory location and the value in d2 onto the stack, then calls a routine; if the result in d0 is not equal to negative 255, it branches to another location.
- `compare_status_and_branch_2`: As library code, it pushes the value negative 255 onto the stack twice and pushes a pointer from a memory location and the value in d2 onto the stack, then calls a routine; if the result in d0 is not equal to negative 255, it branches to another location.
- `compare_and_return_status`: Library code that calls a routine with a pointer and a constant, then compares the result to a value in d1, and either returns zero or returns d0.
- `call_other_and_return_one`: Library code that calls another routine and then returns one.
- `initialize_info_library_tables`: This library code loads a pointer from offset 80 of the input structure, calls a routine with it, then repeatedly calls another routine many times with the input pointer paired with different constant arguments, passing a global variable to some of those calls, and returns.
- `execute_library_calls_2`: This library routine calls another routine multiple times with a fixed first parameter and varying other constant arguments.
- `initialize_display_palette_entries`: Loads a pointer from the input structure into d2, then calls another routine seven times passing that pointer with a fixed set of integer constants that vary each time, zeroing the stack for some of the calls, and returns.
- `register_info_table_entries`: Loads a pointer from the input structure into d2, then calls another routine ten times passing that pointer with a fixed set of integer constants that vary each time, zeroing the stack for some of the calls, and returns.
- `set_info_table_entries`: Loads a pointer from the input structure into d2, then calls another routine ten times passing that pointer with a fixed set of six integer constants that vary each time, zeroing the stack for some of the calls, and returns.
- `compare_table_values`: Library code that compares a byte at offset 61 of a structure in a0 with 4, and if they are not equal, sets d1 to 131 and branches.
- `call_structure_function`: Library code that compares two values from a table at offset 234 of a structure pointed to by a3, and if they are not equal, pushes several values onto the stack and calls another routine using a pointer from a structure at offset 238.
- `execute_multiple_library_calls`: Library code that calls a library function or routine multiple times using a pointer from a structure.
- `execute_library_callback`: Library code that calls a routine with a value from a structure, stores the result in a structure at offset 234, and branches based on whether that value is zero.
- `update_structure_pointers`: Library code that updates two pointers in a structure at a3 by copying values from another structure at a2, clears a longword at offset 242 of a structure at a3, and checks a longword at offset 92 of a structure in a global variable.
- `call_library_function_with_constant`: Library code that pushes a constant onto the stack and calls a library function, then moves the result into a2 and d3 before branching.
- `process_loop_and_copy`: Library code that calls a routine with a constant, stores the result in a structure, and enters a loop that calls another routine and updates a pointer in a4, eventually copying a string from a structure to a destination at offset 238 of another structure if a pointer in a4 is null.
- `update_structure_pointers_2`: Library code that compares values from a structure and a global variable, then calls three different routines and updates a structure at offset 62 using a value from a pointer in a structure.
- `update_pointer_from_structure`: Library code that calls a routine with a pointer from a structure, calls another routine with a pointer from a global variable, and then updates a structure member with a value retrieved from a pointer.
- `update_pointer_dereference_2`: Library code that compares a value at offset 8 of a structure with a value at offset 238 of another structure, then calls a routine and updates a structure at offset 62 if certain conditions are met.
- `update_pointer_dereference`: Library code that calls a routine with a pointer from a structure, calls another routine with a pointer from a global variable, and then replaces a structure member with a value retrieved by dereferencing a pointer.
- `modify_global_state_and_execute`: Library code that sets a byte at offset 242 of a structure to 1, checks a condition involving two structures, calls several library functions, and updates a global pointer.
- `execute_sequential_callbacks_3`: Library code that sets a longword at offset 242 of a structure to one, compares values at offsets 234 and 238 of a structure, and calls two different routines using values from a global variable.
- `execute_sequential_callbacks_2`: Library code that calls a routine with a pointer from a structure, checks if a value at offset 234 of a structure plus 4 equals a value at that same offset, and if so, updates a structure member and calls another routine.
- `execute_sequential_callbacks`: Library code that calls another routine twice using pointers or offsets or registers.
- `check_table_entry_and_branch`: As library code, it saves registers, loads a pointer from a table at offset 100, checks a longword at offset 242 of that pointer, and branches based on the result.
- `clear_structure_field`: Library code that compares two values derived from a structure, and if they are equal, clears a longword at offset 92 of another structure.
- `iterate_and_store_results_4`: Library code that loads a pointer from a structure, calls a routine with a value from a pointer at offset 23e, and then updates a pointer in a0 using an offset from a structure in a5.
- `iterate_and_store_results_3`: Library code that increments a value in d4, iterates through a linked list using a loop, and calls a routine to store a result at offset 92 of a structure.
- `iterate_and_store_results_2`: Library code that calls a routine with a pointer and a constant, then prepares to iterate through a list by loading a pointer from a structure and setting a register to zero.
- `process_string_and_store_result`: Library code that calls a routine to get a string length, then uses that length to call another routine and stores the result in a table at an offset determined by a pointer in a2.
- `iterate_and_store_results`: Library code that performs a loop using d4 as an index to access a table, calls two different routines to process data, and updates a value in a structure at offset 234 if certain conditions regarding a3 and a structure in d3 are met.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_indexed_function`: Reads a byte from an offset of 61 within a structure pointed to by a global variable, then uses that byte as an index into a jump table to call a specific sub-routine.
- `library_info_table_setup`: This is library code from the info library.
- `frag_movea_bra`: Short code fragment that copies 96 to a0, then jumps to another point in the code.
- `frag_movea_bra_2`: Short code fragment that copies 106 to a0, then jumps to another point in the code.
- `frag_movea_bra_3`: Short code fragment that copies 118 to a0, then jumps to another point in the code.
- `frag_movea_move_pea_bra`: Short code fragment that copies 130 to a0, then copies a0 to the stack, then pushes the address of a fixed memory address onto the stack (2 times in a row), then jumps to another point in the code.
- `run_info_setup_sequence`: Calls through the address held in address register a3 three times, then calls the execute_library_calls_2 and register_info_table_entries routines, calls through a3 once more, and returns.
- `run_info_init_sequence`: Calls through the address held in address register a3 seven times and through a4 twice, then calls the execute_library_calls_2, initialize_display_palette_entries and register_info_table_entries routines, calls through a3 once more, and returns.
- `run_info_table_sequence`: Calls through the address held in address register a3 eight times and through a4 twice, then calls the execute_library_calls_2, register_info_table_entries and set_info_table_entries routines, calls through a3 once more, then calls initialize_display_palette_entries and returns.
- `run_info_setup_sequence_2`: Calls through the address held in address register a3 three times, then calls execute_library_calls_2 and register_info_table_entries, and calls through a3 once more before returning.
- `call_address_in_a3`: Calls the routine whose address is held in address register a3 and returns, with no other work in the body.
- `validate_and_dispatch_resource_pointers`: Calls another routine four times with specific constants and table offsets to populate four global variables, then verifies they are non-zero.
- `dispatch_jump_table_2`: This library code calls four different routines using a pointer from a register, stores each result in a global variable, and then performs a conditional branch based on whether those global variables or a value from a pointer are non-zero.
- `dispatch_jump_table`: Loads a pointer from a global variable into a0, reads a byte from an offset of 61 from a0 into d0, and if d0 is between 1 and 5, performs a jump to an address from a table using d0 as an index; otherwise it jumps to a different address.
- `frag_movem_unlk_rts`: Short code fragment that copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `check_global_flag`: Loads a pointer from a global variable into a0, checks a bit at an offset of 104 from a0, and if that bit is set, it branches to another routine; otherwise it clears a byte at an offset of 384 from a different global pointer and branches to another routine.
- `execute_nested_library_calls`: Calls two library functions using pointers from global variables and arguments from the stack, then returns zero if the second call result does not match a specific value.
- `function_epilogue_16`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `verify_structure_members`: Library code that checks a byte at offset 61 of a structure in a0, writes four 16-bit values to specific offsets of four different structures, and calls a routine with a pointer and a constant, returning zero if the result is not 255.
- `frag_moveq_move_jsr_cmp_lea_bne_bra`: Short code fragment that loads 255 into d3, then copies d3 to the stack, then copies a fixed memory address to the stack, then copies d2 to the stack, then calls the routine whose address is in a2, then loads 255 into d1, then compares d1 with d0, then loads the address of the memory at an offset from a7 into a7, then branches if the last result was not zero, then jumps to another point in the code.
- `call_and_compare`: Calls a routine with multiple parameters, compares the result to a constant value, and either returns zero or continues execution based on the comparison result.
- `compare_and_return_status_2`: Library code that calls a routine with a pointer and a constant, then compares the result to a value in d1, and either returns zero or returns d0.
- `function_epilogue_17`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `dispatch_parameter_logic`: This library code extracts a bit-shifted value from a structure, calls a routine, and then conditionally calls another routine using a value from the stack or a bit-shifted constant.
- `function_epilogue_18`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `dispatch_library_parameter_value`: This library code saves registers, loads a pointer from an offset of the stack into a2 and a value from another offset into d2, then performs several conditional branches based on a value loaded from the structure in a2.
- `frag_move_jsr_addq_bra`: Short code fragment that copies d2 to the stack, then copies a2 to the stack, then calls another routine, then adds 8 to a7, then jumps to another point in the code.
- `frag_move_jsr_moveq_bra`: Short code fragment that copies a2 to the stack, then calls another routine, then loads 0 into d0, then jumps to another point in the code.
- `execute_two_calls`: Library code that calls one routine with a stack pointer, then calls a second routine with another stack pointer, sets the result register to one, restores the saved registers, and returns.
- `execute_multiple_callbacks_2`: Library code that calls another routine or a global function pointer multiple times based on a condition checked against a global variable, then calls another routine with a specific pointer and constant, and finally calls another routine with a certain pointer and a constant.
- `setup_string_template`: As library code, this routine calculates a bitmask from a value in a table, and calls a string-formatting routine multiple times with different bit-masked arguments and a global pointer.
- `prepare_format_string`: Library code that writes several bytes to the stack, sets two 16-bit values, and pushes two pointers onto the stack before branching to another routine.
- `format_string_arguments`: This library code prepares a pointer-based argument list or a temporary buffer for a string-formatting routine that uses a pattern or template string and multiple string arguments.
- `frag_move_moveq_movea_bra`: Short code fragment that copies the memory at an offset from a7 to d0, then loads 0 into d1, then copies d1 to a1, then jumps to another point in the code.
- `advance_pointer_to_null_byte`: Increments a1 until the byte at the address formed by a1 plus d0 is zero, then returns the final value of a1 in d0.
- `copy_cstring_to_buffer`: Copies bytes from the source pointer at offset 8 on the stack to the destination pointer at offset 4, advancing both, until a zero byte has been copied.
- `frag_movea_move_moveq_bra`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to d0, then loads 0 into d1, then jumps to another point in the code.
- `copy_byte_block`: Copies bytes one at a time from the pointer in a0 to the pointer in a1, advancing both and incrementing d1, and repeats while d1 is below d0, then returns.
- `frag_movea_bra_4`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to a1, then jumps to another point in the code.
- `compare_strings`: Library code that compares two null-terminated strings and returns true if they are equal.
- `get_string_length`: Library code that counts the number of bytes in a null-terminated string.
- `execute_library_operations_2`: Calls a routine to initialize a value, then performs three subsequent calls to library functions using parameters derived from the initial call, a structure at offset 50 of a pointer, and a decremented value.
- `call_string_formatter`: This library code calls another routine using a pointer to a global variable and a local buffer.
- `shift_and_extend_byte`: Library code that shifts a value in a2 left by two bits, extracts a byte from the resulting address into d1, extends it to a long, and branches to another location.
- `copy_string_data`: Library code that copies bytes from a source location to a destination location and appends a null terminator.
- `initialize_and_call_structure`: Library code that populates a structure at a2 with various values, including a calculated length and several cleared fields, then calls a routine using a pointer from a3 and another routine using a pointer from a2.
- `execute_library_function`: Library code that calls a library function using a pointer retrieved from a global variable offset.
- `execute_library_calls`: Library code that calls a library function or another routine multiple times using a pointer retrieved from a structure member.
- `invoke_table_entry_4`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts_9`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.

### diskfont.library

- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: This library code stores a6 and d0 into global variables, calls a routine using a pointer from a global variable, and if certain results are non-zero, modifies a structure at an offset of d0 by updating a pointer and clearing a field.
- `manipulate_linked_list`: This library code unlinks an element from a linked list if a 16-bit value at an offset is not greater than zero, calls a routine, and then potentially unlinks a second element based on pointer comparisons.
- `update_font_linked_list`: This library code checks two 16-bit values at offsets 56 and 58 of a structure in a6, and if both conditions are met, calls three different routines and performs a linked list insertion by updating pointers in a structure.
- `fall_through_entry_7`: Entry point part-way through straight-line code: runs two instructions (bset.b, moveq) and then falls through into the code that follows, with no return of its own.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `frag_subq_bne_btst_beq_jmp`: Short code fragment that subtracts 1 from the memory at an offset from a6, then branches if the last result was not zero, then tests bit 7 of the memory at an offset from a6, then branches if the last result was zero, then jumps to another point in the code.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `fall_through_entry_8`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_9`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_10`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_11`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `parse_font_resource_stream_3`: Iterates through a linked list of font resource headers to extract metadata and string data into a destination buffer.
- `frag_movea_moveq_bra`: Short code fragment that copies the memory at an offset from a4 to a1, then loads 1 into d4, then jumps to another point in the code.
- `count_null_terminated_string`: Counts the number of bytes in a null-terminated string by iterating through memory and incrementing a register for each non-zero byte.
- `parse_font_resource_stream_2`: Iterates through a data stream to identify font resource headers, populates a structure with metadata including a type identifier and length, and performs string manipulation and memory management to handle font resource data.
- `process_font_string_buffer`: Allocates a message buffer, copies a null-terminated string from a source pointer into a destination buffer, and updates a counter in a structure.
- `count_null_terminated_string_2`: Counts the number of bytes in a null-terminated string by iterating through a buffer in a1 and increments a counter in d4.
- `copy_font_data_and_load_resource`: Copies a sequence of bytes from a source buffer to a destination buffer using a loop, then calls a routine to read data from a disk and checks if the returned value is 4 and if a specific word at a negative offset from the stack equals 61440.
- `parse_font_resource_stream`: Iterates through a data stream to identify font resource headers, populates a structure with metadata including a type identifier and length, and manages a string buffer by calculating string lengths and copying characters into a destination memory area.
- `frag_moveq_movem_unlk_rts_2`: Short code fragment that loads 0 into d0, then copies several registers to or from memory, then removes the stack frame held in a6, then returns.
- `calculate_font_offset`: Library code that compares a 16-bit value at offset 256 of a structure with a value at offset 4 of another structure, then performs arithmetic to calculate a value for d2.
- `update_font_index_and_offset_3`: Library code that performs bitwise operations on bytes from two structures and subtracts a value from d2, then branches based on the result of an AND operation.
- `update_font_index_and_offset_2`: As a library routine, it subtracts a value from a table at a fixed address using a shifted version of a 16-bit value in d4 as an index, then performs conditional checks against a byte at offset 259 of a structure in a1 before accessing another table.
- `update_font_index_and_offset`: Shifts a 16-bit value in register d4 left by two bits, subtracts a value from a table at a fixed address using the shifted result as an index, increments register d3 by its own value, increments register d4, and branches back to the start of the routine if d4 is less than seven; otherwise it branches to another location.
- `return_constant_value`: This library routine returns a constant value without using any arguments or registers.
- `update_font_glyph_metadata_2`: Copies two longwords from a source list into a local stack frame, calls another routine to obtain a font resource, and then performs a series of checks and updates to a font structure, including clearing a word at offset 256 and modifying various fields within a glyph structure based on specific bitwise conditions.
- `push_font_lock_data`: Pushes a value to the stack, calls a routine, stores the result in a location at offset minus twelve of a structure in a6, and then enters a loop that continues if a specific condition is met.
- `update_font_glyph_metadata`: Searches through a font data structure to find a glyph with a specific property, then updates its metadata, increments a counter, and sets a bit in a status byte.
- `frag_clr_moveq_bra`: Short code fragment that clears d4, then loads 255 into d2, then clears d3, then jumps to another point in the code.
- `search_and_update_font_metadata`: Iterates through a range of font indices to find the one with the largest value returned by a sub-routine, then updates a font structure by setting a word at offset 256 to zero, or if a specific constant is found at offset 22, updates several fields including a pointer at offset 26, a value at offset 72, and increments a counter at offset 92.
- `manage_font_resource_2`: As a library routine, it saves d4 to the stack, calls another routine, calculates an offset by multiplying a sign-extended d2 by 260, and clears a word at offset 256 of the resulting address.
- `manage_font_resource`: Calculates an offset into a font data structure using a multiplier of 260, clears a word at offset 256 of the resulting address, and performs conditional cleanup of memory, file handles, and graphics resources based on the input parameters.
- `build_disk_font_table`: Iterates through a list of font data to calculate a total size, allocates memory for a new table, and populates the table by copying specific byte values from the source font structures into the newly allocated memory.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_4`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
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
- `copy_null_terminated_string_3`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `frag_movea_move_bra`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `frag_movea_moveq_bra_2`: Short code fragment that copies the memory at an offset from a7 to a0, then loads 0 into d0, then jumps to another point in the code.
- `count_null_terminated_bytes`: As part of library code, this routine increments a counter while advancing a pointer and reading bytes until a zero byte is encountered, returning the final count.
- `frag_movea_bra_5`: Short code fragment that copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to a0, then jumps to another point in the code.
- `frag_tst_bne_moveq_bra`: Short code fragment that tests the memory at a0 (advancing a0), then branches if the last result was not zero, then loads 0 into d0, then jumps to another point in the code.
- `compare_bytes_and_branch`: As library code, it moves a byte from a pointer in a0 to d0, compares d0 with a byte from a pointer in a1, and branches if they are equal; otherwise, it moves a byte from the decremented a1 pointer to d0, compares it with the byte in a0, and branches if d0 is greater than or equal to that byte, or sets d0 to 255 and branches.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `frag_movea_move_bra_2`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `frag_cmp_bne_move_bra`: Short code fragment that compares d1 with d0, then branches if the last result was not zero, then copies a0 to d0, then jumps to another point in the code.
- `frag_addq_move_bne_moveq_rts`: Short code fragment that adds 1 to a0, then copies the memory at a0 to d1, then branches if the last result was not zero, then loads 0 into d0, then returns.
- `process_library_pointer_list`: As library code, it saves registers, loads a value from the stack into d3 and a byte into d2, calls another routine, and then calculates a new address by adding d3 to a pointer derived from d0 and returns.
- `frag_cmp_bne_move_bra_2`: Short code fragment that compares d2 with the memory at a0, then branches if the last result was not zero, then copies a0 to d0, then jumps to another point in the code.
- `decrement_and_compare_pointer`: As library code, it decrements a0, compares it to d3, and if a0 is greater than or equal to d3, it clears d0 and restores registers from the stack before returning; otherwise it returns.
- `process_pointer_list`: As a library routine, it saves registers, loads a value from the stack into d2, and calls another routine twice using values from the stack, updating d0 based on the results.
- `increment_returned_value`: This library routine returns the value passed in D0 incremented by one.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `frag_movem_jsr_addq_rts_4`: Short code fragment that saves registers on the stack, then calls another routine, then adds 8 to a7, then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Storm

- `parse_hunk_headers`: Processes Amiga DOS hunk headers from a memory block, allocating memory for each hunk and copying data into the allocated blocks.
- `decrement_and_check_2`: Decrements a 16-bit value in a register and checks if it underflows, branching to a specific label if the result is zero or negative.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `decode_bit_pattern`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution.
- `frag_lsr_beq_rts_move_roxr_3`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry_12`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `rotate_and_accumulate_4`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution based on shift flags.
- `write_port_word_2`: Writes a word value to a memory-mapped port address and performs a bitwise OR operation with another register before executing a trap instruction.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:Red

- `process_hunk_headers_4`: Reads and processes Amiga hunk headers from memory, allocating memory blocks for each hunk and copying data into them.
- `decrement_and_check_3`: Decrements a 16-bit value in a register and checks if it underflows, returning immediately if the result is zero.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `decode_bit_pattern_2`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution.
- `frag_lsr_beq_rts_move_roxr_4`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry_13`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `process_bits_and_accumulate`: Processes bits from a buffer through bitwise operations and accumulates results in a register, handling bit rotation and conditional execution.
- `load_and_or_2`: Loads a 16-bit value from a global variable at offset 3045, stores it in a register, then performs a bitwise OR operation between the value in another register and the constant 48.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_resource_manager`: Initializes a resource management structure by setting up global pointers, allocating memory, and preparing data structures for file operations.
- `cleanup_resource_structures`: Deallocates memory blocks by traversing a linked list and executes a series of function pointers stored within a global structure.
- `parse_and_initialize_input`: Parses a string input to extract tokens and initializes various internal structures and resource tables.
- `update_entity_state_structure`: Copies a large block of data from a source structure into a destination structure pointed to by a value in a table.
- `execute_callback_chain_6`: This routine retrieves a value from a global variable and executes a series of function pointers stored at various offsets within that value's structure.
- `execute_three_callbacks`: Checks three specific offsets within a structure for non-zero values and executes the function pointers found there.
- `store_callback_result`: Retrieves a value from a function call and stores it in a table at an offset determined by a parameter.
- `clear_callback_pointer`: Executes a function pointer at an offset from a base address determined by an index and then clears that pointer.
- `process_task_with_parameter`: This routine stores a passed parameter into a structure, initializes a process, and then executes a sequence of operations involving a release and wait mechanism.
- `process_task_with_offset`: This routine checks a global variable, initializes a process using a specific offset, and returns a value from a structure.
- `process_task_with_alternative_offset`: This routine performs process initialization and execution using a different set of structure offsets compared to other similar routines.
- `populate_structure_from_calls`: Repeatedly executes indirect function calls and stores the returned values into various offsets within a structure.
- `execute_multiple_callbacks`: Calls eleven different routines in sequence.
- `copy_data_fields`: Copies four word-sized values from a source structure to various offsets within a destination structure.
- `execute_callback_and_store_2`: Calls a function via a pointer from a global variable, stores the result in a table, and conditionally executes further routines and function pointers based on that result.
- `execute_callback_and_store_3`: Executes a function pointer from a global variable, saves the return value to a table, and performs a sequence of conditional function calls.
- `execute_callback_and_store_4`: Calls a function through a pointer, stores the return value in a table, and executes a series of function pointers from another structure if the initial result is non-zero.
- `execute_callback_and_store_5`: Invokes a function via a global pointer, records the return value in a table, and conditionally executes a chain of function calls.
- `execute_callback_and_store_6`: Calls a function pointer, stores the result in a table, and executes a sequence of function calls from a structure if the result is non-zero.
- `execute_callback_and_process_list`: Executes a function pointer and stores the result, then iterates through a nested list of structures to update internal pointers within each element.
- `call_conditional_pointers`: Checks two different offsets in a structure and calls a function pointer if the values are non-zero.
- `call_single_pointer`: Checks a short value at a specific offset and calls a function pointer if it is non-zero.
- `call_conditional_pointers_v2`: Checks a short and an integer offset in a structure and calls a function pointer if they are non-zero.
- `execute_conditional_function_pointers`: Checks two memory locations via an offset from a pointer; if the values at these locations are non-zero, it executes a function stored at a global address.
- `execute_conditional_function_pointers_2`: Checks two specific offsets within a structure for non-zero values.
- `execute_conditional_function_pointers_3`: Checks two consecutive memory locations for non-zero values and, if found, executes a function pointer stored at a fixed offset from a global variable.
- `fetch_and_store_values`: Calls a function three times to retrieve values, stores them in a table, and then stores a value derived from one of those entries into a structure.
- `initialize_resource_tables`: Allocates multiple memory blocks and populates structured tables with pointers and data retrieved from various function calls.
- `process_structure_callbacks`: This routine conditionally executes multiple function pointers, updates several memory locations, and clears specific structure fields.
- `release_stored_resources`: Checks several stored values and calls various functions to release or close resources based on whether those values are non-zero.
- `call_indirect_function_2`: Calls a function using a pointer located at an offset from a value stored in a table.
- `update_data_pointer`: Stores a word into a memory location and updates a long value through a pointer retrieved from a table.
- `frag_move_rts_4`: Short code fragment that copies the memory at an offset from a4 to d0, then returns.
- `frag_move_rts_5`: Short code fragment that copies the memory at an offset from a4 to d0, then returns.
- `fetch_table_longword_3`: Returns a longword from a table using an index provided in a parameter.
- `fetch_longword_from_table`: Reads the word argument from the stack, scales it by four, and returns the longword at that index in a table at a fixed offset from the global base register.
- `copy_and_execute_callbacks`: Updates several memory locations, copies thirty-two bytes from one area to another if a flag is set, and executes two function pointers from a table.
- `call_indirect_function_alt_2`: Calls a function using a pointer located at a specific offset from a value stored in a table.
- `process_message_queue_2`: Iterates through a message queue to extract specific values and updates several global variables based on the message contents.
- `frag_bsr_tst_bne_clr_rts_2`: Short code fragment that calls another routine, then tests d0, then branches if the last result was not zero, then clears the memory at an offset from a4, then returns.
- `frag_bsr_rts_3`: Short code fragment that calls another routine, then returns.
- `read_joystick_input`: Reads joystick movement and fire button data from hardware registers, processes the input through bitwise operations, and sets a global variable based on the processed values.
- `frag_moveq_bra_15`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `frag_moveq_bra_16`: Short code fragment that loads 2 into d5, then jumps to another point in the code.
- `frag_moveq_bra_17`: Short code fragment that loads 4 into d5, then jumps to another point in the code.
- `frag_moveq_bra_18`: Short code fragment that loads 8 into d5, then jumps to another point in the code.
- `frag_moveq_bra_19`: Short code fragment that loads 5 into d5, then jumps to another point in the code.
- `frag_moveq_bra_20`: Short code fragment that loads 9 into d5, then jumps to another point in the code.
- `frag_moveq_bra_21`: Short code fragment that loads 6 into d5, then jumps to another point in the code.
- `select_result_from_flags`: Clears a word in a global variable and returns 10, or 26 when the d6 argument is zero or a second global word is nonzero.
- `copy_data_block_2`: Sets a flag and copies thirty-two bytes of data from a source buffer to a destination buffer.
- `frag_move_movea_jsr_rts_2`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a4 to a6, then calls another routine, then copies the top of the stack to a6, then returns.
- `call_different_offset_function_2`: Calls a function pointer located at a different specific offset within a structure.
- `copy_strings_to_buffer`: This routine calculates the length of strings, copies them into a local buffer, and then executes a function pointer from a global variable.
- `process_callback_chain`: If a parameter is non-zero, it calls itself recursively; otherwise, it calls three different routines via a table, compares two values, and if they are equal, calls another routine, a third routine, and finally a fourth routine.
- `trigger_resource_cleanup`: Calls a routine that traverses a linked list of pointers to perform conditional cleanup operations on specific entries.
- `execute_conditional_sequence`: Calls a function via a pointer from a structure and, if the return value is zero, executes another routine with specific arguments.
- `execute_conditional_sequence_alt`: Calls a function via a pointer from a structure and, if the return value is zero, executes another routine with different arguments.
- `execute_complex_sequence_2`: Library code that performs a large sequence of operations including copying data, calling multiple function pointers from a structure, and interacting with file system routines.
- `execute_callback_chain_4`: Executes a function via a pointer from a structure and, based on the return value, calls another routine or performs further function calls using pointers from a table.
- `execute_two_functions_2`: Calls two different function pointers located at specific offsets within a structure pointed to by a global variable.
- `initialize_structure_data`: Calls a routine to obtain a pointer, populates a structure with data, and assigns a value from another routine to a field.
- `initialize_process_structure`: Creates a new process and populates a structure with various values, including a pointer, a constant, and an address from a global variable.
- `release_and_wait`: Calls three functions sequentially, involving operations on a global variable and a pointer from a table.
- `execute_callbacks_and_free_2`: Library code that checks several structure fields for non-zero values to execute corresponding function pointers and then calls a memory free routine.
- `link_structures_in_list`: Allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `execute_callbacks_and_free`: Checks several structure fields for non-zero values to execute corresponding function pointers and then calls a memory free routine.
- `wait_message_store_value`: Finds the current task, waits on its message port, takes the next message, stores the low word of d0 into the first long of the message data, replies to the message, and then calls a routine through a code pointer.
- `check_and_execute_callback_2`: Checks if a global variable is non-zero; if so, executes a function pointer from a global table, otherwise clears a global flag.
- `fetch_next_table_entry_2`: Checks if a global pointer is within bounds relative to another global variable; if so, it calls another routine and updates the pointer.
- `initialize_memory_buffers_2`: Writes a pattern of zeros and decrementing bytes into two memory regions and sets a global variable to a constant value.
- `process_callback_chain_3`: Performs bitwise shifts and logical operations, updates a pointer-based chain, and executes indirect function calls or callbacks.
- `update_dynamic_table_2`: Uses a loop to copy bytes from a source list into a destination table, updating several global values and pointers upon completion.
- `process_byte_sequence`: Reads bytes from a sequence, shifts them right by four bits, stores the high nibble and original byte in alternating positions, and manages counters and pointers.
- `frag_bsr_cmpi_beq_move_bra_2`: Short code fragment that calls another routine, then compares d1 with 144, then branches if the last result was zero, then copies d1 to d4, then jumps to another point in the code.
- `check_pointer_and_decrement_2`: Compares two pointers, and if they are not equal, moves a byte from the pointer in a1 to d1 and decrements d6 if d1 is not zero.
- `frag_move_bra_16`: Short code fragment that copies 144 to d4, then jumps to another point in the code.
- `store_and_shift`: Stores a byte value shifted right by four bits and then the original byte into a sequence, updates a counter, and continues processing until the counter reaches zero.
- `unpack_16bit_data`: Unpacks 16 bytes of input data into four separate 16-bit values by shifting and rotating bits, then stores these values into four different memory locations.
- `unpack_20bit_data`: Unpacks 16 bytes of input data into five separate 16-bit values by shifting and rotating bits, then stores these values into five different memory locations.
- `unpack_and_dispatch`: Extracts 16 bits of data from a source buffer, distributes them to multiple destination pointers, and executes a function from a table based on a global counter.
- `initialize_asset_buffer`: Clears a 79-word memory region starting at a fixed address with zeros, initializes several global variables based on bit tests of a return value from another routine, and populates a secondary buffer with data fetched from a table.
- `copy_and_dispatch_data`: Checks if a pointer is null, copies a block of data from a source to a global destination, and executes a function pointer from a table if specific conditions are met.
- `dispatch_function_pointer_3_2`: Saves registers, retrieves a value from a pointer at an offset from a5, calls a routine via a pointer from a table, and returns zero if the result is zero.
- `update_asset_buffer`: Sets three global variables using values from d0 and a calculated address, calls another routine, and then executes a function via a pointer retrieved from a memory location.
- `set_global_range_params_2_2`: Clears a global longword, stores a value from a structure into a second global longword, stores the sum of that value and a second value from the structure into a third global longword, and then calls another routine.
- `copy_table_from_address_2`: Copies a sequence of 32-bit words from a fixed memory address into a destination buffer.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `tokenize_string_input_5`: Parses a null-terminated string into an array of pointers by splitting on whitespace or handling substrings enclosed in double quotes.
- `fall_through_entry_14`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `transform_byte_buffer`: Iterates through a buffer and, if a bit is set in a lookup table, subtracts 32 from the byte value.
- `frag_moveq_move_addq_bra`: Short code fragment that loads 0 into d1, then copies d0 to d1, then copies d1 to the memory at a2, then adds 1 to a2, then jumps to another point in the code.
- `function_epilogue_19`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `append_path_component`: Copies a string to a destination, checks if the destination does not end in a path separator, and if so, appends a character from a global variable before appending a third string.
- `calculate_string_length_2`: The routine receives a pointer to a zero-terminated byte string and advances a pointer until it finds the terminating zero byte.
- `append_second_string_to_first`: Finds the end of the first string, copies bytes from the second string there including the terminator, and returns the original first pointer in d0.
- `frag_clr_move_bra_3`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `format_number_as_reversed_string`: Calls another routine in a loop, writing each returned digit as an ASCII character into a stack buffer, then copies the characters back in reverse order into the output pointer, null-terminates it, and returns the length of the string.
- `frag_clr_move_bra_4`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `format_number_as_octal`: Converts an unsigned value to octal digits by repeatedly taking its low three bits, stores the digits in a temporary stack buffer, then copies them in reverse order into the buffer given in the first argument, terminates the string with a zero byte, and returns the digit count.
- `format_uint_as_hex_string`: Converts the unsigned long in the second argument to lowercase hexadecimal digits using a lookup table, writes them reversed into the caller's buffer, null-terminates it and returns the character count.
- `parse_signed_integer_2`: Checks for a sign character at the start of a string, then parses a sequence of decimal digits into a long integer, returning the number of bytes processed.
- `handle_sign_and_length`: Checks for a minus sign at the current position in a1, negates the value in d1 when present, restores d2, and returns the number of bytes consumed in d0.
- `append_byte_to_list_2`: Increments a counter in a structure and stores a byte into a buffer pointed to by another field in that structure, then advances that pointer.
- `format_string_buffer_2`: Clears a value in a structure, stores a parameter into a field, calls a string parsing routine, and then clears that field again.
- `calculate_target_offset`: This routine calls another routine to obtain a value and then performs conditional checks against a global variable to determine a return value.
- `manage_memory_map`: Searches a linked list of memory blocks to find, split, or allocate a block of a specific size.
- `cleanup_resource_list_2`: This routine iterates through a linked list of pointers and calls two different routines for each entry that meets specific bitwise and value criteria.
- `execute_resource_cleanup`: This routine checks a flag in a structure, calls a cleanup routine if necessary, and then executes a function pointer from a table, potentially handling an error state by updating structure members.
- `calculate_file_offset`: This routine performs resource cleanup and then calculates a target offset based on three input parameters and the result of a function call from a table.
- `initialize_structure_fields_2`: It writes a full-byte bitmask at a fixed offset of a structure and writes full-word bitmasks at two other offsets.
- `free_msgport_signal_and_memory`: Removes the message port from the exec list if its name is set, marks it invalid, frees its signal bit and releases its thirty-four byte memory block.
- `close_file_resource`: Checks a global variable and calls a function pointer from a table to close a resource.
- `compute_fixed_point_product_2`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `divide_32bit_by_16bit`: Performs 32-bit division by a 16-bit divisor with sign handling, normalizing the divisor using bit shifts and utilizing divu and mulu instructions to compute the quotient.
- `unsigned_32bit_divide_by_16bit`: Divides the 32-bit value in d0 by the 16-bit value in d1, normalizing the divisor with shifts and using divu and mulu, returning quotient in d0 and remainder in d1.
- `allocate_and_set_node`: It allocates memory for a structure through a library call and, if successful, initializes certain bytes and a dword with a provided parameter.
- `allocate_and_initialize_structure_2`: The routine allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `lookup_table_index`: The routine validates an index against a range and checks a table entry, returning either a calculated address or a specific error value.
- `free_linked_list_nodes_2`: Checks a pointer from a structure in a4, and if not null, calls a routine using a pointer from a global variable to free the memory pointed to by the structure's contents, then clears two pointers at offsets 10692 and 10696 in a4.
- `append_to_linked_list`: Allocates memory for a new node using a library call, initializes its fields, and inserts it into several linked lists maintained in a global structure.
- `evaluate_resource_status`: Calls another routine to obtain a value and performs conditional checks against a global variable and bit flags to determine a return value.
- `parse_format_specifier_string_3`: Parses a format string to identify type specifiers such as decimal, hexadecimal, or long values, updating a structure on the stack with flags for sign, precision, and width.
- `frag_moveq_bra_22`: Short code fragment that loads 1 into d6, then jumps to another point in the code.
- `frag_moveq_bra_23`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `frag_move_addq_bra`: Short code fragment that copies 1 to the memory at an offset from a5, then adds 1 to a3, then jumps to another point in the code.
- `parse_format_specifier_string_2`: Checks if the current character in a string is the character '0' and, if so, increments the pointer and writes the value 48 to a structure offset.
- `parse_format_specifier_string`: Calls parse_signed_integer_2 to convert a string to a long integer, then checks for a period followed by an asterisk or specific type characters such as 'l', 'h', 'X', 'c', 'd', or 'p' to update a structure in a5 and advance a pointer in a2.
- `parse_opcode_and_dispatch_2`: Calls a routine with a pointer from a structure, increments a pointer in a3 by the returned value, and then compares a byte at the new pointer location to a constant, setting a byte in a structure and incrementing the pointer if they match.
- `parse_opcode_and_dispatch`: Compares a byte from a pointer in a3 against a limit, loads the byte into d1, stores it in a structure at offset negative 12 of a5, and branches to different routines based on its value.
- `check_buffer_value`: Tests a byte at a negative offset from a pointer in a5 and, if it is not zero, loads a longword from a pointer in a0 and increments the pointer in a2 by four.
- `parse_signed_buffer_value`: Reads a 16-bit word from a pointer in a2, increments that pointer by two, and stores the value in a structure at a5 offset negative sixteen.
- `frag_tst_beq_moveq_bra`: Short code fragment that tests d6, then branches if the last result was zero, then loads 43 into d0, then jumps to another point in the code.
- `process_recursive_buffer_update`: Increments two counters in a structure and calls another routine to process a buffer, then performs a recursive call to handle memory offsets and fills a memory block with a character.
- `copy_and_update_counter`: Copies a byte from a pointer in a0 to memory and increments the pointer, then updates a 16-bit value at offset -14 of a structure in a5 using a value from offset -20 of the same structure, and finally sets a byte at offset -5 of the structure to 32 if a specific bit in d7 is set.
- `check_buffer_value_2`: Tests a byte at a negative offset from a pointer in a5 and, if it is not zero, loads a longword from a pointer in a0 and increments the pointer in a2 by four.
- `increment_counter_and_store_word`: Increments a longword value in a structure pointed to by a2 by two, then reads a word from the address in a0 and stores it into a global variable at an offset of negative sixteen from a5 before branching to another routine.
- `frag_tst_beq_movea_addq_move_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at a2 to a0, then adds 4 to the memory at a2, then copies the memory at a0 to d0, then jumps to another point in the code.
- `append_digit_to_ascii_buffer`: Reads a 16-bit word from a pointer in a register, stores it in a structure, and if a flag is set, appends the ASCII character '0' to a string buffer and calls another routine to process the resulting string.
- `update_status_and_pointer`: Writes the byte 48 to an offset of negative 5 from a5, loads a word from an offset of negative 10 from a5 into d0, and if d0 is positive, writes the word 8 to an offset of negative 10 from a5; if a byte at an offset of negative 11 from a5 is non-zero, it loads a pointer from a2 into a0, increments the value at a2 by 4, and moves a longword from the address in a0 into d0.
- `process_recursive_buffer_update_2`: Reads a 16-bit value from a pointer in a2, increments that pointer by 2, and stores the value in a structure.
- `calculate_string_length_and_decrement`: Increments a pointer in a2, loads a value from the new pointer into a1, and stores it in a structure in a5.
- `reset_counter_and_copy_byte`: Sets a 16-bit value at offset -14 of a structure in a5 to 1, increments a pointer in a2, copies a byte from the new pointer to offset -28 of the structure, clears offset -27 of the structure, and updates the value at offset -8 of the structure by either setting it to zero or subtracting 1.
- `frag_moveq_bra_24`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `compare_and_reset_counter`: Compares two 16-bit values at offsets of negative 20 and negative 8 from a pointer in a5, and if the first is not greater than or equal to the second, sets the value at offset negative 8 to zero.
- `execute_callback_loop`: Decrements a 16-bit value at an offset of -8 from a pointer in a5 and, if the value remains positive, calls a function pointer from a table while updating the pointer and a counter.
- `loop_and_call_routine`: Decrements a value on the stack, and if not below zero, copies a byte from a stack offset to d0, pushes it to the stack, calls a routine from a pointer at offset 16 of a5, and loops.
- `loop_and_call_routine_2`: Decrements a value on the stack, and if not below zero, copies a byte from a stack offset to d0, pushes it to the stack, calls a routine from a pointer at offset 16 of a5, and loops.
- `loop_and_call_with_byte`: Decrements a value on the stack, and if not below zero, copies a byte from a pointer at offset 44 of a5 to d0, pushes it to the stack, updates the pointer at offset 44 of a5, calls a routine from a pointer at offset 16 of a5, and loops.
- `function_epilogue_20`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `parse_format_string`: Iterates through a string looking for percent signs to identify format specifiers and calls a routine to process them.
- `call_with_pointer`: Pushes a pointer and a value onto the stack, calls another routine, and then either stores a value from d0 into a memory location or updates a pointer in a2 and branches.
- `call_routine_with_byte`: Copies a byte from d7 to d0, pushes d0 to the stack, calls a routine at the address in a3, increments the stack pointer by 2, and loops.
- `function_epilogue_21`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `execute_callback_list_2`: Iterates through a list of pointers in a table and executes functions if they meet specific bitwise criteria, then calls another routine with a provided parameter.
- `insert_into_memory_map`: The routine manages a list of memory blocks, inserting a new block or merging it with existing adjacent blocks to maintain a sorted map.
- `copy_memory_block_with_offset`: Copies a block of memory from a source pointer to a destination pointer by moving bytes backward using a word-sized counter, while applying a word-sized offset to both pointers if the destination address is lower than the source address.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a1 one byte at a time, using a dbra loop counter, and returns.
- `allocate_memory_block`: Manages a memory map by checking for an existing resource and recursively searching for a block of a specific size, updating pointers and returning a pointer to the allocated block.
- `check_and_clear_signal_flag`: Sets signal bits in the operating system, checks for specific signal conditions, calls a function if certain conditions are met, and performs a bitwise rotation on a value in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_table`: Initializes a data table by loading 16-bit values from memory into data registers and storing them back to the same locations, effectively clearing or resetting the table entries.

### Red Storm Rising v2 (1990)(MicroProse)[cr SKR][h LSD].adf:RedStorm#unpacked

- `initialize_system`: Sets up system resources by initializing global variables, opening necessary libraries, and preparing memory structures for subsequent operations.
- `cleanup_resource_structures_2`: Checks a global variable for a non-zero value, and if so, calls a routine using a function pointer from another global variable, then performs memory deallocation operations including closing a library, exiting an interrupt, forbidding interrupts, and replying to a message.
- `parse_and_initialize_input_2`: Parses string input to extract tokens and initializes internal structures and resource tables.
- `update_entity_state_structure_2`: Copies a large block of data from a source structure into a destination structure pointed to by a value in a table.
- `execute_callback_chain_6_2`: This routine retrieves a value from a global variable and executes a series of function pointers stored at various offsets within that value's structure.
- `execute_three_callbacks_2`: Checks three specific structure offsets for non-zero values and executes corresponding function pointers found at those offsets.
- `store_callback_result_2`: Retrieves a value from a function call and stores it in a table at an offset determined by a parameter, with error handling for null results.
- `clear_callback_pointer_2`: Executes a function pointer at an offset from a base address determined by an index and then clears that pointer.
- `initialize_and_execute_task`: Stores a passed parameter into a structure, initializes a process, and then executes a sequence of operations involving a release and wait mechanism.
- `process_task_with_offset_2`: Checks a global variable, initializes a process using a specific offset, and returns a value from a structure.
- `process_task_with_alternative_offset_2`: Performs process initialization and execution using a different set of structure offsets compared to other similar routines.
- `populate_structure_from_calls_2`: Repeatedly executes indirect function calls and stores the returned values into various offsets within a structure, performing file operations for each call.
- `execute_multiple_callbacks_3`: Calls eleven different routines in sequence.
- `copy_data_fields_2`: Copies four word-sized values from a source structure to various offsets within a destination structure.
- `allocate_and_read_file`: Allocates memory using a global variable, opens a file, reads data into the allocated memory, and closes the file.
- `read_file_to_memory`: Allocates memory, opens a file, reads data into the allocated memory, and closes the file.
- `load_and_seek_file`: Allocates memory, opens a file, seeks to a specific position, reads data into the allocated memory, and closes the file.
- `read_file_to_memory_2`: Allocates memory, opens a file, reads data into the allocated memory, and closes the file.
- `read_file_to_memory_3`: Allocates memory, opens a file, reads data into the allocated memory, and closes the file.
- `load_and_relink_data`: Loads data from a file into a global buffer and updates pointer references within a nested structure, then returns a boolean indicating success.
- `free_conditional_pointers`: Checks two offsets in a structure for non-zero values and frees memory pointed to by those offsets using a global function pointer.
- `call_single_pointer_2`: Checks a short value at a specific offset and calls a function pointer if it is non-zero.
- `free_conditional_memory`: Checks a short and an integer offset in a structure and frees memory if they are non-zero.
- `free_memory_if_not_zero`: Checks two global variables for non-zero values and frees memory blocks pointed to by those variables using a fixed size.
- `free_memory_if_nonzero`: Checks two specific offsets within a structure for non-zero values.
- `free_memory_if_nonzero_2`: Checks a global variable for non-zero value, and if found, frees memory at an offset from another global variable.
- `fetch_and_store_values_2`: Calls a function three times to retrieve values, stores them in a table, and then stores a value derived from one of those entries into a structure.
- `populate_table_entries`: Initializes multiple data structures by allocating memory blocks and populating table entries with pointers and values retrieved from function calls, including error handling for failed allocations.
- `execute_structure_callbacks`: Conditionally executes function pointers from a table, updates memory locations with word and dword values, and clears specific structure fields while managing memory segments and device handles.
- `release_resources`: Checks multiple global pointers and calls CloseLibrary for each non-zero pointer, then allocates memory using AllocMem.
- `frag_move_movea_moveq_jsr_rts`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a4 to a1, then loads 0 into d0, then copies the memory at an offset from a4 to a6, then calls another routine, then copies the top of the stack to a6, then returns.
- `update_data_pointer_2`: Stores a word into a memory location and updates a long value through a pointer retrieved from a table.
- `frag_move_rts_6`: Short code fragment that copies the memory at an offset from a4 to d0, then returns.
- `frag_move_rts_7`: Short code fragment that copies the memory at an offset from a4 to d0, then returns.
- `fetch_longword_from_table_2`: Reads the word argument from the stack, scales it by four, and returns the longword at that index in a table at a fixed offset from the global base register.
- `fetch_longword_from_table_3`: Reads the word argument from the stack, scales it by four, and returns the longword at that index in a table at a fixed offset from the global base register.
- `update_and_execute`: Updates a global variable with a word value from the stack, writes a dword to a structure member, and conditionally copies thirty-two words from one memory region to another if a flag is set.
- `frag_movem_move_ext_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to d7, then copies d7 to d0, then sign-extends d0, then copies the memory at an offset from a4 to a1, then copies the memory at an offset from a4 to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `prepare_and_call_dos_open_2`: Extracts six parameters from a structure, sign-extends them to 32-bit values, pushes them onto the stack along with two constants, and calls another routine.
- `execute_dual_calls_2`: Calls two functions sequentially using pointers retrieved from a table.
- `process_sequence_with_loop`: Initializes a loop counter and performs a series of comparisons and memory operations based on the counter value, updating a global variable and calling a routine with a calculated address.
- `store_word_and_run_sequence`: Copies a word from a global variable into a stack-relative slot, calls a data pointer update routine, calls a DOS library function at offset -306, then calls the data pointer update routine again.
- `call_input_seek_and_return_2`: Loads a pointer from offset 4312 from the base register, calls a function at offset -66 through it, then calls the function at offset -54 and keeps its 32-bit result.
- `frag_move_bsr_addq_rts_3`: Short code fragment that copies the memory at an offset from a4 to the stack, then calls another routine, then adds 4 to a7, then returns.
- `frag_move_bsr_addq_rts_4`: Short code fragment that copies a fixed memory address to the stack, then calls another routine, then adds 4 to a7, then returns.
- `process_input_request`: Clears a byte in a structure and writes a fixed length into another field, then sends an I/O request through the exec library and waits for a response.
- `poll_input_and_scan_bitmask`: Clears a byte in a structure and writes a fixed length into another field, then sends an I/O request through the exec library and waits for it to finish.
- `frag_tst_bne_addq_bra_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then adds 1 to d7, then jumps to another point in the code.
- `store_stack_byte_as_word`: Reads a byte from the stack frame at offset -13, stores it as a word into a global variable, and returns the zero-extended byte in d0.
- `process_message_queue`: Iterates through a message queue, extracts values from messages, and updates global variables based on message contents.
- `frag_bsr_tst_bne_clr_rts_3`: Short code fragment that calls another routine, then tests d0, then branches if the last result was not zero, then clears the memory at an offset from a4, then returns.
- `frag_bsr_rts_4`: Short code fragment that calls another routine, then returns.
- `read_joystick_input_2`: Reads joystick port movement and fire button status from hardware registers, processes the input into a bitmask, and returns the result.
- `frag_moveq_bra_25`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `frag_moveq_bra_26`: Short code fragment that loads 2 into d5, then jumps to another point in the code.
- `frag_moveq_bra_27`: Short code fragment that loads 4 into d5, then jumps to another point in the code.
- `frag_moveq_bra_28`: Short code fragment that loads 8 into d5, then jumps to another point in the code.
- `frag_moveq_bra_29`: Short code fragment that loads 5 into d5, then jumps to another point in the code.
- `frag_moveq_bra_30`: Short code fragment that loads 9 into d5, then jumps to another point in the code.
- `frag_moveq_bra_31`: Short code fragment that loads 6 into d5, then jumps to another point in the code.
- `select_result_from_flags_2`: Clears a word in a global variable and returns 10, or 26 when the d6 argument is zero or a second global word is nonzero.
- `build_frame_pointer_table_3`: Reads two bytes through a pointer loaded from a stack slot, stores their product and the second byte in stack slots, then fills six pointer values.
- `build_frame_pointer_table_2`: Reads two bytes through a pointer loaded from a stack slot, stores their product and the second byte in stack slots, then fills six pointer entries in a stack table with offsets into that data.
- `shift_word_and_call_dos_2`: Shifts the word at offset -6 from a5 left by 3 and stores it back, then calls a DOS wrapper routine with a buffer address at a5 minus 48, zeros, a longword from offset 4336 from a4, the registers d6 and d5, the shifted word, the word at offset -8 from a5, and the constant 224.
- `dispatch_and_store_changed_value`: Calls a DOS library function at a fixed offset from a global structure pointer, chosen by comparing the parameter to a global longword, and then writes the parameter into another global longword only when the stored value differs from it.
- `conditionally_call_dos_function`: Compares a3 with a zero longword and, when they differ, loads two pointers from global structure offsets, sets d0 to 7, d1 to 7, d2 to -3 and d3 to -3, and calls DOS offset -270 through a6, then otherwise skips that call.
- `store_d3_into_global_if_changed`: Calls a DOS library function at offset -270 and then stores the value of a register into a global variable only when a global longword differs from it.
- `load_segment_from_file`: When a global flag is zero, opens a named file, seeks to the start offset from a table indexed by a word argument, reads the segment length into a buffer, closes the file and uses that buffer; otherwise it computes the segment start from the table and adds a global offset.
- `load_file_segment_into_buffer`: Opens a named file, computes the start and length of a segment from a table of offsets, seeks to the start and reads that length into a buffer, closes the file, and then records the segment range and copies data blocks.
- `execute_indirect_function_call_2_2`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `execute_indirect_function_call_3`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `set_flag_and_copy`: Sets a 16-bit value to 1 at offset 1500 of a global structure and copies thirty-two 16-bit values from a source buffer to a destination buffer.
- `frag_move_movea_jsr_rts_3`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a4 to a6, then calls another routine, then copies the top of the stack to a6, then returns.
- `call_function_pointer`: Reads a function pointer from a structure at a specific offset and executes it, then returns.
- `copy_strings_to_buffer_2`: Calculates the length of strings, copies them into a local buffer, and then executes a function pointer from a global variable.
- `check_task_identity`: Compares two task pointers retrieved from a global table and sets a flag to 1 if they are equal, then calls another routine to handle cleanup operations.
- `trigger_resource_cleanup_2`: Calls a routine that traverses a linked list of pointers to perform conditional cleanup operations on specific entries.
- `merge_bitmask_buffers_3`: Calculates memory offsets from a table and performs a bitwise OR operation to merge multiple data buffers into a destination buffer.
- `load_and_merge_bitmask_buffers`: Calls a routine to open a file, reads data into a structure, and calculates a series of memory offsets.
- `merge_bitmask_buffers_2`: Calculates a new address, calls another routine, and then enters a loop to copy bytes from one location to another while incrementing a counter.
- `frag_addq_bra_3`: Short code fragment that adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `merge_bitmask_buffers`: Increments a value at a negative offset from a5, and if it is less than 6, enters a loop that performs bitwise OR operations between two memory buffers using lengths retrieved from a global structure.
- `frag_addq_bra_4`: Short code fragment that adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_addq_bra_5`: Short code fragment that adds 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_movem_unlk_rts_2`: Short code fragment that copies d7 to the memory at an offset from a4, then restores saved registers from the stack, then removes the stack frame held in a5, then returns.
- `position_and_draw_bitmap`: Compares a stored word to the first argument and calls a merge routine when they differ, then computes a halved horizontal offset from a global word times 8 and calls a library function with thirteen-argument-style values from globals and parameters.
- `lock_path_and_release`: Attempts to lock the path string at a fixed offset in the data area with an access mode of -2, calls another routine with a pointer to the lock result when the lock fails, then unlocks the handle if it is nonzero and returns whether it is nonzero.
- `lock_and_process_file_2`: Acquires a file lock using dos.Lock with a specific path and parameter, processes the locked file if successful, and then releases the lock with dos.UnLock.
- `lock_and_process_segment`: Acquires a file lock using dos.Lock with another path and parameter, processes the locked segment if successful, and then releases the lock with dos.UnLock.
- `unpack_bits_2`: Extracts bits from a source buffer and distributes them into multiple destination words across several memory locations.
- `process_file_segment`: Reads a segment from a file starting at an offset calculated from a table, then closes the file and calls routines based on values from another table.
- `frag_move_movea_jsr_rts_4`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a4 to a0, then copies the memory at an offset from a4 to a6, then calls another routine, then copies the memory at an offset from a4 to a0, then calls another routine, then copies the top of the stack to a6, then returns.
- `load_segment_and_link`: Loads a segment using a global function, stores the result in a structure field, and handles error cases by freeing memory and returning null.
- `initialize_process_structure_2`: Creates a new process using a global function and stores the resulting message port at offset 10 of the structure in a5.
- `release_and_wait_2`: Calls three functions sequentially, involving operations on a global variable and a pointer from a table.
- `execute_callbacks_and_free_2_2`: Library code that checks several structure fields for non-zero values to execute corresponding function pointers and then calls a memory free routine.
- `link_structures_in_list_2`: Allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `free_structure_fields`: Checks specific structure fields for non-zero values and frees memory at those addresses with varying sizes, then frees the structure itself.
- `wait_message_store_value_2`: Finds the current task, waits on its message port, takes the next message, stores the low word of d0 into the first long of the message data, replies to the message, and then calls a routine through a code pointer.
- `check_and_execute_callback_3`: Checks a global variable; if non-zero, executes a function pointer from a table, otherwise clears a global flag.
- `fetch_next_table_entry_3`: Checks if a global pointer is within bounds relative to another global variable; if so, it calls another routine and updates the pointer.
- `fill_memory_pattern`: Writes a sequence of decrementing words starting with -1 into a memory region, followed by a sequence of bytes starting with 255 down to 0 into another memory region, then sets a global variable to 511.
- `update_bit_table_entry`: Updates a bit table entry based on input data and performs bitwise operations to adjust table values.
- `update_dynamic_table_entry_2`: Copies a byte value between memory locations, updates a word-sized entry in a separate table, and conditionally increments a counter and updates a value before potentially calling another routine.
- `process_data_with_conditions`: Reads values from multiple global variables, performs conditional operations based on test results, and copies data between memory locations using pointer arithmetic.
- `frag_bsr_tst_bmi_move_bra_2`: Short code fragment that calls another routine, then tests d1, then branches if the last result was negative, then copies d1 to d4, then jumps to another point in the code.
- `frag_cmpa_beq_move_subq_bra_2`: Short code fragment that compares a1 with a5, then branches if the last result was zero, then copies the memory just below a1 (moving a1 down) to d6, then subtracts 2 from d6, then jumps to another point in the code.
- `update_bit_table_and_jump`: Calls another routine to update a bit table entry, performs conditional operations on data values, stores multiple values to global variables, and then jumps to an address stored in a global variable.
- `decode_bits_and_store`: Processes bits from a buffer, performs bit manipulation and stores results into a table.
- `frag_bsr_cmpi_beq_move_bra_3`: Short code fragment that calls another routine, then compares d1 with 144, then branches if the last result was zero, then copies d1 to d4, then jumps to another point in the code.
- `frag_cmpa_beq_move_subq_bra_3`: Short code fragment that compares a1 with a5, then branches if the last result was zero, then copies the memory just below a1 (moving a1 down) to d1, then branches if the last result was zero, then copies d1 to d6, then subtracts 2 from d6, then jumps to another point in the code.
- `frag_move_bra_17`: Short code fragment that copies 144 to d4, then jumps to another point in the code.
- `process_bits_and_update_table`: Calls another routine to update a bit table entry, checks if a value is zero and branches if so, performs bit shifting operations on data values, stores multiple values to global variables, and then jumps to an address stored in a global variable.
- `rotate_and_store_words`: Processes byte data by rotating bits into word registers, stores the resulting words to memory areas, and updates pointers for subsequent iterations.
- `rotate_and_store_five_words`: Processes byte data by rotating bits into five word registers, stores the resulting words to memory areas, and updates pointers for subsequent iterations.
- `unpack_bits`: Extracts bits from a source buffer and distributes them into multiple destination words across several memory locations.
- `initialize_asset_buffer_2`: Clears a memory region with zeros and populates a secondary buffer by fetching entries from a table.
- `set_global_range_params_3`: Sets up global parameters related to range calculations, including updating function pointers based on conditional logic.
- `frag_movea_jmp_2`: Short code fragment that copies a fixed memory address to a0, then jumps to the address in a0.
- `decrement_and_branch_2_2`: Decrements a word in a global variable and branches if it is not zero; otherwise it returns.
- `frag_link_movem_move_movea_jsr_tst_bne`: Short code fragment that creates a stack frame in a5, then saves registers on the stack, then copies the memory at an offset from a5 to d1, then copies 1005 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then branches if the last result was not zero, then loads 0 into d0, then jumps to another point in the code.
- `set_buffer_parameters_and_execute`: Stores a value to a global variable, calculates an address and stores it to two global variables, calls another routine with that address, retrieves a value from a global variable, executes a function via a pointer from memory, and returns a word value.
- `set_global_range_params`: Stores two integer parameters into consecutive global variables, where the second global variable contains the sum of the provided parameters, and then calls another routine.
- `copy_table_from_address_3`: Copies a sequence of 32-bit words from a fixed memory address into a destination buffer.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `tokenize_string_input_2`: Splits a string into an array of pointers by using whitespace as delimiters or by grouping characters contained within double quotes, replacing delimiters with null terminators.
- `fall_through_entry_15`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `transform_byte_buffer_2`: Iterates through a buffer and, if a bit is set in a lookup table, subtracts 32 from the byte value.
- `frag_moveq_move_addq_bra_2`: Short code fragment that loads 0 into d1, then copies d0 to d1, then copies d1 to the memory at a2, then adds 1 to a2, then jumps to another point in the code.
- `function_epilogue_22`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `append_path_component_2`: Copies a source string to a destination, checks if the destination does not end in a path separator, and if so, appends a character from a global variable before appending a third string.
- `strlen_return_length`: Walks the string pointer passed on the stack until a null byte, then returns the character count in d0.
- `append_second_string_to_first_2`: Finds the end of the first string, copies bytes from the second string there including the terminator, and returns the original first pointer in d0.
- `frag_clr_move_bra_5`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `format_number_as_reversed_string_2`: Calls another routine in a loop, writing each returned digit as an ASCII character into a stack buffer, then copies the characters back in reverse order into the output pointer, null-terminates it, and returns the length of the string.
- `frag_clr_move_bra_6`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.
- `format_number_as_octal_2`: Converts an unsigned value to octal digits by repeatedly taking its low three bits, stores the digits in a temporary stack buffer, then copies them in reverse order into the buffer given in the first argument, terminates the string with a zero byte, and returns the digit count.
- `format_uint_as_hex_string_2`: Converts the unsigned long in the second argument to lowercase hexadecimal digits using a lookup table, writes them reversed into the caller's buffer, null-terminates it and returns the character count.
- `parse_numeric_value`: Parses a numeric value from a string starting at the address in a0, handling optional sign characters and converting digits to an integer result stored in d1.
- `handle_sign_and_length_2`: Checks for a minus sign at the current position in a1, negates the value in d1 when present, restores d2, and returns the number of bytes consumed in d0.
- `append_byte_to_buffer`: Increments a 16-bit global variable counter, then stores the low byte of the word parameter at the pointer held in a global variable and advances that pointer by one byte.
- `format_string_wrapper`: Stores parameters into a global structure, calls a formatting routine, and then returns a value from that structure.
- `calculate_target_offset_2`: Retrieves a value from another routine and performs conditional checks against a global variable to determine the return value.
- `manage_memory_map_2`: Searches a linked list of memory blocks to find, split, or allocate a block of a specific size.
- `cleanup_resource_list_3`: This routine iterates through a linked list of pointers and calls two different routines for each entry that meets specific bitwise and value criteria.
- `execute_resource_cleanup_2`: Checks a flag in a structure, calls a cleanup routine if necessary, and then executes a function pointer from a table, potentially handling an error state by updating structure members.
- `seek_and_offset`: Performs a file seek operation and calculates an offset based on input parameters and the result of the seek.
- `clear_fields_and_free_memory`: Writes the byte 255 at offset 8 and the longword an address at offsets 20 and 24 of the structure passed on the stack, then frees a 48-byte block at that address through the exec library.
- `free_msgport_signal_and_memory_2`: Removes the message port from the exec list if its name is set, marks it invalid, frees its signal bit and releases its thirty-four byte memory block.
- `close_file_resource_3`: Checks a global variable and calls a function pointer from a table to close a resource.
- `multiply_fixed_point_halves`: Multiplies the high and low 16-bit halves of d0 and d1 as signed and unsigned words, sums the two cross products, shifts that sum into the upper 16 bits, and adds the low-word product, leaving the combined value in d0.
- `compute_signed_division`: Computes the division of a 32-bit signed value by a 16-bit signed value, handling negative inputs and returning the quotient in d0.
- `unsigned_32bit_divide_by_16bit_2`: Divides the 32-bit value in d0 by the 16-bit value in d1, normalizing the divisor with shifts and using divu and mulu, returning quotient in d0 and remainder in d1.
- `allocate_and_set_node_2`: Allocates a memory block through the system routine and, when it succeeds, stores a type value of five, clears one byte and records the argument in the node.
- `allocate_and_initialize_structure_3`: The routine allocates memory for a structure, populates its fields with provided arguments and system values, and inserts it into a linked list.
- `get_table_entry_address`: It checks if an index is within valid bounds and if a corresponding entry in a table is non-zero, returning the calculated address of that entry or a default value.
- `free_linked_list_nodes_3`: Checks a pointer from a structure in a4, and if not null, calls a routine using a pointer from a global variable to free the memory pointed to by the structure's contents, then clears two pointers at offsets 10692 and 10696 in a4.
- `append_to_linked_list_2`: Allocates memory for a new node using a library call, initializes its fields, and inserts it into several linked lists maintained in a global structure.
- `evaluate_resource_status_2`: Calls another routine to get a value, checks if the result is zero and branches if so, tests a bit flag in the result, performs conditional operations based on the flag, calls multiple routines with parameters, and returns either a specific value or the result of another operation.
- `parse_format_specifier_string_3_2`: Parses a format string to identify type specifiers such as decimal, hexadecimal, or long values, updating a structure on the stack with flags for sign, precision, and width.
- `frag_moveq_bra_32`: Short code fragment that loads 1 into d6, then jumps to another point in the code.
- `frag_moveq_bra_33`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `frag_move_addq_bra_2`: Short code fragment that copies 1 to the memory at an offset from a5, then adds 1 to a3, then jumps to another point in the code.
- `parse_format_specifier_2`: Processes a format specifier string, parsing flags, width, and precision, then stores parsed values into a structure at offset -5 from a5.
- `parse_format_specifier_3`: Processes a format specifier string, parsing width and precision modifiers, handling conversion specifiers 'c', 'd', 'o', 'p', 's', 'u', 'x', and 'X', and updating global variables with parsed values.
- `parse_opcode_and_dispatch_2_2`: Calls a routine with a pointer from a structure, increments a pointer in a3 by the returned value, and then compares a byte at the new pointer location to a constant, setting a byte in a structure and incrementing the pointer if they match.
- `parse_format_character`: Reads a character from a buffer and processes it based on its value, updating various counters and memory locations accordingly.
- `frag_tst_beq_movea_addq_move_bra_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at a2 to a0, then adds 4 to the memory at a2, then copies the memory at a0 to d0, then jumps to another point in the code.
- `update_counter_and_copy`: Updates a 32-bit counter and a 16-bit value in a structure, then performs conditional copying of a byte value into a memory block based on the counter's sign, and finally adjusts two 16-bit values in the structure.
- `frag_tst_beq_moveq_bra_2`: Short code fragment that tests d6, then branches if the last result was zero, then loads 43 into d0, then jumps to another point in the code.
- `update_and_copy_counter`: Updates a 32-bit counter and a 16-bit value in a structure, then performs conditional copying of a byte value into a memory block based on computed differences between 16-bit values.
- `copy_and_update_counter_2`: Copies a byte from a pointer in a0 to memory and increments the pointer, then updates a 16-bit value at offset -14 of a structure in a5 using a value from offset -20 of the same structure, and finally sets a byte at offset -5 of the structure to 32 if a specific bit in d7 is set.
- `frag_tst_beq_movea_addq_move_bra_3`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at a2 to a0, then adds 4 to the memory at a2, then copies the memory at a0 to d0, then jumps to another point in the code.
- `increment_counter_and_store_word_2`: Increments a longword value in a structure pointed to by a2 by two, then reads a word from the address in a0 and stores it into a global variable at an offset of negative sixteen from a5 before branching to another routine.
- `frag_tst_beq_movea_addq_move_bra_4`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at a2 to a0, then adds 4 to the memory at a2, then copies the memory at a0 to d0, then jumps to another point in the code.
- `update_and_copy_counter_2`: Updates a 32-bit counter and a 16-bit value in a structure, then performs conditional copying of a byte value into a memory block based on condition.
- `update_display_buffer`: Stores a byte value at an offset from a5, loads a word from a specific location, tests if the loaded value is positive and branches if not, updates a word value at another offset, tests a byte value and branches if zero, loads a long value from memory, and then branches to another location.
- `process_display_buffer`: Reads a word value from a buffer, stores it in a global structure, and conditionally updates display-related counters and pointers.
- `process_string_length`: Calculates the length of a null-terminated string by advancing a pointer until it finds a zero byte, then stores the difference as a 16-bit value in a global variable.
- `reset_counter_and_copy_byte_2`: Sets a 16-bit value at offset -14 of a structure in a5 to 1, increments a pointer in a2, copies a byte from the new pointer to offset -28 of the structure, clears offset -27 of the structure, and updates the value at offset -8 of the structure by either setting it to zero or subtracting 1.
- `frag_moveq_bra_34`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `compare_and_reset_counter_2`: Compares two 16-bit values at offsets of negative 20 and negative 8 from a pointer in a5, and if the first is not greater than or equal to the second, sets the value at offset negative 8 to zero.
- `execute_callback_loop_2`: Decrements a 16-bit value at an offset of -8 from a pointer in a5 and, if the value remains positive, calls a function pointer from a table while updating the pointer and a counter.
- `frag_subq_blt_moveq_move_movea_jsr_addq`: Short code fragment that subtracts 1 from the memory at an offset from a5, then branches if the comparison was less, then loads 0 into d0, then copies the memory at an offset from a5 to d0, then copies d0 to the stack, then copies the memory at an offset from a5 to a0, then calls the routine whose address is in a0, then adds 2 to a7, then jumps to another point in the code.
- `decrement_and_call`: Decrements a counter at a specific offset in the stack frame and if the result is not negative, retrieves a byte from memory, pushes it onto the stack, and calls a function pointer.
- `loop_and_call_with_byte_2`: Decrements a value on the stack, and if not below zero, copies a byte from a pointer at offset 44 of a5 to d0, pushes it to the stack, updates the pointer at offset 44 of a5, calls a routine from a pointer at offset 16 of a5, and loops.
- `function_epilogue_23`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `parse_format_string_2`: Iterates through a string looking for percent signs to identify format specifiers and calls a routine to process them.
- `call_with_pointer_2`: Pushes a pointer and a value onto the stack, calls another routine, and then either stores a value from d0 into a memory location or updates a pointer in a2 and branches.
- `call_routine_with_byte_2`: Copies a byte from d7 to d0, pushes d0 to the stack, calls a routine at the address in a3, increments the stack pointer by 2, and loops.
- `function_epilogue_24`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `process_resource_entries`: Iterates through a table of resource entries, calls a destructor for valid entries that do not have a specific flag set, and then closes a resource.
- `insert_into_memory_map_2`: Manages a list of memory blocks, inserting a new block or merging it with existing adjacent blocks to maintain a sorted map based on address comparisons.
- `copy_memory_block_with_offset_2`: Copies a block of memory from a source pointer to a destination pointer by moving bytes backward using a word-sized counter, while applying a word-sized offset to both pointers if the destination address is lower than the source address.
- `copy_memory_block_3`: Copies a block of memory from the address in a0 to the address in a1 one byte at a time, using a dbra loop counter, and returns.
- `manage_memory_allocation`: Checks if a global variable is non-zero and branches if not, retrieves a word value from memory, calls another routine with the retrieved value and a pointer, stores zero to a global variable, performs conditional operations based on input, allocates memory using another routine, and returns a calculated address.
- `cleanup_resource_list`: Calls a function via a pointer in a structure and clears that pointer if the function returns a non-zero value.
- `frag_bsr_rts_5`: Short code fragment that calls another routine, then returns.
- `call_dos_function_2`: The routine calls a function pointer located at an offset within a structure pointed to by a global variable.
- `open_dos_file_2`: The routine calls a function pointer at a specific offset within a structure to perform a file opening operation.
- `copy_null_terminated_string_4_2`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
