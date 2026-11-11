# Game 2099 (tv sports football (1988)(cinemaware)[cr qtx][a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02099). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1853604 top-level bytes in 101 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 49 of 49 units identical (bytes and relocations), 100 raw-word lines |
| Functions named and described | pass, 1114 of 1114 named (59 author symbols), 1114 of 1114 described (277 by rule, 820 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6324 of 6347 (99.64%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18546 of 18610 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 174874 |
| code | 60108 |
| data | 1483344 |
| padding | 23150 |
| unused | 112128 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf:fb | 3 | 9208 | 471 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf:i.c.g. | 2 | 117 | 2 |
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
| TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 162 | 86 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1404 | 87 |
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
| logged disk load from TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf | 1 | 145 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 48 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 214 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: file KBlocks.1 (header 406205 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 2 of 2).adf: file KBlocks.0 (header 352150 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 2 of 2).adf: file KBlocks.1 (header 385002 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 64 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f50): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 287 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: 85 executed PCs whose RAM was overwritten before any snapshot match its bytes 75344-75856: recovered as a unit run at an address
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
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: blocks 895-896 (bytes 458240-459264) hold ASCII text
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: blocks 898-900 (bytes 459776-461312) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.50)
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: blocks 903-1234 (bytes 462336-632320) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.78)
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 1 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf: blocks 1236-1696 (bytes 632832-868864) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.91)
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 2 of 2).adf: blocks 15-710 (bytes 7680-364032) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.59)
- tv sports football (1988)(cinemaware)[cr qtx][a3](disk 2 of 2)/TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 2 of 2).adf: blocks 895-1654 (bytes 458240-847360) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.33)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_disk_resource_loop` (runtime segment an address): Repeatedly calls a function via a pointer in a record until it returns a non-zero value, then updates two global variables. Kind: disk hardware/track validation loop. Pass path: the loop breaks and proceeds to u29__sub_360A when iVar2 != 0.
- `configure_hardware_and_call_subroutine` (runtime segment an address): Sets specific values in custom chip registers, loads two pointers from a table, calls a subroutine via a pointer, and branches to another location. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `copy_state_byte_to_register` (runtime segment an address): Copies a state byte into a control word and writes a fixed flag value to a second control word. Kind: disk hardware/track check. Pass path: the branch following the call (e.g., an address in sub_3624).
- `start_disk_sync_transfer` (runtime segment an address): Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers. Kind: disk sync/track check. Pass path: execution continues to the next instruction (movea.l an address(a4), a0).
- `execute_resource_polling_loop` (runtime segment an address): Repeatedly calls a function from a structure until it returns a non-zero value, then calls another function and updates two global variables and hardware registers. Kind: resource/hardware polling loop. Pass path: the loop breaks and proceeds to u40__sub_0342 when iVar2 != 0.
- `configure_hardware_and_call` (runtime segment an address): Modifies custom hardware registers and calls another routine using a pointer from a structure. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `update_control_word_flag` (runtime segment an address): Copies a byte from a memory location into a global variable and then overwrites that same variable with a fixed flag value. Kind: disk hardware/track validation check. Pass path: the branch following the call (specifically the 'bpl.b' instruction in sub_035C, implying a positive result in D1 is the success path).
- `initiate_disk_sync_transfer` (runtime segment an address): Calls a helper routine, executes a delay loop, and then writes disk pointer, sync, and DMA control values to custom chip registers. Kind: disk sync/track check. Pass path: the instruction immediately following the call (e.g., movea.l an address(a4), a0).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of one register to another and then performs a short branch.

### EndCLI

- `find_max_and_populate_buffer`: Traverses a linked list to identify the maximum value in a specific field and then populates a buffer with a header and a sequence of incrementing values.
- `move_and_branch_2`: Copies a long value from one register to another before branching to a new location.

### cd

- `update_buffer_and_copy_data`: Traverses a linked list to find the maximum value in a field, then writes a header and populates a buffer based on that value.
- `move_and_branch_3`: The routine moves a 32-bit value from one register to another and then performs a short branch.

### Port-Handler

- `parse_amiga_hunks`: Iterates through a list of data structures to identify and process AmigaDOS hunk types.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf:fb

- `initialize_and_dispatch`: Fills a block of memory with zeros, sets a global value, and then executes a series of function calls through pointers stored in a global structure.
- `initialize_game_state_structure_2`: Resets multiple global status variables to zero and initializes hardware parameters.
- `execute_indirect_calls_3`: Performs multiple calls to other routines, including one call through a pointer retrieved from a memory offset.
- `execute_callback_chain_3`: Calls two functions via pointers, passes a value to a third function, and then calls itself twice.
- `dispatch_via_table`: Uses a value to index into a table of function pointers and executes the corresponding routine with provided arguments.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `jump_via_table`: Compares a value against a constant, potentially shifts it, and uses it to index into a table to perform a jump.
- `execute_callback_sequence`: Sets two values in a structure, calls a function via a pointer, and executes a sequence of other subroutines and callbacks.
- `dispatch_library_calls`: Sets values in a structure and executes a series of function calls through pointers, including several subroutines.
- `process_hunk_data`: Sets several values in a structure, executes a series of function calls through pointers, and performs a conditional check based on a value in a table before calling more subroutines.
- `update_announcer_state`: Sets two word values in memory, calls a function via a pointer, and conditionally executes a routine using a value retrieved from a pointer.
- `execute_indirect_call_3`: Calls a routine via a pointer, stores the result in a global variable, and if that result is zero, calls another routine before storing a calculated address into a global variable.
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
- `manipulate_data_via_pointers`: Calls several subroutines to modify data at a specific memory location and returns a value from a global variable.
- `perform_data_operation`: Calls subroutines to manipulate data at a specific memory location and returns a value from a global variable.
- `process_parameterized_request`: Checks a condition, then calls a sequence of subroutines to manipulate data via a handle and returns a result.
- `parse_8svx_container`: Processes a data structure by performing multiple memory operations, recursive calls, and string comparisons to extract or validate information.
- `modify_registers_and_memory`: Increments a register, negates a value in memory, performs a bitwise OR on a register, and decrements another register.
- `compare_byte_strings`: Compares two byte strings byte by byte using a loop and a subroutine to read data.
- `compare_and_update_global`: Compares a value retrieved from a function call with a previously stored value, then updates a global variable and returns a result based on the comparison.
- `test_bit_and_branch`: Tests a specific bit in a memory location and branches to a different location based on the result.
- `branch_with_constant`: Loads a constant into a register and branches to another location.
- `test_memory_bit`: Loads a pointer from a memory location, tests a specific bit in the data pointed to, and branches or sets a register based on the result.
- `test_bit_and_set_register`: Tests a specific bit in a memory location and either sets a register to a specific value or branches.
- `clear_register_and_branch_2`: Clears the d0 register and performs a short branch to another routine.
- `check_bit_mask_status`: Evaluates two different bit masks within a memory location and returns zero, one, or negative one based on whether the bits are set or cleared.
- `check_bit_mask`: Checks if specific bits are set or cleared in a memory location using bitwise operations and branches accordingly.
- `branch_with_zero_alt`: Loads zero into a register and branches to a different location than the previous routine.
- `clear_memory_tables`: Performs multiple nested loops to zero out various segments of memory organized in specific patterns and dimensions.
- `test_structure_offsets`: Tests two word values at specific offsets from a pointer.
- `calculate_offset_addresses`: Calculates one or three memory addresses based on values in a structure and a table, then adds a value to the data at those locations.
- `update_conditional_offset_values`: Checks a flag in a structure to determine which set of three memory addresses to calculate.
- `update_calculated_offsets_2`: Calculates three different memory addresses based on values from a table and a global variable, then adds a value to each.
- `accumulate_coordinate_offsets`: Clears a block of memory and then performs a series of additions, summing values from multiple source tables into a destination table using iterative loops.
- `call_library_function_3`: Calls a function via a pointer stored in a table and stores the result in a global variable.
- `call_function_with_argument`: Calls a function using a value retrieved from a global variable and passes four as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `write_to_hardware_registers`: Writes a value from a global variable and the constant one to specific hardware register offsets.
- `execute_empty_recursion`: The routine calls itself three times before returning.
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
- `process_coordinate_table_2`: Iterates through a loop to perform multiple function calls using incrementing values and updates a global variable.
- `clear_memory_word`: Stores a zero value into a memory location calculated from a register.
- `test_and_branch`: Tests a word at a specific offset from a pointer and branches if it is zero.
- `sync_dual_state_buffers`: Synchronizes two sets of three-word data structures based on the status of several bit-flags and a XOR-derived condition.
- `clear_and_copy_words`: Clears three 16-bit words at offsets from a pointer and conditionally copies three words from another memory location to the pointer's offsets.
- `clear_structure_words_2`: Clears three 16-bit words at specific offsets from a pointer and then performs a short branch.
- `test_and_update_words`: Tests a word at an offset from a pointer and, if non-zero, executes three separate subroutines to update three 16-bit words at specific offsets.
- `update_structured_data_2`: Clears three 16-bit words from a pointer, tests a word at another offset, and if not zero, copies three words from one location to another before branching.
- `update_structured_data`: Clears several words in a structure and then conditionally copies values between different memory locations based on comparisons.
- `clear_structure_words`: Clears three specific words at offsets from a pointer and then tests a value at another offset from a different pointer.
- `clear_specific_words`: Checks a value at an offset from a pointer and, if certain conditions are met, writes three specific words to different memory locations.
- `clear_memory_words`: Stores zero values into three specific memory locations and then performs a relative branch.
- `clear_memory_and_branch`: Clears three words at specific offsets from a pointer and then performs a relative branch.
- `format_and_call_digits`: Decomposes an integer into individual digits, applies padding rules, and calls a routine for each digit using values from a table.
- `execute_callback_list`: Iterates through a list of pointers, marking entries with a specific value and calling a function using parameters from a table.
- `process_indexed_callback_list`: Iterates through a list of pointers, marking entries with a specific value to avoid re-execution, and calls a function using parameters retrieved from a table and a global structure.
- `iterate_and_call_function`: Decrements a counter in a loop and calls a function via a pointer if a specific value in a table is not equal to thirty-two.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `call_indirect_functions`: Calls two different functions using pointers stored in a table, passing various parameters including sums of input values.
- `setup_stack_and_branch`: Sets up a stack frame and performs a short relative branch.
- `copy_memory_block`: Copies bytes from one memory location to another using a counter and an index.
- `calculate_accumulated_value`: Performs a loop to combine a global variable with other values and stores the result in a global location.
- `update_complex_accumulator`: Uses a loop and bitwise operations to add a combination of global and local values to a stored integer.
- `increment_table_entry`: Calls a function, uses the result to calculate an offset into a table, and increments the word at that location.
- `update_accumulated_sum`: Calculates a sum using two values from memory and a global variable, then stores the result back into one of those memory locations after adding multiple instances of the global variable to it.
- `copy_and_reset_data`: Copies a longword from one location to another and then sets the original location to all ones.
- `copy_value_via_pointer`: Calls another routine and uses its return value as a base pointer to store a word at a specific offset.
- `handle_disk_prompt`: Copies a string to a buffer and enters a loop to wait for a disk or button input, calling various subroutines based on the result.
- `process_input_loop`: Enters a nested loop structure that calls various routines to handle input, compare values against constants, and process data through pointers.
- `call_and_copy_data`: Calls a function via a pointer and then calls another routine to copy data.
- `process_event_loop_2`: Executes a sequence of subroutine calls and loops that process data from a structure and update a global variable based on returned values.
- `process_event_loop`: Executes a complex loop that calls multiple subroutines and performs conditional branches based on values in a table.
- `process_linked_list_nodes`: Iterates through a linked list structure, performing conditional comparisons and executing function pointers stored within the list elements.
- `decrement_and_branch_on_zero`: Decrements a value in a register and performs conditional branches based on whether the value reaches zero or a specific threshold.
- `branch_to_address`: Performs a relative branch to another location.
- `branch_to_subroutine_2`: Performs a short branch to another location.
- `process_game_state`: A large routine that calls several subroutines to update game state, graphics, display parameters, and check button status.
- `update_game_state_and_graphics`: Copies a block of data from a source table to a local stack buffer and performs a series of state updates by calling various subroutines with specific parameters.
- `copy_data_block`: If a specific global flag is set, it clears that flag and copies five words from one memory location to another.
- `update_display_parameters`: Calls several functions to retrieve a value, calculate a new offset, and perform multiple writes to a table using that offset and provided parameters.
- `check_button_status`: Checks if two specific bits are set in a global variable and returns one if both are active, otherwise returns zero.
- `pass_parameters_to_subroutine`: Passes multiple arguments through to another routine.
- `clip_blitter_rectangle_geometry`: Calculates the intersection boundaries of a sub-rectangle within a larger clipping area.
- `compare_word_values`: Loads a word from an offset of a pointer into a register and compares it against a value in another structure before branching.
- `compare_and_branch_2`: Compares a word from a structure to another value and branches based on the comparison result.
- `mask_and_branch`: Performs a bitwise AND on a word from a structure and branches based on the result or a comparison with a value from another offset.
- `calculate_blitter_sub_rectangle_geometry`: Calculates the dimensions, bitmask values, and memory offsets for a series of sub-rectangles by dividing a primary area into smaller segments.
- `calculate_blitter_sub_rectangles_3`: Calculates coordinates and bitmask values for a series of sub-rectangles by dividing a primary area into smaller segments.
- `calculate_blitter_sub_rectangles_2`: Calculates a series of sub-rectangle coordinates and bitmask values by dividing a primary area into smaller segments.
- `calculate_blitter_sub_rectangles`: Calculates a series of sub-rectangle coordinates and bitmask values by dividing a primary area into smaller segments.
- `configure_blitter_area_sequence`: Calculates blitter register values and iterates through a bitmask to update hardware parameters for multiple drawing operations.
- `update_blitter_parameters`: Manipulates registers and memory to update a value based on an offset and performs a conditional branch.
- `update_blitter_registers`: Manipulates various hardware registers and performs calculations to update data through a loop.
- `branch_to_subroutine`: Performs a short relative branch to a location within another routine.
- `execute_structure_callbacks`: Resets a global pointer and executes a series of function pointers stored within a structure if their corresponding values are non-zero.
- `execute_callback_list_2`: Resets a global pointer and executes a series of function pointers stored within a structure.
- `initialize_game_state_structure`: Populates a structure by calling a sequence of initialization and clamping routines, then performs multiple conditional lookups to store specific values into a table.
- `process_event_data`: Checks a global flag, calls several functions to process parameters, and conditionally performs a write to a memory location.
- `execute_parameter_update`: Checks a global flag and, if a retrieved value is non-zero, performs a series of function calls involving provided parameters and a memory address.
- `load_and_parse_asset_resource`: Opens a data file, validates a specific magic number header, and reads a structured block of metadata into a provided buffer.
- `parse_and_process_data_block`: Extracts header information from a data structure, performs bitwise alignment and arithmetic on length and count fields, and iterates through a nested loop to process individual entries.
- `process_nested_data_blocks`: Iterates through a list of data entries, performing conditional decrements based on bitmask values and populating a destination table.
- `process_data_block_list`: Iterates through a list of data entries, copying values into a destination table and performing conditional cleanup or recursive processing based on the values found.
- `fill_table_with_data`: Iterates through a loop to store values into a table based on nibble extraction from a source value.
- `fill_table_with_default`: Iterates through a loop to store a default value into a table based on nibble extraction from a source value.
- `fill_table_with_indexed_data`: Iterates through a loop to store data from an indexed table into another table based on nibble extraction.
- `decrement_and_branch`: Decrements a register and branches based on whether the value reaches zero at specific thresholds.
- `update_table_pointer`: Calculates an offset based on a value from the stack, updates a pointer in a table, and increments a local counter.
- `call_external_routine`: Pushes a value from the stack, calls a subroutine via a pointer, and then cleans up the stack.
- `iterate_and_call_table_entries`: Clears a word in a table and then iterates through up to eight entries in a list, calling a function for each non-zero entry found.
- `expand_compressed_data_blocks`: Decompresses data from a source buffer into a destination memory area using a nested loop structure.
- `loop_and_branch`: Performs a loop based on a counter and then executes a short relative branch.
- `copy_nested_data_blocks`: Performs a multi-level nested loop to copy bytes from a source to a destination.
- `execute_loop_branch`: Decrements a counter register and branches to a location that may restart the loop.
- `return_null_pointer`: The routine returns a value stored on the stack, which is effectively a null or zeroed pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_conditional_memory_updates`: Increments a global counter and conditionally calls various routines to copy data to different memory locations based on the state of several short-integer flags.
- `update_conditional_state`: Checks a value from a library call and, if not equal to negative one, updates several fields in a structure and calls another routine with a specific argument.
- `reset_global_pointer`: Calls a function with a constant and a pointer, then calls another routine and sets a global variable to a maximum value.
- `initialize_gameport_input_state`: Allocates and initializes a structure to manage input device states by querying the gameport device.
- `execute_indirect_calls_2`: Calls a routine with several arguments and then executes two consecutive indirect function calls via a pointer located at an offset from a register.
- `clear_and_refresh`: Sets a word in a global location to zero and calls another routine twice.
- `remove_linked_node`: Traverses a linked list to find a specific value and removes that node by updating the preceding pointer.
- `swap_linked_list_nodes`: Traverses a linked list to find a specific node and swaps its position by updating pointers between the list and a provided destination.
- `replace_linked_list_node`: Traverses a linked list and replaces a specific node with a new pointer by swapping values between memory locations.
- `swap_two_values`: Copies the value from one memory location to another and then attempts to assign a pointer to the original location.
- `configure_blitter_area_parameters`: Calculates and sets blitter control registers based on dimensions and bitplane data extracted from a source structure.
- `configure_blitter_graphics_parameters`: Sets multiple blitter control and mode registers based on values extracted from a data structure.
- `update_blitter_registers_4`: Moves a word to a blitter register, loads a pointer from a table, and conditionally updates a blitter register and a bit in a register.
- `update_blitter_registers_3`: Copies a word to a blitter register, loads a pointer from a table, and updates another blitter register if a condition is met.
- `configure_blitter_parameters`: Calculates various values based on data in a structure and updates several Blitter hardware registers and global variables.
- `update_blitter_registers_2`: Updates the blitter control and data registers using values retrieved from a pointer stored in a structure.
- `update_blt_registers`: Updates several Blitter hardware registers and modifies a global value based on data from a pointer.
- `compare_offset_bounds`: Compares the sums of two word-sized offsets from two different structures to determine if they overlap.
- `compare_offset_sum`: Calculates the sum of two word-sized offsets from a pointer minus one and compares the result against a value in another register.
- `compare_offset_sum_against_limit`: Calculates a value by adding two word-sized offsets from a pointer and subtracting one, then compares this result against a value stored at another pointer.
- `compare_calculated_value`: Calculates a value by adding two offsets from a pointer and subtracting one, then compares the result against a value in another memory location.
- `store_and_branch`: Stores two registers into memory locations relative to a pointer and performs a short branch.
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
- `compare_converted_strings`: Iterates through a string, converting characters and comparing them until a difference is found, then returns the difference of two converted values.
- `subtract_converted_values`: Calls a routine to convert two values and returns the difference between them.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_immediately_3`: This routine is empty and immediately returns to its caller.
- `call_external_function`: Pushes two arguments onto the stack, calls a routine via a pointer in a register, and stores the result in a local stack frame.
- `update_node_and_recurse`: Increments a value retrieved from a pointer and stores the result in a structure, then either calls a routine with a constant or calls another routine recursively and populates the structure with several constant values.
- `modify_linked_list_data`: Performs a recursive operation on a linked list and writes several constant values into a structure pointed to by a node.
- `traverse_linked_list_3`: Iterates through a linked list of pointers, comparing strings and branching based on the results of external calls.
- `update_pointer_and_branch`: Loads a pointer from an offset of a base register into a register, writes the value pointed to by that register into a different offset of the same base register, and then performs a short branch.
- `call_external_via_stack`: Pushes a value from a stack frame onto the stack, calls a routine via a pointer, and then adjusts the stack pointer before branching.
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
- `allocate_and_initialize_node_tree`: Allocates memory for a new data structure and recursively populates its fields by copying parameters and calculating coordinate offsets from global tables.
- `traverse_linked_list`: Recursively traverses a linked list structure, calling a routine on each node and its children, and performing memory deallocation on specific elements.
- `search_and_store_pointers`: Iterates through a list of values to find matches for a descending sequence of numbers, storing the resulting pointers in a destination table.
- `search_conditional_table`: Searches a table for a value based on a parameter, potentially performing a second search in a different table if a specific bit is set in a byte within a structure.
- `update_blt_parameters_from_table`: Iterates through a list of pointers to update a structure with values derived from registers and dereferenced memory.
- `execute_and_store_result`: Calls another routine and then stores a value from a pointer into a destination table based on the result of a comparison.
- `update_table_values`: Iterates through a list of pointers, updating a structure with values from registers and pointer-dereferenced values for each entry in a loop.
- `open_file_handle`: Attempts to open a file using provided parameters, stores the resulting handle in a global variable, and calls another routine if the handle is null.
- `handle_file_error`: Checks if a global file handle is non-zero and if a parameter is non-zero, then attempts to perform an operation and closes the file handle if an error occurs.
- `execute_and_clear_callback`: Checks if a value at a memory location is non-zero, then calls a function pointer stored at another location using that value as an argument before clearing the original memory location.
- `load_and_allocate_data_structures`: Allocates a memory block based on combined counts from two sources and populates it by iterating through lists of pointers.
- `extend_byte_to_long`: Loads a byte from a stack offset, extends it to a long integer, and branches to another location.
- `process_linked_list_nodes_2`: Recursively traverses a linked list, updates several fields in each node, and clears specific members of a structure if a condition is met.
- `pass_pointer_to_function`: Retrieves a pointer from a global structure and an integer from a register to pass them as arguments to another routine.
- `recurse_unconditionally`: The routine immediately calls itself, creating an infinite recursion loop.
- `call_extend_byte_to_long`: Calls a routine that extends a byte from a stack offset into a long integer and branches to another routine.
- `set_global_flag_via_pointer`: Sets a single byte in a global memory location to one using a pointer retrieved from a data table.
- `decrement_and_test_value`: Decrements a word at a location pointed to by a table entry and clears a memory location if the resulting value is not positive.
- `iterate_and_compare_values`: Increments a value in a loop while comparing it against a limit from a table, then returns a boolean based on the final state.
- `compare_and_branch`: Compares a word from a table with a value in a register and branches if they do not match or if another specific word is non-zero.
- `clear_d0_and_branch`: Clears register d0 and branches to another location.
- `traverse_and_update_linked_list`: Iterates through a linked list, performing conditional routine calls and updating several coordinate fields within each node using arithmetic additions.
- `push_calculated_values`: Calculates two values by adding offsets from a global structure to values from a table and pushes them onto the stack before calling another routine.
- `update_list_coordinates`: Iterates through a linked list while performing arithmetic additions on several fields within each node and calling a routine when the end of the list is reached.
- `traverse_and_update_list`: Iterates through a linked list while updating several values in each node and calling another routine until a null pointer is reached.
- `get_register_value`: Calls a routine and returns a word from a specific memory location.
- `increment_counter`: Increments a value in a global variable if it is not zero.
- `decrement_counter`: Decrements a global variable and resets it to zero if it becomes negative.
- `calculate_and_draw`: Calculates two short values based on input parameters and a table, then calls a routine to perform a drawing operation using those values.
- `insert_sorted_node`: Allocates a new structure, populates it with data from a parameter, and inserts it into a linked list in ascending order based on a short value.
- `insert_into_sorted_list`: Populates a new structure with data from various sources and inserts it into a linked list based on a numerical comparison.
- `update_indirect_pointer`: Copies a longword from a source address into a destination address, then stores that same destination address into a pointer table.
- `store_pointer_to_table`: Stores a pointer from a register into a table and then stores that same pointer into a location pointed to by another table entry.
- `copy_pointer_to_table`: Copies a value from a register into a location pointed to by a table entry and then branches.
- `verify_structure_data`: Compares a byte from the stack with a value in a register and checks if a long value in a structure matches another long value.
- `compare_and_return_pointer`: Compares a byte in a structure to a register and returns a pointer if they are not equal.
- `remove_from_linked_list`: Traverses a linked list to find a node matching specific criteria and then removes it by updating pointers and freeing the memory.
- `traverse_and_free_list`: Recursively traverses a linked list and calls a routine to free each node.
- `parse_asset_header`: Calls a routine to load an asset, then checks the returned value against a specific constant before returning.
- `set_d0_and_branch`: Sets register d0 to one and branches to another location.
- `zero_d0_and_branch`: Clears register d0 and performs a short branch to another routine.
- `process_data_table`: Iterates through a table of eight entries, performing a calculation with values from the table and calling another routine before clearing the entries.
- `update_node_value_via_function`: Calls another routine with two pointers as arguments and stores the returned value into a memory location.
- `increment_and_update_node`: Retrieves a value from a structure, increments it via a function call, and stores the result back into the structure.
- `update_list_node_status`: Calls another routine to manipulate a linked list and sets a word in a node to one.
- `search_linked_list`: Traverses a linked list of pointers while performing string comparisons and calling external routines via stack-based execution.
- `swap_pointer_and_branch`: Loads a pointer from a structure, moves the value at that pointer into a new location within the structure, and branches to another routine.
- `call_external_via_pointer`: Pushes a value from a stack frame onto the stack, calls a routine via a pointer, and then branches to another location.
- `clear_and_test_bit`: Clears a specific bit in a byte at an offset from a pointer and then tests a longword at a different offset from that same pointer.
- `check_pointer_offset`: Loads two pointers from a structure and branches if a value at a specific offset is zero.
- `update_blitter_parameters_2`: Calls a routine to calculate parameters and stores the resulting value into a specific offset of a structure.
- `clear_and_return_pointer`: Loads a pointer from a table, clears a longword at an offset from that pointer, and returns the pointer in a register.
- `test_pointer_and_branch`: Tests a longword at an offset from a pointer and branches if it is zero.
- `execute_conditional_callback_7`: Pushes two pointers onto the stack and calls a subroutine.
- `execute_conditional_callbacks`: Pushes two longwords onto the stack, calls a subroutine, stores the result in a memory location, and then conditionally calls another subroutine based on whether that result was non-zero.
- `copy_and_test_bit`: Copies a longword from a memory location to a destination pointer and tests a specific bit in a byte at an offset from that pointer.
- `execute_callback_and_clear`: Checks if a long word at an offset from a pointer is non-zero, calls a routine using that value, and then clears the original value.
- `process_object_callback`: Checks if a parameter is non-zero, then executes a function pointer at a specific offset from a base address, conditionally executes another function pointer using a value from an offset within the parameter, and finally returns the result of a third function call using the parameter and a constant.
- `clear_pointer_after_call`: Checks if a value at a specific offset in a structure is non-zero, executes the function pointed to by that value, and then clears the pointer.
- `execute_callback_chain_2`: Calls a routine with a parameter, then executes two function pointers stored within a structure using values derived from a third function call.
- `traverse_and_execute_list`: Iterates through a linked list of pointers, executing a routine via a pointer found within each node.
- `initialize_animation_resources`: Attempts to allocate and initialize animation bitmaps, views, and rastports by recursively processing a data structure.
- `swap_and_call_pointers`: Swaps two pairs of longwords within a structure and then executes a series of function calls using pointers retrieved from a table.
- `execute_library_calls`: Calls three different routines using pointers retrieved from global variables.
- `decrement_and_recurse`: Decrements a short value in a global structure and calls itself recursively if the value reaches zero.
- `traverse_and_clear_list`: Recursively traverses a linked list, calling a routine on each node, performing specific memory writes, and clearing several pointers and values in a structure.
- `process_pointer_chain_2`: Checks a value at an offset from a pointer, calls another routine if it is non-zero, clears a bit in a structure, and updates the pointer to the next element in a chain.
- `process_pointer_chain`: Checks a value at an offset from a pointer and, if non-zero, clears a bit in a structure and updates the pointer to the next element in a chain.
- `execute_and_unlk_a5`: Calls a routine, tests a word in a register, and if non-zero, unloads the stack pointer register before returning.
- `reset_and_loop_data`: Clears several global values, enters a loop that performs bitwise shifts and calls a routine, and eventually clears a pointer and calls another routine.
- `execute_and_clear_pointer`: Pushes a value from a structure onto the stack, calls a routine, clears a value in the structure, and calls another routine before branching.
- `update_state_from_linked_list`: Iterates through a linked list to update global state variables and counters based on node type flags.
- `process_linked_list_data`: Iterates through a linked list to update values in a structure based on data within the list nodes and conditional flags.
- `update_structure_from_list`: Traverses a linked list to update specific fields in a structure using values extracted from the list nodes.
- `initialize_input_structure`: Performs a series of checks and assignments involving library calls and structure fields to set a status flag and update memory values.
- `reset_structure_fields_2`: Checks a short value at an offset from a pointer, and if non-zero, writes a constant to a structure field and updates another field with a calculated value before calling another routine twice.
- `allocate_and_fill_array`: Allocates a memory block and then fills an array within that block by calling a routine for each element.
- `execute_indirect_calls`: Retrieves two function pointers from a table using an offset from a base address and executes them with provided arguments.
- `initialize_nested_structures`: Clears several fields in a structure, then attempts to allocate and initialize multiple nested structures through a series of conditional library calls, returning one if successful or zero otherwise.
- `reset_structure_and_call_routines`: Conditionally calls other routines using pointers from a structure and then clears multiple fields within that structure to zero.
- `update_state_from_table`: Performs a series of checks and updates a structure by comparing values from a table and copying data from a source address to a destination buffer.
- `search_and_allocate_buffer`: Calls a routine to obtain a value, then conditionally allocates a memory block via a library call and populates it with a value from another routine if specific criteria are met.
- `execute_recursive_call`: This routine performs a direct recursive call to itself using the provided parameters before returning.
- `update_file_position`: Calculates a new position value based on a table of offsets and updates a parameter, or calls another routine if certain global conditions are met.
- `copy_memory_with_alignment`: Calculates the size of a memory difference between two locations and copies the data in chunks.
- `call_data_transfer_routine`: Calls a routine that transfers data in fixed-size chunks after calculating a length from a table.
- `branch_on_structure_flags`: Checks two different word values at offsets from a pointer and branches to different locations based on whether those values are zero or non-zero.
- `load_and_branch_short`: Loads a longword from an offset of a pointer in a register into a data register and performs a short branch to another location.
- `close_file_handle`: Calls a routine if certain conditions are met, or calls another routine with a specific value if a pointer is non-zero.
- `set_library_parameters`: Writes specific values to a structure and then calls a routine using a pointer from a global variable.
- `reset_library_state`: If a global condition is met, it writes a specific value to a structure and calls a routine, then writes different values to the same structure and calls the routine again.
- `copy_and_uppercase_string`: Copies up to four bytes from a source string to a local buffer while converting lowercase ASCII characters to uppercase.
- `search_and_execute_callback`: Searches through a table for a matching value and, if found, executes a sequence of routines and updates structure flags.
- `iterate_and_update_blocks`: Checks a global value to determine a starting address and then iterates up to eight times, calling another routine and a library function with incrementing pointers.
- `validate_and_update_state`: Compares two values at a specific offset within a structure and, if they differ, updates several fields in a structure pointed to by a global variable before calling another routine.
- `call_search_routine`: Calls another routine passing a value and a length derived from a global variable.
- `search_integer_array`: Iterates backward through an array of long values to check if a specific value exists within the specified count.
- `store_pointer_and_call_parser`: Stores a pointer into a global variable, calls another routine with two parameters, and then stores zero at the same global variable location.
- `increment_and_store_byte`: Increments a global counter and stores a byte from a register into a location pointed to by a global pointer.
- `parse_format_and_update_status`: Parses a format string to convert values into a target buffer and calls a routine to update status structures.
- `call_recursive_routine`: A routine consisting of 48 instructions that calls another routine and returns.
- `parse_printf_format_string`: Parses a format string to extract and convert numeric, floating-point, or string values into a target buffer.
- `store_short_value`: Stores the word value eight into a memory location relative to a pointer in a register and then performs a short branch.
- `store_short_value_alt`: Stores the word value sixteen into a memory location relative to a pointer in a register and then performs a short branch.
- `parse_formatted_string`: Parses a string containing numeric values and format specifiers, using a recursive approach to process characters.
- `format_floating_point_string`: Converts a numeric value into a formatted string representation, handling sign characters, decimal points, and scientific notation.
- `loop_and_update_structure`: Repeatedly calls another routine and updates a value in a structure within a loop.
- `modify_structure_value`: Compares a word in a structure to a constant and, if certain conditions are met, performs arithmetic and updates values on the stack.
- `compare_and_modify_value`: Compares a value at an offset from a register against a constant and performs arithmetic operations on a value stored on the stack based on the result.
- `format_scientific_float_string`: Converts a floating-point value into a string representation using scientific notation.
- `clear_offset_word`: Clears a word at an offset from a pointer and branches to another location.
- `increment_and_store_word`: Loads a word from a memory location, increments it, stores the result back to a different offset relative to a pointer, and then performs a short branch.
- `format_numeric_string_2`: Converts a numeric value into a string representation, handling signs, decimal points, and scientific notation by writing characters to a destination.
- `format_numeric_string`: Converts a numeric value into a string representation, handling signs, decimal points, and scientific notation by writing characters to a destination buffer.
- `append_sign_and_character`: Tests a word value, branches, and then writes specific characters to a memory location pointed to by a register, potentially handling a negative sign.
- `convert_integer_to_string`: Converts a 16-bit integer value into a null-terminated ASCII string representation.
- `execute_callback_via_structure_2`: Checks a global flag and, if zero, executes a series of routines through pointers before calling a routine using an offset and a value from a structure.
- `dispatch_callback_via_structure`: Checks a global flag to determine if a specific routine needs to be initialized or updated.
- `dispatch_callback_via_structure_2`: Checks a global flag to determine if a specific routine needs to be initialized or loaded.
- `update_callback_state`: Checks a global flag to determine if a state-dependent routine must be initialized.
- `dispatch_callback_via_structure_3`: Checks a global state and, if uninitialized, populates a structure by calling a routine and updating a cached value.
- `execute_conditional_callback_6`: Checks a value in a global structure and, if zero, initializes a data block and executes a sequence of function pointers.
- `dispatch_callback_via_structure_4`: Checks a status flag in a memory location and, if zero, initializes a structure and executes a routine through a pointer.
- `execute_callback_via_structure_3`: Checks a global flag and executes a routine through a pointer using an offset derived from a structure.
- `dispatch_callback_via_structure_5`: Checks a global flag and, if not set, initializes a structure and calls a routine to populate it.
- `execute_callback_via_structure`: Checks a global flag, potentially calls a function through a pointer, and eventually executes a routine using an offset derived from a structure.
- `skip_numeric_delimiters`: Checks a byte at a pointer to see if it is a space or a digit, incrementing the pointer if it is a digit.
- `skip_delimiter_character_2`: Checks if a byte at a pointer matches a specific value and increments the pointer if it does.
- `skip_delimiter_character`: Compares a byte at a pointer to a specific value and, if it matches, increments the pointer and clears a register before branching.
- `convert_string_to_integer`: Parses a string of numeric characters into a signed integer value.
- `return_immediately_4`: The routine executes four instructions and returns without performing any observable work.
- `save_registers_and_branch`: Saves several registers to the stack and branches to another location.
- `fill_memory_backward`: Fills a memory range with a byte value by iterating backward from a starting pointer.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `traverse_and_modify_linked_structures`: Validates a header, traverses a list of pointers, and for each entry, performs memory operations involving pointer redirection and calls a routine via a global pointer.
- `compare_and_decrement_pointer`: Compares a byte from a pointer to a value in a register and, if they match, returns a decremented version of that pointer.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_memory_and_call_library`: Fills a memory range with zeros and calls functions from a library based on a global pointer.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tokenize_string_into_array`: Parses a string by skipping whitespace and extracting individual tokens, handling both quoted substrings and space-delimited words.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `skip_table_entry`: Loads a pointer from the stack, compares the word at that pointer to a constant, and either branches or increments the pointer by four before branching.
- `update_table_entries`: Iterates through a table to find non-zero values and updates a secondary list by writing specific values and incrementing pointers.
- `manage_library_resources`: Interacts with library structures to check status, execute functions, and update a pointer in a table.
- `initialize_register_and_branch`: Loads a specific value into a register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_library_function_4`: Retrieves a pointer from a structure and calls a library function, then stores the result in a global variable.
- `process_window_data`: Retrieves data from nested pointers and calls library functions to update a structure.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Checks the signs of two registers, negates them if necessary, and calls another routine to perform division.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `call_update_status_structures`: Calls a routine that updates a data structure based on conditional logic.
- `update_status_structures`: This routine calls two separate subroutines that update a data structure based on conditional logic.
- `update_status_structure_3`: A routine that calls another routine to update a structure based on conditional logic.
- `traverse_linked_list_2`: Iterates through a list of structures by incrementing a pointer and calling another routine for each entry.
- `traverse_and_clear_structure`: Recursively traverses a structure using bit flags to determine further traversal, calls a function via a pointer, and clears the structure fields upon completion.
- `update_status_structure_2`: Iterates through a structure and updates its fields based on conditional logic and external routine results.
- `clear_structure_entries`: Iterates through a list of structures and clears three consecutive longwords in any entry where a specific byte is zero.
- `branch_to_offset_2`: Performs a short branch to a specific instruction offset.
- `clear_and_branch`: Clears three consecutive longwords starting at a pointer in a register and then performs a short branch.
- `update_status_structure`: Checks an interactive status and updates various fields in a structure based on the result of two different routine calls.
- `iterate_linked_list_and_call_function`: Traverses a linked list starting from a pointer stored in a global variable, passing each node and a value derived from the node's second word to another routine.
- `insert_into_linked_list`: Allocates a new node via a function call and inserts it into a linked list by updating pointers and a global head pointer.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: Searches for a specific node in a linked list and removes it by updating the pointers of the preceding node, then calls another routine with the removed node.
- `check_interactive_status`: Validates an index against a table and calls a routine to determine if a specific resource is interactive.
- `attempt_file_deletion`: Attempts to delete a file and, if the operation fails, calls another routine to retrieve an error code and stores that code into a global variable.
- `perform_conditional_io_write`: This routine executes a conditional write operation and handles potential input/output errors.
- `execute_conditional_write`: Calls a routine with specific parameters and, if a bit is set in the result, conditionally calls another routine based on a global value.
- `execute_write_operation`: Calls a function to write four bytes from a specific memory location and then calls another routine to handle the result.
- `execute_callback_and_routine`: Checks if a global function pointer is non-zero to execute it, then calls another routine.
- `clear_table_entry`: Validates an index against a table size and, if valid, clears a longword in the table and potentially calls another routine recursively.
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
- `allocate_and_initialize_structure`: Allocates a memory block using a library function and populates several fields within the new structure before returning it.
- `reset_structure_fields`: Sets specific bytes and longwords in a structure to fixed values and calls another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: Calls another routine passing the input parameter and a fixed constant value of 48.
- `destroy_display_entry_2`: Calls another routine passing the input pointer as an argument.
- `initialize_structure_with_data`: Allocates a memory block and assigns three specific values to its fields based on the provided arguments.
- `reset_and_call_routine`: Sets three specific fields in a structure to fixed values and then calls another routine using a value from the structure.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `populate_and_link_structure`: Copies a block of data from a global table to the stack, allocates a structure, and populates its fields using the copied data and arguments.
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

### TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf:i.c.g.

- `find_max_and_populate_buffer_2`: Traverses a linked list to identify the maximum value in a specific field and then initializes a data buffer with a header and a sequence of incrementing values.
- `transfer_register_and_branch`: Moves the contents of one data register to another and then performs a short relative branch to another location.

### 8

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### 7

- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `return_zero_2`: This routine is empty and returns zero.

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
- `manage_linked_list_node`: Library code that checks a status field in a structure; if zero, it performs a linked list insertion by updating pointers and calling a function via a global pointer, otherwise it sets a specific bit in a byte within the structure.
- `set_memory_bit`: Library code that sets a specific bit in a byte located at an offset from a register.
- `increment_and_call_library`: Library code that checks a bit in memory, increments a short value at an offset if the bit is clear, and calls another library function if the value reaches one.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call_2`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `update_accumulator_bitfield`: Doubles the value of an accumulator and updates a corresponding bitfield based on a calculated difference.
- `calculate_bitfield_difference`: Library code that performs a loop to calculate a value by repeatedly doubling an accumulator and updating a bitfield based on a running difference.
- `adjust_bitfield_value`: Library code that sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `execute_masked_call`: Library code that moves a value to a register, tests it, calls a routine, and then modifies specific bits in a register before branching.
- `call_jump_table_entries`: Library code that calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_register_and_branch_3`: Library code that clears a long register and performs a short branch to another location.
- `execute_library_callback_chain`: Library code that performs a complex sequence of indirect function calls using offsets from a base pointer and applies bitwise masking to a return value.
- `execute_conditional_callback_chain`: Performs a sequence of indirect function calls based on the values of specific registers and conditional logic.
- `execute_conditional_library_calls_2`: Library code that performs multiple conditional calls to routines via pointers stored at various offsets from a base register.
- `execute_conditional_library_calls`: Library code that calls a routine via an offset from a base register and performs conditional branching based on the result.
- `call_and_store_result`: Library code that calls a routine using an offset from a base register and stores the returned value back into a register.
- `branch_on_register_test`: Library code that moves a value to a register, tests a long value, and either branches or loads a constant into a register.
- `mask_return_value`: Library code that calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_integer_approximation`: Library code that performs bit-shifting and iterative comparisons to approximate a value.
- `shift_and_initialize_registers`: Library code that performs a logical shift right on a longword in a data register and loads several registers with specific constant values before branching to another location.
- `execute_loop_with_branching`: Library code that executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `library_push_and_branch`: Library code that pushes a word onto the stack and branches to another location.
- `push_all_ones_and_branch`: Pushes a word containing all bits set to one onto the stack and performs a short relative branch.
- `push_constant_and_branch`: Pushes a word containing the value one onto the stack and performs a short branch to another location.
- `compare_and_return`: Library code compares a byte in a register to a constant, branches if higher, and returns after adjusting the stack pointer and testing a byte.
- `calculate_math_offset`: Library code that performs complex arithmetic and bitwise operations on a value to determine an index or offset for a table lookup.
- `perform_complex_math_loop_3`: Library code that performs a complex loop involving bitwise shifts, arithmetic updates, and table lookups to process values.
- `process_arithmetic_table_loop`: Iterates through a table of values using a loop that performs bitwise shifts and arithmetic updates on two registers.
- `execute_complex_math_loop`: Library code that performs a loop involving arithmetic operations, bitwise shifts, and pointer increments using a table.
- `perform_complex_math_loop_2`: Library code that performs a loop involving arithmetic operations, bitwise shifts, and recursive calls based on a value in a register and data from a pointer.
- `load_constant_and_branch`: Library code loads a constant value into a register and then performs a short branch.
- `recursive_call_wrapper`: Calls itself and then returns, with no other operations.
- `save_and_branch_library`: Library code that pushes a longword onto the stack, calls another routine, and then restores a value from the stack.
- `compare_and_branch_register`: Library code that moves a long value to a register, performs a conditional branch if the value is negative, compares the value to a constant, and branches based on the unsigned comparison result.
- `shift_and_jump`: Library code performs an arithmetic shift right on a register, increments it, and then either branches or jumps to a zero address.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_recursive_bit_manipulation_2`: Library code performs recursive calls involving bitwise rotations and conditional branches based on a byte value.
- `rotate_and_branch`: Library code loads a constant into a register, performs a rotate right through the carry bit, and then performs a short branch.
- `load_and_branch`: Library code loads a constant into a register and then performs a short branch.
- `test_and_branch_register`: This library code loads a word from the stack into a register, tests if it is positive, and branches accordingly.
- `empty_stub_2`: The library code contains five instructions and performs no observable work; it ends immediately.
- `return_immediately`: This library routine consists of four instructions and performs no observable work, immediately returning to its caller.
- `execute_recursive_bit_manipulation`: This library routine performs a series of recursive calls while applying bitwise shifts and sign-extension logic to a value derived from a register.
- `execute_recursive_calls`: Library code that performs a series of recursive calls to itself before returning.
- `accumulate_shifted_values_from_table`: Library code that performs arithmetic shifts on two registers, adds the results to two other registers, and enters a loop that subtracts values from a table and updates a counter.
- `subtract_table_from_accumulator`: The routine walks through a table pointed to by a pointer and repeatedly adds the current table entry to an accumulator, then subtracts subsequent entries while a counter is decremented.
- `test_and_branch_library`: Library code that tests a byte, branches based on its sign, and performs bitwise operations before returning.
- `load_and_branch_immediate`: This library code loads a constant value into a register and performs an unconditional branch.
- `perform_complex_math_loop`: Library code that executes a loop involving bit-shifting, subtractions, and conditional calls to process values.
- `complex_loop_with_branching`: Library code that executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_bit`: Library code that tests a byte, conditionally masks it, and branches to another routine.
- `process_conditional_bit_shifts`: Library code that performs multiple routine calls and applies bitwise shifts and sign extensions to a value based on several conditional checks.
- `empty_library_stub`: The library code contains no instructions and performs no operation.
- `transform_data_register_bits`: Library code that performs a swap, a long word rotation, a bitwise XOR, and an addition on a data register, followed by conditional branches based on the overflow flag to perform further arithmetic and bitwise transformations.
- `compare_and_modify_byte`: This library code compares a byte against two specific constants and performs arithmetic and bitwise rotations on it based on the results.
- `mask_and_test_word`: This library code performs a bitwise AND on a word, tests the result for zero, and branches based on the outcome.
- `test_and_shift_byte`: Library code that compares a byte to a constant, performs logical shifts on a long value, and uses conditional branches based on the results of those shifts.
- `shift_and_set_flags`: This library code performs a logical shift left on a word, rotates a bit into a register, and sets a specific bit in the status register.
- `clear_register_and_branch`: This library code clears a register and performs an unconditional branch.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `invoke_callback_indirect`: Library code that calls a routine at an address calculated by adding a negative offset to a global pointer and returns the original value from a register.
- `invoke_callback_and_return_2`: Library code that receives a pointer in a register and a value in another, calls the routine pointed to, and returns the original value.
- `execute_indirect_callback`: This routine calls another routine and returns the original value held in a register.
- `invoke_callback_approximation`: Library code that calls another routine and returns a value from a register.
- `library_callback_wrapper`: Library code that calls a routine and returns the original value in a register.
- `invoke_callback_with_one`: This library code invokes another routine and then pushes the value one onto the stack before branching.
- `invoke_callback_with_all_ones`: This library code invokes another routine and then pushes a word with all bits set to one onto the stack before branching.
- `store_register_via_pointer`: This library code invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register.
- `call_subroutine_and_return`: Calls a routine via a pointer and returns the original value held in a register.
- `invoke_callback_repeatedly`: Library code that calls a routine via a pointer and may call it again based on a condition.
- `execute_recursive_call_2`: The routine performs a recursive call to itself.
- `execute_recursive_callback`: This routine performs a recursive call to another routine before returning the original value held in a data register.
- `execute_callback_routine`: Calls a routine via a pointer passed in a register and returns the original value from a data register unchanged.
- `invoke_callback_thrice`: Library code that calls a routine via a pointer three times in succession.
- `execute_callback_and_return`: Calls a function pointer provided in a register and returns the resulting value from a data register.
- `invoke_callback_empty`: Library code that calls a routine through a pointer in a register and returns the original value.
- `execute_callback_routine_2`: Invokes a subroutine via a pointer passed in a data register and returns the original value of that register.
- `call_indirect_offset_one`: Library code that calls a routine at an address calculated by adding a negative offset to a global pointer.
- `call_indirect_offset_two`: Library code that calls a routine at an address calculated by adding a different negative offset to a global pointer.
- `load_address_and_branch`: Library code that loads a specific address into a register and performs a short branch.
- `load_address_and_branch_alt`: Library code that loads a different specific address into a register and performs a short branch.
- `call_indirect_offset`: Library code calls a routine at an address calculated by adding an offset to a pointer stored in a global variable.
- `execute_indirect_call_2`: Calculates an address by adding an offset to a value retrieved from a global variable and jumps to that location.
- `load_address_and_branch_2`: Library code loads a specific address into a register and then performs a short branch.
- `load_address_and_jump`: Loads a memory address into a register and performs a short branch to another routine.
- `load_address_and_branch_3`: Loads a constant address into a register and performs a short branch to another location in the library.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: The library code executes a single illegal instruction.
- `libInit2`: The library code stores the value from A0 into a global variable.
- `MIExpunge`: This library code checks a flag in a structure; if zero, it updates a linked list and calls a function pointer before returning a segment list.
- `Exp_InUse`: This library routine sets the eighth bit of a byte located at an offset from a pointer held in an address register.
- `MIOpen`: This library code tests a bit in a structure and, if not set, increments a value at an offset and returns the base pointer.
- `openFail`: This library routine returns a value representing a failure.
- `MIClose`: Library code that decrements a short counter stored in a structure and, if the counter reaches zero and a specific flag bit is set, calls MIExpunge and returns its result.
- `DPFix`: This library routine performs bitwise manipulation on a value pointed to by a register to adjust its precision or scale.
- `dfi4`: This library code tests a specific bit in a register and branches to another location, otherwise it clears a register and branches.
- `dfi6`: This library code clears a long register and performs an unconditional branch.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: This library routine converts a signed single-precision value in a register into a double-precision representation stored through a pointer.
- `DPAbs`: This library routine copies the high word of a double from a source pointer to a destination pointer and writes the low word with its sign bit cleared.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: This library routine subtracts one from the value held in a data register.
- `dtst_2`: This library routine adds one to the value held in a data register.
- `cc_1`: This library routine calls a function whose address is passed in an address register and then returns.
- `cc_2`: This library routine calls a function whose address is passed in an address register and then returns the contents of a stack slot.
- `cc_3`: This library routine calls a function pointer held in register A1 and returns the value left on the stack at offset 18.
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
- `MIIEEEDPCeil`: Library code that calculates a ceiling value by negating a value, applying a floor function, and then negating the result.
- `MIIEEEDPFloor`: Library code that calculates the floor of a value by performing comparisons and conditional additions or subtractions.
- `test_d6_d7`: This library code moves values from registers into others, calls another routine, and then conditionally calls a second routine after restoring registers.
- `lt_zero`: This library routine stores a constant value into a stack slot and then tail calls another routine.
- `fptrap`: This library routine acts as a trap handler that dispatches floating-point exceptions or operations by looking up a handler in a jump table.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `dfsub`: Library code that saves registers to the stack, loads two pointers into address registers, and branches to another routine after modifying a value in memory.
- `dfadd`: Performs addition on two double-precision floating-point numbers stored in memory.
- `dfa11`: Performs subtraction on two double-precision floating-point numbers.
- `dfa14`: This library routine performs conditional logic to store a 64-bit value into a memory location pointed to by a register.
- `dfa13`: Library code that performs complex multi-word arithmetic involving bit shifts and stores the result in a table.
- `dfa5`: Library code that performs complex arithmetic and bitwise manipulations to store a result into a table via a pointer.
- `dfa8`: This library routine compares two longwords and performs arithmetic operations including subtraction and bit shifts on several registers.
- `dfa7`: This library code performs bitwise operations on several registers and stores the resulting values into memory locations pointed to by a register.
- `dfa9`: Library code that clears a long word in a register and a long word at an offset from that register before performing a short branch.
- `dfauo`: This library routine compares a longword against a constant and, if it is less than that constant, stores a modified value and a byte into a table pointed to by a register.
- `dfmul`: Performs multiplication of two double-precision floating-point numbers.
- `dfm4`: This library routine clears a longword in a register and a longword at the address pointed to by that register before performing a short branch.
- `dfmuo`: This library routine compares a longword against a constant and, if it is less than that constant, stores a modified value and a longword of all ones into a table pointed to by a register.
- `dfdiv`: Performs double-precision floating-point division using a bit-shifting iterative algorithm.
- `dfd10`: Executes a division operation on double-precision floating-point numbers.
- `dfd4`: This library routine performs arithmetic operations on values stored in registers and writes the resulting 64-bit result to a memory location pointed to by a register.
- `dfsetz`: This library routine clears two data registers to zero and then performs a short branch.
- `dfdbyz`: This library routine loads a 16-bit constant into a register and then performs a short branch to another location.
- `dfduo`: This library routine compares a longword against a constant and, if it is less than that constant, stores a word and a longword into a table pointed to by a register and updates the condition codes.
- `dfneg`: This library routine loads a word from a pointer, masks its lower bits, and if the result is non-zero, flips the sign bit of the byte at that pointer before branching to another routine.
- `dfcmp`: This library routine is empty.
- `dftst`: This library routine returns the 32-bit value that was passed on the stack.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### version.library

- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: The routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base pointer.
- `decrement_counter_and_call_3`: This library routine decrements a short counter at a fixed offset from a pointer and calls another routine if the counter reaches zero and a specific bit in a flag byte is set.
- `update_library_status_bit`: This routine saves registers and loads a pointer from a register to access a structure.
- `get_library_value`: As part lack of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `call_font_handler`: This library routine calls a routine via a table entry using a library base, and if the result is non-zero, it stores a value in a structure and calls another routine via a table entry.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: This library routine calls two functions via table entries, and if both return non-zero, it performs several memory updates to a structure including setting a pointer and an integer value.
- `unload_font_resources`: This library routine traverses a linked list to remove items based on a short value, updates a counter, and calls another routine to free resources if certain conditions are met.
- `update_library_status_flag`: Checks specific conditions within a library data structure to determine if a resource must be freed or if a status bit should be set.
- `set_bit_and_clear_register`: The routine sets a specific bit in a byte at an offset from a register and then clears a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `check_bit_and_jump`: The routine decrements a word at an offset from a register, performs conditional branches based on that result and a bit test, and finally jumps to an offset relative to a register.
- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `process_font_character_table_2`: Iterates through a table of font character structures to find the entry with the highest metric value.
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
- `increment_pointer_on_non_zero_byte`: This library routine checks a byte at a pointer location and increments that pointer if the byte is not zero.
- `call_other_routine`: This library routine calls another routine and then returns.
- `calculate_string_length`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_bytes_and_branch`: This library routine compares a byte from one memory location with a byte from another.
- `compare_byte_and_branch`: Moves a byte from a pointer into a register, decrements a value in another register, and compares the byte at the new pointer location with the register, branching based on the result.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_font_bytes`: This library routine loads a pointer and two bytes from memory, compares the bytes, and branches based on the result.
- `increment_pointer_and_branch`: This library routine increments a pointer by one and then performs an unconditional branch to another location.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations.
- `extract_font_string_metadata`: Iterates through a linked list of font resource structures to copy character string data into a destination buffer.
- `populate_font_string_buffer`: Iterates through a linked list of string structures to copy character data into a destination buffer.
- `copy_string_to_buffer_3`: Copies a sequence of bytes from a source memory location to a destination buffer.
- `parse_font_resource_data`: Iterates through a font resource stream to identify character metadata and string descriptors.
- `parse_font_resource_header_2`: Iterates through a data stream to identify specific font resource markers and extract character metadata.
- `parse_font_resource_header`: Iterates through a data stream to identify specific font resource markers and extract character metadata.
- `read_and_parse_font_data`: Reads data from a disk source and parses it into a structured buffer.
- `calculate_absolute_value`: This library routine calculates the absolute value of a signed integer.
- `return_zero_constant`: The routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 2 of 2).adf:bootblock

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
- `perform_masked_bit_tests`: Calls another routine four times, applying different bitwise masks to the returned value in each instance, and finally tests a bit in a register.
- `execute_repeated_subroutine`: Repeatedly calls another routine multiple times in succession.
- `update_memory_with_calls`: Calls another routine several times, performs a memory read, and then uses bitwise OR logic to update a byte at a specific memory location.
- `test_bit_after_call`: Calls another routine and then tests a specific bit in a register.
- `execute_conditional_stack_operations`: Calls another routine four times while performing bitwise XOR operations on a register, tests a bit in a register, and pushes bytes onto the stack from both a pointer and a table offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `perform_complex_stack_and_memory_updates`: Calls another routine four times, performs bitwise tests, moves a longword from a pointer to another pointer, compares a byte to a constant, and stores a longword into a memory location.
- `test_and_clear_bit`: Calls another routine and then tests a specific bit in a register before clearing that bit in the same register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_mask_byte`: Calls another routine and then masks the lower byte of a register.
- `modify_memory_and_registers`: Calls another routine, performs a bitwise AND on a value at a pointer, and updates two registers with bitwise OR operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_masked_byte`: Calls another routine, pushes a byte from a register onto a stack pointed to by another register, and clears a register.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reallocate_and_copy_data_structure`: Traverses a linked list to determine a maximum value, then reallocates a memory block and performs multiple block memory copies.
- `transfer_and_branch`: Moves a long value from one register to another and performs a short branch.
- `initialize_audio_playback_context`: Allocates memory for audio structures and initializes playback state flags and buffer pointers.
- `copy_d0_to_d5`: Copies the contents of data register zero to data register five.
- `cleanup_and_release_resources`: Iterates through a list of function pointers to execute them and then releases allocated memory blocks.
- `validate_resource_chain`: This routine performs a series of conditional checks by calling other routines and storing their return values into specific offsets of a state block.
- `return_immediately_2`: This routine performs no operations and returns to its caller immediately.
- `return_constant_value_2`: Returns a fixed integer value.
- `return_address_pointer`: Returns a pointer to another routine.
- `execute_subroutine_sequence`: Saves several registers to the stack and executes a sequence of six subroutine calls and a relative branch.
- `process_resource_loop`: Executes a series of subroutine calls within a loop that terminates based on a boolean condition and modifies a byte at an offset from a register.
- `call_library_function_directly`: Calls one library function on a base pointer held in the state block, then calls a helper routine.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_nested_callbacks`: Sets a field in a record to minus one, calls a function to obtain a pointer, and if successful executes a sequence of four nested function calls through that pointer before returning the stored value.
- `open_library_and_release`: Sets a result field to minus one, opens the dos.library name through the exec base, and when the open succeeds calls two methods on the returned base before closing it.
- `initialize_trackdisk_device_request`: Populates a device request record with a specific length and unit value before calling a library function to open the disk device.
- `store_two_library_results`: Calls two library functions and stores their results in the state record and a second record, one as a long and one as a byte.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls the library function at a specific offset from that base.
- `update_control_structure`: This routine copies data between memory locations and writes specific constant values to a structure pointed to by a register.
- `populate_record_data`: Writes a constant, a short value, and two values from another record into a target record.
- `populate_record_data_2`: Writes a constant, a short value, and two fields from another record into a target record.
- `load_control_word_eight_zero_zero_five`: Loads a specific word into a register and branches to another location.
- `load_constant_and_branch_2`: Loads a word constant into a data register and performs a short branch to another location.
- `load_constant_and_branch_4`: This routine loads a small constant into a data register and performs an unconditional short branch.
- `load_control_word_eight_zero_zero_nine`: Clears a register, loads a specific word into another register, and branches to another location.
- `update_record_fields`: Writes several values into a record, including a constant, two values from another record, and two calculated values scaled by a power of two.
- `register_disk_resource_handler`: Obtains the disk.resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `process_disk_resource_loop`: Repeatedly calls a function via a pointer in a record until it returns a non-zero value, then updates two global variables.
- `invoke_disk_resource_routine`: Calls one function on the disk resource base held in the state block.
- `configure_hardware_and_call_subroutine`: Sets specific values in custom chip registers, loads two pointers from a table, calls a subroutine via a pointer, and branches to another location.
- `branch_to_target`: Performs a short relative branch to another location.
- `update_flag_byte_bits`: Clears the bit selected by the first register value in a state byte, then sets bit two when the second register value is zero.
- `copy_state_byte_to_register`: Copies a state byte into a control word and writes a fixed flag value to a second control word.
- `start_disk_sync_transfer`: Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers.
- `set_top_bit_field`: Stores the incoming value with its top bit set into a 16-bit field of the record addressed by the second register, twice.
- `reset_record_and_call_library`: Sets a state word to a fixed value, sets a second word to two, and calls a library function on the base pointer.
- `count_consecutive_word_repeats`: Calculates the number of times a specific word value repeats consecutively within a memory range.
- `count_consecutive_values`: Iterates through a memory range to count how many times a specific word value repeats consecutively.
- `forward_to_helper_routine`: Calls one helper routine and returns with no other work.
- `compute_bitwise_checksum`: Computes a 16-bit checksum over a run of words by shifting each bit through the accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `return_without_action`: The body contains only a return, with no calls, memory access or arguments used.
- `check_status_and_jump`: Decrements a value in a register, performs conditional branches based on bit tests, and jumps to an offset from a base pointer.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `return_immediately_5`: The routine performs no functional operations and returns to its caller immediately.
- `reallocate_string_list_buffer`: Iterates through a linked list of string structures to calculate total required space and copies the string data into a new contiguous memory buffer.
- `reallocate_string_list_nodes`: Iterates through a linked list of string structures, calculating required buffer space and copying string data to new memory locations.
- `copy_and_reallocate_string`: Calculates the length of a null-terminated string and shifts data within a buffer.
- `parse_and_copy_font_string`: Validates a font handle and parses a null-terminated string to update a buffer and decrement a length counter.
- `calculate_and_copy_string`: Determines the length of a null-terminated string and copies its contents into a destination buffer.
- `parse_and_copy_string_data`: Calculates the length of a null-terminated string and copies its contents into a destination buffer.
- `copy_string_to_buffer_2`: Calculates the length of a null-terminated string and copies its contents into a destination buffer while decrementing a remaining length counter.
- `call_library_functions`: Calls three different library functions using offsets from a base address stored in a global variable and returns a value based on a comparison.
- `return_zero_constant_2`: Returns a zero value to the caller without performing any other operations or accessing memory.
- `compute_font_offset_2`: Calculates a signed offset by comparing two 16-bit values and performing a bitwise comparison of two byte arrays.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `skip_null_byte`: Checks a byte at a pointer; if it is not zero, it increments the pointer and branches.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length_2`: Iterates through a null-terminated string to count the number of characters before the terminator and returns the total count.
- `compare_bytes_in_table`: Compares a byte from one memory location to a byte in another, and if they match, checks if a subsequent byte is non-zero.
- `compare_and_set_byte`: Moves a byte from memory to a register, compares it to a value, and either branches or sets a register to a constant value.
- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward_2`: Calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle_2`: Checks the validity of a font handle by searching for two specific characters and returns the original handle if both are found.
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
- `return_immediately_6`: This routine performs no functional operations and returns to its caller immediately.
- `return_immediately_7`: This routine is empty and returns to its caller immediately.
- `nop_placeholder`: The routine is empty and performs no operations before returning.
- `modify_memory_fields`: Clears the high bit of a byte and increments a short value at specific offsets from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_buffer`: Allocates a memory block, populates a table with incrementing offsets based on a base address, and performs two function calls involving hardware parameters.
- `call_indirect_function`: Fetches a function pointer from a location calculated by adding an offset to a value in a register and calls it with a pointer and a constant value as arguments.
- `concatenate_strings`: Joins three source strings into a destination buffer, inserting a separator character if necessary.
- `execute_conditional_callback`: Checks a function pointer in a global structure and either calls it or calls another routine with the structure as an argument.
- `execute_conditional_callback_2`: Checks if a function pointer stored in a table is null.
- `execute_conditional_callback_3`: Checks if a function pointer stored in a global structure is null.
- `execute_conditional_callback_4`: Checks if a function pointer stored in a global structure is null.
- `parse_and_execute_script`: Iterates through a list of data entries to identify and execute a sequence of commands or function pointers.
- `reset_table_entries`: Clears a two-byte field in a table repeatedly and calls another routine each time, using a counter that runs from a positive start value down to zero.
- `decrement_and_clear_counter`: Decrements a short counter at an offset from a pointer and, if it reaches zero, clears the counter and executes a function pointer from a table if a flag is set.
- `search_table_for_match`: Walks a table of entries using a fixed stride and compares an input value with a field in each entry, calling another routine when a match is found.
- `parse_script_data_block`: Iterates through a data stream to identify and process specific four-byte command tags.
- `call_other_routine_and_return_zero`: Calls another routine and then returns zero.
- `execute_infinite_recursion_2`: Calls itself indefinitely without performing any other operations.
- `mask_lower_bytes`: Calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `execute_infinite_recursion`: The routine calls itself indefinitely, creating an infinite loop of recursive calls.
- `execute_conditional_branch`: Compares a value from a memory location with a register and calls another routine if they are not equal.
- `wait_for_flag_clear_2`: Repeatedly calls another routine while a specific global variable remains non-zero.
- `remove_object_from_table_2`: If the object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_game_state_flag`: Clears a 16-bit value in a global variable and sets a flag byte in the same structure by OR-ing in a constant, then calls another routine.
- `copy_struct_to_message`: Calls another routine and, if certain conditions on the returned values and a pointer are met, copies a small fixed-size block from memory into a structure, updates several fields, and clears a byte in that structure.
- `clear_memory_flags`: Clears a longword at an offset from a register and clears a byte at another offset from that same register.
- `clear_flag_and_iterate_table`: Checks a flag in a global variable, clears it, and then iterates through a table of integers, calling another routine for each non-zero entry until a counter reaches four.
- `process_data_blocks`: Iterates through four blocks of data, clearing specific bytes and words in a destination structure based on values extracted from a source table.
- `increment_and_branch`: Increments a byte in a register, stores that byte into a memory location using an offset from a pointer, and performs an unconditional branch to another routine.
- `increment_pointers_and_branch`: Increments two pointers by four, increments a byte, and branches based on a comparison of that byte with four.
- `copy_struct_fields_and_update_table`: The routine copies seven words from one area of a structure to another area of the same structure and then copies a word from one offset to another.
- `execute_recursive_pointer_calls`: This routine checks if a pointer in a structure is non-null, calls it if it is, and then calls itself three times recursively.
- `process_event_queue_and_update_state_2`: Iterates through a linked list of event entries, decrementing counters and updating status flags.
- `decrement_and_check_counter`: Library code that tests a longword, decrements it in a loop, and conditionally modifies a register.
- `process_event_linked_list`: Iterates through a linked list of event entries, updating status flags and decrementing internal counters.
- `process_event_queue_and_update_state`: Iterates through a linked list of event entries, decrementing counters and updating status flags.
- `update_conditional_flag`: Compares two long values and, if they are not equal, checks a short flag at an offset from a pointer to potentially decrement it and call another routine.
- `clear_flag_after_callback`: The routine checks a short flag at an offset from a pointer and a global variable at another offset.
- `signal_on_flag`: Checks a global flag and, if set, calls another routine via a function pointer stored in a global variable.
- `process_sound_table`: Iterates a table of entries and invokes callbacks stored in each entry, setting bits in a mask based on their return values.
- `setup_sound_channel`: Writes a value derived from an input register to a global variable and then writes the constant 2 to the left channel of audio channel 0.
- `find_and_update_table_entry_2`: Iterates through a table using a circular index until it finds an entry matching specific criteria, then updates the entry with a pointer and other data.
- `find_and_update_table_entry`: The routine iterates through a table using a circular index until it finds an entry matching specific criteria.
- `branch_to_routine`: Loads a constant into a register and performs a relative branch.
- `clear_state_and_wait`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_flag_byte`: Computes an address from a global variable and a register value, writes zero to a byte at that address, and if the following byte equals one, writes two to the first byte.
- `clear_state_and_wait_2`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero, returning the final value in a register.
- `clear_slot_and_notify`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot, calls a callback via an indirection, and clears the next byte and the slot byte.
- `calculate_scaled_value`: Stores a word into a pointer offset and performs a calculation involving division and scaling to update a multi-word value in a structure.
- `store_word_and_branch`: Stores a word from a register into an offset of a pointer and then performs a relative branch.
- `update_scaled_value`: Calculates a scaled value based on a multiplier and a difference between two values, updates a field in a structure, and potentially clears a bit in a flag byte.
- `decrement_and_callback`: Checks a short value, clears a bit in a flag at an offset from a pointer, and calls another routine if the flag was set.
- `increment_and_branch_2`: Adds a constant byte value to a data register and performs a short relative branch to another location.
- `decrement_counter_and_branch_2`: Calls another routine and, if the result is non-zero, decrements a short value at a large offset from the returned pointer and calls another routine if the value reaches zero.
- `update_entity_physics_and_animation`: Updates an entity's position and animation state using fixed-point arithmetic and velocity calculations.
- `decrement_counter_and_branch`: Calls another routine, checks if the returned pointer is non-zero, decrements a short value located at an offset from that pointer, and calls another routine if the new value is zero.
- `copy_struct_fields_and_clear_flag`: Copies several fields from a structure pointed to by one register into a destination pointed to by another when a condition is met, then returns the original value of one source field after clearing it in the source structure.
- `update_sprite_animation_state`: Updates sprite animation frame indices, scaling factors, and timer-based properties based on velocity and fixed-point arithmetic.
- `update_sprite_transform_and_scaling`: Calculates new sprite coordinates, animation frame indices, and scaling factors based on velocity and timer values.
- `update_sprite_animation_and_position`: Calculates new sprite coordinates and animation frame indices based on velocity, timer decrements, and table-driven scaling factors.
- `compare_and_store_byte`: Compares a byte in a register to a constant and branches if they are not equal.
- `branch_on_byte_comparison`: Compares a byte to a constant and branches based on the result, potentially calling another routine.
- `decrement_value_until_zero`: This routine checks a word at an offset from a pointer and decrements it until it reaches zero or a negative value.
- `calculate_resampling_offset`: Calculates a fractional offset by multiplying a value from a table with a scaling factor and adding it to a base value.
- `resample_audio_buffer_5`: Calculates interpolation parameters and bit-shifting factors to resample audio data from a source buffer into a destination buffer.
- `subtract_if_lesser`: Compares two longwords and subtracts a third longword from the second if the first is less than the second.
- `resample_audio_buffer_4`: Calculates playback parameters and performs linear interpolation or bit-shifting to resample audio data into a destination buffer.
- `resample_audio_buffer_3`: Calculates new sample positions and playback parameters based on a frequency ratio.
- `resample_audio_buffer_2`: Performs audio resampling by copying samples from a source buffer to a destination buffer using either direct byte copying or linear interpolation between adjacent samples.
- `resample_audio_buffer`: Performs linear interpolation or fixed-point resampling to transform a source buffer into a destination buffer.
- `update_fixed_point_table`: Performs fixed-point arithmetic to update a value in a structure and then uses a loop to redistribute or resample bytes from a source buffer into a destination buffer.
- `update_sprite_reference_and_scale`: The routine iterates through a table of pointers, comparing a block of data from each entry against a reference block to manage reference counters in a structure.
- `branch_to_offset_3`: Performs a short relative branch to a nearby instruction.
- `decrement_object_counter`: Checks if a pointer in a structure is non-zero, then decrements a short value at a specific offset within that pointed-to structure.
- `process_command_and_update_state`: Parses a command buffer to perform specific operations such as memory allocation, coordinate calculations, or state updates.
- `decrement_counter_and_call`: The routine reads a pointer from an object field, checks it is non-zero, then loads a short from an offset in the pointed-to structure, decrements it and writes it back.
- `compare_and_clear_register`: Compares a word in a register to a constant and clears a longword in another register if the value is less than that constant.
- `update_audio_sample_playback`: Calculates and updates audio sample playback parameters including memory addresses, sample lengths, and volume levels.
- `calculate_audio_sample_parameters`: Calculates sample playback parameters including memory addresses, lengths, and volume levels based on table lookups and bitwise scaling.
- `compare_and_store_byte_2`: Compares a byte in a register to a constant and branches if they are not equal.
- `branch_on_byte_comparison_2`: Compares a byte in a register to a constant and branches if they are not equal.
- `decrement_word_if_nonzero`: Tests a word at an offset from a pointer and decrements it by one if it is not zero.
- `update_dynamic_scaling_parameters`: Calculates and updates several scaled values within a structure using multiplication, bit shifts, and conditional logic.
- `subtract_longword_on_greater_equal`: Compares two longwords and branches if the first is less than the second.
- `calculate_complex_values`: Performs several arithmetic operations involving multiplications and bit shifts to update values in a structure.
- `cleanup_device_and_library`: The routine checks a global pointer, and if non-zero, performs several operations including calling a function via a pointer from a global variable, updating a value in a structure, and calling several other routines before clearing global pointers.
- `execute_signal_sequence`: Calls a sequence of three different functions from a global table, passing an argument to one, and returns a value formed by masking an input register with the result of a table call.
- `store_task_and_flags`: Clears a word in a structure pointed to by a register and, if a flag in another register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the result, and copies the flag into the structure.
- `compare_strings_case_insensitive`: Compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `return_called_register`: Calls another routine and returns the value that was left in an address register.
- `allocate_memory_block`: Checks a global pointer for an existing memory allocation and, if necessary, uses a system call to allocate a new block of memory of a specified size.
- `free_memory_block`: If a pointer is non-zero, calls a function pointer or a system routine to release the memory block at that address.
- `execute_conditional_chain_2`: Calls another routine and, if the return value is non-zero, executes a sequence of conditional routine calls.
- `check_and_branch_on_result`: Calls another routine and tests its return value; if non-zero, it calls several other routines in sequence, conditionally invoking another routine based on a pointer and a bitmask check, before returning a 64-bit value.
- `initialize_object_structure`: Allocates and initializes a complex data structure by setting various offsets, including pointers, status flags, and function addresses.
- `execute_conditional_calls`: Checks a global variable and calls another routine multiple times if the value is non-zero.
- `call_subroutine_and_copy_struct`: Calls another routine and, if certain conditions involving a pointer and returned values are met, copies a fixed-size memory block to a message buffer.
- `copy_first_string_pointer`: Identifies the first non-empty string in a list of pointers and copies its pointer or content to a destination.
- `initialize_sonix_driver`: Allocates and initializes a complex data structure and attempts to copy a string pointer from a list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_envelope_and_pitch`: Updates an audio envelope state machine by modifying pitch and volume parameters based on a counter and a data table.
- `initialize_hunk_data`: Parses Amiga hunk data to populate a structure with specific values, a pointer to a global variable, and a function address.
- `trigger_crowd_audio_event`: Executes a sequence of function calls via a pointer table to update audio parameters and trigger a specific sound effect associated with the crowd stopping.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_match_statistics`: Clears several data tables and then iterates through a structured byte array to compare adjacent values.
- `process_table_comparisons`: Iterates through a table of short integers to perform equality checks and calls other routines based on the results before entering an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_values`: Calls a routine via a pointer to retrieve a value, stores it in two global variables, and performs a conditional byte transfer based on a table lookup.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_increment`: Compares a value at a specific memory location with a value from a table and updates a local variable by calling another routine if they match.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_update_table`: Performs a complex loop that retrieves values from a table, calls other routines to process them, and updates a sequence of values until a limit is reached.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_routine`: Increments a counter and pushes it and a value from a pointer onto the stack before calling another routine.
- `sort_player_attributes`: Performs a comparison-based sort on a subset of an index table by calculating the difference between two specific fields within a data structure.

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

- `execute_complex_logic_chain`: Performs a sequence of conditional checks and library calls, updating various fields in a structure based on the results of those calls.
- `frag_lea_movea_rts`: Short code fragment that loads the address of a location in the code into a5, then copies the memory at an offset from a5 to a4, then copies a fixed memory address to a6, then returns.
- `return_constant_value`: Returns a constant integer value.
- `return_address`: Returns the address of the routine itself.
- `call_multiple_subroutines`: Saves registers to the stack and executes a series of short subroutine calls before branching to a new location.
- `execute_unreachable_loop`: Performs a loop that repeatedly calls a routine and updates a byte in a structure until a condition is met, eventually returning a value.
- `call_indirect_function_via_offset`: Fetches a function pointer from an offset within a structure and executes it.
- `call_library_vector_228_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `execute_conditional_chain`: Sets a field in a structure to a maximum value, then executes a sequence of library functions based on the results of previous calls.
- `open_dos_library_and_call_methods`: Sets a field in a structure to negative one, opens dos.library, and if successful calls two specific methods before closing the library.
- `open_trackdisk_device_request`: Fills a device request record with a unit value and a fixed length, then calls a library function to open the trackdisk.device.
- `store_dual_library_outputs`: Calls two library functions and stores the results into two different records, one as a longword and one as a byte.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function_2`: Calls a library function using an offset from a base address held in a register.
- `update_structure_fields`: Copies a word from one offset to another within a structure and populates several other fields using values from registers and other structure offsets.
- `update_record_fields_2_2`: Writes a constant, two values from another record, and two calculated values scaled by a power of two into a target record.
- `update_record_field_constants`: Writes a constant byte and a constant word into a target record, copies two consecutive dwords from a source record, and stores two calculated dwords scaled by a power of two into the target record before calling another routine.
- `set_d4_and_branch`: Stores a word into a register and branches to another location.
- `load_constant_and_branch_3`: Loads a word constant into a data register and performs a short unconditional branch to another location.
- `set_register_and_branch`: Loads a small constant into a data register and performs a short branch.
- `clear_d3_and_set_d4`: Clears a register, stores a word into another register, and branches to another location.
- `update_record_fields_2`: Writes a constant, two values from another record, and two calculated values scaled by a power of two into a target record.
- `init_disk_resource_request`: Calls the library vector at offset -498 (the referenced text is disk.resource) and stores the returned pointer in the record at offset 20 of the caller's base register.
- `execute_resource_polling_loop`: Repeatedly calls a function from a structure until it returns a non-zero value, then calls another function and updates two global variables and hardware registers.
- `call_disk_resource_function`: Calls a function located at an offset within a structure pointed to by a register.
- `configure_hardware_and_call`: Modifies custom hardware registers and calls another routine using a pointer from a structure.
- `branch_to_offset`: Performs a short branch to a specific memory address.
- `update_flag_byte_bits_2`: Clears a bit at a position determined by adding three to the value in the first register, then sets the second bit if the second register is zero.
- `update_control_word_flag`: Copies a byte from a memory location into a global variable and then overwrites that same variable with a fixed flag value.
- `initiate_disk_sync_transfer`: Calls a helper routine, executes a delay loop, and then writes disk pointer, sync, and DMA control values to custom chip registers.
- `set_top_bit_field_2`: Sets the most significant bit of a 16-bit value and stores it twice into a field within a record pointed to by a register.
- `set_state_and_call_library`: Stores a constant value into a memory location via a pointer, stores the value two into another memory location via a pointer, and then executes a function located at an offset from a base pointer.
- `count_consecutive_matches`: Iterates through a memory range to find the longest sequence of identical words that match a specific target value.
- `count_consecutive_word_matches`: Iterates through a memory range to find the longest sequence of identical words that match a specific target value.
- `forward_to_checksum_helper`: Calls a routine that calculates a bitwise checksum and then returns.
- `compute_bitwise_checksum_2`: Iterates through a sequence of words, processing each bit individually to update a running 16-bit value using a left-shift and conditional exclusive-OR operation.
- `execute_conditional_branch_2`: Performs a conditional branch to a specific address.
- `extend_word_to_long`: Extends a 16-bit word to a 32-bit long and stores it on the stack.
- `execute_supervisor_call`: This routine performs a series of operations and invokes a supervisor-level library call.
- `wait_for_flag_clear`: Repeatedly calls another routine while a specific global variable remains non-zero, then returns the combined values of two input registers.
- `remove_object_from_table`: If an object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_state_flag`: Clears a 16-bit value in a structure and sets a flag byte within the same structure by performing a bitwise OR with a constant, then calls another routine.
- `clear_flag_byte_loop`: Clears a byte at an offset from a pointer, then iterates through a list of pointers, calling another routine for each non-zero entry until a counter reaches four.
- `reset_conditional_flag_byte`: Calculates an address using a global variable, a register, and a constant offset, then clears the byte at that location.
- `reset_and_poll_status`: Clears a global byte and a global word, then repeatedly calls another routine until it returns zero, finally returning the value in register D0.
- `clear_slot_and_notify_2`: Calculates a byte offset from an input value and a base pointer to clear a byte at that location.
- `calculate_scaled_value_2`: Performs arithmetic operations involving multiplication and division to store a calculated value and a scaled result into a table.
- `store_word_and_execute_subroutines`: Stores a word from a register into a location pointed to by a register, then executes a sequence of four subroutines before returning.
- `teardown_device_and_library_context`: Checks a global pointer and, if non-zero, executes a callback via a function pointer, updates a value in a structure, and performs several conditional function calls before clearing the global pointers.
- `execute_signal_sequence_2`: Calls three functions from a global table, passing a register as an argument to the second function, and returns a value formed by masking a register with the second function's return value.
- `update_task_status_and_result`: Clears a word in a structure, and if a flag in a register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the return value, and updates the flag in the structure.
- `execute_conditional_callback_5`: Checks if a register is non-zero and then executes a function pointer stored in a global variable, provided that the pointer is not null.
- `execute_conditional_subroutines`: Checks a global variable and, if it is non-zero, calls another routine four times.

