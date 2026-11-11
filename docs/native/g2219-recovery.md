# Game 2219 (empire - war game of the century v2.05 (1987)(interstel corporation)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02219). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909856 top-level bytes in 132 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 59 of 59 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 2220 of 2220 named (0 author symbols), 2220 of 2220 described (307 by rule, 1913 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3939 of 3939 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 42724 of 42726 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 307468 |
| code | 137136 |
| data | 427921 |
| padding | 26067 |
| unused | 11264 |

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
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf:Preferences | 45 | 6587 | 222 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf:Empire | 33 | 19031 | 1003 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf:CLI | 9 | 362 | 18 |
| icon.library | 15 | 1414 | 108 |
| info.library | 25 | 2476 | 170 |
| version.library | 2 | 41 | 7 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1754 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 323 | 8 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf bytes 103936-104448 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf bytes 104960-105472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf bytes 536576-537088 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: file devs/narrator.device (header 23280 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1110 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 120 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: bytes 103936-104448 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: bytes 104960-105472 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: bytes 536576-537088 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: 3 logged loads cover 1536 bytes of the image
- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: blocks 143-143 (bytes 73216-73728) outside the filesystem hold data, not code (opcode density 0.035, entropy 4.06)
- empire - war game of the century v2.05 (1987)(interstel corporation)/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf: blocks 154-197 (bytes 78848-101376) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.17)
- 3 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `initialize_and_copy_data`: It calls a routine, performs arithmetic on a pointer, compares a value at an offset to a constant, and enters a loop that copies blocks of data between memory locations using a dbra loop.
- `move_and_branch`: It moves the value in d1 to d0 and then performs a relative branch to another location.

### More

- `process_file_data`: Performs a sequence of library calls to handle file operations, parses a string from a buffer into a new list, and updates several global pointers and structure members.
- `process_library_and_files`: Checks a value at offset 20 of a structure, then calls a routine via a pointer at offset 8, performs several library calls including one with a pointer from offset 1C of the structure, and optionally calls another routine using a pointer from offset 20.
- `execute_two_calls`: Loads the address at offset 5c of a4 into a0 and calls a routine at offset -180 of a6, then repeats this process with a routine at offset -174 of a6.
- `call_dos_and_store_result`: It loads an address into a1, calls a library function using a6, stores the result in a structure at offset 8, and returns if the result is not zero.
- `modify_pointer_table`: Loads a pointer from offset 4 of a7 into a0, writes that pointer to its own location, increments the value at that location by 4, clears the value at offset 4 of a0, and writes a0 to offset 8 of a0.
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
- `execute_complex_file_logic`: Performs a large sequence of operations including checking a pointer in a6, calling multiple subroutines to handle file-related tasks, and performing various conditional branches and stack manipulations.
- `validate_read_byte`: Calls two routines, then enters a loop that reads bytes from a table at offset 2C of d4 and compares them against several constant values, returning 1 if the byte is within the range 80 to A0, otherwise returning 0.
- `extend_and_compare_value`: Sets a longword at a pointer in d4 to one, calls a routine, extends a byte from the stack into a 32-bit value in d0, and then compares d0 against a value from a table to determine whether to branch to another location or jump to another location.
- `set_bit_and_branch_4`: Sets bits in d0 using a constant and branches to another location.
- `push_arguments_and_call_routine`: Pushes three values onto the stack, including a constant and two values from a table, then calls a routine.
- `verify_and_execute_seek`: It writes a constant to a location, calls a routine, writes the result to a location, calls another routine, and if certain conditions regarding a structure and a word are met, calls a final routine via a branch to subroutine.
- `call_routine_with_stack_args`: It pushes a value from d0 into a structure at offset 24, pushes a value from a structure at offset 20 onto the stack, and calls a routine.
- `update_file_data`: Calls two routines, then enters a loop that compares values at offset 8 of a structure and increments pointers, eventually calling a routine to write three specific bytes to a location pointed to by offset 8 of a structure.
- `execute_looping_sequence`: It calls a routine with a pointer from d3, then enters a loop that compares two words from a structure in a0, and if they meet a condition, calls another routine using a calculated address from a counter in a6.
- `execute_sequence`: Pushes a longword onto the stack, calls a routine, increments the stack pointer, calls another routine, increments the stack pointer, and then performs a relative branch.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `perform_indirect_call`: It calls a routine using a pointer loaded from offset 8 of a structure in a6, and then calls another routine using a pointer from the stack.
- `execute_indirect_call_2`: Loads a longword from an offset of 8 of a pointer in a0, increments it, pushes it onto the stack, and calls a routine via a pointer in a6.
- `initialize_structure_and_call`: Calls several routines, pushes specific addresses onto the stack, and uses a value from a structure to perform an extended load before calling another routine.
- `execute_file_operations`: Calls several routines to handle file and memory operations, including a call to a routine with an address from a pointer, and stores the result in a local variable before returning.
- `process_extended_byte`: Calls another routine, compares the result to 10, and if it is not 10, extends a byte from offset -4 of a6 into d0 and updates a pointer and a counter at offset -4 and 8 of a6 before returning; otherwise it clears a byte at the address in a0 and returns.
- `compare_and_store_results`: Calls a routine twice while pushing values from a structure onto the stack, then compares a value from a structure to d0 and branches if d0 is greater than or equal to that value.
- `compare_and_call_handler`: It calculates a value based on offsets of a6, compares it to a value at offset 8 of a6, and if they are equal, it pushes three values onto the stack and calls another routine.
- `increment_and_branch`: Increments a longword at offset 8 of a6 and branches to another location.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `increment_offset_values`: Increments a longword at offset 8 of a6 and, if the byte at the address in a0 is not zero, increments a longword at offset -4 of a6 before branching to another location.
- `return_offset_value`: Moves the 32-bit value at offset -4 of a6 into d0 and returns.
- `compare_bytes_and_return`: Compares a byte from offset 8 of a0 with a byte from offset 12 of a0, and if they are not equal, it returns with d0 set to 0; otherwise it branches to another routine.
- `increment_multiple_offsets`: Increments longwords at offsets -4, 8, and 12 of a6 and branches to another location.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `handle_library_and_files`: Performs several conditional checks on global pointers, calls multiple routines including those that push addresses to the stack, and executes a final routine before returning.
- `allocate_and_initialize_structure_2`: It calls a routine to obtain a pointer in d0, then uses that pointer to access a structure and calls another routine with several values from the stack and the structure.
- `allocate_and_copy_data`: It calls a routine to allocate memory, then calls another routine to initialize a structure, and finally calls a third routine to copy data into the allocated memory.
- `initialize_and_copy_structure`: Allocates memory, initializes a structure with values from the stack, and performs a loop to copy data from one table to another based on a count at offset 20 of a structure in a1.
- `execute_indirect_call`: Pushes d2 and a pointer from the stack onto the stack, calls a routine via a pointer in a6, and restores d2 before returning.
- `read_and_extend_byte`: Calls a routine, compares its result to 1, and if they are not equal, extends a byte from offset -4 of a6 into a 32-bit value in d0 before returning.
- `write_buffer_data`: Calls a routine via a pointer in a6, then checks a byte at the address in a3; if the byte is not zero, it calls another routine with arguments from a4 and a pointer at offset 4 of a7.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_raw_do_fmt`: Saves registers, loads addresses from the stack into a3, a0, and a1, calls a routine via a pointer in a6, and restores registers before returning.
- `store_byte_and_increment`: Stores the byte from d0 at the address in a3 and increments a3 before returning.
- `multiply_and_add_signed_values`: Performs signed 16-bit multiplication of two values and adds the results together into a 32-bit register.
- `calculate_absolute_difference`: Calculates the absolute difference between two 32-bit values and performs a series of conditional sign adjustments and bitwise operations on the result.
- `allocate_and_initialize_structure`: Saves registers, calls a routine to allocate memory, and if successful, initializes a structure at a2 with values from the stack and calls several other routines to manage signals and ports before returning the address in a2.
- `cleanup_structure_members`: Checks offset 10 of a structure, then calls three different routines using pointers and values derived from the structure, including setting a byte at offset 8 to 255.

### Stack

- `allocate_and_copy_data_blocks_2`: Calls another routine to determine a size, then iterates through a table to copy blocks of data into a newly allocated memory area.
- `transfer_register_to_register`: Copies the contents of register d1 into register d0 and branches to another routine.

### Else

- `process_data_block`: Calls a routine via a pointer in a5, performs arithmetic on a pointer in a4, and enters a loop that copies data and increments pointers.
- `branch_with_data`: Moves the longword from d1 to d0 and performs a relative branch.

### Delete

- `manage_data_structures`: Calls several routines, performs arithmetic on a pointer, and enters a loop that copies blocks of data and modifies values in a structure.
- `transfer_d1_to_d0`: Moves the value from d1 to d0 and then branches to another location.

### type

- `initialize_complex_structure_2`: Calls a routine, then enters a loop that copies data and performs arithmetic on pointers, eventually calling several routines and using dbra loops to copy blocks of data.
- `branch_with_data_2`: Moves d1 to d0 and branches to a relative offset.

### Path

- `reallocate_and_copy_data_structure_3`: Calls another routine to determine a size, then performs a memory reallocation by copying a block of data and inserting a specific 32-bit marker.
- `transfer_d1_to_d0_and_branch`: Moves the contents of register d1 into register d0 and then branches to another routine.

### EndCLI

- `allocate_and_copy_data_blocks_3`: Retrieves a pointer from a structure, checks a table for a specific value, and if found, calls another routine to allocate memory before copying blocks of data including a specific constant.
- `move_and_branch_5`: Moves the contents of d1 to d0 and then performs a relative branch.

### Dir

- `process_directory_data`: Calls a routine, performs arithmetic on a pointer, compares a value at an offset to a constant, and enters loops to copy data blocks and perform multiple subsequent calls.
- `transfer_register_and_branch_6`: Moves a longword from register d1 to register d0 and performs a short branch into another routine.

### NewCLI

- `allocate_and_copy_data_blocks_4`: Calls another routine to determine a size, then performs a series of memory allocations and block copies involving structured data.
- `transfer_register_and_branch_7`: Moves a 32-bit value from register d1 to register d0 and then performs a short branch into another routine.

### Echo

- `manage_complex_data_structures`: It calls a routine via a5, performs arithmetic on a pointer, and contains multiple loops that use dbra to copy blocks of data and call other routines based on values found in a structure in a2.
- `transfer_long_word`: Moves a long word from d1 to d0 and then performs a relative branch.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `process_data_structure`: Calls a routine via a pointer in a5, manipulates a table based on a value in d1, and performs several loops involving data copying and stack operations.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch into another routine.

### Execute

- `reallocate_and_copy_data_structure`: Retrieves a pointer from a structure, calls another routine to determine a size, and performs a memory reallocation by copying data to a new location.
- `transfer_register_and_branch_2`: Moves the contents of data register 1 into data register 0 and then performs a short branch into another routine.

### makedir

- `allocate_and_initialize_data_structures`: Calls another routine to obtain a memory address, then traverses a table to determine the required size for a new block.
- `transfer_register_and_branch_3`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.

### CD

- `allocate_and_copy_data_blocks`: Calls another routine to determine a size, then iterates through a table to copy blocks of data into a newly allocated memory area.
- `transfer_register_and_branch_4`: Moves the contents of register d1 into register d0 and performs a short branch into another routine.

### EndIf

- `clear_register_d0`: Returns immediately after clearing register d0.

### If

- `reallocate_and_copy_data_structure_2`: Retrieves a pointer from a structure, calls another routine to determine a size, and performs a memory reallocation by copying data to a new location.
- `move_and_branch_4`: Moves the value from d1 to d0 and then performs a relative branch.

### LoadWB

- `load_file_logic`: Executes library calls to manage files, parses a string into a list, and conditionally calls a routine using a value from a structure offset.
- `execute_library_calls`: Calls three different functions through a pointer in a structure and performs a conditional call to a function with a specific constant in d7.
- `call_two_functions`: Calls two different routines using addresses derived from an offset of register a4.
- `execute_library_call`: Loads an address into a1, calls a routine via a pointer in a6, stores the result in a longword at offset 8, and branches to another location if the result is zero.
- `load_and_initialize_library_data`: Allocates memory and initializes a structure by calling several routines to find resident libraries, open them, and copy data into the newly allocated block.
- `compare_byte_sequences`: Compares a byte from one memory location to a byte in another memory location and returns 0, 1, or -1 based on the comparison result.
- `allocate_and_initialize_memory`: Calls a memory allocation routine, then enters a loop that calls another routine to write data to the allocated memory and updates a table of pointers.
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
- `compare_memory_blocks`: Compares bytes from a source at a2 and a destination at a pointer in a1, and if they are not equal, it branches to another location; otherwise it branches to another routine.
- `restore_and_return_2`: Loads an address into a2, restores a0 and a1 from the stack, pushes a longword from the program counter onto the stack, and returns.
- `call_write_buffer_data_via_stack`: Pushes a value from the stack onto the stack, calls a routine via a pointer stored at an offset of 8 from the stack pointer, and restores the stack pointer before returning.
- `write_buffer_loop`: Calls a function to check a byte, and if the byte is not zero, calls another routine to write data to a buffer.
- `store_byte_and_return`: Stores the byte from register d0 into the memory location pointed to by register a3 and increments the pointer.

### Wait

- `process_data_structure_2`: Calls a routine via a pointer in a5, manipulates a pointer from a table, and enters loops to copy data blocks and update a table using dbra.
- `transfer_register_and_branch_5`: Moves the contents of data register 1 into data register 0 and performs a short branch into another routine.

### Port-Handler

- `process_port_data`: Initializes a large structure with various offsets, performs several loops to manipulate data in a table, and calls multiple functions through a pointer in a2.

### Disk-Validator

- `validate_and_update_data`: Performs a complex sequence of data validation and structure updates involving multiple library calls, pointer arithmetic, and conditional loops that modify various offsets within structures pointed to by a6 and a1.

### Ram-Handler

- `copy_memory_and_jump`: Sets up registers for a memory copy operation using a library call, then calculates a new address in a0 and jumps to it.

### serial.device

- `initialize_serial_device`: The routine is empty and returns immediately.
- `update_device_parameters`: Calls two different library routines and, if a condition is met, populates several offsets of a structure in a6 with values derived from d0 and d2.
- `check_bit_and_call`: Tests a bit in a byte at an offset of a1, and if the result is non-zero, calls a library routine via a6.
- `write_byte_and_branch`: Stores the byte value 253 at offset 31 of the structure in a1 and then branches to another location.
- `toggle_interrupt_enable_2`: Writes values to the INTENA register to enable and then disable interrupts, then calls a routine and returns.
- `check_status_values`: Clears a byte at offset 28 of a1 and performs conditional branches based on whether a word at offset 28 of a1 equals 2, 3, or 10.
- `clear_status_and_branch`: If a longword at offset 74 of a6 does not match a1, it clears bit 1 of a byte at offset 296 of a6, clears a longword at offset 74 of a6, and branches to another routine; otherwise it branches to another routine.
- `swap_pointers_and_decrement`: Tests bit 6 of a byte at offset 30 of a1; if set, it swaps two pointers stored at offsets 0 and 4 of a1 and a0, then decrements a word at offset 70 of a6 and branches to another location.
- `compare_and_reset_hardware`: Compares a1 with a value at offset 78 of a6; if they are not equal, or if a word at offset 28 of a1 is not 10, it branches to another location; otherwise, it clears bit 5 of a word at offset 470 of a6, sets a hardware register to 2048, calls another routine, clears a word at offset 78 of a6, clears bit 4 of a word at offset 472 of a6, and branches to another location.
- `modify_status_and_call`: Tests bit 6 of a byte at offset 30 of a1, performs several bitwise operations and pointer swaps on a1 and a0, and calls a library routine if a specific bit is set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `set_status_bit_conditionally`: Sets a 32-bit register d0 to zero; if a word at offset 32 of a6 is non-zero, it sets bit 6 of a word at offset 472 of a6 and returns.
- `execute_device_sequence`: Calls four different library routines with various parameters and performs pointer manipulation on a structure in a6 before returning.
- `update_structure_and_call_library`: Clears a longword at offset 50 of a1, loads a longword from offset 54 of a1 into d1, and branches to another routine if d1 is zero; otherwise, if certain conditions in a6 are met, it clears bit 0 and sets bit 6 of a byte at offset 48 of a1, performs a pointer swap between a1 and a new pointer derived from a6, increments a word at offset 112 of a6, and calls a library routine via a6.
- `update_device_structure`: Saves d2 and a6 to the stack, loads a pointer from a1 into a6, checks a longword at offset 74 and a word at offset 70, and if conditions are met, performs several data movements and bit manipulations on structures pointed to by a0 and a1.
- `process_serial_buffer_data`: Checks if a buffer length is valid and calls another routine, then iterates through a byte sequence by incrementing a pointer and decrementing a counter.
- `test_bit_and_branch_2`: Tests bit 0 of a byte at offset 443 of a6 and branches accordingly.
- `branch_and_set_byte`: Calls another routine and, if the result is zero, sets a byte at offset 31 of a1 to 9 before branching to another routine; otherwise it branches to another routine.
- `process_buffer_and_call_library`: Copies a byte from offset 82 of a6 to a location in a0, increments a longword at offset 50 of a1 and a longword at offset 130 of a6 if certain conditions are met, runs a loop seven times to compare bytes, and calls a library routine via a6 if specific bits in a1 and a6 are set.
- `restore_registers_and_return`: Restores d2 and a6 from the stack and returns.
- `test_bit_logic`: Performs bitwise transformations on d0 and d1, then checks a bit in d2 against d1 after a conditional bitwise inversion.
- `clear_bit_and_branch_2`: Clears bit 2 of a byte at offset 470 of a6 and branches; otherwise it moves the byte value 15 into offset 31 of a1 and branches.
- `clear_bit_and_branch`: Clears bit 7 of a byte at offset 470 of a6 and branches; otherwise it moves the byte value 9 into offset 31 of a1 and branches.
- `modify_status_byte_2`: Sets a byte at offset 31 of a1 to 12, checks bit 0 of a byte at offset 294 of a6, and if it is zero, sets a byte at offset 31 of a1 to 6, clears a longword at offset 62 of a6, and branches to another routine; otherwise it sets the byte to 6 and branches to another routine.
- `process_serial_interrupt`: Reads a word from the custom chip interrupt register, writes a value to the interrupt mask register, and performs several conditional checks on bits in a structure in a6 to potentially set bit 2 of offset 472 of that structure.
- `modify_interrupt_register`: Compares a byte at offset 34 of a6 with d1, tests bit 2 of offset 472 of a6, and if bit 2 is clear, clears bit 2 of offset 472 of a6 and writes the word 32769 to the INTENA register.
- `test_serial_bits`: Tests bit 5 of an offset in a6, tests bit 1 of another offset in a6, calls another routine, and then tests bit 8 of d0 before returning or branching.
- `test_bit_and_branch_3`: Tests bit 8 of d0 and branches; otherwise it branches.
- `test_bit_and_branch`: Tests bit 0 of a byte at offset 445 of a6 and branches to another location.
- `set_bit_and_update_pointer`: Sets bit 7 of a byte at offset 470 of a6, tests a longword at offset 62 of a6, and if non-zero, moves a longword from offset 42 of a6 to offset 62 of a6 before branching.
- `transfer_buffer_byte`: Compares a longword at offset 54 of a6 with a longword at offset 58 of a6, and if the first is greater, moves a byte from d0 to the address in a0, increments offsets 42 and 58 of a6, and increments a0.
- `call_library_function_8`: If a longword at offset 62 of a6 is non-zero, it moves a longword from offset 42 of a6 to offset 62 of a6, calls a routine via a pointer in a6, and returns.
- `clear_hardware_bit`: Moves a byte from offset 34 of a6 to offset 297 of a6, clears bit 3 of a byte at offset 296 of a6, and if bit 2 of a byte at offset 69 of a6 is set, it clears bit 6 of a longword at offset an address.
- `modify_interrupt_and_status`: Moves a byte from offset 35 of a6 to offset 473 of a6, sets bit 3 of a word at offset 472 of a6, and if bit 2 of a byte at offset 69 of a6 is not set, it sets bit 6 of a longword at offset an address and writes 32771 to a hardware register; otherwise, it writes 32771 to a hardware register and returns.
- `clear_status_byte`: Clears a single byte at offset 473 of a structure in a6 and returns.
- `toggle_device_bit`: Tests bit 6 of offset 470 of a6; if it is zero, it sets the bit, initializes a structure at offset 386 of a6, and calls a routine via a pointer at offset 20 of the new structure.
- `update_status_and_register`: Saves registers to the stack, sets a6 to a1, clears bit 6 of a byte at offset 470 of a6 and branches; otherwise it reads a word from the hardware register at offset a fixed address, masks it with 1023, and if non-zero branches; otherwise it sets bit 2 of a byte at offset 470 of a6 and branches.
- `load_address_and_call`: Loads the address at offset 386 of a6 into a0, calls another routine, and branches to another location.
- `set_bit_and_branch_3`: Sets bit 0 of a byte at offset 470 of a6 and branches.
- `swap_structure_pointers`: If a longword at offset 36 of a1 is non-zero, it clears bit 0 of a byte at offset 30 of a1, and if a word at offset 72 of a6 is non-zero and a longword at offset 78 of a6 is non-zero, it swaps two pointers within a structure and increments a word at offset 72 of a6 before branching to another routine.
- `check_interrupt_and_update_pointers`: Tests bit 2 of a byte at offset 69 of a6; if set, it calls another routine, tests a byte at offset 31 of a1, and if non-zero, sets bit 4 of a byte at offset 30 of a1, clears a longword at offset 94 of a6, copies two pointers from a1 into a6, and writes 32771 to two hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_serial_status`: Reads a byte from a hardware register, checks if bits 3 and 4 are set, and if bit 3 is set, writes the value 13 to offset 31 of a structure in a1.
- `validate_and_process_serial_interrupt`: Checks several status bits in a structure and a linked structure, then decrements a counter at offset 470 of a structure and calls another routine after shifting a value from offset 462 by six bits.
- `update_serial_status`: Writes a word to an offset in a6, loads a byte from a pointer in a0, compares a longword in a1 to d1, and performs bit tests and decrements on a value in a6.
- `update_serial_interrupt_status`: Compares a longword in a structure with a register, writes a word to the interrupt mask register, and performs bitwise operations on a status byte in a structure before branching based on bit tests.
- `update_serial_port_and_dispatch`: Updates a hardware register with a bitwise combination of a byte from a structure and a word from another, increments two counters in a structure, and executes a loop to compare a value against a table of up to eight bytes.
- `update_device_status`: Decrements a counter in a structure, updates status bits in a linked structure, and calls another routine if a specific value is met.
- `restore_registers_and_enable_interrupts`: Restores multiple registers from a stack and sets the interrupt enable bit in the hardware interrupt enable register, then returns.
- `update_interrupt_mask`: Writes a value to the interrupt mask register, clears d1, loads a byte from offset 465 of a1 into d1, performs a bitwise OR between d1 and the word at offset 468 of a1, clears the byte at offset 465 of a1, writes d1 to the interrupt mask register, and then branches based on the value at offset 78 of a1 and offset 72 of a1.
- `reconfigure_interrupt_and_device_params`: Enables interrupts via a hardware register, validates two data structures using two different routines, modifies a bit in a byte at offset 69 of a structure and clears a byte at offset 472 of a structure, populates a new structure with several longword constants, and finally reconfigures the hardware interrupt mask before branching to another routine.
- `branch_to_subroutines`: Calls a subroutine and then performs an unconditional branch to another location.
- `copy_and_clear_structure`: Copies two longwords from offset 38 of a structure in a6 to offsets 42 and 46 of the same structure, then clears a longword at offset 58.
- `modify_interrupt_status`: Loads a longword from offset 78 of a6 into d0, and if non-zero, sets bit 2 of offset 472 of a6, loads a longword from offset 74 of a6 into d0, and if non-zero, calls another routine and branches.
- `clear_bit_and_set_interrupt`: Clears bit 2 of a byte at offset 474 of a structure in a6, loads a long value from offset 74 of the structure into d0, and if d0 is zero, calls another routine and writes the value 32769 to the interrupt request register before branching to another location; otherwise, it calls another routine and returns.
- `toggle_interrupt_enable`: Enables interrupts by writing a value to the interrupt enable register, calls another routine, then disables interrupts by writing a different value to the same register before branching to another routine.
- `clear_buffer_pointers`: Pushes a1 onto the stack, loads the address at offset 162 of a6 into a0 and calls another routine, loads the address at offset 176 of a6 into a0 and calls another routine, clears a longword at offset 70 of a6, and restores a1 from the stack before returning.
- `transfer_data_block`: Copies a longword from the address in a1 to the address in a0, then if d0 is not zero, exchanges d0 and a1, stores a0 at offset 4 of a1, and calls a subroutine if d0 is non-zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_bit_and_call_library`: Sets bit 5 of a byte at offset 30 of a structure in a1, writes 254 to offset 31 of a1, and if bit 0 of that same byte is not set, calls a routine using a pointer from offset 98 of a structure in a6.
- `modify_status_byte`: Loads a byte from a constant address, masks it, checks two bits in a structure in a6, sets bit 4 of the masked byte, stores the result at offset 81 of a1, copies a byte from offset 470 of a6 to offset 80 of a1, masks the byte at offset 80 of a1, copies a longword from offset 58 of a6 to offset 32 of a1, and branches to another routine.
- `test_status_flags`: Clears bit 0 of offset 1e of a1, tests bit 3 of offset 4f of a1, and if bit 3 is set, checks if the word at offset 48 of a6 or the longword at offset 4e of a6 are non-zero.
- `trigger_interrupt_handler`: Loads a longword from offset 106 of a6 into d0, calls a subroutine, checks a byte at offset 31 of a1, and if it is zero, stores a1 at offset 78 of a6, sets bit 5 of offset 294 of a6, and writes a word to a hardware register.
- `call_library_and_clear`: Calls a routine using a pointer from offset 98 of a structure in a6 and then clears a longword at offset 78 of the structure in a6.
- `check_and_reset_interrupt_state`: Clears bit 5 of a word at a specific offset, and if it was set, checks if a value at offset 40 of a structure is equal to 10.
- `enable_interrupts_and_call_library`: Enables interrupts by setting a bit in the hardware register, populates a structure with values from registers, and calls a library function via a pointer.
- `modify_serial_flags`: Tests bit 7 of an offset in a1, sets bit 7 of an offset in a6, clears bit 2 of an offset in a6, and calls another routine based on bit tests in a1 and a6.
- `test_conditions_and_move`: Clears bit 7 of a byte at offset 69 of a6, tests bit 1 of a byte at offset 472 of a6 and branches; otherwise it tests a longword at offset 78 of a6 and branches; otherwise it tests bit 4 of a byte at offset 30 of a1 and branches; otherwise it tests a longword at offset 70 of a6 and if zero moves the byte value 1 into offset 31 of a1 and branches.
- `toggle_interrupt_enable_and_branch`: Enables interrupts by writing to the hardware register, calls another routine, disables interrupts by writing to the hardware register, and then branches to another routine.
- `validate_and_update_device_parameters`: Validates three bytes and a longword within a structure, updates several fields in a secondary structure based on these values, and calls three different routines before writing the value five to offset 31 of the input structure.
- `process_table_indices`: Clears d0 and d1, loads a byte from offset 77 of a1 into d0, sets bit 0 of d0, compares the byte at offset 78 of a1 to two, and if it matches, performs shifts and bit sets before calling a subroutine multiple times with different values stored in offsets 468, 466, and 464 of a6.
- `fetch_table_value`: Checks bit 0 of a byte at offset 69 of a structure in a6; if set, it decrements d0, shifts it left by 1, and uses the result to fetch a word from a table located at offset 4462 from the current program counter.
- `set_byte_and_branch`: Writes the byte value five to offset 31 of a1 and then performs an unconditional branch.
- `calculate_division_quotient`: Loads a longword from offset 450 of a6, performs arithmetic operations including a shift and a subtraction, and if the result is not less than or equal to negative 65535, performs a division and bitwise operations before branching.
- `update_serial_port_configuration`: Divides two words, stores the quotient in a structure offset, and performs conditional bit manipulations on a status register and a hardware register.
- `handle_device_request`: Calls a library routine to initialize a device, then conditionally calls two other routines to manage device state and parameters.
- `load_device_pointers`: Loads two addresses into a3 and a0 from offsets of a6, calls another routine, and then branches to another location.
- `store_pointer_to_structure`: Stores the value in a0 into offset 16 of a structure in a1.
- `update_serial_structure`: Stores the value 16 at offset 9 of a structure in a0, writes 56 to offset a of the same structure, and copies the contents of a3 and a6 to offsets 18 and 14 of the structure before returning.
- `call_library_function`: Writes 56 to offset a of a structure in a1, sets bytes at offsets 9, e, and 8 to 0, 1, and 4 respectively, pushes a6 to the stack, calls a library function using a base from offset 98 of a6, and returns.
- `configure_serial_device`: Performs several subroutine calls to configure structures, calls a library function using a pointer from a6, calls two subroutines to store values in offsets 430 and 434 of a structure in a6, and writes 2049 to the INTENA register before branching to another location.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_and_branch`: Clears the d0 register and branches to an instruction at a nearby address.

