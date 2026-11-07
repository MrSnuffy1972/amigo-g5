# Game 433 (Wing Commander): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00433). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 78 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 257 of 257 named (0 author symbols), 257 of 257 described (40 by rule, 46 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1639 of 1639 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7427 of 7436 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23830 |
| code | 22524 |
| data | 2569828 |
| padding | 18058 |
| unused | 69120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Setmap | 13 | 653 | 31 |
| Else | 2 | 117 | 2 |
| SetPatch | 24 | 833 | 45 |
| EndCli | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| Endif | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| setpatchV38.31 | 2 | 893 | 76 |
| Version | 11 | 574 | 18 |
| d | 1 | 13 | 1 |
| gb | 1 | 4 | 1 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:InstallWingCommander | 2 | 3734 | 67 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:Wing | 8 | 45 | 2 |
| Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:PLANET0.PKT: high-entropy data file (7.70 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:ENDGAME.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:FONTS.PKT: high-entropy data file (7.88 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:PCSHIP03.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:PCSHIP01.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:MIDGAME2.PKT: high-entropy data file (7.94 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:DEMO.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:MIDGAME0.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:PCSHIP02.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:DEMOSAMP.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:MIDGAME3.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:PCSHIP00.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:MIDGAME1.PKT: high-entropy data file (7.91 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:TALKING.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:SCRAMBLE.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:BRIEFING.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:RECROOM.PKT: high-entropy data file (7.97 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:MAINSONG.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:english0.pkt: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:PCSHIP04.PKT: high-entropy data file (7.77 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:MODULE0.PKT: high-entropy data file (7.64 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf:MAINSAMP.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP09.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP07.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:ASTEROID.PKT: high-entropy data file (7.96 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:GAMESAMP.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP12.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:GAMESONG.PKT: high-entropy data file (7.92 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP03.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP10.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP01.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:WINGMEN.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP17.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP08.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP15.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:EXTRA.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP06.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP13.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:COCKPIT.PKT: high-entropy data file (7.94 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SCALEFNT.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP21.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP11.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:PILOT.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP02.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP29.PKT: high-entropy data file (7.93 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:OBJECTS.PKT: high-entropy data file (7.95 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP00.PKT: high-entropy data file (7.98 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf:SHIP18.PKT: high-entropy data file (7.99 bits/byte), classified as packed data
- wing commander (1992)(mindscape)(disk 2 of 3)/Wing Commander (1992)(Mindscape)(Disk 2 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- wing commander (1992)(mindscape)(disk 3 of 3)/Wing Commander (1992)(Mindscape)(Disk 3 of 3).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- wing commander (1992)(mindscape)(disk 1 of 3)/Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:Wing: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Setmap

- `program_startup_entry`: Program entry point: finds its own task, reads the startup message, opens dos.library and the NIL: stream, and allocates the main state block.
- `release_patch_session_resources`: Swaps the incoming value into a slot in a per-task record and returns the previous value.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `propagate_map_update_calls`: Calls a helper with a stored value and, when a flag is set, the address of a global, then calls the helper again with other stored values.
- `store_pair_and_invoke_helpers`: Saves its two parameters into global variables, then calls a helper with a fixed argument list that includes a global address and the constants 440 and 60, and finally calls another helper with the value 20.
- `resolve_keymap_name_and_open`: Chooses a keymap name from the icon tool types, a command-line argument or the default, and converts it to lower case.
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
- `pass_params_to_subroutine`: Calls another routine while passing a global variable, a parameter, and the address of an eight-byte stack slot.
- `call_library_function_minus_348`: Calls a library function through a base pointer held in a global variable, using a fixed offset of minus 348 into its jump table, and returns with no result.
- `format_message_then_emit`: Formats text into a 140-byte stack buffer using a format routine from a global function slot, measures the length of the result, and passes the text and its length to another routine when it is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Else

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_longs_then_call_handler`: Copies sixteen 32-bit values from a fixed offset in a base structure into a caller-supplied buffer when that pointer is non-null.

### SetPatch

- `start_program_parse_arguments`: Entry routine that opens the DOS library and, when started from the CLI, splits the command-line text into argument strings, honouring quotes and star escapes.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `run_flagged_forward_then_finish`: Runs one helper unconditionally, then, if a global flag is set, passes the incoming parameter to a helper and feeds its result to a second helper.
- `install_setpatch_patches`: Checks the argument count and the first argument, printing usage text when the option is wrong.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_and_return_result`: Calls the function whose address is stored at a fixed offset in a structure held in a global variable, and returns the value that call produces.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_function_minus_198`: Calls a library function through the library base held in a global variable, at a fixed offset of minus 198 in its jump table, and returns with no result.
- `call_library_function_minus_294`: Calls a library function through the library base held in a global variable, at a fixed offset of minus 294 in its jump table, and returns with no result.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_function_minus_414`: Calls a library function through the library base held in a global variable, at a fixed offset of minus 414 in its jump table, and returns with no result.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_function_minus_552`: Calls a library function through the library base held in a global variable, at a fixed offset of minus 552 in its jump table, and returns with no result.
- `allocate_and_install_code_stub`: Checks a memory size and a flag in a global structure, and when they qualify it allocates a block of memory under Forbid and Permit and copies a small code stub into it.
- `update_checksum_then_dispatch`: Recomputes the one's-complement checksum over a 23-word array in a structure, then calls a routine through a jump-table entry.
- `disable_interrupts_around_calls`: Checks that disk.resource is present and that a record's size field is small, then disables interrupts through the interrupt enable register and raises a nesting count.
- `copy_field_when_check_passes`: Calls the routine held in a global pointer and, if it returns a non-zero result, copies a 32-bit field from a fixed offset in a global structure into a global variable.
- `call_handler_when_value_matches`: Compares a saved global value with a field of a global structure.
- `call_handler_if_minus_one`: Calls the routine held in a global pointer only when its input value is minus one, and returns without doing anything otherwise.
- `disable_interrupts_and_dispatch`: Disables interrupts through the interrupt enable register and, on the first run, writes a four-letter marker and saved values into low memory.
- `call_jump_table_entry_136`: Calls the routine at a fixed offset of 136 in a jump table held in a global variable, and returns with no result.
- `fill_sixteen_vector_slots`: Stores a value computed from a pointer at a low-memory address into a global variable, then fills sixteen consecutive low-memory slots, starting at a fixed address, with successive addresses inside one routine.
- `run_sixteen_stub_chain`: Calls a chain of sixteen stub routines in sequence and returns the result of the last call.
- `return_input_value_2`: Returns the value passed in the data register unchanged.
- `run_fifteen_stub_chain`: Calls a chain of fifteen stub routines in sequence and returns the result of the last call.
- `return_input_value`: Returns the value passed in the data register unchanged.
- `execute_conditional_call`: Checks if a value in a global variable is below a certain threshold and, if so, calls a function through a pointer located at a fixed offset from a global address.
- `allocate_copy_pair_table`: Allocates a block sized from a count word and copies a list of value-and-size pairs into it.
- `free_counted_blocks_return_flag`: Calls a library routine at offset minus 210 once for each remaining count and then once more, and returns a word read from a pointer with its top bit set.
- `release_block_return_flag`: Calls a library routine at offset minus 210 once, then returns a word read from a pointer with its top bit set.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it does not access memory, registers, or call other code, and immediately returns to its caller.
- `invoke_check_then_two_handlers`: Passes a field at offset 68 of a record to a check routine held in a global pointer.
- `tiny_clr_movea_rts`: Small routine of 3 instructions that clears d0, then copies (a7)+ to a2, then returns.
- `scan_for_marker_word`: Walks forward through an array of 16-bit words, one word at a time, until it finds a word whose high byte is 97, and returns a pointer to that word.
- `advance_past_encoded_entry`: Decodes a branch displacement at the given address, using a short signed value when the low byte is non-zero and a 16-bit value in the next word when it is zero.
- `compute_word_branch_target`: Reads a signed 16-bit displacement stored two bytes after the given address and returns the address formed by adding that displacement to the address two bytes after it.
- `store_three_decoded_addresses`: Walks a chain of address calculations starting from a given address, moving two bytes past each result, and stores the three resulting addresses in global variables.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_function_minus_60`: Calls the library routine at offset minus 60 through a base pointer held in a global variable, and returns with no result.
- `forward_pair_with_stack_slot`: Forwards its two parameters to another routine together with the address of a stack slot, and returns with no result.
- `format_message_and_output`: Formats a message into a 140-byte stack buffer with the format routine held in a global function slot, then measures the text.
- `store_low_byte_at_pointer`: Writes the low byte of the data register to the byte location addressed by the address register, and returns.

### EndCli

- `scan_list_and_install_handler`: Walks a linked list of entries through library calls to find the largest value they reference, then fills a table with a fixed pattern of longs.
- `copy_and_call_routine`: Copies a sequence of words from a structure to a stack location and then calls another routine.

### Echo

- `initialize_task_structure`: Traverses a linked list to find a maximum value and populates a data structure with a sequence of incrementing values.
- `copy_data_and_call_function`: Copies sixteen words from a memory location to the stack and then calls a function via a pointer.

### FastMemFirst

- `start_program_parse_arguments_2`: Entry routine that opens a library and, when started from the CLI, splits the command-line text into argument strings, honouring quotes and star escapes.
- `find_fastmem_and_chip_blocks`: Walks a linked list of memory blocks, choosing the first block with type tag 5 in the fast-memory address range and the first block with type tag 3 below a fixed address limit.
- `call_library_function_minus_132`: Calls the library routine at offset minus 132 through a base pointer held in a global variable, and returns with no result.
- `call_library_function_minus_138`: Calls the library routine at offset minus 138 through a base pointer held in a global variable, and returns with no result.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.

### Endif

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `initialize_task_structure_2`: Traverses a linked list to find a maximum value and populates a data structure with a sequence of incrementing values.
- `copy_stack_data`: Copies a block of sixteen bytes from a source pointer to a destination pointer and calls another routine.

### setpatchV38.31

- `apply_patch_list_with_switches`: Program entry that checks the system version, then applies a list of patch records according to their type codes, printing the patch list and messages such as already installed.
- `allocate_and_copy_patch_stub`: When a library check passes and a flag bit is set, it allocates a memory block and copies a 51-word code stub into it, then calls library routines to install the stub.
- `copy_patch_stub_when_flagged`: When a flag bit is set, it allocates a memory block and copies a 51-word code stub into it, then calls library routines to install the stub.
- `allocate_copy_stub_and_install`: Allocates a memory block and copies a 51-word code stub into it, then calls several library routines to install the stub.
- `probe_memory_size_and_mask`: Checks a low-memory byte and, if it is not a fixed value, probes memory downward from a fixed high address to find where the self-addressing test stops matching.
- `free_patch_slots_clear_fields`: Clears a pointer field at offset 372 in a record and frees it through the library routine at offset minus 210 if it was set.
- `reply_completed_request_message`: Handles a request message by checking its command code, copying a value into the reply field and clearing flags for one code, or finishing the request when a pending field is zero for another.
- `handle_request_message`: Processes a request by checking specific field values and copying data between offsets, then calls another routine.
- `reply_when_type_four_request`: Checks a type byte in a request's linked record.
- `reply_when_pending_field_clear`: Replies to the message through the library routine at offset minus 378 when a pending field is zero, after copying a value into a result field and clearing a flag.
- `call_two_cia_routines`: Calls two other routines in turn and returns zero.
- `relink_flagged_list_entry`: Looks up an entry through a library routine at offset minus 276 and, if its top flag bit is set, unlinks it, clears the flag and relinks it between two library calls that disable and enable the system.
- `forward_to_single_routine`: The pseudo-code for this two-instruction routine could not be produced, so the routine is known only to call one unnamed routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_jump_vector_stub`: Calls the routine reached through a jump-table entry, which passes control on to a fixed address, and returns with no result.
- `save_console_lookup_result`: Calls the library routine at offset minus 96 and, when it returns a pointer, saves that pointer, the 32-bit value at offset 6 inside it and the incoming base pointer into three global variables.
- `check_range_then_call_pair`: Checks whether a stack value lies within a range held in two globals, and if so calls a library routine at offset minus 576, then makes two more calls when that returns non-zero, the second at offset minus 570.
- `forward_to_unresolved_handler`: Passes control to a handler whose address is held in a global variable.
- `check_limit_then_call_pair`: Compares a data register value with a global limit, and when it is not above the limit calls a library routine at offset minus 576 and, if that returns non-zero, makes two more calls, the second at offset minus 570.
- `store_offset_adjusted_lookup`: Chooses one of two constant offsets depending on whether a field equals 44, then calls the library routine at offset minus 96.
- `clear_entry_when_value_matches`: Uses the value in the data register to index a table and compares the entry at a fixed offset with a stored global value.
- `set_list_values`: Checks if a value at a calculated offset matches a specific routine and, if so, sets two longwords to all ones.
- `clear_table_entries_on_match`: Compares a value in a table against another routine and, if they match, sets two adjacent memory locations to a specific constant.
- `execute_indirect_pointer`: Loads a function pointer from a global variable and executes it.
- `disable_one_interrupt_source`: Calls two library routines at fixed offsets, writes the value 64 to the interrupt enable register to switch off one interrupt source, then calls a third library routine and returns one.
- `store_argument_and_return_one`: Saves the address-register input into a global variable and returns one.
- `fill_block_on_sentinel_change`: Calls the routine held in a global pointer and compares its result with a value saved in a table indexed by the input word.
- `update_table_entry`: Compares a value from another routine against a table, updates the table if they differ, and fills a memory block with a pattern if a specific condition is met.
- `fill_memory_pattern`: Updates a table with a value from another routine and fills a memory block with a pattern if the value matches a specific constant.
- `update_table_and_fill_memory`: Compares a value from another routine against a table, updates the table if they differ, and fills a memory block with a pattern if the value matches a specific constant.
- `add_lookup_result_to_cell`: Calls the library routine at offset minus 96 and, if it returns a non-zero value, adds that value into a fixed global cell twice.
- `copy_field_on_matching_codes`: Compares a stack value and an input word with two fixed codes.
- `copy_field_on_matching_code`: Compares an input word with a fixed code.
- `clear_flag_bit_then_call`: Takes a flag word from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `search_list_for_matching_word`: Returns the input value unchanged when the address input is non-zero.
- `search_linked_list`: Traverses a linked list to find an entry matching a specific word and returns a value based on the result.
- `search_linked_list_alt`: Iterates through a linked list to find a matching word and returns a value based on whether a match was found.
- `traverse_linked_list_and_execute`: Iterates through a linked list to find an entry matching a specific short integer value.
- `search_list_for_matching_word_2`: Returns the second address input unchanged when it is non-zero.
- `find_list_entry`: Searches a linked list for a matching word and returns a value based on the search outcome.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `search_list_or_return_two`: Searches a list for an entry whose word field matches the low word of the input, between library calls at offsets minus 132 and minus 138.
- `search_linked_list_and_return_two`: Iterates through a linked list to find a matching word and returns the value two if no match is found.
- `search_list_or_minus_one`: Returns minus one when the input is minus one.
- `search_linked_list_and_return_minus_one`: Searches a linked list for a matching word and returns minus one if the input is minus one or if no match is found.
- `traverse_list_match`: Performs a search through a linked list for a specific word and returns a value based on the search result.
- `search_linked_list_and_return_zero`: Traverses a linked list to find an entry matching a specific word and returns zero if no match is found.
- `search_list_or_return_argument`: Searches a list for an entry whose word field matches the low word of the input, between library calls at offsets minus 132 and minus 138.
- `call_release_then_select_value`: Calls the library routine at offset minus 138, then returns the stacked value when the address register is non-zero and the third parameter otherwise.
- `return_parameter`: Returns the input parameter unchanged.
- `scan_table_for_stored_word`: Takes a pointer from a library lookup when a configuration bit is set, or from a fixed low-memory address otherwise.
- `call_library_vector_636`: Library stub: loads a library base from a global variable and calls the library function at offset minus 636 of that base, returning its result to the caller.
- `dispatch_on_marker_word`: Reads a 16-bit word at a pointer held in a global variable.
- `dispatch_conditional_routine`: Checks a 16-bit word at a pointer held in a global variable and calls another routine if the value does not match a specific constant.
- `forward_to_next_routine`: Calls one further routine and returns with no result, adding no other work.
- `copy_text_into_prefixed_buffer`: Calls a library routine at offset minus 228 and, when it returns non-zero, allocates a buffer with AllocVec sized for the measured input text, then copies the text into it behind a length byte and passes the buffer to a routine at a fixed offset in a base structure.
- `copy_prefixed_text_in_place`: Measures the text at the address register, copies it backwards to a destination ending at a fixed offset from the data register, and writes its length as a leading byte.
- `copy_prefixed_text_in_place_2`: Copies text backwards into a destination buffer and writes its length as a leading byte, then zero-terminates the buffer named by a structure field.
- `return_immediately_4`: The routine performs no operations and returns to its caller immediately.
- `call_library_then_return_inputs`: Calls the library routine at offset minus 306 and then returns the two data-register values that were passed in, ignoring any result from the call.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `process_queue_and_decrement_counters`: Updates several counters in a structure and processes a linked list of entries, potentially calling a library routine for each flagged entry.
- `decrement_and_call_library`: Decrements a value in a structure or calls two library routines depending on a boolean flag.
- `alert_and_init_semaphore`: Calls the exec Alert routine and then the InitSemaphore routine, then calls the library routine at offset minus 138 and returns.
- `drain_flagged_queue_entries`: Takes the first entry off a queue and, while entries have a flag bit set, passes them to a library routine at offset minus 324 and unlinks the next one.
- `drain_flagged_queue_entries_2`: Walks a queue from its head and, for each entry whose flag bit is set, unlinks it and calls the library routine at offset minus 324 on it.
- `increment_counts_then_permit`: Increments a counter byte in a global structure and a count field in a record.
- `clear_flag_then_call_library`: If a pointer field in a record is set, it clears one flag bit in a structure reached through a global pointer and calls library routines at offsets minus 240 and minus 318, then the library routine at offset minus 138.
- `call_handler_when_stack_outside`: Calls the routine held in a global pointer when the stack pointer lies outside a range recorded in a structure, or when a field of another structure equals minus one.
- `call_handler_when_stack_outside_2`: Calls the routine held in a global pointer when the stack pointer lies outside a range recorded in a structure, or when a field of another structure equals minus one.
- `call_if_stack_out_of_bounds`: Checks if a pointer derived from a structure is outside a specific stack range and calls another routine if it is.
- `check_stack_bounds_and_call`: Checks if a pointer is within a specific stack range or if a value in a structure is minus one, then calls another routine if these conditions are met.
- `store_pointer_pair_return_one`: Stores the address-register input and a second register value into two global variables and returns one.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `forward_to_single_routine_2`: The decompiler timed out on this three-instruction routine, so its pseudo-code is unavailable.
- `call_unnamed_routine_undecompiled`: The pseudo-code for this 11-instruction routine could not be produced because the decompiler timed out.

### Version

- `start_program_parse_arguments_3`: Entry routine that allocates a message record and, when started from the CLI, splits the command-line text into argument strings, honouring quotes and star escapes.
- `process_message_reply`: Handles message processing by updating message fields, calling various functions based on flags, and returning a value from the message structure.
- `check_library_version`: Parses the command line for a library or device name, an optional version and revision, and an optional unit.
- `call_with_flag_then_forward`: Calls a routine with the first parameter and the constant 128, then passes the second parameter to another routine.
- `parse_integer_string`: Converts a decimal string to a signed integer, accepting an optional plus or minus sign and reading digits until the first non-digit.
- `compare_strings_ordering`: Compares two null-terminated strings byte by byte and returns minus one, one or zero depending on whether the first string sorts before, after or equal to the second.
- `measure_string_length`: Returns the number of bytes before the terminating zero in a null-terminated string.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatch_function`: Calls a routine through a pointer stored in a global variable.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_function_minus_198_2`: Calls the library routine at offset minus 198 through a base pointer held in a global variable, and returns with no result.
- `call_library_function_minus_210`: Calls the library routine at offset minus 210 through a base pointer held in a global variable, and returns with no result.
- `call_library_function_minus_414_2`: Calls the library routine at offset minus 414 through a base pointer held in a global variable, and returns with no result.
- `call_library_function_minus_444`: Calls the library routine at offset minus 444 through a base pointer held in a global variable, and returns with no result.
- `call_library_function_minus_450`: Calls the library routine at offset minus 450 through a base pointer held in a global variable, and returns with no result.
- `call_library_function_minus_552_2`: Calls the library routine at offset minus 552 through a base pointer held in a global variable, and returns with no result.
- `format_message_and_output_2`: Formats a message into a 140-byte stack buffer with the format routine held in a global function slot, then measures the text.
- `store_byte_to_pointer`: Stores a byte from a register through a pointer.

### d

- `stub_bad_instruction_data`: The decompiled code is bad instruction data: it ORs a byte into memory and then stops, so there is no meaningful routine to describe.

### gb

- `terminate_execution`: The routine is empty and contains no valid instructions.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:InstallWingCommander

- `build_quoted_command_and_run`: Zeroes a block of about 2,400 longs, allocates working memory when the stack is too small, and reads state from the current task and library base.
- `release_install_task_resources`: Saves its incoming value in a per-task record, then conditionally calls cleanup routines and finally closes a library.
- `run_install_step_sequence`: If the stack has enough room, runs a sequence of helper steps that look up and copy values and call library routines, returning zero on success or twenty on failure.
- `check_disk_space_for_install`: Allocates a memory block and locks a directory to read its disk information through library calls, then compares the free space with a required size.
- `find_entry_by_code`: Scans a table of 16-byte entries and returns the first whose code field equals the parameter and whose flag byte is non-zero, or zero if none match.
- `process_install_table_entries`: Walks a table of 16-byte install entries and, for entries with code 2 or 4, looks up a name and performs a copy and library calls, stopping at the first failure.
- `copy_file_in_chunks`: Copies a file in blocks of up to 8192 bytes, reading each block and writing it out, then sets protection on the copy.
- `copy_name_and_lookup`: Copies a name string into a fixed buffer, calls helpers to look it up and process it, and returns a flag that is one when the lookup succeeds and zero otherwise.
- `create_directory_path`: Walks a path one component at a time, locking each directory and creating any that do not exist, using a 256-byte scratch buffer.
- `return_immediately`: Executes a return immediately with no other work.
- `return_immediately_2`: The routine executes a return instruction immediately upon entry, performing no other operations despite the instruction count suggesting additional code may be present or filtered.
- `free_install_memory_lists`: Frees two linked lists of memory blocks by calling the library free routine at offset minus 210 on each node, and clears the head of the first list.
- `split_path_into_components`: Splits a slash-separated path into components by writing a zero byte after each separator and recording where each component starts, for up to 16 components.
- `split_path_into_parts`: Separates a path into a volume part before a colon, a directory part before the last slash, and a file-name part, copying each into the output buffers that were supplied.
- `append_separator_if_missing`: Appends the separator byte held in a global variable to a string when the string is non-empty and does not already end in a slash, colon or backslash.
- `copy_string_to_buffer`: Copies bytes from the source address to the destination address until a zero byte is copied, and returns the start of the destination.
- `convert_integer_to_string`: Converts an integer to decimal digits by repeatedly dividing it, storing each digit as a character.
- `convert_octal_to_string`: Converts an unsigned value to octal digits by repeatedly taking its low three bits.
- `convert_hex_to_string`: Converts an unsigned value to hexadecimal digits using a lookup table of characters.
- `parse_signed_integer`: Reads a signed decimal integer from the string at the address register, including an optional sign, and stores it through the second address.
- `allocate_file_handle_record`: Checks a flag bit in a file record and, when it is clear, calls a routine to obtain a buffer and sets the record's fields from a global value.
- `convert_string_to_uppercase`: Converts each letter of a zero-terminated string to upper case in place, using a character-class table to decide which bytes are lower-case letters.
- `write_to_file_record`: Writes data to a file record after checking the record's flags.
- `read_text_without_carriage_returns`: Reads from a file record into a buffer, then copies the data back in place without carriage-return bytes and returns the number of bytes kept.
- `read_and_strip_newlines`: Reads data from a source, strips carriage return characters, and returns the length of the processed data.
- `seek_file_record`: Seeks within a file record to the position given by the mode parameter, storing the DOS error code when the seek fails.
- `close_file_record`: Closes a file record by calling the DOS close routine when it is open, releasing its buffers when they are present, and clearing its handle.
- `open_file_record`: Adds a file record to a linked list, copies the file name into it and translates the access-mode flags into record flags.
- `normalize_fixed_point`: Divides a 32-bit fixed-point numerator by a denominator, shifting both according to the denominator's magnitude, and returns a 64-bit result made of the quotient and the scaled remainder.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values by combining their 16-bit halves, cross terms and low-half product, and returns the 32-bit result.
- `return_immediately_3`: The routine is empty and performs no operations before returning.
- `close_all_file_records`: Walks the list of open file records and closes each one that needs it, calling the DOS close routine or a stored callback depending on the record's flags.
- `parse_command_line_open_streams`: Splits the command-line text into up to 32 argument strings, honouring quotes and star escapes, then sets up the input and output streams, falling back to the NIL: device.
- `copy_text_and_run_handler`: Copies up to 79 characters from a stored string into a local buffer and, when an output stream is open, writes it with a newline.
- `process_break_signal`: Copies a string into a stack buffer, checks for a specific task or signal, and updates a global variable based on the result of a library call.
- `find_file_record_by_index`: Returns the file record at the given index in the linked list, walking the list from its head.
- `poll_break_signal_and_respond`: Reads the task's signal flags with the exec SetSignal routine and, when a break bit is set and the state is not one, writes the stored message to the output stream followed by a newline.
- `find_byte_in_buffer`: Scans a buffer of the given length for a target byte and returns a pointer to the first match, or zero if the length is used up without a match.
- `append_string_to_buffer`: Appends a zero-terminated source string to the end of a destination string, copying no more than the given count of bytes and writing a terminating zero.
- `append_string_unbounded`: Finds the terminating zero of the destination string and copies the source string after it, including its terminator, then returns the destination address.
- `copy_string_zero_padded`: Copies a zero-terminated source into the destination up to a count of bytes, then pads the rest of the count with zero bytes.
- `copy_byte_block_safe`: Copies a block of the given length from source to destination, copying backwards when the regions overlap so that no bytes are lost.
- `free_pool_block`: Frees a block previously obtained from the memory pool by locating its pool list entry, clearing its in-use bit and merging it with free neighbours.
- `unlink_file_record`: Searches a linked list of structures to find a specific entry based on a provided pointer and offset.
- `allocate_pool_block`: Allocates a block of at least the requested size from the memory pool, first searching existing pool chunks for a free region and otherwise taking a new chunk from the system.
- `allocate_memory_block`: Searches a linked list of memory pools for a free block of a specified size and splits the block if it is larger than requested.
- `run_cleanup_callbacks_then_close`: Runs each callback in a stored list, calls a helper for the entries that need it, and sets a global flag.
- `append_byte_to_buffer`: Appends a byte to a buffer and keeps its used count in a structure field.
- `call_and_return_flag_status`: Calls a routine with the incoming parameter and a stack slot, then returns minus one when a flag field in a global structure has bit 4 or bit 5 set, and zero otherwise.
- `read_line_into_buffer`: Reads characters into the buffer until a newline or the end of input, then terminates the buffer with a zero byte.
- `read_next_input_byte`: Returns the next byte from an input stream, refilling its buffer through helper calls when it is empty and recording end-of-file or error flags.
- `get_file_record_position`: Returns the current position of a file record, adjusting for buffered data and for newline conversions in text mode.
- `calculate_file_offset`: Calculates a file offset by counting newline characters in a buffer and adjusting the result based on specific flags and a length field.
- `reset_file_record_buffers`: Clears the buffer position, count and status fields of a file record, and flushes it first when a flag is set.
- `copy_chars_into_record_buffer`: Copies up to a count of characters from the source into a file record's buffer after checking the record's flags.
- `copy_data_to_buffer`: Copies characters from a source to a destination buffer while checking structure flags and handling newlines.
- `read_bytes_from_record`: Reads up to a requested count of bytes from a file record into a buffer, copying from its internal buffer and refilling it through helper calls.
- `update_file_node_state`: Resets the position and status fields of one file record, or when given no record, checks each record in the list.
- `finish_file_record_close`: Closes one file record by flushing pending output when the record is writable and then clearing its handle field.
- `release_structure_if_active`: Returns -1 when the pointer is null or its active field is zero.
- `open_stream_from_mode_string`: Parses a two-character access mode made of read, write, append and optional plus signs into flag bits, then calls another routine to open the file.
- `find_or_allocate_free_node`: Walks a linked list hanging off a global structure looking for a node whose count field is zero.
- `format_one_conversion_specifier`: Handles one printf-style conversion: it reads flags, width, precision and length modifiers, converts an integer, character, string or pointer argument into text, and pads it.
- `emit_format_string_characters`: Walks a printf-style format string and passes each literal character to an output routine, treating a doubled percent sign as one literal.
- `delete_file_save_error_code`: Calls the DOS library to delete a file.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `forward_to_routine`: Calls one other routine and returns, with no other work on data, registers or memory.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:Wing

- `program_entry_setup_sequence`: Program entry point.
- `decrement_until_minus_one`: Decrements a 16-bit variable repeatedly until it wraps to negative one, then returns.

### Wing Commander (1992)(Mindscape)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