### logged disk load from TV Sports Football (1988)(Cinemaware)[cr QTX][a3](Disk 1 of 2).adf

- `manipulate_string_buffer`: Iterates through a memory buffer to find non-space characters and null terminators.
- `store_indirect_results`: Calls a subroutine through a pointer and stores the returned byte into two different memory locations using an offset derived from another byte.

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

- `decrement_and_call_handler`: Checks a 16-bit value at offset -2b40 of the structure in a4 and, if non-zero, decrements a 16-bit value pointed to by a3, then calls a routine at offset -7d9a using a pointer from the structure in a4.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_two_counters`: Clears two 16-bit fields, one at offset -2b00 and one at offset -2afe, of the structure in a4, then returns.
- `call_and_store_results`: Pushes a constant and a flag, then calls a routine at offset -32592 through a pointer in the structure in a4.
- `call_routines_with_values`: Calls a routine at offset -32682, then calls a routine at offset -32154 three times, each time passing two 32-bit values read from the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_values`: Clears two bytes in a global variable, sets a 16-bit value at offset -11284 of the structure in a4 to 900 if a 16-bit value is less than 2, sets a byte at offset -13845 to 10 if it is zero, sets a byte at offset -13844 to 13 if it is zero, and if two 16-bit values at offsets -13886 and -13882 are both zero, calls a routine using a pointer from a global variable and returns the result minus 1.
- `rotate_byte_and_call_handler`: Sets up its stack frame, then checks a 16-bit value at offset -2b40 of the structure in a4 and jumps away when that value is zero.
- `calculate_and_call_scaled_values`: Performs three separate sequences where it calls a routine at offset -32730 using values derived from multiplying a byte from the structure in a4 by constants, and performs bit manipulation on d0 and d5.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to -(a7).
- `adjust_stack_pointer`: Adds 18 to the stack pointer.
- `call_handler_with_modified_byte`: Decrements a byte from the structure in a4, modifies a byte at the location pointed to by a0, and calls a routine at offset -7fda through a pointer in the structure in a4.
- `store_byte_to_zero_pointer`: Increments d4 by 8, sets a0 to zero, and stores a byte in d0 into the location pointed to by a0.
- `tiny_clr_and`: Small routine of 2 instructions that clears -(a7), then masks (a0) with d3.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `divide_and_rotate_register`: Decrements d0, rotates d4 left by 4 bits, and performs a signed division of d4 by d6.
- `transfer_and_decrement`: Copies a 16-bit word from d0 to d1, performs a bitwise OR of d0 into a memory location at a negative offset from a0, and decrements d1 using a byte from a memory location pointed to by a2.
- `push_and_set_registers`: Pushes a 32-bit value onto the stack, sets d0 to zero, and sets d6 to six.
- `initialize_registers_with_offsets`: Pushes a 16-bit value onto the stack, sets d0 to zero, and performs three sequential bitwise OR operations on d6 with the values twenty-four, thirty, and thirty-six.
- `call_subroutine_with_setup`: Moves d1 to a0, sets d6 to forty-eight, pushes two values onto the stack, calls a subroutine using a pointer from a4, sets d6 to seventy-two, and clears the word at the top of the stack.
- `initialize_and_store_constant`: Clears d0, sets d6 to 90, decrements d0 to negative one, stores the word 6 at the memory location pointed to by a0, and clears d1.
- `call_subroutine_and_update_counter`: Moves d1 to a0, sets d6 to three different values, calls a subroutine using a pointer from a4, sets d6 to two more values, loads a byte from a memory location at a negative offset from a4 into d0, and decrements d0.
- `copy_byte_to_memory`: Reads a byte from a memory location pointed to by a register and writes it to a destination address stored in another register, then increments both pointers and repeats this process in a loop until a counter reaches zero.
- `clear_byte_and_branch`: Clears a byte at a negative offset from a5 and branches to another location.
- `increment_index_on_condition`: Checks two conditions involving values at offsets from a1 and a5, and if both are met, increments a byte at a negative offset from a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_compare_byte`: Increments a byte at a negative offset from a5 and compares it to the value two.
- `increment_and_call_if_small`: Increments a byte at offset -3 of the structure in a5, then compares it to 18 and calls another routine if the value is 17 or less.
- `load_and_shift_byte`: Loads a byte from offset -2 of the structure in a5 into d0, shifts d0 left by 1 bit, and loads a pointer into a0 from offset -10448 of the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_register`: Sets a byte in a global variable to a constant value and then returns.
- `load_byte_and_adjust_stack`: Loads a byte from offset -13846 of the structure in a4 into d0, sets d1 to 34, and adds 4 to the stack pointer a7.
- `set_bit_via_pointer`: Loads a pointer into a0 from offset -13750 of the structure in a4 and performs a bitwise OR on a byte at the address pointed to by a6 using the value 236.
- `compare_bytes_and_clear`: Loads a byte from offset -11465 of the structure in a4 into d0, compares it to a byte at offset -11464 of the structure in a4, and if they are not equal, branches to another routine; otherwise, it clears a byte at offset -2 of the structure in a5.
- `load_byte_to_d0`: Loads a byte from offset -11465 of the structure in a4 into d0.
- `load_byte_to_d1`: Loads a byte from offset -2 of the structure in a5 into d1.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies d1 to d2.
- `load_address_to_a0`: Loads a pointer into a0 from offset -22374 of the structure in a4.
- `load_byte_to_d3`: Loads a byte from offset -2 of the structure in a5 into d3.
- `copy_and_shift_data`: Copies a 16-bit word from a source at a0 plus d0 to a destination at a1 plus d3, moves a 32-bit long value from a global variable to another location in a global variable, and shifts d0 left by 4 bits.
- `call_with_shifted_byte`: Loads a byte from a5 minus 2 into d1, shifts d1 left by 1 bit, calls another routine, and increments a7 by 2.
- `shift_and_increment_stack`: Loads a byte from a5 minus 2 into d2, shifts d2 left by 1 bit, and increments a7 by 2.
- `increment_and_branch_on_byte`: Copies a 16-bit word from a0 plus d0 to a1 plus d2, increments a byte at a5 minus 2, and branches if that byte is less than 3.
- `tiny_unlk`: Small routine of 1 instruction that removes the stack frame held in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