### dk

- `modify_data_table`: Performs several bitwise OR operations on d0 and subtracts 0 from a word at offset 76 of a structure in a6 using d0 as an index.

### is

- `clear_and_decrement`: Performs three bitwise OR operations on d0 and subtracts 0 from a word at the address pointed to by a6.

### d

- `execute_program_entry`: An entry routine consisting of 13 instructions.

### e

- `zeroes_register_and_modifies_memory`: Clears a data register three times, subtracts zero from a word at offset seventy-six of a structure pointed to by a6 using the register as an index, and then performs a sequence of logical OR operations on the data register with various byte constants.

### f

- `clear_d0_and_decrement_stack`: Clears the low byte of register d0 three times and subtracts zero from the word at the address pointed to by the stack pointer offset by the value in register a6.

### cdn

- `clear_and_populate_register`: Clears a 16-bit word in a global variable using a zeroed register as an index, then performs a sequence of logical OR operations to load a series of constant bytes into that same register.

### usa0

- `initialize_system_state`: Sets the initial values of several global variables and prepares the processor registers for the main program execution before returning.

### i

- `clear_d0_and_decrement_a6`: Clears the d0 register three times and subtracts zero from the word at the address pointed to by a6, decrementing a6.

### usa1

- `clear_d0_and_decrement_a6_post_inc`: Clears the d0 register three times and subtracts zero from the word at the address pointed to by a6, then increments a6.

### usa2

- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### n

- `initialize_n_keymap_data`: Clears d0 three times, subtracts zero from a word at offset 76 of a structure using d0 as an index, and then loads several byte constants into d0.

### s

- `initialize_keymap_data`: Clears a 16-bit value in a global variable using an index in register d0, then performs a series of byte-wise logical OR operations to populate register d0 with a sequence of constant values.

### gb

- `initialize_gb_keymap_data`: Clears d0 three times, subtracts zero from a word at offset 76 of a structure pointed to by a2, and then loads several byte constants into d0.

### ch1

- `initialize_ch1_keymap_data`: Clears d0 three times, subtracts the constant 76 from d5, and then loads several byte constants into d0.

### ch2

- `initialize_d0_and_d5_registers`: Clears the d0 register three times, subtracts seventy-six from the d5 register, and then performs a series of logical OR operations on the d0 register using various byte constants.

### parallel.device

