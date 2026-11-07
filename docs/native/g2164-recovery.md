# Game 2164 (ambermoon (1994)(thalion)(disk 01 of 10)(a)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02164). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (932752 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 40 of 40 units identical (bytes and relocations), 19 raw-word lines |
| Functions named and described | pass, 492 of 492 named (0 author symbols), 492 of 492 described (87 by rule, 151 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2422 of 2422 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14588 of 14598 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 78585 |
| code | 47924 |
| data | 497282 |
| padding | 13025 |
| unused | 295936 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Keymap | 1 | 4 | 1 |
| Run | 2 | 117 | 2 |
| Install | 3 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| Avail | 11 | 376 | 12 |
| Path | 2 | 117 | 2 |
| Resident | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Rename | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| NewShell | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| Makedir | 2 | 117 | 2 |
| List | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| LoadWB | 12 | 537 | 30 |
| Version | 11 | 574 | 18 |
| SetMap | 13 | 674 | 30 |
| More | 19 | 2290 | 76 |
| Format | 30 | 2009 | 76 |
| CLI | 11 | 607 | 25 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:AM2_BLIT | 11 | 45 | 2 |
| d | 1 | 13 | 1 |
| gb | 1 | 4 | 1 |
| Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:AM2_CPU | 10 | 45 | 2 |
| Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Ambermoon | 8 | 253 | 7 |
| Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Ambermoon_install | 7 | 253 | 7 |
| Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1129 | 36 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3138 | 124 |

## Cases handled

- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 517 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1406 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ambermoon (1994)(thalion)(disk 01 of 10)(a)/Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:AM2_BLIT: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1994)(thalion)(disk 01 of 10)(a)/Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:AM2_CPU: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1994)(thalion)(disk 01 of 10)(a)/Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Ambermoon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ambermoon (1994)(thalion)(disk 01 of 10)(a)/Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Ambermoon_install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `install_game_files_to_destination` (runtime segment an address): Builds destination paths and copies the game's program, icon, keymap and document files into a chosen directory, then shows an installation message. Kind: Path-entry protection: requires a valid drive path with a ':' or '/' separator, else aborts. Pass path: loop continues past the 'N'/'Y' prompt (valid path accepted).

## Functions


### Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Keymap

- `decrement_pointer_value`: Decrements the 16-bit value stored at the address held in register A6 by 256, then returns.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: Copies sixteen 32-bit words from a fixed offset in the base structure into the buffer passed in, when that buffer is not null, then calls a routine.

### Install

- `fill_table_from_max_entry`: Finds the largest value stored in a linked list of entries, sets up a 140-byte stack buffer, and fills a table sized from that maximum with a repeating pair of words before running the remaining setup steps.
- `copy_block_and_call`: Copies a fixed-size block of words from one memory location to another when the destination is valid, then invokes another routine.

### Else

- `find_max_and_copy_buffer`: Walks a linked list of entries to find the maximum value in a field, then, when a condition holds, writes that value and a run of related words into a header and copies sixteen 32-bit words into a buffer.
- `copy_entries_from_table`: Copies a fixed number of words from a source table into a caller-provided buffer when that buffer is not null, then invokes a routine via a stored pointer.

### Delete

- `find_max_and_prepare_buffer_10`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes a small header into a buffer and copies sixteen 32-bit words from a fixed offset in the base structure into it before invoking another routine.
- `copy_block_and_call_2`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Avail

