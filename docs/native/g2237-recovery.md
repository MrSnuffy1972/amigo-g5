# Game 2237 (wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02237). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901308 top-level bytes in 66 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 30 of 30 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 382 of 382 named (0 author symbols), 382 of 382 described (100 by rule, 91 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1830 of 1830 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 207860 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10509 of 10510 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16316 |
| code | 30970 |
| data | 836381 |
| padding | 14057 |
| unused | 3584 |

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
| d | 1 | 3 | 1 |
| gb | 1 | 3 | 1 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:InstallWingCommander | 2 | 3738 | 68 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing | 8 | 252 | 6 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:bootblock | 1 | 126 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 2 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP03.PKT#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP01.PKT#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PLANET0.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (7369 -> 10282 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:ENDGAME.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (41104 -> 84654 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:FONTS.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (5401 -> 10306 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP03.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (60752 -> 102086 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP01.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (66893 -> 105774 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME2.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (36350 -> 63356 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:DEMO.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (65170 -> 111126 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME0.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (43516 -> 72732 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP02.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (45898 -> 78286 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:DEMOSAMP.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (91572 -> 106888 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME3.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (23773 -> 42588 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP00.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (59027 -> 94082 bytes)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:MIDGAME1.PKT: (NJMP) Data Cruncher (xfd)-packed, unpacked with ancient (31778 -> 59528 bytes)
- 3 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 9 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 86 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/isdosdev: Imploder (xfd) stream found by ancient scan (4544 -> 6152 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/multidie: Imploder (xfd) stream found by ancient scan (5624 -> 8612 bytes) but only 0 of 4 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/wipeout: Imploder (xfd) stream found by ancient scan (4704 -> 6404 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/remove: Imploder (xfd) stream found by ancient scan (4760 -> 6580 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/memavail: Imploder (xfd) stream found by ancient scan (4436 -> 5956 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing: Imploder (xfd) stream found by ancient scan (139392 -> 278468 bytes) but only 0 of 124 sampled windows are in reference RAM: not used
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP03.PKT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:PCSHIP01.PKT#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate, 1 immediate+displacement
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:c/multidie: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- wing commander - platinum edition (1992)(mindscape)(disk 1 of 3)[h lsd][a]/Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 207860 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Setmap

- `main_parse_cli_workbench_args`: Program entry point.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `invoke_optional_handle_routines`: Calls a routine with each of several global handle values when they are set, passing one global by reference, then always calls it with a fixed handle and the incoming argument.
- `store_arguments_and_call_helpers`: Stores its two arguments into two global variables, then calls one helper with a fixed set of arguments that includes a global address and constants.
- `resolve_keymap_name_and_open`: Determines a keymap name from icon tool types, command-line arguments, or a default value, then converts the string to lowercase.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `forward_global_param_slot`: Passes a global value, its argument and the address of an eight-byte stack slot to another routine and returns.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If a destination pointer is non-null, copies sixteen longwords from a fixed block into it, then calls a function pointer held in a register.

### isdosdev

- `run_hdinstall_entry`: Program entry for the HDinstall tool.
- `busy_wait_countdown_loop`: Counts a local value down in a loop until it passes below zero, a plain busy-wait delay, then returns.
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

- `start_setpatch_cli_or_workbench`: Program entry point for the patch command.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `self_calling_guard`: The routine calls another routine unconditionally, then checks a global variable and, if non-zero, calls another routine with the input parameter, passes the result to another routine, and finally calls another routine unconditionally.
- `install_setpatch_handler`: Installs a SetPatch handler and performs a series of conditional patches and library initialisations.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_via_base`: Calls one library function through the library base held in a global, at a fixed negative offset, then returns.
- `execute_indirect_function_2`: Fetches a function pointer from a location relative to a global variable and executes it.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_function_3`: Loads a function pointer from a location within a global data structure and jumps to it.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Fetches a function pointer from a global table and executes it.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if conditions are met, obtains a memory block and a code pointer via library calls, copies a small code stub from another routine into the target, updates a pointer in the global structure, computes a checksum over a short array in the structure and stores its complement, then returns.
- `store_word_checksum_then_call`: Sums twenty-three words from a record, stores the one's complement of the sum in a field, then calls through an address the decompiler could not resolve.
- `disable_interrupts_and_patch`: The routine checks a global variable and calls another routine through a table.
- `call_hook_and_record_owner`: Calls a hook through a stored pointer and, when it returns nonzero, records the current task pointer in a shared slot.
- `call_hook_if_owner_matches`: Returns when a recorded owner differs from the current task and is not minus one.
- `call_hook_if_minus_one`: Returns unless its argument is minus one, in which case it calls a stored hook.
- `install_once_guarded_handler_patch`: Writes the interrupt-enable mask to disable interrupts, then, on the first call when a four-letter signature is absent from low memory, stores the signature and a flagged entry address and calls a handler through a table.
- `call_handler_table_entry_b`: Calls one entry of a handler table held in a global, then returns.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `return_register_value_unchanged_2`: Returns its register input unchanged.
- `call_fourteen_helpers_return_last`: Calls a series of fifteen subroutines in a row and returns the result of the last one.
- `return_register_value_unchanged`: Returns its register input unchanged.
- `set_function_if_slot_range`: Returns when a slot number is above a limit.
- `clone_record_or_free`: Allocates a copy of a record through a library call and copies its slot number and its table of value pairs into the new block.
- `release_counted_entries_and_return`: Calls a library routine once for each remaining count, then returns the value at its pointer with the top bit set.
- `release_entry_return_flagged_value`: Makes one library call and returns the value at its pointer with the top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `call_patch_hook_with_stack_scratch`: Saves registers, builds a scratch stack pointer below the current stack, loads a function pointer from the patch base plus offset and calls it with d0 set to twelve, then returns d1.
- `call_check_then_two_followups`: Calls a check routine with a field from a record.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_words_high_byte_marker`: Advances a word pointer until it reaches a word whose high byte matches a fixed marker value, then returns that pointer.
- `advance_past_variable_length_entry`: Works out the length of a variable-length entry from its first byte, either a one-byte signed displacement or a two-byte one when the low byte is zero, and returns a pointer to the entry that follows.
- `compute_word_offset`: Reads a 16-bit value from two bytes after an input pointer.
- `patch_global_pointers`: The routine sequentially calls a helper routine to compute offsets and stores the returned pointers into a series of global variables.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch_function`: Calls the exec dispatch function through the library base held in a global, then returns.
- `forward_two_params_stack_slot`: Passes its two parameters to another routine together with the address of a fixed stack slot, then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_pointer`: Stores the low byte of its data register at the address held in a pointer register, then returns.

### addbuffers

- `find_max_and_prepare_buffer_2`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_and_call_routine`: Copies a block of data from a table to a destination pointer and then calls another routine.

### multidie

- `reset_install_state_at_entry_2`: Program entry for Ambermoon_install that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_until_minus_one_2`: Decrements a local variable or register value until it reaches negative one and then returns.
- `initialize_and_process_2`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic.
- `initialize_and_process_5`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic before checking a condition.
- `frag_beq_movea_bsr_bra_2`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_2`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_8`: Processes bit pattern data from a source buffer, performing bitwise operations and conditional jumps based on carry flags, then updates a global variable with decoded bit values.
- `shift_and_accumulate`: Processes bits from a buffer by shifting and adding values, updating registers through conditional operations, and loops four times before incrementing a counter and branching to another routine.
- `frag_moveq_bra_4`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_3`: Processes a bit pattern from memory, performing bitwise operations and conditional jumps based on carry flags, then loads a value from a table using an index.
- `frag_add_bne_move_addx_bra_2`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_bitstream_and_expand_pixels`: Reads a bitstream from a source buffer in a0, using bit-shifting and carry-based logic to reconstruct values, then uses these values to index into a table and copy pixel data to a destination buffer.
- `decode_bit_pattern_12`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
- `frag_move_movem_bra_2`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### wipeout

- `reset_install_state_at_entry_3`: Program entry that calls routines and a library function in a loop from 5 down to 2, then clears a word at a fixed location.
- `decrement_register_to_negative_one`: Decrements a 16-bit register until its value reaches negative one and then returns.
- `initialize_and_process_3`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic.
- `process_loop_iterations`: Processes up to four iterations of a loop using pointer arithmetic to update values in memory, with conditional branching based on counter and comparison results.
- `frag_beq_movea_bsr_bra_3`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_3`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_9`: Processes bit pattern data by decoding bits from a source buffer and copying decoded bytes to a destination buffer while managing counters and flags.
- `shift_and_accumulate_2`: Processes bits from a buffer by shifting and adding values, updating accumulators through conditional operations, and repeats this process a specific number of times before returning.
- `frag_moveq_bra_7`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_8`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_9`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_4`: Processes bit patterns from a buffer, performing bitwise operations and conditional jumps based on carry flags, while updating counters and pointers.
- `frag_add_bne_move_addx_bra_3`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_compressed_pixel_data`: Decodes a bitstream by shifting bits from a source buffer in a0 into a register, using the resulting values to index into a table and perform block memory copies from a source buffer to a destination buffer.
- `decode_bit_pattern_13`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
- `frag_move_movem_bra_3`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### remove

- `reset_install_state_at_entry_4`: Performs a series of delay loops, then checks if a specific memory word matches a constant.
- `decrement_word_until_minus_one`: Decrements a 16-bit value stored in a register until it reaches negative one.
- `initialize_and_process_4`: Initializes a counter and processes a loop of up to four iterations, updating values through pointer arithmetic and calling a routine with varying parameters.
- `process_loop_iteration_2`: Processes a loop of up to four iterations, updating values through pointer arithmetic before checking a condition.
- `frag_beq_movea_bsr_bra_4`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_4`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_10`: Processes bit pattern data from a source buffer, decoding bits using carry flags and storing results while managing counters and pointers.
- `shift_and_accumulate_3`: Processes bits from a buffer by shifting and adding values, updating registers through a loop that decrements a counter until it reaches zero.
- `frag_moveq_bra_10`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_11`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_12`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_5`: Processes bit patterns from a buffer, performing shifts and additions based on carry flags, then updates a global variable with decoded values.
- `frag_add_bne_move_addx_bra_4`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_compressed_pixel_data_2`: Decodes a bitstream from a source buffer in a0 to reconstruct pixel values, using a variable-length bit-shifting mechanism and a lookup table to determine block sizes and offsets.
- `decode_bit_pattern_14`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
- `frag_move_movem_bra_4`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### EndCli

- `find_max_and_prepare_buffer_3`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_to_stack`: Copies a sequence of words from a memory location to a stack if the stack pointer is not null.
- `update_counter_and_branch`: Updates counters and performs conditional operations based on comparison results, then calls a handler routine with specific parameters and jumps to another location.

### Echo

- `find_max_and_prepare_buffer_4`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data`: Copies a block of sixteen bytes from a memory location to the stack and then calls another routine.
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
- `find_and_copy_fastmem_block`: Walks a linked list of memory blocks starting from a global variable, storing the first block whose type tag at offset 14 equals 5 and whose value is between an address and an address in d2, and the next block after the first whose type tag equals 3 and whose value is below an address in d3, then returns.
- `call_library_base_e`: Calls one library function through the library base held in a global, then returns.
- `execute_indirect_function`: Calls a function through a pointer located at a specific offset within a global variable.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### CD

- `find_max_and_prepare_buffer_5`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_stack_data_2`: Transfers sixteen bytes from a source address to the stack before executing a subroutine.
- `update_data_entries`: Processes data entries from a table using indexed access, performs conditional operations and updates memory locations based on the results.
- `process_data_entries`: Processes data entries from a table using indexed access, performs conditional operations and updates memory locations based on the results of those checks.
- `finalize_operations`: Completes final operations by calling handler routines with specific parameters, updating memory locations, and performing conditional checks before jumping to another location.

### mouse

- `wait_fire_port_bit_clear`: Busy-waits while bit six of the joystick port register is set, then returns zero.

### EndIf

- `return_zero_stub`: Returns zero and does nothing else.

### If

- `find_max_and_prepare_buffer_6`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen words from a source address to a destination address pointed to by a stack value.
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

- `run_hdinstall_entry_2`: Program entry for the HDinstall tool.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `update_memory_cache`: Increments a counter and performs several operations including adding values to registers, calling functions, and updating memory locations.
- `process_loop_with_accumulation`: Processes a loop that accumulates values into an array through pointer arithmetic while updating counters and performing conditional branches based on comparison results.
- `frag_beq_movea_bsr_bra_5`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_5`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_6`: Processes bit pattern data from a buffer, performing bitwise operations and conditional jumps based on carry flags, then updates a global variable with decoded bit values.
- `initialize_loop_counter`: Sets a loop counter to two and performs a series of bit operations on data from a buffer, updating global variables based on carry flags before exiting.
- `frag_moveq_bra_13`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_14`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_15`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern`: Processes a bit pattern from memory, performing bitwise operations and conditional jumps based on carry flags, then updates a global variable with decoded values.
- `frag_add_bne_move_addx_bra_5`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_compressed_pixel_stream`: Decodes a bitstream using a variable-length prefix system to extract values, which are then used as offsets into a table to retrieve data for copying into a destination buffer.
- `decode_bit_pattern_11`: Processes bit pattern data by reading bytes from a source buffer, performing bitwise operations with carry flags, and storing results into a destination buffer.
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
- `allocate_and_copy_patch_block`: Checks a library result and a system flag.
- `allocate_copy_patch_block_b`: Returns zero unless a flag is set.
- `allocate_copy_stub_and_install`: Copies a 51-word code stub into an allocated memory block and then invokes several library routines to install it.
- `probe_memory_size_and_mask`: Checks a low-memory byte and, if it is not a fixed value, probes memory downward from a fixed high address to find where the self-addressing test stops matching, then updates a fixed-address byte with a bit mask.
- `free_patch_slots_clear_fields`: Clears a pointer field in a record and releases it through a library routine at offset minus 210 when it was set, then walks six consecutive pointer slots freeing each non-zero one while clearing a companion field, and returns 1.
- `finish_request_or_forward_handler`: Repeats the same state check and completion steps as its neighbour, clearing the flag and calling the library vector on a match.
- `reply_message_or_forward_handler`: Runs the completion steps directly when a status byte matches, copying fields, clearing flags and calling a library vector that replies to the message.
- `finish_unflagged_request_or_forward`: When a flag word at a fixed offset is zero, it copies a stored field, clears a flag byte and calls a library vector.
- `call_cia_routine_twice`: Calls the same unnamed routine twice in a row and returns zero.
- `relink_flagged_list_entry`: Looks up an entry through the library routine at offset minus 276 and, when its top flag bit is set, unlinks it, clears the flag, and relinks it between two library calls that disable and enable the system, returning the entry it found.
- `forward_to_unnamed_routine`: Two-instruction wrapper that transfers control to one other routine with no other visible effect.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_through_fixed_vector`: Five-instruction relay that calls a single jump-table entry, which passes control straight on to a fixed address elsewhere in memory.
- `cache_console_device_handle`: Calls a library routine and, when it returns a non-null pointer, saves that pointer, a value read from its sixth longword, and the caller's base pointer into three global slots.
- `dispatch_in_range_or_forward`: Compares a value with lower and upper bounds held in globals.
- `pass_through_saved_vector`: Jumps straight through a saved vector pointer, handing control to the previously installed routine without changing any arguments.
- `dispatch_below_limit_or_forward`: Compares the data register with an upper bound stored in a global.
- `store_offset_adjusted_lookup`: Selects one of two constant offsets depending on whether a structure field at offset 22 in a1 equals 44, then calls the library routine at a fixed offset from a6 and, if it returns a pointer, stores the chosen offset plus that pointer into a global variable.
- `clear_slot_then_forward_nested`: Performs the same indexed table check, and on a match clears the two following fields.
- `jump_through_saved_vector_again`: Two-instruction routine that jumps through the saved original vector pointer, passing control to the earlier handler unchanged.
- `sequence_intena_calls_return_one`: Calls three library routines in sequence and stores the constant 64 in a global between two of those calls.
- `save_argument_into_global_slot`: Copies its input register into a global variable and returns one.
- `update_table_and_fill_memory`: Calls another routine, updates a table with the returned value, and if the value matches a specific constant and a related field is non-zero, fills a memory block with a repeating pattern using a loop.
- `add_library_result_twice`: Calls a library routine and, when it returns a non-zero value, adds that value into one global slot twice.
- `update_on_matching_message_code`: Checks a stack argument and the second argument against two fixed codes, and when both match and a flags word has bit three or four set, copies one stored field into another.
- `copy_field_on_code_match`: When the second argument equals a fixed code and a flags word has bit three or four set, copies one stored field into another.
- `clear_flag_bit_then_call`: Takes a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `search_linked_list`: Traverses a linked list to find a node whose halfword matches a key, using library calls to manage the search process.
- `search_list_with_two_calls`: Makes two library calls around a scan of a linked list of nodes, matching each node's halfword against the key in the data register.
- `traverse_linked_list_with_hooks`: Executes a hook routine, traverses a linked list to find a node matching a specific halfword value, and executes a second hook routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `search_list_returning_two`: Runs a linked-list scan with a library call before and after it, matching the data-register key against a stored halfword.
- `traverse_list_with_hooks`: Executes a hook routine, traverses a linked list to find a node matching a specific halfword value, and executes a second hook routine.
- `search_list_with_two_hooks`: Runs a linked-list scan for a halfword matching the data-register key, bracketed by two unnamed routine calls.
- `search_list_return_argument_default`: Calls a library routine, scans a linked list for a node whose halfword matches the data-register key, and calls a second library routine.
- `return_stack_value_if_present`: Calls a library routine and returns the first stacked argument when its result is non-null, otherwise returns the third argument.
- `return_parameter`: Returns the input parameter unchanged.
- `locate_word_entry_in_table`: Picks a base pointer from a fixed offset or a library call, depending on a flag bit, then scans forward through halfwords for a stored value and saves that position.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `check_patched_branch_then_forward`: Performs the same opcode check on a stored target.
- `relay_to_unnamed_routine`: Calls one unnamed routine and returns.
- `convert_c_string_to_bcpl`: Measures a null-terminated C string and copies it into a newly allocated buffer as a length-prefixed string, with a zero after the last character.
- `write_bcpl_string_to_buffer`: Measures a null-terminated C string and writes it into a caller-supplied buffer as a length-prefixed string, placing the length byte first.
- `copy_bytes_backwards_terminated`: Copies a run of bytes backwards from the end of one buffer into another, stores a byte passed in a data register in front of the copy, and writes a zero terminator.
- `stub_immediate_return`: A three-instruction routine that performs no work and returns straight away to its single unnamed caller.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `drain_flagged_list_nodes`: Increments a global activity counter and decrements a per-structure count.
- `branch_on_sign_decrement_counter`: When the sign flag is set it calls two library routines; otherwise it decrements a 16-bit counter in the caller's structure.
- `call_alert_and_init_semaphore`: Calls two library routines that the facts identify as exec.Alert and exec.InitSemaphore, then a third library routine, and returns.
- `unlink_first_flagged_node`: Removes the first node from a linked list in the caller's structure.
- `walk_list_until_terminator`: Walks a linked list from the head pointer in the caller's structure, calling a library routine on each node and relinking as it goes.
- `advance_count_clear_status_bit`: Increments a per-structure count and, once it is non-zero and a pointer is set, clears bit four of a status byte and calls two library routines.
- `clear_status_bit_or_count`: When a pointer in the caller's structure is set, it clears bit four of a status byte, calls two library routines and tail-calls a third.
- `call_stack_check_handler`: Performs the same stack-bounds and status-word test as its neighbours.
- `call_stack_check_handler_2`: Performs the same stack-bounds and status-word test as its neighbours.
- `save_a1_and_base_globals`: Stores the A1 argument and the A6 base register into two global slots and returns one.
- `relay_call_to_neighbour`: Three-instruction routine that transfers control to one other unnamed routine.
- `call_unnamed_routine_after_helper`: Eleven-instruction routine that calls a two-instruction helper and one unnamed routine.

### Version

- `startup_allocate_and_open_dos`: Program entry that finds the current task, allocates a memory block and opens dos.library, returning an error code on failure.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses command-line arguments to open a specified library or device and validates its version and revision numbers against provided thresholds.
- `call_library_then_unnamed_routine`: Calls a library routine with the first argument and a fixed flag value of 128, then passes the second argument to one unnamed routine.
- `parse_integer_string`: Parses a string to convert it into an integer, handling optional sign characters and digit sequences.
- `compare_strings`: Compares two null-terminated strings byte by byte and returns a value indicating their lexicographical order or equality.
- `measure_null_terminated_length`: Walks a null-terminated string and returns the number of characters before the terminator.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_exec_dispatch_vector`: Calls the exec.Dispatch routine through the library base stored in a global, with no arguments set up here.
- `forward_with_stack_pointer_argument`: Passes its two arguments to one unnamed routine together with the address of a stack location, then returns.
- `call_exec_vector_198`: Calls the exec library routine at offset 198 from the exec base held in a global, and returns its result.
- `call_exec_vector_210`: Calls the exec library routine at offset 210 from the exec base held in a global, and returns its result.
- `call_exec_vector_414`: Calls the exec library routine at offset 414 from the exec base held in a global, and returns its result.
- `call_exec_vector_444`: Calls the exec library routine at offset 444 from the exec base held in a global, and returns its result.
- `call_exec_vector_450`: Calls the exec library routine at offset 450 from the exec base held in a global, and returns its result.
- `call_exec_vector_552`: Calls the exec library routine at offset 552 from the exec base held in a global, and returns its result.
- `call_global_and_checksum_buffer_3`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_at_output_pointer`: Stores the low byte of the data register at the location given by the address register and returns.

### d

- `apply_bitwise_and_mask`: Performs a bitwise AND operation between a byte and the value pointed to by a register, storing the result back at that memory location.

### gb

- `null_entry_point`: This routine is empty and contains no executable instructions.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:InstallWingCommander

- `startup_clear_state_open_dos`: Program entry for the installer that zeroes a large global state block and works out a stack-based layout.
- `release_install_task_resources`: Saves its incoming value in a per-task record, then conditionally calls cleanup routines and finally closes a library.
- `walk_entry_chain_with_delay`: Checks remaining stack space before walking a chain of entries, calling a set of helper routines on each one.
- `query_volume_info_with_lock`: Locks a path, queries volume information with the dos library and unlocks it, with memory allocated and freed around the call.
- `find_table_entry_by_id`: Scans a table of sixteen-byte entries and returns the first entry whose stored identifier matches the argument, or zero when none does.
- `process_typed_table_entries`: Walks a table of sixteen-byte entries and, for entries whose type byte is 2 or 4, calls helper routines with the entry's identifier.
- `copy_file_and_set_protection`: Opens a file with the dos library, examines it and reads its contents into allocated memory, then writes them out.
- `scan_string_against_install_buffer`: Copies a string into a fixed buffer in the install state, then scans it to the end and compares its characters with the argument string.
- `create_directory_if_missing`: Examines a path with a lock first and creates the directory when the examine does not find it.
- `stub_return_without_work`: The entry is eight instructions long, but its pseudo-code shows only a return.
- `stub_no_op_return`: Eight-instruction entry whose only effect is an immediate return to its single caller.
- `free_install_memory_lists`: Frees two linked lists of memory blocks by calling the library free routine at offset minus 210 on each node, and clears the head of the first list.
- `split_path_into_components`: Splits a slash-separated path into components by writing a zero byte after each separator and recording where each component starts, for up to 16 components.
- `split_path_into_parts`: Separates a path into a volume part before a colon, a directory part before the last slash, and a file-name part, copying each into the output buffers that were supplied.
- `append_separator_if_missing`: Appends the separator byte held in a global variable to a string when the string is non-empty and does not already end in a slash, colon or backslash.
- `copy_string_return_end_pointer`: Copies a null-terminated string from the source to the destination, including the terminator, and returns the address where the terminator was written.
- `convert_integer_to_string`: The routine repeatedly calls a fixed-point division routine to extract digits, converting them to ASCII characters.
- `convert_binary_to_string`: The routine converts a binary integer into a string representation by repeatedly extracting bits and appending characters to a buffer, then copies the result to an output pointer and returns the length.
- `convert_hex_to_string`: The routine converts a hexadecimal integer into a null-terminated ASCII string using a lookup table.
- `parse_signed_integer`: The routine reads a signed decimal integer from a zero-terminated string pointed to by a register, handling an optional leading plus or minus sign.
- `allocate_file_handle_record`: Checks a flag bit in a file record and, when it is clear, calls a routine to obtain a buffer and sets the record's fields from a global value.
- `convert_string_to_uppercase`: Converts each letter of a zero-terminated string to upper case in place, using a character-class table to decide which bytes are lower-case letters.
- `write_to_file_record`: Writes data to a file record after checking the record's flags.
- `check_handle_mode_then_call`: Checks that a handle exists and has bit one set, otherwise it records an error code and returns minus one.
- `read_and_strip_newlines`: Reads data from a source, strips carriage return characters, and returns the length of the processed data.
- `seek_file_record`: Seeks within a file record to the position given by the mode parameter, storing the DOS error code when the seek fails.
- `close_file_record`: Closes a file record by calling the DOS close routine when it is open, releasing its buffers when they are present, and clearing its handle.
- `open_file_record`: Adds a file record to a linked list, copies the file name into it and translates the access-mode flags into record flags.
- `normalize_fixed_point`: The routine takes two 32-bit inputs and normalizes a fixed-point value by scaling the numerator with a variable shift derived from the denominator’s magnitude, performing a division with remainder and a borrow correction, then returns a 64-bit result composed of a signed quotient and a scaled remainder.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `stub_empty_return_routine`: The pseudo-code shows only a return, while the facts list 25 instructions and one unnamed call, so only the return is claimed.
- `close_all_file_records`: Walks the list of open file records and closes each one that needs it, calling the DOS close routine or a stored callback depending on the record's flags.
- `parse_command_line_open_streams`: Splits the command-line text into up to 32 argument strings, honouring quotes and star escapes, then sets up the input and output streams, falling back to the NIL: device.
- `copy_message_text_to_stack`: Copies up to seventy-nine characters of a string from the caller's state into a stack buffer and terminates it.
- `process_break_signal`: Copies a string into a stack buffer, checks for a specific task or signal, and updates a global variable based on the result of a library call.
- `find_file_record_by_index`: Returns the file record at the given index in the linked list, walking the list from its head.
- `check_break_signal_and_report`: Calls SetSignal to read the signal mask.
- `find_byte_in_buffer`: Scans a buffer of the given length for a target byte and returns a pointer to the first match, or null when the length runs out without one.
- `append_string_to_buffer`: Appends a source string to the end of a destination string, copying no more characters than the count passed in the data register and always writing a terminator.
- `concatenate_strings_unbounded`: Finds the terminating zero of the destination string and copies the source string, including its terminator, onto that point.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `copy_byte_block_safe`: Copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the source address is higher than the destination.
- `release_pool_block_checked`: Frees a block from the state's memory pool after checking that its address lies inside a recorded region, recording error code twenty-two when it does not.
- `unlink_file_record`: Searches a linked list of structures to find a specific entry based on a provided pointer and offset.
- `allocate_pool_block_from_region`: Rounds the requested size up to a multiple of four and searches the recorded pool regions for a free block of that size.
- `allocate_memory_block`: Searches a linked list of memory pools for a free block of a specified size and splits the block if it is larger than requested.
- `close_all_open_streams`: Runs the registered callbacks, then closes or notifies each node in the stream list and sets the closing flag.
- `append_byte_to_buffer`: Appends a byte to a buffer and keeps its used count in a structure field.
- `call_and_return_flag_status`: Calls a routine with the incoming parameter and a stack slot, then returns minus one when a flag field in a global structure has bit 4 or bit 5 set, and zero otherwise.
- `read_line_into_buffer`: Reads characters from the stream into a buffer until a newline or end of input, then terminates the buffer.
- `read_next_input_byte`: Returns the next byte from an input stream, refilling its buffer through helper calls when it is empty and recording end-of-file or error flags.
- `compute_stream_position`: Works out a stream position from a read result: when a flag is set it subtracts the number of newline characters counted in the buffer, and otherwise it subtracts the remaining buffered count.
- `calculate_file_offset`: Calculates a file offset by counting newline characters in a buffer and adjusting the result based on specific flags and a length field.
- `reset_structure_fields`: Clears several fields within a structure, performs bitwise operations on a field based on a flag, and updates a value based on the result of another routine.
- `check_stream_access_mode`: Checks a stream's mode bits for the access requested.
- `copy_data_to_buffer`: Copies characters from a source to a destination buffer while checking structure flags and handling newlines.
- `copy_string_data`: Copies a sequence of bytes from a source buffer to a destination buffer, updating a counter and checking status flags in a structure.
- `update_file_node_state`: Resets the position and status fields of one file record, or when given no record, checks each record in the list.
- `flush_and_release_stream_node`: Returns minus one for a null or unopened node.
- `validate_structure_state`: Checks if a pointer or a value at offset 24 of a structure is zero, and if certain flags or pointers are present, calls other routines to clear or update structure fields and return a status code.
- `open_stream_from_mode_string`: Parses a two-character access mode made of read, write, append and optional plus signs into flag bits, then calls another routine to open the file.
- `find_or_allocate_free_node`: Finds the first unused node in a list, or allocates and zero-fills a new node when none is free, then passes the node to an unnamed routine.
- `parse_format_string_and_print`: Parses a format string to identify conversion specifiers, flags, and width/precision arguments, then converts corresponding integer or pointer values from a parameter list into ASCII strings using helper routines.
- `frag_movea_move`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to the memory at an offset from a7 (2 times in a row).
- `parse_format_string_print`: The routine parses a format string to identify conversion specifiers and flags.
- `delete_file_save_error_code`: Calls the DOS library to delete a file.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `call_unnamed_routine_then_return`: Calls one unnamed routine and returns, with no other work.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:Wing

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `busy_wait_16bit_countdown`: Loops, decrementing a 16-bit counter until it wraps past zero to minus one.
- `decode_audio_data`: Decodes audio bitstream data into a destination buffer using a helper routine for bit extraction, with loop control based on a counter and conditional execution paths.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `decode_bitstream`: Reads bits from a source buffer and writes decoded bytes to a destination, handling bit extraction and assembly with carry propagation through multiple iterations.
- `frag_negx_ori_move_movem_bra`: Short code fragment that negates d0 with the extend flag, then merges 234 into d0, then copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Wing Commander - Platinum Edition (1992)(Mindscape)(Disk 1 of 3)[h LSD][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `walk_length_prefixed_entries`: Walks a table of variable-length entries, advancing by the size stored at each entry's third byte plus three, and calls two library routines per entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block`: Clears a block of memory by writing zero values through a pointer, decrementing a counter until it reaches negative one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields`: Initializes several fields in a structure including a counter and status byte, then performs conditional operations based on memory offsets and updates pointers.
- `decrement_until_minus_one_3`: Decrements a local variable until it reaches negative one and then returns.
