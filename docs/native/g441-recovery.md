# Game 441 (Tv Sports Football): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00441). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1849236 top-level bytes in 101 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 47 of 47 units identical (bytes and relocations), 100 raw-word lines |
| Functions named and described | pass, 1109 of 1109 named (59 author symbols), 1109 of 1109 described (276 by rule, 816 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6323 of 6346 (99.64%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18302 of 18366 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 174874 |
| code | 59454 |
| data | 1474570 |
| padding | 23602 |
| unused | 116736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf:fb | 3 | 9208 | 471 |
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
| TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
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
| logged disk load from TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf | 1 | 145 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 48 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 214 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: file KBlocks.1 (header 406205 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 2 of 2).adf: file KBlocks.0 (header 352150 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 2 of 2).adf: file KBlocks.1 (header 385002 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
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
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: 85 executed PCs whose RAM was overwritten before any snapshot match its bytes 75344-75856: recovered as a unit run at an address
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
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: blocks 895-896 (bytes 458240-459264) hold ASCII text
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: blocks 898-900 (bytes 459776-461312) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.50)
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: blocks 903-1234 (bytes 462336-632320) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.78)
- tv sports football (1988)(cinemaware)[cr qtx](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf: blocks 1236-1696 (bytes 632832-868864) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.91)
- tv sports football (1988)(cinemaware)[cr qtx](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 2 of 2).adf: blocks 15-710 (bytes 7680-364032) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.59)
- tv sports football (1988)(cinemaware)[cr qtx](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 2 of 2).adf: blocks 895-1654 (bytes 458240-847360) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)
- tv sports football (1988)(cinemaware)[cr qtx](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_disk_resource_and_set_hardware` (runtime segment an address): Repeatedly calls a function on a disk resource pointer until it returns a non-zero value, then writes values to custom hardware registers and a global variable. Kind: loader/disk-check loop. Pass path: The loop continues (iVar2 == 0) until the disk read/check succeeds (iVar2 != 0), at which point it breaks the loop and proceeds to the next stage (u27__sub_250A)..
- `configure_hardware_and_jump` (runtime segment an address): Sets interrupt enable and DMA control registers, then calls a routine via a pointer before branching to another location. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `copy_state_byte_to_register` (runtime segment an address): Copies a state byte into a control word and writes a fixed flag value to a second control word. Kind: disk hardware/track validation. Pass path: the branch following the call (e.g., an address in sub_2524).
- `start_disk_sync_transfer` (runtime segment an address): Calls a helper, waits for a countdown, then writes DMA control, disk pointer, and sync words to custom chip registers before calling two more helpers. Kind: disk hardware sync/track check. Pass path: execution continues to the next instruction (movea.l an address(a4), a0).
- `process_disk_resource_loop` (runtime segment an address): Repeatedly calls a function from a structure pointed to by a register, updates a global variable, and modifies a hardware register. Kind: looping disk resource/track validation loop. Pass path: the loop breaks (iVar2 != 0) and proceeds to u38__sub_0342, indicating a successful resource acquisition or track validation.
- `configure_hardware_and_call` (runtime segment an address): Writes values to custom chip registers, loads two pointers into registers, and calls another routine. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `update_control_word_flag` (runtime segment an address): Copies a byte from a memory location pointed to by a register into a global variable and then overwrites that variable with a fixed flag value. Kind: disk hardware/parameter validation. Pass path: the branch at an address (bpl.b an address) which is taken if d1 remains positive after the decrement.
- `start_disk_sync_transfer_2` (runtime segment an address): Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers. Kind: disk sync/track check. Pass path: execution continues to subsequent instructions (e.g., movea.l an address(a4), a0).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies a longword from one register to another and then performs a short branch.

### EndCLI

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with a header and a sequence of values.
- `transfer_register_and_branch`: Copies a longword from one data register to another and then performs a relative branch to another location.

### cd

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with a header and a sequence of incrementing values.
- `move_and_branch_2`: This routine moves a long value from one register to another and then performs an unconditional branch.

### Port-Handler

- `parse_amiga_hunk_data`: Iterates through a data structure containing hunk type codes to identify and process executable or relocation segments.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf:fb

- `initialize_and_call_library`: Fills a memory range with zeros and then calls several functions from a library based on a global pointer.
- `initialize_game_state_structure_2`: Resets multiple global status variables to zero and initializes hardware parameters.
- `execute_indirect_calls_3`: Calls another routine multiple times and performs an indirect call using a pointer retrieved from an offset.
- `execute_callback_chain_3`: Calls two functions via pointers, passes a value to a third function, and then calls itself twice.
- `dispatch_via_table`: Uses a value from a table to index into a switch statement that calls various functions with provided parameters.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `branch_via_table`: Compares a value in a register, potentially shifts it, and uses it to index into a table to perform a jump.
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
- `manipulate_data_structure`: Calls several subroutines to manipulate data at a specific memory location and returns a value from a global variable.
- `perform_data_operation`: Calls subroutines to manipulate data at a specific memory location and returns a value from a global variable.
- `process_parameterized_request`: Checks a condition, then calls a sequence of subroutines to manipulate data via a handle and returns a result.
- `parse_8svx_header`: Performs several operations involving string comparisons and memory writes to process data structures based on a provided pointer.
- `modify_registers_and_memory`: Increments a pointer in a register, negates a value at a memory location, performs a bitwise OR on a register, and decrements a register.
- `compare_byte_strings`: Compares two byte strings byte by byte using a loop and a helper routine.
- `compare_and_update_state`: Calls several functions to retrieve values and compares them, then updates a global variable and returns a value based on the comparison result.
- `test_status_bit`: Checks a specific bit in a memory location and branches based on the result.
- `branch_with_constant`: Loads a small constant into a register and branches to another location.
- `test_bit_and_branch_2`: The routine loads a pointer from a table, tests a specific bit in the memory location pointed to by an offset, and branches based on the result.
- `test_flag_bit`: Tests a bit in a memory location and branches to different locations depending on the bit state.
- `branch_with_zero`: Loads zero into a register and branches to another location.
- `check_bit_masks`: The routine tests two different bit masks against a word in memory and returns a value based on whether the bits match specific patterns.
- `check_bit_mask`: Tests a specific bit mask against a word in memory and branches based on the result.
- `branch_with_zero_alt`: Loads zero into a register and branches to a different location than the previous routine.
- `clear_memory_tables`: Performs multiple nested loops to zero out various segments of memory organized in specific patterns and dimensions.
- `test_structure_fields`: Tests two different word values at offsets from an address register and returns if certain conditions are met.
- `update_calculated_offsets_3`: The routine calculates memory addresses using various offsets and multipliers and adds a value from a structure to one or more of those locations.
- `update_offset_values`: Calculates three memory addresses using offsets and multipliers and adds a value to each.
- `update_calculated_offsets_2`: Adds a value from a memory location to three different memory locations calculated using offsets and multipliers.
- `accumulate_coordinate_offsets`: Clears a block of memory and then performs a series of additions, summing values from multiple source tables into a destination table using iterative loops.
- `call_library_function_2`: Calls a function via a pointer stored in a table and stores the result in a global variable.
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
- `process_coordinate_loop`: Iterates through a loop to call functions with incrementing values and updates a global variable.
- `clear_global_word`: Sets a word at a specific offset from a pointer to zero.
- `test_memory_word`: Tests a word at a specific offset from a pointer and branches if the value is zero.
- `sync_state_structures`: Synchronizes two sets of three-word data structures based on the results of bitwise XOR operations and conditional checks.
- `clear_and_copy_words`: Clears three words in a structure and, if a fourth word is non-zero, copies three words from another location in the structure.
- `clear_word_values`: Clears three specific word values in a structure using offsets from a register and then performs a relative branch.
- `test_and_update_words`: Tests a word in a structure and, if non-zero, calls three different routines to update three words in the structure based on the returned values.
- `update_word_values`: Clears three word values, tests a fourth value, and if it is zero, copies three word values from one location in a structure to another before branching.
- `update_field_data`: Clears several word values and then updates a set of destination words based on values from a source table and conditional tests.
- `clear_and_test_fields`: Clears three word values at specific offsets from an address register and branches based on a word test.
- `clear_memory_and_branch_2`: The routine calculates an offset, tests a word in memory, and if certain conditions are met, writes three specific words to different memory locations before branching.
- `clear_and_branch_3`: Writes zero values to three memory locations and then performs a relative branch.
- `clear_memory_and_branch`: Clears three words at specific offsets from a pointer and then performs a relative branch.
- `convert_integer_to_digits`: Breaks an integer into individual decimal digits and stores them in a local array.
- `process_pointer_table`: Iterates through a table to update values and calls another routine using pointers from the table.
- `iterate_and_call_table_pointers`: Decrements a counter in a loop and calls a function via a pointer if a value in a table is not equal to thirty-two.
- `iterate_and_call_function`: Decrements a counter in a loop and calls a function via a pointer if a specific value in a table is not equal to thirty-two.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `call_indirect_functions`: Calls two different functions using pointers stored in a table, passing various parameters including sums of input values.
- `initialize_stack_and_branch`: Sets up a stack frame and performs a short relative branch.
- `copy_memory_block`: Copies bytes from one memory location to another by incrementing an index until a limit is reached.
- `calculate_accumulated_value`: Performs a loop to combine a global variable with other values and stores the result in a global location.
- `update_complex_accumulator`: Uses a loop and bitwise operations to add a combination of global and local values to a stored integer.
- `increment_table_entry`: Calls a function, uses the result to calculate an offset into a table, and increments the word at that location.
- `update_accumulated_sum`: Calculates a sum using two values from memory and a global variable, then stores the result back into one of those memory locations after adding multiple instances of the global variable to it.
- `copy_and_reset_data`: Copies a longword from one location to another and then sets the original location to all ones.
- `copy_value_via_pointer`: Calls another routine and uses its return value as a base pointer to store a word at a specific offset.
- `copy_string_and_wait_for_input`: Copies a string from a provided pointer into a local buffer until a colon or null terminator is reached.
- `process_input_stream_loop`: Reads input data into a buffer and compares the resulting values against specific constants.
- `call_and_copy_data`: Calls a function via a pointer and then calls another routine to copy data.
- `process_disk_error`: Checks for a specific return value from a routine and enters a loop that performs conditional calls and updates a global variable.
- `execute_conditional_logic`: Performs a series of routine calls and updates a global variable based on values retrieved from a table.
- `process_linked_list_nodes`: Iterates through a linked list structure, performing recursive calls and updating values within the list nodes.
- `decrement_and_branch`: Subtracts a constant from a register and branches based on the result or performs a further subtraction and branch.
- `branch_to_address`: Performs a relative branch to another location.
- `branch_to_address_2`: Performs a relative branch to a specific address.
- `execute_state_updates_and_input_checks`: This routine calls subroutines to update game state and graphics, copy a data block, update display parameters, and check button status.
- `update_game_state_and_graphics`: Copies a block of data from a source table to a local stack buffer and performs a series of state updates by calling various subroutines with specific parameters.
- `copy_data_block`: If a specific global flag is set, it clears that flag and copies five words from one memory location to another.
- `update_display_parameters`: Calls several functions to retrieve a value, calculate a new offset, and perform multiple writes to a table using that offset and provided parameters.
- `check_button_status`: Checks if two specific bits are set in a global variable and returns one if both are active, otherwise returns zero.
- `pass_parameters_to_subroutine`: Passes multiple arguments through to another routine.
- `calculate_clipping_bounds`: Calculates intersection boundaries and bitmasking parameters for a rectangular area.
- `compare_stack_values`: Compares a word from an offset of a pointer with a word from another pointer and branches based on the result.
- `compare_stack_word`: Compares a word from a stack offset to another word and branches based on the comparison result.
- `mask_and_store_word`: Performs a bitwise AND on a word from a stack offset and stores the result back to a different stack offset.
- `calculate_sprite_clipping_rectangles`: Calculates the adjusted source and destination coordinates and dimensions for a rectangular area by clipping it against screen boundaries.
- `calculate_blitter_clipping_parameters_2`: Adjusts source and destination memory addresses and dimensions by clipping a requested rectangular area against screen boundaries.
- `calculate_blitter_clipping_parameters`: Calculates the adjusted source and destination memory addresses and dimensions for a blitter operation by clipping the requested area against specified boundaries.
- `calculate_blitter_area_offsets`: Calculates source and destination memory addresses and dimensions for a blitter operation, accounting for clipping against a boundary.
- `setup_blitter_area_parameters`: Calculates and stores blitter control parameters, including source and destination addresses, area dimensions, and modification modes.
- `extract_and_test_table_value`: Performs bitwise operations on a register to calculate an offset into a table, then loads a value from that table into a memory location.
- `configure_blitter_parameters`: Manipulates various hardware registers related to the blitter and performs calculations involving a loop and table lookups.
- `branch_to_subroutine`: Performs a short branch to a location within another routine.
- `execute_structure_callbacks`: Calls several subroutines and resets a global pointer.
- `execute_callback_list`: Executes a sequence of function pointers stored in a table and resets a global pointer.
- `initialize_game_state_structure`: Populates a structure by calling a sequence of initialization and clamping routines, then performs multiple conditional lookups to store specific values into a table.
- `process_graphics_event`: Performs a series of function calls involving parameters and a global pointer, potentially updating a value if a condition is met.
- `execute_conditional_updates`: Checks a global flag and a returned value before performing a sequence of function calls that involve passing parameters and a pointer.
- `load_and_parse_resource_file`: Opens a file, validates a magic number header, and reads structured data into a provided buffer.
- `parse_and_allocate_bitstream_data`: Extracts structured data from a bitstream by performing bitwise shifts and masking to retrieve values.
- `expand_and_populate_table`: Decompresses a bitstream to extract values and populates a destination table with either newly allocated memory or specific data values.
- `process_compressed_data_stream`: Iterates through a data stream to expand compressed blocks by reading values and storing them into a destination table.
- `populate_data_table`: Loops through a table to store values retrieved from a lookup table based on nibble extraction.
- `populate_indexed_table`: Iterates through a loop to store values into a table using a base pointer and an incrementing index.
- `populate_indexed_data_table`: Iterates through a source table to fetch values based on an incrementing index and stores them into a destination table.
- `decrement_and_branch_on_zero`: Decrements a register multiple times, performing a conditional branch to another routine if the register reaches zero at specific intervals.
- `increment_and_store_table_entry`: Increments a word-sized index stored in a local offset, calculates a target address using that index and a base pointer, and stores a longword from a pointer at a fixed offset into that target address.
- `call_subroutine_with_stack`: Pushes a value from the stack, calls another routine, and then performs a relative branch.
- `iterate_and_call_table_entries`: Resets a word-sized value to zero and iterates through a table of up to eight pointers, calling a routine for each non-zero pointer found.
- `expand_compressed_data_blocks`: Decompresses a stream of data by reading bytes and repeating them a specified number of times.
- `loop_and_branch`: Performs a loop using a decrementing counter and then executes a short relative branch.
- `copy_nested_data_blocks`: Performs a multi-level nested loop to copy bytes from a source into a destination, using specific byte values to determine copy lengths and offsets.
- `execute_loop_and_branch`: Decrements a counter register and branches to a location that creates a loop.
- `return_stack_value`: This routine is empty and simply returns the value currently held at the top of the stack.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_conditional_memory_updates`: Increments a global counter and conditionally calls various routines to copy data to different memory locations based on the state of several short-integer flags.
- `update_conditional_state`: Checks a value from a library call and, if not negative one, populates a structure with several values, including a pointer to a global variable and a function pointer.
- `reset_global_pointer`: Calls a function with a constant and a pointer, then calls another routine and sets a global variable to a maximum value.
- `initialize_gameport_input_state`: Allocates and initializes a structure to manage input device states by querying the gameport device.
- `execute_indirect_calls_2`: Calls a routine with several arguments and then executes two consecutive indirect function calls via a pointer located at an offset from a register.
- `clear_and_refresh`: Sets a word in a global location to zero and calls another routine twice.
- `remove_linked_value`: Traverses a linked list to find a specific value and removes that node by updating the preceding pointer.
- `swap_linked_list_nodes`: Traverses a linked list and swaps the memory locations of two nodes based on a comparison of values.
- `traverse_and_swap_linked_list`: Iterates through a linked list and performs a swap of two memory locations using a pointer from the list structure.
- `swap_two_values`: Copies the value from one memory location to another and then attempts to assign a pointer to the original location.
- `configure_blitter_graphics_attributes`: Calculates and loads blitter control registers and source/destination addresses based on values retrieved from a data structure.
- `configure_blitter_sprite_attributes`: Calculates and loads blitter control registers and source/destination addresses based on values retrieved from a data structure.
- `update_blitter_registers_2`: Updates the blitter address and control registers using values retrieved from a pointer and a data structure.
- `update_blitter_registers`: Updates the blitter control and address registers using values retrieved from a pointer and a data structure.
- `configure_blitter_parameters_2`: Calculates values based on data from a pointer and updates various blitter hardware registers and a global variable.
- `configure_blitter_registers`: Loads a value into the blitter address register and, if a condition is met, updates the blitter mask register and sets a bit in a status register.
- `update_blt_registers`: Updates various Blitter hardware registers and modifies a global variable based on values from a pointer.
- `compare_table_values_3`: Compares values derived from two different tables by adding two words and subtracting one, then checking against a third value.
- `compare_table_values_2`: Calculates a value by adding two words from a table and subtracting one, then compares it against a word from another table.
- `compare_table_values`: Adds two words from a table, subtracts one, and compares the result against a value from another table.
- `compare_calculated_values`: Calculates a value by adding two words from a table and subtracting one, then compares this result against a value from another table.
- `move_and_branch_3`: Moves two longwords into memory locations and then performs a short branch.
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
- `compare_string_values`: Iterates through a string, converting characters and comparing them, then returns the difference between two converted values.
- `subtract_converted_values`: Calls a routine to convert two values and returns the difference between them.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_immediately_3`: The routine is empty and immediately returns to its caller.
- `allocate_structure_via_pointer`: Calls another routine using two arguments passed via the stack and stores the returned value into a location on the stack before conditionally branching.
- `allocate_and_initialize_structure_2`: Fetches a value from a pointer, increments it, and attempts to allocate memory for a new structure.
- `modify_linked_list_node`: Swaps two nodes in a linked list and writes several multi-byte values into a specific memory location.
- `traverse_linked_list_3`: Iterates through a linked list of pointers, comparing strings at each node until a match is found or the end of the list is reached.
- `swap_pointer_values`: Loads a pointer from a stack offset, moves the value at that pointer to a new location, and branches.
- `call_subroutine_via_stack`: Pushes a value from a stack offset onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `clear_structure_pointers`: Checks two pointers within a structure, calls a routine to process one if it is not null, and then clears both pointers.
- `process_linked_structure`: Calls two routines to manipulate a structure and then uses function pointers to update values within that structure.
- `iterate_linked_structure`: Traverses a linked list by following pointers and calls another routine for each element in the list until a null pointer is reached.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `clamp_and_store_result`: Calls a library function to retrieve a value, clamps that value between a minimum and maximum bound, and stores the result through a pointer obtained from the library call.
- `process_table_data`: Iterates through a table of words, performing a bitwise rotation and addition on each element.
- `recalculate_value_with_modulo`: Obtains a value from another routine and performs a calculation involving division and modulo using a part of a provided parameter.
- `update_table_index`: Decrements a value in a table and uses it to index another part of the same table to perform an addition.
- `allocate_and_initialize_node_tree`: Allocates memory for a new node and recursively initializes its properties, including bitmasking a value based on a byte offset and calculating coordinate offsets using values from global tables.
- `traverse_linked_list`: Recursively traverses a linked list structure, calling a routine on each node and its children, and performing memory deallocation on specific elements.
- `search_value_list`: Searches through a list of word pairs for a specific descending value and stores a pointer to the matching element in a destination table.
- `search_conditional_table`: Searches a table for a value based on a parameter, potentially performing a second search in a different table if a specific bit is set in a byte within a structure.
- `execute_blitter_loop`: Iterates through a list of pointers to perform blitter operations and update a structure with values from a register.
- `update_table_values`: Calls another routine, then updates two values at specific offsets within a structure based on a value retrieved from a pointer.
- `update_table_values_2`: Iterates through a list of pointers, updating a structure with values from a register and a pointer-dereferenced value for each entry in a loop.
- `open_file_handle`: Attempts to open a file using provided parameters, stores the resulting handle in a global variable, and calls another routine if the handle is null.
- `handle_file_error`: Checks if a global file handle is non-zero and if a parameter is non-zero, then attempts to perform an operation and closes the file handle if an error occurs.
- `close_file_handle`: Checks if a global file handle is non-zero, closes it if so, and resets the handle to zero.
- `load_and_allocate_data_structures`: Allocates a memory block based on the sum of two input values and populates it by iterating through two separate lists of data.
- `extend_byte_to_long`: Loads a byte from a stack offset, extends it to a long integer, and branches to another routine.
- `process_linked_list_nodes_2`: Recursively traverses a linked list, calling other routines to manage nodes, and updates several fields within a structure based on values from a source.
- `pass_pointer_to_function`: Retrieves a pointer from a global structure and an integer from a register to pass them as arguments to another routine.
- `recurse_unconditionally`: The routine immediately calls itself, creating an infinite recursion loop.
- `call_extend_byte_to_long`: Calls a routine that extends a byte from a stack offset into a long integer and branches to another routine.
- `set_global_flag_via_pointer`: Sets a single byte in a global memory location to one using a pointer retrieved from a data table.
- `iterate_and_verify_table_2`: Decrements a word in a table and clears a memory location based on the resulting value.
- `iterate_and_verify_table`: Increments a value in a table within a loop and returns a boolean result based on whether specific conditions are met after the loop.
- `compare_and_branch`: Compares a word from a table against a value in a register and checks a second word, branching if they do not match or if a specific bit is set.
- `clear_d0_and_branch`: Clears the d0 register and branches to another location.
- `traverse_and_update_linked_list`: Iterates through a linked list, calling other routines based on a bit test in each node, and performs arithmetic additions on several fields within each node.
- `push_calculated_pointers_and_call`: Calculates two memory addresses by adding offsets to values found in two different structures, pushes these addresses and a third pointer onto the stack, calls another routine, and then restores the stack pointer before branching.
- `update_list_and_values`: Iterates through a linked list while performing arithmetic additions on several fields within each node and calling other routines based on specific conditions.
- `traverse_linked_list_2`: Iterates through a linked list, updating several integer values within each node and calling another routine until a null pointer is reached.
- `get_register_value`: Calls a routine and returns a word from a specific memory location.
- `increment_counter`: Increments a value in a global variable if it is not zero.
- `decrement_counter`: Decrements a global variable and resets it to zero if it becomes negative.
- `calculate_and_draw`: Calculates two short values based on input parameters and a table, then calls a routine to perform a drawing operation using those values.
- `allocate_and_insert_sorted_node`: Allocates a new memory block and populates it with data from provided parameters and a source structure.
- `insert_into_sorted_list`: Copies data from a structure into a new node and inserts that node into a linked list in a sorted position based on a word comparison.
- `swap_pointer_values_2`: Moves a longword from a location pointed to by one register to a location pointed to by another, then stores that second location into the first.
- `store_pointer_to_memory_2`: Stores a pointer from a register into a location specified by another register, then stores that same pointer into a location pointed to by a global variable.
- `store_pointer_to_memory`: Loads a pointer from a structure and stores the value of a register into that memory location.
- `validate_and_return_pointer`: Compares a byte from the stack with a value in a register and checks if a longword in a structure matches another value, returning a pointer based on these checks.
- `compare_and_return_pointer`: Compares a byte in a structure with a register and returns a pointer if they are not equal.
- `remove_list_element`: Traverses a linked list to find and remove a specific node by matching two criteria, then frees the memory of that node.
- `traverse_and_free_list`: Recursively traverses a linked list and calls a routine to free each node.
- `parse_resource_file`: Calls another routine to load a resource, then checks if the returned value matches a specific constant.
- `set_d0_to_one`: Sets the d0 register to one and branches to another location.
- `clear_d0_and_branch_2`: Clears the d0 register and performs a short branch to another routine.
- `process_data_table`: Iterates through a table of eight entries, performing a calculation with values from the table and calling another routine before clearing the entries.
- `update_node_value_and_pointer`: Retrieves a value from a linked list node via a subroutine call, increments it, and stores the result back into the node structure.
- `reorganize_linked_list_node`: Retrieves a value from a node, increments it, and updates a pointer within a structure.
- `update_list_node_status`: Swaps two nodes in a linked list, sets a word in a node to one, and then calls another routine recursively.
- `traverse_linked_list_background`: Traverses a linked list of pointers while performing string comparisons at each node.
- `transfer_pointer_and_branch`: Loads a longword from a location relative to a base pointer into a register, stores that value into another location relative to the base, and branches to another routine.
- `push_pointer_and_call_subroutine`: Pushes a pointer from a memory offset onto the stack, calls a subroutine, adjusts the stack pointer, and branches to another location.
- `modify_bit_and_test_pointer`: Clears a specific bit in a byte at an offset from a pointer, then tests a longword at a different offset from that same pointer.
- `check_nested_pointer`: Loads two pointers from a structure and branches if a longword at a specific offset is zero.
- `update_blitter_data`: Calls a routine to retrieve data from a nested structure and stores the result into a specific memory location.
- `clear_memory_location`: Clears a long word at a location specified by a pointer and returns the original value in a register.
- `test_pointer_and_branch`: Tests a long word at an offset from a pointer and branches if it is zero.
- `execute_conditional_callback_4`: Pushes two pointers onto the stack, calls a subroutine, and then stores the result in a memory location.
- `execute_conditional_callback_3`: Pushes two longwords onto the stack, calls a subroutine, and then conditionally calls another routine based on a test of a word stored at an offset from a pointer.
- `copy_and_conditional_call`: Copies a longword from a location relative to one pointer to a location relative to another, then tests a bit in the second pointer's data.
- `execute_callback_and_clear`: Checks if a long word at an offset from a pointer is non-zero, calls a routine using that value, and then clears the original value.
- `process_object_callback`: Checks if a parameter is non-zero, then executes a function pointer at a specific offset from a base address, conditionally executes another function pointer using a value from an offset within the parameter, and finally returns the result of a third function call using the parameter and a constant.
- `clear_pointer_after_call`: Checks if a value at a specific offset in a structure is non-zero, executes the function pointed to by that value, and then sets that offset to zero.
- `execute_callback_chain_2`: Calls a routine with a parameter, then executes two function pointers stored within a structure using values derived from a third function call.
- `traverse_callback_list`: Iterates through a linked list of pointers and calls a routine for each entry until a null pointer is reached.
- `initialize_animation_resources`: Attempts to allocate and initialize animation bitmaps, rastports, and views by recursively processing a tree of data structures.
- `swap_and_execute_pointers`: Swaps two pairs of longwords within a structure and then executes a series of function pointers retrieved from a list until a terminator is reached.
- `execute_library_calls`: Calls three different routines using pointers retrieved from global variables.
- `decrement_and_recurse`: Decrements a shortword at a global location and calls itself recursively if the value reaches zero.
- `traverse_and_clear_list`: Recursively traverses a linked list, calling a routine on each node, performing specific memory writes, and clearing several pointers and values in a structure.
- `process_linked_list_element`: Checks a pointer, calls another routine if the pointer is non-zero, clears a specific bit in the value pointed to by the pointer, and updates the pointer to the next element in a list.
- `process_pointer_list`: Loads a pointer from a stack offset, clears a bit in the value pointed to by that pointer, and then updates the pointer to the next element in a list.
- `execute_and_unlk_a5`: Calls another routine, tests a word in a register, branches conditionally, and then unlinks a5 before returning.
- `reset_and_iterate_state`: Clears several memory locations, executes a loop that calls other routines and updates structure fields, and performs final cleanup by calling other routines.
- `execute_and_clear_pointer`: Pushes a value from a structure onto the stack, calls a routine, clears a value in the structure, and calls another routine before branching.
- `process_linked_data_stream_2`: Iterates through a linked list to update a structure by accumulating values and modifying status flags based on specific byte identifiers within the list nodes.
- `process_linked_data_stream`: Iterates through a linked list to update values in a structure based on data within the list nodes, including conditional logic for specific flag values.
- `update_structure_from_list`: Traverses a linked list to copy bytes and words from the list nodes into a structure, while also updating specific status flags.
- `initialize_structure_from_input`: Retrieves a value from a pointer, performs conditional checks and library calls, and populates various fields within a structure based on the results.
- `reset_structure_fields_3`: Checks a short value at an offset from a pointer, and if non-zero, writes a constant to a structure field and updates another field with a calculated value before calling another routine twice.
- `allocate_and_initialize_array`: Allocates a memory block via a library call and then populates an array within that block by calling another routine for each element.
- `execute_indirect_calls`: Retrieves two function pointers from a table using an offset from a base address and executes them with provided arguments.
- `initialize_memory_structure`: Clears several fields in a structure, then sequentially calls other routines to allocate memory, initialize fields, and perform operations using function pointers stored in a global variable.
- `reset_structure_and_call_routines`: Conditionally calls other routines using pointers from a structure and then clears multiple fields within that structure to zero.
- `update_state_from_table`: Performs a series of checks and updates a structure by comparing values from a table and copying data from a source address to a destination buffer.
- `search_and_allocate_buffer`: Calls a routine to retrieve a value and then conditionally calls a function pointer to allocate a buffer, populating it with a result from another routine.
- `execute_recursive_call`: This routine performs a direct recursive call to itself using the provided parameters before returning.
- `seek_file_position`: Calculates a position based on a table and updates a value in a structure, or calls a seek routine if certain conditions are met.
- `transfer_data_in_chunks`: Calculates a data length by comparing values in a table, then transfers data in fixed-size chunks by calling a routine repeatedly.
- `call_data_transfer_routine`: Calls a routine that transfers data in fixed-size chunks after calculating a length from a table.
- `branch_on_structure_flags`: Tests two word values at specific offsets from a pointer and branches to different locations based on the results, potentially calling another routine.
- `load_longword_and_branch`: Loads a longword from an offset of a base pointer into a data register and performs a short branch to another location.
- `close_file_handle_2`: Calls a routine if certain conditions are met, or calls another routine with a specific value if a pointer is non-zero.
- `set_library_parameters`: Writes specific values to a structure and then calls a routine using a pointer from a global variable.
- `reset_library_parameters`: If a condition is met, it writes a value to a structure, calls a routine, then writes different values to the same structure and calls the routine again.
- `copy_and_uppercase_string`: Copies up to four bytes from a source string to a local buffer while converting lowercase ASCII characters to uppercase.
- `search_and_execute_callback`: Searches through a list of structures to find a match for a specific value, then executes a routine with parameters derived from the matched structure.
- `iterate_and_update_state`: Iterates up to eight times, calling a routine to update a structure with new values based on a starting address and a step size.
- `validate_and_update_state`: Compares two short values and a long value against provided parameters, updating a structure with new values and calling another routine if certain conditions are met.
- `call_search_routine`: Calls another routine passing a value and a length derived from a global variable.
- `search_integer_array`: Iterates backward through an array of long values to check if a specific value exists within the specified count.
- `parse_format_string_2`: Stores a pointer into a global variable and calls another routine with two parameters.
- `increment_and_store_byte`: Increments a global counter and stores a byte from a register into a location pointed to by a global pointer.
- `call_other_routine_2`: Calls another routine with a single argument.
- `call_recursive_routine`: A routine consisting of 48 instructions that calls another routine and returns.
- `parse_format_string`: Parses a format string to extract and convert numeric, string, or floating-point values into a destination buffer.
- `write_short_value`: Stores a word value of eight into a memory location relative to a pointer and then branches to another location.
- `write_short_value_alt`: Stores a word value of sixteen into a memory location relative to a pointer and then branches to another location.
- `parse_formatted_string`: Parses a string containing numeric values and format specifiers, incrementing a counter based on character matches.
- `format_floating_point_string`: Converts a numeric value into a formatted string representation, handling sign characters, decimal points, and scientific notation.
- `loop_and_call_routine`: Repeatedly calls a routine and updates a value in a structure until a condition is met.
- `modify_stack_values`: Compares a word at a stack offset to a constant and, if certain conditions are met, performs arithmetic and updates stack values.
- `compare_and_modify_register`: Compares a value at a stack offset with a constant, and if not equal, performs arithmetic operations on a value at a stack offset and moves a constant into a register.
- `format_float_to_string`: Converts a floating-point value into a decimal string representation, handling sign, decimal points, and scientific notation.
- `clear_and_branch_4`: Clears a word at a negative offset from a register and branches to another location.
- `increment_and_store_word`: Loads a word from a memory location, increments it, stores the result back to a different offset relative to a pointer, and branches to another location.
- `format_numeric_string_2`: Converts a numeric value into a string representation, handling signs, decimal points, and integer parts by writing characters to a destination.
- `format_numeric_string`: Converts a numeric value into a string representation, handling signs, decimal points, and integer parts by writing characters to a destination buffer.
- `append_character_to_string`: Checks a word value and branches; if a condition is met, it writes a specific character to a destination pointed to by a register, or writes a hyphen if a signed value is negative.
- `convert_integer_to_string`: converts a 16-bit integer value into a null-terminated ASCII string representation.
- `execute_callback_via_offset`: Checks a global flag and, if zero, executes a series of library-style calls before invoking a function via a pointer and an offset.
- `dispatch_callback_via_structure`: Checks a global flag and, if zero, initializes a structure and executes a sequence of function pointers.
- `initialize_and_execute_callback`: Checks a global flag to determine if a specific routine needs to be initialized and stored.
- `update_callback_state`: Checks a status flag in a structure and, if zero, initializes a new state by calling a function through a pointer.
- `dispatch_callback_via_structure_2`: Checks a global flag and, if not set, initializes a structure and calls a routine to populate it.
- `initialize_and_execute_callback_2`: Checks a global flag to determine if a specific value needs to be initialized.
- `dispatch_callback_via_structure_3`: Checks a status flag in a structure and, if zero, initializes a new structure by calling a routine to populate it.
- `execute_callback_via_structure_2`: Checks a global flag and executes a routine through a pointer using an offset derived from a structure.
- `update_callback_state_2`: Checks a status flag in a structure and, if zero, initializes a new state by calling a routine and updating a global variable.
- `execute_callback_via_structure`: Checks a global flag, potentially calls a function through a pointer, and eventually executes a routine using an offset derived from a structure.
- `check_byte_and_increment`: Compares a byte at a pointer to specific constants and either branches or increments the value at that pointer.
- `check_character_and_branch`: Compares a byte at a pointer to a constant and branches based on the result.
- `skip_character_and_branch`: Compares a byte at a pointer to a constant and branches based on the result, potentially incrementing a pointer and performing a branch.
- `convert_string_to_integer`: Parses a string of characters into a signed integer value.
- `empty_routine`: This routine performs no observable work and ends immediately.
- `load_pointer_save_registers_branch`: Loads a longword from the stack into a register, saves several word-sized registers to the stack, and performs a short branch.
- `fill_memory_with_byte`: Fills a memory range with a specific byte by iterating through a pointer and decrementing a counter.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `compare_and_decrement_pointer`: Compares a byte from a memory location to a value in a register and, if they match, returns a decremented version of a pointer in a register.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_memory_and_call_library`: Fills a memory range with zeros and calls functions from a library based on a global pointer.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tokenize_string_into_array`: Parses a string by splitting it into individual tokens based on whitespace and quoted substrings.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `increment_table_pointer`: Saves registers to the stack, loads a pointer from the stack, compares a word at that pointer to a constant, and either branches or increments the pointer by four.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `initialize_register_and_branch`: Loads a constant value into a register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `update_window_field`: Retrieves a pointer from a structure and calls a library function to update a field, repeating the process in a loop if a condition is met.
- `process_window_data`: Traverses a linked structure to retrieve data and calls library functions to update specific fields.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Checks the signs of two registers, negates them if they are negative, and calls another routine to perform division.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `call_remove_linked_list_node`: This routine calls another routine that removes a node from a linked list structure.
- `remove_linked_list_node_2`: A routine that calls two other routines to traverse and modify a linked structure.
- `remove_linked_list_node`: This routine calls another routine to traverse a linked structure and remove a node by updating pointers and clearing status flags.
- `traverse_and_clear_nodes`: Iterates through a sequence of memory locations, calling itself recursively for each node until a specific boundary is reached.
- `recursively_clear_node`: Checks a flag byte in a structure to decide whether to recursively call itself or call other routines, then clears the structure's fields.
- `delete_linked_list_node`: Recursively traverses a linked structure to remove nodes by updating pointers and clearing status flags.
- `search_and_zero_nodes`: Traverses a list of structures and clears three longwords in the first node where a specific byte is zero.
- `branch_to_offset_2`: Performs a short branch to a specific instruction offset.
- `clear_and_branch_2`: Clears three consecutive longwords starting from an offset of a pointer in register A2, moves that pointer to register D0, and branches.
- `update_status_flags`: Calls two different routines and updates various fields in a structure based on their return values.
- `iterate_linked_list_and_call_function`: Traverses a linked list starting from a pointer stored in a global variable, passing each node and a value derived from the node's second word to another routine.
- `insert_into_linked_list`: Calls a routine to allocate memory, then inserts a new node into a linked list by updating pointers.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_from_list`: Searches for a specific node in a linked list and removes it by updating the preceding node's pointer or the list head.
- `validate_index_and_call`: Checks if a provided index is within valid bounds and if a corresponding entry in a table is non-zero before calling another routine.
- `handle_file_deletion`: Attempts to delete a file and, if unsuccessful, retrieves an error code to store in a global variable.
- `execute_io_operations`: A routine that calls other routines to perform conditional writes and handles input/output error reporting.
- `execute_conditional_write`: Calls a routine with specific parameters and, if a bit is set in the result, calls another routine if a global value is non-zero.
- `execute_write_operation`: Calls a function to write four bytes from a specific memory location and then calls another routine to handle the result.
- `execute_callback_and_routine`: Checks if a global function pointer is non-zero to execute it, then calls another routine.
- `validate_and_clear_pointer`: Checks if an index is within bounds and if a pointer in a table is non-zero, then calls another routine with that pointer before clearing it.
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
- `execute_indirect_call`: Performs an indirect call to an address retrieved from a table.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Allocates a memory block, populates it with several bytes and integers from the arguments, and returns the pointer to the new structure.
- `reset_structure_fields`: Sets specific bytes and longwords in a structure to fixed values and calls another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: Calls another routine passing the input parameter and a fixed constant value of 48.
- `reset_structure_fields_2`: Sets multiple fields within a structure to constant values and then calls another routine using the structure pointer as an argument.
- `update_structure_via_function_pointer`: Calls a function pointer stored in a global variable and, if successful, writes a byte, a word, and a longword into the returned memory address at specific offsets.
- `clear_structure_values`: Sets several fields in a structure to specific constant values and calls another routine.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `copy_stack_and_process_data`: Copies a block of data from a global table to the stack, calls another routine, and then populates a structure with the results.
- `call_external_routine`: Calls another routine passing the input pointer as an argument.
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
- `execute_indirect_library_call`: Performs an indirect call to a function located at an offset within a library structure.
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
- `return_zero_2`: The routine clears the accumulator and returns to the caller.

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
- `update_linked_list`: Library code that updates a pointer in a structure and calls a function through a pointer stored in a global variable.
- `set_byte_bit`: Library code that sets a specific bit in a byte located at an offset from a register.
- `increment_and_call_library`: Library code that checks a bit in a byte, increments a short word at an offset if the bit is clear, and calls a library function if the word reaches one.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `call_internal_routine`: Library code that calls another routine and returns.
- `calculate_bitfield_difference`: Library code that performs a loop to adjust a bitfield value using a doubling accumulator and bitwise shifts.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `call_and_mask_bits`: Library code that moves a value to a register, calls another routine, and then sets or clears specific bits in a register.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_register_and_branch`: Library code that clears a long register and branches to another location.
- `execute_conditional_callback_chain`: Retrieves a base pointer from a global variable to access a table of function pointers.
- `execute_library_callbacks`: Library code that executes a series of function calls through pointers stored at various offsets from a base register.
- `execute_library_callback_chain`: Library code that executes a series of function calls through pointers stored at various offsets from a base register, with branching based on register values.
- `execute_conditional_subroutine_chain`: The library code moves a value to a register, calls another routine, and branches based on the result.
- `call_and_preserve_register`: Library code that calls a routine using an offset from a register and preserves the original value of that register.
- `branch_on_test`: Library code that moves a value to a register, tests a long value, and either branches or loads a constant into a register.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_integer_scaling`: Library code that performs bit-shifting and decrementing to calculate a value based on a loop counter.
- `initialize_shift_parameters`: Library code that performs a logical shift right on a register and initializes several registers with constant values before branching.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch_library`: Library code that pushes a word onto the stack and branches to another location.
- `push_stack_and_branch`: Pushes a word of all ones onto the stack and performs a short relative branch to another location.
- `push_constant_and_branch`: Pushes a word containing the value one onto the stack and performs a short relative branch to another location.
- `compare_and_return`: Library code compares a byte to a constant, branches if higher, and returns after adjusting the stack pointer and testing a byte.
- `transform_mathematical_data`: Library code that performs complex bitwise and arithmetic transformations on a value to determine an index for table lookups.
- `perform_iterative_transformations`: Library code that performs iterative arithmetic transformations on data using a table and bitwise shifts.
- `transform_data_iteratively`: Library code that performs iterative arithmetic transformations on data using a table.
- `apply_iterative_table_transform`: Performs iterative arithmetic transformations on two registers using bitwise shifts and additions/subtractions.
- `perform_complex_math`: Library code performs iterative arithmetic operations involving bitwise shifts and additions using a pointer to a table.
- `load_constant_and_branch`: Library code loads a constant into a register and then performs a short branch.
- `recursive_call_wrapper`: Library code that calls itself and then returns, with no other operations.
- `save_and_call_routine`: Library code that pushes a long value onto the stack, calls another routine, and then restores a value from the stack.
- `clamp_and_shift_value`: The library code moves a long value, performs a conditional branch, compares it to a constant, and may clear the value before returning.
- `shift_and_jump`: Library code performs an arithmetic shift right on a long value, increments it, and then either branches or jumps to a zero address.
- `return_without_effect`: Library code calls another routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `process_recursive_bits`: Library code performs conditional checks on a byte and executes a series of recursive calls based on bitwise rotation results.
- `rotate_and_branch`: Library code loads a constant into a register, performs a rotate right through the carry flag, and then performs a short branch.
- `load_constant_and_branch_2`: Loads a specific longword constant into a register and performs a short relative branch to another location.
- `test_and_branch_word`: This library code pops a word from the stack into a register, tests if it is positive, and then performs a conditional branch or clears a register.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `return_immediately`: The library routine consists of four instructions and performs no observable work, immediately returning to its caller.
- `perform_recursive_math`: Library code that performs a series of recursive calls and manipulates a value based on bitwise operations.
- `execute_recursive_calls`: Library code that performs a series of recursive calls to itself before returning.
- `process_table_with_shifts`: The library code performs arithmetic shifts on two registers and executes a loop that adds values to registers and subtracts entries from a table pointed to by a register.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_byte_and_branch`: Library code that tests a byte, performs a bitwise AND, and calls another routine if certain conditions are met.
- `load_and_branch_byte`: This library code loads a constant byte value into a register and performs an unconditional branch.
- `calculate_bit_shifts`: Library code that performs a complex loop involving bit-shifting and conditional jumps to process values based on register contents.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_subroutine`: Library code tests a byte, conditionally masks it, and branches to another routine.
- `calculate_conditional_shift`: Library code performs multiple routine calls and executes conditional bitwise shifts and sign extensions based on a value.
- `perform_no_operation`: This routine contains no instructions and performs no operation.
- `transform_register_d7`: Library code that performs a sequence of swaps, rotations, bitwise XOR, and arithmetic operations on the contents of register d7.
- `modify_byte_value`: This library code compares a byte in a register against two specific constants, performs arithmetic and a bitwise rotation if a match is found, and then tests the result.
- `mask_and_test_word`: This library code performs a bitwise AND on a word, tests the result for equality to zero, and then tests the result as a byte before branching.
- `test_and_shift_byte`: The library code compares a byte to a constant, performs conditional branches, and executes logical shifts on a long value.
- `shift_and_rotate_word`: This library code performs a logical shift left on a word, loads a constant into a register, rotates a byte through the carry flag, and sets a bit in the condition code register.
- `clear_and_branch`: This library code clears a register and performs an unconditional branch.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `invoke_callback_and_return_2`: The library code calls a routine through a pointer in a register and returns the original value from a register.
- `invoke_callback_and_clear`: Library code that clears a long register and calls a routine via a pointer in a register.
- `invoke_callback_chain`: Library code retrieves a base pointer from a global variable to access a table of function pointers and executes a conditional callback chain.
- `invoke_callback_wrapper`: Library code that calls another routine and returns a value.
- `call_subroutine_and_return`: Library code that calls a subroutine and returns the value originally held in a register.
- `invoke_callback_with_one`: This library code invokes another routine and returns the original value from a register.
- `invoke_callback_with_all_ones`: This library code invokes another routine and returns the original value from a register.
- `store_register_via_pointer`: This library code invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register, then returns.
- `invoke_conditional_callbacks`: Library code invokes a routine via a pointer and then conditionally executes three additional routines in sequence.
- `invoke_callback_with_return`: Library code invokes a routine via a pointer and returns the original value from a register.
- `execute_indirect_callback`: Calls a routine via a pointer passed in a register and returns the original value from a data register.
- `execute_callback_and_return`: Calls a routine via a pointer provided in a register and returns the original value held in a data register.
- `execute_callback_and_return_2`: Calls a routine via a pointer provided in a register and returns the original value from a data register unchanged.
- `invoke_routine_thrice`: Library code invokes another routine three times in succession and then returns.
- `test_and_call_indirect`: Library code that tests a byte and calls a routine via a pointer in a register.
- `pass_through_argument`: Library code that calls another routine via a pointer and returns the original value from a register.
- `execute_callback_and_return_3`: Invokes a routine via a pointer passed in a register and returns the original value from a data register unchanged.
- `call_indirect_offset_one`: Library code that calls a routine at an address calculated by adding a negative offset to a pointer stored in a global variable.
- `call_indirect_offset_two`: Library code that calls a routine at an address calculated by adding a different negative offset to a pointer stored in a global variable.
- `load_pointer_and_branch`: Library code that loads a constant address into a register and performs a short branch to another routine.
- `load_address_and_branch_alt`: Library code that loads a different constant address into a register and branches to another routine.
- `call_indirect_with_offset_2`: Library code calls a routine at an address calculated by adding an offset to a pointer stored in a global variable.
- `execute_indirect_offset_call`: Retrieves a pointer from a global variable and calls a routine at an address determined by adding a negative offset to that pointer.
- `load_address_and_branch`: Library code loads a specific address into a register and then performs a short branch.
- `load_pointer_and_branch_2`: Loads the address of a global variable into a register and performs a short branch to another routine.
- `load_address_and_branch_2`: Loads a constant address into a register and performs a short branch to another location.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: The library code executes an illegal instruction.
- `libInit2`: The library code stores the value from A0 into a global variable.
- `MIExpunge`: This library code checks a flag in a structure; if the flag is zero, it updates a linked list and calls a function pointer, otherwise it sets a specific bit in the structure.
- `Exp_InUse`: This library routine sets the eighth bit of a byte located at an offset from a pointer held in an address register.
- `MIOpen`: This library code tests a bit in a structure, increments a value at an offset in the structure if the bit is not set, and returns the pointer to the structure.
- `openFail`: This library routine returns a value representing a failure.
- `MIClose`: The library code decrements a short counter stored in a structure and, if the counter reaches zero and a specific flag bit is set, calls MIExpunge and returns its result.
- `DPFix`: The library code performs bitwise operations and a loop of shifts on a value retrieved from a pointer, returning a processed result.
- `dfi4`: This library code tests a specific bit in a register and performs a conditional branch or clears another register.
- `dfi6`: This library code clears a long register and performs an unconditional branch.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: The routine converts a signed single-precision value in a register into a double-precision representation stored through a pointer.
- `DPAbs`: The routine copies the high word of a double from a source pointer to a destination pointer and writes the low word with its sign bit cleared.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: This library routine subtracts one from the value in a data register.
- `dtst_2`: This library routine adds one to the value in a data register.
- `cc_1`: This library routine calls a function whose address is passed in an address register and then returns.
- `cc_2`: The routine calls a function whose address is passed in an address register and then returns the contents of a stack slot.
- `cc_3`: The routine calls a function pointer held in register A1 and returns the value left on the stack at offset 18.
- `MIIEEEDPFix`: This library code calls another routine and then returns.
- `MIIEEEDPFlt`: Library code that calls a floating point routine and returns the value previously on the stack.
- `MIIEEEDPCmp`: Library code that performs a tail branch to another routine.
- `MIIEEEDPTst`: This library routine performs a tail branch to another routine located at a specific memory address.
- `MIIEEEDPAbs`: Library code that calls an absolute value routine and returns the result.
- `MIIEEEDPNeg`: Library code that performs a tail branch to a negation routine.
- `MIIEEEDPAdd`: Library code that performs a tail branch to an addition routine.
- `MIIEEEDPSub`: Library code that performs a tail branch to a subtraction routine.
- `MIIEEEDPMul`: Library code that performs a tail branch to a multiplication routine.
- `MIIEEEDPDiv`: Library code that performs a tail branch to a division routine.
- `MIIEEEDPCeil`: Library code calculates the ceiling of a value by negating it, applying a floor function, and then negating the result.
- `MIIEEEDPFloor`: Library code that performs comparisons and conditional additions or subtractions to calculate a floor value.
- `test_d6_d7`: This library code moves the contents of two registers into others, calls a routine, and then calls another routine after restoring registers from the stack.
- `lt_zero`: This library routine stores a constant into a stack slot and then tail calls another routine.
- `fptrap`: Performs a floating-point operation by looking up a handler in a jump table.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `dfsub`: Library code that saves registers, moves two values into address registers, modifies a bit in a longword, and branches to another routine.
- `dfadd`: Performs addition on two double-precision floating-point numbers stored in memory.
- `dfa11`: Normalizes a double-precision floating-point number by adjusting the mantissa and exponent.
- `dfa14`: This library routine performs conditional logic to store a 64-bit value into a memory location pointed to by a register.
- `dfa13`: Library code performs complex bitwise manipulations and arithmetic to store a result into a pointer.
- `dfa5`: Library code performs bitwise manipulations and scaling on input values before storing the result into a table.
- `dfa8`: This library routine compares two longwords and performs a subtraction, a logical shift left, and a rotate left through the carry flag based on the comparison result.
- `dfa7`: This library code modifies a bit in a register, performs logical OR operations, stores the results into memory via a pointer, and restores registers from the stack.
- `dfa9`: Library code that clears a longword in a register and a longword at an offset from that register, then performs a short branch.
- `dfauo`: Library code that compares a longword against a threshold and, if lower, stores a bitwise combination of two values into a memory location and another offset.
- `dfmul`: Performs multiplication on two double-precision floating-point numbers stored in memory.
- `dfm4`: This library routine clears a longword in a register and the longword at the offset of four bytes from that register before performing a short branch.
- `dfmuo`: Library code that compares a longword against a threshold and, if lower, stores a bitwise combination of two values into a memory location and sets a different offset to all ones.
- `dfdiv`: Performs double-precision floating-point division on values stored in memory.
- `dfd10`: Performs high-precision division on 64-bit floating-point or fixed-point values.
- `dfd4`: This library routine performs arithmetic operations on bit-packed data and stores the resulting 64-bit value into a memory location pointed to by a register.
- `dfsetz`: This library routine clears two data registers and then performs a short branch.
- `dfdbyz`: This library routine loads a short immediate value into a register and then performs a short branch to another location.
- `dfduo`: Library code that compares a longword against a threshold and, if lower, performs bitwise operations on two values and updates the processor status register.
- `dfneg`: This library routine loads a word from a pointer in a register, performs a bitwise AND, and if the result is not zero, flips the high bit of the byte at that pointer before branching.
- `dfcmp`: This library routine is empty.
- `dftst`: This library routine returns the 32-bit value that was passed on the stack.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### version.library

- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call_2`: Library code that decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `set_library_status_bit`: This routine retrieves a pointer from a register, checks a word at a specific offset within that structure, and sets a bit in a byte at a different offset if the word is non-zero.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `check_and_set_font_handle_2`: This library routine calls a routine via a table entry using a library base and, if the result is non-zero, stores a value into a structure and calls another routine via a table entry.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_library`: This library routine attempts to open two other libraries and, if successful, initializes a structure with a pointer and an offset before returning.
- `manage_font_memory_2`: Library code that iterates through a linked list to remove entries based on a short value, updates a counter, and potentially calls routines to free memory or unload segments.
- `manage_font_memory`: This library routine checks specific conditions in a structure, potentially calls two functions to close a library and free memory, manipulates a linked list, or sets a bit in a byte at an offset.
- `set_bit_and_clear_register`: The routine sets a specific bit in a byte at an offset from a register and then clears a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_conditional_jump`: The routine decrements a word at an offset from a register and performs conditional branches based on the result and a bit test, finally executing a jump to an offset from a register.
- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `scan_font_entries_for_best_metric`: Copies the caller's 8-byte argument block onto the stack and, when a lock on the referenced FONTS: path succeeds, loads a 1005-indexed record and reads a 4-byte header, then allocates a buffer of 260-byte entries and reads the table.
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
- `increment_pointer_on_non_zero`: This library routine checks a byte at a location pointed to by a value on the stack.
- `call_other_routine`: This library routine acts as a wrapper that calls another routine.
- `calculate_string_length`: This library routine iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_byte_sequences`: This library routine compares a byte from a memory location against a sequence of bytes.
- `compare_byte_and_branch`: This library routine compares a byte from a pointer to a value in a register and branches based on the result.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_memory_bytes`: This library routine compares a byte at a pointer location with a value from the stack.
- `increment_and_branch`: This library routine increments a pointer and then performs an unconditional branch to another location.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations.
- `serialize_font_metadata`: Iterates through a linked list of font string structures to copy character data and associated metadata into a destination buffer.
- `populate_font_string_buffer`: Iterates through a linked list of string structures to copy character data into a destination buffer.
- `copy_string_to_buffer_3`: Copies a sequence of bytes from a source location to a destination buffer.
- `parse_font_file_data`: Opens a font file and iterates through its contents to extract character metadata and attribute information.
- `populate_font_attribute_table`: Iterates through a font data block to extract character metadata and attributes.
- `parse_font_header_metadata`: Parses a structured font data block by reading specific header fields and populating a destination buffer with character attributes and offsets.
- `read_and_parse_font_data`: Reads data from a disk source and parses it into a structured buffer.
- `calculate_absolute_value`: This library routine calculates the absolute value of a signed integer.
- `return_zero_constant`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

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
- `test_multiple_bits`: This routine calls another routine multiple times to perform bitwise masking on a register and tests a specific bit.
- `execute_repeated_calls`: Calls a specific subroutine repeatedly in succession.
- `perform_bitwise_or_update`: This routine calls another routine multiple times and performs a bitwise OR operation to update a byte in memory.
- `test_register_bit`: This routine calls another routine and then tests a specific bit in a register.
- `modify_and_stack_data`: This routine calls another routine multiple times to perform bitwise XOR operations, tests a bit, and pushes bytes onto the stack from a pointer or a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_and_store_data`: This routine calls another routine multiple times to test bits, move data between registers and memory, and store a constant value to a memory location via a pointer.
- `call_test_bit_and_or`: Calls another routine, tests a bit in a register, and performs a bitwise OR operation on a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine`: calls a subroutine and performs a byte-sized move of a register to itself.
- `modify_memory_and_registers`: calls a subroutine, performs a byte-sized move of a register to itself, applies a bitwise AND mask to a memory location pointed to by a register, and sets specific bits in two registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_byte_to_stack`: calls a subroutine, pushes a byte from a register onto the stack, and clears the contents of a register.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_buffer_state`: Updates internal audio playback structures by checking hardware status and synchronizing buffer pointers.
- `copy_register_value`: copies the contents of a long register to another long register.
- `cleanup_and_release_resources`: Iterates through a list of function pointers to execute them and then releases allocated memory blocks.
- `validate_sequential_return_values`: This routine calls several other routines in sequence and checks if their return values are zero.
- `return_immediately_2`: This routine performs no operations and returns to its caller immediately.
- `return_constant_value`: Returns a constant integer value.
- `return_address_register`: Returns the value held in an address register.
- `execute_sequential_subroutines`: Saves several registers to the stack and executes a sequence of six subroutines before branching to a new location.
- `process_loop_with_hardware_updates`: Executes a series of subroutines within a loop that updates a byte in memory and terminates based on a return value.
- `call_library_function_directly`: Calls one library function on a base pointer held in the state block, then calls a helper routine.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_conditional_chain`: Sets a field in a record to minus one, calls a routine to obtain a pointer, and if successful executes a sequence of four further routines before calling a final routine via a global variable.
- `open_library_and_release`: Sets a result field to minus one, opens the dos.library name through the exec base, and when the open succeeds calls two methods on the returned base before closing it.
- `initialize_trackdisk_device_request`: Populates a device request record with a specific unit value and length, then executes a library function to open the disk device.
- `store_two_library_results`: Calls two library functions and stores their results in the state record and a second record, one as a long and one as a byte.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function`: Calls a library function using an offset from a base address held in a register.
- `update_structure_fields`: This routine copies values from several memory locations into a structure and then executes a subroutine via a pointer.
- `populate_record_data`: Writes a constant, a fixed value, two values from another record, and two calculated values scaled by a power of two into a target record.
- `populate_record_data_2`: Writes a constant, two values from another record, and two scaled values into a target record before calling a subroutine.
- `load_and_branch_constant`: Loads a word into a register and performs a relative branch.
- `load_and_branch_small_constant`: Loads a small word into a register and performs a relative branch.
- `load_constant_and_branch_3`: This routine loads a small constant into a register and then performs an unconditional relative branch.
- `clear_and_branch_constant`: Clears a register, loads a word into another register, and performs a relative branch.
- `update_record_fields`: Writes several values into a record, including a constant, two values from another record, and two calculated values scaled by a power of two.
- `register_disk_resource_handler`: Obtains the disk.resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `process_disk_resource_and_set_hardware`: Repeatedly calls a function on a disk resource pointer until it returns a non-zero value, then writes values to custom hardware registers and a global variable.
- `invoke_disk_resource_routine`: Calls one function on the disk resource base held in the state block.
- `configure_hardware_and_jump`: Sets interrupt enable and DMA control registers, then calls a routine via a pointer before branching to another location.
- `branch_to_target`: Performs a short relative branch to another location.
- `update_flag_byte_bits`: Clears the bit selected by the first register value in a state byte, then sets bit two when the second register value is zero.
- `copy_state_byte_to_register`: Copies a state byte into a control word and writes a fixed flag value to a second control word.
- `start_disk_sync_transfer`: Calls a helper, waits for a countdown, then writes DMA control, disk pointer, and sync words to custom chip registers before calling two more helpers.
- `set_top_bit_field`: Stores the incoming value with its top bit set into a 16-bit field of the record addressed by the second register, twice.
- `reset_record_and_call_library`: Sets a state word to a fixed value, sets a second word to two, and calls a library function on the base pointer.
- `count_consecutive_matches`: Calculates a value based on the number of times a specific word repeats consecutively within a memory range.
- `count_consecutive_values`: Iterates through a memory range to count how many times a specific word value repeats consecutively.
- `forward_to_helper_routine`: Calls one helper routine and returns with no other work.
- `compute_bitwise_checksum`: Computes a 16-bit checksum over a run of words by shifting each bit through the accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `return_without_action`: The body contains only a return, with no calls, memory access or arguments used.
- `check_library_status`: Decrements a value, checks a library function, and performs conditional branches based on a bit test.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `flatten_linked_string_list`: Iterates through a linked list of string structures to calculate their total length and copy the character data into a destination buffer.
- `concatenate_linked_string_list`: Iterates through a linked list of string structures, calculating total length and copying each string into a new destination buffer.
- `copy_and_reallocate_string_data`: Copies data from a source pointer to a destination buffer and performs length calculations.
- `parse_and_render_font_string`: Parses a null-terminated string from a data source and copies its characters into a destination buffer.
- `parse_and_copy_string_data_2`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `parse_and_copy_string_data`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `copy_string_to_buffer_2`: Calculates the length of a null-terminated string and copies its characters into a destination buffer.
- `call_library_functions`: Calls three different library functions using offsets from a base address stored in a global variable and returns the absolute value of a register.
- `return_zero_constant_2`: Returns a zero value without performing any other operations or accessing memory.
- `compute_font_offset_2`: Calculates a signed offset by comparing two 16-bit values and iterating through the bits of two byte arrays.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_empty_string`: Checks if a byte at a pointer is zero and increments the pointer if it is not.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length_2`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_byte_with_table`: Compares a byte from a pointer against a byte in a table and returns zero if they match and the pointer's byte is zero.
- `compare_and_clamp_byte`: Moves a byte to a register, compares it against a value, and sets the register to a constant if the comparison fails.
- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward_2`: Calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle_2`: This routine checks the validity of a font handle by attempting two specific operations.
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
- `call_empty_routine`: This routine calls an empty subroutine and then returns to its caller.
- `return_immediately_4`: This routine is empty and performs no operations before returning to its caller.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning to its caller.
- `modify_memory_fields`: Clears the high bit of a byte and increments a word at specific offsets from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_buffer`: Allocates a memory block, populates a table with incrementing offsets based on a base address, and performs two function calls involving hardware parameters.
- `call_indirect_function`: Fetches a function pointer from a location calculated by adding an offset to a value in a register and calls it with a pointer and a constant value as arguments.
- `concatenate_strings`: Joins three source strings into a destination buffer, inserting a separator character if necessary.
- `execute_conditional_callback`: Checks a function pointer in a global structure and either calls it or executes an alternative routine if the pointer is null.
- `execute_conditional_callback_alt`: Checks a different function pointer in a global structure and either calls it or executes an alternative routine if the pointer is null.
- `execute_conditional_callback_third`: Checks a third function pointer in a global structure and either calls it or executes an alternative routine if the pointer is null.
- `execute_conditional_callback_fourth`: Checks a fourth function pointer in a global structure and either calls it or executes an alternative routine if the pointer is null.
- `traverse_and_copy_string_table`: Iterates through a table of pointers to find the first non-null entry and then traverses a linked list of string structures.
- `reset_table_entries`: Clears a two-byte field in a table repeatedly and calls another routine each time, using a counter that runs from a positive start value down to zero.
- `decrement_and_clear_counter`: Decrements a short counter stored at an offset from a pointer and, if the counter reaches zero or was already invalid, clears the counter and, when a flag at the base of the structure is non-zero, calls another routine and invokes a function pointer stored in a table.
- `search_table_for_match`: Walks a table of entries using a fixed stride and compares an input value with a field in each entry, calling another routine when a match is found.
- `parse_resource_header_and_allocate`: Parses a structured data block by checking for specific magic identifiers and iterating through a list of sub-elements.
- `call_other_routine_and_return_zero`: Calls another routine and then returns zero.
- `execute_infinite_recursion_2`: Calls itself indefinitely without performing any other operations.
- `mask_lower_bytes`: Calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `execute_infinite_recursion`: The routine calls itself indefinitely, creating an infinite loop of recursive calls.
- `execute_conditional_callback_2`: Compares a value from a global structure with a register and calls another routine if they are not equal.
- `wait_for_flag_clear_2`: Repeatedly calls another routine while a specific global variable remains non-zero.
- `remove_object_from_table_2`: If the object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_game_state_flag`: Clears a 16-bit value in a global variable and sets a flag byte in the same structure by OR-ing in a constant, then calls another routine.
- `copy_struct_to_message`: Calls another routine and, if certain conditions on the returned values and a pointer are met, copies a small fixed-size block from memory into a structure, updates several fields, and clears a byte in that structure.
- `clear_status_flags`: Clears a longword at a specific offset and clears a bit in another memory location.
- `clear_flag_and_iterate_table`: Checks a flag in a global variable, clears it, and then iterates through a table of integers, calling another routine for each non-zero entry.
- `process_data_table_2`: Iterates through a table of four longwords, clearing specific offsets in a destination structure and updating values based on comparisons with data from a source table.
- `increment_and_branch_2`: Increments a byte in a register, stores the result into a memory location using a pointer with an offset, and performs an unconditional branch to another routine.
- `increment_pointers_and_conditional_branch`: Increments two longwords in registers and a byte in a register, then performs a conditional branch based on a comparison of that byte to a constant value.
- `copy_struct_fields_and_update_table`: The routine copies seven words from one area of a structure to another area of the same structure and then copies a word from one offset to another.
- `execute_recursive_pointer_calls`: This routine checks if a pointer in a structure is non-null and calls it.
- `process_event_queue_and_update_timers`: Iterates through a linked list of event structures, decrementing counters and updating pointers based on specific byte values.
- `decrement_and_loop_counter`: Library code contains a loop that decrements a value at an offset from a pointer and performs conditional updates to registers.
- `process_event_linked_list`: Iterates through a linked list of data structures, decrementing internal counters and updating pointers based on specific byte values.
- `process_linked_event_queue`: Iterates through a linked list of data structures, decrementing counters and updating pointers based on specific byte values.
- `compare_and_decrement_flag`: Compares two longwords and, if they are not equal, checks a short flag at an offset from a pointer to potentially decrement it and call another routine.
- `clear_flag_after_callback`: The routine checks a short flag at an offset from a pointer and a global variable at another offset.
- `signal_on_flag`: Checks a global flag and, if set, calls another routine via a function pointer stored in a global variable.
- `process_sound_table`: Iterates a table of entries and invokes callbacks stored in each entry, setting bits in a mask based on their return values.
- `setup_sound_channel`: Writes a value derived from an input register to a global variable and then writes the constant 2 to the left channel of audio channel 0.
- `search_and_update_slot_2`: Iterates through a table to find a matching entry based on specific byte criteria, then updates a slot with data from another routine.
- `search_and_update_slot`: The routine iterates through a table to find a matching entry based on specific byte criteria, then updates a slot with data from another routine and sets a status byte.
- `set_register_and_branch_2`: Loads a constant value into a register and performs an unconditional branch to another location.
- `clear_state_and_wait`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_flag_byte`: Computes an address from a global variable and a register value, writes zero to a byte at that address, and if the following byte equals one, writes two to the first byte.
- `reset_and_poll_status`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero, returning the final value in a register.
- `clear_slot_and_notify`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot, calls a callback via an indirection, and clears the next byte and the slot byte.
- `calculate_scaled_value`: Stores a word into a pointer offset and performs a calculation involving multiplication and division to update a value in a structure.
- `store_word_and_branch`: The routine stores a word from a register into an offset of a pointer and then performs an unconditional branch.
- `update_scaled_value`: Calculates a scaled value based on a multiplier and a difference between two stored values, updates a field, and potentially clears a bit in a flag byte.
- `clear_flag_and_call_routine`: Checks a short value, clears a bit in a flag at an offset from a pointer, and calls another routine if the flag was set.
- `increment_and_branch_3`: Adds a constant byte value to a data register and performs a relative branch to another location.
- `decrement_and_recursively_check_counter`: Decrements a short integer located at an offset from a pointer returned by another routine.
- `update_sprite_state_and_position`: Updates sprite animation frames, scaling factors, and coordinate positions using fixed-point arithmetic.
- `decrement_counter_and_branch`: Calls another routine, checks if the returned pointer is non-zero, decrements a short value located at an offset from that pointer, and calls another routine if the new value is zero.
- `copy_struct_fields_and_clear_flag`: Copies several fields from a structure pointed to by one register into a destination pointed to by another when a condition is met, then returns a field from the source after clearing it.
- `update_sprite_animation_and_frame`: Updates sprite animation state by calculating frame indices and scaling factors using fixed-point arithmetic.
- `update_sprite_transform_and_distance`: Calculates new sprite coordinates, scaling factors, and distance-based properties using fixed-point arithmetic.
- `update_sprite_position_and_scaling`: Calculates new coordinate and scale values for a sprite based on fractional arithmetic and distance-based scaling logic.
- `compare_and_store_byte`: Compares a byte in a register to a constant and branches if they are not equal.
- `branch_on_byte_comparison`: Compares a byte to a constant and branches to another routine or a different location based on the result.
- `decrement_word_if_positive`: Checks a word at an offset from a pointer.
- `calculate_audio_resampling_offset`: Calculates a new offset value by multiplying a value from a table with a scaling factor and adding it to a base value.
- `update_audio_resampling_parameters`: Calculates pitch, frequency, and step increment values for audio playback based on a source sample rate and a target playback rate.
- `subtract_and_branch`: Compares two longwords and subtracts a longword from a second longword if the first is not less than the second.
- `calculate_audio_resampling_parameters`: Calculates pitch, frequency, and step increment values for audio playback based on a source sample rate and a target playback rate.
- `resample_audio_buffer_2`: Calculates resampling parameters and performs audio sample interpolation or direct copying into a destination buffer.
- `resample_audio_buffer`: Resamples a source buffer into a destination buffer using either direct byte copying, linear interpolation between adjacent samples, or a more complex interpolation method.
- `resample_fixed_point_buffer`: Updates a fixed-point value and resamples a source buffer into a destination buffer using either linear interpolation or a modulo-based distribution method.
- `update_fixed_point_table`: Performs fixed-point arithmetic to update a value in a structure and then uses a loop to redistribute or resample bytes from a source buffer into a destination buffer.
- `update_sprite_reference_and_scale`: The routine iterates through a table of pointers, comparing a block of data from each entry against a reference block to manage reference counters in a structure.
- `branch_to_next_instruction`: Performs a short relative branch to a nearby instruction.
- `decrement_structure_short`: Checks if a pointer in a structure is non-zero, then decrements a short value at a specific offset within that pointed-to structure.
- `process_command_buffer_and_update_state`: Parses a command buffer to perform conditional logic, calculates geometric or coordinate-based values using bitwise shifts and multiplications, and updates a structured data block.
- `decrement_counter_and_call_3`: The routine reads a pointer from an object field, checks it is non-zero, then loads a short from an offset in the pointed-to structure, decrements it and writes it back.
- `compare_and_clear`: Compares a word in a register to a constant and clears a long word in a register if the value is less than that constant.
- `update_interpolated_audio_parameters`: Calculates interpolated audio sample values and playback timing by performing fixed-point arithmetic, bitwise scaling, and linear interpolation between two data points.
- `calculate_interpolated_audio_parameters`: Calculates interpolated values for audio playback by performing fixed-point arithmetic and bitwise scaling on data retrieved from memory tables.
- `compare_and_store_byte_2`: Compares a byte in a register to a constant and branches if they are not equal.
- `compare_and_branch_2`: Compares a byte in a register to a constant and branches if they are not equal.
- `decrement_word_if_nonzero`: Tests a word at an offset from a pointer and decrements it by one if the value is not zero.
- `update_physics_and_collision_state`: Calculates new positional and velocity values for an object by performing scaled multiplications and bitwise shifts on structure members.
- `subtract_if_less`: Compares two longwords and branches if the first is less than the second.
- `calculate_complex_values`: Performs several arithmetic operations involving multiplications and bit shifts to update values in a structure.
- `cleanup_device_and_library`: The routine checks a global pointer, and if non-zero, performs several operations including calling a function via a pointer from a global variable, updating a value in a structure, and calling several other routines before clearing global pointers.
- `execute_signal_sequence`: Calls a sequence of three different functions from a global table, passing an argument to one, and returns a value formed by masking an input register with the result of a table call.
- `store_task_and_flags`: Clears a word in a structure pointed to by a register and, if a flag in another register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the result, and copies the flag into the structure.
- `compare_strings_case_insensitive`: Compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `return_called_register`: Calls another routine and returns the value that was left in an address register.
- `allocate_memory_block`: Checks if a memory pool is available or needs allocation, calls a memory allocation routine, and if successful, stores a size value into the allocated block.
- `free_memory_block`: Checks if a pointer is non-zero and then calls either a specific function pointer or a standard memory freeing routine to release the memory.
- `call_and_branch_on_result`: Calls another routine and, if the return value is non-zero, executes a sequence of conditional routine calls.
- `check_and_branch_on_result`: Calls another routine and tests its return value; if non-zero, it calls several other routines in sequence, conditionally invoking another routine based on a pointer and a bitmask check, before returning a 64-bit value.
- `initialize_object_structure`: Allocates and initializes a complex data structure by setting various offsets, including pointers, status flags, and integer values.
- `execute_conditional_calls_2`: Checks a global variable and calls another routine multiple times if the value is non-zero.
- `call_subroutine_and_copy_struct`: Calls another routine and, if certain conditions involving a pointer and returned values are met, copies a fixed-size memory block to a message buffer.
- `copy_first_string_pointer`: Identifies the first non-empty string in a list of pointers and copies its pointer or content to a destination.
- `initialize_sonix_driver`: Allocates and initializes a complex data structure and attempts to copy a string pointer from a list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_envelope_and_pitch`: Updates a pitch and volume tracking variable based on a counter and a bitmask from a data table.
- `initialize_hunk_data`: Parses Amiga hunk data to populate a structure with specific values, a pointer to a global variable, and a function address.
- `trigger_crowd_audio_event`: Executes a sequence of function calls via a pointer table to update audio parameters and trigger a specific sound effect associated with the crowd stopping.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_match_statistics`: Clears several data tables and then iterates through a structured byte array to compare adjacent values.
- `compare_table_elements`: Iterates through a table of sixteen-bit values to find consecutive matches and calls other routines based on the results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_values`: Calls a function pointer to retrieve a value and stores it in two consecutive memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_table_match`: Compares a value at a specific memory location with a value in a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_sequence`: Retrieves a value from a table using an index and performs conditional updates to a local variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_call_sort`: Increments a value, pushes it and a value from a table onto the stack, calls another routine, and then performs a loop that decrements a value from a table until it reaches a specific limit.
- `sort_player_attributes`: This routine performs a comparison-based sort on a subset of a data table using attribute differences.

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

- `validate_and_store_results`: The routine calls several other routines and stores their return values into specific offsets of a structure, returning a specific constant if certain conditions are not met.
- `frag_lea_movea_rts`: Short code fragment that loads the address of a location in the code into a5, then copies the memory at an offset from a5 to a4, then copies a fixed memory address to a6, then returns.
- `return_constant_value_2`: Returns a fixed integer value to the caller.
- `return_function_pointer`: Returns a pointer to another routine.
- `execute_multiple_subroutines`: Saves registers to the stack and executes six consecutive subroutines before branching to a new location.
- `process_loop_with_status_update`: The routine executes a loop that checks a condition, updates a byte in a structure, and performs several other operations, returning a value based on the loop's outcome.
- `call_indirect_function_via_offset`: Fetches a function pointer from an offset within a structure pointed to by a register and executes it.
- `call_library_vector_228_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `release_library_resource`: Sets a result field to minus one, opens a library, and if successful, calls three methods on the returned base before calling another routine and returning the result.
- `initialize_and_query_dos_library`: Sets a field in a structure to negative one, opens dos.library, and if successful, executes two specific methods before closing the library and returning the previously stored value.
- `open_trackdisk_device_request`: Fills a device request record with a unit value and a fixed length, then calls a library function to open the trackdisk.device.
- `store_dual_library_outputs`: Calls two library functions and stores the results into two different records, one as a long and one as a byte.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_offset_routine`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `update_structure_fields_2`: Copies values from one memory location to another and populates a structure with several multi-byte constants and values from other memory locations.
- `update_record_from_registers_2`: Calculates several values using three input registers and stores them into a record at various offsets.
- `initialize_record_structure`: Loads a pointer from a memory offset and populates a structure with constant values, copied data from other memory locations, and scaled values derived from registers.
- `set_register_and_branch`: Stores a constant value into a register and branches to another location.
- `load_register_and_branch`: Loads a word into a data register and performs a short unconditional branch to another location.
- `set_register_and_branch_3`: Loads a small constant into a data register and performs a short branch.
- `set_registers_and_branch`: Clears a register, stores a constant value into another register, and branches to another location.
- `update_record_from_registers`: Calculates values based on three input registers and stores them into a record at various offsets.
- `register_disk_resource_handler_2`: Obtains a pointer to a disk resource and stores it in a state record along with a fixed value, a code pointer to the current routine, and another constant value.
- `process_disk_resource_loop`: Repeatedly calls a function from a structure pointed to by a register, updates a global variable, and modifies a hardware register.
- `invoke_disk_resource_routine_2`: Calls a function located at an offset within a structure pointed to by a register.
- `configure_hardware_and_call`: Writes values to custom chip registers, loads two pointers into registers, and calls another routine.
- `branch_to_offset`: Performs a short branch to a specific memory address.
- `modify_state_byte_bits`: Clears a bit at a position determined by the first register and sets the second bit if the second register is zero.
- `update_control_word_flag`: Copies a byte from a memory location pointed to by a register into a global variable and then overwrites that variable with a fixed flag value.
- `start_disk_sync_transfer_2`: Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers.
- `set_top_bit_field_2`: Sets the most significant bit of a word and stores the result twice into a field at a fixed offset from a pointer held in a register.
- `reset_state_and_invoke_library`: Stores a constant value into a memory location pointed to by a register, stores the value two into another location pointed to by the same register, and then executes a function via a pointer stored in a base register.
- `count_max_consecutive_matches`: Calculates the length of the longest sequence of identical word values within a table.
- `count_consecutive_values_2`: Searches through a table of words to find and count a sequence of identical values.
- `call_helper_routine`: Calls one unnamed routine and then returns.
- `compute_bitwise_checksum_2`: Computes a 16-bit checksum over a sequence of words by shifting each bit through an accumulator and performing an exclusive-OR with a feedback value whenever the most significant bit is set.
- `test_bit_and_branch`: Tests a specific bit in a byte and branches if it is set.
- `save_extended_register`: The routine saves a value from a register onto the stack and stores an extended version of a word from a memory location into a local variable.
- `execute_supervisor_call`: This routine performs a series of operations and invokes a supervisor-level library call.
- `wait_for_flag_clear`: The routine loops, calling another routine, until a global variable is cleared to zero, then returns the combined value of the two input registers.
- `remove_object_from_table`: If an object pointer is non-zero, the routine clears a status flag in a global structure, iterates over a 64-byte name field calling another routine for each non-empty byte, calls another routine three times, searches a linked list in a global table for the object pointer to replace its entry with the next pointer, and calls another routine once more.
- `reset_state_flag`: Clears a 16-bit value in a global structure and sets a flag byte by performing a bitwise OR with a constant, then calls another routine.
- `clear_flag_byte_loop`: Clears a byte at an offset from a pointer, then iterates through a list of pointers, calling another routine for each non-zero entry until a counter reaches four.
- `clear_flag_byte_2`: The routine computes an address from a global variable and a value in a register, writes zero to a byte at that address, and if the following byte equals one it writes two to the first byte.
- `reset_and_poll_status_2`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero.
- `clear_slot_and_notify_2`: The routine computes a byte offset from the input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero it marks the slot, calls a callback stored four bytes after the slot via an indirection through a pointer, clears the next byte and the slot byte, then returns the input value.
- `calculate_scaled_value_2`: Performs arithmetic operations involving multiplication and division to store a calculated value and a scaled result into a table.
- `execute_cleanup_sequence`: Stores a word to a location, restores registers, and then conditionally calls several subroutines before returning.
- `cleanup_task_resources`: Checks a global variable, and if non-zero, executes several function pointers from a table and clears various global flags and pointers.
- `execute_signal_sequence_2`: Calls a sequence of three functions from a global table, passing an argument to the second, and returns a value formed by masking an input register with the result of the second call.
- `store_task_and_flags_2`: The routine clears a word in a structure pointed to by a register and, if a flag in a register is non-zero, stores a value from a register into the structure, calls another routine via a function pointer taken from a global variable, stores the returned value into the structure, copies the flag into the structure, and returns the original register values concatenated.
- `execute_conditional_function_pointer`: Checks if a register is non-zero, then executes a function pointer stored in a global variable or at an offset from another global variable depending on whether a specific pointer is null.
- `execute_conditional_calls`: Checks a global variable and calls another routine multiple times if that variable is non-zero.

### logged disk load from TV Sports Football (1988)(Cinemaware)[cr QTX](Disk 1 of 2).adf

- `parse_and_clear_string`: Iterates through a string to find the end of a sequence of non-space characters and then writes a null terminator to that position.
- `store_indirect_function_results`: Calls a function pointer located at an offset from a register and stores the returned byte into a memory location determined by another register and an offset.

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

- `check_flag_and_call_handler`: Tests a 16-bit value at offset -2b40 of the structure in a4 and branches away when that value is zero.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_two_counters`: Clears two 16-bit fields, one at offset -2b00 and one at offset -2afe, of the structure in a4, then returns.
- `call_and_store_results`: Pushes a constant and a flag, then calls a routine at offset -32592 through a pointer in the structure in a4.
- `call_routines_with_values`: Calls a routine at offset -32682, then calls a routine at offset -32154 three times, each time passing two 32-bit values read from the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_flags_and_call_handler`: Clears two bytes at offsets -2cc9 and -2cc8 of the structure in a4, sets a 16-bit value at offset -2c14 to 900 when a passed value is less than 2, initializes two flag bytes at offsets -3615 and -3616 when they are zero, and otherwise calls a handler at offset -7d9a returning its result minus 1.
- `rotate_byte_and_call_handler`: Sets up its stack frame, then checks a 16-bit value at offset -2b40 of the structure in a4 and jumps away when that value is zero.
- `call_indexed_routine_repeatedly`: Calls a routine at offset -32730 several times, each time passing a pointer from the structure in a4 and a computed table index.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to -(a7).
- `advance_stack_pointer`: Advances the stack pointer by 18 bytes and returns immediately.
- `prepare_and_call_handler`: Reads a byte from offset -3615 of the structure in a4, decrements it, and pushes it on the stack.
- `clear_and_store_byte`: Clears register d1 to zero, adds 8 to the longword at the address in d4, copies the zero into the address in a0, sets bit 7 of register d0, and stores that modified value of d0 at the location pointed to by a0.
- `tiny_clr_and`: Small routine of 2 instructions that clears -(a7), then masks (a0) with d3.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `decrement_and_divide_values`: Decrements d0, rotates d4 right by 4 bits, and divides d4 by the value in d6.
- `copy_and_clear_custom_register`: Copies the 16-bit value in d0 into d1, clears bit 0 of the custom chip register at offset -43df from a6, then decrements the byte pointed to by a2 and returns.
- `push_global_and_set_d6`: Loads the address of a global variable onto the stack, clears the d0 register, and sets bits 0 and 6 of the d6 register.
- `push_global_and_set_flags`: Pushes the address of a global variable at offset 24 onto the stack, clears d0, then sets bits 0, 5 and 6 of d6 and returns.
- `call_and_push_stack_frame`: Copies d1 into a0, sets bits 5 and 6 of d6, pushes two addresses onto the stack, calls the routine at offset -32730 from a4, sets bit 7 of d6, and pushes an empty stack frame before returning.
- `store_six_and_clear_d1`: Clears d0, sets bits 3 and 5 of d6, decrements d0, stores the 16-bit value 6 at the location pointed to by a0, clears d1, and returns.
- `call_and_decrement_byte`: Copies d1 into a0, sets bits 6, 7, 8, 9, 10 and 11 of d6, calls the routine at offset -32730 from a4, sets bits 12 and 13 of d6, reads a byte at offset -13846 from a4 into d0, decrements d0, and returns.
- `increment_structure_byte`: Loads the byte at offset 2 of the structure pointed to by a6 into the low byte of d0, increments that value by one, stores the result back to offset 2, then returns.
- `clear_byte_and_branch`: Clears the byte at offset -2 from a5, then branches to the routine at offset 1430 and does not return.
- `increment_byte_if_conditions`: Reads a short at an address computed from a5 and a4, and if that value is zero while a short at an address computed from a1 is nonzero, increments the byte at offset -2 from a5; otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_compare_byte`: Increments the byte at offset -2 from a5, compares it with the value 2, and returns.
- `increment_byte_loop`: Increments a byte at offset -3 of the structure in a5, then compares it against the decimal constant 18 and branches back to this routine while the value is still below that constant.
- `shift_byte_and_load`: Reads a byte from offset -2 of the structure in a5 into a data register, shifts it left by one bit, then loads the address of a table located at offset -26c0 from a4 into a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `describe_structural_facts`: Contains 14 instructions and is entered from outside its static flow, but the decompiler timed out so only these structural facts are known.
- `load_byte_and_adjust_stack`: Loads a byte from offset -3616 of the structure in a4 into a data register, loads the decimal constant 34 into another register, then increments the stack pointer by 4.
- `load_address_set_bits`: Loads the address of a table at offset -35c6 from a4 into a0, then sets bits corresponding to the decimal constant 236 of the byte pointed to by a6.
- `compare_bytes_clear`: Reads a byte from offset -2cc9 of the structure in a4, compares it against the byte at offset -2cc8, branches away when the two differ, otherwise clears the byte at offset -2 of the structure in a5.
- `load_byte_from_struct`: Reads a byte from offset -2cc9 of the structure in a4 into a data register.
- `decrement_byte_via_a5`: Decrements the byte value stored at the address pointed to by register a5 and places the result into data register d1.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies d1 to d2.
- `load_table_address`: Loads the address of a table at offset -5746 from a4 into a0.
- `decrement_byte_store_d3`: Decrements the byte stored at offset -2 of the structure pointed to by a5 and places the result into data register d3.
- `copy_word_and_shift_d0`: Copies a 16-bit word from the source pointed to by a0 to the address formed from a1 and d3, loads a long from a global variable into another global at offset -28d4, then shifts d0 left by 4 bits.
- `shift_byte_and_call_handler`: Reads a byte at offset -2 from a5, shifts it left by 1, calls another routine, then adds 2 to the stack pointer.
- `shift_byte_and_adjust_stack`: Reads a byte at offset -2 from a5, shifts it left by 1, then adds 2 to the stack pointer.
- `copy_word_and_loop_on_count`: Copies a 16-bit word from the source pointed to by a0 to the address formed from a1 and d2, increments the byte at offset -2 of a5, and branches back when that byte is less than or equal to 3.
- `tiny_unlk`: Small routine of 1 instruction that removes the stack frame held in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