- `allocate_message_and_open_dos`: Allocates a message structure through the system library, reads the current task and the current directory, opens the dos library and waits for a reply message.
- `reply_and_release_message`: Finds the current task and exchanges its stored message pointer with the argument.
- `sum_memory_region_sizes`: Walks a list of memory-region entries and sums the sizes of those whose attribute bits match the mask passed in.
- `print_memory_usage_table`: Parses a memory type selector from the command arguments, queries the available, in-use, maximum and largest memory figures through the system library, and prints a table with the headings Type, Available, In-Use, Maximum and Largest.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_with_stack_slot_pointer`: Passes its two arguments and a pointer to a fixed stack slot to a routine, returning nothing.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `format_and_emit_stack_text`: Formats text through the system formatting entry into a 140-byte stack buffer, measures the resulting string, and passes the buffer and its length to an output routine when the string is not empty.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Path

- `find_max_and_prepare_buffer_11`: Walks a linked list of entries to find the maximum value in a field, then stores that maximum and a small header into a buffer.
- `copy_block_if_valid`: Copies a fixed block of sixteen 32-bit values from a source table pointed to by a register into a destination buffer passed as an argument, repeating the copy until the counter reaches zero.

### Resident

- `find_max_and_prepare_buffer_12`: Walks a linked list of entries to find the maximum value stored in a field, then writes that value into a header and fills a block of entries with a repeating pattern.
- `copy_words_and_call_3`: Copies a fixed number of words from one memory location to another when the destination is not null, then invokes another routine.

### Assign

- `find_max_and_prepare_buffer_13`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes a small header into a buffer and copies sixteen 32-bit words from a fixed offset in the base structure into it.
- `copy_block_and_call_3`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination pointer is valid, then invokes another routine.

### EndCLI

- `find_max_and_prepare_buffer_14`: Walks a linked list of entries to find the maximum value in a field, then writes a small header structure into a buffer.
- `copy_block_and_call_hook`: Copies a 16-word block from a source table into a destination buffer when the buffer pointer is valid, then invokes a routine stored in a global slot.

### Rename

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes that maximum plus a small header into a buffer and copies sixteen 32-bit words from a fixed offset in the base structure into it.
- `copy_buffer_and_call`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### Dir

- `find_max_and_copy_words`: Walks a linked list of entries to find the maximum value in a field, then, if a condition is met, writes that value and a small header into a buffer.
- `copy_buffer_if_valid`: Copies a fixed block of 15 words from a source table pointed to by a register into the buffer passed as an argument, but only when that buffer is not null.

### NewCLI

- `find_max_and_prepare_buffer_3`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes a small header into a buffer and copies sixteen 32-bit words from a fixed offset in the base structure into it.
- `copy_buffer_and_call_2`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination pointer is valid, then transfers control to a routine whose address was passed in.

### NewShell

- `find_max_and_prepare_buffer_4`: Walks a linked list of entries to find the maximum value stored in a field, then, when a condition is met, writes a small header into a buffer and copies sixteen 32-bit words from a fixed offset in the base structure into it before invoking another routine.
- `copy_sixteen_longs_then_invoke`: Copies sixteen longwords from a fixed offset in a register-addressed block into a caller-supplied buffer when that buffer pointer is non-null, then invokes a routine through a register-held code pointer.

### Echo

- `scan_max_and_install_handler`: Walks a linked list to find the largest value in a stored field, floored at 149, and writes it with an incrementing fill pattern into a newly prepared block.
- `copy_words_and_call`: Copies a fixed number of words from a source table into a destination buffer when the destination is not null, then invokes another routine.

### Execute

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then builds a small header structure containing that value and copies sixteen 32-bit words from a fixed offset into a caller-provided buffer.
- `copy_words_and_call_2`: Copies a fixed number of words from one memory location to another when the destination pointer is not null, then invokes another routine.

### Copy

- `find_max_and_fill_buffer`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes that maximum plus a small header into a buffer and fills sixteen 32-bit words from a fixed offset.
- `copy_word_block`: Copies a fixed block of sixteen words from a source table into a destination buffer when the destination pointer is valid, then invokes another routine.

### Makedir

- `find_max_and_prepare_buffer_6`: Walks a linked list of entries to find the maximum value stored in a field, then writes that value into a small header and fills a block of entries with a repeating pattern.
- `copy_buffer_and_call_3`: Copies a fixed block of sixteen words from a source table into a destination buffer when that buffer is not null, then invokes another routine.

### List

- `find_max_and_prepare_buffer_7`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes that value and a sequence of header words into a buffer.
- `copy_word_block_2`: Copies a fixed block of sixteen words from a source table into a destination buffer when the buffer pointer is valid, then invokes another routine.

### CD

- `find_max_and_prepare_buffer_8`: Walks a linked list of entries to find the maximum value in a field, then stores that maximum along with a small header of words into a buffer.
- `copy_words_and_call_pointer`: Copies a fixed count of 32-bit words from a field offset in a global structure into a caller-provided buffer, but only when that buffer pointer is not null, then invokes a routine stored in a global function pointer.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `find_max_and_prepare_buffer_9`: Walks a linked list of entries to find the maximum value stored in a field, then writes that value into a small header structure and fills sixteen 32-bit words from a fixed offset into a caller-provided buffer.
- `copy_words_then_call`: Copies a fixed number of words from a source table into a destination buffer when that buffer is not null, then invokes another routine.

### LoadWB

- `start_program_cli_or_wb`: Allocates a control block through exec, then branches: when started from the Workbench it takes the startup message, and when started from the CLI it splits the command line into an argument table, honouring double quotes and asterisk escapes for newline and escape characters.
- `restore_state_and_cleanup`: Fetches a per-task pointer through a global library base, stores the given value into a slot of the structure it points to, and then makes cleanup calls gated by fields of a saved structure.
- `launch_workbench_tool_task`: Checks for a resident Workbench task and, when a tool is to be started, allocates a 162-byte block, copies a global command string into it and calls the library routines that launch the tool.
- `compare_zero_terminated_strings`: Compares two zero-terminated strings byte by byte, returning minus one if the first is smaller at the first difference, one if it is larger and zero when they match.
- `build_linked_list_from_table`: Walks a table of entries, allocating an eight-byte node for each through the library and storing a value obtained from the entry in the node's second field, with nodes chained by shifted pointers.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `invoke_library_vector_132`: Calls a library function through a vector at a fixed negative offset from a library base kept in a global variable, with no arguments and no result.
- `call_indirect_function`: Calls a function whose address is stored in a global variable, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `pad_release_string_on_match`: Checks a text buffer against a stored version string and, when the buffer's character at offset twenty is a two and the prefix matches, copies a dot and space with zero fill at offset seventeen.
- `write_dot_and_four_spaces`: Writes a dot followed by four spaces into a text buffer, starting at a fixed offset of seventeen bytes.
- `skip_matching_prefix`: Advances through a buffer for as long as its bytes match a stored string, stopping at the end of that string or at the first difference, and returns with the pointer moved past the matched prefix.
- `stub_empty_return`: Contains only a return and performs no work.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `invoke_exec_dispatch`: Calls the Dispatch function of the exec library through a stored library base, with no arguments.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `forward_params_with_stack_slot`: Forwards its two parameters to another routine together with a pointer to a stack slot, then returns.
- `measure_stack_string_and_forward`: Calls a stored library function, then measures the length of a 140-byte stack buffer up to its terminator.
- `store_byte_through_pointer`: Writes the low byte of a data register to the memory address held in an address register and returns.

### Version

- `parse_command_line`: Parses a command line by splitting it into tokens and copying each token into a buffer, handling quoted strings and escape sequences.
- `restore_structure_state`: Stores a passed value into a slot within a structure pointed to by a global variable, then invokes several callbacks from a table based on flag checks before returning the previously stored value.
- `check_library_version_and_report`: Parses the library or device name and optional version, revision and unit arguments from the command line, opens the named library or device and prints its version or a usage line.
- `call_helper_with_fixed_flag`: Calls a routine with its first argument together with the constant 128, then calls another routine with its second argument.
- `parse_signed_decimal_string`: Reads an optional plus or minus sign and then a run of decimal digits, accumulating them into an integer and negating the result for a minus sign.
- `compare_strings_byte_by_byte`: Walks two null-terminated strings in lockstep, returning a negative value when the first string sorts before the second, a positive value when it sorts after, and zero when they match.
- `calculate_string_length`: Counts the bytes of a zero-terminated string up to the terminator and returns that count.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_dispatched_handler`: Invokes a function pointer stored within a structure at a fixed offset and returns its result.
- `recurse_with_stack_slot`: Calls another routine passing two parameters along with a pointer to a specific location on the current stack.
- `call_table_handler`: Invokes a function pointer stored within a structure at a fixed offset and returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_indirect_global`: It invokes the routine referenced by a global variable's stored pointer and then returns without using any library calls.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_indirect_handler`: It invokes the routine whose address is stored at the offset located 552 bytes before the start of the code table.
- `call_global_and_checksum_buffer`: Invokes a function pointer held in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is reached and forwards the buffer and its inverted checksum to another routine when non-zero.
- `store_byte_from_register`: Copies a byte held in a register into the memory location pointed to by another register and returns.

### SetMap

