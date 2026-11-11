# Game 2098 (tv sports football (1988)(cinemaware)[cr qtx][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02098). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1849236 top-level bytes in 101 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 48 of 48 units identical (bytes and relocations), 100 raw-word lines |
| Functions named and described | pass, 1110 of 1110 named (59 author symbols), 1110 of 1110 described (277 by rule, 815 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6323 of 6346 (99.64%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18312 of 18376 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 174874 |
| code | 59480 |
| data | 1474545 |
| padding | 23601 |
| unused | 116736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf:fb | 3 | 9208 | 471 |
| 8 | 1 | 2 | 1 |
| 7 | 1 | 16 | 2 |
| 8 | 1 | 2 | 1 |
| 7 | 1 | 2 | 1 |
| 9 | 1 | 25 | 2 |
| 10 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| 14 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| mathtrans.library | 17 | 830 | 97 |
| mathieeedoubbas.library | 3 | 632 | 60 |
| version.library | 2 | 41 | 7 |
| diskfont.library | 13 | 830 | 53 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 86 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1287 | 85 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2132 | 95 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 172 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 372 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 181 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 35 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 701 | 53 |
| logged disk load from TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf | 1 | 145 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 48 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 214 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: file KBlocks.1 (header 406205 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 2 of 2).adf: file KBlocks.0 (header 352150 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 2 of 2).adf: file KBlocks.1 (header 385002 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 64 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 285 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 938 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 119 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 248 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 176 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 72 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 35 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 216 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: 85 executed PCs whose RAM was overwritten before any snapshot match its bytes 75344-75856: recovered as a unit run at an address
- 245 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 7 first-run image unit(s) in 2 version group(s); 1262 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: blocks 895-896 (bytes 458240-459264) hold ASCII text
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: blocks 898-900 (bytes 459776-461312) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.50)
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: blocks 903-1234 (bytes 462336-632320) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.78)
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf: blocks 1236-1696 (bytes 632832-868864) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.91)
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 2 of 2).adf: blocks 15-710 (bytes 7680-364032) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.59)
- tv sports football (1988)(cinemaware)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 2 of 2).adf: blocks 895-1654 (bytes 458240-847360) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_disk_resource_loop` (runtime segment an address): Repeatedly calls a function on a disk resource pointer until it succeeds, then modifies hardware registers and a global variable. Kind: track/sector validation loop. Pass path: the loop continues (iVar2 == 0) until a non-zero value is returned, at which point it breaks to the post-check routine.
- `configure_hardware_and_call_subroutine` (runtime segment an address): Sets specific values in the custom chip registers and then calls a subroutine using pointers stored in registers before branching to another location. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `copy_state_byte_to_register` (runtime segment an address): Copies a state byte into a control word and writes a fixed flag value to a second control word. Kind: disk hardware/track check. Pass path: the branch following the call (the code continues execution normally).
- `start_disk_sync_transfer` (runtime segment an address): Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers. Kind: disk sync/track check. Pass path: execution continues to the next instruction (bsr.w an address).
- `invoke_disk_resource_loop` (runtime segment an address): Repeatedly calls a function located at an offset within a structure pointed to by a register until it returns a non-zero value, then calls another routine and updates a global variable and a hardware register. Kind: disk resource/track availability loop. Pass path: the loop breaks and proceeds to u39__sub_0342 when iVar2 != 0.
- `configure_hardware_and_call` (runtime segment an address): Writes values to custom hardware registers, loads two pointers into registers, and calls another routine. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `update_control_word_flag` (runtime segment an address): Copies a byte from a memory location into a global variable and then overwrites that same variable with a fixed bitmask value. Kind: disk hardware/track validation. Pass path: the branch following the call (e.g., bpl.b an address in sub_035C).
- `start_disk_sync_transfer_2` (runtime segment an address): Waits for a countdown, then writes the DMA control, disk pointer, and sync words to custom chip registers before calling two helper routines. Kind: disk hardware sync/timing check. Pass path: execution continues to the next instruction (movea.l an address(a4), a0).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of one register to another and then performs a short branch.

### EndCLI

- `update_linked_list_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a buffer with a header and subsequent data.
- `move_and_branch_2`: Copies a long value from one register to another before branching to a new location.

### cd

- `find_max_and_populate_buffer`: Traverses a linked list to identify the maximum value in a specific field and then populates a data buffer with a header and subsequent values.
- `move_long_and_branch`: This routine moves a long value from one register to another and then performs a relative branch.

### Port-Handler

- `parse_amiga_hunk_headers`: Iterates through a data structure containing hunk type codes to identify and process executable or relocation segments.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf:fb

