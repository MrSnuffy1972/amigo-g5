# Game 2221 (empire - war game of the century v2.05 (1987)(interstel corporation)[cr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02221). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909856 top-level bytes in 128 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 59 of 59 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 2220 of 2220 named (0 author symbols), 2220 of 2220 described (306 by rule, 1912 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3939 of 3939 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 42724 of 42726 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 288039 |
| code | 137136 |
| data | 424889 |
| padding | 25488 |
| unused | 34304 |

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
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf:Preferences | 45 | 6587 | 222 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf:Empire | 33 | 19031 | 1003 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf:CLI | 9 | 362 | 18 |
| icon.library | 15 | 1414 | 108 |
| info.library | 25 | 2476 | 170 |
| version.library | 2 | 41 | 7 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1754 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 323 | 8 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf bytes 103936-104448 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf bytes 104960-105472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf bytes 536576-537088 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: file devs/narrator.device (header 23280 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1110 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 120 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: bytes 103936-104448 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: bytes 104960-105472 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: bytes 536576-537088 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: 3 logged loads cover 1536 bytes of the image
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: blocks 143-143 (bytes 73216-73728) outside the filesystem hold data, not code (opcode density 0.035, entropy 4.06)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf: blocks 154-197 (bytes 78848-101376) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.17)
- 3 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `manage_data_structures`: It calls a subroutine, performs arithmetic on a pointer, compares a value in a register against a constant, and uses a loop to copy data between a structure in a2 and other memory locations, eventually calling several subroutines and returning.
- `move_and_branch`: It moves the value from d1 to d0 and then performs a short branch to another location.

### More

- `process_resource_data`: Performs a series of library calls and complex logic to process data from a structure, including a loop that copies bytes from one location to another based on specific character comparisons.
- `handle_resource_error`: Checks a value at offset 20 of a structure in d1, and if non-zero, performs a series of library calls including one using a pointer from offset 8, one using a pointer from offset 28, and others using offset 20 and offset 8 to manage resources or messages.
- `execute_library_calls`: Loads the address at offset 92 of a4 into a0 and calls a routine at offset -192 of a6, then repeats this process with the same address in a0 and calls a routine at offset -180 of a6.
- `call_dos_function`: It loads an address into a1, calls a library function at an offset from a6, stores the result in a global variable at offset 8, and returns if the result is zero.
- `update_pointer_table`: Loads a pointer from offset 4 of a7 into a0, stores a0 at the address pointed to by a0, increments the value at that address by 4, clears the value at offset 4 of that address, and stores a0 at offset 8 of that address.
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
- `manage_complex_logic`: Performs a large sequence of conditional checks, register saves, and multiple subroutine calls using arguments from the stack or a structure in a6, eventually returning via an unlk instruction.
- `validate_byte_range`: Calls two different routines using pointers from a structure in d4, then checks a value at offset 4 of a6 against several constants and performs a loop that increments the value at offset 4 of a6 if it falls within specific ranges.
- `extend_and_jump_on_value`: It links a6, stores 1 in a global variable via a pointer in d4, calls another routine, extends the byte from the stack into a longword in d0, and then performs a comparison and a jump or branch based on the result.
- `set_bit_and_branch_2`: Sets bits in d0 using a mask and branches to another location.
- `execute_help_sequence`: Calls a subroutine, pushes a pointer from d3 onto the stack, calls another subroutine, pushes the value 50 into the stack, calls a third subroutine, and then unlinks a6 before returning.
- `perform_conditional_subroutine_calls`: It stores a value in a memory location, calls a subroutine, stores the result in another location, calls a second subroutine, and then calls a third subroutine if certain conditions regarding a structure in a0 are met.
- `call_subroutine_with_stack`: It calls a subroutine using a pointer from a structure, pushes a value from offset 20 of a structure onto the stack, and stores the result of a previous operation into a global variable at offset 24.
- `process_file_data`: Calls two routines using pointers from a structure in d4, then enters a loop that compares values at offset 8 of a structure with constants and increments offsets in a structure until a condition is met, eventually writing specific bytes to a structure.
- `loop_and_call_subroutines`: It initializes a counter on the stack, calls a routine using a pointer from d3, and then enters a loop that compares values from a structure and calls another routine using a pointer from a table until the counter reaches 11.
- `execute_unnamed_sequence`: Pushes a value onto the stack, calls a subroutine, increments the stack pointer, calls another routine, and then performs a short branch.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `dispatch_write_request_3`: It pushes a value from offset 8 of a6 onto the stack, pushes a constant, and calls a routine at a fixed address.
- `call_dispatch_write`: Loads a pointer from offset 8 of the stack, increments a 16-bit value from offset 2c of that pointer, and calls a routine at a fixed address with the incremented value pushed onto the stack.
- `perform_multiple_calls`: Calls several subroutines using values from the stack and a pointer from a table, and stores the result of a comparison in offset -4 of a6 before returning.
- `execute_conditional_logic_3`: Calls several routines and uses the results to perform conditional branches, eventually returning a value from a register to a local variable.
- `read_and_extend_byte_2`: Calls another routine, stores the result in a byte at a location pointed to by a pointer in a6, and if the result is not 10 and is less than a value stored in a local variable, increments both that local variable and the pointer in a6; otherwise, it clears the byte and returns.
- `compare_and_store_results`: Calls a subroutine twice, stores the results in offsets -4 and -8 of a6, and compares the value at offset -4 of a6 with d0.
- `increment_and_compare_structure`: It calculates an address based on offsets in a6, compares the value at that address with a value at offset 12 of a6, and if they are not equal, calls another routine with three parameters from the stack.
- `increment_and_branch`: Increments a longword at offset 8 of a6 and branches to another location.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `increment_local_and_pointer`: Increments a longword at offset -4 of a6 and a longword at offset 8 of a6 if the byte at a location pointed to by a6 is not zero, otherwise it increments only the value at offset -4 of a6.
- `return_offset_value`: Moves the 32-bit value at offset -4 of a6 into d0, restores a6, and returns.
- `compare_bytes_at_offsets`: Compares a byte at offset 8 of a structure in a0 with a byte at offset 12 of a structure in a0, and if they are not equal, sets d0 to zero and returns; otherwise it returns via another routine.
- `increment_three_values`: Increments three longwords at offsets -4, 8, and 12 of a6 and branches to another location.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `execute_conditional_logic_2`: Performs several conditional branches and calls multiple subroutines based on values in a table and a byte at offset 8 of a6.
- `initialize_task_from_result`: It calls a routine, uses the result to access a structure in a1, and then calls another routine using several values stored on the stack and in a6.
- `allocate_and_initialize_task`: It calls a routine to allocate memory, and if successful, calls another routine to initialize a structure, then calls a third routine using values from the newly allocated memory and a structure in a0.
- `initialize_task_structure`: Allocates memory, populates a structure with several values, and performs a loop that copies data from one table to another using a counter, then calls several routines to clean up or message the new task.
- `dispatch_write_request_2`: Pushes d2 onto the stack, loads d2 from offset 8 of the stack, pushes a pointer from offset 12 of the stack, pushes d2 onto the stack, pushes a pointer from offset 20 of the stack, calls a routine at offset 0, restores a7 from offset 12 of the stack, and returns.
- `read_and_extend_byte`: Calls a routine using a pointer from a structure in d1, compares the result in d0 to 1, and if not equal, extends the byte from offset -4 of a6 into a 32-bit value in d0 before returning.
- `dispatch_write_request`: Saves registers, loads pointers from the stack into a4, a0, and a1, calls a routine at offset -522 of a6, and then conditionally calls another routine at offset 44 of a2 if a byte at a specific stack location is not zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_external_routine`: Saves registers, loads pointers from the stack into a3, a0, and a1, and calls a routine at an offset relative to a6.
- `store_byte_and_increment`: Stores the byte from d0 into the memory location pointed to by a3 and increments a3 before returning.
- `multiply_and_add_values`: Performs a series of 16-bit multiplications and additions using the swap instruction to handle word-sized components, returning the result in d0.
- `calculate_absolute_difference`: Calculates the absolute difference between two 32-bit values using a loop that performs bitwise shifts and subtractions, then returns the result.
- `create_task_structure`: Allocates memory for a new task structure, populates its priority, name, and signal fields, and calls routines to initialize the task and add it to a port.
- `cleanup_structure_members`: Checks if offset 16 of a structure in a2 is non-zero, then calls three different routines using pointers and values derived from the structure in a2 to clear or free members.

### Stack

- `reallocate_and_copy_data_structure_4`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_register_to_register`: Copies the longword value from register d1 to register d0 and then branches to another routine.

### Else

- `manage_complex_structures`: Calls a routine via a pointer in a5, performs arithmetic on a pointer derived from a structure in a4, and enters several loops that copy data between structures and update various offsets in a structure in a2.
- `transfer_register_value_2`: Moves the value from d1 to d0 and then performs a short branch.

### Delete

- `manage_data_structures_2`: Calls a routine using a pointer from a5, performs arithmetic on a pointer, and uses a loop to copy data between memory locations while updating various offsets in a structure in a2.
- `transfer_d1_to_d0`: Moves the value from d1 to d0 and branches to another location.

### type

- `manage_complex_data`: It calls a routine, performs arithmetic on a pointer, and enters a loop that copies data and performs multiple conditional calls to other routines, including a loop using dbra to copy blocks of memory.
- `move_register_to_register_and_branch`: Copies the contents of register d1 into register d0 and then performs a short branch to another routine.

### Path

- `reallocate_and_copy_data_structure_5`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_register_and_branch_6`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.

### EndCLI

- `reallocate_and_copy_data_structure_6`: Calls another routine to determine a size, then performs a series of memory operations including pointer arithmetic, data copying via loops, and updating a structure with new pointers and length values.
- `move_and_branch_6`: Moves the contents of d1 to d0 and then performs a short branch.

### Dir

- `process_directory_data`: Calls a subroutine, manipulates a pointer from a table, and performs several conditional branches and loops to copy data and update a value at offset 48 of a structure in a2.
- `move_register_to_register_and_branch_2`: Copies the contents of register d1 into register d0 and then branches to another routine.

### NewCLI

- `reallocate_and_copy_data_structure_7`: Calls another routine to determine a size, then checks a table to see if a new allocation is required.
- `transfer_register_to_register_2`: Moves the contents of register d1 into register d0 and then branches to another routine.

### Echo

- `manage_complex_data_structures`: It calls a routine in a5, performs arithmetic on a pointer to a table, and enters multiple loops that use dbra to copy blocks of data between structures and update various offsets in a structure in a2.
- `move_and_branch_4`: Moves the contents of register d1 to d0 and then performs a short branch.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `process_complex_data`: Calls a subroutine via a pointer in a5, manipulates a table structure, performs multiple nested subroutine calls, and uses loops to copy data between memory locations and structures.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another location.

### Execute

- `reallocate_and_copy_structure_data`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_register_and_branch_2`: Copies the contents of register d1 into register d0 and then performs a short branch to another location.

### makedir

- `reallocate_and_copy_data_structure`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `transfer_register_and_branch_3`: Moves the contents of data register 1 into data register 0 and then performs a short branch to another location.

### CD

- `reallocate_and_copy_data_structure_2`: Calls another routine to determine a size, then performs a memory reallocation by copying an existing data block to a new location and updating a pointer in a structure.
- `transfer_register_and_branch_4`: Moves the contents of register d1 into register d0 and then performs a short branch to another location in the code.

### EndIf

- `return_immediately`: Returns immediately after clearing d0.

### If

- `reallocate_and_copy_data_structure_3`: Calls another routine to determine a size, then performs a memory reallocation by copying data to a new location and updating a pointer in a structure.
- `move_and_branch_5`: Moves the contents of d1 to d0 and then performs a relative branch.

### LoadWB

- `load_resource_configuration`: Executes library calls and conditional logic to process a structure, including a loop that copies bytes and calls a subroutine if a specific value is not zero.
- `execute_library_calls_2`: Retrieves a pointer from the stack, calls three different routines via a pointer in a6, and handles a specific value by calling a routine with a constant value of 100.
- `call_two_routines`: Calls two different routines using addresses derived from an offset of a4.
- `execute_library_calls_5`: Loads a pointer into a1, calls a routine at an offset relative to a6, stores the result in offset 8 of the stack, and returns if the result is not zero.
- `initialize_workbench_environment`: Allocates memory, populates a table of values, performs several library calls including memory allocation and task finding, and updates a structure in a2.
- `compare_byte_sequences`: Compares a byte from one memory location to a byte in another memory location, returning 0, 1, or -1 based on the comparison and the end of the sequence.
- `allocate_and_initialize_structure`: Calls a routine to allocate memory, then uses a loop to call another routine to populate a table of values, and finally updates several offsets in a structure.
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
- `compare_memory_blocks`: Compares bytes from two memory locations pointed to by a2 and a PC-relative address, and if they are not equal, it restores registers and returns; otherwise it branches to another location.
- `restore_and_return`: Loads a pointer into a2, restores a0 and a1 from the stack, pushes a value from a PC-relative address onto the stack, and returns.
- `call_subroutine_via_stack_pointer`: Pushes a value from the stack onto the stack, calls another routine using a pointer stored at an offset of 12 from the stack pointer, and then restores the stack pointer before returning.
- `write_byte_with_check`: Calls a routine to check a byte, and if it is not zero, calls another routine to write a value to a destination specified in a4.
- `store_byte_at_pointer`: Stores the byte from d0 at the memory location pointed to by a3 and increments the pointer.

### Wait

- `process_data_structures`: Calls a subroutine via a5, performs arithmetic on a pointer, and uses loops to copy data between memory locations and update a table at offset 8 of a structure in a2.
- `transfer_register_and_branch_5`: Copies the contents of register d1 into register d0 and performs a short branch to another location.

### Port-Handler

- `process_port_data`: Initializes a large structure by copying values from a table, performs several conditional checks, and calls various routines via a5 to update specific offsets in a table based on calculated indices.

### Disk-Validator

- `process_disk_validation`: Performs complex data manipulation involving multiple library calls, updating various offsets in a structure in a6, and conditionally writing bytes to a table in a0 based on calculated indices and values.

### Ram-Handler

- `copy_memory_wrapper`: Sets up parameters and calls a library function to copy memory, then returns the difference between the source and destination pointers.

### serial.device

- `return_immediately_2`: This routine does nothing and returns immediately.
- `update_device_parameters`: Calls two library functions and, if a register check passes, updates several offsets in a structure in a6 using values from d0 and d2.
- `check_status_bit`: Performs a bitwise AND on a byte at an offset of a1 with a constant and, if the result is non-zero, calls a library function; otherwise it returns.
- `write_status_byte`: Writes the value 253 to an offset of 31 of the structure in a1 and branches to another location.
- `update_interrupt_enable`: Writes values to the INTENA register and calls a subroutine.
- `check_device_type`: Clears a byte at offset 31 of a1 and performs conditional branches based on a 16-bit value at offset 28 of a1.
- `clear_device_pointer`: Compares a1 with a value at offset 4a of a6, and if they are not equal, clears bit 1 of offset 1d8 of a6 and clears the longword at offset 4a of a6 before branching to another routine.
- `swap_device_pointers`: Checks bit 6 of a byte at offset 30 of a1 and, if set, performs a pointer swap between two structures and decrements a 16-bit value at offset 70 of a6 before branching to another location.
- `update_device_configuration`: Compares a1 with a value at offset 78 of a6, checks if a 16-bit value at offset 28 of a1 is 10, clears bit 5 of a 16-bit value at offset 486 of a6, writes 2048 to a hardware register, calls another routine, clears a 32-bit value at offset 78 of a6, clears bit 4 of a 16-bit value at offset 472 of a6, and branches to another location.
- `modify_device_state`: Checks a bit in a byte at offset 30 of a1, performs pointer swaps and register decrements, and conditionally calls a library function.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_zero_register`: Sets d0 to zero, checks a 16-bit value at offset 32 of a6, and if non-zero, sets bit 6 of a 16-bit value at offset 472 of a6.
- `reset_device_settings`: Calls four different library functions with various parameters and performs pointer arithmetic and structure updates before returning.
- `exchange_structure_pointers`: Clears a 32-bit value at offset 32 of a1, compares a 16-bit value at offset 112 of a6 with a 32-bit value at offset 54 of a1, and if the comparison passes, clears bit 0 and sets bit 6 of a byte at offset 48 of a1, performs a pointer swap between a1 and a value at offset 4 of a structure in a0, increments a 16-bit value at offset 112 of a6, and calls a library function.
- `update_linked_list_node_3`: Checks a pointer and a 16-bit counter within a structure, then replaces a node in a linked list by updating pointers and decrementing the counter.
- `process_serial_data_2`: Performs several bit tests on a structure in a6 and branches, calls another routine, and conditionally clears a bit in d0 or performs a branch.
- `test_device_bit`: Tests bit 0 of a byte at offset 443 of a6 and branches accordingly.
- `set_device_byte`: Calls another routine and, if the result is zero, sets a byte at offset 1f of a structure in a1 before branching to another location.
- `process_serial_data`: Copies a byte from offset 54 of a6 to d0, moves a 32-bit value from offset 54 of a1 to offset 130 of a6, and if d0 is non-negative, increments a 32-bit value at offset 50 of a1 and a 32-bit value at offset 130 of a6, then enters a loop that compares bytes from a table to d0 up to seven times, clears several values in a6 and a1, and calls a library function.
- `restore_registers`: Restores d2 and a6 from the stack and returns.
- `test_bit_in_mask`: Manipulates d0 to create a bitmask, then checks a bit in a structure in a6 and performs a bitwise test against a value from a global variable; otherwise it returns.
- `clear_status_bit_and_set_byte`: Clears bit 2 of a byte at offset 470 of a structure in a6; if bit 2 was clear, it moves the value 15 into a byte at offset 31 of a1, otherwise it branches to another routine.
- `clear_status_bit_2`: Clears bit 7 of a byte at offset 470 of a6, and if the bit was set, moves the value 9 into a byte at offset 31 of a1.
- `update_device_status`: Sets a byte at offset 1f of a structure in a1, tests bit 0 of offset 1d6 of a6, and if set, sets a byte at offset 1f to 6, clears a longword at offset 3e of a6, masks bit 4-7 of offset 1d6 of a6, and branches to another location.
- `update_serial_interrupt_status`: Reads the interrupt status register and writes a value to the interrupt mask register, then clears bit 6 of a byte at offset 470 of a structure in a6.
- `modify_interrupt_enable`: Compares a byte at offset 34 of a structure in a6 with d1, tests bit 2 of a byte at offset 472 of the structure, and if bit 2 is clear, clears it and writes the value 32769 to the INTENA register.
- `test_status_bits`: Tests bit 5 of a byte at offset 472 of a structure in a6 and bit 1 of a byte at offset 477 of a structure in a6, then calls another routine and performs further bit tests on d0.
- `test_bit_and_branch_2`: Tests bit 8 of d0; if bit 8 is clear, it branches to another routine, otherwise it branches to another routine.
- `test_status_bit`: Tests bit 0 of a byte at offset 445 of a6 and branches to another location.
- `set_status_bit_3`: Sets bit 7 of a byte at offset 470 of a6, and if a longword at offset 62 of a6 is zero, copies a longword from offset 42 of a6 to offset 62 of a6.
- `process_data_buffer`: Compares two longwords at offsets 54 and 58 of a structure in a6, and if the first is greater than the second, copies a byte from a0 to a pointer at offset 42 of a6, increments offsets 42 and 58, and performs further pointer and arithmetic operations before branching.
- `load_and_call_function`: Checks if a longword at offset 3e of a6 is non-zero, otherwise loads a longword from offset 2a of a6 into offset 3e of a6, calls a routine using a pointer from offset 62 of a6, and returns.
- `update_io_control_register`: Moves a byte from offset 34 of a structure in a6 to offset 473 of the same structure, clears bit 3 of a 16-bit value at offset 472, and if bit 2 of a byte at offset 69 is set, clears bit 6 of a hardware register and otherwise returns.
- `enable_interrupts_and_set_register`: Moves a byte from offset 35 of a6 to offset 473 of a6, sets bit 3 of a 16-bit value at offset 472 of a6, and if bit 2 of a byte at offset 69 of a6 is zero, sets bit 6 of a long value at offset an address and writes 32770 to a hardware register.
- `clear_status_byte`: Clears a byte at offset 473 of a structure in a6 and returns.
- `toggle_device_bit`: Tests bit 6 of offset 470 of a6 and sets it if it is not set, then initializes a structure in a1, calls a routine via a pointer at offset 20 of a1, and updates offset 31 of a1 based on the returned byte.
- `update_status_and_check_interrupts`: Saves registers, moves a pointer from a1 to a6, clears bit 6 of a byte at offset 470 of a6, and if bit 6 was set, it checks if bit 0 of the hardware register at a fixed address is non-zero; if it is zero, it sets bit 2 of a byte at offset 470 of a6 and branches to another routine, otherwise it branches to another routine.
- `load_address_and_call`: Loads the address at offset 386 of a6 into a0, calls another routine, and branches to another location.
- `set_status_bit_and_branch`: Sets bit 0 of a byte at offset 470 of a6 and then branches to another routine.
- `swap_structure_pointers`: Clears a longword at offset 32 of a1, and if offset 36 of a1 is non-zero, clears bit 0 of offset 30 of a1 and performs a series of pointer swaps between a1 and a structure in a0 before branching to another location.
- `configure_interrupt_and_status`: Tests bit 2 of a byte at offset 69 of a6, calls another routine if set, checks a byte at offset 31 of a1, sets bit 4 of a byte at offset 30 of a1, clears a 32-bit value at offset 94 of a6, copies two 32-bit pointers from a1 into a6, and writes 32770 to two hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_serial_status`: Reads a byte from the CIAB_PRA register, checks if bits 3 and 4 are set, and if bit 3 is set, writes the value 13 to offset 31 of the structure in a1.
- `handle_serial_interrupt`: Tests a byte at offset 473 of a structure in a1, saves d2 and a6, and performs multiple conditional checks on offsets 470, 78, 30, 472, 69, 31, 4, and 462 of structures, including a call to another subroutine.
- `update_interrupt_mask_3`: Writes a word to offset 476 of a structure in a6, reads a byte from a pointer in a6 into d0, and conditionally decrements a longword at offset 94 of a structure in a6.
- `manage_interrupt_requests`: Compares a longword in d1 with a longword at offset 148 of a6; if d1 is less than or equal to that value, it moves the value 1 into a longword at a fixed address, clears bit 4 of a byte at offset 472 of a6, and performs further bit tests on offsets 105 and 70 of a6 and offset 70 of a6 to determine whether to branch to another routine or set bit 4 of a byte at offset 472 of a6.
- `update_interrupt_mask_2`: Calls another routine, modifies a bitmask using a byte from offset 67 of a6 and a word from offset 468 of a6, writes the result to the hardware register at a fixed address, and if certain conditions are met, performs a loop to compare bytes and calls a routine via a pointer at offset 98 of a6.
- `update_interrupt_configuration`: Checks a word at offset 72 of a6 and, if non-zero, performs a series of pointer manipulations, updates offset 78 of a6, decrements the word at offset 72 of a6, modifies bits 4 and 6 of offset 30 of a1, clears offset 94 of a6, and calls another routine if offset 28 of a1 equals 10.
- `enable_interrupts`: Saves d2 and a6 to the stack and writes 1 to the INTENA register.
- `update_interrupt_mask`: Writes 1 to the interrupt enable register, clears d1, loads a byte from offset 313 of a1 into d1, performs a bitwise OR between d1 and the word at offset 308 of a1, clears the byte at offset 313 of a1, writes d1 to the interrupt mask register, and then performs conditional branches based on the values at offset 78 and offset 72 of a1.
- `initialize_device_parameters`: Writes a word to a hardware register, calls another routine, pushes a pointer from a1 to the stack, and if a longword at offset 116 of a6 is non-zero, it sets a1 to that longword and calls another routine; it then performs several longword writes to a structure pointed to by a1 and calls two other routines before branching to another routine.
- `branch_to_handler`: Calls a routine via a branch to stack and then performs an unconditional branch to another location.
- `copy_structure_fields`: Copies a longword from offset 38 of the structure in a6 to offset 42, copies it again to offset 46, and clears the longword at offset 58 of the structure in a6.
- `modify_status_and_branch`: Loads a longword from offset 46 of a6 into d0 and, if non-zero, sets bit 1 of offset 248 of a6, loads a longword from offset 42 of a6 into d0 and, if non-zero, calls another routine before branching.
- `clear_bit_and_set_interrupt`: Clears bit 2 of a byte at offset 472 of a structure in a6, checks if a longword at offset 74 of the same structure is zero, and if not, calls another routine before writing the value 32769 to the interrupt request register and branching to another routine.
- `toggle_interrupt_enable`: Writes 2049 to the hardware register at a fixed address, calls another routine, and then writes 34817 to the same hardware register.
- `call_subroutines_with_stack_pointers`: Pushes a pointer from a1 to the stack, calls another routine using an address derived from a6, calls another routine using a different address derived from a6, clears offset 70 of a6, and restores a1 from the stack before returning.
- `transfer_pointer_and_call`: Copies a longword from a pointer in a0 to a pointer in a1, exchanges d0 and a1, stores a0 at offset 4 of a1, and if d0 is not zero, sets a1 to d0 and calls a routine via a branch to stack before branching back to the start of the routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_bit_and_call`: Sets bit 5 of offset 30 in a1, writes 254 to offset 31 in a1, and if bit 0 of offset 30 in a1 is not set, calls a routine using a pointer from offset 98 of the structure in a6.
- `process_serial_status`: Loads a byte from a constant address into d0, masks it with 111 in binary, checks bit 2 of offset 69 of a6 and bit 3 of offset 472 of a6, sets bit 4 of d0, stores d0 at offset 81 of a1, stores a byte from offset 470 of a6 at offset 80 of a1 with the lower 5 bits masked, stores a longword from offset 58 of a6 at offset 32 of a1, masks a byte at offset 470 of a6 with 249 in decimal, and branches to another routine.
- `test_status_flags`: Clears bit 0 of offset 1e of a1, tests bit 3 of offset 4f of a1, and checks if the words at offsets 48 and 4e of a6 are non-zero.
- `configure_interrupt_and_branch`: Loads a longword from offset 106 of a6 into d0, calls a routine via a branch to stack, checks bit 5 of offset 31 of a1, and if it is not set, stores a1 at offset 78 of a6, sets bit 5 of offset 470 of a6, and writes 35328 to the interrupt enable register before branching to another location.
- `clear_field_and_call`: Calls a routine using a pointer from offset 98 of the structure in a6 and then clears the longword at offset 78 of the structure in a6.
- `handle_interrupt_condition`: Clears bit 5 of offset 470 in a1, and if bit 5 is set, checks if a word at offset 40 of a structure pointed to by a1 is 10; if so, it sets bit 1 of offset 470 in a6, clears offset 120 of a6, writes 2048 to offset 62 of a0, calls a routine using a pointer from offset 152 of a6, and writes 32769 to the INTENA register.
- `execute_library_command`: Writes 1 to the INTENA register, sets up a structure on the stack with values at offsets 48, 54, and 40, calls a routine using a pointer from offset 32 of the stack, and if the byte at offset 49 of the stack is not zero, writes 11 to that offset and clears d0.
- `modify_device_status`: Tests bit 7 of a byte at offset 79 of a structure in a1 and bit 3 of a byte at offset 472 of a structure in a6, then calls another routine and modifies bits in a1 and a6.
- `check_conditions_and_set_byte`: Clears bit 7 of a byte at offset 69 of a6, tests bit 1 of a byte at offset 472 of a6, and if bit 1 is set, or if a longword at offset 78 of a6 is non-zero, or if bit 4 of a byte at offset 30 of a1 is set, it branches to another routine; otherwise it moves the value 1 into a byte at offset 31 of a1 and branches to another routine.
- `toggle_interrupt_enable_and_branch`: Disables interrupts by writing to the interrupt enable register, calls another routine, re-enables interrupts by writing to the same register, and then branches to another routine.
- `validate_and_sync_serial_parameters`: Validates three byte values and a longword range within a structure, then copies these bytes and two longwords into a secondary structure.
- `fetch_table_value_2`: Checks a bit in a structure at offset 69 of a6; if set, it shifts a value and calls another routine to fetch a word from a table, then stores the result in a global variable.
- `fetch_table_value`: Checks bit 0 of offset 69 in a6; if set, it decrements d0, shifts it left by 1, and uses the result to fetch a word from a table located at pc plus 286 and returns it in d0.
- `set_byte_and_branch`: Stores 5 in offset 31 of a1 and performs an unconditional branch to another location.
- `perform_division_calculation`: Loads a longword from offset 450 of a6 into d0 and d1, shifts d0 left by 3, subtracts d1 from d0, compares d0 to 65535, and if d0 is not less than or equal to that value, shifts d0 right by 5, performs a 16-bit unsigned division of d0 by d1, masks d1 with 65535, shifts d1 right by 5, and branches back to an earlier instruction.
- `calculate_division_and_update_register`: Divides d0 by d1, stores the quotient in offset 464 of a6, clears bit 5 of offset 472 of a6, and if bit 0 of offset 69 in a6 is set and offset 66 in a6 is 8, it performs a bitwise OR on d0 with 15, sets bit 5 of offset 472 of a6, and writes d0 to the SERPER register.
- `initialize_serial_device`: Calls multiple library routines to configure a device, using a loop to perform operations based on the result of a library call.
- `load_serial_addresses`: Loads two addresses into registers a0 and a3 from offsets of a6 and a3, then calls another routine and branches to another location.
- `store_pointer`: Stores the longword in a0 into offset 16 of the structure in a1.
- `write_serial_parameters`: Writes a byte to offset 9 of the structure in a0, writes a longword to offset 10 of the structure in a0, and stores the contents of a3 and a6 into offsets 18 and 14 of the structure in a0 before returning.
- `call_serial_function`: Writes a longword to offset 10 of the structure in a1, writes bytes to offsets 9, 14, and 8 of the structure in a1, pushes a6 onto the stack, calls a library function using a pointer from offset 98 of a6, restores a6 from the stack, and returns.
- `configure_serial_device`: It calls two subroutines to set up data, writes a byte to offset 8 and a pointer to offset 14 of a structure, calls a library function using a pointer from a6, calls two subroutines to store results in a6, writes a value to the INTENA register, and branches.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_d0_and_branch`: Clears the d0 register and branches to an instruction at a relative offset.