- `initialize_keymap_setup`: Initializes a stack frame in a global structure, then parses a command-line argument or icon KEYMAP tool type, trims trailing whitespace, decodes quoted strings with escape sequences, and opens the matching keymap file.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within the pointed-to structure, then conditionally invokes several cleanup callbacks stored in a table before returning the saved parameter.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_indirect_from_table`: Loads a function pointer from a global table and jumps to it via an indirect call.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `invoke_exec_reschedule`: Calls the Reschedule function of the exec library through a stored library base, with no arguments.
- `process_stored_entries_in_reverse`: Walks a table of stored entries from the last index downwards, passing each non-zero entry to a routine after shifting it right by two to recover a pointer.
- `store_params_call_fixed_args`: Stores its two parameters into global slots, then calls a routine with a fixed set of eight arguments including a pointer to a global and the constants 440 and 60.
- `select_keymap_from_arg_icon`: Takes a keymap identifier from the command argument or the icon's KEYMAP tool type, lowercases it and opens the matching file under the keymaps directory.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `invoke_dispatched_handler`: Reads a routine entry from a global dispatch table and invokes it through an indirect call.
- `call_stored_pointer`: Fetches a stored code pointer from a global table and transfers control to it indirectly.
- `invoke_stored_handler`: Loads a handler address from a global table and jumps to it through an indirect call.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_dispatched_routine`: Reads a function pointer from a global dispatch table and calls it indirectly.
- `call_keymap_routine`: It invokes the routine whose address is stored at the location pointed to by the value in register d1, then returns.
- `invoke_keymap_routine`: Invokes the routine whose entry point is stored in a global variable, passing the current value of data register 1 as an argument.
- `invoke_dispatched_pointer`: Reads a function pointer from a global dispatch table and invokes it indirectly.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_routine`: It invokes the routine whose entry pointer is stored at a fixed offset before a global variable and then returns.
- `invoke_keymap_routine_2`: It invokes the routine whose address is stored at the negative offset of register d1, which points into the keymap library.
- `call_global_function`: It fetches a function pointer from a global structure and calls it, then returns.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `forward_param_global_stack_slot`: Forwards its parameter to another routine together with a global value and a pointer to a stack slot, then returns.
- `call_library_vector_348_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_via_pointer`: It stores a byte value from a register into a memory location pointed to by a pointer.

### More

- `initialize_and_parse_command`: Initializes a global structure and stores the incoming parameters into it, then checks whether a command string is present.
- `restore_patch_state_2`: It retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `page_file_with_progress_display`: Takes a file name from the arguments or prompts for one, then displays the file a page at a time in a window with a percentage progress indicator and a quit hint.
- `call_routine_for_four_globals`: Calls a routine four times in turn, passing a fixed field of a structure together with each of four consecutive global values.
- `validate_input_buffer`: Calls two routines to fill an eighty-byte block of a buffer, then returns one as soon as it finds a byte within the first eighty that is a control character other than the usual text controls, or a value in the 128 to 159 range.
- `read_console_key_and_dispatch`: Waits for a console event, reads one key code and dispatches it through a switch: it skips escape-sequence bodies, handles a few navigation and quit codes, and returns a result code.
- `call_setup_then_register_handler`: Passes a global value to one routine, calls a second routine, then invokes a code pointer held in an address register and returns.
- `initialize_state_and_run_steps`: Initializes two fields of a state structure and stores the result of a routine call in another field.
- `update_entity_and_increment_counter`: Calls a routine twice, passing the second result along, and stores the final result in a field of the structure.
- `read_line_marking_form_feed`: Reads up to 128 bytes into a buffer and counts the characters before the first newline or form feed.
- `process_entries_with_quit_check`: Chooses a loop limit of nine, thirteen or fourteen from a flag and a field, then processes queued entries from a global table through a routine until the limit is reached.
- `search_until_match`: Sets up a search and repeatedly steps through it until a routine reports a match, returning one on a match and zero when the counter check fails first.
- `run_call_sequence_return_result`: Runs a fixed sequence of routine calls on a parameter block, passing a global value and the first argument at one step, and returns the result of the call that uses both parameters.
- `check_flag_and_call`: Calls a routine with its first two arguments, then calls a second routine with the first argument only when a 16-bit flag in a structure is zero.
- `call_stored_value_then_forward`: Calls a routine with a stored field, a minus-one argument and a pointer, then calls a second routine with a 16-bit value read from a linked structure.
- `call_fixed_then_forward_decremented`: Calls a routine with a zero argument and the constant one, then calls a second routine with a 16-bit value from a linked structure reduced by one.
- `emit_text_lines_to_handler`: Walks a text buffer line by line, counting characters per line up to a limit taken from a structure and passing each segment through a handler routine.
- `process_then_dispatch_via_pointer`: Calls a routine with its second argument, then calls another routine using a pointer taken from a structure, a global variable and the first argument.
- `set_flag_and_forward`: When a field of a structure is zero, sets a 16-bit field of the structure reached through a pointer to one.
- `dispatch_by_flag`: Calls a routine with a global pointer and a third argument chosen by a flag in the parameter block: either a global pointer, or a 16-bit value from a linked structure plus one.
- `dispatch_handler_by_flag`: Calls a routine with a global pointer repeated three times when a flag in the parameter block is zero.
- `clamp_negative_result`: Calls a routine twice with the same first argument and different third arguments, then returns the second result with negative values clamped to zero.
- `consume_from_buffer`: Copies up to a requested number of bytes from the structure's buffer into a destination, clamped to the bytes still remaining, after first refilling the buffer when it is short.
- `refill_buffer_keeping_remainder`: When unread bytes remain and the buffer is full, moves those bytes to the front of the buffer, then reads more data into the free space through a routine.
- `adjust_window_bounds`: Clamps a computed position against a window's bounds and adjusts the window's fields accordingly.
- `read_line_from_buffer`: Reads bytes one at a time from a source into a destination, storing the newline if one arrives, and stops after a newline or after about 127 bytes.
- `copy_null_terminated_string_2`: Copies bytes from a source to a destination until the null terminator has been copied.
- `convert_string_to_uppercase`: Converts each lowercase ASCII letter in a zero-terminated string to uppercase by subtracting the case offset, leaving other bytes unchanged.
- `parse_signed_decimal_2`: Skips an optional leading plus or minus sign in a character string, then accumulates each following ASCII digit into an integer, negating the result if the sign was minus.
- `call_with_mask_if_set`: When the first argument is non-zero and a pointer in a structure is set, calls a routine with that pointer, the first argument and a minus-one argument.
- `process_text_and_free_structure`: Calls routines on the structure's fields, then handles its text: when the flag is set and the text is non-empty, it reads characters until a newline arrives.
- `pass_single_value_to_handler`: Stores its argument in a one-element stack buffer and passes it, with a constant of 994 and a size of one, to a routine that takes its target from a pointer in a global structure.
- `allocate_pass_and_copy_fields`: Allocates a 36-byte block and passes its shifted pointer through a routine together with a constant of 25.
- `build_template_block_return_field`: Allocates a 68-byte block, links it to itself and stores a template value and a parameter in it, then copies a number of words from a source into the block.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `dispatch_reschedule_handler`: Fetches a handler pointer from a fixed library offset and jumps to it, returning nothing.
- `invoke_exec_switch`: Calls the Switch function of the exec library through a stored library base, with no arguments of its own.
- `call_indirect_routine_2`: It calls the routine stored at a function pointer located near a global variable, passing no arguments.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `invoke_exec_allocmem`: Calls the AllocMem function of the exec library through a stored library base, using arguments already in place.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_indirect_pointer`: It dereferences a stored pointer at a fixed global offset and jumps to the routine it holds, passing no arguments.
- `call_indirect_routine_3`: It invokes the routine whose address is stored in a global function pointer, passing no arguments.
- `call_indirect_global_2`: It invokes the routine whose address is stored in a global variable and then returns without doing anything else.
- `call_indirect_global_ptr`: It invokes the routine whose entry pointer is stored at a global variable located 210 bytes below the address in register d1, then returns without modifying anything else.
- `invoke_handler_from_global`: Fetches a handler pointer from a global variable and invokes it, then returns.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_indirect_routine_4`: It dereferences a pointer stored in a global variable and jumps to the routine it holds, then returns.
- `call_indirect_routine_5`: It dereferences a pointer stored at a fixed global offset and invokes the routine it points to, then returns without using any arguments.
- `call_indirect_from_base`: It invokes the routine whose address is stored at the global variable located 384 bytes below the given base, passing no arguments.
- `call_indirect_global_ptr_2`: It dereferences a pointer stored at a fixed global offset and calls the routine that pointer references, passing no arguments.
- `call_indirect_routine_6`: It invokes the routine whose address is stored at the pointer located at the global variable, passing no arguments.
- `call_indirect_routine_7`: It dereferences a pointer stored at a fixed offset from a global variable and jumps to the routine it points to, performing no other work itself.
- `recurse_with_stack_slot_2`: Invokes another routine passing two parameters along with a pointer to a fixed stack slot, returning nothing.
- `read_byte_or_minus_one`: Calls a routine to read one byte into a small stack buffer.
- `call_global_and_checksum_buffer_3`: Invokes the function pointer stored in a global variable, then scans a stack buffer byte by byte, updating a rolling checksum until a null terminator is reached.
- `store_byte_at_a3`: Stores the byte currently in a data register into the memory location pointed to by a fixed address, then returns.
- `divide_32bit_with_remainder`: Divides a 32-bit value by a divisor: for divisors above 65535 it runs a sixteen-step shift-and-subtract division, and otherwise it divides the two halves of the value by the 16-bit divisor.
- `multiply_32bit_with_partial_products`: Computes the low 32 bits of a 32-bit multiplication from four 16-bit partial products and returns them together with the second operand with its two halves swapped.
- `call_routine_and_return`: Calls one other routine with no arguments and then returns.
- `negate_result_on_flag`: Calls a routine and returns its result, negated when the routine leaves a non-zero value in an address register.
- `invoke_exec_rawdofmt`: Calls the RawDoFmt function of the exec library through a stored library base, using arguments already in place.
- `store_byte_via_a3`: Stores a single byte taken from register D0 into the memory location pointed to by the value in address register A3.
- `call_indirect_pointer_2`: It invokes the routine whose address is stored in a global function pointer, then returns without using any return value.
- `allocate_signal_block_and_dispatch`: Obtains a signal bit through a routine, then allocates a 34-byte block and fills its fields with the parameters, the signal bit and a value from another routine.
- `remove_and_free_signal_block`: Removes the block from its list when it is linked, marks its status fields as deleted, releases its signal bit and frees the 34-byte block.
- `find_substring_in_text`: Searches a zero-terminated text for a zero-terminated pattern, returning one when the pattern occurs and zero otherwise.
- `initialize_list_header`: Initializes a list header in place: its first word points to the word after it, its second word is cleared, and its third word points back to the header.

### Format

- `initialize_and_parse_command_2`: Initializes a global structure and dispatches to a library switch routine, then parses a command string by trimming trailing whitespace and decoding quoted tokens with escape sequences into a buffer.
- `restore_patch_state_3`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_exec_function`: Invokes a function pointer retrieved from an offset in the exec library base and returns its result.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_debug`: Library stub: loads its arguments and the exec library base, calls Debug and returns its result to the caller.
- `call_exec_switch`: Invokes the Switch function of the exec library by dereferencing a function pointer from the library base.
- `invoke_exec_forbid`: Calls the Forbid function of the exec library through a stored library base, with no arguments.
- `call_exec_dispatch`: Invokes the Dispatch function of the exec library by dereferencing a function pointer from the library base.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `invoke_exec_schedule`: Calls the Schedule function of the exec library through a stored library base, using arguments already in place.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `invoke_exec_allocabs`: Calls the AllocAbs function of the exec library through a stored library base, using arguments already in place.
- `call_exec_reschedule`: Invokes the Reschedule function of the exec library by dereferencing a function pointer from the library base.
- `format_disk_from_arguments`: Parses the DRIVE, NAME and option keywords such as FFS, NOICONS and QUICK from the command line, printing a usage line when they are missing.
- `release_format_resources`: Releases each allocated resource in turn, freeing a 260-byte buffer and a 36-byte block when they are present, then passes the status argument to a final routine.
- `map_disk_error_to_message`: Runs a disk operation and retries it while the operation reports one specific code.
- `report_format_status`: Outputs one of two status messages chosen by a flag and returns the status code as a number.
- `record_matching_table_entry`: Searches an eight-entry global table for the given byte and stores the matching entry, or zero when none matches, into a field of a block obtained from a routine.
- `find_matching_list_entry`: Walks a linked list of entries and returns one when it finds an entry matching the given value, either by a stored value or by a comparison routine applied to a name.
- `calculate_string_length_2`: Returns the number of characters in a null-terminated string before its terminator.
- `copy_cstring_to_buffer`: Copies a null-terminated byte string from a source pointer to a destination pointer, writing each byte and advancing both pointers until the terminating zero byte is stored.
- `convert_char_to_uppercase`: Returns the uppercase form of a single byte when it is a lowercase ASCII letter, and the byte unchanged otherwise.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings after converting each byte to uppercase, returning one when they are equal and zero otherwise.
- `truncate_string_at_colon`: Walks a string up to its first colon or terminator and writes a zero byte there, truncating the string before the colon.
- `pass_value_and_map_failure`: Stores its first argument in a one-element stack buffer and passes it, with a constant of 31 and a size of one, to a routine.
- `create_block_from_source_words`: Allocates a 68-byte object, copies up to seven words from a source array into it, and links it to a handle from another routine.
- `call_library_function`: Invokes a function from a library identified by the negative offset in its base and returns its result.
- `invoke_indirect_routine`: Calls a routine whose entry point is stored in a global variable located near the start of the code section, then returns.
- `call_registered_handler`: It invokes the function whose address is stored at a fixed offset within a global table, effectively dispatching to a routine registered in that data structure.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_indirect_routine_8`: It calls the routine referenced by a global function pointer and then returns.
- `call_indirect_routine_9`: Invokes a routine whose entry pointer is stored at a fixed offset from a global base address, then returns without using any arguments.
- `invoke_indirect_pointer`: Fetches a code pointer stored in a global variable and jumps to it via an indirect call.
- `call_indirect_global_3`: It calls the routine whose address is stored in a global variable and then returns.
- `call_indirect_pointer_3`: It invokes the routine whose address is stored in a global variable, then returns.
- `call_indirect_routine_10`: It dereferences a pointer stored at a global variable and calls the routine that pointer refers to, passing no arguments.
- `call_table_function_v2`: It invokes the routine whose address is stored at a fixed offset within a global function table.
- `call_stack_pointer_handler`: It invokes the routine whose entry pointer is stored in a global variable table, passing the current stack pointer as an argument.
- `call_table_handler_2`: It invokes the routine whose entry pointer is stored at a fixed offset within a global function table, then returns.
- `call_table_dispatch`: It invokes the function whose address is stored at a fixed offset within a global table, effectively dispatching to a routine registered in that table.
- `call_indirect_routine_11`: Dereferences a pointer stored in a global variable and invokes the routine it points to, passing no arguments.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `indirect_call_via_global_ptr`: It invokes the routine whose entry pointer is stored at a fixed offset from a global variable, passing no arguments.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_indirect_handler_2`: Fetches a function pointer from a global table and jumps to it via an indirect call.
- `forward_argument_to_library_stub`: Passes its one incoming argument, a library base pointer read from a global variable, and the address of its own stack argument to a library call stub at a fixed offset.
- `call_indirect_d3_base`: It invokes the routine whose address is stored in a global variable located just below another global, then returns without using any arguments.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `format_disk_track_by_track`: Formats the disk one cylinder at a time: it obtains a disk port and a track buffer, fills each sector's header words with a DOS identifier, the sector number and the cylinder number, then writes and verifies the track.
- `configure_handle_then_forward`: Makes three calls on a handle held in a global structure, passing the constants twenty and thirty, then one, then forwards its parameter together with a value returned by another routine.
- `call_global_and_checksum_buffer_4`: Invokes a function pointer stored at a fixed offset within a global variable, then walks a stack buffer byte by byte computing a rolling checksum until a null terminator is found.
- `store_byte_via_pointer_2`: Stores a byte from a register into the memory location pointed to by a global pointer.
- `divide_thirty_two_bit`: Performs a 32-bit by 32-bit division, using a sixteen-step restoring division loop for large divisors and falling back to compiler built-in operations for small ones.
- `run_sixteen_step_division`: Runs a sixteen-step restoring division loop that shifts the dividend's halves together and subtracts the divisor whenever the partial remainder reaches it.
- `multiply_thirty_two_bit`: Multiplies two 32-bit values by combining their 16-bit halves and returns the 64-bit product split across two registers.
- `invoke_table_entry`: Invokes a function pointer retrieved from an offset within a global variable, calling the address stored at that specific location in the global data structure.
- `store_byte_via_a3_2`: Copies a single byte from register D0 to the memory location pointed to by the A3 register.
- `allocate_and_initialize_block`: Calls a library function to allocate a memory block, fills in fields including a stored parameter and flag bytes, and calls further library functions, returning the block address or zero.
- `reset_object_and_recurse`: Recursively processes a linked structure when a pointer field is non-zero, sets a byte field to all ones and a four-byte field to all ones, then calls another routine on a byte field value and again with a constant second argument.
- `forward_param_with_constant`: Forwards its parameter to another routine together with the constant 48, then returns.
- `forward_single_param`: Forwards its parameter unchanged to another routine, then returns.
- `setup_trashcan_drawer_globals`: Saves a field, sets it to a sentinel while it fills several global values, then calls routines that use them.
- `execute_with_status_check`: Invokes a routine through a global library base pointer and returns either a value from a stack slot or a negative error code.
- `invoke_indirect_return`: It invokes the routine whose address is stored in a global variable and then returns.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores its pointer argument's own address into the memory location that pointer references, increments that stored value by four, zeroes the following word, and writes the incremented pointer into the word after that.
- `allocate_node_with_fields`: Allocates a block of the requested size and, when that succeeds and a pointer is supplied, stores a byte value of five, the low word of the size argument and the pointer into fixed offsets of the block.
- `reset_node_then_pass_on`: Returns zero for a null pointer.
- `call_indirect_from_global`: This routine invokes a function whose pointer is stored in a global variable and then returns without performing any other operations.
- `call_indirect_table`: It invokes the routine whose address is stored at a fixed location in a global table, then returns without modifying anything else.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.

### CLI

- `parse_command_line_and_start`: Allocates a control block, opens dos.library, and splits the command line into argument strings while honoring double quotes and asterisk escape sequences.
- `invoke_exec_supervisor`: Calls the Supervisor function of the exec library through a stored library base, using arguments already in place.
- `invoke_exec_exitintr`: Calls the ExitIntr function of the exec library through a stored library base, using arguments already in place.
- `call_exec_switch_2`: Fetches a function pointer from a fixed offset in the exec library base and calls it with no arguments.
- `call_indirect_from_offset`: It invokes the routine whose address is stored at the offset located just before a value in a global variable, passing that value as an argument, then returns.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `invoke_exec_makefunctions`: Calls the MakeFunctions function of the exec library through a stored library base, using arguments already in place.
- `invoke_exec_findresident`: Calls the FindResident function of the exec library through a stored library base, using arguments already in place.
- `invoke_exec_enable`: Calls the Enable function of the exec library through a stored library base, using arguments already in place.
- `invoke_exec_allocentry`: Calls the AllocEntry function of the exec library through a stored library base, using arguments already in place.
- `start_cli_session_with_scripts`: Allocates a 64-byte command-line structure and fills in its fields, then sets up a console window from either a default specification or one given on the command line.
- `build_linked_list_from_table_2`: Walks a table of entries, allocating an eight-byte node for each through the library and storing a value obtained from the entry in its second field, with nodes chained by shifted pointers.
- `walk_pointer_chain_recursive`: Follows a chain of linked records, recursing on the second field of each record and then moving to the record its first field points to, until the link is zero.
- `parse_signed_decimal`: Reads an optional leading plus or minus sign and then accumulates decimal digits from a null-terminated string into a 32-bit integer, negating the result for a minus sign.
- `call_library_jump_vector_198`: Calls one Amiga library function through a jump vector at offset minus 198 from the library base held in a global, with no arguments and no result.
- `call_library_jump_vector_210`: Calls one Amiga library function through a jump vector at offset minus 210 from the library base held in a global, with no arguments and no result.
- `call_library_jump_vector_294`: Calls one Amiga library function through a jump vector at offset minus 294 from the library base held in a global, with no arguments and no result.
- `call_library_jump_vector_414`: Calls one Amiga library function through a jump vector at offset minus 414 from the library base held in a global, with no arguments and no result.
- `call_library_jump_vector_552`: Calls one Amiga library function through a jump vector at offset minus 552 from the library base held in a global, with no arguments and no result.
- `call_library_jump_vector_78`: Calls one function through a jump vector at offset minus 78 from a different library base held in a global, with no arguments and no result.
- `call_library_jump_vector_90`: Calls one function through a jump vector at offset minus 90 from a different library base held in a global, with no arguments and no result.
- `call_library_jump_vector_96`: Calls one function through a jump vector at offset minus 96 from a different library base held in a global, with no arguments and no result.
- `divide_32_bit_with_remainder`: Divides a 32-bit dividend by a 32-bit divisor and returns the quotient and remainder together.
- `multiply_32_bit_integers`: Multiplies two 32-bit values by summing 16-bit partial products, returning the low 32 bits of the product in the first result word.
- `negate_on_flag`: Calls another routine, returns its primary result, and negates that result when the secondary result register is non-zero.

### Disk-Validator

- `validate_disk_bitmap_blocks`: Entry routine of the disk validator that formats a disk number into a status text, then scans bitmap words over a block range, testing individual bits against stored block flags and counting mismatches.

### Ram-Handler

- `copy_memory_and_dispatch`: Program entry that calls a helper through a global pointer and then makes an indirect call through a jump table whose targets could not be recovered.

### Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:AM2_BLIT

- `initialize_blitter_state`: Program entry that runs a short countdown loop of library-vector calls, clears a result word, and if a value found through a linked pointer matches a fixed constant, saves it, writes a global and clears that global.
- `decrement_until_minus_one`: Decrements a 16-bit counter in a loop until it reaches minus one, then returns.

### d

- `stub_bad_instruction_data`: The bytes at this entry do not decode to valid instructions, so there is no real routine.

### gb

- `jump_and_stop`: The routine executes a single instruction that jumps to another routine and then stops.

### Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:AM2_CPU

- `initialize_game_state_2`: Initializes game state by running a sequence of setup routines, walking a structure through a loop, clearing a field, and swapping a memory value when a specific constant is found.
- `decrement_until_minus_one_2`: Decrements a 16-bit counter in a loop until it reaches minus one, then returns.

### Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Ambermoon

- `initialize_game_state`: Program entry that calls a helper and then runs a short countdown loop of library-vector calls, clears a result word, and if a value found through a linked pointer matches a fixed constant, saves it, writes a global and clears that global.
- `decrement_until_minus_one_3`: Decrements a local counter in a loop until it reaches negative one, then returns.
- `decode_bitstream_into_buffer`: Decoding loop that repeatedly reads values through a function pointer, tests individual bits of each value, and adds a second value into memory at a running offset.
- `copy_bytes_backward_counted`: Copies bytes from a source to a destination while moving both pointers backward, decrementing a count until it reaches zero.
- `copy_bytes_backward_flagged`: Copies bytes backward from a source to a destination under a count, and checks a flag byte that can send the copy into a separate branch.
- `write_bytes_in_quads_backward`: Writes groups of four bytes backward into memory from a source pointer, merging the leading byte with a fixed bit mask.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:Ambermoon_install

- `initialize_install_state`: Program entry in the installer that runs a short countdown loop of library-vector calls, clears a result word, and if a value found through a linked pointer matches a fixed constant, saves it, writes a global and clears that global.
- `decrement_to_negative_one`: Repeatedly subtracts one from a signed 16-bit value held in a register until that value equals negative one, then exits.
- `accumulate_and_check_globals`: Repeatedly invokes another routine in a loop, interpreting its return value to decide whether to accumulate a running sum and add it into a sequence of integers through a pointer, or to skip work.
- `copy_and_unpack_data`: Copies a run of bytes from a source buffer into a destination buffer, decrementing both pointers as it goes.
- `reverse_copy_expand_flags_2`: Copies a byte string from a source to a destination while reversing the byte order within each group of four bytes.
- `reverse_copy_expand_flags`: Copies a byte string from a source to a destination while reversing the byte order of each group of four bytes.
- `stub_bad_instruction_data_2`: The bytes at this entry do not decode to a valid instruction, so there is no real routine.

### Ambermoon (1994)(Thalion)(Disk 01 of 10)(A).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_222`: Calls one Amiga library function through a jump vector at offset minus 222 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `guard_task_state_with_forbid`: Calls the Amiga library functions that forbid and permit task switching, looks up the current task, and frees memory, branching on a global state value.
- `allocate_memory_block_checked`: Allocates memory through the Amiga system and then calls further library functions to check the allocation.
- `query_library_field_when_ready`: Calls one library function and, if it returns non-zero, calls a second function with the same argument.
- `dispatch_library_handle_entry`: Obtains a handle from a library function and, if the handle is zero, releases the result and returns minus one.
- `store_handle_and_release`: Calls one library function, and if it returns a non-zero handle, stores it in a global and runs three further routines.
- `run_library_session_by_status`: Calls a library function to obtain a handle and, if it is non-zero, runs six helper routines plus one more for status codes 2 or 3, then calls a second library function with the handle.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `loop_library_call_while_pending`: Calls a library function repeatedly while a check routine reports a non-zero value, running a helper after each call.
- `call_library_if_check_passes`: Runs a check routine and, only if it returns non-zero, calls one library function.
- `search_word_table_for_key`: Scans a table of 16-bit entries terminated by minus one, using a helper to find each entry's length and skip past it, until an entry equals the supplied word.
- `measure_null_terminated_string`: Counts the characters of a zero-terminated string and returns the count.
- `wait_for_blitter_then_spin`: Polls the DMA control register until the blitter busy bit clears, stores a handler address into a structure, and then enters an endless loop.
- `set_global_to_constant_64`: Stores the constant 64 into a global variable and returns.
- `run_library_setup_once`: Uses a global flag so that its library work runs once: it calls a library function and, depending on the result, calls a second function or stores a flag from a status code of 205.
- `match_text_and_read_digit`: Compares an input string against a fixed reference text and, on a full match, checks whether the next character is a decimal digit.
- `set_flag_if_field_zero`: Stores minus one in a global variable when a field of the structure it references is zero, and zero otherwise.
- `wait_and_get_port_message`: Waits on a message port and then takes the next message from it.
- `reply_message_under_forbid`: If a stored message pointer is set, it calls the Forbid library function, replies to that message, and clears the pointer.
- `open_library_and_read_field`: Opens an Amiga library by calling the OpenLibrary vector, repeats that call twice more, and copies a 16-bit field from the resulting library base.
- `close_library_if_open`: Calls the CloseLibrary vector on the stored library base up to three times, each call guarded by a check that the base is non-zero.
- `set_flag_from_library_result`: Calls a library function and stores minus one in a global variable when it returns non-zero, and zero otherwise.
- `call_library_210_if_set`: Calls one Amiga library function through a jump vector at offset minus 210, but only when a flag global is non-zero.
- `initialize_library_resources`: When a global is clear, calls a series of library functions to set up and open resources, stores a structure field into the global, and finishes with a further library call.
- `release_entry_and_clear_state`: When a global is active, stores it into a structure field, looks up an entry through a library function, and clears the global if the entry's flag bits are zero.
- `store_current_task_pointer`: Calls the exec FindTask function and stores its result, the current task, in a global variable.
- `store_adjusted_char_then_call`: If a flag global is clear, stores the character at the input pointer plus 64 into a fixed RAM byte and calls a helper.
- `copy_null_terminated_string`: Copies a zero-terminated string byte by byte from the source pointer to the destination pointer, including the terminator.
- `set_flag_if_field_below_limit`: Sets a global variable to minus one when a 16-bit field of the referenced structure, reduced by 36, is negative, and to zero otherwise.
- `open_device_transfer_and_close`: Allocates a signal, opens a device, performs an I/O request, then closes the device and frees the signal.
- `release_library_if_active`: If a global variable is non-zero, calls one library function and then clears the global.
- `probe_with_saved_field`: Saves a structure field, sets it to minus one, and calls a library function to probe.
- `call_library_vector_348`: Calls one Amiga library function through a jump vector at offset minus 348 and returns with no result.
- `retry_library_call_with_mode`: Calls a library function and, if it fails, retries it with a fixed mode constant, then calls two more library functions depending on the outcome.
- `finish_library_call_with_mode`: When a global is active, sets the same fixed mode constant and calls a library function.
- `run_in_supervisor_mode`: Sets a global from a bit in a structure field and calls the exec Supervisor function with the result.
- `store_vector_base_register`: Stores the processor vector base register into a global variable, or zero when a flag global is clear.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helper_into_global`: Single-instruction entry that calls one helper and copies its register result into a global variable.
- `run_install_options_dialog`: Shows installer prompts that let the user choose playing from floppy disk or from hard disk, or leave the installation program.
- `write_installer_script_lines`: Writes a list of script lines through repeated helper calls.
- `install_game_files_to_destination`: Builds destination paths and copies the game's program, icon, keymap and document files into a chosen directory, then shows an installation message.
- `call_helper_with_local_buffer`: Calls a helper with a ten-byte local buffer and the constant 1, then returns.
- `copy_bounded_string_to_buffer`: Gets a string into a 100-byte local buffer and copies it out.
- `store_table_entry_and_call`: Stores a value from a table indexed by the input register into a global variable, then calls two helpers with fields from a structure, the second receiving the global's address and the constant 1.
- `run_copy_command`: Formats a copy command line from two text arguments into a 200-byte stack buffer and runs it through the system execute entry.
- `prompt_yes_or_no_answer`: Repeats a prompt, reads one character, and stops when the character is Y or N, returning that character.
- `create_directory_then_unlock`: Calls the directory creation step with its argument and, when that returns a lock handle, releases the lock through the second library.
- `create_save_game_directories`: Creates the save-game directories by building numbered directory names and calling the create-directory and unlock wrappers, looping over eleven entries.
- `create_save_file`: Allocates a buffer, builds a file name, and calls helpers to create the saves file.
- `read_file_size_via_lock`: Locks a named file, examines it into a 260-byte info block, and returns the size field from that block when the examine succeeds.
- `report_file_operation_error`: Shows a message for a file error, choosing between couldn't be opened, couldn't be read, couldn't be written, wasn't found, and not enough memory, and then asks the user to press RETURN.
- `check_install_folder_exists`: Locks the installer's named folder and, depending on whether the lock succeeds or reports not-found, sets a global flag.
- `set_flag_if_field_above_limit`: Sets a global to one when a 16-bit field of the referenced structure is above 35, and to zero otherwise.
- `test_file_lock_available`: Takes a read lock on a named file and releases it if the lock is obtained, returning whether the lock succeeded.
- `walk_parent_directories`: Walks upward from the current directory, repeatedly examining each level through a parent-directory call, until the loop finishes.
- `copy_string_to_buffer`: Copies bytes from the source string into the destination until a zero byte, including the terminator, and returns the original destination pointer.
- `measure_string_length`: Walks a zero-terminated byte string to its terminating zero and returns the number of bytes before it.
- `convert_lowercase_to_uppercase`: Converts a single lowercase letter to uppercase and leaves every other byte unchanged.
- `convert_uppercase_to_lowercase`: Converts a single uppercase letter to lowercase and leaves every other byte unchanged.
- `reset_global_and_call_helper`: Clears a global variable and calls a helper with an argument taken from the caller's stack.
- `resolve_global_source`: With a zero argument it sets the global either from a helper result or to minus one when a flag bit is set.
- `scan_formatted_string`: Scans a string against format characters and stores each converted value through the pointer arguments, using a 130-byte local buffer.
- `poll_handler_until_ready`: Calls a function pointer stored in a global with argument zero while a status bit is set, then calls it with argument one and returns minus one if that result is minus one, otherwise zero.
- `parse_signed_number_from_stream`: Reads an optional plus or minus sign and then digits through a character-reading function pointer, combining them into a number and storing the result through the last pointer argument.
- `read_next_nonzero_token`: Reads values through a helper until one is not zero, returning minus one at the end of input.
- `read_next_buffered_byte`: Returns the next byte from a buffer while the read position is below its limit, advancing the position.
- `fill_and_read_stream_byte`: Clears a pending flag, refills the stream buffer through a helper when its pointer is empty, and then reads characters through a loop that calls another helper for each one.
- `push_back_char_to_buffer`: Writes a character back into a buffer by decrementing its position, and returns minus one when the character is invalid or the buffer is already at its start.
- `find_character_in_string`: Returns a pointer to the first occurrence of a given character in a zero-terminated string, or zero when the character is not present.
- `validate_handle_and_read`: Checks a file-handle index against a table and sets a status code when the handle is missing or not readable.
- `stub_return_only`: Routine consisting only of a return, with no calls, memory access or hardware register use.
- `allocate_stream_state_structure`: Allocates a state structure sized by six times a global value, sets two of its fields to one, and fills in the current directory and input handles through library wrappers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `process_command_buffer`: Processes a command buffer by walking it through a pointer table and calling helpers for each element.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_string_with_limit`: Appends a source string to the end of a destination string, copying at most 32766 characters.
- `append_string_limited`: Finds the terminator of the destination string, then copies bytes from the source into it for up to the given count, stopping at a zero byte and inserting a terminator.
- `copy_string_padded_to_count`: Copies up to the given count of bytes from the source to the destination, stopping at a zero byte, and fills the rest of the count with zero bytes.
- `allocate_stream_state_structure_2`: Allocates a stream state structure by opening a library, retrieving a base pointer, and storing the resulting handle and base into the caller's structure.
- `store_pointer_and_call_helper`: Saves a pointer argument into a global variable, calls a helper with the second argument and a stack-passed value, then clears the byte at the saved pointer and returns the helper's result.
- `format_string_output`: Sets a global pointer to a destination, processes a formatted string, and terminates the output with a null byte.
- `store_byte_and_advance_pointer`: Writes a byte to the location the global pointer refers to, advances that pointer by one, and returns the byte.
- `forward_to_helper_with_stack_arg`: Calls a helper with its argument and a stack-passed value, then returns.
- `format_number_into_buffer`: Converts an integer into characters in a buffer, choosing the sign and digits according to a width argument and the value's sign.
- `format_output_string`: Formats a string from a format string and a list of arguments into a 187-byte local buffer, handling width and sign, and calls two helpers to do so.
- `divide_signed_with_sign_fix`: Works out the sign of the result from both inputs, calls a division helper, negates that result when the signs differ, and returns it combined with the second input.
- `call_keeping_low_register`: Calls another routine and returns a 64-bit pair whose high word is the call's second result and whose low word is the caller's original second value.
- `recurse_without_work`: Calls itself through the same routine and then returns, performing no other work.
- `divide_32_bit_with_remainder_2`: Computes the quotient and remainder of a 32-bit integer division from two register inputs.
- `call_helper_with_fixed_address`: Calls a helper with its argument and a fixed address constant, then returns.
- `handle_newline_flush`: Forwards a character to a helper and, when that character is the line-feed value ten and a flag bit is set, calls a second helper with minus one.
- `put_char_into_buffer`: Stores a character in a bounded buffer while space remains, advancing the position and returning the byte.
- `reset_entry_table`: Walks a table of 22-byte entries from a fixed start up to the end marker, calling a helper on each entry.
- `close_stream_entry`: Returns minus one for a null structure.
- `combine_stream_flags`: Combines the results of several helper calls, flushing when a flag is set, into a single combined status value.
- `write_buffered_stream_data`: Writes data to a stream structure through helpers, returning minus one when the stream's error bit is set and otherwise using its buffer and position fields.
- `allocate_free_table_entry`: Scans a table of 22-byte entries for one whose in-use byte is zero, clears its fields and returns it, or returns zero when no entry is free.
- `find_and_clear_free_entry`: Variant of the free-entry search that clears the first unused entry it finds and returns it, returning zero when none is free.
- `clear_entry_fields`: Zeroes the first three 32-bit fields of the entry pointed to by the address register, then returns.
- `init_buffer_with_fallback`: Allocates a 1024-byte buffer and records it in the structure.
- `free_linked_entry_list`: Walks a linked list of entries, releasing each one through a helper with its size plus eight, then clears the list head and returns zero.
- `release_entry_chain`: Releases each entry in a linked chain by calling a helper with its address and size plus eight, then clears the chain head and returns zero.
- `push_entry_onto_list`: Allocates an entry for the given structure and links it onto the head of a list.
- `forward_argument_to_helper`: Calls one helper with its argument and returns.
- `find_list_entry_for_block`: Walks a linked list looking for the entry whose block address matches the argument minus eight, returning minus one when no entry matches.
- `unlink_list_entry`: Removes a matching entry from a linked list by updating either the list head or the previous entry's next pointer, then calls a helper.
- `unlink_entry_from_list`: Unlinks the entry by setting either the list head or the previous entry's link to its successor, then calls a helper and returns zero.
- `set_list_head_to_next`: Sets the list head to the next pointer of the given entry, calls a helper, and returns zero.
- `lookup_table_entry_flag`: Looks up an entry in a table by index and returns whether that entry is non-zero, marking the table as inactive when the index is out of range.
- `delete_file_record_error`: Deletes a named file through the DOS wrapper.
- `stub_return_zero_constant`: Routine that only returns the constant zero, with no calls, memory access or hardware register use.
- `write_to_handle_table_entry`: Validates a handle index against a table and writes data through the DOS write wrapper when the handle is valid.
- `check_flag_mask_then_return`: Calls a helper with a mask of 4096 and returns its value only when that bit is set and the global is zero.
- `write_four_bytes_to_output`: Writes four bytes from the global variable to the standard output handle through library wrappers, then calls one helper with the constant one.
- `run_optional_hook_then_helper`: Calls a function pointer held in a global variable if it is set, then calls a helper with its argument.
- `release_global_record_resources`: Releases a group of resources held in a global record: it frees each sized entry in a loop, calls the optional hook, unlocks and drops the lock entries, then frees the record itself.
- `empty_return_routine`: This routine executes three instructions and then returns without modifying any data or registers.
- `release_stream_entry`: Releases a stream entry by closing its handle and clearing the entry, returning whether the entry was previously active.
- `compute_partial_product_high_word`: Combines 16-bit partial products of two register operands into a high word, and returns it together with the second operand unchanged as the low word.
- `dispatch_via_jump_table`: Dispatches through a function pointer stored in a jump table and returns.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_dir`: Wrapper that calls the dos library function CreateDir and returns its result to the caller.
- `invoke_table_routine`: Loads an address from a slot in a global jump table using a negative offset, then invokes the routine stored there and returns.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_indirect_routine_12`: Indirectly invokes the routine stored at the address held four bytes before a global variable and then returns.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_execute`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_indirect_function_pointer`: This routine invokes a function pointer stored in a global variable and returns its result.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open_wrapper`: Fetches a routine address stored in a global variable and invokes it, returning whatever that routine produces to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_parent_dir`: Wrapper that calls the dos library function ParentDir and returns its result to the caller.
- `invoke_table_function`: It invokes a routine whose address is stored in a slot of a function pointer table, then returns.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `dispatch_table_routine`: It looks up an entry in a table of routine pointers indexed by a value stored at a global variable and then jumps to that routine to execute it.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_jump_vector_414_2`: Calls one Amiga library function through a jump vector at offset minus 414 from a global base.
- `call_indirect_routine_13`: It loads a pointer from a global variable and invokes the routine stored there, effectively dispatching to a function held in a table.
- `call_library_jump_vector_198_2`: Calls one Amiga library function through a jump vector at offset minus 198 from a global base.
- `call_library_jump_vector_198_3`: Calls one Amiga library function through a jump vector at offset minus 198 from a global base.
- `call_library_jump_vector_294_2`: Calls one Amiga library function through a jump vector at offset minus 294 from a global base.
- `call_indexed_routine`: Looks up an entry in a table indexed by a value stored in a global variable and then invokes the routine found at that table slot.
- `call_library_jump_vector_132`: Calls one Amiga library function through a jump vector at offset minus 132 from a global base.
- `call_library_jump_vector_210_2`: Calls one Amiga library function through a jump vector at offset minus 210 from a global base.
- `invoke_indirect_routine_2`: Reads a 32-bit value from a global variable, indexes into a jump table at that offset, and invokes the routine found there.
- `call_library_jump_vector_372`: Calls one Amiga library function through a jump vector at offset minus 372 from a global base.
- `call_library_jump_vector_552_2`: Calls one Amiga library function through a jump vector at offset minus 552 from a global base.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_jump_vector_306`: Calls one Amiga library function through a jump vector at offset minus 306 from a global base.
- `call_library_jump_vector_384`: Calls one Amiga library function through a jump vector at offset minus 384 from a global base.
- `call_library_jump_vector_96_2`: Calls one Amiga library function through a jump vector at offset minus 96 from a global base.
- `call_library_jump_vector_90_2`: Calls one Amiga library function through a jump vector at offset minus 90 from a global base.
- `call_library_jump_vector_78_2`: Calls one Amiga library function through a jump vector at offset minus 78 from a global base.