- `initialize_and_call_library`: Fills a memory range with zeros and then executes functions from a library based on a global pointer.
- `initialize_game_state_structure_2`: Resets multiple global status variables to zero and initializes hardware parameters.
- `execute_callback_chain_4`: Calls several different routines via pointers stored at various offsets from a register.
- `execute_callback_chain_3`: Calls two functions via pointers, passes a value to a third function, and then calls itself twice.
- `dispatch_table_index`: Uses a value to index into a table of function pointers, executing different logic branches based on the selected index.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `branch_on_table_index`: Compares a value to a constant, optionally shifts it, and uses it to fetch an offset from a table to perform a jump.
- `execute_callback_sequence`: Sets two values in a structure, calls a function via a pointer, and executes a sequence of other subroutines and callbacks.
- `dispatch_library_calls`: Sets values in a structure and executes a series of function calls through pointers, including several subroutines.
- `process_hunk_data`: Sets several values in a structure, executes a series of function calls through pointers, and performs a conditional check based on a value in a table before calling more subroutines.
- `update_announcer_state`: Sets two word values in memory, calls a function via a pointer, and conditionally executes a routine using a value retrieved from a pointer.
- `execute_indirect_call_2`: Calls a routine via a pointer, stores the result in a global variable, and if that result is zero, calls another routine before storing a calculated address into a global variable.
- `reset_conditional_flags`: Checks several memory locations and, if non-zero, resets various global variables to zero or one.
- `process_conditional_audio_logic`: Updates a word in memory and calls a function with three parameters.
- `calculate_kick_accuracy_and_update_state`: Calculates kick accuracy values by recursively calling a routine with different parameters and stores the results in a data structure.
- `dispatch_halftime_logic`: Stores a value into a memory location and executes several functions via a table of pointers before calling itself recursively.
- `clear_status_variables`: Sets a series of specific memory locations to either zero or one.
- `execute_music_callback`: Calls a function via a pointer and then calls itself recursively with a global variable.
- `execute_callback`: Calls a subroutine through a pointer found at a specific offset, passing a value and a parameter.
- `check_global_flag_and_recurse`: Checks if a value at a specific offset from a global variable is zero; if so, it calls itself recursively.
- `update_calculated_offsets`: Calls a subroutine to get a value and uses it to update several global variables with calculated offsets.
- `initialize_hardware_parameters`: Calls several subroutines and writes specific constant values to a set of memory locations.
- `sum_indexed_bytes`: Calculates a sum by retrieving and adding bytes from a table using an index and multiple offsets.
- `manipulate_data_structure`: Calls several subroutines to modify data at a specific memory location and returns a value from a global variable.
- `perform_data_operation`: Calls subroutines to manipulate data at a specific memory location and returns a value from a global variable.
- `process_parameterized_request`: Checks a condition, then calls a sequence of subroutines to manipulate data via a handle and returns a result.
- `parse_8svx_format`: Processes data from a pointer, performing several comparisons and memory operations before returning a value.
- `modify_registers_and_decrement`: Increments a register, negates a value at a pointer, performs a bitwise OR on a register, and decrements a counter.
- `compare_byte_strings`: Compares two byte strings byte by byte using a loop and a subroutine to read data.
- `compare_and_update_global`: Compares a value retrieved from a function call with another value, then updates a global variable and returns a result based on the comparison.
- `test_bit_and_branch`: Tests a specific bit in a memory location and branches based on the result.
- `branch_with_constant`: Loads a constant into a register and branches to another location.
- `test_bit_and_branch_2`: Loads a pointer from a global table, tests a specific bit in the pointed-to byte, and branches to another routine or sets a register to one based on the result.
- `test_bit_and_set_d0`: Tests a specific bit in a memory location and sets a register to a constant value if the bit is set.
- `clear_d0_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `check_bit_masks`: Checks if specific bit patterns are present in a value pointed to by a global table and returns zero, one, or all ones based on those patterns.
- `check_bit_mask`: Checks if specific bits are set in a memory location using bitwise operations and branches accordingly.
- `branch_with_zero_alt`: Loads zero into a register and branches to a different location than the previous routine.
- `clear_memory_tables`: Performs multiple nested loops to zero out various segments of memory organized in specific patterns and dimensions.
- `test_structure_offsets`: Sets up a stack frame and performs conditional checks on two word values at specific offsets from a pointer.
- `update_calculated_offsets_3`: Calculates one or three memory addresses based on values in a structure and adds a value from that structure to the shorts at those locations.
- `update_conditional_offset_values`: Checks a flag in a structure to determine which set of three memory addresses to calculate.
- `update_calculated_offsets_2`: Calculates three different memory addresses based on values in a structure and adds a value to the shorts at those locations.
- `accumulate_coordinate_offsets`: Clears a block of memory and then performs a series of additions, summing values from multiple source tables into a destination table using iterative loops.
- `call_library_function_3`: Calls a function via a pointer stored in a table and stores the result in a global variable.
- `call_function_with_argument`: Calls a function using a value retrieved from a global variable and passes four as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `write_to_hardware_registers`: Writes a value from a global variable and the constant one to specific hardware register offsets.
- `update_coprocessor_hardware`: Writes a longword and a word into hardware registers using an offset calculated from a provided parameter.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `initialize_font_pointers`: Calls multiple library functions to populate several global variables with pointers or values related to diskfont.library.
- `execute_callback_chain`: Checks several global variables and, if they are non-zero, calls a function using those values as arguments.
- `execute_handler_routine`: A 37-instruction routine called via a pointer in data that includes a call to another routine.
- `tiny_move_move`: Small routine of 2 instructions that copies -(a0) to d0, then copies d0 to d0.
- `execute_parameterized_call`: Calls a function via a pointer using a value from a memory location and a provided parameter.
- `execute_table_calls`: Passes various parameters to functions called via a table of pointers.
- `manipulate_graphics_buffer`: Performs multiple library calls to draw, clear, or move data within a buffer based on calculated coordinates and parameters.
- `process_coordinate_table`: Iterates through a table of eight pairs of values to perform operations using a calculated base address and parameters.
- `call_wrapper_function`: Passes three parameters directly to a function called via a pointer in a table.
- `process_coordinate_table_2`: Iterates through a loop to perform multiple function calls using a calculated base address and incrementing values.
- `clear_global_word`: Stores a zero word into a global memory location.
- `test_and_branch_2`: Tests a word at a specific offset within a structure and branches based on the result.
- `update_game_state_parameters`: Updates multiple sets of 16-bit values in memory based on conditional checks of bitwise XOR results and status flags.
- `clear_and_copy_words`: Clears three 16-bit values at offsets from a register and conditionally copies three 16-bit words from another location to a new destination based on a test.
- `clear_state_words`: Clears three 16-bit values at specific offsets from a register and performs a relative branch.
- `test_and_update_words`: Tests a 16-bit value and, if non-zero, calls three different routines to sequentially update three 16-bit values in memory using the results from the register.
- `update_state_buffers_2`: Clears three 16-bit values, checks a 16-bit value at an offset, and if zero, copies three 16-bit words from one location to another before branching.
- `update_state_buffers`: Clears several words and then copies values between different memory locations based on conditional checks.
- `clear_and_test_words`: Clears three words at specific offsets from a pointer, tests a word at an offset from another pointer, and branches based on the result.
- `copy_words_conditionally`: Tests a word at a calculated offset and another word at a fixed offset, then copies three words between specific memory locations if both tests pass.
- `clear_memory_words`: Copies three words from one set of memory locations to another and then branches.
- `clear_memory_locations`: Clears three words at specific offsets relative to a register and then branches to another routine.
- `format_numeric_value`: Decomposes an integer into individual decimal digits and stores them in a local array.
- `filter_and_dispatch_table`: Iterates through a table to mark specific entries with a constant value and then executes function pointers for the remaining entries.
- `filter_and_dispatch_table_entries`: Iterates through a table to mark specific entries with a constant value, then executes a function pointer for each remaining entry using parameters derived from the table and a global structure.
- `iterate_and_call_function`: Decrements a counter in a loop and calls a function via a pointer if a specific value in a table is not equal to thirty-two.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `call_indirect_functions`: Calls two different functions using pointers stored in a table, passing various parameters including sums of input values.
- `initialize_stack_frame`: Sets up a stack frame by linking and clearing a local variable before branching to another routine.
- `copy_memory_block`: Copies bytes from one memory location to another using a loop and an index.
- `calculate_accumulated_value`: Performs a loop to combine a global variable with other values and stores the result in a global location.
- `update_complex_accumulator`: Uses a loop and bitwise operations to add a combination of global and local values to a stored integer.
- `increment_table_entry`: Calls a function, uses the result to calculate an offset into a table, and increments the word at that location.
- `update_accumulated_sum`: Calculates a sum using two values from memory and a global variable, then stores the result back into one of those memory locations after adding multiple instances of the global variable to it.
- `copy_and_reset_data`: Copies a longword from one location to another and then sets the original location to all ones.
- `copy_value_via_pointer`: Calls another routine and uses its return value as a base pointer to store a word at a specific offset.
- `process_disk_message`: Copies a string into a buffer and enters a loop to check for disk status or user input, executing various subroutines based on the result.
- `process_disk_events`: Enters a loop that monitors a value from a routine and executes a sequence of subroutines based on that value.
- `call_and_copy_data`: Calls a function via a pointer and then calls another routine to copy data.
- `process_data_sequence`: Executes a series of function calls and enters loops that compare values from a table against a global variable.
- `execute_conditional_sequence`: Iterates through a structure to call multiple routines and perform table updates based on specific comparison values.
- `process_linked_list_nodes`: Iterates through a linked list structure, performing recursive calls and executing function pointers stored within the list elements.
- `decrement_and_branch_2`: Subtracts a long value from a register, performs conditional branches based on whether the result is zero or reaches a specific value.
- `branch_to_routine`: Performs a short branch to another routine.
- `branch_to_offset_3`: Performs a short branch to another location in the code.
- `process_game_state`: A large routine that calls several subroutines to update game state, graphics, display parameters, and check button status.
- `update_game_state_and_graphics`: Copies a block of data from a source table to a local stack buffer and performs a series of state updates by calling various subroutines with specific parameters.
- `copy_data_block`: If a specific global flag is set, it clears that flag and copies five words from one memory location to another.
- `update_display_parameters`: Calls several functions to retrieve a value, calculate a new offset, and perform multiple writes to a table using that offset and provided parameters.
- `check_button_status`: Checks if two specific bits are set in a global variable and returns one if both are active, otherwise returns zero.
- `pass_parameters_to_subroutine`: Passes multiple arguments through to another routine.
- `calculate_blitter_clipping_rects_4`: Calculates intersection boundaries and clipping parameters for a rectangular area.
- `compare_and_branch_word`: Compares a word from a structure with a word from another structure and performs conditional branches.
- `compare_and_branch_2`: Compares a word from a structure with another word and branches based on the comparison result.
- `mask_and_store_word`: Extracts a word from a structure, applies a bitwise AND mask, and stores the result back into a different offset within the same structure.
- `calculate_blitter_clipping_rects_3`: Calculates the sub-rectangle coordinates and bitwise parameters for blitter operations by performing division and bounds checking against a clip.
- `calculate_blitter_clipping_rects_2_2`: Calculates sub-rectangle coordinates and bitwise parameters for blitter operations by performing division and bounds checking against a clip.
- `calculate_blitter_clipping_rects_2`: Calculates sub-rectangle coordinates and bitwise parameters for blitter operations by performing division and bounds checking against a clip.
- `calculate_blitter_clipping_rects`: Calculates sub-rectangle coordinates and bitwise parameters for blitter operations by performing division and bounds checking against a clipping area.
- `configure_blitter_area_loop`: Calculates blitter source and destination coordinates and modes based on table lookups and bitwise shifts.
- `update_blitter_registers_2`: Manipulates a register value using bitwise operations and performs a memory move based on a calculated address.
- `update_blitter_registers`: Manipulates various hardware registers and performs calculations involving a loop and a pointer-based function call.
- `branch_to_subroutine`: Performs a short relative branch to a location within another routine.
- `execute_structure_callbacks`: Resets a global pointer and executes several function pointers stored within a structure.
- `execute_callback_list`: Resets a global pointer and executes a series of function pointers stored within a structure.
- `initialize_game_state_structure`: Populates a structure by calling a sequence of initialization and clamping routines, then performs multiple conditional lookups to store specific values into a table.
- `process_event_data`: Checks a global flag, calls several functions to process parameters, and conditionally performs a write to a memory location.
- `update_parameter_state`: Checks a global flag and, if a retrieved value is non-zero, performs a series of function calls involving provided parameters and a memory location.
- `load_and_parse_asset_file`: Opens a file, verifies a specific magic number header, and reads a data block into a destination structure.
- `unpack_and_process_data_stream`: Extracts values from a data structure by parsing nibble-based control codes and populating a destination array.
- `process_stream_data_array`: Parses a data stream using nibble-based control codes to populate a destination array with values from a source table or immediate data.
- `populate_array_from_stream`: Iterates through a source data stream to populate a destination array, using control nibbles to either copy a value from a source table, insert a fixed constant, or repeat a previously processed value.
- `fill_table_with_data`: Iterates through a loop to store values from a table into another table based on nibble-extracted values.
- `fill_table_with_max_value`: Iterates through a loop to store a maximum value into a table based on nibble-extracted values.
- `fill_table_with_indexed_data`: Iterates through a loop to store data from an indexed table into another table based on nibble-extracted values.
- `decrement_and_branch`: Decrements a register and performs conditional branches based on the resulting value.
- `update_table_value`: Calculates an offset using a value from a stack frame, modifies a value in a table, and increments a counter.
- `call_subroutine_with_stack_adjustment`: Pushes a value onto the stack, calls another routine, adjusts the stack pointer, and performs a short branch.
- `iterate_and_call_table_entries`: Resets a value in a table and then iterates through a list of up to eight pointers, calling a routine for each non-zero entry found.
- `expand_compressed_data_stream`: Decodes a custom compressed data stream by reading control bytes and repeating values a specified number of times.
- `branch_conditionally_2`: Performs a conditional branch based on a loop counter and then an unconditional short branch.
- `perform_nested_data_copy`: Executes a nested loop structure to copy bytes from a source to a destination, using multiple counters and conditional logic to determine the amount of data moved.
- `execute_loop_branch`: Decrements a counter and branches to a location that may restart the loop.
- `return_stack_value`: This routine returns a value currently residing on the stack.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_conditional_memory_updates`: Increments a global counter and conditionally calls various routines to copy data to different memory locations based on the state of several short-integer flags.
- `update_conditional_state`: Checks a condition based on a returned value and, if met, initializes several fields in a structure and calls another routine with specific arguments.
- `reset_global_pointer`: Calls a function with a constant and a pointer, then calls another routine and sets a global variable to a maximum value.
- `initialize_gameport_input_state`: Allocates and initializes a structure to manage input device states by querying the gameport device.
- `execute_indirect_calls_2`: Calls a routine with several arguments and then executes two consecutive indirect function calls via a pointer located at an offset from a register.
- `clear_and_refresh`: Sets a word in a global location to zero and calls another routine twice.
- `remove_linked_item`: Traverses a linked list to find a specific value and removes that node by updating the preceding pointer.
- `swap_linked_list_nodes`: Traverses a linked list to find a specific node and swaps its position with another node by updating their pointers.
- `replace_linked_list_node`: Iterates through a linked list and replaces a node with a new pointer from a structure.
- `swap_two_values`: Copies the value from one memory location to another and then attempts to assign a pointer to the original location.
- `calculate_blitter_area_parameters`: Calculates bitplane counts, area dimensions, and control register values from a source data structure.
- `configure_blitter_graphics_parameters`: Calculates bitplane and area parameters from a data structure to update blitter control and mode registers.
- `update_blitter_registers_5`: Moves a word into a blitter register, loads two pointers from a structure, and conditionally moves a value into another blitter register.
- `update_blitter_registers_4`: Updates the blitter configuration registers and a global variable using values from a structure.
- `configure_blitter_parameters`: Calculates various values based on data from a structure and updates several Blitter hardware registers and a global variable.
- `update_blitter_registers_3`: Loads a value from a structure into a register and updates the blitter destination address and control registers.
- `update_blt_registers`: Updates several Blitter hardware registers and a global variable using values from a structure.
- `compare_structure_bounds`: Compares the dimensions of two structures to determine if they overlap.
- `compare_calculated_offset_2`: Calculates a value by summing two words from a structure and subtracting one, then compares it against a word in another structure.
- `compare_calculated_offset`: Calculates a value by adding two words from a structure and subtracting one, then compares the result against a word in another structure.
- `compare_calculated_value`: Calculates a value by adding two words from a structure and subtracting one, then compares it against a value in another structure.
- `store_and_branch`: Stores two values into memory locations relative to a pointer and then performs a short branch.
- `clear_global_values`: Sets several memory locations to zero and calls another routine.
- `copy_and_process_data_blocks`: Copies a block of thirty-two words from one memory location to another and then calls another routine using a pointer from a structure.
- `pack_nibbles_to_table`: Reads bytes from a source table and packs their high nibbles or certain bit patterns into a destination table.
- `calculate_scaled_differences`: Calculates the difference between high nibbles of two source tables and stores the result divided by a value into a destination table.
- `update_table_from_differences`: Computes differences between elements in a table and stores the results into another table, or copies a table if certain counters reach zero.
- `recursively_check_and_copy`: Recursively checks a byte flag in a structure and calls a routine to copy or process memory blocks if a bit is set.
- `copy_memory_block_2`: Copies a certain number of words from a source pointer to a destination pointer.
- `process_and_copy_elements`: Performs conditional insertion or deletion of bytes within a list based on specific criteria and then copies values from a table into a destination.
- `initialize_byte_table`: Fills a table with values where each byte is double its index.
- `copy_structure_elements`: Copies four sets of data from a source table to a destination table using a loop.
- `compare_converted_strings`: Iterates through a string, converting characters to uppercase and comparing them until a difference is found, then returns the difference of two converted values.
- `subtract_converted_values`: Calls a routine to convert two values and returns the difference between them.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_immediately_2`: The routine is empty and immediately returns to its caller.
- `call_subroutine_with_pointer`: Pushes a pointer and a word onto the stack before calling a subroutine.
- `initialize_structure_node`: Allocates a new structure via a routine call and populates it with several constant values or data from another structure.
- `initialize_structure_data`: Performs a linked list operation and then writes several constant values into a structure.
- `traverse_linked_list_4`: Iterates through a linked list of pointers, comparing strings at each node until a match is found or the end of the list is reached.
- `swap_pointer_and_branch`: Loads a pointer from a memory location into a register, stores the value pointed to by that register into a different memory location, and then performs a short branch.
- `call_subroutine_via_pointer`: Pushes a pointer from a memory location onto the stack, calls a subroutine, adjusts the stack pointer, and branches to another location.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `clear_structure_pointers`: Checks two pointers within a structure, calls a routine to process one if it is not null, and then clears both pointers.
- `process_linked_structure`: Calls two routines to manipulate a structure and then uses function pointers to update values within that structure.
- `traverse_linked_structure`: Iterates through a linked list of pointers and calls another routine for each node encountered.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `clamp_and_store_result`: Calls a library function to retrieve a value, clamps that value between a minimum and maximum bound, and stores the result through a pointer obtained from the library call.
- `process_table_data`: Iterates through a table of words, performing a bitwise rotation and addition on each element.
- `recalculate_value_with_modulo`: Obtains a value from another routine and performs a calculation involving division and modulo using a part of a provided parameter.
- `update_table_index`: Decrements a value in a table and uses it to index another part of the same table to perform an addition.
- `allocate_and_initialize_node_tree`: Allocates memory for a new data structure and recursively initializes its child nodes and properties.
- `traverse_linked_list`: Recursively traverses a linked list structure, calling a routine on each node and its children, and performing memory deallocation on specific elements.
- `search_and_store_pointers`: Iterates through a list of values to find matches for a descending sequence of numbers, storing the resulting pointers in a destination table.
- `search_conditional_table`: Searches a table for a value based on a parameter, potentially performing a second search in a different table if a specific bit is set in a byte within a structure.
- `update_blt_parameters_from_table`: Iterates through a list of pointers to update a structure with values derived from a table and hardware registers.
- `update_hardware_registers`: Calls another routine and then writes a word and a long value to specific offsets within a structure pointed to by a register.
- `update_table_values`: Iterates through a list of pointers, updating a structure with values from registers and a pointer-dereferenced value for each entry in a loop.
- `open_file_handle`: Attempts to open a file using provided parameters, stores the resulting handle in a global variable, and calls another routine if the handle is null.
- `handle_file_error`: Checks if a global file handle is non-zero and if a parameter is non-zero, then attempts to perform an operation and closes the file handle if an error occurs.
- `close_file_handle`: Checks if a global file handle is non-zero, closes the file if it is, and resets the handle to zero.
- `allocate_and_populate_data_array`: Allocates a memory block based on the sum of two lengths and populates it by iterating through two separate lists of data sources.
- `extend_byte_to_long`: Loads a byte from a structure, sign-extends it to a long integer, and branches to another routine.
- `process_linked_list_nodes_2`: Recursively traverses a linked list while updating several fields in a structure.
- `pass_pointer_to_function`: Retrieves a pointer from a global structure and an integer from a register to pass them as arguments to another routine.
- `recurse_unconditionally`: The routine immediately calls itself, creating an infinite recursion loop.
- `call_extend_byte_to_long`: Calls a routine that extends a byte from a stack offset into a long integer and branches to another routine.
- `set_global_flag_via_pointer`: Sets a single byte in a global memory location to one using a pointer retrieved from a data table.
- `decrement_and_check_limit`: Decrements a word in a structure and clears a global variable if the value reaches a certain threshold.
- `increment_and_check_limit`: Increments a word in a structure within a loop until it reaches a limit defined in another structure, then returns a boolean result.
- `compare_and_branch`: Compares a word from a table against a value in a register and checks a bit in another register before branching or returning.
- `clear_d0_and_branch_2`: Sets the d0 register to zero and performs a short branch to another routine.
- `traverse_and_update_linked_list`: Iterates through a linked list, performing conditional subroutine calls and updating four integer members within each node using arithmetic.
- `push_pointers_and_call_routine`: Calculates two addresses by adding offsets to values from two different structures and pushes them onto the stack along with a third pointer before calling another routine.
- `iterate_and_update_list`: Iterates through a linked list while performing arithmetic on several structure members.
- `traverse_linked_list_2`: Iterates through a linked list while updating several values in a structure and calling another routine for each node.
- `get_register_value`: Calls a routine and returns a word from a specific memory location.
- `increment_counter`: Increments a value in a global variable if it is not zero.
- `decrement_counter`: Decrements a global variable and resets it to zero if it becomes negative.
- `calculate_and_draw`: Calculates two short values based on input parameters and a table, then calls a routine to perform a drawing operation using those values.
- `insert_into_sorted_list_2`: Allocates a new structure and inserts it into a linked list in ascending order based on a word value.
- `insert_into_sorted_list`: Copies data from one structure into a new one and inserts it into a linked list in ascending order based on a word value.
- `swap_two_pointers`: Moves a pointer from one memory location to another and then stores the original pointer in the first location.
- `store_pointer_to_memory_2`: Stores a pointer into a location pointed to by a register and then updates another memory location with that same pointer.
- `store_pointer_to_memory`: Copies the contents of a register to a location pointed to by an offset from another register and then branches.
- `check_structure_and_return`: Compares a byte from the stack against a value in a register and returns either a pointer or a zero value based on several comparisons.
- `compare_and_return_pointer`: Compares a byte in a structure to a register and, if they are not equal, returns a pointer stored in a register.
- `remove_from_list`: Traverses a linked list to find a node matching two specific criteria and then removes that node by updating pointers and freeing the memory.
- `traverse_and_free_list`: Recursively traverses a linked list and calls a routine to free each node.
- `verify_asset_header`: Calls another routine to load an asset, then checks if the returned value matches a specific constant value.
- `set_d0_to_one`: Sets the d0 register to one and branches to another location.
- `clear_d0_and_branch_3`: Clears the d0 register and performs a short branch to another routine.
- `process_data_table`: Iterates through a table of eight entries, performing a calculation with values from the table and calling another routine before clearing the entries.
- `call_subroutine_with_short`: Pushes a pointer and a short value onto the stack to call a subroutine.
- `update_node_status`: Allocates a structure and sets a word in that structure to one after performing several other routine calls.
- `update_list_node_status`: Calls another routine to perform a linked list operation and then sets a word in a structure to one.
- `search_linked_list`: Traverses a linked list of pointers while performing string comparisons at each step to find a specific entry.
- `swap_and_branch`: Moves a longword from an offset of a structure into a register, stores that register back into a different offset of the same structure, and branches to another routine.
- `push_call_and_branch`: Pushes a longword from an offset of a frame pointer onto the stack, calls a subroutine, adjusts the stack pointer, and performs a short branch.
- `clear_bit_and_test_pointer`: Clears a specific bit in a byte at an offset from a pointer and then tests if a longword at another offset is zero.
- `test_pointer_offset_value`: Loads two pointers from a structure and branches based on whether a value at a specific offset within the second pointer is zero.
- `update_blitter_parameters`: Calls another routine to calculate values and then stores the result into a specific offset within a structure.
- `test_and_clear_pointer`: Checks if a longword at an offset from a pointer is non-zero, and if so, clears that longword and returns the original value in a register.
- `test_pointer_and_branch`: Tests a longword at an offset from a pointer and branches if it is zero.
- `execute_callback_and_store_result`: Pushes two longwords onto the stack and calls a routine via a pointer in a register.
- `execute_conditional_callback_6`: Pushes two longwords onto the stack, calls a subroutine, and then conditionally calls another routine based on the result of the first call.
- `copy_and_test_status`: Copies a longword from a location relative to a frame pointer into a destination address.
- `execute_callback_and_clear`: Checks if a long word at an offset from a pointer is non-zero, calls a routine using that value, and then clears the original value.
- `process_object_callback`: Checks if a parameter is non-zero, then executes a function pointer at a specific offset from a base address, conditionally executes another function pointer using a value from an offset within the parameter, and finally returns the result of a third function call using the parameter and a constant.
- `clear_pointer_after_call`: Checks if a pointer at a specific offset in a structure is non-zero, executes the function at that pointer, and then clears the pointer to zero.
- `execute_callback_chain_2`: Calls a routine with a parameter, then executes two function pointers stored within a structure using values derived from a third function call.
- `traverse_and_execute_list`: Iterates through a linked list of pointers, executing a routine via a pointer found within each node.
- `initialize_animation_resources`: Attempts to allocate and initialize animation bitmaps, views, and rastports by traversing a node tree and populating tables.
- `swap_and_execute_pointers`: Swaps two pairs of longwords within a structure and then executes a series of function calls retrieved from a list.
- `execute_library_calls`: Calls three different routines using pointers retrieved from global variables.
- `decrement_and_recurse`: Decrements a short value in a global structure and calls itself recursively if the value reaches zero.
- `traverse_and_clear_list`: Recursively traverses a linked list, calling a routine on each node, performing specific memory writes, and clearing several pointers and values in a structure.
- `clear_bit_and_update_pointer`: Tests a pointer from a structure, calls another routine if it is non-zero, clears a specific bit in the value pointed to by that pointer, and then updates the pointer with a new address from a table.
- `process_pointer_chain`: Loads a pointer from a structure, clears a bit in the value pointed to by that pointer, and then updates the pointer with a new address from the same location.
- `execute_subroutine_and_unlk`: Calls a subroutine, tests a word in a register, and either returns or unloads an address from a register.
- `manage_structure_and_list`: Clears a global pointer, updates several status flags, and enters a loop that executes a subroutine based on a bit-shifted value.
- `call_and_clear_pointer`: Pushes a value from a structure onto the stack, calls another routine, clears a longword in the structure, and calls a second routine.
- `update_linked_list_state`: Iterates through a linked list to accumulate values into specific structure offsets and update state flags.
- `process_linked_list_data`: Iterates through a linked list to update values in a structure based on data within the list nodes, including conditional logic for specific byte flags.
- `update_list_state`: Traverses a linked list to update structure fields and modify specific bytes based on values found in the list nodes.
- `initialize_nested_structure`: Retrieves a pointer from a structure, validates it, and if successful, populates fields within a nested structure and performs library calls.
- `reset_structure_fields_2`: Checks a short value at an offset from a pointer, and if non-zero, writes a constant to a structure field and updates another field with a calculated value before calling another routine twice.
- `allocate_and_fill_table`: Allocates a block of memory using a library call and then populates a table with pointers retrieved from another library call.
- `execute_indirect_calls`: Retrieves two function pointers from a table using an offset from a base address and executes them with provided arguments.
- `allocate_and_initialize_structure_2`: Allocates a memory block using a library function and populates several fields within that block using provided parameters, conditionally calling other routines based on the results.
- `reset_structure_and_call_routines`: Conditionally calls other routines using pointers from a structure and then clears multiple fields within that structure to zero.
- `update_state_from_table`: Performs a series of checks and updates a structure by comparing values from a table and copying data from a source address to a destination buffer.
- `search_and_return_pointer`: Calls a routine to obtain a value and then conditionally allocates memory via a library function to store a result, returning the pointer or zero.
- `execute_recursive_call`: This routine performs a direct recursive call to itself using the provided parameters before returning.
- `calculate_file_offset`: Calculates a new offset based on values in a table and updates a parameter, or calls another routine if certain global conditions are met.
- `copy_memory_segments`: Calculates a size difference between two values retrieved from a table and updates a structure.
- `call_data_transfer_routine`: Calls a routine that transfers data in fixed-size chunks after calculating a length from a table.
- `check_structure_flags`: Tests two word-sized values at specific offsets from a pointer and conditionally calls another routine if both are non-zero.
- `load_and_branch_short`: Loads a longword from an offset of a pointer into a register and performs a short branch to another location.
- `close_file_handle_2`: Calls a routine if certain conditions are met, or calls another routine with a specific value if a pointer is non-zero.
- `set_library_parameters`: Writes specific values to a structure and then calls a routine using a pointer from a global variable.
- `reset_library_parameters`: If a global condition is met, it writes a specific value to a structure and calls a routine, then writes different values to the same structure and calls the routine again.
- `copy_and_uppercase_string`: Copies up to four bytes from a source string to a local buffer while converting lowercase ASCII characters to uppercase.
- `search_and_execute_callback`: Searches through a table for a matching value and, if found, conditionally executes routines and updates structure fields based on the result.
- `iterate_and_update_blocks`: Checks a global flag to set a starting address, then iterates up to eight times, calling another routine with incrementing offsets and a fixed value.
- `validate_and_update_state`: Compares two short values and a long value against parameters, updating a structure with new values and calling another routine if certain conditions are met.
- `call_search_routine`: Calls another routine passing a value and a length derived from a global variable.
- `search_integer_array`: Iterates backward through an array of long values to check if a specific value exists within the specified count.
- `store_pointer_and_call`: Stores a pointer into a global variable and calls another routine with three parameters.
- `increment_and_store_byte`: Increments a global counter and stores a byte from a register into a location pointed to by a global pointer.
- `parse_percentage_and_update_list`: Parses a string representing a percentage value and subsequently removes a node from a linked list.
- `call_recursive_routine`: A routine consisting of 48 instructions that calls another routine and returns.
- `parse_percentage_string`: Parses a string containing percentage values, including support for signs, decimals, and variable-width numeric formats.
- `update_offset_word_and_branch`: Stores a word value into a memory location calculated from a pointer in a register and then performs a short branch.
- `store_short_value`: Writes a word to an offset of a pointer and performs a short branch to another routine.
- `parse_formatted_string`: Parses a string containing numeric values and percentage symbols, handling sign prefixes and specific character delimiters.
- `format_floating_point_string`: Converts a numeric value into a formatted string representation, handling sign characters, decimal points, and scientific notation.
- `loop_and_call_routines`: Repeatedly calls two different routines and updates a value in a structure within a loop.
- `compare_and_clear_flag`: Compares a value in a structure to a constant and, if certain conditions are met, clears a word at a specific offset in that structure.
- `compare_and_modify_register`: Compares a value at a memory offset to a constant and performs arithmetic on a value stored on the stack based on the result.
- `format_scientific_notation_string_2`: Converts a numeric value into a scientific notation string by calculating the exponent and iterating through digits to append them to a character buffer.
- `clear_and_branch_2`: Clears a word at a negative offset from a pointer and branches to another routine.
- `increment_and_store`: Loads a word from a pointer, increments it, stores the result back to a different offset of that pointer, and branches to another routine.
- `format_scientific_notation`: Converts a numeric value into a scientific notation string by iterating through digits and appending an exponent with a sign.
- `format_scientific_notation_string`: Converts a numeric value into a string representation using scientific notation, including a sign and an exponent.
- `append_numeric_characters`: Checks a value and conditionally writes the character 'e' or a hyphen followed by a negated value into a destination pointed to by a register.
- `convert_integer_to_string`: Converts a 16-bit integer value into a null-terminated ASCII string representation.
- `execute_callback_via_structure_2`: Checks a global flag and, if zero, executes a routine through a pointer found in a structure before calling another routine using an offset.
- `dispatch_callback_via_structure`: Checks a global flag to determine if a specific routine needs to be initialized or updated.
- `execute_callback_via_structure_3`: Checks a global flag to determine if a specific routine needs to be initialized or updated.
- `update_callback_state`: Checks a global status flag to determine if a specific routine needs initialization.
- `dispatch_callback_via_structure_2`: Checks a global flag and, if zero, initializes a structure and executes a secondary routine to populate it.
- `dispatch_callback_via_offset`: Checks a global flag and, if zero, initializes a structure and executes a function through a pointer.
- `dispatch_callback_via_structure_3`: Checks a status flag in a structure and, if zero, initializes a data block and executes a secondary routine.
- `execute_callback_via_structure_4`: Checks a global flag and executes a routine through a pointer using an offset derived from a structure.
- `dispatch_callback_via_structure_4`: Checks a global flag to determine if a cached routine must be initialized.
- `execute_callback_via_structure`: Checks a global flag, potentially calls a function through a pointer, and eventually executes a routine using an offset derived from a structure.
- `skip_numeric_characters`: Checks if a byte at a pointer is a space or a digit, incrementing the pointer if a match is found.
- `skip_matching_character`: Compares a byte at a pointer to a constant and increments the pointer if a match is found.
- `skip_character_and_branch`: Compares a byte at a pointer to a constant, and if it matches, increments the pointer and branches to another location.
- `convert_string_to_integer`: Parses a string of numeric characters into a signed integer value.
- `return_immediately_3`: The routine executes four instructions and returns without performing any observable work or modifying any state.
- `save_registers_and_branch`: Moves a pointer from the stack to a register, saves several word-sized registers, and branches to another location.
- `fill_memory_backward`: Fills a memory range with a byte value by iterating backward from a starting pointer.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `compare_and_decrement_pointer`: Compares a byte from a pointer to a value in a register and, if they match, returns a decremented version of that pointer.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_memory_and_call_library`: Fills a memory range with zeros and calls functions from a library based on a global pointer.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tokenize_string_into_array`: Parses a string by splitting it into individual tokens based on whitespace or quoted substrings.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `increment_table_pointer`: Compares a word at a pointer to a constant, and if it matches, increments the pointer by four and branches to another location.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `initialize_register_and_branch`: Loads a constant value into a register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_library_function_4`: Loads a pointer from a structure, calls a library function, and stores the result in a global variable.
- `process_window_data`: Traverses a structure to call library functions and updates a table with values retrieved from a pointer.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Checks the signs of two registers, negates them if negative, and calls another routine to perform division.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `remove_linked_list_node`: This routine calls another routine that removes a node from a linked list structure.
- `traverse_linked_list_structures_2`: This routine iterates through a linked list of structures to update internal status fields based on conditional logic.
- `traverse_linked_list_structures`: This routine calls another routine to traverse a linked list of structures and update internal status fields based on conditional logic.
- `traverse_linked_list_3`: Iterates through a list of structures by incrementing a pointer and calling another routine for each entry.
- `clear_recursive_structure`: Recursively traverses a structure based on flags in a byte field, calls a function via a pointer, and clears the structure's fields to zero.
- `process_linked_structure_2`: Traverses a linked list of structures and updates internal status fields based on conditional logic.
- `clear_structure_list`: Iterates through a list of structures and clears the first three words of each entry if a specific byte is zero.
- `branch_to_offset_2`: Performs a short branch to a specific instruction offset.
- `clear_register_and_branch`: Clears the contents of a register and several memory locations pointed to by that register, then branches to another routine.
- `update_status_fields`: Checks an interactive status and updates various fields in a structure based on the result of another routine.
- `iterate_linked_list_and_call_function`: Traverses a linked list starting from a pointer stored in a global variable, passing each node and a value derived from the node's second word to another routine.
- `push_to_linked_list`: Allocates a new node using a function call and inserts it into a linked list by updating pointers.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `unlink_linked_list_node`: Traverses a linked list to find a specific node matching a provided pointer, then updates the previous node's pointer to bypass it.
- `check_interactive_status`: Validates an index against a table and calls another routine to determine if a specific resource is interactive.
- `handle_file_deletion`: Attempts to delete a file and, if unsuccessful, calls another routine to retrieve and store an error code in a global variable.
- `execute_io_operations`: A routine consisting of 41 instructions that calls other routines related to conditional writes and I/O error handling.
- `execute_conditional_write`: Calls a routine with specific parameters and, if a bit is set in the result, performs a conditional call to another routine.
- `execute_write_operation`: Calls a function to write four bytes from a specific memory location and then calls another routine to handle the result.
- `execute_callback_and_routine`: Checks if a global function pointer is non-zero to execute it, then calls another routine.
- `clear_table_entry`: Checks if an index is within the bounds of a table and if the entry is non-zero, then clears the entry by setting it to zero or calls another routine if a specific bit is set.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_date_stamp`: Wrapper that calls the dos library function DateStamp and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `execute_indirect_call`: Performs an indirect call to an address retrieved from a pointer within a structure.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Allocates a memory block using a library function and populates several fields within that block using provided parameters.
- `reset_structure_fields`: Sets specific bytes and longwords in a structure to fixed values and calls another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: Calls another routine passing the input parameter and a fixed constant value of 48.
- `destroy_display_entry_2`: Calls another routine passing the input pointer as an argument.
- `update_structure_fields`: Calls a function via a pointer in a global variable and, if successful, writes a byte, a word, and a longword to specific offsets within the returned pointer.
- `reset_structure_values`: Sets three specific fields in a structure to fixed values and then calls another routine using a value from the structure.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `process_complex_data_structure`: Copies a block of data from a global table to the stack, calls another routine to allocate memory, and then populates a new structure with values derived from the allocated block and parameters.
- `destroy_display_entry`: It calls another routine passing the input pointer as an argument.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `indirect_jump_stub_3`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `indirect_jump_stub_4`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_indirect`: Performs an indirect call to a function located at a specific offset within a library base retrieved from a global variable.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_300_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_486`: Library stub: loads a library base from a global variable and calls the library function at offset minus 486 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### 8

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### 7

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.

### 8

- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.

### 7

- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.

### 9

- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero`: The routine clears the accumulator and returns zero to the caller.