### dk

- `modify_d0_and_memory`: Performs several logical OR operations on d0 and subtracts 0 from a 16-bit value at offset 76 of a structure in a6 using d0 as an index.

### is

- `clear_d0_and_decrement_a6`: Performs three logical OR operations on d0 and subtracts 0 from the address pointed to by a6.

### d

- `execute_unnamed_routine`: Performs 13 instructions without a return or a call.

### e

- `modify_keymap_data`: Clears a 16-bit value at offset 76 of a structure pointed to by a6 using a zeroed register, then performs a series of byte-wise logical OR operations on the same register with various constants.

### f

- `clear_d0_and_decrement_stack`: Clears the data register zero three times and subtracts zero from the value pointed to by the stack pointer offset by the address in a6.

### cdn

- `initialize_keymap_data`: Clears a 16-bit value in a global variable using an offset derived from a register, then performs a sequence of byte-wise logical OR operations on a register with several constant values.

### usa0

- `initialize_system_state`: Sets several global variables to zero and initializes a status flag at a fixed offset.

### i

- `clear_d0_and_decrement_a6_2`: Sets the low byte of d0 to zero three times and subtracts zero from the word at the address pointed to by a6, decrementing a6.

### usa1

- `clear_d0_and_increment_a6`: Sets the low byte of d0 to zero three times and subtracts zero from the word at the address pointed to by a6, then increments a6.

### usa2

- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### n

- `set_keymap_values_n`: Sets d0 to zero three times, subtracts zero from a word at offset 76 of the structure in a6 using d0 as an index, and then sets several bytes in d0 to specific values.

### s

- `initialize_keymap_data_2`: Clears a 16-bit value at offset 76 of a structure pointed to by a6 and populates a series of bytes into the d0 register through a sequence of logical operations.

### gb

- `set_keymap_values_gb`: Sets d0 to zero three times, subtracts zero from a word at offset 76 of the structure in a2, and then sets several bytes in d0 to specific values.

### ch1

- `set_keymap_values_ch1`: Sets d0 to zero three times, subtracts 76 from d5, and then sets several bytes in d0 to specific values.

### ch2

- `clears_and_populates_register_d0`: Clears register d0 three times, subtracts 76 from a value in register d5, and then performs a series of logical OR operations on register d0 using several immediate byte constants.

### parallel.device

