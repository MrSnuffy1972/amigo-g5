# Game 1003 (ultimate quiz, the (1993)(agas)[hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01003). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904784 top-level bytes in 45 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 22 of 22 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 613 of 613 named (0 author symbols), 613 of 613 described (106 by rule, 90 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1064 of 1064 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10730 of 10730 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 488031 |
| code | 32604 |
| data | 327212 |
| padding | 12905 |
| unused | 44032 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ultimate Quiz, The (1993)(AGAS)[HD].adf:ami1demo.dat | 2 | 206 | 7 |
| Ask | 2 | 183 | 5 |
| type | 2 | 363 | 12 |
| endcli | 2 | 117 | 2 |
| echo | 2 | 241 | 10 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 413 | 15 |
| LoadWB | 12 | 535 | 30 |
| Ultimate Quiz, The (1993)(AGAS)[HD].adf:Quiz.exe | 21 | 182 | 7 |
| Disk-Validator | 1 | 386 | 1 |
| d | 1 | 3 | 1 |
| mathtrans.library | 17 | 848 | 106 |
| icon.library | 10 | 1421 | 110 |
| info.library | 25 | 3000 | 181 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 15 | 1040 | 76 |
| Ultimate Quiz, The (1993)(AGAS)[HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 521 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 340 | 8 |
| Ultimate Quiz, The (1993)(AGAS)[HD].adf:ami1demo.dat#unpacked | 2 | 864 | 19 |

## Cases handled

- ultimate quiz, the (1993)(agas)[hd]/Ultimate Quiz, The (1993)(AGAS)[HD].adf:Quiz0.Data: (Pac.Pic.) Data Cruncher (xfd)-packed, unpacked with ancient (21812 -> 51364 bytes)
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 471 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 245 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ultimate quiz, the (1993)(agas)[hd]/Ultimate Quiz, The (1993)(AGAS)[HD].adf:ami1demo.dat: crunched executable decrunched by XFDMaster (PowerPacker 2.3 (xfd), 28444 -> 54068 bytes), 23 of 23 windows confirmed in reference RAM: recovered as a hunk executable
- ultimate quiz, the (1993)(agas)[hd]/Ultimate Quiz, The (1993)(AGAS)[HD].adf:Quiz.exe: CrunchMania Normal (xfd) stream found by ancient scan (166140 -> 283712 bytes) but only 30 of 124 sampled windows are in reference RAM: not used
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (37 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target, 1 immediate+address
- ultimate quiz, the (1993)(agas)[hd]/Ultimate Quiz, The (1993)(AGAS)[HD].adf:ami1demo.dat: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ultimate quiz, the (1993)(agas)[hd]/Ultimate Quiz, The (1993)(AGAS)[HD].adf:Quiz.exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ultimate Quiz, The (1993)(AGAS)[HD].adf:ami1demo.dat

- `hunk_relocator_and_loader`: Walks the AmigaDOS hunk table, allocates memory with exec.AllocMem, copies hunk data, applies relocations for hunk types 1001-1004, frees the task with exec.FreeMem and jumps to the loaded code.
- `count_first_relocation_entry`: Tests d5; if zero, increments d6 and sets d5 to one, otherwise returns unchanged, tracking the first relocation entry during hunk loading.
- `resolve_hunk_relocation_chain`: Follows the relocation chain from the pointer at a5, walking linked entries in a1 and decrementing d1 until the chain ends, then returns the resolved address.
- `decompress_rle_bitmap_to_a1`: Reads packed control words from memory at a0, expands run-length and literal data backwards into the buffer ending at a1, and writes the resulting pointer to the COLOR00 hardware register.
- `load_one_into_d0`: Loads the immediate value 1 into data register d0 and falls through into the following routine.
- `shift_bits_into_d1_with_space_fill`: Clears d1, then for each of d0 bits shifts d5 left through the extend bit into d1, reloading d5 from -(a0) and resetting d7 to 32 whenever d7 counts down to zero.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Ask

- `ask_allocate_and_install_question_buffer`: Calls library vectors through a5 to find the largest node value, allocate a buffer, fill it with incrementing words, copy 16 longs into a2 offset 536, set a2 offset 8 to a local routine, and return zero or minus one.
- `copy_d1_to_d0_and_branch_back`: Moves d1 into d0 as the return value and branches back into the preceding routine's code rather than returning directly.
- `initialize_and_store_pointers`: Calculates memory addresses using shift and add operations, stores calculated values at specific offsets in a structure, and performs conditional jumps based on comparison results.
- `process_and_call_routines`: Loads a longword from a1 into d1, shifts it left by two bits, tests a longword at an offset in a0 plus the shifted value, and branches if zero.
- `check_and_set_flag`: Tests a longword pointer and sets a global variable to five if the test passes, then calls a library routine twice with specific parameters.

### type

- `type_command_allocate_and_run`: Looks up the type command in the resident list, allocates a buffer, fills it with command words, copies sixteen longs from the argument block, installs a return address and calls the command dispatcher, returning zero or minus one.
- `type_command_return_d1_in_d0`: Moves d1 into d0 as the return value and branches to the shared exit path that restores the stack and returns to the caller.
- `initialize_quiz_structure`: Initializes quiz-related data in a structure starting at address in a1, clears several fields, sets up counters and status flags, and prepares for quiz question processing.
- `update_counters_loop`: Contains a loop that reads a byte from memory, compares it with fixed values, and updates counters in a table.
- `set_global_variable_to_negative_one`: Sets d1 to a fixed address, stores that value at offset 612 of a structure in a2, and then branches to offset 552.
- `set_another_global_variable_to_negative`: Sets d1 to a fixed address, stores that value at offset 212 of a structure in a1, and then branches to offset 552.
- `update_score`: Updates a score counter in a table based on byte comparisons and sets flags in global variables.
- `compare_and_update_counters`: Reads a byte from memory, compares it with fixed values, updates counters in a table, and sets flags in global variables based on the comparison outcomes.
- `update_counters_loop_2`: Increments a counter at offset 560 of a structure, reads a byte from memory using an index, compares it with the counter value, and branches if the byte is greater than or equal to the counter.
- `set_global_variables_and_branch`: Sets specific global variables to predefined values, makes multiple library calls with fixed parameters, and branches to different offsets based on test conditions.
- `branch_to_offset`: Branches unconditionally to offset 696.
- `unknown_function_caller`: Loads a value from memory location 336(a2) into a4, makes a function call through a5, then jumps to an address in a6.

### endcli

- `endcli_command_allocate_and_run`: Looks up the endcli command in the resident list, allocates a buffer, fills it with command words, copies sixteen longs from the argument block, installs a return address and calls the command dispatcher, returning zero or minus one.
- `copy_d1_to_d0_and_branch`: Moves the longword in d1 into d0, then branches unconditionally to another routine's code at a fixed offset.

### echo

- `echo_command_allocate_and_run`: Reads the echo command line, finds the largest argument size, allocates a buffer, fills it with argument pointers, copies the argument block, and invokes the command with the parsed arguments.
- `copy_d1_to_d0_and_branch_2`: Moves the longword in d1 into d0 and then branches to another routine, so d0 carries d1 as the value passed onward.
- `initialize_quiz_data_2`: Reads values from a table using an index from a structure, performs arithmetic operations on them, and stores results back to the same structure.
- `initialize_quiz_data`: Reads values from a table using an index from a structure, performs arithmetic operations on them, stores results to the same structure, and updates global variables based on comparisons.
- `copy_global_value`: Reads a long integer from a global variable at offset 150 of the structure in a1, then stores it to another global variable at offset 148 of the same structure.
- `calculate_and_compare`: Sets a global variable to one, reads values from structure offsets, performs arithmetic operations on them, compares the result with another value, and updates the global variable based on the comparison outcome.
- `copy_and_store`: Copies a longword value from a memory location at offset 336 of the structure in a1 to another location at offset 332 of the same structure, then branches to an address.
- `copy_and_compare`: Copies longword values between structure offsets, compares a value with another longword at an offset, and branches if the first is greater.
- `load_and_test_value`: Loads a longword from a memory location, shifts it left by two bits, tests a longword at an offset plus the shifted value, and branches if not zero.
- `initialize_and_call`: Tests a longword value at offset 584 of a structure in a2 and branches to label if zero.

### EndIf

- `return_zero_for_end_if`: Clears d0 to zero and returns immediately, giving the EndIf script command a false result.

### If

- `evaluate_if_condition_and_branch`: Looks up the If command's condition, allocates and fills a jump table, runs the test through library calls, and returns zero or minus one.
- `copy_d1_to_d0_and_jump`: Moves d1 into d0 as the result and branches forward into the If routine's continuation code.
- `initialize_quiz_state`: Initializes multiple global variables related to quiz state and sets up pointers based on input parameters.
- `check_input_flag`: Loads a value from memory at an offset, compares it with another fixed constant, and branches to an error handler if the comparison fails or if the loaded value is zero.
- `validate_input_entry`: Reads a value from a table using an index, performs a comparison with a fixed constant, and branches to an error handler if the result is not zero or if another condition fails.
- `process_input_sequence`: Reads input values from a structure, validates them against table entries, updates global state based on comparisons, and performs conditional operations including string checks and function calls.
- `validate_input_sequence`: Checks input values against table entries and updates global state based on comparisons.
- `jump_to_unknown_routine`: Performs an unconditional branch to another routine referenced through a jump table, without modifying any data or registers.
- `check_table_entry`: Reads a value from a table using an index from a global structure, updates another global variable with that value, and based on the value either branches or continues execution.
- `jump_to_table_entry`: Reads a value from a table using an index from a global structure, updates another global variable with that value, and based on the value executes further operations.
- `check_global_flag`: Loads an immediate value into a register, compares it with a global variable, and branches based on the comparison result.
- `process_input_flags_2_2`: Reads a value from a table using an index from a global structure, updates another global variable with that value, and based on the value either sets or clears bits in a third global variable.
- `process_input_flags_2`: Initializes a global variable to zero, reads a value from memory using an index, and based on that value either sets or clears bits in the global variable, then performs several library calls with fixed parameters.
- `process_input_and_update_flags`: Processes input values through multiple library calls, updates flags in global variables based on comparisons, and handles conditional branching based on test results.
- `process_input_flags`: Processes input values through library calls, updates a global variable with the result, compares it with fixed constants, and handles conditional branches based on those comparisons.

### LoadWB

- `loadwb_open_and_parse_startup_args`: Opens the Workbench startup message port, allocates a message, reads the startup argument string, splits it into a token pointer array with quote and escape handling, and returns the token count or an error code.
- `loadwb_cleanup_and_reply_startup_msg`: Finds the Workbench task, locates the startup message, closes its file and library, frees its memory, replies the message and returns the original argument value.
- `loadwb_run_startup_tool_and_icon`: Decodes the startup command, opens the tool library, loads and runs the startup program, then opens the Workbench screen and processes the startup icon before returning.
- `compare_strings_return_order`: Compares two null-terminated strings passed on the stack, returning zero if equal, minus one if the first is lower, or one if higher.
- `build_linked_node_list`: Walks a linked list of offsets, allocating eight-byte nodes and copying each node's second field via a called routine, linking nodes and returning the first.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_three_arg_function_pointer`: Saves d2-d3, loads two arguments and a pointer to a third from the stack, pushes them and calls the function pointer, then cleans the stack and restores d2-d3.
- `format_string_via_rawdofmt_then_dispatch`: Calls exec RawDoFmt into a 140-byte stack buffer, measures the resulting string length, and if non-empty passes the buffer and length to another routine.
- `store_d0_byte_to_a3_postincrement_2`: Writes the low byte of d0 to the memory pointed to by a3, then advances a3 by one byte and returns to the caller.
- `compare_string_with_table`: Compares the value in a2 with a fixed address and branches if equal.
- `write_dot_and_spaces_to_a2_offsets`: Writes a period byte then four space bytes into the five bytes at offsets 17 through 21 from a2, then branches forward to another routine.
- `compare_a2_string_with_pc_table`: Copies a2 into a0, points a1 at a nearby byte table, and compares bytes until a zero terminator or mismatch, branching to different targets on match, mismatch or empty table.
- `restore_a0_a1_and_jump_via_pc_vector`: Loads a2 with a nearby address, pops a0 and a1 from the stack, pushes the longword at the program counter and returns through it.

### Ultimate Quiz, The (1993)(AGAS)[HD].adf:Quiz.exe

- `startup_free_mem_and_chain_loader`: Saves all registers, sets d6 to 19, calls a helper repeatedly to walk a resource chain, frees a memory block via exec FreeMem using a6, clears a pointer and returns.
- `compute_chain_pointer_from_d6`: Copies d6 into d0, loads a pointer from just before the program counter, doubles it twice, loops dbra over d0, then advances a1 by four and returns.
- `decompress_bitstream_to_buffer`: Reads two lengths from a table, clears a buffer, calls helper routines to copy and expand bit-packed data into memory, then copies decoded bytes into a destination buffer.
- `decode_huffman_symbol_from_bitstream`: Reads bits from d6 and a2, walks the code-length table at a4 comparing cumulative counts, then returns the symbol word from the table indexed by a0 and the accumulated code.
- `extract_masked_bitfield_value`: Shifts d6 right by d1, adjusts d7, rotates in a word from a2, doubles d1 and returns d0 masked with the word at the pc-relative table entry.
- `build_huffman_length_and_symbol_tables`: Calls the bitfield extractor to get a count, then fills a0 with symbol lengths and a4 with symbols, accumulating a3, using d5 and d4 as loop counters before restoring registers.
- `build_quadratic_sequence_table`: Pushes d0-d7, then fills 15 words at a4 from successive a3 inputs, accumulating a running sum and doubling it, and restores registers before returning.

### Disk-Validator

- `disk_validator_main_entry`: Program entry that calls AmigaOS library vectors through a5 to read and validate disk blocks, updating counters and buffers in the a2 structure.

### d

- `keymap_data_table`: The bytes are keymap data, not executable code; decoded as instructions they produce trap and breakpoint opcodes with no real routine.

### mathtrans.library

- `illegal_instruction_trap_word`: Executes the single illegal instruction word, which triggers the 68000 illegal instruction exception and does not return normally.
- `store_a0_to_global_slot`: Copies the longword in a0 into the absolute memory location at offset 408 and returns to the caller.
- `close_library_free_mem_and_return`: If the library open count is zero, unlinks the library node from its list, calls exec FreeMem on the base memory, then returns the stored global longword in d0; otherwise sets the flags byte bit and returns zero.
- `set_library_open_flag_bit3`: Sets bit 3 of the byte at offset 14 from a6, then returns zero in d0 to the caller.
- `open_mathffp_and_track_open_count`: If bit 3 of the byte at offset 14 from a6 is clear, increments the word at offset 32 from a6, and on the first open stores a global and calls a routine with two arguments, saving the result; returns a6, or minus one if the flag is set.
- `return_minus_one_error`: Loads minus one into d0 and returns immediately, signalling failure to the caller.
- `mathtrans_decrement_and_dispatch`: Library code that decrements the word at offset 32 of a6, and when it reaches zero with bit 3 of offset 14 set, calls the cleanup routine and dispatches through a stored pointer.
- `mathtrans_negative_input_abs_wrapper`: Masks the low seven bits of d0, calls the conversion routine at the next address, sets the carry flag in the condition codes and returns.
- `mathtrans_positive_float_to_fixed_convert`: Takes a positive value in d0, saves d4-d6, normalizes the mantissa and exponent, then iterates shifting and subtracting to build a fixed-point result in d0.
- `ieee_extended_divide_loop`: Iteratively shifts d5 and d6, subtracts d6 from d0, and adjusts d0 and d1 to compute a quotient bit by bit, then packs the result into d0 and returns after restoring d4-d6.
- `call_library_vector_and_set_flags`: Copies d0 to d7, tests the low byte of d7, calls a library vector through absolute address 20, then sets bit 1 and clears bit 3 of d0 before branching away.
- `mathtrans_float_operation_sequence`: Saves d6, then repeatedly loads d3 or d5 and immediate constants into d0/d1 and calls library vectors at offsets -66, -78 and -84 from a5, restoring d6 before returning.
- `clear_d1_and_branch_to_shared_tail`: Clears the longword in d1 to zero, then branches unconditionally into the shared tail of the neighbouring library routine.
- `mathtrans_compare_and_dispatch_vectors`: Saves d2-d6 and a5, loads the library base into a5, calls several library vector offsets to compare and convert values in d0 and d1, then branches to the shared exit.
- `mathtrans_compare_zero_and_convert`: Copies d6 into d4, compares d5 against zero via a library vector, and if positive converts d5 through another vector before branching to the shared exit.
- `mathtrans_conditional_angle_transform`: Copies d5 to d2, then depending on d6 and d4 calls library math routines via a5 offsets, using constant angle value and d7 sign to transform d2 and return it in d0.
- `mathtrans_angle_scale_by_d7`: Moves d7 to d1 and calls the a5 library routine, then if d4 is nonzero multiplies d2 by a constant angle and calls further a5 routines, returning result in d2.
- `mathtrans_convert_d2_via_library`: Moves d2 into d0, calls the library routine at a5 offset minus sixty, stores the returned d0 back into d2 and branches to the common exit.
- `load_negated_pi_constant_into_d2`: Copies d2 into d0, tests d4 and branches away if it is nonzero, otherwise loads the negated pi constant into d2 and branches onward.
- `compute_negated_pi_via_library_calls`: Loads the negated pi constant into d2, calls the library offset twice with d2 in d1, moves the result to d2 and d7, calls another routine, masks d0 with the condition codes, restores registers and returns.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `decode_float_exponent_and_mantissa`: Saves registers, strips the sign bit from d7, compares it against a packed constant, optionally negates and swaps values, calls a routine, then derives an exponent and mantissa for further processing.
- `init_math_trans_loop_state`: Shifts d7 right by one, clears d6, loads d5 with a large constant, sets a0 to absolute address four, and sets d1 and d2 to 24 and 1 before branching away.
- `iterative_math_trans_core_loop`: Repeatedly shifts and subtracts d4/d5/d7 using d2 as shift count, accumulates memory at a0 into d6, advances a0 and d2, loops via dbra d1, then calls two library routines and restores registers.
- `push_negative_two_and_branch`: Pushes the word value minus two onto the stack and branches forward into another routine, serving as a small library entry stub.
- `push_minus_one_word_and_branch`: Pushes the word value minus one onto the stack and branches forward into shared library code, passing that value as an argument.
- `push_one_word_and_branch`: Pushes the word value one onto the stack and branches forward into shared library code, passing that value as an argument.
- `compare_d7_byte_and_cleanup_stack`: Compares the low byte of d7 with the decimal value 184, branches if higher, otherwise adds two to a7, tests the byte and returns.
- `dispatch_angle_by_d7_range`: Pushes a zero word, tests the byte in d7 and returns early for values below 57 or above 184, otherwise saves d1-d6/a0 and branches to the sine or cosine path based on doubled d7.
- `compute_sine_cosine_series`: Uses d2 as an angle, calls the range-reduction and polynomial helpers, then runs a 24-term series over a table to produce sine and cosine values in d7 and d6.
- `reduce_negative_angle_series`: Adjusts d2 by subtracting 198 and negating, shifts d7 by the reduced count, compares against d4 and runs the 24-term table series to finish the negative-angle cosine path.
- `cordic_atan_rotation_loop`: Initialises sine and cosine accumulators and an arctangent table pointer, then iterates 24 times shifting and adding or subtracting table values into d5, d6 and d7 based on the sign of d7.
- `cordic_second_half_and_dispatch`: Continues the CORDIC rotation by adding d4 to d5, subtracting d3 from d6 and adding the next table word to d7, then reads the angle argument from the stack and branches to the result or error paths.
- `load_positive_infinity_into_d7`: Loads the immediate value representing positive infinity into d7 and branches back into the shared library code, used as a result path for the math routine.
- `finish_math_transform_and_return`: Branches if zero to the shared exit, copies d5 to d6, calls the conversion routine, copies d6 to d7, tests the low byte of d7, restores d1-d6 and a0 from the stack, skips two bytes and returns.
- `save_d5_call_convert_store_d6`: Pushes d5 onto the stack, calls the conversion routine, stores d6 into the caller's stack slot, pops d6 back and branches to the shared exit.
- `clamp_d6_to_byte_range`: Copies d6 to d4, branches if negative, compares d6 with 255 and branches if above, otherwise sets d6 to zero and returns.
- `shift_d4_right_8_add_1_then_jump`: Arithmetically shifts d4 right by 8 bits, adds 1 to d4, and if the result is zero branches to a nearby address, otherwise jumps to address zero.
- `convert_angle_with_overflow_fallback`: Saves d6, masks d7 to 7 bits, calls a conversion routine, and on overflow or zero result returns zero in d7 after restoring d6 from the stack.
- `convert_angle_restore_d7_and_d6`: Saves d6 and d7, calls conversion routines on d7, and on success restores d7 and d6 from the stack before returning to the caller.
- `mathtrans_convert_d7_via_library_calls`: Saves d6, tests and increments the byte in d7, then calls several library routines with d7 and a constant in d6, restoring d6 and returning.
- `load_negative_flag_into_d7`: Loads the constant value into d7, rotates its low byte right through extend, then branches into the shared continuation code.
- `load_positive_flag_into_d7`: Loads the constant value into d7 and branches into the shared continuation code without any rotation.
- `pop_d6_test_sign_branch`: Pops a word from the stack into d6, tests its low byte, and branches based on the sign; if negative it sets d7 to zero and branches onward.
- `set_d7_ff_shift_and_restore_regs`: Loads d7 with 255, shifts its low byte right by one, sets the carry flag via ccr, restores d1-d6 and a0 from the stack, then returns.
- `load_d7_advance_stack_test_byte`: Loads d7 with a long immediate, moves the stack pointer forward by thirty bytes, tests the low byte of d7, and returns to the caller.
- `mathtrans_apply_exponent_transform`: Saves registers, tests d7 exponent, masks sign, calls library math helpers to scale and combine d7 and d2, returning the transformed value in d2.
- `mathtrans_compute_polynomial_series`: Builds a fixed-point value in d6 from d2 using three table-driven multiply-accumulate passes over a0, applies sign from the stack byte, calls a helper and returns the result in d7.
- `mathtrans_shift_accumulate_loop`: Loops d1 times, shifting d5 and d6 right by d2 bits, adding them into d5 and d6 and subtracting successive longwords from d7 via a0.
- `subtract_d4_d5_subtract_d3_d6_add_loop`: Subtracts d4 from d5 and d3 from d6, then repeatedly adds the longword at (a0)+ to d7 while decrementing d1, finishing with rts.
- `test_d7_branch_and_clear_sign_flag`: Tests d7, branches on zero or positive, clears bit 7 of d7 and calls a nearby routine, then sets the carry flag and returns.
- `set_d7_minus_one_and_branch_back`: Loads d7 with minus one and branches backward into the preceding routine, acting as an entry stub for that code.
- `mathtrans_convert_angle_to_sincos`: Saves registers, calls three library helpers to convert the angle in d7 into a scaled sine/cosine pair, then sets up a0, d1 and d2 for a table lookup and branches onward.
- `mathtrans_divide_and_scale_result`: Loops subtracting shifted divisors from d5 and d7, calls a helper, then adjusts the quotient in d6 and d7 and calls two more library routines before restoring registers and returning.
- `mathtrans_jump_table_stub`: Calls three library routines through pc-relative and absolute jumps, loads a constant into d6, moves d0 into the condition codes and returns.
- `test_d7_sign_and_clear_bit7`: Tests the sign of byte d7; if negative, clears bit 7 of d7, calls the following routine, then sets the carry flag in the condition code register before returning.
- `save_regs_call_two_library_entries`: Calls a library entry, saves d3 through d5 on the stack, calls another library entry, restores d3 through d5, then jumps to a third library entry.
- `convert_d7_byte_to_float_bits`: Doubles d7, and if nonzero flips bit 7, shifts the byte right arithmetically, subtracts 130, swaps halves and rotates the long word left by 7 bits before returning.
- `normalize_extended_byte_in_d7`: Rotates d7 through swap and ror, flips bit 7 with eori, doubles the low byte, adds 5 on overflow, flips bit 7 again and rotates right once before returning.
- `classify_d7_byte_and_branch`: Tests the low byte of d7 against the values 124 and 126, adjusts it by adding 133 when equal to 126, rotates d7 right and branches based on the resulting byte.
- `clear_d7_bit8_and_test`: Clears bit 8 of d7 with andi, tests the full longword and low byte, sets d7 to zero with moveq and branches onward.
- `check_d7_byte_and_shift_loop`: Compares the low byte of d7 with 254, and if not equal shifts d7 right by 8 then by 1, branching onward when the result is nonzero or falling through otherwise.
- `build_d7_byte_with_roxr`: Shifts d7 left by 8, loads 255 into d7, rotates the low byte right through extend, sets the carry flag, then branches to another library routine.
- `zero_d7_and_branch`: Sets d7 to zero with a quick move and branches to another routine in the library, returning no other value.
- `call_a0_preserve_d3_d7_return_d0`: Saves d3 through d7, moves d0 into d7, clears d1, calls the routine pointed to by a0, then restores d0 and d3 through d7 from the stack and returns.
- `call_a0_with_d1_in_d6_return_d0`: Saves d3 through d7, copies d1 into d6 and d0 into d7, clears d1, calls the routine at a0, then restores d0 and d3 through d7 and returns.
- `load_vector_table_pointer_and_branch`: Loads the address of a vector table into a0 and branches to the start of the library code, passing control onward without returning here.
- `load_offset_address`: Loads a memory address with a fixed offset into register a0 and branches to another routine.
- `load_negative_offset`: Loads a memory address with another fixed negative offset into register a0 and branches to another routine.
- `load_math_offset_48`: Loads a memory address with a specific offset of minus 48 into register a0 and branches to another routine.
- `load_math_offset_54`: Loads a memory address with a specific offset of minus 54 into register a0 and branches to another routine.
- `load_math_offset_60`: Loads a memory address with a specific offset of minus 60 into register a0 and branches to another routine.
- `load_math_offset_66`: Loads a memory address with a specific offset of minus 66 into register a0 and branches to another routine.
- `load_math_offset_72`: Loads a memory address with a specific offset of minus 72 into register a0 and branches to another routine.
- `load_negative_offset_16`: Loads a memory address with a fixed negative offset into register a0 and branches to another routine.
- `call_library_function`: Loads a memory address with a specific offset into register a0 and calls a library function using d6 and d7 as arguments.
- `load_math_trans_base_648`: Loads the address 648 into a0 and branches to the shared library dispatcher, passing that pointer as an argument.
- `load_math_trans_base_652`: Loads the address 652 into a0 and branches to the shared library dispatcher, passing that pointer as an argument.
- `load_math_trans_base_zero`: Loads the address zero into a0 and branches to the shared library dispatcher, passing that pointer as an argument.
- `load_offset_thirty_into_a0_and_branch`: Loads the immediate value 30 into address register a0 and branches unconditionally to another library routine, which continues execution.
- `load_offset_twelve_into_a0_and_branch`: Loads the immediate value 12 into address register a0 and branches unconditionally to another library routine, which continues execution.
- `load_offset_six_into_a0_and_branch`: Loads the immediate value 6 into address register a0 and branches unconditionally to another library routine, which continues execution.
- `mathtrans_save_d7_call_and_store`: Loads an absolute address into a0, saves d3-d7, keeps d0 in d7, pushes d1, calls the routine at a0 with d1 zeroed, then stores d7 through the popped d1 pointer and returns d6 in d0.
- `mathtrans_jump_table_entry_34`: Loads the constant offset 52 into a0 and branches to the shared library dispatch code, acting as a jump-table entry in mathtrans.library.
- `mathtrans_jump_table_entry_zero`: Loads zero into a0 and branches to the shared library dispatch code, acting as the first jump-table entry in mathtrans.library.
- `load_offset_110_into_a0_and_branch`: Loads the immediate value 110 into address register a0 and branches to a shared library entry point, part of mathtrans.library.
- `load_offset_38_into_a0_and_branch`: Loads the immediate value 38 into address register a0 and branches to a shared library entry point, part of mathtrans.library.
- `load_zero_into_a0_and_branch`: Loads zero into address register a0 and branches to a shared library entry point, part of mathtrans.library.
- `load_math_trans_base_and_branch`: Loads the address of the math translation table into a0 and branches to the shared library entry point.
- `load_zero_base_and_branch`: Loads a zero base address into a0 and branches to the shared library entry point.
- `jump_to_absolute_address_in_a0`: Loads the absolute address zero into a0 and branches to the address zero, transferring control away from this library stub.
- `load_offset_20_into_a0_and_branch`: Loads the immediate value 20 into address register a0 and branches to another library routine, passing a0 as an argument.
- `load_negative_30_into_a0_and_branch`: Loads the immediate value minus 30 into address register a0 and branches to a shared library routine for further processing.
- `load_negative_36_into_a0_and_branch`: Loads the immediate value minus 36 into address register a0 and branches to a shared library routine for further processing.
- `load_math_offset_minus42_into_a0`: Loads the address forty-two bytes below the current program counter into a0, then branches forward to shared library code at a fixed offset.
- `load_math_offset_minus48_into_a0`: Loads the address forty-eight bytes below the current program counter into a0, then branches forward to shared library code at a fixed offset.
- `load_math_offset_minus54_into_a0`: Loads the address fifty-four bytes below the current program counter into a0, then branches forward to shared library code at a fixed offset.
- `load_math_offset_minus_60_and_branch`: Loads the address sixty bytes before the math library base into a0 and branches forward to shared library code.
- `load_math_offset_minus_66_and_branch`: Loads the address sixty-six bytes before the math library base into a0 and branches forward to shared library code.
- `load_math_offset_minus_72_and_branch`: Loads the address seventy-two bytes before the math library base into a0 and branches forward to shared library code.
- `load_negative_offset_into_a0`: Loads the address formed by sign-extending the immediate value minus seventy-eight into address register a0, then branches to another routine.
- `call_library_offset_with_d6_d7_args`: Saves d0, d1 and a0, adds the library base from memory to a0, moves d6 to d1 and d7 to d0, calls the computed address, then restores the saved registers and returns.
- `call_library_offset_with_d7_arg`: Saves d0 and a0, adds the library base from memory to a0, moves d7 to d0, calls the computed address, stores the result in d7, restores registers and returns.
- `normalize_ieee_extended_exponent`: Takes a floating value in d6, makes it positive, normalizes it by shifting and doubling while adjusting the exponent byte in d4, and returns the exponent in d6.

### icon.library

- `library_open_stub_return_minus_one`: Returns minus one in d0 as an error result and returns immediately, acting as a stub library open entry.
- `increment_counter_clear_bit3_return_a6`: Adds one to the word at offset 32 of the library base in a6, clears bit 3 of the byte at offset 14, then returns the base in d0.
- `decrement_open_count_and_close_if_zero`: Sets d0 to zero, decrements the word at offset 32 of the library base in a6, and if it reaches zero with bit 3 of byte at offset 14 set, calls the close routine at offset 256.
- `close_library_and_unlink_from_list`: If the word at offset 32 of a6 is zero, unlinks the library node from its list, computes a negative offset from the base, and calls the library close vector at offset -210 before returning the saved value.
- `set_close_flag_and_return_zero`: Sets bit 3 of the byte at offset 14 of the library base in a6, marking it for closing, then returns zero in d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_and_store_base`: Saves a2, stores a0 and a6 into the new structure at offsets 42 and 34, opens dos.library via exec, stores the base at offset 38, and alerts on failure.
- `copy_icon_fields_to_globals`: Copies the two longwords at offsets 34 and 38 of the a2 structure into the global slots at 402 and 406, returns a2 in d0 and pops a2.
- `save_a0_a6_call_vector`: Pushes a0 and a6 onto the stack, calls the routine whose address is stored at memory location zero, then discards both saved registers and returns.
- `save_a0_a1_a6_call_vector`: Pushes a0, a1 and a6 onto the stack, calls the routine whose address is stored at location a8, then removes the three saved registers and returns.
- `icon_lib_save_a0_a2_call_helper`: Pushes registers a0 through a2 and a6 onto the stack, calls a library helper routine, then discards the saved registers and returns.
- `icon_lib_save_a0_a1_call_helper`: Pushes registers a0, a1 and a6 onto the stack, calls a library helper routine, then removes the saved registers and returns.
- `icon_lib_save_a0_call_helper`: Pushes register a0 and a6 onto the stack, calls a library helper routine, then adds eight to the stack pointer and returns.
- `icon_lib_push_a0_a6_call_helper`: Pushes a0 then a6 onto the stack, calls the library helper at offset 1682, discards both saved registers and returns.
- `icon_lib_push_a6_call_helper`: Pushes a6 onto the stack, calls the library helper at offset 1704, discards the saved register and returns to the caller.
- `icon_lib_push_a0_a2_a6_call_helper`: Saves a0 through a2 and a6 on the stack, calls the library helper at offset 1820, then releases the saved registers and returns.
- `icon_lib_call_768_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 768, then discards both saved registers and returns.
- `icon_lib_call_7fc_with_a0_a6`: Pushes a0 and a6 onto the stack, calls the library routine at offset 7fc, then discards both saved registers and returns.
- `icon_lib_call_818_with_a0_a1`: Saves a0 and a1 on the stack, calls the library routine at offset 818, then removes the two saved registers and returns.
- `icon_library_call_864_with_saved_regs`: Saves a0 and a1 on the stack, calls the library routine at offset 864, then discards the saved registers and returns to the caller.
- `icon_library_call_8d6_with_saved_regs`: Pushes a0 and a1 onto the stack, invokes the library routine at offset 8d6, cleans the stack by adding eight and returns.
- `icon_library_store_value_at_offset_94`: Saves a6, loads the library base from the stack argument, calls the base's offset minus 294 routine, stores the second argument at offset 148 of the result, restores a6 and returns.
- `allocate_and_init_icon_object`: Calls allocation and initialization helpers with the two arguments, then copies ten longs and several fields into the new icon object and links it to its parent.
- `copy_stack_fields_into_icon_object`: Copies ten longs from the stack frame into the icon object at a2, then stores individual fields and links the object to its parent if present.
- `return_zero_and_restore_d2_d4_a2`: Library routine that sets d0 to zero as its return value, restores d2 through d4 and a2 from the stack frame, releases the a6 frame and returns.
- `copy_icon_header_and_dispatch`: Library routine builds a local header from the icon structure fields, copies eleven longwords from offset 96, then calls another library routine with the two arguments and the header.
- `open_icon_file_and_load_icon`: Library routine formats a path, opens the icon file, reads the icon header and its image data, then loads the associated images before returning a status.
- `icon_open_validate_and_load_arrays`: Calls the open hook, checks the icon structure magic and version, clears two fields, then loads the image, selected and unselected arrays and returns success or failure.
- `icon_load_selected_and_unselected_images`: Loads the selected and unselected image data into the icon structure, then allocates and fills the optional array and mask buffers, returning success or failure.
- `icon_load_image_array_entries`: Loops over the image array count, allocating and filling each entry from the icon structure, then optionally loads the mask and returns success.
- `set_d4_zero_and_branch_to_cleanup`: Sets d4 to zero and branches to the shared exit path at the start of the following routine, leaving other registers untouched.
- `return_d4_restore_regs_and_unwind`: Moves d4 into d0 as the return value, restores d2-d5 and a2-a5 from the a6 frame, releases the frame and returns to the caller.
- `icon_library_open_icon_entry`: Library entry that saves registers, allocates a small buffer, calls library helpers with the icon parameters and returns a status in d0 after restoring the frame.
- `icon_render_with_optional_second_image`: Saves d6, calls library helpers to set up drawing, then draws the icon image at offset 26 of a2 and optionally the image at offset 66, returning d4 status.
- `icon_draw_primary_and_secondary_images`: Calls helpers to draw the images at offsets 22 and 26 of a2, then draws the text at offset 50 and walks the list at offset 54, returning d4 status.
- `icon_advance_list_and_draw_text`: Advances a3 by four bytes and increments the stack counter until a zero longword is found, then calls the draw callback and reloads a3 from offset 54 of a2.
- `icon_add_tooltype_entries`: Walks the null-terminated tool-type pointer list at a3, calling the a4 hook twice per entry and storing the incremented count at -4(a6), then handles the entry at offset 70(a2), calls two library helpers and stores d6 at 26(a2) before returning d4 in d0.
- `icon_push_a2_load_caller_arg`: Pushes a2 onto the stack, loads the caller's argument from 12(a7) into a0, sets a2 to two bytes past that address and branches forward into the following code.
- `icon_free_chain_until_head`: Frees the object in d0 via a library call, then repeatedly frees the object at a2 and advances until a2 equals the head pointer at 8(a2), finally freeing any remaining non-null d0 and popping a2.
- `call_icon_cleanup_with_offset`: Loads the first stack argument into d0 and the second into a0, then pushes a0 plus 140 and d0 and calls a library routine before discarding the arguments.
- `allocate_and_initialize_icon_block`: Saves d2-d3/a2, allocates 65536 bytes of chip memory, initializes fields at offsets 140 and 142, and returns the block pointer or zero on failure.
- `finalize_icon_block_or_return_null`: Pushes a2 and d2 with the block offset, calls a routine, and if it succeeds returns a2 in d0, otherwise leaves zero as the result.
- `icon_lib_return_zero_pop_d2_d3_a2`: Library code that sets d0 to zero as its return value, restores d2, d3 and a2 from the stack, and returns to the caller.
- `icon_lib_decrement_count_and_store_pair`: Library code that decrements the word at a2, calls a helper when it reaches zero, then stores d2 and a stack argument into the table pointed to by a2.
- `icon_lib_store_pair_after_decrement`: Library code that decrements the word at a2, then writes d2 and a stack argument into the table at offset ten of a2, returning one.
- `alloc_icon_header_and_init`: Saves d2-d4/a2, allocates a 65536-byte block with tag 94, builds an icon header at offset 78, initializes it and calls three library helpers, returning the block pointer or zero.
- `branch_to_cleanup_exit`: Branches unconditionally to the shared cleanup exit path, transferring control without touching any registers or memory itself.
- `free_icon_block_and_return_zero`: Pushes tag 94 and d4, calls the free routine, clears d4 and d0 to zero, pops d2-d4/a2 and returns zero to the caller.
- `call_icon_helper_with_offset_length`: Pushes d2, loads two arguments from the stack, adds 78 to the second argument, pushes both and calls a library helper, then restores d2 and returns.
- `allocate_icon_with_size_argument`: Saves d2-d4 and a2-a3, loads an argument pointer and size, calls a library routine with the size, stores the result in d3 and a2 in d4, then branches based on the result.
- `check_icon_data_for_equals_byte`: Saves d3, d2 and a3, calls a library routine, tests the result, adds d3 to a3, compares the next byte with 61 and returns the pointer if equal.
- `pop_next_icon_entry_or_zero`: Loads the longword at a2 into a3 and d0, advances a2 by four, and if that value is zero sets d0 to zero before restoring d2-d4/a2-a3 and returning.
- `call_icon_callback_with_argument`: Saves d2-d5, loads two longword arguments from the stack into d2 and d3, pushes d3, calls the routine at address zero, stores the result in d5, discards the argument and branches onward.
- `allocate_icon_block_and_compute_offset`: Pushes the constant 124 and d2, calls the routine at offset 16, saves the result in d4, discards both stack entries, and if nonzero returns d4 minus d2.
- `check_icon_pair_match`: Calls two library routines to compare d0 with d5 and then test a value; returns 1 in d0 if both checks pass, otherwise falls through to shared exit code.
- `advance_d2_past_d4_or_zero`: Tests d4; if nonzero copies d4 to d2 and increments it by one, then branches to the shared exit path.
- `return_zero_in_d0_restore_regs`: Sets d2 to zero, tests it, then sets d0 to zero, restores d2 through d5 from the stack and returns to the caller.
- `parse_icon_copy_spec_into_icon`: Clears byte 30 of the icon structure, parses the copy specification string via library calls, stores the parsed count and remaining text into the icon, and returns the icon pointer in d0.
- `parse_copy_spec_after_icon_prefix`: Advances a4 past five bytes, calls the parser on the remaining text with length 28, and on success sets d3 to one and advances a4 by three before branching onward.
- `set_d3_zero_and_branch`: Sets d3 to zero and branches forward to continue the icon copy-spec parsing, marking the parse as unsuccessful.
- `parse_decimal_number_from_string`: Accumulates digits from the string at a4 into d3 as a decimal number, calls the digit converter for each character, then formats and stores the result via library calls before returning a2.
- `convert_ascii_digit_to_value`: Loads the argument from the stack into d1, checks it lies between the characters zero and nine, and returns its numeric value in d0 or branches to the failure path.
- `return_minus_one_constant`: Loads minus one into d0 and returns immediately, serving as the failure result for the digit conversion routine.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `icon_allocate_and_attach`: Saves d2-d5, reads three arguments from the stack, allocates a structure via a library call, then attaches it through another call and frees it on failure, returning the handle or zero.
- `icon_create_list_header`: Saves d2/a2, allocates a small structure, stores the value ten in its word field and in the caller's output word, then links the structure into the caller's list and returns success.
- `icon_return_zero_restore_regs`: Sets d0 to zero as the return value, restores d2 and a2 from the stack and returns to the caller.
- `call_vector30_compare_third_arg`: Loads two arguments from the stack, pushes a third, calls the function at absolute address 48, compares the result with the saved third argument and returns it or zero.
- `return_d1_in_d0`: Copies the value in d1 into d0 and returns immediately, serving as a tiny return-value routine.
- `call_vector4c_compare_third_arg`: Loads two arguments from the stack, pushes a third, calls the function at absolute address 76, compares the result with the saved third argument and returns it or zero.
- `return_d1_in_d0_2`: Copies the longword in d1 to d0 as the return value and returns immediately to the caller.
- `unsigned_32bit_divide_d1_by_d2`: Divides the longword in d1 by d2, using word divisions when d2 fits in sixteen bits and a shift-subtract loop otherwise, returning quotient in d0 and remainder in d1.
- `restoring_division_loop_d1_by_d2`: Performs a sixteen-step shift-subtract division of the longword in d1 by d2, leaving the quotient in d1 and remainder in d0, then returns.
- `mul32x32_high_low_unsigned`: Multiplies the 32-bit values in d0 and d1 as unsigned, combining partial 16-bit products, and returns the 32-bit result in d0 while preserving d2.
- `forward_call_to_helper`: Saves d2 on the stack, moves d1 into d2 and d0 into d1, calls the 32-bit divide routine, restores d2 from the stack and returns.
- `call_helper_return_register`: Saves d2 on the stack, moves d1 into d2 and d0 into d1, calls the 32-bit divide routine, copies the value in d1 into d0, restores d2 and returns.
- `negate_result_on_flag`: Calls the 32-bit divide routine, then negates the returned value in d0 when the address register a0 holds a nonzero word.
- `call_and_negate_on_flag`: Calls the 32-bit divide routine, copies the value from d1 into a working register, and negates it when the address register a0 is nonzero before returning that value.
- `format_string_via_rawdofmt`: Saves a2-a4/a6, loads the format string and argument pointer from the stack, calls exec RawDoFmt through a6, then restores registers and returns.
- `store_d0_byte_to_a3_postincrement_3`: Writes the low byte of d0 to the memory pointed to by a3 and advances a3 by one byte, then returns to the caller.
- `strlen_count_until_null`: Reads bytes from the string pointed to by the first stack argument, counting until a null byte or 255 bytes, then returns the complemented count in d0.
- `find_first_matching_byte`: Scans the string at the first stack argument for the byte value in the second argument, returning a pointer to the match in d0 or zero when the null terminator is reached.
- `return_zero_and_branch`: Loads zero into d0 and branches forward into shared library code, providing a null result path for the byte search.
- `copy_string_until_nul`: Loads two pointers from the stack, copies bytes from the source to the destination one at a time until a zero byte is copied, then returns.
- `skip_to_end_of_string`: Loads two pointers from the stack, advances the first pointer past bytes until a zero byte is found, steps back one, then branches into the string copy code.
- `compare_strings_up_to_length`: Loads two pointers and a length from the stack, compares bytes one at a time, returns zero when a zero byte matches, otherwise returns the differing byte comparison result.
- `return_one_or_minus_one_flag`: Loads 1 into d0, then a conditional branch skips the next instruction, otherwise loads -1 into d0, and returns to the caller.
- `scan_string_until_zero_or_count`: Loads a0 and a1 from the stack, loads a count into d0, then reads bytes from (a0)+ until a zero byte or the count runs out, branching away.
- `decrement_a0_call_shared_helper`: Subtracts one from a0, calls the shared helper routine at the following address, then returns to the caller.
- `copy_string_with_padding`: Copies bytes from the source pointed to by a1 to the destination pointed to by a0, decrementing d0, stopping on a zero byte, then pads the destination with zero bytes while d0 remains positive.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `init_linked_list_header`: Takes a pointer argument from the stack, stores the pointer plus four at its start, clears the next field and points the third field back to itself.
- `draw_icon_with_offsets`: Saves d2-d3/a2, reads three stack arguments, calls a rectangle fill with the icon pointer, computes scaled offsets from icon fields, calls a conversion routine twice, then calls the fill again with the icon data.
- `save_regs_and_load_three_args`: Pushes d2, d3 and a2 onto the stack, loads the three longword arguments from the caller's stack into d2, d3 and a2, then branches to another routine.
- `walk_icon_list_calling_two_helpers`: Saves registers, calls a helper with argument 16 and the current node, then calls it again with the node's byte field scaled by four and its longword field, following the next pointer until a call fails or the list ends, returning 1 on success and 0 on failure.
- `return_zero_restore_regs`: Sets d0 to zero as the return value, pops d2, d3 and a2 from the stack and returns to the caller.
- `create_icon_from_image_spec`: Saves d2-d5/a2, allocates a 20-byte icon structure, copies image data via library calls, fills width/height fields and returns the icon pointer or -1.
- `return_minus_one_restore_registers`: Loads -1 into d0 as the failure return value, restores d2-d5 and a2 from the stack, then returns to the caller.
- `begin_icon_remap_with_zero_counter`: Saves d2-d6/a2, loads three arguments from the stack into d2, d3 and d4, clears d6 to zero and branches into the remap loop.
- `allocate_icon_node_and_link`: Pushes a size and tag onto the stack, calls the allocator, stores the returned pointer at offset 56 of the stack frame, unwinds the stack, and branches based on the result and d6.
- `build_icon_list_from_entries`: Walks a linked list of icon entries, allocating nodes and copying fields via library calls, storing pointers at offsets in a2, and returns d6 or minus one on failure.
- `return_minus_one_restore_registers_2`: Loads minus one into d0 as the failure return value, restores d2 through d6 and a2 from the stack, and returns to the caller.

### info.library

- `illegal_instruction_trap_stub`: Executes the illegal instruction word, which triggers a processor exception; it is a deliberate trap stub with no return.
- `store_library_base_and_data_pointer`: Saves a6 and d0 into absolute memory, copies d0 into a1, stores a0 at offset 34 of that pointer and also at another absolute location, then returns.
- `call_absolute_zero_after_saving_a6`: Pushes a6 onto the stack, calls the routine at address zero, discards the pushed longword by adding four to the stack pointer, and returns.
- `library_call_via_a6_offset_78`: Pushes a6 onto the stack, calls the library routine at offset 120 through a6, then discards the saved a6 and returns.
- `library_call_via_a6_offset_12`: Pushes a6 onto the stack, calls the library routine at offset 18 through a6, then discards the saved a6 and falls through to the next routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `library_call_with_a1_and_a6_saved`: Pushes a1 and a6 onto the stack, calls the library routine at offset 1506 through a6, then discards both saved registers and returns.
- `clear_bit3_and_increment_open_count`: Loads the structure pointer from four bytes above the return address, clears bit 3 of its byte at offset 14, increments the word at offset 32, and returns the pointer in d0.
- `close_info_library_and_release`: Saves d2 and a2, loads the library base from the stack, calls the library close routine, and if the open count word at offset 32 is nonzero sets bit 3 at offset 14 and calls the release routine, returning zero in d0.
- `close_library_with_range_arguments`: Saves d2 and a2, calls the library close routine, then pushes the sum and difference of the words at offsets 16 and 18 of a2, calls two library routines, restores d2 and a2, and returns the saved d2 value in d0.
- `decrement_open_count_and_close`: Loads the library base from the stack, decrements its open count word, and if it reaches zero with bit 3 set, calls the close routine with the base.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `close_library_and_free_resources`: Saves d2/a2, then conditionally calls cleanup routines for the passed arguments and several global pointers, freeing each non-null resource before restoring registers and returning.
- `allocate_and_init_library_base`: Saves a2-a4, allocates a 944-byte block with the given flags, stores the pointer in a global, and returns zero if allocation failed.
- `init_info_library_structures`: Copies a base pointer from (a2) into (a4) and stores offsets into many global pointers, then copies 11-long and 9-long blocks from static tables into those structures and calls a helper twice with size 12.
- `open_dos_library_and_store_base_2`: Saves a2-a3, calls the dos.library open routine with a name pointer, stores the returned base in a global, and branches to an error path if it is zero.
- `open_icon_library_and_store_base`: Pushes a size and name, calls the icon.library open routine via a2, stores the returned base in a global, and branches on failure.
- `open_intuition_library_store_base`: Pushes a longword and the address of the intuition.library name, calls the library open function via a2, stores the returned base at offset 72, then cleans the stack and branches on the result.
- `open_graphics_library_store_base`: Pushes a longword and the graphics.library name, calls the open function via a2, stores the returned base at offset 76, and on failure calls the function in a3 with arguments 5, 0 and 0 before returning zero.
- `return_one_restore_a2_a3`: Sets d0 to 1 as the return value, restores registers a2 and a3 from the stack, and returns to the caller.
- `push_a2_and_test_library_flag`: Pushes a2 onto the stack, loads a1 with the absolute address 88, tests the longword there and branches away if it is nonzero, otherwise branches elsewhere.
- `load_node_and_check_type_seven`: Loads a0 from the pointer at a1, compares the byte at offset 61 of that structure with 7, and branches away if it differs, otherwise branches elsewhere.
- `load_node_fields_and_return_offset_value`: Loads a0 from the pointer at a1, reads a2 from offset 56 of that structure, tests the longword at offset 164 and branches if zero, else returns that longword in d0.
- `get_node_field_via_list_head`: Loads the pointer at a1 into a0, checks byte at offset 61 of that structure, and if it equals one, follows offsets 76 and 442 to return a longword in d0.
- `return_a2_or_follow_offset_76`: Moves a2 into d0; if a2 is zero it branches away, otherwise loads the longword at offset 76 of a2 into a0 and branches onward.
- `return_zero_and_pop_a2`: Sets d0 to zero as the return value, pops a2 from the stack and returns to the caller.
- `open_disk_info_and_store_result`: Opens disk.info via the library open call, stores the returned handle at offset 152 of the new info structure, then checks a flag byte and branches to the shared continuation.
- `copy_info_name_and_query_status`: Copies the name field from the info structure into a local buffer, calls the query routine with -2, clears the global flag word, and returns the status or a field from the nested structure.
- `allocate_info_buffers_and_initialize`: Allocates two 65536-byte buffers, stores their pointers in globals, validates them, then allocates a third buffer sized from the info structure and fills the info fields.
- `allocate_10000_bytes_and_store_result`: Pushes 65536 and a pointer to address 260, saves d3, calls library routine at 244, stores the returned d0 into (a4), then pops the stack and branches based on the result.
- `check_node_flag_and_call_cleanup`: Loads d4 into a1, reads the pointer at (a1) into a0, tests byte at offset 61 of that structure, and if equal to 1 pushes address 96 and d2, calls routine at 76, then pops the stack.
- `free_d2_block_if_nonzero`: Tests d2 and returns if zero; otherwise pushes (a4) and d2, calls library routine at 48, pops the stack, and branches depending on the returned d0.
- `compute_info_buffer_size_and_allocate`: Calls library helpers to compute a size from the d4 structure, stores size plus one at offset 100 of the a3 structure, clamps it to at least 256, then allocates a buffer and stores the pointer at offset 96, returning nonzero on success.
- `store_info_buffer_and_source_pointers`: Calls a helper with the buffer pointer at offset 96, then copies fields from the d4 and a4 structures into offsets 104 and 112 of the a3 structure, restores registers and returns 1.
- `initialize_info_library_state`: Clears several library globals, saves the caller argument, then calls the setup routine and branches to cleanup or continues initialization depending on the returned status.
- `open_info_library_base_and_store_result`: Pushes a size value, a low memory pointer and a library base offset, calls a library open routine, stores the returned pointer at offset 100 in the base, then cleans the stack and branches on the result.
- `open_info_library_base_and_branch`: Pushes a6-relative longword and calls a library open routine via pc, tests d0, discards four stack bytes, and branches to a shared tail passing a3 and two arguments.
- `call_library_allocator_and_save_pointer`: Calls an indirect library routine, moves the returned pointer into a3 and d2, then branches depending on whether the allocation succeeded.
- `alloc_68_byte_block_store_in_a4`: Pushes the immediate value 68 and calls a subroutine, stores the returned pointer at (a4), pops the argument, then branches to another routine with a3 and two stack arguments.
- `info_library_open_and_dispatch`: Pushes a4 contents and a3 offset fifty, calls a library routine, then calls another with a3 and a6-relative argument, tests a global flag and dispatches through a2 with one or zero.
- `call_pc_relative_get_d2_test_and_branch`: Calls a pc-relative subroutine, saves its result in d2, and if nonzero pushes d2 and -4(a6) to call another subroutine, branching based on the result.
- `info_library_cleanup_and_close`: Checks a global word and d2, calls several library routines to release resources, invokes a callback through a2, then restores registers and returns.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
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
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `info_library_open_query_context`: Builds a parameter block on the stack with constants and a global pointer, calls the open routine, and on success passes the handle to a configure routine before returning it in d0.
- `info_library_dispatch_by_node_type`: Loads a node pointer and value from the stack, reads a word from the node, bounds-checks it against 17, and jumps through a nearby word table.
- `library_return_one_with_a2_a3_restore`: Library code sets d0 to 1 as return value, pops a2 and a3 from the stack, then returns to the caller.
- `library_toggle_flag_by_message_code`: Library code reads a word from the second argument, maps codes 6 to 9 and 17 to 19 to bit masks, XORs the mask into a global flag, then calls a routine with the first argument's field.
- `set_flag_bits_and_store_xor_value`: Sets bits in the byte at a4 and in a global variable and in an indexed byte of the structure at a4, stores d1 XOR 4 into a longword at offset 112 of the structure pointed to by a global variable, then pushes a longword from offset 44 of a1 and calls another routine.
- `xor_store_and_call_helper_2`: Exclusive-ORs the value in d0 with the value in d1, stores the result at offset 112 of a structure pointed to by a global variable, then calls another routine.
- `xor_store_and_call_helper_3`: Exclusive-ORs the value 1 with the value in d1, stores the result at offset 112 of a structure pointed to by a global variable, then calls another routine.
- `xor_store_and_call_helper_4`: Exclusive-ORs the value 16 with the value in d1, stores the result at offset 112 of a structure pointed to by a global variable, then calls another routine.
- `xor_store_and_call_helper_5`: Exclusive-ORs the value 32 with the value in d1, stores the result at offset 112 of a structure pointed to by a global variable, then calls another routine.
- `xor_store_and_call_helper`: Exclusive-ORs d0 with d1, stores the result at offset 112 of the structure pointed to by a global variable, then pushes the longword at offset 44 of the structure in a1 and calls another routine with it.
- `info_library_open_and_validate`: Library code that opens the info device, copies fields into the global info structure, opens and checks the named info file, and returns a boolean success flag in d0.
- `check_info_state_and_push_disk_string`: Loads the global info structure pointer into a0, compares the byte at offset 61 with 1, and if not equal pushes the address of the Disk string before branching onward.
- `info_library_validate_and_close`: Library code that copies the info name into a stack buffer, compares it with the global info structure, checks the file against stored values, and returns a success flag in d0.
- `open_info_library_four_entries`: Saves d2-d3/a2, calls the library open routine with the name argument and the first global entry, then returns 1 if that call succeeds or 0 if it returns -1.
- `open_second_info_entry`: Pushes -1, the second global entry pointer and d2, calls the routine through a2, compares the result with -1 and branches onward.
- `open_third_info_entry`: Pushes -1, the third global entry pointer and d2, calls the routine through a2, compares the result with -1 and branches onward.
- `compare_jsr_result_to_ff_and_clean_stack`: Pushes d3, the longword at absolute address 320 and d2 onto the stack, calls through a2, compares the returned d0 with 255, removes three longwords from the stack and returns zero if equal, otherwise branches away.
- `call_vector_114_and_return_one`: Pushes d2, calls the routine whose address is stored at absolute address 276, sets d0 to one, discards the pushed longword, restores d2 and d3 and a2 from the stack and returns.
- `info_library_draw_status_screen`: Library routine that saves a2 and a3, loads the caller's structure pointer, then repeatedly pushes coordinate and colour words and calls the drawing helper to render the NAME, STATUS and TYPE screen before restoring registers.
- `draw_seven_rect_segments`: Pushes seven sets of rectangle coordinates and a flag onto the stack and calls the library drawing routine for each, then restores the stack and registers.
- `draw_seven_rect_segments_alt`: Pushes seven different rectangle coordinate sets with fill flags onto the stack and calls the library drawing routine for each, then cleans up and returns.
- `draw_nine_rect_segments`: Pushes nine rectangle coordinate sets with fill flags onto the stack and calls the library drawing routine for each, then releases the stack and returns.
- `info_library_show_nine_message_boxes`: Saves d2/a2, loads the library base from the stack, then calls the message routine nine times with different argument sets to display nine info boxes, and restores the stack and registers.
- `info_library_check_version_and_set_error`: Saves registers, loads the library base and a value from the stack, reads a byte at offset 61 of the global structure, and sets d1 to 131 unless that byte equals 4.
- `info_library_report_missing_node_error`: Loads error code 154 into d1, compares a node pointer with its own first field, and if unequal calls the error reporting routine with several global values and the node's field.
- `info_open_alloc_and_init_node`: Copies d1 to d0, adds 8, pushes arguments and calls library allocation and initialisation helpers on a2, then optionally frees a global pointer before restoring d2 and a2-a3.
- `info_alloc_node_and_attach_children`: Saves registers, reads the caller argument, allocates a 14-byte node with 65536-byte pool, stores it at offset 234 of the library base and returns zero if allocation failed.
- `info_link_node_children_and_set_head`: Uses a3 to reach the library base, writes the node's own address into its first and eighth bytes, clears offset 242 and walks the child list at offset 92 of the global base.
- `open_info_library_stream`: Pushes a 65536-byte size, a low-memory pointer and d2, calls the library open routine, stores the returned handle in a2 and d3, then cleans the stack and branches based on the result.
- `iterate_info_entries_and_open_streams`: Loops over a pointer array, opening a 65536-byte stream for each entry, storing handles at offset 10 of a2, calling three library routines, and finally copying a pointer into offset 238 before restoring registers.
- `dispatch_info_entry_by_state`: Compares fields of the global info structure at offsets 234 and 238; if equal or matching, calls three library routines with a2 fields, otherwise branches to alternate handling, then restores a2-a3.
- `advance_info_node_to_next_link`: Pushes two library globals and the node's offset-ten field, calls two library routines, then sets the node pointer at offset 238 to its next link and calls the cleanup routine before restoring a2-a3.
- `check_info_list_bounds_and_dispatch`: Compares the list head's offset-eight field and offset-four pointer against the tail at offset 238; if either matches it calls the notify and message routines with fields from a2, otherwise it advances the node and calls the other path.
- `reset_info_node_to_list_head`: Pushes two library globals and the node's offset-ten field, calls two library routines, then copies the node's first link into offset 238 and calls the cleanup routine before restoring a2-a3.
- `info_lib_append_new_node`: Library code that sets a flag in the info base, flushes the current node, allocates two nodes of sizes 14 and 128, links them, and appends the new node to the list.
- `info_lib_remove_current_node`: Library code that sets a flag, flushes the current node, unlinks the current node from the list, frees it, and advances the current pointer to the next node.
- `info_lib_free_and_advance_node`: Library code that reads the next pointer from the current node, frees that node, and if the list wraps, resets the current pointer to the list head.
- `info_library_cleanup_and_close_2`: Calls library helper at offset 144, pushes fields at offsets 46 and 62 of a2 plus a2 and zero, calls helper at offset 88, then restores d2/a2-a3 and returns.
- `info_library_scan_node_list`: Saves d2-d6/a2-a5, loads constant 100 into d3 and 88 into a4, sets a1 to d3, loads a0 from (a1), tests long at offset 242 of a0 and branches based on the result.
- `info_library_clear_node_field`: Loads a0 from (a1), reads pointer at offset 234, sets a2 to that plus 4, compares it with the value at (a0), and if equal clears long at offset 92 of (a4).
- `load_info_vectors_and_chain_next`: Pushes the library base from absolute address 272, sets a5 to d3, loads a2 from (a5), fetches a0 from offset 238 of a2, pushes a0's offset 10, calls the library vector at 22, sets d4 to 1, reloads a3 from offset 234 of the new a0, pops 8 bytes and branches onward.
- `count_entries_alloc_and_copy_table`: Walks the a3 list counting nodes in d4, scales by 4, allocates that many bytes via the library call, stores the pointer at offset 92 of a2, then copies each node's data into the new table, returning 1 on success or 0 on allocation failure.
- `free_table_and_rebuild_entry_list`: Frees the table at offset 92 of a2 through the library call, resets d4, reloads a3 from offset 234 of the a5 structure, then allocates and copies each list node's data into a fresh table, returning 1 on success or 0 on failure.
- `allocate_and_fill_info_node_buffer`: Calls library helpers to allocate a buffer for the current node, stores the pointer in the node table, fills it from the node data, then advances to the next node until the list ends.
- `copy_node_data_and_advance_list`: Copies the current node's data into its allocated buffer via library calls, increments the node index, then walks the node list and returns one when the end is reached.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_info_layout_by_mode_byte`: Saves registers, loads the info structure and mode byte, bounds-checks it between one and six, then jumps through a table to the matching layout drawing code.
- `draw_nine_rectangles_with_fill`: Calls the drawing routines through a3 and a4 in sequence, interleaving calls to a draw-nine-segments helper and a self-referencing call, and ends by making two more calls through a3.
- `load_read_only_string`: Loads the address 96 into a0 and then branches to offset 448 in the same routine.
- `load_validating_string`: Loads the address 106 into a0 and then branches to offset 448 in the same routine.
- `load_read_write_string`: Loads the address 118 into a0 and then branches to offset 448 in the same routine.
- `call_address_in_a3_2`: Pushes the constant 1 and a longword from a2 onto the stack, calls the routine whose address is in a3, then drops the pushed arguments and returns.
- `draw_rect_groups_via_a3`: Calls the routine whose address is in a3 three times, calls two other routines, calls the routine in a3 once more, then returns.
- `draw_multiple_rect_sets`: Calls the routine in a3 seven times and the routine in a4 twice, calls three other routines, calls the routine in a3 once more, then returns.
- `draw_rects_and_show_messages`: Calls the routine in a3 eight times and the routine in a4 twice, calls three other routines, calls the routine in a3 again, calls one further routine, then returns.
- `draw_rectangle_segment_groups`: Calls the routine at a3 three times, calls two other routines, then calls the routine at a3 once more and returns; the decompiler shows no stack pushes of coordinates in this body.
- `call_address_in_a3`: Pushes the constant 1 and the longword in a2 onto the stack, calls the routine whose address is in a3, then drops the pushed arguments and restores registers d2, a2 through a5 from the stack before returning.
- `info_library_allocate_four_buffers`: Saves registers, calls an allocation routine four times with sizes and flags, stores results in globals, and returns zero if any allocation failed.
- `allocate_second_raster_set_and_check`: Calls the allocator four more times for additional bitmaps, stores results in globals and a4, then tests all four and branches to setup or failure.
- `dispatch_info_library_version_index`: Loads the library base from absolute address 88, reads byte at offset 61 into d0, and if it is between 1 and 5 jumps through the word table indexed by twice that value.
- `restore_registers_unlink_and_return`: Restores d2, d3, a2, a3 and a4 from the stack frame at offset -44 from a6, releases the a6 frame and returns to the caller.
- `check_library_flag_and_clear_byte`: Saves d2 and d3, loads the argument from the stack into d2, tests a longword at offset 104 of the base at absolute 100, and if zero clears the byte pointed to by absolute 352.
- `open_info_file_via_library`: Pushes a filename and two library base pointers, calls the open routine, then calls the read routine with a buffer and returns zero if it fails or the result otherwise.
- `restore_d2_d3_and_return`: Pops registers d2 and d3 from the stack and returns to the caller, restoring the saved register values.
- `set_ports_and_open_four_files`: Checks a status byte, writes port values to four structures, then opens four files through the library and returns one only if all succeed.
- `call_a2_with_ff_and_148_vector`: Pushes -1, the longword at absolute address 328, and d2 onto the stack, calls the routine in a2, then compares d0 with -1 and branches based on the result.
- `call_a2_with_ff_and_154_vector`: Pushes -1, the longword at absolute address 340, and d2 onto the stack, calls the routine in a2, then compares d0 with -1 and branches based on the result.
- `call_a2_with_ff_and_158_vector`: Pushes -1, the longword at absolute address 344, and d2 onto the stack, calls the routine in a2, compares d0 with -1, and returns zero in d0 if they differ.
- `return_one_pop_d2_d3_a2`: Sets d0 to 1 as the return value, restores d2, d3 and a2 from the stack, then returns to the caller.
- `poll_signal_bit_and_dispatch`: Reads a signal bit number from the library base, shifts 1 into d4, waits on the signal mask, calls the signal wait and dispatch routines, and loops until a callback returns zero.
- `return_zero_pop_d2_d3_d4_a2`: Sets d0 to 0 as the return value, restores d2, d3, d4 and a2 from the stack, then returns to the caller.
- `dispatch_info_node_by_type`: Saves d2-d3/a2, loads node pointer and value from the stack, reads the node type at offset 20 and branches to type-specific handlers for values 4, 64 and 512, otherwise falls through to a default handler.
- `push_d2_a2_call_then_cleanup`: Pushes d2 and a2 onto the stack, calls the routine whose address is in a4, then adds 8 to the stack pointer to discard both saved registers and branches onward.
- `push_a2_call_clear_d0_branch`: Pushes a2 onto the stack, calls the routine whose address is in d4, sets d0 to zero as the return value, then branches onward without restoring a2.
- `call_two_library_hooks_return_one`: Pushes d3 and a2, calls the library routines at offsets b8 and d4, sets d0 to 1, restores d2-d3/a2 and returns.
- `dispatch_info_library_open_sequence`: Saves a2, loads the argument, calls three library routines, checks a status byte, conditionally calls another, then invokes two more routines with the object and flag before restoring a2.
- `configure_four_info_flags_from_status`: Reads a status longword, masks and inverts its low bits, then calls a helper four times passing indices 0-3 and the corresponding flag bits.
- `draw_info_strings_with_offsets`: Sets up a small offset table and two string pointers, then calls the string drawing routine with the passed text and coordinates.
- `format_and_draw_two_info_lines`: Formats a string into a stack buffer, then draws two text lines using the offset table, coordinates and passed string pointers.
- `load_first_arg_and_clear_d1_a1`: Loads the first stack argument into d0, sets d1 to zero, clears a1, then branches into another routine.
- `skip_to_end_of_string_at_offset`: Increments a1 until the byte at a1 plus d0 is zero, then returns the resulting a1 pointer in d0.
- `copy_null_terminated_string`: Copies bytes from the source pointer in the second stack argument to the destination pointer in the first, including the terminating zero byte.
- `load_three_args_and_branch`: Loads the first two stack arguments into a0 and a1 and the third into d0, sets d1 to zero, then branches to another routine.
- `copy_bytes_until_count_reached`: Copies bytes from the address in a0 to the address in a1, incrementing both pointers and d1 until d1 reaches the count in d0, then returns.
- `load_two_stack_args_into_a0_a1`: Loads the first two longword arguments from the stack into a0 and a1, then branches forward into shared library code.
- `compare_strings_skip_first_char`: Advances a0 and a1 past their first bytes, then compares the two null-terminated strings byte by byte, returning nonzero in d0 if they differ and zero if equal.
- `strlen_return_length_plus_one`: Reads the byte at the caller-supplied address plus an incrementing index until a zero byte is found, then returns that index plus one in d0.
- `format_and_emit_info_string`: Saves registers, calls the local formatter on the last argument, then passes the base pointer, two values and the length minus ten to three library routines through the object at offset 50.
- `build_stack_buffer_then_emit`: Creates a stack frame, formats the fifth argument into a twelve-byte local buffer using a global format, then forwards all arguments plus that buffer to the emit routine.
- `load_byte_from_indexed_table`: Saves d2/a2-a3, loads a0 and d0 from the stack arguments, computes a0 times four into a3, reads a signed byte from that table offset into d1, sets d2 to one and a2 to one, then branches onward.
- `copy_bytes_until_limit_and_terminate`: Copies bytes from a3-offset source to d0-offset destination while incrementing a2 until it exceeds d1, then writes a zero terminator at d1 plus d0 and restores d2/a2-a3.
- `initialize_info_structure_and_open`: Fills two caller-supplied structures with fields from the stack arguments, calls two setup routines, then invokes the open routine with a3, a2 and minus one, returning success or failure.
- `call_a3_offset_3e_and_return_one`: Pushes a3, calls the function pointer stored at offset 62 in the a3 structure, then returns 1 in d0 after restoring registers d2-d3/a2-a3.
- `draw_rect_via_three_graphics_calls`: Calls three library functions with the a2 structure's offset 50 pointer, passing coordinates reduced by 10, to set up and draw a rectangle.
- `format_string_via_rawdofmt_2`: Loads the exec library base from address 4, then calls RawDoFmt with the format string at pc-relative offset 36 and the argument list from the stack.
- `store_d0_byte_to_a3_postincrement`: Writes the low byte of d0 to the memory pointed to by a3, then increments a3 by one and returns to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.

### version.library

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `store_a6_and_a0_into_d0_struct`: Copies d0 into a1, then stores a6 at offset 34 and a0 at offset 38 of that structure before returning.
- `increment_a6_counter_clear_bit3_return_a6`: Adds one to the word at offset 32 of a6, clears bit 3 of the byte at offset 42, returns a6 in d0.
- `decrement_open_count_and_close_2`: Sets d0 to zero, decrements the word at offset 32 of the library base in a6, and if it reaches zero with bit 3 of byte at offset 42 set, calls the close routine.
- `open_library_increment_and_set_flag`: Saves d2/a5-a6, builds a stack frame, loads the library base from offset 34 into a6, and if the word at offset 32 is nonzero sets bit 3 of byte at offset 42 and returns zero in d0.
- `expunge_library_free_memory`: Saves d2, calls the library remove routine with a5, then calls the free-memory routine using the base and size words at offsets 16 and 18, and returns the saved value in d0.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `jump_to_library_init`: Branches unconditionally to the diskfont library initialization code at the start of the library, transferring control without touching registers.
- `open_diskfont_library_and_init`: Saves a0 and a2, calls the library open routine with size 56 and two data pointers, stores a0 at offset 34 of the result, then calls the close routine and returns 1 on success.
- `open_dos_and_graphics_for_diskfont`: Saves a6, opens dos.library and graphics.library via exec.OpenLibrary, stores their bases in globals, initializes a list header at offset 42 of the library base and returns that base.
- `close_font_and_free_library_base`: Closes the font resource via exec.CloseLibrary, then frees the library base memory using exec.FreeMem with the base size, and returns zero in d0.
- `close_diskfont_library`: Walks the library's open-resource list, unlinks entries with no users, calls UnLoadSeg on their segments, decrements the open count, and frees the library base when unused.
- `close_library_if_unused_and_unlink_base`: Checks the open count and font counter, closes two libraries via exec.CloseLibrary, frees the base memory with exec.FreeMem, unlinks the base node and returns the saved value.
- `set_open_flag_and_return_zero`: Sets bit 7 of the byte at offset 14 from a6 and loads zero into d0, then falls through to the next routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `open_diskfont_library`: Clears the library's open flag bit, increments the open counter at offset 38 of the library base, and returns the base pointer in d0.
- `decrement_count_and_jump_close_vector`: Subtracts one from the word at offset 38 from a6; if nonzero it returns, otherwise if bit 7 at offset 14 is set it jumps through the vector at offset minus 18 from a6.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `build_font_list_from_diskfont_entries`: Sets up a stack frame, saves registers, clears the first word of the output buffer, then walks the diskfont list at the library base, copying each font name and entry fields into the caller's buffer and counting entries.
- `load_font_name_pointer_and_set_flag`: Loads a pointer from offset ten of the a4 structure into a1, sets d4 to one, and branches forward into the shared font-name scanning code.
- `scan_font_name_and_advance_list`: Increments d4 while scanning bytes at a1 until a zero terminator, subtracts the name length and ten from d5 and d2, then reloads a1 from the a4 structure and clears a3 before branching onward.
- `copy_font_name_and_advance_chunk`: Copies bytes from a1 into the chunk buffer at a3 plus d5, increments a3, then writes chunk header fields from a4 and bumps the count at d6 before advancing a2 by ten bytes.
- `open_font_file_and_check_suffix`: Opens the font file named at offset 8 of a4, checks for a dot suffix, tests the size field, opens the file again and sets d4 to one before branching.
- `count_font_name_length_and_adjust_buffers`: Increments d4 while scanning the null-terminated name at a1, subtracts the length from d5 and d2, then resets a1 to a4 plus 8 and clears a3.
- `copy_glyph_bytes_until_width_limit`: Copies bytes from the a1 pointer into the a3-indexed buffer one at a time, incrementing a3, until the 16-bit width in d4 is reached, then calls the read routine and checks for a 3840 marker.
- `read_glyph_record_and_append_entry`: Reads a 260-byte glyph record into the a6 frame, and if valid and d2 has room, writes a 10-byte entry to a2, bumps the d6 counter and advances a2 by 10.
- `return_zero_restore_registers`: Sets d0 to zero as the return value, restores d2 through d6 and a2 through a4 from the stack frame, releases a6 and returns to the caller.
- `diskfont_compute_kerning_adjust`: Saves d2-d6, loads the font and glyph pointers from the stack, and computes a kerning adjustment from the glyph pair's spacing and style bytes, returning it in d0.
- `diskfont_apply_pair_kerning_loop`: Adjusts the kerning value in d2 by subtracting per-bit table entries for the glyph pair's style bytes, looping over seven bits and using a0 and a1 as glyph and font pointers.
- `diskfont_subtract_second_glyph_kerning`: Subtracts per-bit kerning table entries for the second glyph's style byte from d2, looping over the remaining bits and using a1 as the font pointer.
- `scale_d4_index_and_subtract_table_entry`: Shifts d4 left by two into d0, subtracts the longword at offset 72 plus d0 from d2, doubles d3, increments d4 and branches back while d4 is below seven, else branches away.
- `return_error_code_minus_32767`: Loads the constant a fixed address into d2 and d0 as the return value, restores d2 through d6 from the stack and returns to the caller.
- `open_font_and_select_best_match`: Builds a font request on the stack, opens it, reads its metrics, then scans candidate fonts for the best match and returns the chosen font pointer.
- `open_fonts_dir_and_store_lock`: Pushes a negative two mode and a null name, calls the open routine, stores the returned lock in library base offset minus twelve, cleans the stack and branches on the result.
- `load_font_file_and_pick_best_size`: Opens the FONTS directory, reads a font header, allocates and reads the font file, then scans its size entries for the best match and returns the chosen font pointer.
- `clear_font_result_registers`: Clears the word in d4 and d3, sets d2 to minus one and branches onward, signalling no font was selected.
- `scan_font_entries_pick_best_match`: Loops over font entries at d5 indexed by d3 times 260 bytes, calls the comparison routine with each entry and the a6 context, keeps the highest score in d4 and its index in d2, then opens the chosen font and initializes its structure.
- `clear_font_entry_flag_and_chain`: Pushes d4, calls the memory free routine, computes the entry address from d2 times 260 plus d5, clears the word at offset 256, pops the stack and branches to the common exit.
- `release_font_entry_and_restore_registers`: Clears the word at offset 256 of the entry selected by d2, frees d3 if set, and when d2 is negative frees the cached font resources, then restores d2-d7/a2-a4, releases a3 if distinct from a4, and returns a4 in d0.
- `load_diskfont_from_file`: Opens the named font file, reads its header, allocates a font structure sized from the file, fills character entries and returns the new font pointer or zero.
- `build_font_from_open_file`: Continues loading an open font file, allocates and fills the font structure with character data, closes the file and returns the font pointer or zero.
- `free_font_via_library`: Loads the font pointer from the stack argument, pushes its size field and the pointer, calls the library free routine and returns.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
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
- `copy_string_to_destination`: Reads source and destination pointers from the stack, copies bytes from source to destination including the terminating null, advancing both pointers, then returns.
- `load_string_pointer_and_length`: Loads a string pointer from the stack into a0 and a length value from the stack into d0, then branches forward into shared library code.
- `skip_to_string_end_and_call`: Advances a0 past the null-terminated string, pushes d0 and a0 on the stack, calls a library routine, cleans the stack and returns.
- `load_first_stack_arg_into_a0`: Loads the first longword argument from the stack into address register a0, sets d0 to zero and branches onward into shared library code.
- `count_string_length_in_d0`: Increments d0 for each byte read through a0 until a zero byte is found, returning the string length in d0.
- `load_two_stack_args_for_compare`: Loads the first stack argument into a1 and the second into a0, then branches into the shared string comparison code.
- `scan_string_until_null_advance_a0`: Reads bytes from the address in a0, incrementing a0, until a zero byte is found; then sets d0 to zero and branches onward.
- `compare_two_strings_bytewise`: Loads a byte from a0, compares it with the byte at a1 and increments a1; on mismatch it compares again and returns minus one in d0 if a0 is lower, otherwise branches onward.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `load_stack_args_into_a0_and_d0`: Loads the longword at offset four from the stack pointer into a0 and the byte at offset eleven into d0, then branches to shared code.
- `compare_bytes_and_copy_pointer`: Compares the low bytes of d0 and d1; if equal it copies a0 into d0, otherwise it branches away without changing d0.
- `advance_pointer_scan_next_byte`: Increments a0, loads the next byte into d1, and if that byte is zero returns zero in d0, otherwise branches back to the comparison.
- `allocate_and_offset_by_length`: Saves d2 and d3, loads a length from the stack into d3 and a byte into d2, calls a library routine, then adds d3 to the returned pointer and branches on.
- `match_byte_and_return_pointer`: Compares the byte at the address in a0 with d2 and, if equal, copies a0 into d0 before branching onward; otherwise it branches to the alternate path.
- `decrement_pointer_compare_d3_return_zero`: Subtracts one from a0, compares it with d3, branches if greater or equal, otherwise sets d0 to zero, restores d2-d3 from the stack and returns.
- `check_two_offsets_advance_result`: Saves d2, loads the argument into d2, calls the lookup routine with offsets 47 and 58, and returns the found value plus one, or the argument unchanged.
- `increment_d0_return_via_d2`: Adds one to d0, copies the result through d2, restores d2 from the stack and returns the incremented value in d0.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `save_d0_d1_a0_call_vector`: Pushes d0, d1 and a0 onto the stack, calls the routine at address zero, then discards twelve bytes of stack and returns.
- `save_a0_call_e0_vector`: Pushes a0 onto the stack, calls the routine at address two hundred twenty-four, discards four bytes of stack and returns.
- `save_a0_a1_call_vector_restore_stack`: Pushes registers a0 and a1 onto the stack, calls the routine whose address is zero, then adds eight to the stack pointer and returns.
- `save_a1_call_vector_restore_stack`: Pushes register a1 onto the stack, calls the routine at address 576, then adds four to the stack pointer and returns.

### Ultimate Quiz, The (1993)(AGAS)[HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_7`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_hardware_state_2`: Clears a global variable, calls a routine to initialize hardware, and if a specific field at offset 172 of the returned structure is non-zero, calls two additional routines and stores the resulting pointer in the global variable.
- `initialize_graphics_and_hardware_state`: Opens a library, allocates memory, and clears a block of memory using a loop.
- `initialize_structure_members`: Clears four 32-bit words at a location pointed to by a5, then stores several calculated pointers into offsets of the structure in a5.
- `initialize_graphics_buffer_and_hardware`: Copies a list of words from a table into a destination buffer, then populates several memory blocks with specific patterns and values, including interleaved word pairs and data from a source table.
- `copy_word_list`: Decrements a counter in d0 and enters a loop that copies words from a source pointed to by a1 to a destination pointed to by a0, incrementing both pointers by two each iteration.
- `store_swapped_words`: Decrements a counter in d0 and enters a loop that swaps the high and low words of d1 and stores them into a destination pointed to by a0, while adding d2 to d1 each iteration.
- `configure_hardware_parameters`: Writes values to various offsets of a structure in a6, including a value derived from adding a constant to a value at offset 24 of a structure in a5, and waits for a bit to clear in a byte at offset 2 of a6.
- `configure_hardware_parameters_alt`: Writes values to various offsets of a structure in a6, including a value derived from adding 4 to a value at offset 24 of a structure in a5, and waits for a bit to clear in a byte at offset 2 of a6.
- `clear_memory_blocks`: Decrements d0 by two bits via a logical shift right and enters a loop that writes zero to the location pointed to by a0 for each iteration.
- `process_quiz_input_and_update_display`: Decrements a counter in a structure and, if zero, updates a display buffer with a pointer and then parses a string to calculate a value for a counter.
- `initialize_hardware_settings`: If a byte in a global variable is zero, it configures several hardware registers including DMA and interrupt enable registers, writes specific values to a memory location, and updates two other global variables before returning.
- `update_hardware_configuration`: If a byte in a global variable is non-zero, it performs a loop checking a bit in a memory location, updates several hardware registers with values derived from a table and bitwise operations, and clears the global variable before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_zero_data`: Performs twelve consecutive logical OR operations on register d0 with zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_byte`: Copies a byte from a memory location to a global variable, sets bit 15 of a 16-bit value at offset 154 of a structure in a6 using a value from a global variable, and clears a byte at offset 234 of a global variable.
- `initialize_hardware_registers`: Checks a global variable and if it's zero, initializes several hardware registers including CIAA_ICR, DMACONR, INTENA, and others with specific values before setting the global variable to -1.
- `configure_dma_and_interrupts`: If a global variable is non-zero, configures DMA channels and interrupt registers, waits for a bit to clear, then restores several register values before clearing the global variable.
- `clear_audio_volumes_and_initialize`: Processes data from memory locations, updates arrays of pointers, clears audio volume registers, and resets several global variables related to sound output.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_channels`: Increments a global counter and, if it reaches six, resets it to zero; otherwise, it checks four specific flags and, if a flag is set, calls a routine to update the hardware audio registers using a pointer to a specific audio channel structure.
- `update_audio_hardware`: Processes a structure in a6 by performing various arithmetic operations on its fields based on a mode byte, updates a destination structure via a5, calls another routine four times to configure hardware registers, and manages a DMA-related pointer and counter.
- `update_sprite_attribute_table`: Reads a value from a structure to index into a table, then updates several fields in a destination structure including a pointer, two 16-bit values, and a 32-bit value based on a conditional check.

### Ultimate Quiz, The (1993)(AGAS)[HD].adf:ami1demo.dat#unpacked

- `initialize_system_2`: Initializes system memory structures and display parameters by allocating memory blocks, setting up hardware registers including DMA control, and handles message processing through Amiga OS library calls.
- `initialize_system`: Initializes system memory structures and display parameters by allocating memory blocks, setting up hardware registers including DMA control and blitter configurations, and managing audio channel states.
- `initialize_memory_structure`: Initializes a memory structure by clearing several longword slots, setting pointers to various offsets from an initial address, and updating multiple fields in a data structure.
- `initialize_display_data`: Initializes multiple memory regions with sequential word values, clears memory areas, and sets up display-related parameters.
- `copy_word_pairs`: Copies word pairs from one memory location to another while decrementing a counter, advancing both pointers by two bytes each iteration.
- `swap_and_accumulate`: Swaps the high and low words of a value, stores them alternately at memory locations, accumulates values in a register, and repeats based on a counter.
- `initialize_globals`: Initializes several global variables with specific values including a status flag, counters, and memory addresses, performing conditional jumps based on bit tests.
- `setup_parameters`: Sets up various parameters in global memory locations, including flags, counters, and address pointers, with conditional logic based on hardware status bits.
- `clear_words_by_four`: Clears memory words by four-byte units, shifting a counter right by two bits and writing zero values to memory locations until the counter reaches zero.
- `initialize_display`: Initializes display parameters including screen addresses and timing values, then updates a counter and processes input data from a buffer.
- `initialize_hardware`: Initializes hardware registers and global variables when a condition is met, setting interrupt and DMA control values while updating several memory locations with specific data.
- `restore_hardware_state`: Restores hardware state by configuring interrupt and DMA settings, waiting for specific DMA status conditions, and updating various registers and memory locations based on global variable values.
- `initialize_hardware_copy`: Copies hardware register values to global variables and configures interrupt and DMA control registers, similar to the first routine but with different register settings and a different conditional branch.
- `restore_hardware_state_copy`: Restores hardware state by configuring interrupt and DMA settings, waiting for specific DMA status conditions, and updating various registers and memory locations based on global variable values, with different DMA control values than the second routine.
- `clear_audio_registers`: Clears audio volume registers and related memory locations, processes data from a table to update memory pointers, and initializes several audio-related global variables based on input data.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_channels_2`: Updates audio channel registers based on input data and manages audio playback state, including handling volume and period changes for multiple audio channels.
- `update_input_axes`: Updates joystick or gamepad axis values based on input data, clamps them within defined ranges, and stores the results in a global structure.
- `decode_input_packet`: Reads input data from a buffer, decodes axis values and applies deltas to running totals, then copies decoded fields to a destination structure.
