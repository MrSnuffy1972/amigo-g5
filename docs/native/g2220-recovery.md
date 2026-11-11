# Game 2220 (empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02220). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909856 top-level bytes in 130 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 59 of 59 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 2220 of 2220 named (0 author symbols), 2220 of 2220 described (308 by rule, 1912 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3939 of 3939 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 42724 of 42726 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 300784 |
| code | 137136 |
| data | 426561 |
| padding | 25919 |
| unused | 19456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| More | 20 | 1225 | 68 |
| Stack | 2 | 117 | 2 |
| Else | 2 | 117 | 2 |
| Delete | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| Path | 2 | 117 | 2 |
| EndCLI | 2 | 117 | 2 |
| Dir | 2 | 117 | 2 |
| NewCLI | 2 | 117 | 2 |
| Echo | 2 | 117 | 2 |
| Lab | 1 | 2 | 1 |
| Skip | 2 | 117 | 2 |
| Execute | 2 | 117 | 2 |
| makedir | 2 | 117 | 2 |
| CD | 2 | 117 | 2 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| LoadWB | 12 | 475 | 30 |
| Wait | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| serial.device | 4 | 976 | 83 |
| dk | 1 | 3 | 1 |
| is | 1 | 3 | 1 |
| d | 1 | 3 | 1 |
| e | 1 | 3 | 1 |
| f | 1 | 3 | 1 |
| cdn | 1 | 3 | 1 |
| usa0 | 1 | 3 | 1 |
| i | 1 | 3 | 1 |
| usa1 | 1 | 3 | 1 |
| usa2 | 1 | 3 | 1 |
| n | 1 | 3 | 1 |
| s | 1 | 3 | 1 |
| gb | 1 | 3 | 1 |
| ch1 | 1 | 3 | 1 |
| ch2 | 1 | 3 | 1 |
| parallel.device | 3 | 322 | 26 |
| clipboard.device | 10 | 1433 | 128 |
| printer.device | 19 | 3272 | 217 |
| generic | 7 | 11 | 4 |
| Epson | 11 | 351 | 28 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf:Preferences | 45 | 6587 | 222 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf:Empire | 33 | 19031 | 1003 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf:CLI | 9 | 362 | 18 |
| icon.library | 15 | 1414 | 108 |
| info.library | 25 | 2476 | 170 |
| version.library | 2 | 41 | 7 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1754 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 323 | 8 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf bytes 103936-104448 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf bytes 104960-105472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf bytes 536576-537088 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: file devs/narrator.device (header 23280 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1110 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 120 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: bytes 103936-104448 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: bytes 104960-105472 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: bytes 536576-537088 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: 3 logged loads cover 1536 bytes of the image
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: blocks 143-143 (bytes 73216-73728) outside the filesystem hold data, not code (opcode density 0.035, entropy 4.06)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr vf]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf: blocks 154-197 (bytes 78848-101376) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.17)
- 3 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `process_data_and_tables`: This routine calls a routine using a pointer from a2, performs arithmetic on a value in d1, accesses a table using a scaled index, and contains multiple loops that copy data between memory locations and call other routines.
- `move_and_branch`: It moves the value in d1 to d0 and then performs a short branch to another location.

### More

- `process_string_data`: Calls a library function, then parses a string by checking characters against specific values and copying them to a new buffer, before calling several other routines and returning.
- `handle_library_error`: Checks a value in a table, calls a routine via a pointer in a structure, and if certain conditions are met, calls several other routines including one that takes a pointer from a global variable and another that takes a pointer from a structure in a6.
- `execute_dual_calls`: Loads an address from an offset of a4 into a0 and calls a routine at an offset of a6, then repeats this process with the same address.
- `call_dos_library_function`: It loads an address into a1, calls a library function at an offset from a6, stores the result in a global variable at offset 8, and returns if the result is not zero.
- `update_pointer_table`: Loads a 32-bit value from an offset of a7 into a0, stores a0 at its own address, increments the value at that address by 4, clears a byte at an offset of 4 from a0, and stores a0 at an offset of 8 from a0.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_seek`: Library stub: loads its arguments and the dos library base, calls Seek and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_delay`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `execute_complex_task_sequence`: Performs a large sequence of operations including multiple library calls, stack manipulations, and conditional branches based on the results of various subroutines.
- `validate_byte_range`: Calls two different routines, then checks a value at an offset of a table; if the value is within specific ranges, it returns 1, otherwise it returns 0.
- `execute_and_compare_result`: Stores 1 in a location pointed to by d4, calls a routine, stores the returned byte at an offset of a6, extends the result to a longword, and then performs a comparison and a jump based on a table lookup.
- `set_bit_and_branch_3`: Sets bits in d0 using a constant and branches to another location.
- `execute_task_sequence_2`: It calls another routine, pushes a value from a pointer on the stack onto the stack, calls another routine, pushes a constant 50 onto the stack, and calls another routine before returning.
- `update_status_and_call`: It stores a constant in a memory location, calls a routine with a pointer from offset 12 of a structure, calls another routine with a pointer from d3, and then calls a subroutine if certain conditions from a structure in a0 are met.
- `call_subroutine_with_stack_cleanup`: It pushes a pointer to a value at offset 44 onto the stack, pushes a value from offset 20 of a structure, and calls a routine before cleaning the stack and returning.
- `process_file_data`: Calls two routines, then enters a loop that increments pointers and compares bytes, eventually calling another routine and writing three specific bytes to a memory location.
- `loop_task_sequence`: It initializes a counter on the stack, calls a routine with a pointer in d3, and enters a loop that compares two words from a structure in a0, calls another routine with a pointer from a table, and decrements the counter.
- `execute_task_sequence`: Pushes a constant onto the stack, calls a routine, increments the stack pointer, calls another routine, and then branches to another location.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `dispatch_task_via_stack`: It pushes a value from offset 8 of the stack into a0, pushes a constant onto the stack, and calls a routine using the address in a0.
- `dispatch_task_pointer`: Loads a pointer from an offset of 8 from the stack into a0, increments a 16-bit value from offset 2c of that pointer, and calls a routine using that incremented value as a pointer.
- `execute_multiple_tasks`: Calls several routines using values from the stack and global pointers, including one that uses a value from a structure at offset 44 of a pointer in a0.
- `execute_system_tasks`: Calls a routine with a value of 65536 passed on the stack, and if the result is non-zero, performs a series of conditional calls to other routines using values stored in the stack and a structure in a6, finally returning the value from a register.
- `compare_and_update_byte`: Calls another routine, stores the result in a byte at a pointer from a6, and if the result is not 10, it either increments a value at a pointer from a6 and a local variable or clears a byte at a pointer from a6 before returning.
- `verify_and_store_result`: Calls a routine twice using values from the stack, stores the results in a structure at offset -4 and offset -8, and compares the results.
- `increment_and_verify_pointer`: It calculates a new address by adding offsets from a6 to a pointer, compares it to a value at offset -8 of a6, and if they are not equal, calls another routine with three arguments from the stack.
- `increment_and_branch`: Increments a 32-bit value at an offset of 8 from a6 and branches to another routine.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `increment_pointer_and_value`: Increments a 32-bit value at an offset of 8 from a6, and if a byte at a pointer from a6 is not zero, it increments a 32-bit value at an offset of -4 from a6 and branches to another routine.
- `return_with_offset`: Moves a 32-bit value from an offset of -4 of a6 into d0, then returns.
- `compare_bytes_and_exit`: Compares a byte at offset 8 of a structure in a0 with a byte at offset 12 of a structure in a1, and if they are not equal, clears d0 before returning.
- `increment_three_values`: Increments three 32-bit values at offsets of -4, 8, and 12 from a6 and branches to another routine.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `handle_system_tasks`: Performs a series of conditional calls to various routines based on values in global pointers and stack-based pointers.
- `allocate_and_initialize_structure_3`: It calls a routine, uses the result to access a structure, and then calls another routine with a pointer, a constant, and a value from a structure in a1.
- `allocate_and_fill_structure`: It calls a routine to allocate memory, then calls another routine to initialize a structure, and finally calls a third routine to copy data into a structure using values from a table.
- `allocate_and_initialize_structure_2`: Allocates memory, initializes a structure with several fields, and enters a loop that copies 16-bit words from one table to another based on a count, or performs a cleanup sequence involving multiple calls if the allocation fails.
- `dispatch_task_via_pointer`: Pushes d2 onto the stack, loads a pointer from an offset of 8 from the stack into d2, pushes a pointer from an offset of 12 from the stack onto the stack, pushes d2 onto the stack, pushes a pointer from an offset of 20 from the stack onto the stack, and calls a routine using a pointer from a6.
- `read_and_compare`: Calls a routine, compares the result in d0 to 1, and if they are not equal, returns a value from a stack location extended to 32 bits.
- `dispatch_write_task`: Calls a routine via a pointer in a6, then checks a byte at a pointer in a3; if the byte is not zero, it calls another routine using arguments from the stack.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_external_function`: Saves registers, loads a pointer from an offset of 20 from the stack into a3, loads a pointer from an offset of 24 from the stack into a0, loads a pointer from an offset of 28 from the stack into a1, loads a constant into a2, and calls a routine using a pointer from a6.
- `store_byte_and_increment`: Stores a byte from d0 into the memory location pointed to by a3 and increments a3.
- `multiply_signed_values`: Performs signed multiplication of two 32-bit values by splitting them into 16-bit components and using multiple multiply and add operations.
- `calculate_absolute_difference`: Calculates the absolute difference between two 32-bit integers using a loop that performs bitwise shifts and subtractions, then returns the result.
- `allocate_and_initialize_structure`: Saves registers, calls a routine with a value of negative 1, and if the result is not negative 1, it calls another routine to allocate memory, then uses the result to populate a structure with values from the stack and calls several other routines to initialize and add a port.
- `cleanup_structure_members`: Checks a value at an offset of 10 of a structure in a2, and if non-zero, calls three different routines to handle memory, signals, and ports associated with that structure.

### Stack

- `reallocate_and_copy_table_data_4`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_register_and_branch`: Copies the contents of a 32-bit register into another 32-bit register and then performs a short branch to another routine.

### Else

- `process_complex_table`: Calls a routine using a pointer from a5, performs arithmetic to calculate an index, accesses a table, and enters a loop that uses dbra to copy blocks of data between structures while calling another routine via a pointer in a5.
- `transfer_register_and_branch_2`: Moves the value from d1 to d0 and branches to another location.

### Delete

- `manage_data_tables`: Performs complex table manipulations including calling a routine with a pointer from a2, looping to copy data between structures, and updating a value at offset 48 of a structure in a2.
- `transfer_register_and_branch_3`: Moves the value from d1 to d0 and then branches to another location.

### type

- `process_complex_data_2`: As library code, it calls a routine, performs arithmetic on a value in d1, accesses a table using a scaled index, and contains multiple loops that call another routine and copy data between structures.
- `copy_and_branch`: It moves the value from d1 to d0 and then branches to another location.

### Path

- `reallocate_and_copy_table_data_5`: Calls another routine to determine a size, then performs a memory reallocation by copying data from an old table to a new location.
- `transfer_long_word_and_branch_3`: Copies a 32-bit value from register d1 to register d0 and then branches to another routine.

### EndCLI

- `reallocate_and_copy_table_data_6`: Calls another routine to determine a size, then reallocates a memory block by copying existing data to a new location and updating a pointer in a structure.
- `move_and_branch_6`: Moves the contents of d1 to d0 and then performs a short branch to another location.

### Dir

- `process_data_tables`: Calls a routine using a pointer from a5, performs arithmetic on d1, and uses a loop to copy data between tables and structures while calling other routines.
- `copy_long_and_branch`: Copies a 32-bit value from register d1 to register d0 and branches to another routine.

### NewCLI

- `reallocate_and_copy_table_data_7`: Calls another routine to determine a size, then reallocates a memory block by copying existing data to a new location and updating a pointer in a structure.
- `transfer_register_and_branch_4`: Moves a 32-bit value from register d1 to register d0 and performs a short branch to another routine.

### Echo

- `process_complex_data_tables`: It calls a routine using a pointer in a5, performs arithmetic on d1, and enters multiple loops that use dbra to copy blocks of data between tables and structures based on values retrieved from a2.
- `transfer_d1_to_d0`: Moves the value in d1 to d0 and then performs a short branch to another location.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `process_complex_data`: Calls a routine using a pointer from a5, performs arithmetic on d1, accesses a table using a scaled index, and contains two loops using dbra.
- `transfer_long_word_and_branch`: Copies a 32-bit value from register d1 to register d0 and then performs a short branch to another routine.

### Execute

- `reallocate_and_copy_table_data`: Calls another routine to determine a size, then performs a memory reallocation by copying data from an old table to a new location.
- `transfer_long_word_and_branch_2`: Moves a 32-bit value from register d1 to register d0 and then performs a short branch to another routine.

### makedir

- `reallocate_and_copy_data_table`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_register_to_register`: Copies the contents of a 32-bit register into another 32-bit register and branches to another routine.

### CD

- `reallocate_and_copy_table_data_2`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_data_and_branch`: Moves a 32-bit value from register d1 to register d0 and then branches to another routine.

### EndIf

- `return_zero`: Clears register d0 and returns immediately.

### If

- `reallocate_and_copy_table_data_3`: Calls another routine to determine a size, then performs a memory reallocation by copying data from an old table to a new location and updating a pointer in a structure.
- `move_and_branch_5`: Copies a longword from register d1 to d0 and then branches to another location.

### LoadWB

