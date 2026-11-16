# Game 2236 (wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02236). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901308 top-level bytes in 67 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 31 of 31 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 384 of 384 named (0 author symbols), 384 of 384 described (79 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1830 of 1830 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 207860 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10626 of 10627 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16316 |
| code | 31284 |
| data | 838871 |
| padding | 14325 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Setmap | 13 | 653 | 31 |
| Else | 2 | 117 | 2 |
| isdosdev | 7 | 250 | 17 |
| Type | 1 | 426 | 2 |
| SetPatch | 24 | 831 | 43 |
| addbuffers | 2 | 117 | 2 |
| multidie | 7 | 250 | 17 |
| wipeout | 7 | 250 | 17 |
| remove | 7 | 250 | 17 |
| EndCli | 2 | 182 | 3 |
| Echo | 2 | 241 | 10 |
| lab | 1 | 2 | 1 |
| Cls | 2 | 20 | 1 |
| FastMemFirst | 7 | 207 | 6 |
| CD | 2 | 282 | 5 |
| mouse | 2 | 4 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 413 | 15 |
| memavail | 7 | 250 | 17 |
| setpatchV38.31 | 2 | 1010 | 75 |
| Version | 11 | 574 | 18 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Type | 2 | 117 | 2 |
| d | 1 | 3 | 1 |
| gb | 1 | 3 | 1 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:InstallWingCommander | 2 | 3738 | 68 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing | 8 | 252 | 6 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:bootblock | 1 | 126 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 2 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP03.PKT#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP01.PKT#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PLANET0.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (7369 -> 10282 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:ENDGAME.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (41104 -> 84654 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:FONTS.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (5401 -> 10306 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP03.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (60752 -> 102086 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP01.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (66893 -> 105774 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME2.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (36350 -> 63356 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:DEMO.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (65170 -> 111126 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME0.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (43516 -> 72732 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP02.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (45898 -> 78286 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:DEMOSAMP.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (91572 -> 106888 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME3.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (23773 -> 42588 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP00.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (59027 -> 94082 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:MIDGAME1.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (31778 -> 59528 bytes)
- 3 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 9 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 86 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/isdosdev: Imploder (xfd) stream found by ancient scan (4544 -> 6152 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/multidie: Imploder (xfd) stream found by ancient scan (5624 -> 8612 bytes) but only 0 of 4 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/wipeout: Imploder (xfd) stream found by ancient scan (4704 -> 6404 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/remove: Imploder (xfd) stream found by ancient scan (4760 -> 6580 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/memavail: Imploder (xfd) stream found by ancient scan (4436 -> 5956 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing: Imploder (xfd) stream found by ancient scan (139392 -> 278468 bytes) but only 0 of 124 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP03.PKT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:PCSHIP01.PKT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate, 1 immediate+displacement
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:c/multidie: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 207860 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Setmap

- `run_setmap_with_parsed_arguments`: Program entry for the Setmap tool: it allocates a message structure, opens the dos library, and parses its command text into an argument list, handling quoted strings and star escapes for newline and escape characters.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `invoke_optional_handle_routines`: Calls a routine with each of several global handle values when they are set, passing one global by reference, then always calls it with a fixed handle and the incoming argument.
- `store_arguments_and_call_helpers`: Stores its two arguments into two global variables, then calls one helper with a fixed set of arguments that includes a global address and constants.
- `resolve_keymap_name_and_open`: Determines a keymap name from icon tool types, command-line arguments, or a default value, then converts the string to lowercase.
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
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_routine`: Copies sixteen longwords from a fixed offset in the base structure into a caller-supplied buffer when one is given, then calls a routine through an address register.

### isdosdev

- `run_hdinstall_entry`: Program entry for the HDinstall tool.
- `decrement_until_minus_one`: Decrements a 16-bit local counter until it reaches minus one, then returns.
- `initialize_and_process`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic before checking a condition and calling a routine.
- `process_loop_iteration`: Processes a loop iteration by checking a condition and calling another routine, then updates a value at an offset calculated from a register and a global variable before decrementing a counter and branching based on its value.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_7`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer while managing counters and a global variable.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_2`: Processes a bit pattern from a buffer, performing bitwise operations and conditional jumps based on carry flags, then updates a global variable with decoded values.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_bitstream_and_copy_pixels`: Decodes a variable-length bitstream from a source buffer in a0 to reconstruct a value, then uses that value to perform a block memory copy from a source table to a destination buffer.
- `decode_pixel_data`: Processes pixel data from a source buffer, decodes bit flags, and copies decoded bytes to a destination buffer while managing counters and pointers.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Type

- `parse_type_command_arguments`: Parses command-line arguments to identify hex or number flags, validates that they are not used simultaneously, and handles error reporting for invalid options or file access failures.
- `format_output_string`: Formats strings and numbers into a character buffer using specific text patterns and hexadecimal characters.

### SetPatch

- `run_setpatch_with_parsed_arguments`: Program entry for the SetPatch tool: it allocates a message structure, opens the dos library, and parses its command text into an argument list, handling quoted strings and star escapes.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: Installs a SetPatch handler and performs a series of conditional patches and library initialisations.
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
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `check_flag_and_copy_word`: Calls the routine pointed to by a stored handler slot and, when it returns nonzero, copies the 32-bit field at a fixed offset in the global structure into a global word.
- `jump_to_pending_handler`: Compares a stored word with a field of the global structure and returns unless they differ and the stored word is not minus one.
- `check_pending_handler`: Compares a stored word with a field of a global structure and executes another routine if the word is not minus one and if they differ.
- `initialize_signature_and_dispatch`: Sets a flag word to 16384.
- `jump_through_dispatch_pointer`: Jumps through a routine pointer at offset 136 from a global structure pointer, with no other work.
- `initialize_patch_function_table`: Stores a computed address in a global and fills a 16-entry table at a low fixed address with successive addresses of a routine, advancing two bytes per entry.
- `return_input_value_2`: Returns the 32-bit value passed in the data register unchanged.
- `call_routine_fifteen_times`: Calls the same routine fifteen times in succession and returns the value produced by the final call.
- `return_input_value`: Returns the 32-bit value passed in the data register unchanged, performing no other operations.
- `call_setfunction_if_small`: Returns without action when a 16-bit count field in the global structure exceeds 35.
- `allocate_and_copy_pointer_table`: Allocates a block through a library call, then copies a table of 32-bit pointer and value pairs from the source structure into it.
- `free_table_and_return_flag`: Repeatedly calls the second library vector, which releases a block, once for each remaining entry counter value and once more.
- `free_entry_and_return_flag`: Calls the second library vector, which releases a block, then returns the word at the stored pointer with its top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `call_patch_hook_with_stack_scratch`: Saves registers, builds a scratch stack pointer below the current stack, loads a function pointer from the patch base plus offset and calls it with d0 set to twelve, then returns d1.
- `check_field_then_dispatch`: Passes a field of the structure to a checking routine.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker_word`: Advances a pointer through 16-bit words until the high byte of the current word equals 97, then returns a pointer to that word.
- `decode_instruction_displacement`: Reads a displacement from the instruction word at the pointer, using its low byte when that byte is nonzero and otherwise the following word.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_60`: Calls the library routine at offset minus 60 from a global base pointer with no arguments and returns.
- `call_routine_with_stack_slot`: Forwards its two parameters to a second routine together with the address of a fixed stack slot, then returns.
- `call_global_and_checksum_buffer_3`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Writes the low byte of the data register to the memory location addressed by the address register, then returns.

### addbuffers

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_and_call_vector`: If the pointer argument is non-null, copies sixteen 32-bit words from a fixed offset in the base structure into the pointed-to location.

### multidie

- `run_hdinstall_entry_2`: Program entry for the HDinstall tool.
- `decrement_until_minus_one_3`: Counts a 16-bit local value down until it equals minus one, then returns.
- `initialize_and_process_2`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic.
- `process_loop_iterations`: Processes up to four iterations of a loop using pointer arithmetic to update values in memory, with conditional branching based on counter and comparison results.
- `frag_beq_movea_bsr_bra_2`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_2`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_5`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and writing decoded results to a destination buffer while managing counters and a global variable.
- `shift_and_accumulate_3`: Processes bit patterns by shifting and adding values from memory locations, updating counters and accumulators through iterative operations.
- `frag_moveq_bra_4`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_6`: Processes a bit pattern from a buffer to determine decoding parameters and performs bitstream decoding operations based on the extracted values.
- `frag_add_bne_move_addx_bra_2`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_bit_pattern_8`: Processes bit patterns from a source buffer, decodes pixel data using a lookup table, and copies decoded bytes to a destination buffer while managing counters and pointers.
- `decode_bitstream_fragment`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
- `frag_move_movem_bra_2`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### wipeout

- `reset_install_state_at_entry_2`: Calls routines and a library function in a loop, clears a word at a fixed location, and swaps a memory word with a global variable if certain conditions or constants match.
- `decrement_until_minus_one_5`: Decrements a local 16-bit variable until it reaches minus one and then returns.
- `initialize_and_process_3`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic.
- `process_loop_iterations_2`: Processes up to four iterations of a loop, updating values through pointer arithmetic based on counter contents, then checks a condition and branches accordingly.
- `frag_beq_movea_bsr_bra_3`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_and_compare`: Copies four bytes from a source address to a destination address, decrements a counter, and branches back if the counter is not zero, then loads a value from the destination and returns.
- `decode_bit_pattern_9`: Processes bit pattern data from a source buffer, decoding bits using carry flags and conditional operations while updating a global variable, then loads a constant value into a register before branching.
- `shift_and_accumulate_4`: Processes bits from a buffer by shifting and adding values, updating counters and accumulators through bitwise operations.
- `frag_moveq_bra_7`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_8`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_9`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_10`: Processes a bit pattern from a buffer, performing bitwise operations and conditional jumps based on carry flags, then loads decoded values from a table and performs arithmetic operations on them.
- `shift_and_accumulate_5`: Shifts a byte value from a global variable and accumulates it into a register, then performs a conditional branch based on the result.
- `decode_bit_pattern_11`: Processes bit patterns from a source buffer, decodes flags, and copies decoded data to a destination buffer while managing counters and offsets.
- `decode_bitstream_fragment_2`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
- `frag_move_movem_bra_3`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### remove

- `reset_install_state_at_entry_3`: Performs a loop that calls routines and a library function, clears a word, and swaps a memory word with a global variable if a specific constant is matched.
- `decrement_until_negative_one`: Decrements a local or temporary register value until it reaches negative one and then returns.
- `initialize_and_process_4`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic and calling a routine with varying parameters.
- `process_loop_iterations_3`: Processes up to four iterations of a loop, updating values through pointer arithmetic before checking a condition.
- `frag_beq_movea_bsr_bra_4`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_byte_pairs`: Copies pairs of bytes from one memory location to another in a loop, decrementing a counter until it reaches zero.
- `decode_bit_pattern_12`: Processes bit pattern data from a source buffer, decoding bits and copying bytes to a destination while managing counters and carry flags.
- `shift_and_accumulate_6`: Processes bits from a buffer by shifting and adding values, updating registers through a loop that decrements a counter until it reaches zero.
- `frag_moveq_bra_10`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_11`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_12`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_13`: Reads bits from a buffer and performs conditional operations based on bit values, updating counters and pointers during decoding.
- `shift_and_accumulate_7`: Shifts a byte value from a global variable and accumulates it into a register, then performs a conditional branch based on the result.
- `decode_bit_pattern_14`: Processes bit patterns from a source buffer, decodes flags, and copies decoded bytes to a destination buffer while managing counters and a global table.
- `decode_bitstream_fragment_3`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
- `frag_move_movem_bra_4`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### EndCli

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: copies a fixed number of words from a source pointer to a destination pointer if the destination pointer is not null.
- `update_counter_and_branch`: Updates counters and performs conditional operations based on comparison results, then calls a handler routine with specific parameters and jumps to another location.

### Echo

- `find_max_and_prepare_buffer_4`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies a fixed number of dwords from a source pointer to a destination pointer if a certain pointer is not null.
- `update_counter_and_branch_3`: Reads a value from a table using an index from a structure, updates counters and limits in the structure based on table data, and performs conditional execution based on the read value.
- `update_counter_and_branch_2`: Reads a value from a table using an index from a structure, updates counters and limits in the structure based on table data, and performs conditional jumps based on comparison results.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `increment_and_compare`: Increments a counter and compares it against limits, updating values in a structure based on the comparison results and calling other routines conditionally.
- `frag_move_bra_2`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_counters`: Updates several counters and performs conditional operations based on comparison results, then calls a handler routine with specific parameters.
- `frag_move_lsl_tst_bne_moveq_movea_jsr_b`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 10 into d1, then copies 352 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `conditional_execute`: Checks a condition and executes a series of handler calls based on the result, with potential additional calls depending on a register value before jumping to another location.

### lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Cls

- `open_dos_library`: Opens the dos.library through the library vector table and stores the returned base pointer in a local variable.

### FastMemFirst

- `initialize_fast_memory_and_parse_args`: The routine attempts to locate a fast memory block and then retrieves the command line arguments from the operating system.
- `find_and_copy_fastmem_block`: Walks a linked list of memory blocks to identify a fast-memory block with a specific type tag and a chip-memory block with a different type tag, then executes a routine on the fast block and another routine with the identified block pointers.
- `call_library_vector_132`: Calls the library routine at offset minus 132 from the global base pointer and returns.
- `call_library_vector_138`: Calls the library routine at offset minus 138 from the global base pointer and returns.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### CD

- `find_max_and_prepare_buffer_5`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_frame_data`: copies a fixed block of sixteen bytes from a memory location offset from a pointer into the current stack frame, then calls another routine.
- `update_data_entries`: Processes data entries from a table using indexed access, performs conditional operations and updates memory locations based on the results.
- `process_data_entries`: Processes data entries from a table using indexed access, performs conditional operations and updates memory locations based on the results of those checks.
- `finalize_operations`: Completes final operations by calling handler routines with specific parameters, updating memory locations, and performing conditional checks before jumping to another location.

### mouse

- `wait_while_flag_bit_set`: Loops while bit 64 of a global flag word is set, then returns zero.

### EndIf

- `return_zero_status`: Returns zero immediately without other work.

### If

- `find_max_and_prepare_buffer_6`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block_2`: Copies a fixed number of words from a source pointer to a destination pointer, provided a certain check passes.
- `update_object_flags`: Updates object flags based on conditions checked against global variables and table entries, then calls another routine multiple times depending on those checks.
- `frag_move_lsl_tst_beq_moveq_cmp_bgt_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then loads 10 into d2, then compares d2 with the memory at an offset from a2, then branches if the comparison was greater, then loads 255 into d1, then jumps to another point in the code.
- `frag_move_lsl_tst_beq_moveq_cmp_bgt_bra_2`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then loads 20 into d2, then compares d2 with the memory at an offset from a2, then branches if the comparison was greater, then loads 255 into d1, then jumps to another point in the code.
- `process_fragment_data`: Reads data from a table using a pointer, performs comparisons with a fixed value of 36, and updates flags based on the results.
- `validate_and_process`: Reads data from multiple offsets in a structure, performs comparisons with a fixed value of 36, and updates flags based on the results.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_move_lsl_tst_beq_moveq_cmp_bne_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then loads 1 into d2, then compares d2 with the memory at an offset from a1, then branches if the last result was not zero, then loads 255 into d1, then jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_moveq_cmp_beq_tst_bne_bra`: Short code fragment that loads 1 into d1, then compares d1 with the memory at an offset from a1, then branches if the last result was zero, then tests the memory at an offset from a1, then branches if the last result was not zero, then loads 255 into d1, then jumps to another point in the code.
- `update_and_clear_flags`: Reads a value from a table using a pointer, updates a global variable with that value, and clears a flag in a structure when the value is zero.
- `clear_and_update_flags`: Clears a global variable at offset 328 of a structure and sets a bit in a status byte at offset 34 of the same structure, then performs a series of conditional operations based on the value read from a memory location.
- `process_input_data`: Reads a long value from a function call and stores it at offset 600 of a structure in a2, then compares it with the value 32 and branches back to the start if they are equal.
- `process_input_data_2`: Reads a long value from a function call and stores it at offset 600 of a structure, then compares it with the value 32 and branches if equal.

### memavail

- `run_hdinstall_entry_3`: Program entry for the HDinstall tool.
- `decrement_until_minus_one_4`: Decrements a local variable until it reaches negative one and then returns.
- `update_memory_cache`: Increments a counter and performs several operations including adding values to registers, calling functions, and updating memory locations.
- `process_memory_blocks`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic before checking a condition.
- `frag_beq_movea_bsr_bra_5`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_byte_quadruplet`: Copies four bytes from a source address to a destination address, decrementing a counter and branching back if not zero, then loads the final value from the destination.
- `decode_bit_pattern`: Processes bit pattern data by reading bits from a source buffer and writing decoded bytes to a destination buffer, managing counters and carry flags during bit extraction.
- `shift_and_accumulate`: Processes bit patterns by shifting and adding values from memory locations, updating accumulators through conditional operations, and performs a loop of three iterations.
- `frag_moveq_bra_13`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_14`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_15`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_3`: Reads bits from a buffer and uses them to control memory operations, including copying data and updating counters based on decoded values.
- `shift_and_accumulate_2`: Shifts a byte value in a register and accumulates it into another register, then branches based on the result.
- `decode_bit_pattern_4`: Processes bit patterns from a source buffer, decodes flags, and copies decoded bytes to a destination buffer while managing counters and a global table lookup.
- `decode_bitstream_2`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer while managing memory pointers and counters.
- `frag_move_movem_bra_5`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### setpatchV38.31

- `apply_patch_list_with_switches`: Checks the system version and, if a specific Kickstart version is met, iterates through a list of patch records to apply modifications to system libraries.
- `modify_memory_flags`: Modifies bytes and words in memory using bitwise OR operations with immediate values, writes a long value to a memory location with an offset from a0, performs a bit toggle operation on a byte at a specific address, and applies a bitwise AND operation to a long value in memory before branching to a fixed address.
- `initialize_patch`: Initializes patch-related data by checking the Kickstart version and managing cache settings, then processes a patch list.
- `initialize_patch_2`: Initializes patch-related data by setting up pointers and calling version checking routines, then stores a processed status value in a global variable.
- `initialize_patch_3`: Initializes patch-related variables and performs kickstart version checks using OS library calls, then sets a global flag based on the result.
- `frag_move_beq_clr_swap_cmp_bcc_moveq_br`: Short code fragment that copies a3 to d0, then branches if the last result was zero, then copies d6 to d0, then clears d0, then swaps the halves of d0, then copies the memory at an offset from a3 to d1, then compares d1 with d0, then branches if the carry flag is clear, then loads 0 into d5, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_bcc_moveq_bra`: Short code fragment that compares d1 with d0, then branches if the last result was not zero, then copies d6 to d0, then masks d0 with 65535, then copies the memory at an offset from a3 to d1, then compares d1 with d0, then branches if the carry flag is clear, then loads 0 into d5, then jumps to another point in the code.
- `frag_moveq_bra_16`: Short code fragment that loads 0 into d5, then jumps to another point in the code.
- `frag_move_beq_clr_swap_cmp_bls_moveq_br`: Short code fragment that copies a3 to d0, then branches if the last result was zero, then copies d6 to d0, then clears d0, then swaps the halves of d0, then copies the memory at an offset from a3 to d1, then compares d1 with d0, then branches if the comparison was lower or same (unsigned), then loads 0 into d5, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_bls_moveq_bra`: Short code fragment that compares d1 with d0, then branches if the last result was not zero, then copies d6 to d0, then masks d0 with 65535, then copies the memory at an offset from a3 to d1, then compares d1 with d0, then branches if the comparison was lower or same (unsigned), then loads 0 into d5, then jumps to another point in the code.
- `frag_moveq_bra_17`: Short code fragment that loads 0 into d5, then jumps to another point in the code.
- `compute_offset`: Calculates a difference between two values and uses the result as an offset to call a routine, storing the return value back into a register before branching to a fixed address.
- `check_kickstart_version`: Verifies the Kickstart version and manages cache settings, storing the result in a global variable while handling patch list processing.
- `frag_move_bra_3`: Short code fragment that copies d6 to d4, then jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies d6 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies d6 to the memory at an offset from a5, then jumps to another point in the code.
- `patch_kickstart`: Checks and patches the Kickstart version, manages cache settings, and handles patch list processing.
- `check_vector_then_copy_stub`: Calls a library vector and, when the result is nonzero and bit 8 of a flag byte in the global structure is set, allocates a memory block, copies a 50-word stub routine into it, and returns one.
- `copy_stub_if_flagged`: If bit 8 of a flag byte in the global structure is set, allocates a memory block, copies a 50-word stub routine into it, runs library calls on it, and returns one.
- `allocate_copy_stub_and_install`: Copies a 51-word code stub into an allocated memory block and then invokes several library routines to install it.
- `probe_memory_size_and_mask`: Verifies a fixed low-memory byte holds an expected value, then probes memory downward from a fixed high address to locate where a self-referencing pointer test stops matching, and finally stores a bit mask derived from that result into a fixed-address byte.
- `free_patch_slots_clear_fields`: Clears a pointer field in a record and releases it through a library routine at offset minus 210 when it was set, then walks six consecutive pointer slots freeing each non-zero one while clearing a companion field, and returns 1.
- `dispatch_message_and_reply_alt`: Performs the same message handling as its neighbouring routine, entered one instruction later, and otherwise passes control to a shared routine.
- `reply_message_when_type_four`: If the message's byte value is 4, copies stored values into the message, clears flag bytes, and replies through the ReplyMsg library vector.
- `branch_on_empty_message`: Checks if a message field is zero and, if so, copies data, clears a flag, and calls a library routine.
- `call_two_routines_return_zero`: Calls two routines in sequence with no arguments and returns zero.
- `relink_flagged_list_entry`: Looks up an entry through the library routine at offset minus 276 and, when its top flag bit is set, unlinks it, clears the flag, and relinks it between two library calls that disable and enable the system, returning the entry it found.
- `call_one_routine_only`: A two-instruction routine that makes a single call to another routine and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_single_routine_and_return`: Calls one routine with no arguments and returns.
- `store_library_result_globals`: Calls a library vector and, when the result is nonzero, stores the result, a field read from it, and the base pointer into three globals.
- `range_check_and_dispatch`: If the argument lies within a stored range, calls a library vector and, if it returns nonzero, calls a routine and another vector.
- `jump_through_saved_handler`: Tail-jumps through a saved routine pointer with no other work.
- `check_upper_bound_and_dispatch`: If the argument is at or below a stored upper bound, calls a library vector and, if it returns nonzero, calls a routine and another vector.
- `store_offset_adjusted_lookup`: Selects one of two constant offsets depending on whether a structure field at offset 22 in a1 equals 44, then calls the library routine at a fixed offset from a6 and, if it returns a pointer, stores the chosen offset plus that pointer into a global variable.
- `patch_table_and_branch`: checks if a table entry matches a certain routine and, if so, overwrites two adjacent table entries with a constant before branching to another routine.
- `execute_stored_routine`: executes a routine stored at a specific memory location.
- `run_vectors_and_set_flag`: Calls three library vectors in sequence, sets a global flag word to 64, and returns one.
- `store_argument_and_return_one`: Stores the register argument in a global word and returns one.
- `update_table_and_fill_memory`: compares a value from another routine against a table, updates the table if they differ, and fills a memory block with a pattern if the value matches a specific constant.
- `add_library_result_twice`: Calls a library vector and, when the result is nonzero, adds that result to a stored word twice.
- `copy_field_on_matching_event`: If the stack argument equals 96994 and the second argument equals 21264, and bit 3 or bit 4 of a status field is set, copies a longword field to another field.
- `copy_field_on_event_code`: If the second argument equals 21264 and bit 3 or bit 4 of a status field is set, copies a longword field to another field.
- `clear_flag_bit_then_call`: Takes a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `search_linked_list_node`: Calls a library routine, then traverses a linked list of nodes, checking a field in each node, before calling another library routine and returning a status.
- `traverse_list_with_checks`: executes a routine, then traverses a linked list of nodes checking a certain field before executing another routine.
- `traverse_list_and_execute`: executes a routine, then iterates through a linked list of structures until a certain field matches a value, and executes another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `traverse_list_and_return_constant`: executes a routine, traverses a linked list, and returns a constant value if a certain condition is met.
- `traverse_list_with_error_return`: checks if a register contains a specific value, executes a routine, and then traverses a linked list, returning a specific error value or the register value.
- `execute_and_traverse_list`: Executes a routine via a pointer, traverses a linked list of structures while checking a specific field for a termination value, and then executes a second routine.
- `search_node_list_or_param`: Searches a linked list of nodes for one whose 16-bit field matches the argument, while multitasking is suspended through library vectors.
- `release_lock_and_return_value`: Calls a library vector that resumes multitasking, then returns the stack argument when the pointer argument is nonzero and the third parameter otherwise.
- `return_parameter`: Returns the input parameter unchanged.
- `search_word_list_store_pointer`: Chooses a pointer slot from a fixed low address, or from a library vector when bit 0 of a flag byte in the global structure is set.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `branch_on_marker_word`: checks if a word at a certain address matches a specific value and, if not, executes a routine via a pointer.
- `forward_to_next_routine`: Calls one further routine and returns with no result, adding no other work.
- `allocate_vec_and_copy_string`: Measures a null-terminated argument string, allocates a buffer with exec.AllocVec, and copies the string into the buffer while recording its length in the buffer.
- `insert_length_prefixed_string`: Measures a null-terminated string, moves the existing buffer contents to make room, copies the bytes backward into the buffer and writes the length as a prefix.
- `shift_bytes_and_store_length`: Moves a block of bytes one position backward for the count held in a register, stores a fill byte before the block, terminates the string at its length plus one, and calls a library vector.
- `return_void`: this routine is empty and performs no operations before returning.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `decrement_count_and_pop_node`: Increments a counter byte in the global structure and decrements a count in the entry.
- `alert_or_decrement_count`: If the condition flag is negative, calls an alert library routine and a second library routine.
- `alert_and_init_semaphore`: Calls the alert library routine and initializes a semaphore through a library vector, then calls another library vector and returns.
- `process_flagged_list_nodes`: Removes nodes from an entry's list, calling a library vector for each node whose flag bit is set and relinking the nodes as it goes.
- `process_flagged_list_nodes_2`: Increments a counter, calls a library routine, and removes nodes from a list if a specific bit is set, relinking the remaining nodes.
- `clear_status_bit_and_release`: Increments a counter byte in the global structure and a count field of the entry.
- `clear_status_bit_and_release_2`: If a pointer field in the entry is set, clears a status bit, calls two library vectors and resumes multitasking.
- `check_stack_bounds_and_call_2`: Calls a stored routine when the stack address falls outside a range recorded in the global structure or when a stored flag equals minus one.
- `call_stack_check_handler`: Performs the same stack-bounds and status-word test as its neighbours.
- `store_two_values_return_one`: Stores the address-register argument and the base pointer into two globals and returns one.
- `call_single_routine_and_return_2`: Calls one other routine and returns.
- `call_single_routine_and_return_3`: Calls one other routine after a short sequence of instructions and returns.

### Version

- `parse_current_directory_string`: Retrieves the current directory path and parses it into a list of tokens, handling quoted strings and converting certain escaped characters into newlines or escape codes.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `call_routine_with_fixed_argument`: Calls the first routine with the constant 128 and then calls the second routine with its second argument.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
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

- `find_max_and_prepare_buffer_7`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_loop`: Copies a fixed number of words from a source pointer to a destination pointer.

### d

- `stub_bad_instruction_data`: Stub made of data rather than code: the decompiled output reports bad instruction data, so no routine behaviour is recorded.

### gb

- `halt_on_bad_instruction`: The routine contains invalid or bad instruction data.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:InstallWingCommander

- `open_dos_and_setup_installer`: Program entry that opens dos.library, duplicates a lock on the current directory, allocates working memory and waits for messages through library calls.
- `release_install_task_resources`: Saves its incoming value in a per-task record, then conditionally calls cleanup routines and finally closes a library.
- `run_delayed_call_sequence`: Checks a stack-limit field against the current stack address, then runs a long sequence of calls including a dos.Delay wait, and returns an integer status.
- `query_volume_info_with_lock`: Locks a path, queries disk information through the dos.Info library call and releases the lock, using a memory block obtained and freed through exec library calls.
- `scan_fixed_size_entry_table`: Walks a table of 16-byte entries referenced by the global structure, testing a byte at offset 14 of each entry, and makes one call with the result.
- `process_named_entry_with_checks`: Checks a stack-limit field, then processes a named entry through five routine calls and returns an integer status.
- `copy_file_with_protection`: Opens a file through dos calls, examines and reads its contents into memory obtained from exec, writes the data to a destination file and sets its protection bits.
- `process_path_string_with_calls`: Processes a path string argument through six routine calls and returns a status value.
- `create_directory_path`: Walks a path string, examining each component with dos.Examine and creating missing directories with dos.CreateDir, using locks and temporary memory obtained from exec.
- `return_immediately`: The routine executes a return instruction immediately upon entry, performing no other operations despite the instruction count suggesting additional code may be present or filtered.
- `return_immediately_2`: The routine executes a return instruction immediately upon entry, performing no other operations.
- `free_install_memory_lists`: Frees two linked lists of memory blocks by calling the library free routine at offset minus 210 on each node, and clears the head of the first list.
- `split_path_into_components`: Splits a slash-separated path into components by writing a zero byte after each separator and recording where each component starts, for up to 16 components.
- `split_path_into_parts`: Separates a path into a volume part before a colon, a directory part before the last slash, and a file-name part, copying each into the output buffers that were supplied.
- `append_separator_if_missing`: Appends the separator byte held in a global variable to a string when the string is non-empty and does not already end in a slash, colon or backslash.
- `copy_string_return_end`: Copies bytes from a source string to a destination until a zero byte is copied, and returns the destination pointer at that zero byte.
- `convert_integer_to_string`: The routine repeatedly calls a fixed-point division routine to extract digits, converting them to ASCII characters.
- `convert_binary_to_string`: The routine converts a binary integer into a string representation by repeatedly extracting bits and appending characters to a buffer, then copies the result to an output pointer and returns the length.
- `convert_hex_to_string`: The routine converts a hexadecimal integer into a null-terminated ASCII string using a lookup table.
- `parse_signed_integer`: The routine reads a signed decimal integer from a zero-terminated string pointed to by a register, handling an optional leading plus or minus sign.
- `allocate_file_handle_record`: Checks a flag bit in a file record and, when it is clear, calls a routine to obtain a buffer and sets the record's fields from a global value.
- `convert_string_to_uppercase`: Converts each character of a zero-terminated string to upper case when that character's entry in a global attribute table has bit 1 set.
- `write_to_file_record`: Writes data to a file record after checking the record's flags.
- `check_handle_and_set_error`: Calls two routines to look up a file handle.
- `read_and_strip_newlines`: Reads data from a source, strips carriage return characters, and returns the length of the processed data.
- `seek_file_record`: Seeks within a file record to the position given by the mode parameter, storing the DOS error code when the seek fails.
- `close_file_record`: Closes a file record by calling the DOS close routine when it is open, releasing its buffers when they are present, and clearing its handle.
- `open_file_record`: Adds a file record to a linked list, copies the file name into it and translates the access-mode flags into record flags.
- `normalize_fixed_point`: The routine takes two 32-bit inputs and normalizes a fixed-point value by scaling the numerator with a variable shift derived from the denominator’s magnitude, performing a division with remainder and a borrow correction, then returns a 64-bit result composed of a signed quotient and a scaled remainder.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `return_empty_subroutine`: This routine is empty and performs no operations before returning.
- `close_all_file_records`: Walks the list of open file records and closes each one that needs it, calling the DOS close routine or a stored callback depending on the record's flags.
- `parse_command_line_open_streams`: Splits the command-line text into up to 32 argument strings, honouring quotes and star escapes, then sets up the input and output streams, falling back to the NIL: device.
- `copy_string_up_to_79`: Copies a string of at most 79 characters from a structure field into a local 81-byte buffer and returns the length copied.
- `process_break_signal`: Copies a string into a stack buffer, checks for a specific task or signal, and updates a global variable based on the result of a library call.
- `find_file_record_by_index`: Returns the file record at the given index in the linked list, walking the list from its head.
- `poll_break_signal_bits`: Reads the task's signal state through the SetSignal library vector and keeps the bits selected by the mask 12288.
- `find_byte_in_buffer`: Searches a buffer of a given length for a specific byte value, returning a pointer to its first occurrence or null when the buffer does not contain it.
- `append_string_to_buffer`: Copies a counted number of bytes from a source string to the end of a destination string, then returns.
- `concatenate_zero_terminated_strings`: Finds the terminating zero byte of the first string, copies the second string including its terminator after it, and returns the first string's pointer.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `copy_byte_block_safe`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the source address is higher than the destination.
- `release_matching_memory_block`: Returns immediately for a null pointer.
- `release_matching_memory_block_2`: Performs the same record search as its neighbouring routine, releasing the matching block through the FreeMem library vector.
- `start_block_allocation_or_fail`: Records error code 22 and returns null when its size argument is zero.
- `allocate_memory_block`: Searches a linked list of memory pools for a free block of a specified size and splits the block if it is larger than requested.
- `run_callbacks_and_scan_table`: Calls each routine stored in a linked list of callbacks in the global structure, then walks a fixed table of records testing their flag bits.
- `append_byte_to_buffer`: Increments a counter in the buffer header, stores the byte at the next position, and when the count exceeds 511 calls one routine and clears a second field.
- `call_and_return_flag_status`: Calls a routine with the incoming parameter and a stack slot, then returns minus one when a flag field in a global structure has bit 4 or bit 5 set, and zero otherwise.
- `read_line_into_buffer`: Reads characters into the buffer until a newline or the end of input, then terminates the buffer with a zero byte.
- `read_next_input_byte`: Returns the next byte from an input stream, refilling its buffer through helper calls when it is empty and recording end-of-file or error flags.
- `check_entry_flags_or_error`: Returns minus one and records error code 9 when a field of the argument structure is zero.
- `calculate_file_offset`: Calculates a file offset by counting newline characters in a buffer and adjusting the result based on specific flags and a length field.
- `reset_structure_fields`: Clears several fields within a structure, performs bitwise operations on a field based on a flag, and updates a value based on the result of another routine.
- `check_entry_flags_for_name`: Returns zero when either count argument is zero.
- `copy_data_to_buffer`: Copies characters from a source to a destination buffer while checking structure flags and handling newlines.
- `copy_string_data`: Copies a sequence of bytes from a source buffer to a destination buffer, updating a counter and checking status flags in a structure.
- `update_file_node_state`: With no node argument, walks the linked list of nodes from a fixed head and records an error when a helper called on an active node reports failure.
- `close_stream_node`: Returns minus one if the node is missing or has no state set.
- `validate_structure_state`: Checks if a pointer or a value at offset 24 of a structure is zero, and if certain flags or pointers are present, calls other routines to clear or update structure fields and return a status code.
- `open_stream_from_mode_string`: Parses a two-character access mode made of read, write, append and optional plus signs into flag bits, then calls another routine to open the file.
- `find_or_allocate_free_node`: Searches the linked list of nodes from a fixed head for one whose active field is zero, allocating a new node through a helper when none is free.
- `parse_format_string_and_print`: Parses a format string to identify conversion specifiers, flags, and width/precision arguments, then converts corresponding integer or pointer values from a parameter list into ASCII strings using helper routines.
- `frag_movea_move`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to the memory at an offset from a7 (2 times in a row).
- `parse_format_string_print`: The routine parses a format string to identify conversion specifiers and flags.
- `delete_file_save_error_code`: Calls the DOS library to delete a file.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `forward_call_to_helper`: Calls one other routine and then returns, passing no data through and changing nothing itself.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:Wing

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one_2`: Decrements a 16-bit counter in a loop until it passes zero and reaches minus one, then returns.
- `decode_audio_data`: Decodes audio bitstream data into a destination buffer using a helper routine for bit extraction, with loop control based on a counter and conditional execution paths.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `decode_bitstream`: Reads bits from a source buffer and writes decoded bytes to a destination, handling bit extraction and assembly with carry propagation through multiple iterations.
- `frag_negx_ori_move_movem_bra`: Short code fragment that negates d0 with the extend flag, then merges 234 into d0, then copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `walk_length_prefixed_entries`: Walks a table of variable-length entries, advancing by the size stored at each entry's third byte plus three, and calls two library routines per entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory by writing zero values through a pointer, decrementing a counter until it reaches negative one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields`: Initializes several fields in a structure including a counter and status byte, then performs conditional operations based on memory offsets and updates pointers.
- `decrement_until_minus_one_3_2`: Decrements a local variable until it reaches negative one and then returns.