- `process_parallel_data`: Sets d0 and d1 to 97, performs conditional branches based on status flags, and then modifies bytes at the addresses in a2 and a0 before clearing bit 0 of a word at the address in a6.
- `update_device_status_flags`: Clears a byte at offset 118 of a structure, checks bit 5 of a byte at offset 53 of another structure, and if set, modifies bits 5 and 4 of offsets 117 and 118 of the first structure respectively.
- `execute_parallel_command_2`: Checks a bitmask in a structure, calls an Amiga OS function, and if a word at offset 28 equals 2, modifies bits in two CIA registers.
- `execute_parallel_command`: Compares a word at offset 28 of the structure in a1 to 3, clears bit 0 of a CIA register, checks bit 0 of another CIA register, and if conditions are met, calls another routine and sets bit 2 of a byte at offset 118 of the structure in a6.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_parallel_port_byte`: Writes the byte value 253 to offset 31 of a1 and branches to another location.
- `clear_status_word_2`: Tests a longword at offset 36 of a1 and, if non-zero, branches to another location; otherwise, it clears offset 32 of a1 and branches.
- `update_structure_pointers_2`: Clears bit 0 of offset 30 of a1, checks bit 2 of offset 282 of a6, and if set, sets bit 6 of offset 30 of a1, loads a pointer from offset 80 of a6 into a0, loads a pointer from offset 4 of a0 into d0, stores a1 at offset 4 of a0, stores d0 at offset 4 of a1, stores a1 at offset of a0, stores d0 at offset of a1, and increments the word at offset 114 of a6.
- `update_parallel_status`: Sets bit 4 of a byte at offset 30 of a structure in a1, copies a pointer from offset 40 of a1 to offset 42 of a6, and performs conditional bit manipulations on offsets 116 and 118 of a6 and a hardware register at a fixed address based on values in a1.
- `update_parallel_device_queue`: Decrements a 16-bit counter in a structure, updates a pointer in a linked list, and clears bit 6 of a status byte before calling another routine.
- `handle_parallel_data`: Copies a pointer from a6 to a1, tests bit 5 of offset 30 of a1, reads a byte from the CIA hardware register at a fixed address, and if conditions are met, copies a byte to a pointer at offset 42 of a6 and executes a loop to compare bytes.
- `execute_parallel_device_command`: Copies a longword from offset 46 of a6 to offset 32 of a1, clears bit 3 of offset 118 of a6 and bit 4 of offset 30 of a1, calls a library function at offset 50 of a6, clears offset 38 of a6, and if offset 114 of a6 is not zero, clears bit 2 of offset 118 of a6 and calls a library function at offset 34 of a6 with d0 set to 16.
- `test_cia_and_branch`: Moves a byte from a0 to offset 193 of a1, branches if non-zero, tests bit 3 of offset 118 of a6, and branches or performs an unconditional branch based on the result.
- `call_device_handler_2`: Loads a pointer from offset 32 of a6 into a6, then calls a routine at an offset relative to a6, and finally branches to another location.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_function_with_parameter`: Checks a value at offset 38 of a6, and if non-zero, calls a library function at offset minus 18 of a library base found at offset 34 of a6 using a parameter passed in a1.
- `call_library_function_via_offset`: Checks a value from an offset of 38 in a structure, and if not zero, pushes two pointers onto the stack to call a function at an offset of 18 from a library base address, then clears a register and returns.
- `exchange_and_call_routine`: Loads a pointer from offset 76 of a6 into a0, moves the pointer from a1 into d0, and if non-zero, updates a pointer at offset 4 of a1, then calls another routine if d0 is non-zero.
- `clear_word_at_offset`: Clears a 16-bit word at offset 114 of a6 and returns.
- `set_hardware_bits_and_call`: Sets bit 5 of a byte at offset 30 of a1, sets a byte at offset 31 of a1 to 254, and calls a library function at offset minus 378 of a library base found at offset 50 of a6.
- `modify_cia_registers`: Calls another routine, modifies bits in a CIA register at a fixed address, modifies bits in a CIA register at a fixed address, updates offset 116 of a6, and calls another routine.
- `set_status_byte_and_branch`: Tests bit 2 of offset 118 of a6 and, if set, writes the byte value 1 to offset 31 of a1 before branching.
- `copy_data_conditionally`: Tests bit 1 of a byte at offset 53 of a1; if set, it copies a pointer from offset 54 of a1 to offset 85 of a6 and another pointer from offset 58 of a1 to offset 94 of a6.
- `manage_parallel_queue`: Calls several routines to manipulate a structure in a6, including setting a byte at offset 12 of a structure, performing a loop, and calling other routines using pointers from a6.
- `clear_and_branch`: Clears register d0 and branches to another routine.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_clipboard_handler`: Saves registers a0 and a6 to the stack, calls a routine at an address, and then increments the stack pointer by 8.
- `call_clipboard_handler_3`: It saves d0 and a1 to the stack, calls a routine at address 2754, increments the stack pointer, and returns.
- `call_clipboard_handler_2`: Pushes the pointer from a1 onto the stack, calls a subroutine at a fixed address, increments the stack pointer by 4, and returns.
- `jump_to_handler`: Performs an indirect jump to the address stored at offset 246 of a1.
- `dispatch_handler`: Pushes a pointer from a1 onto the stack, calls a routine at a fixed address, and then returns.
- `push_and_call_routine_3`: Pushes the value in a1 onto the stack, calls a routine at a fixed address, and then increments the stack pointer by 4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_clipboard_operations_2`: Performs a loop that calls a routine at an address stored in a stack-based location, using values from a2 and d3.
- `execute_clipboard_operations`: Stores a longword from d3 into a location pointed to by a pointer at offset 34 of a structure in d2, then calls three different routines using a pointer from the stack and a pointer from a structure in a2.
- `execute_library_calls_3`: Calls a library function at offset minus 138 and another at offset minus 432 using a pointer from a stack-based address.
- `update_structure_values`: Calls two routines, then if the result in d0 is not zero, accesses a structure at offset 80 of a2 to increment a longword at offset 14 of a sub-structure.
- `clear_and_call_library_2`: Clears a longword at offset 14 of a3, pushes the address at offset 72 of a2 onto the stack, calls a routine at offset 128 of a3, and branches to another routine.
- `clear_and_call_library`: Clears a 16-bit word at offset 94 of a2, calls a library function at offset 16 of a2, and returns the value from a3 in d0.
- `process_library_data`: Calls a routine at offset 0 of a2, then if a longword at offset 72 of a2 is not zero, calls a routine at offset 56 of a2 using a pointer from offset 72 of a2.
- `dispatch_library_requests`: Pushes the address at offset 72 of a2 onto the stack, calls a routine at offset 0 of a2, clears offset 86 of a2, calls a routine at offset 16 of a2, and restores registers d2 and a2 before returning.
- `process_clipboard_request`: Saves registers, loads three pointers from the stack into a2, a3, and a4, calls another routine, and then conditionally calls two different routines using values from a2 and a3.
- `compare_pointer_offsets`: Loads a longword from offset 90 of a structure in a2 into d0, compares it with a value on the stack, and performs arithmetic and comparisons using offsets 48 and 44 of the stack and offset 42 of the structure in a2.
- `compare_pointer_values`: Performs bitwise operations and comparisons on values from the stack and a structure in a2, then branches based on the results.
- `compare_and_update_pointer`: Copies a longword from d1 to d0, compares d0 with a longword at offset 14 of a structure in a1, and if d0 is lower than or equal to that value, updates a1 with a new pointer and branches to another location; otherwise it branches to another location.
- `compare_and_branch_5`: Compares d5 with the result of subtracting d2 from 512, and if d5 is less than or equal to that result, branches to another location; otherwise it moves d2 to d4 and branches to another location.
- `copy_data_with_limit`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d4 as an index, increments d4, and loops if d4 is less than 512; it then calculates a value in d5 based on d0, d2, and a pointer in a1.
- `transfer_register_value`: Moves a word from d2 to d4 and branches to another routine.
- `copy_data_and_compare`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d4 as an index, increments d4, and compares the extended d4 value against the sum of d5 and d2, branching if the comparison is not met.
- `store_pointer_and_call_library`: Stores a pointer from a1 into offset 86 of a2, calls a library function at offset 16 of a2, and returns the value from d3 in d0.
- `process_clipboard_requests_2`: Calls a routine, then performs several conditional checks on a structure in a2; it may call various library functions, perform arithmetic on values from a structure, and contains a loop that iterates using d0 and d2.
- `process_library_calls`: Pushes the address at offset 512 of a2 onto the stack, sets a1 to d4, pushes offset 18 of a1 onto the stack, pushes offset 68 of a2 onto the stack, calls a routine, compares d0 to 512, and if d0 is not 512, clears offset 94 of a2, pushes offset 530 of a2 onto the stack, pushes d4 onto the stack, and calls another routine.
- `branch_to_clipboard_address`: It pushes the address at offset 72 of a2 onto the stack, calls a routine, increments the stack pointer, and branches.
- `execute_clipboard_operation`: Clears a word at offset 94 of a structure in a2, sets bit 40 of a byte at offset 30 of a structure in a4, and conditionally calls two different routines using values from a6 and a2.
- `process_clipboard_list`: Loads a value from offset 36 of a structure in a6 into d2, performs a logical shift on d2, loads a value from offset 36 of a structure in a6 into d3, and calls a subroutine if a value from offset 134 of a structure in a2 is non-zero.
- `loop_and_update_structure`: Contains a loop that tests a longword at an offset of a pointer in d4 and calls another routine, then repeats the loop after updating d4.
- `check_limit_and_branch`: Tests d4 and branches if it is zero; otherwise it tests a longword at offset 20 of a structure in a6 and branches if it is lower than or equal to zero; if not, it calculates a value by subtracting d3 from 512, compares it to a longword at offset 20 of the structure in a6, and branches based on the carry flag; finally, it moves d3 to d1 and branches to another location.
- `copy_bytes_and_update`: Runs a loop that copies bytes from a3 to a structure at offset 18 of a1 using d1 as a counter, then calls a routine and performs arithmetic on a value at offset 20 of a6.
- `initialize_loop_index`: Moves a word from d3 to d1 and branches to another routine.
- `loop_copy_bytes_to_table`: Copies a byte from offset 18 of a1 plus offset d1 to a3, increments d1, and loops if the extended d1 value is less than the sum of d2 and d3, where d2 is loaded from offset 20 of a6; upon completion, it adds offset 20 of a6 to d5, clears offset 20 of a6, and branches to another routine.
- `call_library_function`: Stores a longword from d4 into offset 86 of the structure in a2, calls a library function, moves the result from d5 to d0, and restores registers before returning.
- `check_status_bits`: Calls a routine at offset 0 of a2, then checks bits 7 and 0 of a byte at offset 30 of a2, and if both are zero, calls a routine at offset 344 of a2.
- `modify_byte_and_call`: Clears bit 0 and sets bit 7 of the byte at offset 30 of the structure in a2, calls a library function, and restores a2 before returning.
- `modify_status_bits`: Checks bit 7 of a byte at offset 30 of a structure in a2, and if it is not set, calls a routine and potentially modifies bits 7 and 4 of that same byte before returning.
- `compare_table_values`: Loads a pointer from offset 4 of a7 into a0, then sets a1 to offset 112 of a0 and a0 to offset 96 of a0, checks if the pointers in a1 and a0 are non-zero, and compares the longword at offset 48 of a0 to the longword at offset 48 of a1.
- `branch_to_offset`: Sets d0 to zero and branches to another routine.
- `set_d0_to_zero_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_structure_fields_4`: Pushes d2 onto the stack, loads pointers from offsets 8 and 12 of the stack into a0 and a1, and moves a longword from offset 16 of the stack into d2; it then copies data between offsets 40, 16, 48, 152, 146, and 148 of the structures in a0 and a1, pushes d2 and a1 onto the stack, calls another routine, cleans the stack, and returns d2.
- `modify_clipboard_status`: Sets bit 5 of a byte at offset 30 of a structure in a3, then checks a byte at offset 66 of a structure in a2; if the byte is zero, it clears bit 1 of the byte at offset 30 of the structure in a3 and branches to another routine.
- `update_clipboard_status_2`: Sets a byte at offset 66 of a structure in a2, then checks a word at offset 64.
- `clear_and_branch_5`: Clears the word in register d3 and branches to another location.
- `update_clipboard_device`: Checks if a pointer at offset 112 of a2 is non-zero, copies a value from a structure to offset 64 of a2, and if that value is 9, calls several routines including a library function.
- `push_and_call_routine_4`: Pushes the longword in a2 onto the stack, calls a routine, increments the stack pointer by 4, writes 1 to offset 94 of the structure in a2, and branches to another location.
- `update_clipboard_status`: Clears d3, compares a word at offset 64 of a2 to 9, and if equal, calls a routine, checks if the result is zero, verifies a pointer at offset 96 of a2, and if the pointer is not null, calls another routine and writes 3 to offset 64 of a2.
- `clear_d3_and_branch_2`: Clears the 16-bit D3 register and performs a short branch to another location.
- `update_clipboard_data_2`: Loads a pointer from a structure in a2, and if not null, copies a word from offset 28 of that pointer to offset 64 of the structure in a2, then calls another routine if the word equals 9.
- `validate_and_expand_clipboard`: Checks if a word at offset 64 of a structure is equal to 2, then verifies a pointer at offset 96 and tests bit 4 of a byte at offset 48 of that pointed-to structure.
- `compare_clipboard_values`: Loads a longword from offset 90 of a2 into d2, loads a longword from offset 36 of a4 into d0, adds a longword from offset 44 of a4 to d0, compares d0 to d2, and if d2 is not lower than d0, moves a longword from offset 36 of a4 to offset 32 of a4.
- `modify_clipboard_offset`: Subtracts a longword at offset 44 of a4 from a longword at offset 90 of a2, stores the result at offset 32 of a4, and then adds that result back to the longword at offset 44 of a4 before branching.
- `clear_clipboard_status`: It clears a longword at an offset of a4, sets a longword at another offset of a4 to a constant, decrements a word in a2, calls another routine, and then enters a loop that clears a word in a2 and branches back to the start of the loop.
- `clipboard_process_request`: Compares a word at offset 64 of a2 to 3, and if equal, checks a pointer at offset 112 of a2; if that pointer's value at offset 28 is 3, it calls another routine and performs arithmetic on a value at offset 44 of the structure in a4.
- `copy_and_branch_data`: Moves a longword from offset 2c of a4 to d2, stores d2 at offset 5a of a2, pushes a4 and a3 onto the stack, calls another routine, and branches to another location.
- `check_clipboard_status`: Compares a word at an offset of a structure to four and, if equal, writes four to a different offset in a structure.
- `clear_d3_and_branch`: Clears the word in d3 and branches to another location.
- `clear_clipboard_data_conditionally`: It clears d3, compares 64 to a value at offset 64 of a2, and if they are equal, it pushes two pointers and a value from offset 90 of a2 onto the stack, calls another routine, and if the result in d0 is not 4, it adds 8 to the value at offset 90 of a2.
- `clear_clipboard_data`: Clears a longword at offset 90 of a structure in a2 and branches to another location.
- `reset_clipboard_status`: Clears a longword and a word in a structure, calls another routine, and if bit 5 of a byte at offset 30 of a structure in a3 is set, clears bit 0 of that byte and clears a byte at offset 66 of the structure in a2 before returning.
- `compare_and_load_pointer_2`: Loads a pointer from a structure in a1 into a2, then if a longword at offset 0 of that structure is not zero, compares a longword at offset 72 of a2 with a longword at offset 72 of a0.
- `set_d1_and_branch`: Sets register d1 to zero and branches to another routine.
- `call_library_with_args`: Pushes the contents of offset 4 of the structure in a2, a0, and a1 onto the stack, calls a library function, and restores a2 before returning.
- `write_status_byte_2`: Writes the byte value negative one to offset 31 of a structure pointed to by a0, then calls another routine.
- `call_routine_with_stack_data`: Pushes d2 onto the stack, loads a longword from offset 8 of the stack into d2, pushes d2 onto the stack, calls another routine, cleans the stack, and returns d2.
- `push_call_parameter_and_execute`: Pushes a value from a stack-relative location onto the stack, calls another routine, and then restores the stack and the original value from the stack into a register before returning.
- `push_and_call_subroutine`: Pushes two longwords from the stack onto the stack, calls another routine, and then pops one longword from the stack before returning.
- `push_and_call_subroutine_2`: Pushes two longwords onto the stack, calls another routine, and then pops the first longword into a register before returning.
- `execute_chained_calls`: Pushes a value from the stack into d2, then calls three different subroutines sequentially using d2 as an argument, and finally restores the stack and returns.
- `update_clipboard_buffer`: Saves registers a2 and a3, loads a pointer from a structure, and if a longword at offset 48 is zero, it updates a pointer in a structure from another structure, sets bit 4 of a byte at offset 30, and calls another routine.
- `call_clipboard_routines`: Pushes a2 to the stack, pushes an offset of a3 to the stack, calls a subroutine, pushes a2 and a3 to the stack, calls another subroutine, and restores a2 and a3 from the stack.
- `increment_clipboard_offset`: Saves registers a2 and a3, loads a pointer from a structure, and if a longword at offset 48 is zero, it increments a longword at offset 56 of another structure, updates the pointer at offset 48, and calls another routine.
- `push_and_call_subroutines`: Pushes the contents of register a2 onto the stack, pushes an offset of 112 from the structure in a3 onto the stack, calls a subroutine, pushes the contents of a3 onto the stack, calls another subroutine, and restores registers a2 and a3 from the stack before returning.
- `store_pointer_from_offset_38`: Copies a longword from offset 56 of a structure in a1 into offset 48 of a structure in a0, then calls another routine.
- `store_pointer_from_offset_3c`: Copies a longword from offset 60 of a structure in a1 into offset 48 of a structure in a0, then calls another routine.
- `update_clipboard_pointer`: Saves registers, calls another routine, and then performs bitwise operations and comparisons on values retrieved from a structure in a2 and a3.
- `compare_and_branch`: Compares a longword at an offset of a3 to d3, and if they are not equal, branches to another routine; otherwise it clears d4 and d2 and branches to another routine.
- `update_clipboard_data`: Tests d4, and if non-zero, pushes a constant onto the stack, calls a subroutine, loads the result into a3, and if a3 is non-zero, branches to another location; otherwise it writes a byte to offset 49 of a structure in a2 and calls another subroutine.
- `update_clipboard_structure`: Sets several fields in a structure pointed to by a3, calls multiple subroutines, and performs conditional logic involving a loop that iterates based on a value in a6.
- `execute_library_call`: Pushes two values onto the stack, calls a library function, compares d0 to 512, and if equal, branches to another routine; otherwise it clears a word at offset 94 of a3, subtracts 512 from a value at offset minus 4 of a6, and branches to another routine.
- `test_and_clear_word`: Tests a word at offset 94 of a structure in a3 and branches if it is zero; otherwise it tests a longword at offset minus 4 of a structure in a6 and branches if it is less than or equal to zero; if not, it pushes a3 onto the stack, calls another routine, compares the result in d0 with zero, and if not zero, clears a word at offset 94 of a3 and branches to another location.
- `process_clipboard_data`: Calls a routine with a pointer from a0, then if the result matches a value from the stack, it clears a word at offset 94 of a3, calls another routine, and if certain conditions are met, performs a series of library calls and data movements involving a3 and a0.
- `process_structure_data`: Increments a longword at offset 24 of a3, increments a word at an offset in a table, stores a3 into offset 24 of the structure in a2, and performs several conditional calls and data transformations involving a6 and a3 before returning a signed longword derived from offset 31 of the structure in a2.
- `process_clipboard_requests`: Initializes registers from the stack, calls a subroutine to decrement a value in a structure, and then conditionally calls several other subroutines based on the state of a structure in a3.
- `call_and_verify`: Pushes d4 and d3 onto the stack, calls a library function, compares d0 to d4, and if equal, branches to another routine; otherwise it clears a word at offset 94 of a3, sets d4 to zero, updates a4 from a pointer, and branches to another routine.
- `clear_status_word`: Clears a word at offset 94 of a3 and branches to another routine.
- `execute_multiple_calls_2`: Pushes d3 onto the stack and calls another routine, pushes d5 onto the stack and calls another routine, pushes d6 onto the stack and calls another routine, tests a longword at offset 68 of a structure in a3, and if it is not zero, pushes that longword onto the stack and calls another routine before branching to another location.
- `update_structure_and_call_library_2`: Calls a routine with a3, then if a word at offset 94 of a3 is non-zero and a longword at offset 90 of a3 is zero, it calls several routines and library stubs, eventually setting offsets 20 and 24 of a2 to zero and decrementing a word at a global location.
- `check_and_call_library`: Tests a word at offset 252 of a structure, and if not zero, pushes a pointer from a structure onto the stack, calls a library function, pushes a pointer from offset 220, calls another routine, moves a longword from offset 342 into d0, cleans the stack, and branches to another routine.
- `set_bit_and_return`: Sets bit 3 of the byte at the address pointed to by a5 and returns with d0 set to zero.
- `update_status_and_dispatch`: Sets a byte at offset 8 of a structure in a2 to 5, performs a bitwise AND on a byte at offset 30 of that structure, and calls a function from a table at offset 332 if a comparison with 12 fails.
- `call_table_function`: Pushes a2 to the stack, calculates an offset from a word at offset 28 of the structure in a2, calls a function from a table at offset 332, calls a library function, and restores registers before returning.
- `dispatch_write_request_4`: Pushes a pointer from a2 onto the stack, calls a routine, and if bits 7 and 6 of a byte at offset 30 of a2 are both zero, it writes the value 254 to offset 31 of a2 and potentially calls two more routines.
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
- `update_linked_list`: Loads a pointer from offset 4 of the stack into a0, stores that pointer at its own address, increments the value at that address by 4, clears the longword at offset 4 of that address, and stores the original pointer at offset 8 of that address.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_exec_rawdofmt`: Saves registers a2 through a6, loads a3 from offset 20 of the stack, loads a0 from offset 24 of the stack, loads a1 from offset 28 of the stack, loads a2 from the program counter, loads a6 from offset 4, calls a library function, restores registers, and returns.
- `store_byte_and_increment_2`: Stores a byte from d0 at the address pointed to by a3 and increments a3.

### printer.device

- `branch_to_offset_4`: Performs an unconditional branch to an offset.
- `branch_with_parameter`: Sets d0 to negative 2 and calls another routine before returning.
- `process_printer_command`: Sets bit 7 of a byte at offset 30 of a structure in a1, copies a byte from d2 to offset 31 of a1, and performs several library calls based on bits in the structure.
- `execute_library_call_2`: Calls a library routine with a value in d0, and if the result is not zero, updates a pointer at a global location with the result and stores the value in a0 at offset 34 of the structure pointed to by a1 before calling another library routine.
- `manage_printer_device`: Performs several library calls to update offsets 16, 20, and 24 of a structure, then calls a library routine to fill a memory range with a repeating pattern, and finally calls three more library routines and clears a byte at a global location.
- `perform_multiple_library_calls_2`: Loads three different pointers from a structure into a1, calls a library function three times using a library base from a structure, performs arithmetic on a1 using values from a6, and calls a library function using a library base from a structure before branching to another routine.
- `execute_handler_calls`: Calls three different routines in sequence and then returns.
- `branch_with_parameter_alt`: Sets d0 to negative 3 and calls another routine before returning.
- `set_status_bit`: Sets bit 0 of a byte at offset 61 of a6 and returns.
- `clear_status_bit`: Clears bit 0 of a byte at offset 61 of a6 and returns.
- `loop_branch_to_handler`: Checks if a longword at a location pointed to by a1 is non-zero, and if so, calls a routine and branches back to the start of this routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_subroutine`: Performs a short branch to another routine.
- `manage_device_structures`: Calls four different library routines to update various offsets in a structure in a6, including clearing a bit at offset 2486 if a comparison matches.
- `clear_bit_on_match`: Loads an address from offset 94 of a6 into a1, compares d0 to a1, and if they are equal, clears bit 1 of the byte at offset 2470 of a6 and branches to another routine; otherwise it branches to another routine.
- `set_structure_bit_and_call`: Checks two bits in d5 and d2, then if both are set, sets bit 5 at offset 30 of a structure in a1, stores a1 into a location at offset 8 of d1, and calls a routine at an offset determined by a value at offset 28 of a1 shifted left by 2.
- `call_subroutine_via_stack`: Loads a pointer from offset 4 of the stack into a0, loads a value from offset 8 of the stack into d0, and calls a subroutine via bsr.
- `set_device_status_and_handler`: Calls a routine, and if the result is zero, sets bit 0 of a byte at offset 2486 of a6, then populates a structure at offset 108 of a6 with values from d0, a0, and a constant before calling another routine.
- `adjust_stack_and_branch`: Increments the stack pointer by 8 and branches to another location.
- `set_bit_and_branch`: Sets bit 0 of a byte at offset 190 of a6, loads the address of a value at offset 94 of a6 into a1, and branches to another location.
- `check_byte_and_modify`: Loads a pointer from a constant address, checks a byte at offset 31 of that pointer, and if it is not zero, branches to another routine; otherwise, it performs a bitwise NOT and AND on a byte from offset 1554 of a6 and returns if the result is zero.
- `execute_library_sequence_2`: Calls four different library routines using a pointer from a global address, sets bit 0 of a byte from a global address, and clears bit 0 of a global variable if certain conditions are met.
- `compare_and_branch_register`: Compares d0 with a value calculated from a6, and if they are not equal, clears a bit at an offset of a6, calls a subroutine, and branches.
- `compare_and_branch_6`: Compares a longword at offset 176 of a6 with a value in d0, and if they are not equal, calls another routine and then tests d0.
- `call_and_branch`: Calls a subroutine via bsr and then branches to another location.
- `initialize_structure_fields`: Loads a pointer into a1 from offset 16 of a6, writes 9 to offset 28 of a1, copies a 32-bit value from offset 50 of a0 to offset 32 of a1, and clears offset 36 of a1.
- `call_device_handler`: Clears a byte at offset 30 of a1, then calls a routine at an offset specified by a value at offset 20 of a1.
- `compare_byte_at_offset`: Loads a pointer from offset 272 of a6 and returns if the byte at offset 8 of that pointer is not equal to 7; otherwise, it returns.
- `execute_library_calls_4`: Calls a routine at an offset specified by a value at offset 20 of a1, then calls another routine at an offset specified by a value at offset 12 of a6.
- `check_status_byte`: Loads a byte from offset 31 of a structure in a0, and if it is not zero, checks if bit 1 or 2 is set by performing an AND operation on a byte at offset 30 of a6.
- `execute_printer_library_calls`: It calls a subroutine, performs several library calls using a pointer from a global variable, sets a bit in d1, tests a pointer at offset 332 of a6, calls a library function, and if the result in d0 matches a specific address, clears a bit at offset 2486 of a6 and calls another subroutine.
- `compare_and_set_status`: Compares a value in d0 with the address in a1 plus 94, and if they are not equal, clears bit 1 of a byte at offset 9b6 of a structure in a6, calls another routine, and branches.
- `branch_on_address_comparison`: Compares the value in a1 with a constant, and if equal, calls a subroutine; otherwise, it branches to another location based on a test of d0.
- `branch_to_subroutine_2`: Calls a subroutine and then performs an unconditional branch.
- `branch_on_hardware_check`: Saves registers to the stack, compares a byte at offset 2489 of a6 to 1, and if not equal, branches to a location after pushing a value onto the stack.
- `modify_hardware_and_call`: Sets a0 to a constant, loads a pointer from offset 4 of the stack into a2, enables interrupts by writing to the INTENA register, increments a byte at offset 294 of a2, performs bitwise operations on a byte from offset 512 of a0, and calls a subroutine via a pointer from the stack.
- `execute_conditional_library_calls`: Calls a routine using a pointer from offset 24 of d1, then checks d0; if d0 is zero, it clears bit 0 at offset 2486 of a6 and calls a routine with a pointer at offset 108 of a6, or if bit 1 at offset 2486 of a6 is also set, it calls a routine with a pointer at offset 190 of a6 and returns 1.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `push_and_call_routine`: Pushes the value of a1 onto the stack twice, calls a routine at an address in a1, restores a1 from the stack, and calls another routine before returning.
- `dispatch_printer_commands`: Pushes a pointer to the stack, clears a longword at offset 32 of a structure, and calls three different subroutines conditionally based on the value of d0.
- `dispatch_printer_callback`: Pushes the pointer in a1 onto the stack twice, calls a routine at offset 12c, restores a1 from the stack, calls another routine, and returns.
- `push_and_call_printer`: As library code, it pushes a1 onto the stack twice, calls a routine, increments the stack pointer, loads a1 from the stack, and calls another routine before returning.
- `execute_callback_and_library_calls`: Calls a routine at offset 104 of a structure in a4, then if d0 is zero, performs several library calls and writes values to a structure at offset 272 of a4.
- `perform_division_loop`: Compares d2 to 65535, and if not greater, performs a series of divisions and swaps using d1 and d2, then enters a loop that subtracts a value from d0 and increments d1 using a dbra instruction.
- `calculate_offset_address`: Performs multiple signed multiplications using d0, d1, and d2, adds the results, and returns the resulting address in d0.
- `push_registers_and_call`: Pushes d2 onto the stack, moves d1 to d2, moves d0 to d1, calls another routine, restores d2 from the stack, and returns.
- `push_registers_and_call_variant`: Pushes d2 onto the stack, moves d1 to d2, moves d0 to d1, calls another routine, moves d1 to d0, restores d2 from the stack, and returns.
- `negate_and_call_routine`: Negates the values in two registers and calls another routine using a bitwise inverted version of the first register as a pointer, then returns the result.
- `negate_and_call_routine_2`: Negates two input values and performs a bitwise NOT on a third before calling another routine with the modified values.
- `call_printer_handler`: Links the stack frame, saves registers, and calls a routine at a specific address.
- `clear_printer_buffer`: Compares a longword from offset 36 of a2 to -1, and if they are equal, clears a longword at a specific address and another at a4.
- `transfer_printer_pointer`: Moves a longword from a pointer in a3 to a fixed address, clears a word at a fixed address, and branches to another location.
- `transfer_printer_data_2`: Copies bytes from a source buffer to a destination table using a loop, then performs several library calls and arithmetic operations on values from the stack and a global table.
- `zero_registers_and_branch`: Sets registers d2, d4, and d6 to zero and branches to another routine.
- `process_printer_data`: Performs a series of calculations and conditional branches, including a loop that compares a value in d1 with a value in d0, and calls three different routines based on various conditions and register states.
- `copy_data_loop`: Copies bytes from one memory range to another in a loop, then either branches to another routine or stores a long value from d5 into a structure at offset 32 of a2.
- `clear_memory_and_unlock`: Clears a long integer at offset 32 of a structure pointed to by a2, moves a byte from d2 to d0, and unlocks a resource using a pointer from a6.
- `initialize_printer_structure`: Saves registers to the stack, loads several values into d3, d4, and a3, calls a routine, and then performs conditional checks on a byte from a3 before potentially calling another routine and branching to another location.
- `clear_and_branch_3`: It clears a longword at offset -4 of a6 and branches to another location.
- `compare_and_write_byte`: Compares a byte from a structure with a value from a table, and if they match and a global variable is zero, adds 55 to a byte from another structure and stores it in a memory location.
- `compare_and_store_byte`: Compares d0 to 2 and d5 to 12, and if both conditions are met, stores the byte 75 at the address in a2, otherwise it returns.
- `store_byte_conditionally`: Compares d0 to 2 and d5 to 12, and if both conditions are met, stores the byte 75 at the address in a2.
- `conditionally_store_byte`: Checks if d5 is not equal to 9 and if the byte at a4 is not zero, then stores the byte 59 at the address in a2 and branches to another routine.
- `copy_byte_from_table`: If a value in d5 is less than 6, it uses d5 as an offset into a table at a constant address to copy a byte to the location pointed to by a2, then branches to another routine; otherwise it branches to another routine.
- `check_and_set_byte`: Checks if d5 is non-zero and if a byte at a pointer in a4 is at least 29; if both conditions are met, it stores a byte in a memory location pointed to by a2 and branches to another routine, otherwise it branches to another routine.
- `store_byte_with_logic`: Calculates a value using a multiplication of d5 and 7, compares a byte from a4 with a value in a table, and conditionally calls a routine to store an extended byte into a location in a2.
- `clear_printer_tables`: Clears several longwords at specific offsets in various memory locations and a structure pointed to by a2, then branches to another location.
- `process_indexed_byte_table`: Increments a longword at a location pointed to by a0, fetches a byte from a table using that value, calls another routine, and enters a loop that performs several table lookups, byte extensions, and calls to other routines.
- `process_device_byte`: It compares a byte at a3 to a constant, and if equal, calls another routine, compares a byte from a table to a byte from a3, and enters a loop that increments a byte in a4 and branches back to the start of the loop if a comparison with a constant succeeds.
- `compare_device_byte`: Compares a byte at the address in a3 to 91, and if they are not equal, branches to another routine; otherwise, it branches to another routine.
- `process_printer_sequence`: Compares a byte from a table with a byte at a pointer in a3, and if they match, copies a byte from another table into a4 and conditionally calls a routine.
- `set_printer_byte`: Compares a byte at the address in a3 to 10, and if they are not equal, checks a word at offset 104; if that word is not zero, branches to another routine, otherwise it writes 3 to a byte at the address in a4 and branches to another routine.
- `write_printer_data`: Writes a byte to a4, calls a routine with an extended byte, and if the result is less than zero, performs several longword updates to global variables and a structure in a2.
- `write_two_bytes`: Writes two different bytes to two consecutive memory locations calculated using a pointer in a2 and an offset in d2, then branches to another routine.
- `write_single_byte`: Writes a byte to a memory location calculated using a pointer in a2 and an offset in d2, then branches to another routine.
- `verify_device_byte`: Compares a byte at the address in a3 to 155; if equal, it pushes two longwords onto the stack, calls a subroutine, and branches to another routine.
- `calculate_pointer_offset_3`: Compares a byte at a3 to 160, loads a pointer from a global address, performs several checks on structure members, and eventually calculates an address using a value from a3 and a global longword before clearing a global variable.
- `scan_ascii_characters`: Checks if a byte at a pointer is a specific character, then iterates through a sequence of ASCII digits to increment a counter and returns.
- `increment_and_update_pointer`: Increments a1 and a global longword, then checks a byte at a1; if it is zero, it loads a pointer from a2, adds a global longword to it, and if the result is less than or equal to 256, it continues, otherwise it loads a value from a global variable, subtracts 1, and stores it in a global longword before branching.
- `calculate_pointer_offset_2`: Calculates an address by subtracting 160 from a byte in a3, shifting the result left by 2, adding it to d4, and then loading a pointer from that calculated address into a1.
- `check_byte_range`: Checks if a byte at the address in a1 is equal to 92; if not, it increments a1 and checks if the new byte is between 48 and 57 inclusive, otherwise it branches to another routine.
- `calculate_offset_and_store`: Checks if a byte at a pointer is a digit, performs a calculation to determine a value, and stores that value at an address derived from a pointer in a2 and a offset in d2.
- `move_byte_with_offset`: Copies a byte from the address in a1 to a destination calculated by adding a1 and d2 to a value at a2, then increments the pointer at a2 and branches to another routine.
- `process_printer_byte`: Loads a longword from a location in a2 into d1, increments the longword in a2, uses d1 as a base to add d2 and fetch a byte from a structure in a3, and performs several comparisons and branches based on values in global variables and d3.
- `subtract_and_clear_pointers`: Performs subtractions using values from d3 and two pointers, clears a long integer at offset 284 of a structure, and moves a long integer from a2 into d0.
- `call_with_extended_byte`: Clears a global variable, extends a byte to a word, calls a subroutine with that word, and then branches based on the result.
- `update_global_counter`: Adds d3 to a 32-bit value from a global variable and compares the result to 256; if the result is less than or equal to 256, it branches to another routine, otherwise it stores d3 in a global variable, checks a byte at an offset of a6, and either calls another routine or branches to another routine.
- `decrement_and_update_pointer`: Decrements a 32-bit value from a global variable, stores the result in another global variable, and then branches to another routine.
- `set_register_and_branch`: Sets d4 to zero and branches to another routine.
- `copy_byte_loop`: Increments a pointer in a global variable, adds a value from d2 to the pointer, and then enters a loop that copies bytes from a source table to a destination address, incrementing a counter in d4 until it reaches the value in d3.
- `clear_and_initialize_structure`: Saves d2-d3 and a2-a3 to the stack, loads three values from the stack into d3, d2, and a2, sets a3 to zero, clears the byte at the address in a2, and branches.
- `clear_byte_at_offset`: If d1 is not 59, increments a3, adds a2 to a3, and clears the byte at the resulting address before branching to another routine.
- `store_constant_to_pointer`: If the byte in d1 is 32, it loads a pointer from the stack into a0, sets d0 to 1, stores d0 at the address in a0, and branches to another routine; otherwise it branches to another routine.
- `compare_and_load_pointer`: Compares d1 to 34 and, if equal, loads a pointer from an offset of the stack into a0 and sets d0 to 2 before branching to another routine.
- `compute_and_store_scaled_value`: Calculates a destination address by adding a2 and a3, computes a value by adding a byte from d1 and a scaled byte from a structure, stores this in the destination, calls another routine, and then stores a3 into a global variable at an offset determined by d1 if several range checks on d1 fail.
- `fetch_and_increment_value`: Loads a pointer from the stack into a0, increments the value at that pointer, loads a byte from a table at a constant address using the incremented value as an offset into d0, compares the value at a0 to d1, and if d1 is not less than a0, checks if a global variable is less than 256 and calls another routine if it is; otherwise it branches to another routine.
- `transfer_byte`: Moves a byte from d0 to d1, clears d0, and then moves the original byte from d1 back into d0 before returning.
- `update_global_pointer`: Sets a1 to a constant, decrements a 32-bit value from a global variable, stores the result in the address pointed to by a1, and branches to another routine.
- `validate_and_set_flag`: Decrements a pointer in a1, and if the resulting value is not less than or equal to zero, it checks a byte from a table against a constant; if the byte masked with 127 equals 27, it sets a byte at an offset in a global variable to 1 and returns, otherwise it loops.
- `clear_and_initialize_tables`: Clears three bytes at offsets 2, 4, and 6 of a structure, clears a byte at offset 2720 of a global table, sets a byte at offset 58 of another global table to 1, clears a longword at offset 476 of a structure, and if certain conditions are met, calls two different routines and writes specific byte values to offsets 0, 1, and 2 of a structure.
- `copy_data_table`: Copies six 16-bit words from a table pointed to by a global variable into a destination table starting at offset 0 of d3.
- `compare_data_tables`: Compares six 16-bit words from a table pointed to by a global variable against a destination table in d3, returning zero if all match and negative one otherwise.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `process_callback_data`: Saves registers to the stack, copies four bytes from a structure to the stack, calls a subroutine using a pointer and a value from the structure, and then performs a comparison and a branch.
- `execute_callback_logic`: Checks d2, and if zero, calls a routine with a pointer, then if the result is non-zero, calls another routine, and if d2 and d1 are both zero, calls a third routine before restoring registers and returning.
- `execute_library_function`: Saves registers, loads a pointer from the stack, and calls a subroutine at an address retrieved from a global variable.
- `test_stack_word`: Compares a word at offset 22 of the stack to 54 and 53, and if the word is 53, stores a bitmask of the result in a byte at offset 102 of a global variable.
- `calculate_address_from_stack`: Compares a word at offset 22 of the stack to 51 and a byte at a global location to 1, then if both match, it updates offset 22 of the stack and calculates a pointer in a0 using a value from a1 and a shifted version of a word from the stack.
- `compare_and_set_byte`: Compares a byte in d1 to 255 and, if they are not equal, branches to another routine; otherwise, it sets d1 to 255 and branches to another routine.
- `loop_and_copy_bytes`: Compares d1 to a constant, and if equal, enters a loop that increments d3, calculates addresses using d2 and d4, moves bytes between memory locations, and eventually returns after checking specific values in the stack.
- `dispatch_jump_table`: Compares a value from offset 8 of a7 against 7, and if it is within range, uses it to index into a jump table to perform a jump.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `update_device_parameters_4`: Copies several words from a structure in a3 to specific locations in a table, then calls a routine via a pointer to update a value at an offset in a table.
- `call_device_routine`: Writes a word to a location pointed to by a pointer in d7, pushes several values onto the stack, calls a routine via a pointer in a2, and branches to another location if bit 0 of a value at offset 32 of a structure in a2 is not set.
- `check_device_status`: Checks specific bits in a structure at offset 21 of a2, performs several comparisons and writes to various offsets in a structure, and calculates a value by shifting a word from a structure at offset 32.
- `calculate_device_offsets`: Calculates the difference between two 16-bit values from offsets 20 and 16 of a1, adds 1 to the result, and stores it in a longword at a location pointed to by d7; it then performs a similar calculation using values from offsets 22 and 18 of a1 and stores the result in a location pointed to by d7.
- `validate_device_offsets`: Loads values from a structure pointed to by a2, performs several comparisons against values in a structure pointed to by a5, and branches or pushes a value to the stack based on the results.
- `calculate_device_offset_and_compare`: Calculates a value by subtracting two words from a structure in a2, calls a subroutine using a value from a structure in a5, and if the result is negative, it compares a word from a2 to five hundred twelve and branches based on the comparison.
- `compare_word_and_branch_3`: Compares a word in d0 with 512 and branches if not equal, otherwise it performs an unconditional branch.
- `branch_with_constant_3`: Loads the constant twelve into d1 and branches to another location.
- `branch_with_constant_alternative`: Loads the constant fifteen into d1 and branches to another location.
- `update_device_parameters_3`: Calls a subroutine with d3 in d0, then compares a longword from a structure in a2 with d3, and if d3 is smaller, performs a multiplication using a word from a structure and updates d3 before branching.
- `update_device_parameters_2`: Calls a routine, compares a value from a structure in a2 with d3, and if smaller, performs arithmetic and calls two other routines using values from a structure in a2.
- `update_device_status_2`: Stores 7 in a location at offset -4 of a6, checks a byte at offset 424 of a6, and if it is non-zero, adds 6 to itself and checks a byte at offset 426 of a6 before updating a value at offset -4 of a6.
- `clear_device_status`: Clears a longword at offset 16c, tests a byte at offset 15e and branches if zero, tests a byte at offset 166 and branches if zero, clears the stack, pushes a longword from offset 3, and branches to another routine.
- `update_device_status_and_counters`: Writes a byte to a location, performs several arithmetic operations to update three different counters in a table, updates a pointer in a table, and performs bitwise operations to update a value in a table before branching.
- `update_printer_state_3`: Writes a byte to a location, calculates two sums from words in a table and stores them in another table location, and then checks a bit in a global variable.
- `update_printer_data_2`: Performs bit tests on a global variable, manipulates a value in a4 through logical shifts, and calls a routine using a pointer from a6.
- `update_printer_parameters`: Checks a bit in a global variable, performs several arithmetic operations and calls a routine multiple times to update values at various offsets in a structure pointed to by a2, a4, or a6, and finally stores a result in a location pointed to by a2.
- `calculate_printer_parameters`: Performs several calls to a routine using values from a4, a6, and a global variable, compares results, and eventually compares values in a structure pointed to by a5 with a value in a4.
- `calculate_printer_offset_2`: Loads a word from a pointer in a global variable, compares it to a longword in a0, and if not equal, stores a byte derived from a calculation into a global variable and branches to another routine.
- `calculate_printer_offset`: Copies a word from a global variable into another global variable, calculates a difference between two words from a structure, stores the result in a global variable, and branches to another routine if a comparison between a longword and a global variable fails.
- `calculate_printer_buffer_size`: Copies two words from a structure into a global table, calculates a difference between them, stores the result in another global variable, and enters a loop that clears a longword in a table at an offset determined by a counter.
- `process_printer_data_buffer`: Calls another routine and then enters a loop to extract nibbles from a word to populate a table, followed by a second loop that processes data in chunks of 32 and 64 elements to perform bitwise operations and calls another routine to write values to a table.
- `execute_printer_routine`: Writes specific values to memory locations, loads a word from a pointer at offset 18C, and calls a routine using a pointer from offset 18E. It then stores the result in a global variable at offset 16E.
- `call_printer_routine`: Pushes a longword onto the stack, calls a routine using a pointer from offset 18e, and stores the result in a global variable.
- `test_and_call_routine_2`: Tests bytes at offsets 374 and 376, and if both are zero, calls a routine and branches to another location.
- `test_printer_status`: Tests a byte at a fixed address and, if non-zero, calls a routine and branches to another location.
- `test_and_call_routine`: Tests a byte at a fixed address, and if not zero, calls a routine at an address in a pointer and branches.
- `update_status_from_call`: Calls another routine, stores the result in d0 into a global variable, and if that variable is not zero, branches to another location; otherwise it branches to another location.
- `divide_and_call_subroutine`: Loads a word from a pointer at offset 228 of d7 into d2, performs a division using a value at offset 24 of a pointer in a2, and if the result is non-zero, calls a subroutine via a pointer in a2.
- `call_and_restore_registers`: Clears two longwords on the stack, calls a routine at an offset from the program counter, restores d2 through d6 and a2 through a5 from the stack, and returns.
- `calculate_weighted_sum`: Pushes d2 to the stack, checks a byte at offset 224 of d7, and if set, calculates a weighted sum using bytes at offsets 8, 9, and 10 of the stack, then compares a value at offset 2656 of a global table to 1 to perform further arithmetic on the result.
- `compare_stack_bytes`: Checks a bit in a structure pointed to by a value from a pointer in a global variable, then compares two sets of bytes from the stack to determine which value to store at offset 11 of the stack.
- `calculate_scaled_value_5`: Loads a byte from a table into d0, extends it to a longword, multiplies it by a word from a table, adds another extended word from a table to the result, and pushes the result onto the stack before branching to another location.
- `calculate_and_call_function`: It adds two values from a table into d0, pushes d0 onto the stack, performs a signed multiplication and addition using values from a table, calls another routine, and if the result in d3 is not 65535, stores a value from a table into a2.
- `calculate_table_index`: Tests a byte at a pointer in d7, and if non-zero, performs arithmetic on d3 to calculate an index, then loads a longword from a table into a location pointed to by a2.
- `modify_memory_and_branch`: Writes a longword to a memory location, tests bit 3 of a byte at an offset from a pointer, and either performs a subtraction or branches.
- `shift_and_compare_word`: Copies d2 to d1, copies d3 to d0, performs a logical shift right on d0, and then branches based on a comparison of d0 with the value 1.
- `compare_word_and_branch`: Compares a word in d0 to 2 and branches to a different location if they are not equal, otherwise it performs an unconditional branch.
- `decrement_and_branch_3`: Decrements a word in d0 by 3 and branches to a different location if the result is non-zero, otherwise it performs an unconditional branch.
- `write_byte_and_branch_2`: Moves a byte from d1 to a location at offset 273 and branches to another routine.
- `write_byte_and_branch_3`: Moves a byte from d1 to a location at offset 274 and branches to another routine.
- `call_and_store_pointer`: Moves a byte into a location at offset 273, pushes a pointer from a2 onto the stack, calls a subroutine via jsr, stores the result in d0 at the pointer in a2, and branches to another location.
- `update_global_counters`: Loads a value from a table using a scaled index in d0, then adds four different byte values from specific global memory locations to four corresponding long integer locations.
- `calculate_and_update_pointers`: Calculates a value by multiplying two words from a table, stores it in a table, and then performs four sequential operations where it divides a value by two, adds it to a global variable, and calls another routine to update that variable.
- `execute_conditional_callbacks_3`: Clears a byte in a structure, calls a subroutine with two words from a3 and a4, and if the result is non-zero, calls another subroutine using a pointer from a structure and stores the result.
- `dispatch_conditional_function`: Checks two bytes at offsets 384 and 348 of a structure in d7, and if both are zero, pushes two words from a3 and a4 to the stack, calls a routine via a pointer from offset 284 of d7, and then potentially calls another routine via a pointer from offset 398 of d7.
- `execute_conditional_calls`: Pushes two words from a3 and a4 onto the stack, calls another routine, and if the result is not zero, pushes two more words and calls a routine via a pointer in a0 before storing the result in a location pointed to by a2.
- `execute_multiple_calls`: Pushes two words from a3 and a4 onto the stack, calls a subroutine via a pointer from offset 352 of d7, and if the result is non-zero, pushes two more words and calls another subroutine via a pointer in a0.
- `clear_d0_and_restore_registers`: Clears d0 and restores a2 through a4 from the stack before returning.
- `increment_and_conditional_call`: Increments a word at a constant address, performs a division using a value at offset 24 of a pointer in d1, and if the result is zero, calls a subroutine via a pointer in a0 and stores the result in a2.
- `call_and_store_result`: Pushes the address of a constant onto the stack, clears three longwords on the stack, calls a routine at an offset from a pointer at offset 430 of a global table, stores the result in d0 into the location pointed to by a2, and branches based on a condition.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `setup_printer_pointers`: Saves registers a2 through a4 on the stack, loads three constant addresses into a2, a4, and a3, moves a longword from a global variable into the location pointed to by a4, and branches to another location.
- `push_parameters_and_call`: Pushes two words from a structure into the stack, calls another routine, stores the result in a location pointed to by a3, and then performs several memory moves and updates to a structure before branching.
- `printer_update_state`: Pushes two words from memory onto the stack, calls a subroutine, and then writes the result to several offsets of a structure pointed to by d7.
- `clear_and_branch_2`: Clears a 16-bit word at a specific memory location and branches to another routine.
- `update_printer_state`: Calls two subroutines and enters a loop that increments a value in a structure and compares it against a word in a register.
- `update_printer_counter`: It multiplies a byte from a structure in d7 by a word from a structure in d7, adds the result to another word in a structure in d7, increments a word in a structure in d7, and calls another routine if a comparison between two words in d7 fails.
- `increment_printer_counter`: Adds a word from a structure in a3 to a word at a location in a4, compares the result in d0 to a constant, and if d0 is not lower, clears d0 and returns.
- `clear_and_copy_counter`: Saves registers a2 through a4 on the stack, loads three constant addresses into a2, a4, and a3, clears a longword at a global variable, moves a word from a global variable into the location pointed to by a4, and branches to another location.
- `execute_printer_command`: Pushes two 16-bit values from a structure onto the stack, calls another routine, stores the returned word into a location pointed to by a register, and updates several global pointers and values before branching.
- `clear_printer_pointers`: Clears four longwords at offsets 11c, 120, 124, and 128 of a structure in a6, clears a word at offset 154 of a structure, and branches to another routine.
- `refresh_printer_buffer`: Calls a subroutine and enters a loop that increments a value in a structure and compares it against a word in a register, then calls another subroutine and writes the result to a structure.
- `call_and_store_result_3`: Calls a subroutine, stores the resulting value in a memory location, and then branches based on whether the result is zero.
- `decrement_printer_counter`: It increments a word in a structure in d7, decrements another word in a structure in d7, and if the decremented word is not positive, adds a sign-extended byte from a structure in d7 to another word in a structure in d7, then calls another routine if a comparison fails.
- `increment_and_check_printer_counter`: Adds a 16-bit value from a pointer in a3 to a 16-bit value at a pointer in a4, then compares the result against a constant in a global variable.
- `initialize_printer_state_buffers`: Clears a 32-bit value at offset 372 of a structure, copies a 16-bit word from offset 380 of a structure to offset 320 of another structure, and branches to another routine.
- `push_and_call_values`: Pushes two 16-bit values from memory onto the stack, calls another routine, stores the result in a memory location pointed to by a3, and branches to another routine.
- `update_structure_values_2`: Clears a word at offset 142, moves a longword from offset 14e to 14a, moves a longword from offset 10 to 114, moves a word from offset 17a to 13a, and then branches.
- `reset_printer_state`: Pushes two values from a pointer in d7 onto the stack, calls another routine, stores the result in a word at offset 13e of a structure in d7, clears four longwords at offsets 11c, 120, 124, and 128 of a structure in d7, clears a word at offset 13c of a structure in d7, and branches to another routine.
- `process_printer_output`: Calls two subroutines and enters a loop that increments a value in a structure and compares it against a word in a register, then calls two more subroutines and writes a longword to a structure.
- `process_printer_multiplication`: It multiplies a byte from a structure in d7 by a word from a structure in d7, adds the result to a word in a structure in d7, increments a word in a structure in d7, and calls another routine if a comparison fails.
- `update_printer_state_2`: Decrements a word at a location in a3, and if it is not higher than zero, increments a word at a location in a4, compares the result in d0 to a constant, and if d0 is not lower, clears d0 and returns.
- `initialize_printer_counters`: Saves a2-a4 on the stack, loads three constant addresses into a2, a4, and a3, clears d0, stores d0 in two different memory locations, moves a word from a location in a4 to a different location, and branches.
- `push_two_words_and_call`: Pushes two 16-bit values from a table onto the stack, calls another routine, stores the returned word into a location pointed to by a register, and adjusts the stack pointer before branching.
- `copy_printer_memory_blocks`: It clears a word at a constant address, copies three longwords from one location to another, copies a word to another location, and branches to another routine.
- `reset_printer_counters`: Clears four longwords in a structure, calls a subroutine, and then calls another subroutine while pushing two words onto the stack and writing the result to a structure.
- `update_status_from_callback`: Calls another routine, stores the returned value into a location pointed to by a2, and if the value is zero, loads it back into d0 before branching to another routine; otherwise, it loads the value into d0 and branches to another routine.
- `update_printer_buffer_and_status`: Increments one word and decrements another in a structure, then if the decremented value reaches zero, it updates a value in the structure and calls another routine before storing the result in a location pointed to by a2.
- `decrement_and_compare_counter`: Decrements a word at a location pointed to by a3, increments a word at a location pointed to by a4, compares a word in d0 to a constant, and either returns or restores registers.
- `modify_list_values`: Adds a scaled value from a stack word to a long integer at a pointer in a0, then subtracts a different scaled stack word from that same long integer if a condition is met.
- `compare_masked_data`: Saves registers to the stack, loads several values from the stack, and compares a calculated value against a byte from a global variable before branching.
- `compare_masked_bytes`: Performs bitwise AND operations on d2 and d4 with the value 3, uses the results to offset a pointer into a global table, compares the byte at that location with d3, and returns a long integer based on the comparison result.
- `compare_and_update_value`: Loads a value from the stack, calls another routine, and if the result is less than 4, adds 10 to a value and calls a second routine.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `initialize_printer_structure_2`: Clears a longword at offset 58 of a structure pointed to by a2, pushes two values onto the stack, calls another routine, and then checks a word at offset 2614 of a structure to decide whether to branch or load a new address into a2.
- `fetch_table_address`: Shifts d3 left by 2, loads a word from a table at an offset determined by the result in d0 into a2, moves the value in a2 to d5, and branches.
- `branch_to_offset_352`: Performs a short branch to an instruction at offset 352 of the current routine.
- `branch_on_byte_test_2`: Tests a byte at a location pointed to by a2 and branches to one location if non-zero, otherwise it branches to another location.
- `set_address_pointer`: It loads the constant value 20 into a4 and branches to another routine.
- `process_printer_bytes`: Increments a2, sign-extends two bytes from a4 and a2 into longwords, pushes them onto the stack, calls another routine, and if the result is not zero, enters a loop that checks bytes from a4 and a2, calling another routine if the byte from a4 is not zero; otherwise, it pushes a word onto the stack, calls another routine, and branches to another routine.
- `manage_printer_resources`: Performs a sequence of library calls and complex logic, including a loop that copies bytes from one location to another and calls several subroutines.
- `execute_dos_library_calls`: Pushes two longwords onto the stack, calls two subroutines, adjusts the stack pointer, and restores registers d2 through d6 and a2 through a5 before returning.
- `call_unload_segment`: Checks a value at offset 86 of a structure, and if non-zero, calls a function pointer at offset 8 of a second structure, then calls another routine using a pointer from offset 86 of the first structure.
- `compare_case_insensitive_bytes`: Compares two bytes from the stack by converting them to lowercase if they are within the range of alphabetical characters, and sets d0 to 1 if they match.
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
- `initialize_library_function_pointers`: Loads a function pointer from a global variable and calls it three times with different addresses passed in a1, storing each returned pointer into a structure at offsets 12, 16, and 20.
- `execute_library_calls_6`: Calls a routine twice using pointers from a global variable and a stack-based pointer, then sets d0 to 255 and branches to another routine.
- `call_library_functions`: Performs three consecutive sequences where it loads a pointer from a structure, pushes a6 to the stack, loads a function address from another structure, and calls that function.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_printer_sequence`: Calls a function from a table, and if the return value is zero, it populates a structure at an offset from a4 and performs four library calls using addresses retrieved from a structure.
- `dispatch_via_table`: Saves registers to the stack, loads values from the stack, and performs a jump to an address retrieved from a table using a value from the stack as an index.
- `return_zero`: Clears d2 and moves it into d0 before returning.
- `setup_printer_parameters`: Saves registers to the stack, loads four values from the stack into a1, a3, a4, and d2, and performs conditional branches based on a word at offset of a1.
- `copy_printer_data`: Copies bytes from a table into a destination buffer using a loop that runs up to eighteen times, then performs several conditional checks on a pointer to write specific bytes to a destination and a stack-based address.
- `update_printer_data`: Checks if a word at offset 2646 of a pointer is 2048, then performs several memory copies and writes specific bytes to a pointer in a1 and a location pointed to by a0 based on various conditions.
- `copy_bytes_from_stack_pointer`: Reads a byte from a location pointed to by a stack offset, decrements it, and stores it at the address in d4, then reads the next byte from the stack offset and stores it at the address in d2 in a loop.
- `copy_byte_from_table_in_loop`: Copies bytes from a table at offset zero to a destination calculated from a pointer in a6 and a value in d3, using a loop that runs until d5 reaches eight.
- `update_printer_status`: Compares a word at a1 to 66, writes a byte to a pointer, clears a byte at a1, and then writes either 96 or 253 to a memory location depending on a comparison of a word at an offset.
- `check_and_write_byte`: Loads a pointer from a memory location into a0, compares a word at offset 32 of that pointer against 1024, and if equal, writes the byte 137 to the address in a0 before branching to another routine; otherwise it writes 137 to the address in a0 and branches to another routine.
- `store_constant_and_branch`: Stores the byte 80 at the address in a0 and branches to another routine.
- `write_bytes_from_table`: Writes a sequence of bytes from a table into a memory location calculated from a pointer and an offset, continuing for up to eight iterations based on a value in a register, otherwise it returns.
- `update_status_byte`: Compares a word at the address in a1 to 32; if it is not equal, it returns, otherwise it tests a byte at the address in a3; if that byte is not zero, it returns, otherwise it stores the byte 1 at the address in a3, stores the word 27 at the address in a1, and branches.
- `clear_byte_and_set_word`: Clears a byte at the address in a3 if it is negative, then sets a word at the address in a1 to 4992 and branches to another routine; otherwise it branches to another routine.
- `set_d0_and_branch`: Sets d0 to 255 and branches to another routine.
- `update_status_and_value`: Compares a word at the address in a1 against 33, and if equal, tests a byte in a3; if that byte is zero or less, it writes 255 to a3 and 29 to a1 before branching to another routine; otherwise it branches to another routine.
- `test_and_clear_byte`: Tests a byte in a3, and if it is less than or equal to zero, clears that byte and writes the word 28 to the address in a1, then branches to another routine.
- `branch_to_offset_2`: Branches to a relative offset.
- `update_status_byte_via_commands`: Compares a word at a pointer against several constants to set a byte at a3 to 0, 1, or 255, or performs a sequence of writes to an address calculated from a4 and d3 if the pointer matches specific values.
- `write_control_byte`: Tests a word at the address in a1, and if it is zero, it loads a pointer from a table and writes the byte 253 to an offset of that pointer.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf:Preferences

- `initialize_system_and_process_data`: Performs library calls to find a task and open a directory, processes a data structure to copy bytes from a source to a destination while performing specific character conversions, and calls several other routines to manage stack and memory.
- `execute_library_callbacks`: Loads a pointer from a global variable into a6, then calls three different routines using a6 with arguments retrieved from a table, while also performing a library call with a specific constant in d7.
- `call_library_offsets`: Calls two different routines at offsets minus 180 and minus 174 from the library base in a6, using the address at offset 92 of a4 as the argument.
- `call_library_function_6`: Loads a constant into a1, calls a routine using a pointer from a6, stores the result in a global variable at offset 8, and otherwise returns.
- `initialize_system_preferences`: Saves registers, calls three different routines using pointers in a3 and a4, performs a loop that copies bytes from one table to another using a counter in d2, checks and updates two longwords at offsets 20 and 28 of a structure, calls a routine to process data, and performs several other subroutine calls including one that uses a pointer in a3 and another that uses a pointer in a4 before returning.
- `initialize_preferences`: Performs a complex sequence of operations including calling several routines to initialize pointers, copying 57 words between two memory areas, updating multiple global longwords and words from a structure, clearing several bytes and words in a global structure, and performing arithmetic to calculate a new address for a global pointer before calling several routines and returning.
- `manage_system_resources`: Executes a sequence of subroutine calls to initialize resources, manipulate bitfields in a global variable, and perform arithmetic-based table lookups.
- `release_resource_pointers`: Calls a routine with a pointer, extends two words to longs and pushes them to the stack, then calls four different routines using pointers from a global table.
- `call_subroutine_with_stack_data`: Pushes a longword from the stack onto the stack, accesses a structure via a pointer in a global variable, and calls a routine twice with different stack parameters.
- `process_indexed_data_in_loop`: Loads a word, a longword, and a pointer from the stack, then enters a loop that calls a routine with three parameters, increments a counter and a pointer, and repeats the process based on whether a counter reaches 4 or 20.
- `extract_bits_and_call_task`: Extracts bit fields from a word provided on the stack, uses them to calculate new values, and calls another routine with these values and a pointer.
- `update_preference_data`: Calculates an index based on a value in d3, updates a pointer in a table at offset 0 of a structure in a2, and performs several conditional updates to various offsets in a table using values from other tables and arithmetic involving multiplication by 44.
- `dispatch_via_jump_table`: Retrieves a value from a structure, uses it to index into a jump table, and jumps to the resulting address; otherwise it calls another routine.
- `process_preference_data`: Saves a2 to the stack, loads d1 and d0 from the stack, and if d0 is not zero, calls a routine with d1; it then performs several bitwise operations and calls multiple routines using values from a global table and a structure, finally returning with a2 updated from the stack.
- `update_semaphore_and_table_data`: Calls several routines to manage semaphores and signals, then performs a series of additions and subtractions using values from multiple tables to update a destination table.
- `process_structure_values`: Saves registers to the stack, performs multiple calculations involving bit shifts and logical operations on values from a structure and a second structure, calls several subroutines with various parameters pushed onto the stack, and finally calls two other routines and two library-style subroutines before restoring registers and returning.
- `clamp_and_update_structure_values`: Clamps two byte values to a specific range, checks them against values in a structure, and if certain conditions are met, updates two 16-bit fields in a structure and calls two other routines.
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
- `calculate_and_store_index`: Calculates a value based on three 16-bit words from the stack and several bit shifts, stores the result in a memory location, and calls another routine with the original stack values.
- `process_structured_data`: Calculates several values using bitwise operations and stores them in global variables, then performs three separate blocks of code that each call another routine using values from a structure.
- `process_preference_tables`: Performs a complex series of table lookups and arithmetic operations involving multiple structures, calling several routines to process data based on values retrieved from tables and comparing them against bounds.
- `calculate_pointer_difference`: Calculates the difference between a pointer in a1 and a pointer derived from a1 plus one, then returns the result in d0.
- `call_library_vector`: Pushes three values onto the stack, including a value from offset 4 of a7 and a constant, then calls a routine at an offset from a library base found in a global variable.
- `perform_complex_calculations`: Performs a series of arithmetic operations and calls several routines using a pointer in a2 and values from the stack, including multiple calls with arguments derived from d2 and d3.
- `process_bitfield_logic`: Tests various bits in a 16-bit value from the stack and, depending on the results, performs arithmetic and calls several routines using a pointer in a2 and a value from d5.
- `clamp_value_to_nine`: Compares a 16-bit value from offset 6 of a7 to nine; if it is nine, it returns zero, otherwise it returns eight.
- `decrement_and_clamp`: Decrements a 16-bit value from offset 6 of a7; if the result is not zero, it returns the decremented value, otherwise it returns sixteen.
- `update_list_elements`: Performs multiple arithmetic operations on values from the stack and calls several routines using a pointer from a global variable and a value from offset 50 of a0.
- `push_parameters_and_branch`: Checks if a word at offset 6 of the stack is non-zero, and if so, pushes four specific values onto the stack and branches to a relative offset.
- `push_stack_parameters`: Pushes three values onto the stack and calls a routine using a PC-relative address.
- `update_table_data`: Calculates various offsets using division and multiplication, writes bytes to a table pointed to by a2, and calls another routine multiple times.
- `perform_division_loop_2`: If a value in d2 is not negative one, it performs a series of divisions and then enters a loop that runs up to sixteen times to perform arithmetic and comparisons.
- `calculate_offset_address_2`: Performs multiple signed multiplications and additions using d0, d1, and d2 to calculate a final address, which is then returned in d0.
- `transfer_registers_and_call`: Pushes d2 to the stack, moves d1 to d2 and d0 to d1, calls another routine, and then restores d2 from the stack before returning.
- `transfer_registers_and_call_2`: Pushes d2 to the stack, moves d1 to d2 and d0 to d1, calls another routine, moves d1 to d0, and restores d2 from the stack before returning.
- `process_signed_value`: Pushes d2 to the stack, performs sign-related operations on d2 and d1, and calls another routine if d1 is negative.
- `process_signed_value_2`: Pushes d2 to the stack, performs sign-related operations on d2, d0, and d1, and calls another routine if d1 is negative.
- `transfer_stack_values`: Saves d2 and d3 to the stack, loads two words from the stack into d2 and d3, and branches to another location.
- `calculate_scaled_offset`: Calculates a scaled offset by multiplying a value in d2 by 44 and adding it to a base address, then calls a routine using a pointer from a global variable in a loop until d2 reaches d3.
- `load_parameters_to_registers`: Saves registers d2 and d3 to the stack, loads two 16-bit words from the stack into d2 and d3, and branches to another routine.
- `iterate_and_call_table_entries`: Calculates an offset by multiplying a value in d2 by 44, then enters a loop that calls another routine using a pointer from a table at that calculated offset.
- `dispatch_table_entry`: Loads a value from a global variable and, if it is between 0 and 7 inclusive, uses it to jump to a routine from a table; otherwise it returns with d0 set to 1.
- `calculate_and_store_values`: Loads a value from a global pointer into d3, writes 47 to two global byte locations, clears a third byte, writes 58 to a fourth, calls another routine, and performs a series of arithmetic operations including divisions and shifts to store several calculated bytes into a table.
- `process_table_indices`: Initializes several registers, loads a value from a global pointer, and enters a loop that calls another routine to perform arithmetic, compares values against constants, and eventually calls another routine with three arguments from the stack.
- `calculate_table_indices`: Performs multiple divisions by ten on four input values and uses the results to calculate and store bytes into several specific locations in a table.
- `initialize_system_resources_2`: Calls three different routines with specific parameters, updates two global byte values, performs arithmetic to calculate an index, and then calls another routine using a pointer retrieved from a table.
- `calculate_complex_offset`: Performs a series of arithmetic operations using bytes from several memory locations to update various registers, then calls three different routines and finally calls another routine with two arguments from the stack.
- `update_table_values`: Updates several bytes in a table based on comparisons, performs arithmetic using values from a table, and calls another routine if certain conditions are met.
- `update_character_data`: Sets several byte values in global locations based on comparisons, performs multiple arithmetic calculations involving shifts and subtractions using values from various global offsets, and ends with a jump to a location determined by a table lookup.
- `call_subroutine_conditionally`: Compares d3 to one, and if d3 is greater than one, sets d3 to twelve, decrements d4, and calls another routine if d4 is not negative.
- `configure_device_structure`: Loads two values from the stack, calls another routine in a loop, and if successful, performs a series of arithmetic operations to calculate an offset, stores it in a structure, and calls several routines to release resources and manage memory.
- `initialize_device_and_resources`: Clears the stack, calls two routines to populate two global pointers, and then performs a complex sequence of conditional calls to manage memory, semaphores, and device-specific logic based on bit tests and comparisons.
- `initialize_task_structure_2`: Pushes a pointer onto the stack, calls a subroutine, and if the result in d0 is not zero, performs a sequence of operations including setting a bit in a global variable, storing a pointer in a structure, and calling another subroutine with a pointer and a new address on the stack.
- `release_system_resources`: Loads a value from the stack, checks a bit in a structure, and if the condition is met, calls several routines to remove a task, close a device, and free memory.
- `send_io_request`: Sets up an I/O request structure with a command code and two long values from the stack, then calls a routine to process the request.
- `initialize_preference_list`: Allocates memory, clears several global variables, and enters a loop to copy bytes from a source to a destination while performing various checks and calling several routines.
- `compare_byte_sequences_2`: Compares two byte sequences pointed to by values on the stack, returning 1 if the first sequence is less than the second and 0 otherwise.
- `allocate_and_initialize_list`: Allocates 64k bytes of memory, updates a global pointer to point to this new memory, and then enters a loop to copy bytes from the new memory to a destination based on an offset in a global variable.
- `free_and_clear_list`: Calculates a size based on a global value, calls a routine to free memory, and then checks a pointer to decide whether to call another routine to free a second memory block.
- `validate_preference_sequence_2`: Iterates through a loop to copy bytes between tables and calls another routine, then performs a second loop to copy bytes between different tables.
- `validate_preference_sequence`: Iterates through a sequence of bytes starting from a pointer in a6, comparing them against values from a table, and returns 1 if the sequence is valid or 0 otherwise.
- `update_configuration_parameters`: Calls several subroutines to update values from a structure, performs arithmetic to calculate offsets, and branches to another routine.
- `dispatch_table_entry_3`: Clears d2, loads a pointer from a global variable into a0, compares a longword at offset 20 of that pointer to 16, and if they are not equal, it branches, otherwise it calls a routine at an address from the table and branches.
- `dispatch_result_and_message`: Calls a subroutine, moves the result to d2, pushes a longword onto the stack, calls another subroutine, moves the result back to d0, and branches based on whether d0 is less than 1, greater than 1, or between 0 and 1.
- `decrement_and_branch_4`: Decrements d0 by 2, and if it is not zero, branches to an earlier instruction; otherwise it branches to another location.
- `restore_and_branch`: Pushes two values onto the stack, calls another routine, sets d0 to zero, and branches to another location.
- `push_parameters_and_branch_2`: Pushes two 32-bit values onto the stack and branches to another location.
- `call_external_routine_2`: Loads a pointer from a location pointed to by a2 into a0, pushes a longword from an offset of a0 onto the stack, calls a routine at a specific address, stores the result in a global variable, and then branches to another location or continues.
- `restore_registers_from_stack_2`: Restores the d2, a2, a3, and a3 registers from the stack using the address pointed to by the stack pointer and returns.
- `copy_structure_words`: Copies four 16-bit words from a source structure pointed to by a6 to a destination structure pointed to by a1.
- `dispatch_via_table_5`: Loads a pointer from a structure, retrieves a value from offset 38 of that structure, and performs a jump to an address found in a table using that value as an index.
- `load_and_branch`: Pushes a value onto the stack, calls another routine, sets d0 to one, and branches to another location.
- `restore_registers_2`: Restores registers a2 through a4 from the stack and returns.
- `check_and_call_library_2`: Compares a value from a global variable against a constant and, if they match, calls another routine after performing arithmetic on a value.
- `decrement_and_branch`: Decrements a 16-bit value from the stack and branches if the result is not zero; otherwise it sets d0 to zero and branches to another instruction.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `load_long_word`: Loads a 32-bit word from an offset of 4 on the stack into d0 and returns.
- `update_port_signal`: Calls a routine to perform an operation on a structure, sets a byte at offset 24 of a structure to 1, performs several arithmetic operations using a table to calculate values, and calls another routine to set a byte at offset 24 of a structure to 255.
- `store_and_clear_register`: Stores a 32-bit value from d2 onto the stack, loads a 16-bit value from an offset of the stack into d1, and clears d2 before branching to another location.
- `compare_and_decrement`: Increments d2, sets d0 to d2, compares a value at an offset of a0 with d1, and branches based on the result of a comparison between d2 and zero.
- `decrement_d2_and_restore`: Sets d0 to the value in d2, decrements d0 by 1, loads a 32-bit word from the stack into d2, and returns.
- `process_data_table`: Clears a long word in a global variable, performs a multiplication, calls a routine, and enters a loop that copies words from one table to another, eventually updating several global pointers and performing bitwise operations on a table until a counter reaches 258.
- `call_library_function_2`: Pushes two long words onto the stack and calls a routine at a specific address.
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
- `update_pointer_structure`: Takes a pointer from the stack, sets the value at that address to itself, increments the value at that address by four, clears a long value at an offset, and sets another long value at an offset.
- `manage_configuration_files`: Attempts to open and write to files using pointers from a table, and if certain conditions are met, calls routines to close or delete files.
- `load_configuration_data`: Opens a file, reads data into a buffer, and if successful, calls a routine to process the buffer and another routine to handle the file system.
- `allocate_and_configure_port`: Pushes registers to the stack, calls several routines to allocate memory and signals, and configures a structure in a2 with values from the stack before calling more routines.
- `release_resource_structure`: If a value at an offset in a structure is not zero, it calls three different routines to free memory, release a signal, and remove a port.
- `allocate_and_initialize_structure_2`: Calls two routines to obtain pointers, then if certain conditions are met, calls a third routine and writes several values, including a pointer with an offset of 15996, into a structure.
- `signal_and_delay_handler`: Checks a pointer in a global table; if not null, it sets a bit in a global variable, performs a calculation using a byte from the table, and calls another routine, then potentially calls a second routine if certain conditions are met.
- `fetch_preference_value`: Saves registers, calls another routine, loads a pointer from offset 8, accesses a pointer at offset 56 of that pointer, sign-extends a byte from offset f of the new pointer, shifts it left by 4, pushes the result onto the stack, calls another routine, and branches to another routine.
- `dispatch_preference_message_2`: It clears d3, sets d2 to 1, loads a pointer from a2 into a0, and if a word at an offset of a0 equals 16, calls another routine and branches to the end.
- `dispatch_preference_message`: Loads a pointer from a2 into a0, compares a word at offset 20 of that pointer to 64 and then to 32, calls another routine, stores the result in d3, and if d2 is not zero, pushes a pointer from a2 onto the stack, calls another routine, and compares the result in d3 to 1; depending on the comparison, it branches to one of three other routines.
- `decrement_and_branch_5`: Decrements a word in d0 and branches if it is not zero, otherwise it branches to another location.
- `call_routine_with_stack_parameters`: Pushes two longwords onto the stack, calls a routine at an address in a pointer, clears d0, increments the stack pointer by 8, and branches to a relative offset.
- `push_parameters_and_branch_4`: Pushes two longwords onto the stack and branches to another location.
- `fetch_and_store_message`: Loads an address into a0, pushes a value from offset 56 of a0 onto the stack, calls a routine, stores the result in a2, and branches to another location.
- `restore_registers_from_stack`: Restores two 32-bit data registers and two 32-bit address registers from the stack and returns immediately.
- `copy_structure_fields_2`: Copies several word and byte fields from one structure to another and calls three different routines using pointers from the stack.
- `process_preference_data_2`: Performs multiple subroutine calls to manage data structures, updates several global variables and table entries, and uses a loop to process a list of items.
- `dispatch_via_table_2`: Loads several pointers from memory, performs a comparison on a word from a structure, and jumps to a routine address retrieved from a table.
- `restore_registers_and_return`: Clears d0 and restores d2 through d3 and a2 from the stack before returning.
- `check_byte_and_branch`: Saves registers, loads a word from offset 22 of the stack, and if a byte at offset 355 of a global variable is 3, clears d0 and branches to a routine at offset 8 of a pointer; otherwise it branches to a relative offset.
- `process_indexed_values`: Calculates three different indices from a word, stores them in global variables, and calls three different routines using values from a table and a pointer from a3.
- `calculate_and_store_offset`: Calculates a value based on offsets from the stack, stores the result in a memory location pointed to by a calculated address, and calls another routine.
- `process_table_entries_and_call_handler`: Reads multiple values from a table using an index from the stack, performs arithmetic on them, and calls another routine multiple times with these values as arguments.
- `call_library_with_parameters`: Calls another routine three times, passing a specific address and the value negative one as arguments each time.
- `execute_conditional_callbacks_2`: Performs arithmetic on values from the stack to calculate offsets, calls several routines using pointers from a table, and executes a final routine if certain conditions are met.
- `execute_conditional_callbacks_in_loop`: Calls a routine using a pointer from a structure, then enters a loop that performs bitwise operations and arithmetic to calculate values, calls another routine with three parameters, and increments two counters until they reach 15.
- `modify_table_values_with_bitmasks`: Performs several bitwise operations and arithmetic on values retrieved from a table and stores the results back into the table at calculated offsets.
- `calculate_timing_and_call_routines`: Performs arithmetic on values from two tables to calculate offsets, then calls several different routines using these calculated values.
- `call_routine_with_argument`: Pushes d2 to the stack, loads a value from the stack into d2, pushes a constant to the stack, calls another routine, and restores d2 before returning.
- `free_memory_block`: Loads a value from the stack, uses it as a pointer to call a routine that frees memory, and then restores the original value from the stack before returning.
- `allocate_and_initialize_block`: If a value in d2 is zero, it calls a routine to allocate memory and, if successful, initializes the new block with a byte, a word, and a long value before returning the pointer.
- `free_and_clear_block`: If a value in d3 is not zero, it sets several fields in a structure to specific values and calls another routine to free the memory.
- `initialize_preference_tables`: Copies bytes from one table to another in a loop, calls several subroutines to update specific values, and performs arithmetic to index into a table.
- `validate_and_call_preference`: Clears d2, sets a byte at a specific global location to 1, compares a value at offset 20 of a structure in a4 against 64 and 32, calls a subroutine, and if a condition is met, pushes a pointer from a4 onto the stack and calls another subroutine before branching based on the result in d0.
- `decrement_and_branch_alt`: Decrements d0 by 2, and if it is not equal to zero, branches to an earlier instruction; otherwise it branches to another location.
- `copy_preference_bytes`: Calls two routines, then enters a loop that copies bytes from a source table to a destination table and another table for 30 iterations if d0 is not zero.
- `copy_preference_table_loop`: Checks a value in a global variable and, if zero, enters a loop that copies bytes from a source table to two different destination tables for 30 iterations.
- `copy_preference_table_2`: Enters a loop to copy bytes between tables, then calls another routine and branches to another location.
- `call_preference_handler`: Loads a pointer from the address in a2 into a0, loads a pointer from offset 86 of that pointer into the stack, calls a routine, stores the result in the address in a4, increments the stack pointer, and branches if d0 is zero.
- `restore_registers_extended`: Restores registers d2 through d4 and a2 through a4 from the stack and returns.
- `copy_preference_table`: Copies several 16-bit and 8-bit values from one structure to another, then enters a loop that copies bytes from one table to another 30 times, finally clearing a long value in a global variable.
- `dispatch_via_table_3`: Loads a value from a structure, performs range checks, and then uses a jump table to branch to another location.
- `clear_registers_2`: Sets register d0 to zero and restores registers d2, d3, a2, a3, and a4 from the stack.
- `initialize_system_resources`: Performs a series of calls to various routines using pointers from a table, including calls that pass specific word constants and use a calculated offset from a global pointer to call another routine.
- `zero_d0_and_return`: Loads a word from an offset of 6 from the stack into d0, sets d0 to zero, and returns.
- `update_table_from_pointer`: Compares a word at a pointer to 1, and depending on comparisons with a global longword, writes values to a global pointer and calls another routine.
- `update_table_with_offset`: Calls three routines, calculates an offset using multiplication by 44, and calls a fourth routine with a pointer to a location in a table at that offset.
- `update_preference_value`: Compares a 16-bit value at a specific memory location against a global limit, and if the value is within bounds, updates a second memory location with this value and calls another routine.
- `update_table_with_offset_alt`: Calls three routines, calculates an offset using multiplication by 44, and calls a fourth routine with a pointer to a location in a table at that offset using a different global variable for the initial routine calls.
- `compare_and_call_routine_2`: Compares d0 to a longword at a specific address, and if d0 is greater than that value, writes the word 1 to a global pointer and calls another routine.
- `update_table_with_offset_third`: Calls three routines, calculates an offset using multiplication by 44, and calls a fourth routine with a pointer to a location in a table at that offset using a third global variable for the initial routine calls.
- `execute_preference_callback_2`: Calls several subroutines to update configuration data, performs bitwise operations on a byte, and calculates offsets for further subroutine calls.
- `execute_parameterized_preference`: Clears d2, compares a value at offset 20 of a structure in a4 against 64 and 32, calls a subroutine, pushes a pointer from a4 onto the stack, calls another subroutine, and branches based on the result in d0.
- `decrement_and_branch_counter`: Decrements a 16-bit value in a register by two and branches to another routine if the result is not zero, otherwise it branches to a third routine.
- `execute_parameterized_function`: Pushes two 32-bit values onto the stack, calls a routine, clears the d0 register, adjusts the stack pointer, and branches to another location.
- `push_parameters_and_branch_3`: Pushes two 32-bit values onto the stack and branches to another routine.
- `execute_preference_callback`: Loads a pointer from a structure in a2 into a0, pushes a value from an offset of 86 in that structure onto the stack, and calls another routine.
- `restore_registers_3`: Restores registers d2, a2, a3, and a4 from the stack.
- `copy_specific_bytes`: Copies a 16-bit word and three 8-bit bytes from a source structure to a destination structure using offsets provided in a1 and a0.
- `dispatch_indexed_routine`: Loads a pointer from a global structure, retrieves a 16-bit value from offset 38 of the pointed-to structure, and performs a jump to an address from a table using a calculated index if the value is between 94 and 110 decimal.
- `set_d0_and_branch_2`: Sets register d0 to one and branches to another routine.
- `set_d0_and_branch_3`: Sets register d0 to two and branches to another routine.
- `branch_to_address`: Performs a short branch to a specific address.
- `restore_registers_short`: Restores registers d2, d3, a2, and a4 from the stack.
- `update_configuration_data`: Calls several routines using a pointer from a table, then uses a value from a global variable to index into a table and calls another routine with the resulting pointer.
- `modify_global_byte`: Calls several routines using a pointer from a table, then uses a value from a global variable to index into a table to call another routine, and finally performs bitwise operations on a global variable.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr].adf:Empire

- `branch_to_offset_3`: Performs a short branch to an instruction at a fixed offset.
- `update_table_entry_3`: Saves registers to the stack, calculates an offset into a table using a value from a3, compares a longword at an offset of the calculated address with a value at another address, and if they are not equal, stores a new longword and potentially clears a following word.
- `copy_memory_blocks`: Saves registers to the stack, calls a routine via a pointer in a6, and if a condition is met, performs a series of arithmetic operations and two loops using dbra to copy bytes from a2 and a1 into a stack-based buffer.
- `cleanup_global_pointers`: Loads a pointer from the stack and calls it, then proceeds to check several global offsets in a structure and calls various routines to free memory or perform library functions based on whether those offsets are non-zero.
- `call_library_functions_2`: Loads an address from an offset of 92 in a structure pointed to by a3 into a0, calls a routine at an offset of negative 384 from a6, loads the same address into a0 again, and calls a routine at an offset of negative 372 from a6.
- `store_result_and_return`: Loads an address into a1, clears d0, calls a subroutine, stores the result in a location at a negative offset from a4, and returns if d0 is zero; otherwise it returns.
- `jump_to_zero_3`: Performs an absolute jump to zero.
- `pass_parameters_to_library`: Saves registers to the stack, loads a library base from a pointer at a fixed offset, populates a0, a1, a2, a3 and d0 through d3 from the stack, calls a routine at an offset of negative 348 from a6, and restores the registers before returning.
- `calculate_scaled_offset_2`: Writes zero to an offset of negative 32744 from a4, compares a word at offset 8 of a5 with a word at offset negative 420 from a4, and if certain conditions are met, calculates a value by multiplying a word from a5 by 6 and uses it to determine a new pointer.
- `process_offset_result`: Pushes a word from a5 onto the stack, calls another routine, and if the result in d0 is non-zero, moves a constant into d0 and branches; otherwise it returns.
- `test_and_clear_word_2`: Tests bit 2 of a byte at offset 1 of a pointer retrieved from offset minus 6 of a5, and if set, clears a word at that pointer location before branching to another location.
- `update_status_word`: Clears a word at offset -2 of a5, pushes a longword from offset 2 of a0 onto the stack, calls another routine, and if a condition is met, clears a word at a pointer in a5 and returns.
- `validate_and_call_handler`: Calculates a value by subtracting a longword from another at offsets 12 and 4 of a structure, stores it at offset -4 of a5, and if non-zero, pushes three values onto the stack and calls another routine.
- `check_bit_and_call`: Links a5, tests bit 1 of a word at offset 13 of a pointer from a5, and if set, calls another routine and stores the result at a negative offset from a5.
- `process_structure_data_4`: Clears a word at offset minus 2 of a5, loads a pointer from offset 8 of a5, and performs bitwise checks on a word at offset 18 of that pointer.
- `tiny_moveq_unlk_rts_3`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `update_pointer_from_table`: Loads a pointer from an offset relative to a4 into a0, stores it at offset minus 4 of a5, and branches based on whether the stored pointer is zero or if a word at offset 18 of the pointed-to structure is non-zero.
- `transfer_value_to_structure`: Checks a long value at an offset in a5; if non-zero, it pushes 24 onto the stack, calls another routine, and stores the result back into the structure.
- `fill_memory_loop`: Fills a memory area pointed to by a location in a5 with zeros using a loop that runs 23 times, then calls another routine with three values pushed onto the stack.
- `process_structure_byte_2`: Loads a pointer from offset 16 of a5, tests a word at offset 18 of that pointer, and if non-zero, calls another routine, stores a word from offset -528 of a4 into offset -6 of a5, and performs comparisons on a byte from offset 1 of a pointer at offset 12 of a5.
- `update_and_call_routine`: Writes a word to offset -6 of a structure in a5, increments a value at offset -10 of a5, and enters a loop that compares a byte from a structure in a0 with a constant, calls a routine, and updates a value at offset -4 of a5.
- `test_and_set_d0`: Tests a byte at an offset of negative eleven from a5; if the byte is zero, it branches, otherwise it sets d0 to 128 and branches.
- `set_bit_field`: Sets a word at offset negative 2 of a5 to the value 16386 and then branches to another routine.
- `check_structure_byte`: Tests a byte at offset minus 11 of a5 and, if zero, branches to another location; otherwise it sets d0 to 2 and branches to another location.
- `process_indexed_loop`: Initializes a counter with a value including bit 15 set, calls another routine, and then enters a loop that increments the counter and writes it to an offset in a structure until the counter reaches zero.
- `test_and_set_bit_7`: Tests a byte at offset 11 from the address in a5; if the byte is not zero, it branches to another routine, otherwise it loads the value 128 into d0 and branches to another routine.
- `set_single_bit`: Sets a word at offset negative 2 of a5 to the value 1 and then branches to another routine.
- `test_and_set_d0_constant`: Tests a byte at an offset of eleven from a pointer in a5; if the byte is non-zero, it loads the constant two into d0 and branches to another routine, otherwise it branches to another routine.
- `initialize_and_call_routine`: Sets up a word in d0 with several bits set, calls another routine, writes the result to a structure, and enters a loop that increments d0 until it reaches zero.
- `test_byte_and_set_d0`: Tests a byte at an offset of negative eleven from the structure pointed to by a5; if the byte is zero, it branches to another routine, otherwise it loads the value one hundred twenty-eight into d0 and branches to another routine.
- `write_constant_to_structure_offset`: Writes the value two as a 16-bit word to an offset of negative two from a structure pointed to by a5, then performs a short branch.
- `zero_d0_and_branch`: Sets register d0 to zero and performs a short branch into another routine.
- `initialize_structure_fields_2`: Subtracts a0 from itself, loads a pointer from an offset of 16 in a5 into a1, and writes several values to offsets of the structure in a1, including a zeroed d0 and a value from an offset of negative 6 in a5; if a bit test of that offset fails, it performs an OR operation between d0 and a value from an offset of negative 2 in a5 and stores the result at offset 18 of the structure in a1.
- `check_structure_word`: Saves registers to the stack, loads a pointer from offset 8 of a5 into a2, and compares a word at offset 12 of a5 with a word at offset 14 of a5, otherwise it sets d0 to zero and returns.
- `decrement_and_extract_byte`: Compares d5 and d4 against d6 and d7 respectively; if both are within range, it accesses a structure via a pointer in a5, decrements a word at an offset, and if that word is non-negative, increments a long value at an offset and extracts a byte from a different offset in the same structure.
- `call_and_store_result_5`: Pushes a0 onto the stack, calls a routine, and if the returned value in d0 is not negative 65535, stores it at an offset of minus 6 in a5; otherwise, it moves d5 into d0.
- `copy_byte_and_increment`: Copies a byte from an offset of negative 6 in a5 to the address in a2, increments a2 by 1, increments d4 by 1, and branches to another routine.
- `increment_d5_and_branch`: Increments d5 by 1 and branches to another routine.
- `move_d5_to_d0`: Moves the contents of d5 into d0, restores d4 through d7 and a2 from the stack, unlocks a5, and returns.
- `update_buffer_and_counter`: Compares two words from offsets 14 and 12 of a5 against zero, and if both are non-zero, reads a byte from a pointer at offset 8 of a5, decrements a word at offset 10 of a structure at offset 16 of a5, and if that word is not negative, writes the byte to a pointer at offset 4 of that same structure.
- `loop_and_call_routine_2`: Loads a word from a negative offset of a5, masks it, calls another routine, increments the value, and loops if the value is not zero.
- `increment_d4_and_branch`: Increments d4 by 1 and branches to another routine.
- `increment_d5_and_branch_2`: Increments a 16-bit value in register d5 and performs an unconditional short branch to another routine.
- `move_d5_to_d0_alt`: Moves the contents of d5 into d0, restores d2, d4 through d7 and a2 from the stack, unlocks a5, and returns.
- `push_value_to_stack`: Links the stack frame, moves a word from offset 8 of a5 into d0, pushes d0 onto the stack, and calls another routine before returning.
- `check_and_branch_structure`: Links the stack frame, saves registers, and if the word at offset 8 of a5 is greater than 0, sets d0 to 0 and branches; otherwise it continues.
- `compare_and_update_pointer_3`: Compares d7 to 8, and if less than 8, performs arithmetic on d7, loads a pointer from a location relative to a4 into a2, compares a value at offset 4 of a2 to d7, and if they are equal, updates a pointer at offset -8 of a5 with the value at a2.
- `update_pointer_structure_3`: Subtracts d7 from a value at offset 4 of a2, compares the result to 8, and if not less than 8, performs arithmetic to update a pointer at offset 4 of a new structure and stores that structure at offset -8 of a5.
- `update_pointer_and_branch`: Stores a pointer from a2 into a location at offset -8 of a5, updates a2 to point to the value at the old a2, and branches.
- `calculate_and_store_values_2`: Performs arithmetic on a word from a global variable, calls two routines, performs bitwise shifts and additions, calls a third routine, and then calls a fourth routine if a value in d0 is not zero.
- `clear_d0_and_restore`: Sets d0 to zero, restores d6, d7 and a2 from the stack, unlocks a5, and returns.
- `check_ascii_range`: Loads a 16-bit word from an offset of a5, compares it against the range between 48 and 57, and if it is within that range, sets d0 to 1 and branches.
- `load_and_test_longword`: Calls another routine, stores the result in a longword at offset minus 10 of a5, and branches if the result is non-zero, otherwise it sets d0 to negative 255 and branches to another location.
- `increment_and_store_pointer`: Adds 10 to a 32-bit value from an offset of a5, stores the result in a 16-bit word at an offset of a pointer from a5, calls another routine, and otherwise sets d0 to zero before branching.
- `update_pointer_structure_2`: Adds 10 to a 32-bit value from an offset of a5, stores the result in a 16-bit word at an offset of a pointer from a5, calls another routine, and if a global variable is not zero, updates that variable with a new pointer calculated from an offset of a5.
- `update_and_test_pointer`: Loads a 16-bit word from offset 8 of a structure, pushes it to the stack, calls another routine, stores the result back into offset -4 of the structure, and returns -1 in d0 if the result is zero.
- `free_memory_via_pointer`: Loads a pointer from a global variable, checks if it is zero, and if not, loads a 16-bit word from an offset of that pointer, pushes it onto the stack, and calls another routine using a pointer from a global variable.
- `update_linked_list_2`: Calculates the difference between two pointers, stores the result in a structure, and conditionally updates a pointer in a linked list or a secondary structure.
- `process_indexed_value`: Clears a byte at offset -1 and a word at offset -32744 of a structure in a4, then enters a loop that compares a word from offset -6 of a5 with a word at offset -420 of a4, performing arithmetic and checking a value at a calculated address before incrementing the value at offset -6 of a5.
- `compare_and_store_word_3`: Compares a word from offset -6 of a5 with a word from offset -580 of a4, and if they are equal, writes 24 to offset -576 of a4 and sets d0 to 255 before branching.
- `calculate_and_store_offset_2`: Multiplies d1 by 6, adds the result to a base address in a4, stores the resulting address at offset -18 of a5, and clears a word at offset -10 of a5 if certain bit tests pass.
- `modify_status_word`: Sets a word at offset minus 10 of a5 to 1, performs an XOR operation on a word at offset 12 of a5 based on a bit from a value at offset minus 452 of a4, and checks bit 3 of offset 13 of a5.
- `write_constant_and_branch`: Writes the value 22 to an offset of a4 and branches.
- `evaluate_status_and_call`: Loads a word from offset 12 of a5 into d0, performs bitwise operations, and if certain bits are set, writes a byte to offset -1 of a5, pushes a word from offset -10 of a5 and a longword from offset 8 of a5 onto the stack, calls another routine, and then stores d0 at offset -14 of a5 before branching.
- `process_status_updates`: Calls several subroutines to update longwords at specific offsets of a5, performs bitwise checks on a word at offset 12 of a5, and branches based on the results.
- `store_and_branch_on_value`: Pushes the word negative 1005 into d0, pushes it onto the stack, pushes a longword from offset 8 of a5 onto the stack, calls another routine, adjusts the stack, stores d0 into a structure in a5 at offset negative 14, and branches based on a word at a negative offset from a4.
- `modify_structure_values`: Writes a 16-bit word from an offset of the stack frame to a location pointed to by another offset, then writes a 32-bit long from a third offset to a location at a 2-byte offset from that same destination.
- `update_scaled_counter`: Multiplies a 16-bit value from an offset of a pointer in a4 by 3677, adds 3, masks the result with 32767, and stores it back to the same offset.
- `store_word_to_offset`: Stores a 16-bit word from an offset of 8 in a5 to an offset of -572 in a4 and returns.
- `call_pointer_subroutine`: Pushes a word from a structure in a5 onto the stack, calls another routine, stores the result in a location at an offset of negative 4 from a5, and then either sets d0 to negative 1 or branches elsewhere.
- `execute_pointer_call`: Pushes two words from a structure in a5 onto the stack, calls another routine using a pointer from a0, restores the stack, stores d0 into a structure in a5 at offset negative 8, and branches based on a word at a negative offset from a4.
- `load_pointer_from_stack`: Loads a 32-bit long from an offset of the stack frame into d0 and returns.
- `setup_stack_frame`: Creates a stack frame, moves a word from an offset of a5 into d0, pushes d0 and a pointer from an offset of a5 onto the stack, calls another routine, cleans up the stack, and returns.
- `check_signed_value`: Loads a 16-bit value from the stack into d7, and if it is not greater than zero, it sets d0 to negative one and branches.
- `update_pointer_table_3`: Calculates a value based on d7, updates a pointer at offset minus twelve of a5, and conditionally updates a pointer and a longword at offset four of a structure if certain comparisons match.
- `update_clipboard_pointer_2`: If a comparison between a2 and d2 matches, it copies a pointer from a2 to a location at offset -12 of a5, updates a longword at offset 4 of that location by adding d7 to a value from offset 4 of a2, and stores the new pointer at offset -8 of a5.
- `subtract_and_branch_2`: Loads a pointer from an offset of a5 into a1, compares a0 with a1, and if a0 is greater than or equal to a1, subtracts d7 from a longword at a negative offset of a4 and branches to another location; otherwise, it sets d0 to a constant and branches.
- `compare_and_subtract`: Compares a0 and a1, and if they are not equal, branches; otherwise, it tests a pointer in a2, and if it is not null, compares a value at a2 with d2 and subtracts d7 from a value at a negative offset of a4 before branching.
- `modify_linked_data`: Adds d7 to a 32-bit value at an offset of a2, and if a comparison between a2 and d2 matches, adds a 32-bit value from an offset of a pointer to an offset of a2 and updates that pointer with a new value.
- `update_pointer_table_2`: Stores a 32-bit value from a2 into two locations on the stack, loads a pointer from the location pointed to by a2, and branches.
- `clear_and_set_long`: Clears a 32-bit long at a location pointed to by an offset of the stack frame and then stores the value of d7 into a 4-byte offset from that same location.
- `increment_and_store_byte`: Increments two different 16-bit values at offsets of a4 and stores a byte from an offset of the stack frame into a location pointed to by the second offset.
- `update_and_clear_byte`: Clears a word at a specific offset of a4, copies a longword from a structure in a5 to a location at an offset of 25720 from a4, pushes a value from a5 onto the stack, calls another routine, clears a byte at a location pointed to by a0, and returns d0 which contains a word from a4.
- `test_character_byte`: Clears three 16-bit words on the stack, loads a byte from a pointer at offset 8 of a structure, and if that byte equals 45 decimal, sets a stack value to 1 and branches.
- `compare_and_return_pointer`: Checks if a byte at a pointer from a5 is non-zero, then checks if a byte at a pointer from a new location is non-zero, and if that byte matches a byte from a different pointer in a5, it loads that pointer into d0.
- `increment_stack_offset`: Increments a 32-bit value at an offset of a5 and branches.
- `increment_structure_offset`: Increments a 16-bit word at an offset of a5 and branches.
- `zero_d0_and_return_2`: Sets d0 to zero and returns.
- `call_subroutine_with_stack_data_2`: Pushes two long words from offsets 8 and 12 of a structure onto the stack and calls another routine.
- `compare_lowercase_characters`: Fetches two bytes from two different pointers, converts them from lowercase to uppercase if they are within the range of 97 to 122, and returns if the resulting difference is zero and the second byte was zero.
- `process_dual_byte_increments`: Checks two 16-bit words and two bytes at specific offsets of a structure, increments two different offsets, calls another routine twice with different arguments, and stores a calculated 16-bit result at offset -2 of the structure.
- `decrement_structure_offset`: Decrements a 16-bit word at an offset of a5 and branches.
- `check_byte_and_set_d0`: Checks a 16-bit word at an offset of a5, and if it is non-zero, checks a byte at a pointer from a5; if that byte is also non-zero, it sets d0 to 1.
- `set_d0_if_byte_nonzero`: Loads a pointer from an offset of a5, checks if the byte at that pointer is non-zero, and if so, sets d0 to 255 and branches.
- `unlock_and_clear_d0`: Unlocks the semaphore associated with a6 and clears register d0 before returning.
- `convert_lowercase_to_uppercase`: Loads a 16-bit word from the stack, compares it against the decimal values 97 and 122, and subtracts 32 from the value if it falls within that range before returning.
- `call_and_store_result_4`: Creates a stack frame, pushes a word from offset 8 of a5 onto the stack, calls another routine, and then stores the returned longword into offset minus 6 of a5.
- `dispatch_byte_value`: Reads a byte from a structure in a5, increments the pointer in a5, and uses the byte to either branch, compare against a table, or perform an indirect jump based on a second byte read from the same structure.
- `increment_and_copy_byte`: Increments a word at offset -4 of a5, decrements a word at offset -302 of a4, and if the new value at offset -302 of a4 is not negative, copies a byte from offset -1 of a5 to a location pointed to by offset -496 of a4.
- `process_byte_and_call`: Loads a byte from offset minus one of a structure in a5 into d0, masks d0 with 255, pushes d0 onto the stack, and calls another routine before branching to a fixed address.
- `update_structure_element`: Increments a 16-bit value at a negative offset from a pointer in a5, decrements a 16-bit counter at a negative offset from a4, and if the counter is non-negative, stores a byte from a negative offset of a5 into a destination pointed to by a pointer at a negative offset from a4 before incrementing that pointer and branching to another routine.
- `extract_and_call_structure_byte`: Loads a byte from an offset of minus one from a pointer in a5, masks it to eight bits, pushes it onto the stack, and calls another routine.
- `call_with_stack_setup`: Pushes a pointer to a location relative to a4 onto the stack, sets d0 to 255, pushes d0 onto the stack, calls another routine, adjusts the stack pointer, loads a word from offset minus four of a5 into d0, unlinks a5, and returns.
- `modify_structure_bits`: Loads a word from offset 18 of a structure at offset 8 of a5, performs bitwise operations to determine a value based on bit 15, stores that value at offset -1 of a5, and if bits 6 and 7 of the original word are non-zero, clears a word at offset 8 of the structure at offset 8 of a5.
- `update_structure_bits`: Tests bits 7 and 6 of offset 19 of a2; if both are set, it pushes a2 onto the stack, pushes negative one into d0, pushes d0 onto the stack, calls another routine, adjusts the stack, tests offset 16 of a2, clears offset 8 of a2 if offset 16 is zero, and if bit 2 of offset 19 is not set, sets offset 16 of a2 to one, calculates a new address based on a2, and stores it in offset 12 of a2 before branching.
- `call_status_handler`: Pushes a4 onto the stack, calls another routine, restores the stack, tests d0, and if d0 is zero, sets bit 5 of a byte at offset 19 of a structure in a2 before branching to another routine.
- `update_pointer_and_value`: Tests a byte at offset minus 1 of a5 and, if non-zero, increments a word at offset 8 of a2.
- `push_a2_and_call`: Pushes a2 onto the stack, calls another routine, and then branches.
- `set_bit_and_set_d0`: Sets bit 4 of a byte at an offset of a2, sets d0 to 255, and then branches.
- `move_and_branch_3`: Moves a 16-bit word from an offset of a5 into d0 and then branches.
- `update_control_bits`: Tests bit 1 of offset 19 of a2, sets bit 0 of offset 19 of a2 if bit 1 was not set, pushes a word from offset 16 of a2, a longword from offset 12 of a2, and a word from offset 20 of a2 onto the stack to call another routine, stores d0 at offset -6 of a5, and performs several bitwise checks on d0 to set bits 5 and 4 of offset 19 of a2 or negate d1 and store it at offset 8 of a2.
- `update_status_and_check_bits`: Stores d0 in offset 8 of a structure, updates a pointer at offset 4, and if a bitwise AND of a 16-bit word at offset 18 with 50 is non-zero and a byte at offset -1 of a5 is non-zero, sets d0 to 255 and stores it in offset 8 of the structure.
- `clear_and_set_d0`: Sets d0 to zero, stores it in a 16-bit word at an offset of a2, sets d0 to 255, and then branches.
- `decrement_and_update_table`: Decrements a 16-bit word at an offset of a2, and if the result is non-negative, increments a pointer at an offset of a2 and loads a byte from the new pointer into d0 after masking it.
- `call_subroutine_with_registers`: Pushes a2 onto the stack, calls another routine, restores d2 and a2 from the stack, and returns.
- `process_structure_data_8`: Links a5, saves registers, loads a word from a structure in a2 into d1, performs a bitwise AND on d2, and either branches to another routine or moves a constant into d0.
- `modify_and_call_routine`: Performs arithmetic on d2 based on bit 15 of d0, stores the result at a negative offset from a5, checks a word at offset 16 of a2 and bit 2 of d1, and if conditions are met, calls another routine and sets bit 5 of a byte at offset 19 of a2.
- `set_bit_and_negate_word`: Sets bit 1 of a byte at offset 19 of a structure in a2, then checks a byte at offset -2 of a5; if the byte is not zero, it negates a 16-bit word at offset 16 of a2 and stores it at offset 10 of a2 before branching.
- `decrement_and_copy_byte`: Decrements a 16-bit word at an offset of a2, and if the result is non-negative, increments a pointer at an offset of a2, loads a byte from a pointer in a5, and stores the masked byte at the new pointer location.
- `mask_and_call_routine`: Masks a 16-bit word from offset 8 of a structure in a5, pushes that value and the current value of a2 onto the stack, calls another routine, and then branches to another routine.
- `check_status_bit_2`: Tests bit 2 at offset 13 of a structure in a2 and branches if it is clear, otherwise it compares a word at offset 8 of a5 with 65535 and branches accordingly.
- `call_if_value_matches`: Stores d1 at offset -1 of a5, checks if the byte at offset -2 of a5 is zero or if d1 equals 10, and if so, pushes 2 to the stack, pushes the address at offset -532 of a4, pushes a word from offset 20 of a2, pushes 2 into offset -10 of a5, calls another routine, and then stores 2 at offset -6 of a5.
- `call_with_stack_parameters`: Pushes a word onto the stack, pushes a word from offset 14 of a2, pushes a word into offset -10 of a5, calls another routine, and then writes a constant to offset 8 of a5.
- `execute_complex_state_logic`: Sets a bit in a structure, performs multiple conditional checks on a word at offset 8 of a5, calls several different routines with varying stack parameters, and updates several longwords and bytes in a5 and a2 based on the results.
- `set_structure_bit`: Clears a word at a negative offset of a5, compares that word with a constant, and if it is not equal to that constant, sets bit 5 of a byte at offset 19 of a2.
- `modify_structure_bit`: Compares a word at a negative offset of a5 with d1, and if they are equal, branches; otherwise, it sets bit 4 of a byte at offset 19 of a2, tests a byte at a negative offset of a5, and if it is zero, branches; otherwise, it negates a word from offset 10 of a2 and stores it at a negative offset of a2 before branching.
- `set_byte_if_bit_set`: Tests bit 2 of a byte at offset 19 of a structure, and if it is clear, it stores zero at offset 10 of a2 and branches.
- `copy_and_decrement_count`: Copies a 16-bit word from offset 16 of a2 to offset 10 of a2, sets a pointer at offset 4 of a2 to the value at offset 12 of a2, and if a 16-bit word at offset 8 of a5 is not 65535, decrements the word at offset 10 of a2 and, if it is not negative, stores a byte from d1 into the pointer at offset 4 of a2 and increments that pointer.
- `process_structure_data_2`: Masks a byte from offset 8 of a structure in a5, pushes a2 and the masked byte onto the stack, calls another routine, and then checks bits 4 and 5 of a byte at offset 18 of a2 to either branch or set d0 to negative one.
- `compare_and_branch_2`: Loads a 16-bit word from offset minus 8 of a structure in a5 into d1; if the value is not all ones, it clears d0 and branches to another routine, otherwise it continues.
- `movem_and_unlk_a5`: Moves the contents of d1 into d0, restores d2 and a2 from the stack, unlinks a5, and returns.
- `check_structure_status`: Links a stack frame, loads a pointer from a structure, and branches to another routine if a word at offset 16 is zero or if bit 3 of a byte at offset 19 is set; otherwise it sets d0 to zero and branches to another routine.
- `copy_and_modify_fields`: Pushes a word from offset -848 of a4 to the stack and calls a subroutine, then copies d0 to offsets 4 and 12 of a structure in a0, and if d0 is not zero, writes a word to offset -848 of a4 and sets d0 to 255 before branching.
- `modify_structure_fields`: Copies a 16-bit word from an offset of minus 844 from a4 to offset 16 of a0, performs a bitwise AND on offset 18 of a0 with 65523, and writes 0 to offsets 10 and 8 of a0 before unlinking a5 and returning.
- `dispatch_table_entry_4`: Initializes a stack frame with several zeroed bytes and a pointer, then checks a byte at a pointer in a0; if that byte is not zero and meets certain criteria, it performs a jump to an address calculated using a table and the byte value.
- `apply_bitwise_or_values`: Performs bitwise OR operations on a byte at offset 96 of a4 with 32, a byte at offset 0 of a3 with 43, and a byte at offset 96 of a6 with 43, then branches.
- `write_byte_and_branch`: Writes the byte value one to offset minus 2 of a structure in a5 and branches to another location.
- `increment_and_branch_2`: Increments a longword at offset 8 of a structure in a5 and branches to another routine.
- `increment_and_compare_byte`: Increments a longword at offset 12 of a structure in a5, moves a byte from a pointer at offset 12 of a5 into d0, updates a byte at offset negative 1 of a5 with d0, and compares d0 and the byte at offset 37 of the structure in a0 to determine branching.
- `negate_conditional_longwords`: Negates a longword in d0 if it is negative, negates a longword in d1 if it is negative, calls another routine, and then negates d1 again.
- `perform_math_operations`: Pushes d2 to the stack, performs several swap operations on d0, d1, and d2, and executes two unsigned divisions using d1 as the divisor, before restoring d2 from the stack and returning.
- `multiply_and_add_values_2`: Moves d2 and d3 into a0 and a1, performs several unsigned multiplications using d0, d1, d2, and d3, adds d3 to d2, clears d2, adds d2 to d0, and returns with a0 and a1 updated.
- `copy_memory_block`: Loads two pointers and a word from the stack, and if the word is not negative and the first pointer is not greater than the second, enters a loop that moves bytes from the end of the memory blocks toward the beginning using a decrementing counter.
- `load_and_return`: Loads a longword from offset 4 of the stack into d0 and returns.
- `copy_byte_conditionally`: Loads pointers from the stack into a0 and a1, tests a byte at the address in a1, and if non-zero, copies a byte from a0 to a new location at a1 minus 1; if the byte was zero, it decrements a1 and copies a byte from a0 to a1 before checking a condition to return.
- `load_stack_parameters`: Loads two pointers from offset 4 and 8 of the stack and a word from offset 12 of the stack into a1, a0, and d0 respectively before branching.
- `validate_handler_status`: Compares d7 with a value from a4, tests a word at a pointer in a2, and tests bit 2 of a byte at offset 1 of a2 before branching.
- `initialize_offset_value`: Checks a value from the stack, and if it is zero, it stores the constant 8912 at offset 48 of a structure and returns.
- `call_exec_set_signal`: Calls a routine at an offset of a6 with a constant in d1, masks the result with 12288, stores the result at offset -4 of a5, and branches based on whether the result is non-zero.
- `execute_signal_handler`: Calls a subroutine, then conditionally calls a routine at an offset of a6, and if the result is negative one, calls another routine at an offset of a6.
- `execute_conditional_library_call`: Checks a global variable and, if non-zero, calls another routine and then calls a library function via a pointer in a structure.
- `calculate_and_branch_logic`: Calls a subroutine, then conditionally calls a routine at an offset of a6, and if the result is negative one, calls another routine at an offset of a6 before performing several comparisons on a word from a5.
- `process_clipboard_result`: Calls a subroutine, then conditionally calls a routine at an offset of a6, and if the result is zero, calls another routine at an offset of a6.
- `move_d7_to_d0`: Moves the contents of d7 to d0, restores registers from the stack, unlocks a5, and returns.
- `call_library_function_9`: Calls a subroutine, loads a longword from offset eight of a5 into d1, and calls a routine at an offset of minus thirty-six from a6.
- `execute_multiple_library_calls`: Calls a subroutine, then performs a sequence of up to four calls to routines at offsets of a6, updating a longword at offset minus four of a5 between calls, and finally calls a routine at an offset of a6 if a condition is met.
- `load_d0_from_a5`: Loads a long value from an offset of minus 4 from a5 into d0, unlocks a5, and returns.
- `execute_library_calls_8`: Checks a value at an offset in a4, calls another routine, clears a word in a4, and then calls a library function using a pointer from a6, stores the result in a structure at offset -4 of a5, and if that result is not zero, calls another library function using a pointer from a6.
- `update_pointer_and_call_library`: Loads a longword from an offset of negative four from the address in a5 into d0, calls a routine using a pointer from a4, stores d0 at offset negative four of a5, and if d0 is zero, calls another routine using a pointer from a4 and performs several writes to a4 before returning.
- `load_pointer_and_unlock_a5`: Loads a longword from an offset of negative four from the address in a5 into d0, unlocks a5, and returns.
- `open_library_and_clear_byte`: Checks a longword at offset 19088 of a4, calls a routine using a pointer from a structure at offset 4, calls another routine with a pointer from offset -32684 of a4, clears a byte at offset -80 of a5 using a value from offset -1 of a0, and calls a third routine with several parameters from the stack.
- `clear_d0_and_restore_a6`: Sets d0 to zero, restores a6 from the stack, unlocks a5, and returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `modify_pointer_structure`: Loads a pointer from offset 4 of the stack into a0, copies a0 to the location pointed to by a0, increments the value at that location by 4, clears the long value at offset 4 of the structure, and sets the long value at offset 8 of the structure to a0 before returning.
- `process_pointer_table`: Sets up a stack frame, loads a pointer from offset 8 of a5, compares a word at offset 18644 of a4 with 32, and if it is less than 32, checks a byte from the pointer in a3 to see if it is 32, 9, or 10, and if it is 10, increments the pointer in a3 and branches back to the start of the routine.
- `jump_to_address_20`: Performs a long jump to an address.
- `jump_to_handler_status_validator`: Performs a long jump to the validate_handler_status routine.
- `process_window_data`: Compares a word from a structure in a5 against constants, and if it is zero, copies four words from a5 to offsets of a4, calls a routine using a pointer from a4, and if the returned value in d0 is not zero, calls another routine before potentially clearing d0 and branching.
- `handle_menu_selection`: Loads a 16-bit word from a structure into d0, compares it to 2 and 1, and if d0 is 0, loads two pointers from a4, calls a routine at an offset of -72 from the second pointer, clears a long value at offset 19112 of a4, and if a long value at offset 19108 of a4 is non-zero, loads it and calls a routine at an offset of -450 from the second pointer.
- `update_menu_strip_status`: Calls update_menu_strip with a pointer from a structure and, if a 16-bit value at offset 14 is non-zero, sets bits 0 and 8 of a 16-bit word at offset 12 of the resulting pointer.
- `update_menu_strip`: Clears a bit at offset 12 of a structure pointed to by a5, then calls two routines using pointers from a4 to clear and set a menu strip, potentially repeating the process if a value at an offset of a4 is non-zero.
- `update_item_status`: Calls a routine via a pointer in a table, stores the result in a structure, and if a specific word is non-zero, sets bit 4 of a byte at offset 13 and clears a byte at offset 18 of that structure.
- `query_graphics_and_call_subroutine`: Calls two library functions with specific command codes, calculates four values from a structure, pushes them onto the stack along with a pointer, and calls another routine.
- `compare_and_call_subroutine`: Loads a long value from a structure into d1, compares it to a value at offset -17966 of a4, and if they are equal, pushes several values and constants onto the stack before calling a subroutine at a relative offset.
- `load_resource_pointers`: Links a stack frame, loads a pointer from offset 8 of a5 into a0, retrieves words from offsets 8 and 10 of that pointer, stores bytes from d0 into offsets -12 and -11 of a5, and branches if a longword at offset 19108 of a4 is non-zero.
- `compare_resource_deltas`: Loads a pointer from a4, performs several subtractions between bytes and words from the pointed-to structure, and compares the results against d5 and d4.
- `calculate_scaled_resource_deltas`: Calculates percentage-based scaling and resource deltas by dividing values from a structure by 100 and 100, calling two subroutines, and then performing multiple subtractions using bytes from a structure and words from a stack frame to update several 16-bit values at negative offsets of a5.
- `divide_and_scale_values`: Divides d3 by 2, adds a word from offset -4 of a5 to the result, performs a second division using d5 and d1, adds the results, performs subtractions using bytes from offsets 55 and 57 of a3, and stores the final result in d6 and 1 in offset -40 of a5.
- `calculate_scaled_differences_2`: If a word at offset minus 34 of a5 is not less than or equal to zero, it subtracts two bytes from offset 54 and 56 of a3 from a word at offset 8 of a3, performs several divisions, and stores the results in d6 and d7.
- `calculate_scaled_differences`: If d0 is not less than or equal to zero, it subtracts bytes from offset 54 and 56 of a structure in a3 from a word at offset 8 of a3, performs two division operations using d1 and d2, and stores the result in offset minus 40 of a structure in a5.
- `calculate_structure_fields`: Performs division operations using values from a5 and d5 to update offsets -30 and -32 of a structure in a5, then updates offsets -34 and -36 based on a comparison of a word from offset -24896 of a4 and the newly calculated values.
- `process_structure_values_2`: Adds a word from a global variable to d3, updates several fields in a structure in a5 based on comparisons, and calls another routine if a specific condition is met.
- `push_and_branch`: Pushes a word from offset c of the structure in a5 and a pointer from offset 8 of the structure in a5 onto the stack, calls a routine, and branches to another location.
- `calculate_scaled_indices`: Calculates two values by dividing the negation of words at offsets 34 and 36 of the structure in a5 by 8, stores these results back into those offsets, and then performs conditional logic to update offsets 30 and 32 of the structure in a5 using a global long value.
- `calculate_and_call_subroutine`: Adds a word from a4 to d1, stores values in a5, compares d2 to 39 and branches if less than or equal, otherwise it performs arithmetic and calls another routine before branching to another location.
- `push_and_branch_2`: Pushes a word from offset 12 of a5 and a pointer from offset 8 of a5 onto the stack, calls a subroutine, and branches to another location.
- `calculate_and_dispatch_values`: Performs several arithmetic operations using bytes and words from a structure in a3 to populate d6 and d7, then pushes these values and others onto the stack before calling two different subroutines.
- `update_calculated_values`: Performs arithmetic on bytes and words from a structure in a3 to calculate values for d6 and d7, calls a library function via a pointer in a6, and calls another routine multiple times with various parameters from a3 and a5 before updating values in a5 and branching.
- `calculate_and_update_offsets`: Performs arithmetic using bytes from a structure in a3 and compares the result against values in a structure in a5, then writes a calculated word to offsets -2 and -6 of the structure in a5.
- `compare_and_update_values`: Performs arithmetic on bytes from a structure in a3 and compares the result against values in a structure in a5, branching based on several conditional tests.
- `perform_arithmetic_updates`: Subtracts d0 from d1, stores the result in a structure, performs several additions and subtractions involving d1, d3, d5, and d7, updates values at offsets -4, -8, and -11 of a5, and compares d2 with d6.
- `calculate_and_call`: Performs arithmetic on d0, d1, d3, d4, and d6, stores values in offsets -6 and -2 of the structure in a5, compares values to decide whether to update offset -2 of the structure in a5, and if certain conditions are met, pushes several extended long values and a value from offset 50 of the structure in a3 onto the stack to call another routine.
- `set_bit_and_call_library`: Sets bit 0 of a byte at offset 25 of a structure in a3, loads a pointer from offset 86 of a3 into a0, and calls a library routine using a pointer from a6.
- `process_menu_and_stack`: Copies two words from a structure pointed to by a5 into a new structure, calls a routine using a pointer from a global variable and a value in a3, and if the result in d0 is zero, calls another routine and then potentially clears the stack and calls a third routine.
- `increment_global_counter`: Increments a 16-bit word at an offset of a4 and moves the value of a3 into d0 before returning.
- `handle_pointer_logic`: Moves a pointer from a structure in a1 to a location in a5, calls a routine using a pointer from a6, and compares a value in a5 with a global variable before potentially calling another routine.
- `handle_pointer_and_library_call`: Loads a pointer from offset 19104 of a4 into a0, stores it at offset minus four of a5, decrements a value at offset minus 17768 of a4, and if the result is not negative, compares a0 with offset 19104 and offset 19108 of a4; if matches occur, it calls a routine via a pointer in a6 using a value from offset 86 of a structure in a1, and depending on the return value in d0, may call another routine via a6.
- `handle_refresh_event`: Loads a pointer from a5, calls a routine, compares a value to 4, calls another routine, compares a pointer to a value in a4, and if a word at offset -25212 of a4 is zero, calls a routine with two arguments on the stack.
- `compare_and_clear_bit`: Compares a longword at offset 19108 of a4 with a0, and if they are equal, calls a routine, loads a pointer from offset -4 of a5 into a0, calls a library routine using a pointer from a6, calls another routine, and finally clears bit 0 of a byte at offset 37 of the structure in a0.
- `traverse_and_modify_linked_data`: Loads a long value from offset 16 of a structure pointed to by a0 into offset -12 of the structure in a5, then if that value is non-zero, loads a long value from offset 18 of the new address into offset -8 of the structure in a5, and finally writes a byte from offset 12 of the structure in a5 into offset 4 of a structure pointed to by a0.
- `transfer_structure_data`: Loads a pointer from offset -12 of a5 into a0, moves a long value from offset 26 of the address in a0 to offset -4 of a5, and if that value is non-zero, moves a byte from d0 into the address in a0 and moves a long value from offset 16 of a0 to offset -4 of a5.
- `swap_pointer_values`: Loads a pointer from an offset of the structure in a5 into a0, moves the long value at the address in a0 into an offset of the structure in a5, and branches to another location.
- `update_linked_node`: Loads a pointer from offset 8 of a5 into a0, moves a long value from offset 24 of a0 to offset -4 of a5, and if that value is non-zero, moves a byte from d0 into the address in a0 and moves a long value from offset 16 of a0 to offset -4 of a5.
- `update_linked_list_node_2`: Moves a pointer from a structure to a new location, checks if it is null, and if not, updates a byte at an offset within the new structure before updating the pointer again.
- `unlock_and_return`: Unlocks a5 and returns immediately.
- `process_memory_list`: Checks a global pointer, clears a 16-bit value at offset -2 of the structure in a5, and if that value is less than 3, calculates an offset to call a library routine and clears a long value at a specific offset within a structure.
- `jump_to_address_30`: Performs a jump to a longword address.
- `jump_to_structure_data_handler`: Redirects execution to a routine that queries two status values from a library, extracts three bytes from a structure to calculate new values, and calls another routine.
- `jump_to_address_21`: Performs an indirect jump to the address stored in a longword.
- `call_library_offset_38`: Calls a library function at offset 38 from a library base loaded from a global variable.
- `call_library_offset_0`: Calls a library function at offset 0 from a library base loaded from a global variable.
- `jump_to_menu_selection_2`: Performs an indirect jump to a specific address and calls process_menu_selection.
- `jump_to_address_9`: Performs a jump to the address stored at offset 4.
- `jump_to_address_40`: Performs a long jump to the address stored at the location pointed to by the instruction.
- `modify_and_call_subroutine`: Checks if a word from a5 is negative; if so, it stores 45 at a location calculated from a4 and a5, calls another routine with a5 and a4 on the stack, and clears a byte at a location calculated from a4 and a5.
- `dispatch_scaled_byte`: Divides a 16-bit value from a structure by 10, calls another routine if the result is non-zero, and then stores a byte calculated from a second structure value into a table at an offset determined by the first division result.
- `process_system_message`: Tests a byte at an offset of negative 18132 from a4, calls another routine if non-zero, tests a word at offset negative 18134 from a4, and if non-zero, calls a routine at an address from a pointer in a6 using an address from offset negative 18138 of a4, then potentially calls another routine via a pointer in a6 and stores the result in d0 at offset negative 4 of a5.
- `push_and_call_routine_2`: Pushes a longword from an offset of a5 onto the stack, calls another routine, and branches to another location.
- `loop_and_clear_memory`: Enters a loop that calculates an address based on a value at an offset of -6 from a5, calls a routine via a pointer in a6, clears a longword at an offset of 4 from that address, and decrements the value at offset -6 of a5.
- `close_device_and_jump`: Tests a long value at a negative offset from a4, calls a routine at an offset from a6, and performs a jump to a long address after pushing a value from a4 onto the stack.
- `dispatch_indexed_pointer`: Calculates an index by shifting a word from a global variable, performs a bitwise AND with another word from a global variable, calls another routine if the result is non-zero, and finally performs a long jump to an address in a table using an offset derived from a value at offset 8 of the structure in a5.
- `push_registers_and_call_2`: Pushes d2, d0, d1, and d2 onto the stack, clears the word at the top of the stack, calls another routine, and then restores d2 from the stack before returning.
- `modify_structure_and_copy`: Performs multiple operations including copying 16 words from a source at offset 24930 of a4 to a destination at an offset of d0 from a5, updating various fields in a structure at offset negative 18138 of a4, and calling a routine via a pointer on the stack.
- `process_and_copy_data`: Loads a byte from a4, extends it to a long, and if non-zero calls another routine; it then copies 16 words from a source at an offset of 24930 from a4 to a destination pointed to by a1, and finally calls a routine at a relative offset after updating several fields in a structure pointed to by a0.
- `retrieve_and_check_pointer`: Sets up a stack frame, saves a6, loads a pointer from an offset of a4 into a0, calls a routine at an offset of a6, stores the result in d0 at an offset of a5, and branches to another location if d0 is not zero.
- `free_memory_and_decrement_counter`: Sets up a stack frame, loads a pointer from an offset of a5 into a0 and a1, calls a routine at an offset of a6, decrements a word at an offset of a4, and returns.
- `jump_to_pointer_5`: Performs a long jump to an address stored in d2.
- `jump_to_external_address`: Performs an unconditional jump to a long address stored in memory.
- `jump_to_address_5`: Performs a jump to a long address.
- `jump_to_zero_2`: Performs a jump to address zero.
- `execute_conditional_subroutines_3`: Calls two subroutines, then calls a third via a branch with link, and if a word at an offset from a4 is zero, calls a fourth subroutine before returning.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_sequence_of_subroutines`: It calls several subroutines, pushing various values onto the stack as arguments, clears a word at offset -25212 of a4, and performs conditional jumps based on the results in d0.
- `dispatch_library_calls_2`: It calls a routine, clears a word at an offset of a5, loads a pointer from an offset of a structure in a1, calls a library function using a pointer from a global variable, and then either branches or calls the library function again based on the result in d0.
- `initialize_status_fields`: Sets up a stack frame, moves a word from a4 offset minus 24624 to a structure in a5, sets three words in that structure to specific values, and branches to another routine if certain conditions in a4 are met.
- `validate_range_values`: Compares a word from a5 offset 8 against two constants, compares a word from a5 offset 10 against two other constants, and if all conditions are met, moves d1 to d0 and branches to another routine.
- `process_data_structure`: Pushes two words onto the stack, calls a subroutine, stores the result at offset minus four of a structure in a5, and if a value at a4 offset minus 18516 is not zero, performs several comparisons and calls three different subroutines before branching to another routine.
- `loop_library_calls`: Checks a word at offset -36 of a5, and if non-zero, calls a library routine using a pointer from a structure at offset 86 of offset 19104 of a4, stores the result in offset -14 of a5, and if d0 is non-zero, branches to another routine; otherwise it calls a library routine using the same pointer and branches to this routine.
- `update_event_structure_and_dispatch`: Copies a block of data from a structure into a local stack frame, calls a library function, and if a specific long value equals four, performs a series of calls to update counters and dispatch a message via a stack-passed pointer.
- `compare_and_update_counter`: Loads a long value from offset -18 of a5 into d1, compares it to an address, and if not equal, increments a word at offset -38 of a5, compares that word to 2, and if it is greater than 2, clears a word at offset -36 of a5 and branches to another location; otherwise it sets d0 to zero, stores it at offset -36 of a5, and branches.
- `decrement_and_compare_d1`: Decrements d1 in a loop, and if d1 equals 232, it sets d0 to zero and stores it at an offset of a5 before branching.
- `process_conditional_data`: If d1 equals 104, it clears a word at offset -38 of a5, sets a word at offset -34 of a5 to 2, loads four longwords from a5 into d0-d3, calls a routine using a pointer from a4, and then performs further conditional checks and assignments.
- `check_conditional_logic`: It clears a word at an offset from a5, compares a word at another offset from a5 to two, and if not equal, calls a subroutine using a word from a5 and then branches.
- `process_conditional_structure_logic`: It compares a word at offset -34 of a5 with 1, and if equal, performs a series of subroutine calls using values from a5 and a4, including a branch that adds 23 to a value in d0.
- `call_and_branch_2`: Moves a word from offset -10 of a5 to an offset in a4, pushes a word from offset -8 of a5 to the stack, calls another routine, and then branches to another location.
- `call_subroutine_and_branch`: It pushes a word from an offset of a5 onto the stack, calls a subroutine, and then branches to another location.
- `branch_on_structure_value`: Pushes a word from offset -6 of a5 to the stack, calls a subroutine, increments the stack pointer, compares a word from offset 5868 of a4 with a word from offset -4 of a5, and either calls another subroutine or branches.
- `branch_on_value`: Compares a word from a global variable to several constants and branches accordingly; if the value is 1, it pushes two words from a5 and a global variable onto the stack, calls a routine, and then updates a word at offset -2 of a5 before branching.
- `call_and_branch_short`: Calls another routine, clears d0, and branches to another location.
- `call_and_branch_standard`: Pushes a word from offset -4 of a5 to the stack, calls another routine, clears d0, and branches to another location.
- `call_indirect_subroutine`: Pushes a word from offset -4 of a5 onto the stack, calls a subroutine at offset 8628 of the program counter, increments the stack pointer by 2, restores registers from the stack, unlinks a5, and returns.
- `compare_and_branch_9`: Saves registers to the stack, compares a word from a structure to 80 decimal, and either branches to another location or clears d0 and branches elsewhere.
- `mask_and_branch`: Performs a bitwise AND on a word at offset a5 with 192, then branches if the result is zero, otherwise it clears d0 and branches to another location.
- `jump_via_scaled_index`: Compares d1 to 0 and 10, subtracts 1 from d1 if it is between 0 and 10, and then performs a jump to an address calculated by adding the result of d1 multiplied by 2 to a base address.
- `process_conditional_logic`: Tests bits in a structure at a5 to set bits in another structure, then performs arithmetic and comparisons using values from a4 and a5 to determine a value to be used in a subsequent calculation.
- `increment_and_branch_3`: Increments d7 by one and branches to another routine.
- `execute_subroutine_and_restore`: Pushes a constant 1 onto the stack, calls another routine, restores d7 and a2 from the stack, and returns.
- `calculate_quotients_and_store`: It performs several division operations using values from a global variable and a structure in a5, stores the results and several constants into various offsets of the structure in a5, and then branches if a word at offset 8 of a5 is non-zero.
- `update_value_from_table`: Calls a routine via a pointer in a6, and if a word at an offset of a5 is non-zero, performs arithmetic on a value from a5 and uses it as an offset to read a word from a global table.
- `update_and_test_values`: It writes a constant and a byte to a structure pointed to by a global variable, pushes two words from a5 onto the stack and calls a routine, performs arithmetic on values from a5 to update offsets in a5, pushes a value from a5 onto the stack and calls another routine, and branches if d0 is not zero.
- `test_and_branch_3`: Clears the stack, pushes a pointer from offset -126 of a4, calls a subroutine, increments the stack pointer, tests a word at offset 14 of a5, and either branches or sets d0 to 1.
- `set_status_and_call`: Sets bit 0 of a byte at offset 37 of a structure in a0, stores a word in a5, and calls one of two different routines via a pointer in a6 depending on the value at offset 40 of a5.
- `update_structure_from_source`: Copies several words and longwords from a source structure pointed to by a5 to another location, calls a routine via a pointer in a6, and then performs conditional logic involving a longword at offset -50 of a5 and several calls to other routines.
- `process_status_and_flags`: Checks several conditions involving a longword at offset -50 of a5 and words at offsets -36, 8, -42, -24, and -22 of a5; if conditions are met, it calls several subroutines using pointers from a4 and a6, and updates a word at offset -42 of a5.
- `clear_status_word_3`: Clears a word at offset -42 of a5 and branches to another routine.
- `update_status_flags`: Clears a word at offset -42 of a5, sets a word at offset -32 of a5 to zero, and if a longword at offset -50 of a5 is 8, checks if a word at offset -52 of a5 is 69, then sets bit 1 of a byte at offset 15 of a5 before branching.
- `modify_structure_and_branch`: Compares d3 to 104 and, if not equal, sets bit 0 of a byte at offset 14 of a5, stores d1 into offset -34 of a5, and branches to another location.
- `check_and_modify_structure`: Compares d3 to the constant 232, and if not equal, clears bit 0 of a byte at offset 14 of a5, stores d0 into offset -40 of a5, and either stores 17 into offset -32 of a5 or branches to another location based on bit 1 of a byte at offset 14 of a5.
- `process_resource_and_branch`: Compares d2 to 1024 and, if not equal, pushes two words from a5 onto the stack, calls another routine, stores d0 into offset -50 of a5, and performs several comparisons on d0 before clearing offset -64 of a5 and branching to another location.
- `execute_complex_conditional_sequence`: Pushes a pointer from a4 onto the stack, calls a subroutine, compares a word from offset -24624 of a4 against 1 and 2, calls another subroutine if a word at offset -18506 of a4 is non-zero, clears bit 0 of a byte at offset 25 of a structure in a0, and calls a third subroutine if a word at offset -42 of a5 is non-zero.
- `calculate_and_store_quotients`: Checks a word at offset -54 of a5, and if non-zero, pushes two words from a5 onto the stack to call a subroutine, then performs divisions on a value from a4 to store the resulting quotients into offsets -2 and -4 of a5.
- `calculate_signed_difference`: Subtracts a word from d3, then subtracts a word from a value at offset -12 of a5 from a longword loaded from that same offset, and branches based on the result.
- `calculate_difference_2`: Subtracts a word in d1 from a longword in d5, compares the result to d3, and if d3 is greater than or equal to the result, loads d2 into d3 and subtracts d0 from d3, negating d3 if the result is negative.
- `subtract_and_move`: Subtracts d0 from d2, moves the result to d3, and branches to another routine.
- `calculate_absolute_difference_3`: Moves d4 to d0, subtracts d1 from d0, and if the result is positive branches to another routine, otherwise it negates d0 and branches to another routine.
- `execute_conditional_subroutines`: Subtracts d1 from d4, stores the result at offset minus 30 of a5, and if the result is greater than a value at an offset in a4, it calls two different subroutines using values from the stack before branching.
- `execute_conditional_logic_7`: Tests a word at offset minus 42 of a structure in a5; if non-zero, it pushes 2 onto the stack, calls another routine with a pointer from a5, compares a word from offset minus 24898 of a4 with a word from offset minus 2 of a5, and if the first is greater, calls another routine with a word from offset minus 24896 of a4 decremented by 1.
- `execute_conditional_calls_4`: Compares the sum of d1 and a word from a4 with d2, calls a routine with values from a4 and d1 if the comparison passes, and then performs a second conditional call using a different word from a4.
- `compare_and_call_routine_3`: Adds a word from offset 20322 of a4 to d1, decrements it, and if it is not less than d2, increments d1, pushes two words from a4 onto the stack, calls another routine, and then tests a word at offset 8 of a5.
- `execute_library_sequence_3`: Performs a sequence of library calls, calculates several averaged values using arithmetic and bit shifts, and writes them into a structure before performing more library calls.
- `execute_multiple_calls_3`: Clears a word at offset minus 42 of a5, pushes two values onto the stack, and calls another routine.
- `call_subroutine_with_stack_2`: Sets d0 to one, stores it at offset minus 38 of a5, pushes a pointer from offset minus 20 of a5 onto the stack, pushes two words from offset minus 8 and minus 6 of a5 onto the stack, calls another routine, cleans the stack, and branches.
- `execute_logic_block`: Clears the stack, calls a routine, clears a bit in a structure at offset 25 of a0, calls a library routine, calls another routine, performs a multiplication and addition on a value from a5, and stores the result in a structure at offset 10 of a5.
- `process_menu_data`: Compares a word from a global variable with a value at offset 8 of a structure in a5, calls a routine using pointers from a4, and if a condition is met, performs a loop that decrements a value at offset -2 of a5 and calls another routine.
- `dispatch_via_offset`: Saves registers to the stack, loads a word from a structure, and if it is not 50 decimal, performs a bit shift and jumps to an address calculated from that value.
- `branch_to_offset_5`: Performs a short branch to an offset of 7466 in decimal.
- `clear_and_branch_4`: Sets d0 to 0 and branches to another location.
- `call_subroutine_on_match`: Compares a word at offset minus 9616 of a4 with 8, and if they are not equal, pushes a constant 1 onto the stack and calls another routine before returning.
- `process_table_and_call_routine`: Calculates an address based on a word at offset -18500 of a4, performs several arithmetic operations and byte loads from a structure at offset 19108 of a4, and if certain conditions are met, calls a routine using a pointer from offset 19092 of a4 and then calls another routine using four 16-bit values from the stack and a pointer from offset 50 of a structure in a4.
- `push_values_and_call_routine`: Calls a routine using a pointer from a4, then pushes four 16-bit values from a5 onto the stack, pushes a pointer from a structure in a4 onto the stack, calls another routine, and finally increments several values in a5 and d6 before branching.
- `calculate_offset_values`: If a word at a specific offset in a4 is less than two, it loads a byte from offset 54 of a structure in a0, extends it to a word, adds one, and stores it at offset minus 8 of a5; it then repeats a similar calculation using a value derived from d1 and the same byte from a0 to store a second result at offset minus 4 of a5.
- `calculate_and_store_offsets`: Calculates a value based on a byte from a structure in a0 and stores it at offset negative eight and offset negative four of a5, then checks a value from a4 and either performs further calculations involving a byte from a0 and a value from a4 to store a final result at offset negative two of a5 or branches to another routine.
- `perform_multiple_library_calls`: Calculates values based on a byte from a0 and stores them in a structure at a5, then performs five library calls using values from that structure and pointers from a4 and a0.
- `jump_to_address_45`: Performs an indirect jump to the address in 4022.
- `jump_to_indirect_address_7`: Jumps to an address stored in a memory location.
- `jump_to_zero`: Performs a jump to an address at offset zero.
- `jump_to_address_2`: Jumps to the address stored in a0.
- `jump_to_address_25`: Performs a jump to the address stored in a longword at offset 2ea.
- `jump_to_structure_update`: Performs a long jump to another routine.
- `jump_with_constant_128`: Loads the value 128 into register d0 and jumps to a longword address.
- `jump_to_address_39`: Performs a long jump to the address stored at offset 14222 in decimal.
- `jump_with_constant_32`: Loads the value 32 into register d0 and jumps to a longword address.
- `jump_to_absolute_address_6`: Performs an unconditional jump to an absolute long address.
- `jump_to_address_26`: Performs a jump to the address stored in a longword at offset 3e4.
- `jump_to_address_27`: Performs a jump to the address stored in a longword at offset 3fc.
- `jump_with_constant_8`: Sets register d0 to 8 and performs a long jump to an offset of 36.
- `jump_to_address_42`: Performs an indirect jump to the address stored in the longword at address 3784.
- `jump_to_pointer_4`: Performs a long jump to an address stored in a register.
- `jump_to_external_address_2`: Performs a long jump to a specific memory location.
- `jump_to_zero_5`: Performs a long jump to address zero.
- `jump_to_address_37`: Performs a long jump to the address contained in the constant 746.l.
- `jump_indirect_address`: Performs a long jump to the address stored at offset 246 of the program counter.
- `jump_to_resident_library_stub`: Performs an indirect jump to a routine that loads arguments and the executive library base to call a resident function.
- `jump_to_offset_32`: Performs a jump to an address at offset 40 decimal.
- `jump_with_constant_152`: Loads the value 152 into register d0 and jumps to a longword address.
- `jump_to_menu_handler`: Performs an absolute long jump to a specific address in memory.
- `jump_with_constant_0`: Sets register d0 to 0 and performs a long jump to an offset of 36.
- `jump_to_handler_address`: Performs a long jump to a destination address stored in a pointer.
- `jump_with_constant_136`: Loads the value 136 into register d0 and jumps to a longword address.
- `jump_to_preference_table_initialization`: Performs an indirect jump to a location stored in a data pointer to execute initialize_preference_tables.
- `jump_to_external_address_4`: Performs an indirect jump to a longword address stored in memory.
- `jump_with_constant_24`: Sets register d0 to 24 and performs a long jump to an offset of 36.
- `jump_with_constant_80`: Loads the value 80 into register d0 and jumps to a longword address.
- `jump_to_indirect_address_8`: Performs a long jump to an address stored at a specific location in memory.
- `jump_with_constant_184`: Loads the value 184 into register d0 and jumps to a longword address.
- `jump_to_library_stub`: Performs an indirect jump to a routine that handles writing data via the operating system library.
- `jump_to_address_fifty_six`: Performs a long jump to a longword address after loading the word value 56 into register d0.
- `jump_to_address_indirect`: Performs a long jump to the address contained in the constant 242.l.
- `forward_jump_to_offset_init`: This routine is a single unconditional jump to another location in the program, with no other work of its own.
- `jump_to_address_one_hundred_four`: Performs a long jump to a longword address after loading the word value 104 into register d0.
- `jump_to_another_routine_2`: Performs an unconditional jump to another routine.
- `jump_to_address_41`: Performs an absolute long jump to the address 58.
- `jump_to_address_35`: Performs a long jump to the address stored at offset 16684.
- `jump_to_address_forty`: Performs a long jump to a longword address after loading the word value 40 into register d0.
- `jump_to_offset_thirty_six`: Performs a long jump to an offset of 36 after setting register d0 to 16.
- `jump_to_stack_loader`: Performs a long jump to a routine that loads two pointers from the stack into a1 and a0 and a word into d0.
- `set_d0_and_jump`: Sets d0 to 0 and performs a long jump to an offset of 36.
- `set_d0_and_jump_2`: Sets d0 to 8 and performs a long jump to an offset of 36.
- `set_d0_and_jump_3`: Sets d0 to 16 and performs a long jump to an offset of 36.
- `set_d0_and_jump_4`: Sets d0 to 24 and performs a long jump to an offset of 36.
- `load_value_thirty_two`: Loads the word value 32 into register d0 and jumps to a longword address.
- `load_value_forty`: Loads the word value 40 into register d0 and jumps to a longword address.
- `load_value_forty_eight`: Loads the word value 48 into register d0 and jumps to a longword address.
- `load_value_fifty_six`: Loads the word value 56 into register d0 and jumps to a longword address.
- `load_value_sixty_four`: Loads the word value 64 into register d0 and jumps to a longword address.
- `load_value_seventy_two`: Loads the word value 72 into register d0 and jumps to a longword address.
- `load_value_eighty`: Loads the word value 80 into register d0 and jumps to a longword address.
- `load_value_eighty_eight`: Loads the word value 88 into register d0 and jumps to a longword address.
- `load_value_ninety_six`: Loads the word value 96 into register d0 and jumps to a longword address.
- `load_value_one_hundred_four`: Loads the word value 104 into register d0 and jumps to a longword address.
- `load_value_70`: Loads the word value 112 into register d0 and jumps to a longword address.
- `load_value_78`: Loads the word value 120 into register d0 and jumps to a longword address.
- `load_value_80`: Loads the word value 128 into register d0 and jumps to a longword address.
- `load_value_88`: Loads the word value 136 into register d0 and jumps to a longword address.
- `load_value_90`: Loads the word value 144 into register d0 and jumps to a longword address.
- `load_value_98`: Loads the word value 152 into register d0 and jumps to a longword address.
- `load_value_a0`: Loads the word value 160 into register d0 and jumps to a longword address.
- `load_value_a8`: Loads the word value 168 into register d0 and jumps to a longword address.
- `load_value_b0`: Loads the word value 176 into register d0 and jumps to a longword address.
- `load_value_b8`: Loads the word value 184 into register d0 and jumps to a longword address.
- `load_constant_c0`: Loads the word value 192 into d0 and performs a long jump to an address.
- `load_constant_c8`: Loads the word value 200 into d0 and performs a long jump to an address.
- `load_constant_d0`: Loads the word value 208 into d0 and performs a long jump to an address.
- `execute_library_call_3`: Calls a routine via a pointer in a6 using a library call, then performs a conditional branch based on a word in a5 before calling the same library routine again and branching to another location.
- `calculate_and_call_quotients`: Links a5, divides a word from offset 8 of a5 by 100, subtracts a value from offset 20326 of a4, divides a word from offset 8 of a5 by 100, swaps the high and low words, subtracts a value from offset 20320 of a4, pushes the results onto the stack, and calls another routine.
- `clear_and_call_routine`: Calculates an address using a4, clears a byte at that address, pushes registers onto the stack, calls another routine, clears a word at an offset of a4, and returns.
- `modify_structure_based_on_table`: If a word at offset -24790 of a4 is greater than zero and certain bits are set in a structure at offset 19104 of a4, it calls a routine using a pointer from offset -12 of a5 and, if a longword at offset -25816 of a4 is less than a longword at offset -12 of a5, it adds a constant to offset -8 of a5.
- `compare_and_modify_data`: Compares a longword from a structure in a5 with a longword from a table in a4, and if the table value is not greater, it adds a constant to a value at offset -4 of a5, decrements a longword at offset -8 of a5, and branches back to the start.
- `compare_and_decrement_counter`: Compares a longword from a table in a4 with a longword at offset -4 of a5, and if it is not greater, it decrements a value in a register using a word from a table in a4 in a loop, then calls another routine and branches.
- `update_counters_and_call_library`: Pushes two words from a table onto the stack, calls another routine, then calls a library function using a pointer from a4.
- `copy_structure_members`: Copies three long words from offsets minus 12, minus 8, and minus 4 of the structure in a5 to offsets minus 25048, minus 25044, and minus 25040 of the structure in a4, then restores a6 and a7 and unlocks a5 before returning.
- `negate_word_from_table`: Links a stack frame, retrieves a word from offset -24534 of a4 into d1, and if the word is negative, negates it and branches to another location; otherwise, it negates d1 and branches to a different location.
- `recursively_process_divided_values`: Checks if a 16-bit value at offset 8 of a structure is negative; if so, it calls another routine with the value 45, then performs a recursive call by dividing the negated value by 10 and calling itself if the quotient is non-zero, finally calling another routine with a value derived from the quotient divided by 16 and offset by 48.
- `copy_bytes_in_loop_3`: It checks a word at offset 8 of a5, and if non-zero, enters a loop that copies bytes from a source at a0 to a destination at a1 until a zero byte is encountered.
- `execute_complex_data_sequence`: Copies a byte from a table to a structure, calls several routines with various stack arguments, and performs a loop to copy bytes before calling a library function via a pointer in a6.
- `call_multiple_routines`: Pushes three pointers onto the stack and calls a routine, then resets the stack pointer and calls another routine before branching.
- `copy_byte_loop_4`: Copies bytes from a source address to a destination address in a loop, then pushes two values onto the stack and calls another routine before returning.
- `compare_and_update_pointer_2`: Calls a library function via a pointer in a6, then compares a longword at offset -12 of a5 with a longword at offset -25666 of a4, and if the comparison passes, adds 1440 to offset -8 of a5 and decrements offset -12 of a5.
- `query_and_process_structure_data`: Queries a library for two different status values, extracts three bytes from a structure to calculate new values, and calls another routine with these values as arguments on the stack.
- `process_structure_update`: Calls a routine, and if it returns with a non-zero d0, checks offset 8 of a5; if that is zero, it calculates an offset using a value from a4, compares a word at that offset with a value from a4, and if they are not equal, updates a value in a4 and calls another routine.
- `compare_and_call_if_not_equal`: Links a5, saves d2, loads a word from offset minus 30736 of a4 into d0, calls another routine, calls another routine with d1 set to negative one, stores d0 at offset minus two of a5, compares d0 with a word from offset minus 24898 of a4, and if they are not equal, pushes a word from offset minus 24896 of a4 and d0 onto the stack before calling another routine.
- `calculate_and_store_scaled_value_2`: Loads a word from offset -30716 of a4, performs arithmetic involving a left shift by 6 and a subtraction, calls another routine, stores the result in a local stack variable, compares it to a word at offset -24896 of a4, and if they are not equal, pushes two words from a4 onto the stack and calls another routine.
- `update_and_call_library_2`: Calculates several values based on bytes from a structure in a4, pushes them onto the stack, and calls a routine; it then performs multiple library calls using a base in a6 and a pointer in a1, and conditionally calls another routine and updates values in a5 before branching.
- `execute_complex_library_sequence`: Performs several library calls using a base in a6 and a pointer in a1, calculates several values from bytes in a structure in a4, pushes them onto the stack, and calls a routine; it then conditionally performs more library calls and updates values in a5 before branching.
- `process_structure_data_5`: Calls two library functions using pointers from a structure in a4, decrements a word at offset -18 of a5, and if the result is not zero, performs arithmetic on values from a structure to call another routine twice and then calls two more library functions.
- `execute_conditional_logic_6`: Calls two library functions, decrements a word at offset -18 of a5, and if not zero, performs several checks against a structure in a4 before calling another routine twice and two library functions.
- `process_complex_structure`: Performs several library calls, calculates multiple averaged values using bit shifts, and calls several other routines while updating various offsets in a structure.
- `calculate_and_dispatch_quotients`: Divides a word from a structure in a5 by 100 twice to obtain two values, calls three different routines using these values and other data from a5 and a4, and performs several stack operations.
- `process_structure_byte`: Loads a pointer from offset 8 of a structure in a5, checks if the byte at that pointer is zero, and if not, pushes the byte to the stack and calls another routine.
- `calculate_and_push_values`: Calculates a value by adding a word from a table at an offset in a4 to a word from a structure in a5, then adds a byte from another table location to that sum.
- `update_table_entry`: Compares a 16-bit value from a global variable against 98, and if it is less than 98, copies a byte from offset 9 of a structure in a5 to a calculated address in a0, then decrements the global variable.
- `verify_and_branch`: Links a stack frame, copies a word from a5 to a location in a4, and if a word at offset -24624 of a4 is 1, performs several comparisons and calls two subroutines before branching.
- `fetch_structure_bytes`: Calculates several offsets using a word from offset -18526 of a4, fetches bytes from those calculated locations, and stores them into offsets -16, -14, and -12 of a5.
- `execute_callback_sequence_2`: Copies a byte from one location in a4 to another, performs several conditional checks, and calls multiple routines including a library call and a routine via a pointer from a4.
- `execute_callback_sequence`: Pushes several values onto the stack to call a routine three times with different parameters, then if a word at offset -24942 of a4 is not zero, it performs a multiplication and calls another routine, finally entering a loop that reads bytes from a table in a3 and adds them to a value at offset -2 of a5.
- `execute_conditional_callbacks_4`: Pushes several values onto the stack, calls a routine, stores a word in a structure in a5, calls another routine, and if a word at offset -8 of a5 is 2, it calls a third routine before returning.
- `transfer_byte_and_branch_on_result`: Copies a byte from a4 to a5, checks a word at offset 8 of a5 and a byte at offset -24777 of a4, then calls a routine with three parameters from a4 and a5 and branches to another routine if the result is not zero.
- `execute_conditional_calls_6`: Pushes two values from a4 onto the stack and calls a routine, stores the result in offset -4 of a5, and if the result is not zero, repeats the process with different offsets from a4 before branching.
- `copy_bytes_and_execute_library_call`: Copies bytes between locations in a4 and a5, calls several routines using parameters from a4 and a5, calls a library function via a pointer in a6, and stores the result in offset -90 of a5.
- `process_data_and_call_subroutines_2`: Copies a byte from a location in a4 to a location in a5, then performs several calls to other routines using values from a4 and a5 on the stack, eventually storing a word from d0 into offset -86 of a5.
- `call_with_parameters`: Pushes two values onto the stack, calls a routine, and then loads a word from an offset of a structure in a5 into d0 before returning.
- `check_and_branch_logic`: Loads a byte from offset -24638 of a structure in a4 into d0, stores it at offset -6 of a structure in a5, and then branches to another routine based on the value at offset 12 of a structure in a5.
- `store_pc_relative_address`: Loads the address of the next instruction into a0, stores it at offset -12 of a structure in a5, and branches to another routine.
- `branch_with_zero_3`: Clears d0 and performs a relative branch.
- `clear_and_accumulate_data`: Clears a word at offset 19068 of a structure in a4, calls a routine using a pointer from offset -2 of a5, and then loops through a table starting at offset -2 of a5 to add each byte to the value at offset 19068 of a structure in a4.
- `accumulate_offset_data_with_initial_add`: Adds a word from offset -2 of a structure in a5 to a value at offset 19068 of a structure in a4, calls a routine using a pointer from offset -6 of a5, and then loops through a table starting at offset -6 of a5 to add each byte to the value at offset 19068 of a structure in a4.
- `accumulate_offset_data_alt5`: Calls a routine using a pointer from offset -12 of a structure in a5, then loops through a table starting at offset -18296 of a structure in a4 to add each byte to a value at offset 19068 of a structure in a4.
- `accumulate_offset_data_alt4`: Calls a routine using a pointer from offset -12 of a structure in a5, then loops through a table starting at offset -18594 of a structure in a4 to add each byte to a value at offset 19068 of a structure in a4.
- `accumulate_offset_data_alt3`: Calls a routine using a pointer from offset -12 of a structure in a5, then loops through a table starting at offset -24794 of a structure in a4 to add each byte to a value at offset 19068 of a structure in a4.
- `accumulate_offset_data_alt2`: Calls a routine using a pointer from offset -12 of a structure in a5, then loops through a table starting at offset -18580 of a structure in a4 to add each byte to a value at offset 19068 of a structure in a4.
- `accumulate_offset_data_alt`: Calls a routine using a pointer from offset -12 of a structure in a5, then loops through a table starting at offset -24940 of a structure in a4 to add each byte to a value at offset 19068 of a structure in a4.
- `accumulate_offset_data`: Calls a routine using a pointer from offset -12 of a structure in a5, then loops through a table starting at offset -24896 of a structure in a4 to add each byte to a value at offset 19068 of a structure in a4.
- `accumulate_fixed_offset_24898`: Calls another routine with a count of 2, then loops through a table of bytes starting at offset -24898 of a structure in a4 to add each byte to a global 16-bit value at offset 19068.
- `accumulate_fixed_offset_24814`: Calls another routine with a count of 2, then loops through a table of bytes starting at offset -24814 of a structure in a4 to add each byte to a global 16-bit value at offset 19068.
- `accumulate_fixed_offset_20328`: Calls another routine with a count of 600, then loops through a table of bytes starting at offset 20328 of a structure in a4 to add each byte to a global 16-bit value at offset 19068.
- `accumulate_fixed_offset_24812`: Calls another routine with a count of 8, then loops through a table of bytes starting at offset -24812 of a structure in a4 to add each byte to a global 16-bit value at offset 19068.
- `accumulate_fixed_offset_24804`: Calls another routine with a count of 8, then loops through a table of bytes starting at offset -24804 of a structure in a4 to add each byte to a global 16-bit value at offset 19068.
- `accumulate_scaled_offset_20928`: Multiplies a 16-bit value from a global variable plus one by 24, calls another routine, and loops through a table of bytes to add each byte to a global 16-bit value at offset 19068 of a structure in a4.
- `accumulate_table_values_via_call_2`: Calls another routine with a pointer and two arguments, then iterates through a table to add each byte value to a global variable at offset 19068.
- `accumulate_table_values_via_call`: Calls another routine with a pointer and two arguments, then enters a loop that reads a byte from a table at an offset determined by a value in a register and adds it to a global 16-bit variable.
- `accumulate_scaled_table_values`: Calls another routine with a pointer and two arguments, then enters a loop that reads bytes from a table and adds them to a global 16-bit variable.
- `accumulate_scaled_offset_21824`: Doubles a 16-bit value from a global variable, calls another routine, and loops through a table of bytes to add each byte to a global 16-bit value at offset 19068 of a structure in a4.
- `accumulate_table_bytes_scaled_offset`: Doubles a 16-bit value from a global variable, calls another routine, and loops through a table of bytes to add each byte to a global 16-bit value.
- `accumulate_table_bytes_scaled`: Shifts a 16-bit value from a global variable left by two, calls another routine, and loops through a table of bytes to add each byte to a global 16-bit value.
- `accumulate_bytes_from_table_3_2`: Calls another routine and then iterates through a table of bytes to add each byte to a global variable.
- `accumulate_bytes_from_table_3`: Calls another routine with two arguments, then iterates through a table of bytes to add each byte to a global variable.
- `accumulate_byte_from_table_2`: Calls another routine with two arguments, then loops through a table of bytes to add each byte to a global variable.
- `accumulate_table_bytes_two_offset_a`: Calls another routine with a pointer to a global variable and two arguments, then loops through a table of 2 bytes to add each byte to a global 16-bit value.
- `accumulate_table_bytes_eight_offset_a`: Calls another routine with a pointer to a global variable and two arguments, then loops through a table of 8 bytes to add each byte to a global 16-bit value.
- `accumulate_table_bytes_eight`: Calls another routine with a pointer and two arguments, then loops through a table of 8 bytes to add each byte to a global 16-bit value.
- `accumulate_table_bytes_conditional`: Checks if a 16-bit value at an offset of a5 is equal to 7205, otherwise doubles a 16-bit value from a global variable, calls another routine, and loops through a table of bytes to add each byte to a global 16-bit value.
- `accumulate_table_bytes`: Doubles a 16-bit value from a global variable, calls another routine with a pointer and two arguments, then loops through a table of bytes to add each byte to a global 16-bit value.
- `call_and_accumulate_table_bytes`: Calls another routine using a pointer from a stack frame, then iterates through a table of bytes to add each byte value to a 16-bit value at a fixed offset in a global variable.
- `accumulate_bytes_from_table_2`: Calls another routine and then iterates through a table of bytes, adding each byte to a value located at an offset in a global variable.
- `process_variable_eight_elements`: Calls a routine with a pointer and two arguments, then iterates through a table of a variable number of bytes to add each byte to a value at a fixed offset in a global variable.
- `accumulate_byte_from_table`: Calls another routine and then iterates through a table of bytes, adding each byte to a value at a specific offset in a global structure.
- `accumulate_table_bytes_to_offset`: Calls another routine and then iterates through a table of bytes, adding each byte from the table to a value at an offset in a global structure.
- `accumulate_bytes_from_table`: Calls another routine and then iterates through a table of bytes, adding each byte to a value at a fixed offset in a global structure.
- `process_two_elements_2`: Calls a routine with a pointer and two arguments, then iterates through a table of 2 bytes to add each byte to a value at a fixed offset in a global variable.
- `accumulate_eight_bytes_to_offset`: Calls another routine and then iterates through a table of eight bytes, adding each byte to a value at a fixed offset in a structure pointed to by a register.
- `process_eight_elements_2`: Calls a routine with a pointer and two arguments, then iterates through a table of 8 bytes to add each byte to a value at a fixed offset in a global variable.
- `process_sixty_four_elements`: Calls a routine with a pointer and two arguments, then iterates through a table of 64 bytes to add each byte to a value at a fixed offset in a global variable.
- `process_element_table_data`: Calls a routine with two values on the stack, then iterates through a table to read a byte from an offset and add it to a value at a fixed offset within a structure.
- `process_seventy_two_elements`: Pushes 72 and 1 onto the stack, calls a subroutine via a pointer with an offset from a4, and then iterates through a table to add a byte from the table to a value at offset 19068 of a4.
- `call_routine_and_update_data`: Pushes two values onto the stack, calls a routine via a pointer, and then uses the number of items pushed to fetch a byte from a table to add to a value at a fixed offset in a global structure.
- `call_routine_and_update_table`: Pushes two 16-bit values onto the stack, calls a routine via a pointer in a5, and then uses the number of items pushed to index into a table to add a byte value to a global variable.
- `check_and_process_four`: Compares a word at offset -2 of a5 to 11313; if equal, it pushes 4 and 1 onto the stack, calls a subroutine via a pointer with an offset from a4, and then iterates through a table to add a byte from the table to a value at offset 19068 of a4.
- `process_four_elements_2`: Calls another routine with four and one passed as arguments on the stack, then reads a byte from a table at an offset and adds it to a value at a fixed offset from a pointer in a4.
- `process_four_elements`: Pushes 4 and 1 onto the stack, calls a subroutine via a pointer with an offset from a4, and then iterates through a table to add a byte from the table to a value at offset 19068 of a4.
- `call_subroutine_and_update_table`: Pushes two values onto the stack, calls a subroutine via a pointer, and then adds a byte from a table to a value at a fixed offset from a pointer in a4.
- `process_two_elements`: Pushes 2 and 1 onto the stack, calls a subroutine via a pointer with an offset from a4, and then iterates through a table to add a byte from the table to a value at offset 19068 of a4.
- `process_eight_elements`: Pushes 8 and 1 onto the stack, calls a subroutine via a pointer with an offset from a4, and then iterates through a table to add a byte from the table to a value at offset 19068 of a4.
- `process_doubled_value_table`: Calculates a doubled 16-bit value from a global variable, pushes it and a constant onto the stack, calls a subroutine via a pointer with an offset from a4, and then uses a byte from a table to calculate a new address.
- `fetch_table_byte_and_add_to_offset_3`: Calls another routine, then retrieves a byte from a table at an offset derived from the return value and adds it to a value at a fixed offset in a global variable.
- `fetch_table_byte_and_add_to_offset_2`: Calls another routine with a pointer, then retrieves a byte from a table at an offset derived from a value in a global variable and adds that byte to a value at a fixed offset in a global variable.
- `fetch_table_byte_and_add_to_offset`: Pushes two values onto the stack, calls another routine, and then retrieves a byte from a table at an offset relative to a value, adding that byte to a value at a fixed offset from a pointer in a4.
- `fetch_table_entry_and_update_offset_2`: Pushes two values onto the stack, calls another routine, and then retrieves a byte from a table at an offset relative to a value.
- `fetch_table_entry_and_update_offset`: Pushes two values onto the stack, calls a subroutine via a pointer, and then retrieves a byte from a table at an offset relative to a value on the stack.
- `call_subroutine_and_offset_table_lookup`: Pushes two 16-bit values onto the stack, calls a subroutine via a pointer, and then retrieves a byte from a table at an offset relative to a pointer in a4 to calculate a new address.
- `call_subroutine_and_offset_table_value_2`: Pushes two 16-bit values onto the stack, calls a subroutine via a pointer, and then calculates a new address by adding a byte from a table to a base address.
- `process_data_table_via_pointer`: Pushes two 16-bit values onto the stack, calls a subroutine via a pointer with an offset from a4, and then uses a byte from a table to calculate a new address.
- `call_subroutine_and_offset_table_value`: Pushes three values onto the stack, calls a subroutine via a pointer, and then calculates a new address by adding a byte from a table to a base address and branches to another location.
- `iterate_table_entries`: Pushes several values onto the stack, calls a subroutine using a pointer from a5 offset minus 12, and then uses a byte from a table to calculate and add a value to a global variable in a loop.
- `update_table_value_2`: Calculates a value based on a word from a4 offset minus 24794, pushes several values onto the stack, calls a subroutine using a pointer from a5 offset minus 12, and then uses a byte from a table to calculate and add a value to a global variable.
- `process_table_offset_and_update_value`: Calls another routine with parameters pushed to the stack, then calculates a new address by reading a byte from a table at an offset derived from a doubled 16-bit value and adds it to a global variable.
- `call_process_table_data_and_update_offset`: Calls process_table_data with parameters from the stack and then calculates a new address by adding a byte read from a table to a value at a fixed offset.
- `process_table_data`: Pushes a long from offset 8 of a5, the word 240, and the word 1 onto the stack, pushes the address of offset 24486 of a4 onto the stack, calls a subroutine using a pointer from offset -12 of a5, adjusts the stack, and then performs a sequence of operations involving a word in d7, a byte from a table at an offset calculated from a4, and adds the result to offset 19068 of a4 before branching.
- `update_table_from_stack`: Pushes several values onto the stack, calls a subroutine using a pointer from a5, and then calculates a value by adding a byte from a table to a global variable based on a modified word from a4.
- `update_global_value`: Pushes several values onto the stack, calls a subroutine using a pointer from a5, and then calculates a new value by adding a byte from a table to a global variable based on a modified word from a4.
- `update_table_entry_5`: It calculates a value from a word at an offset from a4, calls a routine using a pointer from a5 and arguments from the stack, and then uses the result to access a byte from a table and add it to a value at an offset from a4.
- `update_table_via_pointer_2`: It calls a routine via a pointer in a0 using several arguments from the stack, then uses the result to calculate an offset and adds a byte from a table to a value at offset 19068 of a4.
- `update_table_from_pointer_call`: Calls a routine via a pointer in a5 with several arguments, then uses the result to calculate an offset and add a byte from a table to a value at offset 19068 of a4.
- `update_table_via_pointer`: Pushes several values onto the stack, calls a routine via a pointer in a0, and if the return value is non-zero, updates a value at offset 19068 of a4.
- `call_and_update_table`: Pushes several values onto the stack, calls a routine via a pointer in a0, and if the return value is non-zero, calculates an offset and adds a byte from a table to a global variable.
- `call_and_update_index_2`: Pushes several values onto the stack, calls a routine via a pointer in a0, and if the return value is non-zero, updates a value at an offset in a4 using a byte from a calculated address.
- `call_and_update_index`: Pushes a longword, a word, a word, and a longword onto the stack, calls a routine via a pointer in a0, and if the return value is non-zero, calculates a new address by adding a longword to a longword and adds a byte from that address to a global variable.
- `call_and_modify_global_value`: Pushes several values to the stack, calls a routine via a pointer at offset -12 of a5, and if the result is non-zero, calculates a new address and adds a byte from that address to a global variable at offset 19068 of a4.
- `call_and_modify_offset`: Pushes several values onto the stack, calls a subroutine via a pointer in a5, and if the return value is non-zero, modifies a word at an offset in a4 based on a byte from a table.
- `call_and_modify_global`: Pushes several values onto the stack, calls a subroutine via a pointer in a0, and if the return value is non-zero, modifies a word in a4 using a byte from a calculated address.
- `call_and_increment_offset`: Pushes several values onto the stack, calls a routine via a pointer in a5, and if the return value is non-zero, calculates a new address and adds a byte from that address to a value at offset 19068 of a4.
- `call_function_and_update_value`: Pushes several values to the stack, including a longword from a5 and a word from a4, calls a routine via a pointer in a5, and then uses the result to calculate an offset and add a byte to a value at a specific offset in a4.
- `execute_and_update_table`: Pushes a longword from offset 8 of a5, a word from offset minus 18594 of a4, the word 1, and a longword from offset 23196 of a4 onto the stack, then calls a subroutine using a pointer from offset minus 12 of a5.
- `execute_conditional_logic_8`: Compares a word from offset -2 of a5 against three constants, and if none match, pushes a pointer from a5, two words, and a longword from a4 onto the stack to call a subroutine, then performs arithmetic and stores a byte into a location calculated from a4.
- `compare_and_call_structure`: Checks a word at offset 12 of a5, and if zero, pushes several values onto the stack, calls another routine, and compares a word from a4 with a value from the stack.
- `call_subroutine_with_stack_args`: Pushes a longword from offset 8 of a5 and two words onto the stack, calls a routine with a pointer from a4, and then performs arithmetic on d1 before branching.
- `execute_conditional_callback`: Checks a word at offset 12 of a5, and if zero, pushes a pointer from offset 8 of a5 and two constant words onto the stack to call a subroutine, then compares a word from offset -4 of a5 against a value in a4 and performs a jump based on the result.
- `calculate_and_call_routine_2`: Increments a word from a table in a4, pushes it and a longword from a5 onto the stack to call a routine, then pushes a longword from a5 and several other values to call a library routine before returning.
- `process_structure_pointer`: Stores a routine address into a structure in a5, pushes values onto the stack, calls that routine, and then calculates an offset into a table in a4 based on the result.
- `process_structure_data_3`: Compares a longword at an offset relative to a5 to 1, calls a routine using a pointer from a structure in a5, and if certain conditions are met, calculates an address using a sign-extended byte from a structure in a5 and a value in d7, then adds a byte from that address to a value at an offset relative to a4.
- `calculate_offset_address_4`: Calculates a new address by adding a sign-extended byte from offset -8 of a structure in a5 to a value in d1, which was copied from a3, and moves the result to a3.
- `copy_byte_and_add_offset`: Calls a routine using a pointer from a structure in a5, then uses a value from a structure in a5 to calculate an address and adds a byte from that address to a value at offset 19068 of a4.
- `copy_byte_loop_3`: Copies a byte from offset -7 of a structure in a5 to the address in a3, increments a3, and loops if the byte loaded from offset -8 of the structure was not zero.
- `restore_registers_a3`: Restores d2, d7, and a3 from the stack and returns.
- `search_with_offset_calculation`: Calculates a longword value by multiplying a word from a structure in a5 and adding it to a longword from a table in a4, then enters a loop that compares bytes at an offset to check for equality and increments a register until a condition is met.
- `process_negative_offset_and_search`: If d7 is greater than 1, it negates d7, stores the result at offset -13 of a5, calls a routine using a pointer from offset -18 of a5, and then enters a loop that calculates an address from a pointer in a5 and a byte at that address to add to a value in a4.
- `calculate_offset_and_add_byte`: Calls a routine using a pointer from offset 18 of a structure in a5, then uses a value from a3 to calculate an address and adds a byte from that address to a value at offset 19068 of a4.
- `calculate_pointer_offset`: Calculates a new address by adding a sign-extended value from d7 to a value in d1, which was copied from a3, and moves the result to a3.
- `search_table_for_match`: Increments a counter in a loop, calculating offsets from a pointer in a3 to compare bytes at two different locations, and calls another routine if a match is not found or if certain conditions are met.
- `process_offset_data`: Stores a sign-extended byte from d0 into a structure in a5, calls a routine using a pointer from a structure in a5, and if a counter in d6 is not zero, calculates an address based on a structure in a5 and a sign-extended d0, then adds a byte from that address to a value at an offset relative to a4.
- `calculate_address_and_add_byte`: Calls a routine using a pointer from offset 18 of a structure in a5, then uses a sign-extended value from d7 and a pointer in a3 to calculate an address and adds a byte from that address to a value at offset 19068 of a4.
- `calculate_offset_address_5`: Adds a sign-extended 32-bit value from register d7 to a pointer in register a3, stores the resulting address in register a3, and branches to another routine.
- `call_and_update_global`: Clears a byte at offset -13 of a structure in a5, calls a routine at an address stored at offset -18 of the structure, and then adds a byte from a calculated address to a global variable at offset 19068 of a4.
- `restore_registers_a3_extended`: Restores d2 through d7 and a3 from the stack and returns.
- `test_and_store_pointer`: Checks a word at offset 8 of the structure in a6, branches if it is zero, otherwise calls another routine, stores the resulting d0 into offset -4 of the structure in a5, and branches to another location.
- `store_and_test_d0`: Clears the stack, calls a subroutine, stores the resulting d0 into an offset of a5, and if d0 is non-zero, branches to another location; otherwise it sets d0 to zero and branches elsewhere.
- `dispatch_multiple_routines`: Pushes two values onto the stack and calls a routine, then pushes a value from a5 and calls another routine, and if d0 is not zero, calls a third routine, then performs two more calls with different stack parameters.
- `process_and_dispatch_values`: Pushes a long value from a structure in a5 onto the stack, calls another routine, and then calls a second routine.
- `fill_table_with_constant`: Loops while d7 is less than or equal to 59, calculating an address based on a table at offset 23176 of a4 and writing the value 3 to that address and the byte at offset 99 of that address.
- `fill_table_values`: Loops while d7 is greater than or equal to 99, writing the value 3 to offset 170c and offset 0 of a table entry located at an offset from a global variable in a4.
- `restore_registers_d7`: Restores d7 and a3 from the stack and returns.
- `allocate_and_manage_memory_structures`: Allocates memory using a library function, copies bytes from a table to another location, and calls several routines including a library function via a6 to manage structure members.
- `allocate_and_copy_data`: Calls a routine using a pointer from a structure in a5, then performs several loops involving calls to other routines and library calls to manipulate values and copy bytes between structures.
- `call_and_increment_counter`: Calls a routine using a pointer from a global variable in a6, clears a value at offset -16 of a structure in a5, and increments a value at offset -4 of the same structure before branching.
- `divide_and_update_list`: Calls a routine, performs a division using a value from a5, and if a comparison fails, enters a loop that updates a pointer at offset -18 of a5.
- `manage_memory_and_structures`: Copies a byte in a loop, then performs several conditional checks involving table lookups and calls a routine multiple times with different stack arguments, and finally calls a routine via a pointer in a6.
- `free_and_reallocate_memory`: If a value at offset -22 of a structure in a5 is non-zero, it calls a routine to free that memory; then, if a value at offset -26 of a5 is non-zero, it calls another routine with a length of 260 using the pointer at that offset, and finally moves the value at offset -10 of a5 into d0.
- `copy_and_test_byte`: It copies a byte from a location relative to a4 to a location relative to a5 in a loop, then checks a word at an offset from a5 and a byte at an offset from a4 to decide whether to clear a byte at an offset from a5.
- `execute_conditional_copy_and_calls`: It copies a byte from a source at a0 to a destination at a2 in a loop, then calls two different subroutines if certain conditions are met based on the results and values in a5.
- `execute_conditional_logic_calls`: Calls a routine with a pointer from a4, and if the result is not zero, calls another routine with a pointer from a5, then branches.
- `copy_and_execute_logic`: Copies bytes in two loops, then calls a routine via a pointer in a6, and subsequently calls other routines using values from a5 and a4.
- `copy_and_test_result`: Calls a routine via a pointer in a6, stores the result in a structure at an offset of a5, and if the result is non-zero, calls another routine before returning.
- `copy_byte_loop_2`: Copies bytes from a source address derived from a5 to a destination address derived from a4 in a loop, then moves a value from offset -164 of a5 into d0.
- `execute_multiple_calls_5`: Saves registers, pushes several values onto the stack, and calls a routine three times using different parameters, then decrements a word at offset -10 of a5 and branches if it reaches zero.
- `evaluate_byte_and_compare`: Checks a byte at offset -7 of a5, performs bitwise operations to derive d5 and d6, and then compares d7 against d5 to determine a branch.
- `write_table_entries`: Loops while d5 is not negative, writing a byte from d6 to a table entry at an offset from a global variable in a4, and decrements d5 and d7 in each iteration.
- `branch_on_d7`: Compares d7 to zero and branches if it is less than or equal to zero; otherwise it clears d0 and branches to another location.
- `update_counter_and_table`: Clears a word at offset -24794 of a4, and if that word is less than 5976, it uses it to index a table at offset 23176 of a4 to fetch a byte, then uses that byte to index another table at offset -18388 of a4 to fetch another byte; if the second byte is 255, it doubles the value at offset -24794 of a4, increments it, and stores the result at a new location calculated from the doubled value and offset 21224 of a4.
- `dispatch_and_unlk`: Calculates a value from a word at offset -24794 of a4, calls a subroutine with several parameters, and if the result does not match a doubled version of the original value, calls another subroutine before unlinking a5 and returning.
- `update_table_via_loop`: Loops while d7 is not zero, calculating an address based on a table at offset 23176 of a4 and copying a byte from one location to another.
- `update_table_with_difference`: Loops while d7 is not zero, calculating a value by subtracting a word at an address in a0 from 6000 and storing the result at an address in a1, then decrements d7.
- `restore_registers_d7_short`: Restores d5, d6, and d7 from the stack and returns.
- `scale_and_copy_byte`: Loops while d7 is less than 5936, calculating an address based on a table at offset 23176 of a4 and copying a byte from one location to another using values derived from d7 and d5.
- `increment_and_branch_6`: Adds 100 to register d7 and branches to another location.
- `calculate_scaled_value_2`: Decrements a 16-bit value from a global variable, performs several divisions and multiplications using 100 as a divisor, and stores a calculated result into a table at an offset determined by the original value.
- `restore_registers_a2`: Restores d2 through d7 and a2 from the stack and returns.
- `calculate_bounds_and_store`: Loads several bytes from a structure pointed to by a4, performs arithmetic to calculate new values, stores these values into a structure pointed to by a5, and then performs conditional branches based on the results.
- `initialize_structure_fields_3`: Sets up a stack frame, saves register a6, writes two 16-bit values to specific offsets in a structure pointed to by a5, and branches to another location.
- `copy_block_and_call_library`: Copies 11 longwords from a global variable to a location at offset -74 of a5 in a loop, sets several fields in a structure in a5 using a calculated address and a constant, and then calls a routine using a pointer from a global variable.
- `dispatch_library_calls_and_write_byte`: Calls a subroutine, stores the result in a5, and if it is not zero, calls two library functions, then checks a word in a4 and if it is not zero, it calculates an address and writes a byte to it.
- `loop_and_store_values`: If a word in d7 is less than 5968, it enters a loop that reads bytes from two different offsets in a4, and if a byte is not 255, it increments a counter in a4, writes d7 to a location in a4, and repeats the loop.
- `process_complex_loop`: Performs several comparisons and arithmetic operations, calls a routine, and enters a loop that performs multiple calculations, calls two different routines, and increments values in a table in a4 based on the results.
- `increment_and_jump`: Adds 1 to register d7 and branches to another location.
- `call_and_restore_registers_2`: Checks if a longword at offset -10 of a5 is zero, otherwise it pushes a pointer from a table in a4 onto the stack, calls another routine, and restores registers before returning.
- `jump_to_address_15`: Performs a long jump to the address stored in the constant 4224.
- `jump_to_zero_4`: Performs an indirect jump to address zero.
- `jump_to_long_address`: Performs a long jump to a target address stored at a specific location.
- `jump_to_pointer_7`: Jumps to a location stored in a longword in a register.
- `jump_to_status_check_3`: Jumps to check_status_values after being invoked via a pointer in data.
- `jump_to_indirect_address_24`: Performs a long jump to the address stored at a specific location.
- `jump_to_distant_address`: Performs a long jump to a different location in memory.
- `jump_to_address_16`: Performs a long jump to the address stored in the constant 7888.
- `jump_to_pointer_3`: Performs a long jump to the address stored in register a0.
- `jump_to_external_routine_4`: Calls another routine via a long jump.
- `jump_to_address_17`: Performs a long jump to the address stored in the constant 3072.
- `jump_to_address_33`: Performs a long jump to the address stored in the constant 2016.
- `jump_to_table_entry`: Jumps to a location stored in a longword at offset 41e.
- `jump_to_bounds_check`: Jumps to a location stored in a longword at offset 3be.
- `jump_to_library_function`: Performs a jump to an address stored at an offset of minus 306 from a library base loaded from a global variable.
- `jump_to_constant`: Performs a long jump to the constant address 896.
- `jump_to_address_18`: Performs a long jump to the address stored in the constant 7964.
- `jump_to_callback`: Performs an unconditional jump to the address stored in a long word in a register.
- `jump_to_indirect_address_15`: Performs an indirect jump to an address stored in a longword at a specific location.
- `jump_to_address_34`: Performs an unconditional long jump to another routine.
- `jump_to_fixed_address`: Performs an indirect jump to a fixed address.
- `jump_to_address_31`: Performs an indirect jump to a longword address.
- `jump_to_external_routine_5`: Performs an indirect jump to a location stored in a long word.
- `jump_to_write_function`: Performs a jump to an address stored at offset zero.
- `jump_to_fixed_address_2`: Performs a long jump to a fixed address after calling a routine that loads the decimal value 56 into register d0.
- `jump_to_address_19`: Jumps to an address stored in a longword.
- `jump_to_structure_checker`: Calls a routine that saves registers to the stack, loads a pointer from offset 8 of a structure in a5 into a2, and compares a word at offset 12 of that structure with a word at offset 14.
- `jump_to_negate_conditional_longwords_2`: Calls a routine that negates a longword in d0 if it is negative, negates a longword in d1 if it is negative, calls another routine, and then negates d1 again.
- `jump_to_address_36`: Performs a long jump to a fixed address.
- `jump_to_indirect_address_5`: Jumps to an address stored in a pointer located in data memory.
- `jump_to_pointer`: Jumps to the address stored in a longword at a specific location in data.
- `jump_to_fixed_address_3`: Performs a long jump to a fixed address after loading the value 216 into a data register.
- `jump_to_hardware_register_6`: Performs a jump to the address stored in the hardware register for the custom chip.
- `test_byte_and_branch`: Tests a byte at a negative offset from a4 and branches to another routine if the byte is zero; otherwise it returns.
- `update_counters_and_library`: Decrements d0 and enters a loop to compare values from a structure in a0 against constants, then if certain conditions are met, calls a subroutine and a library function using values derived from a structure in a4.
- `update_global_counters_2`: Performs several conditional checks on values in d4, d5, d6, and d7, using division and subtraction to calculate new values which are then stored in various global word locations.
- `process_menu_selection`: Saves registers, performs comparisons on values from the stack and a4, calls several routines including a library call via a6, and branches to another routine based on a test of a value in a4.
- `check_bits_and_call_library`: Saves registers to the stack, loads a pointer from a4, sets a2 to point to an offset of 24 from that pointer, checks if bit 16 is set in the longword at a2, checks if bit 19 is set in the longword at a2, and if neither bit is set, calls a library routine using a pointer from a6 before branching to another routine.
- `store_and_call_subroutine`: Stores two words from the stack into global memory locations, pushes a5 onto the stack, and calls another routine.
- `evaluate_table_conditions`: Saves registers, performs several checks against constants, and if a byte from a5 is 3, executes a series of calculations involving multiplication and table lookups to set bits in d7.
- `load_jump_constant_d8`: Loads the word value 216 into d0 and performs a long jump to a fixed address.
- `load_jump_constant_38`: Loads the word value 56 into d0 and performs a long jump to a fixed address.
- `load_jump_constant_b0`: Loads the word value 176 into d0 and performs a long jump to a fixed address.
- `modify_byte_with_offset`: Loads a byte from a pointer in a5, uses it as an offset to read another byte from a location relative to a4, and then modifies a value in d0 based on a bit test and subtraction before branching.
- `increment_and_test_byte`: Loads a pointer from an offset of 8 in a5, increments the byte at that location, and loops while the byte is non-zero before unlocking a5 and returning.
- `calculate_scaled_values`: Divides a 16-bit value from a structure by 100, subtracts a value from a global variable, and stores the result after scaling and adding constants into three consecutive words at a destination pointer.
- `update_scaled_offsets`: Subtracts a global variable from a 16-bit value, scales the result, and stores it at a destination pointer, then performs a similar calculation with a different value and stores it at an offset of 2 from that same pointer.
- `calculate_division_result_2`: Loads a word from offset 8 of a5 into d0, subtracts 23 from d0, divides the extended longword by 16, adds a word from a global variable to the result, and either branches to another routine or sets d1 to 1.
- `calculate_division_sequence`: Copies d0 to d1, subtracts 23 from d1, extends d1 to a longword, divides it by d2, adds d3 to the result, compares d1 to 98, and if not greater, repeats the arithmetic with d1 compared to 1, otherwise it sets d1 to 1 and branches.
- `perform_division_arithmetic`: Subtracts a constant from d0, extends d0 to a longword, divides it by d2, adds d3 to the result, stores the result in d1, and branches to another location.
- `calculate_division_result`: Loads a word from an offset of a5 into d1, performs a series of arithmetic operations involving a long extension of d2 and a division by d3, adds a word from a negative offset of a4 to d2, and stores d1 at a negative offset of a5 before branching.
- `calculate_scaled_value_3`: Subtracts 20 from d2, extends it to a longword, divides it by d3, adds d4, and then performs a second similar calculation with different comparison branches.
- `divide_and_branch`: Subtracts sixteen from d0, extends d0 to a longword, divides it by d3, adds d4 to the result, stores the result in d2, and branches to another routine.
- `multiply_and_add_constant`: Multiplies a 16-bit value by 100, adds the value from d1 to the result, and returns it in d0.
- `dispatch_stacked_subroutines`: If a word at an offset from a4 is not zero, it calls two subroutines using offsets from the program counter, pushes a word from an offset of a5 onto the stack, and calls a third subroutine before returning.
- `update_flag_and_increment_pointer`: Calls another routine, then checks a byte at the pointer in a0; if non-zero, it increments the pointer in a5 and updates a 16-bit value at an offset of -2 from a5 with the bitwise NOT of the byte.
- `toggle_boolean_flag`: Checks a 16-bit value at an offset of -2 from a5; if it is non-zero, it sets it to 1, otherwise it returns.
- `calculate_scaled_value_and_compare`: Calls another routine, performs arithmetic involving division and multiplication to calculate an address, and compares bytes at two memory locations before potentially clearing a 16-bit value at an offset of -10 from a5.
- `add_and_branch`: Adds sixteen to d5 and branches to another routine.
- `calculate_offset_address_6`: Tests a word at a negative offset from a5, and if non-zero, calculates an address using d4, d0, d1, and d7 to load a longword into d0.
- `calculate_and_store_scaled_value`: Divides an extended version of d7 plus one by thirty-two, stores the quotient in d7, and if it does not equal d6, updates a global variable in a4 by dividing it by sixty-nine, stores the quotient back to that variable, and loads a value from a table into d0 using an offset derived from d1.
- `test_and_branch_zero`: Tests a word at a negative offset from a4 and branches if it is zero; otherwise it sets d0 to 0 and branches to another routine.
- `compare_multiple_bounds`: Compares several values derived from a4 with values from a5 and branches to another routine if any of the comparisons fail, otherwise it sets d0 to zero and branches.
- `increment_and_compare`: Increments a word at a negative offset from a5, compares the new value with 8, and if it is 8 or less, performs arithmetic using a table at a negative offset from a4 and values from a5 to compare against a value at offset 10 of a5.
- `increment_structure_word`: Increments a 16-bit word at a negative offset from a5 and branches to another routine.
- `set_d0_to_zero`: Sets d0 to 0 and returns.
- `divide_and_store_byte`: Divides a 16-bit word from a5 by 10, adds 48 to the result if it is non-negative, and stores the byte at a location determined by a pointer in a4 and a value from a5.
- `write_byte_to_global`: Writes the byte 32 to a global variable at an offset from a4, then calculates a value by dividing a 16-bit value from a structure by d1, adds 48, and stores the byte result at an offset of 1 from the global variable.
- `calculate_division_and_compare`: Sets up a stack frame, saves registers, loads a word from an offset of a5 into d0, extends it to a longword, divides it by a constant, adds a word from an offset of a5 to the result, stores the word result at a negative offset of a5, and compares the result to one to decide whether to set d2 to one.
- `jump_to_address_3`: Performs a long jump to the address stored in a constant location.
- `jump_to_subroutine_3`: It performs a long jump to an address at offset 80.
- `jump_to_handler_2`: Performs an absolute long jump to another routine.
- `jump_to_address_twenty_four`: Performs a long jump to a longword address after loading the word value 24 into register d0.
- `load_constant_18`: Loads the word value 24 into d0 and performs a long jump to an address.
- `compare_memory_with_offset`: Loads a word from an offset of a4 into d7, decrements d7, and if d7 is not zero, compares a word at an offset of a4 plus double the value of d7 with a word at an offset of 8 in a5.
- `increment_and_compare_d7`: Increments d7 by 14 if it is less than 9724, otherwise it returns; it also checks a byte at an offset from a pointer in a4 and branches based on the result.
- `update_offset_value`: Adds 20 to d7, compares the result with a word at an offset of -18594 from a4, and if the result is not higher, stores the result back into that same memory location before restoring registers and returning.
- `compare_and_branch_d1_d2`: Compares a word from a table at an offset from a4 with a word at an offset from a5, and branches based on the comparison and the sign of a word at an offset from a4.
- `process_complex_data_logic`: Performs extensive data manipulation involving multiple table lookups, arithmetic on values from a5, and conditional calls to subroutines based on comparisons of bytes and words retrieved from various offsets in a4 and a5.
- `validate_and_clear_byte`: Checks several values at offsets from a4 and a5 against values in a5, and if all conditions are met, clears a byte at an offset from a pointer in a4.
- `restore_registers_and_a6`: Restores d2 through d7 and a6 from the stack, unlocks a5, and returns.
- `process_structured_data_2`: Calls several routines to perform divisions and comparisons on values from a structure, and conditionally writes a byte to a memory location based on multiple checks.
- `write_to_calculated_address`: Multiplies d6 by 200, adds it to an address calculated from a4, adds a value from offset -6 of a5 to that address, and writes 1 to the resulting location before branching to another location.
- `check_and_call_on_byte`: Checks a byte at an address calculated from a4 and d5, and if that byte is 255, it pushes d6 and d7 to the stack, calls another routine, and branches to another location; otherwise it returns.
- `check_byte_and_call`: Extends d5 to a longword, calculates an address using a4 and d5, compares a byte at that address with 2, and if it is not less than 2, performs similar address calculations to fetch another byte and calls another routine with d0, d6, and d7 pushed onto the stack.
- `compare_and_store_word_2`: Calculates a base address by multiplying a word from offset 10 of the structure in a5 by 40 and adding it to a global address, then compares a word from offset 8 of the structure in a5 with a word at an offset from that base address.
- `store_word_conditionally`: Sets d5 to 20 and enters a loop that checks a word at an offset from a3; if the word is not non-zero, it stores d7 at that location.
- `loop_store_word_at_offset`: Enters a loop that stores a word at an offset of 2 from a memory location calculated using a value in d5 and the pointer in a3.
- `store_d7_to_pointer`: Stores the word in d7 at the address pointed to by a3, restores d2, d5, d6, d7, and a3 from the stack, unlocks a5, and returns.
- `calculate_table_offset`: Calculates a base address using values from offsets 10 and 12 of the structure in a5 and a global address, then compares a word from offset 8 of the structure in a5 with a word at an offset from that base address.
- `store_word_loop`: Sets d4 to 5 and enters a loop that checks a word at an offset from a3; if the word is not non-zero, it stores d7 at that location.
- `loop_store_word_at_offset_alt`: Enters a loop that stores a word at an offset of 2 from a memory location calculated using a value in d4 and the pointer in a3.
- `store_d7_to_pointer_alt`: Stores the word in d7 at the address pointed to by a3, restores d2, d4, d5, d6, and d7 from the stack, unlocks a5, and returns.
- `calculate_quotients_and_difference`: Performs multiple divisions using values from offsets 8 and 10 of a5 and the constant 100, stores the results in d7, d6, d5, and d4, and then calculates the difference between d7 and d5, negating the result if it is negative.
- `subtract_and_negate_values`: Subtracts a word in d5 from a long value in d0, then subtracts a word in d4 from a long value in d1, and if the result in d1 is negative, negates d1.
- `calculate_differences`: Subtracts a word in d4 from a long value in d1, and if the result is not less than or equal to zero, subtracts a word in d5 from a long value in d7 and negates the result if it is negative.
- `subtract_and_branch_d5_d7`: Subtracts a word in d5 from a long value in d7 and branches to another location.
- `subtract_and_negate_d4_d6`: Subtracts a word in d4 from a long value in d6, and if the result is negative, negates it before branching.
- `subtract_d4_from_d6`: Subtracts the word in d4 from d6, stores the result in d0, restores d2, d4, d5, d6, and d7 from the stack, unlocks a5, and returns.
- `compare_and_clear_byte`: Saves registers to the stack, compares two words from a structure in a5, and if they are equal, writes 65535 to a pointer in a3 and clears a byte at a calculated address derived from a4.
- `perform_division_and_call`: Performs multiple divisions using values from a5 and constants, stores the results in a5, calculates a value based on comparisons, and calls another routine after pushing two values onto the stack.
- `calculate_difference`: Tests a word at a specific offset, then calculates the difference between two words from a structure and negates the result if it is negative.
- `subtract_and_negate`: Subtracts d0 from d1, moves d1 to d2, loads two words from offsets -6 and -8 of a5, subtracts d0 from d3, stores d2 at offset -10 of a5, and branches based on whether d3 is positive, otherwise it negates d3 and branches.
- `update_and_loop_counter`: Subtracts d0 from d1, stores the result in d3 and at offset -12 of a5, compares d3 with d2, and if d3 is not greater than d2, updates d2 by subtracting 1 and branches back to an earlier instruction if d2 is not zero.
- `perform_division_and_update`: Tests a word at a negative offset of a5, calls two routines, performs a division and swap on d0, and then updates a word at a3 based on the result of a second routine call.
- `modify_and_call_subroutine_2`: Tests a word at a negative offset from a4 and branches if non-zero; otherwise it subtracts d5 from a word at a3, masks the result, calls another routine with the result and a word from offset 24918 of a4, and writes the result back to a3 if d0 is non-zero.
- `test_and_set_word`: Tests a word at an offset from a4; if it is non-zero, it branches, otherwise it writes a word of all ones to the address in a3 and branches.
- `calculate_table_index_2`: Compares two words from a structure in a5, and if the first is not less than or equal to the second, calculates a value by doubling d7, adds it to an offset relative to a4, and moves a word from that location into d4.
- `calculate_and_store_difference`: Calculates a value by shifting and subtracting d6 from itself, then uses this value to access a word from a table at an offset relative to a4, calls another routine with that word and two values on the stack, and if the calculated value is non-zero stores the retrieved word into a location pointed to by a3.
- `calculate_absolute_difference_2`: Tests a word at an offset of negative 24540 from a4, and if non-zero branches elsewhere; otherwise it calculates the difference between two words from a5, negates the result if it is negative, and branches.
- `modify_and_call_subroutine_3`: Subtracts a word from a structure in a3 from the sum of d4 and d4, masks the result with 7, stores it back in a3, pushes several values onto the stack, calls another routine, and then sets d0 to 1 or 0 based on a test of d0.
- `check_and_set_word`: Tests a word at an offset of 24886 from a4; if non-zero, it branches to another routine, otherwise it sets the word at a3 to 65535 and branches to another routine.
- `clear_byte_at_calculated_address`: Calculates an address by adding a word from an offset of 24886 from a4 to a pointer from an offset of 24886 from a4, clears the byte at that address, restores d2 through d7 and a3 from the stack, unlocks a5, and returns.
- `validate_division_results`: Calculates two quotients by dividing a word from a structure at offset 8 of a5 by 100, then checks if the first quotient is not 59 and not 0, and if the second quotient is not 99 and not 0; if all conditions are met, it sets d0 to 0 and branches to another routine, otherwise it branches to another routine.
- `set_d0_and_restore_registers`: Sets d0 to 1, restores d5, d6, and d7 from the stack, unlocks a5, and returns.
- `update_table_values_2`: Loads a word from offset 8 of a5, enters a loop that calculates an offset using d1 and a4, reads a byte from a location determined by adding d7 and a value from that offset to a base address, and calls a routine if the byte is non-zero.
- `move_d6_to_d0_and_restore`: Moves the long value from d6 into d0, restores d5, d6, and d7 from the stack, unlocks a5, and returns.
- `increment_and_update_structure`: Sets a word at offset 14 and offset 10 of a5 to zero, enters a loop that calculates an offset using d1 and a4, calls a routine if a value added to d7 is non-zero, and increments a word at offset 10 of a5 if a byte at a calculated address is zero.
- `increment_pointer_value`: Loads a pointer from an offset of 14 from a5 into a0, increments the word at that address, and branches to another routine.
- `restore_registers_and_unlock`: Restores d5 through d7 and a7 from the stack, unlocks a5, and returns.
- `check_status_values`: Loads three 16-bit words from a structure into d7, d6, and d5, then checks if d6 is zero and if d7 is 65534 and d5 is zero; if all conditions are met, it sets d0 to 2 and branches.
- `check_memory_and_branch`: Links a5, saves d2, d3, and a2 to the stack, loads a word from an offset of a5 into d0 and d1, calculates an address using a pointer in a4 and d1, and if the word at that address is non-zero, branches to another location, otherwise it loads 999 into d0 and branches.
- `execute_conditional_jsr`: Checks a word at offset 12 of a structure, and if non-zero, performs two sequences of calls to other routines using values derived from offsets 8 and 10 of a structure.
- `load_constant_and_branch`: Loads the word 999 into d0 and branches to another routine.
- `process_and_call_multiple_times`: Performs several operations involving adding offsets to a4, calling another routine with d0 set to 50, and comparing d0 with 1000; it calls another routine multiple times using values derived from a5 and a4 before branching to another location.
- `load_constant_and_restore`: Loads the word value 999 into d0, restores d2, d3, and a2 from the stack, unlocks a5, and returns.
- `update_local_data`: Loads a 16-bit word from a structure into d1, compares it to 1, and if equal, sets d0 to 2, stores d0 at an offset of -2 from a5, stores a value from a memory location at offset -4894 of a4 into a5 offset -4, and branches.
- `set_conditional_values`: Compares d1 to 2; if not equal, it branches to another routine, otherwise it sets d0 to 1, stores it at an offset of -2 from a5, compares a word at an offset of -18644 from a4 to 2, and if not equal, it sets d0 to 3, stores it at an offset of -4 from a5, and branches to another routine; otherwise it branches to another routine.
- `perform_complex_math_scaling`: Calculates a value by performing multiple multiplications and divisions using data from two different memory locations, compares the results, and stores a final processed value in d0 before returning.
- `process_offset_values`: Subtracts 36 from a word at offset 12 of a5, adds the result to itself, calculates an address using a value from offset -24914 of a4, adds a word from offset 8 of a5 and a word from the calculated address, stores these in offsets -10 and -2 of a5, and branches based on a test of a word at offset -24636 of a4 and the sum in d2.
- `compare_quotients_of_values`: Calls two subroutines with values from the stack, then divides a word from offset 8 of a5 by 100 and compares the resulting quotient to the quotient of a word from offset minus 2 of a5 divided by 100.
- `branch_on_subroutine_result`: Pushes d2 to the stack, calls a subroutine, and if the result in d0 is non-zero, it sets d0 to zero and branches; otherwise it branches to another location.
- `update_and_clamp_values`: Copies a word from offset minus 10 of a5 to a location pointed to by offset 14 of a5, then copies a word from offset minus 2 of a5 to a location pointed to by offset 8 of a5, calls another routine, and if the result in d0 is zero, compares a word from offset minus 24898 of a4 to 58 to set d1 to 58 or 1.
- `clamp_value_range`: Compares a word from offset -24898 of a structure in a4 to 58, and if greater, stores 58 in offset -6 of a5; otherwise, it compares a word from offset -24896 of a4 to 98, and if greater, stores 98 in offset -6 of a5, or if it is greater than 1, sets d1 to 1.
- `calculate_quotients_and_call`: Compares a word from offset -24896 of a4 to 98, performs several multiplications and divisions using values from a5 and a4, and calls another routine with two words pushed onto the stack.
- `execute_repeated_calls`: Performs a series of calls to two different routines, using values derived from dividing a word at offset 12 of the structure in a5 by 100 and using the quotient and remainder to determine arguments and jump targets.
- `perform_long_jump`: Performs an absolute long jump to a specific address.
- `jump_to_address_6`: Performs a long jump to the address stored at offset 6928.
- `jump_to_address_7`: Performs a long jump to the address stored at offset 1536.
- `jump_to_address_38`: Performs a long jump to the address stored in a register.
- `jump_to_external_routine_2`: Jumps to another routine via a long jump instruction.
- `jump_to_menu_selection`: Performs a long jump to the process_menu_selection routine.
- `jump_to_address_8`: Performs a long jump to the address stored at offset 7964.
- `transfer_structure_pointers`: Pushes two pointers derived from a4 and two values from offsets 8 and 10 of a5 onto the stack, then calls another routine via a jump.
- `jump_to_address_two_hundred_twenty_four`: Performs a long jump to a longword address after loading the word value 224 into register d0.
- `jump_to_address_29`: Performs a long jump to the address 7692.
- `jump_to_absolute_address_4`: Performs an absolute long jump to a specific memory location.
- `jump_to_absolute_address_7`: Performs an unconditional jump to a 32-bit address stored in memory.
- `load_constant_e0`: Loads the word value 224 into d0 and performs a long jump to an address.
- `push_structure_elements`: Pushes a pointer to a structure, two values from offsets 10 and 8 of a5, and a constant 1 onto the stack, calls another routine, and returns.
- `push_arguments_and_call_routine`: Pushes three values onto the stack, including a constant and two values from a structure, then calls another routine and returns.

The function list continues in: g2221-recovery-functions-2.md.