- `execute_library_sequence`: Calls a library function, then performs a string parsing loop to populate a buffer, and finally calls two more library functions and another routine.
- `execute_library_calls`: Calls three different library functions using pointers from a structure and a table, then returns.
- `call_library_functions_twice`: Calls two different library functions using the same address offset from a pointer in a4, then returns.
- `execute_library_call`: Loads a pointer into a1, calls a routine using a pointer from a6, stores the result in an offset of 8 from a6, and if the result is zero, branches to another routine before returning.
- `initialize_and_process_data_table`: Allocates memory and performs a string comparison using a XOR-based decryption routine, then enters a loop to iterate through a table of structures, calling several subroutines to process data, calculate offsets, and update values within the allocated memory blocks.
- `compare_byte_sequences`: Compares a byte from one memory location to a byte in another memory location, returning 0, 1, or -1 based on the comparison and the presence of a null terminator.
- `allocate_and_initialize_memory`: Calls a routine to allocate memory, then enters a loop that calls another routine to write data into the allocated space, finally updating a table with the new memory address.
- `call_write_2`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input_2`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output_2`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_dup_lock`: Library stub: loads its arguments and the dos library base, calls DupLock and returns its result to the caller.
- `call_io_err`: Library stub: loads its arguments and the dos library base, calls IoErr and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_2`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_find_task_2`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_put_msg_2`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_get_msg_2`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_wait_port_2`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library_2`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_set_function`: Library stub: loads its arguments and the exec library base, calls SetFunction and returns its result to the caller.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_copy_mem`: Library stub: loads its arguments and the exec library base, calls CopyMem and returns its result to the caller.
- `compare_memory_blocks`: Compares bytes from two memory locations, using a byte at offset 46 of the program counter as a condition, and branches if the bytes are not equal.
- `restore_registers_and_return_10`: Loads a constant into a2, restores a0 and a1 from the stack, pushes a constant from the program counter onto the stack, and returns.
- `execute_task_via_pointer`: Pushes a pointer from the stack to the stack, calls a routine at an address stored in a longword, and restores the stack pointer before returning.
- `process_buffer_and_call_routine`: Calls a library function, then loops through a buffer to check for non-zero bytes, and if a byte is found, calls another routine with arguments from the stack.
- `store_byte_and_return`: Stores the byte from register d0 into the memory location pointed to by a3 and increments a3 before returning.

### Wait

- `process_complex_data_structure`: Calls a routine using a pointer from a5, performs arithmetic on a value in d1, and enters loops that use dbra to copy blocks of data between memory locations and update a table at offset 8 of a structure in a2.
- `transfer_register_to_register_2`: Moves a 32-bit value from register d1 to register d0 and branches to another routine.

### Port-Handler

- `initialize_complex_structure`: Initializes a large structure by setting various offsets with calculated values, then performs multiple conditional calls to library functions to populate the structure based on data from a table.

### Disk-Validator

- `process_data_structures`: Performs complex manipulations on multiple data structures, including calling several routines via a function pointer in a6, updating various offsets in a structure in a1, and performing conditional bitwise operations and memory writes based on values retrieved from a2 and a0.

### Ram-Handler

- `copy_memory_wrapper`: Calls a function at an offset from a library pointer in a6 to copy memory from a source in a2 to a destination in a0, then returns the difference between the original and new destination pointer.

### serial.device

- `return_immediately`: The routine does nothing and returns immediately.
- `update_device_parameters`: Calls two different library routines using a function pointer in a6, and if certain conditions are met, updates several fields in a structure at an offset from a6 using values from d0 and d2.
- `check_and_call_routine`: Checks a bit in a byte at an offset from a1 against a mask, and if the result is non-zero, calls a routine via a function pointer in a6.
- `write_status_byte`: Writes the byte value 253 to an offset of 31 from a1 and then branches to another location.
- `toggle_interrupt_enable_2`: Writes values to the INTENA register to enable and then disable interrupts, calls a routine, and then writes to INTENA again before returning.
- `check_status_type`: Clears a byte at offset 31 of a structure in a1 and branches to different locations based on whether a word at offset 28 of that structure is equal to 2, 3, or 10.
- `clear_status_on_mismatch`: Compares a pointer in a1 with a value at offset 74 of a6, and if they are not equal, clears bit 1 of offset 474 of a6 and clears the pointer at offset 74 of a6.
- `swap_structure_pointers`: Tests bit 6 of a byte at offset 30 of a1; if set, it performs a series of pointer swaps between a structure in a1 and another structure, decrements a 16-bit value at offset 70 of a6, and branches to another location.
- `update_library_and_hardware`: Compares a1 with a value at offset 78 of a6; if they are not equal and the word at offset 28 of a1 is 10, it clears bit 5 of a byte at offset 294 of a6, writes 2048 to a hardware register, calls another routine, clears a value at offset 78 of a6, clears bit 4 of a byte at offset 296 of a6, and branches to another location.
- `modify_status_and_call`: Tests a bit in a byte at offset 30 of a1, and if set, performs several pointer swaps and bit manipulations on a structure in a1 before calling a routine via a function pointer in a6.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_bit_2`: Sets d0 to 0; if the word at offset 32 of a6 is non-zero, it sets bit 6 of a byte at offset 296 of a6 and returns.
- `execute_device_commands`: Calls four different library routines with various parameters, including passing values from a1 and d0, and performs a pointer update on a structure in a1 before returning.
- `exchange_list_nodes`: Clears a 32-bit value at offset 50 of a1, loads a 32-bit value from offset 54 of a1 into d1, and if d1 is not zero, checks two values in a6; if certain conditions are met, it clears bit 0 and sets bit 6 of a byte at offset 48 of a1, performs a pointer exchange between a1 and a structure at offset 258 of a6, increments a 16-bit value at offset 112 of a6, and calls a routine at offset -180 of a6.
- `process_serial_data_2`: It saves d2 and a6 to the stack, loads a pointer from a1 into a6, and if a longword at offset 116 of a6 is zero, it performs several operations including updating a pointer at offset 116, decrementing a word at offset 112, clearing a longword at offset 144, and setting or clearing bits in a structure at offset 48.
- `process_serial_status`: Checks a value at offset 56 of a6, and if it is non-zero, performs several bit tests on offsets 472, 69, 445, and 472 of a6, and if certain conditions are met, clears bit 0 of d0 using a byte from offset 66 of a6.
- `test_bit_and_branch_2`: Tests bit 0 of a byte at offset 445 of a structure in a6 and branches to different locations based on the result.
- `set_byte_and_branch_2`: Calls a routine, and if the result is zero, sets a byte at offset 31 of a structure in a1 before branching.
- `process_serial_data`: Loads a byte from offset 66 of a6 into d1, clears bit 0 of d0, and moves a 32-bit value from offset 54 of a1 to a location pointed to by offset 82 of a6; it then enters a loop that decrements d1 and compares a byte from a table to d0, and if the loop completes, it clears several bits in a6 and a1 and calls a routine at offset -378 of a6.
- `restores_registers`: restores d2 and a6 from the stack and returns.
- `test_bit_in_value`: Performs bitwise transformations on d0 to derive a bit index, then tests that bit against a value retrieved from an offset in a6, returning based on the result.
- `clear_status_bit`: Clears bit 2 of a byte at offset 470 of a structure in a6, and if bit 2 was already clear, stores the value of 15 in a byte at offset 31 of a structure in a1, then branches to another routine.
- `clear_bit_and_set_byte`: Clears bit 7 of a byte at offset 470 of a structure in a6 and, if the bit was already clear, sets a byte at offset 31 of a structure in a1 before branching.
- `modify_status_byte`: Sets a byte at offset 31 of a structure in a1, tests bit 0 of offset 470 of a6, and if set, sets a different byte at offset 31 of a1 and clears offset 478 of a6.
- `process_serial_interrupt`: Clears bit 6 of a byte at offset 470 of a structure in a6, and if bit 0 of a byte at offset 69 of a6 is set, it clears bit 2 of a byte at offset 472 of a6 after comparing a byte from offset 66 of a6 with a byte at offset 35 of a6.
- `disable_serial_interrupts`: It compares a byte at offset 34 of a6 with d1, and if the byte at offset 472 of a6 has bit 2 set, it clears bit 2 of that same byte and writes the value 32769 to the INTENA hardware register.
- `test_serial_bits`: Tests bit 5 of an offset in a6, bit 1 of another offset in a6, and bit 8 of d0, and calls another routine if bit 1 is set and bit 8 is not clear.
- `test_status_bit`: Tests bit 8 of d0, and if bit 8 is set, branches to another routine; otherwise it branches to another routine.
- `test_bit_and_branch`: Tests bit 0 of a byte at offset 445 of a6 and branches to another location.
- `set_bit_and_move_pointer`: Sets bit 7 of a byte at offset 470 of a structure in a6 and, if a longword at offset 62 of that structure is non-zero, moves a longword from offset 42 to offset 62 before branching.
- `process_data_buffer`: Compares two longwords at offsets 54 and 58 of a structure in a6, and if the first is not less than or equal to the second, stores a byte from d0 to a pointer at offset 42 of a6, increments offsets 42 and 58 of a6, and then performs a series of pointer manipulations and conditional branches.
- `transfer_pointer_and_call_2`: Moves a longword from offset 42 of a6 to offset 62 of a6, calls a routine using a pointer from a6, and restores registers before returning.
- `clear_hardware_bit`: Moves a byte from offset 34 of a6 to offset 473 of a6, clears bit 3 of offset 472 of a6, and if bit 2 of offset 69 of a6 is set, clears bit 6 of the hardware register at offset bf d000.
- `configure_interrupts`: Moves a byte from offset 35 of a6 to offset 473 of a6, sets bit 3 of a byte at offset 472 of a6, tests bit 2 of a byte at offset 69 of a6, and if set, sets bit 6 of a hardware register and writes 32769 to a hardware register before returning.
- `clear_status_byte`: clears a byte at an offset from a6 and returns.
- `toggle_device_bit`: Tests bit 6 of offset 470 of a6 and sets it if it is not set, then prepares a structure in a1 to call a routine at offset 20 of a1 using a pointer from a6, and finally updates offset 31 of a1 based on the return value in d0.
- `update_serial_config`: Saves d2, a3, and a6 to the stack, sets a6 to a1, clears bit 6 of a byte at offset 470 of the structure in a6, and if bit 6 was set, tests bit 0 of the word at the address in the custom device register and, if non-zero, sets bit 2 of a byte at offset 470 of the structure in a6 before branching to another routine; otherwise it branches to another routine.
- `call_subroutine_via_offset`: Loads the address at offset 386 of a6 into a0, calls another routine, and branches to another location.
- `set_status_bit_3`: Sets bit 0 of a byte at offset 470 of a structure in a6 and branches to another routine.
- `swap_structure_pointers_3`: Clears offset 32 of a1, and if offset 36 of a1 is non-zero, performs a series of pointer swaps between a structure in a1 and another structure, then increments a 16-bit value at offset 72 of a6.
- `handle_interrupt_status`: Tests bit 2 of a byte at offset 69 of a6; if set, it calls another routine, tests a byte at offset 31 of a1, and if non-zero, sets bit 4 of a byte at offset 30 of a1, clears a 32-bit value at offset 94 of a6, copies two pointers from a1 into a6, and writes 32769 to two hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_serial_status`: Reads a byte from a hardware register, checks if bits 3 and 4 are set, and if so, writes the value 13 to an offset of 31 in a structure pointed to by a6.
- `update_serial_state`: It tests a byte at offset 473 of a1, saves d2 and a6 to the stack, and if several conditions regarding bits in a6 and a1 are met, it decrements a word at offset 470 of a6, performs a left shift on a word from offset 462 of a6, and calls another routine.
- `update_serial_register_2`: Writes 504 to an offset in a6, loads a byte from a pointer in a6 into d0, and if a value at offset 36 of a1 is positive, decrements a value at offset 94 of a6.
- `update_serial_status_2`: Compares a value in a structure to a register, sets a bit in the interrupt request register, and performs conditional bit tests and clears on a structure in a6 before returning.
- `update_serial_register`: Calls another routine, sets a bit in a byte from offset 67 of a6 and writes the result to the serial data register, increments offsets 86 and 94 of a6, and then performs a loop to compare a byte against a value in a table before calling another routine via a pointer in a6.
- `rotate_serial_buffer_node`: Updates a linked list by replacing a node with a new one, modifies status bits in the node structure, and calls another routine if a specific field equals ten.
- `enable_interrupts`: Saves registers to the stack and writes the value 1 to the INTENA hardware register.
- `update_interrupt_mask`: Writes 1 to the interrupt enable register, clears d1, loads a byte from offset 417 of a1 into d1, performs a bitwise OR between d1 and a word from offset 412 of a1, clears the byte at offset 417 of a1, writes d1 to the interrupt mask register, and branches based on the value at offset 78 of a1 or the value at offset 72 of a1.
- `reconfigure_serial_interrupts`: Enables interrupts via the hardware register, updates several longword fields in a structure, and then disables interrupts before branching to another routine.
- `call_subroutine_and_branch`: Calls a subroutine and then performs a branch.
- `copy_structure_fields`: Copies a longword from offset 38 of a structure in a6 to offset 42 of the same structure, copies it again to offset 46, and clears the longword at offset 58.
- `handle_serial_event`: If the longword at offset 46 of a6 is non-zero, it sets bit 2 of offset 1d8 of a6; then, if the longword at offset 4a of a6 is non-zero, it calls another routine before branching to another location.
- `clear_interrupt_flag`: Clears bit 2 of offset 440 of a6, and if a longword at offset 74 of a6 is non-zero, calls another routine and writes the value 32769 to offset 156 of a6.
- `toggle_interrupt_enable`: Writes 2049 to the interrupt enable register, calls another routine, and then writes 34817 to the interrupt enable register.
- `clear_serial_buffers`: Pushes a1 to the stack, loads the address of offset a2 of a6 into a0 and calls another routine, loads the address of offset b0 of a6 into a0 and calls another routine, clears offset 46 of a6, and restores a1 from the stack before returning.
- `transfer_pointer_and_call`: Copies a longword from a1 to a0, stores a0 at offset 4 of a1, and if d0 is not zero, calls a subroutine and then branches back to the start of this routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_serial_parameters`: Sets bit 5 of an offset 30 in a structure in a1, writes 254 to offset 31, and if bit 0 of offset 30 is not set, calls a routine via a pointer found at offset 98 of a structure in a6; otherwise it returns.
- `modify_status_and_branch`: Loads a byte from offset 194 of a0, masks it, checks bits in a6, sets bit 4 of the masked byte, stores it at offset 81 of a1, copies a byte from offset 470 of a6 to offset 80 of a1, masks it, copies a longword from offset 58 of a6 to offset 32 of a1, and branches to another routine.
- `test_status_bits`: Clears bit 0 of offset 30 of a1, tests bit 3 of offset 79 of a1, and then checks if the longwords at offsets 72 and 78 of a6 are non-zero.
- `update_serial_status`: Loads a longword from a global variable into a register, calls another routine, and if a specific byte in a structure is non-zero, updates a global pointer, sets bit 5 of a byte at offset 471 of a structure in a6, and writes a value to the interrupt enable register before branching to another routine.
- `clear_structure_field`: Calls a routine via a pointer found at offset 98 of a structure in a6 and then clears the longword at offset 78 of that structure.
- `handle_serial_interrupt`: Clears bit 5 of a value at offset 71 of a1, and if bit 5 is set, checks if a value at offset 40 of a structure in a1 is 10; if so, it sets bit 1 of a value at offset 70 of a6, clears offset 120 of a6, writes 2048 to a hardware register, and calls a routine via a pointer at offset 152 of a6 before writing 32769 to the INTENA register.
- `execute_serial_command`: Writes 1 to the INTENA register, populates a structure on the stack with values 0, a value in d0, and 9, then calls a routine via a pointer at offset 32 of that structure; if the byte at offset 49 of the structure is not zero, it writes 11 to that offset and returns.
- `modify_status_byte_2`: Tests bit 7 of an offset in a1, and if set, sets bit 7 of an offset in a6, clears bit 2 of an offset in a6, and calls another routine if bit 3 of an offset in a6 is set.
- `check_and_set_status_byte`: Clears bit 7 of a byte at offset 69 in a structure, then performs several conditional checks on bits and values at offsets 49, 48, and 30 of structures in a6 and a1, finally setting bit 0 of a byte at offset 31 in a1 if certain conditions are met.
- `toggle_interrupt_enable_and_call_routine`: Disables interrupts by writing to the INTENA register, calls another routine, re-enables interrupts by writing to the INTENA register, and then branches to another routine.
- `validate_and_update_device_parameters`: Validates three bytes and a longword within a structure, updates several fields in a second structure based on bit tests, and calls two subroutines before writing a byte to offset 31 of the input structure.
- `fetch_and_store_values`: Clears d0 and d1, loads a byte from offset 77 of a1 into d0, sets bit d1 of d0, compares d0 to 2, and if not equal, performs a series of shifts and bit sets before checking bit 0 of offset 69 of a6 to call a subroutine and store values in a6.
- `fetch_table_value`: Checks bit 0 of an offset 69 in a structure in a6; if set, it decrements d0, shifts it left by 1, and uses the result to fetch a word from a table located at a specific offset relative to the program counter.
- `set_byte_and_branch`: Writes 5 to offset 31 of a1 and branches to another routine.
- `calculate_scaled_division`: Loads a longword from offset 450 of a6 into d0 and d1, shifts d0 left by 3, subtracts d1 from d0, compares d0 to 65535, and if not less than or equal, performs a series of shifts and a division before branching back.
- `calculate_division_result`: Divides d0 by d1, stores the quotient in an offset of a6, clears bit 5 of an offset of a6, and if an offset of a6 is 8 and bit 0 of an offset of a6 is set, it performs an OR operation on the quotient and writes it to a hardware register after setting bit 5 of an offset of a6.
- `configure_serial_device_2`: Calls a library function, then if successful, sets values in a structure and calls two more library functions, including one with a zero argument and one with a one argument.
- `load_serial_pointers`: Loads two addresses into a0 and a3 from offsets of a6 and calls another routine twice before branching to another location.
- `store_pointer`: Stores the longword in a0 into offset 16 of a structure pointed to by a1.
- `update_serial_structure`: Writes the value 16 to offset 9 of a structure in a0, stores 56 at offset 16 of the structure in a0, and copies the contents of a3 and a6 to offsets 18 and 14 of that structure before returning.
- `initialize_serial_parameters`: Writes 56 to offset 16 of a structure in a1, sets bytes at offsets 9, 14, and 8 to 0, 1, and 4 respectively, and calls a library function using a pointer from a6.
- `configure_serial_device`: Library code that performs several subroutine calls to set up device parameters, calls a library function using a pointer from a6, and writes a value to the INTENA hardware register.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_d0_and_branch`: Clears the d0 register and performs a short branch to a previous instruction.

### dk

- `modify_d0_and_memory`: Performs several bitwise OR operations on d0 and subtracts 0 from a 16-bit value at offset 76 of a structure in a6 using d0 as an index.

### is

- `clear_d0_and_decrement_a6`: Performs three bitwise OR operations on d0 and subtracts 0 from the address stored in a6.

### d

- `execute_thirteen_instructions`: Performs thirteen instructions without a return or a loop.

### e

- `initialize_keymap_data`: Clears a 16-bit value in a global variable using a zeroed register as an index, then performs a series of byte-wise logical OR operations on the same register with various constants.

### f

- `clear_d0_and_decrement_stack`: Clears the lower byte of register d0 three times and subtracts zero from the value pointed to by the address in register a6.

### cdn

- `initialize_keymap_data_2`: Clears a 16-bit value in a global variable using an offset of 76 from a base pointer in a6 and performs a series of byte-wise logical OR operations on the d0 register with various constants.

### usa0

- `initialize_program_execution`: Sets the initial state of the processor and prepares the environment for the main program execution before returning.

### i

- `clear_d0_and_decrement_a6_2`: Clears the byte in d0 three times and subtracts zero from a 16-bit value at the address pointed to by a6, decrementing a6.

### usa1

- `clear_d0_and_decrement_a6_post_inc`: Clears the byte in d0 three times and subtracts zero from a 16-bit value at the address pointed to by a6, then increments a6.

### usa2

- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### n

- `initialize_keymap_n`: Clears d0 three times, subtracts zero from a 16-bit value at offset 76 of a structure using d0 as an index, and then writes several specific bytes into d0.

### s

- `initialize_keymap_data_3`: Clears a 16-bit word in a global variable using a zeroed register and then performs a sequence of byte-wise logical OR operations to populate a register with a specific bitmask pattern.

### gb

- `initialize_keymap_gb`: Clears d0 three times, subtracts zero from a 16-bit value at offset 76 of a structure pointed to by a2 using d0 as an index, and then writes several specific bytes into d0.

### ch1

- `initialize_keymap_ch1`: Clears d0 three times, subtracts 76 from a 32-bit value in d5, and then writes several specific bytes into d0.

### ch2

- `modify_data_register_sequence`: Subtracts seventy-six from a long word in register d5 and performs a series of logical OR operations on register d0 using several constant byte values.

### parallel.device

- `process_parallel_handler`: Sets d0 and d1 to 97, performs conditional branches based on status flags, and if certain conditions are met, writes specific bytes to the address in a0 and the address in a2, then clears bit 0 of a byte at the address pointed to by a6.
- `update_device_status_flag`: Clears a byte at offset 118 of a structure, checks bit 5 of a byte at offset 53 of another structure, and if certain conditions are met, increments a word at offset 32 of the first structure and moves two longwords into offsets 54 and 58 of the second structure.
- `execute_library_command`: Loads 81 into d1, performs a bitwise AND with a byte from offset 30 of a1, and if non-zero, calls a library routine via a pointer in a6, then checks a word at offset 28 of a1, bit 4 of offset 118 of a6, and modifies hardware registers before branching.
- `execute_parallel_command`: Compares a 16-bit value at offset 28 of a structure to 3, clears bit 0 of a hardware register, checks bit 0 of another hardware register, and if bit 2 of a byte at offset 118 of a structure is also zero, it sets d0 to 144, pushes a6 and a1 onto the stack, calls another routine, restores the registers, and sets bit 2 of a byte at offset 118 of a structure.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_parallel_byte`: Writes the byte value 253 to offset 1f of a1 and branches to another location.
- `check_parallel_status`: If the longword at offset 24 of a1 is non-zero, it branches to another location; otherwise, it clears offset 20 of a1 and branches to another location.
- `swap_structure_pointers_2`: Clears bit 0 of offset 30 of a1, checks bit 2 of offset 118 of a6, and if set, sets bit 6 of offset 30 of a1 and performs a series of pointer swaps between a1 and a structure in a6 before incrementing a word at offset 114 of a6.
- `update_parallel_status`: Sets bit 4 of an offset 30 byte in a structure pointed to by a1, copies a pointer from offset 40 of a1 to offset 42 of a6, and performs conditional bit manipulations on offsets 116 and 118 of a6 and a hardware register based on the value at offset 36 of a1 and a comparison of a value at offset 28 of a1 with 2, finally storing a byte in a hardware register.
- `process_parallel_device_queue`: Decrements a counter in a structure, updates a pointer in a linked list, clears bit 6 of a status byte at offset 30, and calls another routine.
- `handle_parallel_data`: Tests bit 5 of offset 46 of a1 and bit 3 of offset 118 of a6, reads a byte from a hardware register, stores it at a pointer from offset 42 of a6, and performs a loop to compare a byte against a table before returning.
- `update_parallel_device_status`: Copies a longword from offset 46 of a structure in a6 to offset 32 of a1, clears bit 3 of offset 118 in a6 and bit 4 of offset 30 in a1, calls a routine at offset 50 of a6, clears offset 38 of a6, and if offset 114 of a6 is not zero, clears bit 2 of offset 118 in a6 and calls a routine at offset 34 of a6 with d0 set to 16.
- `poll_parallel_device_status`: Reads a byte from a hardware register via a pointer and branches based on the result, then checks bit 3 of a byte at offset 118 of a structure in a6 and branches to another routine or back to the start of this routine depending on the bit state.
- `call_device_handler`: Calls a routine using a pointer from a structure in a6, after loading a longword from a specific offset into d0 and updating a pointer in a1.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_function_offset_16`: Checks if a value at offset 26 of a6 is non-zero, then calls a library function at an offset of minus 18 from a library base found at offset 22 of a6, passing a pointer from offset 26 of a6 and the constant 16 as arguments.
- `call_library_function_offset_144`: Checks if a value at offset 26 of a6 is non-zero, then calls a library function at an offset of minus 18 from a library base found at offset 22 of a6, passing a pointer from offset 26 of a6 and the constant 144 as arguments.
- `update_handler_pointer`: Loads a pointer from offset 4c of a6 into a1, loads a longword from a1 into d0, and if non-zero, writes d0 back to a1, swaps d0 and a1, stores a0 at offset 4 of the new a1, and if d0 is non-zero, calls another routine before branching to another location.
- `clear_word_at_offset_114`: Clears a 16-bit word at offset 72 of a6 and returns.
- `set_hardware_bits_and_call_library`: Sets bit 5 of an offset 30 byte and sets an offset 31 byte to a specific value in a structure pointed to by a1, then calls a library function at an offset of minus 378 from a library base found at offset 50 of a6.
- `modify_cia_and_library_state`: Calls a routine with d0 set to 16, modifies bits in a hardware register via a pointer, performs bitwise operations on a hardware register and a value from a structure at offset 116 of a6, and calls another routine after updating a structure at offset 52 of a1.
- `set_parallel_flag`: Tests bit 2 of offset 76 of a6, and if zero, writes the byte value 1 to offset 1f of a1 before branching to another location.
- `transfer_structure_fields`: Tests bit 1 of a byte at offset 53 of a1; if set, it copies a 32-bit word from offset 54 of a1 to offset 85 of a6 and another 32-bit word from offset 58 of a1 to offset 94 of a6.
- `handle_parallel_queue`: Calls several routines using a pointer in a6, performs a loop that updates a linked list and clears a byte at offset 12 of a structure, and eventually calls two more routines using a pointer from a structure in a6.
- `clear_d0_and_branch_2`: Clears d0 and branches to another routine.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_handler_call`: Saves a0 and a6 to the stack, calls another routine, increments the stack pointer by 8, and returns.
- `call_clipboard_handler`: Saves d0 and a1 to the stack, calls a routine at an address in a pointer, and then increments the stack pointer before returning.
- `dispatch_clipboard_callback`: It pushes a pointer from a1 onto the stack, calls a routine at a specific address, increments the stack pointer, and returns.
- `jump_to_clipboard_handler`: Performs a long jump to the address at offset f76.
- `call_handler_function`: Pushes the value of a1 onto the stack, calls a routine at a specific address, and then returns.
- `dispatch_library_call_2`: Pushes a1 onto the stack, calls another routine, and increments the stack pointer by 4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_library_loop`: Enters a loop that stores a longword from d3 into a structure at an offset of a pointer in d2 and calls a routine using a pointer from the stack.
- `execute_library_calls_4`: Stores a longword from d3 into a structure at an offset of a pointer in d2, calls two library functions using a pointer from the stack, and then calls a third routine using a pointer from a structure in d2 if a specific byte is non-zero.
- `execute_multiple_library_calls`: Calls two library functions at offsets of minus 138 and minus 432 from library bases stored in a pointer on the stack, then restores registers and returns.
- `update_structure_data`: Calls two routines, then if a returned value is non-zero, it accesses a structure via a pointer in a2 to check a byte at offset 4 of a sub-structure and potentially increments a longword at offset 14 of a different structure pointed to by a3.
- `clear_offset_e_and_call`: Clears a longword at offset 14 of a3, pushes the value at offset 72 of a2 onto the stack, calls a routine, and branches to another routine.
- `clear_and_call_library`: Clears a 16-bit word at offset 94 of a2, calls a library function at offset minus 138 from a library base, and returns d0 with the value of a3.
- `process_pointer_data`: Saves registers, loads a pointer from a stack offset into a2, calls a routine, and if a value at offset 72 of a2 is non-zero, it calls another routine using a pointer from offset 48 of a2.
- `call_library_and_clear_offset`: Pushes the value at offset 72 of a2 onto the stack, calls a routine, clears offset 86 of a2, calls another routine, and returns.
- `dispatch_clipboard_task`: Calls a routine using a pointer from the stack, then if a byte at offset 94 of a2 is not zero and a longword at offset 68 of a2 is not zero, it sets bit 6 of a byte at offset 30 of a4 and calls another routine using values from the stack.
- `compare_pointer_values_2`: It loads a longword from offset 90 of a2 into d0, compares it with a longword on the stack, and if the value on the stack is not lower than d0, it performs arithmetic using values from offset 44 and offset 44 of a2 and the stack.
- `compare_pointer_values`: Checks a value on the stack, performs a logical shift right on a value from the stack, and if a pointer at offset 86 of a2 is non-zero, compares a value in d0 with a longword at offset 14 of a structure in a1.
- `compare_and_branch_long`: Moves a long value from d1 to d0, compares d0 with a long value at offset 14 of a1, and branches if d0 is less than or equal to that value; otherwise, it updates a1 with a pointer from a1 and branches.
- `compare_and_branch_4`: Compares d5 with the result of subtracting d2 from 256, and if d5 is less than or equal to that result, it moves d2 into d4 and branches to another routine; otherwise it returns.
- `copy_bytes_with_limit`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d4 as an index in a loop, then performs arithmetic using d2, d5, and d0 before branching to another routine.
- `move_word_to_register`: Moves a 16-bit word from d2 to d4 and branches to another routine.
- `copy_bytes_and_compare`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d4 as an index in a loop, then compares the extended version of d4 against the sum of d1 and d2 before branching to another routine.
- `store_pointer_and_call_library`: Stores a pointer from a1 into offset 86 of a2, calls a library function at offset minus 138 from a library base, and returns d0 with the value of d3.
- `manage_clipboard_data`: Calls several library functions and other routines to manipulate data structures, including conditional loops and stack manipulations based on values in a2.
- `call_multiple_library_functions`: Pushes a value from a global address onto the stack, calls a routine, and if d0 is not 256, clears offset 94 of a2 and calls another routine using a pointer in d4.
- `execute_clipboard_jump`: Pushes the address of a value at an offset of a2 onto the stack, calls a routine, increments the stack pointer, and branches to another location.
- `execute_clipboard_operation`: Clears a word at offset 94 of a2, and if the word is not zero and a longword at offset 68 of a2 is not zero, it sets bit 6 of a byte at offset 30 of a4, calls a routine using a pointer from offset 24 of a6, and calls another routine using a pointer from offset 20 of a6.
- `traverse_structure_list`: It loads a longword from offset 36 of a6 into d2, performs a logical shift right on d2 by 9, loads a longword from offset 36 of a6 into d3, and if a pointer from offset 134 of a2 is zero, it calls another routine and then enters a loop that compares d0 with a longword at offset 20 of a structure in a1.
- `loop_structure_updates`: Contains a loop that tests d4 and compares d0 with a longword at offset 14 of a structure in a1, and if conditions are met, calls another routine and repeats the loop.
- `check_offset_and_branch`: Tests d4 and branches if zero, tests a long value at offset 20 of a6 and branches if less than or equal, then compares 256 minus d3 with a long value at offset 20 of a6; if d3 is less than that value, it moves d3 into d1 and branches, otherwise it moves 256 minus d3 into d0 and branches.
- `copy_bytes_in_loop_3`: Runs a loop that copies bytes from a source at a3 to a destination at an offset of a1 using d1 as a counter, then performs arithmetic on a6 and calls another routine.
- `move_word_to_register_offset`: Moves a 16-bit word from d3 to d1 and branches to another routine.
- `copy_bytes_with_offset_check`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d1 as an index in a loop, compares the extended d1 against the sum of d2 and d3, and if the condition is met, adds offset 20 of a6 to d5, clears offset 20 of a6, and branches to another routine.
- `call_library_function`: Stores the value from d4 into a structure at offset 86 of a2, calls a library function, moves the result from d5 to d0, restores registers, unlinks a6, and returns.
- `check_bits_and_call`: Calls a routine using a pointer from a stack offset, then checks bits 7 and 0 of a byte at offset 30 of a structure in a2, and if both are zero, calls another routine using a pointer from a2.
- `modify_byte_and_call`: Clears bit 1 of a byte at offset 30 of a2, sets bit 7 of the same byte, calls a library function, and restores a2 before returning.
- `modify_structure_bits`: Checks bit 7 of a byte at offset 30 of a structure in a2, and if it is not set, calls a routine; it then potentially clears bit 1 or bit 4 of that same byte and calls another routine before returning.
- `compare_structure_values`: Loads two structures from offsets 4 and 70 of a0, checks if the pointer at offset 112 of the second structure is null, checks if the pointer at offset 96 of the first structure is null, and compares a longword at offset 48 of the first structure with a longword at offset 48 of the second structure.
- `set_zero_and_branch`: Sets d0 to zero and branches to another routine.
- `set_d0_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_members_2`: Pushes d2 onto the stack, loads pointers from offsets 8 and 12 of the stack into a0 and a1, moves a long value from offset 16 of a7 into d2, copies values between a0 and a1 at specific offsets, pushes d2 and a1 onto the stack, calls another routine, and returns d2.
- `modify_clipboard_status_byte`: Sets bit 5 of a byte at offset 30 of a structure in a3, and if a byte at offset 66 of a structure in a2 is zero, it clears bit 1 of that same byte and branches; otherwise it branches to another location.
- `modify_clipboard_status`: Sets a byte at offset 66 of a structure in a2, and if a word at offset 64 is zero, it calls a subroutine.
- `clear_d3_and_branch_2`: Clears a 16-bit word in register d3 and performs a short branch to another routine.
- `handle_clipboard_request_2`: Checks if a pointer at an offset of a2 is non-zero, copies a word from a structure to an offset in a2, and if that word is 9, calls three different routines including a library function.
- `call_and_set_word`: Saves a2 to the stack, calls another routine, increments the stack pointer by 4, writes 1 to an offset of 5e of the structure in a2, and branches to another location.
- `handle_clipboard_request`: Compares a word at offset 64 of a structure in a2 to 9, calls two routines, and if a pointer at offset 96 of a4 is non-zero, calls another routine and writes 3 to offset 64 of a2.
- `clear_register_and_branch`: Clears register d3 and branches to another location.
- `update_clipboard_pointer`: Loads a pointer from a structure in a2, and if non-zero, updates a pointer in a2 from a structure in a4, compares a word from a4 to nine, and if they match, calls another routine.
- `process_clipboard_request`: Compares a word at offset 64 of a2 to 2, and if equal, performs several checks on a structure at a4, pushes multiple pointers onto the stack, calls another routine, and updates a value in a4.
- `compare_and_update_offset`: Loads a longword from offset 91 of a structure in a2, adds two longwords from offset 36 and 44 of a structure in a4, compares the result to the value from a2, and if the result is not lower, stores the value from offset 36 of a4 into offset 32 of a4.
- `update_offset_value_2`: Subtracts a longword at an offset of a4 from a longword at an offset of a2, stores the result in a longword at an offset of a4, and then adds that result back to the longword at an offset of a4 before branching.
- `update_clipboard_state`: It clears a longword at an offset of a4, decrements a word in a structure in a2, calls another routine, and then enters a loop that clears a word in a2 and branches back to the start of the loop.
- `manage_clipboard_data_2`: Compares a word at offset 64 of a structure in a2 to 3, and if equal, checks a pointer at offset 112 of a2 and performs a series of stack operations and a routine call.
- `copy_and_call_data`: Moves a longword from an offset of a4 to d2, stores d2 at an offset of a2, pushes a4 and a3 onto the stack, calls another routine, and branches to another location.
- `check_clipboard_offset`: Compares a word at offset 28 of a structure in a4 to 4; if equal, it writes 4 to offset 64 of a structure in a2 and branches.
- `clear_d3_and_branch`: Clears the word in d3 and branches to another location.
- `clear_clipboard_offset`: Compares a word at an offset of a2 to 4, and if equal, checks if a longword at an offset of a2 is greater than 8; if so, it pushes several values onto the stack, calls a routine, and then either branches or increments a longword at an offset of a2.
- `clear_clipboard_data`: Clears a longword at offset 90 of a2 and branches to another location.
- `clear_and_modify_bits`: Clears a long value at offset 90 and a word at offset 110 of a2, pushes a3 and a4 onto the stack, calls another routine, clears a word at offset 64 of a2, and if d3 is not zero, checks bit 5 of a byte at offset 30 of a3 and clears bit 7 of that byte if the bit is set, before clearing a byte at offset 66 of a2 and returning.
- `compare_and_load_values`: Loads a pointer from a stack offset into a2, retrieves a longword from a structure pointed to by a2 into d1, and if d1 is non-zero, compares a longword at offset 48 of a structure in a2 with a longword at offset 48 of a structure in a0.
- `set_d1_and_branch`: Sets d1 to zero and branches to another routine.
- `call_library_with_stack_args`: Pushes the contents of a2 offset 4, a0, and a1 onto the stack, calls a library function, cleans up the stack, restores a2, and returns.
- `write_byte_and_call`: Writes the byte value 253 to an offset of 31 from a pointer in a0, then calls another routine.
- `execute_handler_via_stack`: Pushes d2 onto the stack, loads a value from offset 8 of the stack into d2, pushes d2 onto the stack, calls another routine, and returns d2.
- `write_byte_and_call_wrapper`: Pushes two values onto the stack, calls another routine, and then restores the stack and a register before returning.
- `write_byte_and_call_wrapper_2`: Pushes two values onto the stack, calls another routine, and then restores the stack before returning.
- `push_and_call_subroutine_3`: Pushes a value from a stack-relative location onto the stack, calls another routine, and then restores the stack and a register before returning.
- `execute_multiple_wrappers`: Pushes three values from the stack into d2, then calls three different routines using d2 as an argument before returning.
- `update_clipboard_data`: Saves registers to the stack, loads two pointers from the stack, tests a longword at an offset of the first pointer, and if zero, copies a longword from a third pointer to that offset and calls another routine.
- `call_subroutine_with_pointer`: Pushes a pointer from a2 to the stack, calls a subroutine with a pointer from a3, and then calls another subroutine before restoring a2 and a3 from the stack.
- `increment_clipboard_pointer`: Saves registers to the stack, loads two pointers from the stack, and if a longword at an offset of the first pointer is zero, increments a longword at an offset of the second pointer and calls another routine.
- `call_subroutine_with_offset_pointer`: Pushes a pointer from a2 to the stack, calls a subroutine with a pointer from a3 at offset 112, and then calls another subroutine before restoring a2 and a3 from the stack.
- `store_offset_38_and_call`: Loads a pointer from offset 24 of a structure in a0, retrieves a longword from offset 56 of that pointer, and stores it at offset 48 of the structure in a0 before calling another routine.
- `store_offset_3c_and_call`: Loads a pointer from offset 24 of a structure in a0, retrieves a longword from offset 60 of that pointer, and stores it at offset 48 of the structure in a0 before calling another routine.
- `compare_clipboard_data`: Calls a routine using a pointer from a4, then if a byte at offset 69 of a2 masked with 247 is not zero, it compares a longword at offset 14 of a structure in a3 with a longword from offset 8 of a6.
- `compare_and_branch`: Compares a longword at a pointer in a3 with d3, then branches to another routine if they are not equal; otherwise it clears d4 and d2 and branches to another routine.
- `handle_clipboard_error`: It tests d4, and if non-zero, pushes a constant onto the stack, calls another routine, compares the result in d0 with zero, and if zero, writes a byte to offset 49 of a2 and calls another routine.
- `process_clipboard_structure`: Sets several fields in a structure at a3, calls four different routines, and performs conditional logic involving clearing a word at offset 94 of the structure and iterating through a loop based on a value in a6.
- `call_library_and_modify`: Pushes two values onto the stack, calls a library function, and if the result in d0 is 512, it clears a word at offset 94 of a structure in a3 and subtracts 512 from a value at offset minus 4 of a6; otherwise it returns via a branch.
- `test_and_clear_word`: Tests a word at offset 94 of a3 and branches if zero, tests a long value at offset -4 of a6 and branches if less than or equal, otherwise it pushes a3 onto the stack, calls another routine, and if the result in d0 is not zero, clears a word at offset 94 of a3 and branches.
- `process_clipboard_data`: Pushes a pointer from the stack, calls a routine, and if the result is not equal to a value from the stack, clears a word at offset 94 of a structure in a3, calls another routine, and performs several conditional calls involving a structure in a0 and a pointer in a3.
- `process_complex_structure`: Increments a long value at offset 24 of a3, increments a word at an offset in a memory location pointed to by d2, stores a3 into a2 at offset 24, and performs several conditional calls and data manipulations involving a6 and a3 before returning a signed long value from a byte at offset 31 of a2.
- `dispatch_structure_callbacks`: Calls a routine using a pointer from a2, decrements a value at offset 18 of a3, and conditionally calls several other routines using pointers from a structure, including a loop that processes data in increments of 512.
- `call_library_and_clear`: Pushes d4 and d3 onto the stack, calls a library function, and if d4 matches d0, it clears a word at offset 94 of a structure in a3, clears d4, updates a4 from a pointer, and branches to another routine; otherwise it returns via a branch.
- `clear_and_branch`: Clears a word at offset 94 of a structure in a3 and branches to another routine.
- `call_multiple_pointers`: Pushes d3 onto the stack and calls a routine, pushes d5 onto the stack and calls a routine, pushes d6 onto the stack and calls a routine, and if a long value at offset 104 of a3 is non-zero, pushes that value onto the stack and calls a routine.
- `execute_structure_callbacks`: Calls a routine with a structure in a3, and if offset 94 is non-zero and offset 90 is zero, calls three other routines and updates offsets 20 and 24 of a structure in a2 based on a value in d0, then conditionally calls two more routines after decrementing a word at a global location.
- `execute_conditional_calls`: Tests a word at offset 252 of a structure, and if it is non-zero, it pushes a pointer from a global variable to the stack, calls a library function, pushes a pointer from offset 220, calls another routine, moves a value from offset 218 into d0, cleans the stack, and branches to another routine.
- `set_bit_and_return`: Sets bit 3 of a byte at the address stored in a5 and returns with d0 set to zero.
- `process_handler_data`: Saves registers to the stack, loads a pointer from the stack into a2, writes the byte value 5 to an offset of 8 from that pointer, performs a comparison between a word at offset 28 of a2 and 12, and then either calls a routine at an offset of 966 from the program counter or calls another routine via a pointer in a2.
- `call_table_function`: Pushes a2 to the stack, calculates an offset from a word at offset 28 of a2, calls a function from a table at offset 332, calls a library function, and restores registers before returning.
- `dispatch_status_update`: Loads a pointer from offset 8 of the stack into a2, calls a routine, and if bits 7 and 6 of offset 30 of the structure in a2 are both zero, writes the byte 254 to offset 31 of that structure and conditionally calls two more routines.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_432`: Library stub: loads a library base from a global variable and calls the library function at offset minus 432 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `update_linked_list`: Loads a pointer from offset 4 of the stack into a0, stores a0 at its own address, increments the value at that address by 4, clears a long value at offset 4 of the structure, and stores a0 at offset 8 of the structure.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_exec_rawdofmt`: Saves registers a2-a4 and a6, loads pointers from the stack into a3, a0, and a1, loads a library base into a6, calls a library function at an offset from a6, and restores the registers before returning.
- `store_byte_and_increment_2`: Stores a byte from d0 at the address in a3 and increments a3.

### printer.device

- `branch_to_next_routine`: Performs a short branch to another location.
- `load_and_branch`: Loads the value negative 2 in d0 and branches to another routine before returning.
- `execute_printer_command`: Calls a routine via a pointer from a library, sets bit 7 of a byte at offset 30 of a structure in a3, and conditionally performs a linked list manipulation before calling two other library routines.
- `execute_library_call_3`: Saves registers to the stack, loads three addresses into a0, a1, and a2, calls a library function with a value in d0, and if the return value in d0 is non-zero, updates a global pointer with the value in a1, writes the value in a0 to an offset of 34 of the structure in a1, calls another library function, and returns with d0 set to 1.
- `perform_library_operations_2`: Calls several library functions using a base address from a global variable, updates multiple offsets in a structure with the results, and if a specific condition is met, enters a loop that writes a sequence of words to a memory location.
- `perform_multiple_library_calls`: Performs three consecutive sequences of pushing a pointer from a table into a1, pushing a6 to the stack, calling a library function at offset minus 414 from a library base, and popping a6; then it performs arithmetic on a1 and d0 using values from a6 and calls a library function at offset minus 210 from a library base before branching to another routine.
- `call_three_subroutines`: Calls three different subroutines in sequence and then returns.
- `load_and_branch_short`: Loads the value negative 3 in d0 and branches to another routine before returning.
- `set_status_bit_zero`: Sets bit 0 of a byte at offset 3d of a structure in a6 and returns.
- `clear_status_bit_zero`: Clears bit 0 of a byte at offset 3d of a structure in a6 and returns.
- `branch_on_pointer_test`: Loads a pointer from an offset of a6 into a1, tests the value at that pointer, and if it is zero, calls a routine and branches back to the start of this routine; otherwise it calls another routine and branches away.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `manage_library_state`: Performs multiple library calls to update various offsets in structures, including clearing a longword at offset 4 of a new structure, setting bits in d7, and conditionally clearing bit 0 at offset 2486 of a structure in a6.
- `clear_bit_conditionally`: Loads an address from an offset of a6 into a1, compares d0 to a1, and if they are equal, clears bit 1 of a byte at offset 2454 of a structure in a6 before branching to another routine.
- `set_bit_and_call_function`: Checks two bits in d5 and d2, then if both are set, sets bit 5 at offset 30 of a structure in a1, stores a1 into a location pointed to by d1, and calls a routine using a pointer found in a table at offset 42 of a6.
- `call_subroutine_via_stack`: Loads a pointer from offset 4 of the stack into a0, loads a value from offset 8 of the stack into d0, and calls a subroutine using a branch and link instruction.
- `modify_structure_and_call`: Calls a routine, and if the result in d0 is zero, sets bit 0 of offset 2486 of a structure in a6, then populates a structure in a1 with values from d0 and a0 before calling another routine.
- `adjust_stack_and_branch`: Increments the stack pointer by 8 and branches to a different location within the routine.
- `set_bit_and_branch`: Sets bit 0 of a byte at offset 190 of a structure in a6, loads the address of a location at offset 190 of a6 into a1, and branches to a different location.
- `check_byte_and_modify_d0`: Loads a pointer from a constant address, checks a byte at offset 31 of that pointer, and if it is not zero, branches to another routine; otherwise, it performs bitwise operations on a byte from offset 1554 of a6 and returns.
- `execute_library_calls_6`: Calls three library functions using a pointer from a structure in a6, sets bit 0 of a byte from a6, and clears bit 0 of a byte at offset 2486 of a6 if a comparison between d0 and offset 108 of a6 succeeds, then calls another routine.
- `compare_and_branch_on_offset`: Compares a value in d0 with an offset of 58 from a6, and if they are equal, clears bit 1 of an offset from a6, calls another routine, and branches.
- `compare_and_call_3`: Loads a longword from an offset of 176 from a6 into a1, compares it with d0, and if they are not equal, calls a subroutine and then branches based on the result in d0.
- `call_and_branch`: Calls a subroutine and then branches to a different location.
- `initialize_structure_fields`: Loads a pointer from offset 4 of a structure into a0, then writes a word to offset 28 of a structure in a1, a word from offset 32 of a0 to offset 20 of a1, and clears a long value at offset 24 of a1.
- `call_offset_routine`: Clears a byte at offset 1e of a structure in a1, then calls a routine using a pointer found at offset 14 of that same structure.
- `compare_byte_at_offset`: Loads a pointer from offset 272 of a6 and returns if the byte at offset 8 of that pointer is not equal to 7; otherwise, it returns.
- `execute_indirect_calls`: Calls a routine using a pointer at offset 14 of a structure in a1, then calls another routine using a pointer at offset 12 of a structure in a6.
- `check_byte_and_pointer`: Saves a pointer to the stack, loads a pointer into a0, reads a byte from an offset of 31 of that pointer, and if the byte is non-zero, loads a pointer from a global variable into a6, reads a byte from an offset of 982 of that pointer, performs a bitwise AND with 3, and if the result is non-zero, returns.
- `perform_printer_library_operations`: Calls a subroutine, performs several library calls using a pointer from a table, sets a bit in d1, and then conditionally clears a bit at a specific offset and calls another subroutine.
- `compare_and_set_bit`: Compares d0 with a value at offset 46 of a6, and if they are not equal, it clears bit 1 of a byte at offset 470 of a6 and calls another routine.
- `branch_on_comparison`: Compares a value in d0 with the address in a1, and if they are equal, calls another routine; otherwise it branches if d0 is not zero or branches to another location.
- `call_and_branch_2`: Calls another routine and then branches to another location.
- `branch_on_offset_check`: Saves registers to the stack, compares a byte at offset 955 of a structure in a6 to 1, and if they are not equal, branches to a different location.
- `modify_interrupt_and_call`: Sets a0 to a constant, loads a pointer from offset 4 of the stack into a2, and enables interrupts by writing to the INTENA register.
- `execute_conditional_subroutine`: Calls a routine using a pointer from a structure in d1, then checks d0; if d0 is zero, it clears bit 0 at offset 2486 of a6 and calls another routine with a pointer from offset 108 of a6; if bit 1 at offset 2486 is also set, it calls another routine with a pointer from offset 190 of a6.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `call_via_stack_pointer`: Pushes the value of a1 onto the stack twice, calls a routine using a pointer at the top of the stack, and then calls another routine using a value retrieved from the stack.
- `execute_printer_tasks`: Pushes a pointer from a1 onto the stack, clears a longword at offset 32 of the structure in a1, and calls three different routines conditionally based on the value in d0.
- `execute_printer_command_2`: Pushes the value in a1 onto the stack twice, calls a routine using a pointer from a1, and then calls another routine using a value from a1 before returning.
- `dispatch_printer_command`: As library code, it pushes two copies of a1 onto the stack, calls a subroutine via a pointer, and then calls another subroutine via a branch and link instruction.
- `process_device_request`: Calls a routine via a pointer at offset 104 of a structure in a4, then if d0 is zero, populates a structure at offset 272 of a4 with values from the stack and calls three different library routines.
- `perform_division_loop`: Performs a series of calculations involving division and subtraction using d2 and d1, and contains a loop that repeats based on a dbra instruction.
- `calculate_offset_address`: Calculates a new address by performing multiple unsigned multiplications using d0, d1, and d2, adds the results, and returns the resulting address in d0.
- `push_registers_and_call`: Pushes d2, d1, and d0 onto the stack, calls a routine, and then restores d2 from the stack before returning.
- `push_registers_and_call_modified`: Pushes d2, d1, and d0 onto the stack, calls a routine, moves the value from d1 to d0, and then restores d2 from the stack before returning.
- `process_signed_integer_and_call`: Pushes d2 onto the stack, manipulates d1 and d0 based on their signs, and if d1 is not zero, calls a routine and potentially restores d2 before returning.
- `handle_signed_values_and_call`: Pushes d2 onto the stack, performs sign-based manipulations on d0 and d1, and if certain conditions are met, calls a routine and restores d2 before returning.
- `call_subroutine_with_parameters`: Links a6 and saves registers, then calls a subroutine at offset 936 using a2, a3, and a4 as arguments.
- `clear_table_entry`: Compares a longword at an offset of a2 to negative one, and if equal, calculates an address using a value from a2 and an offset, checks a byte at that address, and clears two longwords if the byte is zero.
- `store_pointer_and_clear_word`: Stores a longword from a3 into a global address and clears a word at a global address.
- `transfer_printer_buffer_to_device`: Iterates through a buffer to copy bytes into a destination table using an index, then calls another routine to process the data and subsequently calls two more routines based on the results of the first call and a byte check in a global structure.
- `zero_registers_and_branch`: Clears registers d4, d6, and d2 to zero and branches to another routine.
- `process_table_data`: Performs a loop that compares a byte from a table at an offset of a6 with d3, calls a routine with a pointer to a table element, calls another routine, and conditionally calls a third routine based on comparisons and register values.
- `copy_table_data_loop`: Copies bytes from one table to another in a loop based on a comparison with a global value, then either branches to another routine or stores d5 at offset 32 of a structure in a2.
- `clear_and_restore_registers_2`: Clears a longword at offset 32 from a pointer in a2, moves a byte from d2 into d0, and restores registers d2 through d6 and a2 through a4 from the stack before returning.
- `process_printer_byte`: Calls another routine, performs comparisons on a byte from a3, and conditionally calls another routine before branching to another location.
- `clear_and_branch_3`: Clears a longword at a negative offset of a6 and branches to another location.
- `update_byte_with_offset`: Compares a byte from a3 with a byte from a table at an offset determined by a word from a3, and if they match and a global longword is zero, adds 55 to a byte from a4 and stores it at the address in a2.
- `compare_and_set_byte`: Compares a value in d0 to 1 and 5, and if d0 is 5, writes a byte value of 255 to the location pointed to by a2, otherwise it returns.
- `write_conditional_byte`: Compares a value in d0 to 2, and if it is not equal, compares d0 to 12, and if d0 is 12, writes the byte value 75 to a location pointed to by a2 before branching.
- `set_conditional_byte`: Compares d5 to 9 and checks a byte at a4; if d5 is not 9 and the byte at a4 is not zero, it branches, otherwise it stores a byte into the location pointed to by a2 and branches.
- `copy_byte_from_table`: Compares a longword in d5 to 6, and if it is greater than or equal to 6, it uses the value in d5 as an offset to copy a byte from a table at address 266 to the location pointed to by a2.
- `check_byte_and_store`: Checks if d5 is non-zero and if a byte at a pointer in a4 is at least 29; if either condition fails, it branches, otherwise it stores the value 12 at a pointer in a2 and branches.
- `update_printer_byte`: Compares a byte from a4 with a byte from a table at an offset calculated from d5, and if they match, it extends a byte from a table into a longword, adds d6 to it, and writes the result to a2, potentially calling another routine and entering a loop that increments d5 and d6.
- `clear_printer_memory_tables`: Clears several longwords at specific offsets in various memory locations and tables, then branches to another routine.
- `process_printer_command_sequence`: Fetches a byte from a table using an incrementing index, compares it against a value, and calls another routine to execute commands.
- `handle_printer_sequence`: It compares a byte at a pointer in a3 to 40, and if equal, calls another routine, updates a byte at a4, and enters a loop that compares a value from a table against a register and branches back to the start of the loop.
- `compare_printer_byte`: Compares a byte at a pointer in a3 to 91; if they are not equal, it branches to another routine, otherwise it branches to another routine.
- `process_printer_sequence`: Compares a byte from a table at an offset in d0 with a byte at a pointer in a3, and if they match, it copies a byte from another table into a4, clears three bytes in a global structure, and calls another routine.
- `check_printer_status_2`: Compares a byte at a pointer in a3 to 10; if they are not equal, it checks a word at offset 104 and branches if it is not zero; otherwise, it writes 3 to a byte at a pointer in a4 and branches to another routine.
- `write_printer_data`: Writes a byte to a4, calls another routine using a pointer from a global structure, and if a byte at offset 4 of a global structure is zero, it performs arithmetic on a longword from a2 and updates two global longwords.
- `write_two_bytes_to_table`: Writes the value 10 to a byte at a calculated offset in a table, increments the table pointer, and then writes the value 13 to a second calculated offset in the table before branching.
- `write_single_byte_to_table`: Writes the value 10 to a byte at a calculated offset in a table, increments the table pointer, and then branches.
- `handle_printer_byte`: Compares a byte at a pointer in a3 to 155; if equal, it pushes two longwords onto the stack, calls a routine, and branches.
- `calculate_pointer_offset_2`: Compares a byte at a3 with 160, and if it is not less, performs several checks on a structure in a0 and a structure in a pointer from a global variable, eventually calculates an address using d1 and d4 and loads a longword into a1.
- `check_ascii_range`: Compares a byte at a pointer in a1 with specific character ranges and increments a counter in d5 if certain conditions are met, then branches to another routine.
- `increment_and_update_pointer`: Increments a pointer in a1 and a value at a global location, then checks if a value at a pointer in a0 is greater than 256; if so, it updates a global pointer with a value from another location, otherwise it returns.
- `calculate_target_pointer`: Loads a byte from a3 into d1, subtracts 160 from d1, shifts d1 left by 2, adds d4 to the result to form a pointer in a0, loads a pointer from that address into a1, and branches.
- `skip_non_numeric_characters`: Checks if a byte at a1 is 92, and if not, increments a1 and checks if the new byte is between 48 and 57 inclusive, otherwise it increments a1 again and branches.
- `convert_ascii_digit`: Checks if a byte at a1 is a numeric character, performs arithmetic to convert it to a value, and writes the result to a destination at a0 calculated using a2 and d2.
- `move_byte_to_offset`: Calculates a destination address by adding a2 and d2 to a value from a pointer, moves a byte from a1 to that destination, and then branches based on the next byte in a1.
- `process_byte_and_check_bounds`: Loads a longword from a2, increments a2, fetches a byte from a3 at an offset determined by a2 and d2, compares d3 to a global longword, and performs several conditional branches based on comparisons with other global longwords.
- `subtract_and_clear_pointers`: Performs subtractions using d3 and values from two different global pointers, clears a long integer at an offset of a global pointer, and moves a value from a2 into d0.
- `call_and_extend_byte`: Clears a global longword, extends a byte from a6 to a word in d1, pushes a pointer from a6 and the word in d0 onto the stack, and calls another routine.
- `update_pointer_and_branch`: Adds d3 to a longword from a global variable at offset 124, and if the result is greater than 256, it stores d3 in a global variable at offset 128, checks a byte at offset 27 of a6, and either calls a routine at a specific address or branches.
- `update_global_pointer`: Decrements a longword from a global variable, stores the result in another global variable, and branches to another routine.
- `set_d4_and_branch`: Sets d4 to zero and branches to another routine.
- `copy_byte_loop`: Copies bytes from a table to a destination address in a loop, where the destination address is incremented by a value in d2 and the source index is incremented by 1 each iteration until d4 reaches d3.
- `clear_byte_and_branch`: Saves registers to the stack, loads three values from the stack into d3, d2, and a2, clears the byte at the address in a2, and branches to another routine.
- `clear_byte_at_offset`: Compares d1 to 59, and if it is not 59, increments a3, sets a0 to a3 plus a2, and clears the byte at that location before branching.
- `set_flag_and_store_pointer`: Compares d1 to 32, and if it is equal, it loads a pointer from offset 32 of the stack into a0 and stores the value 1 at the location pointed to by a0.
- `check_d1_and_load`: Compares d1 to 34 and, if equal, loads a pointer from a stack offset into a0 and branches to another routine.
- `update_and_call_handler`: Calculates an address by adding a2 and a3, computes a value by subtracting 48 from d1 and adding the result of a byte at an offset of a2 and a3 multiplied by 10, stores this value at the calculated address, pushes d2 and d3 onto the stack, calls a subroutine, and then performs several comparisons on the result in d0 before storing a3 into a global variable at an offset of 288.
- `increment_counter_and_call_handler`: Increments a longword at a pointer from the stack, fetches a byte from a table using the new value as an offset, compares a longword from the stack to a global variable, and if it is less than 256, calls a routine at an address stored in a pointer.
- `transfer_byte`: Moves a byte from d0 to d1, clears d0, and then moves the byte from d1 back into d0 before returning.
- `update_pointer_table_2`: Sets a1 to a constant, decrements a longword from a global variable, and stores the result in another global variable before branching to another routine.
- `validate_and_set_flag`: Decrements a pointer in a1 and, if the resulting value is not less than or equal to zero, checks if a byte from a table at a0 matches a specific pattern; if it does, it sets a byte at a specific offset in a global variable and returns the pointer in a1.
- `clear_bytes_and_call_subroutines`: Clears three bytes at specific offsets, clears a byte at offset 2720 of a structure in a0, sets a byte at offset 58 of a structure in a0 to 1, and conditionally calls two different routines based on the results of a test on d1 and a pointer in d3.
- `copy_table_values`: Copies six 16-bit values from offsets 2646 through 2666 of a structure pointed to by a pointer in a global variable into a table starting at offset 0 of d3.
- `compare_table_values`: Compares six 16-bit values from a structure against a table in d3, returning zero if all match or negative one if any mismatch occurs.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `process_device_data`: Stores four bytes from a pointer in a0 onto the stack, pushes a word from offset 32 of a0 and a value from offset 260 of a6 onto the stack, and calls another routine via a pointer.
- `handle_device_requests`: If d2 is zero, calls a routine with a constant on the stack, and if the result in d1 is non-zero, calls another routine; it then conditionally calls a third routine if both d1 and d2 are zero.
- `call_library_function_9`: Saves registers, loads a pointer from a structure in a2, pushes several values onto the stack, and calls a routine via a pointer found at an offset from a global variable.
- `update_status_byte_2`: Compares a word at offset 22 of the stack to 54 and 53, and if the word is 53, clears d1, sets it to the value at offset 22 of the stack, performs a bitwise AND with 1, stores the result in a global variable at offset 262, and returns.
- `calculate_address_offset`: Compares a word at offset 22 of the stack to 51 and a byte at a global location to 1, then if both match, calculates a pointer by shifting a value from the stack and adding an offset from a structure in a1.
- `compare_and_set_d1`: Compares the byte in d1 to 255; if it is not equal, it branches to another location, otherwise it sets d1 to 255 and branches to another location.
- `loop_and_copy_bytes`: Compares d1 to 254, and if equal, clears d1 and enters a loop that increments d3, calculates addresses using d2 and d4, and moves bytes between memory locations until a condition is met.
- `dispatch_jump_table`: Compares a value in d2 against a constant, and if it is not less than seven, uses a value from a table to perform a jump.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `update_printer_status_2`: Copies several values from a structure in a6 to specific memory locations and calls a routine using a pointer from a structure, then conditionally clears a word at a specific location.
- `update_printer_status`: Writes a word to a location, pushes two values onto the stack, calls a routine using a pointer from a structure, and branches based on a bit test in a structure.
- `check_printer_status_and_scale`: Checks bits in a structure in a2 to determine status, updates several bytes in a structure using values from a table, and scales a word from a structure by eight to store it in a global variable.
- `calculate_printer_offsets`: Calculates two values by subtracting a word at offset 16 of a structure in a1 from a word at offset 20 of the same structure, and stores the results into locations pointed to by d7.
- `process_printer_scaling`: Loads a value from a structure in a2, scales it, and stores it in a global variable; it then performs several comparisons of values from a structure and calls a routine via a pointer if certain conditions are met.
- `calculate_printer_scaling_offset`: Calculates a value by subtracting two words from a structure in a2, calls a subroutine via a pointer, and then compares the result to zero.
- `compare_and_branch_word_2`: Compares a word in d0 to 32768 and branches to one of two locations based on the result.
- `branch_with_constant_4`: Loads the constant 12 into d1 and branches to another location.
- `branch_with_constant_alternative`: Loads the constant 15 into d1 and branches to another location.
- `update_printer_scaling`: Calls a subroutine with d3 in d0, then compares a longword from a structure in a2 to d3, and if d3 is smaller, performs a multiplication using a word from a structure in a1 and stores the result in d3.
- `handle_printer_data`: Calls a subroutine, compares a longword from a structure in a2 to d3, and if d3 is larger, performs several operations including multiplying a word from a2 by 10 and calling subroutines.
- `update_device_pointer`: Sets d6 to 6 and d0 to 7, then stores d0 at offset -4 of a6.
- `prepare_device_call`: Clears a byte at offset 460 of a global variable, checks if a byte at offset 350 is zero, and if not, checks if a byte at offset 358 is zero; if both are non-zero, it clears the stack and pushes a longword from a global variable.
- `update_device_status_and_counters`: Performs several arithmetic operations on words stored at various offsets to update counters, moves a longword from a structure in a4 to another location, and performs bitwise operations on a word at a specific offset before branching.
- `update_printer_status_and_scale_values`: Calculates two sums from 16-bit values stored in a table and writes them to specific memory locations, then updates a signed value in a structure and conditionally scales it using two different offsets from a structure in a2 via calls to calculate_offset_address and handle_signed_values_and_call.
- `scale_and_update_printer_data`: Checks bit 4 of a value at a pointer in d7, performs bitwise shifts on values in a4, calls a routine using a pointer in a6, and then conditionally moves a value from offset -14 of a6 into a location pointed to by d4.
- `update_printer_scaling_and_bounds`: Checks a bit in a global variable and, if set, performs bitwise shifts on values stored in a table and calls several routines to perform arithmetic comparisons and updates a structure's fields based on the results.
- `execute_printer_calculations`: Calls several routines to perform arithmetic, compares results, and if conditions are met, accesses a structure via a5 and compares its members with values in a4 and d4.
- `compare_and_copy_words`: Loads a word from a location pointed to by a register, compares it with a value in a0, and if not equal, performs several word moves between memory locations pointed to by d4 and other registers.
- `calculate_printer_difference`: Copies a word from a global variable to another, calculates the difference between two words, stores the result in a global variable, and performs conditional logic involving a bit shift and negation before branching.
- `update_printer_status_table_2`: Copies a word from a pointer in a global variable to another global variable, calculates the difference between two words, stores the result in a global variable, and then enters a loop that clears a byte at an offset of a table based on a counter.
- `update_printer_status_table`: Iterates through a loop to extract bit-field values from a word and stores them into a table, then performs conditional logic to update a second table with processed data or calls another routine to handle specific data values.
- `execute_printer_command_3`: Writes values to memory locations pointed to by d7, loads a word from an offset of a structure in d7 into d0, and calls a subroutine using a pointer from a2.
- `call_library_function_10`: Pushes a longword onto the stack, clears three longwords on the stack, loads a pointer from a global variable into a2, calls the routine at that pointer, and stores the result in a global variable.
- `test_and_branch_printer`: Tests bytes at offsets 156 and 158 of a global variable; if both are zero, it branches to another location, otherwise it calls another routine and branches to another location.
- `test_and_call_printer_routine`: Tests a byte at a global offset, calls a routine if the byte is non-zero, and then branches to another location.
- `test_and_call_routine`: Tests a byte at a global offset, and if non-zero, calls a routine and branches to another location.
- `update_global_pointer_2`: Calls another routine, stores the result in a global word variable, and branches to another instruction if that variable is non-zero.
- `divide_and_call_subroutine`: Loads a word from a global variable into d2, performs a division using a value at offset 24 of a structure in a2, and if the result is non-zero, calls a subroutine using a pointer from a global variable and stores the result in a global variable.
- `call_and_restore_registers`: Clears two 32-bit values on the stack, calls a routine, and then restores d2 through d6 and a2 through a5 from the stack before returning.
- `calculate_weighted_sum`: Checks a byte at offset 256 of d7, and if set, performs a series of multiplications and additions on bytes from the stack, then compares a value in a table to 1 and performs further arithmetic on d0 and d1.
- `compare_and_store_byte`: Checks bit 2 of a value at a pointer in a global variable; if set, it compares bytes from offsets 8 and 9 of the stack to offsets 9 and 10 of the stack, and if they match, stores a byte from offset 10 of the stack into offset 11 of the stack, then returns.
- `calculate_printer_values`: Performs arithmetic using values from a table, including a multiplication, and stores the result in a longword on the stack before branching to another location.
- `calculate_and_store_table_address`: Performs arithmetic using values from a table pointed to by d7, calls a routine, and if the result is not equal to 65535, stores a value from a table into a location pointed to by a2.
- `calculate_table_offset`: Tests a byte at a pointer in d7, and if it is non-zero, it calculates an offset using d3, performs bitwise operations on d3, and loads a word from a table into a2 using an offset derived from d3.
- `modify_memory_and_register`: Stores a longword from an offset into the address in a2, checks bit 3 of a byte at offset 21 of a structure in a0, and if bit 3 is not set, subtracts d2 from 15 and stores the result in d1.
- `shift_and_compare_values`: Moves d2 to d1, moves d3 to d0, performs a logical shift right on d0, and branches based on the result of a comparison with one.
- `branch_on_value`: Compares a word in d0 to 2, and if it is not equal, branches to another location; otherwise, it branches to another location.
- `decrement_and_branch_3`: Decrements a word in d0 and branches to different locations depending on whether the result is non-zero.
- `store_byte_and_branch`: Stores a byte from d1 into a memory location and then branches to another routine.
- `store_byte_and_branch_2`: Stores a byte from d1 into a different memory location and then branches to another routine.
- `call_and_store_pointer`: Moves a byte from d1 to a location at offset 433, pushes a pointer from a2 onto the stack, calls a subroutine, stores the result in the location pointed to by a2, and branches to a different location.
- `update_table_values`: Calculates an offset using d3, loads a word from a table into a2, and then adds four different bytes from a global structure to four specific long integer locations in that structure.
- `calculate_and_update_tables`: Calculates a value by multiplying a word from a table by a second word from another table, stores it in a third table, and then performs four successive operations where it shifts a value, adds it to a global variable, and calls another routine to update that variable.
- `execute_nested_callbacks`: Clears a byte at a global offset, calls a routine with two values from a3 and a4 pushed onto the stack, and if the result is non-zero, calls another routine using a pointer from a global variable.
- `dispatch_nested_calls`: Checks two bytes at specific offsets in d7, and if both are zero, pushes two words from a3 and a4 onto the stack, pushes a pointer from a global variable onto the stack, and calls a subroutine; if the result is non-zero, it pushes the words from a3 and a4 again, pushes a value onto the stack, loads a pointer from a global variable into a0, calls that pointer, and stores the result in a location pointed to by a2.
- `call_with_parameters`: Pushes two words from a3 and a4 onto the stack, calls a routine with a pointer from a global variable, and if the result is non-zero, pushes two more words and calls a second routine using a pointer from a global variable.
- `push_and_call_twice`: Pushes two words from a3 and a4 onto the stack, calls a subroutine using a pointer from a global variable, and if the result is non-zero, performs a second sequence of pushing values and calling a subroutine using a pointer from a global variable.
- `clear_and_restore_registers`: Sets d0 to zero and restores a2, a3, and a4 from the stack before returning.
- `increment_and_conditional_call`: Increments a word at a global address, performs a division using a value at offset 24 of a structure pointed to by a value from that global address, and if the result is zero, calls a subroutine using a pointer from a global variable and stores the result in a location pointed to by a2.
- `call_and_store_result`: Pushes the address of a constant onto the stack, clears three 32-bit values on the stack, calls a routine using a pointer from offset 494 of a0, stores the result in d0 into the location pointed to by a2, and then branches based on a condition.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `setup_printer_pointers`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a3, and a4, moves a word from a global variable to an offset of a4, and branches to another instruction.
- `push_printer_parameters`: Pushes two words from global memory onto the stack, calls another routine, and then stores the result in a location pointed to by a3 before branching.
- `update_printer_state`: Pushes two words from memory onto the stack, calls another routine, and then writes the result to several memory locations and clears a word at a global offset.
- `clear_long_and_branch`: Clears a longword at a specific memory location and branches to another routine.
- `update_printer_counters`: Calls a routine, increments a counter in a global variable, and if a comparison with another global value passes, calls two more routines and stores a longword in a structure.
- `calculate_printer_increment`: It multiplies a byte from a global pointer by a word from another global pointer, adds the result to a third global value, increments a fourth global value, and then calls another routine if a comparison between two global values succeeds.
- `increment_and_check_limit`: Adds a word from a structure in a3 to a word at a location in a4, compares the new value in a4 to a constant, and if it is not lower, clears d0 and returns.
- `clear_and_copy_printer_data`: Saves registers a2 through a4 to the stack, loads three constant addresses into a2, a3, and a4, clears a global word variable, copies a word from a global variable to an offset of a4, and branches to another instruction.
- `update_printer_status_and_pointers`: Pushes two 16-bit values onto the stack, calls another routine, stores the result in a word at a pointer in a3, clears a global word, and updates three global pointers or values using data from the stack and other global locations.
- `clear_printer_table`: Clears four longwords at offsets 11c, 120, 124, and 128 of a global structure and clears a word at offset 154, then branches to another routine.
- `increment_printer_table`: Calls a routine, increments a counter in a global variable, and if a comparison passes, calls two more routines and another routine with two words pushed onto the stack.
- `call_and_store_result_2`: Calls another routine, stores the result in d0 at the address in a2, and if d0 is zero, loads d0 from the address in a2 before branching.
- `process_printer_decrement`: It increments a global word, decrements another global word, and if that word is not greater than zero, performs a multiplication and addition before potentially calling another routine and branching.
- `increment_printer_buffer_counter`: Adds a 16-bit value from a pointer in a3 to a 16-bit value at a pointer in a4, then compares the result against a global constant.
- `initialize_printer_buffer_pointers`: Clears a 16-bit value at a global offset, copies a 16-bit word from a global location to a pointer at offset 320, and branches to another routine.
- `push_arguments_and_call`: Pushes two words from global memory onto the stack and calls another routine, then stores the result in a location pointed to by a3 and branches.
- `copy_global_values`: Clears a word at a global offset, copies several longwords and a word between various global locations, and then branches to another routine.
- `update_printer_state_2`: Pushes two words from a structure onto the stack, calls another routine, stores the result in four longwords at offsets 11c, 120, 124, and 128 of a global structure, clears a word at offset 13c, and branches to another routine.
- `loop_printer_updates`: Calls a routine, increments a counter in a global variable, and if a comparison passes, calls two more routines and stores a longword in a structure.
- `execute_printer_update`: It performs a multiplication of a byte and a word from global pointers, adds the result to a global value, increments a counter, and calls another routine if a comparison succeeds.
- `decrement_and_check_limit`: Decrements a word at a location in a3, increments a word at a location in a4, compares the value in a4 to a constant, and if it is not lower, clears d0 and returns.
- `initialize_printer_counters`: Saves registers a2 through a4 to the stack, sets a2, a3, and a4 to specific constants, clears d0, writes d0 to two global word locations, and writes a word from a location in a4 to another location.
- `push_arguments_and_call_alt`: Pushes two words from global memory onto the stack and calls another routine, then stores the result in a location pointed to by a3 and branches to a different location.
- `copy_printer_parameters`: It clears a word at a global offset, copies several longwords and a word between various global locations, and then branches to another routine.
- `reset_printer_values`: Clears four longwords in a global structure, calls a routine, and then pushes two words onto the stack before calling another routine and storing a word in a global variable.
- `update_pointer_from_call`: Calls another routine, stores the returned value into a location pointed to by a2, and if the value is zero, branches to another routine; otherwise, it reloads the value into d0 and branches to another routine.
- `modify_printer_counters`: Increments a word at offset 142, decrements a word at offset 13e, and if that word is not greater than zero, adds a sign-extended byte from offset 15a to a word at offset 13a.
- `decrement_and_check_counter`: Decrements a word at a location pointed to by a3, increments a word at a location pointed to by a4, and if the value in a4 is greater than or equal to a global word, it restores registers and returns.
- `modify_table_entry`: Adds a value derived from a word at offset 14 of the stack to a long integer at a pointer from offset 20 of the stack, then subtracts a value derived from a word at offset 18 of the stack from that same long integer if the first addition resulted in a non-negative value.
- `compare_masked_values`: Saves registers to the stack, loads several values from the stack, and performs a comparison between a value in d1 and a calculated value derived from d2 and a byte at a global location, then branches.
- `compare_masked_bytes`: Performs bitwise AND operations on d2 and d4 with a mask derived from d1 and d5, compares a byte at a calculated offset in a global structure with d3, and returns a signed long integer based on the comparison result.
- `compare_and_call`: Pushes d2 onto the stack, calls a routine with a value from the stack, and if the returned value is less than 4, adds 10 to a value and calls another routine.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `initialize_printer_structure`: Clears a longword at offset 58 of a structure in a2, pushes two values onto the stack, calls another routine, and then performs conditional branches based on a word at offset 2614 of a structure in a2.
- `fetch_table_pointer`: Shifts d3 left by two, uses the result to index into a table at address 14, loads the resulting pointer into a2 and d5, then branches.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `branch_on_byte`: Tests a byte at a location pointed to by a2 and branches to another routine if it is non-zero, otherwise it branches to a different location.
- `set_pointer_constant`: It loads the constant value of 32 into a4 and branches to another routine.
- `process_byte_sequence`: Increments a2, sign-extends bytes from a4 and a2 into longwords, pushes them onto the stack, calls another routine, and if the result is not zero, loops by checking bytes in a4.
- `manage_device_locking`: Checks a value in a structure, calls a routine to clear it, then performs a sequence of library calls including loading and unloading segments, and finally loops to copy bytes from one location to another.
- `call_dos_functions`: Calls two library routines using d6 and d4 as arguments and returns a constant value of negative one in d0.
- `execute_unloader`: Checks a value at an offset of a pointer in a global variable; if non-zero, it calls a routine at an offset of a second pointer and then calls another routine using a value from the first pointer as an argument.
- `compare_ascii_characters`: Compares two bytes from the stack by converting them to lowercase if they are within the range of ASCII letters, and returns one if they are equal or zero otherwise.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `call_lock_2`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_2`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_current_dir_2`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### generic

- `return_constant_7`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_8`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_9`: Returns the constant an address in d0 to the caller without any other effect.
- `clear_registers`: Loads three values from the stack into d0 and then clears d0 before returning.

### Epson

- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_library_pointers`: Stores a pointer from the stack into a destination, loads a library base into a6, and performs three consecutive calls to routines via a6, storing each return value in a destination table and branching if a return value is zero.
- `call_library_functions_4`: Loads two different pointers from global variables and calls a routine at each pointer using a6 as a base, then sets d0 to 255 and branches to another routine.
- `call_library_functions`: Calls a library function three times using different pointers retrieved from a structure in d1, saving and restoring a6 on the stack for each call.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_handler_sequence`: Calls a function from a table, and if the return value in d0 is zero, it populates a structure at an offset from a4 and performs a sequence of four library calls using pointers from a table.
- `dispatch_via_table_2`: Saves registers to the stack, loads several values from the stack, and performs a jump to an address retrieved from a table based on a value in d0.
- `return_zero_2`: Clears d2 and moves the zero value into d0 before returning.
- `initialize_printer_parameters`: Loads several values from the stack into a1, a3, and a4, sets d2, d3, d4, and d5 to specific constants, and branches to another location.
- `process_printer_data_loop`: Copies bytes from a table into a destination buffer using a loop that runs up to 18 times, then performs several conditional checks on a structure pointed to by d1 to write specific bytes to a location at offset 48 of the stack.
- `process_printer_data`: Checks if a word at offset 2646 of a structure pointed to by a pointer in d1 is 2048; if so, it increments a value in d3, stores a byte at a calculated address in a2, copies bytes from two different locations to a destination pointed to by a stack pointer, writes 65 to a1, and if a byte at offset 2720 of a structure in a0 is non-zero, clears a byte at a0.
- `copy_bytes_from_stack`: Loads a pointer from the stack, moves a byte from that location into d0, decrements d0, and stores it in a1; then it loads a second pointer from the stack, increments it, and moves the byte from that location into the location pointed to by a0.
- `copy_byte_from_table_2`: Copies a byte from a table at offset zero into a destination address calculated from a pointer in a6 and a value in d3, then enters a loop that continues while d5 is greater than 8.
- `write_printer_control_bytes`: Compares a word at a1 with 66, and if equal, writes a byte to an offset of a structure in a0, clears a byte at a1, and if a word at offset 2646 of a structure in a0 is 2048, writes a byte to the address in a0.
- `check_printer_status`: Loads a pointer from a memory location, compares a word at offset 104 of that pointer to 1024, and if equal, stores the byte 137 at the address in a0, otherwise it branches.
- `store_constant_byte`: Moves the value in d2 to a0 and stores the byte 80 at the location pointed to by a0, then branches.
- `update_printer_data_table`: Increments two counters and copies a byte from a table at offset zero to a destination address calculated from a pointer and an offset, then repeats this process if the counter is less than eight; otherwise it returns.
- `update_control_byte_2`: Compares a word at the address in a1 to 32, and if equal, checks a byte at a3; if that byte is zero, it writes 1 to a3 and 27 to the word at a1, otherwise it returns.
- `update_byte_on_condition`: Checks a byte at the address in a3; if it is non-negative, it clears that byte, otherwise it writes the word 30 to the address in a1 and branches to another routine.
- `set_d0_and_branch_2`: Sets d0 to 255 and branches to another routine.
- `update_control_byte`: Compares a word at the address in a1 to 33; if equal, it tests a byte in a3, and if that byte is zero, writes 255 to a3 and 29 to the word at a1, otherwise it returns.
- `update_byte_on_condition_2`: Tests a byte in a3; if it is less than or equal to zero, it clears the byte in a3 and writes 28 to the word at the address in a1, otherwise it branches to another routine.
- `branch_to_offset`: Performs a short branch to a relative offset.
- `process_control_codes`: Compares a word at a1 against several constants to set a byte at a3 to specific values, and if a1 matches 2, performs a series of writes to a destination calculated from a2 and a4.
- `write_byte_on_condition`: Checks a word at the address in a1; if it is non-zero, it branches; otherwise, it loads a pointer from a table in d1 and writes the byte 253 to an offset of 2720 from that pointer.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf:Preferences

- `initialize_preferences`: Calls several library functions to manage files and tasks, processes a string by copying specific characters into a new buffer, and performs multiple calls to other routines using values from a structure.
- `execute_library_operations`: Loads a pointer from a stack offset into a6, then performs several conditional calls to library functions using values from a table, including a call with a specific constant in d7, before returning.
- `call_library_functions_2`: Calls two different library functions at offsets minus 180 and minus 174 from a base address in a6, using a value from offset 92 of a4 in a0.
- `call_library_function_and_store_result`: Loads a pointer into a1, clears d0, calls a function via a pointer relative to a6, stores the return value in a global variable at offset 8, and branches to another routine if the result is zero; otherwise it returns.
- `initialize_system_preferences`: Saves registers, calls several routines using pointers from a3 and a4, performs a loop that copies bytes from one table to another, checks and updates longwords at offsets 20 and 28 of a global structure, and calls several routines to manage memory and ports.
- `initialize_preference_structure`: Performs a complex sequence of library calls, copies data between tables using loops, clears several words and longwords in a global structure, and sets various configuration values including bitwise flags and integer constants.
- `dispatch_preference_data`: Executes a series of conditional subroutine calls, bitwise operations on a global variable, and table-driven jumps to manage system resources and preference settings.
- `release_resource_structure_2`: Pushes several long words onto the stack, calls five different routines, and then restores the stack pointer before returning.
- `call_subroutine_twice`: Pushes values onto the stack and calls another routine twice with different parameters, once with a single argument and once with two arguments, before returning a value from the stack.
- `process_data_in_loops`: Iterates through two different sequences of data using a loop, calling another routine with values from a structure and a pointer, and uses different offsets for the second sequence based on a comparison.
- `extract_nibbles_and_call_routine`: Checks if a 16-bit value is greater than 4, then extracts high and low nibbles from a 16-bit word to use as arguments for a call to another routine.
- `update_preference_table`: Calculates an index based on a value in d3, updates a table at a pointer in a2 with various data from other tables, and clears several longwords in a global variable if a counter reaches 112.
- `dispatch_via_table_jump`: Retrieves a value from a table, uses it to perform a jump to a calculated address, and otherwise performs a single call to another routine.
- `process_preference_data_2`: Pushes a pointer to the stack, performs several conditional library calls based on values in a structure, and executes bitwise operations and arithmetic before returning a pointer in a2.
- `update_semaphore_and_table_values`: Calls three different routines to manage state, then performs a series of additions and subtractions using values from multiple tables to update a series of memory locations.
- `process_preference_structure`: Performs multiple calculations involving bit shifts and logical operations on values from a structure, calls several subroutines with various arguments from a table, and performs arithmetic on values from a2 to determine final results before returning.
- `clamp_and_update_structure_values`: Clamps two byte values to a specific range, checks a flag, and if set, updates 16-bit values in a structure using offsets and then calls two different routines.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_66_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_library_vector_162_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_276_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_324_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_forbid_2`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem_3`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem_2`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_add_task`: Library stub: loads its arguments and the exec library base, calls AddTask and returns its result to the caller.
- `call_rem_task`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `call_find_task_3`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_wait`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_signal`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_alloc_signal_2`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal_2`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_add_port_2`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `call_rem_port_2`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `call_get_msg_3`: Library stub: loads its arguments and the exec library base, calls GetMsg and returns its result to the caller.
- `call_reply_msg`: Library stub: loads its arguments and the exec library base, calls ReplyMsg and returns its result to the caller.
- `call_find_port`: Library stub: loads its arguments and the exec library base, calls FindPort and returns its result to the caller.
- `call_close_library_3`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_send_i_o`: Library stub: loads its arguments and the exec library base, calls SendIO and returns its result to the caller.
- `call_abort_i_o`: Library stub: loads its arguments and the exec library base, calls AbortIO and returns its result to the caller.
- `call_open_library_3`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_init_semaphore`: Library stub: loads its arguments and the exec library base, calls InitSemaphore and returns its result to the caller.
- `call_obtain_semaphore`: Library stub: loads its arguments and the exec library base, calls ObtainSemaphore and returns its result to the caller.
- `call_release_semaphore`: Library stub: loads its arguments and the exec library base, calls ReleaseSemaphore and returns its result to the caller.
- `call_attempt_semaphore`: Library stub: loads its arguments and the exec library base, calls AttemptSemaphore and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_init_code`: Library stub: loads its arguments and the exec library base, calls InitCode and returns its result to the caller.
- `call_init_struct`: Library stub: loads its arguments and the exec library base, calls InitStruct and returns its result to the caller.
- `call_deallocate`: Library stub: loads its arguments and the exec library base, calls Deallocate and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_add_tail_2`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_rem_task_2`: Library stub: loads its arguments and the exec library base, calls RemTask and returns its result to the caller.
- `call_set_signal`: Library stub: loads its arguments and the exec library base, calls SetSignal and returns its result to the caller.
- `call_wait_2`: Library stub: loads its arguments and the exec library base, calls Wait and returns its result to the caller.
- `call_signal_2`: Library stub: loads its arguments and the exec library base, calls Signal and returns its result to the caller.
- `call_alloc_trap`: Library stub: loads its arguments and the exec library base, calls AllocTrap and returns its result to the caller.
- `call_free_trap`: Library stub: loads its arguments and the exec library base, calls FreeTrap and returns its result to the caller.
- `call_add_port_3`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `calculate_and_store_offset_2`: Calculates a value based on three 16-bit words from the stack, stores the result in a memory location, and calls another routine with several arguments from the stack.
- `process_multiple_structures`: Calculates values based on a stack parameter, stores them in global variables, and then performs three separate sequences of pushing structure members to the stack, calling a routine, and performing bitwise operations before returning.
- `process_preference_tables`: Saves registers, performs a series of calls to other routines using values derived from multiple tables and offsets, and includes a loop that decrements a value in d3 and branches back to an earlier instruction.
- `calculate_pointer_offset`: Loads a pointer from offset 4 of the stack into a0 and a1, then iterates through a memory range to find the distance between a1 and the end of a sequence of non-zero bytes, returning that distance in d0.
- `call_library_vector`: Pushes several values onto the stack, including a value from offset 4 of the stack and the constant minus 1, then calls a library function at offset 296 of a base address loaded from offset 8 of the stack.
- `perform_complex_math_and_calls`: Performs a series of arithmetic operations involving multiplication and bit shifts on values from the stack, and calls several subroutines using a pointer in a2 and various constants passed via the stack.
- `process_bitfield_math`: Extracts values from the stack and performs multiple conditional branches based on bit tests in d4, executing various arithmetic sequences involving bit shifts and subtractions before calling subroutines with a pointer in a2.
- `compare_and_return_constant`: Compares a 16-bit value from offset 6 of the stack to 9; if equal, it returns 0, otherwise it returns 8.
- `decrement_and_compare`: Decrements a 16-bit value from offset 6 of the stack by 1; if the result is not zero, it branches to another location, otherwise it returns 0 or 16 based on the result.
- `update_preference_structure`: Retrieves three 16-bit values from the stack, uses them to perform a series of arithmetic operations including additions and subtractions, and calls several subroutines to update a structure pointed to by a global variable.
- `push_values_to_stack`: Checks if a word at offset 6 of the stack is non-zero; if so, it pushes four specific constant values onto the stack and branches to another routine.
- `update_preference_structure_2`: Pushes three 16-bit values onto the stack, calls a routine, and then restores the stack pointer before returning.
- `calculate_offset_address_3`: Performs several arithmetic operations and divisions on a 16-bit value from the stack, writes bytes to a destination pointed to by a structure, and calls another routine multiple times.
- `perform_division_loop_2`: If a longword is not negative, it performs a series of divisions and swaps; otherwise, it enters a loop that performs 16 iterations of addition and subtraction to update a value and a counter.
- `calculate_offset_address_2`: Performs multiple signed multiplications and swaps on registers to calculate a value, adds this value to a base pointer, and returns the resulting address in d0.
- `swap_and_call`: Pushes d2 to the stack, swaps d0 and d1, calls another routine, and then restores d2 before returning.
- `swap_and_call_variant`: Pushes d2 to the stack, swaps d0 and d1, calls another routine, moves d1 to d0, and restores d2 before returning.
- `process_signed_values`: Pushes d2 to the stack, performs conditional negation on d2 and d1, and calls another routine if certain conditions are met.
- `process_signed_values_variant`: Pushes d2 to the stack, performs conditional negation and bitwise inversion on d0, d1, and d2, and calls another routine if certain conditions are met.
- `transfer_stack_parameters`: Saves two registers to the stack, loads two 16-bit values from the stack, and branches to another location.
- `calculate_loop_offset`: Calculates a value based on a 16-bit input multiplied by 44, then calls a library function using a pointer from a table and an offset derived from the calculation, repeating this in a loop until a counter reaches zero.
- `transfer_stack_words_and_branch`: Saves registers d2 and d3 to the stack, loads two 16-bit words from offsets 14 and 18 on the stack into those registers, and performs a short branch to another routine.
- `calculate_loop_offset_with_stack`: Calculates a value based on a 16-bit input multiplied by 44, then calls a library function using a pointer from a table and an offset derived from the calculation, repeating this in a loop while adjusting the stack pointer.
- `dispatch_via_table`: Loads a value from a global variable and, if it is between 0 and 7 inclusive, uses it to jump to an address stored in a table.
- `process_signed_values_variant_2`: Loads a value from a global pointer into d3, writes 47 to two global byte locations, clears a byte at a global location, writes 58 to a global location, calls another routine, and performs a series of arithmetic operations involving shifts, subtractions, and divisions before storing results into several global byte locations.
- `calculate_table_indices_2`: Initializes several registers, loads a value from a global pointer, and enters a conditional sequence that calls another routine multiple times, performs subtractions and comparisons, and eventually calls another routine with three values pushed onto the stack.
- `calculate_table_indices`: Performs a series of divisions by ten on four input values and uses the results to calculate and store bytes into five different locations in a table using arithmetic operations including shifts and subtractions.
- `initialize_system_resources_2`: Calls three different routines with specific constants, then performs a series of table lookups and assignments to two global variables, calculates an index using arithmetic and bit shifts, and calls a routine using a pointer retrieved from a table.
- `calculate_complex_offset`: Performs multiple arithmetic operations involving sign extension and shifts on bytes retrieved from various memory locations, then calls several routines and eventually calls another routine with a value derived from subtracting a large constant from a register.
- `update_string_and_calculate`: Updates several bytes in a table based on comparisons, performs arithmetic on multiple values using a series of shifts and additions, and calls another routine if a specific condition is met.
- `decrement_and_call_2`: Loads three global pointers into registers, performs several conditional byte updates to those locations, calculates a series of values using arithmetic and shifts based on values at several global offsets, and finally performs a jump to an address stored in a table.
- `decrement_and_call`: Compares d3 to 1, and if d3 is greater than 1, decrements d4 and calls another routine if d4 is non-negative.
- `allocate_and_configure_port_2`: Loads two values from the stack, calls another routine, and then conditionally calls several other routines to manage memory or devices, performs arithmetic on d2 and d3, and writes a value to an offset of 32 in a structure pointed to by a2.
- `manage_device_resources`: Performs a series of conditional checks and calls multiple routines to manipulate pointers and status bits, including calls to process data, perform bitwise operations, and interact with hardware or system resources via several subroutines.
- `add_preference_task`: Pushes a pointer onto the stack, calls a subroutine, and if the result in d0 is not zero, performs a series of operations including setting a bit in a global variable, storing a pointer in a structure, and calling another subroutine.
- `release_resource_structure_3`: Loads a value from the stack and two global pointers, performs a bitwise check, and then conditionally calls several routines to free memory, close devices, or remove tasks, and finally clears a global location.
- `send_io_request`: Populates an I/O request structure with a command, a longword, and a pointer from the stack, then calls a routine to process the request.
- `initialize_preference_list`: Allocates memory, clears a global variable, and enters a loop to populate a list by copying bytes from a source to a destination, then performs several memory operations and calls other routines to manage the list and its contents.
- `compare_byte_sequences_2`: Compares two byte sequences pointed to by arguments on the stack and returns 1 if the first sequence is less than or equal to the second, otherwise it returns 0.
- `allocate_and_initialize_list`: Allocates a block of memory, updates a global pointer to point to this new block, and then enters a loop to write bytes from a source to the new memory at specific offsets.
- `free_and_clear_structure`: Calculates a size based on a value in a global variable, calls a library function to free memory, and then conditionally frees another pointer and clears a global structure.
- `validate_preference_data`: Clears a word in a global variable and enters a loop to validate data by comparing bytes against tables, calling another routine to perform checks before returning a result.
- `validate_byte_array`: Iterates through a loop to compare bytes from a source array against values in a table, returning 1 if all checks pass or 0 otherwise.
- `update_preference_attributes`: Calculates several values using multiplication and bit shifts, then performs multiple subroutine calls to update structure members and global variables.
- `dispatch_table_entry_3`: Clears d2, loads a pointer from a global variable, compares a value at offset 20 of that pointer to 16, and if equal, calls a routine at an address stored in a global variable.
- `dispatch_message_result`: Calls a subroutine, moves the result to d2, calls another subroutine using a pointer from a memory location, and then branches based on whether the result in d0 is less than 1, equal to 1, or greater than 1.
- `decrement_and_branch_4`: Decrements d0 by 2; if it is not zero, it branches to an earlier instruction, otherwise it branches to another location.
- `restore_and_branch`: Pushes two values onto the stack, calls another routine, and then branches to another location.
- `push_and_branch_2`: Pushes two longwords onto the stack and branches to another location.
- `call_external_routine`: Copies the value from a2 to a0, pushes a longword from an offset of a0 onto the stack, calls a routine at an address in a global variable, stores the result in a global variable, and then branches to another location or continues.
- `restore_registers_from_stack`: Restores the d2, a2, and a3 registers from the stack using the address pointed to by the stack pointer and then returns immediately.
- `copy_word_values`: Copies four 16-bit values from a structure pointed to by a register to another structure pointed to by a register.
- `dispatch_via_table_5`: Loads a pointer from a structure, retrieves a word from an offset of that pointer, and jumps to an address found in a table using that word as an index.
- `load_and_branch_2`: Pushes a value onto the stack, calls another routine, and then branches to another location.
- `restore_registers`: Restores registers a2 through a4 from the stack and returns.
- `check_and_clear_structure`: Compares values from a structure against constants and a global variable, then calls another routine and clears a location in a structure before returning.
- `decrement_and_branch`: Decrements a 16-bit value from the stack, and if the result is not zero, branches to another location; otherwise it sets d0 to zero and branches to another location.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `load_long_word`: Loads a 32-bit value from the stack into d0 and returns.
- `modify_port_attributes`: Calls another routine, sets a byte at offset 24 of a structure, performs several calculations using a table to derive values, and calls another routine before setting a byte at offset 24 of a structure to 255.
- `store_and_clear`: Stores a word from the stack into d1, pushes d2 onto the stack, and clears d2 before branching.
- `compare_and_decrement`: Increments d2, compares a value at an offset of a0 with d1, and branches based on the result and the value of d2.
- `decrement_d2_and_pop`: Sets d0 to the value in d2, decrements d2 by one, and pops a long word from the stack into d2 before returning.
- `update_data_tables`: Clears a long word in a global variable, calls another routine, and then enters a loop that copies words from one table to another, eventually updating several global pointers and performing bitwise operations on a table value.
- `call_library_function_2`: Pushes two long words onto the stack and calls a routine at a library vector.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read_2`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write_3`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input_3`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output_3`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_delete_file`: Library stub: loads its arguments and the dos library base, calls DeleteFile and returns its result to the caller.
- `call_rename`: Library stub: loads its arguments and the dos library base, calls Rename and returns its result to the caller.
- `call_lock_3`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock_3`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine_2`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_date_stamp`: Library stub: loads its arguments and the dos library base, calls DateStamp and returns its result to the caller.
- `call_delay_2`: Library stub: loads its arguments and the dos library base, calls Delay and returns its result to the caller.
- `update_pointer_structure`: Takes a pointer from the stack, sets the value at that pointer to itself, increments the value at that address by four, clears a longword at an offset of four, and sets a longword at an offset of eight.
- `manage_system_files`: Attempts to open and read files using library calls, and if certain conditions are met, performs write and delete operations on files specified by pointers in a table.
- `load_configuration_file`: Opens a file, reads its contents, and if successful, calls another routine to process the data and potentially calls a routine to manage memory.
- `allocate_and_configure_port`: Pushes registers to the stack, calls several routines to allocate memory and signals, and then populates a structure with values from the stack before calling more routines to add a port and potentially a task.
- `release_resource_structure`: Checks a value at an offset of ten in a structure; if present, it calls a routine, sets a byte at offset eight to negative one, and calls two other routines using values from the structure, otherwise it returns.
- `allocate_and_configure_structure`: Calls two routines to perform operations with a value from the stack, then conditionally calls a third routine and modifies bytes at specific offsets in a structure, including setting three long values using an offset of 15996.
- `execute_conditional_handler`: Checks a pointer in a table; if not null, it sets a bit in a global variable, performs a bitwise shift on a byte from another table, and calls a routine with the shifted value and the table pointer, otherwise it calls a routine with a value from a stack and then potentially calls another routine with a value from a table and a pointer.
- `execute_preference_callback`: Saves registers, loads a pointer into a2, calls another routine, loads a pointer from offset 8 of a structure, follows a pointer at offset 56 of that structure to a byte at offset f, sign-extends it, and calls another routine with the result multiplied by 1.
- `dispatch_preference_message_2`: It clears d3, compares a longword at offset 20 of a structure in a2 to 16, and if equal, calls another routine and branches.
- `dispatch_preference_message`: Loads a pointer from a2, compares a longword at offset 20 of that pointer to 64 or 32, calls another routine, and if the result in d3 is not zero, pushes a pointer from a2 onto the stack and calls another routine.
- `decrement_and_branch_5`: Decrements a word in d0, branches if it is not zero, otherwise it branches to another location.
- `call_routine_with_parameters`: Pushes two longwords onto the stack, calls a routine at an address in a register, increments the stack pointer, and branches to another routine.
- `push_parameters_and_branch`: Pushes two longword constants onto the stack and performs a short branch to another routine.
- `transfer_and_call_message`: Loads a pointer into a0, pushes a value from an offset of a0 onto the stack, calls another routine, stores the result in a2, and branches to another location.
- `return_immediately_2`: Returns immediately after restoring registers d2, d3, and a2 from the stack.
- `copy_and_initialize_data`: Copies several bytes from one structure to another, runs a loop to copy values from a table, and calls three different routines.
- `process_preference_structure_2`: Performs a complex sequence of table lookups, memory copies, and multiple subroutine calls to update various global variables and structure members based on calculated offsets and bitwise operations.
- `dispatch_signal_handler`: Loads several pointers from a structure, performs a comparison on a 16-bit value, and jumps to an address determined by a table lookup.
- `restore_registers_and_return`: Clears d0 and restores d2, d3, a2, and a4 from the stack before returning.
- `check_and_branch_to_table`: Saves registers to the stack, checks a word at offset 22 of the stack, and if a byte at offset 20 of a pointer in a1 is 3, it branches to a routine that clears d0 and branches to another routine.
- `process_structured_data`: Calculates three different values using bitwise operations and writes them to a table, then calls another routine three times with different parameters.
- `calculate_and_store_offset`: Calculates a value based on two 16-bit words from the stack and a constant, stores the result in a memory location pointed to by a calculated address, and calls another routine.
- `process_table_entries_and_call_handler`: Performs multiple lookups in a table using a value from the stack, performs arithmetic on the retrieved bytes, and calls another routine multiple times with these values on the stack.
- `call_library_with_parameters`: Calls another routine three times, passing a constant and a value from a table as arguments on the stack each time.
- `execute_conditional_callbacks`: Performs multiple calculations using values from the stack and calls several routines, including a loop that conditionally executes calls based on a comparison.
- `execute_conditional_loop`: Calls a routine using a pointer from a structure, performs bitwise operations on values derived from a table, and enters a loop that calls another routine while incrementing two counters.
- `modify_bitmask_in_table`: Calculates bitmask values using several 16-bit words from the stack and performs bitwise AND and OR operations on specific locations in a table.
- `calculate_offsets_and_call_routines`: Performs several calculations involving byte values from two tables, uses these to derive addresses, and calls three different routines with the results on the stack.
- `call_allocation_routine`: Pushes d2 to the stack, loads a value from the stack into d2, pushes a constant, calls another routine, and restores d2 before returning.
- `call_free_block`: Loads a pointer from the stack and calls a routine to free a block of memory.
- `allocate_and_initialize_block`: If a pointer is not null, it calls a routine to allocate memory, and if successful, initializes the new block by writing a byte, a word, and a longword to specific offsets before returning the pointer.
- `free_and_clear_block`: If a pointer is not null, it sets a byte at offset eight to negative one, sets longwords at offsets twenty and twenty-four to negative one, and calls a routine with a word from offset eighteen and the block pointer.
- `validate_and_initialize_preferences`: Iterates through a table to copy bytes into a structure, then performs multiple subroutine calls to initialize various system parameters and update global variables.
- `validate_preference_count`: Clears d2, sets a byte at a global location, compares a value in a structure against 64 and 32, calls a subroutine, and then branches based on the result in d0 and the value of a global byte.
- `decrement_and_branch_preference_counter`: Decrements a 16-bit word in register d0 by two and branches to another routine if the result is not zero, otherwise it branches to a third routine.
- `copy_and_validate_data`: Calls two routines, then enters a loop that copies bytes from a table to a global variable and another table based on a counter up to 30.
- `copy_preference_bytes`: Checks a global word, then enters a loop that copies bytes from one table to two other locations based on a counter up to 30.
- `copy_and_call_preference`: Enters a loop to copy bytes between tables, then copies a long word from a global variable to another location and calls another routine.
- `call_message_handler`: Loads a pointer from a structure in a2, pushes a value from offset 86 of that structure onto the stack, calls a routine, and stores the result in a location pointed to by a4.
- `restore_registers_extended`: Restores registers d2 through d4 and a2 through a4 from the stack and returns.
- `copy_preference_block`: Copies several 16-bit and 8-bit values from one structure to another, then enters a loop that copies bytes from one table to another 30 times before clearing a long value in a global variable.
- `dispatch_via_table_3`: Loads a pointer from a global variable, retrieves a word from an offset, and if the word is within a specific range, uses it to calculate an offset for a jump to another location.
- `clear_d0_and_restore`: Sets register d0 to zero and restores registers d2 through d3 and a2 through a4 from the stack.
- `initialize_system_resources`: Performs a series of calls to various routines using pointers from a table, including setting signals, allocating traps, and adding ports, while calculating offsets based on a global pointer and a value from a table.
- `zero_d0_and_return`: Loads a 16-bit word from an offset of 6 from the stack into d0, sets d0 to zero, and returns.
- `update_global_word`: Compares a word at a memory location to 1; if it is 1, it writes 1 to a different memory location, otherwise it compares a value to a longword in a global variable and, if the value is higher, writes it to a global variable before calling another routine.
- `process_preference_set_a`: Calls three routines, performs arithmetic to calculate an offset, and calls a fourth routine using a pointer derived from that offset.
- `update_preference_value`: Compares a word at a memory location to 1; if it is 1, it writes 1 to a different memory location, otherwise it compares a value to a longword in a global variable and, if the value is not lower, writes it to a global variable before calling another routine.
- `update_preference_pointer`: Loads a value from a memory location, stores it into a global variable, and then calls another routine to update a specific entry in a table by calculating an offset based on a 16-bit value multiplied by 44.
- `compare_and_call_2`: Compares 1 to a longword at a specific address; if 1 is less than or equal to that value, it branches to another location, otherwise it writes 1 to a global variable and calls another routine.
- `update_preference_data`: Fetches a value from a memory location, stores it in a global variable, and then calculates an offset into a table by multiplying a value by 44 to call another routine with a pointer to that table entry.
- `dispatch_port_attributes`: Performs bitwise masking and subtraction to calculate values, then executes several subroutine calls to update status bytes and structure members.
- `process_preference_data`: Compares values in a structure against 64 and 32, calls a subroutine, then calls another subroutine using a pointer from a4, and branches based on the result in d0.
- `decrement_and_branch_counter`: Decrements a 16-bit value in register d0 and branches to another routine if the result is not zero, otherwise it branches to a third routine.
- `push_arguments_and_call_byte_copier`: Pushes two 32-bit values onto the stack, calls a routine to copy a 16-bit word and three bytes between structures, clears register d0, adjusts the stack pointer, and branches to another location.
- `push_two_pointers_and_branch`: Pushes two 32-bit pointers onto the stack and performs a short branch to another routine.
- `execute_message_call`: Loads a pointer from a structure in a2, pushes a value from offset 86 of that structure onto the stack, calls a routine, and stores the result in a location pointed to by a4, then branches based on the result.
- `restore_registers_2`: Restores registers d2, a2, a3, and a4 from the stack and returns.
- `copy_specific_bytes`: Copies a 16-bit word and three bytes from one structure to another using pointers provided on the stack.
- `dispatch_indexed_jump`: Loads a pointer from a global table, retrieves a word from offset 38 of the pointed structure, and performs a jump to an address calculated from a table using that word as an index after checking if it is between 94 and 111.
- `set_d0_and_branch_3`: Sets register d0 to 1 and branches to another routine.
- `set_d0_and_branch_4`: Sets register d0 to 2 and branches to another routine.
- `branch_to_address`: Performs a short branch to a specific address.
- `restore_registers_short`: Restores registers d2, d3, a2, a3, and a4 from the stack and returns.
- `setup_port_and_dispatch`: Calls several routines to set up environment parameters and then calls another routine using a pointer retrieved from a table after scaling a value from a global variable.
- `update_status_byte`: Calls several routines to initialize settings and then calls a routine using a pointer from a table, finally performing bitwise operations to update a byte in a global variable.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr VF].adf:Empire

- `branch_to_offset_2`: Performs a short branch to an offset of 272.
- `update_table_entry_2`: Saves registers to the stack, calculates an offset into a table using a value from d0, compares a longword at an offset of the table with a value at another location, and either stores a new longword or clears a longword at an incremented pointer.
- `copy_memory_block_with_loop`: Calls a library function, performs arithmetic on a pointer, and uses two loops to copy bytes from one memory area to another.
- `cleanup_multiple_pointers`: Loads a pointer from the stack, calls a routine at that pointer, and then sequentially checks and calls various routines using pointers stored at various offsets from a4 and a6.
- `call_library_functions_3`: Loads a pointer from an offset of 92 in a3 into a0, calls a routine at an offset of -384 in a6, loads the same pointer into a0 again, calls a routine at an offset of -372 in a6, and then returns.
- `execute_library_call_4`: Loads an address into a1, clears d0, calls a library function, stores the result in a location relative to a4, and otherwise returns.
- `jump_to_address_38`: Performs an absolute jump to an address.
- `call_library_with_arguments`: Saves registers to the stack, loads a library base address from a table, populates a0 through d3 and d0 through d2 with values from the stack, calls a routine at an offset of -348 in a6, restores the registers, and returns.
- `calculate_offset_address_4`: Checks a value at offset 8 of a structure in a5 and compares a value at offset -420 of a4 against it, then performs multiplication and pointer arithmetic to update a destination address if certain conditions are met.
- `process_stack_and_branch`: Pushes a word from a structure in a5 onto the stack, calls another routine, and if the returned value in d0 is non-zero, stores d0 into a structure in a5, otherwise it sets d0 to 255 and branches.
- `test_and_clear_word_2`: Tests bit 2 of a byte at an offset of a pointer, and if set, clears a word at that pointer's location before branching to another routine.
- `execute_and_clear_status`: Clears a word at an offset of a5, calls another routine, and if a value at an offset of a4 is non-zero, clears a word at an offset of a0 before returning.
- `calculate_and_call_handler`: Calculates a difference between two values from a structure, stores it in a location in a5, and calls another routine with three values from a structure on the stack.
- `process_conditional_data_2`: Links a5, tests bit 1 of a word at an offset of a pointer from a5, and if set, pushes a pointer and two values to the stack, calls another routine, and stores the result at a negative offset from a5.
- `process_structure_data`: Clears a word at offset -2 of a5, loads a pointer from offset 8 of a5, and if specific bits in a word at offset 18 of that pointer are zero and a word at offset 16 is zero, it calls two different routines using values from the pointer before storing a word in offset -4 of a5.
- `tiny_moveq_unlk_rts_3`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `update_pointer_from_table`: Loads a pointer from an offset of a table into a global variable, checks if the pointer is null, and if not, performs several checks on the data at that pointer before updating a value at offset -8 of a structure in a5 and branching to another routine.
- `check_and_store_value`: Checks a longword at offset -4 of a5, calls a subroutine, and if the result is non-zero, stores that result back at offset -4 of a5.
- `fill_memory_and_call`: Copies a byte of 0 to a memory location pointed to by a pointer from offset -4 of a5 using a loop that runs 23 times, then pushes three values onto the stack and calls another routine.
- `process_conditional_pointer`: Tests bit 1 of a word at an offset of a pointer from a5; if set, it calls a routine and then checks a byte at an offset of a pointer to determine whether to clear a word at an offset of a5.
- `process_indexed_entry`: Writes 8000 hex to offset minus 6 of a5, increments a value at offset minus 10 of a5, calls another routine with several parameters on the stack, and enters a loop that updates a value in a5 based on the result.
- `test_byte_and_branch`: Tests a byte at offset negative 11 from a5; if the byte is zero, it branches to another location, otherwise it sets d0 to 128 and branches to another location.
- `set_structure_word`: Sets a 16-bit value of 16386 at offset -2 of a structure in a5 and branches to another location.
- `check_byte_and_set_register`: Tests a byte at offset minus 11 of a structure in a5 and, if it is zero, branches to another location; otherwise it sets d0 to 2 and branches to another location.
- `process_indexed_loop`: Pushes several values onto the stack, calls another routine, and then enters a loop that updates a value at offset minus 4 of a structure in a5 until d0 reaches zero.
- `test_and_set_flag`: Tests a byte at offset 11 of the structure in a5; if the byte is zero, it branches to another routine, otherwise it loads the value 128 into d0 and branches to another routine.
- `set_structure_word_small`: Sets a 16-bit value of 1 at offset -2 of a structure in a5 and branches to another location.
- `test_and_branch_byte`: Tests a byte at an offset of negative eleven from a pointer in a5; if the byte is zero, it branches to another routine, otherwise it loads the value two into d0 and branches to another routine.
- `process_indexed_loop_with_mask`: Pushes several values onto the stack, including a masked constant, calls another routine, and then enters a loop that updates a value at offset minus 4 of a structure in a5 until d0 reaches zero.
- `test_and_set_flag_2`: Tests a byte at offset negative 11 of the structure in a5; if the byte is zero, it branches to another routine, otherwise it loads the value 128 into d0 and branches to another routine.
- `set_structure_word_alt`: Sets a 16-bit value of 2 at offset -2 of a structure in a5 and branches to another location.
- `clear_d0_and_branch_3`: Sets d0 to zero and branches to another location.
- `initialize_structure_fields_2`: Clears several fields in a structure pointed to by a value at offset 16 of a5 and sets a bit in a field at offset 18 if a specific condition is met.
- `check_condition_and_branch`: Saves registers to the stack, loads a word from offset 8 of a structure in a5 into a2, loads a word from offset c of a5 into d7, and if d7 is not zero, branches to another routine; otherwise, it sets d0 to zero and branches to another routine.
- `increment_and_modify_data`: Compares d5 and d4 against d6 and d7, then if conditions are met, modifies a word at offset 8 of a structure in a0 and copies a byte from an address in a1 to d0.
- `update_and_branch`: Pushes a0 onto the stack, calls another routine, and if the result in d0 is not 65535, stores d0 at offset -6 of a5 and either branches or moves d5 to d0.
- `copy_byte_and_increment`: Copies a byte from offset -6 of a structure in a5 to the address in a2, then increments a2 and d4 before branching to another location.
- `increment_d5_and_branch`: Increments d5 by 1 and branches to another location.
- `move_d5_to_d0`: Copies the value from d5 to d0, restores d4 through d7 and a2 from the stack, unlocks a5, and returns.
- `update_buffer_from_structure`: Compares two words from a structure against zero, and if both are non-negative, reads a byte from a pointer, masks it, increments the pointer, and writes the byte to another pointer if a counter is non-negative.
- `loop_and_call_routine_2`: Masks a word from a negative offset of a5, pushes a pointer and the masked word to the stack, calls another routine, and then enters a loop that increments d0 and branches if it is not zero.
- `increment_d4_and_branch`: Increments d4 by 1 and branches to another location.
- `increment_register_and_branch`: Increments a 16-bit value in register d5 and performs a short branch to another routine.
- `move_d5_to_d0_alt`: Copies the value from d5 to d0, restores d2 and d4 through d7 and a2 from the stack, unlocks a5, and returns.
- `call_subroutine_with_word`: Links a5, loads a word from offset 8 of a5 into d0, pushes d0 to the stack, and calls a subroutine at offset 6fc.
- `check_and_branch_on_pointer`: Saves registers, loads a word from an offset of a5, and if that word is greater than zero, sets d0 to zero and branches; otherwise it returns.
- `compare_and_update_pointer`: Compares d7 to 8, and if less, performs arithmetic on d7, loads a pointer from a structure at offset -430 of a4 into a2, and if the pointer is not zero, compares a value at offset 4 of a2 with d7 and potentially updates a pointer at offset -8 of a5.
- `update_pointer_structure_4`: Subtracts d7 from a value at offset 4 of a2, and if the result is not less than 8, it calculates a new address using a2 and d7, stores a value from a structure at that address into offset 4 of a new location, and updates a pointer at offset -8 of a5.
- `update_and_dereference_pointer`: Stores a pointer from a2 into a location at an offset of a5, loads a new pointer from the address pointed to by a2 into a2, and branches.
- `calculate_and_store_results`: Performs arithmetic on d0 and d1, calls two subroutines, performs bit shifts and arithmetic on d0, calls a third subroutine, stores a value in a structure at offset -16 of a5, and calls a fourth subroutine if d0 is non-zero.
- `clear_d0_and_restore_2`: Sets d0 to zero, restores d6 through d7 and a2 from the stack, unlocks a5, and returns.
- `check_ascii_range_2`: Loads a 16-bit word from an offset of 8 in a5 and compares it against the range between 48 and 57 inclusive; if it falls within this range, it sets d0 to 1 and branches, otherwise it returns.
- `call_and_store_result_3`: Pushes a word from a structure onto the stack, calls another routine, and stores the result in a longword at an offset of a structure, otherwise it sets d0 to negative 255 and branches.
- `increment_list_pointer`: Adds 10 to a 32-bit value from offset 8 of a5, stores the result at offset -4 of a5, and calls another routine.
- `update_list_pointer`: Adds 10 to a 32-bit value from offset 8 of a5, stores the result at offset 8 of a structure pointed to by -4 of a5, calls another routine with a parameter from a4, and if a global variable at offset -440 of a4 is not null, updates that variable with a pointer derived from the structure in a5.
- `update_list_element`: Loads a word from offset 8 of a5, pushes it to the stack, calls another routine, and then stores the returned value into offset -4 of the structure in a5.
- `free_memory_block`: Checks if a pointer at offset 25708 of a4 is non-null, and if so, retrieves a 16-bit word from offset 8 of a structure pointed to by the value at offset -4 of a5, then calls another routine using that word as a parameter.
- `update_linked_list_2`: Calculates the difference between two pointers, stores the result in a structure, and if a global pointer is null, updates that pointer or a pointer within a second structure with the new address.
- `process_indexed_table_entry`: Clears a byte at offset minus 1 of a structure in a5, clears a word at offset minus 32744 of a structure in a4, and enters a loop that compares a word from offset minus 6 of a5 with a word from offset minus 420 of a4, performing arithmetic and checking a table at an offset of a4 before incrementing a value at offset minus 6 of a5.
- `compare_and_clear_word`: Compares a word from offset minus 6 of a structure in a5 with a word from offset minus 612 of a structure in a4, and if they are equal, stores 24 at offset minus 576 of a structure in a4 before branching to another routine.
- `calculate_offset_and_clear`: Multiplies a word by six, adds it to a base address in a4, stores the result in a location in a5, and clears a word in a5 if certain bit conditions are met.
- `modify_offset_word`: Sets a byte at offset -10 of a5 to 1, modifies a word at offset 12 of a5 based on a bitwise operation with a word at offset -452 of a4, and if bit 3 of a byte at offset 13 of a5 is set, performs further bitwise manipulations on the word at offset 12 of a5.
- `write_constant_and_branch`: Stores the constant 22 in a 16-bit word at offset -576 of a4 and branches.
- `evaluate_and_call_subroutine`: Loads a word from offset 12 of a5 into d0, and if bit 10 of d0 is set, sets a byte at offset -1 of a5, pushes two words from a5 onto the stack, calls another routine, and stores d0 at offset -14 of a5.
- `update_status_and_call_subroutines`: Tests bit 9 of d0, and if clear, calls a subroutine with a constant and a value from a structure, updates a longword, and performs further conditional subroutine calls based on bit tests and sign checks.
- `store_constant_and_call`: Pushes the word 1003 onto the stack, pushes a word from offset 8 of a5 onto the stack, calls a subroutine, adjusts the stack pointer, stores d0 into a structure at offset -14 of a5, and then either returns or sets d0 to negative one based on a test of a value at offset -32720 of a4.
- `write_structure_values`: Writes a 16-bit word from an offset of the stack frame to a location pointed to by another offset, and writes a 32-bit long from a third offset to a location at a 2-byte offset from that same destination.
- `update_scaled_value`: Multiplies a 16-bit word from a global variable by 3677, adds 3, masks the result with 32767, and stores the new value back into the global variable.
- `store_word_to_offset`: Stores a 16-bit word from an offset of 8 in a5 to an offset of -572 in a4 and returns.
- `call_and_store_result_4`: Pushes a word from a structure in a5 onto the stack, calls another routine, stores the returned longword at offset -4 of the structure in a5, and otherwise sets d0 to negative 1 and branches.
- `load_and_call_pointer`: Pushes two words from a structure in a5 onto the stack, loads a pointer from offset -4 of a5 into a0, pushes a word from offset 2 of a0 onto the stack, calls a subroutine, restores the stack pointer, stores d0 into a structure at offset -8 of a5, and then either returns or sets d0 to negative one based on a test of a value at offset -32720 of a4.
- `load_pointer_from_stack`: Loads a 32-bit long from an offset of the stack frame into register d0.
- `push_and_call_routine_2`: Sets up a stack frame, moves a word from an offset of a5 into d0, pushes d0, pushes a word from an offset of a5, pushes a value from an offset of a5, calls another routine, cleans the stack, unlinks a5, and returns.
- `compare_and_branch_6`: Saves registers to the stack, loads a value from an offset in the stack frame into d7, and if d7 is not greater than zero, it sets d0 to negative one and branches.
- `update_pointer_structure_3`: Calculates a value based on d7, updates a pointer at offset -12 of a5, and if a pointer from a stack location is non-zero, updates a location at offset -20 of a5 and potentially updates a pointer at offset -12 of a5.
- `update_pointer_structure_2`: Compares a pointer in a2 with d2 and, if they are not equal, copies a pointer from a2 to a location at offset -12 of a5, copies a value from offset 4 of a2 to d0, adds d7 to d0, stores the result at offset 4 of the location in a3, and finally stores a3 at offset -8 of a5.
- `subtract_and_branch_2`: Compares a longword at an offset of a5 with a0, and if a0 is greater or equal, subtracts d7 from a longword at an offset of a4 and branches to another location; otherwise it sets d0 to negative one and branches.
- `compare_and_subtract_long`: Compares a0 and a1, and if they are not equal, branches to another location; otherwise, it checks if the long value at a2 is non-zero, and if so, compares the long value at a2 with d2, potentially subtracting d7 from a value at an offset of a4 before branching.
- `modify_structure_data`: Adds d7 to a 32-bit value at offset 4 of a2, and if a comparison between a2 and d2 matches, it adds a 32-bit value from offset 4 of a structure pointed to by d2 to the value at offset 4 of a2 and updates the pointer at a2.
- `update_pointers_and_branch`: Stores the value of a2 into an offset of the stack frame, stores a value from another offset of the stack frame into a different offset of the stack frame, updates a2 with a pointer from a2, and branches.
- `clear_and_set_long`: Clears a 32-bit long at a location pointed to by an offset in the stack frame and then stores the value of register d7 into a 4-byte offset of that same location.
- `increment_and_store_byte`: Increments a 16-bit value at a global offset, loads a byte from an offset in the stack frame, stores it at a location pointed to by another global offset, and then increments that global offset.
- `clear_and_call_handler`: Clears a word at a global offset, moves a pointer from a structure in a5 to a global location, pushes a word from offset 12 of a5 and a pointer to another routine onto the stack, calls that routine, clears a byte at the location pointed to by a0, and returns.
- `check_character_match`: Loads a byte from a pointer at offset 8 of a5, compares it to 45, and if they match, sets three words at offsets -2, -4, and -8 of the stack frame to 1 before branching.
- `compare_and_assign_pointer`: Checks if a byte at a pointer from offset 8 of a5 is non-zero, then compares a byte from a pointer at offset 12 of a5 with a byte from offset 8 of a5, and if they match, stores that pointer in d0.
- `increment_stack_value`: Increments a 32-bit value at offset -4 of a5 and branches.
- `increment_offset_value`: Increments a 16-bit word at offset 8 of a5 and branches.
- `zero_d0_and_return_2`: Sets register d0 to zero and returns.
- `call_subroutine_with_args_3`: Pushes two longwords from offsets 12 and 8 of a5 onto the stack and calls another routine.
- `compare_case_insensitive_bytes`: Loads two bytes from two different pointers, converts them to uppercase if they are lowercase letters, and returns if the resulting values are equal and the second byte was not zero.
- `process_dual_byte_conversion`: Checks if words at offsets 16, 8, and 12 of a5 are non-zero, then calls another routine twice using bytes from offsets 8 and 12 as arguments, and finally stores the result of a subtraction into offset -2 of the structure in a5.
- `decrement_offset_value`: Decrements a 16-bit word at offset 16 of a5 and branches.
- `check_status_byte`: Tests a word at offset 16 of a5 and, if the byte at the pointer in offset 8 of a5 is non-zero, sets d0 to 1 and branches.
- `check_byte_status`: Loads a pointer from offset 12 of a5 into a0, and if the byte at that pointer is non-zero, sets d0 to 255 and branches.
- `unlock_mutex_and_clear_d0`: Clears register d0 and unlocks the mutex held in address a5 before returning.
- `convert_ascii_to_integer`: Loads a word from the stack, compares it against the decimal values 97 and 122, and if it falls within that range, subtracts 32 from it before returning.
- `call_and_store_result_5`: Pushes a word from offset 8 of a5 onto the stack, calls another routine, and then stores the returned value in offset -6 of a5, otherwise it sets d0 to -1 and branches.
- `increment_and_dispatch_byte`: Increments a word at offset 8 of a5, stores a byte from that location at offset -1 of a5, and then uses a value from offset 8 of a5 to perform a jump to an address from a table if a comparison succeeds.
- `increment_and_copy_byte`: Increments a word at an offset of a5, decrements a word at an offset of a4, and if the latter is non-negative, copies a masked byte from an offset of a5 to a location pointed to by a global variable.
- `process_masked_byte`: Loads a byte from an offset of minus one from a5 into d0, performs a bitwise AND on d0 with 255, pushes d0 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `decrement_and_update_byte_buffer`: Decrements a 16-bit value in a global variable and, if the result is non-negative, copies a byte from an offset of minus one from a5 to a destination pointer stored at an offset of minus 496 from a4, then increments that pointer and masks the byte before branching to another routine.
- `mask_and_process_byte`: Loads a byte from an offset of minus one from a5, masks it to eight bits, pushes it onto the stack along with a pointer to an offset of minus five hundred four from a4, calls process_masked_word, cleans the stack, and branches to another routine.
- `execute_masked_word_call`: Pushes a pointer from an offset of minus 500 from a4 onto the stack, loads 255 into d0, pushes d0 onto the stack, calls another routine, adjusts the stack pointer, loads a word from an offset of minus 4 from a5 into d0, unlinks a5, and returns.
- `update_status_bit`: Reads a word from a structure, checks bit 15 to set or clear a byte at an offset of a5, and if bits 4 or 5 of the word are set, clears a word at an offset of the structure.
- `validate_and_update_status_structure`: Tests bits 6 and 7 of a byte at offset 19 of a structure; if both are set, it pushes the structure pointer and a value of negative one onto the stack and calls another routine.
- `call_status_update`: Pushes a2 to the stack, calls another routine, restores the stack, tests d0, and if non-zero, sets bit 5 of a byte at an offset of a2.
- `update_counter_and_byte`: Tests a byte at offset -1 of a5, and if not zero, increments a word at offset 8 of a2, then if that word is not greater than zero, it loads a byte from a pointer in a2, stores it in offset -4 of a5, and performs conditional branches based on its value.
- `call_subroutine_with_a2`: Pushes a2 onto the stack, calls another routine, and then branches.
- `set_bit_and_branch_2`: Sets bit 4 of a byte at offset 19 of a2, sets d0 to 255, and branches.
- `move_and_branch_4`: Moves a 16-bit word from offset -4 of a5 into d0 and branches.
- `update_status_via_subroutine_call`: Tests bit 1 of a byte at offset 25 of a structure in a2; if not set, it sets bit 0 of that byte, pushes three values from the structure onto the stack, and calls another routine.
- `update_status_and_register`: Stores d0 at offset 8 of a2, loads a pointer from offset 12 of a2, and if a bitwise test on a word from offset 18 of a2 passes and a byte at offset -1 of a5 is non-zero, sets d0 to 255 and stores it at offset 8 of a2.
- `clear_and_set_d0`: Sets d0 to 0, stores it at offset 8 of a2, sets d0 to 255, and branches.
- `decrement_and_copy_byte`: Decrements a word at offset 8 of a2, and if the result is non-negative, copies a byte from a pointer at offset 4 of a2 to d0 and increments that pointer.
- `call_subroutine_with_registers`: Pushes a2 onto the stack, calls another routine, restores d2 and a2 from the stack, unlocks a5, and returns.
- `process_masked_word`: Links a5, saves registers, loads a word from an offset of a2, performs a bitwise AND on a copy of it, stores a word at a negative offset from a5, and branches based on the result.
- `process_conditional_logic`: Performs bitwise operations on d0, stores a signed byte at a negative offset of a5, checks a word at an offset of a2, and calls another routine if certain conditions are met.
- `set_bit_and_negate_word`: Sets bit 0 of a byte at offset 19 of a structure in a2, checks a byte at offset -2 of a5, and if it is not zero, negates a word at offset 16 of a2 and stores the result at offset 10 of a2.
- `update_and_copy_data`: Decrements a word at offset 10 of a2, and if the result is non-negative, copies a byte from a pointer at offset 4 of a2 using a byte from offset 8 of a5 and increments that pointer.
- `push_masked_word`: Masks a word from offset 8 of a structure in a5 with 255, pushes the result and the value of a2 onto the stack, calls another routine, and branches to another location.
- `check_conditional_flag`: Tests bit 2 of a byte at offset 19 of a structure in a2, and if set, compares a word from offset 8 of a structure in a5 with the value 65535, otherwise it sets d0 to zero and branches.
- `execute_conditional_call`: Stores a byte from d1 at offset minus 1 of a structure in a5, and if a byte at offset minus 2 of a5 is non-zero and d1 equals 10, it pushes 2 to the stack, pushes an address from a4 minus 532, pushes a word from offset 20 of a2, pushes 2 to offset minus 10 of a5, calls another routine, cleans the stack, and stores 2 at offset minus 6 of a5 before branching to another routine.
- `call_and_set_values`: Pushes a word and a value from a structure onto the stack, calls another routine, stores the result in a location in a5, and sets a word at an offset in a5 to negative one.
- `execute_complex_update`: Sets a bit in a structure, performs multiple conditional checks on values in a5 and a2, and calls several different routines to update various longwords and words in a5 and a2.
- `reset_word_and_set_bit`: Sets a word at an offset of a5 to zero, compares that word to negative one, and if it is not negative one, sets bit 5 of a byte at an offset of a2.
- `set_bit_and_negate_word_2`: Compares a word at an offset of a5 with d1, and if they are equal, branches to another location; otherwise, it sets bit 4 of a byte at an offset of a2, checks a byte at an offset of a5, and if that byte is zero, branches to another location, otherwise it negates a word from an offset of a2 and stores it at another offset of a2 before branching.
- `test_bit_and_store`: Tests bit 2 of a byte at an offset of a2, and if that bit is clear, it stores zero into a different offset of a2 and branches.
- `copy_and_decrement_counter`: Copies a word from offset 16 of a2 to offset 10 of a2, moves a pointer from offset 12 of a2 to offset 4 of a2, and if a word at offset 8 of a5 is not 65535, decrements the word at offset 10 of a2 and, if it is non-negative, copies a byte from the pointer at offset 4 of a2 to the address in a0 and increments that pointer.
- `process_structure_byte`: Masks a byte from offset 8 of a structure in a5, pushes a2 and the masked byte onto the stack, calls another routine, and then checks if bit 4 or 5 of a byte at offset 18 of a2 is set, branching or returning -1 based on the result.
- `compare_word_and_branch`: Loads a word from offset minus 8 of a structure in a5 into d1, compares it to 65535, and branches to another location if they are not equal; otherwise it clears d0 and branches to another location.
- `restore_registers_and_return_2`: Moves the contents of d1 into d0, restores d2 and a2 from the stack, unlocks a5, and returns.
- `check_structure_status`: Links a stack frame, loads a pointer from an offset of the stack, tests a word at an offset of that pointer, and branches if it is zero or if bit 3 of a byte at another offset is set.
- `copy_values_to_structure`: Calls a subroutine, and if d0 is non-zero, copies d0 to offsets 4 and 12 of a structure in a0; otherwise it writes a word to offset -848 of a4 and sets d0 to 255.
- `update_structure_fields`: Copies a word from an offset of minus 844 from a4 to offset 16 of a0, performs a bitwise AND on offset 18 of a0 with 40949, and writes 0 to offsets 10 and 8 of a0 before unlocking a5 and returning.
- `dispatch_table_entry_4`: Initializes a stack frame with several zeroed bytes and a pointer, then checks a byte at a pointer in a0; if non-zero, it masks the byte, calculates an index, and jumps to a location in a jump table based on that index.
- `perform_bitwise_operations`: Performs bitwise OR operations on a byte at offset 96 of a4, a byte at offset 42 of a6, and a byte at offset 43 of a6, then branches to another location.
- `write_byte_and_branch`: Writes the byte value 1 to offset -2 of a structure in a5 and branches to another location.
- `increment_global_long`: Increments a longword at offset 8 of a structure in a5 and branches to another location.
- `increment_and_compare_byte`: Increments a value at offset 12 of a structure in a5, compares a byte from offset 12 of a5 with 37, and branches based on the result of these comparisons.
- `negate_conditional_longwords`: Negates the longword in d0 if it is negative, negates the longword in d1 if it is negative, and then calls another routine before negating d1 again.
- `perform_math_operations`: Pushes d2 to the stack, swaps the high and low words of d1, and if d1 is not zero, branches to another location; otherwise it performs a series of swaps and word divisions using d0, d1, and d2 before restoring d2 from the stack and returning.
- `perform_multiple_multiplications`: Loads a0 and a1 from d2 and d3, performs several word-sized multiplications using d0, d1, d2, and d3, adds d3 to d2, and returns with a0 and a1 updated.
- `loop_decrement_memory`: Loads two pointers from the stack and a word from the stack into d0, then enters a loop that decrements d0 and moves bytes from the end of the memory range pointed to by a1 to the end of the range pointed to by a0 until d0 reaches zero.
- `decrement_and_return`: Decrements d0 in a loop, loads a longword from offset 4 of the stack into d0, and returns.
- `copy_byte_conditionally`: Loads a0 and a1 from the stack, tests a byte at a1, and if it is non-zero, copies a byte from a0 to a decremented a1; if the byte was zero, it decrements a1 and returns.
- `setup_pointers_and_branch`: Loads two pointers from the stack into a0 and a1 and loads a word from the stack into d0, then branches to another location.
- `check_handler_status`: Saves registers to the stack, loads a pointer into a2 from a4, and compares a value in d7 to a value from a4; it then checks if the word at the pointer in a2 is zero or if bit 2 of a byte at an offset of a2 is set before branching.
- `set_constant_and_branch`: Checks a value from the stack, and if it is not zero, it stores the constant 5584 into an offset of a structure and returns.
- `call_system_function`: Calls a routine at an offset of a structure in a6 using the constant 12288, masks the result with 12288, stores it in a stack location, and branches based on whether the result is non-zero.
- `call_system_function_12288`: If a value at offset -32720 of a4 is non-zero, it calls a subroutine, then calls a library function at offset -42 of a6, and if the result is all ones, calls a library function at offset -132 of a6.
- `call_system_function_48`: If a value at offset -32720 of a4 is non-zero, it calls a subroutine, then calls a library function at offset -48 of a6, and if the result is all ones, calls a library function at offset -132 of a6.
- `calculate_branch_address_2`: If a value at offset -32720 of a4 is non-zero, it calls a subroutine, then calls a library function at offset -66 of a6, and if the result is all ones, calls a library function at offset -132 of a6.
- `transfer_d7_to_d0_2`: If a value at offset -32720 of a4 is non-zero, it calls a subroutine, then calls a library function at offset -30 of a6, and if the result is zero, calls a library function at offset -132 of a6.
- `transfer_d7_to_d0`: Moves the value from d7 to d0, restores registers from the stack, unlocks a5, and returns.
- `call_routine_and_increment_2`: If a value at offset -32720 of a4 is non-zero, it calls a subroutine, then calls a library function at offset -36 of a6 using a pointer from a structure.
- `load_d0_from_a5_2`: If a value at offset -32720 of a4 is non-zero, it calls a subroutine, then calls a series of library functions at offsets -84, -90, -72, and -30 of a6, and if a value at offset -4 of a5 is zero, calls a library function at offset -132 of a6.
- `load_d0_from_a5`: Loads a longword from offset minus 4 of the address in a5 into d0, unlocks a5, and returns.
- `execute_library_logic_2`: Checks a value at an offset in a structure in a4, calls another routine, clears a word at an offset in a4, then loads a longword from offset 8 of a structure in a5 into d1 and calls a library routine using a pointer from a6.
- `execute_library_call_2`: Loads a longword from a specific offset in a5 into d1, calls a routine using a pointer from a4, stores the result in a location at offset minus 4 of a5, and if the result is zero, calls another routine using a pointer from a4 and performs several writes to a4 before returning.
- `load_d0_from_a5_alt`: Copies a longword from the address in a5 minus 4 into d0, unlocks a5, and returns.
- `initialize_library_and_clear_byte`: Checks a value at offset 19088 of a4, calls a routine using a pointer from a6, loads a byte from offset -1 of a pointer in a0, pushes that byte and the pointer onto the stack, calls another routine, clears a byte at offset -80 of a5 using a masked value from a0, and then pushes several values onto the stack before calling another routine.
- `clear_d0_and_restore_a6`: Sets d0 to zero, restores a6 from the stack, unlocks a5, and returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `update_linked_list_node`: Takes a pointer from the stack, copies it to itself, increments the pointer by 4, clears the longword at the new pointer location, and stores the new pointer at an offset of 8 from the original pointer.
- `process_byte_stream`: Links a5, saves registers, loads a pointer from offset 8 of a5 into a3, and if a word at offset 18644 of a4 is less than 32, it checks a byte from a3; if that byte is not 20, 9, or 10, it branches, otherwise it increments a3 and branches back to the start of the routine.
- `jump_to_address_in_pointer_3`: Performs a jump to the address stored in a pointer located in a data structure.
- `jump_to_check_handler_status`: Jump stub: passes control straight on to check_handler_status.
- `initialize_window_logic`: Checks a value from a structure in a5, writes several words and longwords into a structure at offsets relative to a4, calls a routine using a pointer from a structure in a4, and branches to another routine based on the result of a comparison.
- `manage_window_state`: Compares a word from offset 8 of a5 to 2, 1, or 0, and depending on the result, either calls a routine at offset -72 from a pointer in a6 (loaded from offset 19088 of a4) after clearing a long at offset 19112 of a4, or calls a routine at offset -450 from a6 after loading a pointer from offset 19108 of a4 into a0.
- `update_menu_item`: Loads a3 from offset 8 of a5, calls a routine at offset -144 from a6 using a pointer from a3, stores the result at offset -4 of a5, and if the word at offset 14 of a5 is non-zero, performs an OR operation with 257 on the word at offset 12 of the structure pointed to by the value at offset -4 of a5.
- `update_menu_strip`: Clears a bit in a structure at offset 12 of a pointer from a5, then calls two routines using a pointer from a4 to update a menu strip with the contents of a3.
- `set_menu_bit`: Loads a3 from offset 8 of a5, calls a routine at offset -144 from a6 using a pointer from a3, stores the result at offset -4 of a5, and if the word at offset 14 of a5 is non-zero, sets bit 4 of the byte at offset 13 of the structure pointed to by a3 and clears the byte at offset 18 of that same structure.
- `execute_library_calls_2`: Calls two routines at offsets -342 and -354 from a6 using a pointer from offset 19092 of a4, calculates four values based on a structure pointed to by a0 (which is loaded from offset 12 of a5), pushes these values and a pointer from offset 8 of a5 onto the stack, and calls another routine.
- `call_subroutine_with_args_2`: Compares a long from offset 12 of a5 with a long at an offset of a4, and if they are equal, pushes several values onto the stack including -8, 15, d1, and the word from offset 8 of a5 before calling a routine at a relative address.
- `load_structure_pointer`: Loads a pointer from offset 8 of a5 into a0, copies words from offsets 8 and 10 of that pointer into d5 and d4, and checks if a longword at offset 19108 of a4 is non-zero to load it into a3.
- `compare_structure_fields`: Loads a pointer from offset 19104 of a4 into a3 and checks if it is zero.
- `scale_and_process_data`: If a word at offset 12 of a5 is non-zero, it performs divisions on d0 and d1, pushes the results to the stack, and calls another routine.
- `scale_and_store_values`: Divides d3 by 2, adds a word from offset -4 of a5 to the result, performs a second division using d5 and d2, performs arithmetic on bytes from offset 55 and 57 of a3 relative to a word at offset 10 of a3, and stores the final result in offset -40 of a5.
- `calculate_scaled_values_2`: Checks a word in a5, and if not less than or equal to zero, performs arithmetic involving bytes from a structure in a3, performs two divisions, and stores a result in offset -40 of a5 before branching to another routine.
- `calculate_scaled_quotients_2`: If d0 is not less than or equal to zero, it subtracts bytes from offset 54 and 56 of a3 from a word at offset 8 of a3, performs two division operations using d1 and d2, and stores the result in offset -40 of a5.
- `calculate_scaled_quotients`: Performs division on values from a5 and d3 using a divisor in d1, then stores the results in a5 and updates values from a4.
- `process_arithmetic_and_branch`: Adds a word from offset 24896 of a4 to d3, performs several comparisons and arithmetic operations on values stored at offsets 34, 36, and 30 of a5, and calls another routine if a comparison between d0 and d2 succeeds.
- `push_and_branch`: Pushes a word from offset c of a5 and a long from offset 8 of a5 onto the stack, calls a routine using a branch and link instruction, and then branches to another location.
- `calculate_scaled_indices`: Calculates two values by dividing the negation of words at offsets 34 and 36 of a5 by 8, stores these in the same offsets, and then updates offsets 30 and 32 of a5 based on a value from a global variable and the calculated values.
- `calculate_and_call_subroutine_2`: Performs arithmetic using values from a4 and a5, stores results in a5, and if a comparison between two values in a5 succeeds, calls another routine before branching to another location.
- `push_and_branch_3`: Pushes a word from offset 12 of a5 and a pointer from offset 8 of a5 onto the stack, calls a subroutine, and then branches to another location.
- `calculate_and_dispatch_values`: Performs several arithmetic operations using bytes and words from a structure in a3, calculates two new values in d6 and d7, pushes these values and others onto the stack, and calls two different routines.
- `process_arithmetic_and_messages`: Performs arithmetic operations using bytes from a structure in a3, calls a library function at a specific offset, and uses several library calls to modify IDCMP settings before performing arithmetic and updating a structure in a5.
- `calculate_arithmetic_results`: Performs arithmetic operations using bytes from a structure in a3 and values in d5 and d6, then writes a result to a structure in a5 and branches.
- `update_arithmetic_values`: Performs arithmetic on bytes from a structure in a3 and compares the result against values derived from a5, then branches based on the status of bytes at specific offsets in a5.
- `process_arithmetic_updates`: Performs several arithmetic operations on d0, d1, d3, d5, and d7, stores a word in a5 at offset -8, and updates values at offsets -4 and -11 of the structure in a5 based on various comparisons and conditional branches.
- `process_and_call_routine`: Performs arithmetic on d0, d1, d2, d3, and d6, updates a word at offset -2 of a5, and if certain conditions are met, sets a byte at offset -12 of a5 and calls a routine by pushing four extended long values and a pointer from offset 50 of a3 onto the stack.
- `set_bit_and_call_library`: Sets bit 0 of a byte at offset 25 of a structure in a3, then calls a routine using a pointer from a6 and branches to another routine.
- `update_window_parameters`: Writes two words from offsets relative to a5 into a structure pointed to by a pointer from a5, calls a routine using a pointer from a structure in a4, and then either calls another routine or calls a third routine after checking a word at offset -38 of a5.
- `increment_global_counter`: Increments a word at an offset relative to a4 and moves the value of a3 into d0 before restoring registers and returning.
- `manage_pointer_and_call`: Copies a pointer from offset 72 of a1 to offset -4 of a5, calls a routine at an offset from a pointer in a6, and then either calls another routine or copies a pointer from offset 19108 of a4 to offset -4 of a5.
- `manage_structure_and_call_2`: Loads a longword from offset 19104 of a4 into a0, stores it at an offset of minus 4 from a5, decrements a word at offset minus 17768 of a4, and performs several conditional branches and calls to routines using pointers from a6 and a1.
- `manage_structure_and_call`: Calls two library functions, compares a value in a0 with a value at a specific offset in a4, and if certain conditions are met, pushes two values to the stack and calls another routine.
- `compare_and_clear_bit`: Compares a longword at offset 19108 of a4 with a0, then calls two different routines, clears bit 0 of a byte at offset 37 of a structure in a0, and returns.
- `transfer_and_modify_byte`: Loads a long from offset 16 of a structure in a0 into offset -12 of a5, then loads another long from offset 18 of the new address into offset -8 of a5, and finally stores a byte from offset 12 of a5 into offset 4 of the address in a0.
- `transfer_pointer_data`: Loads a pointer from offset -12 of a5 into a0, moves a long from offset 26 of the address in a0 to offset -4 of a5, and if that long is non-zero, moves a byte from offset 12 of a5 into the address in a0 and moves a long from offset 16 of a0 to offset -4 of a5.
- `swap_pointer_values`: Loads a pointer from an offset relative to a5 into a0, then moves the long value at the address pointed to by a0 into a location relative to a5, and branches to another location.
- `update_structure_byte_2`: Loads a pointer from offset 8 of a5 into a0, moves a long from offset 24 of the address in a0 to offset -4 of a5, and if that long is non-zero, moves a byte from offset 12 of a5 into the address in a0 and moves a long from offset 16 of a0 to offset -4 of a5.
- `update_structure_byte`: Loads a0 from offset 8 of a5, moves a long from offset 14 of a0 to a location relative to a5, and if that long is non-zero, moves a word from offset c of a5 into d0, then moves that byte into offset 4 of the structure pointed to by the value at offset -8 of a5, and finally moves a long from offset c of that structure back to the location relative to a5.
- `unlock_address_register_five`: Releases the lock on address register five and returns.
- `free_and_clear_memory_block`: Checks a value at a global offset, and if it is between 3 and 7, calculates an address in a structure, calls a library routine to free memory with sizes 632 and 200, and then clears a long at that address and decrements a word at offset -2 of a5 in a loop.
- `jump_to_process_loop_with_library_call`: Jump stub: passes control straight on to process_loop_with_library_call.
- `jump_to_address_19`: Performs a long jump to the address contained in the constant value 1520.
- `jump_to_address_20`: Performs an indirect jump to the address stored in a longword.
- `jump_to_library_offset_forty`: Jumps to a library function at offset 40 from a base address loaded from a global variable.
- `jump_to_library_offset_zero`: Jumps to a library function at offset 0 from a base address loaded from a global variable.
- `jump_to_external_address_6`: Performs a long jump to a specific address in memory.
- `jump_to_address_8`: Jumps to an address stored in a longword at a specific location.
- `jump_to_address_40`: Performs an indirect jump to the address stored at the location specified by the jump instruction.
- `dispatch_and_clear_byte`: If a word at offset 8 of the structure in a5 is negative, it stores the byte 45 at an address calculated from a pointer in a4 and a value in a0, calls another routine, and then clears a byte at an address calculated from a pointer in a4 and a value in a0; otherwise it stores 45 at a calculated address and returns.
- `dispatch_and_store_byte_at_index`: Divides a word from offset 12 of the structure in a5 by 10; if the result is non-zero, it calls another routine with three parameters from the structure, then increments a word at offset 14 of the structure and stores a byte at an offset in a global variable calculated from the division of a word at offset 12 by 10, scaled and offset by 48.
- `process_system_messages`: Checks a byte at an offset of a4, calls a subroutine, checks a word at another offset of a4, and if not zero, uses a pointer from a4 to call a subroutine via a pointer in a6, potentially calling another subroutine via a6 if a value in d0 is not zero.
- `push_and_call_subroutine`: Pushes a 32-bit value from offset minus 4 of a5 onto the stack, calls a subroutine, adjusts the stack pointer, and branches to another location.
- `loop_and_clear_memory`: Enters a loop that checks a word at offset -6 of a5; if the value is less than 5, it calculates an address based on that value, calls another routine, clears a longword at offset 4 of the calculated address, and decrements the value at offset -6 of a5.
- `close_device_and_jump`: Tests a longword at an offset from a4, and if non-zero, calls a routine at an offset from a6, pushes a value from a4 to the stack, and jumps to a routine at a relative offset.
- `dispatch_indirect_jump`: Calculates a bitmask using a value from a global variable and a word from another global variable, calls a routine if the result is non-zero, and then performs a jump to an address determined by a value at offset 8 of a structure in a5 and a shifted value in d0.
- `push_registers_and_call_2`: Pushes d2 and several other values onto the stack, sets d0 to 64, sets d1 to 214, and calls another routine before restoring d2 from the stack and returning.
- `manage_data_structures`: Calculates an offset from a word in a5, copies 16 bytes from a4 to a destination, performs multiple subroutine calls to update various fields in structures pointed to by a4 and a5, and includes a loop using dbra.
- `process_message_data`: Extends a byte from an offset of a4, copies 16 bytes from a source at an offset of a4 to a destination in d0, and calls a routine using a pointer from a structure.
- `get_message_and_store_result`: Sets up a stack frame, moves a pointer from offset minus 18314 of a4 into a0, loads a pointer from offset 4 into a6, calls a subroutine at offset minus 372 of a6, stores the result in a local variable at offset minus 4 of a5, and branches to another location based on whether that result is zero.
- `free_memory_and_decrement_counter`: Links a5, restores a6 from the stack, sets a0 and a1 to the value stored at an offset of a5, loads 68 into d0, calls a routine at an offset of a6, decrements a word at an offset of a4, restores a6, and returns.
- `jump_to_indirect_address_2`: Performs a long jump to the address stored in a specific memory location.
- `jump_to_external_address`: Performs an unconditional jump to a long address.
- `jump_to_address_6`: Performs a jump to a longword address.
- `jump_to_zero`: Performs a jump to address zero.
- `execute_conditional_logic_5`: Calls a subroutine, stores a word from d0 into a structure at offset -2 of a5, and then checks a word at offset -24780 of a4 to decide whether to call another subroutine or branch.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_sequential_calls`: Saves a6, calls four different routines with various arguments on the stack, clears a word at an offset from a4, and branches based on the result of a test on d0.
- `dispatch_messages_and_call_library`: It calls a routine, clears a word at an offset of a structure in a5, calls a routine at an offset of a structure in a6, and then calls another routine at an offset of a structure in a6.
- `set_structure_fields`: Pushes a word from a global variable into a structure at offset -10, sets three words at offsets -36, -34, and -38 of a5, and branches based on values in a4.
- `validate_structure_values`: Compares a word at offset 8 of a5 against two constants and a word at offset 10 of a5 against two other constants, otherwise it moves d1 to d0 and branches.
- `process_event_callback`: Pushes two words onto the stack, calls a subroutine, stores the result at an offset of -4 in a5, and if a value at a specific offset in a4 is non-zero, it calls another subroutine, performs table lookups using values from a0 and a1, and calls two more subroutines before branching.
- `dispatch_event_callback_and_retry`: Tests a word at offset negative 36 of a structure, and if non-zero, calls a function pointer retrieved from a table and a secondary function pointer based on the result stored at offset negative 14 of the structure, before looping back to the start of the routine.
- `process_event_structure_and_dispatch`: Copies several fields from a structure pointed to by a stack offset into local stack storage, calls a routine via a function pointer, and if a specific longword field equals four, calls another routine, checks a global flag, and performs a stack-based call to another routine before calling two more routines and branching back to a previous instruction.
- `update_counter_conditionally`: Loads a longword from offset -18 of a5 into d1, and if it equals an address, increments a word at offset -38 of a5, compares that word to 2, and if it is greater than 2, sets d0 to 0 and stores it at offset -36 of a5 before branching to another location; otherwise it returns.
- `decrement_and_test`: Decrements d1 in a loop, and if the loop finishes, compares a word at offset -20 of a5 to 232, and if equal, sets d0 to 0 and stores it at offset -34 of a5.
- `update_structure_fields_4`: Compares d1 to 104, and if equal, sets a word at offset -34 of a5 to 2, clears a word at offset -38 of a5, loads four longwords from a5 into d0 through d3, calls a subroutine using a pointer from a4, and then conditionally updates a word at offset -36 of a5.
- `process_conditional_logic_5`: It clears a word at offset -36 of a5, compares a word at offset -34 of a5 to two, and if they are not equal, pushes a word from offset -4 of a5 to the stack and calls a subroutine.
- `process_conditional_logic_4`: Compares a word at offset -34 of a5 to one, calls several routines using values from a5 and a4 on the stack, and performs an addition on a value from d0 before branching.
- `execute_indirect_call_with_offset`: Pushes a word from offset -10 of a structure in a5 onto the stack, calls a routine at a relative offset, and then sets d0 to 10 before branching to another routine.
- `call_relative_subroutine`: It pushes a word from an offset of a structure in a5 onto the stack, calls a routine, and then branches to another location.
- `dispatch_status_value_2`: Pushes a word from a structure in a5 onto the stack, calls a routine, increments the stack pointer, compares a word from a global variable to a value in a4, and conditionally calls another routine.
- `dispatch_status_value`: Compares a word from a global variable against constants, pushes two words from a structure onto the stack, calls a subroutine, and updates a word in a structure.
- `call_and_branch_3`: Calls another routine using a relative offset, clears d0, and branches to another routine.
- `execute_indirect_call_with_stack_value`: Pushes a word from offset -4 of a structure in a5 onto the stack, calls a routine at a relative offset, sets d0 to 10, and branches to another routine.
- `execute_indirect_call`: Pushes a word from offset -4 of a structure in a5 onto the stack, calls a subroutine at a relative offset, increments the stack pointer, and restores registers from the stack before returning.
- `compare_and_branch_7`: Links a5, saves registers, and compares a word from offset 8 of a5 to 80; if it is not less than 80, it sets d0 to zero and branches to another routine.
- `mask_and_test_word`: Performs a bitwise AND on a word from offset a of a structure in a5 with the constant 192, and if the result is non-zero, sets d0 to zero and branches; otherwise it branches to another location.
- `jump_via_offset`: Compares d1 to 0, and if it is between 0 and 10, subtracts 1 from d1 and performs a jump to an address calculated from the program counter plus the value in d1 shifted left by 1.
- `process_conditional_bit_flags`: Tests various bits in a structure at offset 11 of a5 and sets bits in a structure at offset 8 of a5, then performs arithmetic and comparisons involving a structure in a4 and a value in d7.
- `increment_and_branch_2`: Increments d7 by one and branches to another routine.
- `execute_short_sequence`: Pushes a word of 1 onto the stack, calls another routine, restores d7 and a2 from the stack, and returns.
- `calculate_and_store_quotients_2`: It calculates two quotients using division, stores several words at various negative offsets of a structure in a5, and branches to another routine if a word at offset 8 of a5 is non-zero.
- `call_library_and_modify_value`: Calls a library routine using a pointer from a6, and if a word at offset 8 of a5 is non-zero, modifies a value at offset -68 of a5 and reads a word from a calculated address.
- `calculate_and_store_results_2`: It writes a longword and a byte to a structure, calls a subroutine, performs arithmetic on values from a5 to update offsets in a5, calls another subroutine, and branches if d0 is not zero.
- `modify_structure_fields_2`: Pushes a value onto the stack, calls a routine, increments the stack pointer, and then conditionally branches based on a word at an offset in a5.
- `modify_structure_fields`: Sets bit 0 of a byte at an offset in a structure, writes a word to a structure, and calls a subroutine twice using a pointer from a structure if certain conditions are met.
- `transfer_structure_data`: Copies several fields from a structure pointed to by a memory offset in a5 to other offsets in a5, calls a routine using a pointer from a6, and if a specific value in a5 is four, performs several more calls and jumps.
- `process_conditional_logic_3`: Performs several conditional checks on a structure in a5, and if conditions are met, it calls multiple subroutines using pointers from a structure in a4 and updates values in a structure in a5.
- `clear_structure_word`: Clears a 16-bit word at offset -42 of a structure in a5 and branches to another routine.
- `clear_structure_fields`: Clears two 16-bit words at offsets -42 and -32 of a structure in a5, then checks if a longword at offset -50 of a5 is 8; if so, it compares a word at offset -54 of a5 to 69, sets bit 1 of a byte at offset 15 of a5, and branches to another routine.
- `set_structure_byte`: Compares d3 to 104 and, if equal, clears bit 0 of a byte at offset 6 of a5, stores 1 in a word at offset -34 of a5, and branches to another location.
- `modify_structure_bits_2`: Compares d3 to 232; if not equal, it branches to another location, otherwise it clears bit 0 of a byte at offset 6 of a5, moves d0 to offset -40 of a5, and either moves 17 to offset -32 of a5 or branches to another location based on bit 1 of a byte at offset 6 of a5.
- `process_conditional_values`: Compares d2 to 4096 and, if equal, pushes two words from a5 onto the stack, calls another routine, and then performs several comparisons on d0 before potentially clearing a word at offset -40 of a5 and branching to another location.
- `process_conditional_logic_2`: Pushes d0 to the stack, calls a subroutine, compares a word from a global structure in a4 to 1 or 2, and if not matching, checks a word in a global structure; it then calls another subroutine, clears bit 25 of a structure pointed to by a0, and potentially calls a third subroutine based on a word at offset -42 of a5.
- `calculate_split_values`: If a word at offset -54 of a5 is not zero, calls a subroutine, performs division on a word from offset 22632 of a4 by 100 and by 100, stores the quotients in a5, and calculates an absolute difference between a value in a5 and a value at offset -6 of a5.
- `calculate_absolute_difference_7`: Stores d2 in d3, subtracts a word in d0 from d3, and calculates the difference between a word from offset -12 of a5 and a word in d1.
- `calculate_absolute_difference_6`: Subtracts a word in d1 from a longword in d4 and compares the result to d3.
- `move_and_branch_7`: Subtracts d0 from d2, stores the result in d3, and branches to another routine.
- `calculate_absolute_difference_5`: Subtracts d1 from d4, stores the result in d0, and if the result is negative, negates it before branching.
- `process_and_call_subroutines`: Subtracts d1 from d4, stores the result in a structure in a5, compares it to a value at a specific offset in a4, and if it is not less than or equal, calls two different subroutines with specific stack setups before branching to another routine.
- `compare_and_call_conditional`: Tests a word at offset -42 of a5; if non-zero, it pushes 2 onto the stack, calls a routine with a pointer from a5, then compares a word from offset -24898 of a4 with a word from offset -2 of a5, calling another routine with decremented d1 and values from a4 if the comparison is not greater than zero.
- `compare_and_call_twice`: Adds d1 to a word from offset 20324 of a4, and if the result is not less than d2, increments d1 and calls a routine with d1 and a value from offset 24896 of a4, then potentially performs a second call with d1 and a value from offset 24898 of a4.
- `calculate_average_and_call`: Adds a word from a global variable to d1, compares the result to d2, and if the result is not less than d2, increments d1, calls another routine with two words from the stack, and checks a word at an offset of a5.
- `calculate_and_store_averages`: Calls several routines using pointers from a4, performs arithmetic to calculate averages from values in a5, and stores the results back into a5 before calling more routines and setting a byte at offset minus 42 of a5.
- `perform_stack_operations`: Clears a word at offset -42 of a5, pushes two words from a5 and two values from a4 onto the stack, calls two different routines, and then performs arithmetic using words from a5 before storing results in offsets -6 and -8 of a5.
- `call_subroutine_with_stack`: Sets d0 to 1, stores it at offset -38 of a5, pushes a pointer from offset -20 of a5 onto the stack, pushes two words from offset -8 and -6 of a5 onto the stack, calls another routine, cleans the stack, and branches to another location.
- `calculate_and_store_value`: Calls two different routines, clears a bit in a location pointed to by a0, performs a multiplication and addition on a value from a5, and stores the result in a location pointed to by a0.
- `process_conditional_table_pointers`: Compares a word from a table with a value at offset 8 of a structure in a5, calls a subroutine using pointers from a4, and if a condition is met, enters a loop that calculates an offset, fetches a byte from a table, and calls another subroutine.
- `dispatch_via_table_6`: Links a5, saves registers, compares a word from offset 8 of a5 to 50, and if it is not less than 50, performs a jump to an address calculated by shifting d0 left by 2 and adding it to a base address.
- `branch_to_offset_3`: Performs a short branch to an offset.
- `clear_and_branch_4`: Sets d0 to 0 and branches to another routine.
- `call_if_match`: Compares a word at offset -24656 of a4 to 8, and if they are equal, it pushes a word of 1 onto the stack and calls another routine before returning.
- `process_loop_with_library_call`: Calculates an address based on a word at offset -18500 of a4, performs several byte extractions and additions to update values at negative offsets of a5, calls a library routine using a pointer from a6, and then calls another routine using a pointer from a0 while pushing four extended longwords onto the stack.
- `process_and_call_subroutine`: Extends a word from a5 into a longword, calls a routine using a pointer from a4, extends four words from a5 into longwords, pushes them onto the stack, pushes a longword from a structure in a0 onto the stack, calls another routine, and then modifies several values in a5 and d6 before branching.
- `calculate_offset_values_2`: Compares a word from offset -24896 of a4 to 2; if less than 2, it loads a byte from offset 54 of a structure in a0, extends it, adds 1, and stores it at offset -8 of a5, then performs a calculation involving d1 and d3 to store a second value at offset -4 of a5.
- `calculate_offset_values`: Loads a byte from a structure in a0, performs arithmetic to calculate and store a value at offset minus 8 of a5 and another at offset minus 4 of a5, then checks a value from a4 to decide whether to calculate and store three additional values at offsets minus 6, minus 2, and another location in a5.
- `perform_library_operations`: Calculates two values based on a byte from a0 and stores them at offsets minus 6 and minus 2 of a5, then performs five calls to library functions using arguments derived from a0, a4, and the stack before restoring registers and returning.
- `execute_indirect_jump_2`: Performs an indirect jump using the address stored in a longword at a specific location.
- `jump_to_address_27`: Performs an indirect jump to an address stored in a longword.
- `jump_to_lock`: Performs a jump to an address.
- `jump_to_address_3`: Performs a long jump to the address contained in a0.
- `jump_to_address_23`: Performs a long jump to the address stored at offset 2ea of the current instruction.
- `jump_to_conditional_data_update`: Performs a long jump to the process_conditional_data_update routine.
- `jump_to_offset_2424`: Jumps to the address stored at offset 2424.
- `jump_to_calculation_routine`: Performs a jump to another routine that calculates and stores four values by reading bytes from a structure in a4 and writing the results to a structure in a5.
- `jump_to_offset_23ac`: Jumps to the address stored at offset 23ac.
- `jump_to_null_pointer`: Performs a jump to a null address.
- `jump_to_address_24`: Performs a long jump to the address stored at offset 3e4 of the current instruction.
- `jump_to_address_25`: Performs a long jump to the address stored at offset 3fc of the current instruction.
- `jump_to_offset_238e`: Jumps to the address stored at offset 238e.
- `jump_to_external_routine_6`: Jumps to another routine located at a long address.
- `jump_to_address_15`: Performs a long jump to the address stored in the constant value 1054.
- `jump_to_address_16`: Performs a long jump to the address stored in the constant value 1520.
- `jump_to_zero_address`: Performs a long jump to address zero.
- `jump_to_address_36`: Performs a long jump to the address stored in the longword at 746.l.
- `jump_to_address_41`: Performs a long jump to the address stored at offset 25 of a1.
- `jump_to_resident_library_stub`: Performs an indirect jump to a routine that loads arguments and the executive library base to call a resident function.
- `jump_to_address`: Performs a long jump to the address stored at a specific location in memory.
- `jump_to_offset_2442`: Jumps to the address stored at offset 2442.
- `jump_to_fixed_address`: Performs a long jump to the address 16512.
- `jump_to_offset_2384`: Jumps to the address stored at offset 2384.
- `jump_to_address_39`: Performs a long jump to the address stored at offset 800.
- `jump_to_offset_242e`: Jumps to the address stored at offset 242e.
- `jump_to_indirect_address_12`: Performs an indirect jump to an address stored in a memory location.
- `jump_to_absolute_address_5`: Performs a long jump to a specific absolute address.
- `jump_to_offset_23a2`: Jumps to the address stored at offset 23a2.
- `jump_to_offset_23e8`: Jumps to the address stored at offset 23e8.
- `jump_to_indirect_address_8`: Performs an indirect jump to a long address stored at a specific memory location.
- `jump_to_offset_246a`: Jumps to the address stored at offset 246a.
- `jump_to_library_stub`: Performs an indirect jump to a routine that handles writing via the DOS library.
- `jump_to_subroutine_2`: Performs an absolute long jump to another routine.
- `jump_to_address_short`: Performs a long jump to the address stored in the longword at 242.l.
- `store_constant_and_jump`: Checks a value from the stack and, if non-zero, stores the constant 5584 into a structure offset before jumping to another routine.
- `jump_to_subroutine_3`: Loads the value 104 into register d0 and jumps to another routine.
- `jump_to_external_routine_5`: Jumps to another routine located at a fixed address.
- `perform_indirect_jump`: Performs an indirect jump to the address stored at a constant offset.
- `jump_to_address_34`: Performs an indirect jump to the address stored at offset 16512.
- `jump_to_external_routine_7`: Performs a long jump to another routine after being called by process_conditional_logic_4.
- `jump_to_offset_36`: Performs a long jump to a routine that sets register d0 to 16 and then continues execution at an offset of 36.
- `jump_to_indirect_address_3`: Performs a long jump to the address stored at a specific location.
- `set_d0_and_jump`: Sets d0 to 0 and jumps to an offset of 36.
- `set_d0_and_jump_2`: Sets d0 to 8 and jumps to an offset of 36.
- `set_d0_and_jump_3`: Sets d0 to 16 and jumps to an offset of 36.
- `set_d0_and_jump_4`: Sets d0 to 24 and jumps to an offset of 36.
- `load_d0_with_32`: Loads the word 32 into register d0 and jumps to another routine.
- `load_d0_with_40`: Loads the word 40 into register d0 and jumps to another routine.
- `load_d0_with_48`: Loads the word 48 into register d0 and jumps to another routine.
- `load_d0_with_56`: Loads the word 56 into register d0 and jumps to another routine.
- `load_d0_with_64`: Loads the word 64 into register d0 and jumps to another routine.
- `load_d0_with_72`: Loads the word 72 into register d0 and jumps to another routine.
- `load_d0_with_80`: Loads the word 80 into register d0 and jumps to another routine.
- `load_d0_with_88`: Loads the word 88 into register d0 and jumps to another routine.
- `load_d0_with_96`: Loads the word 96 into register d0 and jumps to another routine.
- `load_d0_with_104`: Loads the word 104 into register d0 and jumps to another routine.
- `load_index_70`: Loads the word value 112 into d0 and jumps to another routine.
- `load_index_78`: Loads the word value 120 into d0 and jumps to another routine.
- `load_index_80`: Loads the word value 128 into d0 and jumps to another routine.
- `load_index_88`: Loads the word value 136 into d0 and jumps to another routine.
- `load_index_90`: Loads the word value 144 into d0 and jumps to another routine.
- `load_index_98`: Loads the word value 152 into d0 and jumps to another routine.
- `load_index_a0`: Loads the word value 160 into d0 and jumps to another routine.
- `load_index_a8`: Loads the word value 168 into d0 and jumps to another routine.
- `load_index_b0`: Loads the word value 176 into d0 and jumps to another routine.
- `load_index_b8`: Loads the word value 184 into d0 and jumps to another routine.
- `set_d0_and_jump_5`: Loads the word value 192 into d0 and performs a long jump to an address.
- `set_d0_and_jump_c8`: Loads the word value 200 into d0 and performs a long jump to an address.
- `set_d0_and_jump_d0`: Loads the word value 208 into d0 and performs a long jump to an address.
- `execute_library_call_loop`: Performs a sequence of library calls using a pointer from a4, decrements a value at offset 8 of a5, and repeats the process if the value is greater than zero.
- `calculate_and_call_quotients`: Links a5, divides a word from offset 8 of a5 by 100 and subtracts a word from offset 20326 of a4, then divides a word from offset 8 of a5 by 100, swaps the result, subtracts a word from offset 20320 of a4, pushes both results to the stack, calls another routine, and returns.
- `clear_and_call_routine`: Calculates an address using a4, clears a byte at that address, pushes several registers onto the stack, calls another routine, and clears a word at an offset of a4 before returning.
- `update_structure_values_2`: Checks bits in a structure at a0 and, if conditions are met, calls a library routine, compares a value at offset -12 of a5 with a value at offset -25816 of a4, and either adds a constant to offset -8 of a5 or decrements offset -12 of a5.
- `compare_and_decrement_structure`: Compares a longword at offset -8 of a5 with a longword at offset -25844 of a4, and if the value in a5 is not greater, it adds a constant to offset -4 of a5, decrements offset -8 of a5, and branches back to the start.
- `compare_and_decrement_value`: Compares a longword at offset -10672 of a4 with a longword at offset -4 of a5, and if the value in a5 is not greater, it decrements a word at offset -24790 of a4 and calls a subroutine.
- `update_structure_counters`: Calls a subroutine, then performs arithmetic to update three longword values at offsets -25816, -25812, and -25808 of a structure in a4 based on a value at offset -25802.
- `copy_structure_members`: Copies three longwords from offsets -12, -8, and -4 of the structure pointed to by a5 into offsets -25816 and -25812, -25808, and -25804 of the structure pointed to by a4, then restores registers and returns.
- `negate_word_if_negative`: Loads a word from an offset of a4 into d1, and if that word is negative, negates it and moves it into d0 before branching.
- `process_signed_value_and_divide`: Links a5, tests a 16-bit value from a structure, and if negative, calls a routine with 45 in d0; otherwise, it performs several divisions and calls a routine multiple times with modified values.
- `copy_bytes_loop_2`: Copies bytes from a source at offset -467f of a4 to a destination at offset -d4 of a5 in a loop, otherwise it branches.
- `manage_memory_and_calls`: Copies bytes from a table in a4 to a destination in a5 in a loop, then performs several conditional calls to subroutines using various parameters from a4 and a5.
- `push_parameters_and_call_2`: Pushes three longwords onto the stack, calls a subroutine, resets the stack pointer, pushes two more longwords, calls another subroutine, and then branches to another location.
- `copy_bytes_and_call_subroutine`: Copies bytes from one location to another in a loop, then pushes two values onto the stack and calls another routine.
- `compare_and_modify_values`: Calls a library routine using a pointer from a5, compares a longword from offset -12 of a5 with a longword at offset -26666 of a4, and if the comparison passes, adds a constant to offset -8 of a5 and decrements offset -12 of a5.
- `execute_library_tasks_with_loop`: Calls several library functions, performs arithmetic on bytes from a structure, calls another routine with five arguments, and then enters a loop that calls a final library function based on a byte test.
- `process_conditional_data_update`: Calls a routine, and if it returns with a non-zero d0, checks a byte in a5; if that byte is non-zero, it performs arithmetic on a value from a4 to compare it with a word in a table, potentially updates a value in a4, and calls another routine.
- `compare_and_dispatch`: Links a5, saves d2, loads a word from offset minus 30736 of a4 into d0, calls another routine, calls a second routine, compares d0 with a word at offset minus 24898 of a4, and if they are not equal, pushes two words to the stack and calls another routine before returning.
- `calculate_and_call_handler_2`: Calculates a value by shifting and subtracting a word from a global variable, calls a routine with that value, and if a comparison with another global variable fails, pushes two words onto the stack and calls another routine.
- `update_and_call_library_routines`: Performs several library calls using a base pointer in a4, calculates several values from bytes in a structure in a4, pushes these values onto the stack, calls a routine, and then conditionally calls more library routines and another routine based on a bit test in a5 and a comparison.
- `execute_complex_library_sequence`: Performs several library calls, calculates multiple values from bytes in a structure in a4, pushes these values onto the stack, calls a routine, and then conditionally calls more library routines and another routine based on a bit test in a5 and a comparison.
- `process_structure_data_2`: Calls two library functions using pointers from a structure in a4, decrements a word at offset -18 of a5, and if the result is non-zero, performs arithmetic to update several values in a5 before calling another routine and two more library functions.
- `evaluate_and_update_values`: Calls two library functions, decrements a word at offset -18 of a5, and if non-zero, performs checks against a structure in a4 before calling another routine twice and two library functions.
- `compute_scaled_values`: Calls several routines, performs a loop to check a byte in a table, calls another routine to calculate values, performs arithmetic to average results into a5, and calls more routines to store the final values.
- `calculate_quotients_and_call_subroutines`: Divides a word from offset 8 of a5 by 100 twice to obtain values in d6 and d7, calls three different routines using the results, and then performs two conditional calls to another routine using values from offset -8 and -6 of a5 and a pointer from offset 19116 of a4.
- `process_byte_from_pointer`: Loads a pointer from an offset of a structure in a5, checks if the byte at that pointer is non-zero, and if so, pushes a signed byte to the stack and calls another routine before branching.
- `calculate_and_call_handler_3`: Calculates an offset by adding a word from a structure in a5 to a value at a base address in a4, retrieves a byte from that calculated address, and calls a subroutine with the resulting word and byte on the stack.
- `update_byte_at_offset`: Compares a 16-bit value from a global variable against 98, and if it is greater than or equal, copies a byte from offset 9 of a structure in a5 to a destination calculated by adding the value to an offset of a4, then increments the global variable by 1.
- `evaluate_and_call_subroutines`: Copies a word from offset 8 of a5 to offset 22632 of a4, performs several comparisons and byte extractions from a4, and calls two subroutines with values from the stack.
- `calculate_structure_offsets`: Loads a word from an offset of a4, uses it to calculate several offsets to read bytes from a structure, stores these bytes into offsets of a5, and then conditionally reads another byte from a structure and branches.
- `process_data_sequences`: Copies a byte from a source at offset 22636 of a4 to a destination at offset 24726 of a4 in a loop, then performs a series of checks and calls to subroutines and a library function.
- `execute_multiple_subroutines_with_math`: Calls a routine three times with different stack parameters, performs a multiplication, calls a routine again, and then enters a loop that reads bytes from a table at a3 and adds them to offset -2 of a5.
- `execute_multiple_subroutines_2`: Pushes two words onto the stack, calls a subroutine, stores the result in offset -8 of a structure in a5, calls another subroutine, and if the value at offset -8 of a5 is 2, it calls a third subroutine.
- `validate_and_call_handler`: Copies a byte from a4 to a5, checks a word at offset 8 of a5 and a byte at offset -24777 of a4, then calls another routine with three parameters and branches to another routine if d0 is not zero.
- `execute_conditional_calls_5`: Pushes two longwords from a4 onto the stack and calls another routine, stores the result in offset -4 of a5, and if d0 is not zero, performs a second set of calls and branches to another routine.
- `update_structure_state`: Copies bytes between locations in a4 and a5, performs several calls using parameters from a4 and a5, calls a library routine via a pointer in a6, and branches to another routine if d0 is not zero.
- `process_structure_data_4`: Copies a byte from a location in a4 to another location in a4, then performs several calls using values from a5 and a4, eventually writing a word from d0 into offset -86 of a5 and branching to another routine.
- `call_subroutine_and_load_word`: Pushes two longwords onto the stack, calls another routine, and then loads a word from an offset of a5 into d0 before returning.
- `branch_based_on_byte_value`: Loads a byte from offset -24638 of a4, compares a word from offset 12 of a5 to 1, and branches to another routine, another routine, or performs a long jump to an address stored in a location.
- `store_pc_relative_address`: Stores the address of a location relative to the program counter into a location at offset -12 of a5 and performs a branch to another routine.
- `clear_d0_and_branch_5`: Clears register d0 to zero and performs a short branch to another routine.
- `clear_and_process_offset_2_a5_data`: Clears a word at offset 19068 of a4, calls a routine with a pointer to offset -2 of a5 and a count of 2, then adds a byte from a calculated offset to the global variable.
- `process_offset_6_a5_data`: Adds a word from offset -2 of a5 to a global variable at offset 19068 of a4, then calls a routine with a pointer to offset -6 of a5 and a count of 2, and adds a byte from a calculated offset to the global variable.
- `process_offset_18296_data`: Calls a routine with a pointer to offset -18296 of a4 and a count of 2, then adds a byte from a calculated offset to a global variable at offset 19068 of a4.
- `process_offset_18594_data`: Calls a routine with a pointer to offset -18594 of a4 and a count of 2, then adds a byte from a calculated offset to a global variable at offset 19068 of a4.
- `process_offset_24794_data`: Calls a routine with a pointer to offset -24794 of a4 and a count of 2, then adds a byte from a calculated offset to a global variable at offset 19068 of a4.
- `process_offset_18580_data`: Calls a routine with a pointer to offset -18580 of a4 and a count of 2, then adds a byte from a calculated offset to a global variable at offset 19068 of a4.
- `process_offset_24940_data`: Calls a routine with a pointer to offset -24940 of a4 and a count of 2, then adds a byte from a calculated offset to a global variable at offset 19068 of a4.
- `process_offset_24898_data`: Calls a routine with a pointer to offset -24898 of a4 and a count of 2, then adds a byte from a calculated offset to a global variable at offset 19068 of a4.
- `process_fixed_count_offset_24898`: Calls another routine with a pointer to offset -24898 of a4 using a count of 2, then adds a byte from a calculated offset to a global variable.
- `process_fixed_count_offset_24814`: Calls another routine with a pointer to offset -24814 of a4 using a count of 2, then adds a byte from a calculated offset to a global variable.
- `process_fixed_count_offset_20328`: Calls another routine with a pointer to offset 20328 of a4 using a count of 600, then adds a byte from a calculated offset to a global variable.
- `process_fixed_count_offset_24812`: Calls another routine with a pointer to offset -24812 of a4 using a count of 8, then adds a byte from a calculated offset to a global variable.
- `process_fixed_count_offset_24804`: Calls another routine with a pointer to offset -24804 of a4 using a count of 8, then adds a byte from a calculated offset to a global variable.
- `process_multiplied_offset_20928`: Multiplies the value at offset -18580 of a4 plus one by 24, calls another routine with a pointer to offset 20928 of a4, and then adds a byte from a calculated offset to a global variable.
- `process_doubled_offset_21224`: Doubles a 16-bit value from offset -24794 of a4, calls another routine with a pointer to offset 21224 of a4, and then adds a byte from a calculated offset to a global variable.
- `process_doubled_offset_21424`: Doubles a 16-bit value from offset -24794 of a4, calls another routine with a pointer to offset 21424 of a4, and then adds a byte from a calculated offset to a global variable.
- `process_doubled_offset_21624`: Doubles a 16-bit value from offset -24794 of a4, calls another routine with a pointer to offset 21624 of a4, and then adds a byte from a calculated offset to a global variable.
- `process_doubled_offset_22024`: Doubles a 16-bit value from offset -24794 of a4, calls another routine with a pointer to offset 22024 of a4, and then adds a byte from a calculated offset to a global variable.
- `process_offset_22024_doubled`: Doubles a 16-bit value from offset -24794 of a4, calls another routine with a pointer to offset 22024 of a4, and loops to add bytes from that sequence to a global variable offset 19068.
- `process_offset_22224_shifted`: Shifts a 16-bit value from offset -24794 of a4 left by 2, calls another routine with a pointer to offset 22224 of a4, and loops to add bytes from that sequence to a global variable offset 19068.
- `iterate_offset_negative_24792`: Calls another routine with a pointer to offset -24792 of a4 and loops 2 times to add bytes from that sequence to a global variable offset 19068.
- `iterate_offset_22624`: Calls another routine with a pointer to offset 22624 of a4 and loops 8 times to add bytes from that sequence to a global variable offset 19068.
- `iterate_offset_22632`: Calls another routine with a pointer to offset 22632 of a4 and loops 2 times to add bytes from that sequence to a global variable offset 19068.
- `iterate_offset_negative_24790`: Calls another routine with a pointer to offset -24790 of a4 and loops 2 times to add bytes from that sequence to a global variable offset 19068.
- `iterate_offset_negative_24788`: Calls another routine with a pointer to offset -24788 of a4 and loops 8 times to add bytes from that sequence to a global variable offset 19068.
- `iterate_offset_22720`: Calls another routine with a pointer to offset 22720 of a4 and loops 8 times to add bytes from that sequence to a global variable offset 19068.
- `process_offset_22716`: Checks if a 16-bit value at a global variable offset -2 is 7237, otherwise doubles a 16-bit value from offset -18296 of a4, calls another routine, and loops to add bytes from a sequence starting at offset 22716 of a4 to a global variable offset 19068.
- `accumulate_byte_offset_values_2`: Doubles a 16-bit value from a global variable, calls another routine with a pointer to a location in a4, and then loops to add bytes from a sequence starting at that pointer to a value at a global variable offset 19068.
- `accumulate_byte_offset_values`: Calls another routine with a pointer to a location in a4, then iterates through a sequence of bytes to add each byte value to a value at an offset of 77928 in a4.
- `update_table_from_byte_offset`: Calls another routine with a pointer to a location in a4, then iterates through a sequence of bytes to add each byte value to a value at a specific offset in a4.
- `update_table_from_one_sixty`: Calls a routine with a pointer at offset 23016 of a4, then enters a loop that iterates 160 times, reading a byte from a table and adding it to a value at offset 19068 of a4.
- `update_table_from_twenty_three_one_eighty`: Calls a routine with a pointer at offset 23180 of a4, then enters a loop that iterates 8 times, reading a byte from a table and adding it to a value at offset 19068 of a4.
- `accumulate_table_values_via_subroutine`: Calls a subroutine using a pointer from a structure, then iterates through a table to add bytes from the table to a value at a specific offset in a global variable.
- `call_routine_and_accumulate_table_values`: Calls a routine using a pointer from a structure, then iterates through a table to add bytes from the table to a value at a specific offset in a structure.
- `update_table_from_small_count`: Calls a routine with a pointer at offset -24612 of a4, then enters a loop that iterates 2 times, reading a byte from a table and adding it to a value at offset 19068 of a4.
- `update_table_from_negative_offset`: Calls a routine with a pointer at offset -18590 of a4, then enters a loop that iterates 8 times, reading a byte from a table and adding it to a value at offset 19068 of a4.
- `update_table_from_eight`: Calls a routine with a pointer at offset 23200 of a4, then enters a loop that iterates 8 times, reading a byte from a table and adding it to a value at offset 19068 of a4.
- `update_table_from_sixty_four`: Calls a routine with a pointer at offset 23208 of a4, then enters a loop that iterates 64 times, reading a byte from a table and adding it to a value at offset 19068 of a4.
- `process_item_list_and_update_table`: Pushes two small values onto the stack, calls another routine using a pointer from a structure, and then enters a loop that iterates through a list of items to add a byte value from a table to a global variable at a specific offset.
- `process_seventy_two_items`: Pushes 72 and 1 onto the stack, calls a subroutine using a pointer from a structure, and then enters a loop that iterates 71 times to add a byte from a table to a value at an offset of 19068 from a base address.
- `fetch_indexed_data_via_subroutine`: Pushes two integer constants onto the stack, calls a routine via a pointer in a structure, and then calculates an address by adding an offset to a base address, retrieves a byte from that address, and adds it to a value at a fixed offset from a global pointer.
- `call_subroutine_and_fetch_byte_2`: Pushes two small integer constants onto the stack, calls a subroutine via a pointer in a structure, and then calculates an address by adding a byte from a memory location to a base offset before branching.
- `check_and_process_four`: Compares a word at an offset of -2 in a structure to 11297; if it matches, it pushes 4 and 1 onto the stack, calls a subroutine, and enters a loop that iterates 3 times to add a byte from a table to a value at an offset of 19068 from a base address.
- `call_subroutine_and_fetch_byte`: Pushes four and one onto the stack, calls a subroutine using a pointer from a structure, and then retrieves a byte from a table using an offset derived from the stack pointer and adds it to a value at a fixed offset in a global buffer.
- `process_four_items`: Pushes 4 and 1 onto the stack, calls a subroutine using a pointer from a structure, and then enters a loop that iterates 3 times to add a byte from a table to a value at an offset of 19068 from a base address.
- `call_function_and_update_value`: Pushes two integer arguments onto the stack, calls a routine via a pointer from a structure, and if the second argument is not zero, reads a byte from a table and adds it to a value at a fixed offset from a base pointer.
- `process_two_items`: Pushes 2 and 1 onto the stack, calls a subroutine using a pointer from a structure, and then enters a loop that iterates 1 time to add a byte from a table to a value at an offset of 19068 from a base address.
- `process_eight_items`: Pushes 8 and 1 onto the stack, calls a subroutine using a pointer from a structure, and then enters a loop that iterates 7 times to add a byte from a table to a value at an offset of 19068 from a base address.
- `process_doubled_count_table`: Calculates a doubled word from a global variable, pushes it and a constant onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset 23366 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_18526`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset -18526 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_23386`: Pushes a word from a global variable and a constant onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset 23386 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_18524`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset -18524 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_23486`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset 23486 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_18500`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset -18500 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_18462`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset -18462 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_offset_18390`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at offset -18390 from a4, adding a byte from the table to a value at offset 19068 of a4.
- `process_table_entries_2`: Pushes two values onto the stack, calls a subroutine using a pointer from a structure, and then iterates through a table starting at an offset from a4, adding a byte from the table to a value at offset 19068 of a4.
- `iterate_table_entries`: Pushes several values onto the stack, calls a subroutine, and then performs a calculation involving a byte from a table to add a value to a global variable, branching to another routine if the constant 192 is not zero.
- `execute_table_update`: Doubles a word from a global variable, pushes several values onto the stack, calls a subroutine, and then performs a calculation involving a byte from a table to add a value to a global variable, branching to another routine if the doubled value is not zero.
- `update_table_value_3`: Doubles a word from a global variable, pushes several values onto the stack, calls a subroutine, and then performs a calculation involving a byte from a table to add a value to a global variable.
- `call_subroutine_and_update_table_3`: Doubles a word from a global variable, pushes it and a constant onto the stack, and calls a subroutine via a pointer.
- `calculate_and_call_subroutine_3`: Doubles a word from offset -24794 of a4, pushes a longword from offset 8 of a5 and two words to the stack, calls a subroutine via a pointer in a5, and if d0 is non-zero, it calculates an address using a byte from a table and adds it to a value at offset 19068 of a4 before branching to another location.
- `process_offset_data`: Pushes a longword from offset 8 of a5 and two words to the stack, calls a subroutine via a pointer in a5, and if d0 is non-zero, it calculates an address using a byte from a table at a specific offset and adds it to a value at offset 19068 of a4 before branching to another location.
- `fetch_and_offset_data`: Pushes two values onto the stack, calls a routine via a pointer from a5, and then calculates a new address by adding a byte from a data structure to a constant offset and branching to another location.
- `update_global_counter`: Pushes several values onto the stack, calls a subroutine using a pointer from a5, and then uses the result to calculate an offset to add a byte from a memory location to a global variable at offset 19068 of a4.
- `update_value_via_callback`: It calculates a value from an offset of a4, calls a subroutine using a pointer from a5 and values from the stack, and then performs a loop that updates a value at an offset of a4 using a byte from a table.
- `execute_callback_and_update_2`: Calls a routine using a pointer from a5 and several arguments from a4 and a5 on the stack, then adds a byte from a table to a value at offset 19068 of a4.
- `execute_callback_and_update`: Calls a subroutine using a pointer from a5 with several arguments on the stack, then updates a value in a table at an offset from a4 based on the result.
- `call_subroutine_and_update_value`: Pushes several values onto the stack, calls a subroutine using a pointer from a5, and if the return value is non-zero, calculates an offset to add a byte from a table to a value at a specific offset in a4.
- `call_subroutine_and_update_table_2`: Pushes several values onto the stack, calls a routine using a pointer from a5, and if the return value is non-zero, calculates an offset and updates a byte in a table.
- `call_and_increment_offset_3`: Pushes several values onto the stack, calls a routine using a pointer from a structure in a5, and if the return value is non-zero, updates a global variable using a byte from a calculated address.
- `call_and_increment_offset_2`: Pushes three values onto the stack, calls a subroutine using a pointer from a structure, and if the returned value is not zero, increments a value at a calculated address.
- `call_and_update_offset`: Pushes several values onto the stack, calls a routine using a pointer from a5, and then uses a value from a4 to update a byte at a calculated address.
- `call_and_update_value`: Pushes several values onto the stack, calls a routine using a pointer from a structure in a5, and uses the result to calculate an offset into a table in a4 to add a byte to a value.
- `call_and_update_table_2`: Pushes several values onto the stack, calls a routine via a pointer in a5, and then uses the result to calculate an address and add a byte to a global variable.
- `call_and_update_table`: Pushes several values onto the stack and calls a routine via a pointer in a5, then uses the result to calculate an offset and add a byte from that offset to a value at offset 19068 of a4.
- `call_subroutine_and_update_table`: Calculates a value by doubling a word from a structure in a4, pushes several values onto the stack, and calls a subroutine via a pointer in a5.
- `execute_and_update_offset`: Pushes several values onto the stack, calls a subroutine using a pointer from a5, and then modifies a value in a4 based on the result of a calculation involving a word from a4 and a byte from a calculated address.
- `branch_on_specific_values`: Compares a word from offset -2 of a5 against three specific constants and branches if a match is found; otherwise, it pushes a pointer from offset 8 of a5 and several values onto the stack, calls a subroutine, and performs arithmetic on a value from a global structure in a4.
- `compare_and_branch_8`: Checks a word at offset 12 of a5; if it is zero, it pushes several values onto the stack, calls a subroutine, and then compares a word from offset -4 of a5 to a word at offset 19068 of a4 to set d0.
- `calculate_and_jump`: Pushes a longword from offset 8 of a5 and two words onto the stack, calls another routine, and then performs a branch using a value derived from d1.
- `execute_conditional_subroutine_2`: Checks a word at offset 12 of a5, and if zero, pushes a pointer from offset 8 of a5 and several constants onto the stack before calling a subroutine and performing a comparison with a word at offset 19068 of a4.
- `call_subroutine_and_calculate_d0`: Calls a subroutine with a word from a4 and a pointer from a5, then calls another routine using a value from a4, and finally calculates a value in d0 using bitwise operations and returns.
- `calculate_offset_and_call_2`: Stores a pointer to a subroutine in a structure at offset -16 of a5, pushes several values onto the stack, and calls that subroutine.
- `process_conditional_offset_and_call`: Checks if a longword at an offset of a5 is one, tests a byte at an offset of a5, and if it is greater than zero, stores it at another offset of a5, pushes several values onto the stack, calls a routine using a pointer from an offset of a5, and then performs arithmetic to add a byte from a calculated address to a value at an offset of a4.
- `calculate_offset_address_5`: Calculates a new address by adding a signed byte from an offset of a5 to a longword in a3, then moves this result into a3 and branches.
- `call_and_calculate_offset`: Negates a byte from offset -7 of a5, calls a routine using a pointer from offset -16 of a5, and then adds a value derived from a byte at an offset of a5 to a value in a4.
- `copy_byte_loop_3`: Decrements a byte at an offset of a5, and if it is not zero, copies a byte from an offset of a5 to the address in a3, increments a3, and loops.
- `restore_registers_and_unlk_3`: Restores d2, d7, and a3 from the stack and unlinks a5 before returning.
- `calculate_offset_and_loop`: Multiplies a word from a structure in a5 by 6032, adds it to a longword from a structure in a4, and enters a loop that compares bytes and increments d7 until a condition is met or d7 reaches 126.
- `process_and_update_value`: If d7 is greater than 1, it stores the negative of d7 at offset -13 of a5, calls a routine using a pointer from offset -18 of a5, and then calculates a new value by adding a byte from a memory location indexed by a modified d6 to a value at offset 19068 of a4 before branching.
- `call_and_update_pointer`: Calls a routine using a pointer from offset -18 of a5, and then adds a value derived from a byte at an offset of a3 to a value in a4.
- `update_pointer_with_d7`: Calculates a new address by adding the longword in d7 to the longword in a3, moves the result into a3, and branches.
- `iterate_and_compare_bytes`: Increments a value in d7 and uses it to index into two different memory locations pointed to by a3, comparing the bytes found there and branching if they are equal or if certain bounds are exceeded.
- `call_with_scaled_offset`: Stores a byte at an offset of a5, pushes several values onto the stack, calls a routine using a pointer from an offset of a5, and then adds a byte from a calculated address to a value at an offset of a4.
- `call_and_calculate_scaled_offset`: Calls a routine using a pointer from offset -18 of a5, and then adds a value derived from a byte at an offset of a3 to a value in a4 using a loop.
- `update_pointer_with_d7_alt`: Adds the longword in d7 to the longword in a3, stores the result in a3, and branches.
- `call_and_update_global`: Clears a byte at an offset of a5, calls a routine using a pointer from a5, and if the return value is non-zero, calculates an address based on a value in a5 and adds a byte from that address to a global variable at an offset of a4.
- `restore_registers_and_unlk_4`: Restores d2 through d7 and a3 from the stack and unlinks a5 before returning.
- `check_and_store_result`: Checks a word at offset 8 of a structure in a5, branches if it is zero, otherwise calls a subroutine, stores the returned value in d0 into offset -4 of the structure in a5, and branches to another location.
- `store_and_branch`: Clears the stack, calls a subroutine, stores d0 into a structure at offset -4 of a5, and then either sets d0 to 0 and branches or continues based on the value of d0.
- `execute_multiple_subroutines_4`: Pushes several values onto the stack, calls three different routines, and performs a final call with two values on the stack.
- `process_conditional_jumps`: Calls several subroutines and performs arithmetic on the results of a word at offset 8 of a5, using conditional branches to decide whether to call further subroutines or perform specific data movements.
- `loop_write_constant_to_table`: Loops while d7 is less than 59, writing the value 3 to two consecutive bytes at an address derived from a table in a4 and a scaled version of d7.
- `write_constant_to_table`: Loops while a value in d7 is less than 99, writing the value 3 to an offset of a structure at an address derived from a table in a4 plus d7.
- `restore_registers_and_unlk_5`: Restores d7 and a3 from the stack and unlinks a5 before returning.
- `allocate_and_initialize_structures`: Allocates memory using a library call, copies bytes from a table in a4 to another location in a4, and performs multiple conditional library calls to manage the new memory.
- `allocate_and_manage_structures`: Calls a memory allocation routine, performs several loops to manipulate values at offsets of a5, and uses a library function to update a value at offset -18 of a5.
- `call_and_increment_offset`: Calls a routine using a pointer from a table in a4, clears a longword at an offset of a5, increments a word at an offset of a5, and branches.
- `calculate_division_and_update_list`: If a word at offset -2 of a5 is positive, calls a subroutine, performs a division using a value from a5, and enters a loop that updates a pointer in a structure at offset -18 of a5.
- `manage_memory_and_structures`: Copies bytes in a loop, performs several conditional checks involving table lookups, calls multiple routines, and updates a pointer in a5.
- `free_and_reallocate_memory`: If a pointer at offset -22 of a5 is non-zero, it calls a library function to free that memory; then, if a pointer at offset -26 of a5 is non-zero, it calls another library function with a size of 260 and a pointer from that offset.
- `copy_and_test_data`: It enters a loop that copies bytes from a source at offset -160 of a5 to a destination at offset -18127 of a4, then performs several conditional checks on values in a5 and a4.
- `dispatch_handler_sequence`: Copies bytes from a source at offset -160 of a5 to a destination at offset -18127 of a4 in a loop, then calls two different routines if certain conditions are met.
- `call_handlers_sequentially`: Pushes two longwords onto the stack and calls a subroutine, then if the result is non-zero, pushes two other longwords and calls a second subroutine.
- `copy_data_and_call_handlers`: Copies bytes between two memory locations in two separate loops, then performs a series of conditional checks and calls subroutines using values from a5 and pointers from a4.
- `copy_and_call_handler`: Calls a routine using a pointer from a6, pushes two values onto the stack, calls another routine, and then moves d0 into a structure at offset -164 of a5.
- `copy_bytes_in_loop`: Copies bytes from a source at offset -160 of a5 to a destination at offset -17903 of a4 in a loop, then moves a value from offset -164 of a5 into d0.
- `execute_multiple_checks`: Pushes several values onto the stack, calls a subroutine three times with different parameters, and decrements a word in a structure.
- `check_and_compare_byte`: Checks a byte at offset -7 of a5, and if it is non-zero, performs arithmetic to derive values in d5 and d6, then compares d5 and d7 before branching.
- `loop_write_to_table`: Loops while a value in d5 is non-negative, writing a byte from d6 to an address derived from a table in a4 plus d7, and decrements d5 and d7 in each iteration.
- `branch_on_signed_comparison`: Compares register d7 to zero and branches if it is less than or equal to zero; otherwise it clears d0 and branches to another routine.
- `update_counter_and_table`: Clears a word at offset -24794 of a structure in a4, then enters a loop that reads bytes from a table using an offset derived from a value at offset 23176 of a4, and if a byte is 255, it doubles the value at offset -24794 of a4 and writes a new word to a table at offset 21224 of a4.
- `execute_conditional_logic_4`: Calculates a value from a word at offset -24794 of a4, calls a subroutine with several values on the stack, and if the result does not match the doubled input value, calls another subroutine before returning.
- `loop_copy_byte_from_table`: Loops while d7 is non-zero, copying a byte from a table in a4 to a destination address derived from a value in d6 and a5.
- `calculate_and_store_difference`: Loops while a value in d7 is non-zero, calculating a value by subtracting a word at an address in a0 from 6511, and stores that result at an address in a1 derived from a0 and d7.
- `restore_registers_and_unlk_6`: Restores d5, d6, and d7 from the stack and unlinks a5 before returning.
- `loop_calculate_scaled_values`: Loops while d7 is less than 5984, performing arithmetic with d5 and d6 to copy bytes from a table in a4 to a destination address.
- `increment_and_branch_5`: Adds 100 to register d7 and branches to another location.
- `calculate_scaled_value_2`: Decrements a word from a global variable, performs several divisions and multiplications using registers d0 through d3, and stores a result in a memory location at an offset from a pointer in a1.
- `restore_registers_and_unlk_7`: Restores d2 through d7 and a2 from the stack and unlinks a5 before returning.
- `calculate_and_store_values`: Loads bytes from a structure at a4, performs arithmetic to calculate four values, stores these values in a structure at a5, and branches based on comparisons of d6 and d2.
- `initialize_structure_fields_3`: Sets up a stack frame, saves registers, writes two 16-bit values to offsets -78 and -76 of a structure in a5, compares a value at offset 12 of that structure to 1, and branches to another location if they are not equal.
- `copy_and_call_library`: Copies 11 longwords from a global variable to offset -74 of a5 in a loop, sets several fields in a structure at a5 using a calculated address and a constant, and then calls a library function using a pointer from offset -25272 of a4.
- `execute_library_calls_and_loop_table`: Calls a subroutine and two library functions using a pointer from a structure in a4.
- `loop_table_values`: Sets a word at offset -24794 of a4 to zero and enters a loop that reads bytes from a table using d7 as an index, performs arithmetic, and writes a value to a table at offset 21224 of a4.
- `process_loop_and_table_updates`: Performs several calculations and calls a routine via a6, then enters a loop that performs multiple calculations, calls two different routines via a6, and increments values at specific memory locations.
- `increment_and_branch_short`: Adds 1 to register d7 and branches to another location.
- `call_subroutine_with_stack_setup`: Tests a longword at offset -10 of a5, and if it is not zero, pushes a longword from a structure in a4 onto the stack and calls a subroutine.
- `jump_to_address_13`: Performs a long jump to the address stored in the constant value 4224.
- `perform_indirect_jump_2`: Performs an indirect jump to the address stored at offset zero.
- `jump_to_long_address_2`: Performs a long jump to a destination address stored in a pointer.
- `jump_to_address_22`: Performs a jump to an address stored in a longword at a specific location.
- `jump_to_conditional_evaluator_2`: Jumps to the routine evaluate_conditional_values, which loads three words from offsets 8, 10, and 12 of a structure in a5 into d7, d6, and d5, then checks if d6 is zero and if d7 equals 65534.
- `jump_to_indirect_address_25`: Performs a long jump to an address located at a specific offset in a different routine.
- `jump_to_long_address_10`: Performs a long jump to a specific address.
- `jump_to_address_long`: Performs a long jump to the address stored in the constant value 7888.
- `jump_to_pointer_3`: Performs a long jump to the address stored in register a0.
- `jump_to_distant_routine`: Performs an indirect jump to a location specified by a long word.
- `jump_to_constant_address`: Performs a long jump to the address stored in the constant value 3072.
- `jump_to_address_33`: Performs an indirect jump to the address stored at the constant value 2048.
- `jump_to_external_routine_3`: Performs a long jump to an address located at a specific memory location.
- `jump_to_comparison_routine`: Performs a long jump to a routine that compares multiple values stored in registers a4 and a5.
- `jump_to_library_offset`: Performs a jump to an address located at an offset of minus 306 from a library base stored in a global variable.
- `jump_to_constant`: Performs a long jump to the address 896.
- `jump_to_address_fixed`: Performs a long jump to the address stored in the constant value 7964.
- `jump_to_null_pointer_2`: Performs an unconditional jump to a null address.
- `jump_to_indirect_address_15`: Performs an indirect jump to a long address stored in memory.
- `jump_to_long_address_3`: Performs a long jump to a specific address stored in a pointer within the data section.
- `jump_to_subroutine_5`: Loads the value 176 into register d0 and performs a long jump to another routine.
- `jump_to_address_30`: Performs an indirect jump to the address stored at 4202.
- `jump_to_absolute_address_7`: Performs an absolute long jump to a specific address.
- `jump_to_pointer`: Jumps to the address stored in a longword at a specific location in memory.
- `jump_to_long_address_7`: Performs a long jump to a specific address after a caller has loaded the value 56 into register d0 via set_jump_parameter_38.
- `jump_to_address_14`: Performs a jump to a longword address stored in a register.
- `jump_to_indirect_address_26`: Performs a long jump to the address stored in the location specified by a pointer in data.
- `jump_to_address_31`: Performs an indirect jump to the address stored at 7692.
- `jump_to_division_routine`: Jumps to a routine that divides a longword extended from a word at an offset of a5 by 100 and adds a word from an offset of a5.
- `jump_to_pointer_4`: Jumps to the address contained in a longword pointer located in data memory and calls perform_division_loop_2.
- `jump_to_pointer_2`: Jumps to the address stored in a longword at a location pointed to by a global variable.
- `jump_to_long_address_8`: Loads the value 216 into register d0 and performs a long jump to another routine.
- `jump_to_hardware_register_7`: Jumps to the address stored in the custom chip register at offset 532.
- `test_byte_and_return`: Tests a byte at a negative offset from a4 and returns if it is zero; otherwise it returns.
- `evaluate_and_update_parameters`: Decrements a word in d0, and if the loop counter is not zero, performs several comparisons against values in a structure in a0, calls a subroutine, and performs arithmetic to write values to a table via a library call.
- `update_system_parameters_2`: Performs several conditional checks on d4, d5, d6, and d7, using division and subtraction to calculate values that are stored in a table at various offsets.
- `handle_window_logic`: Performs several comparisons and conditional branches, calls a routine using a pointer from the stack, calls a library function, and calls another routine using a pointer from the stack.
- `check_bit_and_call`: Saves registers to the stack, loads a pointer from a global variable, checks if bit 16 or bit 15 is set at an offset of 24 from that pointer, and if neither is set, calls a routine via a pointer in a6 before branching to another routine.
- `store_and_call_routine`: Copies two words from the stack to specific memory locations and calls another routine before returning.
- `evaluate_multiple_conditions`: Saves registers, compares d0 against several constants, and if a byte from a5 is 3, performs a series of arithmetic operations and memory reads to set bits in d7 before branching.
- `set_jump_parameter_d8`: Loads the word value 216 into d0 and performs a long jump to an address.
- `set_jump_parameter_38`: Loads the word value 56 into d0 and performs a long jump to an address.
- `set_jump_parameter_b0`: Loads the word value 176 into d0 and performs a long jump to an address.
- `modify_byte_and_branch`: Loads a byte from a pointer at an offset of a5, uses it to offset a pointer from a4 to load another byte, and if bit 1 of that byte is set, it subtracts 32 from the byte and branches.
- `increment_and_verify_byte`: Loads a pointer from an offset of 8 in a5, increments the byte at that pointer, and enters a loop that continues if the byte at that pointer is not zero, otherwise it unlocks a5 and returns.
- `calculate_structure_dimensions`: Calculates two values using division and subtraction from a global variable, then stores them into a structure at offsets 0, 2, 4, and 6 after applying bit shifts and constant additions.
- `update_structure_offsets`: Calculates two new values based on differences between local parameters and global variables, then writes them to specific offsets within a structure pointed to by a parameter.
- `calculate_clamped_division`: Loads a word from offset 8 of a5 into d0, extends it to a longword, subtracts 23 from d1, divides it by 16, adds a word from a global variable to d1, and then either branches to another routine or sets d1 to 1.
- `calculate_and_clamp_division`: Copies d0 to d1, subtracts 23 from d1, extends d1 to a longword, divides it by d2, adds d3 to the result, and branches based on whether the value is greater than 98 or less than 1.
- `divide_and_store_result_2`: Subtracts 23 from d0, extends it to a longword, divides it by d2, adds d3 to the result, stores the result in d1, and branches to another location.
- `calculate_division_result_2`: Moves a word from an offset of a5 into d0, extends it to a long, performs a division using d3 and d2, adds a value from an offset of a4 to d2, stores d0 at an offset of a5, compares d2 to 1, and branches based on the result.
- `calculate_scaled_quotient`: Subtracts 20 from d0, extends the result to a longword, divides it by d3, adds d4 to the result, and then performs conditional branches based on whether the result is greater than 58 or between 1 and 58 inclusive.
- `divide_and_store`: Subtracts a constant from d0, extends it to a longword, divides it by d3, adds d4 to the result, stores the result in d2, and branches to another routine.
- `multiply_and_add_constant`: Multiplies the value in d2, which is the product of 58 and 100, by a word in d1 and adds the result to d0.
- `dispatch_nested_calls_2`: Checks a word at offset -18498 of a4, calls a subroutine, clears the stack, calls another subroutine, pushes a word from offset 8 of a5 onto the stack, calls a third subroutine, and returns.
- `update_structure_value`: Initializes a word at offset -2 of a5, calls another routine, then checks a byte at a pointer in a5; if the byte is not zero, it increments the pointer in a5 and adds the bitwise NOT of the byte to the word at offset -2 of a5.
- `toggle_boolean_flag`: Checks a word at offset -2 of the structure in a5; if it is not zero, it sets it to 1 and returns, otherwise it returns immediately.
- `calculate_and_compare_data`: Saves registers, calls another routine, performs division and multiplication on values derived from a5 and a4, and compares bytes from two calculated addresses, clearing a word at offset -10 of a5 if certain conditions are met.
- `add_and_branch`: Adds a constant to d5 and branches to another routine.
- `calculate_offset_pointer`: Tests a word at a negative offset from a5, and if non-zero, calculates an address by adding a scaled version of d0 and d1 to a base address derived from a4, then loads a longword from that address into d0 before branching.
- `perform_division_check`: Divides d7 plus one by 32, compares the quotient to d6, and if they are not equal, performs a series of divisions and writes to a global variable in a4, then uses the result to load a longword from a structure in a4 into d0 before returning.
- `test_and_branch`: Saves registers to the stack, tests a word at a negative offset from a4, and if it is not zero, branches to another routine; otherwise it sets d0 to 0 and branches to another routine.
- `compare_multiple_bounds`: Compares several values from a4 and a5, using conditional branches to skip a sequence of checks that eventually sets d0 to zero and branches to another routine.
- `increment_and_compare_value`: Increments a word at a negative offset from a5, compares the new value with 8, and if it is 8 or less, performs arithmetic using a value from a table and compares it to a value at offset 10 of a5.
- `increment_offset_value_2`: Increments a 16-bit word at a negative offset from a pointer in a5 and branches to another routine.
- `set_d0_to_zero`: Sets d0 to 0 and returns.
- `divide_and_store_byte`: Divides a word from offset 8 of the structure in a5 by 10, adds 48 to the result if it is non-negative, and stores the byte at an address determined by a pointer in a4 and a value in a0.
- `write_byte_to_global_offset`: Writes the byte 32 to a global variable at an offset determined by a pointer in a4, then divides a word from offset 8 of the structure in a5 by d1, adds 48 to the high word of the result, and stores that byte at offset 1 of the global variable.
- `calculate_scaled_division_2`: Sets up a stack frame, extends a word from an offset of a5 to a longword, divides it by 100, adds a word from an offset of a5, stores the result at an offset of a5, and then sets d2 to one if the result is less than one.
- `jump_to_address_4`: Performs a long jump to the address stored in a constant.
- `jump_to_address_43`: It performs a long jump to the address stored at offset 80.
- `jump_to_handler`: Performs an unconditional jump to another routine.
- `jump_to_offset_five_one_eight`: Performs a long jump to a specific address after being called by another routine.
- `set_d0_and_jump_18`: Loads the word value 24 into d0 and performs a long jump to an address.
- `compare_and_branch_on_value`: Links a5, restores d7 from the stack, loads a word from an offset of a4 into d7, and if d7 is not 1, performs a series of operations involving adding d1 to a pointer in a4 and comparing a word at that location to an offset of a5 before branching.
- `increment_d7_conditionally`: Loads a pointer from a table using a value from a4, checks the byte at that pointer, and if it is non-zero, compares d7 to a constant and adds 20 to d7 if it is less than 6652.
- `update_offset_value`: Compares a 16-bit value at offset -18594 of a4 to d7 plus 20, and if the value is not higher than that sum, stores the result of d7 plus 20 back into that same offset of a4 before returning.
- `compare_and_set_d0`: Loads a 16-bit value from a4 at offset -18500, and if it is not negative, calculates an address using a value from a5 and a value from a4 at offset -18292, compares the word at that address to the value from a4, and if they are equal, sets d0 to 1; otherwise it returns.
- `validate_and_update_entity_data_2`: Performs extensive data validation and arithmetic involving multiple offsets in a5 and a4, including several conditional branches, calls to subroutines, and updates to various memory locations based on division results and byte comparisons.
- `validate_and_clear_byte`: Loads a byte from a5 at offset -13 and uses it as an offset into a4 to check a byte; if certain conditions regarding d1 and a5 offsets are met, it calculates an address using a value from a4 at offset 24898, 24922, and 24886 to compare and eventually clear a byte at a calculated address.
- `restore_registers_and_return_6`: Restores d2 through d7 and a6 from the stack, unlocks a5, and returns.
- `validate_and_update_entity_data`: Compares bytes from two different memory locations derived from a base address and a structure offset, then performs a series of conditional checks involving multiplication and memory reads to update a byte at a specific offset in a table.
- `write_one_to_calculated_address`: Multiplies d0 by 200, calculates an address using a4 and d0, calculates another address using a5 and d0, and writes 1 to that address before branching to another location.
- `check_and_call_on_byte`: Calculates an address using a4 and d5, tests a byte at that address, and if it is not zero, calculates a second address using a4 and d5, tests a byte there, and if it is not zero, calculates a third address using a4 and d5, tests a byte there, and if it is 255, pushes d6 and d7 to the stack, calls another routine, and branches to another location.
- `check_byte_and_call`: Extends d5 to a longword, calculates an address using a4 and d5, compares a byte at that address with 2, and if it is not less than 2, performs further address calculations and calls a subroutine with d6, d0, and d7 on the stack.
- `compare_and_skip_array`: Calculates an address in an array based on a value at offset 10 of a5 and a constant, then compares a word at an offset from that address with a value from offset 8 of a5.
- `conditionally_store_word`: Calculates a longword offset from a3 using d1, and if the word at that location is zero, stores d7 into that location.
- `loop_store_word`: Decrements d5 and, if not zero, uses the new value to calculate a longword offset from a3 to store a word at that location, then branches back to the start of the routine.
- `store_word_at_pointer`: Stores a 16-bit value from d7 into the address held in a3, restores d2, d5, d6, d7, and a3 from the stack, unlocks a5, and returns.
- `check_array_element`: Calculates an address in an array using values from offsets 10 and 12 of a5 and a constant, then compares a word at that address with a value from offset 8 of a5.
- `conditionally_store_word_in_array`: Checks if a word at a calculated offset within an array is zero, and if so, stores a 16-bit value from a register at that same offset.
- `loop_store_word_small`: Decrements d4 and, if not zero, uses the new value to calculate a longword offset from a3 to store a word at that location, then branches back to the start of the routine.
- `store_word_at_pointer_2`: Stores a 16-bit value from d7 into the address held in a3, restores d2, d4, d5, d6, and d7 from the stack, unlocks a5, and returns.
- `calculate_quotients_and_difference`: Performs multiple division operations using values from a5 and d1, stores the results in d7, d6, d5, and d4, and then calculates the difference between d7 and d5, negating the result if it is negative.
- `calculate_absolute_difference_3`: Subtracts a word in d5 from a word in d0, and subtracts a word in d4 from a word in d1, then returns the absolute value of the result in d1.
- `calculate_differences`: Subtracts d4 from d1, and if the result is not less than or equal to zero, subtracts d5 from d0 and returns the absolute value if the result is negative.
- `subtract_and_branch`: Subtracts d5 from d7 and branches to another location.
- `calculate_absolute_difference_2`: Subtracts d4 from d6, and if the result is negative, negates it before branching to another location.
- `calculate_difference`: Subtracts d4 from d6 and stores the result in d0, restores d2, d4, d5, d6, and d7 from the stack, unlocks a5, and returns.
- `clear_byte_at_offset_2`: Sets up a stack frame, compares two words from a5, and if they are equal, writes 65535 to a location in a3, then clears a byte at a calculated address using a pointer and an offset from a4.
- `calculate_and_store_quotients`: Performs several divisions using a constant 100 and a constant 100 decimal, stores the resulting words at offsets minus 2, minus 8, minus 4, and minus 6 of a5, performs comparisons to increment d6 and d7, calculates a value in d1, loads a word from an offset of a4 into d0 and a3, pushes two words to the stack, calls another routine, and branches based on the result in d0.
- `calculate_signed_difference`: Tests a word at an offset in a4, and if non-zero, calculates the difference between two words from a structure in a5 and stores the absolute value in d2.
- `calculate_difference_and_store`: Subtracts d0 from d1, stores the result in d2, loads two words from offsets minus 6 and minus 8 of a structure in a5, subtracts d0 from the result in d3, stores d2 at offset minus 10 of a structure in a5, and branches based on whether d3 is positive or negative.
- `update_and_branch_pointer`: Subtracts d0 from d1, stores the result in d3 and at offset minus 12 of a5, then compares d3 with d2; if d3 is not equal to d2 and not greater than d2, it updates d2 by subtracting 1 and branches back to the start of the routine, otherwise it sets d0 to 0 and branches to another routine.
- `calculate_and_store_offset_3`: Tests a word at offset -10 of a5, calls two different routines, performs arithmetic involving a division and a swap of d0, and if a condition is met, stores a value into a pointer at a3.
- `modify_and_call_pointer`: Tests a value at offset -24796 of a4; if non-zero it branches, otherwise it subtracts d5 from a word at a3, masks the result, pushes the result and a word from offset 24918 of a4 onto the stack, calls a subroutine, and if the result is non-zero writes the masked value back to a3.
- `conditional_clear_word`: Tests a 16-bit value at an offset from a4; if it is non-zero, it branches to another routine, otherwise it writes a 16-bit value of all ones to the address in a3 and branches to another routine.
- `compare_and_fetch_table_word`: Compares two words from a structure in a5 and, if the first is not less than or equal to the second, calculates an offset using d7, fetches a word from a table via a pointer in a4, and branches.
- `calculate_and_store_difference_2`: Calculates a value based on register d6, uses it to access a word from a table via a pointer derived from a4, pushes two words onto the stack, calls another routine, and then either stores a word from d0 into a memory location at a3 or branches to another routine.
- `calculate_absolute_difference_4`: Tests a word at a negative offset from a4, and if not zero, subtracts a word from offset -12 of a5 from a word at offset -14 of a5, calculates the absolute difference, and branches.
- `process_conditional_word_update`: Subtracts d0 from d1, performs bitwise operations to update a word at the address in a3, pushes several values onto the stack, calls another routine, and then sets d0 to 1 or 0 based on a test of d0.
- `check_and_set_word`: Tests a word at an offset from a pointer in a4; if it is non-zero, it branches to another routine, otherwise it sets the word at a3 to 65535 and branches to another routine.
- `clear_byte_at_calculated_address`: Adds a 16-bit offset at offset 24534 of a4 to a pointer at offset 24886 of a4, clears the byte at that resulting address, restores d2 through d7 and a3 from the stack, unlocks a5, and returns.
- `validate_division_results`: Loads a word from a structure in a5, performs two divisions by 100, and branches to another routine if the quotient or the remainder of the first division is 0, 59, or if the quotient of the second division is 0 or 99; otherwise it sets d0 to 0 and branches to another routine.
- `set_d0_and_return`: Sets d0 to 1, restores d5, d6, and d7 from the stack, unlocks a5, and returns.
- `loop_and_call_subroutine`: Decrements d5 in a loop, calculates an offset from a4, adds values from memory, and calls a subroutine if a byte at a calculated address is non-zero.
- `move_d6_to_d0`: Moves the value from d6 into d0, restores d5, d6, and d7 from the stack, unlocks a5, and returns.
- `update_and_loop_values`: Clears two words at offsets from a5, then enters a loop that calculates an offset from a4, calls a subroutine, and conditionally increments a word at an offset from a5.
- `increment_pointer_target`: Loads a pointer from an offset of a5 into a0, increments the word at that address, and branches to another routine.
- `restore_registers_and_unlk`: Restores d5 through d7 and a7 from the stack, unlocks a5, and returns.
- `evaluate_conditional_values`: Loads three words from offsets 8, 10, and 12 of a structure in a5 into d7, d6, and d5, then checks if d6 is zero and if d7 equals 65534 and d5 is non-zero, otherwise it sets d0 to 2 and branches.
- `link_and_test_memory`: Links a5, saves d2, d3, and a2 to the stack, loads a word from offset 10 of a5 into d0 and d1, calculates an address using d1 and a constant, and branches based on the word at that address.
- `execute_conditional_jumps`: Tests a word at an offset of a5 and, if non-zero, performs two sequences of fetching bytes from memory and calling subroutines based on the results.
- `load_constant_and_branch`: Loads the word 999 into d0 and branches to another routine.
- `execute_multiple_subroutines`: Calculates an address using a1 and d0, fetches a byte, extends it to a longword, and calls another routine; then performs similar address calculations and calls another routine twice more, branching to another location at the end.
- `load_constant_and_restore`: Loads the word value 999 into d0, restores d2, d3, and a2 from the stack, unlocks a5, and returns.
- `update_local_data`: Loads a word from offset 8 of a structure in a5 into d1, compares it to 1, and if equal, sets d0 to 2, loads a word from an offset of a4 into d2, and stores d0 and d2 at offsets -2 and -4 of the structure in a5 before branching.
- `set_conditional_values`: Compares d1 to 2; if not equal, it branches to another routine, otherwise it sets d0 to 1, stores it at an offset from a5, checks a word at an offset from a4, and if that word is 2, branches to another routine; otherwise it sets d0 to 3, stores it at an offset from a5, and branches to another routine.
- `perform_complex_math`: Performs a series of arithmetic operations including multiplications and divisions using values from memory locations relative to a4 and a5, and stores the final result in d0 before restoring registers and returning.
- `update_structure_values`: Subtracts 36 from a word at offset 12 of a5, uses the result to calculate an offset into a table via a4, adds a word from a pointer in a5 to a word from that table, stores the results in offsets -10 and -2 of a5, and branches based on a comparison with a global variable.
- `compare_quotients_and_branch`: Calls two subroutines with values from the stack, then performs two divisions by 100 on values from a structure in a5, comparing the results and branching to another routine if they are equal.
- `test_d0_and_branch`: Saves d2 to the stack, calls a subroutine, and if d0 is not zero, sets d0 to zero and branches to another routine; otherwise, it branches to another routine.
- `update_and_clamp_values`: Copies a word from offset -10 of a5 to an address in a0, copies a word from offset -2 of a5 to an address in a0, calls a routine with d0 and 1 on the stack, and if d0 is zero, compares a word from offset -24898 of a4 against 58 and 1, potentially setting d0 to 58 or 1.
- `clamp_and_store_values`: Compares a word from a structure in a4 against 58 and 98, then stores a modified value into a structure in a5.
- `calculate_quotients_and_call`: Performs several arithmetic operations including multiplication and division on values from a structure and a global variable, then calls another routine with the resulting values on the stack.
- `execute_multiple_calls`: Performs a series of calls to two different routines using values derived from dividing a word at offset 12 of a5 by 100, including one call where the argument is a value from a memory location at a negative offset from a4.
- `jump_to_offset_handler`: Performs an indirect jump to a long address.
- `jump_to_address_7`: Performs a long jump to an address.
- `jump_to_long_address`: Performs a long jump to a specific address.
- `jump_to_address_37`: Performs a long jump to the address 746.
- `jump_to_address_18`: Performs a long jump to the address contained in the constant value 1086.
- `jump_to_window_logic`: Performs an indirect jump to a routine located at a specific memory address.
- `jump_to_external_address_2`: Performs a long jump to an address stored in a pointer located in the data section.
- `jump_to_long_address_9`: Performs a long jump to a destination address stored in a memory location.
- `jump_to_long_address_4`: Loads the word value 224 into register d0 and performs a long jump to another routine.
- `jump_to_address_28`: Performs a long jump to the address 7692.
- `jump_to_absolute_address_4`: Performs an absolute long jump to a specific memory location.
- `jump_to_absolute_address_6`: Performs an absolute long jump to a specific address and calls calculate_and_call_quotients, which divides a word from offset 8 of a5 by 100, subtracts a word from offset 20326 of a4, and then divides a word from offset 8 of a5 by another value.
- `set_d0_and_jump_e0`: Loads the word value 224 into d0 and performs a long jump to an address.
- `push_parameters_and_call`: Links a5, pushes a pointer from offset minus 2 of a5 and a pointer from offset minus 24924 of a4 to the stack, pushes a word from offset 10 of a5 and a word from offset 8 of a5 to the stack, calls another routine, adjusts the stack pointer, and unlinks a5 before returning.
- `push_parameters_and_call_variant`: Links a5, pushes a pointer from offset minus 2 of a5 and a pointer from offset minus 24938 of a4 to the stack, pushes a word from offset 10 of a5 and a word from offset 8 of a5 to the stack, calls another routine, adjusts the stack pointer, and unlinks a5 before returning.
- `push_parameters_and_call_offset`: Links a5, pushes a pointer from offset 24170 of a4 and a pointer from offset minus 24936 of a4 to the stack, pushes a word from offset 10 of a5 and a word from offset 8 of a5 to the stack, calls another routine, adjusts the stack pointer, and unlinks a5 before returning.
- `push_parameters_and_call_short`: Links a5, pushes a pointer from offset minus 2 of a5 and a pointer from offset minus 25322 of a4 to the stack, pushes a word from offset 10 of a5 and a word from offset 8 of a5 to the stack, calls another routine, adjusts the stack pointer, and unlinks a5 before returning.
- `update_table_and_compare`: Links a5, saves d7, writes several values to a table in a4, and compares a word from offset 8 of a5 with a value derived from a table in a4, branching if they are not equal.
- `calculate_scaled_value_4`: Pushes two words onto the stack, calls another routine, performs arithmetic on the result in d0 by doubling it and adding 25, and then compares the result to 125, either setting d0 to 125 or branching.
- `calculate_and_branch`: Pushes words from offset 10 of a5 and offset 19080 of a4 to the stack, calls another routine, performs arithmetic on the returned value in d1, stores the result in d0, pushes d0 and a word from offset 10 of a5 to the stack, calls another routine, updates a pointer at offset 18 of a5 with a word from offset 19084 of a4, and branches based on a comparison of d7 and 9999.
- `increment_d0_from_d7`: Copies the value of d7 into d0 twice, increments d0 by 1, restores d7 from the stack, unlocks a5, and returns.

The function list continues in: g2220-recovery-functions-2.md.
