# Game 1279 (dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01279). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914128 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 30 raw-word lines |
| Functions named and described | pass, 467 of 467 named (0 author symbols), 467 of 467 described (24 by rule, 441 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2210 of 2210 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 36864 bytes, 22 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10437 of 10437 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22830 |
| code | 35712 |
| data | 834252 |
| padding | 1878 |
| unused | 19456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:escape | 36 | 7880 | 389 |
| Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:lair2 | 1 | 89 | 3 |
| Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1037 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 381 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:lair2#unpacked | 1 | 1037 | 22 |

## Cases handled

- extended reference run ignored: it does not re-execute 48 of the 2186 PCs the canonical run logged (2138 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- runtime segment an address (f100): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 808 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 224 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:lair2: crunched executable unpacked with ancient scan (TetraPack 2.1 (xfd), 20816 -> 36864 bytes), 18 of 18 windows confirmed in reference RAM, loaded at an address: recovered as u7
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 4 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 immediate, 1 operand
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf: blocks 2-508 (bytes 1024-260608) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.41)
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf: blocks 514-514 (bytes 263168-263680) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf: blocks 517-664 (bytes 264704-340480) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.94)
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf: blocks 882-1002 (bytes 451584-513536) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.15)
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf: blocks 1009-1009 (bytes 516608-517120) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- dragon's lair - escape from singe's castle (1989)(readysoft)(disk 1 of 5)[cr cls][t +1 cls]/Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:lair2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 packed track region(s), 378880 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_wait` (escape): Sets hardware registers for CIAA and custom memory, clears a bit in a global variable, and enters a loop that tests bit 4 of a hardware register, calling another routine up to 99 times. Kind: disk hardware/track sync check. Pass path: the loop continues or the code proceeds to the instruction after the call (e.g., the bsr.w an address or the loop incrementing an address).
- `execute_task_sequence` (escape): Clears a global counter, calls three different routines in a loop, and if they all succeed, performs multiple writes to a structure in a1 and calls two more routines before returning when the counter reaches 4. Kind: disk/track integrity check. Pass path: u0__sub_23_0352() returns with uVar3=true (loop terminates).
- `initialize_disk_buffer_and_hardware` (escape): If a global byte is zero, it sets it to 255, copies data from one buffer to another in three distinct stages including a pattern fill, manipulates hardware registers via a pointer in a5, and performs a series of calls and loops that include hardware register configuration and a data integrity check via a comparison loop. Kind: disk hardware/track check. Pass path: the branch where iVar8 == 0 (skipping the u0__sub_31_0000(0) error/reset path).
- `initialize_linked_list_structure` (escape): Calls two routines to allocate and setup a structure, then if successful, writes 9 to offset 28 of a table pointed to by a1, clears offset 30 of that table, and calls a routine using a pointer from a global variable. Kind: disk/track integrity check. Pass path: the branch where in_ZF is true (skipping the initialization/trap logic).
- `release_allocated_resources_3` (escape): Checks a value at a memory location, and if non-zero, writes 10 to offset 28 of a table, writes 246 to offset 40 of that table, clears offset 30, and calls two routines before branching. Kind: loader/resource cleanup with conditional device/file handle release. Pass path: The branch where _u0__d33_0230 is set to 0 and the function returns normally after the device/file handle check..

## Functions


### Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:escape

- `jump_to_null_address`: Performs an unconditional jump to address zero.
- `initialize_data_registers`: Sets bit 4 of a byte at the address in a4 based on d4, then performs a series of logical OR operations to populate d0, d1, d2, and d3 with specific constant values.
- `manage_state_and_update_graphics`: Updates a structure in a6 with various parameters based on the value of a global variable, and performs bitwise operations and table lookups to modify sprite attributes or memory values.
- `process_random_data_table`: Performs complex bitwise operations and loops on data from two global pointers to generate or verify values, potentially updating a global pointer and a global byte.
- `update_sprite_attribute_and_fetch_data`: Modifies a bitmask in a register based on the difference between two global bytes, then uses a global value to index into a table and stores the resulting word into a structure at offset 126 of a6 before calling another routine.
- `process_data_table`: Calculates an index using a value from a table and a global variable, then uses that index to fetch a byte, updates a global byte with a bitmask, and potentially calls two other routines before updating a global word and byte.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `set_status_byte`: Tests a byte at a specific address, and if it is zero, sets a byte at another address to 40 and returns after setting d0 to 255.
- `calculate_offset_bounds`: Calculates a value based on an input pointer and several global variables, applying conditional offsets and bounds checking.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `set_byte_and_return`: Checks a byte at a global address, and if it is zero, writes the value 40 to a location pointed to by a global variable before returning with d0 set to 255.
- `check_bounds_and_offset`: Tests three bytes from a global table, and if they are all zero, compares a value from a global pointer against a limit, potentially adding offsets to it before returning zero or a large value.
- `return_255`: Sets d0 to 255 and returns.
- `update_structure_fields_2`: Calculates new values based on a pointer in a0 and global variables, writes them to offsets of a structure in a6, updates the pointer in a0, and sets a byte in a global variable to 255.
- `manage_buffer_and_pointers`: Checks three bytes for non-zero values, compares a global longword against a3, and if certain conditions are met, traverses a linked list of structures to update a pointer and a byte.
- `copy_and_update_buffers`: Calculates a size from a subtraction, performs a loop to copy data from one buffer to another while setting a bit in a structure, and updates several global pointers and a byte.
- `restore_registers`: Restores registers d2 through d4 and a1 from the stack and returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decrypt_and_copy_data`: Uses a table and XOR operations to decrypt a sequence of words and copies them to a destination buffer calculated from a global variable.
- `clear_byte`: Clears a single byte at a global address.
- `reset_and_update_tables`: Clears several bytes and words in global variables, sets a byte to 255, and if a global byte is non-zero, performs a series of table lookups, calls another routine, and executes a loop that increments a pointer based on values in a table before storing the result in a global variable.
- `process_bit_manipulation`: Performs a loop of bitwise rotations, inversions, and XOR operations on a value from a global variable, then stores the high word of the result in a global variable.
- `clear_and_test_flag`: Clears a byte in a global variable and sets bit 0 of the CIAA interrupt mask register; if the cleared byte is zero, it enters an infinite loop, otherwise it returns.
- `configure_hardware_and_wait`: Sets hardware registers for CIAA and custom memory, clears a bit in a global variable, and enters a loop that tests bit 4 of a hardware register, calling another routine up to 99 times.
- `toggle_interrupt_mask`: Sets a bit in a register, clears a bit specified by a global word in that same register, calls another routine, sets the bit back, and enters an infinite loop.
- `clear_bit_and_loop_routine`: Clears a bit in a hardware register using a bit index loaded from a global word, then calls another routine twice within a loop that repeats 199 times.
- `wait_for_button_and_loop`: Tests bit 5 of a hardware register, and if set, calls another routine within a loop that runs 99 times before setting a bit in a register and entering an infinite loop.
- `execute_task_sequence_2`: Sets a global variable to 1, calls another routine, pushes a value from a global variable onto the stack, calls another routine, and if the result is not zero, performs several writes to global variables and calls two library routines via a6, otherwise it enters a loop that calls another routine while incrementing a global variable from 3 to 7.
- `clear_and_call_routine`: Calls another routine, loads a longword from offset 140 into d0, branches if zero, calls a library routine via a6, pushes a value from a global variable onto the stack, copies d0 to a global variable, calls another routine, clears the longword at offset 140, restores a value from the stack to a global variable, and clears a longword at offset a6.
- `execute_task_sequence`: Clears a global counter, calls three different routines in a loop, and if they all succeed, performs multiple writes to a structure in a1 and calls two more routines before returning when the counter reaches 4.
- `loop_hardware_configuration`: Writes 3 to a global variable, enters a loop where it calls another routine, increments the global variable, and repeats until the variable equals 7, then resets the variable to 3 and returns.
- `allocate_device_resource`: Allocates memory for a device, sets specific bytes and words at offsets 8, 14, and 18 of the allocated structure, and updates a global pointer with the result of a second allocation call.
- `initialize_task_structure`: Allocates memory, sets a byte at offset 15 of the structure, and if a subsequent allocation succeeds, clears several fields in the structure and builds a linked list of structures starting at offset 20.
- `release_allocated_resources`: Clears a global variable and calls a routine to free the memory at that address, then clears another global variable and calls a routine to close a device.
- `free_task_and_memory`: Checks a byte at offset 15 of a structure pointed to by a global variable; if it is not negative one, it calls a routine to free a signal, then clears the global variable and calls a routine to free memory.
- `allocate_and_execute_tasks`: Allocates memory using a library, calls a routine to set a global byte, performs several more memory allocations, and if successful, initializes a structure and calls two routines before freeing the allocated memory.
- `execute_callback_sequence`: Stores the value of a register into a global variable and calls three different routines using pointers from a table.
- `verify_and_load_disk_data`: Checks if a disk is inserted and compares a value from a global variable against a table to verify a signature; if valid, it performs a series of bitwise operations and shifts to reconstruct a multi-byte value from several global bytes and stores the result back into a global variable before calling another routine.
- `toggle_hardware_bits_loop`: Calls another routine, modifies bits in a structure in a5, and repeatedly calls another routine while performing bitwise operations on a5 and reading a byte from a hardware register until a loop is entered.
- `clear_bit_and_wait`: Sets a bit in a register, clears a bit specified by a register in that same register, calls another routine, reads a hardware register, sets the bit back, and enters an infinite loop.
- `dispatch_command_id_3`: Compares a value in register d0 against several constants to branch to different subroutines or set a global byte to specific values.
- `set_byte_to_80`: Sets a byte at offset 899 to 128 and returns.
- `set_byte_to_ff`: Sets a byte at offset 7de to 255 and returns.
- `clear_byte_at_7de`: Clears a byte at offset 7de and returns.
- `initialize_and_populate_data_structures`: Initializes several data structures by performing a series of calls to other routines, populates a table with values from a memory block using two loops, and updates various fields in a structure in a6 using calculated offsets and constants.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `setup_and_jump_to_routine`: Loads a pointer from a global variable into a2, loads a pointer from another global variable into a3, sets a byte in a global variable to 255, loads a pointer from a global variable into a0, adds 10 to d1, pushes a pointer from a0 onto the stack, and jumps to another routine.
- `prepare_stack_pointer`: Sets a byte at offset 3d4 to 1, subtracts 28880 from d0, shifts d0 left by 2, and pushes the address of a global variable onto the stack.
- `calculate_and_store_values`: Calculates a new pointer by shifting d0 left by 3, adding it to a base pointer from offset 872 plus 4096, and storing it in offset 876; then, based on a byte at offset 3ec and a word at offset 84e, it performs arithmetic to fetch two words from a table and stores them in offsets 3da and 3dc.
- `clear_byte_at_868`: Clears a byte at offset 868 and returns.
- `execute_initialization_sequence`: Loops until a global byte is zero, sets a global byte to 255, calls several routines including a library sequence, and if a returned value is non-zero, it jumps to a routine via a pointer from a global variable.
- `perform_bitmask_blitting`: Performs a nested loop to copy and merge data between two memory blocks using bitwise logic, where each iteration calculates a mask based on values at specific offsets and writes the results to a destination table and several other memory buffers.
- `execute_library_sequence`: Clears a structure, calls five library routines, searches a table for a specific word, and then executes several loops of library calls to process data from multiple tables before performing a final jump.
- `update_memory_pointers`: Clears a byte at a global address, calls another routine, copies a 32-bit value from a global variable to an offset of 44 of a structure pointed to by a global variable, and calls two functions from a table before performing an indirect jump using a third table.
- `swap_memory_blocks`: Performs a nested loop to swap two blocks of memory, where the outer loop runs five times and the inner loop runs 511 times.
- `check_status_and_increment_byte`: Checks if several bytes at specific global locations are zero, and if so, performs a series of checks on a byte from a global variable.
- `load_address_a0`: Loads a constant address into register a0.
- `push_call_and_mask_bit`: Pushes the contents of a0 onto the stack, calls a routine at the address stored in a0, restores a0 from the stack, and clears bit 8 of a word at a global address.
- `validate_structure_data`: Calls two routines, clears a bit in a global word, calls another routine, compares a global longword to a constant, and if it matches, accesses a structure via a global pointer to compare a word at offset 38 against two different constants; depending on the result, it either branches, sets d0 to negative one, or sets two global words with bit 8 enabled.
- `dispatch_event_id`: Compares a 16-bit value in register d0 against several constants and branches to different subroutines based on the match, otherwise it returns.
- `initialize_structure_and_call_2`: Clears a byte in a global variable, sets a byte in another global variable to 255, and if certain global bytes are non-zero, pushes three pointers onto the stack, calls another routine, and performs several conditional checks and calls involving a structure in a6 and a pointer from a global variable.
- `store_pointer_data`: Writes 255 to a global address, checks a pointer in a2, and if it is null, writes a global longword to the location in a2 and another longword to a different global address.
- `load_and_call_pointer`: Loads a pointer from offset 98c into a0, pushes the long value at offset 6 of the structure in a0 onto the stack, and loads the word at offset 2 of the structure in a2 into d1.
- `initialize_structure_parameters`: Writes several words to a structure in a6, performs a comparison on a longword from a global variable, calls another routine, and if two specific global bytes are non-zero, updates offsets 168 and 184 of the structure in a6 before performing further writes and returning.
- `clear_byte_at_offset`: Clears a byte at offset 3e5 and returns.
- `compare_pointer_value`: Loads a longword from offset 2 of the structure in a2 into d0, compares it to a constant, and branches if they are not equal.
- `set_status_byte_2`: Loads a longword from offset 2 of the structure in a2 into d0, branches if d0 is zero, otherwise it sets a byte at offset 7e0 to 255.
- `loop_and_clear_byte`: Pushes d0 onto the stack, calls another routine, sets a3 to the value in d0, increments a global variable, restores d0 from the stack, decrements d0 and branches to itself if not zero, clears a byte at a global variable, and returns.
- `execute_pointer_routines`: Calls another routine, calls another routine, and then returns.
- `decrement_and_increment_counter`: Decrements a word at offset 2 of the structure in a2 if it is non-zero, and if it is, increments a longword at a global address.
- `adjust_and_compare_pointer`: Loads a pointer from offset 6 of the structure in a2 into a2, checks a byte in a global variable, and if it is not zero, adds 3398 to a2 and branches to another routine.
- `compare_and_load_word`: Adjusts a pointer in a2, then compares the word at offset 10 of the structure in a2 against two specific values; if a match is found, it loads a longword from offset 22 of the structure in a2 into d0.
- `call_on_id_match`: Compares a word at an offset in a2 against three specific values, and if a match is found and a longword at an offset in a2 is non-zero, it calls another routine.
- `parse_and_relocate_dos_hunks`: Iterates through a memory buffer to identify and process AmigaDOS hunk types, performing memory allocations and relocations based on the hunk data.
- `manage_segmented_data_buffers`: Iterates through a list of structures to update their contents, performs memory allocations and deallocations using library calls, and copies data between tables using loops.
- `transfer_and_jump`: Disables interrupts, copies a longword from a table at a5 to a destination at a1, and jumps to a location if the destination value is not equal to a constant.
- `copy_and_patch_memory`: Copies a block of memory from a source to a destination, then scans memory segments to write a specific 16-bit magic word into locations that match a value stored at a pointer.
- `scan_memory_for_magic_word`: Searches through memory segments for a specific 16-bit magic word, updating a global variable with the last seen value and incrementing the pointer by 256 kilobytes in a loop, otherwise it returns a pointer to the found location or zero.
- `process_linked_list`: Iterates through a list, updating values in a structure at a2 and a3 based on a count in d1, and clears memory at a1 in a loop.
- `release_system_resources`: Calls a routine seven times with varying parameters from a table, then performs a series of library calls to clear pointers and free memory if a specific global byte is non-zero.
- `free_and_clear_memory`: Clears a pointer in a global variable and calls a routine to free it, then iterates through a table to clear entries and call a second routine to free them.
- `update_address_tables`: Copies several longwords from a table at a0 into various global variables, including several that are modified by adding specific constants like 8192, 43008, or an address before being stored.
- `update_address_table`: Reads three longwords from a table at a0 and stores them into three different global variables, adding specific offsets to each value before storage.
- `initialize_hardware_registers`: Writes specific byte and word values to several hardware registers, including CIA and INTENA.
- `manage_interrupt_and_copy_data`: Writes a longword to a global variable, modifies the INTENA register, calls two routines, and if a specific bit is set in the interrupt status register, enters a loop that copies 4096 words from a source to a destination.
- `wait_for_interrupt_flag`: Writes to the INTENA register and enters loops that poll the status of an interrupt flag, eventually writing a value to the DMA control register.
- `poll_interrupt_status_loop`: Sets a loop counter and writes to the INTENA register, then repeatedly polls an interrupt flag for up to 14 iterations before writing a final value to INTENA.
- `reset_hardware_and_copy_data`: Sets interrupt and DMA registers, copies a block of data from a table to a destination if a byte at a global variable is non-zero, and calls two routines after updating hardware registers.
- `process_segmented_data`: Calculates a value based on a table entry, and if certain global byte conditions are met, it either calls one of two different routines or, if a value from the stack is between 8 and 12, saves registers, calculates a destination address using a pointer and an offset, and calls a routine at an address stored in a register.
- `copy_segmented_data_blocks`: Calculates the number of data blocks of size 5632 from a source pointer and enters a loop to copy bytes from a source table to a destination buffer.
- `allocate_and_initialize_string`: Calls a routine to check a condition, sets a byte at a specific memory location to 65, and if successful, uses several library functions to allocate memory, copy a string, and perform cleanup operations before returning.
- `reset_status_bytes`: Writes specific byte values to several global locations and calls another routine, then checks a byte at a global variable and returns if it is zero or equals 30.
- `setup_copy_parameters`: Loads the values 1028 and 340 into registers d2 and d3, then calls another routine.
- `initialize_disk_buffer_and_hardware`: If a global byte is zero, it sets it to 255, copies data from one buffer to another in three distinct stages including a pattern fill, manipulates hardware registers via a pointer in a5, and performs a series of calls and loops that include hardware register configuration and a data integrity check via a comparison loop.
- `process_interleaved_data`: Clears a long at a specific location, calls another routine, writes a constant to a pointer in a1, performs several bitwise operations and loops to update a long value using a mask, and calls another routine multiple times.
- `split_and_interleave_data`: Splits words from a source buffer into two destination buffers by masking bits, then performs a bitwise manipulation on a third destination buffer using a loop.
- `copy_memory_block_2`: Copies bytes from a source address to a destination address using a loop counter in d3, provided a global pointer is not zero.
- `setup_memory_copy`: Loads a global pointer into a0, adds d2 to it, subtracts a constant, sets a1 to d2, and branches to another routine.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a1 one byte at a time, using a dbra loop counter, and returns.
- `calculate_and_set_flag`: Calls a routine twice, calculates a value by multiplying d7 by 128 and adding a constant, calls another routine, sets a byte at a global location to 255, calls a final routine, and clears that same byte.
- `calculate_memory_offset`: Multiplies a value in d7 by 128, adds 1368 to the result, and then calls another routine before branching to another location.
- `compare_and_clear_buffer`: Adds 2 to a pointer in d2, calls another routine, compares the result in d0 with the value at the pointer, and if they are not equal, enters a loop to clear bytes at the pointer location until a counter reaches zero.
- `clear_and_copy_data`: Calls another routine, clears a byte at the pointer in a0 and loops 371 times, sets several values in a global table, calls another routine to copy 4 blocks of 128 bytes, and finally calls another routine to copy 980 bytes from a global pointer.
- `load_constants`: Loads the value 1030 into a0 and 166 into d1.
- `sum_buffer_values`: Adds words from a buffer pointed to by a0 to an initial value of 21068, repeating for the count in d1.
- `store_constant_result`: Calls another routine and stores the resulting value from d0 into the memory location pointed to by a0.
- `update_buffer_with_result`: Adds 2 to a pointer in d2, calls another routine, and stores the resulting value from d0 into the memory location pointed to by the pointer in d2.
- `initialize_system_resources`: Checks a global byte, calls two library routines to allocate memory, populates a table with calculated offsets, calls another routine, uses library routines to free memory, and clears specific bytes in a structure.
- `dispatch_segmented_data_2`: Calls two routines, then compares a value from a structure at offset 38 of a pointer in a global variable against two different tables; if a match is found, it calls a routine from one of those tables and loops.
- `call_wait_port`: Tests a byte at a global address and branches if it is zero; otherwise, it loads a pointer from a global variable into a6 and calls a routine at an offset of negative 448 from a6.
- `process_message_and_reply`: Calls a routine via a pointer in a global variable, and if the result is not zero, copies three fields from the returned pointer to global variables before calling another routine via a pointer in a global variable.
- `dispatch_segmented_data`: Updates a global byte, clears an offset in a structure, calls a routine, and then uses a table to select and call one of several different routines based on the value of the global byte.
- `traverse_pointer_chain`: If a global variable is non-zero, it follows a chain of pointers starting from an offset of 62 in a structure, calling another routine if a specific bit is set in each element; otherwise, it performs an indirect jump using a pointer from a global variable.
- `toggle_status_bit`: Performs an eor operation with 128 on a byte at offset 409, tests that byte, and if it is not zero, sets a byte at offset 3fd to 255.
- `sync_status_bits`: Loads addresses from global variables into a0 and a1, calls another routine twice with different address pairs, calls a third routine, and calls a fourth routine twice with different address pairs.
- `set_byte_four`: Stores the value 4 into a global variable at offset 140 and calls another routine.
- `set_byte_three`: Stores the value 3 into a global variable at offset 140 and calls another routine.
- `set_byte_two`: Stores the value 2 into a global variable at offset 140 and calls another routine.
- `set_byte_one`: Stores the value 1 into a global variable at offset 140 and calls another routine.
- `execute_handler_sequence`: Calls five different routines in sequence and then performs an unconditional branch to an earlier instruction in the routine.
- `swap_memory_blocks_2`: Calculates a loop count based on a global variable, then swaps words between two memory locations using a loop.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `process_status_and_copy_data`: Calls another routine, toggles a bit in a global variable, and enters a loop that calls another routine, calls a third routine, and performs conditional calls and data copying based on the results.
- `load_string_pointers`: Calls another routine, loads the address of a string into a0, tests d0 and branches if zero, otherwise it loads a different string address into a0, sets d0 to 73, sets d1 to 153, and branches.
- `execute_conditional_calls`: Calls two different routines, checks a byte in a global variable, and if it is not zero, checks d0 and branches to another location.
- `execute_conditional_logic`: Calls two different routines via a global pointer, performs a loop of 4 iterations calling another routine if certain conditions are met, and then calls two more routines using values from the stack to compare them and return a result.
- `execute_conditional_logic_2`: Calls a routine, checks a byte in a global variable, and if it is non-zero, loads a pointer from a global variable into a6, calls a routine with d1 and d2 as arguments, and then calls another routine if d0 is non-zero.
- `process_buffer_math`: Calls three different routines via a global pointer, calculates a value by multiplying d7 by 128, pushes this to the stack, calls another routine, and finally calls another routine to update a buffer.
- `process_buffer_with_math`: Calls another routine, checks a global byte, and if zero, uses a library routine to set d1 and d2, then performs a multiplication on d7 and d2, pushes the result to the stack, and calls three library routines and another routine.
- `sync_bit_status_chain_2`: Inverts the high bit of a byte in a global variable, then performs two consecutive cycles of extracting high bits from a three-element pointer chain into a bitmask, XORing that mask with a constant, and writing the result back into the high bits of the same pointer chain.
- `check_and_allocate_memory`: Tests a byte in a global variable, and if non-zero, tests another byte in a global variable; if that is also non-zero, it loads two addresses into registers, tests a byte in a global variable, and if it is zero, calls another routine before loading a longword from a global variable into d1 and branching.
- `validate_and_mask_memory_offset`: Compares a longword loaded from a pointer in a3 against an address; if the value is greater than or equal to that constant, it branches to another routine, otherwise it performs a bitwise OR on a word at a longword offset in a global variable and branches to another routine.
- `update_memory_segment_pointers`: Updates a series of global pointers with offsets derived from a base address, modifies a value in a register, and clears a byte in a global variable before branching.
- `setup_memory_pointer`: Loads a pointer from a global variable, adds an offset to it, tests a byte in a global variable, and if it is zero, calls another routine before writing a longword to an offset of 4 from the modified pointer and branching.
- `allocate_and_verify_memory_segments`: Checks if three memory segments are allocated by inspecting a structure, and if not, requests memory of sizes an address, 32768, and 13824 bytes respectively.
- `search_pointer_list`: Loads a pointer from an offset of 1636 in a0, compares the value at offset 128 of that pointer with d1, and if they are equal, branches to another location; otherwise, it increments the pointer by 4 and loops.
- `check_and_branch_pointer`: Loads a long from a pointer in a0, compares it to a constant, and branches to another location or loads a new address into a1 based on the result.
- `shift_and_clear_data`: Shifts a block of 32-bit words within a structure, then clears a long value at a1, a long value at offset 128 of a1, and a byte in a global variable.
- `reset_buffer_and_call_allocator`: Clears a byte at a global address, sets a byte to 255 in another global location, and checks a byte in a third global location; if that byte is not zero, it calls another routine with a constant value in register d2 and a pointer from a global variable in register d0, then sets a byte to 255 in a fourth global location before branching to another routine.
- `xor_bitmask_sync_pointer_chains`: Calls a subroutine with a specific value in d2, clears a 4-byte field at offset 52 of a structure pointed to by a global pointer, and sets a global byte to 255 before performing a series of bitwise operations that extract high bits from two separate three-element pointer chains, XORs them with constants, and writes the resulting bitmask back into the chain elements.
- `execute_trap_sequence`: Tests a byte at a specific location, and if it is not zero, loads two long values from a table into a0 and a1, clears a long at a specific location, and executes a trap.
- `execute_jump_table`: Loads a value from an offset of 2 into a0 and jumps to the address contained in a0.
- `adjust_stack_pointer_and_return`: Adds 4 to the stack pointer and returns immediately.
- `clear_byte_2`: Clears a byte in a global variable at offset 140 and calls another routine.
- `update_sync_bit_mask`: Sets a 16-bit value in a global variable to 7968 and stores it at a memory location, then calculates a bitmask by setting a single bit based on the lower 5 bits of that value and stores the resulting mask in another global variable.
- `toggle_and_sync_bit_status_chain`: Toggles the high bit of a global byte, then iterates through two separate three-element pointer chains to extract the high bits into a bitmask, XORs the mask with a constant, and writes the result back into the high bits of the pointer chain elements.
- `update_status_pointers`: subtracts 772 from d0, stores the result in a long word at offset 84 of a structure, decrements d0, sets bit 0 of d1, and stores d1 in a long word at offset 80 of the structure.
- `sync_bit_status_chain`: Iterates through two separate three-element pointer chains to extract the high bits of each byte into a bitmask, XORs the resulting masks with global variables, and then writes the bits back into the high bit of the bytes in the chains.
- `load_word_and_branch`: Moves a word from a global variable into register d1 and branches to another routine.
- `swap_and_update_bit_groups`: Calculates a value based on a global word, stores it in a global variable, and performs a loop to rotate or swap bits across five different global pointers.
- `swap_memory_block_via_index`: Reads a word from a global variable, masks the lower four bits, subtracts one, and uses the result to calculate an offset for a memory swap loop.
- `calculate_offset_and_call`: Loads a word from a global variable, masks the lower 4 bits, subtracts 1, and calls another routine before branching to another location.
- `sync_bit_status`: compares the high bit of a byte at a pointer in a0 with the high bit of a byte at a pointer in a1, clears bit 7 of the byte at a1, and sets bit 7 of the byte at a1 if the bits were not equal.
- `set_high_bit`: clears bit 7 of a byte at a pointer in a1 and then sets bit 7 of that same byte to match the high bit of a byte at a pointer in a0.
- `apply_bitmask_logic`: If a global byte is non-zero, it swaps a pointer in a global variable and calls another routine; otherwise, it calls a routine via a global pointer, then enters a loop that calls another routine for each non-zero byte in a list.
- `release_memory_buffers`: Calls another routine, then checks two global pointers; if they are non-zero, it clears them and calls a routine for each to release the associated resource.
- `call_and_branch`: Calls another routine and then performs an unconditional branch to another location.
- `swap_memory_blocks_3`: Calls three different routines, performs a loop to swap words between a global variable and a calculated memory location, and then branches back to a previous routine.
- `process_data_and_clear_memory`: Calls another routine, then conditionally executes loops that call another routine based on a value from a global variable, clears several longwords in a table pointed to by a pointer from a global variable, and performs a division-based loop using a value from a global variable.
- `process_data_blocks`: calculates an offset using d7, iterates through a series of data structures to perform bitwise operations between a source table and a destination table using nested loops.
- `format_string_data`: Saves a3 to the stack, retrieves a value from a global table, calls another routine, and then enters a loop to find the end of a null-terminated string and potentially replaces characters with spaces or clears them.
- `search_null_terminated_strings`: decrements d0 by half its value and enters a loop that searches through a list of null-terminated strings starting at a global address.
- `parse_and_decompress_segmented_data`: Parses a segmented data stream by searching for specific magic markers, then either unpacks nibble-encoded words into a table or performs a run-length decompression of bytes into a destination buffer.
- `set_list_constants`: Sets d0 to zero and d1 to 65279, then branches to another location.
- `initialize_list_parameters`: Sets d0 to 256 and d1 to 65535, then loads a pointer from a global location into a0 and adds 62 to it before branching to another location.
- `update_linked_list_values`: iterates through a linked list, performing a bitwise OR and AND operation between d0, d1, and a word at offset 3 of each structure before following the next pointer.
- `handle_messages`: checks a global variable and, if zero, calls a function at an offset from a pointer in a6, then calls another function at a different offset from the returned pointer if the first call succeeded.
- `update_global_bytes`: Stores two bytes from a global variable into the stack, sets one global variable to 1 and another to 0, calls another routine, and then restores the original values from the stack to the global variables.
- `load_address_to_a2`: loads the address of a constant into a2.
- `parse_byte_stream`: Iterates through a buffer in a2, checking for specific byte values, and calls a library routine and another routine if certain global bytes are non-zero.
- `handle_interrupt`: Checks bit 0 of a value read from a hardware register, calls another routine if the bit is set, writes a word to a hardware register, and returns from interrupt.
- `set_global_byte`: Writes the byte negative one to a global variable and returns with d0 set to zero.
- `handle_keyboard_interrupt`: Checks a bit in the interrupt status register, and if set, modifies the CIAA control register, calls another routine, and then clears a bit in the control register before returning from interrupt.
- `set_bit_in_table`: Manipulates bits in d0 and then sets a bit in a table at an offset derived from d0 and d1, then branches to another routine.
- `modify_bit_and_loop`: If the high nibble of d0 is not all ones, it clears a bit in a table at an offset determined by d0 and d1, then enters a loop that decrements d0 from 1000 to zero.
- `initialize_graphics_buffers`: Initializes several memory buffers by filling them with specific values through nested loops, clears a block of memory starting at a hardware register, and updates a global variable with bits from a hardware status register.
- `process_input_state`: Calls another routine, enters a loop that checks the high bits of a CIAA Port A register against a global byte, and conditionally updates a global pointer or toggles a bit at an offset within a structure pointed to by a global variable.
- `validate_boundary_ranges`: Traverses a pointer chain from a global variable and checks if a structure at each step satisfies several boundary conditions involving comparisons against values loaded from other global variables.
- `traverse_tile_pointer_list`: Loads a pointer from a global variable, follows a chain of pointers through a structure at offset 62, and calls a routine if a bit is set in the byte at offset 13 of the current pointer.
- `calculate_and_write_tile_data`: Calculates a memory address using a multiplication of a value from a structure and a constant, then enters a loop that writes specific values to a table based on bitwise manipulations and arithmetic derived from the input structure.
- `check_status_byte`: Calls two other routines and then returns if the byte at offset 6 of the structure in a6 is not decimal 240 or decimal 242.
- `update_status_string`: Reads two values from global variables, performs arithmetic and bit shifts, and writes the resulting bytes into a buffer pointed to by a third global variable.
- `calculate_joystick_delta`: Calculates the difference between a 16-bit value from a hardware register and a 16-bit value from a global variable, performs sign-corrected arithmetic on the high and low bytes, calls another routine, and updates two global variables with clamped values based on offsets 10 and 12 of a structure in a0.
- `apply_bitmask_to_buffer_2`: Saves registers, loads two bytes from global variables, loads a pointer from a global variable, calls a routine, and then enters a loop that runs five times, adding 8400 to the pointer in each iteration.
- `apply_bitmask_to_buffer`: Iterates through a source buffer, performs bitwise operations on bytes using a mask derived from a second buffer, and writes the results into a destination buffer organized in a specific stride.
- `write_constant_pointers`: Calls another routine and then writes two long constants to the memory locations pointed to by a0.
- `populate_data_tables`: Populates multiple data structures with various longword constants and pointers based on the state of several global variables.
- `initialize_pointer_list`: Checks a global long value, calls three different routines, and writes two long constants to the locations pointed to by a0 before branching to another routine.
- `setup_pointer_list_one`: Clears a long value at a global address, calls two different routines, and writes two long constants to the locations pointed to by a0.
- `setup_pointer_list_two`: Clears a long value at a global address, sets d0 to 4, calls two different routines, and writes two long constants to the locations pointed to by a0.
- `populate_data_table_2`: Stores two longwords at the location pointed to by a0, calls another routine, stores a0 in a global variable, calls two more routines, stores a longword calculated by adding d5 to a constant into the location pointed to by a0, stores a0 in a global variable, calls another routine, and branches to another routine.
- `initialize_data_table_structure`: Writes two long constants to a memory location, calls another routine, and then populates a series of global pointers with calculated addresses and associated data values.
- `check_mouse_and_write_pointers`: Compares a byte from a global variable to 77; if equal, it writes a 16-bit value from a global variable to an offset of 7862 of a structure in a1 and updates the global variable, then writes four 32-bit values to the memory pointed to by a0.
- `add_offset_to_pointer`: Adds the value in d5 to an address and stores the result at the memory location pointed to by a0.
- `update_pointer_table`: Compares a pointer in a global variable with another global variable; if they are not equal, it returns, otherwise it sets the second global variable to the value of a pointer in a third global variable.
- `fill_and_copy_data`: Loops through a count in d0, writing incrementing 16-bit values starting from 384 to a0 and copying 16-bit words from a1 to a0.
- `set_loop_constants_and_branch`: Loads the value 224 into register d1, sets register d4 to 6, and performs a short branch to another routine.
- `prepare_loop_parameters_e4`: Loads the value 228 into d1, sets d4 to 6, and branches to another location.
- `write_interleaved_data`: Loops through a count in d0, writing a 16-bit value starting at 224 and incrementing by 4, a 16-bit value containing the high word of a pointer in a1, and a 16-bit value containing the low word of that same pointer, while incrementing a1 by a value in d3.
- `write_table_data_loop`: Writes a word to a0, fetches a long from a global address to a1, adds a second global address to a1, and branches to another routine to begin a loop.
- `populate_data_table`: Writes a 16-bit constant and a long pointer to a memory location pointed to by a0, then enters a loop that writes two 16-bit values and a 32-bit value to a table for two iterations.
- `populate_data_table_via_loop`: Calculates a base address by adding a global pointer and a word from a global variable, then enters a loop that writes four words to memory at that address, incrementing the values and the pointer in each iteration, before branching to another routine.
- `populate_memory_table_via_loop`: Calculates a base address using two global pointers and a value from a pointer in a0, then enters a loop that writes four words to memory at that address, incrementing the destination pointer and a data value by eight each iteration, before calling another routine.
- `initialize_linked_list_structure`: Calls two routines to allocate and setup a structure, then if successful, writes 9 to offset 28 of a table pointed to by a1, clears offset 30 of that table, and calls a routine using a pointer from a global variable.
- `release_allocated_resources_3`: Checks a value at a memory location, and if non-zero, writes 10 to offset 28 of a table, writes 246 to offset 40 of that table, clears offset 30, and calls two routines before branching.
- `initialize_structure_and_call`: Stores a pointer from a0 into a global variable, clears two longwords at offsets 14 and 18 of that structure, writes 512 to offset 4 and 32768 to offset 8, calls another routine, and then branches to another location.
- `branch_to_offset_4`: Loads the value at the address pointed to by a0 into a0 and branches to an offset.
- `move_pointer_to_d0`: Moves the value in a1 to d0 and restores a1 from the stack before returning.
- `process_joystick_input_2`: Clears a longword at offset 16 of a structure, sets offset 6 to 255, calls a routine, then performs bitwise operations on a byte from a global variable and updates offset 6 of the structure, and finally modifies offset 12 of the structure based on bits in a byte from a routine call.
- `allocate_and_initialize_structure`: Calls a routine to allocate memory, and if successful, stores the pointer in a global variable and initializes several fields at specific offsets within that memory block, otherwise it stores zero in the global variable and returns.
- `allocate_and_setup_linked_list`: Allocates memory for a structure, sets a field at offset 15 to negative one, and if a subsequent call returns a valid pointer, clears several fields, sets offset 8 to 4, and builds a linked list structure starting at offset 20 of the allocated block.
- `release_allocated_resources_2`: Checks a global variable and, if non-zero, clears it and calls a routine using a pointer from another global variable; then checks a second global variable and, if non-zero, clears it and calls a routine using a pointer from the first global variable.
- `verify_media_and_display_error_2`: Sets a flag and modifies a bitfield in a global variable, then calls a routine to display a message.
- `verify_media_and_display_error`: Attempts to verify disk information by calling a routine with a string and a length, then checks a library function for a specific result; if the result is non-zero, it calls a library routine to display an error message and branches to another location.
- `copy_disk_info_and_verify_media`: Copies a string from a global location to a destination buffer and then performs a sequence of checks to verify disk information.
- `write_hardware_registers`: Sets two registers to specific hardware addresses, and if d7 is non-zero, writes bytes from d6 and d5 to those addresses, calls another routine, and clears a third hardware address.
- `search_table_for_id`: Sets two global bytes to 1 and clears two other global bytes, then searches a table for a 16-bit value constructed from two registers.
- `update_pointer_and_trigger_routine`: Stores a word from d0 into a global pointer, sets a byte in a global variable to 255, and calls a routine.
- `update_state_and_call_library`: Stores a byte from a register into a memory location, compares a byte from that location with a value in a register, and calls another routine.
- `compare_byte_and_branch`: Compares a byte at a global variable to 254 and branches to one location if equal, otherwise it branches to another location.
- `call_indirect_with_registers`: Saves d0 and d1 into d7 and d4, calls a routine using an offset from a pointer in a6, restores d0 from d7, and branches to another routine.
- `check_dos_error_and_notify`: Checks if a value returned from a library call matches a specific constant, then if it does not, calls another routine to handle a DOS error, writes the value thirty-two to a byte, and calls another routine to pass a pointer.
- `increment_and_check_counter`: Increments a 16-bit counter in a register and compares it to 160; if it reaches 160, it calls another routine, performs a library call using a pointer from a global structure, increments a second counter, and then branches to another routine based on whether the second counter is zero or matches a specific value.
- `compare_and_reset_counters`: Compares d6 to 7 and d7 to zero; if d6 is not 7, it branches to another routine, otherwise if d7 is not zero, it branches to another routine, otherwise it sets d7 and d6 to 1 and branches to another routine.
- `set_global_byte_and_branch`: Writes the byte value 128 to a global variable and branches to another routine.
- `load_error_address`: Loads the address of a global string into a0 and branches to another location.
- `load_address_and_branch`: Loads a constant address into a register and performs a short branch to another routine.
- `handle_dos_error`: Calls a routine at an offset of negative 132 from a6, pushes d0 onto the stack, calls another routine, and then writes decimal 32 to a byte at an offset of 3 from a pointer in a3 before branching to another location.
- `dispatch_error_handler`: Loads two addresses into registers, pushes the current stack pointer to the stack, and calls a routine via an indirect jump.
- `branch_to_exit`: Loads an address into register a0 and branches to another routine.
- `modify_global_word`: Calls two routines, clears bit 8 of a global word, clears a byte at a global offset, and sets a byte at a global offset to 255.
- `write_string_bytes`: Writes the bytes 2e, 69, 6e, 66, 6f, and 00 to the memory location pointed to by a1.
- `execute_library_sequence_2`: Calls a sequence of four library routines using a6, performing conditional checks on the return values and returning either zero or negative one.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `convert_and_copy_string`: Iterates through a source string, converting lowercase characters to uppercase and copying them to a destination, then potentially copies a second string from a global address to the end of the first.
- `update_string_characters`: Calculates a character value by adding an offset to a byte in d6, stores it into two different memory locations, and calls two other routines.
- `wait_for_value_match`: Loops until a byte at a global address equals 64, then checks a word at offset 38 of a structure pointed to by a global variable against three specific values, returning negative one if none match or zero if a match is found.
- `return_negative_one_2`: Loads the value negative one into a data register and returns immediately.
- `call_indirect_handler`: Saves registers to the stack, loads a function pointer from a global table into a6, calls that function, and restores the registers before returning.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `process_table_entries`: Checks a 16-bit word in a global variable; if zero, it calls another routine, sets the variable to 2, sets a byte at offset 388 to 255, clears a long at offset 2168, and then enters a loop that fetches a word from a table using the value in the global variable as an index, updates a global word, and conditionally calls several routines and performs memory operations.
- `increment_global_pointer`: Adds 2 to a long value at a specific location and branches to another location.
- `clear_global_counters`: clears a 16-bit word at offset 2026 and a byte at offset 974.
- `allocate_memory_with_validation`: Checks if a global pointer is null and if a global word value is between 24 and 26 inclusive; if so, it calls a memory allocation routine with a size of 13 and a specific attribute, then updates a global byte with a character from a source and stores a modified value into a global byte array.
- `attempt_memory_allocation`: Clears a global byte and, if it is not zero, calls a library function to allocate a memory block of size 4813 with a specific attribute and, if successful, sets a byte at an offset in a global word to 255 before branching to another routine; otherwise, it clears the global byte and branches to another routine.
- `allocate_memory_block`: Checks a global byte, then attempts to allocate a memory block using two library calls with different parameters; if the allocation succeeds and the returned address is not a specific constant, it calls a third library function and clears a pointer in a global variable, otherwise it clears the pointer and returns.
- `perform_complex_initialization`: Performs a series of conditional checks and calls to various routines, modifies a byte in a global variable, and if a global word is greater than 26, calls another routine to write a value to a pointer; it then enters a loop that iterates through a list of words, comparing them against two calculated values, and finally updates a global byte and returns.
- `update_status_and_call_handlers`: Adds d1 and d2, and if the result is zero or five, performs bitwise operations on a word from a global variable and stores the result in a location pointed to by another global variable; otherwise, it calls two routines and returns.
- `disable_interrupts`: Clears d0, pushes it to the stack, checks a global longword, and if non-zero, calls a routine via a pointer in a global variable before clearing that global longword and branching.
- `set_interrupt_mask`: Calls another routine, writes the value 255 to a memory location, and returns.
- `write_data_table`: Writes a word to a table at a global address, then conditionally writes several words to the same table based on the value of a global word and a global byte, calling another routine during the process, and finally updates two global longwords.
- `process_word_data`: Loops through a source list, checking a byte at a global address, and writes two words per iteration to a destination pointed to by a0, where the second word is the first word with bit 0 inverted and bits 15 through 15 cleared.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `fetch_table_value`: Loads a word from a table at a global address using an offset derived from another global variable, then returns the value shifted right by 8 bits.
- `initialize_and_copy_data`: Sets several bytes and words in a structure, calls two different routines, and then copies 64 longwords from a source table to a destination starting at a global variable offset.
- `return_negative_one`: Loads the value negative one into d0 and returns.
- `update_timer_and_hardware_registers`: Checks a bit in the interrupt status register and a global flag, then increments a counter and updates a longword in the DMA controller register.
- `restore_interrupt_context`: Writes values to two memory locations, stores a byte from d0 into a global variable, and restores d0 through d7 and a0 through a6 from the stack before returning from an interrupt.
- `process_joystick_input`: Performs bitwise operations on the contents of the custom chip status register and the joystick data register to calculate a value, which is then combined with a bit from the CIAA port and stored in a memory location.
- `update_status_byte`: Compares d1 to a global byte; if they are equal, it returns.
- `clear_status_byte`: Clears a byte in a global variable and returns.
- `unpack_variable_length_bitstream`: Reads a bitstream from a source pointed to by a0, decoding bit sequences of varying lengths into bytes and storing them into a destination buffer pointed to by a2 until the buffer reaches the limit in a1.
- `execute_initialization_sequence_2`: Calls several routines, clears a longword in a global variable, and jumps to another routine.
- `initialize_hardware_environment`: Clears a block of memory, clears two global bytes, calls a routine, writes a longword from a global variable to a hardware register, writes zero to a hardware register, writes three function pointers to specific memory locations, writes two words to hardware registers, and jumps to another routine.
- `process_event_queue_and_update_state`: Checks a global byte and a counter; if conditions are met, it decrements the counter and calls a command handler.
- `update_object_state_and_dispatch`: Checks a global byte and returns if zero; otherwise, it calls two routines, pushes the current program counter onto the stack, writes the value 64 to a global word, retrieves a pointer from a structure offset by 8192, pushes a longword from that structure onto the stack, and jumps to a routine via a long pointer.
- `loop_and_update_state`: Calls copy_pointers in a loop until a register is zero, then calls dispatch_command_handler, adds thirty to a global variable, calls decrement_table_values, clears a byte in a global variable, and returns.
- `dispatch_conditional_routine`: Tests a word at the address in a2, and if it is non-zero, tests two global bytes; if both are zero, it calls a routine and then branches to another location.
- `dispatch_event_via_table`: Checks a global byte for a specific value, calls another routine, and then uses a bitmask derived from a word in a2 to select and call a function from a jump table located via the program counter.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_update_structure_word`: Loads a word from a structure in a2 into d0 and checks if it equals zero or 32976; if it matches and a global variable at offset 2158 equals 22, it writes 4128 to offset 264 of the structure in a2, otherwise it returns.
- `update_structure_values`: Compares d0 against several word constants and, if matches occur, clears long values at offsets 2 and 6 of a2, performs a swap on a value in d1, and writes it to offset 2 of a2.
- `update_threshold_value`: Compares a word at a global address to 24 and compares d0 to 4128, then writes 4192 to the location in a2 and increments a2 by 5 if both conditions are met.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `load_and_test_pointer`: Calls another routine, moves a longword from the address in a3 to d1, fetches the next longword from that address into d2, tests bit 29 of d2, and calls another routine if that bit is set.
- `validate_and_calculate_pointer`: Calculates a new address by adding a masked 32-bit value in d2 to a value in d1, then checks several global bytes and compares the result against a global long value before calling another routine.
- `check_and_unpack_data`: Tests bit 31 of a byte, then if zero, performs an OR operation on a byte from a long pointer into d0 and calls another routine.
- `expand_compressed_data`: Checks two conditions, then calls a routine to unpack a bitstream and uses the result to determine how many words to copy from a source to a destination.
- `copy_and_increment_data`: If two global bytes are not negative one, it copies 10 words from a source table to a destination table and then copies 4 words to another location; it then increments a global variable and potentially adds a value from a structure to another global variable based on a comparison.
- `check_pointer_and_set_byte`: Checks if a longword in d1 is non-zero and if a longword at a global address is non-zero; if both are true, it sets a byte at a global address to 1 and otherwise returns.
- `process_conditional_updates`: Ors three bytes from a structure into d0, and if d0 is zero and a global byte is non-zero, it calls one routine, clears that global byte, and calls another routine; otherwise, if a global byte is non-zero, it clears it and calls a routine; finally, it sets a global byte to 1 and adds 7 to a global longword.
- `process_byte_and_call`: Saves registers to the stack, writes a byte to a memory location, calls another routine, and if a byte from a pointer in a3 is not 64, calls a fourth routine before returning.
- `compare_and_call_routine`: Compares a byte in d0 to 3 and, if it is not equal, branches to another location; otherwise it calls a routine and then branches to another location.
- `compare_and_branch_byte`: Compares a byte in d0 to 160 and, if not equal, branches to another location; otherwise it calls another routine, calls another routine, and branches to another location.
- `compare_byte_and_branch_2`: Compares a byte in d0 to 176 and, if not equal, branches to another location; otherwise it calls two different routines and branches to a third location.
- `check_and_copy_data`: Compares a byte in d0 to 2 and, if not equal, calls another routine; otherwise it calls another routine, sets d1 to 0, and branches to a different location.
- `clear_memory_block`: Clears bit 5 of a byte at an offset from a pointer in a3 and then fills a memory block starting at a pointer from a global variable with zeros using a loop.
- `clear_d7_and_branch_short`: Clears register d7 and performs a short branch to another routine.
- `execute_command_and_update_state`: Fetches a command byte from a pointer in a3, and if it matches a specific value, calls a routine at the address following that byte.
- `compare_and_branch`: Calls a routine, loads a byte from a location relative to a3, and if that byte is not 130, calls another routine before branching.
- `check_byte_and_call_routine`: Compares a byte in d0 to 160 and, if it is not equal, branches to another location; otherwise it calls a routine and then branches to another location.
- `compare_and_call_byte`: Compares a byte in d0 to 176 and, if not equal, branches to another location; otherwise it calls another routine and branches to another location.
- `compare_byte_and_call`: Compares a byte in d0 to 129 and, if not equal, branches to another location; otherwise it calls another routine and branches to a third location.
- `check_and_copy_sprite_data`: Compares a byte in d0 to 131 and, if not equal, calls another routine; otherwise it calls another routine, loads a value from a stack pointer into d1, and branches to a different location.
- `copy_compressed_bytes`: Uses a nested loop to copy bytes from a table to a destination address, where the source index and destination pointer are updated based on values read from a stream in a3.
- `transfer_byte_with_offset_loop`: Moves a byte from a table at offset d0 in a4 to the address in a1, then updates a1 using d3 or d7 and iterates through two nested loops controlled by d6 and d4.
- `copy_bytes_with_offset`: Iterates through a loop to copy bytes from a table to a destination, using a value from a3 to determine the source index and adding a constant to the destination pointer.
- `transfer_byte_with_signed_offset_loop`: Moves a byte from a table at offset d0 in a4 to the address in a1, then updates a1 by adding 44 or subtracting d3 and iterates through two nested loops controlled by d6 and d4.
- `copy_bytes_to_multiple_destinations`: Copies bytes from a source in a3 to three different destinations, including two tables and a third memory area, with the number of bytes copied determined by a global variable.
- `expand_structured_data`: Calls another routine and then uses a loop to write multiple bytes to a destination address, where the destination and source indices are derived from a stream in a3 and a table.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `expand_data_with_offset`: Calls another routine and then uses a loop to write bytes to a destination address, where the destination pointer is incremented by 44 after each iteration of the loop.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `update_sprite_attribute_data`: Reads a series of indices from a data stream to fetch sprite attribute values from a table, then performs bitwise operations to merge these values with a mask and writes the results to specific offsets within a sprite structure.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `expand_compressed_data_stream`: Reads a stream of bytes from a source pointer and expands them into a destination buffer by using bytes as indices into a table or as length/offset modifiers.
- `increment_and_branch_pointer`: Adds a word from d0 to d0, adds the result as an offset to a table pointed to by a2 to update a1, then branches to another routine.
- `copy_bytes_from_table`: Loads a byte from a table using an index retrieved from a pointer, stores that byte into a destination address, increments the destination address by a 16-bit value, and returns.
- `populate_sprite_attributes_from_table`: Iterates through a data stream to fetch attribute bytes from lookup tables and writes them to specific offsets within a sprite data structure.
- `loop_and_populate_attributes`: Decrements d5 and d7 in a loop, adding a word from d0 to d0 and updating a1 using a table at a2, otherwise it returns.
- `populate_sprite_data_from_table`: Reads a series of byte indices from a source table to fetch sprite attribute bytes from a lookup table and writes them to specific offsets within a destination structure.
- `select_pointer_based_on_test`: Tests a byte at a global address and, if non-zero, loads a pointer from one global variable into a1, otherwise it loads a pointer from another global variable into a1.
- `select_pointer_by_bit_test`: Calls another routine, tests bit 3 of a byte at a negative offset from a3, and if that bit is zero, loads a pointer from one global variable into a1; otherwise, it tests a byte at another global variable and, if that byte is zero, loads a pointer from a second global variable into a1 before returning.
- `update_structure_from_globals`: Tests a byte at a global variable, clears it if non-zero, then loads two longwords from global variables, calls two different routines, and performs a final update to a structure field using a value derived from a global variable.
- `select_and_compare_pointers`: Calls two different routines, loads a longword from a global variable into d0, and if a byte at a global variable is non-zero, loads a different longword into d0; it then compares d0 to a value in d1 and calls another routine if they are not equal.
- `update_structure_fields`: Waits for a bit to clear in a structure, then updates several fields in that structure using values from d0 and d1, and calculates a new value for a field at offset 58 based on a global variable and a constant.
- `increment_and_clamp_global_counter`: Checks a global variable and a global word; if certain conditions are met, it increments a global byte and clamps its value between 1 and 5.
- `process_structure_data`: Decrements d2 and, if not zero, checks if a word at the pointer in a0 is non-zero and if a specific bitmask matches a value in a structure; it then potentially loads a byte from a table and performs bitwise operations.
- `compare_and_branch_pointer`: Compares a word in d0 against 32800, and if it does not match, loads a pointer from offset 6 of the structure in a0 into a0, adds 33570 to a0, and branches to another routine.
- `compare_and_load_address`: Compares d0 against three specific word constants and, if none match, loads a pointer from a0 into a0 and branches to another routine.
- `populate_structure_via_loop`: Calculates a pointer and an offset, then enters a loop that runs two or three times depending on a global byte; in each iteration, it writes the pointer and offset to specific offsets of a structure pointed to by a6, updates them with constant increments, and writes several 16-bit values to other offsets within the same structure.
- `update_counters_and_call_routines`: Checks two global variables, decrements one and increments another in a loop, and if certain conditions are met, calls four different routines using pointers from four global variables.
- `decrement_global_word_2`: Tests bytes at offsets a03 and a06 of a structure and a byte at offset 7de of another, branches if any are zero, subtracts 2 from a word at offset a04, and if the result is positive, sets that word to 42; otherwise it sets it to 42 and branches.
- `increment_and_call_tables`: Increments a word in a global variable, and if it reaches 44, resets it to zero, then calls two different routines using pointers from a0 and a6.
- `initialize_and_transform_tables`: Clears a memory block using a loop, calls two different routines to process data in a loop, and then performs a nested loop to transform bitstream data from one table into another.
- `process_bitstream_data`: Performs a nested loop that reads bytes from a source, performs bitwise shifts and rotations, and writes the results to a destination table.
- `copy_data_block`: Performs a nested loop to copy words from a source table to a destination table using specific stride increments.
- `load_and_store_pointers`: Calls another routine, calls another routine, stores the value in a3 into a global variable at offset 344, adds d1 to d0, and stores the result in a global variable at offset 428.
- `clear_and_initialize_memory`: Clears several specific bytes and longwords in global variables, runs a loop that clears 32 words starting at a global variable, calls three different routines, and finally loads a pointer from a global variable into a0.
- `zero_memory_buffer`: Clears a large block of memory by zeroing out 669 longwords using a loop.
- `load_pointer_and_return`: Calls another routine, moves a long value from the address in d0 into a3, then returns.
- `load_pointer_and_branch`: Loads a pointer from a global variable into a0, moves a long value from an offset of a0 into d1, moves the next long value from a0 into d0, and branches to a new location if d0 is non-zero.
- `store_pointer_to_global`: Loads a pointer from an offset in a3 into a global variable and then returns.
- `traverse_and_update_pointer`: Iterates through a list of longwords, adding a value from an offset in a structure to the current element and comparing it to a value in a3, then updates a global variable with the next pointer if the condition is met.
- `dispatch_command_id_2`: Compares a word in d0 against several constants and branches to specific subroutines.
- `store_and_increment_pointer`: Stores a longword from a2 plus an offset of 2 into a location, calls another routine, pushes d0 onto the stack, fetches a byte from a3 and increments it, calls another routine, restores a3 from the stack, increments a global variable, and returns.
- `invert_byte_at_address`: Reads a byte from a global address, performs a bitwise NOT operation on it, and writes the result back to the same address.
- `store_pointer_at_offset`: Stores the value of a3 into a memory location at offset 2 from the pointer in a2.
- `dispatch_command_handler`: Jumps to another routine via a long pointer.
- `update_dispatch_pointer`: Stores the pointer held in a3 into a memory location at offset 2 from the address in a2, then jumps to another routine.
- `dereference_and_jump_to_pointer`: Copies a longword from an offset of 2 bytes within a structure pointed to by a2 into a3, then performs an absolute long jump to the address stored in a3.
- `update_and_increment_table`: Calls another routine, clears 16 words in a table starting at a global variable using a loop, then iterates through a loop to compare words from two tables and updates the first table with incremented values based on bitwise differences, finally adds a value to a global variable and branches to another routine.
- `decrement_table_values`: Calls two different routines, then enters a loop that iterates 16 times, decrementing values in a table based on their bit patterns in the low, middle, and high bytes, and finally adds a value to a global variable.
- `test_and_call_handler`: Tests a byte at a global variable address and, if it is not zero, calls another routine; otherwise it branches to a different location.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_boolean_flag`: Reads a byte from a global variable, performs an exclusive OR operation with one, and writes the result back to the same location.
- `dispatch_command_id`: Compares a 16-bit value in d0 against several constants and branches to specific subroutines or performs data manipulation based on the match.
- `jump_with_calculated_address`: Loads a word from an offset of 2 of a structure in a2, and if it is not zero, calculates a new address by adding 8194 to a value from a global pointer and jumps to that address; otherwise it jumps to a different address.
- `clear_three_globals`: Clears three 32-bit global variables, then returns.
- `setup_structure_fields`: Calls another routine, clears a byte at a global variable, adds 10 to a longword from a global variable, calls another routine, and then updates several fields in a structure based on a word from a pointer in a2.
- `reset_structure_and_globals`: Writes a word to a global variable, checks two bytes in other globals, and if both are zero, writes several words to offsets of a structure in a6 and clears three 32-bit global variables before copying a longword from a table to another global variable.
- `copy_pointers`: Copies a longword from one global variable to a location pointed to by a third global variable, then copies a longword from another global variable into a fourth global variable.
- `initialize_structure_and_globals`: Writes several words and longwords to a structure in a6, clears several bytes and words in various globals, copies a longword from a global variable to another global variable, and clears three 32-bit global variables.
- `set_structure_and_byte`: Writes three words to a structure in a6, writes two longwords and a word to other offsets in a6, and clears a byte in a global variable.
- `update_split_word_values`: Calls another routine, clears a 16-bit word in a global variable, and then extracts the low and high words from a 32-bit value at offset 2 of the structure in a2 to store them into two consecutive 16-bit global variables.
- `process_buffer_and_state`: Clears a longword in a global variable, performs several range checks on d0 against various memory locations, and enters a loop that calls two different routines while toggling a global byte.
- `set_byte_to_all_ones`: Calls another routine and then stores the value 255 into a global variable.
- `clear_byte_value`: Calls another routine and then stores 0 into a global variable.
- `update_structure_and_byte`: Copies two longwords from a global variable into a structure in a6, writes several words to that structure, and sets a byte in a global variable to two.
- `handle_interrupt_request`: Saves registers, sets a6 to the base of the hardware registers, masks bits 11 and 12 of the interrupt request register, calls another routine, writes the masked value back to offset 158 of the hardware registers, restores registers, and performs an interrupt return.
- `update_state_and_structure`: Increments a byte in a global variable if bit 10 of d2 is set and the byte is between 2 and 4, calls another routine, and if bit 7 of d2 is set, performs arithmetic on a longword in a global variable and updates a structure in a6 with values from other globals.
- `decrement_global_byte`: Decrements a byte in a global variable and calls another routine if the resulting value is zero.
- `increment_and_toggle_byte`: Adds 8192 to a longword in a global variable, toggles the least significant bit of another global variable, and sets a byte in a third global variable to one.
- `decrement_global_word`: If a word at a global address is non-zero and its counterpart at another global address is non-zero, it decrements the word at the second global address and returns.
- `copy_and_test_sum`: Copies a word from one global address to another, adds a word from a third global address to d0, and calls another routine if the result is negative.
- `limit_and_set_volume`: Clamps a value in d0 to a maximum of 64, clears a word in a global variable, writes the value to another global variable, and if two specific global bytes are zero, writes the value to the AUD0VOL and AUD1VOL hardware registers.
- `copy_pointer_to_address`: Copies a 32-bit value from a global variable to the memory location pointed to by the value at offset 33800 in a1.
- `clear_status_bytes`: Clears two bytes at offset 868, loads a pointer from offset 872 into a2, clears a long value at offset 850, and returns.
- `dispatch_event_handler_by_id`: Compares a value in register d0 against several constants and branches to specific subroutines based on the match.
- `populate_graphics_parameter_tables`: Iterates through several loops to populate a series of structures with pointer pairs and constant values, using a loop counter to perform two iterations per block.
- `call_handler_and_clear_byte`: Calls another routine and then clears a byte in a global variable.
- `call_handler_and_set_byte`: Calls another routine and then sets a byte in a global variable to 255.
- `call_handler_and_clear_byte_alt`: Calls another routine and then clears a byte in a different global variable.
- `clear_memory_and_set_flags`: If two global variables are non-zero, it clears a block of memory starting at an offset of 31648 from a base address in a global variable using a loop of 2671 iterations, calls another routine, clears a byte in a global variable, sets a byte in a global variable to 255, sets another global variable to 0, and clears a final global byte before returning.
- `compare_and_branch_pointer_2`: Copies a pointer from a register to another register, checks if the word at the address pointed to by that register plus 2 is equal to decimal 32816, and branches based on the result.
- `modify_and_jump_pointer`: Tests d0 and branches if zero, calls another routine, increments d1, multiplies d1 by 10, adds the result to a2, and jumps to another routine.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `execute_system_initialization`: Checks a pointer, if zero it performs several operations including writing a word and its swapped version to a structure, clears a block of memory using a loop, calls a routine, reads a hardware register and masks it, calls three different routines, and finally enters a loop that branches back to an earlier instruction.
- `process_input_and_calculate_index`: Iterates through a null-terminated string in a pointer, calls another routine, and then enters a loop that checks a byte from a global variable against bits 6 and 7 of the CIAA Port A register.
- `initialize_and_test_bit`: Clears registers d5, d6, and d7, calls another routine, and if d1 is not less than 32, sets bit 7 of d7 and branches based on the result.
- `loop_and_copy_byte`: Sets bit 6 of d1, loops 18 times to set bit 6 of d1 and increment d5, then performs a loop that compares a byte from a table to d4 and branches.
- `xor_table_data`: Performs a loop 18 times where it calculates offsets into two tables based on a byte from a third table, then XORs a longword from the first table with a longword from a second table and stores the result in the second table.
- `initialize_memory_and_dma`: Clears a block of memory using a loop, copies 127 longwords from one location to another, calls another routine, and writes values to several offsets in a structure.
- `update_pseudo_random_seed`: Uses the vertical blank pointer and a global variable to perform a series of bitwise operations and shifts to update a seed value, then performs a 16-bit multiplication.
- `check_byte_match`: Masks d0, loads a pointer from a relative address, and if a byte at that pointer matches a value at a global address, it branches to another routine.
- `check_and_clear_byte`: Loads a byte from offset 868 into d0; if the byte is not zero, it updates a2 using a pointer at offset 2 of the structure in a2 and a pointer at offset 872, subtracts 10 from a2, clears the byte at offset 868, and returns.
- `add_offset_to_global_2`: Adds a long value from offset 2 of the structure in a2 to a long value at offset 7e6 and returns.
- `add_offset_to_global`: Adds a 32-bit value from an offset of 2 in a2 to a global variable.

### Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:lair2

- `unpack_bitstream_to_color_registers`: Reads a bitstream from a pointer, processes bits through a loop to extract values, and writes the resulting bytes to a destination buffer while updating the hardware color registers.
- `shift_and_store_byte`: Loads a longword from a pointer in a0, extracts the lowest byte, stores it in a 16-bit word at a location in a5, and returns the longword after a logical shift right and setting the highest bit.
- `perform_bit_shift_loop`: Enters a loop that performs a logical shift right on a longword in d0, calling a routine when d0 reaches zero, and uses a loop counter in d1.

### Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_address`: Performs a short branch to a specific address.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `configure_blitter_parameters`: Stores a value in a global variable and then writes various configuration values to blitter registers, polling a hardware bit between writes.
- `set_fire_button_and_interrupts`: Sets bit 1 of a CIAA register, stores two words from hardware registers into global variables, and writes a value to the INTENA register.
- `initialize_data_tables`: Copies a block of data from a source to a destination, then populates several tables with interleaved word values and longwords.
- `write_split_long_values`: Loads a long value from a global variable, adds 124 to it, and then writes the high and low words of several derived long values into a table starting at an offset of 26 from the base address in a1.
- `update_global_pointer`: Moves a longword from a global variable to another global variable and then sets the first global variable to a constant value.
- `configure_hardware_registers`: Writes to several hardware registers including custom chip registers and interrupt registers, and performs a loop that waits for a bit in a hardware register to be set before writing to other registers.
- `initialize_and_call_graphics_routines`: Calls a routine using a pointer from a global variable, stores a calculated address into a structure, and then performs a sequence of calls to another routine using a series of constant values in d1, before storing a final calculated address into a global variable and returning.
- `search_and_call_handler`: Iterates through a buffer in a0 until it finds a byte with the value 255, then calls a routine via a pointer at a specific offset in a6 using the address following the marker.
- `update_table_and_toggle_flag`: Calculates a series of 16-bit values using multiplication and addition, writes them into a table, and then toggles a bit in a global variable to update a value at a specific memory location.
- `calculate_sprite_transformations`: Processes a list of words from a structure, performing arithmetic on values and updating a coordinate list, then modifies hardware registers and calls a routine to handle sprite data.
- `process_coordinate_list`: Loops through a list of words until it encounters the value 17545, performing complex fixed-point arithmetic using values from several tables to calculate and store new coordinates into a destination buffer.
- `calculate_sprite_transform_data`: Reads two pairs of 16-bit values from a table using offsets in d6 and d7, performs arithmetic to determine a specific constant based on their differences, and writes the resulting transformed coordinates, scaling factors, and calculated offsets into a structure pointed to by a0.
- `set_blt_registers`: Adds two values from a global variable and a constant, stores the result in the bitplane control register, and sets the bitplane width, height, and mode registers.
- `update_display_parameters`: Increments a global counter and, if it reaches eight, parses a data stream to update several pointers and structure members, then updates hardware registers with specific values.
- `render_sprite_data`: Calculates source and destination addresses based on global pointers, then enters a loop to copy data and perform bitwise rotations while updating hardware registers.
- `configure_hardware_structure`: Checks a bit in a register and, if not set, writes a constant to a structure offset, sets a hardware register value, and stores a calculated address into another structure offset.
- `update_animation_frame`: Clamps a value from a structure to a specific range, updates a global byte, and manages a counter that modifies a value at an offset within a table of structures.
- `check_joystick_status`: Tests bit 6 of the CIAA Port A Data register and bit 2 of a byte at offset 22 of a structure in a0, then performs an indirect jump using a value from a table.
- `branch_to_offset_2`: Performs a short branch to a specific offset in another routine.
- `branch_to_offset_3`: Performs a short relative branch to a specific offset within the code segment.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `branch_to_offset_5`: Performs a short branch to a relative offset of 55784.
- `branch_to_address_alt`: Performs a short branch to a different specific address.
- `branch_to_event_trigger`: Performs a short branch to a location within another routine.
- `update_memory_and_return`: Branches to an offset, otherwise it loads a pointer from a relative address, checks a byte in register d1, and either branches to another offset or clears a byte at an offset of a pointer in a3 and writes a word to a specific memory location before returning.
- `decrement_structure_byte`: Loads a pointer from a table into a3, then if d1 is not zero, checks if the byte at offset 1 of the structure in a3 is non-zero; if it is, it decrements the byte at offset 0 of the structure in a3.
- `update_conditional_state`: Checks a byte in d2 and a byte at offset 1 of a3; if certain conditions are met, it increments a byte at offset 1 of a3, updates a structure in a1 with several values, and modifies a global variable and a value at offset 6 of a structure.
- `update_structure_fields_3`: Calculates values based on register d2 and d7, then updates multiple fields in a structure pointed to by a0, including several pointer assignments and word values, before branching to another routine.
- `set_global_constant`: Writes a constant to a global variable and clears a byte at an offset from a register.
- `decrement_global_counter`: Decrements a global variable and, if it reaches zero, clears a byte at an offset from a register and resets the global variable to five.
- `process_event_trigger`: Checks a byte in a register and, if it is zero, performs a series of pointer calculations and hardware register updates within a loop.
- `prepare_state_update_a`: Loads an address into a3 and, if d1 is not zero, loads a value into d4 and branches to another routine; otherwise, it branches to another routine.
- `clear_byte_and_set_word`: Clears a byte at offset 1 of the address in a3 and writes the value 15 to a global word.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `prepare_state_update_b`: Loads an address into a3 and, if d1 is not zero, loads a value into d4 and branches to another routine; otherwise, it branches to another routine.
- `clear_byte_and_set_word_alt`: Clears a byte at offset 1 of the address in a3 and writes the value 15 to a global word.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `conditionally_load_constant_and_branch`: Loads a pointer into a register from a relative offset, then checks a byte in a register; if the byte is zero, it branches to another routine, otherwise it loads a 16-bit constant into a register and branches to another routine.
- `update_state_and_structure_2`: Clears a byte at offset 1 of a3 and sets a global word to 15; if d1 is not zero, it increments a global word and returns; otherwise, it performs complex pointer arithmetic to update a structure at a0 using values from a1 and a2, then decrements a global word and returns if it is not zero.
- `decrement_and_return`: Checks a byte at a specific memory location; if it is non-zero, it branches to another offset, otherwise it pushes a0 to the stack, retrieves a pointer from the stack, decrements a word at an offset of that pointer, and either branches to another offset or restores a0 from the stack and returns.
- `configure_hardware_and_call`: Clears bit 1 of a hardware register, copies a block of bytes to offset 768, writes values to a hardware register, calls two other routines, and returns.
- `update_hardware_registers`: Pushes a pointer to the stack, modifies bitplane control registers via the hardware custom chip base address, calls eight different routines and one indirect routine, modifies bitplane control registers again, and then compares a hardware register value to a global variable before returning from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_and_sprites`: Increments a global byte, then performs several conditional branches to either call a routine to update audio registers using a structure in a6 or execute a loop that searches a table to update a word in a register.
- `update_sprite_attributes_and_audio`: Updates a sprite's position, graphics pointer, and attributes by reading from a table based on a nibble-encoded index, then modifies hardware audio registers and DMA settings.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_object_position_and_state`: Processes a control structure in a6 to update a 16-bit position value based on a command byte, performing addition, subtraction, or clamping to specific bounds, and modifies a hardware register to toggle a bit in the joystick fire status.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_4`: Returns the constant -1 in d0 to the caller without any other effect.

### Dragon's Lair - Escape from Singe's Castle (1989)(ReadySoft)(Disk 1 of 5)[cr CLS][t +1 CLS].adf:lair2#unpacked

- `branch_to_address_2`: Performs a short relative branch to another location.
- `call_old_open_library_2`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `configure_blitter_parameters_2`: Stores a value in a global longword, waits for a bit in a custom register to clear, and then sequentially updates blitter registers and custom DMA registers while waiting for specific bits to clear between writes.
- `enable_interrupt_and_store_registers`: Sets bit 1 of the CIAA PRA register, stores two words from custom register offsets into global longwords, and writes 30847 to the custom INTENA register.
- `build_lookup_table`: Copies a block of data from a source to a destination, then populates a large table with a series of calculated 32-bit values and 16-bit constants.
- `write_split_longwords`: Calculates several values based on a global longword and writes them as a series of 16-bit words into a table starting at an offset of 26 from a pointer.
- `update_pointer_table_2`: Copies a longword from a global variable to a specific location and then updates that same location with a new constant value.
- `configure_cop_registers`: Writes to several custom chip registers and calls multiple other routines in sequence.
- `initialize_system_parameters`: Calls three library functions, updates a global pointer with a value from a table plus an offset, and performs several sequential calls to another routine with varying arguments.
- `find_end_and_call`: Iterates through a buffer pointed to by a register until it finds a byte with the value negative one, then calls a function pointer located at an offset of negative 60 from a register.
- `update_table_and_flag`: Performs arithmetic on a value from a global variable to update a table of 16-bit words, then toggles a bit in a global variable and updates a third table based on the result.
- `process_coordinate_data`: Manipulates a table of values by adding or subtracting elements, performs a loop to adjust values by a constant, and updates several hardware registers and global variables.
- `process_coordinate_list_2`: Loops through a list of 16-bit words until a specific marker is found, performing complex fixed-point multiplication and division using values from several tables to write new 16-bit coordinates to a destination table.
- `calculate_bounding_box_and_update`: Reads two pairs of coordinates from a table, calculates their differences and absolute values, and performs arithmetic to determine a specific value and a flag.
- `configure_blt_hardware`: Calculates a value by adding two constants, stores it in the BltDMA depth register, and then sets the BltDMA control, mode, and size registers with specific values before returning.
- `update_structure_and_blt`: Increments a global word and, if it reaches eight, resets it and performs a series of pointer updates and data movements involving a table and a structure in a0.
- `perform_blt_loop`: Updates a global pointer, checks a value in a structure in a0, and enters a loop that performs multiple BltDMA register writes and pointer increments based on values from a table and a structure.
- `setup_blt_parameters`: Calculates an address using a global pointer and two constants, waits for a bit in a structure in a0 to clear, and then writes values to the structure in a0 before returning.
- `update_index_and_table`: Calculates a new value based on a byte from a structure in a0 and a global variable, updates a global byte, and if a calculated index differs from a global word, updates a table and a structure field.
- `dispatch_input_and_blt`: Checks the status of CIAA_PRA and a bit in a structure in a0, then performs an indirect jump to a routine at an offset determined by a global word.
- `decrement_global_counter_2`: Checks a byte at a specific address and, if it is non-zero, decrements a word in a location pointed to by a value on the stack.
- `update_hardware_and_dispatch`: Clears a bit in a global variable, performs several sequential calls to other routines, and updates hardware registers based on values read from the custom chip.