- `process_parallel_device_request`: Sets d0 and d1 to 97, performs conditional branches based on status flags, and then modifies bytes at the addresses in a0 and a2 before clearing bit 0 of a word at the address in a6.
- `update_parallel_device_state`: Clears a byte at offset 118 of a structure, checks bit 5 of a byte at offset 53 of a structure, and if certain conditions are met, increments a word at offset 32 of a structure, moves a byte to offset 53 of a structure, sets bit 4 of a byte at offset 118 of a structure, and moves two longwords into offsets 54 and 58 of a structure.
- `execute_parallel_command`: Performs a bitwise AND between a constant and a byte at offset 30 of a1, calls a library routine via a pointer at offset 50 of a6, compares a word at offset 28 of a1 to two, and if it matches, checks a bit in a structure in a6 and conditionally modifies hardware registers.
- `execute_parallel_device_command`: Compares a word at offset 28 of a structure to 3, clears bit 0 of a CIA register, checks bit 0 of another CIA register, and if bit 2 of a byte at offset 118 of a structure is not set, calls another routine with d0 set to 144 and then sets bit 2 of a byte at offset 118 of a structure.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_command_byte`: Writes the byte value 253 to offset 31 of a1 and branches.
- `clear_structure_field`: Tests a longword at offset 36 of a1, and if non-zero, branches; otherwise it clears a longword at offset 32 of a1 and branches.
- `update_structure_pointers_2`: Clears bit 0 of offset 30 of a1, checks bit 2 of offset 280 of a6, and if set, sets bit 6 of offset 30 of a1 and performs a series of pointer swaps and assignments between a1 and a stack-based structure before incrementing a value at offset 180 of a6.
- `configure_parallel_port_status`: Sets bit 4 of a value at offset 30 of a structure, updates several fields in a local stack frame, and conditionally writes a byte to a hardware port based on a comparison of a word at offset 28 of the structure.
- `update_parallel_device_link`: Decrements a 16-bit counter in a structure, updates a pointer in a linked list, and calls another routine after clearing bit 6 of a status field at offset 30.
- `transfer_parallel_byte`: Reads a byte from a CIA register and writes it to a memory location pointed to by a structure member, then increments two counters in that structure.
- `process_parallel_device_request_2`: Copies a longword from offset 46 of a structure in a6 to offset 32 of a1, clears bit 3 of offset 118 in a6 and bit 4 of offset 30 in a1, calls a library function at offset 50 of a6, clears offset 38 of a6, and if a 16-bit word at offset 114 of a6 is not zero, clears bit 2 of offset 118 in a6 and calls a library function at offset 34 of a6 with d0 set to 16.
- `check_hardware_status`: Moves a byte from a0 to offset an address of the hardware, and if non-zero, branches; otherwise it tests bit 3 of offset 118 of a6, and if non-zero, branches, otherwise it branches.
- `call_parallel_update`: Calls a subroutine using a pointer from a structure in a6, then branches to another location.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_function_16`: Checks a longword at offset 38 of a6, and if non-zero, calls a library function at offset minus 18 of the library base found at offset 22 of a6, passing the value from offset 26 of a6 in a1 and the constant 16 in d0.
- `call_library_function_144`: Checks a longword at offset 38 of a6, and if non-zero, calls a library function at offset minus 18 of the library base found at offset 22 of a6, passing the value from offset 26 of a6 in a1 and the constant 144 in d0.
- `update_linked_list_4`: Loads the address at offset 76 of a6 into a0, loads the address at offset 74 of a6 into a1, loads a longword from a1 into d0, and if non-zero, stores d0 at offset 0 of a0, exchanges d0 and a1, stores a0 at offset 4 of a1, and if d0 is non-zero, sets a1 to d0 and calls another routine before branching.
- `clear_word_offset_72`: Clears a 16-bit word at offset 72 of a6 and returns.
- `set_cia_bits_and_call`: Sets bit 5 of a byte at offset 30 of a1, sets a byte at offset 31 of a1 to 254, and calls a library function at offset minus 442 of the library base found at offset 50 of a6.
- `update_cia_port_bits`: Masks bits in a global variable and a byte from a global variable, then updates the port address register of a CIA chip and a bit in a structure at offset 116 of a6 before calling another routine.
- `set_status_byte`: Tests bit 2 of offset 118 of a6, and if zero, writes the byte value 1 to offset 31 of a1 and branches.
- `load_structure_offsets`: Checks bit 1 of a byte at offset 53 of a1; if set, it copies a longword from offset 54 of a1 to offset 58 of a6 and another longword from offset 58 of a1 to offset 62 of a6.
- `handle_parallel_device`: Saves registers, calls a routine using a pointer from a6, performs several conditional calls to other routines, manipulates a linked list structure, and calls two more routines before returning.
- `clear_and_branch_2`: Clears register d0 and branches to another routine.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_handler_and_restore`: Saves a0 and a6 to the stack, calls a routine at a specific address, increments the stack pointer by 8, and returns.
- `call_clipboard_handler_3`: Saves d0 and a1 to the stack, calls a routine at address 2754, cleans the stack, and returns.
- `call_clipboard_handler_2`: Pushes the pointer in a1 onto the stack, calls a routine at address 3516, increments the stack pointer by 4, and returns.
- `jump_to_handler`: Performs an indirect jump using the longword at offset 246 of the address space.
- `call_clipboard_handler`: Pushes the value of a1 onto the stack, calls a routine at an address from a table, and then returns.
- `push_and_call_3`: Pushes a1 onto the stack, calls a routine at a specific address, and increments the stack pointer by 4.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `execute_clipboard_transfer`: Enters a loop that moves a longword from d3 into a memory location pointed to by a structure in d2 and calls a routine via a pointer.
- `transfer_clipboard_data`: Stores a 32-bit value from d3 into a memory location pointed to by a structure in d2, then calls two library functions and a routine via a pointer in a6, finally setting d0 to zero before returning.
- `call_two_library_functions`: Pushes a pointer from a memory location onto the stack, calls a library function at offset minus 138 of a library base, calls a library function at offset minus 432 of a library base, and restores registers.
- `increment_clipboard_counter`: Calls two routines, then checks if d0 is zero; if not, it accesses a structure at offset 80 of a pointer in a2, checks a value at offset 4 of that structure, and if non-zero, increments a 32-bit value at offset 14 of a structure in a3.
- `call_library_with_parameter`: Clears a byte at offset 14 of a3, pushes the address at offset 72 of a2 onto the stack, calls a library function at offset 128 of a library base, and branches to another routine.
- `clear_and_call_library`: Clears a 16-bit word at offset 94 of a2, calls a library function at offset 16 of a library base, and moves the value from a3 into d0.
- `process_library_pointer`: Calls a library function at offset 0, then if a value at offset 72 of a2 is non-zero, calls another routine at offset 56 of a structure pointed to by a0.
- `execute_library_calls_3`: Pushes the address at offset 72 of a2 onto the stack, calls a library function at offset 0, clears a byte at offset 86 of a2, calls a library function at offset 16 of a2, and restores registers d2 and a2 before returning.
- `execute_clipboard_operation`: Calls a routine via a pointer, checks a word at offset 94 of a2, and if certain conditions are met, calls another routine using a pointer from a structure and then calls a third routine via a pointer from a3.
- `compare_clipboard_values`: Loads a longword from offset 5a of a2 into d0, compares it to a longword on the stack, and if d0 is not less than or equal to the stack value, performs arithmetic and comparisons using values from offset 5a of a2 and offset 2c of the stack.
- `calculate_clipboard_limit`: Loads values from the stack, performs a logical shift right on d1, compares d1 to a longword at offset 14 of a structure in a1, and branches based on the result.
- `compare_and_update_pointer_2`: Moves a 32-bit value from d1 to d0, compares d0 with a long value at offset 14 of a structure in a1, and if d0 is not lower than or equal to that value, it updates a1 with a new pointer and branches to another location.
- `calculate_copy_limit`: Compares d5 to the result of subtracting d2 from 512, and if d5 is not less than or equal to that result, it sets d4 to d2 and branches to another routine; otherwise it returns.
- `copy_data_loop`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d4 as an index in a loop that runs until d4 reaches 512, then calculates a value into d5 based on d2 and a constant 512 before branching to another routine.
- `copy_index_to_register`: Copies a 16-bit value from d2 to d4 and branches to another routine.
- `copy_data_with_limit`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d4 as an index in a loop, then compares a 32-bit value in d0 with the sum of d1 and d2, and if d0 is not greater than or equal to that sum, sets d5 to zero before branching to another routine.
- `store_and_call_library`: Stores the value in a1 into offset 86 of a2, calls a library function at offset 16 of a library base, and moves the value from d3 into d0.
- `manage_clipboard_state`: Saves registers, calls a routine, and then performs a series of conditional checks on a structure in a2, including calling several library stubs and other routines to update or clear values.
- `execute_conditional_library_calls`: Pushes an address from a global variable onto the stack, sets a1 to d4 and pushes offset 18 of a1, calls a routine via a pointer at offset 0 of a1, and if a 32-bit value in d0 is not 512, clears a 16-bit word at offset 94 of a2, pushes d4, and calls a routine via a pointer at offset 18 of a1.
- `execute_clipboard_jump`: Pushes the address at offset 72 of a2 onto the stack, calls a routine at that address, cleans the stack, and branches to another location.
- `update_clipboard_state`: Clears a word at offset 94 of a2, and if it is non-zero and a longword at offset 68 of a2 is non-zero, calls two different routines using pointers from a6 and a3.
- `traverse_clipboard_list_2`: Calculates a bit-shifted value from a structure in a6 and calls another routine if a value at offset 86 of a structure in a2 is zero, then iterates through a linked list by comparing a value to a longword at offset 14 of the current node.
- `traverse_clipboard_list`: Contains a loop that compares d0 to a longword at offset 14 of a pointer in a1, updates a pointer by reading from memory, and calls another routine before repeating the loop.
- `calculate_offset_limit`: Checks if d4 is zero or if a long value at offset 20 of a structure in a6 is not higher than zero; if either is true, it returns, otherwise it calculates a value by subtracting d3 from 512, compares it to the value at offset 20 of the structure in a6, and either moves d3 to d1 and branches or moves 512 into d0.
- `copy_and_update_counter`: Enters a loop that copies bytes from a source to a destination using an offset from a structure in d4, then calls a routine and performs arithmetic on a value at offset 20 of a structure in a6.
- `copy_index_to_register_alt`: Copies a 16-bit value from d3 to d1 and branches to another routine.
- `copy_bytes_to_table`: Copies bytes from a table at offset 18 of a1 to a destination at a3 using d1 as an index in a loop that continues as long as the extended 32-bit d1 is less than the sum of a value at offset 20 of a6 and d3, then adds the value at offset 20 of a6 to d5, clears offset 20 of a6, and branches to another routine.
- `call_library_function_2`: Stores a 32-bit value from d4 into a structure at offset 86 of a2, calls a library function, moves the result from d5 to d0, and restores registers before returning.
- `check_status_bits`: Calls a routine via a pointer in a2, then checks bit 7 and bit 0 of a byte at offset 30 of a2; if bit 7 is clear and bit 0 is clear, it calls another routine via a pointer in a2.
- `modify_and_call_library`: Clears bit 1 of a byte at offset 30 of a2, sets bit 7 of the same byte, calls a library function, and restores a2 before returning.
- `check_and_modify_status`: Checks bit 7 of a byte at offset 30 of a structure in a2, calls a routine via a pointer if the bit is clear, and if a comparison with d2 succeeds, clears bit 1 of that byte and calls another routine.
- `compare_structure_values`: Loads a pointer from offset 4 of the stack into a0, then accesses offsets 70, 60, and 30 of the structure at a0 and offset 30 of the structure at offset 70 of a0 to compare two 32-bit words, returning 1 in d0 if the first is less than the second.
- `set_zero_and_branch`: Sets d0 to zero and branches to another routine.
- `set_d0_to_zero_and_branch`: Sets register d0 to zero and performs a short branch to another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `transfer_structure_data`: Pushes d2 onto the stack, retrieves two pointers from offsets 8 and 12 of the stack, moves a long value from offset 16 of a structure in a7 to d2, copies several values between a0 and a1 using offsets 40, 16, and 48, and calls another routine before returning.
- `modify_clipboard_status_3`: Links a6, saves registers, loads a pointer from offset 8 of a6 into a2 and a pointer from offset 12 of a6 into a3, sets bit 5 of offset 30 of a3, and if the byte at offset 66 of a2 is non-zero, clears bit 1 of offset 30 of a3 before branching.
- `handle_clipboard_request_3`: Sets a byte at offset 66 of a structure in a2, then if a word at offset 64 is 9, it calls another routine and potentially copies a word from a sub-structure at offset 48 into offset 60 of the structure in a2.
- `clear_d3_and_branch_3`: Clears the 16-bit register d3 and performs a short branch to another location.
- `handle_clipboard_request_2`: Checks a pointer in a structure, copies a word from a sub-structure to offset 64 of a2, calls a routine if that word is 9, and then performs several calls to other routines using pointers from a2 and a6.
- `push_a2_and_call_library`: Pushes a2 onto the stack, calls a routine at a fixed address, increments the stack pointer, writes 1 to offset 94 of the structure in a2, and branches to another location.
- `handle_clipboard_request`: Clears d3, compares a word at offset 64 of a structure in a2 to 9, and if equal, calls a routine with a pointer from a2.
- `clear_d3_and_branch_2`: Clears register d3 and branches.
- `update_clipboard_data`: Loads a pointer from offset 112 of a structure in a2, and if not zero, moves a longword from offset 48 of that pointer to offset 60 of the structure in a2, then calls another routine if a word at offset 28 matches nine.
- `process_clipboard_request`: Compares a word at offset 64 of a2 to 2, and if equal, performs several checks on a structure at a4 before calling another routine and updating values at offsets 32 and 44 of a4.
- `compare_clipboard_values_2`: Loads a byte from offset 91 of a structure in a2 into d2, loads a longword from offset 36 of a structure in a4 into d0, adds a longword from offset 44 of the structure in a4 to d0, and if d2 is not lower than d0, moves the longword from offset 36 of a4 to offset 32 of a4.
- `modify_clipboard_offset`: Subtracts a longword at an offset of a4 from a longword at an offset of a2, stores the result at an offset of a4, and then adds that same result back to another offset of a4 before branching.
- `modify_clipboard_status_2`: It clears a longword at offset 32 of a4, sets a longword at offset 48 of a4 to one, decrements a word at offset 110 of a2, calls a routine, and then either clears a word at offset 64 of a2 or enters a loop.
- `modify_clipboard_data`: Compares a word at offset 64 of a structure in a2 to 3, and if equal, checks a pointer at offset 112 of a2 and compares a word at offset 28 of that pointer to 3; if both match, it calls another routine and updates several offsets in the structure.
- `copy_status_word`: Moves a longword from an offset of a4 to d2, stores d2 at an offset of a2, calls another routine, and then branches to another location.
- `check_clipboard_status`: Compares a word at offset 28 of a structure in a4 to four and, if equal, writes four to offset 64 of a structure in a2.
- `clear_d3_and_branch`: Clears the word in d3 and branches to another location.
- `modify_clipboard_status`: Clears d3, compares a word at offset 64 of a2 to four, and if equal, compares a longword at offset 90 of a2 to eight; if the latter is greater, it pushes several values onto the stack, calls a routine, and if the result in d0 is not four, increments a longword at offset 90 of a2 and branches to another location.
- `clear_clipboard_status`: Clears a longword at offset 90 of a structure in a2 and branches to another location.
- `clear_and_modify_status`: Clears a long value at offset 90 and a word at offset 110 of a structure in a2, pushes a3 and a4 to the stack, calls another routine, clears a word at offset 64 of a2, and if d3 is not zero, checks bit 5 of a byte at offset 30 of a3 to either clear bit 5 or clear a byte at offset 66 of a2 before returning.
- `compare_and_update_pointer`: Loads a pointer from a structure in a1 into a2, then if the value at offset 0 of that structure is non-zero, compares a value at offset 72 of a2 with a value at offset 72 of a0 and updates d1 based on the result.
- `set_d1_and_branch`: Sets register d1 to zero and branches to another routine.
- `call_library_with_stack_args`: Pushes the values from a2 offset 4, a0, and a1 onto the stack, calls a library function, and restores a2 before returning.
- `write_status_byte`: Writes the byte value minus one to an offset of 31 from a pointer in a0 and calls a routine via a pointer.
- `execute_handler_via_stack`: Pushes d2 onto the stack, loads a value from offset 8 of the stack into d2, pushes d2 onto the stack, calls a routine via a pointer, and returns.
- `update_status_and_call_handler`: Writes a byte value minus one to an offset of 31 from a pointer in a0 and calls another routine via a pointer.
- `update_status_byte_via_pointer`: Retrieves a pointer from the stack, writes a byte value minus one to an offset of 31 from that pointer, and calls another routine via a pointer.
- `update_status_and_call_handler_2`: Writes a byte value minus one to offset 31 of a structure pointed to by a register and calls a routine via a pointer retrieved from the stack.
- `dispatch_handler_calls`: Pushes three values onto the stack and calls three different routines via pointers retrieved from the stack, then restores the stack pointer.
- `update_clipboard_status_2`: Saves a2 and a3, loads pointers from a7, tests a long value at offset 48 of a2, and if zero, updates a pointer at offset 56 of a3, sets bit 4 of offset 30 of a2, and calls another routine if a comparison between d0 and a pointer at offset 48 of a2 passes.
- `execute_clipboard_call`: Pushes a pointer from a2 to the stack, pushes offset 96 of a3 to the stack, calls a routine, pushes a2 and a3 to the stack, calls another routine, adjusts the stack pointer, and restores a2 and a3 from the stack.
- `increment_clipboard_pointer`: Saves a2 and a3, loads pointers from a7, tests a long value at offset 48 of a2, and if zero, increments a word at offset 56 of a3, updates a pointer at offset 48 of a2, and calls another routine if a comparison between d0 and a pointer at offset 48 of a2 passes.
- `execute_clipboard_call_alt`: Pushes a pointer from a2 to the stack, pushes offset 112 of a3 to the stack, calls a routine, pushes a2 and a3 to the stack, calls another routine, adjusts the stack pointer, and restores a2 and a3 from the stack.
- `store_pointer_at_offset_30`: Loads a longword from offset 56 of a structure in a1 and stores it at offset 48 of a structure in a0, then calls a routine via a pointer.
- `store_pointer_at_offset_3c`: Loads a longword from offset 60 of a structure in a1 and stores it at offset 48 of a structure in a0, then calls a routine via a pointer.
- `traverse_clipboard_list_3`: Calls a routine, performs bitwise operations on a byte at an offset from a pointer in a2, and then compares a longword at offset 14 of a structure in a3 with d3.
- `compare_and_branch`: Compares a longword at an offset of a3 to d3, and if they are not equal, branches to another location; otherwise it clears d4 and d2 and branches to another location.
- `process_clipboard_data_3`: Tests d4, and if not zero, pushes a fixed address and a constant onto the stack, calls a routine, loads the result into a3, and if a3 is not zero, calls another routine; otherwise it writes 255 to offset 31 of a2 and calls a third routine.
- `process_clipboard_structure`: Initializes several fields in a structure pointed to by a3, calls four routines via a4, increments a global counter, and performs conditional calls and structure updates based on results.
- `call_library_and_modify`: Pushes two values onto the stack, calls a library function, and if the return value is 512, clears a 16-bit word at offset 94 of a3 and subtracts 512 from a value at offset minus 4 of a6, otherwise it branches to another location.
- `check_and_clear_word`: Tests a word at offset 94 of a structure in a3, checks a long value at offset -4 of a structure in a6, and if both conditions are met, pushes a3 to the stack, calls another routine, and if the result in d0 is not zero, clears a word at offset 94 of a3.
- `process_clipboard_data`: Calls a routine via a pointer in a0, then if the result matches a value from the stack, it clears a word at offset 94 of a3 and performs several conditional calls involving a3, a0, and d3.
- `process_complex_structure`: Increments a long value at offset 24 of a3, increments a word at an offset in a table, stores a3 into a2 at offset 24, and performs several conditional library calls and stack operations involving extended values before returning d2.
- `update_clipboard_status`: Decrements a value at offset 18 of a structure, calls several routines using pointers and stack-passed arguments, and conditionally clears a word at offset 94 of a structure.
- `call_library_and_clear_2`: Pushes d4 and d3 onto the stack, calls a library function, and if d0 equals d4, it clears a 16-bit word at offset 94 of a3, sets d4 to zero, and updates a4 with a value from a pointer at a4 before branching to another location; otherwise it restores the stack and returns.
- `clear_and_branch_3`: Clears a 16-bit word at offset 94 of a3 and branches to another location.
- `call_multiple_routines`: Pushes d3 to the stack and calls a routine, pushes d5 to the stack and calls another routine, pushes d6 to the stack and calls that same second routine, and if a long value at offset 104 of a3 is not zero, pushes that value to the stack and calls the first routine again.
- `update_structure_status`: Calls a routine using a3, then if offset 94 of a3 is non-zero and offset 90 is zero, it performs several calls and eventually sets offsets 20 and 24 of a2 to zero before decrementing a word at a global variable and conditionally calling two more routines.
- `test_and_call_library`: Tests a word at offset 252, and if it is not zero, pushes a pointer from offset 310 onto the stack, calls a library function, pushes a pointer from offset 314 onto the stack, calls another library function, moves a value from offset 314 into d0, cleans the stack, and branches to another location.
- `set_bit_and_return`: Sets bit 3 of a byte at offset 234 of a6 and returns with d0 set to zero.
- `process_clipboard_data_2`: Saves registers, retrieves a pointer from the stack, calls a routine via that pointer, writes the value five to an offset of eight from the pointer, performs a bitwise AND on a byte at offset thirty of the pointer, and then calls another routine via a pointer in a6.
- `call_table_function`: Pushes a2 to the stack, calculates an offset from a word at offset 28 of a2, calls a function from a table at offset 332, calls a library function, and restores registers before returning.
- `check_and_modify_status_2`: Calls a routine via a pointer in a2, checks bits 7 and 6 of a byte at offset 30 of a2, writes a byte to offset 31 of a2, and conditionally calls two other routines if bit 5 is set.
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
- `update_linked_list`: Loads a pointer from offset 4 of the stack into a0, stores a0 at its own address, increments that address by 4, clears a long value at offset 4 of the new address, and stores the new address at offset 8 of that location.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `save_registers_and_call`: Saves registers a2 through a6 on the stack, loads values into a3, a0, and a1 from the stack, loads a6 from a pointer at offset 4, calls a routine at an offset of a6, restores the registers, and returns.
- `store_byte_and_increment_2`: Stores a byte from d0 into the memory location pointed to by a3 and increments a3.

### printer.device

- `branch_to_next_instruction`: Performs a short branch to an instruction at a relative offset.
- `load_and_call`: Loads the value negative 2 in d0 and calls another routine before returning.
- `process_printer_request`: Sets bit 7 of an offset 30 value in a structure pointed to by a1, copies a byte from d2 to offset 31 of that structure, and performs a series of conditional pointer updates and library calls using values from a structure in a3 and a library pointer from a global variable.
- `initialize_printer_pointer`: Saves registers, loads three addresses into a0, a1, and a2, calls a library function with d0 set to 2689, and if the return value in d0 is not zero, updates a pointer at a global location with the value in a1 and calls another library function with a0 at offset thirty-four of the pointer.
- `execute_printer_operations`: Performs several library calls to update values at offsets 10, 14, and 18 of a global variable, then calls a library function to fill a memory range with a loop, and finally executes three more library calls.
- `execute_multiple_library_calls`: Performs three consecutive calls to a library function using pointers from a table, then performs a fourth call to a different library function using a value from a6, and finally branches to another location.
- `call_status_handlers`: Calls three different subroutines using brief branches and then returns.
- `load_and_call_printer`: Loads the value negative 3 in d0 and calls another routine before returning.
- `set_status_bit_zero`: Sets bit 0 of the byte at offset 61 of the address in a6 and returns.
- `clear_status_bit_zero`: Clears bit 0 of the byte at offset 61 of the address in a6 and returns.
- `loop_printer_handler`: Checks if a pointer in a1 is non-zero; if so, it calls a routine via a branch and then branches back to the start of this routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_printer_routine`: Performs a short branch to another routine after being triggered by a handler pointer in a data table.
- `update_printer_state`: Performs several library calls to update various offsets in a structure in a6, including clearing a 32-bit value at offset 4 of a linked list and setting bits in d7 based on results from previous calls.
- `clear_bit_on_match`: Loads an address into a1 from an offset of a6, compares d0 to a1, and if they are equal, clears bit 1 of a byte at offset 2454 of a6 before branching to another location; otherwise it branches to another location.
- `set_printer_flag_and_call`: Checks bits in d5 and d2, then if a pointer in a6 plus offset 72 is not null, sets bit 5 of the byte at offset 30 of that pointer, stores the pointer into offset 8 of a structure in d1, and calls a routine at an offset from a table determined by a value at offset 28 of the pointer multiplied by 4.
- `call_subroutine_via_stack`: Loads a pointer from offset 4 of the stack into a0, loads a longword from offset 8 of the stack into d0, and calls a subroutine via bsr.
- `set_printer_bit`: Calls a subroutine, then if the result is zero, sets bit 0 of a byte at offset 2486 of a6 and conditionally calls another subroutine after loading a structure into a1.
- `adjust_stack_and_branch`: Increments the stack pointer by 8 and branches to another location.
- `set_bit_and_branch`: Sets bit 0 of a byte at offset 158 of a6, loads the address of a byte at offset 190 of a6 into a1, and branches to another location.
- `check_status_byte`: Loads a pointer from offset 8, checks a byte at offset 31 of that pointer, and if it is not zero, branches to another routine; otherwise, it performs bitwise operations on a byte from offset 950 of a6 and returns if a specific condition is met.
- `execute_library_calls_8`: Calls four different library routines using a pointer from a structure, sets bit 0 of a byte at offset 327 of a6, and if a byte at offset 332 of a6 points to a non-zero value, clears bit 0 of a global variable at offset 2486 of a6 before calling another routine.
- `compare_and_branch_status`: Compares a value in d0 with an offset of a6, and if they are not equal, clears bit 1 of an offset of a6, calls another routine, and branches.
- `compare_address_and_call`: Loads a longword from an offset of two hundred seventy-two from a6 into a1, compares it to d0, and if they are not equal, calls a subroutine via bsr and then tests d0.
- `call_and_branch`: Calls a subroutine via bsr and then branches to another location.
- `initialize_structure_fields`: Loads the address at offset 272 of a6 into a1, writes 9 to offset 28 of a1, copies a pointer from offset 4 of a structure into offset 4 of a1, and clears the longword at offset 36 of a1.
- `call_offset_function`: Clears a byte at offset 30 of a1, then calls a routine at an offset determined by the value at offset 20 of a1.
- `compare_byte_at_offset`: Loads a pointer into a1 from offset 272 of a6 and returns if the byte at offset 8 of that pointer is not equal to 7; otherwise, it returns.
- `execute_indirect_calls`: Calls a routine at an offset determined by the value at offset 20 of a1, then calls another routine at an offset determined by a value from a global variable.
- `check_status_byte_2`: Saves a6 to the stack, loads a pointer from an offset of eight from the stack, reads a byte from offset thirty-one of that pointer, and if it is non-zero, performs a bitwise AND on a byte from an offset of six from a6 and branches if the result is non-zero.
- `execute_printer_library_calls`: Calls a subroutine, performs several library calls using a pointer from a global table, sets a bit in d1, calls another library function, and if a comparison between d0 and a value at offset 108 of a6 succeeds, clears bit 0 of a longword at offset 2486 of a6 and calls another subroutine.
- `compare_and_set_bit`: Compares a value in d0 with an address calculated from a6, and if they are not equal, clears bit 1 of offset 950 of a6 and calls another routine before branching.
- `branch_on_pointer_comparison`: Compares the value in d0 with the address in a1, which is offset 272 of a6, and if they are equal, calls another routine; otherwise, it branches to another location.
- `call_and_branch_2`: Calls a routine and then branches to another location.
- `branch_on_byte_comparison`: Saves registers, compares a byte at offset 2459 of a6 to 1, and if not equal, branches to a location after pushing a value onto the stack.
- `toggle_interrupt_and_update_counter`: Enables interrupts by setting a bit in the hardware register, increments a byte in a structure pointed to by a global variable, modifies a byte at an offset in a custom data block, and conditionally calls another routine or modifies a global variable based on a comparison of a value in a structure.
- `execute_conditional_library_calls_2`: Calls a routine using a pointer from offset 24 of d1, then restores registers and checks d0; if d0 is zero, it clears bit 0 of offset 2486 of a structure, and depending on subsequent checks, calls another routine with a pointer at offset 108 or 190 of a6.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `push_and_call_handler`: Pushes two copies of a1 onto the stack, calls a routine at an offset, adjusts the stack pointer, loads a new pointer from the stack into a1, and calls another routine before returning.
- `handle_printer_request`: Pushes a pointer to the stack, clears a longword at offset 32 of that pointer, calls three different routines, and restores the pointer from the stack before returning.
- `dispatch_printer_pointer`: Pushes a pointer from a1 onto the stack twice, calls a routine via a pointer at offset 12, restores the stack, and calls another routine via a branch and link before returning.
- `process_printer_request_2`: Pushes two copies of a1 onto the stack, calls a routine, restores a1 from the stack, calls another routine, and returns.
- `execute_complex_sequence`: Calls a routine at an offset from a structure, then if d0 is zero, populates a structure at offset 272 of a structure with values from the stack and performs three library calls using pointers from a global variable.
- `perform_division_loop`: If d2 is not greater than 65535, it performs a series of 16-bit divisions and swaps to calculate values, then enters a loop using a dbra instruction that subtracts a value from d0 and increments d1 until d0 is no longer greater than a1.
- `calculate_offset_address`: Performs multiple signed multiplications using d0, d1, and d2, calculates a sum, and uses the result to add an offset to a pointer in a0, returning the resulting address in d0.
- `push_registers_and_call`: Pushes d2 onto the stack, moves d1 to d2, moves d0 to d1, calls another routine, and then restores d2 from the stack before returning.
- `push_registers_and_call_modified`: Pushes d2 onto the stack, moves d1 to d2, moves d0 to d1, calls another routine, moves d1 to d0, restores d2 from the stack, and then returns.
- `calculate_and_call_negative`: Pushes d2 onto the stack, moves d1 to d2, and if d2 is negative, negates it; it then checks d1, and if d1 is negative, negates it and performs a bitwise NOT on d0 before calling another routine and returning.
- `calculate_and_call_complex`: Pushes d2 onto the stack, moves d0 to a0, moves d1 to d2, and if d2 is negative, negates it and performs a bitwise NOT on d0 before calling another routine and returning with a modified d0.
- `check_printer_status`: Links a6, saves registers, loads a pointer from offset 8 of a6 into a2, and calls a routine with a3 and a4 as arguments, then branches based on the result in d0.
- `clear_printer_buffer`: Compares a longword from a structure in a2 to negative one, and if they are equal, clears a longword at a specific address and another longword at a different address.
- `update_printer_address`: Moves a longword from a pointer in a3 to a fixed address and clears a word at a fixed address.
- `transfer_printer_data`: Copies bytes from a source buffer to a destination buffer using a loop that runs until a counter reaches 256, then calls several routines to handle the data transfer and status updates.
- `zero_registers_and_branch`: Clears registers d4, d6, and d2 to zero and branches to another routine.
- `process_printer_data_stream`: Iterates through a data buffer by comparing an index against a limit, calls a routine to process elements, and updates a counter or structure based on the result of those calls.
- `copy_data_loop_2`: Copies a byte from one memory location to another within a loop, then checks d2 to decide whether to store a longword from d5 into a structure at offset 32 of a pointer in a2 or branch to another routine.
- `clear_memory_and_unlock`: Clears a longword at offset 32 of a pointer in a2, sets d0 to a byte from d2, and unlocks a resource using a pointer from a6.
- `update_character_data`: Saves registers, calls a routine, and then performs conditional checks on a byte from a3 to decide whether to call another routine or update a value at a specific memory location.
- `clear_and_branch_register`: Clears a longword at offset minus four of a6 and branches to another location.
- `update_byte_from_table`: Compares a byte from a3 with a byte from a table at offset 0 of a0, and if they match and a global variable is zero and a byte at offset 2486 of a6 is two, it adds 55 to a byte from a4 and stores it in a2.
- `compare_and_set_byte`: Compares a value in d0 to one and then to twelve, and if both conditions are met, stores the byte 75 at the address in a2, otherwise it branches to another location.
- `set_constant_byte`: Compares d0 to two, and if it matches, compares d0 to d5 and if d0 equals twelve, stores the byte 75 at the address in a2, otherwise it branches.
- `conditionally_set_byte`: Compares d5 to nine and checks a byte at a pointer in a4; if d5 is not nine or the byte in a4 is not zero, it branches, otherwise it stores a byte into a location calculated from a2 and branches.
- `store_byte_from_table`: Compares a long value in d5 to six and, if d5 is greater than or equal to six, stores a byte from a table at offset d5 into the memory location pointed to by a2.
- `conditional_byte_store`: Checks if d5 is non-zero and if a byte at a pointer in a4 is at least 29, otherwise it branches; if the condition is met, it stores a byte into a memory location at a pointer in a2 and branches to another routine.
- `update_character_table`: Performs a loop that compares a byte from a4 with a value from a table, and if they match, extends a byte from a structure and calls a routine via a pointer, while also updating several global variables and a counter in d6.
- `clear_printer_memory`: Clears several longwords at specific offsets within various memory locations and a structure, then branches to another routine.
- `process_character_table_2`: Increments a longword at a location pointed to by a0, fetches a byte from a table using the incremented value and stores it in a3, calls another routine if the byte is 27, and enters a loop that compares a byte from a table against a value in d5, potentially calling another routine with an extended longword and updating a pointer.
- `process_character_table`: It compares a byte at a3 to 40, and if equal, calls a routine, updates a byte at a3, and enters a loop that compares a value against a table and increments a byte at a4.
- `check_character_constant`: Compares a byte at the address in a3 to 91 and branches to another location if they are not equal.
- `process_printer_sequence`: Compares a byte from a table with a byte at a3, and if they match, copies a byte from another table to a4, clears three global bytes, and calls a routine via a pointer in a loop.
- `set_buffer_byte`: Compares a byte at the address in a3 to 10, and if not equal, checks a longword at offset 104 and branches accordingly; otherwise it writes the value 3 to the address in a4 and branches to another location.
- `handle_printer_buffer`: Writes a byte to a4, calls a routine via a pointer, and if a global byte at offset 164 is zero, performs several operations involving a pointer in a2 and updates global longwords.
- `write_two_bytes_and_branch`: Writes the value 10 to a memory location calculated from a pointer in a2 and an offset in d2, increments the pointer in a2, then writes the value 13 to the next calculated memory location, increments the pointer again, and branches to another routine.
- `write_single_byte_and_branch`: Writes the value 10 to a memory location calculated from a pointer in a2 and an offset in d2, increments the pointer in a2, and branches to another routine.
- `dispatch_character_event`: Compares a byte at a pointer in a3 to 155 and, if equal, pushes two longwords onto the stack and calls another routine.
- `calculate_pointer_offset_2`: Compares a byte at a3 to 160, and if it is not less, loads a pointer from a structure, checks a word at offset 8 of a sub-structure, and if it is not less than 33, calculates a new address using a value from a3 and a pointer from a global variable.
- `validate_ascii_range`: Checks if a byte at a pointer is a specific character, then verifies if the following byte is a numeric character within the range of 0 to 9, and calls another routine if the count of valid characters reaches three.
- `increment_and_update_pointer`: Increments a1 and a global variable, then checks a byte at a1; if it is zero, it performs arithmetic on a pointer from a2 and potentially updates a global variable and a pointer at offset 112.
- `calculate_pointer_offset`: Loads a byte from a3 into d1, subtracts 160 from d1, shifts it left by two, adds d4 to the result, and loads a pointer from the resulting address into a1.
- `skip_non_numeric_characters`: Checks if a byte at a pointer in a1 is equal to ninety-two; if not, it increments the pointer and checks if the new byte is between forty-eight and fifty-seven inclusive, branching to the end if it is or continuing if it is not.
- `convert_ascii_to_byte`: Checks if a byte at a pointer is a numeric character, performs a mathematical calculation to transform it, and stores the resulting byte at an offset from a pointer in a2.
- `move_byte_with_offset`: Moves a byte from a pointer in a1 to a destination address calculated by adding a2 and a pointer from a2, then branches based on the value at a1.
- `process_printer_byte`: Loads a longword from a pointer in a2 into d1, increments the pointer in a2, moves a byte from a3 at the address in a0 into memory, and performs several comparisons and branches based on values at various global offsets and d3.
- `modify_memory_via_offsets`: Subtracts a longword at offset 274 from d3, subtracts that result from a longword at offset 266, clears a longword at offset 284, and moves a longword from a2 into d0.
- `call_routine_with_extended_byte`: Clears a global variable, extends a byte from offset 11 of a6 to a word, and calls a routine via a pointer at offset 0 of a6 using that word as an argument.
- `update_pointer_and_call`: Adds d3 to a long value from a global variable, compares the result to two hundred fifty-six, and if the result is greater than two hundred fifty-six, stores d3 in a global variable and either calls a routine or branches based on a bit in a global variable.
- `update_global_pointer`: Decrements a 32-bit value from a global variable, stores the result in another global variable, and branches to another routine.
- `set_d4_and_branch`: Sets register d4 to zero and branches to another routine.
- `copy_byte_loop`: Increments a pointer in a global variable, adds a value from d2 to the pointer, and enters a loop that copies a byte from a stack-relative address to the destination pointer until a counter in d4 reaches the value in d3.
- `clear_and_branch_5`: Saves registers to the stack, loads several values from the stack, clears a byte at the address in a2, and branches to another routine.
- `clear_byte_on_match`: Compares d1 to fifty-nine; if it matches, it increments a3, uses the result to calculate a destination address via a2, clears a byte at that address, and branches.
- `set_flag_on_match`: Compares a byte in d1 to thirty-two; if it matches, it loads a pointer from offset twenty in the stack into a0, sets d0 to one, and stores d0 at the location pointed to by a0.
- `check_byte_and_load`: Compares a byte in d1 to 34, and if it matches, loads a pointer from a stack offset into a0 and sets d0 to two before branching to another routine.
- `update_and_call_table`: Calculates an address by adding a2 and a3, computes a value based on d1 and a byte from a1 plus a2, stores that value at the calculated address, calls a routine via a pointer, and if the resulting byte in d0 is within specific ranges, updates a global pointer with a3.
- `process_table_and_call`: Loads a pointer and a long value from the stack, increments the value at the pointer, fetches a byte from a table using the incremented value, compares the pointer value to d1, and if the pointer value is less than d1, compares a global variable to two hundred fifty-six and calls a routine if it is not less than two hundred fifty-six.
- `transfer_byte`: Moves a byte from d0 to d1, clears d0, and then moves the value from d1 back into d0 before returning.
- `decrement_global_pointer`: Loads a constant into a1, decrements a 32-bit value from a global variable, and stores the result back into that global variable before branching to another routine.
- `check_and_set_flag`: Decrements a pointer in a1, and if the resulting value is not less than or equal to zero, it performs a loop that checks if a byte from a global table, masked with 127, equals 27; if the condition is met, it sets a byte at a global variable to 1 and returns.
- `initialize_and_call_subroutines_2`: Clears three bytes at specific offsets, clears a byte at offset 2720 of a structure pointed to by a pointer in d1, sets a value at offset 58 of a structure pointed to by a pointer in d1, and calls two different routines based on the results of tests on d1 and a pointer at offset 12 of d3.
- `copy_data_to_table`: Copies six 16-bit words from offsets 2646 through 2666 of a structure pointed to by a0 into a table starting at offset 0 of d3.
- `compare_table_values`: Compares six 16-bit words from a structure pointed to by a0 against a table starting at offset 0 of d3, returning 0 if all match and -1 otherwise.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `call_via_pointer_with_stack_data`: Loads a pointer from offset 8 of a6, stores four bytes from offset 34 of that pointer onto the stack, sets d2 to a word from offset 32 of that pointer, and calls a routine via a pointer at offset 0 of a6.
- `execute_conditional_sequence`: If d2 is zero, it calls a routine via a pointer, then if d1 is zero, it calls another routine, and if both d1 and d2 are zero, it calls a third routine via a pointer before returning.
- `execute_library_call_4`: Saves registers, pushes several pointers onto the stack, calls a routine via a pointer retrieved from a global variable, and moves the result into d1.
- `update_status_byte`: Compares a word at offset twenty-two of the stack to 54 and 53, and if the word is 53, clears d1, sets it to the value at offset twenty-two of the stack, performs a bitwise AND with one, stores the result in a global variable at offset two hundred sixty-two, and branches.
- `load_offset_pointer`: Compares a word at offset 22 of the stack and a byte at a global variable to specific values, then calculates an address using a word from the stack and a pointer from a global variable to load a long into d4.
- `branch_on_byte_value`: Compares the byte in d1 to 255 and branches to another routine if they are not equal; otherwise, it sets d1 to 255 and branches to another location.
- `copy_bytes_in_loop_2`: Compares d1 to 254 and, if equal, enters a loop that clears d1, increments d3, and copies bytes from memory locations calculated using d2, d3, and d4 until d1 is non-zero.
- `dispatch_jump_table`: Pushes d2 to the stack, compares a value from the stack to 7, and if it is not less than 7, uses a value from a table to jump to a new address.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `update_printer_status_3`: Copies several word and long values from a structure in a3 to various memory locations, then checks a bit in a word from a structure in a2 to decide whether to clear a word at a specific location.
- `update_printer_status_2`: Writes a word to a location, pushes two values onto the stack, calls a routine via a pointer, and then checks bit 0 of a byte at an offset from a pointer in a5 before branching.
- `update_printer_status`: Checks bits in a structure at a negative offset from a5, performs several conditional checks and writes bytes to various offsets in a structure, and calculates a value by shifting a word from a structure left by 3 bits.
- `calculate_printer_offsets_2`: Subtracts a word at offset 16 of a1 from a word at offset 20 of a1, adds 1, and stores the result in a longword at an address in d7; it then repeats a similar subtraction using offsets 18 and 22 of a1 to store a result at another address in d7.
- `check_printer_offsets`: Loads a value from a pointer in d7, shifts it left by 3, and stores it in a global variable; it then performs several conditional checks on values from a2 and a7 before potentially clearing the stack and branching.
- `calculate_printer_offset_3`: Calculates a difference between two words from memory locations, calls a routine via a pointer, and then performs conditional branches based on the result and a comparison of a word from a second memory location against 1024.
- `compare_and_branch_word_2`: Compares a word in d0 with 32768 and branches if not equal, otherwise it performs a relative branch.
- `branch_with_constant_3`: Loads the constant 12 into d1 and then branches to another routine.
- `branch_with_constant_4`: Loads the constant 15 into d1 and then branches to another routine.
- `calculate_printer_offset_2`: Calls a routine with d3 in d0, then compares a longword from a structure in a2 with d3, and if d3 is larger, performs a multiplication using a word from a structure in a0 and a value in d2 before storing the result in d3.
- `update_printer_state_5`: Calls a routine, compares values in a structure, and if certain conditions are met, performs multiplications, calls another routine, and stores a result at an offset in a6.
- `modify_printer_offset`: Sets d6 to 6 and d0 to 7, stores d0 at offset -4 of a6, checks a byte at offset 168, and if non-zero, adds d6 to d6; otherwise, it adds the value at offset -4 of a6 to d0 and stores the result at offset -4 of a6.
- `prepare_printer_request`: Clears a byte at offset 16c, checks if a byte at offset 15e is zero and if a byte at offset 166 is zero, then clears the stack and pushes a longword to the stack before branching.
- `update_printer_state_4`: Writes a byte to a memory location, performs several arithmetic operations to update values at various offsets in memory, and uses bitwise operations to update a value at offset 6650 of a table before branching.
- `update_printer_state_3`: Performs several additions and stores the results in a table, then conditionally calls two different routines and updates values in a structure pointed to by a4 based on bits checked in a table.
- `update_printer_offsets_2`: Tests a bit in a register, performs shifts and divisions on values from a4, calls another routine, and updates a value in a4.
- `update_printer_offsets`: Performs several calculations involving bit tests and shifts, calls another routine multiple times to retrieve values from a table or global variables, and updates a value at a location pointed to by a register.
- `calculate_printer_offsets`: Performs several calls to a routine using values from a4 and a6, compares d0 and d2, and if d0 is greater than d2, performs more calls and eventually stores a value into a location pointed to by a4 before branching.
- `transfer_printer_data_2`: Loads a word from a location pointed to by a pointer in d7, compares it to a value in a0, and if not equal, stores a word from a pointer in d4 into a table and another word into a different table before branching.
- `calculate_printer_offset`: Copies a word from a table to another table, calculates a difference between two words, stores the result in a global variable, and if a comparison with a value in a4 fails, it updates two words in a table and branches.
- `calculate_buffer_difference`: Calculates the difference between two words from a table, stores the result in a global variable, and enters a loop that clears entries in a table at offset 0 of a pointer in a2 based on a counter up to 64, otherwise it sets a value in a table and branches.
- `process_device_data`: Extends d2, pushes d2 and a value from offset 36 of a3 to the stack, calls a routine, and uses the result to update bytes in a table at offset 10 of a2 using various bit shifts and masks in a loop.
- `execute_device_command`: Writes values to specific memory locations, loads a word from a structure, and calls a routine using a pointer from a global variable, finally storing the result in a specific location.
- `execute_library_call_7`: Pushes a longword onto the stack, clears three longwords on the stack, loads a pointer from a global address, calls a routine at that pointer, and then stores the result in a global longword.
- `test_bytes_and_call`: Tests two bytes at fixed addresses and, if both are zero, calls a routine at a fixed address and branches to another location.
- `test_printer_status`: Tests a byte at a fixed address, and if it is zero, branches to another location; otherwise it calls another routine and branches to another location.
- `test_and_call_routine`: Tests a byte at a specific memory location, and if it is non-zero, calls a routine at an address in a pointer and then branches.
- `update_printer_pointer`: Calls a subroutine, stores the result in a longword at a fixed address, and branches if the stored value is non-zero.
- `divide_and_call_subroutine`: Performs a division using a value from a structure pointed to by a6, and if the result is non-zero, calls a subroutine via jsr and stores the result in a location pointed to by a global variable.
- `call_and_restore_registers`: Clears two 32-bit values on the stack, calls a routine at an offset from the program counter, adjusts the stack pointer, restores d2 through d6 and a2 through a5 from the stack, and unlinks a6 before returning.
- `calculate_weighted_value`: Pushes d2 onto the stack, checks a byte at offset 256 of a structure in d7, and if non-zero, performs a series of multiplications and additions using bytes from offsets 8, 9, and 10 of the stack, then compares a value from a table to 1 to perform further arithmetic.
- `select_byte_from_stack`: Checks bit 2 of a byte at an offset from a pointer in a global variable, then compares two bytes from the stack and potentially stores one of them at offset 11 of the stack.
- `calculate_dos_parameters`: Saves registers, checks a byte at an offset of a structure in d7, and if non-zero, performs arithmetic involving multiplication and addition before branching to another location.
- `calculate_and_open_dos_file`: Calculates a value by adding two words from a structure in d7, calls a routine with this value on the stack, multiplies a byte from a structure in d7 by a word from the same structure, adds another value from d7, calls a routine, and if the result in d3 is not 65535, stores a longword from a structure in d7 into a pointer in a2.
- `fetch_table_value_2`: Tests a byte at a pointer in d7, and if non-zero, calculates an index using d3 to load a longword from a table into the address in a2.
- `write_and_subtract_value`: Writes a longword from offset 4 of a6 to the address in a2, tests bit 3 of a byte at offset 21 of a pointer in a0, and if bit 3 is not set, subtracts d2 from 15 and branches.
- `test_and_branch_nibble`: Shifts d0 right by four bits and branches based on whether the resulting value is less than or equal to one or greater than one.
- `compare_and_branch_3`: Compares d0 to two and branches if it is not equal, otherwise it branches to another location.
- `decrement_and_branch_4`: Decrements a word in d0 and branches to one of two locations depending on whether the result is non-zero.
- `store_byte_and_branch`: Stores a byte from d1 into a global variable and branches to another routine.
- `store_byte_and_branch_2`: Stores a byte from d1 into a different global variable and branches to another routine.
- `store_and_call_subroutine`: Stores a byte from d1 into a global variable, pushes a pointer from a2 onto the stack, calls a subroutine via jsr, stores the result in the location pointed to by a2, and branches to another location.
- `update_global_counters`: Loads a value from a table using a scaled index from d3, then performs four separate additions of bytes from specific global locations into four different global longwords.
- `calculate_and_update_pointers`: Calculates a value by multiplying a word from a global variable with another word from a global variable, stores it in a global variable, and then performs four sequential operations where it divides a value by two, adds it to a global variable, and calls another routine.
- `update_device_state`: Clears a byte in a global variable, calls a routine with two words from a3 and a4 on the stack, and if the result is non-zero, calls another routine via a pointer in a0.
- `execute_conditional_calls_2`: Tests bytes at two offsets from d7, and if both are zero, pushes two words from a3 and a4 onto the stack, pushes a pointer from a stack offset, calls a routine via a pointer, and if the result is non-zero, pushes two more words and calls a routine via a pointer in a0.
- `execute_conditional_subroutines`: Pushes two words from a3 and a4 onto the stack, calls another routine, and if the result is not zero, pushes two more words and calls a routine via a pointer in a global variable before branching.
- `execute_multiple_subroutines`: Pushes two 16-bit values from a3 and a4 onto the stack, calls a subroutine via jsr, and if the result is non-zero, performs a second sequence of pushing values and calling another subroutine via jsr.
- `clear_d0_and_restore_registers`: Sets d0 to zero and restores a2 through a4 from the stack before returning.
- `increment_counter_and_call`: Increments a word at a global variable, performs a division using a value from a structure pointed to by a0, and if the result is zero, calls a subroutine via jsr and stores the result in a location pointed to by a2.
- `call_and_store_result`: Pushes the address of a constant onto the stack, clears three 32-bit values on the stack, calls a routine at an offset from a pointer in a0, stores the result in d0 into the location pointed to by a2, and branches based on a condition.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `initialize_printer_addresses`: Saves registers a2 through a4, loads three constant addresses into a2, a4, and a3, writes a word from a longword at offset 380 to a location in a4, and branches.
- `push_printer_parameters`: Pushes two words from a structure pointed to by d7 onto the stack, calls another routine, stores the returned word in a location pointed to by a3, clears a word at offset 66 of the structure in d7, and performs several longword and word transfers between various pointers before branching.
- `update_printer_registers`: Pushes two words from memory onto the stack, calls another routine, and then writes the result into several memory locations and a word at offset 38 of a structure.
- `clear_longword_and_branch`: Clears a longword at a specific memory location and branches to another routine.
- `update_printer_state_2`: Increments two counters in a structure, calls two routines, and conditionally stores a value from d0 into a structure at a2.
- `update_printer_counter`: It multiplies a byte from a pointer in d7 by a word from a pointer in d7, adds the result to a word at a pointer in d7, increments a word at a pointer in d7, and calls a routine if a comparison matches.
- `increment_printer_counter`: Adds a word from a structure in a3 to a word in a structure in a4, compares the result in a4 to a constant, and if the result is not below that constant, sets d0 to 0 and returns.
- `clear_printer_counter`: Saves registers a2 through a4, loads three constant addresses into a2, a4, and a3, clears a longword at offset 316, writes a word from offset 380 to a location in a4, and branches.
- `push_parameters_and_call_subroutine`: Pushes two 32-bit values onto the stack and calls another routine, then stores the returned word into a location pointed to by a register, clears a 16-bit value in a global variable, and performs several longword and word moves between global variables and memory locations before branching to another routine.
- `clear_device_pointers`: Clears four longwords at offsets 11c, 120, 124, and 128 of a structure in a6 by writing zero to them, then branches to another location.
- `adjust_device_parameters`: Increments a counter in a structure, calls two routines, and pushes two words from a structure onto the stack before calling another routine.
- `store_result_and_branch`: Calls another routine, stores the result in the address pointed to by a2, and if d0 is zero, it loads the value from a2 into d0 and branches.
- `decrement_printer_timer`: It increments a word at a pointer in d7, decrements a word at a pointer in d7, and if the decremented value is not greater than zero, performs a multiplication and addition before conditionally calling a routine.
- `increment_and_check_printer_counter`: Adds a 16-bit value from a pointer in a3 to a 16-bit value at a pointer in a4, then compares the result against a constant; if the result is greater than or equal to that constant, it clears d0 and restores registers a2 through a4 before returning, otherwise it returns.
- `reset_printer_data`: Saves registers a2 through a4, loads three constant addresses into a2, a4, and a3, clears a longword at offset 340, writes a word from offset 380 to a location in a4, and branches.
- `push_parameters_and_call`: Pushes two words from memory onto the stack, calls another routine, stores the result from d0 into a memory location at a pointer in a3, and branches to another routine.
- `transfer_global_data`: Clears a word at offset 322 of a global structure, copies a longword from offset 302 to 310, copies a longword from offset 16 to 276, copies a word from offset 378 to 314, and branches to another location.
- `update_device_state_2`: Pushes two words from a structure onto the stack, calls another routine, stores the result in a structure at offset 3e, clears a word at offset 3c, and branches to another location.
- `execute_device_logic`: Increments a counter in a structure, calls two routines, and stores d0 into a structure at a2.
- `calculate_and_update_device_counter`: Multiplies a 16-bit value from a global variable by a sign-extended byte from a global variable, adds the result to another global variable, increments a third global variable, and calls another routine if a specific value matches a global variable before storing a word into a structure at a2.
- `adjust_device_counter`: Decrements a word at a location in a3, and if it is higher than zero, increments a word at a location in a4, compares the new value in a4 to a constant, and if it is not below that constant, sets d0 to 0 and returns.
- `initialize_printer_state`: Saves a2 through a4 on the stack, sets a2, a4, and a3 to specific constant addresses, clears d0, writes d0 to offsets 220 and 244, copies a word from offset 380 to offset 336, and branches to another routine.
- `push_parameters_and_call_routine`: Pushes two 16-bit values from global variables onto the stack and calls another routine, then stores the returned word into a location pointed to by a register before adjusting the stack pointer and branching.
- `copy_printer_data`: It clears a word at offset 354, copies three longwords between various offsets, and branches to another location.
- `reset_device_pointers`: Clears four longwords in a structure, calls a routine, and pushes two words from a structure onto the stack before calling another routine.
- `store_result_and_branch_alternative`: Calls another routine, stores the result in the address pointed to by a2, and if d0 is zero, it loads the value from a2 into d0 and branches to a different location.
- `process_counter_and_branch`: Increments a word at offset 42, decrements a word at offset 3e, and if the latter is not greater than zero, performs arithmetic with a byte from offset 5a and a word at offset 3a, then compares a word at offset 38 with the result before calling another routine and storing the result in a pointer.
- `decrement_and_check_counter`: Decrements a word at offset 52 of a structure in a3, and if the result is not higher than zero, increments a word at offset 68 of a structure in a4, compares a word from a4 to a value at offset 300, and either branches or restores registers and returns.
- `adjust_pointer_value`: Adds a scaled word from the stack to a longword at a pointer, then subtracts a different scaled word from that same longword if the result is not negative.
- `compare_extended_values`: Saves registers to the stack, loads several values from the stack, and compares an extended longword value against a value derived from a byte at a global address, branching if a condition is met.
- `compare_masked_bytes`: Performs bitwise AND operations on d2 and d4 with the value 3, uses the results to offset into a table at a global address, compares the byte at that location with d3, and returns a longword based on the comparison result.
- `compare_and_call_subroutine_2`: Pushes d2 onto the stack, loads a value from the stack into d2, calls another routine, and if d0 is less than four, adds ten to d2 and calls a second routine.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `initialize_printer_structure`: Clears a longword at offset 58 of a structure pointed to by a value from a table, calls another routine with a pointer to a stack frame, and then checks a word at offset 2614 of a structure to decide whether to branch or load a new address into a2.
- `fetch_indirect_address`: Shifts d3 left by 2, uses the result to index into a table at address 14 to load a new address into a2, moves that address into d5, and branches to another routine.
- `branch_to_middle_of_routine`: Performs a short relative branch into the middle of another routine.
- `test_byte_and_branch`: Tests a byte at a location pointed to by a2 and branches to another location if it is non-zero, otherwise it branches to another location.
- `set_address_register`: It loads the constant 32 into a4 and branches.
- `process_byte_stream`: Increments a pointer in a2, extends a byte from a4 and a byte from a2 into longwords, pushes them onto the stack, calls another routine, and loops through bytes in a4 until it reaches a zero byte or branches to another location.
- `manage_file_locking`: Checks a value in a structure, calls several routines to manage file locks and directories, and performs a loop to copy bytes from one location to another.
- `call_unlock_and_current_dir`: Pushes d6 and d4 onto the stack, calls a routine at offset 30, calls a routine at offset 1c, and returns with d0 set to negative one.
- `call_unloader_via_pointer`: Checks a value at offset 86 of a structure, and if non-zero, calls a routine at an offset from a second pointer in that structure, then calls another routine using a pointer from offset 86.
- `compare_case_insensitive_bytes`: Compares two bytes from the stack by converting them to lowercase if they are within the range of 'a' to 'z', and sets d0 to one if they match after conversion.
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
- `execute_library_calls_7`: Stores several values into a structure pointed to by d1, sets a pointer in that structure from a PC-relative address, and calls a routine three times using different PC-relative addresses for the arguments, storing each result in the structure.
- `execute_library_calls_4`: Loads two different pointers into a1 and calls a routine at an offset from a6 twice, using values from the stack to manage a6.
- `call_library_functions`: Performs three consecutive calls to a library function using an address retrieved from a pointer in a structure, passing different values from a structure in d1 as arguments.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_printer_sequence`: Calls a function from a table, and if the return value in d0 is zero, it populates a structure at an offset of a global variable and makes four subsequent calls to various library functions and other routines.
- `dispatch_via_table_2`: Saves registers to the stack, loads several values from the stack, and performs a jump to an address retrieved from a table based on a value from the stack.
- `return_zero`: Clears d2 and moves it into d0 before returning.
- `initialize_printer_buffers`: Saves registers, loads several pointers from the stack into a1, a3, and a4, and then performs a conditional branch based on a word at an offset of a1.
- `copy_and_modify_data`: Copies bytes from a table into a destination buffer using a loop that runs up to 19 times, then performs conditional writes to a structure and a memory location based on values at specific offsets.
- `process_data_structure_3`: Checks if a word at offset 2646 of a pointer in a0 is 2048, then performs several memory operations including writing a byte to an offset of a pointer in a1, copying bytes from two different locations to new destinations, and clearing a byte at a location in d4.
- `copy_bytes_from_stack_pointers`: Loads a pointer from the stack, decrements a byte from that location, and stores it in a1; then it increments the stack pointer and stores a byte from the new location into a destination in a0.
- `copy_byte_from_table`: Copies a byte from a table at an offset determined by a register to a destination address calculated from a pointer and a constant, then enters a loop that repeats while a register is greater than a value.
- `write_control_bytes`: Compares a word at a1 to 66, and if equal, writes the byte 253 to an offset of a pointer in a1, clears a byte at a1, and if a word at offset 2646 of a pointer in a1 is 2048, writes 96 to the address in a2.
- `check_and_write_control`: Loads a pointer from a register, compares a word at an offset of that pointer to 1024, and if equal, writes the byte 137 to the address in a0 before branching.
- `write_control_byte`: Stores a byte with the value eighty into the address held in a0 and branches to another routine.
- `write_control_code_sequence`: Reads a byte from a table using an incremented index and writes it to a destination address calculated from a pointer and an offset, then repeats this process up to eight times if a counter allows, otherwise it returns.
- `test_and_write_byte`: Compares a word at a1 to 32; if it matches, it checks a byte at a3, and if that byte is zero, it sets it to one and writes 27 to the word at a1.
- `clear_and_write_word`: Tests a byte at the address in a3; if it is non-zero, it branches away, otherwise it clears that byte, writes the word 30 to a location at offset 30 of a structure in a1, and branches to another routine.
- `set_d0_and_branch`: Sets register d0 to 255 and branches to another routine.
- `update_status_byte_2`: Compares a word at a pointer to 33, and if equal, tests a byte in a3; if that byte is zero, it writes 255 to the address in a3 and writes 29 to the address in a1 before branching.
- `test_and_clear_byte`: Tests a byte in a3; if it is less than or equal to zero, it clears the byte at the address in a3, otherwise it writes the word 28 to the address in a1 and branches to another routine.
- `branch_to_routine`: Performs a short branch to another routine.
- `process_control_codes`: Compares a word at a pointer against several constants to set a byte in a3, and if the word is 2, performs a sequence of three byte writes to an offset from a pointer in a4 using values from a stack offset.
- `write_byte_if_zero`: Tests a word at the address in a1; if it is non-zero, it branches away, otherwise it loads a pointer from a table in d1 and writes the byte 253 to an offset of that pointer.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf:Preferences

- `initialize_preferences`: Calls several library functions to access the current directory and task information, then processes a data stream by copying bytes and performing conditional logic based on specific byte values, eventually calling three routines and returning.
- `execute_library_calls_2`: Loads a pointer from a table into a6, then calls three different routines at specific offsets from that base, with one branch path performing an additional library call using a value from a table and a constant.
- `call_library_offsets`: Loads an address from an offset of a4 into a0 and calls a routine at offset minus 180 from a6, then repeats this process with a routine at offset minus 174 from a6.
- `execute_library_calls_5`: Loads a pointer into a1, calls a routine using a pointer from a6, stores the result in a global variable, and otherwise returns.
- `initialize_system_preferences`: Saves registers, calls several routines, performs a loop to copy bytes from one table to another, updates several longwords in a structure, and calls multiple routines using pointers from the stack.
- `initialize_preference_structure_2`: Saves registers, performs multiple library calls and subroutine calls to initialize various values, copies data between tables using loops, and populates a large structure with specific constants and calculated values before returning.
- `dispatch_preference_data`: Executes a sequence of subroutine calls, bitwise operations on a global variable, and arithmetic involving a table to manage system resources and data.
- `release_multiple_resources`: Calls a routine with a pointer, pushes three extended longwords onto the stack, and calls four different routines using pointers from global variables.
- `call_subroutine_with_parameters`: Saves a longword from the stack, accesses a structure via a pointer in d8, and calls another routine twice with different parameters derived from that structure and the stack.
- `process_data_segments`: Saves registers, checks a word from the stack, and enters a loop that calls a routine using values from a structure and the stack, adjusting pointers and counters until a limit is reached.
- `extract_nibbles_and_call_routine`: If a value from the stack is greater than 4, it extracts two nibbles from a word, pushes them onto the stack, and calls another routine.
- `update_preference_structure`: Updates a structure by copying various 16-bit values from several source tables into specific offsets, and performs conditional updates to other fields based on a value in a table.
- `dispatch_via_jump_table`: Uses a value from a structure to index into a jump table, and if the value is within a specific range, it performs a single byte write and calls another routine.
- `process_preference_data`: Saves a2 to the stack, performs several subroutine calls using values from the stack and a table, and uses bitwise operations to test and process data before returning a pointer in a2.
- `perform_semaphore_and_table_operations`: Calls several routines to manage semaphores and signals, then performs extensive arithmetic using values from multiple tables to calculate and push several longwords onto the stack before calling another routine.
- `process_coordinate_updates`: Performs multiple calculations involving bit shifts and logical operations on values from a structure, calls several subroutines with parameters pushed from the stack, and updates various fields in a structure based on the results.
- `clamp_and_update_coordinates`: Clamps two input bytes to a specific range and updates two 16-bit values in a structure using offsets derived from a table.
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
- `calculate_and_store_index`: Calculates a value using several stack-based inputs and bit shifts, stores the result in a location pointed to by a2, and calls another routine with multiple arguments from the stack.
- `update_structured_data`: Calculates various offsets using bitwise operations, writes values to global variables, and calls several routines to process data from multiple structures.
- `process_nested_tables`: Saves registers, calls a routine, and enters a loop that performs arithmetic on d4 and accesses various offsets in tables pointed to by a2, a3, and a4, calling other routines with values derived from these tables and performing stack operations.
- `find_null_byte_offset`: Calculates the distance between a pointer in a1 and a specific byte in memory by incrementing a pointer until a zero byte is found, then returns that distance in d0.
- `call_library_vector`: Pushes a value from a stack offset, a constant, and a pointer from a stack offset onto the stack before calling a routine at an offset of 312 from a pointer in a global variable.
- `calculate_and_call_routines`: Performs a series of arithmetic operations involving multiplication and bit shifts on values from the stack and calls several different routines using a pointer from a table and various stack-passed arguments.
- `process_bitfield_logic`: Tests various bits in a 16-bit value from the stack and, depending on the results, performs complex arithmetic and calls several different routines using a pointer from a table and values derived from the stack.
- `clamp_value_to_nine`: Compares a 16-bit value from the stack to nine; if it is not nine, it returns zero, otherwise it returns eight.
- `decrement_and_clamp`: Decrements a 16-bit value from the stack; if the result is not zero, it returns zero, otherwise it returns sixteen.
- `update_table_via_routines`: Loads a pointer from a table and calls several different routines using various combinations of 16-bit values from the stack as arguments, including several arithmetic operations on those values before the calls.
- `push_values_to_stack`: Tests a word at offset 6 of the stack; if it is non-zero, it pushes four specific 16-bit values onto the stack and branches to another routine.
- `call_table_routines`: Pushes four 16-bit values onto the stack and calls a routine using a pointer from a table.
- `calculate_and_store_offsets`: Calculates various offsets using signed multiplication and division, stores bytes into a destination pointed to by a2, and calls another routine multiple times.
- `perform_division_loop_2`: If a value in d2 is not negative, it performs a division using d1 and d2, then enters a loop that performs repeated additions and comparisons to modify d1 and d0 based on d2.
- `calculate_offset_pointer`: Performs multiple signed multiplications using d0, d1, and d2 to calculate a 32-bit offset, adds this offset to a pointer in d2, and returns the resulting address in d0.
- `push_d2_and_call`: Pushes d2 to the stack, moves d1 to d2 and d0 to d1, calls another routine, and then restores d2 from the stack before returning.
- `push_d2_and_call_with_move`: Pushes d2 to the stack, moves d1 to d2 and d0 to d1, calls another routine, moves d1 to d0, and restores d2 from the stack before returning.
- `handle_signed_division`: Pushes d2 to the stack, performs sign checks on d1 and d2, and calls another routine if certain conditions are met.
- `handle_signed_division_extended`: Pushes d2 to the stack, performs sign checks on d0 and d1, and calls another routine if certain conditions are met.
- `transfer_stack_values`: Saves d2 and d3 to the stack, loads two 16-bit values from the stack into d2 and d3, and branches to another location.
- `calculate_scaled_offset`: Calculates a scaled offset by multiplying a value in d2 by 44 and uses it to access a table via a0, then calls another routine in a loop based on d3.
- `save_registers_and_branch`: Saves registers d2 and d3 onto the stack, loads two 16-bit words from the stack into d2 and d3, and performs a short branch into another routine.
- `calculate_scaled_offset_with_stack_init`: Initializes the stack with a specific value, calculates a scaled offset by multiplying a value in d2 by 44 to access a table via a0, and calls another routine in a loop based on d3.
- `dispatch_via_table`: Loads a value from a global variable and, if it is between 0 and 7, uses it to jump to an address retrieved from a table.
- `calculate_and_store_values`: Loads a value from a table, performs several arithmetic operations including divisions and shifts, calls another routine, and stores the resulting bytes into several locations in a table.
- `process_table_indices_2`: Performs arithmetic on a value from a table, calls another routine multiple times based on conditional checks, and eventually calls another routine with three values from the stack.
- `calculate_table_indices`: Performs a series of divisions and arithmetic operations on four input values to calculate and store five bytes into a table at specific offsets.
- `initialize_system_resources_2`: Calls three different routines with a pointer from the stack, then performs arithmetic to calculate an index into a table and calls a routine using a pointer from that table, followed by two more calls to different routines using values and pointers from the stack.
- `calculate_complex_offsets`: Performs multiple arithmetic operations using signed bytes from various memory locations to update registers, then calls several routines and finally calls another routine with a value derived from subtracting a constant from a register.
- `calculate_table_indices_2`: Updates three bytes in a table based on comparisons, performs arithmetic on several values, and calls another routine if a condition is met.
- `update_string_and_calculate_offset`: Modifies bytes in three different memory locations based on comparisons, performs a series of arithmetic operations involving shifts and subtractions using values from several memory offsets, and ends with a jump to a calculated address.
- `call_subroutine_conditionally`: Compares d3 to 1, and if d3 is greater than 1, sets d3 to 12, decrements d4, and calls another routine if d4 is non-negative.
- `manage_device_and_task`: Performs a series of calls to various routines, including memory allocation and device operations, calculates a new address using arithmetic and shifts, and stores it in a structure at offset 32.
- `manage_system_resources`: Clears two stack slots, calls a routine, and then performs a series of conditional checks and calls to various routines involving memory, semaphores, and device operations, including a call to a routine using a pointer from a global variable.
- `handle_task_pointer`: Calls a subroutine, compares the result in a1 to zero, and if zero, performs several operations including writing a value to a global variable, storing a pointer in a structure, and calling another subroutine with a pointer and a constant.
- `release_resources_and_tasks`: Checks a byte at an offset in a structure, and if a condition is met, calls several routines to release resources, remove tasks, and close a device.
- `send_io_request`: Populates an I/O request structure with a value from the stack, a value from the stack, and a pointer from the stack, then calls a routine to process the request.
- `process_preference_list`: Allocates memory and populates a list by copying bytes from a source, then performs a loop to update a table of values and calls several routines to manage memory and data structures.
- `compare_byte_sequences_2`: Compares two byte sequences pointed to by registers and returns 1 if they match or meet specific criteria, otherwise returns 0.
- `allocate_and_populate_list`: Allocates memory, updates a global pointer to point to the new memory, and enters a loop to copy bytes from a source to the new memory for 30 iterations.
- `free_and_clear_list`: Calculates a size, calls a routine to free memory, and then conditionally frees another memory block and clears a global variable.
- `validate_preference_data`: Iterates through a loop to compare bytes from two tables and calls another routine to validate a sequence, returning 1 if a condition is met or 0 otherwise.
- `validate_byte_array`: Iterates through a sequence of bytes, comparing them against values in a table and checking a condition at a specific offset, returning 1 if all checks pass.
- `update_port_status_2`: Performs several subroutine calls, modifies a global word, and calculates an offset to call another routine based on a value in a structure.
- `dispatch_table_entry_2`: Clears d2, loads a pointer from a global variable, compares a constant to a value at an offset of that pointer, and calls a subroutine if they are equal.
- `dispatch_result_status`: Calls a subroutine, moves the result to d2, pushes a longword from a memory location onto the stack, calls another subroutine, and then branches based on whether the result in d0 is less than 1 or greater than 1.
- `decrement_and_branch_5`: Decrements d0 by 2; if it is not zero, it branches to another location, otherwise it branches to another location.
- `restore_and_branch`: Pushes two long words onto the stack, calls another routine, and then branches to another location.
- `push_and_branch_2`: Pushes two longwords onto the stack and branches to another location.
- `call_external_routine`: Loads a pointer from a2 into a0, pushes a longword from an offset of a0 onto the stack, calls a routine at a fixed address, stores the result in a longword at a fixed address, and branches based on the zero flag.
- `restore_registers`: Returns immediately after restoring d2, a2, and a3 from the stack.
- `copy_word_values`: Copies four 16-bit words from a structure pointed to by a0 to a destination structure pointed to by a1, where the destination address is retrieved from a stack offset.
- `dispatch_via_table_5`: Loads a pointer from a structure, retrieves a value from that pointer, and performs a jump to an address found in a table based on that value.
- `load_and_branch`: Pushes a long word onto the stack, calls another routine, and then branches to another location.
- `restore_registers_2`: Restores registers a2 through a4 from the stack and returns.
- `check_and_call_handler`: Compares values from a table against constants and a global variable, then calls another routine if specific conditions are met.
- `decrement_and_branch`: Decrements a 16-bit value from the stack and branches if the result is not zero; otherwise it clears d0 and branches to another instruction.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `load_long_word`: Loads a 32-bit long word from a stack offset into d0 and returns.
- `update_port_status`: Calls a routine using a pointer from a2, sets a byte at offset 24 of a structure in a0 to 1, performs several calculations involving a table at a3 and writes results back to a0, then calls another routine and sets a byte at offset 24 of a0 to 255.
- `setup_and_clear_register`: Pushes a longword onto the stack, moves a 16-bit word from the stack into d1, clears d2, and branches to another location.
- `compare_and_decrement`: Increments d2, compares a value at an offset in a table with d1, and enters a loop that branches back to an earlier instruction if a condition is met.
- `decrement_d2_and_pop`: Sets d0 to the value of d2, decrements it by 1, and pops a long word from the stack into d2 before returning.
- `copy_and_modify_table`: Clears a long word at a global address, calls a routine, and then enters a loop that copies words from a table to a destination, potentially updating several global pointers and performing bitwise negation on values from a table at a3.
- `call_library_function_3`: Pushes two values onto the stack and calls a routine at a specific address, then cleans up the stack by adding 8 to a7 before returning.
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
- `update_structure_pointer`: Takes a pointer from the stack, sets the value at that address to itself, increments the value at that address by four, clears a longword at an offset of four from that address, and sets a longword at an offset of eight from that address to the original pointer.
- `manage_system_files`: Opens and closes files using several library calls, performs a write operation, and conditionally calls a routine to delete a file.
- `load_and_process_file`: Opens a file, reads its contents, and if successful, calls a routine to allocate memory and then calls another routine to process the data.
- `initialize_task_structure`: Pushes registers to the stack, calls several routines to allocate memory and find tasks, and populates a structure at a pointer in a2 with various values before returning.
- `release_resource_structure`: Checks a value at an offset of ten in a structure; if non-zero, it calls a routine with a pointer to the structure, sets a byte at offset eight to one, and then calls two other routines using values and pointers derived from the structure.
- `process_data_and_update_pointers`: Calls two different routines with specific constants, then conditionally calls a third routine and updates several fields in a structure, including adding a large constant to a pointer.
- `signal_and_delay_handler`: Checks a pointer in a global variable; if non-zero, it sets a bit in a global variable, calls a routine with a pointer from the global variable, and then calls another routine with a value from a global variable and a pointer from the stack; otherwise, it calls a routine with a value from the stack and then calls another routine with a value from the stack.
- `fetch_preference_value`: Saves registers, calls another routine, loads a pointer from a global variable, accesses a structure to retrieve a byte, and calls another routine after shifting that byte left.
- `dispatch_preference_call`: It clears d3, sets d2 to one, compares a longword at offset 20 of a structure in a2 to 16, and if equal, calls a routine via a pointer and updates d2.
- `dispatch_preference_logic_2`: Compares a longword at offset 20 of a structure in a2 with 64 or 32, calls another routine, and if a value in d2 is not zero, pushes a pointer from a2 onto the stack and calls another routine.
- `decrement_and_branch_6`: Decrements a word in d0, branches if it is not zero, otherwise it branches to another location.
- `push_and_branch`: Pushes two longwords onto the stack, calls a routine at an address in a pointer, increments the stack pointer by 8, and branches to another routine.
- `push_constants_and_branch`: Pushes two longwords, 248 and 16, onto the stack and branches to another location.
- `fetch_and_store_message`: Loads a pointer into a0, calls a routine via a pointer, stores the result in a2, and then branches to another location.
- `restore_registers_from_stack`: Restores two 32-bit data registers and two 32-bit address registers from the stack and returns immediately.
- `copy_and_call_structures`: Copies several 16-bit and 8-bit values from one structure to another using a loop, then calls three different routines.
- `process_preference_table`: Performs a series of complex operations including multiple subroutine calls, table lookups, and memory writes to various global and structure locations.
- `dispatch_via_table_3`: Loads several pointers from a structure, performs a comparison on a 16-bit value, and jumps to a routine address retrieved from a table.
- `restore_registers_and_return_2`: Clears d0 and restores d2 through d4 and a2 from the stack before returning.
- `check_and_branch_2`: Saves registers, loads a word from offset 22 of the stack, and compares a byte at a global address to 3; if they are not equal, it branches to another location, otherwise it clears d0 and branches to another routine.
- `process_indexed_data`: Calculates three different indices using bitwise operations, stores them in global variables, and calls another routine three times using data from a table and a third routine.
- `calculate_and_store_offset`: Calculates a value based on three input words from the stack and a constant, stores the result in a memory location pointed to by a calculated address, and calls another routine.
- `process_preference_table_entries`: Reads a value from a stack-passed word, uses it to index into a table to retrieve multiple bytes, and passes these bytes as arguments to another routine multiple times.
- `call_library_with_parameters`: Calls another routine three times, passing a global address and a specific constant as arguments each time.
- `execute_conditional_callbacks_2`: Performs arithmetic on values from the stack to calculate several longwords, then calls various routines multiple times based on conditional checks and loop branches.
- `execute_nested_loops`: Pushes registers to the stack, calls a routine via a pointer, and then enters a loop that performs arithmetic on registers and accesses a table to call another routine multiple times based on conditions.
- `update_memory_with_bitmask_logic`: Performs several bitwise operations and arithmetic on values retrieved from the stack and a table to update specific memory locations.
- `calculate_offsets_and_call_routines`: Performs arithmetic on bytes from two tables to calculate several longwords, then calls another routine multiple times using these calculated values.
- `call_routine_with_argument`: Pushes d2 to the stack, loads a value from the stack into d2, pushes a constant to the stack, calls another routine, and restores d2 from the stack before returning.
- `call_deallocate_resource`: Pushes two longwords from the stack onto the stack, calls a routine to deallocate a resource, and then restores the stack pointer and the first longword into d2 before returning.
- `allocate_and_initialize_block`: If a value in d2 is zero, it calls a routine to allocate memory and, if successful, initializes the new block by writing a byte, a word, and a longword to specific offsets using values from the stack.
- `deallocate_resource_block`: If a value in d3 is non-zero, it sets a byte at offset eight of a structure to one, sets two longwords at offsets twenty and twenty-four to negative one, and calls a routine with a word from offset eighteen and the structure pointer.
- `initialize_preference_resources`: Iterates through a loop to copy bytes from a table, performs multiple subroutine calls using values from a structure, and updates several global pointers.
- `validate_preference_range`: Sets a byte in a global variable, compares a value at an offset of a pointer to 64 and 32, calls a subroutine, pushes a pointer to the stack, calls another subroutine, and branches based on the result in d0.
- `decrement_and_branch_short`: Decrements d0 by 2; if it is not equal to zero, it branches to another location, otherwise it branches to another location.
- `copy_and_validate_data`: Pushes two longwords onto the stack, calls two other routines, and then enters a loop to copy bytes from one table to another based on a counter.
- `copy_table_bytes`: Checks a word in a global variable and, if zero, enters a loop to copy bytes from one table to another and a third table.
- `copy_preference_table_2`: Enters a loop to copy bytes between two tables, and if a counter reaches 30, copies a pointer to a global variable and calls another routine.
- `execute_subroutine_and_store`: Loads a pointer from a2, pushes a value from an offset of that pointer onto the stack, calls a subroutine, stores the result in a4, and branches based on the result.
- `restore_registers_extended`: Restores registers d2 through d4 and a2 through a4 from the stack and returns.
- `copy_preference_table`: Copies several 16-bit and 8-bit values from one structure to another, then enters a loop that copies bytes from one table to another until a counter reaches 30, finally clearing a long value in a global variable.
- `dispatch_preference_logic`: Loads a pointer from a structure, checks if a value at an offset is within a specific range, and then performs a jump to an address calculated from a table.
- `clear_registers_and_return`: Sets register d0 to zero and restores registers d2, d3, a2, a3, and a4 from the stack before returning.
- `initialize_system_resources`: Performs a series of calls to various routines using parameters from a table, including several calls that use values derived from a global table and bit-shifted constants.
- `return_zero_in_d0`: Loads a word from the stack into d0 and then clears d0 before returning.
- `update_preference_value`: Compares a value at a memory location with a global limit, then updates a pointer in a table with either the original value or a new value from the memory location before calling another routine.
- `process_preference_set_a`: Calls three routines to process data, calculates an offset using multiplication, and calls a fourth routine using that offset.
- `update_preference_entry`: Compares a word at a pointer in a0 to 1, performs checks against a global longword, writes values to a global longword at a specific address, and calls another routine.
- `update_preference_table_entry`: Loads a value from a table into a global variable, calls another routine with a pointer, and then calculates an offset by multiplying a value from a global variable by 44 to call a third routine with a pointer to a specific entry in a table.
- `compare_and_call`: Compares d0 to a longword at 2176; if d0 is greater than that value, it writes the word 1 to the address in 194 and calls another routine.
- `initialize_preference_structure`: Loads a value from a global variable into a memory location, calls another routine to perform a task, and then calculates a memory offset by multiplying a value from a global variable by 44 to call a third routine with a pointer to that calculated address.
- `update_configuration_settings`: Executes a sequence of subroutine calls, performs bitwise operations on a byte from a global table, and calculates values to pass to other routines.
- `execute_preference_logic`: Compares a value at an offset of a pointer to 64 and 32, calls a subroutine, pushes a pointer to the stack, calls another subroutine, and branches based on the result in d0.
- `decrement_and_branch_counter`: Decrements a 16-bit value in register d0 and branches to another routine if the result is not zero, otherwise it branches to a third routine.
- `copy_structure_fields`: Pushes two pointers onto the stack and calls a routine to copy a 16-bit word and three 8-bit bytes between them, then clears the d0 register and returns via a branch.
- `push_structure_pointers_and_branch`: Pushes two longwords onto the stack and branches to a location within another routine.
- `update_preference_value_from_message`: Loads a pointer from a structure in a2, pushes a longword from an offset of 86 into the stack, calls another routine, stores the result in a location pointed to by a4, and branches based on the zero flag.
- `restore_registers_and_return_3`: Restores registers d2, a2, a3, and a4 from the stack and returns.
- `copy_specific_bytes`: Copies a 16-bit word and three 8-bit bytes from one structure to another using pointers provided on the stack.
- `dispatch_via_table_7`: Loads a pointer from a global variable, retrieves a word from an offset of that pointer, and uses it to perform a jump via a table if the word is between 94 and 111 decimal.
- `set_d0_and_branch_2`: Sets d0 to 1 and branches to another routine.
- `set_d0_and_branch_3`: Sets d0 to 2 and branches to another routine.
- `branch_immediately`: Performs a short branch to a fixed address.
- `restore_registers_and_return_4`: Restores registers d2, d3, a2, a3, and a4 from the stack and returns.
- `execute_resource_setup`: Calls several routines using a pointer from a structure, performs a calculation using a value from a global table, and calls another routine with the result.
- `update_configuration_byte`: Calls several routines, performs a calculation using a value from a global table to call another routine, and then modifies a byte in a global variable using a bitwise OR operation.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation).adf:Empire

- `branch_to_offset`: Performs a short branch to an offset.
- `update_table_entry_3`: Saves registers to the stack, calculates an offset into a table using a value in d0, compares a longword at an offset of the table with a value at another offset, and if they are not equal, updates the table with the new value and potentially clears a subsequent memory location.
- `transfer_clipboard_data_2`: Saves registers, calls a routine, and then executes two loops using dbra to copy bytes between a structure in a2 and a stack-based buffer, while also performing arithmetic on values from a5.
- `cleanup_and_free_memory`: Calls a routine using a pointer from the stack, then performs a series of conditional calls to another routine using various offsets from a structure pointed to by a4.
- `call_library_functions_2`: Loads an address from a table at offset 92 of a structure in a3 into a0, calls a routine at offset negative 384 of a6, loads the same address into a0 again, and calls a routine at offset negative 372 of a6.
- `execute_library_call_2`: Loads an address into a1, calls a subroutine, and writes the result to an offset of a structure before returning or branching.
- `jump_to_zero_3`: Performs an absolute jump to address zero.
- `call_library_with_arguments`: Saves registers to the stack, loads a library base address from a table into a6, populates a0, a1, a2, a3, d0, d1, d2, and d3 from the stack, calls a routine at offset negative 348 of a6, and restores the registers.
- `calculate_and_store_offset_2`: Sets a word at a negative offset from a4 to zero, checks a value at offset 8 of a5, and if certain conditions are met, multiplies a value from a5 by 6, calculates a new address based on a constant and this product, and then performs a second multiplication and address calculation before branching.
- `process_status_and_call`: Pushes a word from a5 onto the stack, calls another routine, stores the result in a location at a negative offset from a5, and if the result is non-zero, sets d0 to 255 and branches.
- `check_status_bit`: Tests bit 2 of a byte at an offset from a pointer in a0; if the bit is not set, it clears a word at that pointer, otherwise it branches to another location.
- `execute_library_call_with_status`: Clears a word at an offset of a5, calls another routine, and if a condition is met, stores a value in a5 and clears a word at an offset of a structure in a0 before returning.
- `check_device_status`: Sets up a stack frame, calculates a difference between two values from a structure pointed to by a2, stores the result in offset 4 of a5, and if the result is non-zero, pushes three values onto the stack and calls another routine before branching.
- `branch_on_bit_test`: Links a5, loads a pointer from offset 8 of a5, tests bit 1 of a value at offset 13 of that pointer, and if set, calls another routine and stores the result in offset -2 of a5.
- `process_structure_data_5`: Checks bits 2 and 3 of a word at offset 18 of a structure pointed to by a5, and if they are zero and offset 16 is non-zero, calls two different routines using values from offsets 12 and 20 of the structure before storing a result in a5.
- `tiny_moveq_unlk_rts_3`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `update_pointer_from_table`: Loads a pointer from an offset relative to a4 into a0, stores it at offset minus 4 of a5, and if the stored pointer is non-zero, checks a word at offset 18 of that pointer; if that word is non-zero, it replaces the pointer at offset 8 of a5 with the value at offset minus 8 of a5 and then updates the pointer at offset 4 of a5 with a value from the pointer in a0 before branching.
- `test_and_store_longword`: Tests a longword at an offset from a structure in a5, calls another routine, and if the result is non-zero, stores it back into the structure at that same offset.
- `clear_structure_byte`: Clears a range of bytes in a structure pointed to by a0, then pushes three values from a5 onto the stack and calls another routine.
- `process_structure_byte_2`: Loads a pointer from a structure, calls another routine if a word at an offset is non-zero, and then updates a word at offset minus six and a longword at offset minus ten of a structure based on a byte comparison.
- `update_counter_and_call`: Writes 32768 to offset minus 6 of a structure in a5, increments a value at offset minus 10 of a5, compares a byte at offset 1 of a pointer in a0 to 43, and if it matches specific values, enters a loop that calls a routine using parameters from the stack and updates a value at offset minus 4 of a5.
- `test_and_set_field`: Tests a byte at offset negative 11 from a5; if the byte is zero, it branches, otherwise it sets d0 to a constant and branches.
- `set_field_and_branch`: Sets a word at offset negative 2 of a5 to a value composed of 2 and the bit 14 mask, then branches to another location.
- `check_and_initialize_counter`: Tests a byte at offset -11 of a structure in a5 and branches if it is zero, otherwise it sets d0 to 2 and branches to another location.
- `loop_and_set_word`: Calls a routine with a pointer from a5, then enters a loop that increments d0 and stores it at an offset of a5 until d0 reaches 12.
- `test_and_set_flag`: Tests a byte at a negative offset from a5; if the byte is zero, it branches to another routine, otherwise it loads the value 128 into d0 and branches to another routine.
- `set_field_and_branch_2`: Sets a word at offset negative 2 of a5 to 1 and then branches to another location.
- `test_and_initialize_counter`: Tests a byte at an offset of negative 11 from a pointer in a5; if the byte is not zero, it loads the value 2 into d0 and branches to another routine, otherwise it returns.
- `loop_and_set_word_with_mask`: Calls a routine with a pointer from a5, then enters a loop that increments d0 and stores it at an offset of a5 until d0 reaches 12, using a d0 value initialized with several bitwise OR operations.
- `test_and_set_flag_2`: Tests a byte at a negative offset from a5 and branches to another routine if it is zero; otherwise, it loads the value 128 into d0 and branches to another routine.
- `set_field_and_branch_3`: Sets a word at offset negative 2 of a5 to 2 and then branches to another location.
- `clear_d0_and_branch`: Sets d0 to zero and then branches to another location.
- `initialize_structure_fields_2`: Calculates an offset in a0, loads a pointer from offset 16 of a5 into a1, sets several fields in the structure at a1 using values from the stack and a0, and if a word at offset negative 6 of a5 is not zero, it branches to another routine; otherwise, it performs a bitwise OR between 32768 and a word from offset negative 2 of a5 and stores the result at offset 18 of the structure in a1.
- `check_structure_word`: Loads a pointer from offset 8 of a5 into a2 and a word from offset 12 of a5 into d7, then if d7 is non-zero, it continues; otherwise, it sets d0 to zero and branches.
- `modify_structure_fields`: Compares d5 and d4 against d6 and d7, and if certain conditions are met, modifies a word at offset 8 and a longword at offset 4 of a structure in a0, which is retrieved from offset 16 of a structure in a5.
- `update_status_from_call`: Calls a routine, stores the result in a structure at offset minus six of a5, and branches based on whether the result is negative one.
- `copy_byte_and_increment`: Copies a byte from offset negative 6 of a5 to the address in a2, increments a2 by 1, increments d4 by 1, and then branches to another location.
- `increment_d5_and_branch`: Increments d5 by 1 and then branches to another location.
- `move_d5_to_d0`: Moves the contents of d5 into d0, restores d4, d5, d6, d7, and a2 from the stack, unlinks a5, and returns.
- `increment_and_store_byte_2`: Compares two words from a structure, and if they meet certain criteria, reads a byte from a pointer, increments that pointer, and stores the byte into a location specified by a structure offset.
- `loop_and_increment_d0`: Loads a word from offset -6 of a5, masks it, calls another routine, increments d0, and loops if d0 is not zero, otherwise it moves d5 into d0 and branches.
- `increment_d4_and_branch`: Increments d4 by 1 and then branches to another location.
- `increment_data_register_and_branch`: Increments a 16-bit value in data register five and performs an unconditional short branch to another routine.
- `move_d5_to_d0_variant`: Moves the contents of d5 into d0, restores d2, d4, d5, d6, d7, and a2 from the stack, unlinks a5, and returns.
- `load_word_and_call`: Links a5, loads a word from offset 8 of a5 into d0, pushes d0 onto the stack, and calls another routine before returning.
- `check_and_return`: Links a5, saves registers, loads a longword from an offset of a5, and if that longword is greater than zero, sets d0 to zero and branches; otherwise it returns.
- `compare_and_update_pointer_4`: Compares d7 to 8, if less it calculates a new value for d7 using shifts and additions, loads a pointer from offset -430 of a4 into a2 and stores it at offset -8 of a5, then compares a value at offset 4 of a2 with d7 to decide whether to update a pointer at offset -8 of a5.
- `update_structure_pointer_2`: Calculates a value by subtracting d7 from a longword at offset 4 of a2, and if the result is not less than 8, it updates a pointer in a structure using a value from a2 and a calculated address.
- `update_pointer_and_branch`: Stores a2 at an offset of a5, updates a2 with a value from the address pointed to by a2, and then branches.
- `perform_complex_calculations`: Performs several arithmetic operations including additions and bit shifts, calls three different routines, and stores a result in a local stack variable at offset -16 of a5.
- `clear_d0_and_restore`: Sets d0 to zero, restores d6, d7, and a2 from the stack, unlinks a5, and returns.
- `check_ascii_range`: Loads a 16-bit word from an offset of the stack frame and compares it against the range between 48 and 57 inclusive, otherwise it returns.
- `load_and_test_value`: Links a5, pushes a word from offset 8 of a5 onto the stack, calls another routine, and then tests the result in d0, branching if it is non-zero.
- `allocate_and_update_pointer`: Adds 10 to a 32-bit value from the stack, calls a routine via a pointer in a6, and stores the result in a local stack variable before branching.
- `update_list_pointer`: Adds 10 to a 32-bit value from an offset of the stack frame, stores the result in a 16-bit word at an offset of a structure in a0, calls another routine, and then conditionally updates a global variable with a pointer derived from a structure in a5.
- `call_and_store_result_4`: Loads a word from the stack, pushes it onto the stack, calls another routine, and then stores the result in a local stack variable before returning or branching.
- `free_memory_block`: Loads a pointer from a global variable, checks if it is non-zero, and if so, loads a 16-bit word from an offset of a structure pointed to by that pointer to pass as an argument to a call to a routine at an offset of a structure in a6.
- `update_linked_list_structure`: Calculates the difference between two pointers and stores the result in a new structure, then conditionally updates a pointer in a table or another structure based on whether a pointer at an offset is null.
- `process_indexed_value`: Clears a byte at offset minus 1 of a structure and a word at offset minus 32744 of a structure, then enters a loop that compares a word from a structure to a value, performs arithmetic, and checks a value at an offset of a structure before incrementing a counter.
- `compare_and_branch_pointer`: Compares a word from offset minus 6 of a5 with a word from offset minus 6184 of a4, and if they are not equal, branches to another routine; otherwise, it writes 24 to offset minus 608 of a4, sets d0 to 255, and branches to another routine.
- `update_offset_pointer`: Multiplies a word in d1 by 6, adds the result to a pointer based on a4, stores the new pointer at offset -18 of a5, and if certain bits are set in offset 15 of a5, clears a word at offset -10 of a5.
- `modify_structure_field`: Sets a word at offset minus 10 of a5 to 1, performs bitwise operations on a value from a4 to update offset 12 of a5, and if bit 3 of offset 13 of a5 is set, performs further arithmetic and stores the result at offset minus 4 of a5.
- `write_constant_and_branch`: Stores the constant 22 in a 16-bit word at an offset of a structure in a4 and branches to another routine.
- `evaluate_and_call_structure`: Loads a word from offset 12 of a5 into d0, performs bitwise operations, and if certain bits are set, writes a byte to offset minus 1 of a5, pushes two values from a5 onto the stack, calls another routine, and stores d0 at offset minus 14 of a5.
- `execute_conditional_logic_2`: Tests bit 9 of d0 and performs a series of conditional subroutine calls and memory writes to a structure pointed to by a5.
- `execute_library_call_5`: Pushes the word 10061 onto the stack, pushes a pointer from offset 8 of a5 onto the stack, calls another routine, adjusts the stack pointer, stores d0 into a location at offset minus 14 of a5, and then either branches or sets d0 to negative one.
- `write_data_to_pointer`: Writes a 16-bit word from an offset of the stack frame to a location pointed to by another offset, and stores a 32-bit value at a 2-byte offset from that same location.
- `update_scaled_value`: Multiplies a 16-bit value from an offset of a4 by 3677, adds 3, masks the result with 32767, and stores it back to the same offset in a4.
- `store_word_to_offset`: Stores a 16-bit word from an offset of 8 in a5 to an offset of -572 in a4 and returns.
- `call_and_store_pointer`: Pushes a word from a structure in a5 onto the stack, calls another routine, stores the returned value from d0 at a negative offset of a5, and then either sets d0 to negative one or branches.
- `load_and_call_pointer`: Pushes two words from a structure in a5 onto the stack, calls another routine using a pointer from offset minus 4 of a5, restores the stack pointer, stores d0 into a location at offset minus 8 of a5, and then either branches or sets d0 to negative one.
- `load_pointer_from_stack`: Loads a 32-bit value from an offset of the stack frame into d0 and returns.
- `push_and_branch_3`: Sets up a stack frame, moves a word from an offset of a5 into d0, pushes d0 and a value from an offset of a5 onto the stack, calls another routine, and returns.
- `compare_and_branch_d7`: Loads a value from an offset of the stack into d7, then branches if d7 is not greater than zero; otherwise it sets d0 to a negative value and branches to another location.
- `update_pointer_and_value`: Adjusts d7 based on its value, accesses a pointer from a5, and if a comparison succeeds, updates a pointer and a longword in a structure.
- `update_pointer_table`: If a pointer in a2 matches d2, it copies a pointer from a2 to a location at an offset of a3, adds d7 to a value from a2, stores the result at an offset of a3, and finally stores a3 at an offset of a1.
- `subtract_from_memory`: Compares a longword at an offset of a1 with a longword in a0, and if a0 is greater than or equal to a1, subtracts d7 from a longword at an offset of a4.
- `compare_and_subtract`: Compares two long values in a0 and a1, and if they are not equal, branches to another location; otherwise, it checks a long value at a2, and if it is not zero, compares a long value at a2 with d2, and if d2 is not greater than or equal to the value at a2, subtracts d7 from a long value at an offset of a4.
- `modify_structure_data`: Adds the value of d7 to a 32-bit word at an offset of a structure in a2, and if certain conditions are met, updates that structure using data from another structure pointed to by d2.
- `update_stack_and_pointer`: Stores the value of a2 into a stack offset, stores a value from another stack offset into a different stack offset, and then updates a2 with the value pointed to by a2 before branching.
- `clear_and_set_long_value`: Clears a 32-bit memory location pointed to by an offset of the stack frame and then stores the value of d7 into a 4-byte offset of that same location.
- `increment_and_store_byte`: Increments a 16-bit value at an offset of a4, loads a byte from an offset of the stack frame, stores it into a location pointed to by another offset of a4, and then increments that pointer by 1.
- `clear_and_store_data`: Clears a word at a specific offset in a4, moves a value from a5 into a location in a4, pushes a value from a5 onto the stack, calls another routine, and then clears a byte at a location pointed to by a0 before returning.
- `check_ascii_character`: Initializes several stack locations with zero, loads a byte from a pointer in a5, and if that byte is 45 decimal, sets three stack locations to 1 and branches.
- `compare_and_return_pointer`: Checks a byte at a pointer from a structure, then compares a byte from another pointer with a byte from the first pointer, and if they match, moves the second pointer into d0.
- `increment_stack_offset`: Increments a 32-bit value at an offset of the stack frame and branches to another routine.
- `increment_structure_offset`: Increments a 16-bit word at an offset of the stack frame and branches to another routine.
- `return_zero_2`: Sets d0 to zero and returns.
- `call_subroutine_with_stack_data`: Pushes two values from the stack onto the stack and calls another routine.
- `compare_lowercase_characters`: Compares two bytes from two different pointers, converts them to lowercase if they are within the range of 97 to 122, and returns if the resulting values are equal and the second byte is not zero.
- `process_string_bytes`: Checks three memory locations for non-zero values, then performs two separate sequences of incrementing pointers and calling a routine with a byte from those pointers, finally storing a calculated word in a stack location.
- `decrement_structure_offset`: Decrements a 16-bit word at an offset of the stack frame and branches to another routine.
- `test_structure_byte`: Tests a 16-bit word at an offset of a5, and if non-zero, tests a byte at a pointer from a5, setting d0 to 1 if the byte is non-zero.
- `test_byte_and_set_d0`: Loads a pointer from an offset of the stack frame into a0, tests the byte at that address, and if non-zero, sets d0 to 255 before branching to another routine.
- `unlock_address_register_five`: Clears register d0 and unlocks address register a5 before returning.
- `convert_ascii_to_lowercase`: Loads a word from the stack, compares it against 97 and 122, and subtracts 32 from it if it falls within that range before returning.
- `dispatch_pointer_value`: Pushes a word from offset 8 of a5 onto the stack, calls another routine, stores the result in offset minus 6 of a5, and otherwise sets d0 to minus 1 and branches.
- `dispatch_table_entry_3`: Loads a byte from a structure in a5, increments a pointer in that structure, and performs an indirect jump using a value from a table.
- `update_global_byte`: Decrements a value at an offset in a global variable, and if the value is non-negative, copies a byte from a structure into a location pointed to by a global variable.
- `extract_and_branch_data`: Loads a byte from offset minus one of a5 into d0, performs a bitwise AND on d0 with 255, pushes d0 onto the stack, calls another routine, adjusts the stack pointer, and branches to another location.
- `update_structure_byte_from_table`: Decrements a 16-bit value in a global variable, and if the result is non-negative, loads a pointer from a table, increments that table pointer, and writes a byte from a structure at a negative offset relative to a5 into the memory location pointed to by the table pointer.
- `mask_and_call_structure_data`: Loads a byte from the structure at offset negative one relative to a5, masks it to eight bits, pushes it onto the stack, and calls extract_structure_data before branching to another routine.
- `call_and_cleanup_stack`: Pushes a pointer from offset minus 500 of a4 onto the stack, loads 255 into d0, pushes d0 onto the stack, calls another routine, adjusts the stack pointer, loads a word from offset minus four of a5 into d0, unlinks a5, and returns.
- `update_status_bit`: Loads a word from a structure, checks bit 15 to set a byte at offset minus one of a structure, and then checks if other bits are set to either clear a word at offset 8 of the structure or branch elsewhere.
- `test_bits_and_update_structure`: Tests bits 7 and 6 of offset 19 of a2; if both are set, it pushes a2 and a value of negative one onto the stack, calls another routine, adjusts the stack pointer, tests a word at offset 16 of a2, clears a word at offset 8 of a2 if the test fails, and if bit 2 of offset 19 of a2 is set, sets a word at offset 16 of a2 and updates a pointer at offset 12 of a2 using a value from offset 16 of a2 plus 22.
- `call_and_set_bit`: Pushes a2 to the stack, calls another routine, and if the result in d0 is non-zero, sets bit 5 of offset 19 of a2 and branches; otherwise it sets d0 to -1 and branches.
- `update_list_and_byte`: Tests a byte at offset minus 1 of a5, and if not zero, increments a word at offset 8 of a2, potentially increments a pointer at offset 4 of a2, and moves a byte from the new pointer location into d0 and offset minus 4 of a5.
- `call_subroutine_with_a2`: Pushes a2 onto the stack, calls a subroutine, and then branches to another location.
- `set_bit_and_branch_2`: Sets bit 4 of a byte at an offset of a2, sets d0 to 255, and branches to another location.
- `move_stack_to_d0`: Moves a 16-bit word from an offset of the stack frame into d0 and branches to another location.
- `update_status_and_negate`: Tests bit 1 of offset 19 of a2, sets bit 0 if not set, pushes three values from a2 onto the stack, calls another routine, stores d0 at offset minus 6 of a5, and may set bits 4 and 5 of offset 19 of a2 or negate d0 and store it at offset 8 of a2.
- `update_status_word`: Stores d0 in a 16-bit word at an offset of a2, updates a pointer in a2, and if a bitwise AND of a word at an offset of a2 with 50 is non-zero and a byte at an offset of a5 is non-zero, sets a word at an offset of a2 to 255.
- `clear_and_set_d0`: Sets d0 to 0, stores it in a 16-bit word at an offset of a2, sets d0 to 255, and branches to another location.
- `decrement_and_copy_byte`: Decrements a 16-bit word at an offset of a2, and if the value is non-negative, increments a pointer at an offset of a2 and moves a byte from that pointer into d0.
- `call_subroutine_with_registers`: Pushes a2 onto the stack, calls another routine, restores d2 and a2 from the stack, unlinks a5, and returns.
- `extract_structure_data`: Links a5, saves registers, loads a word from offset 12 of a structure in a2 into d1, performs a bitwise AND on d1, stores a word from d0 into offset -8 of a5, and either branches to a location or sets d0 to a constant and branches elsewhere.
- `process_status_and_call_2`: Manipulates d0 based on d1, stores a byte at offset -2 of a5, tests a word at offset 16 of a2, and if bit 2 of d1 is clear and a word at offset 8 of a5 is not 65535, calls another routine and sets bit 5 of offset 19 of a2.
- `set_bit_and_negate_word`: Sets bit 1 of a byte at offset 19 of a structure in a2, checks a byte at offset -2 of a5, and if it is not zero, negates a word from offset 16 of a2 and stores it at offset 10 of a2.
- `decrement_and_store_byte`: Decrements a 16-bit word at an offset of a2, and if non-negative, increments a pointer at an offset of a2 and stores a byte from a structure in a5 at that pointer.
- `push_and_call_routine`: Masks a word from offset 8 of a structure in a5, pushes that word and the current value of a2 onto the stack, calls another routine, and then branches to another routine.
- `check_device_status_2`: Tests bit 2 of a byte at offset 19 of a structure, and if set, compares a word at offset 8 of a structure to 65535 before clearing d0 and branching.
- `dispatch_conditional_subroutine_call`: Checks if a byte in a structure equals ten; if so, it pushes two words onto the stack, pushes an address, and calls another routine before storing the result in a structure offset.
- `dispatch_and_set_flag`: Pushes a word onto the stack, pushes a pointer to offset -1 of a5, pushes a word from offset 20 of a2 onto the stack, calls another routine, stores the result in offset -6 of a5, and sets a word at offset 8 of a5 to 255 before branching.
- `process_device_command`: Sets a bit in a structure in a2, checks a byte at offset -2 of a5, and if valid, performs a series of operations including incrementing pointers, calling multiple routines with various stack parameters, and updating several fields in a5 based on returned values.
- `set_bit_conditionally`: Clears d0, stores it as a word at an offset of a5, compares that word to a constant, and if it is not equal to negative one, sets bit 5 of a byte at an offset of a2.
- `set_bit_and_negate`: Compares a word at an offset of a5 with d1, and if they are equal, branches; otherwise, it sets bit 4 of a byte at an offset of a2, checks a byte at an offset of a5, and if it is zero, copies a word from an offset of a2 to d0, negates it, and stores the result at an offset of a2.
- `test_bit_and_store_zero`: Tests bit 2 of a byte at an offset of a2, and if that bit is not set, it stores zero into a word at an offset of a2 before branching.
- `update_structure_from_byte`: Copies a word from offset 16 of a2 to offset 10 of a2, moves a pointer from offset 12 of a2 to offset 4 of a2, and if a word at offset 8 of a5 is not all ones, decrements the word at offset 10 of a2 and, if it is non-negative, stores a byte from a5 into the memory pointed to by the pointer at offset 4 of a2.
- `process_structure_byte`: Extracts a byte from offset 8 of a structure in a5, pushes a2 and the byte onto the stack, calls another routine, and then checks a bitmask against a byte at offset 18 of a2 to either branch or set d0 to negative one.
- `check_and_branch`: Loads a word from offset minus 8 of a structure in a5 into d1 and branches to another routine if the value is not equal to 65535; otherwise it clears d0 and branches to another routine.
- `restore_and_unlk`: Moves the contents of d1 into d0, restores d2 and a2 from the stack, unlocks a5, and returns.
- `check_structure_bit`: Links a5, loads a pointer from offset 8 of a5 into a0, tests a word at offset 16 of a0, and branches if it is zero or if bit 3 of offset 19 of a0 is set; otherwise it sets d0 to zero and branches.
- `load_and_store_values`: Pushes a word from offset -840 of a4 to the stack, calls a routine, increments the stack pointer, loads a pointer from offset 8 of a5 into a0, stores d0 into offsets 4 and 12 of a0, and if d0 is non-zero branches, otherwise it sets a word at offset -608 of a4 to 12 and sets d0 to 255 before branching.
- `update_structure_fields`: Copies a word from an offset of minus 840 from a4 to offset 16 of a0, performs a bitwise AND on offset 18 of a0 with 40949, and sets offsets 10 and 8 of a0 to zero before unlocking a5 and returning.
- `dispatch_jump_table_2`: Initializes a stack frame, copies several bytes to the stack, and if a byte at a pointer in a0 is not zero, it performs a bitwise AND on that byte and uses the result to jump to an offset within a table.
- `apply_bitwise_or_and_branch`: Performs a bitwise OR on a byte at offset 96 of a4, performs a bitwise OR on a byte at offset 96 of a4 with the value 32, performs a bitwise OR on a byte at offset 96 of a6 and increments a6, then branches to another location.
- `write_byte_and_branch_2`: Writes the value one to offset minus two of a structure in a5 and branches to another location.
- `increment_and_branch_2`: Increments a longword at offset 8 of a structure in a5 and branches to another routine.
- `increment_and_compare_byte`: Increments a longword at offset 12 of a5, loads a byte from a pointer at offset 12 of a5 into d0, stores that byte at offset minus 1 of a5, and then performs several conditional branches based on whether d0 is zero or decimal 37.
- `negate_and_call_routine`: If d0 is negative, it negates d0; if d1 is negative, it negates d1, then calls another routine and negates d1 again.
- `perform_word_division`: Pushes d2 to the stack, swaps the high and low words of d1, and if d1 is not zero, branches to another routine; otherwise it performs two word divisions using d1 and various swapped values in d0 and d2 before restoring d2 from the stack and returning.
- `perform_complex_multiplication`: Moves d2 and d3 into a0 and a1, performs several unsigned multiplications using d0, d1, d2, and d3, and modifies d0 and d2 through additions and swaps before returning.
- `loop_memory_move`: Loads two pointers and a count from the stack, and if the count is not less than or equal to zero, enters a loop that moves bytes from one pointer to another while decrementing the count using a dbra instruction.
- `loop_and_return`: Decrements d0 in a loop to another routine, loads a longword from offset 4 of the stack into d0, and returns.
- `copy_byte_conditionally`: Loads pointers from the stack into a0 and a1, checks a byte at a1, and if non-zero, copies a byte from a0 to a modified a1 address in a loop.
- `setup_memory_pointers`: Loads two pointers from the stack and a word from offset 12 of the stack, then branches to another location.
- `check_memory_and_bit`: Compares d7 to a value from a table, then checks if a word at a specific offset of a2 is zero or if bit 2 of a byte at an offset of a2 is set, branching accordingly.
- `set_constant_and_return`: Checks a value from the stack, and if it is zero, it stores a constant into a memory location at an offset of a pointer in d0 before returning.
- `call_library_function_10`: Sets d1 to 12288, calls a routine at an offset of a6, masks the result in d0 with 12288, stores the result on the stack, and branches based on whether the result is non-zero.
- `call_library_with_mask`: Calls a subroutine, then calls a library function at an offset of -42 from a6, and if the result is negative one, calls another library function at an offset of -132 from a6.
- `call_library_with_offset`: Calls a subroutine, then calls a library function at an offset of -48 from a6, and if the result is negative one, calls another library function at an offset of -132 from a6.
- `process_library_calls`: Calls a subroutine, then calls a library function at an offset of -66 from a6, and if the result is negative one, calls a library function at an offset of -132 from a6, before performing conditional branches based on a word at offset 16 of a5.
- `execute_conditional_library_call`: Calls a subroutine, then calls a library function at an offset of -30 from a6, and if the result is zero, calls a library function at an offset of -132 from a6.
- `restore_and_return`: Moves the value from d7 into d0, restores d7 from the stack, unlocks a5, and returns.
- `call_and_increment_pointer_2`: Calls a subroutine, then calls a library function at an offset of -36 from a6, and returns with d0 set to zero.
- `perform_multiple_library_calls`: Calls a subroutine, then sequentially calls library functions at offsets -84, -90, -72, and -30 from a6, using values from a5, and branches based on the results.
- `load_from_offset`: Loads a longword from a memory location four bytes before a5 into d0, unlocks a5, and returns.
- `request_library_function_with_semaphore`: Checks a flag in a structure, calls another routine, and then attempts to call a library function using a value from the stack; if the library call succeeds, it stores the result on the stack, otherwise it calls a second library function and returns a failure value.
- `process_stack_and_library_call`: Loads a longword from an offset of eight from a5 into d1, calls a routine using a pointer from a4, stores the result in d0 at an offset of negative four from a5, and if d0 is zero, calls another routine using a pointer from a4 and updates two values in a4 before returning.
- `load_pointer_and_unlock_stack`: Loads a longword from an offset of negative four from the address in a5 into d0, unlocks the semaphore in a5, and returns.
- `initialize_and_call_library_2`: Checks a value at offset 19088 of a4, if zero it calls a routine using a pointer from a table, then uses a pointer from offset -32684 of a4 to clear a bit at offset -80 of a structure in a5, and finally calls another routine with several parameters from the stack.
- `clear_d0_and_return`: Clears d0, restores a6 from the stack, unlocks a5, and returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `update_linked_list_2`: Loads a pointer from the stack into a0, copies a0 to the location pointed to by a0, increments the value at that location by 4, clears the word at offset 4 of the structure, and sets the word at offset 8 of the structure to a0 before returning.
- `process_byte_sequence_2`: Links a5, saves d7 and a2 through a3 to the stack, loads a pointer from offset 8 of a5 into a3, compares a word at offset 18644 of a4 with 32, and if it is greater than or equal, branches to another routine; otherwise, it loads a byte from a3 into d1, and if d1 is 32, 9, or 10, it branches to another routine, otherwise if d1 is 10 it increments the value at a3 and branches to another routine.
- `execute_indirect_jump`: Performs an indirect jump using the address stored in the instruction.
- `jump_to_indirect_address_22`: Performs an indirect long jump to an address stored in a pointer located in data memory.
- `initialize_window_structure`: Checks a value from a structure in a5, writes four words to offsets relative to a4, calls a routine using a pointer from a4, and then either calls another routine or returns.
- `manage_window_and_menu`: Compares a 16-bit value from offset 8 of a structure in a5 against 1 and 2, then either calls a routine at offset -72 from a pointer in a6 or calls a library routine at offset -450 from a6 to close windows or clear menus.
- `update_menu_item_address`: Loads a pointer from offset 8 of the structure in a5 into a3, loads a word from offset 12 of the structure in a5 into d0, loads a pointer from offset 19088 of the structure in a4 into a6, and calls a routine at a negative offset from a6.
- `update_menu_bar`: Clears a bit in a structure, then calls two library routines twice if a specific memory location is not zero to update a menu bar using a3 as an argument.
- `update_intuition_item_state`: Calls a routine via a pointer in a table to retrieve an address, then if a specific word value is non-zero, sets bit 4 of a byte at offset 13 of that address and clears a byte at offset 18 of the retrieved structure.
- `execute_library_callback`: Calls a routine at offset -342 from a library base in a6, then calls a routine at offset -354 from the same base.
- `compare_and_call_subroutine`: Compares a 32-bit value from offset 12 of a structure in a5 with a value from a global offset in a4, and if they are equal, pushes several arguments onto the stack and calls a subroutine.
- `load_structure_fields`: Loads a pointer from offset 8 of a5 into a0, copies words from offsets 8 and 10 of a0 into d5 and d4, and if a longword at offset 19108 of a4 is non-zero, loads that address into a3.
- `compare_structure_fields`: Performs several subtractions using bytes and words from a structure in a3 and compares the results against d5 and d4, potentially writing a word to an offset in a5.
- `process_scaled_values_3`: Checks a word at offset 12 of a5, performs several divisions and arithmetic operations involving bytes from a3 and words from a5, and stores various results in offsets -30, -32, -34, -36, and -40 of a5.
- `compute_scaled_quotients`: Divides a longword d3 by 2, adds a word from offset -4 of a5 to the result, performs a second division using d5 and d1, adds the results, performs subtractions using bytes from offsets 55 and 57 of a3, and stores the final value at offset -40 of a5 before branching.
- `calculate_scaled_difference`: Checks if a word at offset minus 34 of a5 is greater than zero, then subtracts bytes from offsets 54 and 56 of a3 from a word at offset 8 of a3, performs divisions and additions using d1, d2, and d3, and finally stores a word in offset minus 40 of a5 before branching.
- `calculate_division_results`: If d0 is not less than or equal to zero, it subtracts bytes from a structure in a3 from words at offsets 8 and 56 of the same structure, performs two signed divisions using d0 and d2, and stores the result in offset minus 40 of a structure in a5.
- `calculate_structure_fields`: Performs division operations using values from a structure in a5 and a global variable, then updates several fields within that structure.
- `process_structure_values`: Adds a word from a global variable to d3, updates several fields in a structure in a5, and calls another routine if a comparison between d0 and d2 succeeds.
- `branch_with_stack_manipulation`: Pushes a word from offset c of the structure in a5 and a long from offset 8 of the structure in a5 onto the stack, calls a routine using a branch and link, adjusts the stack pointer, and branches to another location.
- `calculate_scaled_values`: Divides negated values from a structure in a5 by 8, stores the results back in the structure, and conditionally updates offsets 30 and 32 of the structure using a value from a global table.
- `calculate_and_call_routine_2`: Adds a word from a4 to d2, stores 1 at offset -32 of a5 and the result in offset -30 of a5, compares d2 to 39 and branches if less than or equal; otherwise it performs arithmetic on d1 and d2, and if a comparison matches, pushes two words from a4 onto the stack and calls another routine.
- `push_data_and_branch`: Pushes a word from offset 12 of a5 and a longword from offset 8 of a5 onto the stack, calls a subroutine, and then branches to another routine.
- `calculate_and_process_values`: Performs several arithmetic operations using bytes and words from a structure in a3, checks a value at a negative offset from a5, and if non-zero, pushes several values onto the stack, calls another routine, and conditionally calls a third routine before returning.
- `update_calculated_values`: Performs arithmetic on bytes and words from a structure in a3, calls a library routine, conditionally calls another routine with several values from a5 and a3 on the stack, and updates two words in a5 based on a comparison.
- `compare_and_update_fields`: Performs arithmetic using bytes from a3 and compares the result against a value in d1, then conditionally updates two words at offsets -2 and -6 of a5 and performs further arithmetic and comparisons.
- `compare_and_update_values`: Performs arithmetic on values from a structure in a3, compares the result against a value in a5, and conditionally updates a field in a5 based on several byte tests.
- `perform_arithmetic_loop`: Performs multiple arithmetic operations on values from a structure in a5 and d5, updates several fields in the structure, and contains a loop that branches back to an earlier instruction based on comparisons of d1, d2, and d3.
- `loop_and_call_routine`: Performs arithmetic on d0, d1, d3, and d6, stores results in the structure in a5, and enters a loop that compares values and branches back to an earlier instruction.
- `set_window_bit`: Sets bit 0 of a byte at offset 25 of a structure in a3, loads a pointer from offset 34 of a3 into a0, calls a library routine using a pointer from a global variable, and then either branches to another routine or calls a second library routine using a pointer from a global variable.
- `update_window_state`: Copies two words from a structure in a5 to a new location, calls a library routine using a pointer from a global variable, and then either calls another routine or performs stack operations and calls a third routine based on the results and a value in a5.
- `increment_global_and_return`: Increments a 16-bit value at a fixed offset from a4 and moves the value of a3 into d0 before returning.
- `handle_request_and_event`: Calls a routine using a pointer from a structure in a5, compares a value to a global variable, and calls another routine if they match.
- `update_event_status_and_call_handler`: Decrements a 16-bit value in a global variable and, if it reaches zero, compares two pointers in a structure, then calls a routine via a function pointer and potentially calls another routine based on the result.
- `process_event_and_refresh`: Loads a pointer from offset -8 of a5, calls a routine, and if a comparison with 4 succeeds, calls another routine, compares a pointer in a5 with a pointer in a4, and if a word at offset -25212 of a4 is zero, calls a routine with two arguments on the stack.
- `compare_and_clear_bit`: Compares a longword at offset 19108 of a4 with a0, and if they are equal, calls a routine, loads a pointer from offset -4 of a5 into a0, calls a library routine using a pointer from offset 19088 of a4, calls another routine, clears bit 0 of a byte at offset 25 of a structure in a0, and returns.
- `traverse_and_modify_links`: Follows a chain of pointers starting from a structure in a5, moving longwords and bytes between memory locations, and updates a pointer in the structure.
- `transfer_pointer_data`: Loads a pointer from offset -12 of a structure in a5 into a0, moves a long value from offset 26 of the address in a0 to offset -4 of the structure in a5, and if that value is not zero, writes a byte from d0 to the address in a0.
- `swap_pointer_values`: Loads a pointer from an offset in the structure in a5 into a0, moves the value at the address pointed to by a0 into the offset in the structure in a5, and branches to another location.
- `update_linked_list_node_2`: Loads a pointer from offset 8 of a structure in a5 into a0, moves a long value from offset 24 of the address in a0 to offset -4 of the structure in a5, and if that value is not zero, writes a byte from d0 to the address in a0.
- `update_linked_list_node`: Moves a pointer from a structure into a new location, checks if it is null, and if not, updates a byte in the new structure using a value from the old structure before updating the pointer again.
- `return_immediately`: Unlocks a5 and returns immediately.
- `process_indexed_memory_block`: Checks a global pointer, clears a word in a structure in a5, and if a specific condition is met, calls a library routine and clears a longword at a calculated offset.
- `jump_to_address_21`: Performs a longword jump to a specific address.
- `jump_to_indirect_address_11`: Performs a long jump to an address stored in a pointer.
- `jump_to_address_17`: Performs a long jump to an address.
- `call_library_offset_38`: Calls a library function at offset 38 from a library base loaded from a global variable.
- `call_library_offset_0`: Calls a library function at offset 0 from a library base loaded from a global variable.
- `jump_to_external_routine`: Performs a long jump to a routine located at a fixed address.
- `jump_to_address_5`: Jumps to an address stored at a specific location.
- `jump_to_address_32`: Performs a long jump to the address stored at a specific location.
- `process_negative_value`: If a 16-bit value at offset 8 of a5 is negative, it stores 45 at a location calculated from a4 and a local variable, negates the value at offset 8 of a5, calls another routine with a pointer to a local variable and a pointer to a location in a4, then clears a byte at a location calculated from a4 and a local variable before returning; otherwise it returns.
- `divide_and_store_byte`: Divides a 16-bit value from offset 12 of the structure in a5 by 10, calls another routine with three arguments if the result is not zero, and then stores a byte calculated from a division of offset 12 by 10 into a table at an index derived from offset 14.
- `manage_message_port`: Checks a byte at an offset of a4, calls another routine, checks a word at another offset of a4, and if non-zero, uses a pointer from offset 18138 of a4 to call a library routine via a pointer at offset 4 of a6, then stores d0 at offset -4 of a5.
- `push_and_call`: Pushes a 32-bit value from an offset of a5 onto the stack, calls another routine, adjusts the stack pointer, and branches to another routine.
- `loop_and_clear_memory`: Enters a loop that calls another routine and clears a longword at a calculated address based on a value at offset -6 of the structure in a5, decrementing that value until it is no longer less than 5.
- `close_device_and_cleanup`: Checks a longword at an offset of a4, calls a routine at an offset of a6, and then calls a routine at an offset of a4 before restoring registers and returning.
- `dispatch_indexed_jump`: Calculates an index by masking a value from a global variable, calls another routine, and then performs a jump to an address calculated from a table using a value from a structure in a5.
- `push_registers_and_call_2`: Pushes d2 and several other values onto the stack, calls another routine, and then restores d2 from the stack before returning.
- `update_structure_and_copy`: Performs multiple operations including copying 16 bytes from a table, writing specific bytes and words to a structure, calling a routine with a pointer to a structure, and performing a loop using a dbra instruction to copy data.
- `copy_and_update_structure`: Extends a byte from an offset of a4 to a longword, calls another routine, copies 16 bytes from a source at an offset of a4 to a destination at a pointer in d0, writes several values to a structure pointed to by d0, calls another routine, and updates two bytes at an offset of a4.
- `fetch_message_and_store`: Sets up a stack frame, pushes a6 to the stack, loads a pointer from an offset of a4 into a0, loads a pointer into a6, calls another routine, stores the result in a location at an offset of a5, and branches to another routine if the stored value is not zero.
- `free_memory_and_decrement_counter`: Calls a library routine to free memory using a pointer from a structure at an offset of a5, and decrements a global variable at an offset of a4 before returning.
- `jump_to_address_16`: Performs a jump to a longword address stored in d2.
- `jump_to_external_address`: Performs an unconditional jump to a long address stored in memory.
- `jump_to_address_4`: Performs a jump to a specific address.
- `jump_to_zero`: Performs a jump to address zero.
- `execute_conditional_logic_5`: Calls two routines, then calls a third via a branch with link, and if a word at a negative offset of a4 is zero, it pushes a word from a and calls another routine.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `execute_sequential_calls`: Calls several routines with various word arguments on the stack, clears a value at a negative offset of a4, and performs conditional jumps based on the results in d0.
- `dispatch_message_and_call`: It saves registers, calls a routine, calls another routine, clears a word in a structure, loads a pointer from a global table, calls a library function, and if the result is non-zero, calls a second library function before branching.
- `initialize_status_values`: It pushes a word from a global variable onto the stack, sets d0 to 1 and d1 to 0, stores these in offsets -36 and -34 of a5 and offset -38 of a5 respectively, and then checks two global variables.
- `validate_range_values`: It compares a word from a5 at offset 8 with several constants, and compares a word from a5 at offset 10 with two other constants, otherwise it branches to another location.
- `process_message_data`: It pushes two words onto the stack and calls a routine, then checks a word at an offset in a4; if non-zero, it calls another routine, performs several memory lookups and comparisons using offsets from a4, and finally calls two more routines.
- `loop_library_messages`: Checks a word at offset -36 of a5, and if not zero, calls a routine using a pointer from a structure in a4, stores the result in offset -14 of a5, and branches to another routine if d0 is not zero.
- `process_structure_and_call_handlers`: Copies several values from a structure pointed to by a5 into the stack, calls a routine via a pointer from a6, and then performs several more calls to routines using pointers from a4 and a6.
- `compare_and_update_counter`: Loads a longword from offset -18 of a5, compares it to an address, and if equal, increments a word at offset -38 of a5, then compares that word to 2 and updates offset -36 of a5 accordingly.
- `decrement_and_compare`: Decrements d1 in a loop, and if d1 is not zero, it checks if a word at offset -20 of a5 equals 232; if it does, it sets d0 to 0 and stores it at offset -34 of a5.
- `process_structure_data_8`: If d1 equals 104, it clears a word at offset -38 of a5, sets a word at offset -34 of a5 to 2, loads four longwords from a5 into d0-d3, calls a routine using a pointer from a4, and if d0 is not zero and the word at offset -34 of a5 is 2, it sets a word at offset -36 of a5 to 0.
- `clear_and_call_subroutine`: It clears a word at a negative offset of a5, compares a word at a different negative offset of a5 to two, and if not equal, it pushes a word from a negative offset of a5 to the stack, calls a routine, and returns.
- `process_conditional_logic_3`: Compares a word at a negative offset of a5 with a constant, calls several routines with arguments from the stack or a5, and performs conditional branches based on the results.
- `call_subroutine_with_parameter`: Moves a word from a negative offset in a5 to a location in a4, pushes a word from a negative offset in a5 onto the stack, calls a subroutine, and then sets d0 to ten before branching.
- `call_subroutine_with_offset`: It pushes a word from an offset of a structure onto the stack, calls a routine, increments the stack pointer, and branches.
- `compare_and_branch_6`: Pushes a word from a negative offset of a5 onto the stack, calls a subroutine, increments the stack pointer, compares a word from a4 with a value from a5, and either calls another subroutine or branches.
- `compare_and_branch_value`: Compares a word from a4 to several constants, and if it equals 1, pushes two words onto the stack, calls a subroutine, and then writes a word to a5 and branches based on the result.
- `call_and_branch_3`: Calls a subroutine, sets d0 to zero, and then branches.
- `call_subroutine_with_stack_param`: Pushes a word from a negative offset in a5 onto the stack, calls a subroutine, increments the stack pointer, sets d0 to ten, and then branches.
- `call_subroutine_via_stack_2`: Pushes a word from offset -4 of a5 onto the stack, calls a subroutine at a relative offset, increments the stack pointer by 2, moves several registers to the stack, unlinks a5, and returns.
- `compare_and_branch_4`: Links a5, saves registers, and compares a word from an offset of a5 with 80; if the value is not less than 80, it clears d0 and branches.
- `test_masked_value`: Performs a bitwise AND on a word from a structure with 192, and if the result is non-zero, clears d0 and branches to another routine; otherwise it returns.
- `jump_via_offset_table`: Compares d1 to 0, and if it is less than or equal, branches to another routine; if d1 is between 0 and 10, it subtracts 1 from d1 and, if d1 is not less than 0, it checks if d1 is greater than or equal to 10, otherwise it shifts d1 left by 1 and jumps to an address calculated from the program counter plus the value in d1.
- `evaluate_bitfield_and_offset`: Tests bits in a structure at a5 to set specific bits in a structure at a5, then performs arithmetic and comparisons using values from a4 and a5.
- `increment_and_branch_3`: Increments d7 by one and branches to another routine.
- `call_and_restore_registers_3`: Pushes a word to the stack, calls another routine, and then restores d7 and a2 from the stack before unlinking a5 and returning.
- `calculate_division_results_2`: It performs signed division on a word from a global table and a constant, stores the results in several offsets of a structure, and branches if a word at offset 8 of the structure is non-zero.
- `call_and_update_value`: Calls a library routine, then if a word at offset 8 in a5 is non-zero, performs arithmetic on a word at offset -68 of a5 and fetches a longword from a table using the result.
- `calculate_and_set_values`: Writes a longword and a byte to a structure, pushes two words from a5 onto the stack, calls a routine, performs arithmetic on the results to update three words in a5, pushes a word from a5 onto the stack, calls another routine, and then enters a loop that continues if d0 is not zero.
- `test_and_call_handler`: Clears the stack, pushes an address from a4 onto the stack, calls a subroutine, increments the stack pointer, and then checks a word at an offset of a5 to decide whether to set d0 to 1 and branch.
- `set_bit_and_call_handlers`: Sets bit 0 of a byte at an offset from a pointer in a4, writes a word to a5, and then calls a subroutine twice using addresses from a4 and a pointer in a6 if certain conditions are met.
- `copy_structure_data`: Copies several words and longwords from a structure pointed to by a negative offset of a5 into other negative offsets of a5, calls a subroutine using a pointer from a6, and then performs several conditional calls and jumps based on values in a4 and a5.
- `process_structure_data_6`: Checks several conditions involving a5 and a4, then calls a subroutine, performs arithmetic on a value from a table at offset -31310 of a4, updates a structure at offset 19116 of a4, and calls several other subroutines using a pointer from offset 19092 of a4.
- `clear_offset_value`: Sets d0 to zero, stores it at an offset of -42 from a5, and branches to another location.
- `check_and_set_bit`: Clears two words at offsets -42 and -32 of a5, compares a longword at offset -50 of a5 with 8, and if equal, compares a word at offset -52 of a5 with 69 to potentially set bit 0 of a byte at offset 15 of a5.
- `set_byte_and_branch_2`: Compares d3 to 104 and branches if not equal; otherwise it sets bit 0 of a byte at offset 15 of a5, moves 1 to offset -34 of a5, and branches to another routine.
- `test_and_set_bits`: Compares d3 to 232 and branches if not equal; otherwise, it clears bit 0 of a byte at offset 15 of a5, moves d0 to offset -40 of a5, and either moves 17 to offset -32 of a5 or branches based on bit 1 of a byte at offset 15 of a5.
- `compare_and_branch_long`: Compares a longword in d2 with 1024 and branches if not equal; otherwise it pushes two words from a5 onto the stack, calls another routine, adds 4 to the stack pointer, moves d0 to offset -32 of a5, and performs several comparisons on d0 before potentially clearing offset -40 of a5 and branching.
- `execute_handler_logic_2`: Pushes d0 to the stack, calls a subroutine, checks a word from a global structure in a4, and if it is not 1 or 2, clears bit 0 of a byte at offset 25 of a structure in a0; it also calls another subroutine if a word at offset -42 of a5 is non-zero.
- `calculate_division_quotients`: Calls a subroutine with two words from a5 on the stack, performs two divisions on the result using 100 and 100 as divisors, and stores the resulting quotients into offsets minus 2 and minus 4 of a5.
- `calculate_signed_difference_3`: Subtracts a word from a longword and, if the result is negative, negates it before branching.
- `calculate_difference_and_negate_2`: Subtracts a word in d1 from a longword in d4 and stores it in d5, then compares d5 to d3; if d5 is not less than or equal to d3, it subtracts d0 from d2, stores the result in d3, and if d3 is negative, negates it.
- `update_register_values`: Subtracts d0 from d2, stores the result in d3, and branches.
- `calculate_absolute_difference_2`: Copies d4 to d0, subtracts d1 from d0, and if the result is negative, negates d0 before branching.
- `process_conditional_logic_2`: Subtracts d1 from d4, stores the result in d0, then stores d0 in d3 and at offset minus 30 of a5; if d3 is greater than or equal to a value at an offset from a4, it clears the stack, pushes a value from an offset from a4, calls another routine, adjusts the stack, pushes 1 into d0, pushes d0 onto the stack, calls another routine, adjusts the stack, and branches.
- `execute_conditional_logic_4`: Checks a word at offset minus 42 of a structure in a5, calls another routine with a pointer from a5 if non-zero, then compares a word from a4 with a word from a5 to decide whether to call another routine with values from a4 and a5 or perform a long jump.
- `execute_conditional_calls_4`: Compares a sum of a global variable and d1 against d2, then calls another routine twice using different values from a4 and a5 depending on comparison results.
- `call_subroutine_with_data`: Adds a word from a global variable to a register, compares the result to another value, and if the result is not less than the comparison value, calls another routine with two values from the stack.
- `process_scaled_values`: Calls several routines to manipulate data, performs arithmetic to calculate and store four scaled values in a structure at a5, and then calls more routines using extended longwords derived from those values.
- `execute_conditional_subroutines_2`: Clears a word at offset minus 42 of a5, calls two different routines with values from a4 on the stack, and then performs arithmetic using values from offsets minus 2, minus 4, minus 24, and minus 28 of a5 to determine a branch.
- `call_subroutine_with_stack`: Sets d0 to one, stores it at offset minus 38 of a5, pushes two words from offset minus 8 and minus 6 of a5 onto the stack, calls another routine, cleans the stack, and branches to another location.
- `execute_handler_logic`: Clears the stack, calls a routine, clears a bit in a structure at offset 19104 of a4, calls a library routine if a word at offset -42 of a5 is not zero, calls another routine, and finally stores a calculated word into a structure at offset 10 of a5.
- `process_conditional_menu_calls`: Compares a word from a global variable with a value at offset 8 of a structure in a5, then performs two conditional calls to a routine at offset -54 of a structure in a6 using data from offsets 19104 and 19108 of a4.
- `jump_via_scaled_index`: Links a5, saves registers, compares a word from an offset of a5 with 50, and if not less, performs a jump to an address calculated by shifting the value in d0 left by two and adding it to a base address.
- `branch_to_offset_2`: Performs a short relative branch to an offset.
- `set_and_branch_2`: Sets d0 to 0 and branches to another routine.
- `call_on_constant_match`: Compares a word at offset minus 24656 of a4 to 8, and if it is not equal, pushes 1 to the stack and calls another routine.
- `process_data_and_call_subroutine`: Calculates several values based on a word at offset -18500 of a4, performs conditional arithmetic and stores results in a structure in a5, calls a routine using a pointer from a6, and then calls another routine using four longwords from a5 and a pointer from a5 as arguments.
- `push_values_and_call_subroutine`: Extends a word from an offset of negative ten from a5 to a longword in d0, calls a routine using a pointer from a4, extends four words from offsets of negative eight, negative six, negative four, and negative two from a5 to d0 through d3, pushes these onto the stack, pushes a value from a pointer in a0 onto the stack, calls another routine, and then increments several values before branching.
- `calculate_offset_values`: Compares d1 to two, and if d1 is less than two, fetches a byte from offset 54 of a structure in a0, performs arithmetic involving d1 and d3, and stores the resulting values at offsets minus 8 and minus 4 of a structure in a5.
- `calculate_and_store_offsets_2`: Loads a pointer from a4, extracts a byte from offset fifty-four of the pointed structure, performs arithmetic to calculate a value, stores it at offset negative eight from a5, and then performs further arithmetic and conditional checks to store additional calculated values at offsets negative four, negative six, and negative two from a5.
- `perform_complex_calculations_and_calls`: Calculates values based on a byte from a0 and stores them in a structure at a5, then performs five library calls using values from that structure and pointers from a0 and a4 before restoring registers and returning.
- `jump_to_address_35`: Performs a long jump to the address stored at 4022.
- `perform_indirect_jump`: Performs a jump to an address stored in a longword at a specific location.
- `jump_to_lock`: Performs a jump to an address provided by a library stub.
- `jump_to_indirect_address_5`: Performs an indirect jump to the address stored in a0.
- `jump_to_indirect_address_14`: Performs an indirect jump to the address stored at offset 2ea of a longword.
- `jump_to_indirect_address_39`: Performs a long jump to the address stored in a pointer located in the data section.
- `jump_to_offset_128`: Performs an indirect jump to the address stored at offset 128 of the longword at address 2424.
- `jump_to_calculation_routine`: Calls a routine that loads bytes from a structure in a4, performs arithmetic to calculate four values, stores them in a structure in a5, and branches to another routine.
- `jump_to_offset_32`: Performs an indirect jump to the address stored at offset 32 of the longword at address 23ac.
- `jump_to_absolute_address_5`: Performs an absolute long jump to a target address.
- `jump_to_indirect_address_15`: Performs an indirect jump to the address stored at offset 3e4 of a longword.
- `jump_to_indirect_address_16`: Performs an indirect jump to the address stored at offset 3fc of a longword.
- `jump_to_offset_8`: Performs an indirect jump to the address stored at offset 8 of the longword at address 238e.
- `jump_to_address_33`: Performs a long jump to an address stored in a longword at offset ec8.
- `jump_to_address_15`: Jumps to a longword address.
- `jump_to_external_address_3`: Performs a long jump to a specific address in memory.
- `jump_to_zero_address`: Performs a jump to a longword address of zero.
- `jump_to_address_29`: Performs an indirect jump to an address stored in a longword.
- `perform_indirect_jump_2`: Performs a long jump to an address stored at offset 25 of a pointer.
- `jump_to_resident_library_stub`: Performs an indirect jump to a routine that locates a resident library.
- `jump_to_indirect_address`: Performs an indirect jump to the address stored at a specific location in memory.
- `jump_to_offset_152`: Performs an indirect jump to the address stored at offset 152 of the longword at address 2442.
- `jump_to_fixed_address`: Performs a long jump to address 4080.
- `jump_to_offset_0`: Performs an indirect jump to the address stored at offset 0 of the longword at address 2384.
- `jump_to_address_31`: Performs a long jump to the address 800.
- `jump_to_offset_136`: Performs an indirect jump to the address stored at offset 136 of the longword at address 242e.
- `jump_to_indirect_address_38`: Performs an indirect jump to an address stored in a pointer located in the data section.
- `perform_indirect_jump_offset`: Performs a long jump to an address stored at a specific offset.
- `jump_to_offset_24`: Performs an indirect jump to the address stored at offset 24 of the longword at address 23a2.
- `jump_to_offset_80`: Performs an indirect jump to the address stored at offset 80 of the longword at address 23e8.
- `jump_to_absolute_address_2`: Performs an unconditional jump to a specific longword address.
- `jump_to_offset_184`: Performs an indirect jump to the address stored at offset 184 of the longword at address 246a.
- `jump_to_library_stub`: Performs an indirect jump to a routine that handles writing data via the DOS library.
- `jump_to_index_56`: Performs an indirect jump to an address that loads the value 56 into register d0.
- `jump_to_division_routine_2`: Calls calculate_division_result_2 via a long jump.
- `jump_to_constant_setter`: Calls a routine that checks a value on the stack and, if zero, stores a constant into a memory location at an offset of a pointer held in d0.
- `jump_to_index_104`: Performs an indirect jump to an address that loads the value 104 into register d0.
- `jump_to_handler_2`: Performs a long jump to the address at offset 3f28.
- `jump_to_indirect_address_27`: Performs an indirect long jump to an address stored in memory.
- `execute_indirect_jump_2`: Performs an indirect jump to the address stored at an offset of 1042.
- `jump_to_index_40`: Performs an indirect jump to an address that loads the value 40 into register d0.
- `jump_with_d0_set`: Performs an indirect jump to an address that sets register d0 to 16.
- `jump_to_memory_setup`: Performs a long jump to another routine after setup_memory_pointers has loaded two pointers from the stack and a word from offset 12 of the stack.
- `set_d0_and_jump`: Sets d0 to 0 and performs an indirect jump to an offset of 36.
- `set_d0_and_jump_2`: Sets d0 to 8 and performs an indirect jump to an offset of 36.
- `set_d0_and_jump_3`: Sets d0 to 16 and performs an indirect jump to an offset of 36.
- `set_d0_and_jump_4`: Sets d0 to 24 and performs an indirect jump to an offset of 36.
- `load_index_32`: Loads the word value 32 into register d0 and jumps to another routine.
- `load_index_40`: Loads the word value 40 into register d0 and jumps to another routine.
- `load_index_48`: Loads the word value 48 into register d0 and jumps to another routine.
- `load_index_56`: Loads the word value 56 into register d0 and jumps to another routine.
- `load_index_64`: Loads the word value 64 into register d0 and jumps to another routine.
- `load_index_72`: Loads the word value 72 into register d0 and jumps to another routine.
- `load_index_80`: Loads the word value 80 into register d0 and jumps to another routine.
- `load_index_88`: Loads the word value 88 into register d0 and jumps to another routine.
- `load_index_96`: Loads the word value 96 into register d0 and jumps to another routine.
- `load_index_104`: Loads the word value 104 into register d0 and jumps to another routine.
- `load_offset_112`: Loads the word value 112 into d0 and jumps to another routine.
- `load_offset_120`: Loads the word value 120 into d0 and jumps to another routine.
- `load_offset_128`: Loads the word value 128 into d0 and jumps to another routine.
- `load_offset_136`: Loads the word value 136 into d0 and jumps to another routine.
- `load_offset_144`: Loads the word value 144 into d0 and jumps to another routine.
- `load_offset_152`: Loads the word value 152 into d0 and jumps to another routine.
- `load_offset_160`: Loads the word value 160 into d0 and jumps to another routine.
- `load_offset_168`: Loads the word value 168 into d0 and jumps to another routine.
- `load_offset_176`: Loads the word value 176 into d0 and jumps to another routine.
- `load_offset_184`: Loads the word value 184 into d0 and jumps to another routine.
- `set_d0_and_jump_5`: Sets d0 to 192 and performs a long jump to an address at offset 24.
- `set_d0_and_jump_6`: Sets d0 to 200 and performs a long jump to an address at offset 24.
- `set_d0_and_jump_7`: Sets d0 to 208 and performs a long jump to an address at offset 24.
- `execute_library_calls_9`: Calls a routine via a pointer in a6, decrements a word at offset 8 of a5, and then calls another routine via a pointer in a6 if the word in d0 is not less than or equal to zero.
- `calculate_and_call_routine`: Links a5, extracts a word from offset 8 of a5, extends it to a longword, divides it by 100, subtracts a word from offset 20326 of a4, extracts a word from offset 8 of a5, extends it to a longword, divides it by 100, swaps the high and low words, subtracts a word from offset 20320 of a4, pushes the results onto the stack, calls another routine, adjusts the stack pointer, unlinks a5, and returns.
- `clear_byte_and_call`: Calculates an address by adding a value from a4 to a pointer, clears a byte at that address, pushes registers onto the stack, calls another routine, clears a word at an offset of a4, and returns.
- `update_structure_via_library`: If a word at an offset in a4 is not zero and certain bits are set in a structure at an offset in a4, it calls a routine using a pointer from a5, then conditionally updates a value at an offset in a5 and branches to another routine.
- `decrement_and_loop`: Compares a longword at an offset in a5 with a longword at an offset in a4, and if the value in a5 is not greater, it enters a loop that increments a value at an offset in a5, decrements a value at an offset in a5, and branches back to the start.
- `compare_and_decrement_counter`: Compares a longword at offset -4 of a5 with a longword at offset -25808 of a4, and if the value in a5 is not greater, it enters a loop that decrements a value at offset -24790 of a4 and calls another routine.
- `update_structure_counters`: Pushes two words from a table onto the stack, calls another routine, and then performs conditional arithmetic to increment or decrement values at offsets minus 25816, minus 25812, and minus 25816 of a structure in a4 based on comparisons with constant values.
- `copy_structure_members`: Copies three longwords from offsets minus 12, minus 8, and minus 4 of a structure pointed to by a5 into offsets minus 25560, minus 25564, and minus 25568 of a structure pointed to by a4, then restores registers and returns.
- `negate_if_negative_2`: Loads a word from an offset in a4 into d1, and if d1 is negative, negates it and stores the result in d0.
- `process_table_entry`: Loads a 16-bit value from offset 8 of a structure in a5, and if it is negative, it pushes 45 to the stack, calls another routine, negates the value, performs division by 10, and if the result is non-zero, calls another routine recursively before performing further division and arithmetic and calling another routine.
- `copy_byte_loop_4`: Checks a word at offset 8 of a5; if non-zero, it enters a loop that copies bytes from a location derived from a4 to a location derived from a5 until the byte is zero.
- `execute_complex_state_update`: Copies a byte from a4 to a5, calls several subroutines with various arguments from a4 and a5, and performs an indirect call to a routine at offset -416 of a4.
- `execute_indirect_calls_2`: Pushes three pointers onto the stack and calls a routine at offset 16076 of the program, then pushes two more pointers and calls a routine at offset 15974 of the program before performing an indirect branch.
- `copy_byte_and_call`: Copies bytes from a source in a4 to a destination in a4 using a loop, then pushes two values onto the stack and calls another routine before returning.
- `update_structure_and_call_2`: Calls a routine via a pointer at offset -416 of a4, then compares a longword at offset -12 of a5 with a longword at offset -25666 of a4, and if the comparison passes, increments offset -8 of a5 and decrements offset -12 of a5 before branching back to the start of this routine.
- `execute_library_sequence`: Calls several library functions via a pointer in a6, performs arithmetic on bytes from a structure in a0, calls another routine with five arguments, calls more library functions, and then enters a loop that checks bytes at an offset in a4, decrementing a pointer and calling a library function if a byte is non-zero.
- `process_conditional_data`: Calls a routine, and if the result in d0 is non-zero, checks a byte at an offset of a5; if that byte is non-zero, it performs arithmetic on a pointer from a4 to compare a word at that location with a value from a global variable, and if they are not equal, it performs further arithmetic and calls another routine.
- `execute_conditional_handler`: Links a5, saves d2, loads a word from offset minus 30736 of a4 into d0, calls another routine, calls another routine with d1 set to 65535, compares d0 with a word at offset minus 24898 of a4, and if they are not equal, pushes a word from offset minus 24896 of a4 and d0 onto the stack, calls another routine, adjusts the stack pointer, restores d2, unlinks a5, and returns.
- `calculate_and_call_handler`: Calculates a value by shifting a word from a global variable and subtracting the original value, calls another routine with this result, and then optionally calls a second routine if the result does not match a value from a global variable.
- `update_structure_and_call_library_2`: Calculates a value based on a byte from a structure in a4, calls two library routines, performs arithmetic on several bytes from the same structure, calls another routine with several arguments, and then conditionally calls more library routines and another routine based on a bit test and a comparison.
- `calculate_and_dispatch_values`: Performs arithmetic on bytes from a structure in a4 to calculate several values, calls a routine with these values, and if a bit test in a5 passes, calls more library routines and another routine based on a comparison of a calculated value and a value from the structure.
- `process_indexed_values`: Decrements a word at offset -18 of a structure in a5, then if not zero, performs arithmetic on values from a structure in a4 to update multiple fields in a5 and calls several library functions.
- `process_extended_values`: Decrements a word at offset -18 of a structure in a4, then if not zero, performs checks against a table and calls several routines to update multiple fields in a5.
- `update_scaled_structure_fields`: Calls several routines, calculates an index from a value in a4, and then performs multiple cycles of reading values, calculating scaled results via arithmetic and bit shifts, and calling routines to update a structure at a5.
- `calculate_and_dispatch_values_2`: Performs divisions on a word from a5, pushes various values and pointers onto the stack, and calls several different routines based on the results of calculations and tests of global variables.
- `process_table_entry_2`: Loads a pointer from offset 8 of a structure in a5, checks if the byte at that pointer is zero, and if not, pushes the byte extended to 16 bits onto the stack before calling another routine and branching.
- `calculate_and_push_values`: Calculates a value by adding a word from a table to a word from a structure in a5, uses this to index into another table to retrieve a byte, and pushes both the byte and the calculated word onto the stack before calling another routine.
- `update_table_entry`: Compares a 16-bit value from a global variable against 98, and if it is less than 98, stores a byte from offset 9 of a structure in a5 into a calculated address in a0, then increments the global variable by 1.
- `dispatch_conditional_subroutine`: Sets up a stack frame, compares a byte at a calculated address with another byte, and if they are not equal, pushes three words onto the stack and calls a subroutine before calling another routine via bsr.
- `fetch_table_values`: Links a5, saves registers, calculates several addresses using offsets from a4 to fetch bytes, stores these bytes into the stack at offsets of a5, and then calls another routine if a word at a specific offset equals 1.
- `execute_conditional_sequence_5`: Copies bytes between two memory locations, performs an indirect jump, calls a library routine, and if certain conditions are met, calls several other routines and updates an offset in a5.
- `execute_conditional_sequence_4`: Pushes values onto the stack and calls a routine multiple times with different parameters, then performs a loop that reads bytes from a table at an offset in a4 and adds them to a value in a5.
- `execute_conditional_sequence_3`: Pushes two words onto the stack, calls a routine using a pointer at offset -12 of a5, updates offset -8 of a5 with the result in d0, calls another routine using a pointer at offset -12 of a5, and if the new value in offset -8 of a5 is not 2, calls a third routine before returning.
- `allocate_and_branch`: Copies a byte from a4 to a5, checks a word at offset 8 of a5 and a byte at offset -24777 of a4, then calls a routine with three parameters from a4 and a5 and branches to another routine if the result is non-zero.
- `execute_conditional_calls_6`: Pushes two values from a4 onto the stack and calls a routine, stores the result in a5, and if the result is non-zero, branches back to the start of this routine; otherwise, it calls another routine with parameters from a4 and branches to another routine.
- `update_and_call_library_2`: Copies bytes between a4 and a2 or a5, calls several routines with parameters from a4 and a5, calls a library routine using a pointer from a4, and branches to another routine if a longword from a4 is non-zero.
- `process_structure_data_9`: Copies a byte from a location in a4 to a location in a2, then performs several calls with various parameters from a4 and a5, eventually storing a word from a5 into offset -86 of a5 and calling another routine.
- `push_parameters_and_call_2`: Pushes two values onto the stack, calls another routine, and then loads a word from a structure in a5 into d0 before returning.
- `branch_based_on_byte_value`: Loads a byte from offset -24638 of a4, extends it to a word, and compares a word at offset 12 of a5 to it; it branches to another routine if the value is one, otherwise it branches to another routine if the value is non-zero, or otherwise it branches to another routine.
- `store_pc_relative_address`: Stores the address of a location relative to the program counter into a memory location at offset -12 of a5, then branches to another routine.
- `clear_d0_and_branch_3`: Clears register d0 and performs a short branch to another routine.
- `clear_and_update_scaled_values`: Clears a word at offset 19068 of a4, then calls a routine via a pointer at offset -2 of a5 and loops twice to add a byte from a table at offset -2 of a5 to the global variable.
- `update_scaled_values_with_offset_addition`: Adds a word from offset -2 of a5 to a global variable at offset 19068 of a4, then calls a routine via a pointer at offset -6 of a5 and loops twice to add a byte from a table at offset -6 of a5 to the global variable.
- `fetch_and_accumulate_table_byte`: Calls another routine to retrieve a value, then uses the result to index into a table to fetch a byte, which is added to a value at a fixed offset from a pointer in a4.
- `accumulate_byte_from_table_via_call`: Calls another routine and then iterates through a table to add each byte's value to a global variable at a specific offset.
- `accumulate_bytes_from_table`: Calls another routine and then iterates through a table to add each byte's value to a global variable at a specific offset.
- `accumulate_table_bytes_to_global`: Calls another routine and then loops through a table to extract bytes, adding each byte to a global variable at a specific offset.
- `update_scaled_values_from_table_2`: Calls another routine with a pointer to a location in a table, then loops to add bytes from that table to a global variable.
- `update_scaled_values_from_table`: Calls a routine via a pointer at offset -12 of a5, then loops twice to add a byte from a table at offset -24896 of a4 to a global variable at offset 19068 of a4.
- `update_scaled_values_24898`: Calls a routine via a pointer with offset -24898 of a4, then loops 2 times to add a byte from a table at that offset to a global variable.
- `update_scaled_values_24814`: Calls a routine via a pointer with offset -24814 of a4, then loops 2 times to add a byte from a table at that offset to a global variable.
- `update_scaled_values_20328`: Calls a routine via a pointer with offset 20328 of a4, then loops 628 times to add a byte from a table at that offset to a global variable.
- `update_scaled_values_24812`: Calls a routine via a pointer with offset -24812 of a4, then loops 8 times to add a byte from a table at that offset to a global variable.
- `update_scaled_values_24804`: Calls a routine via a pointer with offset -24804 of a4, then loops 8 times to add a byte from a table at that offset to a global variable.
- `update_scaled_values_20928`: Multiplies a value derived from offset -18580 of a4 by 24, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 20928 of a4 to a global variable.
- `update_scaled_values_21224`: Doubles a word from offset -24794 of a4, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 21224 of a4 to a global variable.
- `update_scaled_values_21424`: Doubles a word from offset -24794 of a4, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 21424 of a4 to a global variable.
- `update_scaled_values_21624`: Doubles a word from offset -24794 of a4, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 21624 of a4 to a global variable.
- `update_scaled_values_alternate_2`: Doubles a word from offset -24794 of a4, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 21824 of a4 to a global variable.
- `update_scaled_values_alternate`: Doubles a word from offset -24794 of a4, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 22024 of a4 to a value at offset 19068 of a4.
- `update_scaled_values`: Multiplies a word from offset -24794 of a4 by 4, calls a routine via a pointer, and then loops based on that result to add a byte from a table at offset 22224 of a4 to a value at offset 19068 of a4.
- `update_two_values_offset`: Calls a routine via a pointer with 2 and 1 pushed to the stack, then loops 2 times to add a byte from a table at offset -24792 of a4 to a value at offset 19068 of a4.
- `update_eight_values_offset_2`: Calls a routine via a pointer with 8 and 1 pushed to the stack, then loops 8 times to add a byte from a table at offset 22624 of a4 to a value at offset 19068 of a4.
- `update_two_values_alternate`: Calls a routine via a pointer with 2 and 1 pushed to the stack, then loops 2 times to add a byte from a table at offset 22632 of a4 to a value at offset 19068 of a4.
- `update_two_values`: Calls a routine via a pointer with 2 and 1 pushed to the stack, then loops 2 times to add a byte from a table at offset -24790 of a4 to a value at offset 19068 of a4.
- `update_eight_values_offset`: Calls a routine via a pointer with 8 and 1 pushed to the stack, then loops 8 times to add a byte from a table at offset -24788 of a4 to a value at offset 19068 of a4.
- `update_eight_values_alternate`: Calls a routine via a pointer with 8 and 1 pushed to the stack, then loops 8 times to add a byte from a table at offset 22720 of a4 to a value at offset 19068 of a4.
- `update_table_via_pointer`: Compares a word at offset -2 of a5 to 7237, and if not equal, doubles a word from offset -18296 of a4, calls a routine via a pointer, and then loops based on the doubled value to add bytes from a table at offset 22716 of a4 to a value at offset 19068 of a4.
- `update_eight_values`: Doubles a word from a table at offset -24794 of a4, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset 22808 of a4 to a value at offset 19068 of a4.
- `update_eight_values_23008`: Pushes 8 and 1 onto the stack, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset 23008 of a4 to a value at offset 19068 of a4.
- `update_eight_values_neg_24680`: Pushes 8 and 1 onto the stack, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset -24680 of a4 to a value at offset 19068 of a4.
- `update_160_values_23016`: Pushes 160 and 1 onto the stack, calls a routine via a pointer, and then loops 160 times to add a byte from a table at offset 23016 of a4 to a value at offset 19068 of a4.
- `update_eight_values_23180`: Pushes 8 and 1 onto the stack, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset 23180 of a4 to a value at offset 19068 of a4.
- `update_two_values_neg_24622`: Pushes 2 and 1 onto the stack, calls a routine via a pointer, and then loops 2 times to add a byte from a table at offset -24622 of a4 to a value at offset 19068 of a4.
- `update_eight_values_neg_18882`: Pushes 8 and 1 onto the stack, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset -18882 of a4 to a value at offset 19068 of a4.
- `update_two_values_neg_24612`: Pushes 2 and 1 onto the stack, calls a routine via a pointer, and then loops 2 times to add a byte from a table at offset -24612 of a4 to a value at offset 19068 of a4.
- `update_eight_values_neg_18590`: Pushes 8 and 1 onto the stack, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset -18590 of a4 to a value at offset 19068 of a4.
- `update_eight_values_23200`: Pushes 8 and 1 onto the stack, calls a routine via a pointer, and then loops 8 times to add a byte from a table at offset 23200 of a4 to a value at offset 19068 of a4.
- `update_seventy_two_values_2`: Pushes 64 and 1 onto the stack, calls a routine via a pointer, and then loops 64 times to add a byte from a table at offset 23208 of a4 to a value at offset 19068 of a4.
- `update_seventy_two_values`: Pushes two small constants onto the stack, calls another routine via a pointer, and then loops to add a byte from a table to a value at a specific offset in a structure.
- `process_seventy_two_updates`: Pushes 72 and 1 onto the stack, calls a routine via a pointer in a5 with an offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `call_subroutine_and_update_global_value`: Pushes two 16-bit values onto the stack, calls a subroutine via a pointer, and then calculates a new value by reading a byte from a memory location offset by a stack-derived index and adding it to a global variable at a fixed offset.
- `call_subroutine_and_update_offset`: Pushes two 16-bit values onto the stack, calls a subroutine via a pointer in a5, and then calculates a new address by adding a byte from a memory location to a base address and branching.
- `process_four_updates_conditional`: Checks if a word at offset -2 of a5 is 11301, and if so, pushes 4 and 1 onto the stack, calls a routine via a pointer in a5 with an offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `process_four_updates_alt`: Pushes 4 and 1 onto the stack, calls a routine via a pointer in a5 with a different offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `process_four_updates`: Pushes 4 and 1 onto the stack, calls a routine via a pointer in a5 with an offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `process_two_updates_alt`: Pushes 2 and 1 onto the stack, calls a routine via a pointer in a5 with a different offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `process_two_updates`: Pushes 2 and 1 onto the stack, calls a routine via a pointer in a5 with an offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `process_scaled_updates_2`: Pushes 8 and 1 onto the stack, calls a routine via a pointer in a5 with an offset from a4, and then loops to add a byte from a table to a value at offset 19068 of a4.
- `process_scaled_updates`: Calculates a value by doubling a global word, pushes it and a constant onto the stack, calls a routine via a pointer in a5, and then enters a loop that calculates an offset from a global variable to read a byte and add 19068 to it.
- `call_routine_and_update_global_value`: Pushes two small constants onto the stack, calls a routine via a pointer, and then performs a loop that reads a byte from a table and adds it to a value at a fixed offset in a global structure.
- `process_variable_updates`: Pushes a value from a global variable and a constant onto the stack, calls a routine via a pointer in a5, and then enters a loop that calculates an offset from a global variable to read a byte and add 19068 to it.
- `call_routine_with_offset_calculation`: Pushes two values onto the stack, calls a routine via a pointer, and then calculates a new address by adding a byte from a table to a base address.
- `call_routine_and_offset_pointer`: Pushes two values onto the stack, calls a routine via a pointer, and then calculates a new address by adding a byte from a table to a base address.
- `call_subroutine_and_offset_data`: Pushes two values onto the stack, calls a subroutine via a pointer, and then calculates a new address by adding a byte from a table to a base address.
- `call_subroutine_and_fetch_offset_2`: Pushes two values onto the stack, calls a subroutine via a pointer, and then calculates a new address by adding a byte from a table to a global variable.
- `call_subroutine_and_fetch_offset`: Pushes two values onto the stack, calls a subroutine via a pointer, and then calculates a new address by adding a byte from a table to a global variable based on a decremented stack value.
- `process_update_list`: Pushes two values onto the stack, calls a routine via a pointer in a5, and then enters a loop that calculates an offset from a global variable to read a byte and add 19068 to it.
- `loop_table_updates`: It pushes several values onto the stack, calls a routine via a pointer from a5, and then enters a loop that calculates an offset from a global variable and adds a byte from that offset to another global variable.
- `iterate_table_values`: It doubles a word from a global variable, pushes several values onto the stack, and calls a routine via a pointer from a5; it then enters a loop that calculates an offset from a global variable and adds a byte from that offset to another global variable.
- `update_table_with_callback`: It doubles a word from a global variable, pushes several values onto the stack, and calls a routine via a pointer from a5; it then performs a loop that calculates an offset from a global variable and adds a byte from that offset to another global variable.
- `update_table_via_callback`: Doubles a word from offset -24794 of a4, calls a routine via a pointer from offset -12 of a5, and if d0 is not zero, it uses a byte from a table to increment a value at offset 19068 of a4.
- `calculate_and_apply_offset`: Doubles a word from offset -24794 of a4, calls a routine via a pointer from offset -12 of a5, and if the returned d0 is not zero, it adds a byte from a table to a value at offset 19068 of a4.
- `call_and_update_offset_2`: Pushes several values onto the stack, calls a routine via a pointer from offset -12 of a5, and if d0 is not zero, it calculates an offset using a byte from a table and adds it to a value at offset 19068 of a4.
- `increment_global_via_offset`: Pushes several values onto the stack, calls a routine via a pointer in a5, and if the returned d0 is not zero, it uses a decremented version of d0 to find a byte at an offset from a4 and adds it to a global variable.
- `update_global_counter`: Pushes several values onto the stack, calls a routine via a pointer in a0, and if the returned d0 is not zero, it calculates an offset using a value from a4 and a decremented version of d0 to add a byte from that memory location to a global variable.
- `call_and_update_table_2`: It calculates a value based on a word at a negative offset of a4, pushes several values onto the stack, calls a routine via a pointer in a0, and then updates a value at a positive offset of a4 using a byte from a calculated address.
- `call_and_update_table`: Pushes several arguments onto the stack, calls a routine via a pointer in a0, and then updates a value at an offset of a4 using a byte from a calculated address.
- `call_offset_based_handler`: Calculates a value from a word at offset -18296 of a4, pushes several arguments onto the stack, calls a routine via a pointer in a0, and updates a value at offset 19068 of a4.
- `call_and_update_offset`: Pushes several values onto the stack and calls a routine via a pointer at offset -12 of a5.
- `call_and_calculate_offset`: Pushes several values onto the stack, calls a routine via a pointer in a0, and if the result is non-zero, calculates a new address by adding a byte from a table to a base address.
- `call_and_update_index_4`: Pushes several values onto the stack, calls a subroutine via a pointer in a5, and if the result is non-zero, calculates a new address and adds a byte from that address to a global variable.
- `call_and_update_index_3`: Pushes three values onto the stack, calls a subroutine via a pointer in a5, and if the result is not zero, calculates a new address and adds a byte from that address to a value in a4.
- `call_and_update_index_2`: Pushes several values onto the stack, calls a subroutine via a pointer in a5, and if the result is not zero, calculates a new index and adds a byte from a table to a value in a4.
- `call_and_index_table_3`: Pushes several values onto the stack, calls a subroutine via a pointer in a5, and if the return value is not zero, calculates an index using a byte from a table in a4 and adds it to a value in a4.
- `call_and_index_table_2`: Pushes several values onto the stack, calls a routine via a pointer in a5, and then uses the result and a value from a4 to calculate an offset and add a byte to a global variable.
- `call_and_index_table`: Pushes several values onto the stack, calls a routine via a pointer in a5, and uses the result to calculate an offset into a table at offset 19068 of a4.
- `call_and_update_index`: Pushes several values onto the stack, calls a routine using a pointer from a5, and then uses the result to calculate an offset into a table to fetch a byte, which is then added to a value in a4.
- `execute_conditional_pointer_call`: Pushes several values onto the stack, including a pointer from offset 8 of a5 and a word from offset minus 18594 of a4, then calls a routine using a pointer from offset minus 12 of a5.
- `compare_and_execute_conditional`: Compares a word from offset -2 of a5 against three specific values and branches if they match; otherwise, it pushes several values onto the stack, calls a subroutine via a pointer from a5, and performs arithmetic on a value from a global structure in a4.
- `compare_and_negate_result_2`: If a word at offset 12 of a5 is zero, it pushes several values onto the stack, calls another routine, and then compares a value from a5 with a word in a4 to set d0.
- `call_routine_with_stack_parameters`: Pushes a longword from a5 and two small values onto the stack, calls a routine, cleans the stack, and then performs arithmetic on d1 and d0 before branching to another routine.
- `execute_conditional_subroutine`: Checks a word at offset 12 of a5, and if zero, pushes a pointer from a5 and several constants onto the stack to call a subroutine, then compares a word from a5 with a value in a4 to update d0.
- `calculate_and_call_subroutine`: Increments a value from a global variable, pushes it and a pointer from a5 onto the stack, calls a subroutine, then performs a second sequence of stack operations and a library call before returning.
- `process_indirect_call`: Stores a routine address in a structure, pushes values onto the stack, calls that routine, and then uses the result to update a value at an offset in a4.
- `process_byte_and_call`: Checks if a value at offset -12 of a structure in a5 is 1, then checks if a byte at offset -7 is non-zero and positive; if so, it stores that byte at offset -8, pushes several values onto the stack, calls a routine at an address from offset -16 of a5, and then performs arithmetic to add a value to a location at an offset of a4.
- `calculate_offset_address_3`: Calculates a new address by adding a signed byte from offset -8 of a structure in a5 to the value in a3, then branches to another routine.
- `decrement_and_copy_byte_2`: Negates a byte from offset -7 of a structure in a5, stores the result in offset -8, calls a routine at an address from offset -16, and then adds a value derived from a byte at an offset to a value at offset 19068 of a structure in a4.
- `copy_byte_loop_3`: Decrements a byte at offset -8 of a structure in a5, and if it is not zero, copies a byte from offset -7 of the structure to the address in a3 and increments a3, repeating this process in a loop.
- `restore_registers_a3`: Restores registers d2, d7, and a3 from the stack and returns.
- `search_and_compare_bytes`: Calculates a target value by multiplying a word from offset 12 of a5 by 6832, uses this to find a range, and enters a loop that compares bytes at an offset from a pointer in a3 to determine if a condition is met.
- `calculate_and_call_offset_byte`: If d7 is greater than 1, it stores the negative of d7 at offset -13 of a5, calls a routine via a pointer at offset -18 of a5, and then enters a loop that calculates an offset from a byte at a location relative to a5, adds that byte to offset 19068 of a4, and branches back to the start of the loop.
- `calculate_and_add_byte`: Calls a routine at an address from offset -18 of a structure in a5, then calculates an offset using a value from d6 and a byte at that offset to add to offset 19068 of a structure in a4.
- `calculate_address_from_d7`: Adds the signed long value in d7 to the address in a3 and branches to another routine.
- `iterate_and_compare_bytes`: Increments a value in d7 and compares bytes at two different offsets derived from a pointer in a3, looping if the value in d7 is less than 126 and the calculated offset is less than a value in d5.
- `fetch_and_update_offset`: Stores a byte from d0 into offset -13 of a structure in a5, pushes several values onto the stack, calls a routine at an address from offset -18 of a5, and then calculates an offset using a value from a structure and adds it to a location at an offset of a4.
- `fetch_and_increment_table_value`: Calls another routine while pushing several values onto the stack, then calculates an offset based on a byte read from a memory location and adds that value to a global variable at offset 19068 from a base register.
- `calculate_offset_address_4`: Adds a sign-extended 32-bit value from a register to a base address stored in a register, updates the base address register with the result, and branches to another routine.
- `execute_and_update_value`: Clears a byte at offset -13 of a structure in a5, calls a routine using a pointer from offset -18 of the structure, and then enters a loop that calculates an address based on a decremented value in d6 and adds a byte from that address to a value at offset 19068 of a structure in a4.
- `restore_registers_a3_extended`: Restores registers d2 through d7 and a3 from the stack and returns.
- `branch_on_stack_value`: Checks a word at offset 8 of a structure in a6, branches if it is zero, otherwise calls another routine and stores the returned value from d0 at offset -4 of a5 before branching.
- `store_and_branch`: Clears the stack top, calls a routine, and if d0 is not zero, it stores d0 at offset -4 of a5 and branches; otherwise it sets d0 to 0 and branches to another location.
- `execute_conditional_calls_5`: Pushes two longwords onto the stack and calls a routine, then conditionally calls another routine based on d0, performs more stack operations, calls a routine again, and finally calls a routine with two values on the stack.
- `process_and_scale_data`: Pushes a longword from a structure in a5 onto the stack, calls several routines, performs bitwise shifts and divisions on d0, and conditionally calls other routines based on the results of those calculations before returning.
- `fill_table_loop`: Enters a loop that writes the value 3 to two consecutive bytes at an offset from a table in a4, where the offset is determined by multiplying d7 by 100, until d7 reaches 59.
- `fill_table_with_constant`: Enters a loop that, while d7 is less than 99, writes the value 3 to offset 170c of a table at an address calculated by adding d7 to a base address from offset 5a88 of a structure in a4, and also writes 3 to the address itself.
- `restore_registers_d7`: Restores registers d7 and a3 from the stack and returns.
- `allocate_and_process_memory_list`: Allocates memory using a library call, copies bytes from a4 to a4 in a loop, and performs conditional calls to subroutines based on values in a5 and a4.
- `manage_structure_and_memory`: Calls a library routine, performs several checks on values within a structure in a5, calls another routine, calls a library routine to handle a value, and enters a loop that copies bytes from one structure to another and updates a pointer.
- `call_function_and_increment`: Loads a pointer from offset -16 of a structure in a5, calls a routine at an address derived from offset -1a0 of a structure in a4, clears the value at offset -16 of the structure in a5, increments a value at offset -4 of the structure in a5, and branches.
- `divide_and_update_list`: If a word at offset -2 of a5 is positive, it calls a routine, performs a division using a word from offset -2 of a5, and enters a loop that updates a pointer at offset -18 of a5 by loading values from it until a condition is met.
- `process_memory_and_tables`: Copies bytes in a loop, then performs multiple conditional checks involving tables and offsets to call various routines, eventually updating a longword in a structure in a5 and potentially calling another routine via a pointer in a6.
- `free_and_reallocate_memory`: If a pointer at offset -22 of a structure in a5 is non-zero, it calls a library function to free that memory; then, if a pointer at offset -26 of the structure is non-zero, it calls another library function using a pointer from offset 4 and a constant of 260 in d0.
- `copy_string_and_test_flags`: It copies bytes from a location derived from a4 to a location derived from a5 in a loop, then checks a word at offset 8 of a5 and a byte at a negative offset of a4 to conditionally clear a byte at a negative offset of a5.
- `copy_and_call_routines`: Copies a string from a location derived from a4 to a location derived from a5 in a loop, then calls two different routines with various arguments on the stack.
- `call_dual_subroutines`: Pushes two longwords onto the stack and calls a subroutine, then if the result in d0 is not zero, pushes two more longwords and calls another subroutine.
- `copy_data_and_call_library`: Copies bytes from a source at offset -25484 of a4 to a destination in a3, then copies bytes from offset -80 of a5 to offset -18047 of a4.
- `call_library_and_store`: Calls a library routine using a pointer in a6, pushes two values onto the stack, calls another routine, stores the result in a structure in a5, and then conditionally calls a routine before returning.
- `copy_string_loop`: Copies bytes from a source address derived from a5 to a destination address derived from a4 in a loop, then loads a value from offset -164 of a5 into d0 before returning.
- `execute_multiple_subroutine_calls`: Performs three separate calls to a subroutine with different parameters pushed onto the stack, then decrements a word in a5 and branches if it is not zero.
- `evaluate_bitmask_conditions`: Tests a byte at offset -7 of a5, performs bitwise operations to derive d5 and d6, and if d6 is not zero, compares d7 with d5 and branches accordingly.
- `loop_write_to_table`: Enters a loop that, while d5 is not negative, calculates a table address by adding d7 to a base address from offset 5a88 of a structure in a4, decrements d7, and writes the byte from d6 to that address.
- `check_register_and_branch`: Compares d7 to zero and branches if it is less than or equal, otherwise it sets d0 to zero and branches.
- `update_conditional_table`: Sets a word at a negative offset of a4 to zero, then enters a loop that reads a byte from a table at offset 23176 of a4 and another byte from offset -18388 of a4, and if the second byte is not 255, writes a word to a table at offset 21224 of a4 and increments a counter.
- `process_conditional_table_entry_2`: Calls a subroutine with parameters from a4 and a5, compares the result with a doubled word from offset -24794 of a4, and calls another routine if they are not equal.
- `copy_bytes_in_loop_3`: Enters a loop that copies a byte from one location to another, where the locations are calculated using a base address from offset 23176 of a structure in a4 and a value in d6, until d7 reaches zero.
- `loop_calculate_and_write`: Enters a loop that, while d7 is not zero, calculates three different addresses based on d7 and a base address from offset 21224 of a structure in a4, then writes a value derived from subtracting a word at the first address from 6000 to the second address.
- `restore_registers_d7_short`: Restores registers d5, d6, and d7 from the stack and returns.
- `scale_and_copy_bytes`: Enters a loop that copies bytes from one location to another, where the locations are calculated using a base address from offset 23176 of a structure in a4 and values derived from d7 and d6, until d7 reaches 5976.
- `increment_and_branch_7`: Adds 100 to register d7 and branches to another routine.
- `calculate_scaled_value`: Decrements a 16-bit value from a global variable, calculates a value using division and multiplication based on two memory locations at offsets from a4, and stores the result in a memory location at an offset from a1.
- `restore_registers_a2`: Restores registers d2 through d7 and a2 from the stack and returns.
- `calculate_and_store_values_2`: Loads bytes from a structure at a4, performs arithmetic to calculate four values, stores them in a structure at a5, and branches to another routine based on comparisons of d6 and d2.
- `setup_structure_and_branch`: Links a5, saves a6 to the stack, writes two 16-bit values to offsets -78 and -76 of a structure in a5, compares a value at offset 12 of the structure to 1, and branches to another routine if they are not equal.
- `initialize_and_call_library`: Copies 11 longwords from a global variable to a location at offset -74 of a structure in a5 using a loop, sets several fields in the structure in a5 with a pointer and a constant, and calls a library function using a pointer from offset -25272 of a4.
- `process_conditional_table_entry`: Calls another routine, performs two library calls using a pointer in a6, and if a word from a4 is not zero, enters a loop that writes a byte to a table location calculated from that word.
- `loop_data_processing`: This routine contains a loop that compares d7 to a constant, reads bytes from two different memory locations using offsets from a4, and if a byte is not 255, it updates a value in a global variable and a value at an offset in a4 before branching back to the start of the loop.
- `process_data_loop`: Performs several comparisons and conditional branches, calls a routine with values from the stack, and enters a loop that performs arithmetic on registers and increments values in a table at an offset in a4.
- `increment_and_jump`: Adds 1 to register d7 and branches to another routine.
- `call_event_handler`: Checks if a longword at offset -10 of a5 is zero, otherwise it calls a routine via a pointer at offset -24778 of a4 and returns.
- `jump_to_address_9`: Jumps to the address contained in the longword at 4224.
- `jump_to_indirect_address_35`: Returns immediately by jumping to the address stored in a pointer.
- `jump_to_subroutine_8`: Calls another routine via a long jump.
- `jump_via_register`: Jumps to the address stored in a longword in d2.
- `jump_to_indirect_address_17`: Performs a long jump to the address stored at the specified location.
- `jump_to_absolute_address_8`: Performs an indirect jump to an address stored at a longword offset.
- `jump_to_distant_address`: Performs a long jump to a different memory location.
- `jump_to_address_10`: Jumps to the address contained in the longword at 7888.
- `jump_to_address_11`: Jumps to the address contained in the longword at the location pointed to by a0.
- `jump_to_indirect_address_32`: Performs an indirect jump to an address stored in a longword at a specific location in memory.
- `jump_to_address_12`: Jumps to the address contained in the longword at 3072.
- `jump_to_address_28`: Performs a long jump to the address stored at the location 2016.
- `jump_via_pointer`: Jumps to the address stored in a longword at offset 41e.
- `jump_via_pointer_2`: Jumps to the address stored in a longword at offset 3be.
- `call_library_function_5`: Performs a jump to an address located at an offset of minus 306 from a library base stored in a global variable.
- `jump_to_long_address_5`: Performs a long jump to a destination address stored at a specific memory location.
- `jump_to_address_13`: Jumps to the address contained in the longword at 7964.
- `jump_to_zero_4`: Performs an indirect jump to an address stored at zero.
- `jump_to_indirect_address_25`: Performs an indirect jump to a location stored in memory.
- `jump_to_push_registers_and_call_2`: Jump stub: passes control straight on to push_registers_and_call_2.
- `jump_to_constant_address`: Performs a long jump to an address stored at a specific location.
- `jump_to_address_26`: Performs a long jump to a fixed address.
- `jump_to_handle_request_and_event_2`: Performs an indirect jump to another routine.
- `call_dos_write`: Performs a jump to an address located at offset 0 to call a routine that uses the DOS library base and provided arguments to write data.
- `jump_to_longword_address`: Loads the word value 56 into register d0 and calls another routine via a longword jump.
- `jump_to_address_14`: Jumps to the address stored in a specific location.
- `jump_to_indirect_address_34`: Performs an indirect jump to a long address stored in a memory location.
- `jump_to_indirect_routine_2`: Performs a long jump to an address stored in a pointer located in a data section and calls another routine.
- `jump_to_division_routine`: Performs a long jump to a routine that divides a longword by 100 after extending a word from an offset of a5 and adding a word from another offset.
- `jump_to_indirect_address_10`: Performs an indirect jump to an address stored in a pointer located in data memory, which calls a routine that performs division and a loop of additions and comparisons.
- `call_alloc_mem_5`: Performs a jump to an address located at offset 0 to call a routine that uses the Exec library base and provided arguments to allocate memory.
- `jump_to_longword_address_2`: Jumps to a longword address after loading the word value 216 into register d0.
- `jump_to_hardware_register_7`: Performs a long jump to the address at 1330.
- `test_byte_and_return`: Tests a byte at offset -17122 of a4 and branches if it is zero; otherwise it returns.
- `calculate_and_refresh_display`: Loads a pointer from offset 4 of a7 and a word from offset 8 of a7, decrements d0, saves registers, clears a global byte, copies two words from offset -24898 of a4 to global variables, and enters a loop to compare values from a structure in a0 against constants.
- `update_display_parameters`: Performs several conditional checks on values in d4, d5, d6, and d7, using division and subtraction to calculate new 16-bit values which are then stored in a table at various offsets.
- `manage_intuition_window`: Saves registers, performs several comparisons and subtractions using values from a4, stores words into a table, calls a routine via a pointer, calls another routine with three arguments, calls a library routine, and calls a third routine via a pointer.
- `check_bits_and_call_library`: Saves registers, loads a pointer from a specific offset of a4, checks bit 18 and bit 19 of the longword at offset 24 from that pointer, and if neither bit is set, calls a library routine via a pointer in a6 before branching to another routine.
- `store_stack_values`: Copies two words from the stack into memory locations, pushes a pointer to the stack, calls another routine, and returns.
- `evaluate_table_conditions`: Checks several constants against a value, and if a byte from a5 is 3, performs a series of arithmetic operations involving multiplication and table lookups at offsets from a4 to set bits in d7.
- `load_constant_and_jump_2`: Loads the word value 216 into d0 and jumps to a longword address.
- `load_constant_and_jump_3`: Loads the word value 56 into d0 and jumps to a longword address.
- `load_constant_and_jump_4`: Loads the word value 176 into d0 and jumps to a longword address.
- `modify_byte_via_offset`: Loads a pointer from an offset of a5, retrieves a byte from that pointer, adds it to a memory location at an offset of a4, and if bit 1 of the retrieved byte is set, it subtracts 32 from the byte and branches.
- `increment_byte_at_pointer`: Loads a pointer from an offset of 8 in a5, increments the byte at that location, and returns after unlinking a5.
- `calculate_and_store_structure_values`: Divides a 16-bit value from offset 8 of the structure in a5 by 100, subtracts a value from a global variable, scales the result by 16 and adds 23, then stores this at offset 10 of a pointer from offset 10 of the structure in a5, and finally performs a second calculation using offset 8 to update offsets 0, 2, 4, and 6 of that same pointer.
- `update_pointer_fields`: Subtracts a global variable from a 16-bit value at offset 10 of the structure in a5, scales the result by 16 and adds 23 to store at offset 12 of a pointer from offset 12 of the structure in a5, then performs a second calculation using offset 8 to update offsets 0, 2, 4, and 6 of that pointer.
- `calculate_division_result_2`: Loads a word from offset 8 of a5, subtracts 23, extends the result to a longword, divides it by 16, adds a word from a specific offset of a4, and then either branches to another routine or sets register d1 to 1 based on the result.
- `calculate_modified_division`: Performs a series of subtractions, extensions, and divisions on d0 using d2 and d3, then compares the result against 98 and 1 to determine the next branch.
- `divide_and_store`: Subtracts 23 from d0, extends d0 to a longword, divides it by the word in d2, adds the word in d3 to the result, and stores the longword result in d1.
- `calculate_division_result`: Moves a word from an offset of a5 into d0, performs a long division using d2 and d3, adds a word from a global variable to d2, stores d0 at an offset of a5, and branches based on a comparison of d2 with 1.
- `perform_arithmetic_and_branch`: Subtracts 20 from a word in d2, extends it to a longword, divides it by d3, adds d4 to the result, compares the result to 58, and branches based on the outcome; otherwise it repeats a similar sequence of subtraction, extension, division, and addition, or sets d2 to 1 and branches.
- `divide_and_store_result`: Subtracts sixteen from d0, extends d0 to a longword, divides it by the value in d3, adds the value in d4 to the result, stores the result in d2, and branches to another routine.
- `multiply_and_add_constant`: Multiplies the value in d2 by 100, adds the value in d1 to the result, and returns it in d0.
- `perform_indirect_calls`: If a word at a negative offset of a4 is not zero, it calls a routine, clears the stack top, calls another routine, pushes a word from a5 onto the stack, and calls a third routine before returning.
- `update_structure_value`: Calls another routine, then checks a byte at a pointer in a0; if non-zero, it increments the pointer in a5 and updates a 16-bit value at offset -2 of the structure in a5 using a bitwise operation.
- `toggle_boolean_flag`: Checks a 16-bit value at offset -2 of the structure in a5; if it is not zero, it sets the value to 1 and returns, otherwise it returns immediately.
- `compare_and_clear_flag`: Calls another routine, performs arithmetic and division on a value derived from a5, compares bytes at two calculated memory locations, and either clears a 16-bit value at offset -10 of the structure in a5 or branches to another routine.
- `add_and_branch`: Adds fourteen to d5 and branches to another routine.
- `calculate_offset_and_load`: Tests a word at a negative offset from a5 and branches if zero; otherwise it multiplies d4 by 256, adds this to a pointer derived from a4 and d0, adds a shifted version of d7 to that pointer, and loads a longword from the resulting address into d0 before branching.
- `divide_and_update_global`: Divides the value in d0 by thirty-two, stores the quotient in d7, and if d7 is not equal to d6, updates a global variable by dividing it by sixty-nine, stores the quotient back in that global variable, and then loads a value from a table into d0 using an offset derived from d1.
- `test_and_branch`: Tests a word at a specific offset of a4 and either branches to another routine or sets register d0 to 0 and branches to another routine.
- `compare_multiple_thresholds`: Compares values from a5 and various offsets in a4, performing several conditional branches based on whether these values are less than or greater than or equal to each other, and finally sets d0 to zero if all conditions are met.
- `increment_and_compare_value`: Sets a word at a negative offset from a5 to 1, copies it to d1, compares d1 to 8, and branches if greater; otherwise it doubles d1, calculates an address using a4 and d1, adds a word from an offset of a5 to the value at that address, compares the result to a word at another offset of a5, and branches if not equal or branches to another location.
- `increment_and_branch_4`: Increments a 16-bit value at a negative offset from a5 and branches to another routine.
- `set_d0_to_zero`: Sets d0 to 0 and returns.
- `divide_and_store_byte_2`: Divides a 16-bit value from offset 8 of a5 by 10, adds 48 to the result if it is non-negative, and stores the result in a byte at a location calculated from a4 and a local variable, then branches to another routine.
- `write_byte_to_global_offset`: Writes the byte 32 to a location pointed to by a global variable, then calculates a value by dividing a 16-bit value from offset 8 of the structure in a5 by d1, adds 48 to the high word of the result, and stores that byte at offset 1 of the pointer.
- `calculate_scaled_division`: Sets up a stack frame, saves registers, extends a word from an offset of a5 to a longword, divides it by 100, adds a word from an offset of a5 to the result, stores the word result at an offset of a5, and sets d2 to 1 if the result is less than 1.
- `jump_to_address_2`: Performs a long jump to the address contained in the constant 3072.
- `jump_to_address_36`: It performs a long jump to the address at offset 50.
- `jump_to_external_address_5`: Performs a long jump to a specific memory address.
- `jump_with_offset_24`: Performs an indirect jump to an address at offset 24 that sets register d0 to 24.
- `set_d0_and_jump_8`: Sets d0 to 24 and performs a long jump to an address at offset 24.
- `compare_and_branch_on_value`: Loads a word from a global variable into d7, decrements d7, and if d7 is not zero, compares a word from a table at an offset of a0 with a value at offset 8 of a structure in a5, then branches based on the result.
- `update_d7_value`: Loads a word from a table using a pointer in a4, compares it with d7, and if d7 is not higher and is at least 9724, adds 20 to d7 and branches.
- `update_offset_value`: Adds 20 to d7, compares the result with a 16-bit value at a negative offset from a4, and if the result is not higher, stores the result back into that same offset from a4 before restoring registers and returning.
- `compare_and_set_d0`: Loads a word from a location relative to a4 into d1, and if d1 is not negative, loads a word from a structure in a5, doubles it, and compares it with a word from a location relative to a4; if they are equal, it branches, otherwise it sets d0 to 1 and branches.
- `execute_complex_table_logic`: Performs a series of calculations involving doubling a word from offset 10 of a5, indexing into a table relative to a4, and updating bytes at offsets minus 11, minus 12, and minus 13 of a5; it conditionally calls several routines, performs divisions, and updates various memory locations based on comparisons of bytes and words from a5 and a4.
- `validate_and_clear_byte`: Loads a byte from a location relative to a5, extends it, and uses it as an offset to check a byte at a location relative to a4; if certain conditions are met involving values in a5 and a4, it clears a byte at a location relative to a4.
- `restore_registers_and_a6`: Restores d2 through d7 and a6 from the stack, unlocks a5, and returns.
- `process_table_and_write`: Calls several routines to perform divisions, checks values in a table relative to a4, and conditionally writes data to memory or calls another routine based on comparison results.
- `calculate_offset_and_write`: Multiplies d6 by 200, uses the result to find an address relative to a4, adds a value from an offset of a5 to that address, and writes 1 to that location.
- `check_table_and_call`: Uses d5 to find an address relative to a4, checks a byte at that location, and if it is 255, pushes d6 and d7 to the stack and calls another routine.
- `fetch_and_call_subroutine`: Extends d5 to a longword, uses it to fetch a byte from a table at an offset of a4, and if that byte is not less than two, fetches another byte from a table using d5 as an offset and calls another routine with d0, d6, and d7 on the stack.
- `compare_and_store_word_2`: Calculates an offset into a table using a value from a structure in a5, then compares a word from a structure in a5 with the word at that table location.
- `conditionally_write_word`: Sets d5 to 20, and if a word at an offset of a3 is not non-zero, writes the word from d7 to that location before branching.
- `loop_and_copy_word`: Decrements d5 and, if not zero, uses the new value to copy a word from an offset of a3 to another offset of a3 in a loop.
- `store_d7_to_pointer`: Stores the 16-bit value from d7 into the memory location pointed to by a3, restores d2, d5, d6, d7, and a3 from the stack, unlocks a5, and returns.
- `calculate_table_offset`: Calculates an offset into a table using values from a structure in a5, then compares a word from a structure in a5 with the word at that table location.
- `conditionally_write_word_small_count`: Sets d4 to 5, and if a word at an offset of a3 is not non-zero, writes the word from d7 to that location before branching.
- `loop_and_copy_word_alt`: Decrements d4 and, if not zero, uses the new value to copy a word from an offset of a3 to another offset of a3 in a loop.
- `store_d7_to_pointer_alt`: Stores the 16-bit value from d7 into the memory location pointed to by a3, restores d2, d4, d5, d6, and d7 from the stack, unlocks a5, and returns.
- `calculate_quotients_and_subtract`: Performs four division operations using values from offsets 8 and 10 of a5 and a constant, stores the results in d7, d6, d5, and d4, and then subtracts d5 from d7, negating the result if it is not positive.
- `subtract_and_negate_2`: Subtracts a word in d5 from a longword in d0, subtracts a word in d4 from a longword in d1, and if the result in d1 is not positive, negates it.
- `calculate_subtraction_results`: Subtracts a word in d4 from a longword in d1, and if the result is not less than or equal to zero, subtracts a word in d5 from a longword in d0 and negates the result if it is positive.
- `subtract_and_branch`: Subtracts a word in d5 from a longword in d7 and branches.
- `subtract_and_negate`: Subtracts a word in d4 from a longword in d6, and if the result is negative, negates the result before branching.
- `calculate_subtraction_result`: Subtracts d4 from d6, stores the result in d0, restores d2, d4, d5, d6, and d7 from the stack, unlocks a5, and returns.
- `compare_values_and_clear_byte`: Saves registers to the stack, compares two words from a structure in a5, and if they are equal, writes 65535 to a location in a3 and clears a byte at an address calculated from a4.
- `calculate_and_dispatch_value`: Performs multiple divisions using a constant and values from a5, stores the results in a5, calculates a value based on comparisons and arithmetic, writes a word from a table in a4 to a location in a3, and calls another routine.
- `calculate_signed_difference_2`: Checks a word at an offset in a4, and if non-zero, calculates the signed difference between two words from a5 and branches based on the sign of the result.
- `calculate_signed_difference`: Subtracts a word from another, stores the result in d2, loads two words from a structure, calculates their difference, stores d2 at offset minus 10 of a structure, and then branches based on the sign of the difference.
- `update_range_limit`: Subtracts d0 from d1, moves the result into d3, stores d3 at offset minus 12 of a5, and if d3 is equal to d2 or greater than d2, it branches to another routine; otherwise it sets d2 to d3 minus 1 and, if d2 is not zero, branches to another routine; otherwise it sets d0 to 0 and branches to another routine.
- `calculate_and_store_result_2`: Tests a word at an offset of a5, calls two different routines, performs division and arithmetic on d0, and then potentially stores a value into a3.
- `modify_and_store_word`: Tests a word at offset minus 24796 of a4 and branches if non-zero, otherwise it subtracts d5 from a word at a3, masks the result, calls another routine, and then either branches or stores the result back to a3.
- `conditional_write_word`: Tests a word at an offset from a4; if it is non-zero, it branches, otherwise it writes a word of all ones to the location pointed to by a3 and branches.
- `compare_and_fetch_value`: Compares a word from an offset of a structure in a5 with a value from another offset of a5, and if the first value is less than or equal to the second, it performs arithmetic using d7 and a4 to fetch a word from a calculated address and branches.
- `calculate_and_store_result`: Calculates a value by shifting and subtracting d6 from d0, calls another routine with two values pushed to the stack, and if d0 is not zero, stores d4 into a location pointed to by a3 and sets d0 to 1.
- `calculate_difference_and_negate`: Tests a word at a negative offset of a4, and if zero, calculates the difference between two words from a5, then either branches or negates the result.
- `update_and_branch_logic`: Subtracts d0 from d1, compares d2 to 1, and if d2 is greater than 1, performs arithmetic using d4 and a pointer in a3 to update a value at that pointer with a masked result, pushes two values onto the stack, calls another routine, and then conditionally sets d0 to 1.
- `update_word_and_branch`: Tests a word at an offset from a pointer in a4; if non-zero it branches to another routine, otherwise it writes 65535 to the address in a3 and branches to another location.
- `clear_byte_at_calculated_address`: Calculates an address by adding two offsets from a4 to a pointer, clears the byte at that address, restores d2 through d7 and a3 from the stack, unlocks a5, and returns.
- `calculate_quotients_and_check_bounds`: Loads a word from an offset of a5 into d7, performs two divisions of d7 by 100 to obtain quotients and remainders in d6 and d5, and branches to another routine if d6 is 0 or 59, or if d5 is 0 or 99; otherwise it clears d0 and branches to another location.
- `set_d0_and_restore_registers`: Sets d0 to 1, restores d5, d6, and d7 from the stack, unlocks a5, and returns.
- `process_data_with_loop`: Loads a word from an offset of a5 into d7, sets d5 to 9, and enters a loop that calculates an offset using d5, reads a byte from a table, and calls another routine if a condition is met, otherwise it increments d6 and branches back.
- `move_d6_to_d0`: Moves d6 into d0, restores d5, d6, and d7 from the stack, unlocks a5, and returns.
- `update_values_in_loop`: Loads a word from an offset of a5 into d7, clears two words at offsets of a5, and enters a loop that calculates an offset, calls another routine, and if the result is zero, increments a word at an offset of a5 before branching back.
- `increment_pointer_value`: Loads a pointer from an offset of a5 into a0, increments the word at that address, and branches to another location.
- `restore_registers_and_unlock`: Restores registers d5 through d7 from the stack, unlocks a6, and returns.
- `evaluate_conditional_parameters`: Loads three 16-bit values from a structure in a5 into d7, d6, and d5, then branches based on d6 and d7 to either return immediately or set d0 to 2.
- `check_memory_and_branch`: Links a5, saves d2, d3, and a2 to the stack, loads a word from an offset of a5 into d0, performs arithmetic on d0, tests a word at a calculated address, and branches based on the result.
- `perform_multiple_lookups`: Checks a word at an offset of a5, then performs two sequences of lookups and calls to another routine, branching based on the result of a comparison.
- `load_constant_and_branch`: Loads the word 1000 into d0 and branches to another location.
- `execute_multiple_lookups`: Performs several lookups using values from a5 and offsets of a4, calling another routine multiple times to process the retrieved data.
- `load_constant_and_restore`: Loads the word value 1000 into d0 and restores registers d2, d3, and a2 from the stack before returning.
- `update_structure_from_memory`: Loads a 16-bit value from offset 8 of a structure in a5, compares it to 1, and if equal, stores 2 in offset -2 of the structure and a value from offset -4894 of a4 into offset -4 of the structure before branching.
- `set_conditional_values`: Compares d1 to 2 and branches if not equal; if equal, it sets d0 to 1 and writes it to an offset of a5.
- `perform_complex_math`: Calculates a value through a series of multiplications, divisions, and additions using data from several memory locations, then stores the final result in d0.
- `process_scaled_values_2`: Subtracts 36 from a word at offset 12 of a5, uses the result to index a table via a4, adds a word from a pointer in a5 to a value from that table, stores the results in offsets -10 and -2 of a5, and branches if a value at offset -24636 of a4 is non-zero or if d2 is negative or less than 5968.
- `compare_division_results`: Calls another routine twice using values from d1 and offset minus 2 of a5, then performs two divisions using a word from offset 8 of a5 and a constant 100, compares the results, and if they are not equal, sets d0 to zero and branches.
- `evaluate_and_set_d0`: Pushes d2 onto the stack, calls another routine, and if d0 is non-zero, sets d0 to zero and branches; otherwise it continues.
- `update_structure_values`: Copies words between offsets of a structure in a5 and a0, calls another routine, and then compares a word from a4 against 58 to set d1 to 58 or 1 based on the result.
- `clamp_and_store_values`: Compares a word from a global variable against 58 and 98, then stores a modified version of the value into a structure in a5.
- `calculate_and_call_division`: Performs several arithmetic operations including multiplication and division on values from a structure and a global table, then calls another routine with the results on the stack.
- `execute_multiple_subroutines_3`: Performs a series of divisions and calls to two different routines using values and pointers derived from a structure in a5.
- `jump_to_secondary_routine`: Calls a routine that calculates a value by adding a word from a table to a word from a structure in a5, uses the result to index into another table to retrieve a byte, and then performs a long jump to another routine.
- `jump_to_complex_multiplication`: Transfers control to another routine after moving two values from registers d2 and d3 into address registers a0 and a1.
- `jump_to_external_address_2`: Performs an unconditional jump to a long address.
- `jump_to_address_30`: Performs an indirect jump to the address stored at a specific location.
- `jump_to_subroutine_7`: Performs an indirect jump to a location stored in a longword at a specific memory address.
- `jump_to_indirect_address_26`: Performs an indirect jump to the address stored at the specified location.
- `jump_to_subroutine_2`: This routine performs a long jump to another routine.
- `jump_to_absolute_address_6`: Performs an unconditional jump to a longword address.
- `jump_with_d0_set_224`: Performs an indirect jump to an address at offset 24 that sets register d0 to 224.
- `jump_to_address_20`: Performs a long jump to the address stored at offset 1e0c.
- `jump_to_absolute_address_3`: Performs an absolute long jump to a specific address.
- `jump_to_absolute_address_7`: Performs an absolute longword jump to a specific address.
- `set_d0_and_jump_9`: Sets d0 to 224 and performs a long jump to an address at offset 24.
- `call_with_stack_parameters`: Links a5, pushes two pointers and two words from a5 onto the stack, calls another routine, and then unlinks a5 before returning.
- `push_parameters_and_call_subroutine_2`: Pushes two pointers and two words from the stack frame onto the stack and calls another routine, then restores the stack pointer and returns.
- `push_parameters_and_call_subroutine_3`: Pushes two pointers from a table into the stack, followed by two 16-bit values from the stack frame, and calls another routine.
- `push_parameters_and_call_routine_2`: Pushes two longwords and two words onto the stack and calls another routine, then adjusts the stack pointer and returns.
- `calculate_and_store_pointer`: Links a5, saves d7, performs arithmetic to calculate an address based on values from a4 and a5, and stores several values into a table in a4 before comparing a word from a5 with a value in d0.
- `calculate_clamped_index`: Pushes two words onto the stack, calls another routine, performs arithmetic on the result, and if the value is less than 125, branches to an earlier instruction; otherwise it sets d0 to 125 and branches.
- `calculate_and_store_index_3`: Calls another routine, performs arithmetic on the result to calculate a new index, stores this index into a memory location via a pointer, and branches based on whether the index is less than 10000.
- `increment_d0_from_d7`: Copies the value of d7 into d0, increments d0 by 1, restores d7 from the stack, unlinks a5, and returns.
- `update_table_and_call`: Links a5, saves d7, writes a word to a table in a4, writes a longword to a table in a4, calculates an address using a value from a4 and d1, writes a word and a longword to a table in a4, and then branches based on a comparison between d0 and a word from a5.
- `clamp_and_call_routine`: Pushes two words to the stack, calls a routine, and then performs arithmetic on the result in d1, clamping the value to 125 if it exceeds that threshold before branching.
- `update_and_call_handler`: Pushes two words onto the stack, calls another routine, performs arithmetic on the result, pushes two more words onto the stack, calls another routine, and then stores a word from a structure into a location pointed to by another structure before checking if a value is less than 10000.
- `increment_register_value`: Copies a value from a register to a destination register, increments that destination by one, restores the previous register state from the stack, and returns.
- `calculate_quotient_difference`: Performs several divisions using values from a structure in a5 and a constant, calculates a difference, and if the result is negative, negates it before returning.
- `negate_and_branch`: Copies d7 to d0, tests d6, and if d6 is negative, copies d6 to d1 and negates it before branching.

The function list continues in: g2219-recovery-functions-2.md.
