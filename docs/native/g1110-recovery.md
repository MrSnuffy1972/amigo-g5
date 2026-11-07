# Game 1110 (wing commander (1992)(mindscape)(de)(disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01110). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (951248 top-level bytes in 47 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 559 of 559 named (0 author symbols), 559 of 559 described (52 by rule, 398 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3191 of 3191 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10859 of 10898 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10642 |
| code | 33904 |
| data | 843039 |
| padding | 10415 |
| unused | 53248 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Setmap | 13 | 653 | 31 |
| Else | 2 | 117 | 2 |
| SetPatch | 24 | 833 | 45 |
| EndCli | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| setpatch.pardon | 20 | 750 | 43 |
| FastMemFirst | 7 | 207 | 6 |
| Endif | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| setpatchV38.31 | 2 | 893 | 76 |
| Version | 11 | 574 | 18 |
| d | 1 | 13 | 1 |
| Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:InstallWingCommander | 6 | 45 | 2 |
| Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:wing | 8 | 253 | 7 |
| Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 222 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 165 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 49 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 77 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4509 | 270 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1170 | 20 |

## Cases handled

- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:PLANET0.PKT: high-entropy data file (7.70 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:ENDGAME.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:FONTS.PKT: high-entropy data file (7.88 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:PCSHIP03.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:PCSHIP01.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:MIDGAME2.PKT: high-entropy data file (7.94 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:DEMO.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:MIDGAME0.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:PCSHIP02.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:DEMOSAMP.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:MIDGAME3.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:PCSHIP00.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:MIDGAME1.PKT: high-entropy data file (7.91 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 57 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 82 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 49 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 72 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1982 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 547 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:InstallWingCommander: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- wing commander (1992)(mindscape)(de)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Setmap

- `parse_quoted_command_arguments`: Splits the command line into space-separated tokens and stores each token in a buffer.
- `save_result_and_cleanup`: Stores its parameter into a slot of a state structure reached through a global pointer.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `propagate_conditional_map_updates`: Calls a sequence of routines on a set of global values, passing some of them only when their flags are set.
- `store_pair_and_run_setup`: Stores its two arguments into global slots, then calls one routine with a fixed set of values, including the sizes 440 and 60, and calls another routine with a constant.
- `load_keymap_by_name`: Opens the libraries and device it needs, converts a keymap name to lower case and finds the matching keymap file in the keymaps directory.
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
- `forward_args_with_stack_slot`: Passes a global value and its argument on to another routine, together with the address of an eight-byte stack slot.
- `call_indirect_table_entry`: Calls a routine through a function pointer read from a global table at a fixed negative offset, then returns.
- `call_then_measure_stack_string`: Invokes a function pointer loaded from a global, then measures the length of a string in a 140-byte stack buffer by scanning to its null terminator.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_block_then_call_pointer`: If the destination pointer is non-null, copies sixteen longwords from a fixed offset within a register-held structure into it.

### SetPatch

- `run_setpatch_entry_point`: Program entry point that opens the DOS library, allocates a control structure, and parses command-line text into an argument array when arguments are present, then runs the setup routine.
- `release_patch_state`: Stores the status parameter in a patch-state structure reached through a global pointer, then conditionally calls cleanup vectors for handles, libraries and resources.
- `run_patch_sequence_conditionally`: Runs an unconditional setup step, then, when a global flag is nonzero, runs a step on the parameter and feeds its result to another step.
- `install_patches_from_options`: Installs patches to system libraries and devices according to command-line options and the system version.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_via_table`: Calls a routine through a function pointer read from a global variable at a fixed offset, and returns that routine's result.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `invoke_setpatch_install`: It invokes the routine whose address is stored at a fixed offset within the SetPatch library, passing no arguments.
- `invoke_setpatch_handler`: It invokes the routine whose entry point is stored at a fixed offset within the SetPatch library and then returns without using any value.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_patch_routine`: Jumps through a function pointer stored in a global variable.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `apply_setpatch_patch`: It invokes the routine whose address is stored at a fixed offset within the SetPatch library, passing no arguments.
- `allocate_and_install_code_stub`: Checks the memory size and flags in a global structure, then allocates a memory block of a suitable type and copies a fixed block of code from another routine into it.
- `store_word_sum_and_call`: Computes the complement of a word sum over a 22-word structure field and stores it in the structure.
- `patch_disk_resource_functions`: Opens the disk resource, and when its library version is below 36 it installs replacement functions with SetFunction and stores the previous function values in two globals.
- `copy_word_if_call_succeeds`: Calls a routine through a function pointer stored in a global variable.
- `conditional_call_handler`: Returns unless a data value matches a memory location or negative one, then calls a function through a global function pointer and returns.
- `call_handler_when_minus_one`: Returns unless the data register holds minus one, in which case it calls a routine through a function pointer held in a global variable.
- `check_marker_and_dispatch`: Sets a global flag, then checks a four-character marker at the start of low memory.
- `call_table_vector_136`: Calls the routine at a fixed offset in a table referenced by a global pointer, then returns.
- `fill_vector_table_with_stubs`: Stores a computed offset into a global variable, then fills sixteen consecutive table entries at a fixed low address with addresses of a routine, advancing the address by two bytes for each entry.
- `call_routine_repeatedly_return_last`: Calls one unnamed routine in a run of consecutive calls and returns the result of the last call.
- `call_and_return_value`: Invokes a routine through a data-table pointer several times and then returns its value.
- `call_many_and_return`: Invokes a routine through a data-table pointer many times and then returns its value.
- `return_input_value`: Returns the value passed in the data register unchanged, with no other operations.
- `skip_setfunction_on_new_versions`: Returns without action when the library version field is above 35.
- `allocate_entry_table_blocks`: Allocates a block sized from a count field, then fills each slot with a value from the input and a further allocated block.
- `loop_call_set_high_bit`: Loops a fixed number of times calling a routine through a pointer derived from a data offset, then returns a value from a pointer with its high bit set.
- `call_and_set_high_bit`: Calls a function through a pointer derived from a data offset and then returns a value from a pointer with its high bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `call_predicate_then_notify`: Calls a routine through a global function pointer with a longword read from a structure.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_words_for_marker`: Advances through an array of 16-bit words from the input pointer until it reaches a word whose high byte equals a fixed marker value, then returns a pointer to that word.
- `decode_relative_entry_pointer`: Decodes a relative displacement from the word at the input pointer, using either its low byte or the following word, then halves and sign-extends it.
- `compute_word_offset`: Reads a 16-bit word two bytes past the input pointer, halves it and sign-extends it from bit fourteen.
- `store_three_derived_pointers`: Calls a pointer-advancing helper routine in sequence to step through a run of entries, then stores three of the returned pointers in global variables.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_routine`: Calls the Dispatch routine of the exec library through a library base pointer held in a global variable, and returns nothing.
- `pass_arguments_with_stack_pointer`: Passes its two parameters on to another routine together with a pointer to a stack slot at a fixed offset, then returns.
- `format_and_measure_stack_text`: Calls the RawDoFmt routine of exec through its library base, then measures the length of a 140-byte stack text buffer up to its null terminator.
- `store_byte_through_pointer`: Writes the low byte of the data register to the memory location addressed by the address register, then returns.

### EndCli

- `run_endcli_program_setup`: Program entry that scans a linked list for its largest value, then builds a table of incrementing longwords and a copy of a stack-held buffer.
- `copy_words_then_call`: Copies a fixed count of words from a source table into a destination buffer pointed to by a register, advancing both pointers until the counter reaches zero.

### Echo

- `find_max_and_copy_block`: It scans a linked list of records to find the largest value in their size fields, then writes that value and a run of incrementing marker words into a buffer.
- `copy_argument_block`: Copies a fixed-size block of words from a source table pointed to by a base register into a destination buffer passed as an argument, repeating the copy until a counter reaches zero.

### setpatch.pardon

- `parse_command_line_arguments`: Parses a command-line argument string, splitting it into tokens and decoding escape sequences, then stores the resulting token pointers in a structure.
- `restore_patch_state`: Retrieves a pointer from a global structure and stores a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_helper_call`: Checks a global variable; if non-zero, it calls a helper routine with an argument, then calls another routine with the result returned by the helper.
- `install_system_patches`: Installs patches into the exec and layers libraries after checking the library version fields and whether the patches are already present.
- `call_data_handler`: Invokes the handler stored at a pointer derived from a base in the data section, calling the routine it points to.
- `call_indirect_global`: It invokes the routine whose address is stored in the global variable located just before the one used by the sibling routine, without returning a value.
- `call_indirect_global_hook`: It invokes the routine whose entry pointer is stored in a global variable and then returns.
- `call_indirect_global_2`: It calls the routine whose address is stored at a fixed offset from a global variable and then returns without using any arguments.
- `call_library_vector_354`: Calls a library routine at a fixed offset from a library base held in a global variable, and returns nothing.
- `call_indirect_global_ptr_2`: It invokes the routine whose address is stored at the offset referenced by a global variable, passing no arguments.
- `invoke_indirect_call`: It invokes the routine whose address is stored at a fixed offset within a global variable, performing a single indirect call with no arguments.
- `invoke_setpatch_hook`: It invokes the routine whose address is stored at a fixed offset within the SetPatches library, passing no arguments.
- `allocate_and_patch_memory`: Checks fields of a global structure and, if the conditions are met, allocates a memory block and obtains a code pointer via library calls, copies a short code stub into the target, updates a pointer in the structure, computes a checksum over a short array and stores its complement, then returns zero.
- `update_checksum_and_dispatch`: Computes a checksum over a short array in a structure and stores its complement, then branches through a jump table to a selected routine.
- `reset_system_after_check`: Writes a marker into low memory and sets an interrupt-related global flag.
- `set_control_value_if_negative`: Sets a global control value to a fixed value when a longword read from a structure is negative.
- `install_vector_with_delay`: Clears two memory words, sets a global control value, and stores a routine pointer at a fixed low-memory vector.
- `wait_then_reset_or_call`: Runs a counted delay loop, then calls the reset routine when a status byte test fails.
- `fetch_library_entry_and_close`: Returns a code pointer that it obtains from a library when a state word is set.
- `open_intuition_library_entry`: Opens intuition.library and calls a function at a fixed offset from its base, then closes the library and returns that function's result.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies d0 to (a3)+, then clears (a3), then returns.
- `copy_string_with_null`: Writes a zero byte at the destination, then copies a null-terminated string from the source after it, and finally writes a minus-one byte after the terminator.
- `reset_after_delay`: Runs a counted delay loop, then calls the reset routine and jumps through a stored pointer.
- `initialize_patch_function_table`: Writes a computed offset to a global variable and fills a 16-entry table with successive addresses of a routine, incrementing the pointer each iteration.
- `empty_return_value`: An empty routine that performs no work and returns a value from the data register.
- `invoke_routine_repeatedly`: It invokes the same routine repeatedly in a fixed sequence and then returns the value produced by the final call.
- `chain_routine_calls`: Calls a sequence of unnamed routines in order, then returns the value produced by the final call.
- `return_input_value_2`: Returns the input value from the data register unchanged without performing any other operations.
- `check_limit_then_call_routine`: Checks whether a value stored in a global variable exceeds a fixed limit and returns early if it does.
- `copy_into_struct_then_set_high_bit`: Copies a value into a newly allocated structure, walks a linked list copying entries up to a limit, then runs a loop calling a routine through a pointer before returning a value with its high bit set.
- `call_routine_then_set_high_bit`: Loops a fixed number of times calling a routine through a pointer stored in a structure, then sets the high bit of a value read from a global variable before returning it.
- `call_set_high_bit`: Calls a routine through a pointer derived from a data offset, then returns a value from a pointer with its high bit set.
- `invoke_cleanup_if_active`: Reads a value from a structure field, and if it is nonzero it stores that value elsewhere and calls a cleanup routine before returning whether the value was nonzero.
- `return_zero_value`: Returns zero in the data register with no other work.
- `scan_for_marker`: Advances a word pointer one element at a time until the high byte of the current word equals 97, then returns the pointer to that element.
- `resolve_relative_offset`: Reads a 16-bit value from a buffer, sign-extends it, and returns a pointer advanced by the computed offset.
- `compute_word_offset_2`: Reads a 16-bit value from two bytes after the input pointer, sign-extends it if the high bit is set, doubles it, and returns that amount added to the input pointer.
- `patch_global_pointers`: Recursively invokes a helper routine, computing successive offsets and storing the resulting pointers into a series of global variables.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_dispatch_routine`: Reads a routine pointer from a global structure and invokes it, then returns.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack, then returns.
- `call_global_and_checksum_buffer_2`: It invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_from_register_2`: Stores a byte from a data register into a location pointed to by an address register, then returns.

### FastMemFirst

- `run_fastmem_entry_point`: Program entry point that allocates a control structure, parses command-line text into a linked list of argument strings with quote handling, and runs the setup routine.
- `find_fast_and_chip_blocks`: Walks the memory block list from a global structure to find the first block inside a fixed high address range with one type tag, and the first block with another type tag below a fixed address.
- `call_indirect_global_hook_2`: It invokes the routine whose address is stored at a fixed offset within a global variable, then returns without doing anything else.
- `call_indirect_from_global`: It invokes the routine whose entry pointer is stored at a fixed offset from a global variable, then returns without doing anything else.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### Endif

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `compute_and_store_max_entries`: Scans a table of entries to find the largest value, then stores that maximum along with a run of sixteen consecutive status flags into a fixed structure.
- `copy_longword_block`: Copies a fixed number of longwords from a fixed offset in a base pointer into a caller-supplied buffer, decrementing a counter until the block is fully transferred.

### setpatchV38.31

- `run_setpatch_v38_installer`: Program entry for the V38 patch installer.
- `install_code_stub_2`: Checks a flag in a structure and, when set, copies a fixed-size code stub into an allocated memory block, then invokes several library routines to finalize the installation.
- `install_code_stub`: Checks a flag in a structure, copies a fixed-size code stub into an allocated memory block, and then invokes several library routines to install it.
- `allocate_copy_stub_and_install`: Copies a 50-word code stub into an allocated memory block and then invokes several library routines to install it, returning one and skipping the copy when the allocation fails.
- `update_control_byte_from_probe`: When a status byte does not hold the sentinel value, probes memory downward from a fixed high address for a self-referencing longword to select a mask.
- `free_patch_table_entries`: Clears a pointer at a fixed offset and frees it when it was set.
- `dispatch_message_by_type`: Checks the type field of a message.
- `handle_dispatched_message`: Examines a message type field and, when it matches a known value, updates the associated structure fields and clears flags before replying to the message through the executor.
- `reply_to_message_type_four`: When the message sub-field holds the value four, updates the associated structure fields, clears a flag and replies to the message through the exec base.
- `reply_when_field_clear`: When a longword at a fixed offset is zero, copies a stored value into another field, clears a flag and replies to the message through the exec base.
- `call_two_setup_routines`: Calls two routines in sequence and returns zero.
- `service_flagged_structure`: Fetches a structure through a library call.
- `call_one_routine_and_return`: Two-instruction routine that calls one other routine and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_fixed_address`: A single jump entry that passes control straight to a fixed address elsewhere in memory.
- `store_device_structure_fields`: Calls a library routine and, when it returns a non-null structure, stores a longword from that structure and the structure pointer into two globals, and the library base into a third.
- `check_range_then_call_library`: Compares a stack value with a range held in two globals.
- `call_stored_routine_pointer`: Calls a routine through a function pointer held in a global variable and returns nothing.
- `check_limit_then_call_library`: Compares an input value with an upper limit held in a global.
- `store_offset_from_library_result`: Chooses one of two constant offsets depending on whether a word field equals a fixed value, then calls a library routine and, when it returns non-null, stores the result plus the offset in a global.
- `clear_matching_table_entries`: Uses the input index to look up a table slot.
- `check_and_fill_offsets`: Compares a stored value at a fixed offset against a known routine address and, when they match, writes all-ones to two consecutive longwords before returning.
- `clear_table_entry_on_match`: Multiplies an input index by four to index a table, and if the slot matches a stored value it writes all-ones to two adjacent fields, otherwise it calls another routine.
- `call_global_routine`: Invokes a routine pointed to by a global variable and then returns.
- `run_three_library_steps`: Calls three library routines in sequence, storing a fixed value in a global between the second and third, and returns one.
- `store_argument_in_global`: Stores the address register input into a global variable and returns one.
- `record_value_and_fill_pattern`: Calls a stored routine and records its result in a table slot chosen by the input.
- `fill_block_on_sentinel`: Reads a value from a lookup table, and when it matches a sentinel and a related entry is nonzero, fills a target block with an alternating bit pattern.
- `fill_block_on_sentinel_value`: Reads a value from a lookup table and, when it matches a sentinel and a related entry is nonzero, fills a target block with an alternating bit pattern.
- `update_table_and_fill_memory`: Reads a value from another routine, stores it in a table when it changes, and fills a memory block with a repeating pattern when it matches a specific constant while a related field is non-zero.
- `add_library_result_to_pointers`: Calls a library routine and, when it returns non-null, adds the result into two stored address values.
- `copy_field_on_matching_event`: When two input values match fixed constants and a flags field has bits set, copies one longword field to another.
- `copy_field_on_matching_value`: When a value equals a fixed constant and a flags field has bits set, copies one longword field to another.
- `clear_flag_bit_then_call`: Fetches a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise, clears its lowest bit, and then calls the library routine at offset minus 636.
- `find_matching_list_node`: Returns the input value unchanged when the address input is nonzero.
- `search_list_by_first_arg`: Returns the input value unchanged when the address input is nonzero, otherwise forbids multitasking, searches a linked list for a node whose short field matches the input, permits multitasking again, and returns zero.
- `search_linked_list_entries`: It calls a routine pointed to by a global variable offset by a fixed amount.
- `search_list_forbid_multi`: Returns the input value unchanged when the address input is nonzero, otherwise forbids multitasking, searches a linked list for a matching node, and then re-enables multitasking.
- `find_matching_list_node_2`: Returns the input value unchanged when the second address input is nonzero.
- `search_list_by_third_arg`: Returns the input value unchanged when the second address input is nonzero, otherwise forbids multitasking, searches a linked list for a node whose short field matches the input, permits multitasking again, and returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `search_list_return_status_two`: Forbids multitasking, searches a linked list for a node whose short field matches the input, then permits again.
- `search_list_return_number`: Disables multitasking, walks a linked list comparing each node's short field to a value, re-enables multitasking, and returns the original input when a matching node is found or a fixed number otherwise.
- `search_list_or_minus_one`: Returns minus one when the input is minus one.
- `search_list_return_minus_one`: Returns negative one immediately when the input is negative one, otherwise disables multitasking, walks a linked list comparing each node's short field to a value, re-enables multitasking, and returns the original input or negative one.
- `find_matching_list_node_3`: Forbids multitasking, searches a linked list for a node whose short field matches the input, then permits again.
- `search_list_return_zero`: Disables multitasking, walks a linked list comparing each node's short field to a value, re-enables multitasking, and returns the original input when a matching node is found or zero otherwise.
- `find_list_node_return_argument`: Forbids multitasking, searches a linked list for a node whose short field matches the input, then permits again.
- `return_third_parameter`: Re-enables multitasking after searching a list, and returns a fixed number when a status flag is set or the third argument otherwise.
- `return_parameter`: It takes a single parameter and returns it unchanged without modifying it.
- `scan_string_and_open_library`: It reads a pointer from a global variable, optionally replacing it with a value returned by a library open routine, then scans a word string until it finds a matching entry and stores the updated pointer.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `call_pointer_unless_marker`: Compares the word at a stored address with a fixed marker value.
- `dispatch_by_marker`: It compares a stored word against a fixed marker value and, when they differ, calls the routine referenced by that address before returning; otherwise it calls itself.
- `call_one_routine_and_return_2`: Calls one other routine and returns.
- `copy_string_into_allocated_vector`: When a library call succeeds, measures a string from the input pointer, allocates a vector to hold it, and copies it in with a length byte before the text and a terminator after.
- `copy_length_prefixed_string`: Measures a string, copies it into a buffer with a length byte placed before the text, adds a terminator, and calls a routine through a pointer in a register-held table.
- `copy_counted_bytes_backwards`: Copies a counted run of bytes backwards from one buffer into another, stores a length byte before them, adds a terminator, and calls a routine through a register-held table entry.
- `return_immediately`: It executes three instructions and then returns without modifying any data or registers.
- `call_routine_return_register_pair`: Calls a library routine through a fixed offset and returns the values left in the two data registers as a 64-bit result.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `decrement_counter_then_remove_node`: Decrements a counter and, when it reaches zero, clears a field and removes a node from a doubly linked list, while other branches call routines or decrement a different counter.
- `dispatch_on_flag_then_call`: Branches on a condition flag to either call two routines through structure pointers or decrement a counter field, then always calls a third routine before returning.
- `init_semaphore_and_call`: Initializes a semaphore, calls a helper routine, then releases the semaphore before returning.
- `remove_node_from_list`: Removes a node from a doubly linked list by rewiring its neighbors, clears a flag bit in the node's data, and increments a counter before releasing the semaphore.
- `insert_node_into_list`: Increments a counter, calls a helper routine, then inserts a node into a doubly linked list or releases the semaphore and returns when the end of the list is reached.
- `increment_counters_then_clear_bit`: Increments a counter and a flag byte, and when both are nonzero it clears a bit in a byte pointed to by a structure field and calls two helper routines, otherwise it increments another counter.
- `clear_bit_or_increment`: When a flag is set it clears a bit in a byte and calls two helper routines, otherwise it increments a counter, then releases the semaphore before returning.
- `call_handler_outside_range`: Calls a stored routine and returns its byte result when the stack pointer lies outside a range recorded in a library structure or a flag field equals minus one.
- `call_handler_outside_range_2`: Calls a stored routine when the stack pointer lies outside a range recorded in a library structure or a flag field equals minus one; otherwise it returns without action.
- `check_stack_and_call_routine`: It checks whether a stack address falls between two stored bounds or a stored word equals negative one, and if so calls the routine referenced by a global pointer and returns its result; otherwise it returns the initial register value.
- `call_stack_check_handler`: Tests whether a stack address falls between two stored bounds or a stored word equals negative one, and if so calls a handler routine through a global pointer.
- `store_arguments_return_one`: Stores the address register input in one global variable and the library base register in another, then returns one.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `call_one_routine_and_return_3`: Calls one other routine.
- `stub_listing_unavailable`: Its decompiled listing could not be produced.

### Version

- `parse_command_line_arguments_2`: Initializes a small task structure with a couple of values, then reads a null-terminated string and splits it into space-separated tokens, copying each token into a buffer while expanding quote and escape sequences.
- `store_value_and_dispatch_by_flags`: Loads a pointer from a global variable, stores a passed value at a field of the pointed structure, and then invokes other routines depending on which non-zero fields the structure contains.
- `check_library_version`: Parses command-line arguments to open a named library or device, then compares its version and revision with the user's thresholds and the Kickstart and Workbench versions.
- `call_two_helpers_with_flag`: Calls one routine with its first argument and the constant 128, then calls another routine with its second argument.
- `parse_integer_string`: Parses a decimal integer from a string, skipping an optional plus or minus sign and then reading digits, and applies the sign to the result.
- `compare_strings`: Compares two null-terminated strings byte by byte.
- `calculate_string_length`: Counts the characters of a null-terminated string up to, but not including, the terminator, and returns that length.
- `call_reschedule_routine`: Calls the Reschedule routine through the exec base and returns nothing.
- `call_global_function_pointer`: Invokes a function pointer loaded from a global variable and then returns.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_indirect_d1_offset`: It invokes the routine whose address is stored at a fixed offset from register d1 and then returns without altering any data.
- `call_indirect_d1_offset_2`: It invokes the routine whose address is stored at the offset located by subtracting 210 from the value in d1, then returns.
- `invoke_indirect_routine`: It invokes a routine whose entry pointer is stored in a global variable, then returns without touching any data.
- `call_indirect_function`: It invokes a routine whose entry point is stored in a global function pointer and then returns without touching any data.
- `call_indirect_global_ptr`: It invokes the routine whose address is stored at a fixed offset before a pointer held in a global variable, then returns without using any other data.
- `call_indirect_handler`: Invokes a function through a pointer obtained from a fixed data-table offset and then returns.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_from_register`: Copies a single byte from a register into the memory location pointed to by another register.

### d

- `stub_bad_instruction_data`: Contains only undecodable instruction data, so no routine behaviour can be recovered.

### Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:InstallWingCommander

- `run_wing_commander_installer`: Installer entry that runs a sequence of unnamed calls, then calls FreeMem through the exec base in a loop.
- `decrement_local_until_minus_one`: Decrements a local short variable in a loop until it equals negative one and then returns.

### Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:wing

- `run_wing_program_entry`: Program entry that runs a sequence of unnamed calls, then calls FreeMem through the exec base in a loop.
- `decrement_counter_to_minus_one`: It repeatedly subtracts one from a signed 16-bit counter held in a register, looping until that counter reaches negative one, then exits.
- `unpack_bitstream_with_runs`: Decodes a bit stream read backwards from the input, using variable-length run lengths to copy previously written data into the output over four passes.
- `decode_backward_bitstream_block`: Continues the backward bit-stream decoding by reading run-length codes and copying earlier output into the destination, using a table of length bases.
- `copy_decoded_runs_to_output`: Decodes backward bit-stream run lengths and copies earlier output into the destination for each run, using a length table.
- `decode_longword_runs_to_output`: Decodes backward bit-stream run lengths and writes the output in four-byte steps, copying earlier output in longword groups.
- `check_bounds_then_trap`: Stores a byte, rotates a word at a fixed offset, then checks an index against a bound and raises a bounds-check trap when it is out of range.

### Wing Commander (1992)(Mindscape)(De)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one`: Returns minus one in the data register and does nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: This routine contains a single instruction and performs no operations on registers or memory before returning.
- `update_state_and_call_subroutines`: stores several values into a table using offsets from a parameter, calls multiple subroutines, and updates a global variable if a comparison fails.
- `reset_table_entry`: calls two subroutines and sets a value in a global variable to all ones if a specific condition is met.
- `call_subroutine_and_return`: calls another routine and returns zero.
- `execute_sequence_of_calls`: Calls three different routines in succession.
- `update_state_and_call`: Stores several values into a table using offsets from a parameter, updates a global variable based on a comparison, and executes a sequence of three subroutines.
- `copy_data_to_table`: copies two values from a pointer into a table and calls several subroutines to process data at specific offsets.
- `copy_data_to_table_2`: Copies two values from a pointer into a table and calls several subroutines to process data at specific offsets.
- `pass_pointer_offsets`: Calculates three memory addresses by applying constant offsets to a value in a register and passes them as arguments to another routine.
- `copy_data_to_table_3`: Copies two values from a pointer into a table and calls two subroutines to process data at specific offsets.
- `execute_two_subroutines`: calls two different subroutines in sequence.
- `set_byte_at_offset`: Calls a routine and then sets a byte to one at a specific offset calculated from a global base and a parameter.
- `load_value_if_zero`: checks if a value in a table is zero, and if so, loads a value from another routine into that table entry.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_single_subroutine`: calls a subroutine with two specific memory locations as arguments.
- `store_parameters_to_table`: stores a byte and a word into a table at an offset calculated from a parameter.
- `sort_six_element_table_2`: Performs a bubble sort on a table of six-byte structures by comparing values at a specific offset and swapping the structures.
- `sort_six_element_table`: Performs a bubble sort on a table of six-byte structures based on a comparison of values at a specific offset.
- `generate_unique_sequence`: Generates a sequence of five unique values by repeatedly selecting from a range and checking against a table of previously selected values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_pointer_table`: fills a table with eight pointers using a loop and sets four additional global pointers before performing a subtraction and storing the result.
- `copy_memory_blocks`: Copies two distinct blocks of data from two different source locations to two different destination locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_to_offset`: Stores a two-byte value from a parameter into a global variable at a specific offset.
- `copy_block_of_data`: Copies a sequence of thirty-four words from a source pointer to a destination table.
- `multiply_and_store_long`: Multiplies a long value by three, stores the result back into a global variable, and returns the product split into two parts.
- `calculate_modulo_remainder`: Calculates the remainder of a value divided by another value plus one.
- `calculate_offset_remainder`: Calculates a remainder based on the difference between two values and adds it to the first value.
- `set_flag_bits`: Sets two specific global word locations to a value derived from a parameter with the lowest bit set.
- `update_range_values`: Calls another routine to retrieve a value and then stores that value and the value plus three thousand into two consecutive global variables.
- `multiply_long_integers`: Performs a signed multiplication of two 32-bit integers and returns the 64-bit result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_update_short`: Compares a passed short value against a global variable and, if different, updates the variable and executes a routine that may reset other global values.
- `call_library_vector_wrapper_relay`: A 24-instruction routine that is entered from outside the static control flow and is reached from two callers, retrieve_linked_list_value and reallocate_and_copy_buffer.
- `store_status_result`: Calls another routine and stores its return value into a global variable.
- `clear_status_pointer`: Calls another routine using a value stored in a global variable.
- `search_linked_list`: Iterates through a list of sixteen-byte structures to find an entry where a specific byte matches a provided parameter.
- `retrieve_linked_list_value`: Calls another routine to obtain a value and then uses that value to retrieve data from a secondary routine.
- `decompress_and_reallocate_data`: Validates a size against a table, calculates offsets, and calls routines to decompress data into a destination buffer.
- `decompress_data_block`: Checks if a requested size is within bounds and then processes a compressed data stream into a destination buffer.
- `reallocate_and_copy_buffer`: Attempts to allocate a new memory block and copies data from an existing buffer to the new location if the size requirements change.
- `reallocate_buffer_data`: Attempts to allocate a new memory block and copies data from an existing buffer to the new location if the size requirements change.
- `reallocate_input_buffer`: Attempts to resize an existing memory buffer by allocating a new block and copying the current contents to the new location.
- `find_index_in_table`: Searches a table for a matching short value and returns a value from a secondary table based on the found index.
- `execute_conditional_routine`: Calls a routine and, if the return value is non-zero, passes that value as a parameter to another routine.
- `append_to_table`: If a search routine returns zero and a counter is below ten, it stores a short and an integer into tables based on that counter and increments the counter.
- `decode_bitstream`: Processes a sequence of bytes using bitwise shifts and XOR operations to reconstruct a value.
- `copy_memory_block`: Copies a block of memory from a source address to a destination address using a count provided in a register.
- `expand_bitmask_data`: Iterates through a nested structure of bytes to expand bit-encoded values into a sequence of characters in a destination buffer.
- `expand_bitmask_data_2`: Iterates through a source buffer to expand bit-encoded data into a destination buffer using bitwise rotations and shifts.
- `extract_bitfield_from_compressed_stream`: Iterates through a compressed data stream by traversing a lookup table or index array to find specific bitfield values.
- `extract_bitfield_value_short`: Extracts a bitfield from a data structure by calculating an offset using bitwise shifts and rotations, then stores the result into a destination.
- `extract_bitfield_value_long`: Extracts a bitfield from a data structure using bitwise shifts and rotations, incorporating a concatenated value from a register and a memory byte.
- `extract_bitfield_value_extended`: Extracts a bitfield from a data structure by calculating an offset using bitwise shifts and rotations, incorporating a concatenated value from multiple bytes.
- `extract_bitfield_value_large`: Extracts a bitfield from a data structure using bitwise shifts and rotations, incorporating a concatenated value from four bytes.
- `extract_bitfield_value`: Extracts a specific bitfield from a data structure by calculating an offset using bitwise shifts and rotations.
- `decompress_bitstream_data`: Processes a compressed data stream by decoding bitfields and expanding them into a destination buffer using bitwise operations and memory copies.
- `parse_command_string`: Parses a string to check for a colon delimiter or uses library functions to validate and fetch data into global memory.
- `execute_conditional_callbacks`: Checks a byte in memory, executes several routines including fetching library values and finding strings in a table, and conditionally executes callbacks via a table.
- `clear_register`: The routine clears a register and returns.
- `fetch_library_values`: Calls two functions via pointers found in a table and stores the returned values into two different global memory locations.
- `reset_and_execute_callbacks`: Checks a global pointer, clears it, and then executes two functions via pointers retrieved from a table.
- `find_string_in_table`: Searches through a table of structures to find an entry that matches a specific integer and a string prefix.
- `query_status_flags`: Calls multiple functions via a table to check specific conditions and returns a value based on the results.
- `evaluate_status_flags`: Calls several functions via a table to check specific conditions and returns a boolean result based on their comparison.
- `clear_register_2`: clears a register and returns from the subroutine.
- `validate_and_query_status`: Checks status flags via a table, attempts to allocate and release memory, and executes several functions via a library pointer to validate a value.
- `compare_and_update_string`: Compares a parameter against a stored value, executes a function via a library pointer if they differ, and updates a global location with a sum of two parameters.
- `update_string_and_memory`: Compares a string with a stored version, updates it if different, and performs memory allocation and constant storage based on register values.
- `store_constant_value`: Stores a constant value into a memory location calculated from a register offset.
- `compare_and_copy_string`: Compares a provided string with a stored string and, if they differ, copies the new string into the stored location and updates a global pointer.
- `clear_and_call_function`: Checks if a global pointer is non-zero, sets it to zero, and then calls a function via a pointer in a table.
- `retry_function_call`: Attempts to call a function up to four times if it returns a negative value, then stores the result and resets several global shorts.
- `call_external_function`: Calls a function through a pointer located at a fixed offset from a global address.
- `parse_structured_data`: Parses a data structure by validating a header, iterating through a list of offsets, and copying specific word sequences into a destination buffer.
- `iterate_and_free_pointers`: Iterates through a table of pointers and calls a routine to free memory for each non-zero entry.
- `copy_limited_words`: Copies a sequence of words from a source to a destination, capping the number of words transferred at a maximum of five.
- `execute_nested_lists`: Iterates through a list of pointers and executes a function for every element in a nested linked list structure.
- `traverse_linked_list`: Iterates through a linked list by following pointers, calling a function for each element until a null pointer is encountered or a counter reaches zero.
- `allocate_and_copy_data`: Allocates a memory block based on a value from a structure and copies data from a source into the new block.
- `update_offset_value`: Checks if a parameter is within specific bounds and then updates a global variable using a calculated offset and a value from a table.
- `reset_routine_flag`: Sets a global variable to zero and returns one.
- `update_structure_fields_2`: Copies two long words from a source into specific offsets of a destination structure.
- `copy_double_word_to_structure`: Copies two consecutive 32-bit values from a source pointer into a structure at a specific offset.
- `calculate_and_allocate_buffer`: Calculates a buffer size from structure offsets, attempts to allocate memory, and performs secondary allocations based on the results.
- `fill_memory_range`: Calculates a starting address and a step value to fill a block of memory with a repeating sequence of values.
- `copy_structure_data`: Copies several fields from a source structure to a destination structure, including a pointer to the source.
- `fill_memory_block`: Fills a specified memory range with a repeated 32-bit value.
- `execute_indirect_call`: Performs an indirect call to a function address retrieved from a table using an index.
- `populate_index_table`: Allocates memory and populates a table with incrementing values retrieved from another routine until a zero value is encountered.
- `calculate_address_pair`: Calculates two values based on a global variable and a constant, then stores them in a provided pointer.
- `execute_queued_callbacks`: Iterates through a list of three pointers, executes any non-zero routine found, and clears the pointer after execution.
- `set_global_pointers`: Stores a parameter into a global variable and assigns two values from an offset of that parameter to other globals.
- `populate_data_tables`: Iterates through a series of pointers to populate a table with calculated values and bit-shifted data derived from global variables.
- `call_stored_routine`: Executes two routines and then calls a third routine using a pointer retrieved from a calculated offset in a global table.
- `execute_indirect_calls`: Executes two consecutive routines using function pointers retrieved from global variables or system tables.
- `execute_conditional_calls`: Executes a sequence of three routines using function pointers retrieved from global variables or system tables.
- `reset_and_call_subroutine`: Sets a value in a structure to zero and calls another routine four times.
- `initialize_data_block`: Sets a series of values within a memory block, including a constant and several values from registers.
- `call_multiple_routines`: Calls another routine four consecutive times.
- `populate_data_structure`: Initializes a memory block by storing various register values and calculated sums into specific offsets.
- `call_subroutine_repeatedly`: Executes another routine six times in succession.
- `call_subroutine_four_times`: Executes another routine four times in succession.
- `call_subroutine_multiple_times`: Performs four consecutive calls to another routine.
- `execute_callback_sequence`: Performs a series of calls to various routines using parameters and global pointers, including a loop that waits for a specific memory value.
- `call_routine_with_global`: Calls another routine using a pointer stored in a global variable.
- `initialize_structure_from_string`: Initializes a structure with various values and a string pointer, then calls a routine through a pointer.
- `configure_blt_parameters`: Writes specific values to memory offsets relative to a pointer and calls a block transfer routine.
- `update_blt_parameters`: Modifies two words in a structure via a pointer, calls another routine with a value from a second structure, and then updates three more words in the first structure.
- `setup_blt_transfer`: Configures hardware registers for a block transfer operation and waits for the hardware to become ready through multiple polling loops.
- `perform_blt_loop`: Sets up hardware registers for a block transfer and executes a loop that updates register values for multiple iterations.
- `copy_structure_fields`: Copies multiple fields from a table into a structure and advances a pointer within that table.
- `advance_data_pointer`: Copies multiple fields from a structure to a destination, advances a pointer within that structure, or increments and clamps a global variable.
- `increment_and_clamp_global`: Increments a global variable and handles a loop to decrement it if it reaches zero, then stores the result in another global.
- `copy_and_branch`: Copies a sequence of data from a source to two identical locations and then executes two subsequent calls.
- `copy_and_branch_alt`: Repeats a sequence of copying data to two locations before executing two function calls.
- `copy_data_and_branch`: Copies a sequence of data to two identical memory locations and then executes two indirect calls.
- `copy_data_to_code`: Copies a sequence of words from a source pointer to a destination address that is treated as code.
- `execute_single_call`: Executes a single routine via an indirect call.
- `generate_color_palette`: Calculates a new set of RGB color values by averaging pairs of colors from a source table based on a distance threshold.
- `store_three_values`: Stores three parameters into specific offsets of a memory block relative to a register.
- `build_index_table`: Clears several global variables and then populates a table with incrementing values based on a calculated size.
- `build_index_table_2`: Clears several global variables, allocates memory, and populates a table with incrementing values.
- `clear_and_release_resources`: Clears two global variables, calls another routine if a third global variable is non-zero, closes a library, and frees memory.
- `process_nibbles_to_table`: Extracts nibbles from a byte stream to construct words and stores them into a table, then conditionally calls another routine.
- `increment_and_wrap_counter`: Copies a value from one memory location to another, increments it, and resets it to zero if it exceeds one.
- `call_clamp_and_store`: This routine calls another routine and then invokes a procedure that clamps a value to a maximum of one and stores it in a global variable.
- `execute_sequence_and_set_globals`: Executes a series of routine calls, iterates through a table to call another routine, and sets several global variables.
- `clear_and_execute_conditional`: Clears two global variables and calls another routine if a third global variable is non-zero.
- `assign_global_value`: Stores a passed parameter into a global variable.
- `calculate_and_store_scaled_values`: Performs several arithmetic operations using input registers and stores the results into a structure pointed to by a register.
- `initialize_graphics_library`: Opens a graphics library, retrieves a function pointer from the library base, and assigns specific library offsets to global variables.
- `close_intuition_library`: Sets a pointer in a global structure, calls several routines, and closes a library if a global variable is non-zero.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `close_library_if_active`: Checks if a global variable is non-zero, and if so, clears it and calls a function pointer located via an offset from a system pointer.
- `allocate_and_assign_global`: Calls a routine to allocate memory, assigns the resulting pointer to multiple global variables, and executes another routine.
- `free_memory_pointer`: Checks if a pointer is non-zero, sets the value at that pointer to zero, and then calls a library function to free the memory.
- `execute_conditional_loop_calls`: Checks a condition against a global variable and a system value, then enters a loop that calls two different function pointers up to five times.
- `initialize_intuition_library`: Opens a graphics library and retrieves a function pointer from it.
- `initialize_and_call_routines`: Sets two global variables and calls another routine five times.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `conditionally_call_routines`: Checks if a global variable is non-zero and, if so, calls two routines and returns the result of the second.
- `clamp_and_store_value`: Checks if a value is greater than one, sets it to zero if so, and stores the result in a global variable before calling another routine.
- `execute_and_clear_pointers`: Clears two global variables, executes several function pointers from a table, and then iterates through a table to execute function pointers for every non-zero entry before clearing them.
- `copy_memory_block_2`: Copies a sequence of words from a source pointer to a destination pointer based on a count provided in a register.
- `initialize_system_state`: Calls a function pointer, updates several global variables with bitmasked values, performs arithmetic on existing globals, and resets several other variables to zero or specific values.
- `call_two_functions`: Sets a global variable to a specific value and then calls two consecutive function pointers from a table.
- `transfer_and_execute_functions`: Copies a block of data to a local stack, increments a global counter, executes several function pointers, decrements the counter, and then executes a series of function pointers using values from the stack.
- `reset_and_call_functions`: Stores a value in a global variable, calls two function pointers, and then clears two other global variables.
- `update_indexed_data_structure`: Increments a counter and, if certain conditions are met, fetches a pointer from a table to update several global variables and a structure's members.
- `do_nothing`: This routine is empty and performs no operations before returning.
- `populate_data_table`: Iterates through a source structure to populate a destination table with shifted and incremented values.
- `reset_counter_value`: Stores zero into a memory location relative to a provided pointer.
- `loop_until_null_2`: Repeatedly calls another routine in a loop until that routine returns zero.
- `return_from_subroutine`: The routine is empty and performs no operations before returning.
- `attempt_memory_allocation`: Checks if a provided integer is greater than zero and then attempts to allocate a block of memory of that size.
- `nop_padding`: This routine is empty and performs no operations.
- `allocate_memory_wrapper`: Checks if a parameter is at least one and then attempts to allocate memory using a system library function.
- `call_memory_check`: Calls a function pointer retrieved from a table.
- `check_available_memory`: Calls a function to check the availability of memory.
- `execute_indirect_function_2`: Loads a function pointer from a memory location and executes it.
- `release_memory_wrapper_2`: Checks if a parameter is non-zero, sets the value at a pointer to zero, and then calls a library function to free the memory.
- `release_memory_wrapper`: Checks if a parameter is non-zero and then calls a library function to free memory.
- `loop_until_null`: Repeatedly calls a routine in a loop until it returns zero.
- `null_operation`: The routine is empty and performs no operations.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `add_to_tracking_list`: Adds two values to a table and increments a counter if the table is not full.
- `remove_from_tracking_list`: Searches a table for a specific value and removes the entry by shifting subsequent elements.
- `pop_last_entry`: Decrements a counter and retrieves the last entry from a table.
- `tiny_moveq_moveq_rts`: Small routine of 3 instructions that loads 0 into d0, then loads 0 into d1, then returns.
- `copy_sixteen_bytes`: This routine performs a sixteen-byte memory transfer from one location to another.
- `copy_byte_block`: Copies sixteen bytes from one memory location to another using sequential byte transfers.
- `copy_memory_block_3`: Copies a block of memory from a source to a destination using a loop of word-sized transfers.
- `copy_memory_block_4`: Copies a large block of data from a source address to a destination address using a loop of 32-bit transfers.
- `register_interrupt_handler`: Adds an interrupt server to the system and stores a value into a memory location using a pointer.
- `write_two_words`: Stores two 16-bit values from a register into a memory location relative to a pointer.
- `clear_data_structure`: Initializes a series of memory locations with zeros or specific 16-bit values.
- `extract_joystick_flags`: Reads the mouse and joystick ports and extracts specific bit flags into a structure.
- `update_hardware_counters`: Reads the joystick/mouse fire buttons and disk status from hardware registers and modifies a structure based on comparisons.
- `update_state_counters`: Modifies various bytes and words in a structure based on comparisons between existing values and a global variable.
- `extract_bit_flags`: Extracts specific bits from a 16-bit value and stores the resulting boolean states into a structure.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `detach_interrupt_and_call`: Removes an interrupt handler, updates a global variable, and then calls another routine.
- `call_library_function`: Executes a function pointer retrieved from a system library table.
- `call_indirect_routine`: Calls a routine through a pointer stored at a specific offset from a global variable.
- `check_string_and_call`: Checks if the first byte of a string is non-zero and, if so, calls a routine through a pointer.
- `store_word_via_pointer`: Stores a two-byte value into a memory location calculated by adding an offset to a pointer held in a register.
- `set_word_zero`: Stores a zero word into a memory location relative to a pointer.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `return_immediately_3`: This is an empty routine that performs no operations and returns to its caller.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `wait_and_check_string`: Polls the DMA controller for completion and checks a null-terminated string.
- `wait_for_memory_match`: Enters an infinite loop while the value at a specific memory offset remains equal to itself.
- `wait_for_dma_stop`: Polls a bit in the DMA controller register and loops until the bit is cleared.
- `check_null_terminated_string`: Iterates through a null-terminated string by checking bytes until a zero value is encountered.
- `add_interrupt_server`: Sets a global variable and then adds an interrupt server to the system by modifying the interrupt enable register.
- `clear_register_and_return`: clears a register to zero and returns from the subroutine.
- `setup_interrupt_vectors`: Stores the addresses of this routine and a global variable into two different memory locations.
- `remove_interrupt_handler`: Removes an interrupt server and modifies a global variable before calling another routine.
- `decrement_and_call_supervisor`: Decrements a global variable and calls a supervisor routine if the value meets specific conditions.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `perform_device_io`: Configures parameters for a device operation and executes a system call to perform input/output.
- `store_parameter_to_memory`: Stores a passed parameter into a memory location calculated from a global pointer and a fixed offset.
- `call_empty_routine`: Calls another routine with a zero argument.
- `call_subroutine_via_pointer`: Loads a value from a memory location into a register and uses it as an address to call another routine.
- `shift_buffer_data`: Checks a value from a pointer and shifts a block of memory within a buffer using a memory copy operation.
- `copy_memory_block_with_offset`: Copies a block of memory from a source to a destination calculated by adding a global variable to a base address.
- `shift_memory_block`: Calculates an offset and performs a memory copy operation to shift a block of data within a buffer.
- `copy_memory_block_5`: Performs a memory copy operation by iterating through a source and destination using an index and a table.
- `update_index_and_fetch_byte`: Compares two values at a memory offset, increments an index within a specific range, and returns a byte from a table or a pointer-based lookup.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `clear_memory_block`: Clears a series of memory locations by writing zeros in a loop.
- `set_status_byte`: Checks a byte at a specific offset from a pointer and, if it is zero, sets it to a specific value before calling another routine.
- `allocate_and_initialize_structure`: Allocates a block of memory, initializes its first three words to zero, and sets a pointer to a value from a library.
- `release_and_clear_structure`: Checks if a pointer is non-zero, performs library calls to clear and release the associated resource, and then frees a memory block.
- `initialize_input_device`: Allocates a structure, initializes it with a pointer to a global variable, and configures device-related fields.
- `dispatch_event_list_callbacks`: Iterates through a linked list and executes specific function pointers stored in the list elements based on a type identifier.
- `traverse_linked_list_2`: Iterates through a linked list and executes various function pointers stored in the list elements based on a value in a register.
- `process_linked_list_nodes`: Iterates through a linked list and executes function pointers stored in each node based on a specific value.
- `initialize_data_structure`: Calls several functions to validate state and then populates a structure with provided parameters and calculated values.
- `reset_structure_fields`: Sets specific bytes and words in a structure to maximum values and calls two functions.
- `call_subroutine_with_constant`: Calls another routine while passing a specific constant value as an argument.
- `initialize_structure_fields_2_2`: Sets a byte and two long integers to their maximum values within a structure pointed to by a parameter, provided the pointer is not null, then calls another routine.
- `assign_structure_values`: If a pointer is provided, it calls a function to obtain a structure and assigns three different values to its offsets.
- `initialize_structure_fields_2`: Sets a byte to maximum value and two long integers to maximum value within a structure if the provided pointer is not null, then calls a function retrieved from a table.
- `initialize_structure_fields`: Sets specific byte and word offsets within a structure to maximum values and then calls a function retrieved from a table.
- `parse_command_string_arguments_2`: Iterates through a character buffer to identify and extract command arguments, handling whitespace, delimiters, and numeric conversions.
- `parse_numeric_command_string_2`: Iterates through a character buffer to identify and convert numeric values, handling signs, decimal points, and scientific notation.
- `parse_command_string_arguments`: Iterates through a character buffer to identify numeric values, signs, and decimal points.
- `parse_numeric_command_string`: Iterates through a character buffer to identify numeric values, signs, and decimal points.
- `parse_command_string_2`: Parses a text string to extract numeric values and command identifiers.
- `parse_command_string_3`: Parses a text string to extract numeric values and command identifiers.
- `parse_command_string_4`: Iterates through a character buffer to parse numeric values and command tokens.
- `parse_numeric_string`: Parses a string of characters into a signed integer value.
- `parse_command_string_tokens_6`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_8`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_7`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_9`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_5`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_4`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_3`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_tokens`: Iterates through a character buffer to identify specific command characters and extract associated numeric values.
- `parse_command_string_tokens_2`: Iterates through a character buffer to identify and extract specific command characters and numeric values.
- `return_error_status`: Returns a value representing a negative error state.
- `format_numeric_string_buffer_5`: Converts a signed integer into a character string, applying specific formatting flags for signs, leading spaces, or padding.
- `format_numeric_string_buffer_3`: Converts a numeric value into a string of characters, handling sign prefixes, leading spaces, and padding.
- `format_numeric_string_buffer`: Converts a numeric value into a string of characters, handling sign prefixes, leading spaces, and padding.
- `format_signed_numeric_string`: Converts a numeric value into a string representation, handling sign characters, spaces, and padding.
- `format_numeric_string_buffer_2`: Constructs a formatted string representation of a number by inserting sign characters, spaces, or decimal separators into a destination buffer.
- `format_signed_numeric_string_7`: Converts a signed integer into an ASCII string representation.
- `format_signed_numeric_string_8`: Converts a signed integer into an ASCII string representation, handling sign prefixes and zero-padding.
- `format_signed_numeric_string_6`: Converts a signed integer into an ASCII string representation.
- `format_signed_numeric_string_5`: Converts a signed integer into an ASCII string representation, handling sign characters, decimal digits, and optional scientific notation or padding.
- `format_signed_integer_string`: Converts a signed integer into an ASCII string representation within a destination buffer.
- `format_signed_numeric_string_4`: Converts a signed integer into an ASCII string representation within a destination buffer.
- `format_signed_numeric_string_3_2`: Converts a numeric value into an ASCII string by writing digits, signs, or spaces into a destination buffer.
- `format_signed_numeric_string_3`: Converts a numeric value into an ASCII string by writing digits, signs, or spaces into a destination buffer.
- `format_numeric_string_buffer_4`: Converts a numeric value into a character string by writing ASCII digits, signs, or spaces into a destination buffer.
- `format_signed_numeric_string_2`: Converts a numeric value into a string representation, handling mathematical signs, leading spaces, or zero-padding based on specific bitmask flags.
- `parse_formatted_numeric_string`: Parses a string of characters to extract numeric values, handling various sign formats, padding, and data type widths.
- `format_numeric_token_string`: Converts a numeric character buffer into a formatted string by inserting mathematical sign characters or spaces.
- `format_command_token_stream_2`: Iterates through a character buffer to identify and extract command tokens, handling numeric conversions and special character delimiters.
- `parse_command_token_stream_2`: Iterates through a character buffer to identify and extract command tokens, handling numeric conversions and special character delimiters.
- `format_command_token_stream`: Iterates through a character buffer to identify and extract command tokens, handling numeric conversions and special character delimiters.
- `copy_string_to_buffer`: Iterates through a source character buffer and copies bytes into a destination buffer pointed to by a linked list structure.
- `parse_input_stream`: Reads a stream of bytes from a source and processes them based on specific control characters.
- `parse_command_token_stream`: Iterates through a character buffer to identify and extract command tokens, handling numeric conversions and special character delimiters.
- `parse_numeric_string_2`: Iterates through a character buffer to convert a sequence of ASCII digits into a signed integer value.
- `parse_numeric_token_stream`: Iterates through a character stream to identify and convert numeric sequences into integer values.
- `parse_command_string_5`: Parses a character stream to extract numeric values and control flags.
- `parse_command_string_tokens`: Iterates through a character stream to extract numeric values and control flags.
- `return_negative_one`: Returns a value of negative one.
- `reorganize_linked_list_nodes`: Updates a linked list structure by manipulating pointers, adjusting bit flags within a status word, and performing recursive traversals.
- `update_linked_list_state`: Manipulates a structure by updating pointers, setting bit flags, and performing conditional calls based on table indices and bitwise checks.
- `update_linked_list`: Manipulates a linked list by updating pointers, setting bit flags in a structure, and advancing through nodes based on specific bitwise conditions.
- `copy_memory_word`: Copies a word from a memory location to the following word position and returns a value from a register.
- `manipulate_linked_list_node`: Modifies a linked list node by updating pointers, setting specific bit flags in a structure, and performing arithmetic on node elements.
- `update_linked_list_node`: Updates a global pointer, modifies a byte in a structure, and performs arithmetic on elements within a table before updating a pointer.
- `set_structure_flags`: Sets a specific bit in a byte within a structure and copies a value from one table index to another.
- `update_structure_pointers`: Updates several fields in a structure, including setting a value to a constant and assigning a pointer to itself plus an offset.
- `update_structure_fields`: Checks a pointer against a calculated offset and updates several fields in a structure based on the result of a function call.
- `clear_linked_list`: Iterates through a linked list calling a function pointer for each node and then clears a pointer in a global structure.
- `traverse_and_call_list`: Iterates through a linked list, calling a function pointer stored in a structure for each node, and clears a value in a global structure upon completion.
- `validate_table_index`: Checks if an index is within valid bounds and if the corresponding entry in a table is non-zero, returning a boolean result.
- `check_function_result`: Calls a function retrieved from a table and returns a boolean indicating whether the result was non-zero.
- `return_zero_2`: This routine is empty and returns a value of zero to its caller.
- `execute_conditional_table_call`: Validates an index against a table and bounds, then either calls a function via a pointer or handles an error state by updating a global word.
- `execute_conditional_function`: Calls a function via a pointer from a table and either calls another function to update a word in a structure or calls the original function with arguments.
- `call_indirect_function_4`: Calls a function via a pointer retrieved from an offset within a structure.
- `validate_table_entry`: Checks if an index is within bounds and if a corresponding table entry is valid, then conditionally calls a routine.
- `process_conditional_logic`: Checks specific bits in a structure to conditionally call a function and updates a global word based on the result of a subsequent function call.
- `execute_conditional_logic`: Checks a bit in a structure to conditionally call a function, then calls another function and updates a word in a structure if the result is negative one.
- `return_void_2`: An empty routine that returns without performing any operations.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `execute_single_instruction`: This routine consists of a single instruction.
- `return_zero`: This routine is empty and simply returns a zero value to the caller.
- `copy_memory_block_6`: Copies a specified number of bytes from a source address to a destination address.
- `parse_command_string_arguments_3`: Iterates through a character buffer to identify and extract command arguments, handling whitespace, delimiters, and numeric conversions.
- `insert_into_linked_list_2`: Creates a new list node by retrieving a pointer from a table, populates it with a value and a pointer, and inserts it into a linked list.
- `copy_string`: Copies a null-terminated string from a source to a destination buffer.
- `calculate_string_length_2`: Calculates the length of a null-terminated string by iterating through its characters.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_data_structures`: Performs multiple function calls to load data, updates various values in a structure based on return values, and performs conditional logic to set boolean flags.
- `execute_version_string_logic`: A routine consisting of 329 instructions that calls an indirect function and references text containing version information.
- `clear_and_load_library`: Clears a block of memory and attempts to load a library, calling an alert routine if the load fails.
- `return_void`: This routine performs no actions and returns.
- `initialize_manx_header_structure`: Constructs and populates a data structure by writing a specific four-byte magic signature and setting various flags and offsets.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `call_indirect_function_2`: Calls a function through a pointer retrieved from a table and, if the result is non-zero, calls another function using the returned value and two parameters.
- `populate_data_list`: Counts the elements in a linked list and then populates a new structure with those elements and several values from global memory.
- `parse_executable_hunks`: Iterates through a memory structure to identify and process specific data segments by comparing values against AmigaDOS hunk type codes.
- `copy_d0_to_d5`: Copies the contents of data register zero to data register five.
- `parse_string_list`: Iterates through a linked list of strings, skipping leading whitespace and splitting each string into individual tokens delimited by spaces.
- `traverse_icon_structure`: Navigates a hierarchy of pointers through multiple function calls to extract and store specific values into a structure.
- `execute_indirect_function`: Loads a function pointer from a memory location calculated using a global offset and executes it with a provided parameter and a stack pointer.
- `call_indirect_function_3`: Executes a function pointer retrieved from a memory location using a provided parameter.
- `insert_into_linked_list`: Retrieves a pointer from a library call and inserts it into a linked list by updating a global pointer and the new node's next pointer.
- `compare_memory_blocks`: Compares two memory blocks byte by byte up to a specified length and returns a result indicating if they are equal or which block is smaller.
- `tokenize_string_into_array`: Parses a string by splitting it into individual tokens based on whitespace or quoted substrings.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_with_limit`: Finds the end of a destination string and copies characters from a source string until a limit is reached or a null terminator is encountered.
- `copy_string_or_pad`: Copies characters from a source to a destination up to a limit, or fills the destination with null bytes if the limit is zero.
