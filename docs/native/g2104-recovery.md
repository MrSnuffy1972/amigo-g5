# Game 2104 (tv sports football (1988)(mirrorsoft)[cr qtx][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02104). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf:fb | 3 | 9208 | 471 |
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
| TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
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
| logged disk load from TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf | 1 | 145 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 48 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 214 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: file KBlocks.1 (header 406205 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 2 of 2).adf: file KBlocks.0 (header 352150 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 2 of 2).adf: file KBlocks.1 (header 385002 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
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
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: 85 executed PCs whose RAM was overwritten before any snapshot match its bytes 75344-75856: recovered as a unit run at an address
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
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: blocks 895-896 (bytes 458240-459264) hold ASCII text
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: blocks 898-900 (bytes 459776-461312) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.50)
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: blocks 903-1234 (bytes 462336-632320) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.78)
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf: blocks 1236-1696 (bytes 632832-868864) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.91)
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 2 of 2).adf: blocks 15-710 (bytes 7680-364032) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.59)
- tv sports football (1988)(mirrorsoft)[cr qtx][a2](disk 2 of 2)/TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 2 of 2).adf: blocks 895-1654 (bytes 458240-847360) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `execute_disk_resource_loop` (runtime segment an address): Repeatedly calls a function via a pointer in a record until it returns a non-zero value, then modifies hardware registers and a global variable. Kind: disk resource/track validation loop. Pass path: the loop breaks and proceeds to u28__sub_250A when iVar2 != 0.
- `configure_hardware_and_jump` (runtime segment an address): Sets specific values in custom chip registers, loads two pointers into registers, calls a routine via a pointer, and branches to another location. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `copy_state_byte_to_register` (runtime segment an address): Copies a state byte into a control word and writes a fixed flag value to a second control word. Kind: disk hardware/track validation. Pass path: the branch following the call (e.g., an address in sub_2524).
- `start_disk_sync_transfer` (runtime segment an address): Calls a helper, waits for a countdown loop, then writes DMA control, disk pointer, and sync words to custom chip registers before calling two more helper routines. Kind: disk sync/track check. Pass path: execution continues to the instructions following the call (e.g., movea.l an address(a4), a0).
- `process_disk_resource_loop` (runtime segment an address): Executes a loop that calls a function at an offset within a structure, updates a global variable, and modifies hardware registers. Kind: looping disk resource/track validation loop. Pass path: the loop breaks and proceeds to u39__sub_0342 when iVar2 != 0 (successful validation/resource availability).
- `configure_hardware_and_call` (runtime segment an address): Modifies custom hardware registers and calls a routine using a pointer from a table. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `update_control_word_flag` (runtime segment an address): Copies a byte from an offset of a pointer into a global variable and then overwrites that same variable with a fixed bitmask. Kind: disk hardware/parameter validation. Pass path: the branch at an address (bpl.b an address) which is taken if d1 remains positive after the decrement.
- `start_disk_sync_transfer_2` (runtime segment an address): Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers. Kind: disk hardware sync/track check. Pass path: execution continues to the next instruction (movea.l an address(a4), a0).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies a long value from one register to another and then performs a relative branch.

### EndCLI

- `find_max_and_populate_buffer`: Traverses a linked list to identify the maximum value in a specific field and then populates a memory buffer with a header and a sequence of incrementing values.
- `transfer_register_and_branch`: Copies the contents of one data register to another and then performs a short relative branch to another location.

### cd

- `find_max_and_populate_buffer_2`: Iterates through a linked list to identify the maximum value in a specific field and then populates a data buffer with a header and subsequent values.
- `move_and_branch_2`: This routine moves a long word from one register to another and then performs a relative branch.

### Port-Handler

- `parse_amiga_hunk_data`: Iterates through a data structure to identify and process AmigaDOS hunk types, such as executable or relocation code.

### Disk-Validator

- `format_numeric_string`: Converts a numeric value into a character representation and stores it in a buffer.

### TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf:fb