### 10

- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.

### 14

- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.

### mathtrans.library

- `execute_illegal_instruction`: Library code that executes an illegal instruction.
- `store_register_to_global`: Library code that stores the contents of a register into a global variable.
- `insert_node_into_list`: Library code that checks a value in a structure; if zero, it inserts a new node into a linked list and executes a function pointer, otherwise it sets a specific bit in a byte within the structure.
- `set_memory_bit`: Library code that sets a specific bit in a byte located at an offset from a register.
- `increment_and_call_library`: Library code that checks a bit in memory, increments a short value at an offset if the bit is clear, and calls another library routine if the value reaches one.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `call_empty_routine`: Library code that is empty.
- `calculate_bitfield_difference`: Library code that performs a loop to calculate a value by repeatedly doubling an accumulator and adjusting a running difference.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `call_and_modify_bits`: Library code that calls a routine and then modifies specific bits in a register before branching.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_register_and_branch_2`: Library code that clears a long register and branches to another routine.
- `execute_library_callbacks_2`: Library code that executes various function pointers from a jump table based on register values and conditional checks.
- `execute_library_callbacks`: Library code that invokes various function pointers from a jump table based on register values and conditional checks.
- `execute_callback_logic`: Library code that executes various subroutines via pointers stored at specific offsets from a base address based on register values.
- `execute_conditional_subroutine_calls`: The routine calls a subroutine via an offset from a pointer and branches based on the result.
- `call_and_store_result`: Library code that calls a routine using a pointer in a register and stores the result back into that register.
- `branch_on_register_test`: Library code that tests a register and either branches to another routine or loads a constant into a register.
- `mask_return_value`: Library code that calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_integer_logarithm`: Library code that performs bit-shifting and iterative comparisons to determine a value related to a power of two.
- `shift_and_setup_registers`: Library code that performs a logical shift right on a longword in a data register and loads several registers with specific constant values before performing a short branch.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_stack_word_and_branch`: Library code that pushes a word onto the stack and performs a short relative branch.
- `library_jump_setup`: Library code that pushes a word onto the stack and branches to another location.
- `push_word_and_branch`: Pushes a word onto the stack and performs a short relative branch.
- `compare_and_branch_stack`: Library code compares a byte to a constant, branches if higher, and otherwise adjusts the stack pointer and returns if a byte is zero.
- `perform_math_transformation`: Library code that performs complex arithmetic operations involving bitwise shifts and iterations through a table of values.
- `perform_complex_math_library_operation`: Library code that performs complex arithmetic operations using bitwise shifts and additions while iterating through a table.
- `perform_complex_arithmetic_2`: This library code performs arithmetic operations using bitwise shifts and additions while iterating through a table.
- `perform_complex_arithmetic`: Library code that performs arithmetic operations using bitwise shifts and additions within a loop while accessing a table.
- `perform_complex_math`: Library code performs a series of arithmetic operations involving bitwise shifts and additions within a loop, potentially calling another routine recursively.
- `load_constant_and_branch`: Library code loads a constant value into a register and then performs a short branch.
- `recursive_call_wrapper`: Library code that calls itself and then returns.
- `push_and_call_routine`: Library code that pushes a longword onto the stack, calls another routine, and then restores the value from the stack.
- `clamp_value_range`: The library code moves a value, checks if it is negative, and then compares it against a constant to either zero it or return.
- `shift_and_jump`: Library code performs an arithmetic shift right on a register, increments it, and then either branches or jumps to a zero address.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_recursive_logic`: Library code checks a value and, if certain bitwise conditions are met, performs a sequence of recursive calls.
- `rotate_and_branch`: Library code loads a constant into a register, performs a rotate right through the carry, and then performs a short branch.
- `load_and_branch`: Library code loads a constant into a register and then performs a short branch.
- `test_and_branch`: This library code loads a word from the stack into a register, tests its value, and performs a conditional branch.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `return_immediately_7`: This library routine performs no observable work and immediately returns to its caller.
- `perform_recursive_math`: Library code that executes a series of recursive calls and performs bitwise manipulations on a value.
- `execute_recursive_calls`: Library code that performs a series of self-referential calls to itself before returning.
- `process_table_with_shifts`: The library code performs arithmetic shifts on two registers, adds them to other registers, and executes a loop that decrements a counter while subtracting values from a table.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_branch_sign`: Library code that tests a byte for zero or negative status and branches to a normalization routine if negative.
- `load_and_branch_2`: This library code loads a constant value into a register and performs an unconditional branch.
- `normalize_fixed_point`: Library code that performs a loop involving bit-shifting and subtractions to normalize a value, followed by a secondary loop for scaling.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_bit`: Library code that tests a byte, performs a bitwise AND, and branches to another routine based on the result.
- `process_conditional_data`: Library code performs several routine calls and uses a value to determine whether to perform bitwise shifts and further routine calls.
- `perform_no_operation`: This routine contains no instructions and performs no operation.
- `transform_register_bits`: Library code that performs a sequence of swaps, rotations, and arithmetic operations on a register.
- `modify_byte_value`: This library code compares a byte in a register against two specific constants, performs arithmetic and a bitwise rotation if a match is found, and then tests the result.
- `mask_and_test`: This library code performs a bitwise AND on a register, then tests the result to decide whether to branch or clear the register.
- `test_and_shift_bits`: The library code compares a register to a constant and performs logical shifts and conditional branches based on the result.
- `shift_and_set_flags`: This library code performs a logical shift left on a word, followed by a series of bitwise operations and a register rotation to set a specific bit in the status register.
- `clear_and_branch`: This library code clears a register and performs an unconditional branch.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `invoke_callback_indirect`: The library code is an unreached gap that performs an indirect call to a routine and returns the original value of a register.
- `invoke_callback_and_return_2`: This library code receives a pointer in a register, calls the routine pointed to, and returns the original value from a data register.
- `execute_callback_and_return`: Library code that calls a routine via a pointer in A0 and returns the original value from D0.
- `invoke_callback_wrapper`: Library code that calls another routine and returns a value.
- `library_call_wrapper`: Library code that calls another routine and returns the original value from a register.
- `invoke_callback_and_return_3`: This library code calls another routine and returns the original value from a register.
- `invoke_callback_and_return_4`: This library code performs a jump to another routine and returns the value held in a register.
- `store_register_via_pointer`: The routine invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `call_callback_conditionally`: Library code that calls a routine via a pointer in A0 and then conditionally executes three additional routines.
- `invoke_callback_repeatedly`: Library code that calls a routine via a pointer in A0 and may call it again based on a condition.
- `execute_recursive_callback`: Library code that calls a routine via a pointer in A0 and performs recursive calls based on bitwise conditions.
- `invoke_callback_wrapper_2`: Library code that performs a call to another routine and returns a value.
- `invoke_callback_wrapper_3`: Library code that executes a routine via a pointer and returns a value.
- `call_callback_thrice`: Library code that calls a routine via a pointer in A0 three times in succession before returning.
- `test_and_call_indirect`: This library code tests a bit in a byte and, based on the result, calls a routine via an indirect pointer and returns a value.
- `invoke_empty_callback`: Library code that calls a routine through a pointer in a register and returns the original value from a register.
- `invoke_callback_wrapper_4`: Library code that calls another routine and returns the result.
- `call_indirect_offset`: Library code that calls a routine at an address calculated by adding an offset to a pointer in a global variable.
- `call_indirect_offset_alt`: Library code that calls a routine at an address calculated by adding a different offset to a pointer in a global variable.
- `load_address_and_branch`: Library code that loads a specific address into a register and branches to another routine.
- `load_address_and_branch_2`: Library code that loads a specific address into a register and performs a short branch to another location.
- `call_indirect_offset_routine`: Library code calls a routine at an address calculated by adding a negative offset to a pointer stored in a global variable.
- `call_indirect_offset_routine_2`: Library code calls a routine at an address calculated by adding a different negative offset to a pointer stored in a global variable.
- `load_address_and_branch_3`: Library code loads a specific address into a register and then performs a short branch.
- `load_address_and_branch_4`: Loads a constant address into a register and performs a short branch to another location.
- `load_callback_address`: Loads the address of a global variable into a register and performs a short branch to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: The library code executes a single illegal instruction.
- `libInit2`: The library code stores the value from A0 into a global variable.
- `MIExpunge`: This library code checks a flag in a structure, and if zero, updates a linked list and calls a function pointer before returning a segment list.
- `Exp_InUse`: This library routine sets the eighth bit of a byte located at an offset from a pointer held in an address register.
- `MIOpen`: This library code tests a bit in a structure, increments a value at an offset within that structure if the bit is clear, and returns the structure pointer in a register.
- `openFail`: This library routine returns a value representing a failure.
- `MIClose`: The library code decrements a short counter stored in a structure and, if the counter reaches zero and a specific flag bit is set, calls MIExpunge and returns its result.
- `DPFix`: The library code performs bitwise operations and a loop of right shifts on a value to normalize or transform it based on specific bit patterns.
- `dfi4`: This library code tests a specific bit in a register and branches based on the result, otherwise clearing a register and branching.
- `dfi6`: This library code clears a long register and performs an unconditional branch.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: The routine converts a signed single-precision value in a register into a double-precision representation stored through a pointer.
- `DPAbs`: The routine copies the high word of a double from a source pointer to a destination pointer and writes the low word with its sign bit cleared.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: This library routine subtracts one from the value held in a data register.
- `dtst_2`: This library routine adds one to the value held in a data register.
- `cc_1`: This library routine calls a function whose address is passed in an address register and then returns.
- `cc_2`: The routine calls a function whose address is passed in an address register and then returns the contents of a stack slot.
- `cc_3`: The routine calls a function pointer held in register A1 and returns the value left on the stack at offset 18.
- `MIIEEEDPFix`: This library code calls another routine and then returns.
- `MIIEEEDPFlt`: Library code that calls a floating point routine and returns the value previously on the stack.
- `MIIEEEDPCmp`: Library code that performs a tail branch to a floating point comparison routine.
- `MIIEEEDPTst`: Library code that performs a tail branch to a floating point test routine.
- `MIIEEEDPAbs`: Library code that performs a tail branch to a floating point absolute value routine.
- `MIIEEEDPNeg`: Library code that performs a tail branch to a floating point negation routine.
- `MIIEEEDPAdd`: Library code that performs a tail branch to a floating point addition routine.
- `MIIEEEDPSub`: Library code that performs a tail branch to a floating point subtraction routine.
- `MIIEEEDPMul`: Library code that performs a tail branch to a floating point multiplication routine.
- `MIIEEEDPDiv`: Library code that performs a tail branch to a floating point division routine.
- `MIIEEEDPCeil`: Library code calculates a ceiling value by negating a value, applying a floor function, and then negating the result.
- `MIIEEEDPFloor`: Library code that calculates the floor of a double-precision value by comparing it to zero and adjusting it via addition or subtraction.
- `test_d6_d7`: This library code moves two registers to others, calls another routine, and then either branches or calls a third routine after restoring registers.
- `lt_zero`: This library routine stores a constant into a stack slot and then tail calls another routine.
- `fptrap`: This routine acts as a trap handler that dispatches floating-point operations by looking up a handler in a jump table.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `dfsub`: Library code that saves registers to the stack, moves two values into address registers, and performs a bitwise change on a value loaded from memory before branching.
- `dfadd`: Performs addition on two double-precision floating-point numbers.
- `dfa11`: Performs subtraction on two double-precision floating-point numbers.
- `dfa14`: This library routine performs conditional logic to store a value into a memory location pointed to by a register.
- `dfa13`: Library code that performs complex bitwise manipulations and arithmetic to store a value into a pointer.
- `dfa5`: Library code performs bitwise manipulations and arithmetic to store a result into a table of longwords.
- `dfa8`: This library routine compares two values and, if the comparison fails, performs a subtraction and a bitwise shift operation involving a register and a carry bit.
- `dfa7`: This library code performs bitwise operations on several registers and stores the resulting values into a table pointed to by a register.
- `dfa9`: Library code that clears a longword in a register and a longword at an offset from that register before performing a short branch.
- `dfauo`: Library code that compares a long value against a threshold and, if lower, stores a bitwise OR of two values and a byte into a memory location pointed to by a register.
- `dfmul`: Performs double-precision floating-point multiplication by normalizing the mantissas, calculating the product, and re-normalizing the result.
- `dfm4`: This library routine clears a long register and the memory at the address pointed to by that register before performing a short branch.
- `dfmuo`: Library code that compares a long value against a threshold and, if lower, stores a bitwise OR of two values and a constant into a memory location pointed to by a register.
- `dfdiv`: Performs double-precision floating-point division using a bit-shifting iterative algorithm.
- `dfd10`: Executes a long division algorithm on double-precision floating-point values.
- `dfd4`: This library routine performs bitwise manipulations and arithmetic on values stored in registers to compute a result.
- `dfsetz`: This library routine clears two data registers and then performs a short branch.
- `dfdbyz`: This library routine loads a constant value into a register and then performs a short branch to another location.
- `dfduo`: Library code that compares a long value against a threshold and, if lower, performs bitwise operations on a long value and updates the processor status register.
- `dfneg`: This library routine loads a word from a pointer, performs a bitwise AND, and if the result is non-zero, toggles a bit in the original memory location before branching to another routine.
- `dfcmp`: This library routine is empty and simply returns a value from the stack.
- `dftst`: This library routine returns the 32-bit value that was passed on the stack.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### version.library

- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call_2`: Library code that decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `test_and_set_library_bit`: This routine retrieves a pointer from a register, tests a word at a specific offset within the pointed structure, and conditionally sets a bit at a different offset before returning zero.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `check_and_set_font_handle_2`: This library routine calls a routine via a table entry using a library base and, if the result is non-zero, stores a value into a structure and calls another routine via a table entry.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: This library routine opens two libraries, checks their return values, and if successful, initializes a structure by setting a pointer and clearing a value.
- `process_linked_list_elements`: Library code that iterates through a linked list to remove elements based on a short value, updates a counter, and conditionally calls other routines and modifies a list structure.
- `update_linked_list`: This library routine checks two conditions; if they are met, it calls two functions, retrieves a value, calls a third function, and updates a linked list structure before returning.
- `set_bit_and_clear_register`: This library routine sets a specific bit in a byte at an offset from a register and then clears a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_conditional_jump`: Decrements a word at an offset from a base register, then performs conditional branches based on the zero flag and a bit test, finally executing a jump to an offset from the base register.
- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `update_font_metrics_table`: Iterates through a table of font character structures to identify the entry with the highest metric value.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg_2`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `increment_pointer_if_nonzero`: This library routine checks a byte at a pointer and increments that pointer if the byte is not zero.
- `call_empty_routine_2`: This library routine performs a tail call to another routine and then returns.
- `calculate_string_length`: As part of a library, this routine iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_byte_sequences`: This library routine compares a byte from one memory location against a sequence of bytes in another.
- `compare_byte_and_branch`: This library routine compares a byte from a pointer against a value in a register and branches based on the result.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_font_bytes`: This library routine compares a byte from a memory location pointed to by a register with a byte from the stack.
- `increment_pointer_and_branch`: This library routine increments a pointer by one and then performs an unconditional branch to another location.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations.
- `extract_font_metadata_and_strings`: Iterates through a linked list of font structures to extract character string data and associated metadata into a destination buffer.
- `populate_font_string_buffer`: Iterates through a linked list of string structures to copy character data into a destination buffer.
- `copy_string_to_buffer_3`: Copies a sequence of bytes from a source pointer to a destination buffer.
- `parse_font_metadata_block_3`: Iterates through a data stream to identify specific font header markers and extract character attribute structures.
- `parse_font_metadata_block_2`: Iterates through a data stream to identify specific font header markers and extract character attribute structures.
- `parse_font_metadata_block`: Iterates through a data stream to identify specific font header markers and extract character attribute structures.
- `read_and_parse_font_data`: Reads data from a disk source and parses it into a structured format.
- `calculate_absolute_value`: This library routine calculates the absolute value of a signed integer.
- `return_zero_constant`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_27`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_28`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_29`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_30`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_31`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_32`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_33`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_34`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_35`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_36`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_37`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_38`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `perform_multiple_bit_tests`: This routine calls another routine multiple times to perform bitwise masking on the result and tests specific bits.
- `execute_repeated_calls`: This routine calls another routine eighteen times in succession.
- `update_byte_with_bitwise_or`: This routine calls another routine multiple times and performs a bitwise OR operation to update a byte in memory.
- `test_bit_after_call`: This routine calls another routine and then tests a specific bit in a register.
- `process_data_with_bit_tests`: This routine calls another routine multiple times, performs bitwise XOR operations, tests bits, and pushes bytes onto the stack from various sources.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manipulate_pointers_and_test_bits`: This routine calls another routine multiple times, tests bits, moves data between registers using pointers, and stores a long value at a memory location pointed to by a register.
- `test_bit_and_clear`: This routine calls another routine, tests a specific bit in a register, and performs a bitwise OR operation with zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_mask_byte`: Calls a subroutine and then masks the byte in a data register.
- `modify_memory_and_registers`: Calls a subroutine, masks a byte in a data register, performs a bitwise AND on a word at a pointer, and updates two data registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_byte_to_stack`: Calls a subroutine, pushes a byte from a data register onto the stack, and clears that register.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_playback_buffer`: Allocates memory for an audio buffer and initializes its header with a magic signature.
- `copy_data_register`: Copies the contents of one data register to another.
- `cleanup_and_release_resources`: Iterates through a list of function pointers to execute them and then releases allocated memory blocks.
- `validate_resource_initialization`: This routine calls several other routines sequentially, checking if their return values are zero.
- `return_immediately`: This routine performs no operations and returns to its caller immediately.
- `return_constant_value`: Returns a constant integer value.
- `return_function_pointer`: Returns the address of another routine.
- `execute_sequential_subroutines`: Saves several registers to the stack and calls a series of subroutines before branching to another location.
- `process_resource_loop`: Executes a loop that calls various subroutines to update data structures, manipulate hardware registers, and perform disk-related operations based on specific conditions.
- `call_library_function_directly`: Calls one library function on a base pointer held in the state block, then calls a helper routine.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_nested_callbacks`: Sets a value to minus one in a record, calls a function to obtain a pointer, and if successful executes a sequence of four further functions via that pointer.
- `open_library_and_release`: Sets a result field to minus one, opens the dos.library name through the exec base, and when the open succeeds calls two methods on the returned base before closing it.
- `initialize_trackdisk_device_request`: Populates a device request record with a specific length and unit value before calling a library function to open the trackdisk device.
- `store_two_library_results`: Calls two library functions and stores their results in the state record and a second record, one as a long and one as a byte.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function`: Calls a library function using a base address loaded from a global variable.
- `update_control_structure`: This routine copies data between memory locations and populates a structure with specific values, including a constant word, before calling another routine via a pointer.
- `populate_record_data`: Copies two values from one record to another, sets a constant and a fixed short value, and calculates two scaled values using arithmetic on registers before calling another routine.
- `populate_record_data_2`: Writes a constant, two values from another record, and two scaled values into a target record structure.
- `load_control_word_eight_zero_zero_five`: Loads a constant word into a register and branches to another routine.
- `load_constant_and_branch_2`: Loads a word constant into a data register and performs a short unconditional branch to another location.
- `load_constant_and_branch_4`: This routine loads a small constant into a data register and performs a short unconditional branch to another location.
- `initialize_registers_and_branch`: Clears a register, loads a word constant into another register, and performs a short branch to another routine.
- `update_record_fields`: Writes several values into a record, including a constant, two values from another record, and two calculated values scaled by a power of two.
- `register_disk_resource_handler`: Obtains the disk.resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `process_disk_resource_loop`: Repeatedly calls a function on a disk resource pointer until it succeeds, then modifies hardware registers and a global variable.
- `invoke_disk_resource_routine`: Calls one function on the disk resource base held in the state block.
- `configure_hardware_and_call_subroutine`: Sets specific values in the custom chip registers and then calls a subroutine using pointers stored in registers before branching to another location.
- `branch_to_target`: Performs a short branch to another location.
- `update_flag_byte_bits`: Clears the bit selected by the first register value in a state byte, then sets bit two when the second register value is zero.
- `copy_state_byte_to_register`: Copies a state byte into a control word and writes a fixed flag value to a second control word.
- `start_disk_sync_transfer`: Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers.
- `set_top_bit_field`: Stores the incoming value with its top bit set into a 16-bit field of the record addressed by the second register, twice.
- `reset_record_and_call_library`: Sets a state word to a fixed value, sets a second word to two, and calls a library function on the base pointer.
- `count_consecutive_pattern_matches`: Calculates a value based on the number of times a specific word repeats consecutively within a memory range.
- `count_consecutive_values`: Iterates through a memory range to count how many times a specific word value repeats consecutively.
- `forward_to_helper_routine`: Calls one helper routine and returns with no other work.
- `compute_bitwise_checksum`: Computes a 16-bit checksum over a run of words by shifting each bit through the accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `return_without_action`: The body contains only a return, with no calls, memory access or arguments used.
- `check_and_jump_on_bit`: Decrements a value at a pointer, performs conditional branches based on bit tests, and jumps to an offset from a base pointer.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `flatten_linked_string_list`: Iterates through a linked list of string structures, calculating the length of each null-terminated string and copying the data into a destination buffer.
- `copy_linked_string_list`: Iterates through a linked list of string structures, calculating the length of each null-terminated string and copying the data into a destination buffer.
- `manipulate_string_buffer`: Calculates the length of a null-terminated string and adjusts a buffer pointer and counter accordingly.
- `parse_and_copy_font_string`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `parse_and_copy_string_data_2`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `parse_and_copy_string_data`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `copy_string_to_buffer_2`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `call_library_functions`: Calls three different library functions using offsets from a base address stored in a global variable and returns an absolute value based on a register.
- `return_zero_constant_2`: Returns a zero value to the caller without performing any other operations or accessing memory.
- `compute_font_offset_2`: Calculates a signed offset by finding the difference between two 16-bit values and adjusting the result through a bitwise comparison of two byte arrays against a global lookup table.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_null_byte`: Checks if a byte at a pointer is null and increments the pointer if it is not.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length_2`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_and_check_byte`: Compares a byte from one pointer to a byte from another pointer and conditionally sets a register to zero.
- `compare_and_set_byte`: Moves a byte from a pointer to a register, compares it to another value, and conditionally sets a register to a constant value.
- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward_2`: Calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle_2`: Checks the validity of a font handle by attempting two specific operations.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `return_immediately_5`: This routine performs no operations and returns immediately to the caller.
- `return_immediately_6`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_4`: This routine is empty and performs no operations before returning to its caller.
- `modify_struct_fields`: Clears the high bit of a byte and increments a short integer at specific offsets within a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_buffer`: Allocates a memory block, populates a table with incrementing offsets based on a base address, and performs two function calls involving hardware parameters.
- `call_indirect_function`: Fetches a function pointer from a location calculated by adding an offset to a value in a register and calls it with a pointer and a constant value as arguments.
- `concatenate_strings`: Joins three source strings into a destination buffer, inserting a separator character if necessary.
- `execute_conditional_callback`: Checks if a function pointer in a global structure is non-null and either calls it or calls another routine with the structure as an argument.
- `execute_conditional_callback_2`: Checks if a pointer stored in a global structure is null.
- `execute_conditional_callback_3`: Checks if a function pointer stored in a table is null.
- `execute_conditional_callback_4`: Checks if a function pointer stored in a global structure is null.
- `parse_and_execute_script`: Iterates through a list of data entries to identify and execute a sequence of commands or function pointers.
- `reset_table_entries`: Clears a two-byte field in a table repeatedly and calls another routine each time, using a counter that runs from a positive start value down to zero.
- `decrement_and_clear_counter`: Decrements a short counter stored at an offset from a pointer and, if the counter reaches zero or was already invalid, clears the counter and, when a flag at the base of the structure is non-zero, calls another routine and invokes a function pointer stored in a table.
- `search_table_for_match`: Walks a table of entries using a fixed stride and compares an input value with a field in each entry, calling another routine when a match is found.
- `parse_resource_script`: Iterates through a data stream to identify and process specific command tags.
- `call_other_routine_and_return_zero`: Calls another routine and then returns zero.
- `execute_infinite_recursion_2`: Calls itself indefinitely without performing any other operations.
- `mask_lower_bytes`: Calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `execute_infinite_recursion`: The routine calls itself indefinitely, creating an infinite loop of recursive calls.
- `execute_conditional_callback_5`: Compares a value from a global variable with a register and calls another routine if they are not equal.
- `wait_for_flag_clear_2`: Repeatedly calls another routine while the word-sized value at offset 872 from A6 is non-zero, then returns the 32-bit combination of D0 and D1 as the result.
- `unregister_object_and_clear_name`: If the provided pointer is non-zero, the routine clears a status flag and a short value in a global structure, iterates through a 64-byte name field to call another routine for each non-null byte, and then calls that routine three additional times.
- `reset_game_state_flag`: Clears a 16-bit value in a global variable and sets a flag byte in the same structure by OR-ing in a constant, then calls another routine.
- `copy_struct_to_message`: Calls another routine and, if certain conditions on the returned values and a pointer are met, copies a small fixed-size block from memory into a structure, updates several fields, and clears a byte in that structure.
- `clear_status_flags`: Clears a longword at an offset from a register and clears a byte at another offset from that same register.
- `clear_flag_and_iterate_table`: Checks a flag in a global variable, clears it, and then iterates through a table of integers, calling another routine for each non-zero entry until a counter reaches four.
- `process_data_table_2`: Iterates through a table of pointers to process data, updating specific fields in a destination structure based on the values found in the source data.
- `increment_and_branch`: Increments a byte in a register, stores that byte into a memory location using an offset from a pointer, and performs an unconditional branch.
- `increment_pointers_and_branch`: Increments two pointers by four and a byte by one, then branches based on a comparison of that byte with four.
- `copy_struct_fields_and_update_table`: The routine copies seven words from one area of a structure to another area of the same structure and then copies a word from one offset to another.
- `execute_recursive_pointer_calls`: Checks a pointer in a structure, calls that pointer, and then calls itself recursively multiple times.
- `update_event_queue_and_timers`: Decrements counters within a table of entries and processes a linked list of events.
- `decrement_counter_in_loop`: Library code that checks a value at an offset from A5, decrements it in a loop, and modifies D0 or D6 based on conditions.
- `process_linked_list_event_queue`: Iterates through a linked list of event entries, decrementing counters and updating pointers within a table.
- `process_event_queue_and_update_slots`: Iterates through a table of entries, decrementing counters and processing linked list elements.
- `update_conditional_flag`: Compares two values and, if they are not equal, checks a short flag at an offset from a pointer to potentially decrement it and call another routine.
- `clear_flag_after_callback`: The routine checks a short flag at an offset from a pointer and a global variable at another offset.
- `signal_on_flag`: Checks a global flag and, if set, calls another routine via a function pointer stored in a global variable.
- `process_sound_table`: Iterates a table of entries and invokes callbacks stored in each entry, setting bits in a mask based on their return values.
- `setup_sound_channel`: Writes a value derived from an input register to a global variable and then writes the constant 2 to the left channel of audio channel 0.
- `search_and_update_slot_2`: Iterates through a table to find a matching entry based on specific byte criteria, then updates a slot with new data and a status flag.
- `search_and_update_slot`: The routine iterates through a table to find a matching entry based on specific byte criteria, then updates a slot with new data and a status flag.
- `set_register_and_branch`: Loads a constant value into a register and performs a relative branch.
- `clear_state_and_wait`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_flag_byte`: Computes an address from a global variable and a register value, writes zero to a byte at that address, and if the following byte equals one, writes two to the first byte.
- `reset_and_poll_status`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero, returning the final value in a register.
- `clear_slot_and_notify`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot, calls a callback via an indirection, and clears the next byte and the slot byte.
- `calculate_weighted_value`: Stores a zero in a memory location and then performs a calculation involving a division and multiplication to update several fields in a structure.
- `store_word_and_branch`: Stores a word from a register into an offset of a pointer and then performs a relative branch.
- `calculate_position_offset`: Calculates a value based on a product of two fields in a structure and updates a coordinate field, or clears a bit in a byte and calls another routine if a condition is met.
- `clear_flag_and_call`: Checks a word at an offset from a pointer and, if non-zero, clears a bit in a structure and calls another routine before clearing a different word.
- `increment_and_branch_2`: Adds a constant byte value to a data register and performs a short relative branch to another location.
- `decrement_and_recursively_call_counter`: Decrements a short integer located at an offset from a pointer returned by another routine.
- `update_object_spatial_state`: Updates an object's position, scale, and animation frame by calculating values from a lookup table and applying them to a data structure.
- `decrement_counter_and_branch`: Calls another routine, checks if the returned pointer is non-zero, decrements a short value located at an offset from that pointer, and calls another routine if the new value is zero.
- `copy_struct_fields_and_clear_flag`: Copies several fields from a structure pointed to by one register into a destination pointed to by another when a condition on a register is met, then returns the original value of one source field after clearing it in the source structure.
- `update_object_animation_state`: Updates an object's animation frame and sprite properties based on its current state and a lookup table.
- `update_object_kinematics`: Calculates and updates an object's position, velocity, and animation frame using fixed-point arithmetic and scaling.
- `calculate_scaled_coordinate_and_timer`: Performs fixed-point division and scaling to determine a coordinate or position based on a structure's parameters.
- `compare_and_store_byte`: Compares a byte in a register to a constant and branches if they are not equal.
- `branch_on_byte_comparison`: Compares a byte in a register to a constant and branches based on the result, then calls another routine and performs a final branch.
- `decrement_word_with_conditional_branch`: Tests a word at a specific offset within a structure and branches based on the result.
- `calculate_audio_resampling_offset`: Calculates a new offset value by multiplying a value from a table by a scaling factor and adding it to an existing value.
- `update_audio_resampling_parameters`: Calculates interpolation coefficients, step sizes, and buffer pointers for audio playback.
- `subtract_if_less_than`: Compares two longwords and, if the first is less than the second, subtracts a third longword from the second.
- `resample_audio_sample_buffer`: Calculates resampling parameters and performs linear interpolation or direct copying of audio samples into a destination buffer.
- `resample_audio_buffer_3`: Calculates resampling parameters and performs audio sample interpolation or direct copying into a destination buffer.
- `resample_audio_buffer_2`: Performs audio sample resampling using either direct byte copying or linear interpolation between adjacent samples.
- `resample_audio_buffer`: Performs linear interpolation or fixed-point resampling to transform a source buffer into a destination buffer.
- `update_fixed_point_table`: Performs fixed-point arithmetic to update a value in a structure and then uses a loop to redistribute or resample bytes from a source buffer into a destination buffer.
- `update_sprite_reference_and_scale`: The routine iterates through a table of pointers, comparing a block of data from each entry against a reference block to manage reference counters in a structure.
- `branch_to_next_instruction`: Performs a short relative branch to a different location in the code.
- `decrement_structure_counter`: Checks if a pointer in a structure is non-zero, then decrements a short value at a specific offset within that pointed-to structure.
- `process_event_command_buffer`: Parses a command buffer to update a structure containing coordinate, scale, and timing data.
- `decrement_counter_and_call_3`: The routine reads a pointer from an object field, checks it is non-zero, then loads a short from an offset in the pointed-to structure, decrements it and writes it back.
- `compare_and_clear_register`: Compares a word in a register against a constant and clears a longword in a register if the value is less than that constant.
- `update_audio_sample_playback`: Calculates sample playback positions, volume levels, and frequency scaling based on table lookups and bitwise transformations.
- `calculate_audio_sample_parameters`: Calculates sample playback positions, volume levels, and frequency scaling based on table lookups and bitwise transformations.
- `compare_and_update_value`: Compares a byte in a register to a constant and branches if they are not equal.
- `branch_on_byte_comparison_2`: Compares a byte in a register to a constant and branches if they are not equal.
- `decrement_word_on_condition`: Tests a word at an offset from a pointer and decrements it by one if the value is not zero.
- `update_physics_and_scaling_parameters`: Calculates new values for a set of structured parameters by performing scaled multiplications and bitwise shifts based on values from two different memory structures.
- `compare_and_subtract_longwords`: Compares two longwords and branches if the first is less than the second.
- `calculate_complex_values`: Performs several arithmetic operations involving multiplications and bit shifts to update values in a structure.
- `cleanup_device_and_library_2`: The routine checks a global pointer, and if non-zero, performs several operations including calling a function via a pointer from a global variable, updating a value in a structure, and calling several other routines before clearing global pointers.
- `execute_signal_sequence`: Calls a sequence of three different functions from a global table, passing an argument to one, and returns a value formed by masking an input register with the result of a table call.
- `store_task_and_flags`: Clears a word in a structure pointed to by a register and, if a flag in another register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the result, and copies the flag into the structure.
- `compare_strings_case_insensitive`: Compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `return_called_register`: Calls another routine and returns the value that was left in an address register.
- `allocate_memory_block`: Checks a global pointer to determine if a memory block can be reused or if new memory must be allocated, then stores a size in the first word of the allocated block.
- `free_memory_block`: Checks if a pointer is non-zero and then calls a function via a global pointer to release the memory.
- `return_called_address_register`: Calls another routine with no arguments, then returns the value left in address register A0 by that call.
- `check_and_branch_on_result`: Calls another routine and tests its return value; if non-zero, it calls several other routines in sequence, conditionally invoking another routine based on a pointer and a bitmask check, before returning a 64-bit value.
- `initialize_object_structure`: Allocates and initializes a complex data structure by calling multiple subroutines to populate various fields.
- `execute_conditional_calls`: Checks a global variable and calls another routine multiple times if the value is non-zero.
- `call_subroutine_and_copy_struct`: Calls another routine and, if certain conditions involving a pointer and returned values are met, copies a fixed-size memory block to a message buffer.
- `copy_first_string_pointer`: Identifies the first non-empty string in a list of pointers and copies its pointer or content to a destination.
- `initialize_sonix_driver`: Allocates and initializes a complex data structure and attempts to copy a string pointer from a list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_envelope_and_pitch`: Updates an audio envelope state machine by modifying volume and period values based on a counter and a data table.
- `initialize_hunk_data`: Parses Amiga hunk data to populate a structure with specific values, a pointer to a global variable, and a function address.
- `trigger_crowd_audio_event`: Executes a sequence of function calls via a pointer table to update audio parameters and trigger a specific sound effect associated with the crowd stopping.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_match_statistics`: Clears several data tables and then iterates through a structured byte array to compare adjacent values.
- `process_table_comparisons`: Iterates through a table of short integers to perform equality checks between adjacent elements and calls other routines based on the results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_values`: Calls a function via a pointer to retrieve a value, stores it in two global variables, and performs a conditional byte transfer into a local variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_increment`: Compares a value at a specific memory location with a value in a table and updates a local variable by calling another routine if they match.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_modify_table`: Iterates through a table using an index, performing conditional updates and function calls based on the values retrieved from a secondary data structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_and_sort_attributes`: Increments a value, pushes it and a value from a pointer onto the stack, calls another routine, and then performs a loop that decrements a value and compares it against a constant.
- `sort_player_attributes`: Performs a comparison-based sort on a subset of an index table by calculating the difference between two specific fields within a data structure pointed to by each index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_state`: Resets several global variables to zero, performs multiple function calls via pointers, and stores a series of constant values into a table.
- `update_coordinate_pairs`: Calls a routine multiple times using pairs of values retrieved from a table, with one call being conditional based on a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_byte_values`: Stores the value two hundred and fifty-five into four consecutive bytes and sets a word to zero in a global variable.
- `set_short_values`: Stores the value three into two consecutive word locations in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_difference_and_call`: Calculates the difference between two values, stores it in a word, updates an integer, and calls another routine with a pointer and a calculated value.
- `update_timer_values`: Stores a zero, calls a routine to retrieve a value, and performs arithmetic to update several integer variables in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_structure_fields`: Calls several routines to populate fields in a structure and returns a specific value if certain conditions are met, or returns a constant value of all ones if they are not.
- `frag_lea_movea_rts`: Short code fragment that loads the address of a location in the code into a5, then copies the memory at an offset from a5 to a4, then copies a fixed memory address to a6, then returns.
- `return_constant_value_2`: Returns a fixed integer value.
- `return_routine_address`: Returns the address of a routine.
- `execute_multiple_subroutines`: Saves registers to the stack and executes a sequence of six different subroutines.
- `update_status_flag_and_loop`: Sets a byte at a specific offset within a structure to a constant value and enters a loop that repeatedly calls other routines until a condition is met.
- `call_indirect_via_offset`: Fetches a function pointer from an offset relative to a base pointer and executes it.
- `call_library_vector_228_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_conditional_logic`: Sets a value in a table to negative one, calls a routine to obtain a pointer, and if successful, performs several nested function calls before calling another routine.
- `reset_field_and_query_dos`: Sets a value in a structure to negative one, opens dos.library, and if successful, executes two specific methods on the library base before closing it.
- `open_trackdisk_device_request`: Fills a device request record with a unit value and a fixed length, then calls a library function to open the trackdisk.device.
- `store_dual_library_outputs`: Calls two library functions and stores a long and a byte result into two different destination records.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function_2`: Loads a library base from a global variable and calls a library function at a specific offset.
- `copy_and_assign_data`: Copies values between memory locations and assigns specific constants to a structure.
- `update_record_fields_2_2`: Writes a constant, two values from another record, and two calculated values scaled by a power of two into a target record.
- `initialize_record_data`: Sets a constant byte and a short value into a target record, copies two words from a source record, and calculates two scaled values using bit shifts and multiplication before calling another routine.
- `set_d4_and_branch`: Stores a constant value into register d4 and branches to another location.
- `load_constant_and_branch_3`: Loads a word into a data register and performs an unconditional short branch to another location.
- `set_register_and_branch_2`: Loads a small constant into a data register and branches to another location.
- `clear_d3_set_d4_and_branch`: Clears register d3, stores a constant value into register d4, and branches to another location.
- `update_record_fields_2`: Writes a constant, two values from another record, and two calculated values scaled by a power of two into a target record.
- `register_disk_resource_handler_2`: Obtains the disk resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `invoke_disk_resource_loop`: Repeatedly calls a function located at an offset within a structure pointed to by a register until it returns a non-zero value, then calls another routine and updates a global variable and a hardware register.
- `invoke_disk_resource_routine_2`: Calls a function located at an offset within a structure pointed to by a register.
- `configure_hardware_and_call`: Writes values to custom hardware registers, loads two pointers into registers, and calls another routine.
- `branch_to_offset`: Performs a short branch to a specific memory address.
- `update_flag_byte_bits_2`: Clears the bit specified by the first register value in a byte at a memory offset, then sets bit two if the second register value is zero.
- `update_control_word_flag`: Copies a byte from a memory location into a global variable and then overwrites that same variable with a fixed bitmask value.
- `start_disk_sync_transfer_2`: Waits for a countdown, then writes the DMA control, disk pointer, and sync words to custom chip registers before calling two helper routines.
- `set_top_bit_field_2`: Sets the most significant bit of a 16-bit value and stores the result twice into a field within a record pointed to by a register.
- `set_state_and_invoke_library`: Stores a constant value into a memory location pointed to by a register, stores the value two into another location via the same base pointer, and executes a function from a library table.
- `count_consecutive_words`: Calculates a value based on a bitmask, then iterates through a memory range to find the longest sequence of identical words.
- `count_consecutive_word_matches`: Iterates through a memory range starting from a calculated offset to find the longest sequence of identical words that matches a specific target value.
- `forward_to_helper_routine_2`: Calls one helper routine and returns.
- `compute_bitwise_checksum_2`: Computes a 16-bit checksum over a run of words by shifting each bit through an accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `branch_conditionally`: Performs a conditional branch to a specific memory address.
- `extend_and_store_word`: Extends a word to a long and stores it into a location relative to a frame pointer.
- `execute_supervisor_call`: This routine performs a series of operations and invokes a supervisor-level library call.
- `wait_for_flag_clear`: Repeatedly calls another routine while a specific global variable remains non-zero.
- `remove_object_from_table`: If the object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_game_state_flag_2`: Clears a 16-bit value in a global structure and sets a flag byte by performing a bitwise OR with a constant, then calls another routine.
- `clear_flag_byte_loop`: Clears a byte at an offset from a pointer, then iterates through a list of pointers, calling another routine for each non-zero entry until a counter reaches four.
- `reset_conditional_flag_byte`: Calculates an address using a global variable and a register-based index, clears the byte at that location, and sets it to two if the subsequent byte is one.
- `clear_state_and_wait_2`: Clears a global variable and a global word, then repeatedly calls another routine until it returns zero.
- `clear_slot_and_trigger_callback`: Calculates an offset from an input value and a base pointer to clear a byte at that location.
- `calculate_scaling_factor`: Performs fixed-point arithmetic to calculate a scaling value and stores the result in a table.
- `store_word_and_execute_subroutines`: Stores a word from a register into a location pointed to by a register, restores registers from the stack, loads a long from a table, and conditionally calls four subroutines before restoring registers and returning.
- `cleanup_device_and_library`: Checks a global pointer and, if non-zero, performs several operations including calling functions via pointers from global variables and updating a structure before clearing global pointers.
- `execute_signal_sequence_2`: Calls a sequence of three functions from a global table, passing a register as an argument to the second function, and returns a value formed by masking an input register with the result of that second function call.
- `update_task_status_and_result`: Clears a word in a structure, then if a flag is set, stores a value into the structure, calls a routine via a global function pointer, stores the return value, and updates the flag in the structure.
- `call_conditional_function`: If a register is non-zero, it checks a global function pointer and calls it, or calls a routine via an indirection if the pointer is null.
- `execute_conditional_subroutine`: Checks a global variable and, if non-zero, calls another routine four times.

### logged disk load from TV Sports Football (1988)(Cinemaware)[cr QTX][a2](Disk 1 of 2).adf

- `clear_string_buffer`: Iterates backward through a memory buffer to find non-space and non-null characters, then writes null terminators to clear the string.
- `store_function_results`: Calls a subroutine twice and stores the returned byte values into two different memory locations using an index derived from another byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_on_condition_and_call`: Checks a 16-bit value at offset -2b40 of the structure in a4 and branches if it is zero; otherwise, it decrements a 16-bit value pointed to by a3, and if that value is greater than zero and the carry flag is not set, it clears a byte at the location pointed to by a0, pushes a constant, and calls a routine at offset -7d9a using a pointer from the structure in a4.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_two_counters`: Clears two 16-bit fields, one at offset -2b00 and one at offset -2afe, of the structure in a4, then returns.
- `call_and_store_three_values`: Calls a routine via a pointer in a structure in a4 using a constant and a value from the stack, then calls another routine three times, passing a constant and two values from the stack each time and storing the returned results into consecutive 32-bit fields in the structure in a4.
- `call_routines_with_values`: Calls a routine at offset -32682, then calls a routine at offset -32154 three times, each time passing two 32-bit values read from the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_values`: Clears two bytes at offset -11465 of a structure, sets a 16-bit value at offset -11288 to 900 if a 16-bit value is less than 2, sets a byte at offset -13845 to 10 if it is zero, sets a byte at offset -13844 to 13 if it is zero, and if two 16-bit values at offsets -13886 and -13882 are zero, calls a routine via a pointer at offset -32378 and returns the result minus 1.
- `rotate_byte_and_call_handler`: Sets up its stack frame, then checks a 16-bit value at offset -2b40 of the structure in a4 and jumps away when that value is zero.
- `call_routines_with_scaled_values`: Calls three separate routines using pointers from a structure in a4.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to -(a7).
- `adjust_stack_pointer`: Adds 18 to the stack pointer and returns.
- `modify_and_call_structure`: Decrements a byte from offset -3615 of the structure in a4, performs several logical operations on d0 and d1, pushes two pointers onto the stack, and calls a routine at offset -7fda using a pointer from the structure in a4.
- `write_byte_to_pointer`: Increments d4 by 8, sets a0 to zero, performs logical operations on d0, pushes a constant, and stores d0 at the location pointed to by a0.
- `tiny_clr_and`: Small routine of 2 instructions that clears -(a7), then masks (a0) with d3.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `perform_signed_division_2`: Decrements d0 by 1, rotates the byte in d4 left by 4, and performs a signed division of d4 by d6.
- `transfer_and_mask_data`: Copies a 16-bit word from d0 to d1, performs a bitwise OR of d0 into a memory location at offset -17375 of a0, and performs a bitwise AND of the byte at the address pointed to by a2 into d1.
- `push_and_set_registers`: Pushes a 32-bit value onto the stack, then performs bitwise OR operations to set d0 to 0 and d6 to 6.
- `initialize_d6_values`: Pushes a 16-bit value onto the stack, clears d0, and performs three successive bitwise OR operations on d6 with the values 24, 30, and 36.
- `call_subroutine_with_d6_values`: Moves the value in d1 to a0, sets d6 to 48, pushes a memory address onto the stack, pushes a constant onto the stack, calls a routine at an offset from a4, sets d6 to 72, and clears a 16-bit value on the stack.
- `decrement_and_store_word`: Clears d0, sets d6 to 90, decrements d0, stores the word 6 at the address pointed to by a0 minus 1, and clears d1.
- `call_routine_and_decrement_d0`: Moves d1 to a0, sets d6 to 114, 120, and 126 through successive OR operations, calls a routine at an offset from a4, sets d6 to 138, 144, and 150, loads a byte from a memory location at an offset from a4 into d0, and decrements d0.
- `update_player_status_flags`: Reads a byte from a memory location pointed to by a register, performs a bitwise AND with a constant, and writes the result to a different byte in a structure.
- `clear_byte_and_branch`: Clears a byte at offset -2 of a5 and branches to a relative address.
- `increment_index_on_condition`: Checks two memory conditions based on a value at offset -2 of a5 and an offset from a4, then increments the byte at offset -2 of a5 if both conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_compare_byte`: Increments a byte at offset -2 of a5 and compares it to the value 2.
- `increment_and_call_routine`: Increments a byte at offset minus 3 of a structure in a5, compares it to 18, and if the value is less than or equal to 18, calls another routine and pushes a constant onto the stack.
- `shift_and_load_pointer`: Loads a byte from offset minus 2 of a structure in a5 into d0, shifts d0 left by 1 bit, and loads a pointer into a0 from offset minus 10368 of a structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_status_byte`: Loads a byte from a global variable into a register, performs a bitwise operation, and stores the result back into the same global variable before returning.
- `load_byte_and_adjust_stack`: Loads a byte from offset minus 13846 of a structure in a4 into d0, sets d1 to 34, and adds 4 to the stack pointer a7.
- `set_byte_via_pointer`: Loads a pointer into a0 from offset minus 13750 of a structure in a4 and performs a bitwise OR on a byte at the address pointed to by a6 using the value 236.
- `compare_bytes_and_clear`: Loads a byte from offset minus 11465 of a structure in a4 into d0, compares it to a byte at offset minus 11464 of a structure in a4, and if they are not equal, branches to another routine; otherwise, it clears a byte at offset minus 2 of a structure in a5.
- `load_byte_to_register`: Loads a byte from offset minus 11465 of a structure in a4 into d0.
- `load_byte_to_d1`: Loads a byte from offset minus 2 of a structure in a5 into d1.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies d1 to d2.
- `load_pointer_to_a0`: Loads a pointer into a0 from offset minus 22374 of a structure in a4.
- `load_byte_to_d3`: Loads a byte from offset minus 2 of a structure in a5 into d3.
- `copy_and_shift_data`: Copies a word from a table at offset d0 in a0 to a destination at offset d3 in a1, moves a long value from a global variable to another global variable, and shifts d0 left by 4 bits.
- `call_handler_with_shifted_byte`: Loads a byte from offset -2 of a5 into d1, shifts d1 left by 1 bit, calls another routine, and increments a7 by 2.
- `shift_and_increment_stack`: Loads a byte from offset -2 of a5 into d2, shifts d2 left by 1 bit, and increments a7 by 2.
- `update_counter_and_branch`: Copies a word from offset d0 in a0 to offset d2 in a1, increments a byte at offset -2 of a5, and branches to another routine if the byte at offset -2 of a5 is less than 3.
- `tiny_unlk`: Small routine of 1 instruction that removes the stack frame held in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
