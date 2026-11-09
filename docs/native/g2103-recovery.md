# Game 2103 (tv sports football (1988)(mirrorsoft)(disk 1 of 2)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02103). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (948116 top-level bytes in 97 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 47 of 47 units identical (bytes and relocations), 100 raw-word lines |
| Functions named and described | pass, 1109 of 1109 named (59 author symbols), 1109 of 1109 described (275 by rule, 818 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6324 of 6347 (99.64%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 18303 of 18367 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 141508 |
| code | 59460 |
| data | 712564 |
| padding | 22808 |
| unused | 11776 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| cd | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf:fb | 3 | 9208 | 471 |
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
| TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
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
| logged disk load from TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf | 1 | 145 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 48 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 214 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 46 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: file KBlocks.1 (header 406205 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
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
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: 85 executed PCs whose RAM was overwritten before any snapshot match its bytes 75344-75856: recovered as a unit run at an address
- 246 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 7 first-run image unit(s) in 2 version group(s); 1256 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying code at an address (28 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (27 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (f50) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (f50)): both forms recovered; writer: not identified statically (indirect store)
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: blocks 895-896 (bytes 458240-459264) hold ASCII text
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: blocks 898-900 (bytes 459776-461312) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.50)
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: blocks 903-1234 (bytes 462336-632320) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.78)
- tv sports football (1988)(mirrorsoft)(disk 1 of 2)/TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf: blocks 1236-1696 (bytes 632832-868864) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.91)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `invoke_disk_resource_routine_3` (runtime segment an address): Calls a function on the disk resource base held in the state block, and if it returns zero, calls another function and repeats the process until a non-zero value is returned, then calls a third function and writes values to hardware registers before branching to another routine. Kind: loader/integrity check loop. Pass path: iVar2 != 0 (the loop breaks and proceeds to the next stage when the routine returns a non-zero value).
- `configure_hardware_and_call_subroutine` (runtime segment an address): Sets the interrupt enable and DMA control registers via the custom chip base, writes a value to the DMA address register, and calls a subroutine using a pointer from a table before branching to another location. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `copy_state_byte_to_register` (runtime segment an address): Copies a state byte into a control word and writes a fixed flag value to a second control word. Kind: disk hardware/track validation. Pass path: the branch following the call (e.g., an address in sub_2524).
- `start_disk_sync_transfer` (runtime segment an address): Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers. Kind: disk sync/track check. Pass path: execution continues to subsequent instructions (e.g., movea.l an address(a4), a0).
- `execute_resource_loop` (runtime segment an address): Calls two functions via pointers from a structure, looping if the second function returns zero, and finally writes three values to hardware registers before branching. Kind: disk hardware/track validation loop. Pass path: the loop continues (iVar2 == 0) until a non-zero value is returned, at which point it breaks to execute the next stage (u38__sub_0342).
- `configure_hardware_and_call_library` (runtime segment an address): Loads the custom chip base address into a1, writes values to offsets 154 and 36 of that base address, loads a pointer from offset 20 of a5 into a6, loads a pointer from offset 32 of a4 into a1, calls a library function using a6, and branches to another routine. Kind: disk hardware/track check. Pass path: bmi.b an address.
- `update_joystick_status_register` (runtime segment an address): Writes a fixed byte to the joystick port control register, copies a byte from an offset of 25 in a structure pointed to by a5 into the same port control register, and sets a bit in the hardware custom chip control register before returning. Kind: manual/code-wheel/colour-card lookup. Pass path: the branch following the call that continues normal execution (likely the 'bpl.b' path in sub_035C).
- `sync_disk_dma_transfer` (runtime segment an address): Calls a helper, waits for a countdown loop to complete, then writes values to the DMA control, disk pointer, and sync registers before calling two other routines. Kind: disk hardware sync/timing check. Pass path: execution continues to the next instruction (bsr.w an address).

## Functions


### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a long word from register D1 to D0 and then branches to another routine.

### EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_register_and_branch`: Moves a longword from register d1 to register d0 and then performs a short branch to another routine.

### cd

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Moves the contents of d1 to d0 and then branches to another location.

### Port-Handler

- `process_amiga_hunk_relocations`: Parses a structure containing hunk information to initialize a control block, then iterates through a list of relocation entries to patch memory values.

### Disk-Validator

- `update_score_and_bitmask_data`: Converts a value from a global variable into a character string and stores it in a buffer, then performs a series of bitwise operations and loops to update a bitmask and a table of values based on several other global variables.

### TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf:fb

- `clear_memory_and_dispatch_library_calls`: Fills a memory range with zeros using a loop, then performs a series of conditional calls to functions stored within a structure pointed to by a global variable.
- `initialize_game_state_structure_2`: Resets several global status variables to zero, initializes hardware parameters, and populates various data structures and pointers to prepare the game environment.
- `execute_callback_sequence_2`: Calls four different routines via pointers and a branch to another routine, while also pushing a value from a memory location at an offset from a4 onto the stack.
- `execute_callback_chain_3`: Calls two functions via pointers, passes a value to a third function, and then calls itself twice.
- `branch_via_table_2`: Stores a value at a location relative to a5, then compares a word to d0 and branches to a jump table.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `branch_via_table`: Compares d0 to 16, and if not less, shifts d0 left by 1 and uses it to index a jump table.
- `execute_callback_sequence`: Sets two values in a structure, calls a function via a pointer, and executes a sequence of other subroutines and callbacks.
- `dispatch_library_calls`: Sets values in a structure and executes a series of function calls through pointers, including several subroutines.
- `process_hunk_data`: Sets several values in a structure, executes a series of function calls through pointers, and performs a conditional check based on a value in a table before calling more subroutines.
- `update_announcer_state`: Sets two word values in memory, calls a function via a pointer, and conditionally executes a routine using a value retrieved from a pointer.
- `execute_indirect_call`: Calls a routine via a pointer, stores the result in a global variable, and if that result is zero, calls another routine before storing the address of that global variable into another global variable.
- `reset_conditional_flags`: Checks several memory locations and, if non-zero, resets various global variables to zero or one.
- `process_conditional_audio_logic`: Updates a word in memory and calls a function with three parameters.
- `calculate_kick_accuracy_and_update_state`: Calculates kick accuracy values by recursively calling a routine with different parameters and stores the results in a data structure.
- `dispatch_halftime_logic`: Stores a value into a memory location and executes several functions via a table of pointers before calling itself recursively.
- `clear_status_variables`: Sets a series of specific memory locations to either zero or one.
- `execute_music_callback`: Calls a function via a pointer and then calls itself recursively with a global variable.
- `execute_callback`: Calls a subroutine through a pointer found at a specific offset, passing a value and a parameter.
- `check_global_flag_and_recurse`: Checks if a word at a specific offset from a global variable is zero; if so, it calls itself recursively, otherwise it returns.
- `update_calculated_offsets`: Calls a subroutine to get a value and uses it to update several global variables with calculated offsets.
- `initialize_hardware_parameters`: Calls several subroutines and writes specific constant values to a set of memory locations.
- `sum_indexed_bytes`: Calculates a sum by retrieving and adding bytes from a table using an index and multiple offsets.
- `manipulate_data_via_table`: Calls three different routines via a table to manipulate data, and if a specific condition is met, it performs further operations using values from a structure in a6.
- `perform_data_operation`: Calls subroutines to manipulate data at a specific memory location and returns a value from a global variable.
- `process_parameterized_request`: Checks a condition, then calls a sequence of subroutines to manipulate data via a handle and returns a result.
- `process_vhdr_data`: Performs several string comparisons and memory operations, including calling a routine to process data and a routine to calculate a value, before updating a word in a structure.
- `modify_registers_and_decrement`: Adds three to a register, negates the value in a memory location pointed to by a2, performs a bitwise OR on d2 with a constant, and decrements d0 by four.
- `compare_and_process_strings`: Compares two byte strings using a routine from a table, and depending on the result, calls another routine to process data or updates a local variable.
- `compare_and_call_subroutines`: Calls several subroutines using offsets from a4, compares a value from a memory location with d0, and performs conditional branches and additional subroutine calls based on comparisons of d0 and values stored on the stack.
- `check_bit_six`: Checks bit 6 of a byte at an offset from a pointer in a4 and branches to another routine if the bit is set, otherwise it returns.
- `load_constant_and_branch_3`: Loads the value one into register d0 and branches to another routine.
- `test_bit_and_branch`: Loads a pointer from an offset of negative 18176 from a4, tests bit 1 of the byte at offset 1 of the pointed-to location, and otherwise sets d0 to 1 and returns.
- `test_bit_one`: Checks bit 1 of a byte at an offset from a pointer in a4 and branches to another routine if the bit is set, otherwise it sets d0 to negative one and branches to another routine.
- `branch_with_zero`: Sets register d0 to zero and branches to another routine.
- `toggle_bit_eight_2`: Loads a pointer from an offset of negative 18176 from a4, checks if bits 0 and 1 of the word at that location are both set or both clear, and if they are not, it toggles them and returns 1.
- `toggle_bit_eight`: Checks if bit 8 of a word at an offset from a pointer in a4 is set, and if it is not set or if it is set but toggling it results in the bit being set, it branches to another routine, otherwise it sets d0 to negative one and branches to another routine.
- `clear_d0_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `clear_memory_tables`: Performs multiple nested loops to zero out various segments of memory organized in specific patterns and dimensions.
- `check_status_and_return`: Tests a word at a negative offset from a4 and branches if non-zero; otherwise it tests another word at a negative offset from a4 and returns if it is zero, otherwise it clears d0 and returns.
- `update_memory_offsets_2_2`: If a byte at offset 13 of a structure in a5 is 31, it calculates a memory address using values from offsets 8 and 10 of the structure and a value from a4, then adds a word from offset 14 of the structure to that address.
- `update_memory_offsets_2`: Calculates three different memory addresses using values from a structure in a5 and a global variable, then adds a word from that structure.
- `update_memory_offsets`: Calculates three different memory addresses using values from a structure in a5 and a global variable, then adds a word from that structure to the values at those addresses.
- `accumulate_coordinate_offsets`: Clears a block of memory and then performs a series of additions, summing values from multiple source tables into a destination table using iterative loops.
- `call_library_function`: Calls a function via a pointer stored in a table and stores the result in a global variable.
- `call_function_with_argument`: Calls a function using a value retrieved from a global variable and passes four as an argument.
- `enable_dma_channel`: An 18-instruction routine that modifies the DMA controller register to enable a DMA channel.
- `write_to_hardware_registers`: Writes a value from a global variable and the constant one to specific hardware register offsets.
- `update_coprocessor_hardware`: Calls a routine three times, passing the constant one, the constant two, and the constant two respectively to update hardware registers.
- `enable_dma_channels`: Sets a global variable to a value that enables DMA channels via the DMACON register.
- `initialize_font_pointers`: Calls multiple library functions to populate several global variables with pointers or values related to diskfont.library.
- `execute_callback_chain`: Checks several global variables and, if they are non-zero, calls a function using those values as arguments.
- `execute_handler_routine`: A 37-instruction routine called via a pointer in data that includes a call to another routine.
- `tiny_move_move`: Small routine of 2 instructions that copies -(a0) to d0, then copies d0 to d0.
- `execute_parameterized_call`: Calls a function via a pointer using a value from a memory location and a provided parameter.
- `execute_table_calls`: Passes various parameters to functions called via a table of pointers and then calls another routine with a value from register d0.
- `manipulate_graphics_buffer`: Performs multiple library calls to draw, clear, or move data within a buffer based on calculated coordinates and parameters.
- `process_coordinate_table`: Iterates through a table of eight pairs of values to perform operations using a calculated base address and parameters.
- `call_wrapper_function`: Passes three parameters directly to a function called via a pointer in a table.
- `process_coordinate_table_2`: Iterates through a table of values to perform operations using a calculated base address and parameters, calling two different routines within a loop.
- `clear_word_at_offset`: Clears a 16-bit word at offset -14488 of a structure in a4 and unlinks a5 before returning.
- `check_and_return`: Tests a word at a negative offset from a4, and if it is zero, it clears d0 and returns; otherwise it returns.
- `update_status_and_sync_flags`: Calculates a XOR difference between a value in a structure and a value at an offset in a5, then updates several status words in a4 based on the result and the state of other flags.
- `clear_and_update_status_words`: Clears three 16-bit values at negative offsets from a4, and if a 16-bit value at offset -13928 from a4 is not zero, copies three 16-bit values from offsets -17878, -17876, and -17880 from a4 to offsets -1526, -1522, and -1514 from a4, then branches to another routine.
- `clear_status_words`: Clears three 16-bit values at negative offsets from a4 and branches to another routine.
- `call_and_store_status_results`: Tests a 16-bit value at offset -13892 from a4 and branches if it is zero, otherwise it calls three different routines, storing the result of the first call at offset -1526 from a4, the result of the second call at offset -1522 from a4, and the result of the third call masked with bit 0 at offset -1514 from a4, then branches to another routine.
- `update_status_words`: Clears three 16-bit values at negative offsets from a4, and if a 16-bit value at a specific negative offset is not zero, copies three 16-bit values from other negative offsets in a4 to new negative offsets in a4 before branching to another routine.
- `update_status_flags`: Clears three words at specific negative offsets from a4, performs an XOR operation on two words, and conditionally updates several words at negative offsets from a4 based on a value at an offset from a5.
- `clear_status_flags`: Clears three words at specific negative offsets from a4, then tests a word at offset 8 of a structure in a5 and branches if non-zero; otherwise it sets d0 to 1 and branches to another routine.
- `clear_and_branch_3`: Calculates an offset using d0, tests a word at that offset from a structure in a4, and if certain conditions are met, writes three words from offsets of negative 13738, negative 13818, and negative 13820 of a4 to offsets of negative 10844, negative 11266, and negative 11270 of a4 before branching.
- `clear_and_branch_2`: Clears three 16-bit words at specific offsets from a structure in a4 and then branches to another routine.
- `clear_and_branch`: Clears three 16-bit words at specific offsets from a structure in a6 and then branches to another routine.
- `calculate_digit_table`: Calls two routines using a pointer and a structure in a5, then performs integer division and multiplication to decompose a 16-bit value from offset 12 of the structure into separate decimal digits stored at offsets -2, -4, -6, -8, and -14.
- `process_table_elements_2`: Loops through a table, setting a word to 10 if a value is zero, decrements a counter in a structure, and finally updates that counter with a byte from another location.
- `process_table_elements`: Loops through a table, setting a word to 32 if a value is zero, decrements a counter in a structure, and finally updates that counter with a byte from a different offset.
- `iterate_and_call_function`: Decrements a counter in a loop and calls a function via a pointer if a specific value in a table is not equal to thirty-two.
- `return_absolute_value`: Returns the absolute value of a passed integer.
- `call_indirect_functions`: Calls two different functions using pointers stored in a table, passing various parameters including sums of input values.
- `initialize_and_branch`: Sets up a stack frame and clears a 16-bit value at an offset from a5 before branching to another routine.
- `copy_memory_block`: Copies bytes from one memory location to another using a loop and an index.
- `calculate_accumulated_value`: Performs a loop to combine a global variable with other values and stores the result in a global location.
- `update_complex_accumulator`: Uses a loop and bitwise operations to add a combination of global and local values to a stored integer.
- `increment_table_entry`: Calls a function, uses the result to calculate an offset into a table, and increments the word at that location.
- `update_accumulated_sum`: Calculates a sum using two values from memory and a global variable, then stores the result back into one of those memory locations after adding multiple instances of the global variable to it.
- `copy_and_reset_data`: Copies a longword from one location to another and then sets the original location to all ones.
- `copy_value_via_pointer`: Calls another routine and uses its return value as a base pointer to store a longword at offset 184 of that pointer, using a value retrieved from a memory location at offset -10132 from a value in a4.
- `handle_input_and_state`: Copies bytes from a table to a buffer, calls several routines to update state and graphics, and performs conditional logic based on a return value.
- `process_event_loop`: Stores a constant in a global variable, calls a routine with two parameters, and then enters a loop that calls another routine and compares a value from a structure to the previously stored constant, potentially calling several other routines before updating the constant.
- `call_and_copy_data`: Calls a function via a pointer and then calls another routine to copy data.
- `execute_parameter_sequence`: Calls a routine four times with specific constant parameters, then calls another routine with a pointer from a5, and if the result is non-zero, calls two more routines using values derived from the previous results and the structure in a5.
- `redirect_execution_flow`: Performs a short unconditional branch to another routine.
- `jump_to_adjacent_code`: Performs a short branch to the instruction immediately following this routine.
- `decrement_and_branch_3`: Subtracts 80 from d0 and branches if it is zero; otherwise it subtracts 2 from d0 and branches to a previous instruction if it is zero, otherwise it branches to another routine.
- `branch_to_another_routine`: Performs a short branch to a location within another routine.
- `branch_to_middle_of_routine`: Performs a short branch to an offset within another routine.
- `update_state_and_graphics`: Performs several conditional checks and calls various routines to update state, display parameters, and button status.
- `update_game_state_and_graphics`: Copies a block of 12 words from a source table to a local stack buffer and performs a series of state updates by calling various subroutines with specific parameters.
- `copy_data_block`: If a specific global flag is set, it clears that flag and copies five words from one memory location to another.
- `update_display_parameters`: Calls several functions to retrieve a value, calculate a new offset, and perform multiple writes to a table using that offset and provided parameters.
- `check_button_status`: Checks if bits 6 and 7 are set in the CIAA Port A Data register and returns 1 if both are active, otherwise it returns 0.
- `pass_parameters_to_subroutine`: Pushes thirteen separate values from various offsets of a structure in a5 onto the stack, calls another routine, and then restores the stack pointer before returning.
- `check_sprite_clipping_bounds`: Compares a word at offset 2 of a structure in a3, shifted left by 3 bits, against a word at offset 20 of a structure in a5; if the shifted value is less than the value in a5, it branches to another routine, otherwise it returns.
- `compare_and_branch_4`: Loads a word from offset 22 of a structure in a5 into d0, compares it to a word at offset 2 of a3, and branches to either a previous instruction or a later instruction.
- `compare_word_and_branch`: Compares a word from offset 14 of a structure in a5 to a word in a2, and branches based on the comparison result.
- `mask_and_store_word`: Reads a word from offset 20 of a structure in a5, performs a bitwise AND with 15, and stores the result at offset 48 of the same structure; it then checks a word at offset 24 of the structure and branches to another routine or returns.
- `update_sprite_clipping_parameters`: Calculates and updates clipping dimensions, coordinate offsets, and lookup table indices within a structure pointed to by a5.
- `calculate_blitter_clipping_offsets`: Performs integer division and bit shifts on register values to update coordinate offsets and dimensions within a structure pointed to by a5.
- `calculate_blitter_dimensions_and_offsets`: Calculates coordinate offsets and dimensions by performing integer division and bit shifts on register values, writes these results to a structure pointed to by a5, and updates several fields within that structure based on comparisons with values from a table in a2.
- `calculate_blitter_area_and_loop`: Performs integer division and multiplication to calculate coordinate offsets and dimensions, writes these values to a structure in a5, and enters a loop that calls setup_blitter_parameters_and_loop while updating various data fields based on values from a table in a3.
- `setup_blitter_parameters_and_loop`: Calculates two longword values based on multiplication and addition of values from a stack frame, writes them to the blitter's area and mode registers, and then enters a loop that iterates based on a byte value to update blitter control registers and bitplane pointers using values from a table.
- `configure_blitter_and_loop_2`: Calculates an address using a value in d7, performs bitwise operations, and moves a longword from an offset of that address to a stack location.
- `configure_blitter_and_loop`: Sets bits in a word, calls a routine, and enters a loop that updates hardware registers for the blitter and bitplane pointers based on values in a structure and a table.
- `branch_to_subroutine`: Performs a short branch to another routine.
- `execute_structure_callbacks`: Calls several subroutines and resets a global pointer.
- `execute_structure_callbacks_2`: Pushes the value 20 onto the stack and calls a subroutine that executes a series of function pointers stored in a list or table.
- `initialize_game_state_structure`: Populates a structure by calling a sequence of initialization and clamping routines, then performs multiple conditional lookups to store specific values into a table.
- `process_event_data`: If a global flag is set, it calls several routines using parameters from a structure, including a conditional call to another routine if a specific value is not 7.
- `execute_conditional_logic`: If a global flag is set and a value returned from a routine is non-zero, it performs a series of calls to update data and handle a pointer.
- `load_tile_asset_data`: Initializes a structure by clearing an array of eight 32-bit values and two 16-bit words, then reads a data stream to populate the structure with dimensions, color information, and a pointer to a memory block.
- `process_tile_stream_data`: Extracts a bitstream from a data structure to update a table with new tile dimensions and a count.
- `parse_tile_stream_and_update_table`: Iterates through a data stream to extract nibbles, decrementing a counter if the nibble matches specific values, and populates a table by calling another routine for each entry.
- `process_tile_data_stream`: Iterates through a data stream to populate a table, calling another routine for each entry; if a zero value is encountered, it clears a field in a structure, calls another routine, and processes a fixed number of elements.
- `clear_table_entry`: Clears a longword at an offset of the stack pointer and branches to another routine.
- `set_stack_offset_value`: Writes a longword of all ones to a memory location calculated by subtracting 14 from a pointer in a5, then branches to another routine.
- `load_table_entry`: Increments a 16-bit value in a structure, uses it to calculate an offset into a table, and moves a longword from that table to a stack offset before branching to another routine.
- `decrement_and_branch_2`: Decrements a value in d0 three times, branching to different locations if the value reaches zero, otherwise it branches to another instruction.
- `update_table_entry`: Calculates an offset using a value from a structure, writes a longword to a table at that offset, increments a counter, and returns if the counter reaches eight.
- `call_external_routine_2`: Pushes a longword from the stack onto the stack, calls another routine, cleans the stack, and branches to another routine.
- `update_table_elements`: Clears a 16-bit word at offset 18 of a structure pointed to by a value at offset 12 of a6, then enters a loop that calls another routine using values from a table at offset -46 of a5, incrementing a counter at offset -70 of a5 until it reaches 8, and finally calls another routine with two values from a5 if a value at offset -50 is non-zero.
- `expand_compressed_tile_data`: Reads a compressed data stream from a source buffer and expands it into a destination buffer using a triple-nested loop.
- `loop_and_branch`: Decrements a counter in d5 and branches to another routine if the counter is not zero, otherwise it branches to a different location.
- `copy_nested_data`: Performs a triple-nested loop to copy bytes from a source buffer to a destination buffer, where the amount of data copied and the destination addresses are determined by values read from the source and various pointers.
- `decrement_and_branch`: Decrements a loop counter in register d5 until it reaches zero and then branches to another routine.
- `restore_registers`: Restores d0 through d7 and a0 through a7 from the stack and returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_conditional_memory_updates`: Increments a global counter and conditionally calls various routines to copy data to different memory locations based on the state of several short-integer flags.
- `update_conditional_data`: Calls a routine and stores the result in a global variable, then if the result is not negative, it clears several memory locations, sets a byte to two, and calls another routine with two arguments.
- `reset_global_pointer`: Calls a function with a constant and a pointer, then calls another routine and sets a global variable to a maximum value.
- `initialize_gameport_input_state`: Allocates and initializes a structure to manage input device states by querying the gameport device.
- `execute_indirect_calls_2`: Calls a routine with four arguments, executes two consecutive indirect function calls via a pointer located at an offset from a register, and returns a value from a memory location at an offset from that same register.
- `clear_and_refresh`: Sets a word in a global location to zero and calls another routine twice.
- `remove_node_from_list`: If a pointer in a structure is not zero, it traverses a linked list to find a matching value and updates the previous node's pointer to bypass it.
- `swap_linked_list_nodes`: Compares two values via a routine and, if the result is negative, swaps the pointers of two nodes in a linked list.
- `traverse_and_swap_linked_list`: Iterates through a linked list by following pointers, calls another routine to compare data, and if the result is not negative, swaps the current node with the next node in the list.
- `swap_two_values`: Copies the value from one memory location to another and then attempts to assign a pointer to the original location.
- `calculate_blitter_area_and_call_handler`: Multiplies a word at offset 2 of a structure by a word at offset 8 of the same structure, passes the result to another routine, and returns the product or zero.
- `configure_blitter_mask_and_attributes`: Clears several blitter control and mask registers, calculates a 16-bit mask from values at offsets 0 and 2 of a structure, and retrieves a byte from a table using an index derived from offset 5 and offset 12 of that structure.
- `configure_blitter_mask`: Copies a word to the blitter mask register, loads a pointer from an offset of 8 in a5, loads a pointer from an offset of 12 of that pointer, and if the pointer is not zero, copies it to the blitter address register and sets bit 10 of d6 before branching.
- `set_blitter_parameters`: Copies a word to the blitter mask register, loads a pointer from an offset of 8 in a6, loads a pointer from an offset of 16 of that pointer into a3, and if a3 is not negative, copies a3 to the blitter destination register and sets bit 3 of d6 before branching.
- `configure_blitter_parameters`: Sets several Blitter registers, calls a routine, and then updates Blitter control and mask registers based on values from a structure and a table.
- `update_blitter_registers`: Moves a word to the Blitter address register, checks a value from a structure, and if it is not negative, moves that value to the Blitter enable register and sets bit 9 of a register.
- `update_blt_registers`: Sets several Blitter registers, calls another routine, and then updates a structure's value and Blitter parameters based on a comparison.
- `check_structure_overlap`: Adds a word from offset 52 of the first structure to a word from offset 12 of the first structure, subtracts one, and compares the result with a word from offset 12 of the second structure; if the result is not greater than or equal to the value in the second structure, it returns zero, otherwise it returns one and stores the pointers to the two structures in two global variables.
- `compare_offset_values_3`: Adds a word from offset 52 of a3 to a word from offset 52 of a3, subtracts one, and compares the result with a word from offset 12 of a2, branching if the result is less than or equal to the value in a2.
- `compare_offset_values_2`: Adds an offset to a word, subtracts one, and compares the result with a word from another structure, branching if the result is greater than or equal to that word.
- `compare_offset_values`: Calculates a value by adding an offset to a word and subtracting one, then compares this result with a word from another structure and branches if the comparison is less than or equal.
- `move_and_branch_3`: Moves two longwords from offsets of a pointer into memory locations and then branches to another routine.
- `clear_global_values`: Sets several memory locations to zero and calls another routine.
- `copy_and_process_data_blocks`: Copies a block of thirty-two words from one memory location to another in a loop, then calls another routine if certain conditions are met, and finally calls a third routine if two specific words are non-zero.
- `pack_nibbles_to_table`: Reads bytes from a source table and packs their high nibbles or certain bit patterns into a destination table through a loop that runs 32 times.
- `calculate_scaled_differences`: Calculates the difference between high nibbles of two source tables and stores the result divided by a value into a destination table.
- `update_table_from_differences`: Computes differences between elements in a table and stores the results into another table, or copies a table if certain counters reach zero.
- `recursively_check_and_copy`: Recursively checks a byte flag in a structure and calls a routine to copy or process memory blocks if a bit is set.
- `copy_memory_block_2`: Copies a certain number of words from a source pointer to a destination pointer.
- `process_and_copy_elements`: Performs conditional insertion or deletion of bytes within a list based on specific criteria and then copies values from a table into a destination.
- `initialize_byte_table`: Fills a table with values where each byte is double its index through a loop that runs 32 times.
- `copy_structure_elements`: Copies four sets of data from a source table to a destination table using a loop.
- `compare_converted_strings`: Iterates through two strings, converts their characters using another routine, and compares them until a mismatch is found.
- `subtract_converted_values`: Calls a routine to convert two values and returns the difference between them.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `transfer_register_to_stack`: Loads a word from an offset of 12 from a location pointed to by a5 into d4, then pushes the previous value of d4 onto the stack before returning.
- `call_external_function`: Calls a routine using a pointer and a constant value, stores the result in a structure at offset minus 4 of a5, and returns if the result is not zero.
- `update_structure_and_call`: Calls a routine using a pointer from offset 8 of a5, increments the result, calls another routine with that result on the stack, and if the result is not zero, writes it to offset 4 of a structure pointed to by offset -4 of a5; otherwise, it calls a third routine with a pointer from offset -4 of a5 and the value 38, then returns 0.
- `update_structure_fields`: Calls a routine using a pointer from a structure, calls another routine with a value from the stack, and then writes several words to offsets 18, 20, 22, 24, and 36 of a structure pointed to by a0.
- `resolve_pointer_string`: Checks if a byte at a pointer in a5 is zero; if so, it loads a new address from a global variable, updates the pointer in a5, and then checks a value at offset 4 of a structure to decide whether to call another routine or return.
- `swap_pointer_and_branch`: Loads a longword from a pointer stored at an offset of 4 bytes from a5 into a0, writes the longword located at the address in a0 to the memory location 4 bytes before a5, and branches to another routine.
- `call_external_pointer`: Pushes a longword from offset 8 of a structure in a5 onto the stack, calls another routine, cleans up the stack, and branches to another location.
- `empty_c_function_2`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_c_function_3`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `clear_structure_pointers`: Checks two pointers within a structure, calls a routine to process one if it is not null, and then clears both pointers.
- `process_linked_structure`: Calls two routines to manipulate a structure and then uses function pointers to update values within that structure.
- `traverse_linked_structure`: Iterates through a linked list of pointers and calls another routine for each node encountered.
- `empty_c_function_4`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `clamp_and_store_result`: Calls a library function to retrieve a value, clamps that value between a minimum and maximum bound, and stores the result through a pointer obtained from the library call.
- `process_table_data`: Iterates through a table of words, performing a bitwise rotation and addition on each element.
- `recalculate_value_with_modulo`: Obtains a value from another routine and performs a calculation involving division and modulo using a part of a provided parameter.
- `update_table_index`: Decrements a value in a global variable and uses it to index another part of the same table to perform an addition.
- `allocate_and_initialize_object_structure`: Allocates a new structure via a library call, populates its fields with values from a passed parameter and a linked list, and performs arithmetic calculations to update specific offsets in the structure.
- `traverse_linked_list`: Recursively traverses a linked list structure, calling a routine on each node and its children, and performing memory deallocation on specific elements.
- `search_and_store_pointers`: Iterates through a list of word pairs, comparing them against a calculated value, and stores the address of the next element in a destination table if a match is found.
- `search_conditional_table`: Searches a table for a value based on a parameter, potentially performing a second search in a different table if a specific bit is set in a byte within a structure.
- `process_blitter_list`: Calculates a value from a structure, calls a routine, and then clears several Blitter registers while setting others to all ones.
- `execute_and_update_values`: Saves registers, calls another routine, restores registers, and then updates a word at offset 64 of a structure with a value from a pointer, before branching.
- `update_structure_from_pointer_list`: Iterates through a list of pointers in a0, updating a structure pointed to by a2 with a constant 16-bit value, a 16-bit value from d2, a 32-bit pointer from the list, and a 16-bit value from d1 for each entry.
- `open_file_handle`: Attempts to open a file using provided parameters, stores the resulting handle in a global variable, and calls another routine if the handle is null.
- `handle_file_error`: Checks if a global file handle is non-zero and if a parameter is non-zero, then attempts to perform an operation and closes the file handle if an error occurs.
- `close_file_handle`: Checks if a global variable is non-zero, calls a routine using that value as an argument, and then clears the global variable.
- `allocate_and_populate_data_structures`: Allocates memory for a combined number of elements based on two input values, then populates the new memory block by iterating through two separate loops to load data from files.
- `extend_byte_to_long`: Loads a byte from a structure, extends it to a word and then to a long, and branches to another routine.
- `manage_linked_list`: Calls a routine to process a pointer from a structure, and if the result is zero, it calls another routine to allocate a node, copies data from a2 to that node, and then potentially calls a third routine to free nodes while clearing several fields in a structure.
- `pass_pointer_to_function`: Retrieves a pointer from a global structure and an integer from a register to pass them as arguments to another routine, then uses a table to jump to a location based on the integer value.
- `extend_byte_and_call`: Loads a byte from offset 9 of a structure pointed to by a5, extends it to a word, pushes it onto the stack, and calls another routine.
- `call_extend_multiple_values`: Extends and shifts three different values from offsets 14, 12, and 10 of a5 to create long integers, pushes them onto the stack, and then extends a byte from offset 9 of a5 to a word and pushes it before calling another routine.
- `set_global_flag_via_pointer`: Extends and shifts three words from offsets 20, 18, and 16 of a5 to create long integers, pushes them onto the stack, pushes a byte from offset 15 and a pointer from offset 10, and then extends a byte from offset 9 of a5 to a word and pushes it before calling another routine.
- `decrement_and_clear_global`: Loads a pointer from offset 8 of a5, checks a word at offset 40 of that pointer, and if it is greater than zero, decrements it; if the new value is still greater than zero, it clears a word at a global offset, otherwise it returns.
- `process_table_entries`: Writes a word to a global variable, calls a routine using a pointer from a2, and if a word from offset 42 of a2 is less than a word from offset 12 of a structure in a0, it calculates an offset, pushes several values from a structure in a3 onto the stack, calls another routine, and increments the word at offset 42 of a2.
- `compare_and_branch_2`: Compares a word from a table at offset 42 of a structure in a2 with a word from a structure in a0, and if they match and a word at offset 40 of a2 is zero, sets d0 to one and returns.
- `set_zero_and_branch`: Sets register d0 to zero and branches to another location.
- `update_status_and_check_bits`: Sets a 16-bit value at offset minus 2 of a5 to 1, calls another routine, and if the result in d0 is not zero, checks bit 0 of a value at offset 4 of a structure in a2; if that bit is set, it checks a 16-bit value at offset 20 of a5 and branches based on the result.
- `call_with_calculated_values`: Calculates two longwords by adding offsets from structures in a2 and a5, pushes them onto the stack along with a pointer to a2, and calls another routine.
- `update_and_check_status`: Calls a routine with a pointer from a2, and if the result is zero, it clears a 16-bit value at offset minus 2 of a5 and checks a value at offset 20 of a5 to potentially call another routine or set d0 to 1.
- `traverse_and_update_structure`: Calculates two values by adding offsets from a structure and a parameter, calls another routine with these values, and then updates several fields within a structure using a loop.
- `get_register_value`: Calls a routine and returns a word from a specific memory location.
- `increment_counter`: Increments a value in a global variable if it is not zero.
- `decrement_counter`: Decrements a global variable and resets it to zero if it becomes negative.
- `calculate_and_draw`: Calculates two short values based on input parameters and a table, then calls a routine to perform a drawing operation using those values.
- `allocate_and_insert_list_node`: Calls a routine with arguments 62 and 65536, and if the result is not zero, copies several values from a structure in a2 to the returned pointer, then if bit 11 of a byte at offset 11 of a2 is set, copies more data from a structure pointed to by offset 12 of a2; finally, it updates a linked list using the returned pointer.
- `insert_into_sorted_list`: Copies several values from a structure in a5 to a structure in a2, checks a bit in a pointer from a5, and if set, copies values from a nested structure into a2; then it compares values in a3 with a2 to determine whether to insert a pointer into a linked list or swap nodes.
- `swap_pointer_values`: Copies the value from a location pointed to by a3 into a location pointed to by a2, then copies a2 into the location pointed to by a3, and branches.
- `store_pointer_twice`: Stores the value of register a3 into a location pointed to by a2, then stores a2 into a location pointed to by a structure at offset 8 of a5, and branches.
- `store_pointer_and_branch`: Stores the value of register a2 into a location pointed to by an offset of a structure, and then branches.
- `verify_structure_data`: Compares a byte from a structure on the stack with a longword from another structure on the stack, and if they match, sets a0 to a pointer from the first structure and returns.
- `compare_byte_and_return`: Compares a byte at offset 9 of a structure with register d1 and returns if they are not equal, otherwise it returns with register a0 pointing to the structure.
- `traverse_and_free_list_2`: Traverses a linked list and, if matching criteria are met, calls a routine to free nodes and updates pointers to remove them from the list.
- `traverse_and_free_list`: Recursively traverses a linked list and calls a routine to free each node.
- `call_and_verify_result`: Pushes two longwords from a structure onto the stack, calls another routine, stores the result in a structure at offset 14 of a5, and then returns if the result is zero or if the result does not equal 12345.
- `set_one_and_branch`: Sets register d0 to one and branches to another location.
- `zero_d0_and_branch`: Clears register d0 and performs a short branch to another routine.
- `process_data_table`: Iterates through a table of eight entries, performing a calculation with values from the table and calling another routine before clearing the entries.
- `execute_and_store_result`: Calls another routine with two pointers on the stack, stores the returned longword into a location at offset negative four from a5, and otherwise returns.
- `update_field_and_call`: Calls a routine using a pointer from offset 8 of a5, increments the result, and if the result is not zero, writes it to offset 4 of a structure pointed to by offset -4 of a5; otherwise, it calls a third routine with a pointer from offset -4 of a5 and the value 16, then returns 0.
- `set_flag_and_call`: Calls a routine using a pointer from a structure, writes a word to offset 10 of that structure, and then calls another routine using a pointer from the same structure.
- `resolve_string_pointer`: Checks if a byte at a pointer is null; if so, it replaces that pointer with a global address.
- `transfer_pointer_and_branch`: Loads a longword from an offset of negative 4 from a structure in a5 into a0, stores that value at a negative 4 offset from the same structure, and then branches to another routine.
- `push_parameter_and_call_routine`: Pushes a longword from an offset of 8 from a5 onto the stack, calls another routine, adjusts the stack pointer by 4, and branches to another routine.
- `clear_bit_and_test_longword`: Clears bit 2 of a byte at offset 8 of a structure pointed to by a0, which is retrieved from offset 8 of a structure in a5, then tests a longword at offset 12 of that same structure and branches to another routine if the value is non-zero; otherwise it returns.
- `load_and_test_pointer`: Loads two pointers from a5 and a0, then tests a longword at offset 48 of the structure in a1, branching if it is zero or otherwise branching.
- `calculate_and_store_area`: Calls another routine, loads a word from offset 12 of a structure in a0 and a word from offset 6 of a structure in a6, pushes these values onto the stack, calls another routine, and then stores the result in a structure at offset 48 of a1 before calling a third routine.
- `clear_and_test_structure_flag`: Clears a longword at offset negative 8 from a pointer stored in a local variable and returns the original value of the pointer in d0 if it was non-zero; otherwise, it sets d0 to zero and returns.
- `test_and_branch`: Tests a longword at offset 12 of a structure in a2 and branches if it is zero; otherwise, it loads the constant 1 into register d0 and branches to another location.
- `call_routine_with_stack_parameters`: Pushes two longwords onto the stack and calls another routine, then stores the contents of register d0 into a longword at an offset of negative 4 from a pointer in a5, tests if that longword is non-zero, and branches if it is; otherwise, it clears register d0 and performs an unconditional branch.
- `execute_conditional_callback_2`: Pushes two longwords onto the stack and calls another routine, then moves a word from the stack into a local variable and tests it; if non-zero, it branches to another routine, otherwise it pushes a longword onto the stack, calls another routine, and branches to an earlier instruction in this routine.
- `copy_and_test_bit_flag`: Copies a longword from an offset of 4 in a structure pointed to by a5 to an offset of 12 in a structure pointed to by a2, then tests bit 2 of a byte at offset 11 of the structure in a2.
- `execute_callback_and_clear`: Checks if a longword at offset 12 of a structure in a0 is non-zero; if so, it calls another routine using that longword as a parameter, clears the longword at offset 12, and then calls another routine using the structure pointer as a parameter.
- `process_object_callback`: Checks if a parameter is non-zero, then executes a function pointer at a specific offset from a base address, conditionally executes another function pointer using a value from an offset within the parameter, and finally returns the result of a third function call using the parameter and a constant.
- `clear_pointer_after_call`: Checks if a longword at offset 12 of a structure in a2 is non-zero, and if so, calls a function using that pointer and then clears the pointer at offset 12 to zero.
- `execute_callback_chain_2`: Calls a routine with a parameter, then executes two function pointers stored within a structure using values derived from a third function call.
- `traverse_and_execute_list`: Loads a pointer from a global variable and, if it is not null, calls a routine via a pointer found within the node and branches to continue the process.
- `initialize_animation_resources`: Attempts to allocate and initialize a series of animation bitmaps, views, and rastports by calling several subroutines to manage memory and object structures.
- `swap_structure_pointers`: Swaps two 32-bit values between two pairs of locations within a structure in a4, calls another routine, and then calls a third routine if a 16-bit value at an offset in a4 is non-zero.
- `execute_library_calls`: Calls three different routines using pointers retrieved from global variables.
- `decrement_and_recurse`: Decrements a 16-bit value at a global offset in a4 and calls another routine if the value is not zero, repeating this in a loop until the value reaches zero.
- `traverse_and_clear_list`: Recursively traverses a linked list, calling a routine on each node, performing specific memory writes, and clearing several pointers and values in a structure.
- `process_and_unlink_structure`: Loads a pointer from an offset of negative 17516 from a4 into a local stack variable, and if that pointer is non-zero, pushes it onto the stack, calls another routine, clears bit 1 of a longword at offset 11 from that pointer, and moves the longword at that pointer to an offset of negative 4 from a5; otherwise it returns.
- `unlink_data_pointer`: Moves a longword from an offset of negative 4 from a4 into an offset of negative 4 from a5, and if that value is not zero, clears bit 1 of a byte at offset 11 of a structure in a0 and updates the pointer at offset 4 of the structure in a5.
- `execute_and_unwind`: Calls another routine, and if d0 is not zero, unlinks a5 and returns; otherwise it branches to another location.
- `reset_and_execute_callbacks`: Calls several routines to clear memory and allocate nodes, performs bitwise shifts on a value from a structure, calls a routine via a pointer from a5, and if certain conditions are met, calls another routine and increments a global counter.
- `clear_and_call_routines`: Pushes a long-word from a structure in a4 onto the stack, calls another routine, clears that long-word in the structure, calls a third routine, and then branches to another location.
- `process_linked_list_state_updates`: Iterates through a linked list, updating two 16-bit values in a structure by adding values from the list nodes and conditionally setting bits in a status table based on a calculated index.
- `update_list_and_set_bits`: Iterates through a linked list, updating two values in a structure by adding values from the list nodes, and conditionally sets bits in a table based on node data.
- `process_list_and_update_state`: Checks a status word in a structure, and if certain conditions are met, updates several fields in a structure using data from a linked list.
- `initialize_nested_structures`: Calls a routine to obtain a pointer, then if that pointer is non-zero, calls another routine with that pointer, and if the result is zero, sets a 16-bit value at an offset in a4 to 1.
- `reset_structure_fields_2`: Checks a short value at an offset from a pointer, and if non-zero, writes a constant to a structure field and updates another field with a calculated value before calling another routine twice.
- `allocate_and_initialize_buffer`: Calls a routine to allocate memory, then iterates through a list of pointers to initialize them by calling another routine for each entry.
- `execute_indirect_calls`: Retrieves two function pointers from a table using an offset from a base address and executes them with provided arguments.
- `allocate_and_set_structure_2`: Clears several fields in a structure, calls a routine to obtain a pointer, and if that pointer is not zero, calls a library function to allocate memory and subsequently calls other routines using various parameters and memory sizes.
- `reset_structure_and_call_routines`: Conditionally calls other routines using pointers from a structure and then clears multiple fields within that structure to zero.
- `update_state_from_table`: Performs a series of checks and updates a structure by comparing values from a table and copying data from a source address to a destination buffer.
- `search_and_copy_string`: Calls a routine to obtain a pointer, then conditionally calls a library function to allocate memory, a routine to copy and uppercase a string, and a search routine using a value from a global variable.
- `execute_recursive_call`: Pushes two long-words from a structure in a5 onto the stack, calls another routine, and then returns.
- `update_range_values`: Calculates a difference between two values from a table and updates a third value based on specific conditions and a parameter.
- `calculate_buffer_offsets`: Calculates memory offsets based on values in a structure and a global table, then performs a series of conditional calls to update a buffer.
- `call_data_transfer_routine`: Pushes three longwords from the stack and calls another routine.
- `check_status_and_call_error`: Checks two short values at offsets from a pointer and, if both conditions are met, calls another routine.
- `move_and_branch_4`: Moves a longword from a negative 26924 offset of a structure in a4 into d0 and then branches to another routine.
- `close_file_handle_2`: Calls a routine if certain conditions are met, or calls another routine with a specific value if a pointer is non-zero.
- `set_library_parameters`: Writes specific values to a structure and then calls a routine using a pointer from a global variable.
- `update_structure_fields_and_call`: Checks a word at a negative offset from a register; if non-zero, it writes the value 32772 to offset 28 of a structure pointed to by another offset in that register, calls another routine, writes the value 9 to offset 28 of that structure, clears a 32-bit value at offset 36 of the structure, and calls another routine again.
- `copy_and_uppercase_string`: Copies up to four bytes from a source string to a local buffer while converting lowercase ASCII characters to uppercase.
- `traverse_and_update_structure_2`: Traverses a linked list within a structure, conditionally calling routines to update fields and managing a status word in a global structure.
- `process_data_blocks`: Checks a value in a global structure, then enters a loop that calls another routine and a second routine up to eight times, incrementing pointers and a counter by 512 and 1 respectively each iteration.
- `validate_and_update_structure`: Compares two words at specific offsets in a global structure with a value in a register; if they match and a third value matches the register, it returns 1, otherwise it updates the structure with new values, calls another routine, and sets a byte in a global variable based on the result.
- `call_search_routine`: Loads a pointer from a global variable and calls another routine passing a value and a length derived from a global variable.
- `search_integer_array`: Iterates backward through an array of long values to check if a specific value exists within the specified count.
- `parse_format_string`: Stores a pointer from offset 8 of a5 into a global variable, calls another routine with a pointer to a value at offset 16 of a5 and a pointer to a structure at offset 12 of a5, then clears a byte at a global offset and returns the result.
- `increment_and_store_byte`: Increments a global counter and stores a byte from a register into a location pointed to by a global pointer.
- `push_and_call_format_string`: Pushes a value from offset 8 of a structure in a5 onto the stack, pushes a PC-relative address, calls another routine, cleans up the stack, restores the frame, and returns.
- `call_recursive_routine`: A routine consisting of 48 instructions that calls another routine and returns.
- `parse_printf_format_string`: Parses a formatted string to extract numeric values, handling sign characters and format specifiers like percent signs.
- `write_short_value`: Writes the word value 8 to an offset of minus 18 from a5 and then branches to another routine.
- `write_short_value_alt`: Writes the word value 16 to an offset of minus 18 from a5 and then branches to another routine.
- `parse_formatted_string`: Parses a string to extract numeric values, handling sign characters and format specifiers like percent signs.
- `format_floating_point_string_2`: Converts a numeric value into a decimal string representation, handling signs, decimal points, and scientific notation.
- `loop_and_call_routine`: Loads values from a3 and a5, calls another routine, and if the result is not less than zero, it enters a loop that calls another routine and increments a word at offset 250 of a5.
- `modify_offset_value`: Compares a word at offset 22 of a5 to 2; if equal, it moves a word from offset 20 to offset 24 of a5, compares a word at offset 250 to 65532, and if the comparison passes, clears a word at offset 22 of a5.
- `check_and_modify_value`: Compares a 16-bit value at offset 22 of a5 to 1, and if equal, performs arithmetic on a 16-bit value at offset minus 8 of a5 and branches based on the result.
- `format_floating_point_string`: Converts a numeric value into a decimal string representation, handling signs, decimal points, and scientific notation by writing characters into a buffer pointed to by a2.
- `clear_offset_value`: Clears a word at an offset of minus 6 from a5 and then branches to another routine.
- `increment_and_store_word`: Loads a word from an offset of minus 2 from a5 into d0, increments d0 by 1, stores the result at an offset of minus 6 from a5, and then branches to another routine.
- `format_numeric_string`: Writes a decimal representation of a value into a destination buffer, handling signs and decimal points by calling several routines and updating a structure.
- `append_sign_character_2`: Writes a character to a destination pointed to by a register, decrements a counter in a structure, and if a second counter is non-zero, decrements it and writes a different character to the destination.
- `append_sign_character`: Checks a 16-bit value at an offset from a5; if it is negative, it writes a minus sign character to a destination pointed to by a2, otherwise it writes a character 'e' to the destination.
- `convert_integer_to_string`: Converts a 16-bit integer value into a null-terminated ASCII string representation.
- `push_constant_and_jump`: Pushes a constant value onto the stack and jumps to a routine at a relative offset from the program counter.
- `execute_callback_d6`: Pushes the constant value negative 40 onto the stack and jumps to a routine at an offset from the program counter.
- `push_constant_and_jump_2`: Pushes a constant value onto the stack and jumps to a routine at an offset relative to the program counter.
- `execute_callback_d0`: Pushes the constant value negative 48 onto the stack and jumps to a routine at an offset from the program counter.
- `execute_callback_e2`: Pushes the constant value negative 30 onto the stack and jumps to a routine at an offset from the program counter.
- `execute_callback_b8`: Pushes the constant value negative 72 onto the stack and jumps to a routine at an offset from the program counter.
- `execute_callback_ac`: Pushes the constant value negative 84 onto the stack and jumps to a routine at an offset from the program counter.
- `push_constant_and_jump_3`: Pushes a constant value onto the stack and jumps to another routine.
- `push_and_jump_alt`: Pushes a different constant value onto the stack and jumps to another routine.
- `execute_callback_via_structure`: Checks a global flag, potentially calls a function through a pointer, and eventually executes a routine using an offset derived from a structure.
- `skip_space_or_digit`: Checks if the byte at the address in a2 is a space or a digit; if it is a digit, it increments the address in a2 and branches to another routine, otherwise it branches to a different location.
- `check_byte_delimiter`: Compares the byte at the pointer in a2 to 45 and, if equal, sets d5 to 1 and increments the pointer; otherwise it sets d5 to 0.
- `check_delimiter_and_advance`: Compares a byte at the pointer in a2 to a constant and, if they are equal, increments the pointer and clears a register.
- `parse_numeric_string`: Iterates through a string of characters, multiplying a running total by ten and adding the numeric value of each character, then optionally negates the result based on a flag.
- `restore_registers_and_return`: Restores registers d4, d5 and a2 that were saved on the stack, removes the stack frame established by the caller, and then returns.
- `setup_and_branch`: Loads a pointer from the stack into a0, saves two words from the stack into d0 and d1, and branches to another routine.
- `fill_memory_with_byte`: Fills a memory range pointed to by a register with a byte value from another register using a loop.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and subtracts a constant to convert it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and adds a constant to convert it to lowercase if it is.
- `traverse_and_modify_linked_structures`: Checks whether the pointed-to word equals the ROM signature, then walks a list of entries, resolving each pointer through a table and zeroing table slots while copying values and filling a structure with fixed constants and computed offsets, and frees memory on exit.
- `compare_and_decrement_pointer`: Compares a byte from a memory location to a value in a register; if they match, it returns the memory location decremented by one, otherwise it returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_memory_and_call_library`: Fills a memory range with zeros and calls functions from a library based on a global pointer.
- `load_address_to_register`: Loads a constant address into register a4 and returns.
- `parse_string_into_tokens`: Parses a string by identifying tokens separated by whitespace or enclosed in double quotes, then stores these tokens into a newly allocated array of pointers.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `validate_table_pointer`: Saves registers to the stack, loads a pointer from the stack, compares the value at that pointer to a constant, and if they are not equal, increments the pointer and branches.
- `update_table_entries`: It computes offsets from a global base, calls a helper routine, then walks a table of short pairs.
- `manage_library_resources`: Checks a library status flag, invokes library functions in a loop, and updates a slot in a pointer table, otherwise invoking a different library function and returning.
- `set_limit_register`: Sets register d0 to a constant value and branches to another routine.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `fetch_and_update_structure`: Retrieves a pointer from a structure at offset 8 of a5, calls another routine, and if the result in d0 is non-zero, stores it in a global variable at offset -10468 of a4.
- `update_window_structure`: Retrieves a pointer from a structure, calls a routine to fetch a sub-object, and if successful, updates a structure at offset 160 with a value derived from the sub-object's address and stores the sub-object's pointer at offset 156 and its scaled address at offset 164.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Checks the sign of d0 and d1, negates them if they are negative while setting d4 to one if d0 was negated, then calls another routine and branches to a different location.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `call_remove_linked_list_node`: Loads a word from offset 8 of a structure in a5, calls another routine with that word and a pointer from offset 10 of a5, and returns.
- `process_linked_list_node`: Loads a word from offset 8 of a5, calls a routine with that word and a pointer from offset 10 of a5, and if the word is 10 and bit 7 of a byte at offset 12 of the structure at offset 10 of a5 is set, calls another routine with a pointer from offset 10 of a5 and a word of 65535.
- `update_linked_list_node`: Compares a value at an offset of 4 from a pointer in a2 with a value at the address pointed to by a0; if the comparison does not set the carry flag, it calls another routine, otherwise it updates a byte at an offset of 9 from a5 into the address in a0 and continues a loop.
- `traverse_linked_list_2`: Iterates through a list of structures by incrementing a pointer by 22 and calling another routine for each entry until a comparison with a value derived from a4 fails.
- `remove_linked_list_node_2`: Checks if a pointer from a global variable is null, then if not, checks bit 2 of a byte at offset 12 of the structure; if set, it calls a routine, performs bitwise operations on a value from offset 13, calls another routine, and potentially calls two more routines via pointers at offset 8 and 18 before clearing the structure's fields and returning.
- `modify_linked_list`: Manipulates a linked list structure by checking specific bits in a control byte, performing calculations via a routine, and updating pointers or clearing fields based on the results.
- `find_first_empty_structure`: Iterates through a list of structures by incrementing a pointer by 22 and checking bit 12 of each structure, returning a pointer to the first structure where that bit is zero or returning zero if the end of the list is reached.
- `branch_to_offset_2`: Performs a short branch to a specific instruction offset.
- `clear_structure_and_branch`: Clears the longword at offset 0, the longword at offset 4, and the longword at offset 8 of the structure in a2, moves a2 to d0, and then branches to another routine.
- `check_table_entry_2`: Calls a routine with the constant 1024, and if the result is zero, sets a word at offset 16 of a structure to 1 and updates a pointer at offset 8; otherwise, it sets offset 16 to 1024, sets bit 1 of a byte at offset 12, updates offset 8 with the result, and potentially calls another routine and sets bit 7 of the byte at offset 12.
- `iterate_linked_list_and_call_function`: Traverses a linked list starting from a pointer stored in a global variable, passing each node and a value derived from the node's second word to another routine.
- `iterate_linked_list_and_call_function_2`: Stores a routine address into a global variable, calls another routine with a pointer derived from a value at offset 8 of a structure, and if the result is not null, updates a global pointer to point to the new node and continues the list traversal.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_linked_list_node`: Searches for a specific node within a linked list, removes it by updating the previous node's pointer, and calls another routine with the removed node.
- `check_table_entry`: Calculates an offset into a table based on a value from a stack parameter, checks if the resulting entry is valid and non-zero, and calls another routine if it is.
- `handle_file_deletion`: Calls a routine to delete a file, and if that fails, calls another routine to handle an error and updates a global variable.
- `process_write_operation`: Calls another routine, multiplies the result by 6, and uses the result to access a structure; it then performs conditional logic to call a write routine or set a global variable based on specific bitwise checks and results.
- `execute_write_operation_2`: Calls a routine with a pointer to memory at offset 1000, and if bit 12 of the result is set, checks a global variable and potentially calls another routine before returning the result.
- `execute_write_operation`: Calls a function to write four bytes from a specific memory location and then calls another routine to handle the result.
- `execute_callback_and_routine`: Checks if a global function pointer is non-zero to execute it, then calls another routine.
- `validate_and_clear_pointer`: Calculates an offset into a table based on a value from the stack, checks if that value is within a specific range, and if the resulting pointer is null, sets a global variable to 2 and returns -1; otherwise, it checks a bit in the pointed-to structure, potentially calls another routine, clears the pointer, and returns 0.
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
- `jump_to_indirect_address`: Performs an indirect jump to an address stored in a location derived from a global variable.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `initialize_structure_and_call_subroutines`: Calls a routine to get a value, then if that value is not negative one, calls a library function to allocate memory, populates the new structure with values from the stack, and calls several other routines using the new structure or its members.
- `reset_structure_fields`: If a value at offset 10 of a structure is non-zero, calls another routine, then sets a byte at offset 8 to 255, a longword at offset 20 to all ones, calls another routine using a byte from offset 15, and calls a library function.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_allocate_with_base`: Pushes the constant 48 onto the stack and calls another routine passing that value along, then cleans up the stack and returns.
- `call_with_frame_pointer_2`: Pushes the pointer stored at offset 8 of the frame onto the stack, calls a routine via a PC-relative address, removes the pushed value, and returns.
- `allocate_and_set_structure`: If a provided pointer is not null, calls a library function to allocate memory, and if successful, sets a byte at offset 8 to 5, a word at offset 18 to a value from the stack, and a longword at offset 14 to the provided pointer.
- `reset_structure_and_call_library`: If a pointer in a6 is not null, sets a byte at offset 8 to 255, sets longwords at offsets 20 and 24 to all ones, and calls a library function using a word from offset 18 and the pointer.
- `indirect_jump_stub_2`: Jump stub: passes control to an address held in or computed from register a4, so the target is only known at run time.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `copy_data_and_update_structure`: Copies eight longwords from a global table to the stack, calls a routine, and if successful, updates a structure with values from the stack and calls three different routines.
- `call_with_frame_pointer`: Pushes the pointer stored at offset 8 of the stack frame onto the stack, calls a routine via a PC-relative address, and returns.
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
- `jump_to_library_function`: Performs an indirect jump to a function address retrieved from a global variable.
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
- `set_register_bits`: Sets various bits in registers d0 and d1 using logical OR operations.

### 8

- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.

### 7

- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.

### 9

- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `set_registers_with_constants`: Performs a series of bitwise OR operations on registers D0 and D1 using various immediate byte values.

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

- `executes_illegal_instruction`: This routine executes an illegal instruction and does not return.
- `store_a0_to_global`: As library code, it stores the value of register A0 into a long word at a fixed memory location and returns.
- `unlink_and_free_memory`: If a word at offset 32 of the structure in a6 is zero, it unlinks a doubly linked list by updating pointers and calls a routine via a pointer in a global variable; otherwise, it sets bit 3 of a byte at offset 14 of the structure in a6 and returns.
- `set_bit_in_structure`: As library code, it sets bit 3 of a byte at an offset of 14 from the address in A6 and then returns with D0 cleared.
- `increment_and_call_library`: As library code, it checks bit 3 of a byte at offset 14 from A6; if not set, it increments a 16-bit value at offset 32 from A6 and, if that value is 1, calls another routine using a pointer from a global variable and returns the result in D0.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `decrement_counter_and_call`: Library code that decrements a 16-bit counter at offset 32 of the structure at a6, calls another routine if the counter reaches zero and bit 3 of the flag byte at offset 14 is set, and calls another routine with a global value when the counter is zero.
- `mask_and_branch_2`: Library code that masks the lower 7 bits of a byte in d0, calls another routine, and sets bit 1 of the condition code register before returning.
- `calculate_bitfield_difference`: If the byte in d0 is non-zero and positive, it performs a series of bitwise shifts and arithmetic operations involving a doubling accumulator and a running difference, eventually branching to another routine.
- `adjust_bitfield_value`: As library code, it sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `mask_and_branch`: Copies d0 to d7, calls a routine via a pointer in a global variable, sets bit 1 and clears bit 3 of d0, then branches to another routine.
- `call_jump_table_entries`: As library code, it calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_library_register`: As library code, it clears the long register d1 and branches to another routine.
- `execute_library_math_logic_2`: Library code that loads a pointer from a table into a5, calls a routine at a negative offset from a5, performs several conditional checks using other library routines via offsets from a5, and finally calls two more routines via bsr before branching to another routine.
- `dispatch_conditional_math_operations`: Moves a value from d6 to d4 and calls a routine at a negative offset from a5 with d5 and zero as arguments.
- `execute_library_math_logic`: Library code that performs conditional calls to various library functions based on the values of d4 and d6, using d5 and d7 to manage data flow through multiple function pointers.
- `execute_conditional_library_calls`: Moves a value from d7 to d1 and calls a routine at an offset from a5; if the result is not negative, it checks d4 and either calls a routine at an offset from a5 with a constant in d1 or calls two routines at offsets from a5 using a constant in d1 before storing the result in d2 and branching to another routine.
- `call_and_store_result`: Copies d2 to d0, calls a routine at an offset from a6, stores the result back in d2, and branches to another routine.
- `conditional_load_constant`: Copies d2 to d0, checks if d4 is non-zero, and if d4 is zero, loads a constant into d2 before branching to another routine.
- `mask_return_value`: As library code, it calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `calculate_library_value`: Library code that performs bitwise operations on d7, calls a routine via a pointer in a list, and performs arithmetic to determine a final value.
- `initialize_library_registers`: Library code that performs a logical shift right on d7, clears d6, and loads several constants into d5, d1, and d2 before branching to another location.
- `execute_loop_with_branching`: The library code executes a loop that performs bit-shifting and decrementing, potentially calling other routines if a counter reaches a certain value.
- `push_and_branch`: As library code, it pushes the word negative 16382 onto the stack and branches to another location.
- `push_and_branch_2`: As library code, it pushes the word negative 16383 onto the stack and branches to another location.
- `push_and_branch_3`: As library code, it pushes the word 1 onto the stack and branches to another location.
- `compare_and_adjust_stack`: As library code, it compares a byte in d7 to 184, branches if higher, otherwise it increments the stack pointer by 2, tests d7, and returns.
- `library_check_range`: As library code, it clears the stack, tests d7, and if d7 is between 56 and 138, it saves registers, moves d7 to d2, doubles d7, and checks if the new value is between 138 and 168 before setting the overflow flag and restoring registers.
- `calculate_bit_shifted_values`: Calls another routine to transform a value in d2, performs bitwise masking and arithmetic to calculate a shift amount, and then calls two more routines to perform further arithmetic on d2 and d7 before executing a loop that updates d5 and d6 using values from a table pointed to by a0.
- `process_math_loop_2`: As library code, it performs arithmetic on d2 and d4, and if d2 is greater than 31, it calculates a value in d7 using a logical shift right and subtraction before branching.
- `process_math_loop`: Library code performs arithmetic on d7 using values from a table pointed to by a0, updating d5 and d6 within a loop that runs 23 times.
- `process_math_data`: Library code adds d4 to d5, subtracts d3 from d6, and adds values from a memory pointed to by a0 to d7 within a loop.
- `load_constant_and_branch`: As library code, it loads the constant an address into d7 and branches to another location.
- `recursive_call_wrapper`: Library code that returns immediately when a register is zero, otherwise calls another routine and restores the stack before returning.
- `wrap_library_call`: Library code that pushes d5 to the stack, calls a routine via a branch and link, and then manipulates d6 and the stack before branching.
- `compare_and_branch`: The library code moves d6 to d4, and if d6 is not negative, checks if d6 is equal to 255; if it is not higher than 255, it clears d6 and returns.
- `shift_and_jump`: As library code, it arithmetic shifts d4 right by 8, increments d4, and branches if the result is zero; otherwise it jumps to a location via an absolute address.
- `return_without_effect`: Library code masks d7, calls another routine, and then calls it again if the overflow flag is not set.
- `call_other_routine_conditionally`: Library code calls another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_logic_2`: Library code tests d7, and if non-zero, increments it and calls another routine.
- `rotate_and_branch`: As library code, it loads the constant an address into d7, rotates d7 right by 1, and branches to another location.
- `load_constant_and_branch_2`: As library code, it loads the constant an address into d7 and branches to another location.
- `test_and_branch_negative`: Loads a 16-bit word from the stack into d6, tests the byte in d6, and branches if it is positive; otherwise it clears d7 and branches to another location.
- `set_negative_flag_and_return`: Sets the negative flag in the status register and restores registers from the stack before returning.
- `load_constant_and_return`: Loads a constant value into a register, adjusts the stack pointer by eighteen bytes, and returns.
- `normalize_library_value`: Library code that performs bitwise operations on d7, calls a routine via a pointer, and uses the result to perform shifts and further calls to other routines.
- `normalize_fixed_point_value_2`: Library code that calculates a shift amount based on d2, calls another routine three times with varying parameters, and then performs arithmetic on d6 using a value from a pointer in a0 and a value from the stack.
- `subtract_table_values`: The library code increments d2, moves d5 and d6 to d3 and d4, performs arithmetic shifts on d3 and d4, and then enters a loop that adds d4 to d5, adds d3 to d6, and subtracts a value from a table pointed to by a0.
- `subtract_table_from_accumulator`: Subtracts d4 from d5, subtracts d3 from d6, adds a value from a table pointed to by a0 to d7, and loops using d1 as a counter.
- `test_sign_and_branch`: Library code that tests the sign of d7, and if it is negative, performs a bitwise AND on d7, calls a routine, and sets the carry flag.
- `set_d7_and_branch`: Sets d7 to negative 255 and branches to another location.
- `execute_complex_math_sequence`: Library code that calls three different routines via pointers, then performs a loop involving bit-shifting and subtraction based on a value derived from d7.
- `complex_loop_with_branching`: The library code executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_byte`: Library code tests a byte in d7, masks it with 127 if it is not positive, calls another routine, and sets bit 1 of the condition code register.
- `execute_complex_branching`: Library code calls another routine, saves registers to the stack, calls a second routine, and then jumps to another location.
- `transform_register_value`: Adds the value of a register to itself, then conditionally performs a series of bitwise operations including an XOR, an arithmetic shift, a subtraction, a word swap, and a left rotation before returning.
- `transform_byte_value`: Library code that swaps d7, performs a long rotate, executes an exclusive OR, and then performs conditional additions and rotations.
- `compare_and_modify_d7`: This library code compares the value in d7 to 124 and 126, and if a match is found, adds 133 to d7 and performs a rotate left through the carry flag.
- `test_masked_word`: Performs a bitwise AND on d7 with a mask, tests the result, and branches if the result is zero; otherwise it clears d7 and branches to another location.
- `check_and_shift_d7`: The library code compares d7 to 254, and if it is not equal, performs two logical shifts on d7 and branches based on the result.
- `shift_and_set_flag`: Shifts d7 left by 8 bits, sets d7 to 255, performs a rotate through carry on d7, sets bit 1 of the status register, and branches to another location.
- `clear_d7_and_branch`: Sets d7 to zero and branches to another location.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `load_address_and_branch_3`: The library code loads a longword into a0 from an offset of 12e and then branches to the next instruction.
- `load_address_and_branch_4`: As library code, it loads a long value into a0 and branches to another routine.
- `load_pointer_from_offset`: Library code loads a longword from an offset of 40 in a table into a0 and returns.
- `load_zero_pointer`: Library code that loads the address 0 into a0 and then branches to another routine.
- `load_address_and_branch_5`: As library code, it loads a longword into a0 and branches to another location.
- `load_address_to_a0`: As library code, it loads a long value from a location in memory into register a0 and then branches to another location.
- `load_address_to_a0_offset_6`: As library code, it loads a long value from a location at offset 6 in memory into register a0 and then branches to another location.
- `store_register_via_pointer`: The routine invokes another routine and then stores a 32-bit value from register d7 into memory addressed by a pointer passed in register a0, then returns.
- `load_pointer_from_offset_alt`: Library code loads a longword from an offset of 52 in a table into a0 and returns.
- `load_null_pointer_2`: Library code loads a zero value into a0 and returns.
- `load_pointer_from_offset_6e`: Library code loads a longword from an offset of 110 in a table into a0 and returns.
- `load_offset_pointer`: Library code that loads the address 38 into a0 and then branches to another routine.
- `load_null_pointer_and_branch`: Loads a null pointer into a0 and performs an unconditional branch to the next instruction.
- `load_pointer_from_offset_cc`: Library code loads a longword from an offset of 204 in a table into a0 and returns.
- `load_address_and_skip`: As library code, it loads a long value into a0 and branches to an instruction 20 bytes ahead.
- `load_null_pointer`: Loads a zero value into a0 and branches to another routine.
- `load_constant_pointer`: Library code that loads the address 20 into a0 and then branches to another routine.
- `load_offset_and_branch_to_indirect_call`: Loads a pointer from a global variable into a0, adds a negative offset of 30 to that pointer, and branches to another routine that performs an indirect call.
- `load_offset_and_branch`: Loads the address of a global variable into a0 and branches to another routine.
- `load_offset_and_branch_2`: Loads a negative offset into a0 and branches to another routine.
- `load_address_and_branch`: Loads the address of a hardware register into a register and performs a short branch to another routine.
- `load_pointer_and_branch`: As library code, it loads a pointer from a global variable into a0 and branches to another location.
- `load_offset_pointer_and_branch`: Loads a pointer from a global variable into a0 and branches to another routine.
- `load_address_and_branch_to_offset`: Loads a longword from a global variable into a0 and branches to a short offset within another routine.
- `load_address_and_branch_to_offset_2`: Loads a longword from a global variable into a0 and branches to an offset within another routine.
- `load_address_and_branch_2`: Loads a longword address into a register and performs a short branch to another routine.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.

### mathieeedoubbas.library

- `residentMI`: The library code executes an illegal instruction.
- `libInit2`: The library code stores the pointer from A0 into a global variable and returns.
- `MIExpunge`: Checks a byte at offset 32 of a structure in a6; if it is non-zero, it sets bit 3 of a byte at offset 14 of that structure and returns; otherwise it unlinks a node from a list, calculates a memory range using offsets 16 and 18, calls a function via a pointer at offset 4 of a6, and returns a pointer from a global variable.
- `Exp_InUse`: As library code, it sets bit 3 of a byte at offset 14 of the structure pointed to by a6 and returns.
- `MIOpen`: Tests bit 3 of a byte at offset 14 of a structure in a6; if set, it branches to another location; otherwise it increments a word at offset 32 of the structure, sets d0 to the address in a6, and returns.
- `openFail`: As library code, it returns a value of negative one in d0.
- `MIClose`: Decrement a short counter stored at offset 32 of the structure in a6, and when the counter reaches zero and bit 3 of the byte at offset 14 is set, call the library expunge routine and return its result, otherwise return zero.
- `DPFix`: The library code moves a pointer from d0 to a1, saves registers to the stack, and performs a series of bitwise operations and a loop to process data from the pointer in a1.
- `dfi4`: Tests bit 1 of d0; if set, it branches to another location; otherwise it clears d1 and branches to another location.
- `dfi6`: Clears d1 and branches to another location.
- `dfix1`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `DPFlt`: As library code, it converts a signed single-precision value in d1 into a double-precision representation stored through the pointer in d0, computing an exponent and mantissa by shifting the input, assembling two 32-bit words while preserving the sign, and writing them via the pointer.
- `DPAbs`: As library code, it copies the high word of a double from the source pointer in d1 to the destination pointer in d0 and writes the low word with its sign bit cleared, producing an absolute value of a double-precision number.
- `dcmp_1`: Small routine of 2 instructions that subtracts 1 from d0, then returns.
- `dcmp_2`: Small routine of 2 instructions that adds 1 to d0, then returns.
- `dtst_1`: As library code, it subtracts one from the value in d0 and returns.
- `dtst_2`: As library code, it adds one to the value in d0 and returns.
- `cc_1`: As library code, it calls a function whose address is passed in an address register and then returns.
- `cc_2`: As library code, it calls a function whose address is passed in an address register and then returns the contents of a stack slot.
- `cc_3`: As library code, it calls a function pointer held in register a1 and returns the value left on the stack at offset 18.
- `MIIEEEDPFix`: This library code calls another routine and then returns.
- `MIIEEEDPFlt`: Library code that calls a floating point routine and returns the value previously on the stack.
- `MIIEEEDPCmp`: Library code that loads an address into a1 and jumps to another routine.
- `MIIEEEDPTst`: Loads the address of another routine into a1 and calls it via a tail branch.
- `MIIEEEDPAbs`: Loads an address from a relative offset into a6 and calls another routine via a tail branch.
- `MIIEEEDPNeg`: Loads a function address from a table using a relative offset and calls it via a tail branch.
- `MIIEEEDPAdd`: Loads an address from a relative offset into a register and calls a double-precision addition routine via a tail branch.
- `MIIEEEDPSub`: Loads an address from a relative offset into a register and calls another routine via a tail branch.
- `MIIEEEDPMul`: Loads an address from a relative offset into a1 and performs a tail branch to another routine.
- `MIIEEEDPDiv`: Loads an address from a table using the program counter and calls the DPDiv routine via a tail branch.
- `MIIEEEDPCeil`: Library code calls a negation routine, then a floor routine, and finally calls the negation routine again via a tail branch.
- `MIIEEEDPFloor`: Library code that calls three different routines to fix, compare, and test values, then moves d6 and d7 into d0 and d1 before returning.
- `test_d6_d7`: This library code moves d6 and d7 into d0 and d1, calls another routine, and if the result is not less than zero, it restores registers from the stack, moves d4 into d0, clears d1, and calls another routine before branching.
- `lt_zero`: This library routine stores a constant into a stack slot and then tail calls another routine.
- `fptrap`: As library code, it retrieves a word from a pointer stored at offset 16 of the stack, updates that pointer to point to the next word, masks the word with 31, shifts it left by two, and jumps to an offset determined by the result.
- `fpoptab`: Jump stub: passes control straight on to badtrap.
- `jump_to_badtrap`: Jump stub: passes control straight on to badtrap.
- `dfsub`: As library code, it saves several registers to the stack, moves d1 and d2 into a0 and a1, loads a longword from the address in a1 into d3, performs a bitwise change on d3, and branches to another routine.
- `dfadd`: Performs addition of two double-precision floating-point numbers stored in memory pointed to by two registers, writing the result back to the first memory location.
- `dfa11`: Subtracts two double-precision floating-point values by manipulating their sign, exponent, and mantissa components, then stores the resulting value into a structure pointed to by a0.
- `dfa14`: Sets register d0 to negative an address, clears d7, performs a bitwise NOT on registers d1 and d2, adds 1 to d2, and adds d7 to d1 with the carry bit before branching to another routine.
- `dfa13`: Library code performs a loop to shift values in d3 and d4, then performs addition with carry into d1 and d2, and branches to another routine based on the result.
- `dfa5`: Library code performs bitwise negation and addition on d1 and d2, then enters a loop that shifts values and adds d4 to d5 based on comparisons with a constant.
- `dfa8`: Compares register d1 with d4, and if d1 is less than d4, subtracts d4 from d5, shifts d2 left by 1, rotates d1 left by 1 with the carry bit, and branches to another routine.
- `dfa7`: This library code tests d5 and, if it is not less than or equal to zero, clears bit 20 of d1, performs logical OR operations with d5 and d0, stores the result in a location pointed to by a0, stores d2 at an offset of 4 from that same location, tests the value at that location, restores registers from the stack, and branches.
- `dfa9`: Library code that clears a 32-bit long word pointed to by a0, clears a 32-bit long word at offset 4 of that same structure, and branches to another routine.
- `dfauo`: Library code that compares a long value in d5 against a constant, and if it is less than that constant, stores a value composed of a bitwise OR between d0 and a constant into a pointer in a0 and stores a byte into offset 4 of that same pointer, then branches to another routine.
- `dfmul`: Performs a double-precision multiplication of two 64-bit values stored in memory, handling sign bits and overflow, and writes the resulting 64-bit product back to the first memory location.
- `dfm4`: As library code, it clears the longword at a0 and the longword at offset 4 from a0, then branches to another routine.
- `dfmuo`: Library code that compares a long value in d0 against a constant, and if it is less than that constant, stores a value composed of a bitwise OR between d4 and a constant into a pointer in a0 and stores a constant into offset 4 of that same pointer, then branches to another routine.
- `dfdiv`: Performs double-precision floating-point division using a software-based iterative algorithm.
- `dfd10`: Executes a loop to perform division on 64-bit values using a bit-shifting algorithm, then stores the resulting quotient and remainder into a memory location pointed to by a register.
- `dfd4`: Performs arithmetic on values in d0 and d5 to compute a result, then stores the resulting 32-bit words into a table pointed to by a0 before calling another routine.
- `dfsetz`: As library code, it sets d5 and d6 to zero and then branches to another routine.
- `dfdbyz`: Loads the value 7 into d3 and performs a short branch to another location.
- `dfduo`: Library code that compares a long value in d7 against a constant, and if it is less than that constant, performs bitwise operations to prepare d5 and d6 before updating the condition code register and branching to another routine.
- `dfneg`: As library code, it moves a pointer from d1 to a0, reads a word from a0, and if the word masked with 32752 is not zero, it performs an eor on the byte at a0; otherwise it branches to another routine and calls another routine.
- `dfcmp`: As library code, it compares two structures pointed to by d1 and d2 by checking their masked longword values, performing logical operations, and updating the condition codes before calling another routine.
- `dftst`: This library routine loads a pointer from d1, reads a 16-bit word from that location into d0 and d3, and if the word at the pointer is not zero when masked with 32752, it returns; otherwise it restores registers and returns.
- `badtrap`: Small routine of 1 instruction that copies 224 to d1.

### version.library

- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_pointers`: The library routine writes two 32-bit values from registers into a structure pointed to by a register, storing one value at an offset and another at a different offset.
- `increment_counter_and_clear_flag`: The library routine increments a short value at an offset from a base pointer and clears a bit in a byte at another offset from the same base.
- `decrement_counter_and_call_2`: Library code that decrements a 16-bit counter at offset 32 of a structure in a6, and if the counter reaches zero and bit 3 of a byte at offset 42 of that structure is set, it calls another routine.
- `get_library_value_2`: As library code, it retrieves a value from offset 34 of a structure in a5, sets bit 3 of a byte at offset 42 of that structure if a word at offset 32 is not zero, and otherwise branches to another routine.
- `get_library_value`: As part of a shared library, this routine retrieves a value from an offset in a passed pointer and calls two other functions via pointers.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `branch_to_font_handler`: This library routine performs a short branch to another routine and, if the result is non-zero, stores the value from a register into a structure at offset 34 and calls another routine via a table entry.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_structure`: This library routine performs several memory updates and calls two functions via pointers, potentially updating a linked list or structure if certain conditions are met.
- `unlink_font_nodes`: Iterates through a linked list of structures, unlinking nodes where a 16-bit value at offset 132 is not greater than zero by updating pointers and calling another routine.
- `manage_font_memory`: As library code, it checks two word values at offsets 38 and 40 of a structure in a6; if both conditions are met, it calls three different routines via pointers, performs pointer manipulation on a list, and returns; otherwise it sets bit 7 of a byte at offset 14 of the structure and returns.
- `set_status_bit`: This library routine sets bit 7 of a byte at an offset from a base pointer and sets d0 to zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_jump`: This library routine decrements a 16-bit word at an offset from a base pointer, checks bit 7 of a byte at another offset, and then jumps to a location relative to a base pointer.
- `return_constant_15`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `update_font_character_metrics`: Iterates through a table of font structures to find the entry with the highest metric value.
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
- `increment_pointer_on_non_zero_byte`: Library code that pushes two longwords onto the stack, loads a pointer from an offset of the stack into a0, tests the byte at that pointer, and either branches to a different location or increments the pointer by 1 before branching to another location.
- `call_other_routine_stub`: This library routine pushes two longwords onto the stack, calls another routine, and then restores the original values of the two longwords from the stack before returning.
- `calculate_string_length`: As library code, it iterates through a null-terminated string to count the number of characters before the terminator and returns the total count in d0.
- `compare_byte_table`: This library routine compares a byte from a pointer in a0 with a byte from a table pointed to by a1, and if they match, checks if the byte at a0 is non-zero.
- `compare_byte_with_table`: Copies a byte from a pointer in a0 to d0, decrements a1, sets a0 to the new value of a1, and compares the byte at the new a0 with d0, optionally setting d0 to 255 if the byte is not less than or equal to d0.
- `return_constant_16`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_branch_byte`: This library routine loads a byte from the address in a0 and compares it with a byte from the stack, then branches to another routine if they are equal or if the byte is zero.
- `increment_pointer_and_branch`: This library routine increments the value in a0 by 1 and then branches to another routine.
- `return_constant_17`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations with different constants.
- `populate_font_metadata_table`: Iterates through a linked list of font data structures, calculates the remaining buffer space, and copies string data into a destination buffer.
- `update_font_buffer_pointer`: Decrements a destination pointer stored at a negative offset from a6 by the value in d4, then calculates a remaining byte count by subtracting the value in d4 plus 10 from a value in d2.
- `copy_font_data_loop`: Copies a sequence of bytes from a source pointer in a1 to a destination buffer pointed to by a value at a negative offset from a6, incrementing a counter in d3 for each byte.
- `parse_font_data_stream`: Checks a bit in a register and, if set, opens a file, locks it, and examines its contents to locate font data.
- `decrement_buffer_pointer_and_copy`: Subtracts a 16-bit value from a pointer stored at offset negative 4 of a stack-based structure, decrements a counter in register d2 by that same value, and if the counter remains non-negative, calls copy_buffer_bytes_loop to copy bytes from a source buffer located at offset 8 of a structure pointed to by a3.
- `copy_buffer_bytes_loop`: Copies bytes from a source buffer into a destination buffer pointed to by a value in a stack-based structure, incrementing a counter for each byte until a specified limit is reached.
- `read_and_parse_disk_data`: Reads a block of data from the disk and, if the read length matches a specific constant, parses the data into a structure by copying a 4-byte longword, a 2-byte word, and two individual bytes into a destination buffer.
- `execute_library_calls_2`: This library routine calls three different subroutines using arguments from the stack and returns either the absolute value of a value in d2 or zero if d2 is non-negative.
- `return_zero_constant`: The library routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf:bootblock

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
- `test_multiple_bits`: Calls another routine four times, performing a bitwise AND on d0 with different constants after each call, and finally tests bit 5 of d0.
- `execute_repeated_calls`: Calls another routine eighteen times in succession.
- `update_byte_with_logic`: Calls another routine four times, performs a bitwise OR between a byte from a pointer and a byte at a memory location, and then calls another routine twice.
- `test_bit_after_call`: Calls another routine and then tests bit 4 of d0.
- `process_and_push_bytes`: Calls another routine four times, performing bitwise XOR operations on d0 with different constants, tests bit 5 of d0, pushes a byte from the pointer in a2 onto the stack, and finally pushes a byte from an offset of 24576 from a0 onto the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transfer_and_store_data`: Calls another routine four times, tests bit 5 of d0, moves a longword from a pointer in a4 to a pointer in a0, compares d0 to 192, tests bit 6 of d0, and stores a constant into a location pointed to by a6.
- `test_bit_and_or`: Calls another routine and then tests bit 7 of d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_reassign_d0`: Calls a subroutine and then moves the byte value from register d0 to itself.
- `modify_memory_and_registers`: Calls a subroutine at offset 100, masks the lower 8 bits of register d0, performs a bitwise AND on a 16-bit word at the address in a2 using the mask 24736, and sets register d6 to 4352 and register d0 to 0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `push_byte_and_clear_d0`: Calls a subroutine at offset 36, pushes the lower 8 bits of register d0 onto the stack pointed to by a1, and sets register d0 to 0.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_buffer_state`: Updates internal audio playback structures by checking hardware status and synchronizing buffer pointers.
- `copy_d0_to_d5`: Copies the 32-bit value from register d0 to register d5.
- `cleanup_and_release_resources`: Iterates through a list of function pointers to execute them and then releases allocated memory blocks.
- `initialize_system_state`: This routine performs a sequence of calls to populate various offsets of a structure in a5 with returned values, and if any call fails, it returns minus one; otherwise, it calls another routine and returns its result.
- `load_pointers_into_registers`: Loads a pointer from a table into a register, then loads two additional pointers from the memory locations pointed to by the first two registers before returning.
- `return_constant_value`: Calculates a constant value by subtracting one address from another and shifting the result right by one, then returns it in d0.
- `return_address_pointer_2`: Loads the address of a label into d0 and returns.
- `execute_sequence_of_subroutines`: Saves registers to the stack, calls six different subroutines, and then performs a short branch to an earlier instruction in this routine.
- `process_data_and_call_subroutines`: Saves registers to the stack, calls several subroutines using values from the stack, and if a specific condition is met, writes a byte to offset 25 of a structure in a5 before calling more subroutines and performing a branch.
- `call_library_function_directly`: Calls a library function using a pointer from a structure in a6, then checks bit 31 of the return value and returns.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `process_nested_calls`: Sets a 32-bit value in a structure to all ones, calls a function from a pointer, and if that returns non-zero, calls several more functions via pointers and a global routine.
- `open_library_and_release`: Sets a result field to minus one, opens the dos.library name through the exec base, and when the open succeeds calls two methods on the returned base before closing it.
- `initialize_trackdisk_device_request`: Calls another routine, populates a device request record with a value from a register and a length of 48, and then calls a library function using a pointer from a global variable.
- `store_two_library_results`: Calls two library functions and stores their results in the state record and a second record, one as a long and one as a byte.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_and_branch`: Calls a library function using a pointer from a structure and then calls another routine.
- `copy_and_branch_long`: This routine calls another routine, copies a longword from offset 32 of the structure in a1 to offset 48 of the same structure, and then branches to another routine.
- `update_record_fields_3`: Writes a constant value of 32770 into a structure at offset 28, copies two 32-bit values from another structure, calculates two values using arithmetic on registers and scales them by 512, and calls another routine.
- `initialize_record_fields`: Sets a byte at offset 8 of a structure to 5, a word at offset 28 to 32778, and copies two 32-bit values from another structure into offsets 40 and 46.
- `set_record_specific_value`: Loads the word 32773 into d4 and branches to another routine.
- `set_record_small_value`: Loads the value 13 into d4 and branches to another routine.
- `set_d3_and_branch`: This routine sets d3 to one and branches to another routine.
- `set_record_high_value`: Clears d3, loads the word 32777 into d4, and branches to another routine.
- `update_record_fields`: Writes several values into a record, including a constant, two values from another record, and two calculated values scaled by a power of two.
- `register_disk_resource_handler`: Obtains the disk.resource pointer through a library call and stores it in the state record, together with a fixed value and a code pointer to the handler.
- `invoke_disk_resource_routine_3`: Calls a function on the disk resource base held in the state block, and if it returns zero, calls another function and repeats the process until a non-zero value is returned, then calls a third function and writes values to hardware registers before branching to another routine.
- `invoke_disk_resource_routine`: Calls one function on the disk resource base held in the state block.
- `configure_hardware_and_call_subroutine`: Sets the interrupt enable and DMA control registers via the custom chip base, writes a value to the DMA address register, and calls a subroutine using a pointer from a table before branching to another location.
- `branch_to_target`: Performs a short branch to a specific address.
- `update_flag_byte_bits`: Clears the bit selected by the first register value in a state byte, then sets bit two when the second register value is zero.
- `copy_state_byte_to_register`: Copies a state byte into a control word and writes a fixed flag value to a second control word.
- `start_disk_sync_transfer`: Calls a helper, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers.
- `set_top_bit_field`: Stores the incoming value with its top bit set into a 16-bit field of the record addressed by the second register, twice.
- `reset_record_and_call_library`: Sets a word at offset 36 of a structure in a0 to 16384, sets a word at offset 156 of the same structure to 2, sets a bit in a byte from a1, and calls a routine using a pointer from a1.
- `calculate_consecutive_count`: Combines bits from two longwords at offsets of 2 and 4 from a pointer, compares the result against a value derived from two bytes, and if they match, counts consecutive occurrences of a specific word in a memory range.
- `count_consecutive_values`: Calculates a memory range based on a value in a0, then iterates through the range to count how many times a specific word value repeats consecutively, returning the count.
- `forward_to_helper_routine`: Calls one helper routine and returns with no other work.
- `compute_bitwise_checksum`: Computes a 16-bit checksum over a run of words by shifting each bit through the accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `clear_bit_and_increment_counter`: Clears bit 7 of a byte at offset 14 from the base address in a6, increments a 16-bit word at offset 38 from the base address in a6, and returns the base address in d0.
- `conditional_jump_based_on_stack`: Decrements a word at offset 38 of the stack frame, otherwise it returns if certain bits in a byte at offset 14 of the stack frame are not set, or jumps to a routine using a pointer from the stack frame.
- `return_constant_18`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_dual_subroutine_calls`: Saves registers to the stack, calls one routine with a pointer in a0, cleans the stack, and then calls a second routine with a pointer in a0 before returning.
- `populate_structure_from_string_list`: Iterates through a linked list of data structures, calculates required buffer space by measuring string lengths, and copies string data into a destination buffer.
- `decrement_buffer_pointer_and_copy_string`: Decrements a pointer stored at offset negative 4 from a6 by the value in d4, then decrements a counter in d2 by the value in d4 plus 10.
- `copy_data_to_structure_and_update_list`: Copies a sequence of bytes from a source pointer to a destination buffer using a loop, then populates a structure with a pointer, two 16-bit values, and two bytes from a source table before incrementing a counter and jumping to another routine.
- `process_font_string_buffer`: Checks a bit in a register and, if set, performs a series of calls to retrieve a font handle and its associated data.
- `decrement_list_pointer_and_copy_buffer`: Subtracts a 16-bit value from a pointer stored at offset negative 4 of a6, then decrements a counter in d2 by that same value and calls copy_buffer_to_entity_list if the counter remains non-negative.
- `copy_buffer_to_entity_list`: Copies bytes from a source buffer into a destination list by iterating through a loop until a counter reaches zero.
- `update_entity_list_from_buffer`: Calls another routine to fetch data, then if the returned value is 164, it populates a structure at a2 with values from a6 and increments a counter at a4.
- `negate_and_return_d2`: Calls three different routines, then checks if a value in d2 is negative; if so, it negates it and stores it in d0, otherwise it returns zero.
- `return_zero_constant_2`: Sets d0 to zero, restores d2 through d7 and a2 through a5 from the stack, unlinks a6, and returns.
- `compute_font_offset_2`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays, adjusting the result by subtracting entries from a global lookup table when bit patterns differ.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `increment_pointer_loop`: Loads a pointer from the stack into a0, checks if the byte at that pointer is zero, and if not, increments the pointer and branches back to the check.
- `copy_string_stub`: Pushes d2 and a0 onto the stack and calls a routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length_2`: Iterates through a null-terminated string to count the number of characters before the terminator and returns the total count.
- `compare_bytes_and_branch`: Compares a byte from a0 with a byte from a pointer in a1, and if they are not equal, it checks if the byte at a0 is non-zero before potentially setting d0 to zero and branching.
- `compare_byte_and_branch`: Loads a byte from a0 into d0, decrements a1, sets a0 to a1, and if the byte at the new a0 is greater than d0, sets d0 to 255 and branches.
- `return_constant_19`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward_2`: Calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character, returning a pointer to the character or null if not found.
- `validate_font_handle_2`: Checks the validity of a font handle by attempting two specific operations with the values 47 and 58; if both succeed, it returns the original handle, otherwise it returns the error code from the failed operation.
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
- `execute_dual_calls_stub`: Clears bit 0 of register d0 and jumps to another routine.
- `jump_to_subroutine`: Performs a long jump to another routine.
- `jump_to_address`: Performs a long jump to a specific address.
- `clear_bit_and_increment_counter_2`: Clears bit 7 of a byte at offset 14 from the base address in a6 and increments a 16-bit word at offset 38 from the base address in a6, then jumps to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display_buffer`: Allocates a memory block, populates a table with incrementing offsets based on a base address, and performs two function calls involving hardware parameters.
- `call_indirect_function`: Fetches a function pointer from a location calculated by adding an offset to a value in a register and calls it with a pointer and a constant value as arguments.
- `concatenate_strings`: Joins three source strings into a destination buffer, inserting a separator character if necessary.
- `execute_conditional_callback`: Checks a value at an offset from a table base; if it is not zero, it calls a function from a second table, otherwise it calls a function from a third table.
- `dispatch_function_pointer`: Checks a value at an offset of 28 from a structure; if it is non-zero, it calls the function pointer located at that offset, otherwise it calls a different function located at an offset of 42 from the same structure.
- `dispatch_function_pointer_2`: Checks a value at an offset of 32 from a table pointer; if the value is non-zero, it calls that function, otherwise it calls a secondary routine via a different table pointer and then returns.
- `dispatch_function_pointer_3`: Checks a value at an offset of 36 from a table; if non-zero, it calls the function pointer stored at that location, otherwise it calls a secondary function via a library call and then executes the function pointer found at that same offset.
- `process_string_table_and_callback`: Iterates through a table of pointers, identifies a non-zero entry, and executes a function pointer from a structure.
- `reset_table_entries`: Clears a two-byte field in a table repeatedly and calls another routine each time, using a counter that runs from a positive start value down to zero.
- `decrement_and_clear_counter`: Decrements a short counter stored at an offset from a pointer and, if the counter reaches zero or was already invalid, clears the counter and, when a flag at the base of the structure is non-zero, calls another routine and invokes a function pointer stored in a table.
- `search_table_for_match`: Walks a table of entries using a fixed stride and compares an input value with a field in each entry, calling another routine when a match is found.
- `parse_resource_header_and_allocate`: Allocates memory for a structure and parses a resource header by checking for specific magic constants.
- `call_other_routine_and_return_zero`: Calls another routine and then returns zero.
- `execute_infinite_recursion_2`: Calls itself indefinitely without performing any other operations.
- `mask_lower_bytes`: Calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `execute_infinite_recursion`: Calls itself indefinitely, creating an infinite loop of recursive calls.
- `dispatch_function_pointer_4`: Loads a value from an offset of 530 from a6 into d0, compares it with a value in d1, and calls another routine.
- `wait_for_flag_clear_2`: Repeatedly calls another routine while a global variable at offset 872 of the structure in a6 is non-zero; otherwise it returns the combined values of d0 and d1.
- `remove_object_from_list`: If the object pointer is non-zero, it clears a long value at offset 6 and a word at offset 50 of a structure in a6, then iterates through a 64-byte name field at offset 64 of the object, calling another routine for each non-zero byte.
- `reset_game_state_flag`: Clears a 16-bit value in a global variable and sets a flag byte in the same structure by OR-ing in a constant, then calls another routine.
- `copy_struct_to_message`: Calls another routine and, if certain conditions on the returned values and a pointer are met, copies a small fixed-size block from memory into a structure, updates several fields, and clears a byte in that structure.
- `clear_and_bit_clear`: Clears a long value at an offset from a register and clears bit 0 of a byte at another offset from that same register.
- `clear_flag_and_iterate_table`: Checks a flag in a global variable, clears it, and then iterates through a table of integers, calling another routine for each non-zero entry until a counter reaches four.
- `process_data_table_2`: Iterates through a table of four entries, clearing specific bytes in a structure at an offset from a6, and updates values based on comparisons of words found in the table.
- `increment_and_store_byte_2`: Increments a byte in d0 and stores the result into a byte at an offset of 16 from a1, then branches to another routine.
- `increment_pointers_and_branch`: Increments two pointers in a0 and a1 by four, increments d7 by one, and branches to another routine if d7 is not four; otherwise it restores registers and returns.
- `copy_struct_fields_and_update_table`: Copies seven words from one area of a structure to another area of the same structure and then copies a word from one offset to another.
- `execute_recursive_pointer_calls`: Saves registers, loads a pointer from offset 44 of a structure in a6, calls that pointer if it is non-zero, and then calls three other routines sequentially before restoring registers and returning.
- `update_timer_and_process_events`: Decrements a 16-bit counter in a structure and, if it reaches zero, performs fixed-point math to update hardware registers and a global variable.
- `decrement_and_loop_on_offset`: Library code tests a longword at an offset of 586 in a structure, decrements it if non-zero, and loops back to the start of the routine if the result is not zero.
- `parse_event_data_and_update_pointer`: Reads a 16-bit word from a list, updates a pointer in a structure at offset 634 of a5, and if the word's high byte is 128 or 129, it calculates a new value by reading a byte from a table at offset 64 of a2 and multiplying it by 6 before storing the result at offset 554 of a5.
- `process_event_queue_and_update_counters`: Iterates through a list of structures, decrementing counters and performing fixed-point math to update values in a structure.
- `update_counter_and_branch`: Compares d0 and d1, and if they are not equal, checks a short value at an offset from a6; if that value is positive, it decrements it and calls another routine, otherwise it branches to another location.
- `clear_flag_after_callback`: The routine checks a short flag at an offset from a pointer and a global variable at another offset.
- `signal_on_flag`: Checks a global flag and, if set, calls another routine via a function pointer stored in a global variable.
- `process_sound_table`: Iterates a table of entries and invokes callbacks stored in each entry, setting bits in a mask based on their return values.
- `setup_sound_channel`: Writes a value derived from an input register to a global variable and then writes the constant 2 to a hardware register at an offset from the custom chip base, which corresponds to the left channel of audio channel 0.
- `clear_slot_and_notify_3_2`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot and continues a loop.
- `clear_slot_and_notify_3`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot and calls another routine to update a pointer and other fields.
- `set_register_and_branch`: Sets d0 to 255 and branches to another routine.
- `clear_state_and_wait`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in d0.
- `clear_flag_byte`: Computes an address from a global variable and a register value, writes zero to a byte at that address, and if the following byte equals one, writes two to the first byte.
- `reset_and_loop_until_zero`: Clears a longword at offset 864 and a word at offset 10 from a structure pointed to by a6, then calls another routine in a loop until it returns zero, returning the final value in d0.
- `clear_slot_and_notify`: Computes a byte offset from an input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero, marks the slot, calls a callback via an indirection, and clears the next byte and the slot byte.
- `calculate_scaled_value`: Clears a word at an offset from a6, and if d0 is not zero, performs a division involving d1 and a value derived from a word at an offset from a6, storing the result in a structure at an offset from a6.
- `store_word_and_branch`: Stores a word from d1 into a location at an offset from a6 and branches to another routine.
- `check_flag_and_call_2`: Loads a word from an offset from a6, and if it is non-zero, clears bit 0 of a byte at another offset from a6 and calls another routine if a condition is met.
- `check_flag_and_call`: Loads a word from an offset from a6, and if it is non-zero, clears bit 0 of a byte at another offset from a6 and calls another routine if a subsequent check succeeds.
- `increment_register_and_branch`: Adds one hundred twenty-two to a byte in a data register and then performs a relative branch to another routine.
- `branch_to_target_2`: Performs a short relative branch to another instruction.
- `process_entity_state_and_motion`: Checks a status byte in a structure to determine an action, then updates a fixed-point position and velocity within a table based on a value.
- `decrement_counter_and_branch`: Calls another routine, checks if the returned pointer is non-zero, decrements a short value located at an offset from that pointer, and calls another routine if the new value is zero.
- `copy_struct_fields_and_clear_flag`: Copies several fields from a structure pointed to by a1 into a destination pointed to by a2 when a condition on d7 is met, then returns the original value of one of the source fields after clearing that field in the source structure.
- `update_entity_state_and_position`: Checks a status byte in a structure to determine an action, then updates a fixed-point position and velocity within a table based on a value at offset 3.
- `update_frame_timing_and_counters`: Calculates a division factor from a value in a structure, updates a 32-bit value at offset 12 and a 16-bit value at offset 2 of the structure in a2, and calls update_timer_and_frame_data.
- `update_timer_and_frame_data`: Calculates a division factor based on a value in a structure, updates a word at offset 0 and a word at offset 2 of a structure in a2, and performs several arithmetic operations to update frame-related values and counters in a2 and a1.
- `compare_byte_and_store_word`: Compares a byte in register d0 to 2; if not equal, it branches to another location, otherwise it stores the word 6 at offset 0 of a structure in a2 and branches to another location.
- `compare_and_branch_3`: Compares a byte in d0 to 3, and if they are not equal, branches to another location; otherwise it calls another routine and then branches to another location.
- `decrement_word_on_specific_conditions`: Tests a word at offset 18 of a structure in a2, and if the value is negative or zero, it either branches to another location or decrements the word at that offset by 1 and branches to another location.
- `calculate_resampling_offset_threshold`: Calculates a value by multiplying a word from a table by a constant, shifting it, and adding it to a word from a structure, then stores a constant in a structure at offset 18 if a specific value in another structure is both non-zero and positive; otherwise it returns.
- `update_resampling_parameters_2`: Updates a structure by storing a byte from a table into offset 20, then calculates a new value for offset 12 based on a comparison between a longword and a calculated threshold.
- `compare_and_subtract_long`: Compares two longwords in d1 and d2, then subtracts d3 from d2 if d1 is less than d2.
- `update_resampling_parameters`: Updates a structure by adding two values, decrementing a counter at offset 2, and adjusting a value at offset 0 by adding a value from offset 4.
- `resample_audio_buffer_data_2`: Calculates a new sample rate using fixed-point arithmetic and updates a destination buffer by either copying bytes from a source buffer at a or performing linear interpolation between samples.
- `resample_audio_buffer_data`: Calculates a new sample rate using fixed-point arithmetic and updates a destination buffer by either copying bytes from a source buffer at a specific stride or performing linear interpolation between source samples.
- `resample_audio_buffer`: Calculates a new sample rate based on a fixed-point multiplier and resamples a source buffer into a destination buffer using linear interpolation.
- `update_fixed_point_table`: Performs fixed-point arithmetic to update a value in a structure and then uses a loop to redistribute or resample bytes from a source buffer into a destination buffer.
- `update_sprite_reference_and_scale`: Iterates through a table of pointers to compare a block of data against a reference block to manage reference counters in a structure.
- `branch_to_update_entity`: Performs a short branch to an offset in another routine.
- `decrement_counter_and_call_3_2`: Decrements a short value at an offset of 30 in a structure pointed to by an offset of 68 in a structure in a1, then calls another routine if the resulting value is zero.
- `update_entity_state_and_position_2`: Processes an entity based on a command byte, calculating new coordinates and scaling factors using values from a table and a structure, then updates a secondary structure with the resulting position and velocity data.
- `decrement_counter_and_call_3`: Reads a pointer from an offset of 68 in a structure in a1, checks it is non-zero, then loads a short from an offset of 30 in the pointed-to structure, decrements it and writes it back.
- `clear_d0_if_small`: Compares a word in d7 to 4 and, if it is less than 4, clears d0 and returns; otherwise it returns.
- `update_sprite_geometry_and_timer`: Calculates a sprite's position and dimensions using bitwise scaling and multiplication, then updates a timer value in a structure.
- `calculate_sprite_geometry_and_timer`: Calculates sprite dimensions and position using bitwise scaling and multiplication, updates a timer value in a structure, and calls another routine to handle hardware-specific updates.
- `compare_and_update_word`: Compares a byte in register d0 to the constant 2 and branches if they are not equal; otherwise, it writes the constant 6 to a word at offset 8 of a structure in a2 and branches to another location.
- `branch_on_byte_comparison`: Compares a byte in register d0 to the constant 3 and branches if they are not equal; otherwise, it calls another routine and then performs an unconditional branch to another location.
- `decrement_conditional_word`: Tests a word at offset 16 of a structure in a2; if it is not zero, it subtracts 1 from that word and branches to another location, otherwise it branches to another location.
- `update_dynamic_physics_parameters`: Calculates and updates several values in a structure, including a scaled value derived from a multiplication and bit shifts, a sign-extended value, and a longword value based on the difference between two values in a table.
- `compare_longs_and_subtract`: Compares two longwords in registers d1 and d2; if d1 is less than d2, it branches to another location, otherwise it subtracts a longword in register d3 from the longword in d2 and branches to another location.
- `calculate_complex_values`: Performs several arithmetic operations involving multiplications and bit shifts to update values in a structure.
- `cleanup_device_and_library_2`: Checks a global pointer, and if non-zero, performs several operations including calling a function via a pointer from a global variable, updating a value in a structure, and calling several other routines before clearing global pointers.
- `execute_signal_sequence`: Calls a sequence of three different functions from a global table, passing an argument to one, and returns a value formed by masking an input register with the result of a table call.
- `store_task_and_flags`: Clears a word in a structure pointed to by a register and, if a flag in another register is non-zero, stores a value into the structure, calls a routine via a global function pointer, stores the result, and copies the flag into the structure.
- `compare_strings_case_insensitive`: Compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `return_called_register`: Calls another routine and returns the value that was left in an address register.
- `allocate_memory_and_store_pointer`: Checks a global pointer, otherwise calls a routine to allocate memory, and if the allocation succeeds and meets a size requirement, stores a pointer to the new memory into a destination structure.
- `free_memory_and_call_handler`: Checks if a pointer is non-zero, calls a routine to free memory if a global function pointer is null, otherwise calls the global function pointer, and then calls another routine via a pointer stored at the freed memory location.
- `return_called_register_2`: Pushes an address register onto the stack, calls another routine, moves the value from that address register into a data register, and returns.
- `check_and_branch_on_result`: Calls another routine and tests its return value; if non-zero, it calls several other routines in sequence, conditionally invoking another routine based on a pointer and a bitmask check, before returning a 64-bit value.
- `initialize_object_structure`: Performs a series of conditional calls to populate a structure with values from various routines, including setting specific bytes at offsets 444, 479, and 480, and initializing a pointer at offset 482 to point to a location within the same structure.
- `dispatch_to_external_routine`: Checks if a global variable is non-zero, and if so, calls another routine four times before jumping to a different routine.
- `call_subroutine_and_copy_struct`: Calls another routine and, if certain conditions involving a pointer and returned values are met, copies a fixed-size memory block to a message buffer.
- `invoke_resource_allocation`: Pushes the current value of a6 onto the stack, loads two pointers from the stack into a0 and a1, loads a constant into a6, calls parse_resource_header_and_allocate, and restores a6 from the stack before returning.
- `initialize_sonix_driver`: Allocates and initializes a complex data structure and attempts to copy a string pointer from a list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_envelope_and_pitch`: Updates a pitch and volume tracking variable based on a counter and a bitmask from a data table.
- `initialize_hunk_data`: Parses Amiga hunk data to populate a structure with specific values, a pointer to a global variable, and a function address.
- `trigger_crowd_audio_event`: Updates audio parameters via a sequence of function calls using values from a table and triggers a specific sound effect associated with the crowd stopping.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_match_statistics`: Clears several data tables and then iterates through a structured byte array to compare adjacent values.
- `compare_and_dispatch_table`: Loops through a table of 12 words, comparing adjacent 16-bit values and calling another routine if a match is found, then performs a final comparison and enters an infinite loop that calls another routine repeatedly using a value from a global table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_values`: Calls another routine twice to store two 32-bit values into a structure at offsets -10472 and -10468, then reads a 16-bit value from a table using an index and stores it at offset -4 of a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_table_match`: Compares a 16-bit value at a specific memory location with a 16-bit value from a table at an offset determined by a value in a structure, and if they match, calls another routine to update that structure value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_table_indices`: Reads a 16-bit value from a table using an index, updates a structure, and enters a loop that calls another routine and increments the index until it reaches 25.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_attribute_sort`: Increments a value in d0, pushes d0 and a value from offset -2 of a5 onto the stack, calls another routine, increments the stack pointer by 4, restores the value from offset -2 of a5, and enters a loop that decrements the value at offset -2 of a5 until it is less than 26, at which point it unlinks a5 and returns.
- `sort_player_attributes`: Clears a table of counters and then performs a comparison-based sort on a subset of an index table.

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

- `execute_multiple_subroutines_2`: Calls several routines and stores their results in a structure at offsets 12, 4, and 16, then calls a final routine with a specific constant in d3 and returns the result.
- `load_pointers_into_registers_2`: Loads a pointer from a location relative to the program counter into a register, then uses that pointer to load two additional addresses into two other registers before returning.
- `return_constant_value_2`: Returns a constant integer value.
- `return_address_pointer`: Returns the address stored in a register.
- `execute_multiple_subroutines`: Saves registers to the stack and calls six different subroutines in sequence.
- `execute_resource_loop_2`: Calls several routines, and if a condition is met, sets a byte at offset 25 of a structure in a5, performs further calls using values from a4, and eventually branches.
- `call_library_function_directly_2`: Loads an address into a0 from an offset of the program counter, calls a library function using the pointer in a6, and returns if bit 31 of d0 is not set.
- `call_library_vector_228_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `update_structure_and_execute_callbacks`: Writes a value of all ones to a four-byte field at an offset of four within a structure pointed to by a value loaded from an offset of sixteen in a register, then calls a routine via a pointer in another register; if that routine returns non-zero, it calls four additional routines via pointers within the returned value before calling a final routine via a pointer in a global variable.
- `open_dos_library_and_execute_methods`: Sets a field in a structure to negative one, opens the dos.library via exec, and if successful, calls two specific methods on the library base before closing the library and returning the stored negative one.
- `open_trackdisk_device_request`: Fills a device request record with a unit value and a fixed length, then calls a library function to open the trackdisk.device.
- `store_library_results_in_records`: Calls two library functions and stores their longword and byte results into a state record and a second record, where the second record is located via a pointer at offset 40 of a structure in a4.
- `call_library_vector_336_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_and_branch_2`: Calls a library function using a pointer from a table and then calls another routine.
- `copy_memory_word`: Copies a 32-bit word from an offset of 32 in a structure to an offset of 48 in the same structure and then branches to another routine.
- `set_record_field_8002`: Sets d4 to 32770 and branches to another routine.
- `set_record_field_800a`: Clears d2 and moves it to d1, sets d4 to 32778, and branches to another routine.
- `set_record_field_8005`: Sets d4 to 32773 and branches to another routine.
- `set_record_field_000d`: Sets d4 to 13 and branches to another routine.
- `set_register_and_branch_2`: Sets register d3 to 1 and branches to another routine.
- `set_record_field_8009`: Clears d3, sets d4 to 32777, and branches to another routine.
- `update_record_fields_2`: Calculates two values by scaling registers by 512 and adding them to a sum of three other values scaled by 22 and 11 respectively, then writes these results along with a constant, a 16-bit value, and two 32-bit values from another record into a structure, before calling another routine.
- `register_disk_resource_handler_2`: Calls a library function using d0, stores d0 at offset 20 of a5, sets a6 to d0, and populates a structure pointed to by a3 with a byte, a long, and a word, while also storing a5 and a pointer to another routine into the structure at offsets 34 and 38.
- `execute_resource_loop`: Calls two functions via pointers from a structure, looping if the second function returns zero, and finally writes three values to hardware registers before branching.
- `invoke_disk_resource_routine_2`: Sets bit 0 of a byte read from offset 24 of a structure in a5 and calls a function at an offset from the pointer in a6.
- `configure_hardware_and_call_library`: Loads the custom chip base address into a1, writes values to offsets 154 and 36 of that base address, loads a pointer from offset 20 of a5 into a6, loads a pointer from offset 32 of a4 into a1, calls a library function using a6, and branches to another routine.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `modify_state_byte_bits`: Clears a bit at an index determined by adding three to a value in d0, then sets bit two of the byte at offset 25 of the structure in a5 if d1 is zero, and returns.
- `update_joystick_status_register`: Writes a fixed byte to the joystick port control register, copies a byte from an offset of 25 in a structure pointed to by a5 into the same port control register, and sets a bit in the hardware custom chip control register before returning.
- `sync_disk_dma_transfer`: Calls a helper, waits for a countdown loop to complete, then writes values to the DMA control, disk pointer, and sync registers before calling two other routines.
- `set_top_bit_field_2`: Sets the most significant bit of a 16-bit field at offset 36 of the structure pointed to by a1 using the value in d0, and writes this value twice.
- `reset_record_and_call_library_2`: Sets a state word to a fixed value, sets a second word to two, and calls a library function on the base pointer.
- `count_consecutive_values_3`: Calculates a value based on two words at a pointer, then compares it against a value derived from two bytes in d6 and d7 to decide whether to call another routine or return negative one.
- `count_consecutive_values_2`: Iterates through a memory range to count how many times a specific word value repeats consecutively.
- `forward_to_helper_routine_2`: Loads four values from a structure pointed to by a5 into registers a0, d4, d0, and d3, then calls a helper routine before returning.
- `compute_bitwise_checksum_2`: Computes a 16-bit checksum over a sequence of words by shifting each bit through the accumulator and exclusive-ORing with a value in d3 when the most significant bit is set.
- `branch_conditionally`: Performs a conditional branch to another routine; otherwise it returns.
- `extend_and_store_word`: Extends a 16-bit word from a register to a 32-bit long and stores it at an offset of negative 4 from a pointer in a5.
- `execute_supervisor_call`: Performs a series of operations and invokes a supervisor-level library call.
- `wait_for_flag_clear`: Repeatedly calls another routine while a specific global variable at an offset of 872 from a6 remains non-zero.
- `remove_object_from_table`: If the object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_game_state_flag_2`: Clears a 16-bit value at offset 464 of a structure in a6, sets bit 0 of a byte at offset 48 of the same structure, and calls another routine.
- `clear_flag_byte_loop`: Clears a byte at an offset from a pointer, then iterates through a list of pointers, calling another routine for each non-zero entry until a counter reaches four.
- `update_flag_state`: Calculates an address by adding a scaled value from a register to a global variable, clears a byte at that address, and if the subsequent byte is one, writes two to the original byte before returning.
- `clear_state_and_wait_2`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_slot_and_notify_2`: Calculates an offset into a table using a multiplier, clears a byte at that location, and if the subsequent byte is non-zero, sets the original byte to three, calls a function pointer stored at an offset of four from that location, and finally clears both the subsequent byte and the original byte.
- `calculate_scaled_division`: Clears a word at offset 464 of a structure in a6, sets offset 460 using a scaled value from offset 0, and if d0 is not zero, performs a division using d1 and d2 to store a result at offset 462 of the structure.
- `store_word_and_branch_2`: Stores a word from d1 into a global variable at offset 0 of the structure pointed to by a6 and branches to another routine; otherwise it saves registers, loads a pointer from a table, calls four different routines, restores registers, and returns.
- `cleanup_device_and_library`: The routine checks a global pointer, and if non-zero, performs several operations including calling a function via a pointer from a global variable, updating a value in a structure, and calling several other routines before clearing global pointers.
- `execute_signal_sequence_2`: Calls three functions from a global table, passing a value to the second function, and returns a 64-bit value composed of a bitwise AND between the second function's result and a register, concatenated with the original value of that register.
- `update_task_status_and_flags`: Clears a word at offset 864 of a structure pointed to by a6; if d1 is non-zero, it stores d0 at offset 868, calls a routine via a pointer at a global offset, stores the result at offset 860, and stores d1 at offset 864 before returning.
- `call_free_mem_and_callback`: If a register is non-zero, it checks a global pointer and calls either that pointer or a library routine, then calls another routine via a pointer retrieved from a register.
- `jump_to_external_routine`: Performs an absolute long jump to another routine.

### logged disk load from TV Sports Football (1988)(MirrorSoft)(Disk 1 of 2).adf

- `process_string_and_set_bit`: Iterates through a string to find the end of a sequence of non-space characters, then clears a byte at a calculated offset and sets a specific bit in a byte at a different offset.
- `update_byte_from_function_return`: Calls a function via a pointer in a global structure and stores the returned byte into a memory location determined by an offset and a byte value from another memory location.

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

- `check_and_call_handler`: Checks a 16-bit value at offset -2b40 of the structure in a4; if non-zero, it decrements a 16-bit value pointed to by a3, clears a byte at the location pointed to by a0, and calls a routine at offset -7d9a using a pointer from the structure in a4.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_two_counters`: Clears two 16-bit fields, one at offset -2b00 and one at offset -2afe, of the structure in a4, then returns.
- `initialize_structure_fields`: Calls a routine via a pointer in a structure in a4, then calls another routine three times using constants and values from the structure to populate three consecutive 32-bit fields in that same structure.
- `call_routines_with_values`: Calls a routine at offset -32682, then calls a routine at offset -32154 three times, each time passing two 32-bit values read from the structure in a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_structure_values`: Clears two bytes in a global variable, sets a 16-bit value at an offset of a4 if a parameter is less than 2, sets two bytes at specific offsets in a global variable if they are zero, and calls another routine if two specific 16-bit values in a global variable are both zero.
- `rotate_byte_and_call_handler`: Sets up its stack frame, then checks a 16-bit value at offset -2b40 of the structure in a4 and jumps away when that value is zero.
- `calculate_and_call_scaled_values`: Performs three separate sequences where it calls a routine at offset -32730 using a value derived from a byte at an offset in the structure in a4 multiplied by a constant, and then performs bit manipulation on d0 and d7.
- `tiny_move_3`: Small routine of 1 instruction that copies d1 to -(a7).
- `adjust_stack_pointer`: Adds 18 to the stack pointer.
- `update_structure_and_call_2`: Decrements a byte from offset -3615 of the structure in a4, sets bits in a location pointed to by a0, and calls a routine at offset -7fda using a pointer from the structure in a4.
- `store_byte_to_pointer`: Increments d4 by 8, sets a0 to zero, sets bits in d0, and stores d0 into the location pointed to by a0 using a value from the stack.
- `tiny_clr_and`: Small routine of 2 instructions that clears -(a7), then masks (a0) with d3.
- `undecodable_bytes_39`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `perform_division`: Decrements d0, rotates d4 left by 4, and performs a signed division of d4 by d6.
- `transfer_and_mask_data`: Copies a 16-bit word from d0 to d1, performs a bitwise OR of d0 with a longword at a negative offset from a0, and performs a bitwise AND of d1 with a byte from a location pointed to by a2.
- `push_address_and_set_registers`: Pushes a longword address onto the stack, sets register d0 to zero, and sets register d6 to six.
- `initialize_registers_with_offsets`: Pushes a 16-bit value onto the stack, sets d0 to zero, and sets d6 to several different byte values.
- `call_subroutine_with_parameters`: Moves d1 to a0, sets d6 to 48, pushes two values onto the stack, calls a routine at a negative offset from a4, sets d6 to 72, and clears the word at the top of the stack.
- `decrement_and_store_word`: Sets d0 to zero, sets d6 to 90, decrements d0, stores the word 6 at a location pointed to by a0, and sets d1 to zero.
- `call_routine_and_decrement_byte`: Moves d1 to a0, sets d6 to three different values, calls a routine at a negative offset from a4, sets d6 to three more values, loads a byte from a negative offset of a4 into d0, and decrements d0.
- `update_player_status`: Reads a byte from a memory location pointed to by a register, compares it to a constant, and if they are not equal, writes a new value to that same memory location before returning.
- `clear_byte_and_branch`: Clears a byte at a negative offset from a5 and branches to another location.
- `check_and_increment_index`: Checks two memory locations based on an index stored at a negative offset from a5; if both conditions are met, it increments that index byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_compare_byte`: Increments a byte at a negative offset from a5 and compares it to two.
- `increment_and_call_subroutine`: Increments a byte at offset minus 3 of a structure in a5, compares it to 18, and if the value is 18 or less, calls another routine and pushes a 32-bit value onto the stack.
- `shift_and_load_pointer`: Loads a byte from offset minus 2 of a structure in a5 into d0, shifts d0 left by 1 bit, and loads a 32-bit address from offset minus 10048 of a structure in a4 into a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_to_memory`: Stores a byte from a register into a memory location pointed to by a register and then returns.
- `load_byte_and_adjust_stack`: Loads a byte from offset minus 13846 of a structure in a4 into d0, sets d1 to 34, and adds 4 to the stack pointer a7.
- `load_address_and_modify_byte`: Loads a 32-bit address from offset minus 13750 of a structure in a4 into a0 and performs a bitwise OR on a byte at the address pointed to by a6 using the constant 236.
- `compare_and_reset_byte`: Loads a byte from offset minus 11457 of a structure in a4 into d0, compares it to a byte at offset minus 11456 of the same structure, and if they are not equal, branches to another routine; otherwise, it sets a byte at offset minus 11457 of the structure to 7 and clears a byte at offset minus 2 of a structure in a5.
- `load_byte_to_register`: Loads a byte from offset minus 11449 of a structure in a4 into d0.
- `load_byte_to_d1`: Loads a byte from offset minus 2 of a structure in a5 into d1.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies d1 to d2.
- `load_address_to_a0_2`: Loads a 32-bit address from offset minus 22374 of a structure in a4 into a0.
- `load_byte_to_d3`: Loads a byte from offset minus 2 of a structure in a5 into d3.
- `transfer_and_shift_data`: Copies a 16-bit word from a table using offsets in a0 and d0 to a destination at an offset of a global variable in a1, updates a long value in a global variable, and shifts d0 left by 4 bits.
- `call_with_shifted_byte`: Loads a byte from a global variable at an offset of a5, shifts it left by 1 bit, calls another routine, and increments the stack pointer by 2.
- `shift_and_increment_stack`: Loads a byte from a global variable at an offset of a5, shifts it left by 1 bit, and increments the stack pointer by 2.
- `increment_and_branch_on_counter`: Copies a 16-bit word from a table to a destination, increments a byte in a global variable at an offset of a5, and branches to another routine if the byte is less than 3.
- `tiny_unlk`: Small routine of 1 instruction that removes the stack frame held in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_40`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
