# Game 274 (KickOff2 v0.2 0760): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00274). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (685467 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 54 of 54 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 479 of 479 named (0 author symbols), 479 of 479 described (10 by rule, 468 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2457 of 2457 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6228 of 6228 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 360414 |
| code | 21104 |
| data | 303155 |
| padding | 794 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| KickOff2.Slave | 1 | 181 | 9 |
| logged disk load from Disk.1 | 1 | 24 | 3 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2674 | 197 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1364 | 122 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1964 | 141 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |

## Cases handled

- KickOff2_v0.2_0760/KickOff2/Disk.1: raw disk image (619520 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 48 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 920 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 420 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 972 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 16 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- KickOff2_v0.2_0760/KickOff2/Disk.1: 48 logged loads cover 346112 bytes of the image
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 6-21 (bytes 1536-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.20)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 646-649 (bytes 165376-166400) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.79)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 722-801 (bytes 184832-205312) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.43)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 914-1159 (bytes 233984-296960) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.51)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 1288-1401 (bytes 329728-358912) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.66)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 1494-1697 (bytes 382464-434688) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.88)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 1788-1989 (bytes 457728-509440) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.75)
- KickOff2_v0.2_0760/KickOff2/Disk.1: chunks 2218-2419 (bytes 567808-619520) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `load_audio_sample_one` (runtime segment an address): Calculates a memory address from a table and loads audio parameters into hardware registers for channel one. Kind: loader-based data-dependent pointer/checksum trap. Pass path: continuation of execution (the routine sets up a pointer and a length-dependent register/memory value, likely a checksum or a specific sample pointer required for subsequent audio-to-data-streamed-to-register logic or a checksum-verified lo.
- `setup_audio_channel_one_alt` (runtime segment an address): Similar to the previous routine, it retrieves audio parameters from a table based on an index. Kind: data-dependent pointer/register-based check. Pass path: the routine uses a complex, indirect pointer-chasing calculation (unaff_A4 + offset) to load a value into a register/memory, which then dictates the subsequent audio register settings; a successful check (valid data/track) results in the co.
- `setup_audio_channel_two_alt` (runtime segment an address): Retrieves audio parameters from a table based on an index. Kind: loader-based sabotage/config-dependent. Pass path: the routine executes normally; a failed check (incorrect pointer/register state) results in invalid pointer de-referencing or wrong audio register writes.
- `setup_sound_effect` (runtime segment an address): Calculates an address from a table and index, then configures audio hardware registers for volume, length, and data pointer, enabling the DMA channel. Kind: data-dependent pointer/offset-based check. Pass path: The routine calculates a pointer or offset (`iVar1`) using a base address (`unaff_A4`) and an offset derived from `in_D0w`. Because `iVar1` is then used to load a value into `_DAT_00dff0d4`, a successful check (correctly loaded/calculated d.
- `wait_and_clear_ciab_bit_2` (runtime segment an address): Waits for a global variable to reach zero, clearing a specific bit in it during the wait, while interacting with CIAA_PRA and CIAB_PRB hardware registers. Kind: disk timing/sync check (CIAB_PRB bit monitoring). Pass path: bne.b an address.
- `clear_ciab_bit_and_delay` (runtime segment an address): Clears a specific bit in a global variable associated with the CIAB PRB hardware register. Kind: track/sync check. Pass path: the loop exits via the sVar2 == sVar1 condition, effectively clearing the bit and then timing out or finishing a specific number of toggles/sync-checks.
- `toggle_ciab_port` (runtime segment an address): The routine modifies a global variable by setting and clearing specific bits, calls another routine between these modifications, and then executes a busy-wait loop. Kind: bit-toggling/timing check. Pass path: DAT_00bfd100 bit 1 cleared/set without loop exit interruption.
- `toggle_ciab_bit_with_delay` (runtime segment an address): Clears and sets a specific bit in a global variable associated with the CIAB PRB hardware register. Kind: CIAB_PRB_bit_toggle_timing_check. Pass path: bra.b an address.
- `wait_for_disk_ready_2` (runtime segment an address): Polls a global status variable in a loop, checking specific bits to determine disk readiness or error conditions. Kind: sync/track check (track/sync/ready check). Pass path: 0.
- `wait_for_flag_clear` (runtime segment an address): The routine checks a global variable in a loop, decrementing a counter each iteration. Kind: disk/track check (sync or track-length check). Pass path: the branch where the return value is 0 (success/flag cleared or certain condition met).
- `wait_for_disk_ready_3` (runtime segment an address): Polls a global variable in a loop, decrementing a counter, until the variable indicates disk readiness or the counter expires. Kind: track/sync check (wait for ready/ready-signal). Pass path: return 0 (success/ready_signal_detected).

## Functions


### KickOff2.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_disk_callback_sequence`: The routine executes function pointers from a structure passed in a register, clears a memory range starting at address 4096, and branches based on the return value of a callback.
- `initialize_and_loop_wait`: Calls several initialization routines, checks a global variable, and enters an infinite loop clearing a memory location if the variable is non-zero.
- `map_input_to_value`: Checks a word input against specific constants to set a global variable, then returns a word from a table indexed by that input.
- `store_word_via_offset`: Stores a word from the stack into a memory location calculated by adding an offset and a base pointer.
- `execute_disk_callback_3`: Executes function pointers from a structure passed in a register, clears a memory range starting at address 4096, and branches based on disk status.
- `execute_disk_callback_2`: Increments a global counter, calls a function pointer from a structure, and if it returns zero, invokes another function pointer with specific arguments and loops calling a third function pointer.
- `execute_disk_callback`: Calls a function pointer from a structure, and if it returns zero, invokes another function pointer with specific arguments and loops calling a third function pointer.
- `compute_vector_difference_loop`: Iterates through a fixed-size array of words, subtracting a running accumulator and a parallel source array from each element.

### logged disk load from Disk.1

- `copy_code_and_halt`: The routine sets global variables, copies a block of data from a code pointer to a global variable address in a loop, and then triggers a trap instruction.
- `copy_and_trap`: Copies a block of data from a source address to a destination address in a loop, then triggers a trap instruction after setting a global variable.
- `set_dma_control`: Writes a value to a global variable associated with the DMACON hardware register to enable DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_color_register`: Calls several routines to retrieve and process values, then writes to the COLOR00 hardware register and calls additional functions based on the result.
- `initialize_hunk_headers`: Writes multiple constant values to memory locations relative to a base pointer, likely setting up AmigaDOS hunk structures, and calls other routines during the process.
- `call_function_twice`: Invokes the same function pointer, derived from a base register offset, two times in succession.
- `configure_color_register`: Calls several routines to retrieve and process values, then writes to the COLOR00 hardware register and calls additional functions based on the results.
- `initialize_hunk_headers_loop`: Checks a flag and, if clear, calls several initialization routines and enters a loop that clears a local variable and calls another routine repeatedly.
- `toggle_bit_and_loop`: Sets a specific bit in a byte pointed to by a global pointer, calls another routine, clears that same bit, and repeats the cycle indefinitely.
- `poll_memory_status`: Polls a memory location for a specific bit to be set, with a timeout counter, and also checks another memory location for a negative value.
- `wait_for_disk_state`: The routine polls a pointer value in loops, checking for specific character values, and writes a constant to an offset before returning the pointer value.
- `wait_for_pointer_state`: The routine repeatedly checks a pointer value until it meets specific conditions, then writes a constant to an offset and loops while decrementing a counter based on bit flags.
- `wait_for_pointer_value`: Loops repeatedly checking a value pointed to by a register, returning when specific conditions on that value are met.
- `compare_pointer_value`: Loops checking a value pointed to by a register against an input byte and other conditions, returning when they match or fail.
- `write_memory_with_delay`: Writes specific bytes to three memory offsets repeatedly, waiting for a bit in one of those locations to clear between iterations.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `modify_byte_and_delay`: Modifies a byte via bitwise operations, calls another routine, then performs a loop writing bytes to memory offsets with delay checks.
- `write_memory_with_delay_2`: It modifies a byte via bitwise operations, then repeatedly writes specific bytes to three memory offsets, waiting for a bit in one location to clear between iterations.
- `modify_byte_and_delay_2`: The routine compares two values, modifies a byte via bitwise operations if needed, and then performs a loop writing bytes to memory offsets with delay checks.
- `write_memory_with_delay_2_2`: Modifies a byte via bitwise operations, then repeatedly writes specific bytes to three memory offsets, waiting for a bit in one location to clear.
- `dispatch_state_handler`: Calls a routine to retrieve a state value, stores it in memory, and then invokes a function pointer from a table indexed by that state value.
- `process_input_event_loop_2`: Polls for input events and dispatches to handlers based on the event code.
- `dispatch_input_event_loop`: Polls for input events in a loop, storing the result in a global variable, and dispatches to handlers based on the event code.
- `process_state_machine_loop`: Executes a nested loop that repeatedly calls subroutines and checks a global short variable for specific values to control flow.
- `handle_input_event_loop`: Polls for input events in a loop, storing the result in a global variable.
- `handle_game_state_transition_2`: Manages game state transitions by checking a global state variable and dispatching to specific handlers.
- `return_immediately`: The routine executes a return instruction to exit immediately.
- `indirect_jump_via_a4`: Calculates an address using a global variable and an offset, then performs an indirect jump to the code pointer stored at that location.
- `check_flag_and_loop`: The routine calls a function pointer to retrieve a status value and stores it in a global variable.
- `scan_string_and_invoke`: Iterates through a null-terminated string located at a fixed offset from A4, decrementing a counter until it reaches zero.
- `wait_for_disk_state_2`: Polls a pointer value in loops, checking for specific character values, and writes a constant to an offset before returning the result.
- `call_table_function`: Calls a single subroutine and then returns a value from the stack.
- `initialize_sequence_array`: Computes a base address from a global variable and a register, then writes the values 0, 1, 2, and 3 into consecutive word-sized slots, followed by a zero.
- `copy_three_words`: Computes a destination address from a global variable and a register, then copies three consecutive words from a source table into that destination.
- `scan_string_invoke`: Iterates through a null-terminated string located at a fixed offset from A4, decrementing a counter until it reaches zero.
- `set_flag_and_pointer_2`: It iterates through a structure array setting flags, then checks specific entries to set global pointers and update flag values based on their state.
- `set_flag_and_pointer`: Checks a short value in a structure; if non-zero, it stores a calculated address into a global variable and sets the short value to 2.
- `search_table_for_match`: Iterates backwards through a table of short values, comparing each entry to a register value, and returns upon finding a match or exhausting the table.
- `invoke_list_search_routine`: Calls a single subroutine that searches a linked list and invokes functions repeatedly.
- `process_game_state_updates`: The routine loops to update game state by calling function pointers and checking conditions on data structures.
- `update_game_state_loop_4`: Polls a global input state variable and dispatches to specific handlers based on event type codes.
- `process_input_event_loop_3`: Polls a global input state variable and dispatches to specific handlers based on event type codes.
- `update_game_state_loop_3`: The routine manages game state transitions by checking a global state variable and dispatching to specific handlers.
- `process_entity_state_loop_3_2`: Iterates through a linked list of entity structures, invoking callback routines to update their state.
- `update_game_state_loop`: The routine executes a continuous loop that calls function pointers to update game state and checks conditions on data structures.
- `process_entity_state_loop_3`: Iterates through a linked list of entity structures, invoking callback routines to update their state.
- `process_entity_state_updates`: Iterates through a linked list of entity structures, invoking function pointers to update coordinates and state, while checking conditions on data structures to manage game state.
- `update_entity_state_loop`: Iterates through a linked list of entity structures, invoking function pointers to update coordinates and state.
- `process_entity_state_loop_2`: Iterates through a linked list of entity structures, invoking function pointers to update coordinates and state.
- `process_entity_state_loop`: Iterates through a linked list of entity structures, invoking function pointers to update coordinates and state.
- `update_game_state_loop_2`: Iterates through a linked list of structures and updates game state by calling function pointers and checking conditions on data structures.
- `return_immediately_2`: The routine is empty and returns immediately without performing any operations.
- `search_linked_list_entry`: Iterates through a linked list of structures by incrementing a pointer until a specific field matches a register value, then calls another routine twice.
- `search_linked_list_for_id_2`: Calls a routine to iterate through a linked list of structures and another routine to process text input.
- `loop_seven_times`: Loops seven times, calling multiple other routines in each iteration, then returns.
- `execute_nested_loop_sequence`: Performs a nested loop structure that iterates seven times in the outer loop and seven times in the inner loop, calling other routines in each iteration.
- `clear_three_globals`: Sets three distinct global variables, accessed via offsets from a base pointer, to zero.
- `increment_and_wrap_counter`: The routine increments a short value at a stack offset and resets it to zero if it exceeds a specific threshold.
- `stub_placeholder`: The routine is an empty stub containing only entry and exit instructions.
- `call_list_state_iterator`: Invokes a routine that iterates through a linked list of structures to check state values and coordinate bounds.
- `loop_through_list_states`: Iterates through a linked list of structures, checking state values and coordinate bounds.
- `search_list_and_return_coords`: Searches a linked list for a specific ID and retrieves coordinate values.
- `return_stack_values`: Returns two short values located on the stack as a combined 64-bit result in registers D0 and D1.
- `process_entity_state_loop_4`: Iterates through a list of entities using a linked list pointer, checking state flags at specific offsets.
- `update_entity_state_table`: Iterates through a linked list of entities, checking state flags and modifying values in a coordinate table based on specific state codes.
- `search_list_increment_coord`: Searches a linked list for an ID, copies coordinate values, and increments a value in a table while looping through state checks.
- `search_list_adjust_coord`: Searches a linked list for an ID, retrieves coordinate values, and adjusts a value in a table by incrementing or decrementing it while looping through state checks.
- `search_list_increment_coords`: Searches a linked list for a specific ID, retrieves coordinate values, and increments a value in a table while looping through state checks.
- `search_list_decrement_coords`: Searches a linked list for an ID, copies coordinate values, and decrements a value in a table while looping through state checks.
- `tiny_move_rts`: Small routine of 2 instructions that copies d4 to d0, then returns.
- `search_list_update_coords`: Searches a linked list for an ID, copies coordinate values from the found structure to local variables, and invokes several function pointers.
- `jump_via_a4_table_2`: It calls a function pointer from a table, stores the result, and conditionally calls other function pointers based on the result value before returning.
- `calculate_signed_offsets`: Loads a value from a table indexed by a register, multiplies it by a modulo-derived constant, and stores the result into two global variables.
- `handle_game_state_transition`: The routine manages game state transitions by checking a global state variable and dispatching to specific handlers.
- `process_game_state_loop`: The routine manages a game state loop by checking a global state variable and dispatching to specific handlers.
- `process_loaded_file_data`: The routine processes a filename string by truncating it at the extension and checking its length against a threshold.
- `process_file_load_dialog`: Displays a file selection interface by parsing directory entries and handling user input to select a filename.
- `process_string_command_table_2`: Iterates through a string command table, parsing characters and invoking function pointers from a global table to update game state.
- `process_input_command_loop`: Iterates through a string command table to parse characters and invoke function pointers from a global table to update game state.
- `parse_string_command_table`: Iterates through a string command table, parsing characters and invoking function pointers from a global table to update game state.
- `process_string_command_table`: Iterates through a string command table, parsing characters and invoking function pointers from a global table to update game state.
- `return_stack_value`: Returns the 64-bit value found at the top of the stack without performing any other operations or modifying memory.
- `process_string_commands`: Initializes memory, then loops to call a function pointer from a table multiple times, interspersed with calls to routines that parse strings, check status, and search linked lists.
- `store_adjusted_state`: Calls another routine to get a value, subtracts a constant from it, and stores the result at a fixed offset from a global pointer.
- `wait_and_dispatch_status`: Waits for a status variable to change and then dispatches control flow based on the new status.
- `process_and_reset_table`: Iterates through four integers in a memory block, calling a routine if a value exceeds a threshold.
- `process_menu_string_list`: Iterates through a null-terminated list of strings to calculate their count, then processes each entry by calling external routines with specific offsets and parameters.
- `process_menu_selection_loop_2`: Polls input events and updates internal state variables based on event codes, navigating through menu options by iterating through a data table and invoking specific routines for each entry.
- `return_stack_word_2`: The routine calls several function pointers from a table with specific arguments and then returns the 32-bit value found at the top of the stack.
- `call_single_routine`: Invokes a single external routine and returns a 64-bit value constructed from a constant and the stack content.
- `call_two_function_pointers`: Executes two routines via function pointers stored at fixed offsets from a base register, then invokes another routine and returns a constructed 64-bit value.
- `return_stack_word_3`: It returns a 64-bit value constructed from a constant and the 32-bit value found at the top of the stack.
- `return_stack_word`: Returns the 32-bit value found at the top of the stack without performing any other operations or modifying memory.
- `jump_via_a4_table`: Performs an indirect jump to a function pointer retrieved from a table offset by the A4 register.
- `store_word_to_offset`: Stores a 16-bit value from a register into a global variable located at a positive offset from the base register.
- `clear_flag_and_return`: Sets a 16-bit value at a negative offset from the base register to zero and returns the 32-bit value located at another negative offset.
- `branch_on_global_zero`: Calls a routine, then checks a global integer variable.
- `merge_bitfields_into_table`: Saves processor registers to the stack and calculates an offset into a global table based on input parameters.
- `call_function_and_increment_counter`: Invokes a function pointer stored in a global table.
- `call_indirect_function`: Loads a function pointer from a global table offset by a base register and executes it.
- `check_character_range`: Examines a byte in a register to determine if it falls within specific ASCII ranges, excluding letters and certain symbols.
- `check_character_range_2`: The routine examines a byte to determine if it falls within specific ASCII ranges, excluding letters and certain symbols, and returns based on the result.
- `tiny_cmp_rts`: Small routine of 2 instructions that compares d0 with d0, then returns.
- `set_dimensions_based_on_flag`: Checks a short flag to determine values for two adjacent word-sized variables.
- `parse_string_with_delimiters`: It processes a string by copying characters to a destination buffer, handling spaces, tildes, parentheses, and null terminators to parse segments.
- `parse_string_with_delimiters_2`: Processes a string by copying characters to a destination buffer, handling spaces, tildes, parentheses, and null terminators to parse segments.
- `invoke_table_and_loop_string`: Calls a function pointer from a global table, stores the result, and iterates through a null-terminated string while calling another routine for each character.
- `swap_byte_via_table_call`: Swaps a byte between a structure offset and a global memory location, invoking a routine from a global table in between the swap operations.
- `invoke_table_and_loop_string_2`: Calls a function pointer from a global table, stores the result, and iterates through a null-terminated string while calling another routine.
- `swap_byte_via_pointer`: Conditionally swaps a byte between a global variable and a structure field using a temporary register, then calls another routine.
- `parse_string_loop_table_calls`: Processes a string by iterating through characters and invoking routines from a global table, while managing dimension flags and string parsing.
- `execute_indirect_function_pointer`: Calls an indirect function pointer from a global table, performs conditional checks on a base register, and invokes several other routines.
- `search_list_and_update_coords`: Searches a linked list for an ID, copies coordinate values from a found structure to local variables, and invokes several function pointers from a global table.
- `copy_coordinates_and_call_functions`: Calls a function pointer, sets a word variable to 2, and copies two word-sized values from a source structure to a destination structure.
- `search_list_and_invoke_repeatedly`: Searches a linked list for an ID, copies coordinate values, and repeatedly calls a specific function pointer from a global table with the search result.
- `copy_coordinates_and_repeat_call`: Calls a function pointer and copies two word-sized values from a source structure to a destination structure.
- `check_status_and_dispatch`: Compares a returned status value against a global variable to determine control flow.
- `process_input_event_list`: Iterates through a linked list of input events, parsing their data and copying payload bytes into a buffer until a termination condition is met.
- `call_init_process_list`: Invokes an indirect function pointer from a global offset, then iterates through a linked list of structures calling a routine for each node, and finally invokes a second indirect function pointer.
- `iterate_list_until_fail`: Traverses a linked list of structures, calling an indirect function on each node until the function returns a negative status or the list ends, then calls a final indirect function and returns its result.
- `traverse_list_check_bounds_2`: Iterates through a linked list of structures, checking if their coordinates fall within bounds defined by local variables.
- `traverse_list_check_bounds`: Iterates through a linked list of structures, checking if their coordinates fall within bounds defined by local variables, and stores a flag from the last checked structure.
- `store_word_to_global`: Stores a word value from a register into a specific global memory location.
- `search_linked_list_for_id`: Iterates through a linked list of structures starting from a global pointer.
- `process_structure_with_conditions`: Evaluates multiple fields within a structure to determine execution paths, copying coordinates or calling indirect functions based on signed and unsigned comparisons.
- `increment_coords_and_call`: Copies coordinates from a structure, increments them, and calls function pointers from a global table, conditionally invoking another routine based on a byte value.
- `wait_and_check_status`: Waits in a loop until a specific short variable becomes non-zero.
- `process_text_input_loop_2_2`: Reads characters from an input source into a buffer, handling backspace and enter key events.
- `process_text_input_loop_2`: Manages a character buffer by appending valid input codes or removing characters on backspace events.
- `process_text_input_loop`: Manages a character buffer by appending valid input codes or removing characters on backspace events.
- `process_input_buffer_loop`: Manages a character buffer, processes input events in a loop, and calls function pointers from a global table, handling specific key codes to control flow.
- `execute_two_helpers`: Calls two distinct helper routines in succession and returns without a value.
- `call_two_routines_sequentially`: Invokes two other routines in sequence without performing any local data manipulation or conditional logic.
- `copy_string_with_limit`: Copies characters from a source pointer to a destination pointer up to 12 characters or until a null terminator or period is encountered.
- `initialize_hunk_headers_loop_2`: Checks a flag and, if clear, calls several initialization routines and enters a loop that clears a local variable and calls another routine.
- `store_word_to_global_2`: Stores a 16-bit value from a register into a global variable at a specific negative offset from a base pointer.
- `store_word_to_offset_2`: Stores a word value from a register into a global variable at a specific negative offset from a base pointer.
- `compare_strings_with_wildcard_2`: Compares two strings character by character, treating asterisks as wildcards.
- `compare_strings_with_wildcard`: Compares two strings character by character, treating asterisks as wildcards, and returns zero if they match or -1 if they differ.
- `return_constant_minus_one`: Returns the constant value -1 (a fixed address) to the caller without performing any other operations.
- `invoke_three_helper_routines`: Calls three distinct helper routines in succession and returns the combined result from registers D0 and D1.
- `loop_call_helper_n_times`: Reads a signed short from a global offset, decrements it, and loops calling a helper routine while the counter is non-negative, then returns the contents of registers D0 and D1.
- `increment_coords_and_call_2`: Copies coordinates from a structure, increments them, and calls function pointers from a global table.
- `process_menu_selection_state`: Implements a state machine loop that polls for input events and updates internal state variables based on event codes, navigating through menu options or text input fields.
- `process_menu_input_loop_2`: Implements a state machine loop that polls for input events and updates internal state variables based on event codes.
- `process_menu_selection`: The routine manages a state machine loop that processes input events to navigate a menu structure.
- `process_input_state_machine_2`: Implements a nested state machine loop that processes input or event codes by calling external routines and updating global state variables.
- `process_input_state_machine`: Implements a nested state machine loop that processes input or event codes by calling external routines and updating global state variables.
- `process_input_state_machine_4`: Implements a state machine loop that polls for input events and updates internal state variables based on event codes.
- `process_game_event_loop`: The routine implements a state machine that processes game events by checking a status code and updating internal state variables.
- `process_game_input_state`: Manages a state machine loop that processes input events and updates game coordinates by calling a list search routine.
- `process_input_event_loop`: Polls an input source in a loop, dispatching actions based on event codes such as key presses or state changes.
- `process_menu_selection_loop`: Manages a state machine loop that polls for input events and navigates through menu options by updating state variables and calling subroutines based on return codes.
- `clear_game_state_fields`: Sets a byte and a word at specific negative offsets from a base pointer to zero.
- `process_input_state_machine_3`: Manages a complex state machine loop that processes input or game events by calling various subroutines based on return codes.
- `process_menu_input_loop`: Polls for input events in a loop, updating state variables based on key presses and calling subroutines to handle navigation or selection.
- `clear_two_global_fields`: Sets a byte and a word at specific negative offsets from a base pointer to zero.
- `truncate_strings_and_call`: Scans two string pointers for a period character, truncating the second string at the period if found.
- `chain_six_routine_calls`: Executes a sequence of six routine calls without local data modification or conditional branching.
- `loop_with_delay_counter`: Invokes several routines, executes a decrementing loop from a specific constant value, and calls additional routines after the loop completes.
- `search_list_and_increment`: Iterates through a linked list of structures starting from a global pointer to find a specific ID. It then copies coordinates from the found structure, increments them, and calls function pointers from a global table.
- `execute_six_routine_sequence`: Calls six other routines in a fixed order without performing local calculations or conditional checks.
- `check_status_and_dispatch_2`: Compares a returned status value against a global variable to determine control flow.
- `check_status_and_dispatch_3`: Calls a routine to wait for a status variable to change, then calls another routine to compare the status and determine control flow.
- `loop_until_status_match`: Repeats a routine call in a nested loop structure until a returned status value matches specific criteria, then returns a constructed result.
- `loop_forever`: The routine consists of an infinite loop with no side effects.
- `check_lowercase_letter`: Checks if a byte value represents a lowercase ASCII letter (between 'a' and 'z').
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_string_termination`: Iterates through a string pointed to by a stack-relative pointer to find its null terminator.
- `append_char_to_string`: Finds the null terminator of a string stored at a global offset and appends a character from a register, followed by a new null terminator.
- `check_string_and_condition`: Iterates through a string pointed to by a stack-relative pointer to find its null terminator.
- `find_terminator_and_convert`: Iterates through a string pointed to by a stack-relative pointer to find its null terminator.
- `convert_integer_to_string`: Converts a 32-bit integer into a decimal ASCII string representation.
- `convert_int_to_decimal_string`: Converts a 32-bit integer into a decimal ASCII string representation, storing the result at a pointer location.
- `concatenate_strings`: Appends the contents of a source string to the end of a destination string, preserving the null terminator.
- `copy_string_to_buffer`: Copies a null-terminated string from a source pointer to a destination pointer, including the null terminator.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer up to a specified count or until a null terminator is encountered.
- `dispatch_indirect_function_calls`: Checks a global pointer for null and recursively calls itself if empty.
- `copy_string_to_buffer_2`: Copies a null-terminated string from a source pointer to a destination pointer, including the null terminator.
- `parse_table_entry`: Executes an indirect function call from a global table, then iterates through a data structure using modulo arithmetic on a global variable.
- `indirect_call_and_store`: Executes a function pointer retrieved from a global table offset, then calculates an index using a modulo operation on a global variable and a byte value.
- `loop_while_flag_nonzero`: Calls an indirect function, loops while a specific register flag is non-zero, and then calls additional routines after the loop terminates.
- `call_indirect_function_2`: Calls a function pointer located at a fixed offset from a base address register.
- `copy_string_and_parse_entry`: Copies a null-terminated string from a source pointer to a destination pointer, including the null terminator.
- `sum_array_and_decompress`: Iterates through a fixed number of integers pointed to by a global pointer, accumulating their sum into a register.
- `sum_array_elements`: Iterates through a fixed number of integers pointed to by a global pointer, accumulating their sum into a register.
- `empty_stub_2`: This routine is empty and performs no operations other than returning immediately.
- `search_string_array`: Iterates through an array of strings, comparing each one against a target.
- `copy_bytes_with_check`: Copies a specified number of bytes from a source pointer to a destination pointer.
- `decompress_bitstream_data`: Decompresses data by reading bits from a stream, refilling the bit buffer as needed.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_word_array_right`: Shifts a sequence of 16-bit words to the right by one bit position.
- `initialize_string_table`: Initializes a table of string pointers by setting entries to a default value.
- `check_external_condition`: Calls an external routine and checks its return value.
- `indirect_function_call`: Performs an indirect jump to a function pointer located at a specific offset from a global base address.
- `copy_block_of_bytes`: Copies a block of bytes from a source address to a destination address.
- `initialize_game_state_loop`: Calls a routine to update the game state and stores the result in a global variable.
- `clear_memory_block`: Clears a block of memory by setting words to zero.
- `copy_word_pairs`: Copies seven 16-bit word values from a source region in memory to a destination region.
- `call_recursive_routine`: Calls another routine and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_bit_and_subtract`: Clears a specific bit in a global byte variable based on a register index.
- `init_audio_dma_channels`: Configures audio DMA channels by setting a global variable and storing a pointer in a structure.
- `initialize_audio_system`: Sets a global function pointer and sequentially calls routines to reset audio channels, clear DMA channel two, and initialize DMA interrupts.
- `reset_audio_dma_channels`: Resets audio and DMA channels by clearing associated memory locations.
- `process_audio_channel_interrupts`: Iterates through four bits of a global status variable to check for audio channel interrupts.
- `initialize_audio_channel_zero`: Resets several memory locations associated with audio channel zero to zero or specific constants.
- `initialize_game_state`: Checks a flag to determine if initialization is needed.
- `setup_audio_channel_zero`: Conditionally configures audio channel zero by writing buffer pointers and control values to memory and hardware registers.
- `reset_audio_channel_zero`: Clears memory locations associated with audio channel zero to zero.
- `check_audio_channel_zero`: Checks the status of the first audio channel and calls a routine to process its command stream if active.
- `configure_dma_interrupts`: Writes to hardware registers to enable DMA channels and interrupts.
- `process_audio_command_stream_3`: Iterates through a linked list of audio commands stored in memory, interpreting values to trigger sound playback or set parameters via a hel.
- `process_audio_command_stream`: Iterates through a linked list of audio commands stored in memory, interpreting values to trigger sound playback or set parameters via a helper routine.
- `process_input_stream`: Reads a value from a pointer-based list and updates an index counter.
- `parse_script_token_stream`: Iterates through a table of signed 16-bit values using a pointer and index stored in the stack frame.
- `load_audio_sample_zero`: Calculates a memory address from a table and loads audio parameters into hardware registers for channel zero.
- `load_audio_sample_one`: Calculates a memory address from a table and loads audio parameters into hardware registers for channel one.
- `load_audio_sample_one_2`: Calculates a memory address from a table using an index derived from a register value.
- `increment_audio_counter`: Increments a word-sized counter stored in a local structure.
- `initialize_audio_channel_one`: Resets several memory locations associated with audio channel one to zero or specific constants.
- `setup_audio_channel_one_3`: Conditionally configures audio channel one by checking a flag in the stack frame.
- `setup_audio_channel_one`: Conditionally configures audio channel one by writing buffer pointers and control values to memory and hardware registers.
- `clear_dma_channel_one`: Resets the source and destination pointers for DMA channel one to zero.
- `check_audio_channel_one`: Checks the status of the second audio channel and calls a routine to process its script tokens if active.
- `disable_dma_interrupts`: Clears a status flag in the stack frame and conditionally writes to the DMACON and INTENA hardware registers.
- `process_audio_script_token`: Iterates through a linked list of audio script tokens, handling control codes to configure audio channels, update counters, or terminate playback based on token values.
- `process_audio_command_stream_2`: Iterates through a command table to configure audio channels, handling specific opcodes for sound effects, volume, and frequency by updating local state variables and calling audio parameter loaders.
- `process_script_token`: Iterates through a linked list of script tokens stored in a table, handling control codes and data values.
- `process_audio_script_token_2`: Iterates through a linked list of audio script tokens, handling control codes to configure audio channels, update counters, or terminate playback.
- `setup_audio_channel_one_2`: Calculates an address from a table and stack offset, then configures audio channel one by setting its period, length, volume, and control registers.
- `setup_audio_channel_one_alt`: Similar to the previous routine, it retrieves audio parameters from a table based on an index.
- `load_audio_channel_params`: Retrieves audio parameters from a table indexed by a register value and stores them into local variables.
- `increment_audio_one_counter`: Increments a 16-bit counter located on the stack.
- `clear_dma_channel_two`: Resets the source and destination pointers for DMA channel two to zero.
- `init_audio_channel_two`: Checks a flag to determine if initialization is needed; if so, it sets period, length, volume, and control registers for audio channel two and marks the flag as initialized.
- `initialize_audio_channel_two`: Checks a global flag; if zero, it initializes audio channel two by setting its period, length, volume, and control registers.
- `reset_audio_channel_two`: Clears the source and destination pointers for DMA channel two to zero.
- `check_audio_channel_two`: Checks the status of the third audio channel and calls a routine to process its script bytes if active.
- `configure_dma_interrupts_2`: Resets a flag and writes values to global variables that correspond to hardware registers for DMA control and interrupt enablement.
- `process_audio_script_byte`: Iterates through a linked list of signed audio script bytes, incrementing a counter and advancing the pointer.
- `process_audio_event_stream`: Iterates through a linked list of audio event parameters stored on the stack, interpreting values to update global state or fetch specific data from a global table based on index ranges.
- `parse_script_token_stream_2`: Iterates through a linked list of script tokens, handling pointer advancement and carry flags.
- `parse_script_byte_sequence`: Iterates through a table of signed bytes, incrementing a pointer and counter until a zero terminator is found.
- `setup_audio_channel_two`: Calculates an address from a table and stack offset, then configures audio channel two by setting its period, length, volume, and control registers.
- `setup_audio_channel_two_alt`: Retrieves audio parameters from a table based on an index.
- `fetch_audio_param_entry`: Calculates an address using an index offset into a global table and retrieves a two-byte value from that location.
- `increment_audio_two_counter`: Increments a 16-bit counter located on the stack.
- `initialize_dma_interrupts`: Clears several memory locations relative to a base pointer and writes specific values to global variables, while enabling DMA channels and interrupts via hardware registers.
- `initialize_audio_channel_state`: Checks a flag in a local structure to determine if initialization is needed.
- `configure_audio_channel`: Conditionally sets up audio channel parameters by writing to memory offsets and hardware registers for period, length, and volume, if a specific flag is zero.
- `reset_dma_pointers`: Zeros out multiple memory locations relative to a base pointer, effectively clearing DMA source and destination addresses and control flags.
- `check_audio_channel_three`: Checks the status of the fourth audio channel and calls a routine to process its token stream if active.
- `reset_dma_interrupt_flags`: Clears a flag in a local structure and writes specific values to global variables.
- `process_audio_token_stream`: Iterates through a linked list of signed short integers representing audio events, decoding values to trigger sound samples, update global state, or modify hardware registers based on specific token ranges.
- `process_audio_event_stream_2`: Iterates through a linked list of audio event codes stored in a stack buffer, decoding values to trigger specific sound samples or update global audio state variables based on the event type.
- `process_script_token_2`: Reads a token from a script pointer and updates the pointer based on carry.
- `parse_token_stream_loop`: Iterates through a pointer-based list of signed short integers, incrementing a counter and advancing the pointer until a zero terminator is found.
- `setup_sound_effect`: Calculates an address from a table and index, then configures audio hardware registers for volume, length, and data pointer, enabling the DMA channel.
- `trigger_audio_sample`: Retrieves audio parameters from a table using an index, configures the audio hardware registers for volume, length, and data pointer, and enables the DMA channel.
- `trigger_audio_sample_call`: Calculates an address by indexing into a table using a register value, retrieves a word from that address, and sets up several global variables with specific constants and data pointers.
- `increment_counter`: Increments a 16-bit value stored at a specific offset from a base pointer by one.
- `initialize_audio_channels`: Checks initialization flags for four audio channels and configures their buffer pointers, period, length, volume, and control registers if they are not yet set.
- `clear_audio_registers`: Zeros out two 16-bit memory locations and sets a third to a specific value, likely resetting audio control or status flags.
- `check_character_validity`: Validates a character value by checking if it falls within specific ASCII ranges or matches a character in a null-terminated string table.
- `check_char_in_string`: Checks if a register value is outside the ASCII digit range.
- `search_byte_in_buffer`: Iterates through a byte array until it finds a matching byte or a null terminator, returning early if a match is found.
- `tiny_move_addi_rts`: Small routine of 3 instructions that copies d4 to d2, then adds 36 to d2, then returns.
- `handle_script_token_dispatch`: Reads a byte from a global memory location and masks it with 15.
- `check_bit_flags_and_call_2`: Iterates through the first three bits of a global variable and conditionally calls another routine based on their state.
- `check_bit_flags_and_call`: Iterates through the first three bits of a global variable.
- `configure_blitter_transfer`: Calculates memory addresses and configures blitter registers for a data transfer, waiting for the blitter to idle before setting control words and enabling interrupts.
- `configure_blitter_registers`: Waits for the blitter to become idle, then writes specific values to the Blitter control registers and enables interrupts.
- `set_memory_values`: Writes specific 16-bit values to two distinct memory locations relative to a base pointer.
- `empty_stub_return`: This routine is empty and performs no operations, simply returning control to the caller.
- `empty_stub_3`: This routine is empty and performs no operations.
- `store_pointer_via_offset`: Stores a byte from a register into memory pointed to by a register, then updates a pointer in a global variable table with an incremented address.
- `iterate_string_dispatch`: Loops through a null-terminated string pointed to by a register, calling a dispatch routine for each character before returning a value.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies (a7)+ to d0, then returns.
- `process_script_tokens_2`: Executes a nested loop that calls a dispatch routine and performs arithmetic operations on registers, checking conditions against a global variable.
- `process_script_tokens`: Executes a nested loop that calls a dispatch routine and performs arithmetic operations on registers, checking conditions against a global variable and a memory location.
- `return_register_value`: Returns the value contained in a data register without modification.
- `execute_indirect_function_table`: Calls functions via a table of pointers, storing results in memory and conditionally invoking additional routines based on return values.
- `return_immediately_3`: The routine executes a minimal sequence of instructions before returning control to the caller.
- `clear_two_word_globals`: Sets two word-sized values in memory to zero and then calls another routine.
- `process_memory_block_loop`: Checks a global status variable and, if positive, iterates through a memory block using nested loops to shift bytes based on conditional checks.
- `toggle_memory_bits_loop`: The routine enters an infinite loop that repeatedly sets and clears specific bits in a memory location pointed to by a register.
- `wait_for_copper_bit`: Polls a byte in memory, waiting for a specific bit to clear or a counter to expire, while also checking a short value.
- `poll_serial_port_status`: Polls a memory-mapped serial port register until specific status bytes are received, handling timeouts and writing configuration values to the port.
- `poll_memory_until_negative`: Repeatedly polls a memory location until the value becomes negative, then writes a value to an offset and continues polling with a timeout.
- `wait_for_negative_byte`: Polls a memory location pointed to by a register, looping until the byte value becomes negative or matches specific conditions.
- `wait_for_specific_byte`: Polls a memory location pointed to by a register, looping until the byte value matches a specific input value or becomes negative.
- `write_copper_list_loop`: Writes a sequence of bytes to memory offsets relative to a base pointer, waiting for a bit in a status register to clear between iterations.
- `return_immediately_4`: The routine is empty and returns immediately without performing any operations or modifying data.
- `toggle_bits_and_copper`: Sets and clears bits in a memory location, calls another routine, then writes a sequence of bytes to memory offsets while waiting for a status bit.
- `toggle_bit_and_delay`: Clears and sets a bit in a memory location, then performs a loop writing values to offsets and waiting for a bit to clear.
- `conditional_bit_toggle_loop`: Compares two values and conditionally toggles a bit in memory, then performs a loop writing values to offsets and waiting for a bit to clear.
- `toggle_bit_with_delay`: Clears a bit in a memory location, compares values, and performs a loop that writes specific bytes to memory offsets while waiting for a status bit to clear.
- `conditional_function_call`: Checks a word value; if it is less than one, it calls a function pointer stored in a global variable table.
- `dispatch_function_call`: Calls a function pointer from a table, stores the result, and conditionally invokes other function pointers or jumps to an indirect address based on the return value.
- `dispatch_function_call_variant`: Calls a function pointer from a table, stores the result, and conditionally invokes other function pointers or jumps to an indirect address based on the return value, using a different offset for one argument.
- `dispatch_function_call_2`: Calls a function pointer from a table, stores the result, and conditionally invokes other function pointers or jumps to an indirect address.
- `sync_memory_word_pairs`: Compares pairs of words in memory, updating one if they differ, and then copies several other word values to new locations.
- `process_script_loop_conditionally`: Checks a register flag to decide between initializing local variables and executing a loop that performs arithmetic modulo operations and conditional calls, or zeroing memory offsets and calling another routine based on a global variable check.
- `process_script_tokens_loop`: Initializes local variables and executes a nested loop that performs arithmetic operations and condition checks before calling a dispatch routine.
- `update_copper_list_state_3`: Calls a function pointer with a constant argument, checks the return value, and conditionally invokes other function pointers or updates memory based on the result.
- `set_script_token_values`: Writes constant values to specific memory offsets relative to a base pointer, selects a value based on a comparison of two global variables, and then calls another routine three times in sequence.
- `handle_script_token_dispatch_2`: Sets a local variable to a constant value and then calls three other routines in sequence.
- `clear_global_word`: Sets a word-sized value in a global variable table to zero.
- `update_copper_list_state`: The routine checks a global state variable and, if changed, executes a sequence of function pointers to process data.
- `set_and_clear_bits`: Sets a specific bit in a byte, calls another routine, clears a different bit in the same byte, and repeats this sequence in a loop.
- `wait_for_flag_clear_2`: Polls a byte pointer for a specific bit to clear, using a decrementing counter loop.
- `check_pointer_and_set_flag_2`: Repeatedly checks if a word value at a pointer is greater than -1.
- `check_pointer_and_set_flag`: Repeatedly checks if a word value at a pointer is greater than -1.
- `loop_check_pointer_value`: Repeatedly checks if the value pointed to by a register is greater than -1 in a tight loop.
- `loop_compare_input_byte`: Similar to R1, it loops checking if a pointer's value is greater than -1.
- `write_memory_and_wait`: Writes specific byte values to three offsets from a base pointer.
- `empty_return_stub`: This routine is empty and immediately returns without performing any operations.
- `modify_flags_and_wait`: Sets and clears specific bits in a byte pointed to by a register, calling other routines in between.
- `toggle_bit_and_delay_2`: Clears and sets a specific bit in a byte pointed to by a register.
- `compare_and_delay_loop`: Compares two word values, calling other routines to update them if they are unequal.
- `write_copper_list_and_wait`: Clears a status bit, then writes specific values to three memory locations offset by 1024, 1280, and 3584 from a base pointer in a loop.
- `update_copper_list_state_2`: Checks a global state variable and, if changed, executes a sequence of function pointers to process data, including updating memory blocks with specific values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields`: Initializes multiple word-sized fields in a memory structure pointed to by a register with zero or maximum word values.
- `clear_stack_variables`: Initializes multiple word and longword-sized variables on the stack to zero or 65535.
- `scan_table_for_tac`: Iterates through a table of pointers, checking each entry for the string '.tac'. If found, it calls a function pointer and decrements a counter.
- `process_color_register_data_2`: Processes data based on a value retrieved from a table, updating structure fields and iterating through a list of items to modify their properties.
- `invert_word_values`: Iterates through a block of memory, subtracting word values at specific offsets from constant values (1392 and 912).
- `call_function_and_terminate_string`: Calls a function pointer located at a fixed offset from a base pointer.
- `call_function_pointer`: Calls a function pointer located at a specific offset from a base pointer.
- `process_color_register_data`: The routine processes data based on a value retrieved from a table.
- `fill_memory_and_process_string`: Iterates a loop calling a function pointer with incrementing arguments, fills memory with specific word values, truncates a string at a period character, and conditionally calls another routine.
- `loop_call_and_fill_memory`: Iterates a counter, calling a function pointer with specific arguments and checking the return value.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `calculate_disk_sector_address`: Calculates a disk sector address based on input parameters and checks if the result is within a valid range.
- `wait_for_disk_operation_3`: Calls a routine to wait for disk readiness and another routine to write a value to the CIAB port register.
- `wait_for_disk_operation_2`: Loops up to six times, calling routines to wait for CIA port changes, decode tile data, and check disk readiness, returning a value upon completion.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `copy_block_of_data`: Copies blocks of data from a source pointer to a destination pointer in a nested loop structure.
- `call_single_helper_routine`: Calls a single unnamed routine and returns immediately.
- `set_ciab_port_value`: Writes a specific byte value to a global variable associated with the CIAB PRB hardware register.
- `return_immediately_5`: The routine is empty and returns immediately to the caller without performing any operations.
- `wait_for_cia_port_change`: Manipulates a global variable and enters a loop that waits for a specific bit condition while clearing bits.
- `decode_game_state`: The routine manipulates a global variable by incrementing it and applying bitwise logic, then enters a loop that calls another routine based on the variable's sign and bit patterns.
- `wait_and_clear_ciab_bit_2`: Waits for a global variable to reach zero, clearing a specific bit in it during the wait, while interacting with CIAA_PRA and CIAB_PRB hardware registers.
- `wait_and_clear_ciab_bit`: The routine waits for a global variable to reach zero, clearing a specific bit in it during the wait.
- `clear_ciab_bit_and_delay`: Clears a specific bit in a global variable associated with the CIAB PRB hardware register.
- `execute_busy_wait`: The routine initializes a counter and decrements it in a loop until it reaches a specific value, effectively creating a delay without performing other operations.
- `toggle_ciab_port`: The routine modifies a global variable by setting and clearing specific bits, calls another routine between these modifications, and then executes a busy-wait loop.
- `toggle_ciab_bit_with_delay`: Clears and sets a specific bit in a global variable associated with the CIAB PRB hardware register.
- `return_immediately_6`: The routine is empty and returns immediately without performing any operations or modifying any data.
- `wait_for_disk_operation`: Loops up to six times, calling routines to wait for CIA port changes, decode tile data, and check disk readiness, returning a value upon completion or error.
- `decode_and_merge_tile_data`: Scans a memory block for a specific magic header, then performs a bitwise interleaving merge of two data streams into a destination buffer based on a bitmask condition.
- `check_register_value`: The routine checks the value of a register and returns a specific constant if the register matches a condition, otherwise it returns a different constant.
- `xor_memory_block_and_compare`: Computes the XOR sum of a block of words in memory pointed to by a register.
- `compute_bitwise_result`: The routine performs bitwise operations on two values, one from a register and one from a memory location pointed to by another register, and returns the combined result.
- `wait_for_disk_ready_2`: Polls a global status variable in a loop, checking specific bits to determine disk readiness or error conditions.
- `wait_for_flag_clear`: The routine checks a global variable in a loop, decrementing a counter each iteration.
- `wait_for_disk_status_bit`: Checks a global variable to determine a base address and waits for a specific bit in the disk status register to be set.
- `fill_memory_with_pattern`: Fills a block of memory with a repeating 32-bit pattern.
- `process_tile_attribute_loop`: Sets a global variable to a specific pattern and calls a routine to adjust tile attributes in a loop.
- `xor_checksum_table`: The routine iterates over a table of 32-bit words, XOR-accumulating them into a running value.
- `configure_blitter_dma_transfer`: Waits for the blitter to be idle, then configures blitter registers for a DMA transfer by setting source, destination, and size parameters.
- `adjust_tile_attribute`: Reads a byte from memory and modifies its high bit based on the value of a preceding byte.
- `toggle_input_flag_bit`: Reads a byte from a pointer and conditionally sets or clears the high bit based on a flag in the preceding byte.
- `store_masked_byte`: The routine takes a byte from a register, masks it to clear the most significant bit, and stores the result through a pointer.
- `apply_tile_mask`: The routine computes a mask from the input value, combines it with the high bits of the input, and stores the result through a pointer.
- `wait_for_disk_ready_3`: Polls a global variable in a loop, decrementing a counter, until the variable indicates disk readiness or the counter expires.
- `wait_for_disk_ready`: Polls a global variable in a loop, decrementing a counter, until the variable indicates disk readiness or the counter expires.
- `setup_dma_and_input`: Writes to the DMACON register to enable DMA channels and calls routines to wait for raster lines, process input, read joystick ports, and initialize keyboard interrupts.
- `read_joystick_button`: Reads the CIAA parallel port register to check for joystick or mouse button presses.
- `return_stack_value_2`: Returns a 64-bit value constructed from a constant high word and a value from the stack.
- `check_input_state`: Accesses the CIAA parallel port register to sample input devices.
- `return_sign_extended_word`: Returns a 32-bit value constructed by sign-extending a 16-bit word from the stack.
- `validate_input_result`: Resets a global variable and calls a helper routine to process input from the CIAA parallel port.
- `clear_input_flag`: Calls a routine to process input from the CIAA parallel port.
- `return_stack_value_3`: The routine returns a 32-bit value formed by combining a constant high word of all ones with a value from the stack.
- `empty_stub_4`: This routine is empty and immediately returns without performing any operations or modifying any data.
- `read_input_loop`: Iterates through a counter while calling helper routines to sample input.
- `allocate_memory_block`: Searches a linked list of memory blocks for an empty entry or a suitable size, shifts data to make space, and initializes the new block with copied data.
- `allocate_block_entry`: Searches a table for an empty entry, shifts a word value, and copies data into a new block structure.
- `copy_block_data`: Shifts a word value and stores it at an offset in a memory block.
- `return_error_code`: Immediately returns a constant value of -1 (a fixed address) without performing any other operations.
- `process_input_result`: Resets a global variable, calls a helper to process CIAA parallel port input, and returns a combined status value based on the result.
- `copy_data_from_table`: Copies bytes from a table indexed by a global variable into a destination buffer.
- `update_global_offset`: Calculates a new value for a global variable based on a division of an input parameter.
- `copy_data_to_table`: Copies bytes from a source buffer into a table indexed by a global variable.
- `check_input_state_2`: Samples the CIAA parallel port register to read the current state of input devices.
- `validate_table_index`: Checks a global variable against conditions involving table lookups and sets the variable to -1 if validation fails.
- `update_table_entry`: Copies data from a source buffer into a table indexed by a global variable, checks input state, and sets the global variable to -1 on failure.
- `set_global_to_minus_one`: Stores the value 65535 into a global variable.
- `process_input_flags`: Checks a global variable for input status, clears specific bits if set, and processes input data from the CIAA parallel port.
- `clear_input_buffer_2`: Validates input results, shifts data in a memory block, zeros out specific memory locations, and returns a combined status value.
- `clear_input_buffer`: Zeros out a block of memory pointed to by a register and processes input flags from the CIAA parallel port.
- `return_combined_value`: Returns a 64-bit value constructed from a constant -1 in the upper word and a value from the stack in the lower word.
- `insert_into_sorted_table`: Iterates through a linked list structure in memory to find an insertion point based on a parameter value.
- `update_lfsr_state`: Iterates through a table of word-sized variables, shifting them left and inserting new bits derived from XOR operations to update a linear feedback shift register state.
- `empty_color00_stub`: This routine is empty and performs no operations, despite being associated with the COLOR00 hardware register in the analysis metadata.
- `empty_custom_stub`: This routine is empty and performs no operations, despite being associated with the custom chip base address in the analysis metadata.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_for_dma_completion`: Polls a global variable in a loop until it becomes zero.
- `configure_copper_blitter_dma`: The routine configures the Amiga custom chip registers for the Blitter and Copper, setting up DMA control and window dimensions.
- `check_table_index_validity`: The routine checks if a register value is less than one.
- `clamp_global_variable`: Checks a global variable against upper and lower bounds (0 and 199 or 319).
- `empty_return_stub_2`: This routine is empty and immediately returns without performing any operations or modifying state.
- `wait_for_timer`: Executes nested loops decrementing counters to simulate a delay, likely waiting for a hardware timer or process completion.
- `configure_blitter_copy_2`: Sets up the Blitter hardware registers for a memory block transfer, calculating source and destination addresses, sizes, and enabling DMA channels.
- `configure_blitter_copy`: Calculates source and destination addresses and sizes for a memory block transfer.
- `process_input_and_delay`: The routine processes input results and validates table indices.
- `configure_blitter_fill`: Calculates addresses and sizes for a memory fill operation.
- `read_joystick_inputs_2`: Reads data from the CIA and joystick ports (JOY0DAT, JOY1DAT) and configures interrupt enable registers.
- `enable_dma_channels`: Writes to the DMACON register to enable DMA channels, specifically setting bit 15 to activate the requested channels.
- `wait_for_global_flag`: Polls a global variable in a loop until it becomes non-zero.
- `read_joystick_inputs`: Reads data from the CIA and joystick ports (JOY0DAT, JOY1DAT).
- `return_immediately_7`: The routine is empty and returns immediately without performing any operations or accessing data.
- `update_pointer_arithmetic`: Calculates a new pointer address based on a global variable and updates two consecutive word-sized values in memory using bitwise shifts and additions.
- `initialize_keyboard_interrupts`: Configures the CIAA interrupt control register and serial data register.
- `wait_for_nonzero_global`: Polls a global variable in a loop until it becomes non-zero, then resets the variable to zero and returns the previously stored non-zero value.
- `lookup_dual_table_bytes`: Selects one of two base addresses based on the state of a global variable.
- `fetch_byte_from_offset`: Reads a byte from a memory location calculated by adding an offset to a base address and combines it with a register value to return a 64-bit result.
- `handle_keyboard_serial_data`: This routine processes keyboard input by reading from the CIAA serial data register and updating global state variables based on bit conditions.
- `handle_keyboard_interrupt_logic`: The routine manipulates a global variable by clearing a bit based on a register value, setting another bit, and then resetting the variable.
- `handle_keyboard_interrupt`: Checks a condition, updates global variables, calls a function, and manipulates CIAA registers to handle keyboard serial data.
- `set_global_variable_eight`: The routine sets a global variable to the value 8 and returns.
- `shift_lfsr_state`: Updates three global word-sized variables by shifting them left and inserting new bits derived from XOR operations on previous states, likely implementing a linear feedback shift register.
- `convert_signed_int_to_ascii`: Converts a signed integer in D0 into an ASCII string at the address in A2.
- `convert_integer_to_ascii`: Converts a signed integer into an ASCII string representation by repeatedly dividing by powers of ten and storing the resulting digits into a buffer.
- `copy_string_null_terminated`: Copies bytes from a source pointer to a destination pointer until a null terminator is encountered, effectively duplicating a null-terminated string.
- `copy_string_with_limit_3`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, ensuring the destination string is null-terminated.
- `search_table_entry`: Iterates through a table of entries, comparing strings and calling helper routines to find a matching entry, returning a calculated address upon success or invoking a jump table on failure.
- `copy_data_with_bounds`: Copies a specified number of bytes from a source pointer to a destination pointer, checking against a calculated upper bound to prevent overflow, and updates the source pointer.
- `adjust_offset_value`: Checks a condition on a register value and subtracts a modified constant from a global variable if the condition is met.
- `initialize_structure_data`: Initializes fields in a memory structure, iterates through a loop updating a value, and calls another routine to complete setup.
- `calculate_offset_and_wait`: Adds a fixed value to a global variable in a loop until a called routine returns a specific status code, then stores the result in another global variable and waits for the same status code again.
- `setup_dma_and_input_2`: Writes to the DMACON register to enable DMA channels and calls routines to wait for raster lines, process input, read joystick ports, and initialize memory buffers.
- `validate_input_result_2`: Resets a global variable and calls a helper routine to process input from the CIAA parallel port, returning a status code.
- `process_input_result_status`: The routine resets a global variable and calls a helper to process input.
- `copy_table_data`: Calculates an index based on parameters and a global variable, updates the global variable, and copies data from a table to a destination buffer.
- `validate_table_index_check`: The routine checks a global variable against several conditions involving table lookups.
- `check_global_and_update`: Checks a global variable against conditions and calls another routine; sets the global variable to -1 if the result is negative.
- `update_global_state_flags`: Checks a global variable for non-zero and positive values, then performs bitwise operations to update a field at an offset within that variable's structure.
- `shift_and_clear_buffer`: Validates input, shifts data in a memory block forward by four bytes, zeros out the end of the block, and returns a status value.
- `insert_into_sorted_table_2`: Iterates through a linked list structure in memory to find an insertion point based on a parameter value and inserts the new entry.
- `propagate_lfsr_chain`: Traverses a linked list of state structures starting from a global pointer until a specific target identifier is found.
- `return_immediately_10`: The routine is empty and performs no operations, simply returning control to the caller.
- `call_empty_color_stub`: This routine consists of a single instruction that invokes an empty stub associated with the COLOR00 hardware register.
- `call_empty_custom_stub`: This routine consists of a single instruction that invokes an empty stub routine associated with the custom chip base address.
- `wait_for_raster_range`: Busy-waits by polling a global variable until it falls within a specific numeric range, effectively synchronizing with the display beam position.
- `configure_dma_buffers`: Writes specific values to a series of memory locations, then loops to populate a buffer with calculated addresses and values, finally setting global variables and calling another routine.
- `setup_blitter_dma_loop`: Executes a loop three times, waiting for a bit in a global variable to clear.
- `configure_blitter_memory_pointers`: Writes a series of constant values to memory locations associated with Blitter configuration.
- `decode_packed_color_data`: Iterates through a block of memory, reading three bytes at a time.
- `expand_run_length_encoded_data`: Processes a stream of data using run-length encoding logic.
- `wait_and_copy_string`: The routine delays execution using nested countdown loops and then copies characters from a source pointer to a destination pointer until a null terminator or a counter limit is reached.
- `execute_nested_delays`: Executes an infinite loop containing nested decrementing counters to simulate a delay, likely waiting for a hardware timer.
- `iterate_bits_and_set_flag`: Iterates through bits of a combined value, checks a global flag, and sets a global variable to specific constants based on bit state and flag value.
- `configure_blitter_copy_2_2`: Sets up the Blitter hardware registers for a memory block transfer, calculating source and destination addresses, sizes, and enabling DMA ch.
- `process_input_loop_delay`: The routine loops through a counter, checking bit conditions in a global variable to determine control flow.
- `read_joystick_inputs_3`: The routine is empty and performs no operations.
- `set_global_and_enable_dma`: The routine sets a global variable to a specific value and then to zero, before calling a helper to enable DMA channels via the DMACON register.
- `enable_dma_channels_2`: The routine calls a helper function to enable DMA channels by writing to the DMACON register.
- `enable_dma_channels_3`: Enables specific DMA channels by writing to the DMACON register.
- `poll_and_clear_flag`: The routine polls a global variable in a loop until it becomes non-zero, then clears the variable and returns the value.
- `check_global_flag`: Checks if a global variable is non-zero and returns a specific constant value based on the result, while also calling a routine to initialize keyboard interrupts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_subroutine_chain`: A routine consisting of eleven instructions that calls another routine.
- `configure_dma_enable`: Sets a bit in a DMA control register to enable certain DMA channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_8`: The routine performs no operations and returns control to the caller immediately.
- `empty_routine`: The routine contains no instructions and returns immediately.
- `do_nothing`: The routine is empty and performs no operations.
- `do_nothing_stub`: The routine is empty and immediately returns without performing any operations.
- `return_immediately_9`: The routine is empty and returns immediately without performing any operations or accessing any data.