- `initialize_and_call_library`: Fills a memory range with zeros and then executes functions from a library based on a global pointer.
- `initialize_game_state_structure_2`: Resets multiple global status variables to zero and initializes hardware parameters.
- `execute_indirect_calls_3`: Performs several recursive calls and executes a function via a pointer retrieved from an offset of a register.
- `execute_callback_chain_3`: Calls two functions via pointers, passes a value to a third function, and then calls itself twice.
- `dispatch_via_table`: Uses a value from a table to select and call one of several different routines with provided parameters.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `branch_via_table`: Compares a value to a constant, optionally shifts it, and then jumps to an address retrieved from a table using the modified value.
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
- `parse_audio_header`: Processes a data structure by performing several memory operations, comparing byte strings, and making recursive calls.
- `modify_registers_and_memory`: Increments a pointer, negates a word at a memory location, performs a bitwise OR on a register, and decrements a byte.
- `compare_byte_strings`: Compares two byte strings byte by byte using a loop and a helper routine.
- `compare_and_update_global`: Compares a value retrieved from a function call with a previously stored value, then updates a global variable and returns a status based on the comparison.
- `test_bit_and_branch`: Tests a specific bit in a memory location and branches to another routine if the bit is not set.
- `branch_with_constant`: Loads a small constant into a register and branches to another location.
- `test_bit_and_branch_2`: Loads a pointer from a memory location, tests a specific bit in the pointed-to byte, and branches if the bit is not set.
- `test_bit_and_jump`: Tests a specific bit in a memory location and branches to a different location if the bit is set.
- `branch_with_zero`: Loads zero into a register and branches to another location.
- `check_bit_masks`: Checks if specific bits are set or cleared in a word from a memory location and returns a status value based on multiple bitmask comparisons.
- `check_bit_mask`: Checks if specific bits are set or cleared in a word from a memory location and branches accordingly.
- `branch_with_zero_alt`: Loads zero into a register and branches to a different location than the previous routine.
- `clear_memory_tables`: Performs multiple nested loops to zero out various segments of memory organized in specific patterns and dimensions.
- `check_structure_values`: Tests two different word values at offsets within a structure and returns if certain conditions are met.
- `update_calculated_offsets_2`: Checks a byte or short value within a structure to select one of three different calculation methods, then adds a value to one or more memory locations determined by those calculations.
- `update_conditional_offset_values`: Checks a condition at a specific offset within a structure to determine which set of three memory locations to update.
- `update_calculated_values`: Calculates three different memory addresses based on offsets and adds a value from a global variable to the contents at those locations.
- `accumulate_coordinate_offsets`: Clears a block of memory and then performs a series of additions, summing values from multiple source tables into a destination table using iterative loops.
- `call_library_function_2`: Calls a function via a pointer stored in a table and stores the result in a global variable.
- `call_function_with_argument`: Calls a function using a value retrieved from a global variable and passes four as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `write_to_hardware_registers`: Writes a value from a global variable and the constant one to specific hardware register offsets.
- `execute_recursive_calls`: The routine calls itself three times before returning.
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
- `iterate_coordinate_table`: Iterates through a loop to perform multiple function calls using incrementing values and updates a global variable.
- `clear_memory_location`: Stores a zero into a specific memory location.
- `test_and_branch`: Tests a word at a specific offset from a pointer and branches to another routine or returns if the condition is met.
- `sync_dual_state_structures`: Synchronizes two sets of state variables by performing XOR operations and conditional updates based on bitwise flags.
- `clear_and_update_structure`: Clears three words at specific offsets within a structure and conditionally copies three words from another memory location into the structure.
- `clear_structure_words`: Clears three word values at specific offsets within a structure and then performs a short branch.
- `execute_conditional_updates`: Tests a word value in a structure and, if non-zero, calls three different routines to update three specific words in the structure based on the returned values.
- `update_structure_values`: Clears three word values in a structure, checks a word value at an offset, and if non-zero, copies three word values from one location in the structure to another before branching.
- `update_structure_fields_2`: Clears several word values in a structure and conditionally copies values between different offsets based on table lookups.
- `clear_structure_fields`: Clears three specific word values at offsets within a structure and performs a conditional branch based on a test.
- `clear_memory_words_2`: Calculates an offset, tests a word at that location, and if certain conditions are met, copies three specific words from one set of memory locations to another.
- `clear_memory_words`: Copies three words from one set of memory locations to another and then branches to another routine.
- `clear_memory_locations`: Clears three words at specific offsets relative to a register and then branches to another routine.
- `format_numeric_value`: Decomposes an integer into individual digits and stores them in a local array.
- `process_callback_list`: Iterates through a list of entries, marking them with a specific value and decrementing a counter.
- `process_indexed_callback_list`: Iterates through a list of entries, marking processed items with a specific byte value to avoid repetition.
- `iterate_and_call_function`: Decrements a counter in a loop and calls a function via a pointer if a specific value in a table is not equal to thirty-two.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `call_indirect_functions`: Calls two different functions using pointers stored in a table, passing various parameters including sums of input values.
- `initialize_stack_frame`: Sets up a stack frame by linking and clearing a local word before branching to another routine.
- `copy_memory_block`: Copies bytes from one memory location to another using a counter and an index.
- `calculate_accumulated_value`: Performs a loop to combine a global variable with other values and stores the result in a global location.
- `update_complex_accumulator`: Uses a loop and bitwise operations to add a combination of global and local values to a stored integer.
- `increment_table_entry`: Calls a function, uses the result to calculate an offset into a table, and increments the word at that location.
- `update_accumulated_sum`: Calculates a sum using two values from memory and a global variable, then stores the result back into one of those memory locations after adding multiple instances of the global variable to it.
- `copy_and_reset_data`: Copies a longword from one location to another and then sets the original location to all ones.
- `copy_value_via_pointer`: Calls another routine and uses its return value as a base pointer to store a word at a specific offset.
- `handle_disk_prompt`: Copies a string into a buffer and manages a loop that waits for a disk or button input.
- `process_input_loop`: Manages a loop that processes input via function pointers and performs conditional branching based on returned values.
- `call_and_copy_data`: Calls a function via a pointer and then calls another routine to copy data.
- `process_data_loop`: Executes a series of function calls and enters a loop that performs conditional checks and updates a global variable based on the results of retrieved pointers.
- `execute_conditional_logic_2`: Performs a series of nested loops and conditional checks.
- `process_linked_list_nodes`: Iterates through a linked list structure, performing conditional branches and executing function pointers stored within the list elements.
- `decrement_and_branch_on_d0`: Subtracts a value from a register, performs conditional branches based on the result, and may branch to another location.
- `branch_to_offset_2`: Performs a short relative branch to another location.
- `branch_to_offset_4`: Performs a short relative branch to a specific location within another routine.
- `process_game_logic`: A large routine that calls several subroutines to update game state, graphics, display parameters, and check button status.
- `update_game_state_and_graphics`: Copies a block of data from a source table to a local stack buffer and performs a series of state updates by calling various subroutines with specific parameters.
- `copy_data_block`: If a specific global flag is set, it clears that flag and copies five words from one memory location to another.
- `update_display_parameters`: Calls several functions to retrieve a value, calculate a new offset, and perform multiple writes to a table using that offset and provided parameters.
- `check_button_status`: Checks if two specific bits are set in a global variable and returns one if both are active, otherwise returns zero.
- `pass_parameters_to_subroutine`: Passes multiple arguments through to another routine.
- `calculate_blitter_clipping_bounds`: Calculates the intersection of a rectangular area with a specified bounding box and updates clipping parameters.
- `compare_and_branch_word`: Loads a word from an offset of a register, compares it to a value at an offset of another register, and performs conditional or unconditional branches.
- `compare_and_branch_2`: Loads a word from a location relative to a register, compares it to a value, and branches based on the carry flag.
- `mask_and_store_word`: Performs a bitwise AND on a word from a table and stores the result into a location relative to a register.
- `calculate_blitter_clipping_parameters_2`: Calculates adjusted source and destination coordinates, bitplane masks, and dimensions for a rectangular blitter operation based on clipping.
- `calculate_blitter_clipping_parameters`: Calculates adjusted source and destination coordinates, bitplane masks, and dimensions for a rectangular blitter operation based on clipping bounds.
- `update_blitter_area_parameters`: Calculates new source and destination coordinates, bitplane masks, and dimensions for a rectangular blitter operation.
- `calculate_blitter_area_and_bitplanes`: Calculates source and destination coordinates and bitplane masks for a rectangular area, then iterates through a bitmask to update blitter control registers.
- `configure_blitter_parameters`: Calculates source and destination offsets and bitplane masks before iterating through a bitmask to update blitter control registers.
- `calculate_offset_and_test`: Manipulates a value in a register using logical shifts and additions to calculate an offset.
- `configure_blitter`: Manipulates various hardware registers related to the blitter and performs complex data processing and conditional branching.
- `branch_to_subroutine`: Performs a short relative branch to a location within another routine.
- `execute_structure_callbacks`: Resets a global pointer and executes several function pointers stored within a structure if their corresponding values are non-zero.
- `call_structure_callbacks`: Calls a routine that resets a global pointer and executes a series of function pointers stored within a structure if they are non-zero.
- `initialize_game_state_structure`: Populates a structure by calling a sequence of initialization and clamping routines, then performs multiple conditional lookups to store specific values into a table.
- `process_event_data`: Checks a global flag, calls several functions to process parameters, and conditionally performs a write to a memory location.
- `execute_conditional_update`: Checks a global flag and calls a sequence of functions to retrieve, transform, and write data based on provided parameters.
- `load_and_parse_resource_file`: Opens a file, verifies a specific magic number header, and reads a structured data block into a destination table.
- `parse_and_process_encoded_data`: Extracts values from a nibble-encoded source buffer and populates a destination table.
- `unpack_and_populate_table`: Extracts nibble-encoded values from a source buffer and stores them into a destination table.
- `process_compressed_data_blocks`: Iterates through a source buffer to extract and store data values into a destination table.
- `update_table_with_zero`: Iterates through a loop to store zero into a table at calculated offsets.
- `update_table_with_max`: Iterates through a loop to store a maximum value into a table at calculated offsets.
- `update_table_with_data`: Loads a value from a table and stores it into another table at calculated offsets.
- `branch_on_decrement`: Decrements a register and performs conditional branches based on whether the value reaches zero at specific stages.
- `increment_and_update_table_entry`: Increments a word-sized counter stored at a negative offset from a base pointer, then uses the scaled counter to calculate an address and update a longword at that location.
- `call_subroutine_via_pointer`: Pushes a value from the stack, calls a subroutine via a pointer, cleans up the stack, and performs a relative branch.
- `iterate_and_call_table_entries`: Resets a counter and iterates through a table of up to eight pointers, calling a routine for each non-zero entry found.
- `expand_compressed_data_blocks`: Decompresses data from a source buffer into a destination buffer using a nested loop structure.
- `loop_and_branch`: Performs a loop based on a counter and then branches to another location.
- `copy_nested_data_blocks`: Performs a multi-level nested loop to copy bytes from a source to a destination, using values from a table to determine loop counts and offsets.
- `execute_loop_branch`: Decrements a counter and branches to a location that may result in a loop.
- `return_stack_value`: This routine returns a value currently held on the stack.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_conditional_memory_updates`: Increments a global counter and conditionally calls various routines to copy data to different memory locations based on the state of several short-integer flags.
- `update_conditional_state`: Checks a value from a library call and, if it meets a condition, initializes several fields in a structure and calls another routine with specific arguments.
- `reset_global_pointer`: Calls a function with a constant and a pointer, then calls another routine and sets a global variable to a maximum value.
- `initialize_gameport_input_state`: Allocates and initializes a structure to manage input device states by querying the gameport device.
- `execute_indirect_calls_2`: Calls a routine with several arguments and then executes two consecutive indirect function calls via a pointer located at an offset from a register.
- `clear_and_refresh`: Sets a word in a global location to zero and calls another routine twice.
- `remove_linked_item`: Traverses a linked list to find a specific value and removes that node by updating the preceding pointer.
- `swap_linked_list_nodes`: Traverses a linked list and swaps the pointers of two nodes based on a comparison result.
- `traverse_and_swap_pointers`: Iterates through a linked list and swaps a pointer from a register with a pointer found within the list nodes.
- `swap_two_values`: Copies the value from one memory location to another and then attempts to assign a pointer to the original location.
- `configure_blitter_draw_modes`: Calculates and sets blitter control registers based on dimensions and bitplane counts provided in a data structure.
- `configure_blitter_graphics_parameters`: Sets multiple blitter control and mode registers using values extracted from a data structure.
- `update_blitter_registers_3`: Moves a word into a blitter register, retrieves two pointers from a structure, and conditionally moves a long value into a blitter register.
- `update_blitter_registers_2`: Updates the blitter destination address and control registers using values retrieved from a structure.
- `configure_blitter_parameters_2`: Calculates values based on data from a structure and updates several Blitter hardware registers and a global variable.
- `update_blitter_registers`: Loads a value from a table into a register and, if not negative, writes it to a blitter destination address register and sets a bit in a global variable.
- `update_blt_registers`: Updates several Blitter hardware registers and modifies a global variable based on a pointer and a calculation.
- `compare_table_ranges`: Compares the boundaries of two structures by checking if their respective offsets and sizes overlap.
- `compare_table_offsets`: Calculates a value by adding two words from a table and subtracting one, then compares the result against a word in another location.
- `calculate_and_compare_offset`: Adds a value from a table to another value from the same table, decrements the result by one, and compares it against a value in another memory location.
- `compare_and_branch`: Performs arithmetic on values retrieved from a table and compares the result against a value in another memory location to determine a branch.
- `store_and_branch`: Stores two values into a table and branches to another location.
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
- `compare_and_subtract_string_values`: Iterates through a null-terminated string, calling two routines to convert and compare values until a mismatch is found.
- `subtract_converted_values`: Calls a routine to convert two values and returns the difference between them.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_immediately_3`: The routine is empty and immediately returns to its caller.
- `call_allocation_routine`: Pushes two arguments onto the stack, calls a routine via a pointer in a register, and stores the result in a location pointed to by a register.
- `modify_linked_node`: Allocates a new node, populates it with several constant word values, and updates pointers within a linked structure.
- `modify_linked_list_node`: Swaps pointers within a linked list and writes several constant word values into a node.
- `traverse_linked_list_5`: Iterates through a linked list of pointers, comparing strings at each node until a match is found or a null pointer is encountered.
- `update_pointer_and_branch`: Loads a pointer from an offset of a register into a second register, stores the value pointed to by that register into a memory location at an offset of the first register, and then performs a short branch.
- `call_external_routine`: Pushes a value from a memory location onto the stack, calls a routine via a pointer, adjusts the stack pointer, and branches to another location.
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
- `allocate_and_initialize_node_tree`: Allocates memory for a new structure and recursively populates its child nodes and properties.
- `traverse_linked_list`: Recursively traverses a linked list structure, calling a routine on each node and its children, and performing memory deallocation on specific elements.
- `search_and_store_pointers`: Iterates through a list of words to find matches for a descending sequence of values, storing the resulting pointers in a destination table.
- `search_conditional_table`: Searches a table for a value based on a parameter, potentially performing a second search in a different table if a specific bit is set in a byte within a structure.
- `process_blitter_list`: Iterates through a list of pointers, updating a structure with values from registers and pointer-dereferenced values for each entry in a loop while interacting with Blitter hardware.
- `update_hardware_registers`: Calls another routine and then writes a word and a long value to specific offsets in a structure pointed to by a register.
- `update_table_values`: Iterates through a list of pointers, updating a structure with values from registers and pointer-dereferenced values for each entry in a loop.
- `open_file_handle`: Attempts to open a file using provided parameters, stores the resulting handle in a global variable, and calls another routine if the handle is null.
- `handle_file_error`: Checks if a global file handle is non-zero and if a parameter is non-zero, then attempts to perform an operation and closes the file handle if an error occurs.
- `close_file_handle`: Checks if a global variable contains a non-zero handle, closes the file if it does, and resets the handle to zero.
- `load_asset_data_structures`: Allocates memory for a collection of objects based on two count values and populates them by reading data from files.
- `extend_byte_to_long`: Saves registers to the stack, extracts a byte from a local offset, and extends it to a long integer before branching.
- `process_linked_list_nodes_2`: Recursively traverses a linked list, updating several fields in a structure and clearing specific memory locations based on bitwise checks.
- `pass_pointer_to_function`: Retrieves a pointer from a global structure and an integer from a register to pass them as arguments to another routine.
- `recurse_unconditionally`: The routine immediately calls itself, creating an infinite recursion loop.
- `call_extend_byte_to_long`: Calls a routine that extends a byte from a stack offset into a long integer and branches to another routine.
- `set_global_flag_via_pointer`: Sets a single byte in a global memory location to one using a pointer retrieved from a data table.
- `decrement_structure_word`: Decrements a word in a structure and clears a global variable if the result is less than or equal to zero.
- `iterate_and_compare_values`: Increments a word in a structure within a loop until it matches a value from another structure, then returns a boolean result.
- `compare_and_clear_register`: Compares a word from a table with a value in a register and, if certain conditions are met, clears the register and returns.
- `clear_register_d0`: Clears the d0 register and branches to another location.
- `traverse_linked_list_4`: Iterates through a linked list, performing conditional calls to other routines and updating coordinate values within the list nodes.
- `push_calculated_pointers_and_call`: Calculates two memory addresses by adding offsets to values retrieved from structures and pushes them onto the stack along with a third pointer before calling another routine.
- `update_list_coordinates`: Iterates through a linked list while performing arithmetic on structure members and calling another routine based on a bit flag.
- `traverse_linked_list_2`: Iterates through a linked list while updating several values in a structure and performing conditional calls based on bits in the list nodes.
- `call_and_return_word`: Calls a routine via a pointer stored in memory and returns a word from a location relative to a register.
- `increment_counter`: Increments a value in a global variable if it is not zero.
- `decrement_counter`: Decrements a global variable and resets it to zero if it becomes negative.
- `calculate_and_draw`: Calculates two short values based on input parameters and a table, then calls a routine to perform a drawing operation using those values.
- `insert_sorted_node`: Allocates a new structure, populates it with data from a source, and inserts it into a linked list in ascending order.
- `insert_into_sorted_list`: Populates a new structure with data from a source and inserts it into a linked list in ascending order based on a word value.
- `copy_pointer_to_table`: Copies a pointer from one memory location to another and then stores that same pointer into a second location.
- `store_pointer_to_table_2`: Copies a pointer from a register into a table and then copies that same pointer into a location pointed to by a global variable.
- `store_pointer_to_table`: Copies a value from a register into a location pointed to by an offset from another register and then branches.
- `check_structure_and_return_pointer`: Compares a value in a structure against a register and returns a pointer from a different location if they match.
- `compare_and_return_pointer`: Compares a byte in a structure to a register and branches if they are not equal, otherwise returns a pointer in a register.
- `remove_list_element`: Traverses a linked list to find and remove a node that matches specific criteria, then frees the memory for that node.
- `traverse_and_free_list`: Recursively traverses a linked list and calls a routine to free each node.
- `verify_resource_header`: Calls another routine to load data and checks if the returned value matches a specific constant.
- `set_d0_to_one`: Sets the d0 register to one and branches to another location.
- `clear_d0_and_branch`: Clears the d0 register and performs a short branch to another routine.
- `process_data_table_2`: Iterates through a table of eight entries, performing a calculation with values from the table and calling another routine before clearing the entries.
- `allocate_node_from_pool`: Attempts to allocate memory by calling another routine with a pointer and a size.
- `allocate_and_recurse_node`: Retrieves a value from a structure, uses it to allocate memory, and either frees the structure if allocation fails or assigns the new memory to a pointer, sets a flag, and calls itself recursively.
- `update_node_and_recurse`: Swaps pointers in a linked list, sets a word in a node to one, and then calls another routine recursively.
- `search_string_list`: Traverses a linked list of pointers to compare byte strings until a match is found or the list ends.
- `transfer_pointer_and_branch`: Moves a pointer from a memory location into a register, stores that pointer back into a different location, and then branches to another routine.
- `push_call_and_branch`: Pushes a value from a memory location onto the stack, calls another routine, adjusts the stack pointer, and performs a short branch.
- `clear_and_test_bit`: Clears a specific bit in a byte at an offset from a pointer, then tests a long value at an offset from that same pointer to determine whether to branch.
- `check_nested_pointer`: Loads two pointers from a structure and branches to another location if a value at an offset from the second pointer is non-zero.
- `update_blitter_parameters`: Calls a routine to calculate a value based on data from a nested structure and stores the result in a table.
- `clear_and_test_flag`: Clears a longword at an offset from a pointer and returns the original value in a data register.
- `test_and_branch_2`: Tests a long word at an offset from a pointer and branches if it is zero.
- `execute_callback_with_result_check`: Pushes two pointers onto the stack and calls a subroutine via a pointer in a register.
- `execute_conditional_callback_3`: Pushes two longwords onto the stack, calls a subroutine, and then either branches based on a word value or calls another subroutine.
- `update_and_call_subroutine`: Copies a longword from a memory location to a destination pointed to by a register, tests a specific bit in that destination, and either branches or pushes the stack pointer to call another routine before returning via a jump.
- `execute_callback_and_clear`: Checks if a long word at an offset from a pointer is non-zero, calls a routine using that value, and then clears the original value.
- `process_object_callback`: Checks if a parameter is non-zero, then executes a function pointer at a specific offset from a base address, conditionally executes another function pointer using a value from an offset within the parameter, and finally returns the result of a third function call using the parameter and a constant.
- `clear_pointer_after_call`: Checks if a value at a specific offset in a structure is non-zero, executes the function pointed to by that value, and then sets that offset to zero.
- `execute_callback_chain_2`: Calls a routine with a parameter, then executes two function pointers stored within a structure using values derived from a third function call.
- `traverse_and_execute_list`: Iterates through a linked list of pointers, executing a routine via a pointer found within each node.
- `initialize_animation_resources`: Attempts to allocate and initialize animation bitmaps, rastports, and views by recursively populating a node tree and allocating memory arrays.
- `swap_and_call_pointers`: Swaps two pairs of longwords stored in memory and then executes a series of function calls via pointers retrieved from a structure.
- `execute_library_calls`: Calls three different routines using pointers retrieved from global variables.
- `decrement_and_recurse_list`: Decrements a value in a global variable and calls itself recursively if the value reaches zero.
- `traverse_and_clear_list`: Recursively traverses a linked list, calling a routine on each node, performing specific memory writes, and clearing several pointers and values in a structure.
- `traverse_and_modify_chain`: Calls another routine if a pointer is non-zero, clears a specific bit in the pointed-to data, and follows a chain of pointers in a loop.
- `process_pointer_chain`: Loads a pointer from a memory location, clears a bit in the value pointed to, and follows a chain of pointers in a loop.
- `execute_and_unstack`: Calls a routine, tests a register, and if non-zero, unloads a register before returning.
- `execute_sequence_and_clear_2`: Clears several global memory locations, performs a series of function calls via pointers, and enters a loop that executes a routine based on a bit-shifted value.
- `execute_sequence_and_clear`: Pushes a value from memory onto the stack, calls a routine, clears a memory location, and calls another routine before branching.
- `update_structure_from_linked_list`: Iterates through a linked list to update a global structure's fields based on node type identifiers.
- `process_linked_list_data`: Iterates through a linked list to update values in a structure based on data found within the list nodes.
- `update_structure_from_list`: Traverses a linked list to update specific fields in a structure using values extracted from the list elements.
- `initialize_input_structure`: Performs a series of library calls and structure updates based on the results of an input device check and pointer validation.
- `reset_structure_fields_2`: Checks a short value at an offset from a pointer, and if non-zero, writes a constant to a structure field and updates another field with a calculated value before calling another routine twice.
- `allocate_and_fill_array`: Allocates a block of memory and then fills an array within that block by calling a routine to retrieve values for each element.
- `execute_indirect_calls`: Retrieves two function pointers from a table using an offset from a base address and executes them with provided arguments.
- `allocate_and_initialize_structure_2`: Clears several fields in a structure, allocates memory via a library call, and then conditionally performs a sequence of nested allocations and field updates based on the success of each step.
- `reset_structure_and_call_routines`: Conditionally calls other routines using pointers from a structure and then clears multiple fields within that structure to zero.
- `update_state_from_table`: Performs a series of checks and updates a structure by comparing values from a table and copying data from a source address to a destination buffer.
- `search_and_copy_string`: Calls a routine to obtain a value, then either calls a library function to find a pointer or performs a conditional memory allocation and data copy based on specific input parameters.
- `execute_recursive_call`: This routine performs a direct recursive call to itself using the provided parameters before returning.
- `calculate_file_offset`: Calculates a relative offset based on values in a table and updates a parameter, or calls another routine if certain conditions are met.
- `transfer_data_in_chunks`: Calculates a data difference and transfers that amount of data from one location to another using a loop that processes fixed-size chunks.
- `call_data_transfer_routine`: Calls a routine that transfers data in fixed-size chunks after calculating a length from a table.
- `branch_if_conditions_met`: Tests two word values at specific offsets from a pointer and branches to different locations based on whether they are zero or non-zero, potentially calling another routine.
- `load_register_and_branch`: Loads a longword from a memory location into a register and then branches to another routine.
- `close_file_handle_2`: Calls a routine if certain conditions are met, or calls another routine with a specific value if a pointer is non-zero.
- `set_library_parameters`: Writes specific values to a structure and then calls a routine using a pointer from a global variable.
- `reset_library_parameters`: Writes specific values to a structure and calls a routine, then writes different values to the same structure and calls the routine again if a condition is met.
- `copy_and_uppercase_string`: Copies up to four bytes from a source string to a local buffer while converting lowercase ASCII characters to uppercase.
- `search_and_update_structure`: Searches through a table for a matching value and, if found, conditionally calls routines to update fields within a structure.
- `iterate_and_update_blocks`: Checks a global flag to determine a starting address and then iterates up to eight times, calling another routine and a library function with incrementing pointers.
- `validate_and_update_state`: Compares two values at a specific offset in a structure and, if they do not match, updates several fields in a different structure before calling another routine.
- `call_search_routine`: Calls another routine passing a value and a length derived from a global variable.
- `search_integer_array`: Iterates backward through an array of long values to check if a specific value exists within the specified count.
- `parse_format_string`: Stores a pointer into a global variable and calls another routine with two parameters.
- `increment_and_store_byte`: Increments a global counter and stores a byte from a register into a location pointed to by a global pointer.
- `parse_format_string_2`: Calls a routine that parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `call_recursive_routine`: A routine consisting of 48 instructions that calls another routine and returns.
- `parse_format_specifier`: Processes a format string to extract width, precision, and type specifiers for numeric or string data.
- `store_short_value`: Stores a word value into a location relative to a pointer and then performs a short branch.
- `update_status_flag`: Writes a word to an offset relative to a pointer and performs a short branch to another routine.
- `parse_formatted_string`: Parses a string containing numeric values and format specifiers, such as percentage signs or signed integers.
- `format_numeric_string_4`: Converts a signed integer into a string representation, supporting fixed-point and scientific notation formats.
- `loop_and_update_structure`: Calls two different routines using pointers and then enters a loop that updates a value in a structure and repeats the process.
- `validate_numeric_range`: Compares a value at a memory offset against a constant and clears a byte if the value falls within a specific range.
- `check_numeric_range`: Compares a value at a memory offset to a constant and performs arithmetic on a register based on the result.
- `format_floating_point_string`: Converts a numeric value into a string representation, handling decimal points, fractional digits, and scientific notation with exponent formatting.
- `clear_offset_word`: Clears a word at a negative offset from a pointer and branches to another location.
- `increment_and_store`: Loads a word from a pointer, increments it, stores the result back to a different offset of that pointer, and branches.
- `format_numeric_string_3`: Converts a numeric value into a string representation by iterating through digits and appending characters to a buffer.
- `format_numeric_string_2`: Converts a numeric value into a string representation, handling signs, decimal points, and integer division to append characters to a buffer.
- `append_character_to_string`: Tests a word value and branches; if a condition is met, it writes a specific character to a memory location pointed to by a register, potentially handling negative values by negating them first.
- `convert_integer_to_string`: Converts a 16-bit integer value into a null-terminated ASCII string representation.
- `dispatch_callback_via_structure`: Checks a global flag and, if zero, initializes a structure and calls a routine to populate it.
- `dispatch_callback_via_structure_2`: Checks a global flag to determine if a cached value must be recomputed by calling a routine through a pointer.
- `execute_conditional_callback_2`: Checks a global flag and, if zero, initializes a structure and executes a sequence of function pointers.
- `update_callback_state`: Checks a status flag and, if zero, initializes a structure and executes a sequence of function calls through pointers.
- `dispatch_callback_via_structure_3`: Checks a global flag and, if zero, initializes a structure and calls a routine to populate it.
- `execute_deferred_callback`: Checks a global variable to determine if a cached routine needs to be initialized.
- `initialize_and_dispatch_callback`: Checks a status flag in a structure and, if not initialized, populates a data block via a recursive call and a secondary function pointer.
- `execute_callback_via_structure_2`: Checks a global flag and executes a routine using an offset derived from a structure.
- `initialize_and_dispatch_callback_2`: Checks a global state variable to determine if a specific routine needs initialization.
- `execute_callback_via_structure`: Checks a global flag, potentially calls a function through a pointer, and eventually executes a routine using an offset derived from a structure.
- `skip_specific_bytes`: Checks if a byte at a pointer is a space or a digit, incrementing the pointer if a match is found.
- `check_byte_and_increment`: Compares a byte at a pointer to a constant and increments the pointer if a match is found.
- `skip_if_byte_matches`: Compares a byte at a pointer to a constant and, if it matches, increments the pointer and clears a register before branching.
- `convert_string_to_integer`: Parses a string of numeric characters into a signed integer value.
- `return_immediately_4`: The routine executes four instructions and performs no observable work before returning.
- `save_registers_and_branch`: Saves several registers to the stack and branches to another location.
- `fill_memory_backward`: Fills a memory range with a byte value by iterating backward from a starting pointer.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `compare_and_decrement_pointer`: Compares a byte from a pointer to a value in a register and, if they match, returns the pointer decremented by one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_memory_and_call_library`: Fills a memory range with zeros and calls functions from a library based on a global pointer.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tokenize_string_into_array`: Parses a source string by splitting it into individual tokens based on whitespace or quoted substrings.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `check_table_offset`: Saves registers, loads a pointer from the stack, compares a word at that pointer to a constant, and either branches or increments the pointer before branching.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `initialize_register_and_branch`: Loads a specific word into a register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_icon_library_function`: Retrieves a pointer from a structure, calls a library function, and stores the result in a global variable before conditionally branching.
- `process_window_data`: Calls library functions to retrieve data and updates a structure with the results.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Checks the signs of two registers, negates them if they are negative, and calls another routine to perform a division.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `remove_linked_list_node`: This routine calls another routine that removes a node from a linked list structure.
- `traverse_linked_structure_2`: A routine that calls another routine to traverse a linked structure and update its fields.
- `traverse_recursive_linked_structure`: This routine calls another routine to traverse a linked structure and update its fields using conditional logic and recursion.
- `traverse_linked_list_3`: Iterates through a list of structures by incrementing a pointer and calling another routine for each entry.
- `process_recursive_structure`: Traverses a nested structure recursively based on flag bits, clearing the structure's fields after processing.
- `process_recursive_structure_2`: Traverses a linked structure and updates its fields based on conditional logic and recursive calls.
- `clear_list_elements`: Iterates through a list of structures and clears the first three longwords of an entry if a specific byte is zero.
- `branch_to_offset_3`: Performs a short branch to a specific instruction offset.
- `clear_and_branch`: Clears several longwords at offsets from a pointer in a register and then performs a short branch.
- `update_status_flags`: Updates various fields in a structure based on the return value of another routine.
- `iterate_linked_list_and_call_function`: Traverses a linked list starting from a pointer stored in a global variable, passing each node and a value derived from the node's second word to another routine.
- `insert_into_linked_list`: Allocates a new node and inserts it into a linked list by updating pointers.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `unlink_linked_list_node`: Traverses a linked list to find a specific node matching a provided pointer, then updates the previous node's pointer to bypass it.
- `validate_index_and_call_function`: Checks if an index is within valid bounds and if a corresponding entry in a table is non-zero before calling another routine.
- `handle_file_deletion`: Attempts to delete a file and uses another routine to check for errors, storing the result in a global variable.
- `perform_conditional_io`: A routine that calls library functions for writing and error reporting based on specific conditions.
- `execute_conditional_write`: Calls a routine to perform a write operation and conditionally calls another routine based on the result.
- `execute_write_operation`: Calls a function to write four bytes from a specific memory location and then calls another routine to handle the result.
- `execute_callback_and_routine`: Checks if a global function pointer is non-zero to execute it, then calls another routine.
- `process_table_entry`: Validates an index against a table size, calls another routine if a bit is clear, and clears the entry in the table.
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
- `allocate_and_initialize_structure`: Allocates a memory block via a library call and populates its fields with provided parameters and results from another routine.
- `reset_structure_fields`: Sets specific bytes and longwords in a structure to fixed values and calls another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: Calls another routine passing the input parameter and a fixed constant value of 48.
- `destroy_display_entry_2`: Calls another routine passing the input pointer as an argument.
- `update_structure_fields`: Calls a function via a pointer in a global variable and, if successful, writes a byte, a word, and a longword to specific offsets within the returned memory address.
- `clear_structure_and_call`: Sets three specific fields in a structure to fixed values and calls another routine using a value from the structure.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `copy_stack_and_update_data`: Copies a block of data from a global table to the stack, then updates a structure's fields based on the results of a library call and another routine.
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
- `execute_library_call`: Performs an indirect call to a function located at a specific offset within a library base.
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
- `store_register_to_global`: Library code that stores the value of a register into a global variable.
- `update_linked_list`: Library code that updates a linked list structure and calls a function through a pointer.
- `set_status_bit`: Library code that sets a specific bit in a byte located at an offset from a register.
- `increment_and_call_library`: Library code that checks a bit, increments a short value at an offset from a register, and conditionally calls another library routine.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `call_other_routine`: Library code that calls another routine.
- `calculate_bitfield_difference`: Library code that performs a loop to calculate a value by repeatedly shifting and subtracting based on an input value and a doubling accumulator.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `modify_and_call_routine`: Library code that moves a long value, calls a routine, and then sets or clears specific bits in a byte before branching.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `library_branch_clear`: This library code clears a long register and branches to another location.
- `execute_conditional_callback_chain`: Retrieves a base address from a global variable and executes a sequence of function pointers at specific offsets based on conditional checks of register values and return flags.
- `execute_library_callbacks_2`: Library code that executes a series of function pointers from a jump table based on register values.
- `execute_library_callbacks`: Library code that executes various routines through pointers at specific offsets from a base address based on register values.
- `call_offset_routines`: The library code moves a value to a register, calls a routine at a negative offset from a base pointer, and then conditionally calls two other routines at different negative offsets based on a register test.
- `execute_and_store_result`: Library code that calls a routine using an offset from a base pointer and stores the result back into a register.
- `branch_on_register_test`: Library code that moves a value, tests a long register, and either branches or loads a constant into a register before branching.
- `mask_return_value`: Library code that calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_integer_power`: Library code that performs bit-shifting and iterative addition to compute a value based on an exponent.
- `shift_and_load_registers`: Library code that performs a logical shift right on a longword in a data register and loads several registers with constant values and a pointer.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_word_and_branch`: Library code that pushes a word onto the stack and performs a short relative branch.
- `library_push_and_branch_alt`: Library code that pushes a different word onto the stack and branches to another location.
- `library_push_and_branch_short`: Library code that pushes a small constant onto the stack and branches to another location.
- `compare_and_return`: Library code compares a byte in a register to a constant, branches if higher, and returns after adjusting the stack pointer.
- `perform_math_transform`: Library code that performs complex bitwise shifts and arithmetic operations on a value before iterating through a table of integers.
- `perform_iterative_bit_manipulation_2`: Library code performs iterative bitwise shifts and arithmetic additions or subtractions on values stored in registers while incrementing a pointer through a table of integers.
- `perform_iterative_bit_manipulation`: Performs a series of bitwise shifts and arithmetic additions or subtractions on values stored in registers and a memory table.
- `perform_complex_math_2`: Library code performs arithmetic operations involving shifts and subtractions within a loop and may call itself recursively.
- `perform_complex_math`: Library code performs a series of arithmetic operations involving shifts and subtractions within a loop, potentially calling itself recursively.
- `load_and_branch`: Library code loads a constant into a register and then performs a short branch.
- `recursive_call_wrapper`: A library routine that calls itself and then returns.
- `save_and_call_routine`: Library code that pushes a longword to the stack, calls another routine, and restores a value from the stack.
- `check_and_clamp_value`: The library code moves a value to another register, branches if it is negative, and then clamps the value to zero if it falls within a specific range.
- `shift_and_jump`: Library code performs an arithmetic shift right on a register, increments it, and then either branches or jumps to a zero address.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_rotations`: Library code checks a byte and, if a bitwise rotation condition is met, calls another routine multiple times with modified values.
- `rotate_and_branch`: Library code loads a constant into a register, performs a rotate right through the carry flag, and then performs a short branch.
- `load_constant_and_branch`: Loads a specific 32-bit constant into a data register and performs a short relative branch to another location.
- `test_stack_word`: This library code pops a word from the stack into a register, tests its value, and performs a conditional branch or clears a register.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `return_immediately`: This library routine consists of four instructions and performs no observable work, immediately returning to its caller.
- `normalize_fixed_point`: Library code that performs arithmetic shifts and conditional branches to normalize a value.
- `process_library_math`: Library code that performs arithmetic shifts, stack operations, and fixed-point normalization.
- `process_table_with_shifts`: The library code performs arithmetic shifts on two registers and then enters a loop that adds the shifted values to registers and subtracts values from a table pointed to by a register.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_branch_logic`: Library code that tests a byte, branches based on its sign, and performs a bitwise AND before calling another routine.
- `load_constant_and_branch_2`: This library code loads a constant value into a register and then performs an unconditional branch.
- `execute_complex_math_loop`: Library code that performs bit-shifting, arithmetic, and conditional branching within a loop to process values.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_byte`: Library code tests a byte, branches if positive, masks the byte, and calls another routine.
- `process_byte_with_logic`: Library code performs conditional checks on a value and executes various routines based on arithmetic comparisons and bitwise manipulations.
- `return_from_routine`: The routine performs no operations and immediately returns to its caller.
- `transform_register_bits`: Library code that performs a series of swaps, rotations, and arithmetic operations on a register.
- `modify_byte_value`: This library code compares a byte against two constants, performs arithmetic and a rotate operation if they match, and then tests the result to determine the next branch.
- `mask_and_test_word`: This library code performs a bitwise AND on a word, then tests the result to determine whether to branch or clear a register.
- `test_and_shift_byte`: The library code compares a byte to a constant, branches if they are not equal, and then performs logical shifts on a long register.
- `shift_and_set_flag`: This library code performs a logical shift left on a word, followed by a rotate through carry and setting a bit in the condition code register.
- `clear_register_and_branch`: This library code clears a register and then performs an unconditional branch.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `invoke_callback_and_return_2`: The library code calls a routine pointed to by a register and returns the original value of a different register.
- `invoke_callback_and_return_5`: Library code receives a pointer in a register, calls the routine at that address, and returns the original value from a long register.
- `invoke_callback_and_return_3`: Library code calls a routine pointed to by a register and returns the original value in a data register.
- `invoke_callback`: Library code that calls another routine and returns a value.
- `library_recursive_call`: Library code that calls itself recursively.
- `invoke_callback_and_return_6`: This library code calls a routine via a pointer and returns the original value from a register.
- `invoke_callback_and_return_7`: This library code executes a subroutine through a pointer and returns the initial value held in a register.
- `store_register_via_pointer`: The routine invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `invoke_callback_and_return_4`: The routine calls a subroutine via a pointer in a register and returns the original value held in a data register.
- `call_routine_repeatedly`: Library code calls a routine and may call it again depending on a condition.
- `call_indirect_routine`: The routine calls another routine and returns the original value held in a data register.
- `invoke_callback_2`: Library code that performs a call to another routine and returns a value.
- `invoke_callback_3`: Library code that executes a routine via a pointer and returns a value.
- `call_routine_thrice`: Library code calls another routine three times in succession before returning.
- `test_and_call_indirect`: Library code tests a byte, branches if positive, masks the byte, and calls a routine via an indirect pointer before returning a value.
- `pass_through_value`: This routine calls another routine via a pointer in a register and returns the original value that was passed in a data register.
- `invoke_callback_4`: Library code that calls a routine through a pointer and returns a value.
- `call_offset_routine`: Library code that calls a routine at an address calculated by adding a negative offset to a global pointer.
- `call_offset_routine_alt`: Library code that calls a routine at an address calculated by adding a different negative offset to a global pointer.
- `load_pointer_and_branch`: Library code that loads a specific address into a register and branches to another location.
- `load_address_and_branch`: Library code that loads a specific address into a register and performs a short branch to another location.
- `call_indirect_offset`: Library code calls a routine at an address calculated by adding a negative offset to a pointer stored in a global variable.
- `call_indirect_offset_2`: Library code calls a routine at an address calculated by adding a different negative offset to a pointer stored in a global variable.
- `load_address_and_branch_2`: Library code loads a specific constant into a register and then performs a short branch.
- `load_pointer_and_branch_2`: Library code loads the address of a global variable into a register and then performs a short branch to another routine.
- `load_address_and_branch_3`: Library code loads another constant into a register and then performs a short branch.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: The library code executes a single illegal instruction.
- `libInit2`: The library code stores the value from A0 into a global variable.
- `MIExpunge`: This library code checks a flag in a structure, and if zero, manipulates a linked list of memory segments before calling a function pointer and returning a segment list.
- `Exp_InUse`: This library routine sets a specific bit in a byte located at an offset from a pointer held in an address register.
- `MIOpen`: This library code tests a bit in a structure and, if not set, increments a value in a structure before returning a pointer.
- `openFail`: This library routine returns a value representing a failure state.
- `MIClose`: The library code decrements a short counter stored in a structure and, if the counter reaches zero and a specific flag bit is set, calls another routine and returns its result.
- `DPFix`: The library code performs bitwise operations and a loop of shifts on a value to normalize it, potentially returning zero based on specific bit patterns and sign checks.
- `dfi4`: This library code tests a specific bit in a register and branches or clears another register based on the result.
- `dfi6`: This library code clears a long register and then performs an unconditional branch.
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
- `MIIEEEDPCmp`: Library code that performs a tail branch to a comparison routine.
- `MIIEEEDPTst`: Library code that performs a tail branch to a test routine.
- `MIIEEEDPAbs`: Library code that performs a tail branch to an absolute value routine.
- `MIIEEEDPNeg`: Library code that performs a tail branch to a negation routine.
- `MIIEEEDPAdd`: Library code that performs a tail branch to an addition routine.
- `MIIEEEDPSub`: Library code that performs a tail branch to a subtraction routine.
- `MIIEEEDPMul`: Library code that performs a tail branch to a multiplication routine.
- `MIIEEEDPDiv`: Library code that performs a tail branch to a division routine.
- `MIIEEEDPCeil`: Library code calculates a ceiling value by negating a value, applying a floor function, and then negating the result.
- `MIIEEEDPFloor`: Library code that calculates the floor of a value by comparing it to zero and performing addition or subtraction.
- `test_d6_d7`: This library code moves two registers into others, calls another routine, and then either branches or calls a third routine after restoring registers.
- `lt_zero`: This library routine stores a constant into a stack slot and then tail calls another routine.
- `fptrap`: This library routine processes floating-point exceptions or traps by performing bitwise operations and arithmetic checks on operand data.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `dfsub`: Library code that saves registers and prepares two pointers for a subsequent subtraction routine.
- `dfadd`: Performs addition of two double-precision floating-point numbers.
- `dfa11`: Calculates the difference between two double-precision floating-point numbers.
- `dfa14`: This library routine writes a 64-bit value to a memory location pointed to by a register.
- `dfa13`: Library code performs complex bitwise and arithmetic operations on multiple registers to calculate and store a result in a table.
- `dfa5`: Library code performs complex bitwise and arithmetic operations to calculate a value and stores the result in a table.
- `dfa8`: This library routine compares two longwords and performs a subtraction and bit shifts on registers based on the comparison result.
- `dfa7`: This library code tests a long register, performs bitwise operations on another register, stores the results into a table, and then branches to another routine.
- `dfa9`: Library code that clears a register and a memory location through a pointer, then performs a short branch.
- `dfauo`: Library code that compares a long word to a constant and, if it is less than that constant, stores a modified long word and a short word into a memory location pointed to by a register.
- `dfmul`: Performs multiplication on two double-precision floating-point numbers.
- `dfm4`: This library routine clears a long register and a long value at an offset from that register before performing a short branch.
- `dfmuo`: Library code that compares a long word to a constant and, if it is less than that constant, stores a modified long word and a specific long word into a memory location pointed to by a register.
- `dfdiv`: Performs double-precision floating-point division using a bit-shifting iterative algorithm.
- `dfd10`: Executes a long division algorithm on double-precision floating-point numbers.
- `dfd4`: This library routine performs arithmetic operations on values stored in registers and writes the resulting 64-bit value to a memory location pointed to by a register.
- `dfsetz`: This library routine clears two data registers and performs a short branch.
- `dfdbyz`: This library routine loads a 16-bit immediate value into a register and then performs a short branch to another location.
- `dfduo`: Library code that compares a long word to a constant and, if it is less than that constant, performs bitwise operations and updates the condition codes.
- `dfneg`: This library routine loads a word from a pointer, performs a bitwise AND, and conditionally flips a bit in the original memory location before branching.
- `dfcmp`: This library routine is empty.
- `dftst`: This library routine returns the 32-bit value that was passed on the stack.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### version.library

- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call_2`: Library code that decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `update_library_status`: This library routine retrieves a pointer from a register, accesses an offset within that pointer, and conditionally sets a bit in a byte at another offset.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `check_and_set_font_handle_2`: This library routine calls another routine via a table entry using a library base and conditionally updates a structure and calls another routine based on the result.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: This library routine attempts to open two libraries and, if successful, initializes a structure by setting a pointer and clearing a value.
- `traverse_and_unload_list`: Library code that iterates through a linked list, removes nodes based on a condition, and calls another routine to unload segments if a counter and a specific value reach zero.
- `modify_byte_and_return`: This library routine checks two conditions based on values at offsets from a register; if the conditions are met, it performs several function calls and manipulates a linked list, otherwise it sets a bit in a byte and returns zero.
- `set_bit_and_clear_register`: The routine sets a specific bit in a byte at an offset from a register and then clears the long register d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_conditional_jump`: The routine decrements a word at an offset from a register, then performs conditional branches based on the zero flag and a specific bit test before jumping to an offset from a register.
- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `update_font_character_metrics`: Iterates through a table of font character structures to identify the entry with the highest metric value.
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
- `increment_pointer_on_non_zero`: This library routine checks a byte at a pointer and increments that pointer if the byte is not zero.
- `execute_callback_routine`: This library routine acts as a wrapper that calls another routine.
- `calculate_string_length`: This library routine iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_memory_bytes`: This library routine compares bytes from two memory locations.
- `compare_byte_and_branch`: This library routine compares a byte from a pointer against a value in a register and branches based on the result.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_font_data`: This library routine loads a byte from a pointer and compares it against a value from the stack.
- `increment_pointer_and_branch`: This library routine increments a pointer by one and then performs an unconditional branch to another location.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations.
- `extract_font_resource_data`: Iterates through a linked list of font resource structures to copy character string data and associated metadata into a destination buffer.
- `populate_font_string_buffer`: Iterates through a linked list of string structures to copy character data into a destination buffer.
- `copy_string_to_buffer_3`: Copies a sequence of bytes from a source pointer to a destination buffer while updating a length counter.
- `parse_font_resource_stream`: Iterates through a data stream to identify specific font resource markers and extract character glyph data.
- `extract_font_glyph_data`: Iterates through a data stream to identify specific font resource markers and extract character glyph data.
- `parse_font_resource_block`: Iterates through a data stream to identify specific font resource markers and extract character glyph data.
- `read_and_parse_font_data`: Reads data from a disk file and parses it into a structured buffer.
- `calculate_absolute_value`: This library routine calculates the absolute value of a signed integer.
- `return_zero_constant`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 2 of 2).adf:bootblock

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
- `test_multiple_bits`: This routine calls another routine multiple times to perform bitwise AND operations on the result and tests a specific bit in a register.
- `execute_repeated_calls`: This routine calls another routine eighteen times in succession.
- `modify_memory_via_calls`: This routine calls another routine multiple times and performs a bitwise OR operation to update a byte in memory.
- `test_bit_after_call`: This routine calls another routine and then tests a specific bit in a register.
- `perform_conditional_stack_operations`: This routine calls another routine multiple times, performs bitwise XOR operations, and pushes bytes onto the stack from both a pointer and a memory offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_bits_and_push_data`: This routine tests bits in registers, moves a long word from a memory location pointed to by one register to a location pointed to by another, and pushes a constant value onto a stack pointed to by a register.
- `call_and_test_bit`: Calls another routine and then tests a specific bit in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_mask_byte`: Calls a subroutine and then masks the lower byte of a data register.
- `modify_memory_and_registers`: Calls a subroutine, masks a byte in a data register, and performs bitwise operations on a memory location and a data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_byte_to_stack`: Calls a subroutine, pushes a byte from a data register onto the stack, and clears that register.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_playback_context`: Allocates memory for audio data structures and initializes playback state flags and buffer pointers.
- `copy_data_register_d0_to_d5`: Copies the contents of data register zero to data register five.
- `cleanup_and_release_resources`: Iterates through a list of function pointers to execute them and then releases allocated memory blocks.
- `initialize_system_resources`: This routine attempts to open a library, initialize a trackdisk device, and register a disk resource handler.
- `return_immediately_2`: This routine performs no operations and returns to its caller immediately.
- `return_constant_value`: Returns a constant integer value.
- `return_address_in_register`: Returns the value held in a register to the caller.
- `execute_multiple_subroutines`: Saves several registers to the stack and performs a sequence of six subroutine calls before branching to another location.
- `process_conditional_loop`: Executes a loop that repeatedly calls a subroutine and checks a boolean result, performing various data manipulations and hardware register updates based on the outcome.
- `call_library_function_directly`: Calls one library function on a base pointer held in the state block, then calls a helper routine.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_nested_callbacks`: Sets a value to minus one in a record, calls a function to obtain a pointer, and if successful executes a sequence of four further function calls through that pointer and a global variable.
- `open_library_and_release`: Sets a result field to minus one, opens the dos.library name through the exec base, and when the open succeeds calls two methods on the returned base before closing it.
- `initialize_trackdisk_device_request`: Sets a device request record with a specific length and unit value, then calls a library function to open the trackdisk device.
- `store_two_library_results`: Calls two library functions and stores their results in the state record and a second record, one as a long and one as a byte.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function`: Calls a library function using a base address loaded from a global variable.
- `copy_and_configure_data`: This routine copies a word from one memory location to another and then populates several fields in a structure pointed to by a register.
- `populate_record_data`: Writes a constant and two values from a source record into a destination record.
- `populate_record_data_2`: Writes a constant, two values from another record, and two calculated values scaled by five hundred twelve into a target record.
- `load_and_branch_constant`: Loads a word into a register and performs a relative branch.
- `load_and_branch_small_constant`: Loads a small word into a register and performs a relative branch.
- `load_and_branch_2`: This routine loads a small constant into a register and performs an unconditional relative branch.
- `clear_and_branch_constant`: Clears a register, loads a word into another register, and performs a relative branch.
- `update_record_fields`: Writes several values into a record, including a constant, two values from another record, and two calculated values scaled by a power of two.
- `register_disk_resource_handler`: Obtains the disk.resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `execute_disk_resource_loop`: Repeatedly calls a function via a pointer in a record until it returns a non-zero value, then modifies hardware registers and a global variable.
- `invoke_disk_resource_routine`: Calls one function on the disk resource base held in the state block.
- `configure_hardware_and_jump`: Sets specific values in custom chip registers, loads two pointers into registers, calls a routine via a pointer, and branches to another location.
- `branch_to_target`: Performs a short branch to a specific instruction address.
- `update_flag_byte_bits`: Clears the bit selected by the first register value in a state byte, then sets bit two when the second register value is zero.
- `copy_state_byte_to_register`: Copies a state byte into a control word and writes a fixed flag value to a second control word.
- `start_disk_sync_transfer`: Calls a helper, waits for a countdown loop, then writes DMA control, disk pointer, and sync words to custom chip registers before calling two more helper routines.
- `set_top_bit_field`: Stores the incoming value with its top bit set into a 16-bit field of the record addressed by the second register, twice.
- `reset_record_and_call_library`: Sets a state word to a fixed value, sets a second word to two, and calls a library function on the base pointer.
- `count_consecutive_word_matches_2`: Calculates a value based on the number of times a specific word repeats consecutively within a memory range.
- `count_consecutive_values`: Iterates through a memory range to count how many times a specific word value repeats consecutively.
- `forward_to_helper_routine`: Calls one helper routine and returns with no other work.
- `compute_bitwise_checksum`: Computes a 16-bit checksum over a run of words by shifting each bit through the accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `return_without_action`: The body contains only a return, with no calls, memory access or arguments used.
- `check_library_status`: Decrements a value in a register, performs conditional branches based on bits in a memory location, and jumps to an offset from a base pointer.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `build_formatted_string_buffer`: Iterates through a linked list of string structures to concatenate their contents into a destination buffer.
- `concatenate_string_list`: Iterates through a linked list of strings, copying each null-terminated string into a destination buffer while updating the buffer pointer and remaining capacity.
- `copy_string_and_update_buffer`: Copies a null-terminated string from a source location to a destination buffer and adjusts the buffer pointer and length accordingly.
- `parse_font_string_and_render`: Parses a string to extract font information and populates a structured buffer.
- `parse_string_and_populate_struct`: Calculates the length of a null-terminated string and copies it into a destination buffer.
- `parse_and_copy_string_data`: Calculates the length of a null-terminated string and copies its contents into a destination buffer.
- `copy_string_to_buffer_2`: Calculates the length of a null-terminated string and copies its contents into a destination buffer.
- `call_library_functions`: Calls three different library functions using offsets from a base address stored in a global variable and returns the absolute value of a register.
- `return_zero_constant_2`: Returns a zero value without performing any other operations or accessing memory.
- `compute_font_offset_2`: Calculates a signed offset by finding the difference between two 16-bit values and adjusting the result based on bitwise differences between two byte arrays using a global lookup table.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_null_byte`: Checks a byte at a pointer and increments that pointer if the byte is not zero.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length_2`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_bytes_and_check_zero`: Compares a byte from one memory location with a byte from another, then checks if a byte at a third location is non-zero.
- `compare_and_set_byte`: Moves a byte from memory to a register, decrements a pointer, compares the byte to the value at the new pointer, and sets a register to a constant if the comparison fails.
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
- `return_immediately_6`: This routine performs no operations and returns to its caller.
- `return_immediately_7`: This routine is empty and performs no operations before returning to its caller.
- `return_immediately_5`: This routine is empty and performs no operations before returning to the caller.
- `modify_memory_values`: Clears the high bit of a byte and increments a word at specific offsets from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_buffer`: Allocates a memory block, populates a table with incrementing offsets based on a base address, and performs two function calls involving hardware parameters.
- `call_indirect_function`: Fetches a function pointer from a location calculated by adding an offset to a value in a register and calls it with a pointer and a constant value as arguments.
- `concatenate_strings`: Joins three source strings into a destination buffer, inserting a separator character if necessary.
- `execute_conditional_callback`: Checks a function pointer in a global structure and either calls it or calls another routine with the structure as an argument.
- `execute_conditional_callback_alt`: Checks a different function pointer in a global structure and either calls it or calls another routine with the structure as an argument.
- `execute_conditional_callback_third`: Checks a third function pointer in a global structure and either calls it or calls another routine with the structure as an argument.
- `execute_conditional_callback_fourth`: Checks a fourth function pointer in a global structure and either calls it or calls another routine with the structure as an argument.
- `traverse_and_copy_linked_list`: Iterates through a linked list of structures, performing conditional checks and executing callback functions.
- `reset_table_entries`: Clears a two-byte field in a table repeatedly and calls another routine each time, using a counter that runs from a positive start value down to zero.
- `decrement_and_clear_counter`: Decrements a short counter stored at an offset from a pointer and, if the counter reaches zero or was already invalid, clears the counter and, when a flag at the base of the structure is non-zero, calls another routine and invokes a function pointer stored in a table.
- `search_table_for_match`: Walks a table of entries using a fixed stride and compares an input value with a field in each entry, calling another routine when a match is found.
- `parse_custom_data_block`: Parses a structured data stream by checking for specific four-byte magic markers.
- `call_other_routine_and_return_zero`: Calls another routine and then returns zero.
- `execute_infinite_recursion_2`: Calls itself indefinitely without performing any other operations.
- `mask_lower_bytes`: Calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `execute_infinite_recursion`: The routine calls itself indefinitely, creating an infinite loop of recursive calls.
- `execute_conditional_branch`: Compares a value from a global structure with a register and calls another routine if they are not equal.
- `wait_for_flag_clear_2`: Polls a specific offset within a global structure until its value becomes zero.
- `clear_and_unlink_object`: If the provided pointer is non-zero, the routine clears a status flag and two other fields in a global structure, iterates through a 64-byte name field to call another routine for each non-null byte, and then calls that routine three more times.
- `reset_game_state_flag`: Clears a 16-bit value in a global variable and sets a flag byte in the same structure by OR-ing in a constant, then calls another routine.
- `copy_struct_to_message`: Calls another routine and, if certain conditions on the returned values and a pointer are met, copies a small fixed-size block from memory into a structure, updates several fields, and clears a byte in that structure.
- `clear_hardware_registers`: Clears a longword at an offset from a register and clears a byte at another offset from the same register.
- `clear_flag_and_iterate_table`: Checks a flag in a global variable, clears it, and then iterates through a table of integers, calling another routine for each non-zero entry until a counter reaches four.
- `process_data_table_3`: Iterates through a table of four entries, clearing specific fields in a destination structure and updating values based on comparisons with data from a source table.
- `increment_and_store_byte_2`: Increments a byte in a register and stores the result into a memory location at an offset from a pointer.
- `increment_pointers_and_branch`: The routine increments two pointers by four, increments a byte, and branches based on a comparison before restoring registers and returning.
- `copy_struct_fields_and_update_table`: The routine copies seven words from one area of a structure to another area of the same structure and then copies a word from one offset to another.
- `execute_recursive_pointer_calls`: Checks a pointer in a structure, calls that pointer, and then calls itself recursively multiple times.
- `process_linked_list_counters`: Iterates through a linked list of structures, decrementing internal counters and updating pointers.
- `decrement_and_check_counter_2`: Library code tests a longword at an offset, decrements it if non-zero, and performs conditional arithmetic based on the result.
- `traverse_linked_list_and_update_counters`: Iterates through a linked list of data structures, performing arithmetic on specific members and decrementing counter values.
- `process_linked_list_elements`: Iterates through a linked list of entries, decrementing counters and updating pointers.
- `update_conditional_flag`: Compares two long values and, if they are not equal, checks a short flag at an offset from a pointer.
- `clear_flag_after_callback`: The routine checks a short flag at an offset from a pointer and a global variable at another offset.
- `signal_on_flag`: Checks a global flag and, if set, calls another routine via a function pointer stored in a global variable.
- `process_sound_table`: Iterates a table of entries and invokes callbacks stored in each entry, setting bits in a mask based on their return values.
- `setup_sound_channel`: Writes a value derived from an input register to a global variable and then writes the constant 2 to a hardware register at an offset from the custom chip base, which corresponds to the left channel of audio channel 0.
- `search_and_update_slot_2`: The routine iterates through a table to find a matching entry based on specific byte criteria, updates a stored index, and then modifies a slot.
- `search_and_update_slot`: The routine iterates through a table to find a matching entry based on specific byte criteria, updates a stored index, and then modifies a slot with new data and a status byte.
- `branch_with_constant_2`: Loads a constant value into a register and performs a relative branch.
- `clear_state_and_wait`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_flag_byte`: Computes an address from a global variable and a register value, writes zero to a byte at that address, and if the following byte equals one, writes two to the first byte.
- `reset_and_poll_status`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero, returning the final value in a register.
- `clear_slot_and_notify`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot, calls a callback via an indirection, and clears the next byte and the slot byte.
- `calculate_scaled_value`: Stores a word into a memory location and performs arithmetic involving scaling and division to update a multi-part value in a structure.
- `store_word_and_branch`: The routine stores a word from a register into an offset of a pointer and then performs a relative branch.
- `update_scaled_value`: Calculates a scaled value based on a multiplier and a difference between two words, updates a field in a structure, and handles a bitwise clear on a byte if a condition is met.
- `clear_flag_and_call`: Checks a word from a stack offset and, if non-zero, clears a bit in a structure and calls another routine before clearing a different word in that structure.
- `increment_and_branch`: Adds a constant byte value to a data register and performs a short relative branch to another location.
- `decrement_and_check_counter`: Calls another routine and, if it returns a non-zero pointer, decrements a short integer located at an offset from that pointer.
- `update_sprite_animation_and_position`: Updates sprite animation frame indices, scaling factors, and timing counters using fixed-point arithmetic.
- `decrement_counter_and_branch`: Calls another routine, checks if the returned pointer is non-zero, decrements a short value located at an offset from that pointer, and calls another routine if the new value is zero.
- `copy_struct_fields_and_clear_flag`: Copies several fields from a structure pointed to by a1 into a destination pointed to by a2 when a condition on d7 is met, then returns the original value of one of the source fields after clearing that field in the source structure.
- `update_sprite_animation_state`: Updates sprite animation frame indices, scaling factors, and timing counters using fixed-point arithmetic.
- `update_sprite_transform_and_position`: Calculates new sprite coordinates, scaling factors, and animation frame offsets using fixed-point arithmetic and bit-shifting.
- `update_physics_and_scaling`: Calculates new position and velocity values using fixed-point arithmetic and bit-shifting.
- `compare_and_store_byte`: Compares a byte in a register to a constant and branches if they are not equal.
- `branch_on_byte_comparison`: Compares a byte to a constant and branches to another routine or a different location based on the result.
- `decrement_word_until_non_negative`: This routine tests a word at an offset from a pointer and decrements it until it is no longer negative or zero.
- `calculate_sample_offset_with_scaling`: Calculates a new sample position by adding a scaled value to an existing offset.
- `update_audio_playback_parameters`: Calculates sample playback positions, volume levels, and interpolation coefficients based on a playback rate.
- `subtract_if_not_less`: Compares two longwords and subtracts a third longword from the second if the first is not less than the second.
- `resample_audio_buffer_4`: Calculates new sample positions and increments based on a playback rate, then performs either direct byte copying or linear interpolation between samples.
- `resample_audio_buffer_3`: Calculates new sample positions and increments based on a playback rate, then performs either direct byte copying or linear interpolation between two source samples to fill a destination buffer.
- `resample_audio_buffer_2`: Performs audio sample rate conversion using either direct byte copying or linear interpolation between two source samples.
- `resample_audio_buffer`: Performs linear interpolation or fixed-point resampling to transform a source buffer into a destination buffer.
- `update_fixed_point_table`: Performs fixed-point arithmetic to update a value in a structure and then uses a loop to redistribute or resample bytes from a source buffer into a destination buffer.
- `update_sprite_reference_and_scale`: The routine iterates through a table of pointers, comparing a block of data from each entry against a reference block to manage reference counters in a structure.
- `branch_to_offset_5`: Performs a short relative branch to a different location in the code.
- `decrement_and_recurse_counter`: Reads a pointer from a structure, decrements a short value at a specific offset within that pointed-to structure, and calls itself recursively if the value reaches zero.
- `update_entity_physics_and_bounds`: Calculates new position and velocity for an object based on input parameters and updates its bounding box.
- `decrement_counter_and_call_3`: The routine reads a pointer from an object field, checks it is non-zero, then loads a short from an offset in the pointed-to structure, decrements it and writes it back.
- `compare_and_clear_d0`: Compares a word in a register to a constant and clears a register if the value is less than that constant.
- `update_audio_envelope_parameters`: Calculates and updates volume, pitch, and envelope timing values by performing table lookups and bitwise scaling.
- `calculate_audio_envelope_parameters`: Calculates volume, pitch, and envelope timing values based on table lookups and bitwise scaling.
- `compare_and_store_byte_2`: Compares a byte in a register to a constant and branches if they are not equal.
- `compare_and_branch_3`: Compares a byte in a register to a constant and branches if they are not equal.
- `decrement_memory_word`: Tests a word at an offset from a pointer and decrements it if it is not zero.
- `update_physics_and_collision_parameters`: Calculates new velocity and position values by performing scaled multiplications and bitwise shifts on data from multiple structures.
- `subtract_if_less`: Compares two longwords and branches if the first is less than the second.
- `calculate_complex_values`: Performs several arithmetic operations involving multiplications and bit shifts to update values in a structure.
- `release_device_and_library`: Checks a global pointer and, if non-zero, performs a sequence of operations including calling functions via pointers from a global variable, updating a structure field, and calling several other routines before clearing the global pointers.
- `execute_signal_sequence`: Calls a sequence of three different functions from a global table, passing an argument to one, and returns a value formed by masking an input register with the result of a table call.
- `store_task_and_flags`: Clears a word in a structure pointed to by a register and, if a flag in another register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the result, and copies the flag into the structure.
- `compare_strings_case_insensitive`: Compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `return_called_register`: Calls another routine and returns the value that was left in an address register.
- `allocate_memory_block`: Allocates a memory block of a specified size using either a custom allocator or a system library call, and if successful, stores a value in the first word of the allocated block.
- `free_memory_block`: Calls a function pointer to release a memory block if the provided pointer is non-zero.
- `call_and_return_address_register`: Calls another routine and then returns the value left in address register A0 by that call, without modifying it.
- `check_and_branch_on_result`: Calls another routine and tests its return value; if non-zero, it calls several other routines in sequence, conditionally invoking another routine based on a pointer and a bitmask check, before returning a 64-bit value.
- `initialize_object_structure`: Allocates and initializes a complex data structure by populating various offsets with specific constants, function pointers, and linked list elements.
- `call_other_routine_conditionally_2`: Checks a global variable and calls another routine multiple times if the value is non-zero.
- `call_subroutine_and_copy_struct`: Calls another routine and, if certain conditions involving a pointer and returned values are met, copies a fixed-size memory block to a message buffer.
- `parse_data_block_stream`: Calls a routine to parse a structured data stream by checking for specific four-byte magic markers.
- `initialize_sonix_driver`: Allocates and initializes a complex data structure and attempts to copy a string pointer from a list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_envelope_and_pitch`: Updates an audio envelope state machine by modifying pitch and volume variables based on a counter and a data table.
- `initialize_hunk_data`: Parses Amiga hunk data to populate a structure with specific values, a pointer to a global variable, and a function address.
- `trigger_crowd_audio_event`: Executes a sequence of function calls via a pointer table to update audio parameters and trigger a specific sound effect associated with the crowd stopping.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_match_statistics`: Clears several data tables and then iterates through a structured byte array to compare adjacent values.
- `process_data_table`: Iterates through a table of short integers to compare adjacent values and calls other routines based on the results before entering an infinite loop that writes a constant to memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_values`: Calls a function via a pointer, stores the result in a global variable, and then performs a conditional update of a short value using a byte from memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_update_pointer`: Compares a value at a memory location with a value retrieved from a table using an index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_modify_table`: Performs a complex traversal of a table using an index, updating values through function calls and incrementing the index until a limit is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sort_index_table`: Performs a comparison-based sort on a subset of an index table by iterating through a loop and calling another routine.
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

- `validate_multiple_pointers`: Calls a routine and stores its result in a structure, then performs a series of conditional checks on subsequent routine returns to populate various offsets in a structure or return a failure value.
- `frag_lea_movea_rts`: Short code fragment that loads the address of a location in the code into a5, then copies the memory at an offset from a5 to a4, then copies a fixed memory address to a6, then returns.
- `return_buffer_size_constant`: Takes no arguments and simply returns the fixed constant 576 (576).
- `return_address`: Returns the address of the routine itself.
- `call_multiple_subroutines`: Saves registers to the stack and executes a sequence of six subroutine calls before branching.
- `execute_loop_with_updates`: Executes a loop that performs several operations, including writing a constant to a structure offset and updating a local variable, and returns a value based on the loop's outcome.
- `invoke_indirect_function`: Fetches a function pointer from an offset within a base pointer held in a register and executes it.
- `call_library_vector_228_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_conditional_logic`: Sets a value to negative one, calls a routine to obtain a pointer, and if successful, performs several nested function calls before calling another routine via a global pointer.
- `open_dos_and_set_field`: Sets a field in a structure to negative one, opens dos.library, and if successful calls two methods on the library base before closing it.
- `open_trackdisk_device_request`: Fills a device request record with a unit value and a fixed length, then calls a library function to open the trackdisk.device.
- `store_dual_library_outputs`: Calls two library functions and stores the resulting longword and byte into two different destination records at specific offsets.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_offset_routine`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `store_data_structure`: Copies values between memory locations and stores a constant into a specific offset of a structure.
- `update_record_fields_2_2`: Writes a constant, two values from another record, and two calculated values scaled by a power of two into a target record.
- `initialize_record_data`: Sets a constant value and two specific fields in a target record using values from a source record.
- `set_d4_and_branch`: Stores a constant value into register d4 and branches to another location.
- `load_constant_and_branch_3`: Loads a word constant into a data register and performs a short unconditional branch to another location.
- `set_register_and_branch`: Loads a small constant into a data register and performs a short branch.
- `set_registers_and_branch`: Clears register d3, stores a constant in register d4, and branches to another location.
- `update_record_fields_2`: Writes a constant and two values from another record into a target record, followed by two calculated values scaled by a power of two.
- `register_disk_resource_handler_2`: Obtains the disk resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `process_disk_resource_loop`: Executes a loop that calls a function at an offset within a structure, updates a global variable, and modifies hardware registers.
- `invoke_disk_resource_routine_2`: Calls a function located at an offset within a structure pointed to by a register.
- `configure_hardware_and_call`: Modifies custom hardware registers and calls a routine using a pointer from a table.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `update_flag_byte_bits_2`: Clears the bit specified by the first register value in a byte at a memory location, then sets the second bit if the second register value is zero.
- `update_control_word_flag`: Copies a byte from an offset of a pointer into a global variable and then overwrites that same variable with a fixed bitmask.
- `start_disk_sync_transfer_2`: Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers.
- `set_top_bit_field_2`: Sets the most significant bit of a word value and stores the result twice into a 16-bit field within a record pointed to by a register.
- `set_state_and_call_library`: Stores a constant value into a memory location pointed to by a register, stores the value two into another location pointed to by that same register, and then executes a function via a pointer found in a library base.
- `count_word_matches`: Iterates through a memory range to find the longest sequence of a specific word value.
- `count_consecutive_word_matches`: Iterates through a memory range to find the longest sequence of a specific word value.
- `forward_to_checksum_helper`: Calls a routine to compute a 16-bit bitwise checksum and then returns.
- `compute_bitwise_checksum_2`: Computes a 16-bit checksum over a run of words by shifting each bit through an accumulator and applying an exclusive-OR with a feedback value when the most significant bit is set.
- `branch_on_bit`: Performs a conditional branch based on a single bit.
- `extend_and_store_word`: Extends a word to a long and stores it into a location relative to a frame pointer.
- `execute_supervisor_call`: This routine performs a series of operations and invokes a supervisor-level library call.
- `wait_for_flag_clear`: Repeatedly calls another routine while a specific global variable remains non-zero.
- `remove_object_from_table`: If the object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_state_flag`: Clears a 16-bit value in a global structure and sets a flag byte by performing a bitwise OR with a constant, then calls another routine.
- `clear_flag_byte_loop`: Clears a byte at an offset from a pointer, then iterates through a list of pointers, calling another routine for each non-zero entry until a counter reaches four.
- `reset_conditional_flag_byte`: Computes an address using a global variable, a register value, and a constant offset, then clears the byte at that address.
- `reset_and_poll_status_2`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero.
- `clear_slot_and_trigger_callback`: Calculates a memory address based on an input value and a base pointer, clears the byte at that location, and if the subsequent byte is non-zero, updates the slot, executes a function pointer stored at an offset, and clears both the subsequent and the slot bytes.
- `calculate_scaled_value_2`: Performs arithmetic operations involving scaling a value by a factor of two hundred and fifty-six and dividing a difference by a third value.
- `perform_cleanup_and_subroutines`: Stores a word to a location, branches, restores registers, loads a long from a table, and conditionally executes several subroutines before returning.
- `cleanup_device_and_library`: The routine checks a global pointer, and if non-zero, performs several operations including calling a function via a pointer from a global variable, updating a value in a structure, and calling several other routines before clearing global pointers.
- `execute_signal_sequence_2`: Calls a sequence of three functions from a global table, passing an argument to the second, and returns a value formed by masking an input register with the result of the second call.
- `update_task_status_and_result`: Clears a word in a structure pointed to by a register and, if a flag in another register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the return value, and copies the flag into the structure.
- `call_conditional_function`: If a register is non-zero, it calls a function through a pointer, which is either a global function pointer or a specific address stored in a global variable.
- `execute_conditional_calls`: Checks a global variable and, if it is non-zero, calls another routine four times.

### logged disk load from TV Sports Football (1988)(MirrorSoft)[cr QTX][a2](Disk 1 of 2).adf

- `process_string_and_bitmask`: Iterates through a data buffer to find specific character delimiters and performs bitwise operations to set bits in a target byte.
- `store_function_results`: Calls a subroutine via a pointer and stores the returned byte into two different memory locations using an offset derived from another byte.

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

- `check_and_call_handler`: Checks a 16-bit value at offset -2b40 of the structure in a4; if it is not zero, it decrements a 16-bit value at offset -a3 of a3, clears a byte at offset 0 of a0, and calls a routine at offset -7d9a using a pointer from offset -2864 of a4.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_two_counters`: Clears two 16-bit fields, one at offset -2b00 and one at offset -2afe, of the structure in a4, then returns.
- `initialize_structure_fields`: Calls a routine via a pointer in a structure in a4 using a constant and a value from the stack, then calls another routine three times, passing a constant and two values from the stack, and stores each returned 32-bit value into consecutive fields of the structure in a4.
- `call_routines_with_values`: Calls a routine at offset -32682, then calls a routine at offset -32154 three times, each time passing two 32-bit values read from the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_fields_2`: Clears two bytes at offset -11463 of a structure in a4, sets a 16-bit value at offset -11284 to 900 if a parameter is less than 2, sets a byte at offset -13845 to 10 if it is zero, sets a byte at offset -13844 to 13 if it is zero, and if two 16-bit values at offsets -13886 and -13882 are both zero, calls a routine using a pointer from a structure and returns the result minus 1.
- `rotate_byte_and_call_handler`: Sets up its stack frame, then checks a 16-bit value at offset -2b40 of the structure in a4 and jumps away when that value is zero.
- `calculate_and_call_offsets`: Performs three separate sequences where it calculates a value by subtracting one from a byte at an offset in a4 and multiplying it by a constant, then calls a routine at offset -32730 using these calculated values and other parameters from the structure in a4.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to -(a7).
- `adjust_stack_pointer`: Adds 18 to the stack pointer.
- `update_structure_via_call`: Decrements a byte at offset -3615 of the structure in a4, sets a byte at offset 0 of a0 to zero, sets d1 to zero, sets d0 to the decremented value, pushes two pointers, sets a byte at offset 48 of a0, calls a routine at offset -7fda, adjusts the stack pointer, clears a word on the stack, and performs a bitwise AND on a byte at offset 822 of a0.
- `write_byte_to_pointer`: Sets d1 to zero, adds 8 to d4, moves d1 to a0, sets d0 to zero and then sets its high bit, pushes a constant, and stores d0 at the location pointed to by a0.
- `tiny_clr_and`: Small routine of 2 instructions that clears -(a7), then masks (a0) with d3.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `perform_division`: Decrements a 16-bit value in d0, rotates a byte in d4 left by 4, and performs a signed division of d4 by d6.
- `transfer_and_mask_data`: Copies a 16-bit word from d0 to d1, performs a bitwise OR of d0 with a value at an offset of 17375 from a0, and performs a bitwise AND of d1 with a byte from a location pointed to by a2.
- `push_and_set_registers`: Pushes a 32-bit value onto the stack, sets d0 to zero, and sets d6 to 6.
- `initialize_registers_with_constants`: Pushes a 16-bit value onto the stack, sets d0 to zero, and sets d6 to 24, 30, and 36 sequentially.
- `call_subroutine_with_parameters`: Moves d1 to a0, sets d6 to 48, pushes two values onto the stack, calls a routine at an offset of 31965 from a4, sets d6 to 72, and clears the word at the top of the stack.
- `decrement_and_store_word`: Sets d0 to zero, sets d6 to 90, decrements d0, stores the value 6 at a location pointed to by a0, and sets d1 to zero.
- `call_routine_with_multiple_constants`: Moves d1 to a0, sets d6 to 114, 120, and 126, calls a routine at an offset of 31965 from a4, sets d6 to 138 and 144, sets d6 to 150, loads a byte from an offset of 13846 from a4 into d0, and decrements d0.
- `update_player_status`: Reads a byte from a table using an index in d0, compares it to a constant, and if equal, writes a new value to a byte at an offset within a structure pointed to by a6.
- `clear_byte_and_branch`: Clears a byte at an offset of 2 from a5 and branches to another location.
- `increment_byte_on_condition`: Checks two conditions involving values at offsets from a4 and a5, and if both are met, increments a byte at an offset of 2 from a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_compare_byte`: Increments a byte at an offset of 2 from a5 and compares it to 2.
- `increment_and_call_routine`: Increments a byte at offset negative 3 of the structure in a5, compares it to 18, and if the value is less than or equal to 18, calls another routine and pushes a constant onto the stack.
- `load_and_shift_byte`: Loads a byte from offset negative 2 of the structure in a5 into d0, shifts d0 left by 1 bit, and loads a pointer into a0 from offset negative 10046 of the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `not_available`: The routine cannot be analyzed because the pseudo-code is unavailable.
- `load_and_adjust_stack`: Loads a byte from offset negative 13846 of the structure in a4 into d0, sets d1 to 34, and adds 4 to the stack pointer a7.
- `modify_byte_via_pointer`: Loads a pointer into a0 from offset negative 13766 of the structure in a4 and performs a bitwise OR on a byte at the address pointed to by a6 with the value 236.
- `compare_and_clear_byte`: Loads a byte from offset negative 11465 of the structure in a4 into d0, compares it to a byte at offset negative 11464 of the structure in a4, and if they are not equal, branches to another routine; otherwise it clears a byte at offset negative 2 of the structure in a5.
- `load_byte_to_d0`: Loads a byte from offset negative 11457 of the structure in a4 into d0.
- `load_byte_to_d1`: Loads a byte from offset negative 2 of the structure in a5 into d1.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies d1 to d2.
- `load_pointer_to_a0`: Loads a pointer into a0 from offset negative 22374 of the structure in a4.
- `load_byte_to_d3`: Loads a byte from offset negative 2 of the structure in a5 into d3.
- `copy_and_shift_data`: Copies a 16-bit word from a memory location indexed by a0 and d0 to a destination indexed by a1 and d3, moves a 32-bit long value from a global variable to another location, and shifts d0 left by 4 bits.
- `call_shifted_byte_handler`: Loads a byte from a memory location offset by -2 from a5 into d1, shifts d1 left by 1 bit, calls another routine, and increments a7 by 2.
- `shift_and_increment_stack`: Loads a byte from a memory location offset by -2 from a5 into d2, shifts d2 left by 1 bit, and increments a7 by 2.
- `increment_and_branch_on_byte`: Copies a 16-bit word from a memory location indexed by a0 and d0 to a destination indexed by a1 and d2, increments a byte at a memory location offset by -2 from a5, and branches if that byte is less than 3.
- `tiny_unlk`: Small routine of 1 instruction that removes the stack frame held in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
