# Game 1241 (nuclear war (1989)(new world computing)[cr pna]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01241). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1806592 top-level bytes in 75 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 846 of 846 named (0 author symbols), 846 of 846 described (263 by rule, 556 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6022 of 6022 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 33276 of 33276 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1424461 |
| code | 120264 |
| data | 194838 |
| padding | 19925 |
| unused | 47104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf:Nuclear war | 3 | 5042 | 127 |
| 8 | 1 | 7 | 2 |
| Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf:echo | 2 | 117 | 2 |
| mathtrans.library | 17 | 830 | 94 |
| diskfont.library | 15 | 1040 | 61 |
| Nuclear War (1989)(New World Computing)[cr PNA](Disk 2 of 2).adf:runnw | 3 | 574 | 36 |
| Nuclear War (1989)(New World Computing)[cr PNA](Disk 2 of 2).adf:quick nuclear war | 3 | 24660 | 470 |
| Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Nuclear War (1989)(New World Computing)[cr PNA](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 516 | 42 |

## Cases handled

- nuclear war (1989)(new world computing)[cr pna](disk 1 of 2)/Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf: file prot (header 3 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nuclear war (1989)(new world computing)[cr pna](disk 1 of 2)/Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf: blocks 1738-1748 (bytes 889856-895488) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `format_event_text_messages` (quick nuclear war): Selects and formats descriptive text strings based on event outcomes, such as population changes or city destruction. Kind: checksum/integrity trap. Pass path: the branch where sVar4 != 0 is taken, leading to the u10__sub_0_1BB98 calls.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a longword from one register to another and then performs a short branch.

### Assign

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with this value and a generated sequence.
- `move_and_branch_2`: Copies a longword from one register to another before executing a short branch.

### EndCLI

- `find_max_and_populate_buffer`: Iterates through a linked list to identify the maximum value in a specific field, then uses that value to initialize a data buffer and copy a block of memory from a source to a destination.
- `move_and_branch_3`: Moves the contents of one register into another and performs a short branch.

### Echo

- `find_max_and_populate_buffer_2`: Traverses a linked list to determine the maximum value in a specific field and then populates a data buffer with a sequence of incrementing values.
- `transfer_register_and_branch`: Copies the contents of one data register to another and then performs a short relative branch to another location in the code.

### Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf:Nuclear war

- `initialize_global_state`: Clears a block of memory with zeros and executes functions via pointers retrieved from a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_nested_function`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `update_state_buffers`: Copies blocks of data between different memory locations based on the value of a global short.
- `process_callback_lists`: Checks several global pointers and executes them with specific constants if they are non-zero.
- `dispatch_summed_callbacks`: Executes several individual function pointers with constant arguments.
- `execute_callback_chain`: Processes a series of function pointers and callback lists stored in a structure, including conditional execution of several specific pointers.
- `dispatch_callback_chain`: Executes a sequence of various function pointers and callback chains, including routines that load assets and manage delays.
- `initialize_game_resources`: Allocates memory and loads various data files including graphics, maps, and sound assets.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `copy_memory_blocks_2`: Copies several long words from one memory location to another and then copies several more long words from a different source location.
- `update_display_coordinates`: Calculates and stores new coordinate values into several global variables based on input parameters and table lookups.
- `configure_dma_transfer`: Updates DMA controller registers and calculates various offsets and values based on input parameters and a pointer.
- `increment_global_short`: Calls two unnamed routines and increments a short value stored at a specific offset from a global pointer.
- `initialize_data_structure`: Clears a block of memory, stores a parameter, and sets a function pointer to itself within a structure.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_subroutine_with_parameter`: Calls another routine with a specific integer value and a pointer to a memory location.
- `process_game_state_loop`: Iterates through a series of data structures to update object attributes, perform memory block copies, and execute a sequence of callback functions.
- `manage_audio_data`: Initializes a data structure, releases and resets a record, and executes several indirect functions related to audio device functionality.
- `process_audio_callbacks`: Manages a sequence of audio-related operations and indirect function calls based on input parameters and state.
- `process_record_and_execute_indirect`: Checks a pointer in a structure and calls a routine if it is non-zero, then executes a function via an indirect pointer from a table, and finally calls another routine with a constant value if the input parameter is non-zero.
- `clamp_and_update_state`: Clamps an input value to a maximum threshold and stores the result in a structure.
- `update_and_overflow_structure`: Checks if a value exceeds a specific threshold; if it does, the value is capped, the primary counter is decremented by the threshold, and a secondary value is incremented by the same amount.
- `copy_memory_blocks_3`: Copies multiple blocks of data between two memory areas using signed length values stored within the source data.
- `initialize_resource_allocation_tables`: Iterates through nested loops to populate memory tables with results from a resource allocation routine.
- `load_game_assets`: Loads audio and data files from the disk into memory and initializes resource allocation tables.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `clamp_modified_values`: Modifies two passed short values by adding a pseudo-random offset derived from another routine.
- `allocate_graphics_buffers`: Attempts to allocate several memory blocks for cloud, sky, and bomber assets.
- `flush_event_pointers`: Checks a series of global pointers and executes them with various constant arguments if they are not zero, clearing the pointers after execution.
- `load_game_assets_2`: Attempts to load multiple data files including background, bomber, and tile assets from disk.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_offset_value`: Calls another routine, adds a constant to the result, and returns the upper sixteen bits of the sum masked to fifteen bits.
- `store_upper_word`: Stores the upper sixteen bits of a provided 32-bit value into a global memory location.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `execute_indirect_jump_2`: Performs an indirect jump to an address stored in a jump table.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to the caller.
- `initialize_window_system`: Opens a file, populates a structure with window data, and manages various callback chains and library functions.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_quoted_string_tokens`: Parses a source string into individual tokens by handling whitespace and quoted substrings.
- `set_register_and_branch`: Loads a value into a register and performs a short branch to another location.
- `copy_string_with_limit`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `initialize_window_structure`: Opens a file and uses library functions to populate a structure with data from a table and various library results.
- `move_d0_to_d5`: Copies the contents of one data register to another.
- `execute_cleanup_callback`: Checks a value at a negative offset from a register and executes the function pointed to by that value if it is non-zero.
- `process_cleanup_chain`: Iterates through a list of items to perform various cleanup tasks, including calling function pointers and manipulating data based on global flags.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Validates an index into a table, calls a routine to close a file if the entry is valid, and clears the entry in the table.
- `call_indirect_dos_close`: Performs an indirect call to a function pointer retrieved from a table.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_execute`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `execute_indirect_function_pointer_3`: Retrieves a function pointer from a nested structure within a global table and executes it.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_library_function_9`: Performs an indirect call to a function at a specific offset within a library.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_630`: Library stub: loads a library base from a global variable and calls the library function at offset minus 630 of that base, returning its result to the caller.
- `allocate_and_populate_record`: Allocates a new record by calling a routine, then stores several parameters and a pointer into the newly allocated memory.
- `release_and_reset_record_2`: If a pointer is non-zero, it calls a routine, then sets a byte and a long to all ones, and calls other routines with a byte or a constant.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `execute_indirect_function_5`: Fetches a pointer from a table located via an offset from a global variable and executes the function at that address.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_4`: Retrieves a pointer from a memory location calculated from a base address and an offset, then executes the function at that address.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_5`: Retrieves a pointer from a nested memory structure and executes the function located at that address.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_function_6`: Fetches a pointer from a table using an offset and executes the function located at the resulting address.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `execute_indirect_function_pointer_6`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_10`: Calls a function from a library using a pointer retrieved from a global variable and an offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_function_11`: Executes a routine at a specific offset relative to a library base address stored in a global variable.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.

### 8

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `modify_register_and_branch`: Tests a bit in a register, performs bitwise operations on two registers, and then branches to another location.

### Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf:echo

- `update_linked_list_and_copy_data_2`: Traverses a linked list to find the maximum value in a specific field and then updates a buffer with a header and new data.
- `move_and_branch_4`: Moves the contents of one register to another and then performs a short relative branch.

### mathtrans.library

- `execute_illegal_instruction`: This library routine contains an illegal instruction.
- `store_register_to_global`: This library routine stores the contents of a register into a global variable.
- `manage_memory_node`: This library routine checks a flag in a structure, and if zero, updates a linked list by moving a pointer and then calls a function through a pointer in a global variable.
- `set_byte_flag`: This library routine sets a specific bit in a byte located at an offset from a register-derived address.
- `initialize_library_state`: This library routine checks a flag byte in a register-derived structure; if clear, increments a counter and on first use stores pointers to itself in global variables, otherwise returns a negative value.
- `decrement_counter_and_call`: Library code that decrements the 16-bit counter at offset 32 of the structure at A6.
- `return_empty`: Library code that is empty.
- `compute_bit_weighted_difference`: This library routine calculates a weighted value by iteratively updating a running difference using a doubling accumulator and bitwise shifts based on an input value.
- `adjust_bitfield_value`: This library routine sets the bit selected by register D1 in D0, then repeatedly shifts a running difference with a doubling accumulator until a termination condition is met.
- `modify_register_and_call`: This library routine moves a value to a register, calls a routine through a pointer, and then modifies specific bits in a register before branching to another location.
- `call_jump_table_entries`: This library routine calls a series of function pointers located at fixed offsets from a register-derived base in an alternating pattern, then calls a third entry once and calls the second entry once more.
- `clear_and_branch_3`: Library code that clears a long register and performs a relative branch.
- `execute_conditional_library_calls_3`: Library code that performs a series of indirect function calls through a jump table based on the values in several registers.
- `execute_conditional_library_calls_2`: This library code performs a series of indirect function calls through a jump table based on the values in several registers.
- `execute_conditional_library_calls`: Library code that performs a series of indirect function calls based on the values of several registers.
- `execute_indirect_calls_2`: Library code that performs an indirect call using an offset from a register, then performs two more indirect calls using different offsets from a register, and finally branches to another location.
- `execute_indirect_call`: This library routine moves a value to a register, calls a routine using an offset from a register, and then moves the result back to that register before branching.
- `conditional_load_constant`: This library routine moves a value to a register, checks a long integer, and either branches to another routine or loads a constant into a register before branching.
- `mask_return_value`: The library code calls a function through a pointer stored in a global variable twice, then calls another routine and returns its result with certain bits cleared.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_528`: Library stub: loads a library base from a global variable and calls the library function at offset minus 528 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `invoke_callback_and_return`: The library code receives a pointer in A0 and a value in D0, calls the routine pointed to by A0, and returns the value originally in D0 unchanged.
- `call_indirect_return_d0`: The library code receives a function pointer in A0 and a value in D0, calls the routine pointed to, and then returns the value that was in D0 on entry.
- `invoke_callback_and_return_3`: Library code that calls a routine pointed to by a register and returns the original value from another register.
- `invoke_callback_library`: Library code that calls a routine pointed to by a register and returns the original value of a long register.
- `invoke_callback_and_return_5`: Library code that receives a pointer in a register and a value in another, calls the routine at the pointer, and returns the original value.
- `return_register_value`: Library code that returns the value currently held in a register.
- `invoke_callback_and_return_7`: This library code calls a routine via a pointer and returns the original value from a register.
- `invoke_callback_with_one`: This library code invokes a routine via a pointer and then pushes the value one onto the stack before branching.
- `invoke_callback_and_return_8`: This library routine calls a subroutine via a pointer provided in a register and returns the original value held in a data register.
- `store_register_via_pointer`: This library code invokes another routine and then stores a 32-bit value from a register into memory addressed by a pointer passed in a register.
- `call_other_routine_conditionally_2`: Library code that invokes another routine and then, based on a boolean flag, invokes three additional routines in sequence.
- `invoke_callback_and_return_6`: Calls a routine via a pointer in a register and returns the original value from a data register.
- `execute_indirect_call_2`: Calls another routine via a pointer and returns the original value held in a data register.
- `invoke_callback_and_return_4`: Library code that calls a routine through a pointer and returns the original value from a register.
- `execute_callback_and_return`: Calls a routine via a pointer provided in a register and returns the original value held in a data register.
- `execute_callback_and_return_3`: Calls a routine through a pointer and returns the original value held in a data register.
- `call_indirect_library`: Library code that calls a routine via a pointer in a register and returns the value that was in a long register.
- `invoke_callback_and_return_2`: This library routine calls a subroutine via a pointer in a register and returns the original value that was passed in a data register.
- `execute_callback_and_return_2`: Calls a routine via a pointer provided in a register and returns the original value from another register unchanged.
- `call_offset_routine`: This library routine calls a routine at an address calculated by adding an offset to a pointer stored in a global variable.
- `call_offset_routine_alt`: This library routine calls a routine at an address calculated by adding a different offset to a pointer stored in a global variable.
- `load_address_and_branch`: This library routine loads a constant address into a register and then branches to another location.
- `load_address_and_branch_2`: Loads a constant address into a register and performs a short branch to another location.
- `call_indirect_offset_one`: Library code calls a routine at an address calculated by adding a negative offset to a pointer stored in a global variable.
- `call_indirect_offset_two`: Library code calls a routine at an address calculated by adding a different negative offset to a pointer stored in a global variable.
- `load_address_and_branch_3`: Library code loads a specific long value into a register and then performs a short branch.
- `load_address_and_branch_two`: Library code loads a different long value into a register and then performs a short branch.
- `load_address_and_branch_three`: Library code loads another long value into a register and then performs a short branch.
- `invoke_callback_and_return_concat`: The library code invokes another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.
- `call_indirect_with_offset`: The library code calls a routine at an address calculated by adding an offset in A0 to a pointer stored in a global variable, then returns the value in D0.
- `normalize_fixed_point_value`: The library code normalizes a signed 32-bit fixed-point value in D6 by swapping its high and low words, adjusting a shift count based on sign and magnitude, then left-shifting the value while decrementing the count until the count reaches zero or the value overflows, returning early on overflow.
- `normalize_fixed_point_value_2`: Library code that performs a loop to normalize a value by shifting and comparing it against powers of two.
- `push_and_branch`: This library code pushes a word onto the stack and performs a short branch.
- `push_all_ones_and_branch`: Pushes a word containing all bits set to one onto the stack and performs a short relative branch.
- `push_constant_and_branch`: Pushes a word containing the value one onto the stack and performs a short relative branch to another location.
- `compare_and_adjust_stack`: Library code compares a byte to a constant, branches if higher, and adjusts the stack pointer and a register before returning.
- `perform_mathematical_calculation`: Library code that performs complex arithmetic operations and bitwise manipulations on a byte value.
- `perform_fixed_point_division`: Performs a division operation using iterative bit-shifting and subtraction.
- `apply_math_transformations`: Library code that performs iterative arithmetic transformations on values using bit-shifted components and a table of data.
- `apply_iterative_bit_shift_transformations`: Performs iterative arithmetic transformations on two values by alternating between addition and subtraction of bit-shifted components.
- `perform_iterative_math_transformations`: Library code that performs iterative arithmetic operations using a pointer to a table and several registers.
- `load_constant_and_branch`: Library code loads a long constant into a register and then performs a short branch.
- `recursive_call_wrapper`: Calls itself and then returns, with no other operations.
- `transfer_and_call_register`: This library code pushes a long value onto the stack, calls another routine, and then restores a value from the stack into a register.
- `compare_and_zero_register`: Library code that moves a value to a register, branches if the value is negative, and if the value is not equal to a specific constant, sets the register to zero.
- `shift_and_branch_conditional`: Library code performs an arithmetic shift right on a register, increments it, and branches or jumps based on the result.
- `return_without_effect`: Library code calls a routine and then calls it again if a certain condition is met.
- `call_other_routine_conditionally`: Library code invokes another routine and then, based on a boolean flag that remains false, invokes three additional routines in sequence.
- `execute_conditional_recursive_calls_2`: Library code that performs a series of recursive calls based on the value of a register and performs bitwise shifts and sign extensions.
- `rotate_and_branch`: Library code loads a long constant into a register, performs a rotate right through the carry, and then branches.
- `load_constant_and_branch_2`: Loads a specific longword constant into a register and performs a short relative branch to another location.
- `test_and_branch_word`: This library code pops a word from the stack into a register, tests if it is positive, and branches accordingly.
- `empty_stub_40`: The library code contains five instructions and performs no observable work; it ends immediately.
- `return_immediately`: This library routine consists of four instructions and performs no observable work, immediately returning to its caller.
- `execute_conditional_recursive_calls`: This library routine performs a series of recursive calls based on the value of a register and performs bitwise shifts and sign extensions on data derived from that register.
- `execute_unreachable_calls`: Library code that contains a sequence of calls to another routine that are unreachable.
- `subtract_table_from_accumulator_2`: Library code that performs arithmetic shifts on two registers, adds them to two other registers, and then enters a loop that subtracts values from a table from a register using a counter.
- `subtract_table_from_accumulator`: This library code subtracts two values from two other registers, then enters a loop that adds values from a table to a third register until a counter reaches zero.
- `normalize_signed_byte`: This library code tests a byte, performs a bitwise AND operation, and calls another routine if the byte is positive.
- `set_register_and_branch_2`: This library code loads a signed byte into a register and performs an unconditional short branch.
- `perform_mathematical_normalization`: Library code that performs bit-shifting and arithmetic operations to normalize a value.
- `complex_loop_with_branching`: Library code that executes a loop involving bit-shifting and conditional jumps, potentially calling other routines and performing arithmetic on a value.
- `call_other_routine_thrice`: Library code that calls another routine three times in succession and then returns.
- `test_and_branch_transform`: Tests a byte in a register and branches to another routine if the value is positive.
- `process_conditional_data_transformations`: Library code that performs a series of conditional indirect calls and bitwise manipulations based on a register value.
- `return_from_routine`: This routine performs no operations and immediately returns to its caller.
- `transform_register_bits`: This library code performs a sequence of swaps, rotations, and arithmetic operations on a register's contents.
- `test_and_modify_byte`: Library code that compares a byte against two constants and performs arithmetic and bitwise shifts if specific conditions are met.
- `mask_and_test_word`: This library code applies a bitwise mask to a word, tests the result for zero, and branches based on the outcome.
- `test_and_shift_byte`: Library code that compares a byte to a constant, and if not equal, performs two logical shifts on a long integer and branches based on the result.
- `shift_and_rotate_word`: This library code performs a logical shift left on a word, loads a constant byte, and then performs a rotate right through the carry bit.
- `clear_and_branch_2`: This library code clears a register and performs an unconditional branch.

### diskfont.library

- `check_and_set_font_handle_2`: The library code calls a routine via a table entry, and if the result is non-zero, stores a value into a structure and calls another routine.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_handle`: Library code that attempts to open two libraries via pointers in a structure, and if successful, initializes a new structure by setting a pointer to itself plus an offset and clearing a field.
- `process_linked_list_nodes`: Library code that iterates through a linked list, performs pointer manipulations and calls a routine for nodes meeting certain criteria, and potentially calls other routines if a counter reaches zero.
- `update_library_status`: This library code checks two conditions in a structure; if they are met, it performs a linked list manipulation and returns a value, otherwise it sets a specific bit in a byte and returns zero.
- `set_bit_and_clear_register`: Sets a specific bit in a byte at an offset from a pointer and clears a register.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_branch`: Decrements a word at an offset from a pointer, branches if not zero, checks a bit in a byte, and jumps to an offset if the bit is set.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_scaled_offset_2`: Library code that computes an integer from two structures using subtraction, scaling, and a loop that subtracts values from a global table based on byte comparisons, then returns the result.
- `load_and_install_disk_font_2`: Opens a disk font file, reads its header, allocates memory, and loads the font segment.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_parent_dir`: Library stub: loads its arguments and the dos library base, calls ParentDir and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `transfer_pointers_and_branch`: Moves two values from a stack frame into registers and branches to another location.
- `skip_to_string_terminator_and_call`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length_2`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `compare_string_bytes_2`: Library code that compares two null-terminated strings and returns a value indicating their relative order.
- `skip_non_zero_bytes`: This library routine tests a byte at a pointer and increments the pointer.
- `compare_and_branch_byte`: This library routine compares a byte from a pointer with a value in a register and performs conditional branches based on the result.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `load_byte_and_branch`: This library routine loads a long word from the stack into a register, fetches a byte from an offset relative to the stack pointer into a second register, and performs a short relative branch.
- `compare_and_assign_pointer`: This library routine compares two bytes.
- `increment_and_test_byte`: This library routine increments a pointer, loads the byte at the new address into a register, and branches if that byte is non-zero.
- `find_last_occurrence_in_string_2`: This library routine receives a pointer to a zero-terminated string and a byte value, determines the string length via a library call, then walks backwards from the end to find the last occurrence of the byte and returns its pointer.
- `find_delimiter_and_advance_pointer_2`: This routine searches a string for specific delimiter characters and advances the pointer past the first found delimiter.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `populate_glyph_metadata_table`: Iterates through a linked list of font data structures to extract character metadata and glyph bitplanes.
- `initialize_loop_counter`: Loads a pointer from a memory location into a register and sets a loop counter to one.
- `process_font_string_list`: Iterates through a linked list of data structures to extract and copy string content into a destination buffer.
- `parse_glyph_metadata_stream`: Iterates through a data stream to extract character metadata and glyph bitplanes.
- `parse_font_resource_stream`: Iterates through a data stream to identify font metadata and character definitions.
- `parse_font_metadata`: Iterates through a data stream to identify specific font character markers and populates a structured table with character attributes, including offsets and bitmask flags.
- `load_font_data`: Reads data from a disk source and populates a structured table with font character information.
- `parse_font_glyph_data`: Iterates through a data stream to extract character metadata and glyph bitplanes.
- `return_zero_stub`: As part of library code, this routine returns zero and performs no operations on registers, memory, or hardware.
- `populate_font_glyph_table`: This routine parses a data structure to build a table of font glyphs by iterating through a list of entries.
- `load_disk_font_table`: As library code, this routine builds a table for a disk font.
- `dispatch_font_request`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.
- `call_other_routine`: This library code calls another routine and then returns.
- `call_another_routine`: This library routine is library code that acts as a thin wrapper which immediately invokes another routine and then returns without performing any additional operations.

### Nuclear War (1989)(New World Computing)[cr PNA](Disk 2 of 2).adf:runnw

- `branch_to_address`: Performs a short branch to a specific memory location.
- `call_external_routines`: Calls two instances of another routine with zeroed arguments and then calls a routine that performs a single return and a routine that allocates and initializes a memory block.
- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current function.
- `allocate_and_initialize_manx_structure`: Allocates a memory block based on a table value and initializes it with a specific four-byte signature.
- `copy_register_to_register`: Copies the contents of one register to another.
- `parse_quoted_string_tokens_2`: Parses a string into individual tokens by handling whitespace and quoted substrings.
- `set_limit_and_branch`: Moves a value to itself, sets a register to a constant, and branches to another location.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter reaches negative one, and ensures a null terminator is written at the next destination byte if the counter expires first.
- `copy_string_pad_zeros_2`: Copies bytes from a source to a destination up to a specified count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `open_window_resource`: Opens a file and retrieves window or icon information from a library to populate a structure.
- `copy_data_register_d0_to_d5`: Copies the contents of a longword from one data register to another data register.
- `execute_conditional_callback`: This routine checks if a value at a specific memory offset is non-zero and, if so, executes the function pointed to by that value.
- `process_callback_list`: Iterates through a list of function pointers and executes them while performing various conditional checks and data assignments.
- `perform_no_operation`: The routine is empty and performs no operations.
- `close_file_handle_2`: checks if an index is within the bounds of a table and if the entry is non-zero, then calls a function to close the resource and clears the table entry.
- `call_close_3`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir_3`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_execute_2`: Wrapper that calls the dos library function Execute and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_open_3`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock_3`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_function_12`: loads a function address from a table within a library base and executes it.
- `call_library_vector_414_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Nuclear War (1989)(New World Computing)[cr PNA](Disk 2 of 2).adf:quick nuclear war

- `initialize_system_state_2`: Zeros a block of memory, updates global variables, and executes routines via pointers from a global structure based on bit flags.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_callback_table`: Iterates through multiple tables of function pointers and executes them with varying constant arguments.
- `initialize_game_state_and_process_events`: Initializes several global state variables and tables, including a zeroed array of six elements.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_blocks`: Copies several longwords from a source table to a destination memory area.
- `configure_dma_transfer_parameters`: Sets values in hardware registers and memory locations, including bitwise calculations for a destination address and length.
- `setup_dma_transfer_b`: Sets specific hardware register values and memory pointers to prepare for a DMA operation.
- `configure_dma_transfer_parameters_2`: Sets values in memory locations associated with DMA control and writes to DMA control registers.
- `setup_dma_transfer_d`: Writes specific values to hardware registers and memory to prepare for a DMA-driven operation.
- `setup_complex_dma_transfer`: Calculates multiple derived values for a data structure based on input parameters and bitmasks, then writes them to memory and hardware registers to initiate a DMA transfer.
- `test_word_and_return`: Tests a word at a specific offset from a pointer and returns if the value is non-zero, otherwise restores registers and returns.
- `calculate_sprite_bounding_box`: Calculates sprite dimensions, offsets, and coordinate structures based on input scaling and bitwise shifts.
- `update_sprite_coordinate_structures`: Calculates and writes multiple sets of coordinate, offset, and dimension values into a destination structure.
- `update_dma_coordinate_structures_2`: Synchronizes multiple coordinate and offset values into a destination structure by repeatedly waiting for a specific bit in a global variable to clear.
- `update_dma_coordinate_structures`: Repeatedly waits for a specific bit in a global variable to clear before calculating and storing multiple coordinate or offset values into a data structure.
- `populate_coordinate_data_structure_3`: Repeatedly waits for a specific bit in a global variable to clear before calculating and storing multiple coordinate or offset values into a data structure.
- `populate_coordinate_data_structure_2`: Repeatedly waits for a specific bit in a global variable to clear before calculating and storing multiple coordinate or offset values into a destination structure.
- `populate_coordinate_data_structure`: Repeatedly waits for a bit in a global variable to clear before calculating and storing multiple coordinate or offset values into a destination data structure.
- `wait_and_populate_structure`: Waits for a specific bit in a global variable to clear before writing calculated values into a data structure.
- `populate_structure_from_table_3`: Waits for a global flag to clear before populating a data structure with values derived from a table and other registers.
- `populate_structure_from_table_2`: Waits for a global flag to clear before populating a data structure with values derived from a table and other registers.
- `update_structure_fields_3`: Waits for a global flag to clear before populating a structure with values derived from a table and other registers.
- `update_structure_fields_2`: Populates a structure with values from a table and other registers after waiting for a global flag to clear.
- `setup_extended_dma_transfer`: Configures a wide range of memory and hardware registers for a DMA operation using parameters to determine source, destination, and length values.
- `setup_conditional_dma_transfer`: Selects specific hardware and memory parameters based on a bitmask and configures them for a DMA operation.
- `prepare_dma_blit_parameters`: Calculates source and destination coordinates by adding input parameters to values retrieved from a table.
- `setup_dma_blit_chain`: Populates a series of DMA control structures with source addresses, destination coordinates, and transfer parameters.
- `configure_dma_blit_structures`: Populates multiple DMA control structures with source addresses, destination coordinates, and transfer parameters.
- `apply_spatial_offset_sequence`: Repeatedly updates a destination structure by adding specific offset values from a table to fields from a source structure.
- `update_entity_spatial_attributes_2`: Repeatedly synchronizes a destination structure by adding offset values from a table to fields from a source structure.
- `update_entity_spatial_attributes`: Repeatedly synchronizes a destination structure by adding offset values from a table to fields from a source structure.
- `synchronize_entity_state_structures`: Waits for a global flag to clear before repeatedly populating a destination structure by adding values from a table to fields from a source structure.
- `populate_structure_fields`: Waits for a global flag to clear and then populates a destination structure by adding values from a table to fields from a source structure.
- `calculate_structure_offsets`: Waits for a global flag to clear and then calculates several field values by adding offsets from a table to values in a structure.
- `populate_structure_from_table`: Calculates several field values by adding offsets from a table to values in a structure and stores the results into another structure.
- `update_structure_fields`: Waits for a global flag to clear, then populates a structure with values from a table and other registers before waiting for the flag again.
- `update_entity_state_and_hierarchy`: Iterates through a list of entities to recursively update their properties and hierarchical states.
- `process_entity_hierarchy_and_state_2`: Recursively traverses a hierarchical entity structure to update properties and performs modulo-style normalization on values.
- `clamp_entity_value_lower_bound`: Compares a 16-bit word at a memory offset to a constant value and replaces it with a different constant if the value is smaller.
- `clamp_entity_value`: Compares a 16-bit word at a memory offset to a constant value.
- `test_and_replace_word`: Tests if a 16-bit word at a specific memory offset is less than a constant and, if not, writes a new word to that location.
- `compare_and_modify_word`: Compares a 16-bit word at an offset from a register against a constant and, if it is not less than that constant, replaces it with a different value.
- `test_and_modify_memory`: Tests a 16-bit word at a memory location offset from a register and, if it is not less than or equal to zero, moves a constant value into that location.
- `update_entity_hierarchy_and_state`: Recursively traverses a hierarchical entity structure to update properties and performs modulo-style normalization on values.
- `process_entity_hierarchy_and_state`: Recursively traverses a hierarchical structure of entities to update their properties, performing fixed-point scaling and modulo-style normalization on specific values.
- `sync_entity_state_buffers`: Iterates through a collection of entities to update their properties based on arithmetic scaling and conditional logic.
- `update_entity_state_and_buffer`: Calculates new values for a set of entities using arithmetic operations and updates their properties in a table.
- `sync_entity_state_buffers_2`: Iterates through an entity collection to perform fixed-point arithmetic scaling on specific properties and synchronize data between primary and secondary attribute tables.
- `update_entity_timers_and_states`: Iterates through a table of entity structures to decrement timer values and update state flags based on specific threshold conditions.
- `decrement_and_update_entity_counters`: Iterates through a table of structures to decrement a timer value for each entry.
- `increment_memory_value`: Adds a small constant to a word located at the address pointed to by a register and then performs a short branch.
- `update_entity_state_table`: Iterates through a table of structures to increment counters and update status values based on conditional checks.
- `process_entity_timer_table`: Iterates through a table of structures to decrement timer values and update state flags based on specific threshold conditions.
- `increment_and_dispatch_callback`: Increments a value at a memory location and conditionally calls another routine using parameters retrieved from a table.
- `update_vblank_callback_state`: Updates a pointer and associated data within a structure located via a global variable.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `reset_and_call_routines`: Sets a global word to zero and calls another routine multiple times using various parameters.
- `format_event_text_messages`: Selects and formats descriptive text strings based on event outcomes, such as population changes or city destruction.
- `evaluate_and_store_status`: Performs a series of conditional checks and function calls to determine a state, then stores the resulting values into provided memory locations via pointers.
- `execute_callback_routine`: Executes a function via a pointer retrieved from a stack-based source after checking a global variable.
- `execute_callback_routine_2`: Performs a callback execution using a pointer from the stack after verifying a state in a global variable.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `format_game_status_strings`: Processes a series of numeric values and string templates to generate formatted status messages regarding megatons, casualties, and military assets.
- `test_global_flag_and_set_register`: Loads a longword from a pointer into a register and tests a specific global variable.
- `return_zero`: Clears the accumulator and returns to the caller.
- `update_game_state_timers`: Manages several countdown timers and state transitions by decrementing counters and updating global variables.
- `update_dma_transfer_queues`: Manages a series of queued DMA operations by decrementing counters and updating source/destination pointers.
- `process_queued_dma_transfers`: Manages a series of countdown timers and queues to trigger multiple DMA operations.
- `update_queued_dma_sequences`: Manages multiple concurrent countdown timers that trigger recursive calls to another routine.
- `update_recursive_sprite_animation_2`: A recursive routine that decrements frame counters and manages sprite animation transitions.
- `update_recursive_sprite_animation`: A recursive routine that decrements counters in a data structure and manages sprite frame transitions.
- `select_jump_target_2`: Compares a longword in a register to a constant and branches if not equal; otherwise, it shifts the register and uses it to fetch a word from a table to perform an indirect jump.
- `test_and_extend_word`: Compares a word at an offset from a pointer to a constant and, if not equal, branches to another routine; otherwise, it extends a word from a different offset in the same pointer into a longword and branches to another routine.
- `select_jump_target`: Compares a longword to a constant and, if not equal, shifts the value left and uses it as an index to load a word from a table before jumping to the resulting address.
- `check_type_and_extend_value`: Compares a word at a memory location to a constant and branches if not equal.
- `update_entity_state_table_2`: Iterates through a table of nine entries to update values based on comparisons between specific data fields.
- `process_recursive_data_table_2`: This recursive routine iterates through a table of nine entries, performing arithmetic updates and conditional logic on values stored at various offsets.
- `process_recursive_data_table_3`: Iterates through a table of nine entries to perform conditional arithmetic and recursive calls based on comparison results.
- `process_recursive_entity_table`: A recursive routine that traverses a hierarchical data structure to update entity states and perform arithmetic on table entries.
- `traverse_recursive_data_table_2`: Performs a depth-first traversal of a data structure by recursively calling itself and updating values within a table.
- `traverse_recursive_data_table`: Iterates through a table of short integers using a counter to perform recursive depth-first traversals.
- `process_recursive_data_table`: Iterates through a table of short integers using a counter, performing recursive calls and updating values based on comparisons and arithmetic operations.
- `execute_conditional_indirect_call`: Checks if a value at a memory location is non-zero, and if so, performs an indirect call using two parameters retrieved from adjacent memory locations.
- `traverse_nested_structures_2`: Recursively processes nested data structures and pointer lists by performing string comparisons and updating a running total.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d1.
- `process_audio_structure`: Traverses nested data structures and pointer lists, performing string comparisons and updating a running total based on results from various subroutines.
- `pop_and_modify`: Pops two longwords from a stack into a register and performs an OR operation on a memory location using an offset.
- `update_pointers_and_registers`: Loads a word into a register, increments a pointer, performs an OR operation on a memory location, and increments a register.
- `parse_resource_header`: Recursively processes structures by searching for specific strings in a table and traversing pointer lists to calculate a value returned to the caller.
- `traverse_nested_structures`: Recursively processes data structures by traversing a pointer list and performing byte string comparisons within a loop.
- `manage_audio_record`: Allocates and initializes a record structure, manages audio device functions, and resets specific fields to all ones.
- `process_nested_loops`: Executes a series of nested loops that repeatedly call other routines to update local variables and counters based on input parameters.
- `process_record_and_library_calls`: Checks a pointer in a structure, calls a routine if it is non-zero, and executes functions via library calls and indirect pointers from a table.
- `clamp_and_update_values`: Clamps a value to a specific range, stores it in a structure, and updates a counter and a secondary value based on the clamped result.
- `transfer_and_clamp_values`: Stores a value into a structure and either resets a counter or increments a secondary value while adjusting the primary counter.
- `fetch_scaled_table_value`: Fetches a 16-bit value from one of several tables based on a switch index and scales it by either ten or one hundred.
- `classify_table_value`: Retrieves a 16-bit value from a table and returns a specific category code based on its range.
- `categorize_table_value`: Retrieves a 16-bit value from a table and returns one of three possible status codes based on its magnitude.
- `update_table_values`: Iterates through a loop of four elements to fetch scaled values from a table and update a corresponding table with a calculated sum, clamped between specific bounds.
- `fetch_table_value`: Calculates an index based on input parameters and retrieves a word from one of three different memory locations.
- `modify_global_value`: Performs arithmetic on a global variable using a value from another routine and stores the result back into that variable.
- `update_table_values_2`: Iterates through nested loops to modify a table of 16-bit values by adding a scaled value from another table and clamping the result, then updates a second table with status codes.
- `select_best_table_index`: Iterates through a five-entry table to find the index of the maximum value, handling ties by either selecting the next highest value or performing a randomized selection if multiple maximums exist.
- `compare_table_entries`: Fetches two values from a table based on provided indices and compares them against a value returned by another routine to return a status code and a quotient.
- `count_positive_table_entries`: Iterates through five entries in a table and counts how many contain a value greater than zero.
- `scale_and_store_table_value`: Multiplies a word from a stack location by a constant to calculate an offset into a table, then stores a value from that table and two constant values back onto the stack.
- `find_first_positive`: Iterates through five entries in a table to find the first entry containing a value greater than zero, storing that value and its index into a stack.
- `find_maximum_value`: Iterates through five entries in a table to find the largest value, storing the maximum value and its corresponding index into a stack.
- `find_minimum_positive`: Iterates through five entries in a table to find the smallest value that is also greater than zero, storing that value and its index into a stack.
- `search_table_index`: Uses a value from another routine to perform a loop that searches through a table for a specific condition, eventually storing and returning an index.
- `store_processed_data`: Checks a return value from another routine and, if valid, writes three different values into a table at an offset determined by a parameter, then updates a final entry in a separate table.
- `update_entity_state_and_attributes`: Updates a state machine within a table based on current status values and performs arithmetic on associated attribute fields.
- `process_entity_state_logic`: Evaluates a value from a table and recursively calls another routine based on specific integer matches.
- `evaluate_table_state`: Finds the index of the largest value in a table and then checks several global variables to determine and return a status code.
- `process_table_entries`: Iterates through a four-entry table to evaluate values and perform conditional logic based on specific integer matches.
- `sort_and_update_entity_state`: Identifies active indices from a table, sorts them based on values in a secondary table, and updates state variables and counters within a structured data block.
- `initialize_and_randomize_entity_data`: Clears a structured data table and populates entity attributes by performing modulo-based randomization on specific fields.
- `calculate_scaled_value`: Calls another routine, divides the extended result by a constant, swaps the high and low words, adds a constant, and stores the result in a table.
- `calculate_scaled_index`: Calls another routine, extends the result to a longword, divides it by sixteen, and extracts the upper bits.
- `calculate_and_store_scaled_index`: Calls another routine, extends the result to a longword, divides it by sixteen, shifts the high word into position, adds a constant, and stores the result in a memory location relative to a base pointer.
- `calculate_scaled_value_2`: Calls another routine, divides the extended result by a constant, swaps the high and low words, increments the result, and stores it in a table.
- `update_entity_attributes`: Iterates through a structured data table to update specific attribute fields for a set of entities.
- `reset_entity_attribute_table`: Iterates through a structured data table containing multiple entities to clear or reset specific attribute fields.
- `reset_entity_attribute_table_2`: Iterates through a structured data table to clear or reset specific attribute fields for a set of four entities.
- `reset_entity_attributes_loop`: Iterates through a structured data table to clear or reset specific attribute fields for a set of entities.
- `execute_dual_calls`: Pushes two words from a stack and performs two consecutive calls to a subroutine before branching.
- `reset_entity_attributes`: Iterates through a structured data table to clear or reset specific attribute fields for a set of entities.
- `find_last_nonzero_entry`: Searches backwards from a starting index to find the first non-zero 16-bit value in a table.
- `find_highest_nonzero_index`: Searches downwards from index three to find the first non-zero entry in a table.
- `search_scaled_index`: Iterates through a table to find the first index where a value is non-zero, using a scaling constant for comparison.
- `search_nonzero_index`: Iterates backwards from index three to identify the first non-zero value in a table.
- `map_index_to_value`: Returns a specific constant value based on the input index provided.
- `map_index_to_value_2`: Takes an input value and returns one of four predefined constants based on a comparison.
- `process_recursive_table`: Performs a recursive traversal of a table using calculated offsets and values.
- `traverse_nested_table`: Iterates through a two-dimensional table structure using nested loops.
- `update_production_statistics`: Iterates through a loop to generate random values that determine the distribution of produced items into specific categories.
- `update_entity_attribute_table`: Iterates through a table to update multiple attribute fields based on classification results and mathematical calculations.
- `process_conditional_data_table`: Iterates through several short integer tables to perform conditional logic based on their values.
- `process_string_and_update_display`: Iterates through a null-terminated string to process individual characters while performing calculations to update display parameters.
- `format_and_display_status_table`: Iterates through a table to evaluate entries and formats numeric values into strings using a floating-point format specifier.
- `process_iterative_data`: Performs a series of library calls and nested routine executions within a loop that runs thirty-nine times.
- `execute_parameterized_loop`: Executes a loop thirty-nine times, making several library calls and routine calls using parameters passed via the stack.
- `update_object_state`: Reads an index from a table to determine an object type and performs calculations based on its coordinate differences.
- `render_scaled_values`: Iterates through a table to calculate scaled values and calls a routine to perform drawing or data processing based on those values.
- `copy_data_block`: Copies a block of five 32-bit words from one memory location to another based on a conditional check of a global variable.
- `copy_conditional_data`: Copies a block of five 32-bit words between memory locations based on the value of a global variable.
- `update_ui_element_table`: Iterates through a table of four entries to update specific display parameters and coordinate values.
- `process_list_elements`: Iterates through a table of four entries, calling another routine with specific parameters derived from memory and a calculated offset.
- `update_ui_with_math`: Calls another routine and performs conditional logic based on the result modulo a constant.
- `branch_on_parameter`: Performs conditional branching based on whether a passed parameter is less than seven.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.
- `update_coordinate_pair`: Modifies two short integer values by incrementing or decrementing them based on a provided mode identifier.
- `update_defector_event_state`: Processes a defection event by retrieving entity coordinates from a table and updating several global state variables.
- `process_combat_event_visuals`: Calculates combat outcome parameters and updates global state based on a selected template index.
- `update_combat_display_state`: Calculates new coordinate and dimension values based on combat event data and updates several global variables.
- `update_sprite_animation_state`: Calculates animation frame indices and timing based on a table lookup and updates global state variables.
- `update_entity_state_and_graphics`: Selects an entity from a table based on a search loop and updates its properties, including a state value determined by a series of threshold comparisons.
- `update_sprite_animation_data`: Determines a sprite frame index based on a modulo operation and a series of threshold comparisons, then updates multiple global variables with calculated coordinate and offset values derived from a data table.
- `test_pointer_and_calculate_index`: Tests a word at a memory location and branches if it is non-zero.
- `call_formatted_routine`: Loads a value from a stack offset, extends it, and calls a routine using parameters pushed onto the stack.
- `format_missile_strike_message`: Calculates memory offsets and screen coordinates to prepare a formatted string for display.
- `update_entity_state_and_timer`: Updates an entity's properties by fetching values from a table and performing arithmetic calculations.
- `perform_bit_rotation`: Performs bitwise rotation and shifting operations on blocks of memory using values retrieved from a table.
- `traverse_linked_list_recursively`: Recursively traverses a linked structure by checking a pointer stored in a global variable and following it.
- `populate_data_buffer`: Fills a buffer with alternating positive and negative byte values in two sequential loops.
- `populate_data_buffer_2`: Writes a sequence of signed bytes into a memory area using two loops that increment and decrement a counter.
- `populate_descending_offset_table`: Iterates through a loop to calculate a sequence of decreasing values and stores them into a memory buffer using an incrementing index.
- `render_sprite_animation_frames`: Iterates through a series of nested loops to update and draw graphical data using varying parameters.
- `increment_nibbles_to_match`: Increments individual nibbles within a table of 32 words until they match the values in a provided source table.
- `decrement_nibbles_to_zero`: Decrements individual nibbles within a table of 32 words until all values in the table reach zero.
- `format_game_status_string`: Constructs a formatted status string by concatenating various game statistics including remaining cities, score, and fame.
- `find_last_positive_index`: Iterates through a five-entry table and returns the index of the last entry containing a non-zero value.
- `load_asset_data_block`: This routine consists of 41 instructions and is called by one unnamed routine.
- `process_keyboard_input_buffer`: Reads characters from an input source and stores them into a memory buffer.
- `tiny_addq`: Small routine of 1 instruction that adds 6 to d0.
- `add_constant_to_register`: Adds a constant byte value to a register.
- `move_long_to_offset_address`: Moves a long word from the location pointed to by one register to a memory address calculated by adding a constant offset to another register.
- `move_long_to_pointer`: Moves a long word from a pointer in one register to a memory location pointed to by another register.
- `clear_high_score_entry`: Searches for a string in a table and, if found, recursively clears a linked structure and updates several related data tables via pointers.
- `empty_stub_22`: Empty routine: returns to the caller immediately without doing anything.
- `clear_high_score_data`: Searches for a string in a table and, if found, recursively clears a linked structure and updates several related data tables via pointers.
- `empty_stub_23`: Empty routine: returns to the caller immediately without doing anything.
- `update_high_score_table`: Shifts entries in multiple tables to make room for a new entry and then updates the table with new values and a string.
- `push_and_fetch_stack_data`: Pushes a longword from a memory location onto a stack, fetches a longword from a table using an index, and performs multiple consecutive loads from a stack pointer.
- `load_game_data_assets`: Loads multiple data files from a disk into memory using a series of loops that iterate through tables of file identifiers and memory addresses.
- `empty_stub_24`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_25`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_26`: Empty routine: returns to the caller immediately without doing anything.
- `swap_and_update_pointers`: Swaps sets of data between two memory regions based on a condition and then updates a group of pointers.
- `load_game_assets_3`: Iteratively opens and loads multiple data files including maps, tiles, masks, and sound files into memory buffers.
- `empty_stub_27`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_28`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_29`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_30`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_31`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_32`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_33`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_34`: Empty routine: returns to the caller immediately without doing anything.
- `reset_and_draw_elements`: Resets a global flag and calls several routines to perform drawing operations with specific parameters.
- `draw_string_and_elements`: Resets a global flag and performs multiple drawing operations, including one that uses the length of a string.
- `compare_coordinate_pairs`: Compares two pairs of values and returns a specific integer code based on their relative magnitudes and equality.
- `process_entity_state_and_coordinates`: Calculates a value based on modulo operations and updates a global variable within a loop.
- `update_sprite_animation_and_position`: Selects a sprite frame based on a modulo operation and updates animation state counters.
- `update_sprite_animation_and_position_2`: Selects a sprite frame based on a modulo operation of a returned value and updates various global coordinate and state variables.
- `update_sprite_and_position_data`: Updates a set of coordinate and attribute values in a table based on a counter and a modulo-based selection.
- `update_entity_animation_and_position`: Updates an entity's animation frame and position by fetching values from tables and applying a calculated offset.
- `format_combat_message_data`: Selects a random string template index using a loop that avoids a specific value, then calculates the absolute difference between two numerical values.
- `process_destruction_event_text`: Selects a random destruction message from a table based on a modulo calculation and formats it with a city name.
- `initialize_alien_landing_event`: Calculates landing coordinates and randomized attributes for an alien landing event, then updates several global state variables and sprite-related data.
- `copy_memory_blocks`: Copies blocks of data between two memory areas using lengths specified within the data itself, handling signed length values.
- `process_complex_logic`: Performs a series of conditional checks and nested calls involving record initialization, string parsing, disk status verification, and indirect function execution to determine a return value.
- `initialize_record_chain`: Sequentially allocates and initializes several records by calling a routine with specific parameters and storing the resulting pointers into a structure.
- `parse_drive_string`: Checks if a string follows a specific format and returns a nibble value if it matches, otherwise returns a maximum value.
- `format_file_string`: Retrieves information via a library call and either writes a hardcoded string or copies a filename from a structure into a buffer.
- `get_short_value_from_structure`: Checks the validity of a structure through library calls and returns a 16-bit value from a specific offset if successful.
- `check_disk_status`: Calls a library function to check a condition and updates a global flag based on the result.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `set_bit_and_branch`: Sets a bit in a data register and branches if the result is non-zero.
- `validate_and_copy_header_sequence`: Validates a multi-stage magic byte signature within a data table and, upon success, copies a block of bytes to a destination buffer.
- `verify_and_copy_data_sequence`: Validates a specific byte sequence against constant characters and, if successful, copies a block of data from a source table to a destination buffer.
- `validate_and_copy_data_sequence`: Checks a sequence of bytes against specific character constants and then copies a set of bytes from a source table to a destination buffer.
- `clear_and_branch_on_zero`: Loads a 16-bit word from the stack, performs a logical OR with zero, and branches based on the result.
- `verify_and_copy_data_block`: Validates a specific sequence of characters against a hardcoded pattern and, if successful, copies a block of four bytes from a source table to a destination.
- `validate_and_copy_table_data`: Checks if a sequence of characters matches specific values and then copies a block of four bytes from a source table to a destination table.
- `validate_and_sync_data_tables`: Checks for specific character matches before copying a block of four bytes from a source table to a destination table.
- `compare_data_tables`: Copies bytes from one table to another and compares the contents against a third table to determine a return value based on a match count.
- `test_and_branch_zero`: Loads a word from the stack into a register and performs a conditional branch based on the result of a logical OR operation with zero.
- `clear_byte_and_branch`: Clears a single byte in memory at an offset from a pointer and performs an unconditional short branch to another location.
- `compare_and_copy_data`: Iteratively compares bytes from two memory locations and copies data from a source table to a destination table based on matching criteria.
- `test_and_branch_d0`: Calls another routine, tests the result in a word register, and branches based on the outcome or clears the register.
- `clear_and_branch`: Clears a byte at a negative offset from a register and performs a short branch.
- `compare_and_increment_bytes`: Iteratively compares bytes from two different memory locations, increments a counter if they match, and updates a target byte with a value from a third location.
- `load_small_constant`: Loads a small constant into a data register and branches to another location.
- `parse_and_bitshift_string`: Parses a control string to determine a bit-shift amount based on specific character markers and then performs a bitwise reconstruction of a byte stream into a new buffer.
- `parse_control_string_and_bitshift`: Scans a source string for specific control characters to determine a bit-shift amount and a termination condition.
- `clear_word_at_offset`: Clears a word at a specific offset from a pointer in a register and branches to another location.
- `clear_byte_at_offset`: Clears a byte at a specific offset from a pointer in a register and branches to another location.
- `store_byte_two`: Stores the value two into a byte at a specific offset from a pointer in a register and branches to another location.
- `store_byte_four`: Stores the value four into a byte at a specific offset from a pointer in a register and branches to another location.
- `store_byte_six`: Stores the value six into a byte at a specific offset from a pointer in a register and branches to another location.
- `branch_by_subtraction`: Subtracts various constant values from a register and performs conditional branches based on the result.
- `perform_bit_shift_copy`: Checks a word and a byte in a structure to determine if a loop should execute.
- `process_string_and_shift`: Scans a string for specific characters to set a bit-shift value, then performs a bitwise shift and merge operation to copy data from one table to another.
- `clear_word_offset_alt`: Clears a word at a specific offset from a pointer in a register and branches to a different location.
- `store_byte_one`: Stores the value one into a byte at a specific offset from a pointer in a register and branches to a different location.
- `store_byte_three`: Stores the value three into a byte at a specific offset from a pointer in a register and branches to a different location.
- `store_byte_five`: Stores the value five into a byte at a specific offset from a pointer in a register and branches to a different location.
- `store_byte_and_branch`: Stores a byte into a memory location relative to a pointer and then performs a short branch.
- `decrement_and_conditional_branch`: Subtracts specific constant values from a data register and performs a conditional branch if the result is zero.
- `check_and_clear_value`: Tests a word in a structure and, if non-zero, compares a byte from the structure to a constant and potentially clears a register before branching.
- `decrement_and_clear`: Decrements a word in a structure, stores the result back, and clears a byte at a different offset.
- `perform_bit_shifting`: Iteratively extracts bits from two consecutive bytes to form a new byte, which is then stored in a destination table.
- `reset_library_parameters`: Sets two values at specific offsets within a structure and calls a library function.
- `reorder_bit_fields`: Performs bitwise manipulations to rearrange the positions of bits from a 16-bit input into a new byte format.
- `execute_indirect_calls`: Calls a routine using a value from a pointer and, if the result is non-zero, calls another routine using that result.
- `process_record_fields`: Checks several offsets within a structure for non-zero values and, if found, calls another routine with that value and a specific constant.
- `forward_then_return_argument`: Calls another routine and returns the 32-bit value that was in the first data register on entry.
- `initialize_string_buffer`: Allocates a memory buffer, populates it with a specific four-byte identifier, and manages the structure of this buffer by setting specific offset values and updating a global pointer.
- `copy_register_value`: Stores the contents of one register into another register.
- `manage_dos_file_buffer`: This routine manages file data by comparing string buffers, allocating memory for file structures, and performing memory copies between source and destination buffers.
- `call_library_functions`: Executes an indirect function call and two library function calls via a library base.
- `copy_data_register`: Copies the contents of one data register to another data register.
- `iterate_and_update_table`: Loops through a table to increment nibble values in a second table until they match the first, calling another routine during the process.
- `decrement_table_values`: Iterates through a table of thirty-two 16-bit values, calling another routine with the nibbles of each value and decrementing non-zero nibbles before storing the result back in the table.
- `initialize_system_resources`: Attempts to open essential Amiga libraries and allocate memory for bitmaps and fonts.
- `empty_stub_35`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_36`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_37`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_38`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_39`: Empty routine: returns to the caller immediately without doing anything.
- `store_formatted_value`: Stores a value into a memory location pointed to by a global variable and then clears that pointer.
- `increment_and_store_byte`: Increments a value in a global variable and stores a byte from a parameter into the memory location pointed to by that global variable.
- `load_word_from_table_2`: Loads a pointer from a stack offset, compares a word at a specific offset to four, and if equal, loads a long value from a pointer into a register.
- `load_word_from_table`: Checks a word at an offset from a pointer and, if it meets a condition, loads a word from a table into a register.
- `populate_buffer_recursively`: Iteratively calls another routine to fetch values from a table and stores them into a buffer while decrementing a pointer, potentially inserting a constant value if a condition is met.
- `parse_formatted_numeric_string`: Parses a string containing numeric values, potentially including signs, decimal points, and scientific notation, while handling specific format specifiers like percent signs and scientific exponents.
- `parse_numeric_string_2`: Parses a numeric string into a signed integer value while handling optional sign prefixes and percentage formatting.
- `process_numeric_data`: Pushes several values from a stack frame onto the stack, calls another routine, and then updates a global variable by adding a signed long value derived from a word in the stack frame.
- `parse_numeric_string`: Parses a string to extract integer or floating-point values by iterating through characters and handling signs, decimal points, and scientific notation.
- `execute_indirect_callback_2`: Sets a local variable to a constant, checks a global value, and if zero, calls a routine before executing an indirect function call via a pointer.
- `format_floating_point_string_2`: Converts a numeric value into a formatted string representation, handling sign characters, decimal points, and scientific notation.
- `validate_and_update_attribute`: Loads values from a structure, calls another routine to validate them, and updates a field in the structure based on the result.
- `validate_attribute_range`: Checks a word at an offset from a pointer against a constant and performs several conditional checks and assignments based on the results.
- `check_numeric_limit`: Compares a value at an offset from a pointer against constants and performs conditional branches or sets a register to a specific value.
- `format_scientific_notation_string`: Converts a numeric value into a null-terminated ASCII string using scientific notation.
- `clear_stack_word`: Clears a word at a negative offset from the stack frame and branches to another location.
- `increment_stack_value`: Loads a word from a negative offset of the stack frame, increments it, and stores the result back into a different negative offset of the stack frame.
- `format_scientific_string`: Converts a numeric value into a null-terminated ASCII string, handling integer parts, decimal points, and scientific notation.
- `format_floating_point_string`: Converts a numeric value into a null-terminated ASCII string, handling integer parts, decimal points, and scientific notation signs.
- `convert_integer_to_ascii`: Checks a word at an offset from a pointer and branches based on its value.
- `convert_integer_to_string`: Converts a 16-bit integer value into a null-terminated ASCII string representation.
- `execute_callback_one`: Executes a function using a pointer retrieved from a stack location.
- `execute_deferred_callback`: Checks a global variable to determine if a specific state has been initialized, then executes a function pointer retrieved from a stack-allocated structure.
- `dispatch_callback_via_pointer`: Checks a global variable to determine if a specific routine needs initialization, then executes a function using a pointer retrieved from a stack-allocated structure.
- `dispatch_callback_via_pointer_2`: Checks a global variable to determine if a specific value is initialized, then executes a function using a pointer retrieved from a stack-allocated structure.
- `execute_callback_with_initialization`: Checks a global variable to determine if a specific value is already stored; if not, it performs a series of recursive calls to initialize the state.
- `dispatch_callback_via_pointer_3`: Checks a global variable to determine if a specific value is initialized, then executes a function using a pointer retrieved from a stack-allocated structure.
- `execute_callback_with_initialization_2`: Checks a global variable to see if a value is already stored; if not, it performs a series of initialization calls.
- `dispatch_callback_via_pointer_4`: Checks a global variable to determine if a specific value is initialized, then executes a function using a pointer retrieved from a stack-based structure.
- `execute_callback_via_function_pointer`: Checks a global variable for a stored value, potentially initializing it by calling other routines, and then executes a function using a pointer and an offset retrieved from memory.
- `execute_library_callback`: Calls a library function and, if the result is zero, performs an error routine before executing an indirect callback.
- `chain_result_to_output`: Passes the result of a function call to a write wrapper followed by an output wrapper.
- `call_output_3`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_552_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `add_and_extract_high_word`: Adds a constant to the result of another routine, stores the 32-bit result in a global variable, and returns the upper 15 bits of the result.
- `store_high_word`: Stores the upper 16 bits of a 32-bit parameter into a global variable.
- `traverse_pointer_list_2`: Recursively traverses a list by comparing two values in a pointer array and increments the first value if it is less than the second.
- `execute_recursive_loop`: Performs a nested loop structure that calls another routine repeatedly until a specific condition is met or a return value of negative one is received.
- `traverse_pointer_list`: Recursively traverses a list by comparing two values in a pointer array, incrementing the first value if it is less than the second, or calling itself otherwise.
- `traverse_and_update_structure`: Checks status flags in a structure and, depending on the results, traverses a list of structures or updates the structure's pointer and length fields.
- `find_string_and_return`: Searches for a string in a table and returns either a null value or the result of another routine using the found entry.
- `find_string_in_table`: Compares a provided string against entries in a table and, upon a match, updates a specific structure with a value and a flag.
- `find_byte_in_string`: Scans a zero-terminated string from its start for the first occurrence of the given byte and returns its position, or zero when the terminator comes first.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `nop_callback_placeholder`: This routine is empty and performs no operations before returning.
- `execute_indirect_jump`: Performs an indirect jump to an address stored in memory.
- `pass_argument_to_table_search`: Passes its argument to a routine that searches a table of pointers and potentially updates an entry.
- `search_and_update_table`: Iterates through a table of 6-byte entries to find an empty slot and updates it with new data or performs file operations if specific bits are set in a parameter.
- `read_file_data`: Performs a file read operation and checks the result for errors.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `traverse_string_for_callbacks`: Iterates through a null-terminated string and executes a routine for each character until a null terminator is reached.
- `initialize_system_state`: Zeros out a memory range, updates global variables, and calls routines via pointers from a global structure based on specific bit flags.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tokenize_string_into_array`: Parses a string into individual tokens by splitting on whitespace or handling quoted substrings.
- `set_register_value`: Moves a value into a register and performs a short branch.
- `copy_string_with_limit_3`: copies characters from a source string to a destination string until a null terminator is reached or a specified length counter reaches zero.
- `copy_and_pad_string`: copies characters from a source to a destination and pads the remaining destination space with null terminators until a length counter is exhausted.
- `multiply_32_bit_values_2`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `populate_structure_from_library`: Retrieves data from a library-based structure and stores several values into a provided structure.
- `copy_d0_to_d5`: Copies the contents of data register zero to data register five.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: Prepares two signed values for division by adjusting their signs and storing a flag in a register before branching to another routine.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the original D1 contents with the value left in D1 by the call.
- `recursive_call`: The routine calls another routine and then returns.
- `perform_32bit_division`: calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1, returning the result in a concatenated 64-bit value.
- `call_update_status_wrapper`: Calls a single routine that wraps two other subroutines.
- `update_status_wrapper`: A routine consisting of 22 instructions that calls two other routines.
- `update_structure_status_wrapper`: This routine acts as a wrapper that calls a routine to update structure status flags and field contents based on recursive processing results.
- `traverse_and_clear_structure`: Iterates through a memory range using a loop to recursively process a data structure.
- `clear_and_traverse_structure`: Recursively traverses a structure based on bit flags, performs deletions or removals, and clears the fields of the current structure.
- `update_structure_status`: Checks status flags in a structure and updates its fields or clears its contents based on the results of recursive processing.
- `find_null_in_table`: iterates through a table of data structures to find an entry where a specific byte offset contains a null value, returning a pointer to that entry.
- `update_status_flags`: Calls another routine and updates various fields in a structure based on the returned value.
- `setup_stack_and_branch`: Clears a register, saves registers to the stack, loads a value from a memory offset into a register, and performs a short branch.
- `traverse_and_clear_list`: traverses a linked list and calls another routine on each element before clearing a global pointer.
- `update_linked_list_node`: Writes the routine's address to a memory location and updates a pointer-based structure with a new value and a parameter.
- `forward_parameter_field`: The routine forwards the second field of its parameter structure to another routine and then returns.
- `remove_from_linked_list`: searches for a specific node within a linked list and removes it by updating the pointers of the preceding node or a global pointer.
- `validate_index_and_call`: checks if an index is within valid bounds and if a corresponding entry in a table is non-zero before calling another routine.
- `handle_file_deletion`: attempts to delete a file and, if unsuccessful, retrieves an error code and stores it in a global variable.
- `write_and_check_io`: Attempts to write data using a DOS wrapper and checks the result, potentially calling an error routine if the operation fails.
- `check_function_result`: Calls a library function and checks if a specific bit is set in the return value.
- `execute_indirect_callback`: Checks if a pointer stored at a memory offset is non-zero and, if so, executes the function at that address.
- `process_structured_data`: Iterates through a loop based on a table limit, executes several library functions via offsets, and performs multiple indirect function calls and pointer assignments.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `close_table_entry`: Validates an index against a table size and checks if a pointer in the table is non-zero.
- `call_dos_close`: Performs an indirect call to a function located at an offset within a structure pointed to by a global variable.
- `call_close_4`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `execute_indirect_function_pointer`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_current_dir_4`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay_2`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input_3`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err_2`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested offset within a structure and executes it.
- `call_lock_2`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `execute_indirect_function`: Retrieves a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_open_4`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `execute_indirect_function_2`: Retrieves a function pointer from a nested table structure using an offset from a global variable and executes it.
- `call_read_3`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek_2`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `execute_indirect_function_3`: Retrieves a function pointer from a nested offset within a structure pointed to by a register and executes it.
- `call_un_lock_4`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_480_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_library_vector_168_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_function_4`: Retrieves a pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_414_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_630_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 630 of that base, returning its result to the caller.
- `allocate_and_initialize_record`: Retrieves a pointer via a library call and populates a structure with provided parameters and a new pointer retrieved from another routine.
- `release_and_reset_record`: If a pointer or certain field is non-zero, it calls a routine, then sets a byte and a long to all ones, and calls other routines with a byte or a constant.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `assign_record_fields`: Calls another routine to obtain a pointer and then stores a byte, a word, and an integer into that pointer's memory locations.
- `reset_record_values`: Sets a byte and two long values to all ones within a structure and then calls another routine using a value from the structure.
- `call_library_function`: Performs an indirect call to a function located at a specific offset within a library base retrieved from a global variable.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_456_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_126_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_function_2`: Executes a function by accessing a library base through a global variable and applying a specific offset.
- `call_library_vector_294_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a routine by calculating an address from a library base stored in a global variable and an offset.
- `call_library_vector_132_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_function_4`: Uses a global variable to locate a library base and then jumps to a function at a defined offset.
- `call_library_vector_210_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_function_5`: Retrieves a library base from a global variable and executes the function found at the specified offset.
- `call_library_vector_372_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain_2`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_function_6`: Performs an indirect jump to a library function using an offset and a base address from a global variable.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_function_7`: Calls a routine by dereferencing a library base from a global variable and adding a constant offset.
- `call_library_vector_378_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_312`: Library stub: loads a library base from a global variable and calls the library function at offset minus 312 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_function_8`: Executes a function located at a specific offset relative to a library base held in a global variable.
- `call_library_vector_384_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_30_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_78_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_372_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_594`: Library stub: loads a library base from a global variable and calls the library function at offset minus 594 of that base, returning its result to the caller.
- `call_library_vector_378_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_564_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_390_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_192_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_210_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_456_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_306_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_354_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_288_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_438`: Library stub: loads a library base from a global variable and calls the library function at offset minus 438 of that base, returning its result to the caller.
- `call_library_vector_54_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_84_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_198_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_210_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_444_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_252_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_294_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_30_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.

### Nuclear War (1989)(New World Computing)[cr PNA](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Nuclear War (1989)(New World Computing)[cr PNA](Disk 2 of 2).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `return_without_action`: The body contains only a return, with no calls, memory access or arguments used.
- `test_library_bit`: Decrements a word at an offset from a pointer, loops if the result is non-zero, and performs a bit test on a byte before branching or jumping.
- `return_constant_zero`: The body only returns zero, with no calls, memory access or arguments used.
- `call_other_routine_2`: Calls another routine and then returns.
- `dispatch_font_request_2`: As part of library code, this routine reads a pointer and an argument from the stack and passes them to another routine.
- `compute_scaled_offset`: Computes an integer from two structures using subtraction, scaling, and a loop that subtracts values from a global table based on byte comparisons, then returns the result.
- `load_and_install_disk_font`: Opens a disk font file, reads its header, and allocates memory for the font data.
- `copy_null_terminated_string_2`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `load_registers_and_branch`: Loads two values from the stack into registers and performs a short branch.
- `skip_to_string_terminator_and_call_2`: The routine increments a pointer until it reaches the terminating zero byte of a zero-terminated string and then calls another routine with the pointer to that terminator.
- `count_string_length`: Receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `compare_string_bytes`: Compares two null-terminated strings and returns a value indicating their relative order.
- `test_and_branch`: Tests a byte at a pointer location, increments the pointer, and branches based on the result or returns zero.
- `compare_and_set_byte`: Compares a byte from one memory location against a byte from another and sets a register to a specific value based on the result.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `find_last_occurrence_in_string`: The routine receives a pointer to a zero-terminated string and a byte value, determines the string length via a library call, then walks backwards from the end to find the last occurrence of the byte and returns its pointer.
- `find_delimiter_and_advance_pointer`: The routine searches a string for delimiters and advances the pointer past the first found delimiter, returning the adjusted pointer.
- `call_library_vector_30_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_84_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_126_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_210_7`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_132_5`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_8`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_78_6`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_480_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `call_subroutine_and_return`: This routine calls another routine and then returns to its caller.
- `return_zero_constant`: This routine returns a constant zero value to the caller.
- `perform_no_operation_2`: This routine is empty and performs no actions before returning.
- `mask_byte_and_increment_short`: Clears the most significant bit of a byte at a memory location and increments a short integer at another location relative to a base pointer.
