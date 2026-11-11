# Game 2222 (empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02222). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (909856 top-level bytes in 128 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 59 of 59 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 2220 of 2220 named (0 author symbols), 2220 of 2220 described (306 by rule, 1914 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3939 of 3939 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 42724 of 42726 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 288015 |
| code | 137136 |
| data | 424889 |
| padding | 25512 |
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
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf:Preferences | 45 | 6587 | 222 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf:Empire | 33 | 19031 | 1003 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf:CLI | 9 | 362 | 18 |
| icon.library | 15 | 1414 | 108 |
| info.library | 25 | 2476 | 170 |
| version.library | 2 | 41 | 7 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1754 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 323 | 8 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf bytes 103936-104448 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf bytes 104960-105472 (code never loaded by a reference run) | 1 | 0 | 0 |
| Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf bytes 536576-537088 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: file devs/narrator.device (header 23280 bytes, blocks hold 4880): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1110 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 120 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: bytes 103936-104448 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: bytes 104960-105472 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: bytes 536576-537088 look like code no run loaded: recovered position-independently (load address unknown)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: 3 logged loads cover 1536 bytes of the image
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: blocks 143-143 (bytes 73216-73728) outside the filesystem hold data, not code (opcode density 0.035, entropy 4.06)
- empire - war game of the century v2.05 (1987)(interstel corporation)[cr][a]/Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf: blocks 154-197 (bytes 78848-101376) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.17)
- 3 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `copy_data_loop`: Calls a routine with an argument, then performs a table lookup that selects a value, copies data between two regions using a loop, and finally restores the stack and returns.
- `copy_register_branch`: Copies the data register into the return register and then branches to a nearby routine, so it performs no work of its own.

### More

- `parse_command_line_args`: Initializes the system by loading the library base, then copies a command-line string into a buffer while converting quoted text and special characters into single tokens, and finally calls the handler routine with the resulting argument count.
- `check_flag_return_early`: Loads a pointer from the stack and checks a flag at offset 20 of a global structure, returning early when the flag is clear or the high bit of a second pointer is set.
- `call_and_push_value`: Loads a pointer from offset 92 of a6, calls a routine at offset -36, then loads a stack pointer and pushes the incoming value onto the stack.
- `call_library_store_result`: Loads a pointer from a table, clears the return register, calls a library function, stores the result into a longword, then returns early when the result is zero.
- `store_and_advance_pointer`: Loads a pointer from the stack, writes it into the pointed-to location, increments it, clears the next slot, and stores it again.
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
- `handle_more_command`: Sets up a frame, checks a flag and returns one or zero accordingly, then parses command-line arguments, opens and reads files, formats output, and dispatches based on a key press before restoring registers and returning.
- `read_and_store_byte`: Calls a read routine and a seek routine, then stores the returned byte at offset -5 of a6 and branches based on its value.
- `call_and_dispatch_result`: Stores one into a global long word, calls a routine and saves its byte result, then indexes a jump table using that value and branches to the matching entry when it is in range.
- `set_d0_bits_and_jump`: Sets bits 0,3,4 and 6 of register d0, then jumps to the routine at the fixed address.
- `format_and_delay_values`: Sets up a stack frame, pushes several pointers and the value negative one, then calls a routine to format a value.
- `store_value_compare`: Sets a flag, clears a word, calls a routine to obtain a value, stores it, then compares two fields and calls another routine when the first is larger before returning.
- `compute_position_write_data`: Pushes a table index and calls a routine to obtain a position, stores it, then pushes the index and a stored value to call a write routine before restoring the stack and returning.
- `seek_and_read_position`: Calls a seek routine and a read routine, storing the result in a global variable, then checks the byte at the current position.
- `format_and_increment_counter`: Creates a stack frame and clears a counter, then compares the counter with the value 11 and returns when it is greater or equal, otherwise it calls a formatting routine and increments the counter in a loop.
- `format_and_check_value`: Pushes a fixed address onto the stack and calls a formatting routine with it.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `format_value_from_struct`: Sets up a stack frame, calls a routine that formats a value, then pushes a pointer from offset 4 of the structure and a fixed address before calling a formatting routine through the exec vector table.
- `increment_and_format_value`: Loads a 16-bit value from offset 44 of the structure in a0, increments it by one, and passes the new value along with a fixed address to a formatting routine before returning.
- `init_task_and_format`: Sets up a stack frame and calls a routine to initialize a task, storing the result.
- `alloc_and_lock_file`: Allocates a 40960-byte block from the dos library and stores the pointer, returning early when the allocation fails.
- `read_and_compare_file_byte`: Reads a value from a file handle and stores it into a byte pointed to by a stack pointer, then loops comparing successive values until it reaches the newline byte or a value not greater than the previous one.
- `compare_routine_results`: Sets up a stack frame and calls a routine using a pointer from a structure offset, storing the result.
- `update_and_compare_counter`: Computes a running pointer from fields at offsets 8, 4 and 8 of the structure in a6, increments it, and stores it at offset 12.
- `increment_counter_and_branch`: Increments a long counter at offset 8 of the structure in a6, then branches back into an earlier routine.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `count_zeroed_buffer_bytes`: Increments a byte counter at offset 8 of the structure in a6, tests the byte it points to, and loops back while that byte is zero, otherwise incrementing a second counter at offset 4 of the local frame.
- `restore_stack_and_return`: Loads a long value from the stack at offset -4 from the pointer in a6 into d0, restores the stack pointer, and returns.
- `compare_counters_and_bytes`: Compares a counter at offset 4 of the structure in a6 with the value at offset 16 and returns zero when the counter is greater or equal.
- `increment_counters_and_branch`: Increments three adjacent long counters at offsets 4, 8 and 12 of the structure in a6, then branches back into an earlier routine.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a6, then returns.
- `cleanup_and_check_flags`: Checks a flag in a global structure and returns early when it is clear.
- `find_and_init_task`: Sets up a stack frame, clears a stack slot, and calls a routine to find a task, storing the result in a1.
- `allocate_and_init_task_2`: Allocates memory for a task and returns zero if the allocation fails.
- `allocate_and_init_task`: Allocates a task structure and, on success, copies pointers into its fields, then copies entries from one table into another using a loop indexed by a counter at offset 12.
- `format_and_count_bytes_2`: Saves a register, pushes two pointers onto the stack, calls a formatting routine, then restores the stack and the register before returning.
- `read_and_compare_result`: Calls a read routine with a file handle, then compares the returned value to one, returning negative one on mismatch.
- `format_and_count_bytes`: Copies incoming pointers from the stack into registers, calls a formatting routine, then loops over a string counting non-zero bytes.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `format_string_from_stack`: Saves several registers, loads three pointers from the stack and a text address, then calls a formatting routine before restoring the registers and returning.
- `store_byte_increment`: Stores the byte in d0 at the address pointed to by a3, then increments that pointer by one.
- `multiply_signed_16bit`: Multiplies two 16-bit unsigned integers from d0 and d1, combines the partial products into a 32-bit result in d0, and negates that result when the original operands had opposite signs.
- `compute_absolute_value`: Computes the absolute value of a 32-bit signed integer passed in d0 and stores the result in d1, negating the input when it is negative and leaving it unchanged when it is already non-negative.
- `allocate_and_init_task_struct`: Loads arguments from the stack, calls a task-lookup routine and branches to a return path when the result is negative, otherwise calls a series of library allocation and port-creation routines, stores their results into a structure, and returns the structure pointer.
- `cleanup_and_free_resources`: Checks a flag at offset 10 of a2, and if set, calls a cleanup routine, sets a byte to 255, and stores signal and port values.

### Stack

- `dispatch_table_handler`: Resolves a jump table using a value from a global variable, saves the caller's registers on the stack, and branches to the selected handler routine, restoring the registers on return.
- `copy_d1_to_d0_2`: Copies the value in d1 into d0, then jumps to the start of copy_data_loop, which performs a table lookup to select a value and copies data between two regions using a loop.

### Else

- `resolve_table_and_copy_blocks`: Loads a pointer, calls a routine, and saves a value.
- `copy_register_and_branch`: Copies the data register d1 into d0 and branches to a nearby routine.

### Delete

- `dispatch_and_copy_data`: Saves d2, loads a routine pointer from a table using a value from offset 356 of the structure in a2, calls the library at offset 12, then branches into a table-driven dispatch that copies data and manages a buffer before returning.
- `copy_d1_to_d0_branch`: Copies the value in d1 into d0, then branches forward into the middle of another routine without returning.

### type

- `copy_data_and_store_pointer`: Calls a routine with an argument, then performs a table lookup that selects a value, copies data between two regions using a loop, and finally stores a pointer and calls the routine again before returning.
- `move_d1_to_d0_4`: Copies the byte in d1 to d0 and then jumps into the middle of another routine.

### Path

- `copy_block_via_loop`: Copies a 16-bit value from a global variable into the structure in a2, then calls a routine with argument 12 and stores the result.
- `move_d1_to_d0_5`: Copies the byte in d1 to d0 and jumps back into the middle of copy_data_loop.

### EndCLI

- `copy_block_from_globals`: Copies a 16-word block from the source pointed to by a global variable at offset 536 to the destination pointed to by a global variable at offset 8, then copies a 16-word block from a global variable at offset 536 to the buffer pointed to by a data register.
- `move_d1_to_d0_6`: Copies the value in data register d1 into data register d0, then branches to a routine located elsewhere.

### Dir

- `handle_cli_lookup`: Looks up a value in a table indexed by a register, calls a library routine with a fixed argument, and branches based on the result.
- `move_d1_to_d0_7`: Copies the value in d1 into d0 and then jumps into the middle of another routine, which continues the data-copy loop.

### NewCLI

- `copy_register_branch_2`: Copies the data register d1 into the return register d0, then branches to a nearby routine, performing no work of its own.
- `copy_byte_to_d0`: Copies the byte in d1 to d0 and then branches to the caller's loop.

### Echo

- `resolve_and_install_handler`: Calls a routine with an argument, then follows a chain of pointers using a table lookup that selects entries by index and repeats until a zero entry is found.
- `transfer_register_value`: Copies the 32-bit value in register d1 into register d0 and then branches to the surrounding routine, which continues by copying data between two memory regions.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `copy_data_block`: Looks up a value in a table using a register index, then copies a block of longwords from one memory region to another while writing a header, and finally calls a routine with an argument.
- `transfer_d1_to_d0`: Copies the value in register d1 into register d0 and then branches to the routine that called it, which continues its data-copy loop.

### Execute

- `copy_data_blocks`: Copies a 16-word block from the source address in d5 to the destination address in a0, then writes a 32-bit value of an address to the following word of the destination and increments the destination pointer by two.
- `transfer_d1_to_d0_2`: Copies the 32-bit value from register d1 into register d0, then branches to the calling routine to continue its execution.

### makedir

- `create_directory_entry`: Copies a 16-character directory name from a source buffer into a destination buffer, writes the DOS command signature a fixed address followed by a plus sign into the destination, and then calls a system routine with the directory name and a flag.
- `move_d1_to_d0`: Copies the value in register d1 into register d0 and then jumps into the middle of another routine.

### CD

- `copy_table_block`: Copies a 16-word table from the address at offset 536 of the structure in a2 into the 16 words starting at offset 536 of the same structure using a loop, then copies the 16 words from that location into the 16 words pointed to by d1.
- `move_d1_to_d0_2`: Copies the value from register d1 into register d0 and then branches to the start of the copy_data_loop routine.

### EndIf

- `clear_register_d0`: Clears the 32-bit register d0 and returns immediately.

### If

- `copy_table_blocks`: Copies a 16-word table from a global variable into a second global variable, then copies a 16-word table from a third global variable into a fourth, and finally returns.
- `copy_d1_to_d0`: Copies the value in data register 1 into data register 0, then branches unconditionally to the routine at offset 104.

### LoadWB

- `parse_command_line`: Saves the caller's return address and a couple of registers, initializes the exec library base, then parses the command-line arguments into tokens before calling the program's main routine.
- `switch_stack_call_exec`: Saves the caller's return address, switches the stack pointer to a temporary stack, and calls three exec library routines by their negative offsets before restoring the original stack and returning.
- `call_library_routines`: Loads the address at offset 92 from a4 into a0 and calls two different library routines by their negative offsets, then returns.
- `call_exec_and_store_result`: Loads a text address into a register, calls an exec library routine, stores the returned value into a global long, and returns immediately when that value is zero.
- `invoke_initial_routine`: Reads two long-word arguments from the stack at offsets 8 and 12, then calls the routine at offset 10_00D0 with a zero and a string pointer, storing its result in a local slot.
- `compare_byte_strings`: Compares two null-terminated byte strings from the addresses in a0 and a1 and returns 0 if they are equal, 1 if the first string is greater, or -1 if it is lesser.
- `allocate_and_init_table`: Allocates a block of memory, walks through it in four-byte steps calling a helper routine for each slot, and stores the resulting table pointer and size into the structure pointed to by a4.
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
- `compare_byte_sequences`: Compares a2 with the sentinel value and returns immediately when they match.
- `restore_registers_and_return_21`: Loads a text address into a register, restores two registers from the stack, pushes a return address onto the stack, and returns.
- `invoke_format_and_count`: Saves the value in register d2 onto the stack, copies the pointer at offset 8 of the stack pointer into d2, pushes that pointer as an argument, restores the saved d2, pushes the address of a global variable as a second argument, and calls a routine that formats and counts bytes, then cleans up the stack and returns.
- `count_bits_format_output`: Counts the number of set bits in a 32-bit value from d0 by testing each byte, then pushes the bit count and a format string and calls a formatting routine.
- `store_byte_advance_pointer`: Copies the byte in d0 to the location pointed to by a3 and advances that pointer by one, then returns.

### Wait

- `dispatch_via_table`: Reads a pointer from offset 356 of the structure in a2 and calls the routine at a5 with argument 12, then looks up an entry in a table using a value derived from a constant and the stored argument.
- `move_d1_to_d0_3`: Copies the 32-bit value from register d1 into register d0 and then jumps to the routine that performs the data copy.

### Port-Handler

- `initialize_port_handler`: Initializes a large data structure by filling many fields with computed addresses and sizes, then dispatches to different code paths based on the values found in the structure's status fields.

### Disk-Validator

- `initialize_global_state`: Initializes a global structure by storing a computed pointer at offset 4 and a small counter at offset 12, then branches based on a value read from offset 744 of another structure.

### Ram-Handler

- `invoke_copy_mem`: Copies the three data arguments from registers into local pointers, resolves the exec library base, and calls the CopyMem routine before restoring the registers and returning.

### serial.device

- `empty_return_routine`: Contains no instructions and simply returns immediately.
- `open_serial_libraries`: Checks whether a pointer argument is null and otherwise returns, then opens two different libraries by their identifiers and stores the returned handles into a structure.
- `check_and_open_library`: Reads a byte from a device status register at offset 30, combines it with a constant, and if the result is nonzero it opens a library by its identifier before returning.
- `store_byte_and_jump`: Writes the byte 253 to the byte at offset 31 of the structure in a1, then jumps to the routine at 1206.
- `toggle_interrupts`: Writes a constant to the interrupt enable register to enable interrupts, then calls a nearby routine.
- `compare_serial_word`: Clears the byte at offset 31 of the structure in a1, then compares the word at offset 28 against the constants 2, 3 and 10, branching differently for each match.
- `clear_node_flag_and_pointer`: Compares a1 with the value at offset 74 of the structure in a6 and returns when they differ.
- `insert_node_into_list`: Tests bit 6 of the byte at offset 30 of the structure in a1 and returns immediately when that bit is clear.
- `compare_and_remove_node`: Compares a1 with the pointer at offset 110 of the structure in a6 and returns to the routine at 1446 when they differ.
- `remove_node_from_list`: Removes a node from a doubly linked list by rewriting the surrounding links, decrements a counter, sets status flags, and calls a library routine if a specific bit is set.
- `call_library_vector_480`: Library stub: loads a library base from a global variable and calls the library function at offset minus 480 of that base, returning its result to the caller.
- `check_flag_then_return`: Loads zero into d0, then tests the word at offset 32 of the structure in a6 and returns to the routine at 1552 when it is zero.
- `allocate_and_open_resources`: Allocates and frees two resources by calling a library routine with different identifiers, then opens two more devices by their identifiers before inserting a node into a linked list.
- `link_node_and_call_library`: Clears the long word at offset 32 of the structure in a1, loads the long word at offset 36 into d1, and returns to the routine at 1206 when that value is zero.
- `handle_serial_buffer_byte`: Saves the d2 and a6 registers and copies a1 into a6, then returns early when the long word at offset 74 is nonzero or the word at offset 70 is not less than zero.
- `shift_buffer_and_clear_flag`: Tests whether a counter is zero and returns when it is, otherwise decrements a second counter and shifts a byte through a circular buffer by copying the next byte to the front.
- `test_serial_bit0`: Tests bit 0 of the byte at offset 445 of the structure in a6, returns when it is clear, and otherwise branches to the routine at offset 134.
- `call_and_set_byte_flag`: Calls the routine at offset 1344 and returns when its result is zero.
- `search_table_and_notify`: Copies a flag byte from offset 102 of the structure in a6 into d0, clears the bit selected by that flag, loads the long word at offset 36 of the structure in a1, stores d0 into the byte pointed to by a pointer at offset 82, and returns immediately when the original flag was zero.
- `restore_registers_return`: Restores two saved registers from the stack and then returns.
- `process_byte_flag_check`: Transforms a byte argument using arithmetic and bitwise operations, reads a value from a structure, and returns without branching if a status bit is clear.
- `clear_serial_flag_byte`: Clears bit 6 of the byte at offset 470 of the structure in a6 and returns immediately when that bit was already clear, otherwise it stores the byte 15 at offset 31 of the structure in a1 and branches away.
- `clear_serial_bit7`: Clears bit 7 of the byte at offset 470 of the structure in a6, stores the byte 9 at offset 31 of the structure in a1 when that bit was set, and otherwise branches to the routine at offset 52.
- `set_byte_and_clear_flags`: Stores the byte 12 at offset 31 of the structure in a1.
- `handle_serial_interrupt`: Reads a word from the interrupt request register and masks it to 10 bits, then returns early when bit 4 of the structure byte at offset 69 is set.
- `enable_interrupts_bit2`: Compares the byte at offset 34 with d1 and returns when they differ, then tests bit 2 of the byte at offset 472 and returns when it is clear.
- `check_status_bits_and_call`: Tests bit 5 of the byte at offset 472 and returns when it is clear, then tests bit 1 of the byte at offset 445 and returns when it is set.
- `test_serial_status_bit`: Tests bit 0 of the byte in d0 and returns immediately when it is set, otherwise it branches away to continue processing.
- `test_bit_and_jump`: Tests bit 0 of the byte at offset 445 of the structure in a6, then jumps to the routine at 2370.
- `set_serial_bit7`: Sets bit 7 of the byte at offset 470 of the structure in a6, returns immediately when the long word at offset 62 is nonzero, and otherwise copies the long word at offset 42 into offset 62.
- `store_byte_and_advance_pointer`: Compares two long words at offsets 54 and 58 of the structure in a6 and returns without work when the first is not greater.
- `store_and_call_routine`: Returns immediately when the long word at offset 62 of the structure in a6 is nonzero.
- `copy_byte_and_clear_ciab`: Copies the byte at offset 34 of the structure in a6 into offset 361, clears bit 3 of the byte at offset 472, and returns when bit 2 of the byte at offset 69 is clear.
- `copy_flag_and_toggle_interrupts`: Copies the byte at offset 35 of the structure in a6 into the byte at offset 473, sets bit 3 of the byte at offset 472, and returns to the routine at 2570 when bit 2 of the byte at offset 69 of that structure is clear.
- `clear_status_byte`: Clears a single byte at offset 473 of a structure and then returns.
- `submit_request_and_set_status`: Tests bit 6 of the status byte at offset 470 and returns immediately when it is set.
- `read_serial_data_register`: Saves registers d2, a3 and a6, points a6 at the serial structure, clears bit 6 of its byte at offset 470 and returns when that bit was already clear, then reads the serial data register, masks its low ten bits and returns when the result is nonzero before setting bit 2 of the status byte and branching away.
- `load_address_and_call`: Loads the address of the structure at offset 242 of the structure in a6 into a0, calls the routine at 1512, and jumps to the routine at 2276.
- `set_serial_flag_bit`: Sets bit 0 of the byte at offset 470 of the structure in a6 and then branches away to continue processing.
- `swap_pointers_and_increment_2`: Clears the long word at offset 32 of the structure in a1 and returns when the long word at offset 36 is zero.
- `check_flag_and_enable_interrupts`: Tests bit 2 of the byte at offset 69 of the structure in a6 and returns to the routine at 2830 when it is clear.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_parallel_busy`: Reads the parallel port status byte and masks it, then returns immediately when the busy bit is clear and otherwise stores a command byte into a device structure field.
- `process_serial_status_bits`: Returns early when the byte at offset 473 is nonzero, then saves the d2 and a6 registers and copies a1 into a6.
- `load_byte_and_check_counter`: Stores the value 200 into the word at offset 476, loads a pointer from offset 86 and reads the byte it points into d0.
- `handle_serial_input_byte`: Compares a long word at offset 94 of the structure in a6 with the value in d1 and returns when the first is not less than the second, then sets an interrupt request bit, clears bit 4 of the status byte at offset 472, and branches based on the low bits of a masked value and on several status flags before falling through.
- `handle_serial_byte_and_call`: Calls a routine and returns when its result is zero.
- `check_and_store_pointer`: Checks whether the byte at offset 72 of the structure in a6 is zero and returns immediately if it is, otherwise it loads a pointer from offset 176, dereferences it, and stores that longword back to the same location.
- `enable_interrupts`: Restores two registers from the stack, then enables interrupts by writing a value with bit 15 set to the interrupt enable register before returning.
- `restore_interrupts_and_set_data`: Sets bit 15 of the interrupt enable register to restore interrupts, clears d1, reads a byte from offset 29 of the structure in a1, combines it with the value at offset 212, writes the result to offset 212, clears offset 29, and stores the value in the serial data register.
- `open_serial_device`: Enables interrupts, calls a setup routine, saves a1, and returns without work when either of two long-word pointers at offsets 74 and 78 is zero, otherwise it loads each pointer, calls an initialization routine for both, marks the structure, restores a1, fills a device buffer with fixed constants, calls two more routines, disables interrupts, and branches to a handler.
- `call_and_jump_routines`: Calls the routine at offset d86 and then jumps to the routine at offset 4b6.
- `copy_structure_fields`: Copies one longword field to a later field in a structure, copies the same source to another field, and clears a third field before returning.
- `test_and_set_flags`: Tests the longword at offset 78 of the structure in a6 and sets bit 2 of the byte at offset 296 when it is nonzero, then tests the longword at offset 74 and sets a flag when that value is nonzero.
- `clear_flag_and_enable_interrupts`: Clears bit 2 of the byte at offset 216 of the structure in a6 and loads a long word from offset 74.
- `toggle_interrupts_and_branch`: Sets an interrupt enable bit, calls a routine at offset de6, sets another interrupt enable bit, and branches to a routine at offset 4b6.
- `call_two_pointers`: Pushes a1 onto the stack, loads the pointer at offset 162 of the structure in a6, and calls the routine at offset 2308 before loading the pointer at offset 176 and calling it again.
- `store_pointer_and_call`: Loads a pointer from the address in a0, reads the longword it points to, and returns immediately if it is zero.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `invoke_library_if_ready`: Sets a control bit and writes a command byte to a device structure, then returns without doing anything if a status bit is already set and otherwise calls a library function through a base register.
- `configure_device_byte`: Reads a byte from a global table, masks it, and returns early if a bit in the processor status or a byte at offset 472 is clear.
- `check_flags_and_branch`: Clears bit 0 of the byte at offset 30 of the structure in a1 and tests bit 3 of the byte at offset 79.
- `call_and_configure_hardware`: Reads a longword from offset 6a of the processor structure and calls the routine at offset f1a.
- `call_library_clear_status`: Saves a base register on the stack, loads a library base from a structure field, calls the library function, restores the base register, and clears a status field before returning.
- `configure_device_if_size`: Clears a control bit and returns if it was already clear, then saves a base register, loads another structure pointer, and returns unless a size field matches a value before setting a bit, clearing a field, writing to a hardware register, and calling a library function.
- `init_device_via_library`: Enables interrupts, saves a base register, fills a structure with initial values, calls a library function, restores the base register, and stores a command byte if the function result is nonzero.
- `toggle_connection_and_call`: Tests bit 7 of the byte at offset 79 and returns when it is clear, otherwise sets bit 7 of the byte at offset 69 and clears bit 2 of the byte at offset 472.
- `check_serial_connection_state`: Clears bit 7 of the byte at offset 45 of the structure in a6 and returns when bit 1 of the byte at offset 296 is set or when the long word at offset 78 is nonzero or when bit 4 of the byte at offset 30 of the structure in a1 is set, then returns when the long word at offset 70 is zero, otherwise it stores the byte 1 at offset 31 of the structure in a1 and branches away.
- `enable_interrupt_and_branch`: Sets an interrupt enable bit, calls a routine at offset fce, clears that interrupt enable bit, and branches to a routine at offset 4b6.
- `validate_and_copy_fields`: Validates several byte fields and a longword field of the structure in a1 against fixed limits and returns early when any check fails, otherwise it copies those fields into the structure in a6.
- `build_flags_and_call`: Clears d0 and d1, reads a byte from offset 77 and sets a bit in d1.
- `index_jump_table`: Returns immediately if a status bit is set, otherwise doubles a value, uses it to index a jump table, and loads the selected entry into a register before returning.
- `set_flag_and_jump`: Sets bit 5 of the byte at offset 31 of the structure in a1 and then jumps to the routine at offset 10de.
- `compute_divisor_value`: Reads a longword from offset 294 of the processor structure, copies it to d1, multiplies d0 by 8, subtracts d1, and returns early if d0 is less than or equal to 65535.
- `set_divisor_conditionally`: Divides two values, stores the quotient in a structure field, reads it back, and conditionally sets a bit and writes a divisor register when a comparison matches and a status bit is set.
- `setup_serial_device`: Saves registers and loads two fixed addresses, then calls a library routine to set up a device, stores the returned base pointer and a flag word into a global structure, and branches away when the setup fails.
- `call_serial_setup_twice`: Loads two pointers from fixed addresses, calls a routine through the first pointer, loads a second pointer, calls the same routine through it, then branches forward into the middle of another routine.
- `store_register_value`: Stores a register value into a field of a structure and then returns.
- `write_serial_struct_fields`: Writes the value sixteen to the byte at offset nine of the structure pointed to by a0, stores the value fifty-six at offset ten, copies the pointer in a3 to offset eighteen, copies the pointer in a6 to offset fourteen, and returns.
- `init_and_call_serial_lib`: Initializes the structure pointed to by a1 by storing fifty-six at offset ten, zero at offset nine, one at offset fourteen, and four at offset eight, then pushes a6, loads a library base from a global variable at offset ninety-eight of a6, calls the library routine at offset minus one hundred sixty-two, restores a6, and returns.
- `init_devices_and_open_lib`: Initializes two device structures and calls their setup routines, then opens a library by base pointer and stores two handler addresses before enabling interrupts and returning one.
- `call_library_vector_162`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `clear_result_and_branch`: Clears the result register d0 to zero, then branches into the middle of another routine to continue its execution.

### dk

- `init_d0_and_skip_offset`: Performs a series of no-op or immediate byte operations on the data register d0 and subtracts zero from the word at offset forty-eight of the structure in a6 indexed by d0, then falls through into the following routine without returning.

### is

- `init_d0_and_predecrement_a6`: Performs three no-op operations on the data register d0 and subtracts zero from the word pointed to by the predecremented a6, then falls through into the following routine without returning.

### d

- `clear_d0_and_fall_through`: Clears the data register d0 to zero, then falls through into the following routine without returning.

### e

- `store_keymap_bytes`: Writes the byte sequence 0, 0, 0, 196, 46, 61, 140, 196, 54, 69, 3 through a pointer in d0, and additionally stores a zero at the 16-bit word located 76 bytes past the value in a6.

### f

- `noop_stall_routine`: Performs a series of no-op byte operations on register d0 and a no-op subtraction on the stack pointer, then returns.

### cdn

- `store_keymap_bytes_2`: Writes the byte sequence 196, 46, 61, 140, 196, 54, 69, 3 into a byte table starting at the address in a0, then stores 0 in the 16-bit value at offset 76 of the structure in a6.

### usa0

- `load_first_char_ascii`: Loads the ASCII code of the first character in the string pointed to by a0 into the low byte of d0, then returns.

### i

- `noop_then_decrement_pointer`: Performs three no-op OR immediate byte operations on register d0, then subtracts zero from the word at the address in a6 and decrements that pointer, and returns.

### usa1

- `noop_then_increment_pointer`: Performs three no-op OR immediate byte operations on register d0, then subtracts zero from the word at the address in a6 and increments that pointer, and returns.

### usa2

- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.

### n

- `load_immediate_constants_d0`: Performs three no-op OR immediate byte operations on register d0, subtracts zero from the word at offset 76 of the structure in a6 indexed by d0, then loads eight immediate byte constants into register d0 and returns.

### s

- `store_keymap_bytes_3`: Writes the bytes 196, 46, 61, 140, 196, 54, 69, and 7 sequentially through the pointer in d0, and clears the 16-bit value at offset 76 in the structure pointed to by a6.

### gb

- `load_immediate_constants_d0_alt`: Performs three no-op OR immediate byte operations on register d0, subtracts zero from the word at offset 76 of the structure in a6, then loads eight immediate byte constants into register d0 and returns.

### ch1

- `decrement_d5_load_constants`: Performs three no-op OR immediate byte operations on register d0, subtracts the immediate value 76 from register d5, then loads eight immediate byte constants into register d0 and returns.

### ch2

- `store_keymap_bytes_4`: Writes the bytes 0, 196, 46, 61, 140, 196, 54, 69, and 3 sequentially through the pointer in data register d0, and decrements the longword in data register d5 by 76.

### parallel.device

- `parallel_handler_setup`: Loads the value 97 into registers d0 and d1, then branches conditionally on the status flags, saves the pointer at the base of the structure in a6 into register a0, loads the value 99 into register d1, branches on the carry flag, writes two immediate byte constants through pointers in a2 and a0, clears bit d0 of the word just below the pointer in a6, and continues into the next routine.
- `manage_parallel_port`: Clears bit 4 of the byte at offset 118 in the structure pointed to by a6, then checks bit 5 of the byte at offset 53 in the structure pointed to by a1.
- `wait_port_and_call_library`: Masks a byte at offset 30 of the structure in a1 and returns early if the result is nonzero.
- `wait_cia_port_a`: Compares a 16-bit value at offset 28 of the structure in a1 with 3 and returns if they differ, then clears bit 0 of the CIA port A register and waits until bit 0 of the CIA port B register is set.
- `tiny_move`: Small routine of 1 instruction that copies 144 to d0.
- `call_library_vector_18`: Library stub: loads a library base from a global variable and calls the library function at offset minus 18 of that base, returning its result to the caller.
- `write_byte_and_jump`: Writes the byte 253 to the byte at offset 31 of the structure in a1 and then jumps to the routine at offset 336.
- `test_and_clear_longword`: Tests the longword at offset 36 of the structure in a1 and jumps to the routine at offset 782 when it is nonzero, otherwise it clears the longword at offset 32 and jumps to the routine at offset 390.
- `swap_pointers_and_increment`: Clears bit 0 of a byte at offset 30 of the structure in a1 and returns early if a bit in a processor byte is clear.
- `init_parallel_device`: Sets a bit in the parallel port control register at offset 30 of the device structure in a1, copies a long pointer from offset 40 to offset 42 of the structure in a6, clears offset 46, and stores a1 at offset 38.
- `register_parallel_handler`: Saves registers d3 and a6, points a6 at the device structure, and returns when the long word at offset 56 is zero, then returns when the word at offset 114 is zero, loads a pointer from a table, stores it into that table slot, swaps it with a1, saves the old pointer at offset 4 of the new structure, decrements the word at offset 114, clears bit 6 of the control byte at offset 30, calls a routine, and branches away.
- `store_port_byte_and_scan`: Loads a pointer, tests bit 5 of a byte at offset 48, and returns when a word at offset 40 is not three.
- `clear_flags_and_call_libraries`: Clears bits 3 and 4 of two status bytes and stores a pointer from the structure in a6 at offset 46 into a word at offset 32 of a1, then calls the library entry point at offset -378.
- `read_cia_and_branch`: Reads a byte from the address in a0 into the CIA register and jumps to the routine at offset 3f4 when it is nonzero, otherwise it tests bit 3 of the byte at offset 118 and jumps to the routine at offset 424 when that bit is set.
- `call_parallel_library`: Saves a6, loads a pointer from offset 10e of the structure in a6, then calls the library at offset -78 of that base and returns its result in d0.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `call_parallel_open`: Reads a pointer from offset 26 of the current library base in a6 and returns immediately if it is zero.
- `invoke_handler_if_present`: Reads a pointer from offset 38 of the structure in a6 and, if it is zero, returns without doing anything; otherwise it saves a6 and a1 on the stack, loads the pointer from offset 34 of the same structure into a6, calls the routine at offset -18 of that new structure, and finally restores a6 and a1 from the stack before returning.
- `check_and_store_pointer_2`: Loads a pointer from offset 76 of the structure in a6, dereferences it, and stores that longword back to the same location, then branches to the routine at offset 500 when the value is zero.
- `clear_device_flag`: Clears the word at offset 72 of the structure in a6 and returns.
- `configure_parallel_port`: Sets bit 5 of the byte at offset 30 of the device structure in a1, writes the value 254 to the byte at offset 31, then pushes a6 onto the stack, switches a6 to the base at offset 32, calls the library vector at offset minus 170, restores a6 from the stack, and returns.
- `read_parallel_status`: Loads the value 16 into d0 and calls a small routine, then reads the CIAB port A register and masks its low three bits into d1 while clearing the corresponding bits in the register.
- `test_bit_and_write_byte`: Tests bit 2 of the byte at offset 118 of the structure in a6 and jumps to the routine at offset 574 when that bit is clear, otherwise it writes the byte 1 to the byte at offset 31 and jumps to the routine at offset 58e.
- `store_parallel_vectors`: Copies the byte at offset 53 of the device structure in a1 to offset 117 of the structure in a6, tests bit 1 of that byte, and returns immediately if it is clear.
- `open_and_configure_parallel`: Saves two registers, opens a device and returns when the handle is zero.
- `clear_d0_and_branch_2`: Clears the d0 register to zero, then branches forward into the middle of another routine without returning.

### clipboard.device

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `clipboard_open_dispatch`: Saves registers a0 and a6, calls the routine at address zero, restores the stack, and returns.
- `call_clipboard_entry_2`: Saves registers d0 and a1, calls the routine at the fixed address, pops its arguments off the stack, and returns.
- `call_clipboard_entry_a1`: Saves the a1 register on the stack, calls the clipboard entry point, removes the saved pointer from the stack, and returns.
- `jump_to_handler`: Jumps unconditionally to the routine at offset f76 and performs no other work of its own.
- `call_clipboard_entry`: Pushes the register a1 onto the stack, calls a routine by its fixed address, removes the two pushed words from the stack and returns.
- `clipboard_toggle_dispatch`: Saves a1 on the stack, calls the routine at address ffa, and restores the stack before falling through into the following routine.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `open_clipboard_handle_2`: Saves d2, a2 and d3, loads a clipboard structure pointer from a global variable, clears the clipboard, then returns zero when the result is nonzero and otherwise branches into another routine.
- `open_and_format_clipboard`: Saves a register pair, loads a clipboard structure pointer from a global variable, calls the clipboard library, then calls a formatting routine that pushes the clipboard base pointer and a fixed offset before returning.
- `open_clipboard_handle`: Pushes the clipboard base pointer onto the stack, calls the clipboard entry point, then calls the system entry point.
- `increment_clipboard_buffer`: Saves registers, opens a clipboard handle and returns immediately when the result is zero, otherwise reads a pointer at offset 80 of the structure, increments the long at offset 14 of the pointed-to data, stores it at offset 14 of the handle, and branches out.
- `clear_handle_and_close`: Clears the long at offset 14 of a2, pushes a pointer at offset 72 of a2 onto the stack, and calls the system entry point.
- `close_clipboard_handle`: Clears the word at offset 94 of the structure in a2, calls the system entry point, moves a3 into d0, restores the saved registers from the stack, and returns.
- `format_clipboard_when_present`: Saves registers, loads a clipboard structure pointer, reads a long from the data at offset 48 and returns when it is zero, otherwise pushes a fixed offset and the pointer onto the stack, calls a formatting routine, and returns.
- `format_and_call_system`: Pushes a pointer at offset 72 of a2 onto the stack and calls a formatting routine, then clears a long at offset 56 of a2 and calls the system entry point.
- `process_clipboard_request_2`: Loads three structure pointers from the stack and calls the clipboard entry point, then returns early when a flag word at offset 94 is zero.
- `compare_clipboard_offsets`: Reads a value from offset 90 of the structure in a2 and compares it with a stack value, jumping away when it is not less.
- `compare_and_select_clipboard`: Tests a value from the stack and returns when it is zero, then shifts another stack value right by nine bits and masks a third value.
- `compare_and_load_pointer`: Copies the 32-bit value in d1 into d0, compares it against the 32-bit value at offset 14 of the structure in a6, and returns without acting when that value is less than or equal to d0, otherwise it loads the pointer at offset 0 of the structure in a1 into a1 and falls through to the code at the routine's start.
- `check_buffer_length`: Copies d3 into d5, then computes the difference between 512 and the word in d2 and returns early when that difference is not within range, otherwise stores d2 into d4 and branches forward.
- `copy_buffer_and_branch`: Loops over 512 iterations, copying a byte from a table at offset 18 of a1 indexed by a counter into a buffer in a3 and incrementing that counter.
- `move_d2_to_d4`: Copies the word in d2 into d4, then branches forward into the middle of another routine without returning.
- `copy_buffer_until_limit`: Loops copying a byte from a table at offset 18 of a1 indexed by a counter into a buffer in a3 until the counter reaches a limit held in d5, then clears d5 and branches into the middle of another routine.
- `store_clipboard_pointer`: Stores a1 at offset 86 of the structure in a2, calls the clipboard entry point, moves d3 into d0, restores the saved registers from the stack, and returns.
- `process_clipboard_handles`: Reads two global longs and compares their sum against a limit, then branches away when the limit is exceeded or a flag word is zero.
- `check_handles_and_call_routines`: Pushes a constant and a pointer at offset 18 of a1, then calls a routine and compares its result with a constant.
- `push_arg_and_call`: Pushes a stack pointer offset and calls the routine at the fixed address, then removes the argument and branches forward into the middle of another routine.
- `handle_clipboard_write`: Clears and rechecks a flag word at offset 94, returning early when it is zero.
- `validate_clipboard_offset_2`: Reads the long word at offset 24, shifts one copy right by 9 bits and masks another copy by 511, then returns early when the pointer at offset 86 is zero.
- `validate_and_call_clipboard`: Returns immediately when the value in d4 is zero, otherwise compares a computed value against a field and returns when it is not less.
- `validate_clipboard_offset`: Checks whether a global variable is zero or negative and returns if so, otherwise computes a value from a constant and a register and branches into another routine.
- `copy_buffer_increments`: Copies d1 into d0, stores a byte from a3 into a table at offset 18 of the structure in a1 indexed by d0, increments d1, and loops until d1 reaches 512 before restoring registers and calling a helper routine.
- `move_d3_to_d1`: Copies the word in d3 into d1, then branches forward into the middle of another routine without returning.
- `copy_buffer_and_update_offset`: Loops copying a byte from a buffer in a3 into a table at offset 18 of a1 indexed by a counter until the counter reaches a limit, then adds the counter to a long at offset 20 of a6 and clears that long.
- `store_clipboard_value`: Stores the 32-bit value in d4 at offset 86 of the structure pointed to by a2, then calls the clipboard library and returns its result in d0 after restoring the caller's registers.
- `toggle_clipboard_flag_check`: Saves a2, loads a clipboard structure pointer, calls the clipboard library, then returns without acting if bit 7 of the byte at offset 30 is set, returns again if bit 0 of that byte is set, otherwise pushes the pointer and calls another routine before returning.
- `toggle_clipboard_flag`: Clears bit 7 of the byte at offset 30 of the structure in a2, then sets it again, and finally calls the clipboard library and returns its result.
- `toggle_clipboard_flag_check_2`: Saves d2 and a2, loads a clipboard structure pointer, then returns without acting if bit 7 of the byte at offset 30 is set, otherwise clears that bit, calls a helper routine, sets bit 6, and restores the registers.
- `compare_handles_and_return`: Loads pointers from a stack-returned address into a0 and a1, then checks whether the pointed-to longs are zero.
- `branch_after_clear`: Loads the constant 0 into d0 and branches to the routine at offset 61c, contributing to its result.
- `clear_d0_and_branch`: Clears the d0 register, then jumps into the middle of another routine.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_clipboard_fields`: Copies three fields from the clipboard structure in a1 into the clipboard structure in a0, then calls a clipboard update routine.
- `set_and_test_clipboard_byte`: Sets bit 5 of a byte at offset 30 of a structure in a3, tests a byte at offset 66 of a structure in a2, and clears bit 0 of the same byte at offset 30 when that test is zero.
- `update_clipboard_state`: Sets a byte at offset 66 of the structure in a2 and calls a handler routine, returning immediately when that handler fails.
- `clear_register_and_branch_4`: Clears the 16-bit value in register d3, then branches unconditionally into another routine at the given address.
- `copy_clipboard_and_call_handler`: Copies two fields from a clipboard structure into another structure, then calls a handler routine when a command word matches nine.
- `save_register_and_call`: Saves register a2 on the stack, calls a routine, restores the stack, stores the word one at offset 94 of the structure in a2, then branches to a shared location.
- `check_command_call_handler`: Clears a register, sets d0 to nine, and compares it with a command word at offset 64 of the structure in a2.
- `clear_register_and_branch_3`: Clears a register and branches to a shared location.
- `copy_clipboard_fields_2`: Checks a pointer for null, copies two fields from one structure to another, and calls a handler routine when a value is within a range.
- `process_clipboard_command`: Checks a command word and a flag bit, updates a counter and clears a flag when valid, then pushes structure pointers and calls a handler before adjusting fields.
- `compare_and_adjust_field`: Loads a long from offset 90 of a2 into d2 and a long from offset 36 of a4 into d0, adds offset 44 of a4 to d0, and compares the two.
- `adjust_field_and_jump`: Loads a value, subtracts a field from it, stores the result back, loads it again, adds it to another field, and jumps to another routine without returning.
- `clear_flag_and_call`: Clears a long field at offset 30 of a4, decrements a word at offset 68 of a2, pushes a3 and a4 onto the stack, calls a routine, then clears a word at offset 64 of a2 when the counter is nonzero.
- `clipboard_check_and_call`: Compares a structure word against four and returns when it differs, then checks a pointer, compares another field against four, and when valid pushes several values and calls a clipboard routine.
- `store_clipboard_pointer_3`: Copies a long from a field at offset 44 of a4 into a field at offset 90 of a2, pushes two pointers, calls a routine through a fixed address, removes the pushed values, then branches forward into the middle of another routine.
- `check_clipboard_size`: Loads the value four into a register, compares it against a word field in a structure, and branches away when the values differ; otherwise it stores four into another word field and branches into a later routine.
- `clear_register_and_branch_2`: Clears the byte in register d3, then branches forward into the middle of another routine.
- `check_size_and_call_clipboard`: Compares a structure field against four and returns when it differs, then pushes arguments and calls the clipboard entry point before incrementing a counter when the result is at least four.
- `clear_state_and_branch`: Clears the long word at offset 90 of the structure in a2 and then branches forward into the middle of another routine.
- `reset_clipboard_state`: Clears a long at offset 144 and a word at offset 176 in the structure in a2, clears a word at offset 64, calls another routine, then clears bit 5 of the byte at offset 30 in the clipboard structure in a3 unless it was already clear.
- `compare_clipboard_pointers`: Saves a2, loads two pointers from the stack, reads a long from the first pointer and returns when it is zero, otherwise compares a long at offset 48 of each pointer and branches when the first is greater, then reloads the long and branches back into a loop.
- `load_zero_and_branch`: Loads the constant zero into d1, then branches forward into the middle of another routine without returning.
- `push_args_and_call`: Pushes the values from a2, a0 and a1 onto the stack, calls the routine at offset 64, restores the stack pointer, and returns to the caller.
- `clear_clipboard_flag`: Loads a clipboard structure pointer, stores the byte value 253 at offset 31 of that structure, calls a helper routine, and returns.
- `invoke_clear_clipboard_flag`: Saves the value in d2 on the stack, loads the pointer passed on the stack into d2, pushes that pointer on the stack, calls the helper routine, removes the pointer from the stack, restores d2, and returns.
- `set_clipboard_flag`: Saves register d2 on the stack, loads a clipboard structure pointer from the first stack slot, stores the byte value 253 at offset 31 of that structure, calls the helper routine, restores d2 from the stack, and returns.
- `invoke_clipboard_handler`: Pushes the value in d2 onto the stack, loads the pointer stored at offset 8 of the stack frame into d2, pushes that pointer, and calls the clipboard device handler, then restores d2 and returns.
- `invoke_clipboard_handler_2`: Pushes the value in register d2 onto the stack, loads the pointer stored at offset 8 from the stack into d2, pushes that pointer, calls the clipboard device handler, adds 4 to the stack pointer, restores d2, and returns.
- `invoke_clipboard_handlers`: Saves register d2 on the stack, loads a clipboard structure pointer from the first stack slot into d2, pushes that pointer, and calls the clipboard handler, then repeats this for two more handlers before restoring d2 and returning.
- `setup_clipboard_flag`: Saves two registers and loads a clipboard structure pointer, returning immediately when the flag at offset 48 is already set.
- `call_compare_pointers_routine`: Saves a2, pushes an address offset 96 from a3, and calls a routine, then saves a2 and a3 and calls another routine before restoring both registers and returning.
- `increment_clipboard_flag`: Saves two registers and loads a clipboard structure pointer, returning immediately when the flag at offset 48 is already set.
- `call_clipboard_compare_routine`: Saves a2, pushes an address offset 112 from a3, and calls a routine, then saves a2 and a3 and calls another routine before restoring both registers and returning.
- `store_clipboard_pointer_2`: Loads a clipboard structure pointer and a secondary pointer from offset 18, copies the longword at offset 38 of that pointer into offset 30 of the structure, calls a helper routine, and returns.
- `store_clipboard_pointer_2_2`: Loads a clipboard structure pointer and a secondary pointer from offset 18, copies the longword at offset 60 of that pointer into offset 30 of the structure, calls a helper routine, and returns.
- `walk_clipboard_list`: Loads request fields from the frame, sets a library base pointer, and fetches a linked list head into a register.
- `compare_long_d3_a3`: Compares a long value at offset 14 of the structure in a3 with the long in d3 and branches away if they differ, otherwise it clears d4 and d2 to zero and falls through to the target routine.
- `dispatch_clipboard_request`: Returns early when d4 is zero, then pushes the value 65537 and the constant 156 and calls a routine, saving the returned pointer into a3.
- `process_clipboard_request`: Initializes a clipboard structure by setting several fields and flags, then calls a sequence of library and helper routines to process the clipboard, branching to different cleanup paths based on the results.
- `call_library_check_512`: Pushes the constant 512 and a value at offset 12 of a0 onto the stack, saves d3, and calls the library entry at offset 48, then compares the result with 512.
- `clear_clipboard_flag_2`: Checks a clipboard flag and a clipboard structure pointer, then clears the flag when no clipboard structure is present.
- `check_clipboard_format`: Checks whether a clipboard structure is present and returns without acting when it is absent, otherwise it pushes the structure pointer and a constant onto the stack and calls a formatting routine.
- `increment_and_dispatch`: Increments a counter at offset 18 of the structure in a3 and a global word at offset 252, saves the pointer at offset 24 of the structure in a2, and returns 1 if the byte at offset 103 is not 1.
- `release_clipboard_resources`: Decrements a clipboard usage counter and, when it reaches zero, releases clipboard resources by calling cleanup routines, checking flags and pointers, and branching to different exit paths based on the results.
- `call_library_compare_d4`: Saves d4 and pushes a value at offset 12 of a4, then saves d3 and calls the library entry at offset 72.
- `clear_byte_94_a3`: Clears byte 94 of the structure in a3 and then branches to the target routine.
- `dispatch_clipboard_cleanup_2`: Pushes registers d3, d5 and d6 onto the stack and calls two routines, then checks a pointer at offset 68 of the structure in a3 and calls the same first routine with that pointer when it is set.
- `dispatch_clipboard_cleanup`: Calls a setup routine and returns early when the flag at offset 94 is set, otherwise it invokes several handlers and decrements a global counter, branching to a second handler when the counter reaches zero and bit 3 of another global is set.
- `test_flag_call_routines`: Tests byte 252 of global memory and branches away when it is nonzero, otherwise it pushes a long from a global variable, calls a routine, pushes another global long, calls a second routine, loads a long from a global variable into d0, removes the pushed value from the stack, and branches to the target routine.
- `set_flag_and_return`: Sets bit 3 of the byte at offset 234 in global memory, loads the constant 0 into d0, and returns.
- `store_clipboard_flag_and_call`: Saves registers d2 and a2, restores a2 from the stack, calls a routine, sets byte 253 at offset 8 of the structure in a2, clears bit 5 of the byte at offset 30, then compares a 16-bit value at offset 28 against 12 and branches away when it is greater.
- `index_and_call_table`: Pushes a2 onto the stack, reads a word at offset 28 of the structure in a2, multiplies it by 4, and uses it to index into a table to obtain a routine address.
- `toggle_clipboard_state`: Loads a clipboard structure pointer and returns without acting when bit 7 or bit 6 of the byte at offset 30 is set, otherwise it sets the byte at offset 31 to 254 and branches to a handler when bit 5 is set.
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
- `link_pointer_structure`: Reads a pointer from offset 4 of the stack into a0, stores that pointer at the address it points to, advances it by 4, clears the long at offset 4 of the pointed-to structure, and stores the pointer at offset 8 of that structure before returning.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_raw_do_fmt_2`: Saves registers a2, a3, a4 and a6, loads three pointers from the stack into a3, a0 and a1, loads a2 from a text string in global memory, sets a6 from the library base, and calls the exec.RawDoFmt entry.
- `store_byte_advance`: Stores the byte in d0 at the address pointed to by a3 and advances that pointer by 4, then returns.

### printer.device

- `branch_to_printer_entry`: Branches unconditionally to a fixed address in the printer library entry point and does not return.
- `load_254_open_printer`: Loads the constant 254 into d0, branches to the printer open routine, and returns.
- `call_printer_open`: Saves registers d2 and a3 on the stack, copies d0 into d2, and saves a6 before calling the library entry at offset -120.
- `open_printer_library`: Saves registers a0 and a2, loads three addresses into a0, a1 and a2, loads the value 43521 into d0 and calls the library entry at offset 84, then tests the returned long word and branches away when it is zero.
- `open_and_init_printers_2`: Opens three printers in sequence by calling the library entry at offset 552 and stores each handle, returning early when one fails, then it initializes a table of 1023 words and opens the graphics and intuition libraries.
- `call_library_adjust_pointer`: Loads a global pointer into a1, saves a6, and calls the library entry at offset 1076 three times with three different global pointers.
- `invoke_printer_routines`: Calls set_printer_flag, then clear_printer_flag, then check_printer_pointer in sequence, and returns.
- `load_253_open_printer`: Loads the constant 253 into d0, branches to the printer open routine, and returns.
- `set_printer_flag`: Sets bit 0 of the byte at offset 61 in a6 and then returns immediately.
- `clear_printer_flag`: Clears bit 0 of the byte at offset 61 in a6 and then returns immediately.
- `check_printer_pointer`: Loads a pointer from offset 72 of a6, returns immediately when it is zero, otherwise calls a printer routine and then branches back to the start of this routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `branch_to_open_printer`: Branches forward to a printer open routine without returning.
- `open_and_init_printers`: Saves registers, opens two printers by calling the library at offsets 330 and 324, stores their handles and a base address at various offsets of the structure, combines two status longs, calls the library at offset 318, then calls the library at offset 372 and clears a bit when the returned address matches one stored at offset 108.
- `compare_address_clear_bit1`: Loads an address at offset 190 of a6 into a1, compares it with the long in d0 and branches to the target routine when they differ.
- `check_printer_bit_return`: Checks bit 5 of d2 and returns immediately if it is clear, then returns without doing anything if the printer structure pointer at offset 72 is null.
- `load_stack_and_call`: Loads a longword from the stack at offset 4 into a0 and a longword at offset 8 into d0, then calls a routine and returns.
- `handle_printer_open`: Calls a flag-reading routine and returns early when the result is nonzero, otherwise it stores a pointer and a value into the structure at offsets 36 and 40, sets the word at offset 28 to 3, and calls a printer-close routine.
- `shift_stack_and_branch`: Increments the stack pointer in a7 by eight and then branches to the routine at address 81a.
- `set_flag_and_branch`: Sets bit 1 of the byte at offset 155 of the structure in a6, loads the address at offset 190 into a1, then branches to the routine at address 806.
- `read_flag_byte_check`: Loads a byte from a global variable into d0 and returns immediately if that byte is nonzero, otherwise loads a byte from another global, clears its bits above 3, and returns immediately if the result is zero.
- `call_library_indexed`: Saves a6, calls the library at index -132, then reads a byte at offset 327 of the structure in a6, sets that bit in d0, and calls the library at index -318.
- `check_printer_structure_flag`: Loads a printer structure address into a1, compares it with d0 and branches away when they differ, then clears a flag bit and calls a routine before branching to another location.
- `check_printer_library_base`: Loads the base address of the printer library into a1, compares it with d0 and branches away when they differ, then calls a routine, tests the result and returns when it is zero.
- `call_then_branch`: Calls a routine and then branches to the routine at address 82e.
- `init_printer_struct`: Computes a pointer to a structure at offset 272 in a6, stores the word 9 at offset 28, copies a long from offset 52 of a global variable into offset 32, and clears the long at offset 36 of that structure.
- `call_printer_close`: Clears the byte at offset 30 in a1, saves a6, switches to the library at offset 20 in a1, and calls its entry at offset -30 before restoring a6 and returning.
- `check_offset_8_value`: Loads the structure at offset 110 from a6 and returns immediately unless its byte at offset 8 equals 7.
- `call_printer_io`: Saves a1 and a6, switches to the library at offset 20 in a1, and calls its entry at offset -36 before restoring a1 and a6.
- `check_serial_and_printer_state`: Saves a6, loads a pointer from a global long word into a0, reads a byte at offset 31 and branches away when it is nonzero, then loads another pointer and masks a byte against 3, branching away when the result is nonzero.
- `query_device_states`: Calls the printer, serial, and related device routines, then reads a status byte, builds a flag from it, and branches away when a handler pointer is invalid.
- `check_offset_pointer`: Computes an address from a base register and compares it with a passed value, branching away when they differ.
- `check_loaded_pointer`: Saves a6, calls the library at index -138, then loads a pointer from offset 332 of the structure in a6 and returns immediately when it is zero.
- `call_printer_and_branch`: Calls a printer routine and then branches to another location, passing control onward without returning.
- `check_printer_request`: It compares a byte at offset 155 of the structure and branches away when it is not one, otherwise it pushes a constant and branches to a nearby routine.
- `request_intuition_with_interrupts`: Disables interrupts, increments a counter at offset 294 of the structure in a2, modifies a byte at offset 512 of a buffer, indexes a table, and calls an intuition request routine, then restores interrupts and returns.
- `call_intuition_auto_request`: Clears d0, saves a6 on the stack, switches to the intuition library, and calls its entry point at offset -348, then restores a6 and pops d2, d3, a2, and a3 from the stack.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `push_a1_call_stub`: Pushes the value of a1 onto the stack twice, calls the routine whose address is at offset 0 in the data segment, then pops four bytes off the stack to restore the stack pointer.
- `handle_printer_buffer_reset`: Saves register a1, clears a printer buffer flag, and calls a printer routine, then branches based on the result to either copy the buffer pointer or call another routine before returning.
- `call_printer_entry`: Pushes the same register value twice onto the stack, calls the library entry point, and pops the stack back to its original position.
- `call_device_open_branch`: Duplicates the a1 register on the stack, calls a device entry point, restores a1, and branches to another routine before returning.
- `handle_printer_request`: Saves a4 and a6, loads a global pointer, and calls the dispatch entry at offset 104.
- `perform_32bit_division`: Returns immediately if a divisor is not 65535, otherwise performs a 32-bit unsigned division using successive 16-bit divisions and a small loop to accumulate the quotient, then returns.
- `compute_32bit_multiplication`: Multiplies two 16-bit values and combines the partial products to compute a 32-bit unsigned multiplication, storing the result in a register before restoring a saved register and returning.
- `swap_registers_and_call`: Moves register d2 onto the stack, swaps the values of registers d1 and d0, calls a routine, restores register d2 from the stack, and returns.
- `swap_registers_and_call_result`: Moves register d2 onto the stack, swaps the values of registers d1 and d0, calls a routine, copies the result into register d0, restores register d2 from the stack, and returns.
- `call_with_absolute_values`: Moves register d2 onto the stack, takes the absolute value of register d1, negates register d0 if register d1 was negative, calls a routine, and returns the result or restores register d2 when the result is zero.
- `call_with_absolute_registers`: Moves register d2 onto the stack, saves a pointer, takes the absolute value of registers d1 and d0, calls a routine, and returns the result or restores register d2 when the result is zero.
- `setup_and_call_serial_check`: Sets up a stack frame, saves several registers, loads a pointer from the caller and two decimal constants into registers, then calls a routine.
- `check_value_and_clear_memory`: Reads a long from a structure, compares it with negative one, and when it differs increments an index and clears two memory locations.
- `store_pointer_clear_flag`: Loads a long pointer from a3 into a global long word at offset 184, clears the word at offset 182, and then branches to a shared location.
- `copy_byte_to_printer_buffer`: Copies the next byte from a source table to a printer buffer, advancing both pointers, and returns when the source index reaches one hundred.
- `zero_registers_jump`: Zeroes the registers d4, d6 and d2, then jumps into another routine.
- `scan_table_and_call`: Increments a counter and compares a computed index against a limit, comparing a byte in a table against a register and looping back while they differ, then calls a routine with an indexed argument and stores its result.
- `copy_bytes_indexed`: Reads a long from a3 and increments it, then copies bytes from a table into another table indexed by that long until the index reaches a value stored in a global long, otherwise it checks register d2.
- `clear_struct_field_and_restore`: Clears the long at offset 32 of a structure, copies a byte into the return register, restores the callee-saved registers, unwinds the frame pointer, and returns.
- `read_printer_response`: Reads two global longs into registers, sets up three pointers from fixed addresses, and calls a routine through a fixed address.
- `clear_state_and_branch_2`: Clears the long word at offset negative four of the structure in a6, then branches forward into the middle of another routine.
- `push_and_call_routine`: Copies the byte at d3 into d1, extends it to a long, pushes a value and a frame offset, then calls another routine and branches away when the result is not greater than zero.
- `compare_constants_branch`: Loads the value 2 into a register and compares it with a long word in a global variable, returning early when they match, then compares the value 12 with a register and returns early when they differ, otherwise falling through.
- `store_byte_if_conditions_2`: Loads the value 2 into d0, compares it with a global long word and branches away when they differ, loads 12 and compares it with d5, branching away when they differ, then stores byte 75 through a2 and branches away.
- `store_byte_if_conditions`: Loads the value nine into register d0, compares it with register d5 and branches away when they differ, checks the byte at register a4 and branches away when nonzero, otherwise stores the value 59 at the address in a2 and branches to 754.
- `store_byte_from_table`: Loads the value six into a register, compares it with register d5, and when d5 is greater loads a byte from a table using d5 as an index and stores it through a pointer.
- `check_byte_store_value`: Returns immediately if register d5 is nonzero, otherwise compares the byte at a4 with 29 and returns if it is less than or equal, otherwise it stores the value 12 into the byte at a2.
- `convert_and_store_byte`: Looks up a byte in a table indexed by two counters and branches away when they differ.
- `clear_globals_and_delegate`: Loads two long values from the stack, clears eight long locations in global data, and jumps to a shared routine at offset 704 to finish the work.
- `load_indexed_byte_and_call`: Reads a pointer from a global long at offset 10E, loads a byte from a table at offset 0 using that value as an index into a3, and returns when the byte equals 27, otherwise calls a routine and continues.
- `check_bracket_and_loop`: Compares a byte at a pointer with 40 and branches away when it differs, otherwise calls a routine, then loops over a table of eleven characters while adjusting the byte at the pointer.
- `check_bracket_byte`: Compares a byte at a pointer with the value 91 and branches away when it differs, otherwise falling through to the code at 5d0.
- `format_print_string`: Loops eight times, comparing an input byte with table entries and storing matching values through a pointer.
- `handle_print_response`: Compares a byte at a pointer with the value 10 and branches away when it differs, tests a global byte and branches when nonzero, stores the value 3 at another pointer, then branches to 546.
- `handle_print_result`: Sets a flag byte, pushes arguments, and calls a formatting routine before checking the result.
- `store_newline_bytes`: Reads a long from a2 and increments it, stores the byte 10 at the address formed from that long plus d2, then reads the next long from a2 and increments it, storing the byte 13 at the matching address.
- `store_single_newline`: Reads a long from a2 and increments it, stores the byte 10 at the address formed from that long plus d2, then jumps into another routine.
- `check_printer_response_code`: Compares a byte in a structure against a fixed value and branches away when it differs, then pushes two registers onto the stack, calls a routine through a fixed address, removes the pushed values, and branches into a later routine.
- `compare_word_below`: Loads a pointer from a global long word, compares the word at offset 8 with 33, and branches away when it is below that value.
- `count_consecutive_digits`: Compares bytes at a pointer against a backslash and, if they fall within the digit range, counts consecutive digits, returning early when the count reaches three.
- `advance_pointer_check`: Increments a pointer in a1 and a global long word, then returns early when the byte at that pointer is zero.
- `index_and_call_table_2`: Clears d1, reads a byte from a3, subtracts 160 from it, multiplies by 4, adds d4 to the resulting pointer, loads a pointer from that address into a0, and branches away.
- `check_digits_in_sequence`: Compares the byte at register a1 with the character 92 and branches away when they differ, otherwise it advances the pointer and checks the next byte against the characters 48 and 57.
- `store_digit_index_byte`: Reads a digit from a pointer, scales it through a table index, and stores the computed byte into a destination array while incrementing a counter.
- `copy_byte_sequence_loop`: Saves the pointer at a2, increments it, forms a destination by adding d2 to the saved pointer, copies a byte from the pointer at a1 into that destination, and loops while the incremented byte is nonzero.
- `copy_byte_and_check_limits`: Copies a byte from a3 into the address formed by d1 and d2, then increments the pointer at offset 2 of a2.
- `compute_offset_and_restore`: Subtracts two global longs from a register value, clears a global long, copies a pointer from a register into the return register, restores the callee-saved registers, and returns.
- `test_byte_branch`: Tests whether the byte in a3 is at least 160 and branches away when it is below, then loads a base pointer from a global long word.
- `add_and_store_global_sum`: Loads a value from a global variable at offset 124, adds a value from register d3, and stores the sum into a global variable at offset 128 when the sum is greater than 256, otherwise it returns.
- `decrement_global_counter`: Decrements a global counter, stores the result back, loads a value into a register, and branches away.
- `clear_register_branch`: Clears a register and branches away.
- `copy_table_bytes_loop`: Reads a pointer from a global variable, increments it, and copies bytes from a table indexed by a counter into the destination pointed to by the incremented value until the counter reaches a limit, then restores registers and returns.
- `initialize_registers_clear`: Pushes several registers onto the stack and restores three of them from the stack, initializing a register and a pointer to zero and clearing the byte at that pointer, then branches into another routine.
- `clear_byte_if_match`: Compares register d1 with the value 59 and branches away when they differ, increments the counter in a3, forms a destination by adding a2 to it, clears that byte, and branches to 2164.
- `store_one_if_space`: Compares register d1 with the space character and, when they match, reads a pointer from the stack, loads the value one into a register, stores it through that pointer, and returns.
- `check_register_branch`: Checks a register against a value and, if they match, loads a pointer from the stack, sets a register, and branches away.
- `load_byte_and_call_routine_2`: Computes an index from a byte in d1 and a byte at a3, multiplies the latter by 10, stores the combined value through a0, pushes d2 and d3, calls a routine, then compares the result with 48 and 57 and branches away when out of range.
- `load_byte_and_call_routine`: Reads a pointer and a value from the stack, loads a byte from a table using an index, increments the counter at that pointer, and calls a routine when the index is less than the value, otherwise it clears a register and returns.
- `swap_register_bytes`: Swaps the byte contents of two registers by using the second as temporary storage, then returns.
- `decrement_stored_counter`: Loads a fixed address into a register, reads a global counter, decrements it, stores it back, and branches away.
- `check_table_byte_flag`: Decrements a counter in a global variable and returns if it is zero, otherwise masks a byte from a table and returns if the masked value is not 27, then sets a flag in a global variable and loads the counter before returning.
- `init_printer_structures`: Pops a value from the stack into d1, clears three bytes in a global structure, then clears a byte in a second structure and stores the value 1 at offset 58 of that structure.
- `copy_printer_values`: Loads a global library pointer and copies six consecutive words from offsets 2646 through 2654 of that structure into a table, then copies the word at offset 2666 into the following table slot before returning.
- `compare_printer_values`: Loads a global library pointer and compares each of six words read from offsets 2646 through 2666 against the matching words in a table, branching away on the first mismatch.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_pointer_branch`: Saves a6, calls the library at index -372, then loads a pointer from offset 108 of the structure in a6, compares it with d0, and branches away when they differ.
- `dispatch_printer_request`: It tests the value in d2 and branches away when nonzero, otherwise it calls a dispatch routine and, depending on the results, calls a printer-request handler or a second dispatch routine before restoring registers and returning.
- `select_printer_buffer`: Saves registers, loads a printer device pointer, and calls a buffer selection routine, then branches to a handler when the result is zero or copies the result into d1 and branches onward.
- `handle_printer_buffer_select`: Compares a 16-bit value on the stack with 54 and branches away when equal, then compares it with 53 and branches away when different, clears d1, copies the stack word into d1, masks it with 1, stores the result into a global byte, and branches away.
- `load_printer_buffer`: It compares a word on the stack with 51 and a global byte with 1, returning early when either differs, otherwise it stores 54 into that stack word, indexes a table using it, and loads a pointer into d4.
- `clamp_d1_to_255`: Compares d1 with 255 and returns immediately if they differ, otherwise loads 255 into d1 and jumps to another routine.
- `copy_buffer_clamped`: Compares d1 with 254 and clears it to zero if equal, then copies bytes between two indexed buffers, looping back to a routine while the source byte is nonzero and returning when a stack word matches a digit or letter constant.
- `dispatch_printer_table`: Saves d2, loads a value from the stack into d2, and uses it to index a jump table by loading a word offset and jumping to it.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d2 to d0, then copies (a7)+ to d2, then returns.
- `copy_device_status_flags`: Copies several words from a device structure into global memory, then reads a word at offset 60 of that structure and clears a flag at offset 18C when bit 12 of it is clear, otherwise it branches away.
- `save_state_and_call`: Stores the constant one at offset 18C of a global variable, saves a3, loads a word from offset 18A of that global into d0, and pushes it along with a pointer from offset 18E onto the stack before calling the routine at that pointer.
- `compute_and_store_differences_2`: Checks specific bits in a status byte and returns early when they are set.
- `compute_and_store_differences`: Reads two pairs of words from the structure in a1 at offsets 16-20 and 18-22, subtracts each pair, increments the result by one, and stores both computed values into a global variable.
- `scale_and_check_status`: Reads two words from a table pointed to by a global structure, scales the first by eight, stores both values in a global variable, then checks four status bytes and branches away when any of them is clear.
- `compute_and_branch_result`: Reads two words from a global structure, subtracts them, calls a routine with the result, and branches to different routines based on the returned value.
- `compare_branch_128`: Compares the word in d0 against the constant 128 and returns when they differ, otherwise it falls through to an unconditional jump that transfers control to another routine.
- `load_constant_and_branch_2`: Loads the constant 12 into d1 and then branches into another routine without returning.
- `load_constant_and_branch_3`: Loads the constant 15 into d1 and then branches into another routine without returning.
- `compare_and_update_value`: Calls a routine with a constant, compares its result against a stored value, and when the new value is larger it updates a stored long and multiplies a table entry into a register.
- `call_routine_check_flags`: Calls a routine with a constant and compares its result against a stored value, then branches away when a global flag is zero or a structure field is too small.
- `test_globals_combine`: Sets up two small stack values, tests two global bytes and branches away from each when nonzero, then combines two stack values and stores the result back.
- `check_globals_then_call`: Clears a global byte, then branches away when a global byte at offset 158 is zero.
- `compute_and_swap_globals`: Clears a global byte, then computes and stores decremented sums of word pairs from global tables into other global words.
- `store_global_word_sums`: Adds two word pairs from global tables and stores each sum into a separate global word, then branches away when a global byte is nonzero.
- `shift_and_store_result`: Tests a bit in a global byte and returns when set, otherwise shifts a value from a pointed structure, calls a multiplication routine, and stores a result into that structure when another bit is clear.
- `compute_and_compare_products_2`: Tests a bit in a global byte and returns when clear, otherwise shifts and stores values from a pointed structure, then calls a routine several times to read and compare two-word products before storing a result and jumping to a common exit.
- `compute_and_compare_products`: Performs two 32-bit multiplications and compares their results, returning to another routine when the first is not greater.
- `store_signed_difference`: Computes the signed difference between a global word and a word from a structure, storing the result as a byte at offset negative 72 of a global table.
- `compute_and_store_difference`: Copies a word from a global variable into another and computes a difference that it stores into a global variable.
- `store_table_difference`: Computes the difference between two words loaded from a global variable and stores the result into another global variable, then clears a byte and a register.
- `store_nibbles_from_table_loop`: Loops over a table, storing the low and high nibbles of each entry into three consecutive bytes, then compares a counter against a register and branches back into the loop while it is smaller.
- `call_global_pointer_routine`: Initializes two global longwords, then loads a value from a global word into a register and calls a routine through a global pointer, passing the structure pointer and cleared arguments.
- `call_routine_and_store_result_3`: Pushes a constant and clears three stack longs, then calls a routine through a global pointer and stores the result before branching.
- `test_flags_then_call`: Tests a global byte at offset 214 and returns when it is zero, then tests another global byte at offset 216 and returns when it is zero, otherwise calls a routine and branches away.
- `test_flag_call_shared`: Tests a global byte and returns immediately when it is zero.
- `check_flag_and_call`: Checks a global byte, and when it is nonzero calls a routine and then branches to a shared location.
- `call_routine_and_store_result_2`: Calls a routine, stores its result in a global long, and returns immediately when that value is zero, otherwise branches away.
- `call_divided_routine`: Loads a word from a global, divides it by a value at offset 24 of a structure, and branches away if the result is zero, otherwise pushes arguments and calls a routine, storing the result and returning.
- `call_read_compare_routine`: Pushes two longwords onto the stack, calls the routine at the current program counter, then pops eight bytes off the stack and restores six registers before returning.
- `compute_weighted_byte_sum`: Saves d2 on the stack and returns immediately if the byte at offset 160 of a global is zero, otherwise multiplies three consecutive stack bytes by 28, 151, and 77 respectively, sums them, adds 128, and arithmetic-shifts the result right by 8 bits.
- `store_max_of_three_bytes`: Reads three consecutive bytes from the top of the stack and compares them as unsigned 16-bit values, storing the largest of the three at the fourth stack byte before returning.
- `compute_printer_offset`: Checks a global byte and branches away when it is zero.
- `compute_and_call_open`: Adds two global words, multiplies a byte by a table value, and pushes both results before calling the routine at the fixed address, then stores its result and branches when it is not negative one.
- `store_table_nibble`: Returns early when a global flag byte is zero, otherwise splits a register into two nibbles.
- `clear_bit_and_branch`: Clears bit 0 of the byte at offset 248 of the structure in a6, calls another routine, and then branches onward without returning.
- `compare_shifted_counter`: Shifts a 16-bit value right by four bits, compares it with one, and branches to different locations depending on whether the result is below or above that value.
- `compare_printer_counter_two`: Compares the 16-bit value in d0 with 2 and branches away when they differ, otherwise branches to a nearby routine.
- `decrement_printer_counter`: It decrements a 16-bit value in d0 and branches away when it does not reach zero, otherwise it branches to a nearby routine.
- `store_byte_and_branch`: Stores the byte in register d1 into the global variable at offset 176, then branches to the routine at 3300.
- `store_byte_and_branch_offset_178`: Stores the byte in register d1 into the global variable at offset 178, then branches to the routine at 3300.
- `call_and_store_result_2`: Stores d1 into a global at offset 177, pushes a longword, calls a routine, stores its result, restores the stack, saves another value into a global, and branches to another routine.
- `accumulate_table_values`: Indexes a four-word table using a value from d3 and stores the fetched long at the address in a2, then adds four one-byte values from a global table into four separate long counters before returning.
- `multiply_and_compute_four_values`: Multiplies a word from a global variable by another word and stores the product, then computes four values by dividing a saved longword by two and adding different base addresses, calling a routine for each and storing each result.
- `call_and_store_printer_result`: Loads two 16-bit values from global memory, calls a routine with them, stores the result, and branches based on the outcome to either call another routine or fall through to a shared exit path.
- `call_routines_from_stack_2_2`: Tests two global bytes and branches away when either is nonzero, then loads two 16-bit values from a3 and a4 onto the stack, pushes a pointer, calls a routine, and branches away when the result is zero.
- `call_routines_from_stack_2`: Pushes two words from the structure into the stack, calls a routine, and branches away if the result is zero, otherwise pushes more argument), save_registers_check_flag (Saves registers d2 through d5 on the stack, loads a byte from offset 23, a word from offset 26 and a word from offset 30 of the stack frame,).
- `call_routines_from_stack`: Pushes two words from the structure into the stack, calls a routine, and branches away if the result is zero, otherwise pushes more arguments, calls another routine, stores the result, and returns.
- `restore_stack_pointers`: Restores the a2, a3 and a4 registers from the stack and returns.
- `call_routine_after_division`: Saves a2 into the stack, sets up pointers to two globals, increments a counter, divides a value by a divisor, and branches away if nonzero, otherwise pushes arguments and calls a routine, storing the result and returning.
- `push_args_call_routine`: Pushes a word address onto the stack, pushes three longwords, loads a routine address from a global and calls it, then stores d0 through a2 and pops twelve bytes off the stack.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `setup_pointers_and_write_field`: Saves three registers, loads three global pointers into them, writes a global word into a structure field, then branches away.
- `push_globals_and_store`: Pushes two 16-bit values from global variables at offsets 130 and 128 onto the stack, calls a routine that adjusts a long word through a pointer, stores the returned word at offset 3 in a5, clears a word at offset 142, copies a long from one global to another, copies a third global into a fourth, and then returns.
- `printer_device_setup_call`: Reads two stack words into registers, pushes them, calls a printer routine, stores the result into a global variable, zeroes several other globals, and returns.
- `clear_global_word`: Clears the word at offset 13c of a global variable, then jumps into another routine.
- `increment_printer_counter`: Calls a setup routine, increments a global counter, and loops back while the counter stays within a stored limit, then calls two more routines and stores the result through a pointer before branching based on the outcome.
- `accumulate_and_multiply`: Multiplies a byte from a global by a word from another global, adds the product to a third global, increments a counter, and branches away when a fourth global exceeds the accumulated value.
- `increment_field_and_compare`: Adds the word pointed to by a3 to the word at offset 4 of a4, then loads that word into d0 and compares it with the global long at offset 300.
- `setup_pointers_and_clear_global`: Saves three registers, loads three global pointers into them, clears a global word, writes a global word into a structure field, then branches away.
- `push_globals_and_branch`: Pushes two 16-bit values from global variables at offsets 130 and 128 onto the stack, calls a routine that adjusts a long word through a pointer, stores the returned word at offset 3 in a5, clears a word at offset 142, copies a long from one global to another, copies a third global into a fourth, and then branches away.
- `clear_printer_fields`: Clears four long fields at offsets 128, 120, 124 and 116 of a structure to zero, clears a word at offset 212, then branches to the code at 1188.
- `compute_printer_dimensions`: Calls a setup routine, loops back while a counter stays within a stored limit, then pushes two global word values onto the stack, calls a routine with them, stores the result into a global field, and cleans up the stack.
- `copy_bytes_and_load`: Copies four consecutive bytes from offsets 34 to 37 of the structure in a0 into offsets -4 to -1 of the frame in a6, then loads a 16-bit value from offset 32 of a0 into d2.
- `decrement_counter_loop`: Increments a counter, decrements another global, and loops back while that global stays positive, then adds a byte into a word and branches away when a comparison fails.
- `increment_and_check_counter`: Adds a 16-bit value from a global variable to another 16-bit value stored through a pointer, then compares the result with the value 300 and branches back to the earlier code if it is less than or equal, otherwise it restores three registers and returns.
- `load_constants_and_transfer`: Loads the decimal constants 366, 320 and 338 into registers a2, a4 and a3 respectively, clears the 16-bit value at offset 4 of the structure pointed to by a3, copies the 16-bit value at offset 124 of the structure pointed to by a2 into offset 4 of the structure pointed to by a4, then branches to another routine.
- `adjust_long_from_globals`: Pushes two zero-extended words from global variables at offsets 130 and 12e onto the stack, calls adjust_long_by_doubled_words, stores the returned long into the structure at a3, then jumps into another routine.
- `clear_and_copy_globals`: Clears a word at offset 142, copies a long from one global variable into another, copies a third global long into a fourth, copies a word from a fifth global into a sixth, and then branches away.
- `store_printer_result`: Pushes two global word values onto the stack, calls a routine through a fixed address, stores the result word, clears four long fields and a word, pops the stack, then branches to 1292.
- `advance_and_store_printer_result`: Increments a 16-bit global variable, reads a second global and compares it to the incremented value, and returns early when that value is greater.
- `compute_and_store_printer_result`: Multiplies a signed byte read from a table at offset 250 by a 16-bit global at offset 222 and adds the product to a 16-bit global at offset 220, then increments a 16-bit counter at offset 226.
- `decrement_field_then_compare`: Decrements the word pointed to by a3 and returns when it becomes zero, otherwise increments the word at offset 4 of a4.
- `setup_registers_and_jump`: Saves a2 through a4 on the stack, then loads the constants 366, 320, and 338 into a2, a4, and a3.
- `adjust_long_via_stack`: Pushes the two 16-bit values from the global variables at offsets 130 and 128 onto the stack, calls adjust_long_by_doubled_words to modify the long pointed to by a3, stores the result back into that long via a3, then returns.
- `copy_globals_and_branch`: Clears a word at offset 142, copies a long from one global to another, copies a second long into a third global, copies a word into a fourth, then branches away.
- `initialize_printer_fields`: Clears four global long fields to zero, calls a setup routine, pushes two global word values onto the stack, calls a routine with them, stores the result into a global field, and cleans up the stack.
- `compare_result_branch`: Calls another routine with two stack pointers, then compares the result with one and branches away when it is not greater.
- `update_printer_counters`: Increments a global word, decrements another and returns when it reaches zero, adds a byte into a word, compares two words and branches when unequal, calls a routine, stores the result, and returns or branches based on the result.
- `decrement_and_compare_offsets`: Decrements a word at offset 3 in a5, returns when it becomes non-positive, otherwise increments a word at offset 4, compares it against a global word at offset 124, and returns when it is below that value.
- `adjust_long_by_doubled_words`: Reads two stack words and a stack pointer, doubles the first value and adds it to a long at the pointer, then subtracts the doubled second value from that same long before restoring registers and returning.
- `save_registers_check_flag`: Saves registers d2 through d5 on the stack, loads a byte from offset 23, a word from offset 26 and a word from offset 30 of the stack frame, then checks a global byte at offset 162 and returns immediately if it is zero.
- `compare_byte_and_set_flag`: Masks a stack word to a value between 0 and 3, uses it to index a table, compares the fetched byte with another register, and sets the return value to 1 or -1 depending on whether the comparison is greater or not before restoring registers and returning.
- `conditional_adjust_pointer`: Saves register d2, loads a pointer from the stack frame, calls a routine with the constant ten, and if the result is less than four it adds ten to the pointer and calls another routine before restoring d2 and returning.
- `call_read_pixel`: Library stub: loads its arguments and the graphics library base, calls ReadPixel and returns its result to the caller.
- `call_get_r_g_b4`: Library stub: loads its arguments and the graphics library base, calls GetRGB4 and returns its result to the caller.
- `call_get_prefs`: Library stub: loads its arguments and the intuition library base, calls GetPrefs and returns its result to the caller.
- `clear_struct_and_call_printer`: Clears a byte at offset 58 of the structure pointed to by a2, pushes two addresses, and calls a routine through a fixed address, then checks a word at offset 2614 and branches based on its value.
- `index_pointer_table_and_jump`: Doubles d3 twice into d0, uses it as an index into a pointer table loaded from a global long at offset 14, and stores the resulting pointer into d5 before jumping to another routine.
- `jump_to_offset_160`: Jumps to the routine located at offset 352 within the same code block, transferring control to that target without returning.
- `test_byte_and_branch`: Tests a byte at offset 2 in a2, returns when it is nonzero, otherwise branches away.
- `load_constant_and_branch`: Loads the constant 32 into a4 and branches into another routine without returning.
- `process_command_bytes`: Increments a pointer, converts two bytes into words and pushes them, calls a routine, discards the results, and loops over the bytes until one is zero before pushing a value, calling a routine, and returning.
- `load_printer_segment`: Checks a global pointer and branches away when it is clear, then unlocks a segment, loads a file from a directory, stores the loaded address and a segment pointer into global fields, and copies a string into a buffer.
- `unlock_and_query_current_dir`: Saves a register on the stack, calls the unlock routine, saves another register, calls the current directory routine, removes both saved values from the stack, loads 255 into the return register, restores the remaining registers, and returns.
- `invoke_handler_if_active`: Checks a flag at a fixed offset of a base pointer and returns early if it is clear, otherwise invokes a handler pointer and then calls the unload segment routine before restoring the stack and returning.
- `compare_uppercase_stack_bytes`: Converts two stack bytes to uppercase by subtracting 32 when each falls between 97 and 122, compares the converted values, and sets the return value to 1 when they match before returning.
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
- `clear_register_return`: Overwrites a register with three successive stack values, clears it, and returns without doing anything else.

### Epson

- `return_constant_10`: Returns the constant an address in d0 to the caller without any other effect.
- `open_three_libraries`: Saves a pointer, sets up the library context, opens three libraries by name, stores each returned handle into a global variable, and returns when any open fails.
- `call_library_twice_2`: Switches the library context, calls a library routine twice with different arguments, restores the context, sets a register, and branches away.
- `call_library_routines_2`: Loads a library base, then makes three separate calls to the same library routine using different argument pointers before restoring the saved base and returning.
- `return_constant_11`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_12`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_library_sequence`: Invokes a routine from a global table and returns if it signals a nonzero result, otherwise initializes a structure, then makes a sequence of library calls for opening, output, and status operations before restoring registers and returning.
- `dispatch_stack_value`: Saves several registers, reads values from the stack, sets up pointers, and dispatches to a routine via a jump table based on a stack value.
- `clear_registers_return`: Clears a register, copies it to another register, restores saved registers, and returns.
- `initialize_printer_state`: Saves several registers, loads three pointers from the stack, zeroes two registers and sets three constants, then compares a structure word against a value and branches away when it differs.
- `write_printer_string`: Writes the characters of a fixed string into a buffer at successive offsets, then clears the byte at offset 12 and the byte at offset 14 of the same buffer.
- `copy_printer_byte`: Checks whether the word at offset 2646 of the pointed-to structure equals 2048 and returns without doing anything when it does not.
- `copy_and_decrement_byte`: Loads a pointer from the stack, reads a byte from it and decrements it, stores the result at the address in a1, increments another pointer, copies a byte from it into the address in a0, and branches to 322.
- `copy_byte_increments_counters`: Copies a byte from a table indexed by a counter in d5 to the address formed by adding a4 to a counter in d3, incrementing both counters, and returns when the d5 counter reaches 256.
- `compare_word_store`: Compares the word at a3 with 66 and branches away when they differ, then loads a pointer from d1 and stores the byte 253 at offset 2720 of that pointer.
- `check_1024_store_137`: Loads a pointer from the address in d1, compares the word at that location with the decimal value 1024, and if they are equal stores the byte 137 at the address in d2 before returning.
- `store_value_80`: Copies the value 80 into the byte at the address in a0, then branches to 424.
- `copy_printer_byte_sequence`: Increments the two byte counters in d3 and d5, computes a destination address by adding the base in a4 to the counter in d3, copies the byte at that offset from the table pointed to by a0 to the destination, then compares d5 against 8 and repeats the copy if d5 is greater, otherwise it jumps to the routine at 630.
- `compare_word_set_byte`: Compares the word at a pointer with the decimal value 32 and returns early when they differ, then returns early when the byte at another pointer is nonzero.
- `clear_byte_if_nonpositive`: Tests the byte at a3 and returns if it is positive, otherwise clears that byte, stores the value 30 at a1, and branches away.
- `set_register_branch`: Sets a register to a value and branches away.
- `compare_33_store_29`: Compares the word at the address in a1 with the decimal value 33, and if they match tests the byte at a3, storing 255 there and 29 at a1 when that byte is zero.
- `check_clear_and_branch`: Tests the byte at the pointer in a3 and returns if it is less than or equal to zero, otherwise clears that byte, stores the decimal value 28 at the word pointed to by a1, and branches into another routine.
- `branch_to_other_routine`: Contains a single branch instruction that jumps to another routine and does not return.
- `write_control_sequence`: Compares a register against several values, writes control bytes to a destination, and otherwise builds a sequence of bytes through an incrementing index.
- `write_byte_if_nonzero`: Tests a word at a pointer and returns if it is nonzero, otherwise loads a pointer from a register, writes a byte to a fixed offset of that pointer, restores registers, and returns.
- `call_library_vector_198_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf:Preferences

- `parse_string_and_call_libraries`: Saves the stack pointer and registers, initializes the environment base, and branches to a handler if a task pointer is valid, otherwise parses a string into tokens and calls library routines to write them.
- `restore_stack_and_call`: Restores the stack pointer from the value saved in the resident structure, pops the original return address, then loads the exec library base and calls the library function at offset minus 414.
- `call_two_library_vectors`: Loads a pointer from offset 92 of the structure in a4 and calls the library function at offset minus 480, then loads the same pointer again and calls the library function at offset minus 372.
- `store_pointers_from_globals`: Loads a pointer from a global variable, stores a word from it into another global variable, then loads a second pointer and stores a word from it into another global variable, and returns.
- `manage_preferences_task`: Saves registers, opens a library and stores the result, then runs a loop that indexes a table by a counter, clears bytes, compares values, and branches to different code paths before restoring the registers and returning.
- `open_libraries_dispatch`: Saves registers, loads three pointers from data, and calls an exec library routine to open a library, storing the result and returning early when it fails.
- `init_node_and_dispatch`: Initializes a library node through a fixed address and returns early when the result is nonzero.
- `push_globals_and_call`: Reads words from global variables, pushes them onto the stack, and calls several routines in sequence, then restores the stack and returns.
- `call_twice_with_flag`: Saves register d2, reads a pointer from a global variable, and calls another routine twice with a computed address and a flag, first with a cleared value and then with the decimal value one.
- `loop_over_word_groups`: Saves registers, reads a flag and pointers from the caller's frame, and returns immediately if the flag is zero, otherwise loops over two groups of words calling a routine for each and restoring registers.
- `split_byte_and_call_remote`: Reads a value from the stack and compares it against four; if it is not less than four it returns immediately.
- `init_preference_tables`: Initializes a table of 111 words at offset 18 of the structure in a2 to zero, then stores the values from a lookup table at offsets 0 through 12 of the structure in a2, and stores two computed indices into the structure at offsets 34 and 18.
- `dispatch_via_jump_table`: Saves several registers and points them at global variables, then reads a pointer and a dispatch index from a library base.
- `read_args_call_routines`: Reads two arguments from the stack and returns early when a word argument is zero.
- `lookup_tables_and_release_semaphore`: Reads a byte index from the stack and uses it to look up values in several tables in global variables, pushing the computed results onto the stack.
- `compute_and_push_args`: Saves several registers, reads a pointer from a global variable, and pushes a series of computed word values onto the stack before calling a routine at a fixed address.
- `clamp_byte_arguments_and_update`: Reads two byte arguments and clamps them into a valid range, then compares a combined value against a limit to decide whether to clamp them further.
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
- `pack_and_call_routine`: Reads two 16-bit values from the structure in a7 at offsets 34 and 38, subtracts 15 from the first, and reads two 16-bit values from offsets 42 and 46.
- `process_nibbles_via_library`: Splits a stack argument into nibbles stored in a global variable, then for three different values loads structure fields, calls a multiply routine and a library vector each time before restoring registers and returning.
- `iterate_and_call_routines`: Loads a counter and a flag from the caller frame, then repeatedly indexes several tables by the counter to push argument groups and call four different routines, incrementing the counter until the loop condition fails.
- `increment_pointer_check_byte`: Copies the pointer from the stack into a0, increments it by one, and returns zero if the byte at the new address is zero, otherwise returns the difference between the incremented address and the original pointer.
- `invoke_indirect_routine`: Pushes the two stack arguments, a negative sentinel, and the value at offset 8 of a global variable onto the stack, then performs an indirect jump to a routine address stored in a global variable and removes the pushed values before returning.
- `compute_offsets_and_call`: Saves registers, reads two bytes from the stack, then repeatedly computes indexed offsets and pushes argument pairs onto the stack while calling the routine in a2, finishing by restoring the saved registers.
- `branch_on_status_bits`: Saves registers, reads values from the stack, then branches on individual bits of a status word to compute different indexed offsets and push argument pairs before calling the routine in a2, restoring the registers at the end.
- `compare_value_to_nine`: Reads a 16-bit value from offset 6 of the structure in a7, and returns zero if it is not nine, otherwise returns eight.
- `decrement_value_check_zero`: Decrements a 16-bit value read from offset 6 of the structure in a7, and returns zero if the result is not zero, otherwise returns sixteen.
- `load_pointer_and_call`: Saves registers, reads three 16-bit values from the stack, then repeatedly loads a pointer from a global variable, pushes arguments onto the stack, and calls the routine in a2, restoring the registers before returning.
- `push_values_if_nonzero`: Reads a word from the caller's frame and returns immediately if it is zero, otherwise pushes four fixed values onto the stack and branches to a routine that consumes them.
- `scale_and_store_value`: Reads a word from a global variable, multiplies it by two, adds a constant, and stores the result into a structure, then returns.
- `split_and_store_nibbles`: Reads a word from a global variable, splits it into nibbles, stores each nibble into consecutive bytes of a structure, and returns.
- `compute_division_result`: Computes the quotient and remainder of a 32-bit unsigned division by a 16-bit divisor, storing the quotient in a global variable and the remainder in another, and when the divisor is zero it instead scans a 16-entry table to find the first entry greater than the divisor, storing the index and the difference.
- `multiply_16bit_values`: Multiplies the 16-bit values in d0 and d1 and returns the 32-bit product in d0, using the 68000's 16-bit multiply result and discarding its high word.
- `reorder_args_call_divide`: Reorders the arguments on the stack, calls the division routine, restores the saved register, and returns without using the result.
- `reorder_args_return_quotient`: Reorders the arguments on the stack, calls the division routine, moves the quotient into d0, restores the saved register, and returns.
- `handle_signs_return_quotient`: Negates the operands when they are negative, calls the division routine, and returns the negated quotient in d0 unless the quotient is zero.
- `negate_operands_return_quotient`: Negates the operands when they are negative, calls the division routine, and returns the negated quotient in d0 unless the divisor is zero.
- `save_and_jump_routine`: Saves two registers, copies two words from the stack into them, and jumps to another routine.
- `index_and_call_library`: Increments a byte counter by one, converts it to a scaled 32-bit index, and pushes that index onto the stack as an offset from a table base, then calls a library stub to process the argument.
- `load_stack_values_and_branch`: Loads two 16-bit values from the stack frame at offsets 14 and 18 into registers d2 and d3, then branches to another routine.
- `push_sentinel_and_call`: Pushes a sentinel value onto the stack, increments a byte counter, computes a scaled 32-bit index, and pushes both the index and the sentinel before calling a library stub, then restores the stack.
- `dispatch_byte_selector`: Loads a byte selector from a global variable, rejects values outside the zero-to-eight range, and dispatches to one of eight handler addresses from a jump table using the selector as an index.
- `divide_and_store_digits`: It reads a value from a global variable, calls a division routine, and stores the resulting quotient and remainder bytes into five consecutive bytes of a structure.
- `convert_values_to_base_11_2`: It loops over three values, converting each from base 10 into base 11 digits and storing the quotient and remainder bytes into a table, then calls a helper routine with the accumulated values.
- `convert_values_to_base_11`: Converts three signed 16-bit values passed on the stack from base 10 into base 11 digits, storing the quotient and remainder bytes into consecutive global bytes, and does the same for a fourth value.
- `initialize_preference_nodes`: Initializes three library nodes by calling their constructors, then copies four byte pairs from a source structure to a destination structure, computes a table index from two 8-bit values, looks up an entry in a table, and calls two more library nodes before returning.
- `compute_table_indices_and_divide`: Reads several byte fields from a global structure, subtracts 72 from each and converts the results into table indices, then loops over a set of entries calling a division routine and a memory-freeing routine while decrementing two counters.
- `clamp_digits_convert_base`: Clamps four digit bytes to printable ASCII characters, then converts the four base-10 digit values into base-11 digits and stores the quotient and remainder bytes into cons.
- `decrement_and_wrap_digits`: It decrements digit bytes in a structure, wrapping them to their maximum when they fall below a threshold, then computes several derived values and dispatches to a table.
- `loop_convert_base_digits`: Checks a loop counter, resets it when it reaches its limit, then converts three base-10 values into base-11 digits and stores the quotient and remainder bytes into cons.
- `open_timer_device`: It opens a timer device, allocates a message port and signal, initializes a structure, and registers the resources so they can be freed later.
- `allocate_ports_and_check_flags`: Allocates two message ports via library calls and stores their pointers, then checks a signal bit and aborts pending I/O if set.
- `allocate_and_store_handle`: Calls a routine to obtain a handle, and if the handle is nonzero stores it in a global variable, writes a pointer and a routine address into it, then returns.
- `free_timer_resources`: It checks a signal bit, and if set it aborts pending I/O, removes the task, closes the device, and frees the allocated memory blocks.
- `send_io_struct_setup`: Stores a 16-bit value at offset 28 and a 32-bit value at offset 32 of a structure pointed to by a0, then calls the I/O handler routine and returns its result.
- `copy_buffer_to_table`: Clears a byte in a table at offset 290 of the frame and then iterates over a 30-byte region, copying non-space bytes from a buffer at offset 260 into that region while skipping spaces.
- `compare_byte_strings_2`: Compares two null-terminated byte strings pointed to by two arguments, returning one when the first is not greater than the second and zero otherwise.
- `allocate_and_fill_buffer`: Allocates memory via an exec library call, stores the returned pointer in a global variable, and fills a 30-byte region of that buffer with a byte from another argument.
- `unlink_and_free_entry`: Removes an entry from a doubly linked list in a global variable by unlinking it and freeing its memory, then clears the list head if it becomes empty.
- `fill_buffers_from_table`: Clears a word in a global variable, then loops over a table, calling a routine for each entry and copying characters into two separate buffers until the entries are exhausted.
- `copy_table_to_buffer`: Copies characters from a source table into a local buffer, skipping entries marked as empty and padding the remainder with spaces.
- `read_value_and_call_library`: Reads a value from a global variable, calls a routine through a fixed address, and returns early when the result is nonzero.
- `check_value_call_routine`: Clears a register, loads a pointer from a global variable, and returns early when the long at its offset does not equal sixteen.
- `call_and_check_result_2`: Calls a routine at a fixed address, saves the result in d2, pushes a pointer from a global variable onto the stack, and calls another routine before returning based on the second result.
- `decrement_until_zero`: Decrements the decimal value in d0 and returns when it reaches zero, otherwise branches back to repeat the decrement.
- `invoke_library_vector`: Pushes two constants onto the stack and calls a library vector, then clears d0, adjusts the stack pointer, and jumps to a shared exit routine.
- `push_constants_branch`: Pushes the constants 16 and 248 onto the stack, then branches into the middle of another routine to invoke a library vector with those arguments.
- `call_and_store_result_4`: Loads a pointer from a global variable, calls a routine with the long at offset 56 of that structure in d0, stores the result in a global variable at offset 1432, and returns when that long is zero.
- `restore_and_return`: Restores the caller's registers and returns immediately.
- `copy_words_between_structures`: Copies four consecutive 16-bit words from the structure pointed to by a0 to the same offsets in the structure pointed to by a1, then returns.
- `dispatch_preference_action`: Saves registers a2 through a4, loads a pointer from a global variable, reads three fields from the pointed-to structure, then compares a word at offset 38 against the constants 70 and 79 and jumps via an indexed table when the value falls within that range.
- `invoke_library_vector_one`: Pushes a constant onto the stack and calls a library vector, sets d0 to one, adjusts the stack pointer, and jumps to a shared exit routine.
- `pop_registers_and_return`: Pops the registers d2, d3, d4 and a2 through a4 from the stack, discarding them, then returns.
- `decode_and_check_value`: Reads a longword from a global variable, splits it into nibbles stored at offsets 1460, 1464 and 1462, and if the value matches a constant and a status byte equals fifteen, pushes a computed argument and calls a routine before clearing the variable.
- `decrement_and_branch_on_zero`: Loads a word from the stack into d0, subtracts one, and if the result is nonzero it jumps to a shared routine; otherwise it loads zero into d0 and also jumps to that routine.
- `return_constant_13`: Returns the constant an address in d0 to the caller without any other effect.
- `return_stack_long`: Loads a long word from the first argument on the stack into d0 and returns it to the caller.
- `configure_signal_port`: Reads a 16-bit value from a global variable, indexes a table in another global structure using that value, and pushes the four resulting 32-bit entries onto the stack, then calls a routine to add a signal port and clears or sets a flag byte at offset 36 of the structure.
- `push_index_branch`: Pushes register d2 onto the stack, loads a word from the stack into d1, clears d2, then branches into the middle of another routine.
- `compare_indexed_word`: Increments d2, doubles it into d0, compares a word indexed by that value against d1, and if the comparison fails it returns zero otherwise it returns the decremented d2.
- `decrement_d2_and_return`: Loads zero into d0, copies d2 into it, subtracts one, pops d2 back from the stack, and returns the decremented value.
- `rebuild_tables_from_global`: Clears a global structure and a counter, then calls the library routine with an index derived from that counter; when the counter is nonzero it copies a block of longs from one table to another, copies several longs from the global structure into a second global area, and finally inverts and stores a range of bytes in a third table.
- `call_exec_library_entry`: Pushes two arguments onto the stack, calls the exec library entry point, removes the arguments from the stack, and returns.
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
- `duplicate_pointer_field`: Copies the pointer stored at offset 4 of the structure in a0 into the four bytes it points to, adds 4 to that value, clears the value at offset 4 of the structure, and stores the pointer into the four bytes at offset 8 of the structure before returning.
- `copy_and_delete_config`: Opens a system configuration file, writes its contents to a second file, and deletes the original when the copy succeeds.
- `read_config_into_memory`: Opens a system configuration file, reads its contents into allocated memory, and forwards the buffer to another routine for processing.
- `create_task_message_port`: Finds the current task, allocates a message port and a signal, initializes a structure with the task and signal data, and returns the structure pointer.
- `free_preferences_resources`: Reads a pointer from the stack, checks the long word at offset 16 of the pointed structure, and if it is non-zero calls another routine and clears the signal bits at offset 8 and offset 20.
- `allocate_library_handle`: Calls a library vector twice to obtain a handle, and if that fails calls another library vector and returns.
- `handle_preferences_signal`: Reads a flag from a global variable and sets bit 0 of a status byte in another global variable, then calls a routine with a pointer to a data structure and a computed signal value.
- `dispatch_preferences`: Saves several registers, loads a fixed address and calls it, then loads a pointer from a global variable, shifts a byte field, calls another routine through a fixed address, and returns its result.
- `compare_and_call_routine`: Clears d3, loads a pointer from a2, compares a long at offset 14 of that structure with 16, and calls a routine through a fixed address when they match.
- `select_printer_handler`: Compares a structure field with the values 64 and 32, calls a routine when it matches, calls another routine through a fixed address, and branches to different handlers based on the result.
- `decrement_and_loop_d0`: Decrements a 16-bit value in d0 and branches back to an earlier instruction while the result is nonzero, otherwise falling through to a branch that leaves the routine.
- `push_args_and_call_routine`: Pushes two fixed values onto the stack, calls a routine by a negative offset, clears d0, removes the pushed values from the stack, and branches away.
- `push_constants_branch_2`: Pushes the decimal constants 248 and 16 onto the stack, then branches into the middle of another routine to invoke it with those two values.
- `fetch_message_and_store`: Loads a pointer from a global variable, pushes a value from its field, and calls a routine through a fixed address.
- `restore_registers_and_return`: Restores the saved registers from the stack and returns immediately.
- `copy_struct_fields_2`: Copies five fields from one structure to another, then indexes a table using a counter and copies a word from it, and returns.
- `init_and_dispatch_preferences`: Initializes three pointers from global variables, calls a routine through a fixed address, and returns early with a flag set when the result is nonzero.
- `index_table_by_value`: Reads a word from a structure, compares it against two constants, and if within range indexes a table and jumps to the selected routine.
- `clear_register_and_return`: Clears the 32-bit result register d0 to zero, restores the caller's d2-d3 and a2-a4 registers from the stack, and returns.
- `compare_byte_and_branch`: Saves several registers, reads a word from the caller's frame, compares a byte at a fixed address to 3, and branches away with d0 cleared when they differ.
- `store_nibbles_and_call`: Extracts nibbles from a word, stores them into a structure, then pushes several values and calls a routine three times, and returns.
- `pack_arguments_and_call_remote`: Reads several word-sized arguments from the stack and packs two of them into a 32-bit value by shifting and adding, then stores that value at the address taken from a global variable.
- `index_table_and_call_routines`: Reads a byte index from the stack and uses it to compute offsets into a table in a global variable.
- `push_pointers_and_call_vector`: Saves register a2 and points a2 at a global variable, then pushes a pointer, a flag value, and the global base onto the stack three times.
- `compute_offsets_and_call_3`: Reads several words from the stack, computes offsets, and calls a routine conditionally based on a flag, then restores registers and returns.
- `loop_and_call_with_bit_values`: Saves registers, calls a routine with a stored pointer, then loops sixteen times computing bit values from a table and pushing arguments for another routine before restoring registers and returning.
- `set_bit_fields_from_arguments`: Reads three word-sized arguments from the stack and uses one of them as a shift amount to compute bit masks.
- `setup_signals_with_branch`: Saves several registers and points them at global variables, then pushes arguments and calls the first routine.
- `allocate_memory_from_stack`: Pushes a flag constant and a size value from the stack, calls the memory allocation routine, and returns the allocated block pointer.
- `free_memory_block_pointer`: Pops a memory block pointer off the stack into d2, calls the memory-freeing routine with that pointer, then restores d2 and returns.
- `allocate_memory_block`: Reads a size from the second argument and a flag from the first argument.
- `free_memory_block`: Takes a memory block pointer from the stack, marks it as freed by writing 255 to the byte at offset 8, sets the 32-bit values at offsets 20 and 24 to -1, reads the block size from the 16-bit value at offset 18, and calls the memory-freeing routine with the size and pointer before returning its result.
- `copy_bytes_and_dispatch`: Clears a counter, copies sixteen bytes from a table into a destination structure, then branches back while the counter is below sixteen.
- `compare_64_32_dispatch`: Clears d2, sets a byte in a global variable to one, compares a long at offset 20 of a structure with the decimal values 64 and 32, and branches to different routines depending on the comparisons.
- `loop_preference_copy`: Decrements the 16-bit counter in d0 by one and, while it is not zero, branches to a routine that copies preference bytes; when the counter reaches zero it branches to another routine.
- `call_and_check_result`: Pushes the constants 248 and 16 onto the stack and calls two routines, then returns immediately if the result in d0 is zero.
- `copy_preference_bytes`: Returns immediately if a byte in a global variable is non-zero, otherwise loops over 31 bytes copying a value from one table into two other tables.
- `clear_tables_and_call`: Clears d4 and loops over a 30-byte region, zeroing bytes in two tables, then copies a word from a global variable and calls a routine before returning zero.
- `push_call_store_result`: Loads a pointer from a register, pushes a structure field onto the stack, and calls a library routine, storing the result and advancing the stack.
- `restore_registers_and_return_2`: Restores registers d2 through d4 and a2 through a4 from the stack and then returns.
- `copy_structures_and_clear`: Copies a block of words from the structure in a0 to the structure in a1, then copies a byte and the words at offsets 1, 126, 158, 160, 162, 164, 166, 176, 178 and 180.
- `dispatch_on_indexed_word`: Saves registers, loads a pointer from a global variable, reads a word from the structure it points to, bounds-checks it, and uses it to index a jump table before dispatching.
- `restore_registers_and_return_3`: Restores the caller's registers d2, d3, a2, a3 and a4 from the stack and sets d0 to zero before returning.
- `dispatch_library_operations`: Checks a word in a global variable and, if nonzero, runs a sequence of library dispatch calls to set signals, allocate and free traps, and add ports and list heads, while indexing into a table via a global vector.
- `zero_stack_argument`: Reads a word from a stack argument into d0, overwrites it with zero, and returns.
- `clamp_and_call_routine`: Compares the word at a global variable with the decimal value one and sets it to one if it is below, then compares it with another global variable and stores the word when it exceeds that value before calling another routine.
- `call_four_routines`: Saves register a2, then calls four different routines while passing various global variables and computed offsets as arguments, and finally restores a2 and returns.
- `clamp_and_call_routine_alt`: Compares the word at a different global variable with the decimal value one and sets it to one if it is below, then compares it with another global variable and stores the word when it is less than or equal to that value before calling another routine.
- `call_four_routines_alt`: Saves register a2, then calls four different routines while passing different global variables and computed offsets as arguments, and finally restores a2 and returns.
- `compare_set_and_call`: Sets d0 to the decimal value one and returns if it is less than or equal to the value at a global variable, otherwise sets a byte at another global variable and calls another routine.
- `call_four_routines_alt2`: Saves register a2, then calls four different routines while passing yet another set of global variables and computed offsets as arguments, and finally restores a2 and returns.
- `read_flags_and_store`: Reads a value from a global variable, calls a routine through a fixed address, then reads two more values and stores them into a structure.
- `compare_64_32_call`: Clears d2, compares a long at offset 20 of a structure with the decimal values 64 and 32, calls a routine when they match, and branches to different routines based on the result.
- `decrement_check_d0`: Decrements the 16-bit value in d0 and returns if it is not zero, otherwise jumps to a routine that receives the constants 248 and 16.
- `push_constants_and_call`: Pushes the decimal constants 248 and 16 onto the stack, calls a routine with those two values, clears the 16-bit result in register d0, adds 8 to the stack pointer, and jumps to the routine at offset 446.
- `push_constants_branch_alt`: Pushes the constants 248 and 16 onto the stack, calls a routine with those two values, then branches into the middle of another routine.
- `push_call_store_flag`: Loads a pointer from a register, pushes a structure field onto the stack, and calls a library routine, storing the result and advancing the stack.
- `restore_registers_and_return_4`: Pops a longword from the stack into d2 and restores a2, a3 and a4 from the stack, then returns.
- `copy_struct_fields`: Copies a word from the structure in a0 to the structure in a1, then copies the bytes at offsets 183, 182 and 184 and returns.
- `index_on_clamped_byte_value`: Loads a pointer from a global variable and reads a word from offset 38 of the pointed-to structure, then checks whether that value falls within the inclusive range 94 to 111.
- `set_value_one_return`: Loads the constant 1 into d0 and jumps to the routine at offset 1564, which restores the saved registers and returns.
- `set_value_two_return`: Loads the constant 2 into d0 and jumps to the routine at offset 1564, which restores the saved registers and returns.
- `return_immediately_2`: It is an empty routine consisting of a single branch that returns immediately.
- `restore_registers_and_return_5`: Restores the values of data registers d2 and d3 and address registers a2, a3 and a4 that were saved on the stack, then returns.
- `dispatch_library_store`: Runs a sequence of library dispatch calls to set signals, allocate and free traps, and index into a table via a global vector, storing the resulting word into a global variable.
- `dispatch_library_modify`: Runs a sequence of library dispatch calls to set signals, allocate and free traps, and index into a table via a global vector, then clears the high nibble of a global byte and merges a global byte into it.

### Empire - War Game of the Century v2.05 (1987)(Interstel Corporation)[cr][a].adf:Empire

- `branch_into_routine`: Unconditionally branches into another routine, performing no work of its own.
- `store_index_in_table`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, otherwise clearing the next slot.
- `build_stack_message`: Saves several registers, sets up a stack frame, and calls a library routine through a global base.
- `invoke_callback_and_cleanup`: It reads a callback pointer from a global variable and invokes it, then performs a sequence of library calls to clean up before returning.
- `call_library_via_a0`: Loads the address of a structure at offset 92 from a5 into a0 twice and calls two different library entry points through a6, then returns.
- `set_100_call_library`: Sets d0 to the decimal value 100, calls a library routine, stores the result in a global variable, and returns if that result is zero.
- `jump_to_address_zero`: Jumps to address zero and never returns.
- `forward_stack_arguments`: Restores the saved registers from the stack, loads a library base pointer into a6, and calls a library entry point passing the caller's arguments from the stack, then restores those registers and returns.
- `index_table_with_counter`: Clears a 16-bit counter in a global variable, then compares a word at offset 8 of the structure in a5 against a value at offset -420 and branches away when it is greater than or equal.
- `call_and_store_result_5`: Loads a word from offset 8 of the structure in a5, calls a routine through a fixed address, stores the returned longword at offset -6 of that structure, and returns zero when the result is negative.
- `check_flag_and_return`: Loads a pointer from a global variable, tests bit 2 of a byte at offset 1, and returns immediately when that bit is clear.
- `request_signal_and_return`: Clears a word, pushes a value from a pointer field, and calls a routine through a fixed address.
- `setup_and_call_with_args`: Sets up a stack frame, saves registers a2, loads a pointer from a global at offset 524 in a4, returns early when it is zero, checks two flag bits at offset 19, and otherwise computes a difference of two longs, stores it as a word, and calls a routine with three pushed arguments.
- `check_flag_and_call_routine`: Sets up the stack frame and loads a pointer from the caller, then returns immediately when a bit in the structure is clear.
- `check_flags_and_call`: Clears a word at offset -2 of the structure in a5, then reads a word at offset 18 of the structure pointed to by a0 and returns immediately when its low two bits are non-zero.
- `tiny_moveq_unlk_rts_3`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `deref_stack_pointer_and_call`: Allocates a 4096-byte stack frame on the stack in a5, saves the incoming pointer value at offset -4, then checks it.
- `test_and_call_routine`: Tests a long at offset -4 of a5 and branches away when nonzero.
- `clear_memory_and_call`: Writes a long value into a pointed structure, then fills 24 consecutive bytes at that address with zero using a loop before pushing arguments and calling a routine.
- `check_flag_and_dispatch_byte`: Loads a pointer from the caller, returns immediately when a word in the pointed structure is zero, otherwise calls a routine and then compares a byte from that structure against two constants, clearing a flag when the second matches.
- `compare_byte_and_dispatch`: Stores the word 32768 at offset -6 of the structure in a5, increments a counter at offset -10, compares a byte in a pointed table against a constant and sets a result register accordingly, then dispatches based on another byte value.
- `store_value_and_branch_2`: It tests a byte in a structure, and when that byte is non-zero it stores a value in the structure and branches into another routine.
- `store_value_and_branch`: Builds the value 16386 in d0, stores it as a word at offset -2 of the structure in a5, then branches into another routine.
- `test_flag_set_result`: Tests a byte at offset -11 of the structure in a5 and returns immediately when it is zero.
- `push_flagged_value_call`: Builds a word with bit 15 and bit 12 set, pushes it along with a value of 12 and a pointer from a5, then calls a routine and stores the returned value into a local variable before incrementing it.
- `store_one_and_branch_2`: It tests a byte in a structure, and when that byte is non-zero it stores the value one in the structure and branches into another routine.
- `store_one_and_branch`: Stores the value 1 as a word at offset -2 of the structure in a5, then branches into another routine.
- `check_structure_byte_zero`: Reads a byte from the structure pointed to by a5 at offset 11, and if that byte is zero it jumps to the following routine; otherwise it loads the value 2 into d0 and continues.
- `push_multi_bit_value_call`: Builds a word with bits 15, 8 and 9 set, pushes it along with a value of 12 and a pointer from a5, then calls a routine and stores the returned value into a local variable before incrementing it.
- `store_two_and_branch_2`: It tests a byte in a structure, and when that byte is non-zero it stores the value two in the structure and branches into another routine.
- `store_two_and_branch`: Stores the value 2 as a word at offset -2 of the structure in a5, then branches into another routine.
- `clear_register_and_branch`: Stores the value 0 in d0, then branches into another routine.
- `initialize_structure_fields`: Initializes a structure pointed to by a1 with zero words at several offsets and a pointer copied from another field, then branches to set a flag bit in a status word when a condition is met, otherwise it stores a combined value and returns.
- `load_stack_frame_and_check_counts`: Loads a pointer from the stack frame and two 16-bit values from offsets 12 and 14, then compares them against values in d6 and d7, branching to the called routine when neither counter is exceeded.
- `decrement_and_read_byte`: Compares a counter in d5 with d6 and a counter in d4 with d7, branching away when either is not exceeded.
- `call_and_store_result_6`: Pushes an argument and calls a routine, stores the returned value into a structure in a5, and returns when that value is not the marker, otherwise it copies a register and jumps away.
- `copy_word_to_byte_pointer`: Reads a word at offset -6 of the structure in a5, writes it to the byte pointed to by a2, advances that pointer, increments a counter in d4, then branches into another routine.
- `increment_counter_d5`: Increments a counter in d5, then branches into another routine.
- `copy_d5_to_d0`: Copies the value of d5 into d0, restores the saved registers from the stack, restores the stack pointer, and returns.
- `copy_byte_table_loop`: Loops over a byte table bounded by two global words, storing each masked byte into a second table and decrementing a counter, returning early when either bound is exceeded.
- `increment_and_loop_value`: Loads a word from the caller, masks it to eight bits, and pushes the values onto the stack before calling a routine.
- `increment_counter_d4`: Increments a counter in d4, then branches into another routine.
- `increment_global_counter`: Increments the 16-bit value stored in the global variable referenced by d5 by one, then branches to another routine.
- `restore_registers_and_return_6`: Restores the caller's registers from the stack and returns.
- `load_word_and_call_routine`: Sets up a stack frame, loads a word from the caller into a register, pushes it on the stack, and calls a routine.
- `check_pointer_and_return`: Sets up a stack frame, saves registers, reads a pointer from the caller, and returns immediately when the pointer is zero.
- `clamp_and_compare_longword`: Compares a longword against 8 and raises it to 8 when below, divides by four with rounding, then compares a longword at offset 4 of a structure and stores a pointer through another pointer when the values match.
- `shift_and_store_longword`: Reads a longword at offset 4 of the structure in a2, subtracts a register from it, and returns immediately when the result is below eight.
- `reload_register_and_branch`: Saves a register on the stack, loads a new value through a pointer, and branches into another routine.
- `combine_and_call_routines`: Reads a word from a global, sign-extends it, and combines it with a register before calling two routines through pointers.
- `clear_d0_and_return`: Sets d0 to zero, restores the saved registers from the stack, restores the stack pointer, and returns.
- `check_digit_range`: Loads a word from offset 8 of the structure in a5 and checks whether it falls between the decimal values 48 and 57 inclusive, returning without setting a flag when it does not.
- `push_call_and_store_result`: Pushes a word from offset 8 of the structure in a5 onto the stack, calls a routine through a fixed address, and stores the result.
- `increment_offset_eight_and_call`: Loads a value from offset 8 of the structure in a5, adds the decimal constant 10 to it, and stores the result at offset 8 of the structure in a0, then calls the queue handler and stores the returned value at offset 4 of its own frame, returning zero when that stored value is zero.
- `store_offset_and_call`: Adds the decimal constant 10 to a value from offset 8 of the structure in a5 and stores the result at offset 8 of the structure in a0, then calls a routine with a constant address and stores the pointer from offset 4 of the structure in a5 when a global long at offset 440 is zero.
- `call_and_store_offset_eight`: Reads the word at offset 8 of the structure in a5, pushes it on the stack, and calls another routine.
- `load_value_and_call`: Reads a pointer from a global long at offset 16392 and returns immediately when it is zero, otherwise it loads a word from offset 8 of the pointed-to structure, pushes it on the stack, and calls a routine using the system library base.
- `link_frame_setup`: Stores the pointer passed at offset 8 on the stack into a1, then writes the value at offset 4 of that structure into offset 4 of the structure at offset 12 on the stack, clears a0, and stores that zero into the structure at offset 8 on the stack.
- `increment_and_scan_table`: Clears a byte and a word in the structure in a4, copies a word into a local variable, and increments that local until a table slot is found to be non-zero.
- `compare_and_index_table`: Compares a word in d1 with a word at offset -164 of the structure in a4 and jumps to a table entry scaled by d1 when they differ, otherwise it stores the constant 24 at offset -576 of a4 and jumps to a routine.
- `scale_index_and_set_flag`: Multiplies a register by 6, adds the product to a base pointer in a4, stores the result as a long at offset 18 in a5, tests a word at offset 14, returns when it is zero, sets a flag bit at offset 15, and clears a word at offset 10.
- `toggle_sign_and_modify_word`: Sets a bit in a word at offset -10 of the structure in a5, then toggles a sign bit of a global word at offset -452 into a word at offset 12 of the structure in a5.
- `set_flag_and_jump_2`: Stores the decimal constant 22 into a long at offset 240 of the structure in a4 and sets d0 to the decimal value 255 before jumping into the middle of another routine.
- `test_masked_value`: Copies a word from offset 12 of the structure in a5 into another register, masks it with 768, and returns immediately when the masked value is zero.
- `test_bit_and_call_routine`: Tests bit 9 of d0 and returns immediately when it is set.
- `call_with_constant_argument`: Loads the constant 1005 into d0, pushes it onto the stack, pushes a pointer from offset 8 of the structure in a5, calls a routine through that address, restores the stack, stores the result at offset 14, and returns 255 when a flag at offset -32712 of a4 is clear.
- `store_stack_values`: Reads a word and a long from the stack frame into memory, then reads a word from the stack frame into d0 before restoring the registers and returning.
- `update_counter_value`: Reads a word from a global variable at offset 572, multiplies it by 5981, adds three, masks the result, and stores the value back into the same global variable.
- `store_word_to_offset`: Stores a 16-bit word from an offset of 8 in a5 to an offset of -572 in a4 and returns.
- `call_indexed_and_store_result`: Copies the word at offset 8 of the structure in a5 onto the stack, calls a routine through the address at offset 4 of that structure, stores the returned long at offset -4 of a5, and returns zero when that value is nonzero.
- `call_indexed_via_pointer_result`: Pushes two values from the structure in a5 onto the stack, loads a pointer from offset 4 into a0, calls a routine through the address at offset 2 of that structure, restores the stack, stores the result in d0 at offset 8, and returns 255 when a flag at offset -32712 of a4 is clear.
- `load_stack_long`: Loads a long from the stack frame into d0 and returns without storing it.
- `load_word_push_call`: Sets up a stack frame, loads a word from offset 12 of the structure in a5 into d0, pushes that value and the pointer at offset 8 onto the stack, stores d0 at offset negative 4 of the structure in a5, calls a routine that returns when a loaded longword is negative, then cleans up and returns.
- `load_and_branch_on_d7`: Loads a longword from offset 12 of the structure in a5 into d7 and returns when it is negative, otherwise it loads a byte value into d0 and branches away.
- `update_clipboard_pointer_2`: Clamps an index to a maximum value, adjusts a pointer table entry, and stores the resulting index and pointer into fields of a structure.
- `update_clipboard_pointer`: Compares the pointer in a2 with the one stored at offset -12 of the structure in a5 and returns when they differ.
- `compare_pointers_and_load`: Compares a pointer from a global variable with a0 and returns when they differ, subtracts d7 from a word in a structure, then loads the decimal value 255 into d0.
- `compare_pointers_subtract`: Compares the pointers in a0 and a1 and branches away when they differ, then tests the longword pointed to by a2 and branches away when it is zero, loads that longword into d1, compares it with d2 and branches away when d1 is not less than or equal, subtracts d7 from a location at offset negative 170 of the structure in a4, loads a value into d0, and branches away.
- `accumulate_and_store`: Adds the value in d7 to the long at offset 4 of the structure in a2 and returns early when that location is zero or differs from d2, otherwise it adds a value from the pointed-to structure to offset 4 and stores the pointed-to value into that location before jumping into another routine.
- `save_and_copy_pointer`: Saves the pointer in a2 onto the stack, copies a longword from offset 20 of the structure in a5 to offset 24, then loads the pointed-to value into a2 and branches away.
- `clear_pointed_structure`: Stores a long from the stack frame into the pointed-to location, clears that location, writes a zero into d0, and stores d0 at offset 4 of the pointed-to location.
- `increment_and_store`: Increments a byte at offset 16060 of a global variable, stores the passed word at the address held in that global variable, and increments that address pointer.
- `clear_stored_pointer_byte`: Clears a word at offset 25724 of a global variable, stores a long passed in the frame at offset 8 into offset 25720, pushes two arguments, calls a routine, then clears the byte at the stored address and loads the cleared word into d0.
- `clear_counters_and_check_minus`: Clears d0 and stores zero at offsets 2, 4, and 8 of the structure in a5.
- `compare_byte_and_return_pointer`: Tests the byte pointed to by a pointer at offset 8 of the structure in a5 and, if it is non-zero, compares the byte at offset 12 against it, returning the pointer when they match.
- `increment_offset_four`: Increments the long at offset 4 of the structure in a5 by one and jumps into the middle of another routine.
- `increment_offset_eight`: Increments the long at offset 8 of the structure in a5 by one and jumps into the middle of another routine.
- `return_zero_value`: Sets d0 to zero and returns without using the stack frame.
- `push_offsets_and_call`: Pushes the words at offsets 12 and 8 of the structure in a5 onto the stack, calls another routine, and removes the two arguments from the stack.
- `compare_lowercase_bytes`: Reads a byte from each of two pointers, converts lowercase letters to uppercase, subtracts the second byte from the first, and returns immediately if the result is nonzero or the second byte is nonzero.
- `compare_two_byte_values`: Tests the word at offset 16 and the bytes pointed to by the pointers at offsets 8 and 12, returning early if any of them is zero.
- `decrement_offset_sixteen`: Decrements the word at offset 16 of the structure in a5 by one and jumps into the middle of another routine.
- `test_word_and_byte_flags`: Tests the word at offset 16 of the structure in a5 and returns with d0 set to 255 when it is non-zero.
- `test_byte_and_set`: Loads a pointer from offset 12 of the structure in a5 and returns with d0 set to the decimal value 255 when the byte it points to is non-zero, otherwise it jumps into the middle of another routine.
- `zero_register_d0`: Clears a 16-bit value in register d0 and returns.
- `convert_byte_to_uppercase`: Reads a byte from the word at the stack offset and branches to a table when the value is outside the lowercase letter range, otherwise subtracts thirty-two from the byte to convert it to uppercase before returning.
- `call_and_store_longword`: Creates a stack frame, pushes a word from offset 8 of the structure in a5, and calls a routine through a fixed address, storing the returned longword at offset -6.
- `index_and_dispatch_byte`: Clears a word in a caller frame, reads a byte from a pointer, and branches to a handler table indexed by that byte when it falls within a range, otherwise it stores the byte and returns.
- `increment_and_copy_byte`: Increments a frame word, decrements a global counter, and returns when it is negative, otherwise increments another global pointer, copies a byte from the caller's frame to that pointer, and jumps to a common routine.
- `push_byte_and_jump`: Loads a byte from offset -1 of the structure in a5 into a register, masks it to a byte, pushes a word from offset -484 of the structure in a4 and the masked value onto the stack, calls the routine at the fixed address, removes the three arguments from the stack, and then jumps to the fixed address without returning.
- `decrement_counter_copy_byte`: Decrements a 16-bit counter at offset 490 of the structure in a4 and, if it is not negative, copies the byte at offset -1 of the structure in a5 to the address in a1, increments that address by one, masks the byte to 8 bits, and calls the loader routine with the resulting byte.
- `push_byte_and_call_loader`: Pops an 8-bit value from the stack, masks it to a byte, pushes a 16-bit value from the caller's frame, calls a routine that loads a word from that caller and a word from a structure, then jumps to a fixed address.
- `push_value_and_call`: Pushes a word from offset -484 of the structure in a4 onto the stack, loads the value -1 into a register and pushes it, calls the routine at the fixed address, removes the two arguments from the stack, loads a word from offset -4 of the structure in a5 into a register, restores the stack frame, and returns.
- `extract_flag_from_status_word`: Reads a word from a pointed structure, derives a flag from its high bit and stores it at offset -1 of the caller frame, then returns when two lower bits are clear, otherwise clears a word in the structure and jumps to a routine.
- `process_structure_flags_2`: Checks bits 7 and 6 of a status byte at offset 19 of the structure in a2 and returns if either is clear, then calls a routine via a fixed address.
- `call_and_set_flag`: Pushes a register onto the stack and calls a routine through a fixed address, then removes the arguments from the stack.
- `read_byte_and_adjust_pointer`: Tests a byte at offset -1 of the structure in a5 and returns when it is zero or negative, otherwise it advances a pointer at offset 4 and reads a byte through that pointer.
- `call_with_a2_push`: Pushes register a2 onto the stack, calls a routine at a relative address, removes the pushed arguments from the stack, restores a2 and d0, then jumps into the middle of another routine.
- `set_flag_and_call`: Sets bit 4 of the byte at offset 19 of the structure in a2, sets d0 to the decimal value 255, and jumps into the middle of another routine.
- `load_frame_word_and_call`: Loads a word from offset 4 of its own stack frame into d0 and jumps into the middle of another routine.
- `call_and_set_flags`: Tests bit 1 of a byte at offset 19 of the structure in a2, sets bit 0, calls a routine through a fixed address, and stores the result at offset -6 of the structure in a5.
- `store_pointer_and_mask_offset`: Stores d0 at offset 8 of the structure in a2, copies the pointer at offset 12 to offset 4, and masks the word at offset 18 with 50.
- `store_zero_and_call`: Clears d0, stores that zero at offset 8 of the structure in a2, sets d0 to the decimal value 255, and jumps into the middle of another routine.
- `decrement_counter_then_load_byte`: Decrements the word at offset 8 of the structure in a2 and returns immediately when it becomes negative.
- `call_and_restore`: Pushes the register a2 onto the stack, calls a routine at a relative address, removes the pushed arguments from the stack, restores a2 and d2, restores the stack frame, and returns.
- `load_and_mask_word`: Sets up the stack frame and loads a word from the caller and a word from a structure into registers.
- `store_sign_result`: Copies a register into another and tests its sign bit, storing a resulting byte at offset negative 2 of a structure.
- `set_bit_then_check_byte`: Sets bit 1 of the byte at offset 16 of the structure in a2, then checks the byte at offset -2 of the structure in a5 and returns immediately when that byte is zero.
- `decrement_index_then_store_byte`: Copies the word at offset 16 of the structure in a2 into offset 10, decrements that stored word, and returns immediately when it becomes negative.
- `push_byte_call_routine`: Loads a word from a global variable at offset 8 of a5, masks its low byte, pushes it on the stack, then calls the routine whose address is stored at offset 8 of a5 and pops the pushed value off the stack.
- `test_flag_and_compare_max`: Tests bit 2 of a byte at offset 19 in a2 and returns when it is clear, then compares a word at offset 8 in a5 against 65535 and returns when they differ.
- `store_byte_and_call_when_ten`: Stores a byte from d1 at offset -1 of the structure in a5 and returns when the byte at offset -2 is zero or when d1 is not ten, otherwise it pushes two arguments and calls a routine through a fixed address.
- `push_args_and_call_routine_2`: Loads the constant 1 into a register, pushes it along with a word from offset 1 in a5 and a word from offset 20 in a2, calls a routine through a fixed address, restores the stack, stores the result at offset 10, and returns the constant 255 in a register.
- `set_flag_and_store_return`: Sets a flag bit at offset 19 in a2, returns when a byte at offset 2 in a5 is zero, compares a word at offset 8 against 65535 and returns when equal, increments a word at offset 10, and otherwise stores a carriage return byte and calls a routine with a pushed pointer.
- `set_flag_when_word_max`: Clears a word at offset negative 6 of the structure in a5, copies it into d1, and sets bit 5 of the byte at offset 13 of the structure in a2 when d1 equals negative one.
- `test_bit_negate_store`: Compares a word at offset negative 10 of the structure in a5 with d1 and branches away when equal, sets bit 4 of the byte at offset 19 of the structure in a2, tests the byte at offset negative 2 of the structure in a5 and branches away when zero, copies a word from offset 16 of the structure in a2 into d0 and d1, negates d1, stores it at offset negative 10 of the structure in a5, and branches away.
- `test_bit_and_clear_word`: Tests bit 2 of the byte at offset 19 of the structure in a2 and returns without acting when it is clear, otherwise it writes a zero to offset 10 and branches away.
- `copy_word_and_check_counter`: Copies the word at offset 16 of the structure in a2 to offset 10, saves the pointer at offset 12, then reads a word from a global variable at offset 8 of a5 and returns when it equals -1.
- `mask_byte_and_call`: Reads a word from a global variable at offset 8 of a5, masks its low byte, pushes a saved a2 and the masked value onto the stack, calls a routine, then restores the value into d1.
- `check_word_and_maybe_return`: Reads a word from a global variable at offset -8 of a5 and returns without doing anything when that value equals the maximum word value, otherwise it loads zero into d0 and falls through.
- `restore_registers_and_return_7`: Copies the value of register d1 into register d0, restores registers d2 and a2 from the stack, restores the stack pointer, and returns.
- `test_structure_and_branch`: Loads a structure pointer from offset 8 and tests two of its bytes, returning immediately when either the byte at offset 16 is zero or bit 3 of the byte at offset 13 is set.
- `push_word_and_call_routine`: Pushes a word from offset 840 of a structure onto the stack and calls a routine, then stores the result into two locations and branches away when nonzero, otherwise storing 12 and returning.
- `clear_and_copy_structure_words`: Copies a word from a structure at offset minus eight hundred and seventy-two into a structure at offset sixteen, clears bits two through five of the word at offset eighteen, zeroes the words at offsets ten and eight, restores the stack pointer, and returns.
- `dispatch_indexed_handler`: Sets up a stack frame and initializes several local words and bytes to zero or fixed values.
- `write_bytes_then_branch`: Performs three unrelated byte operations: a no-op write to a stack slot, a write to a hardware register at offset 96 of a4, and an increment of the byte pointed to by a6, then branches into another routine.
- `set_byte_and_branch`: Writes the byte value one into the byte at offset -2 of the structure in a5, then branches into the middle of another routine.
- `increment_long_branch`: Increments a longword at offset 8 of a5 by one and then branches into the middle of another routine.
- `load_and_compare_byte_sequence`: Saves a5, loads a byte from a pointer at offset 12, increments that pointer, stores the original byte at offset -1, and returns when the byte is zero; otherwise it compares the byte with 37 and, if equal, increments the pointer and compares the next byte, returning when that byte differs.
- `negate_registers_when_positive`: Tests the sign of the longword in d0 and negates it when positive, then does the same for d1, calls a routine, negates d1 again, and returns.
- `compute_divide_result`: Saves register d2 on the stack, swaps the halves of d1, and branches into another routine when d1 is nonzero, otherwise it performs a series of divide operations using a helper routine and restores d2 before returning.
- `multiply_and_combine_words`: Saves two pointers in registers, multiplies the word operands together, combines the partial products, adds them, and restores the saved pointers before returning.
- `copy_byte_block`: Loads two pointers and a count from the stack, returns immediately when the count is zero or negative, and otherwise copies that many bytes from the first location to the second while decrementing the counter.
- `decrement_and_loop`: Decrements register d0 and loops back to the same instruction while it is nonzero, then restores d0 from the stack and returns.
- `copy_byte_when_nonzero`: Compares a byte at a pointer with zero, and when it is nonzero decrements the pointer and copies a byte from another pointer into it, otherwise reads a pointer from the stack and returns.
- `call_string_copy`: Loads two pointers and a count from the stack and branches into the middle of another routine to copy a byte-counted string.
- `check_flag_and_branch`: Sets up a stack frame, compares a register against a longword at offset 420 of the structure in a4, then tests a flag byte and returns when a condition is not met.
- `store_constant_when_zero`: Reads a longword from the stack and returns early when it is nonzero, otherwise it stores a constant at a global address, zeroes a register, and returns.
- `request_set_signal`: Allocates stack space, loads the library base into a6, and calls the exec library to request signal 12288.
- `request_set_signal_2`: Checks a flag and branches away when it is clear, then calls a routine through a library base and returns its result.
- `request_set_signal_alt`: Checks a flag and branches away when it is clear, then calls a different routine through a library base and returns its result.
- `dispatch_indexed_handler_2`: Checks a flag and branches away when it is clear, then calls a routine through a library base and dispatches based on a small index value.
- `restore_registers_and_return_9_2`: Checks a flag and branches away when it is clear, then calls a routine through a library base and returns a fixed negative value on failure.
- `restore_registers_and_return_9`: Restores the caller's registers from the stack, removes the frame with unlk a5, and returns.
- `request_set_signal_36`: Checks a flag and branches away when it is clear, then calls a routine through a library base and returns zero.
- `load_value_and_return_2`: Loads a value from a structure, calls a sequence of library routines, and returns a fixed negative value when a later check fails.
- `load_value_and_return`: Loads a longword from offset minus 4 of a5 into d0, removes the frame with unlk a5, and returns the value.
- `handle_library_request`: Tests a longword at offset -32720 of the structure in a4 and, if it is zero, jumps to the end without doing anything.
- `call_library_store_result_2`: Loads a longword from offset 8 of the structure in a5, sets d2 to the decimal constant 1006, then calls the library at offset -30 from a global base in a6 and stores the returned value at offset -4 of its own structure.
- `load_longword_from_pointer`: Loads a longword from the address stored at offset -4 of the structure pointed to by a5 into d0, then returns.
- `open_library_when_uninitialized`: Sets up a stack frame, checks a longword at offset 19088 of a4 and returns early when it is nonzero, otherwise loads a library base pointer and calls the library entry point.
- `clear_d0_and_restore_registers`: Clears d0 with a moveq, restores the caller's registers from the stack, removes the frame with unlk a5, and returns.
- `call_library_vector_198_4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_330_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `initialize_linked_structure`: Copies the pointer from the first argument into the structure it points to, increments that stored pointer by four, clears the following long word, and stores the original pointer at offset eight of the structure before returning.
- `skip_whitespace_byte`: Saves registers and loads a pointer from the frame, then returns when a word at offset 18644 of the structure in a4 is at least 32, otherwise it reads a byte and continues when it is not a space, tab or carriage return.
- `jump_to_format_handler`: Contains only a jump to a routine at absolute address zero and performs no instructions of its own.
- `jump_to_flag_check`: Jumps directly to a routine that compares a register against a longword at offset 420 of the structure in a4 and returns when a flag byte test succeeds.
- `route_by_selector_value`: Reads a selector word from offset eight of the parameter block in a5 and branches to one of three destinations based on whether it equals two, one, or some other value.
- `dispatch_window_message`: Reads a word into d0 and branches to different handlers when it equals two or one, then branches away when it is nonzero, otherwise loads two pointers from global variables, calls the routine at the first pointer, clears that pointer and calls the library function at offset 450 through the second pointer.
- `load_param_and_call`: Loads a 16-bit value from the incoming parameter block into d0 and a pointer into a0, then calls a library routine at offset -144 of the a6 structure.
- `refresh_menu_state`: Clears bit 9 of the 16-bit value at offset 12 of the structure in a0, then calls the routine at offset -54 of the structure in a6 and the routine at offset -264 of the same structure with a3 as argument.
- `call_and_clear_byte`: Calls a library routine at offset -144 of the a6 structure, then sets bit 4 of a byte at offset 13 of a structure and clears a byte pointed to from offset 18 of that same structure.
- `call_libraries_and_push`: Calls a library routine at offset -342 and another at offset -354, then computes several adjusted values from a structure and pushes them on the stack before calling another routine.
- `push_args_and_call_result`: Loads a long from offset 12 into d1, compares it against a pointer from a global variable and branches away when they differ, then pushes several values including two negative-eight constants and the constant fifteen onto the stack, calls the routine at offset 10_0F1C, adjusts the stack pointer and jumps to the tail.
- `setup_and_load_pointer`: Initializes a local frame, copies two words from a pointed-to structure into registers, stores two zero bytes and two zero words into the local frame, then loads a pointer from a global table when a flag is set.
- `check_structure_bounds`: Reads four byte fields from a structure and subtracts them from paired offsets, then stores a flag when the computed values fall outside a range.
- `compute_centered_coordinates_4`: Scales a word from a5 by dividing by 100, calls two routines with pushed arguments, then computes four coordinate values from byte fields of a3 and stores them before dividing and branching.
- `compute_centered_coordinates_3`: Branches away when the word in d3 is not below or equal to zero.
- `compute_centered_division_offset`: Reads the byte at offset 54 and the word at offset 8 of the structure in a3, subtracts the byte from the word, then repeats with the byte at offset 56, and finally arithmetic-shift-rights the result by one.
- `compute_divisions_and_store`: Tests the value in d0 and returns when it is negative.
- `compute_and_store_divisions`: Computes two signed division results from the words at offset -30 of the structure in a5 and stores them at offsets -30 and -32, then selects between subtracting or adding a computed value to a global word at offset -24896 of the structure in a4 based on the sign of an intermediate result, storing the outcome and a sign flag at offsets -34 and -36.
- `compute_and_call_indirectly`: Adds a value from a global variable to another global word, stores two computed results into a structure in a5, and returns when they match, otherwise it pushes arguments and calls a routine before jumping away.
- `push_args_and_call_2`: Pushes a 16-bit value and a pointer from the incoming parameter block on the stack, then calls another routine before jumping away.
- `compute_divided_bounds`: Reads two negative words from the parameter block, divides each by eight, and stores the incremented quotients back.
- `update_and_store_sum`: Reads a word from a global variable at offset 24898, adds it to d2, and stores a flag and the sum into the structure in a5 at offsets 32 and 30.
- `push_and_call_routine_2`: Pushes a word at offset 12 and a long at offset 8 of the structure in a5, then calls a routine.
- `compute_centered_coordinates_2`: Computes centered coordinates from four byte fields of the structure in a3 by subtracting paired offsets and arithmetic-shift-righting the results, then checks a flag at offset -38 of the structure in a5 and branches away when it is zero.
- `compute_centered_coordinates`: Computes centered coordinates from four structure fields by subtracting paired offsets and arithmetic-shift-righting the results, then stores the derived values into local variables.
- `increment_and_compare_flags`: Reads the byte at offset 54 of the structure in a3, increments it, and adds it to the value in d5.
- `increment_and_dispatch_flags`: Reads a byte at offset 55 of the structure in a3, increments it, and compares a derived value against a threshold.
- `loop_counters_and_set_flag`: Computes a difference and stores it at offset -8, then decrements a counter and branches away when it falls below a threshold, updates a value at offset -4, and continues comparing counters, setting a flag byte at offset -11 when a second condition is met.
- `loop_and_call_routine`: Runs a nested loop that decrements and compares counters stored in a structure, sets a flag byte, then pushes several values on the stack and calls another routine.
- `set_flag_and_dispatch`: Sets bit 0 of the byte at offset 25 of the structure in a3, loads a pointer from offset 56 of that structure, and calls a routine using the library base at offset 4 of a3.
- `copy_and_dispatch_message`: Copies the words at offsets -8 and -6 of a5 into offsets 4 and 6 of the structure pointed to by the longword at offset 8 of a5, then loads that pointer into a0, sets a1 to a3, and calls a library routine.
- `increment_counter_and_return`: Increments a 16-bit counter at offset 17768 of a structure, copies a3 into d0, then restores the caller's registers and returns.
- `dispatch_and_decrement_counter`: Loads a pointer from a structure in a1, calls a routine at offset 19088 of the structure in a4, then decrements a counter at offset 17768 when the pointer matches one at offset 19112, otherwise it returns.
- `process_dispatched_message`: Loads a pointer from offset 19104 of the structure in a4 and decrements a 16-bit counter at offset 17768, clearing it when it underflows.
- `compare_and_dispatch_message`: Copies a pointer from a structure and calls a library function, then returns to another routine when the result is not four.
- `compare_and_update_flags`: Compares a pointer in a0 with one at offset 19108 of the global table, and if they differ it calls a routine and then a library function with a value of one.
- `resolve_pointer_slots`: Reads a pointer from the parameter block and stores the long it references into a temporary slot, returning early when that slot is zero.
- `store_pointer_value_d0`: Reads a pointer from offset -12 of the parameter block, stores the value it references into a temporary slot, branches away when that slot is zero, then loads a word, writes it through the pointer, stores another value and jumps to the tail.
- `reload_pointer_value`: Reads a pointer from the incoming parameter block, stores the value at that pointer back into the same parameter slot, then jumps away.
- `store_pointer_value_d0_alt`: Reads a pointer from offset 8 of the parameter block, stores the value it references into a temporary slot, branches away when that slot is zero, then loads a word, writes it through the pointer, stores another value and jumps to the tail.
- `store_pointer_value`: Reads a pointer from the incoming parameter block and stores the value it points into a temporary slot, then returns immediately if that value is zero.
- `restore_frame_and_return`: Restores the frame pointer and returns.
- `iterate_table_entries`: Returns early when a global pointer is zero, otherwise it clears a counter and branches away when it is at least three.
- `jump_to_compute_indices`: Transfers control to the routine located at offset 7982 without returning.
- `jump_to_date_format`: Immediately transfers control to the routine at the absolute address 2544 without executing any instructions of its own.
- `jump_to_restore_stack`: Performs a jump to the routine that restores d2 from the stack, compares a global word at offset -24898 with a word at offset 8 of the structure in a5, and adds a global constant, then returns.
- `jump_library_stub`: Jumps unconditionally to a fixed library entry point and returns its result to the caller.
- `call_library_offset_270`: Loads a library base from a global variable and calls the library function at offset 270 of that base, returning its result, then returns.
- `jump_to_system_vector`: Jumps to the system vector at 16512 and never returns to this routine.
- `jump_to_target`: Jumps unconditionally to the target address.
- `jump_to_absolute_routine`: Unconditionally jumps to the routine at the absolute address 15976 and never returns.
- `format_negative_number`: Initializes a local counter to zero, and when the incoming word is negative it writes a minus sign, negates the value, and calls the digit-writing routine before clearing the next character and returning the buffer pointer.
- `convert_and_store_digit`: Divides a stack word by ten, and if the quotient is nonzero it calls itself recursively before writing the next digit as a character into a buffer.
- `call_and_check_vectors`: Sets up a link frame and saves a6, tests a byte at offset negative eighteen one three two of a4 and branches away when zero, otherwise calls a routine and tests a word at offset negative eighteen one three four of a4.
- `pop_and_call_vector`: Pops a long word from the structure at offset minus 4 of a5 onto the stack, calls another routine, restores the stack pointer, and jumps to a shared routine.
- `release_table_entry`: Tests a counter at offset negative six of the frame, indexes into a table of four-byte entries, and calls the exec library when the entry is non-zero before clearing it and incrementing the counter.
- `close_device_when_zero`: Tests a long value in the structure at offset 46da of a4 and, when it is zero, loads a pointer from offset 6162 of a4 and calls the close device library routine.
- `check_flag_and_dispatch`: Reads a flag word from a global variable, masks it with a bit selected by another global word, and returns without acting when the bit is clear.
- `push_args_and_call_3`: Pushes the registers d2, a 40, a 214, a 1, and a 0 onto the stack, calls the routine at the given address, removes the eight pushed bytes, restores d2, and returns.
- `allocate_and_copy_blocks`: Reads a word at offset eight of a5, multiplies it by eight to index a table, and when the entry is non-zero calls a routine and copies sixteen words from a table at offset positive twenty four nine three zero of a4 into a structure.
- `allocate_and_copy_block`: Reads a byte from offset 18128 of a4 and, when it is nonzero, calls the allocation routine and copies sixteen words from a table at offset 24930 of a4 to the allocated block.
- `allocate_and_check_pointer`: Sets up a link frame, pushes a6 onto the stack, loads a pointer from a global variable at offset 46da of a4, calls an exec library function, stores the returned pointer in the frame, and when that pointer is null it increments a global counter and jumps to a shared routine.
- `free_mem_and_decrement`: Releases the pointer passed in the structure at offset 8 of a5 back to the exec library, decrements a global counter at offset 18134 of a4, restores the caller's registers, unwinds the frame, and returns.
- `unconditional_jmp_routine`: Performs a single unconditional jump to another routine and returns control to the caller.
- `jump_to_c00`: Performs an unconditional jump to the routine at 3072 and does not return.
- `jump_to_absolute_address`: Unconditionally jumps to the code at the given absolute address and never executes any instruction of its own.
- `empty_routine`: This routine is empty: it contains only a jump to the start of the current routine, which immediately returns control to the caller without modifying any registers, memory, or hardware state.
- `dispatch_with_status_check`: Sets up a frame, pushes a constant address, calls a routine, discards the returned address, stores one into a global word, then checks a register and either calls another routine or returns.
- `return_constant_14`: Returns the constant an address in d0 to the caller without any other effect.
- `call_routines_clear_stack`: Saves register a6, calls a routine, then clears a stack word and calls another routine, adding two bytes to the stack pointer.
- `load_message_then_dispatch`: Sets up a stack frame, saves a6, calls a routine, clears a word in the frame, loads two fields into a1 and a0, calls an exec library function, stores the result, and branches to another routine when it is nonzero, otherwise it calls a second exec library function.
- `init_locals_check_flag`: Sets up a stack frame, saves registers, copies a global word into a local slot, initializes several locals to zero, and jumps to a branch target when a flag at offset -18498 of a4 is zero.
- `check_bounds_jump`: Compares a word at offset 8 of the structure in a5 against the bounds 23 and 615, then compares a word at offset 10 against 20 and 180, and jumps to a common target when any check fails.
- `push_arg_call_store_flag`: Pushes a word from offset 10 of the structure in a5 and a copy of d2, calls a routine, stores the returned word at offset -4 of that structure, then checks a flag at offset -18516 of a4 and jumps to a branch target when it is zero.
- `test_and_call_routine_2`: Tests a word at offset -36 of a5 and returns immediately when it is zero.
- `copy_and_call_routine`: Copies several fields from a structure pointed to by offset -14 of a0 into frame slots, then calls a routine at offset -378 and compares its result against four, branching away when unequal.
- `compare_and_increment_counter`: Loads a longword from offset -18 of a5 and compares it against sixteen million, branching away when unequal.
- `decrement_and_compare_register`: Decrements a register and branches away when it is nonzero, otherwise compares it against a constant and branches away when unequal, then stores zero into a frame word before falling through.
- `check_and_call_double_click`: Compares a register against a constant and branches away when unequal, otherwise stores two into a frame word, clears another, loads four longwords, calls a library routine, and branches when the result is zero.
- `clear_and_call_routine`: Clears a word at offset -24 of a5, skips a nop, compares a word at offset -22 of a5 against two and branches away when unequal, then pushes a local word and calls a routine before clearing d0 and branching away.
- `compare_frame_call_routines`: Compares a frame word against one and branches away when unequal, otherwise stores a global word locally and calls two routines, saving the result to the frame.
- `push_arg_call_and_return_2`: Stores a word from a structure into a global variable, pushes another word onto the stack, and calls a routine.
- `push_word_then_jump`: Pushes a word from offset -4 of the frame in a5, calls a routine, removes the pushed value, sets d0 to zero, and jumps into the middle of another routine.
- `dispatch_offset_and_call`: Pushes a word from offset -4 of the structure in a5 and calls a routine, then restores the stack.
- `dispatch_by_offset_value`: Loads a word from offset -24624 of the structure in a4 and branches to different routines depending on whether it equals four, three, or one.
- `compare_offset_and_call_2`: Calls a routine via a branch, loads a zero into a data register, and jumps to another routine.
- `indirect_jump_return_2`: Pushes a word from a structure onto the stack, calls a routine, restores the stack, loads a constant, and jumps to another routine.
- `push_arg_call_and_return`: Pushes a word from offset -4 of the structure in a5 onto the stack, calls a routine, removes the pushed value, restores the local frame, and returns.
- `compare_word_then_branch`: Saves registers d7 and a2, reads a word at offset 8 of a5, compares it with 80, and when the value is at least 80 it clears d0 and branches to a routine.
- `mask_and_branch_on_zero`: Reads a word at offset 10 in a5, masks it with 192, and returns zero when the result is zero, otherwise it jumps to a routine through a fixed address.
- `validate_and_index_table`: Validates d1 by jumping away when it is below zero, above ten, one or nine, otherwise it scales d1 by two and jumps to a table entry indexed by that value.
- `index_table_and_compare`: Sets bits in a status byte based on flag bits, then scales a counter by two to index a table and compares a word from the structure in a5 against it, reading a byte from a second table when they match.
- `increment_and_branch`: Increments the word in d7 by one and then branches forward into the middle of another routine, so it performs no work of its own.
- `call_with_flag_and_return`: Loads 1 into d0, pushes it, and calls a routine through a fixed address, then removes the pushed argument and restores d0.
- `divide_value_then_store_results`: Sets up a stack frame and saves registers, reads a word from a global into d2, stores it locally, divides it by 100 to get a quotient and remainder, stores several values into local offsets, tests a word at offset 8 of the frame, and branches to a routine when it is zero, otherwise it clears d0 and jumps to a shared path.
- `call_library_increment_index`: Loads a constant into register d0, passes two globals to the library function at offset -342, and branches away when a word at offset 8 of a5 is zero.
- `init_structure_then_call`: Stores a negative word and sets a bit in a structure, then calls a routine with arguments from another structure, computes two scaled sums from that structure, calls a second routine, and returns zero unless the first result is nonzero.
- `call_handler_after_test`: Pushes a value and a pointer, then calls a routine and restores the stack.
- `process_message_and_store`: Clears d0, sets bit 0 of byte 25 of the structure pointed to by a global, stores d0 at offset -34 of a5, and returns early when offset -40 of a5 is zero.
- `save_struct_and_call`: Copies eight fields from the structure pointed to by a0 into stack slots, then calls a routine via a6 and compares the saved first value against four.
- `check_status_and_call_routines`: Checks several status words and compares a counter against a fixed value, branching away when any condition fails, otherwise pushes arguments and calls several routines before storing a flag and returning.
- `store_zero_and_branch`: Loads zero into register d0, stores it as a word at offset -42 of the structure in a5, then branches to a nearby label.
- `store_zeros_check_value`: Stores two zero words at offsets -42 and -32 of the structure in a5, then reads a longword from offset -50 and branches away unless it equals eight.
- `check_value_232_flag`: Compares the value in d3 against the constant 232 and jumps away when they differ, otherwise sets bit 0 of a byte at offset 15 in the structure in a5, stores 1 at offset 34, and jumps to another routine.
- `store_d0_and_check_bits`: Compares d3 against the constant 232 and jumps away when they differ.
- `compare_1024_call_routine`: Returns when the long in d2 differs from 1024, otherwise pushes two words from the structure in a5 and calls a routine, storing its result at offset 32 and returning when that result is zero.
- `call_routines_with_arguments`: Pushes a global pointer and the value 24000 on the stack, calls a routine, and discards the arguments.
- `call_and_divide_results`: Tests a flag in the structure in a5 and jumps away when zero, otherwise calls a routine and divides its result by 100 and by 64, storing both quotients into the local frame.
- `compute_signed_differences_2`: Subtracts d0 from d2 and d1 from d4, then negates the second difference when it is negative before returning.
- `compute_signed_differences`: Computes the difference between d4 and d1, negates it when positive, then computes the difference between d2 and d0 and negates it when negative before jumping away.
- `subtract_and_jump`: Subtracts the value in d0 from the value in d2, copies the result into d3, and jumps to a routine.
- `compute_absolute_difference`: Copies the value in d4 into d0, subtracts the value in d1 from it, and branches away when the result is positive.
- `compute_and_conditionally_call`: Computes the difference between the value at offset -4 of the structure in a5 and the value in d1, stores that difference at offset -30 of the same structure, and compares it with the word at offset 23188 of the structure in a4.
- `test_word_and_call`: Tests the word at offset -42 of the structure in a5 and returns immediately when it is zero.
- `adjust_and_call_twice`: Adjusts a global word by a passed value and calls a routine when it is within bounds, then adjusts another global word and calls the same routine again before jumping away.
- `adjust_and_call_routine`: Reads a global word, adjusts it by a passed value, and returns when below a second value, otherwise calls a routine with two global words and loads a zero result when a structure word is clear.
- `average_and_call_routines`: Calls two routines using pointers from a4, then averages two pairs of words from a5 into four local variables and calls two more routines with the results.
- `call_twice_and_subtract_words`: Clears a word at offset -42 of the structure in a5, then calls a routine with two pushed arguments and removes them from the stack.
- `store_value_and_call`: Loads a value into register d0, stores it at offset -26 of the structure in a5, pushes two words onto the stack, calls a routine, pops the stack, and branches into another routine.
- `process_message_and_scale`: Pushes a pointer and calls a routine, then restores the stack and clears a bit in a structure pointed to by a global.
- `update_titles_and_menus_2`: Clears a word at offset -2 of the frame in a5, then compares a global word against 50 and branches away when it is greater than or equal to 50.
- `index_jump_table_lookup`: Saves registers d2 through d4 and a2, reads a word at offset 8 of a5, and when the value is below 50 it scales it and jumps to a routine through a jump table.
- `jump_to_routine_13`: Jumps to a routine at offset 7466 and does not contain any instructions of its own.
- `set_zero_and_delegate`: Loads a zero into d0 and jumps to a routine, so it only sets a register before delegating to the caller.
- `compare_offset_and_call`: Compares a global word at offset -6030 against 8 and returns when they are equal.
- `compute_indices_then_call_library`: Reads two bytes from a table, computes indices, stores them at offsets of the structure in a5, then calls a library routine and enters a loop that branches back to an earlier instruction.
- `prepare_args_call_library`: Reads a word at offset -10 of the structure in a5, sign-extends it to a longword, and calls the library at offset -342 from a global base in a6, then pushes four extended word values onto the stack before calling another routine.
- `compute_and_store_indices`: Reads a word from a global variable and returns early when it is two or greater, otherwise reads a byte at offset 54 of a structure, scales a second index, and stores both computed values at offsets -8 and -4 of the structure in a5.
- `compute_index_and_store`: Reads a byte at offset 54 of the structure in a0, combines it with a scaled value to compute a word stored at offset -8, then computes a second word at offset -4 and a third at offset -2 before falling into another routine.
- `compute_index_and_call_libraries`: Computes an index from a byte at offset 55 of a0, stores two word values at offsets minus 6 and minus 2 of a5, then calls three different library vectors to pass pairs of those stored values along with a pointer from offset 50 of a0, and finally restores the caller's registers and returns.
- `indirect_jump_return`: Performs an indirect jump to a fixed address and returns immediately.
- `jump_to_routine_10`: Unconditionally jumps to another routine and performs no work of its own.
- `jump_to_lock_stub`: Jumps to the lock routine of the dos library, which loads its arguments and the library base, performs the lock, and returns the result to the caller.
- `jump_to_routine`: Contains only a jump to another routine and performs no instructions of its own.
- `jump_to_window_handler`: Is a single jump instruction that transfers control to a routine loaded from a global vector table and does not return.
- `jump_to_global_routine_2`: Jumps to a global routine and never returns from this routine.
- `jump_to_store_index`: It is a single instruction that jumps to the routine that stores an index and a value, transferring control without modifying any register.
- `jump_to_routine_14`: Jumps to a routine at offset 14734 and does not contain any instructions of its own.
- `load_and_store_value_32`: It is a single instruction that loads the constant 32 into the low word of d0 and jumps to the routine that stores that value in a table slot, transferring control without returning.
- `jump_only_routine`: Contains only a jump instruction and does nothing else.
- `dispatch_table_routine`: Jumps to the routine at offset 2 of the table in a4, passing the byte at offset 18128 of the structure in a4 as a condition, which triggers the allocation routine when that byte is nonzero.
- `call_validate_dispatch`: Performs an unconditional jump to the routine named validate_and_dispatch_index, which sets up a stack frame, stores the constant one at offset negative two of the structure in a5, and loads that value into d1.
- `load_eight_and_jump_2`: It is a single instruction that loads the decimal value 8 into register d0 and jumps to the routine at the absolute address 36, transferring control without modifying any register.
- `jump_to_target_3`: Contains a single unconditional jump to another routine and does nothing else.
- `jump_always_elsewhere`: Unconditionally jumps to another routine and never returns to this location.
- `jump_to_date_formatter`: Unconditionally jumps to another routine that formats a date built from three bytes of a global structure, so this location only serves as a jump entry.
- `jump_to_zero_2`: Executes a single unconditional jump to the absolute address 0 and does not return.
- `call_divide_and_store_quotients`: Jumps to the routine that divides a 16-bit value by 100 twice and stores both quotients, then returns.
- `jump_to_routine_18`: It is empty, containing only a single unconditional jump to a fixed routine address.
- `jump_to_find_resident_stub`: Jumps to the find resident routine of the exec library, which loads its arguments and the library base, performs the search, and returns the result to the caller.
- `jump_to_library_vector_stub`: Jumps to the library function at offset minus 306 of a base loaded from a global variable, returning its result to the caller.
- `store_index_and_jump_7`: Loads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then jumps to another routine.
- `indirect_jump_4080`: Performs an unconditional indirect jump to the address stored at the global variable at offset 1640 and never returns.
- `load_zero_and_jump_2`: It is a single instruction that loads the decimal value 0 into register d0 and jumps to the routine at the absolute address 36, transferring control without modifying any register.
- `jump_to_address_only`: Consists of a single unconditional jump to a fixed address, so it only transfers control without modifying any data.
- `jump_store_index`: Jumps to a routine that loads a pointer from a global variable, derives an index from the low word of d0, stores d0 at the first matching slot of a table, and sets a flag, then returns.
- `empty_routine_4`: It is an empty routine consisting of a single jump instruction that does not return.
- `store_long_and_jump`: Clears a word at offset 25724 of a global variable, stores a long value passed in the frame at offset 8 into offset 25720, pushes two arguments, then jumps to another routine.
- `load_twentyfour_and_jump_2`: It is a single instruction that loads the decimal value 24 into register d0 and jumps to the routine at the absolute address 36, transferring control without modifying any register.
- `load_and_store_value_80`: It is a single instruction that loads the constant 80 into the low word of d0 and jumps to the routine that stores that value in a table slot, transferring control without returning.
- `jump_to_vector_140`: Jumps to the code at absolute address 320 and does not return.
- `store_index_in_table_5_2`: It is a single instruction that derives an index from the low word of d0 and jumps to the routine that stores d0 into the first matching slot of a table, transferring control without modifying any register.
- `jump_to_write_stub`: Jumps to the write routine of the dos library, which loads its arguments and the library base, performs the write, and returns the result to the caller.
- `load_constant_and_store`: Loads the constant 56 into the low word of register d0 and jumps to a routine that stores that value in a table slot, then returns.
- `jump_via_indirect`: Performs a single unconditional jump to the target address and returns control to the caller.
- `jump_to_zero_check`: Jumps directly to a routine that returns early when a longword read from the stack is nonzero, otherwise stores a constant at a global address and zeroes a register.
- `load_hundred_four_and_store`: Loads the constant 104 into the low word of register d0 and jumps to a routine that stores that value in a table slot, then returns.
- `jump_to_routine_16`: Branches unconditionally into another routine and does not return.
- `indirect_jump_from_global`: Performs an unconditional indirect jump to the address stored in a global variable and never returns.
- `jump_to_handler_2`: Consists of a single unconditional jump to the routine at the absolute address 1042, transferring control without modifying any register or memory.
- `load_constant_forty_and_store`: Loads the constant 40 into the low word of register d0 and jumps to a routine that stores that value in a table slot, then returns.
- `load_sixteen_and_jump_2`: Loads the decimal value 16 into register d0 and jumps to a routine at an absolute address without modifying any registers.
- `jmp_string_copy`: Loads two pointers and a count from the stack, then branches into the middle of another routine to copy a byte-counted string.
- `load_zero_and_jump`: Loads the decimal value 0 into register d0 and then jumps to the routine at the absolute address 36, transferring control without modifying any other register.
- `load_eight_and_jump`: Loads the decimal value 8 into register d0 and then jumps to the routine at the absolute address 36, transferring control without modifying any other register.
- `load_sixteen_and_jump`: Loads the decimal value 16 into register d0 and then jumps to the routine at the absolute address 36, transferring control without modifying any other register.
- `load_twentyfour_and_jump`: Loads the decimal value 24 into register d0 and then jumps to the routine at the absolute address 36, transferring control without modifying any other register.
- `store_value_32`: Loads the constant 32 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_40`: Loads the constant 40 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_48`: Loads the constant 48 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_56`: Loads the constant 56 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_64`: Loads the constant 64 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_72`: Loads the constant 72 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_80_2`: Loads the constant 80 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_88`: Loads the constant 88 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_96`: Loads the constant 96 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_value_104`: Loads the constant 104 into the low word of d0 and jumps to the routine that stores that value in a table slot, then returns.
- `store_index_in_table_2`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, then returns.
- `store_d0_in_table_slot`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, then jumps to the routine at 36.
- `store_index_and_jump`: Loads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then jumps to the address in d0.
- `store_index_and_jump_2`: Loads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then sets d0 to 136 and jumps to the handler.
- `store_index_and_jump_3`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table via another routine, then jumps to the address at 36.
- `store_index_and_jump_4`: Loads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then jumps to the address in the jump table.
- `store_index_in_table_3`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, then jumps to the routine at address 36.
- `store_index_and_jump_5`: Loads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then sets d0 to 168 and jumps to the handler at 36.
- `store_index_in_table_4`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table via another routine, then jumps to address 36.
- `store_index_in_table_5`: Loads a pointer from a global variable, derives an index from the low word of d0, and stores d0 into the first matching slot of a table via that pointer.
- `store_index_and_jump_6`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, then jumps to the routine referenced by the text at the given address.
- `store_index_in_table_6`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, then jumps to the handler routine.
- `store_d0_in_indexed_slot`: Writes the 16-bit value in d0 into the memory slot selected by an index derived from the low word of d0, then jumps to a fixed address.
- `loop_call_library`: Sets up a stack frame and saves a6, then repeatedly calls a routine and a library function, decrementing a counter at offset 8 of the frame and looping back while the counter is not zero, otherwise it returns.
- `scale_and_clamp_values`: Scales a 16-bit value from a global variable to a 0-100 range, subtracts a stored value at offset 20326, then does the same for a second value using offset 20320, pushes both results onto the stack, and calls a routine to clamp them.
- `init_local_and_call`: Initializes a local structure at offset -12 of a5 to zero, pushes a pointer and a word onto the stack, calls a routine, clears a field at offset -12 of a4, then restores the stack frame and returns.
- `loop_compare_decrement`: Loads a pointer from a global variable and tests two flag bits in the pointed-to word, returning immediately when either condition fails.
- `decrement_global_loop_2`: Compares a long in the structure in a5 with a global long and branches away when not greater, otherwise it increments the long by a constant and decrements another, then loops.
- `decrement_global_loop`: Compares a long at offset -4 of a5 with a global long and branches away when not greater.
- `add_returned_value_to_long`: Pushes two words from a global table at offsets 19066 and 19064 onto the stack, calls a routine, then adds the returned value to a long at offset -25808 of a4.
- `store_stack_longwords`: Copies three long values from the stack slots at offsets -12, -8 and -4 of a5 into the structure fields at offsets -26664, -26660 and -26656 of a4, then restores a6, unwinds the frame and returns.
- `load_and_negate_word`: Reads a word from offset -24790 of the structure in a4 into d1, and when that value is non-negative it negates it into d0 and falls through to the next routine.
- `format_signed_value`: Loads a 16-bit value from a global variable at offset 8 of a5 and returns immediately if it is negative, otherwise stores the digit 45 on the stack and calls a formatting routine.
- `copy_table_until_zero_byte`: Saves registers a2 and a6, tests a frame word and branches away when zero, otherwise copies bytes from a global table into a frame buffer until a zero byte is reached.
- `copy_and_measure_block`: Copies a byte from one global table to another until a zero byte is reached, then measures the length of the copied block and stores the result.
- `push_args_and_call_6`: Pushes three addresses onto the stack, calls a routine, then adjusts the stack pointer.
- `copy_bytes_until_zero_3`: Copies bytes from a block starting at offset -25690 of a4 to the block starting at offset 24726 of a4, one byte at a time, and stops when a copied byte is zero.
- `call_library_and_loop`: Saves registers d2-d3, a2 and a6, loads a pointer from offset -12 of a5 and calls the library function at offset -192.
- `format_date_and_copy`: Calls two library routines and a formatting routine with a date built from bytes at offsets 54, 55 and 56 of a global structure, then calls two more library routines.
- `build_titles_and_dispatch`: Sets up a frame and saves registers, then calls a routine to build titles and returns early when it fails, otherwise it compares a value against a table entry and, when they match, calls a routine with several arguments and finally jumps to another routine.
- `process_message_values`: Reads a 16-bit value from a global variable, passes it with the constant 40 to another routine, then passes the constant -1 with the result to a further routine.
- `scale_and_dispatch_message`: Reads a word from a global variable, scales it by a factor, stores the result at offset -2 of the structure in a5, then compares it against another global word and calls a routine with two global words when they differ.
- `adjust_values_and_call_library`: Sets up a frame and saves registers, then reads several bytes from a structure, extends and adjusts them, pushes the results, and calls a routine, afterwards it calls library functions and, when a flag is set, adjusts values, calls more library functions, compares a computed value, and jumps to another routine.
- `process_structure_offsets`: Reads the byte at offset 54 of a structure in a0, adds 17, and stores it at offset -6 of the local frame, then calls a library function with argument 2 and another with argument 1.
- `call_library_functions`: Loads two library bases from a global variable, calls a function in each using a constant argument, then returns.
- `compute_and_store_values`: Loads two global tables, checks a byte value at an index, then stores two computed 16-bit results into two local variables and calls another routine with a pointer.
- `loop_and_call_routines`: Saves registers d2, d3 and a6, calls two routines using pointers from a4, then loops a counter from 255 down to zero while reading bytes from tables and calling routines with the results.
- `scale_and_dispatch_values`: Scales two values from a global variable into a 0-100 range, calls a scaling routine with them, then pushes several pointers and calls a dispatch routine using an indirect jump table.
- `push_byte_then_call`: Loads the byte at the address in a3, converts it to a 16-bit value, advances a3 by one, pushes the value on the stack, then calls the routine at 3990 and pops the value off the stack.
- `push_scaled_table_values`: Doubles a word from a global table at offset -18500 of a4 and adds it to a byte read from a table indexed by a word at offset 8 of a5, then pushes both results and calls a routine.
- `store_byte_if_index_valid`: Reads a 16-bit value from a global variable at offset -64da of a4, and if that value is at least 98 it stores byte d1 into a table entry selected by that value, then increments the global counter by one before returning.
- `store_word_check_flag`: Reads a word from offset 8 of the structure in a5 and stores it at offset 22632 of the global variable in a4, then branches away unless a flag equals one.
- `index_tables_and_store_bytes`: Reads two words from a structure, uses them to index two tables, stores the resulting bytes into the local frame, then branches based on a third field.
- `copy_string_check_colon`: Copies a null-terminated byte string from one global table to another, then compares the terminating byte against a colon and branches when it matches.
- `call_and_accumulate_loop`: Calls a routine three times with different arguments, storing the returned value into a word, then loops over a byte table accumulating values into a word.
- `call_and_store_result_7`: Pushes a value and two constants on the stack, calls a routine, and stores the returned value into a word at offset -8 of a5.
- `copy_byte_check_flags`: Copies a byte from a global table into a structure field, then checks a word at offset 8 of a5 and a global byte before calling the routine at offset 3f14.
- `push_args_call_library`: Pushes two global offsets onto the stack and calls the library function at offset 3ede, storing the returned value into a structure field at offset -4 of a5.
- `copy_byte_call_library`: Copies a byte from a global table into a structure field, then calls the library function at offset -72 passing the structure pointer in a6 and the result in d0.
- `copy_byte_check_colon`: Copies a byte from a global table into a structure field at offset -85 of a5, then compares that byte with the decimal constant 58 and branches to the colon handler when they match.
- `call_setup_and_return`: Pushes two global offsets and a cleared stack slot before calling the routine at offset 3e66, then restores the stack and a saved register and returns.
- `dispatch_on_indexed_value`: Saves d7 on the stack, stores the constant 11809 at offset -2 of the structure in a5, and copies a byte from a global variable into a word at offset -6 of a5.
- `store_pointer_then_jump`: Loads the address of a string table into a0, stores it at offset -12 of the structure in a5, and unconditionally transfers control to the routine at offset -2 of a5 without returning.
- `jump_to_zeroed_entry`: Loads the constant zero into d0 and then unconditionally jumps to a fixed address, never returning.
- `clear_then_call_indexed_byte`: Clears the word at offset 19068 of a4, then pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack before calling the routine pointed to by offset -12 of a5.
- `add_then_call_indexed_byte`: Adds the word at offset -2 of the structure in a5 to the word at offset 19068 of a4, then pushes the pointer at offset 8 of a5 and the constants two and one onto the stack before calling the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_60`: Pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack, then calls the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_50`: Pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack, then calls the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_40`: Pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack, then calls the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_30`: Pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack, then calls the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_20`: Pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack, then calls the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_10_2`: Pushes the pointer at offset 8 of the structure in a5 and the constants two and one onto the stack, then calls the routine pointed to by offset -12 of a5.
- `call_indexed_string_byte_10`: Pushes the pointer at offset 8 of the structure in a5 and the constant two on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset -24898 of the structure in a4, and returns.
- `call_indexed_string_byte_9`: Pushes the pointer at offset 8 of the structure in a5 and the constant two on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset -24814 of the structure in a4, and returns.
- `call_indexed_string_byte_8`: Pushes the pointer at offset 8 of the structure in a5 and the constant 600 on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset 20328 of the structure in a4, and returns.
- `call_indexed_string_byte_7`: Pushes the pointer at offset 8 of the structure in a5 and the constant eight on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset -24812 of the structure in a4, and returns.
- `call_indexed_string_byte_6`: Pushes the pointer at offset 8 of the structure in a5 and the constant eight on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset -24804 of the structure in a4, and returns.
- `call_indexed_string_byte_5`: Increments a 16-bit value from a global variable at offset -18580 of the structure in a4, multiplies the result by 24, pushes the product along with the pointer at offset 8 of the structure in a5 and the constant one on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset 20928 of the structure in a4, and returns.
- `call_indexed_string_byte_4`: Doubles a 16-bit value from a global variable at offset -24794 of the structure in a4, pushes the doubled value and the pointer at offset 8 of the structure in a5 on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset 21224 of the structure in a4, and returns.
- `call_indexed_string_byte_3`: Doubles a 16-bit value from a global variable at offset -24794 of the structure in a4, pushes the doubled value and the pointer at offset 8 of the structure in a5 on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset 21424 of the structure in a4, and returns.
- `call_indexed_string_byte_2`: Doubles a 16-bit value from a global variable at offset -24794 of the structure in a4, pushes the doubled value and the pointer at offset 8 of the structure in a5 on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset 21624 of the structure in a4, and returns.
- `call_indexed_string_byte`: Doubles a 16-bit value from a global variable at offset -24794 of the structure in a4, pushes the doubled value along with the pointer at offset 8 of the structure in a5 and the constant one on the stack, then calls the routine pointed to by offset -12 of the structure in a5 with the address of the table at offset 21824 of the structure in a4, and returns.
- `accumulate_indexed_string_byte_7_2_4`: Doubles a 16-bit value from a global variable at offset -24794, pushes it with the value at offset 8 of the structure in a5, a constant 160 and an index onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `accumulate_indexed_string_byte_7_2_3`: Multiplies a 16-bit value from a global variable at offset -24794 by four, pushes it with the value at offset 8 of the structure in a5, a constant 160 and an index onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `accumulate_string_and_add_byte_2`: Pushes the 16-bit value at offset 8 of the structure in a5 along with the constants 2 and 1 onto the stack, then calls the routine at the pointer stored at offset 12 of the same structure to accumulate a string.
- `accumulate_string_and_extend`: Pushes the 16-bit value at offset 8 of the structure in a5 along with the constants 2 and 1 onto the stack, then calls the routine at the address stored at offset -12 of the same structure to accumulate a string.
- `accumulate_string_and_add_byte`: Pushes the byte at offset 8 of the structure in a5 along with the constants 2 and 1 onto the stack, then calls another routine to accumulate a string byte, and finally adds the byte at the position computed from the constant 2 into a global variable at offset 19068 of the structure in a4.
- `accumulate_string_byte_7_4`: Pushes the value at offset 8 of the structure in a5, the constant 2, the constant 1 and an index onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `append_string_byte_to_global`: Pushes the byte at offset 8 of the structure in a5, along with the constants 8 and 1, onto the stack and calls another routine, then adds the first character of the resulting string to the 16-bit value at the global variable.
- `accumulate_string_byte_7_2`: Pushes the value at offset 8 of the structure in a5, the constant 8, the constant 1 and an index onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `compare_then_accumulate_byte_7_2`: Compares the word at offset -2 of the structure in a5 with the constant 12453 and returns immediately when they match.
- `accumulate_indexed_string_byte_7_2_2`: Doubles a 16-bit value from a global variable at offset -24794, pushes it with a constant 160 and an index onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `accumulate_indexed_string_byte_7_2`: Pushes the byte at offset 8 of the structure in a5 and the value 160 onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `accumulate_indexed_string_byte_7`: Pushes the byte at offset 8 of the structure in a5 and the constant 160 onto the stack, then calls the routine whose address is stored at that pointer.
- `accumulate_indexed_string_byte_2_5`: Pushes the byte at offset 8 of the structure in a5 and the constant 160 onto the stack, then calls the routine whose address is stored at offset -12 of a5.
- `accumulate_indexed_string_byte_6`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `accumulate_indexed_string_byte_5`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of the same structure.
- `accumulate_indexed_string_byte_4`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of the structure in a5.
- `accumulate_indexed_string_byte_2_4`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of a5.
- `accumulate_indexed_string_byte_3`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset 12 of the structure in a5, and cleans up the stack afterward.
- `accumulate_indexed_string_byte_2_3`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of a5.
- `accumulate_indexed_string_byte_2_2`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of a5.
- `accumulate_indexed_string_byte_2`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset 8 of the pointer in a5.
- `accumulate_indexed_string_byte`: Pushes the byte at offset 8 of the structure in a5 and the constant 1 onto the stack, then calls the routine whose address is stored at offset -12 of the same structure.
- `accumulate_string_byte_values_2`: Adds the byte at offset 8 of the structure in a5 to the string at offset -18582 from a4, then adds the ASCII value of that string's second character to the 16-bit value at offset 19068 from a4.
- `accumulate_string_byte_values`: Pushes the value from offset 8 of the a5 structure, then pushes the constants 2 and 1, and calls a routine with the address of a string at offset -18578 from a4.
- `check_word_then_push_frame`: Compares a word at offset -2 of a5 with the constant 12321 and branches away if they differ, otherwise pushes a frame value, the constant 1, and an index computed by a helper table, then calls a routine via a pointer in a5 and returns.
- `accumulate_table_values_4`: Pushes the constants 4 and 1 onto the stack, calls another routine via a pointer in a5, then reads a byte from a table at offset 23358 and adds it to a 16-bit value in a structure at offset 19068.
- `accumulate_table_values_3`: Pushes the values 2, 1, and -1 onto the stack, calls another routine via a pointer in a5, then reads a byte from a table at offset 23362 in a4 indexed by a counter and adds it to a 16-bit value at offset 19068 in a4, otherwise it returns.
- `accumulate_byte_into_offset`: Pushes the values 2, 1, and -1 onto the stack, calls a routine via a pointer in a5, then reads the byte at the address computed from the -18538 offset and the loop counter, masks it to 8 bits, and adds the result to the 16-bit value at offset 19068 of the structure in a4.
- `push_frame_index_1`: Pushes a frame value, the constant 1, and an index computed by a helper table, then calls a routine via a pointer in a5 and returns.
- `push_frame_index_192`: Pushes a frame value, the constant 192, and an index computed by a helper table, then calls a routine via a pointer in a5 and returns.
- `accumulate_byte_from_indexed_table`: Doubles the word at offset -18296 of a4 to compute an index, pushes a frame value, that index, and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_8`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_7`: Pushes a frame value, the word at offset -24794 of a4, and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_6`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_5`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_4`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_3`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table_2`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `accumulate_byte_from_table`: Pushes a frame value and the constant 192, then calls the routine pointed to by a pointer at offset -12 of a5.
- `call_and_accumulate_byte_5`: Pushes a frame value and the constant 192, calls the routine pointed to by a pointer at offset -12 of a5, then adds a masked byte read from a table into a global word.
- `accumulate_table_byte`: Doubles the 16-bit value in a global variable, calls the routine pointed to by a pointer in a5 with that doubled value and a constant pushed as arguments, then adds the byte read from a table at offset 23686 in a4 indexed by the previous doubled value minus one to a byte at offset 19068 in a4, otherwise it returns.
- `call_and_accumulate_byte_4`: Doubles a global word, pushes a frame value and constants, calls the routine pointed to by a pointer at offset -12 of a5, then doubles the global again and adds a masked byte read from a table into a global word.
- `call_and_accumulate_byte_indexed_4`: Doubles a global word, pushes a frame value, a constant, and a global address, then calls the routine pointed to by a pointer at offset -12 of a5 and cleans up the stack.
- `call_and_accumulate_byte_indexed_3`: Doubles a global word, pushes a frame value, a constant, and a global address, then calls the routine pointed to by a pointer at offset -12 of a5 and cleans up the stack.
- `call_and_accumulate_byte_indexed_2`: Pushes a frame value, a constant, and a global address, then calls the routine pointed to by a pointer at offset -12 of a5 and cleans up the stack.
- `call_and_accumulate_byte_indexed`: Pushes a frame value and a global address, calls a routine through a pointer, then reads a byte from a computed table address, masks it, and adds it to a global word.
- `call_and_accumulate_byte_3`: Pushes a frame value and a global word, calls a routine through a pointer, then reads a byte from a computed table address, masks it, and adds it to a global word.
- `call_pointer_and_index`: Reads a global word, doubles it, pushes several values, and calls a routine pointed to by a frame value, then restores the stack and branches away when a decremented copy is zero.
- `call_pointer_add_byte`: Saves a frame pointer and a global word on the stack, calls a routine pointed to by a frame value, then reads a byte from a global table and adds it to a global byte.
- `call_then_accumulate_byte`: Calls a routine pointed to by a global, then reads a byte from a table indexed by a frame value and adds it to a byte at offset 19068 of a4.
- `call_and_accumulate_table_2`: Saves a frame value on the stack, pushes two constants and a pointer, then calls the routine pointed to by a global.
- `call_and_accumulate_table`: Pushes two small constants and a pointer, then calls the routine pointed to by a global at offset -12 of a5.
- `call_pointer_and_accumulate`: Pushes a pointer and a couple of globals, then calls the routine at that pointer and restores the stack.
- `call_via_pointer_accumulate_2`: Pushes a pointer from offset 8 of the structure in a5, a word from a global, a one, and another global onto the stack, then calls the routine at offset -12 of a5.
- `call_via_pointer_accumulate`: Pushes several values and calls a routine through a pointer in the structure, then accumulates low bytes from a table into a global word.
- `call_routine_accumulate_bytes_2_2`: Pushes several values onto the stack and calls a routine via a pointer in the structure in a5.
- `call_routine_accumulate_bytes_2`: Pushes several values onto the stack, calls a routine via a pointer in the structure in a5, restores the stack, then reads a word from a table, masks it to eight bits and accumulates it into a global variable.
- `call_routine_accumulate_bytes`: Calls a routine via a pointer in the structure in a5, then reads eight bytes from a table and accumulates their masked values into a global word at offset 19068.
- `call_and_accumulate_byte_2`: Reads a word from offset -18296 of the structure in a4, doubles it, pushes several values, calls a routine from a pointer in a5, then reads a byte from a table and adds it to a global variable.
- `call_and_accumulate_byte`: Pushes several values onto the stack, calls a routine obtained from a pointer in a5, restores the stack, then reads a word from a global table, indexes a byte from another table by that value, and adds the low byte of that byte to a word at offset 19068 of the structure in a4 before jumping to a common exit.
- `compare_and_accumulate_byte`: Compares a word at offset -2 of the structure in a5 against three constant values and jumps to a handler when any matches.
- `call_and_compare_flag`: Branches away when a word at offset 12 of a5 is nonzero, otherwise pushes a pointer and two constants and calls a routine before comparing a global word against a stored value.
- `store_three_bytes_flag`: Pushes a pointer from the stack frame and two constants onto the stack, then calls a routine to store three bytes.
- `test_and_call_routines`: Tests a word at offset 12 of the structure in a5 and jumps away when nonzero, otherwise calls a routine with a pointer, then pushes arguments and calls another routine via an absolute jump.
- `increment_and_call_routines`: Increments a global word and calls a routine, then sets up arguments and calls another routine before restoring registers and returning.
- `call_routine_add_byte`: Saves registers, loads a routine pointer from a global variable, calls it with a value from the stack frame, then adds a byte read from a table indexed by a counter to a global word.
- `add_signed_byte_to_a3`: Compares the long at offset -12 of a5 against one and branches away when it differs, then checks the byte at offset -7 and branches away when it is not positive.
- `offset_a3_by_signed_byte`: Copies a3 into d0 and d1, reads a byte at offset -8 from a5, sign-extends it to a long, adds it to d1, and stores the result into a3 before jumping to 11946.
- `accumulate_table_bytes`: It saves a byte at offset -7 of a5, negates it, and calls a routine through a pointer at offset -16 of a5.
- `copy_byte_if_nonzero`: Reads a byte at offset -8 from a5, decrements that byte, and if the original value was zero it jumps to 11946; otherwise it stores the byte at offset -7 into the location pointed to by a3, advances a3, and repeats.
- `restore_registers_and_return_17`: Restores the caller's stack frame and returns.
- `scan_byte_range_loop`: Sets up a stack frame, computes an address from a global long and a scaled word, and stores a pointer into a stack slot.
- `negate_and_call_table`: Returns when the counter in d7 is not greater than one, otherwise it negates the counter and stores it as a byte at offset -13 of a5.
- `accumulate_table_via_a3`: It pushes a pointer from a global variable at offset 8 of a5 and the contents of a3, then calls a routine through a pointer at offset -18 of a5.
- `offset_a3_by_d7`: Copies a3 into d0 and d1, sign-extends d7 into d2, adds it to d1, stores the result into a3, and jumps to 12270.
- `compare_table_bytes_advance`: Reads a byte from the table entry pointed to by a3 and the following byte, compares them, and when they differ advances a counter in d7 past any value of 126 before returning.
- `push_and_call_via_d7`: Stores the value of d7 at offset -13 of a5, pushes a pointer from a global variable at offset 8 of a5 and the value of d7, then calls a routine through a pointer at offset -18 of a5.
- `accumulate_table_via_d7`: It pushes a pointer from a global variable at offset 8 of a5 and the value of d7, then calls a routine through a pointer at offset -18 of a5.
- `compute_pointer_offset`: Copies the pointer in a3 into d0 and d1, sign-extends the 16-bit value in d7 into d2, adds it to d1, stores the resulting pointer back into a3, then branches to the routine at that address.
- `call_then_accumulate_indexed_byte`: Clears a byte at offset -13 from a5, pushes a value and two ones on the stack, calls the routine pointed to by the value at offset -18 from a5, then runs a loop that masks a byte at a computed index to 255 and adds it to the word at offset 19068 from a4.
- `restore_registers_and_return_18`: Restores the caller's registers by loading d2 through d7 and a3 from the stack, then unwinds the stack frame and returns.
- `frame_check_and_store`: Links the stack with a 16-bit frame, tests the word at offset 8 of a5, and returns immediately when that value is zero.
- `store_result_or_zero`: Pushes a zero, calls a routine, discards the result, stores the returned longword into a frame slot, and branches away when it is nonzero, otherwise loading zero.
- `call_routines_branch`: Pushes two globals and calls a routine, then calls another routine and branches away when register d0 is nonzero.
- `divide_and_store_result_2`: Calls a routine, then checks a word at offset 8 of a5 and branches away when zero.
- `fill_range_with_three_2`: It loops while a counter is less than fifty-nine, multiplying the counter by one hundred and indexing a table at offset 23176 of a4.
- `fill_range_with_three`: Sets d7 to one, and while it is less than 99 it stores the value three at offset 1708 from a0 and the byte at a0, then increments d7 and repeats.
- `restore_stack_and_return_4`: Restores the caller's stack frame by removing the local variables allocated on the stack and then returns to the caller.
- `initialize_structure_and_allocate`: Initializes a structure with zeroed pointers, calls an allocation library function, and returns early when a counter reaches a threshold.
- `process_structure_list`: It returns early when a counter at offset -2 of a5 is at least fifty.
- `call_library_and_increment`: Loads d1 with the long at offset -22 from a5, sets a6 to the library base at offset -1a0 from a4, calls the library function, clears that long, increments the word at offset -4 from a5, and jumps to 13088.
- `divide_and_index_list`: Checks a frame word and branches away when it is not positive, then calls a routine and divides a value by that frame word.
- `copy_and_measure_string`: Copies bytes from a block at offset 22636 of a4 to one at offset 24726 of a4 until a zero byte is reached, then looks up a pointer in a table indexed by a word at offset 6 of a global at offset -18 of a5 and tests the byte it points to.
- `free_memory_blocks`: Reads a pointer from a global variable at offset -22 of a5 and calls exec.FreeMem on it when non-zero, then reads a second pointer at offset -26, loads a size of 260 into d0, and calls the library at offset -90 before restoring registers and returning.
- `copy_buffer_and_check`: Saves a frame pointer and several registers, copies bytes from a global table into a frame buffer until a zero byte is reached, then checks a frame word and a global byte before clearing a local byte and branching away.
- `copy_buffer_call_routine`: Loads a global pointer and copies bytes into a frame buffer until a zero byte is reached, then pushes addresses and calls a routine before testing a frame byte.
- `call_two_routines`: Pushes two addresses onto the stack and calls a routine, then pushes two more addresses and calls another routine before jumping onward.
- `copy_tables_and_measure`: Copies a byte from one global table to another until a zero byte is reached, then copies from a second table the same way.
- `call_library_store_pointer`: Computes a stack pointer, loads a library base from a global at offset -416 of a4, and calls the library function at offset -72.
- `copy_table_until_zero`: Copies long words from a source table at offset -46cf of a4 into a buffer at offset -a0 of a5, advancing both pointers, until a zero long word is copied, then restores registers and returns.
- `call_routine_three_times`: Sets up a stack frame and saves several registers.
- `split_byte_and_branch`: Tests a byte in the structure and branches away when it is zero, otherwise splits it into two parts and computes a difference before jumping away.
- `loop_store_and_decrement`: Decrements d5, and if it becomes negative it jumps to 14248; otherwise it stores d6 at the location pointed to by a0, decrements d7, and repeats.
- `test_register_and_branch`: Tests a register, branches away when it is not below zero, and otherwise loads a zero into a data register before jumping to another routine.
- `loop_index_tables_until_marker`: Clears a word in a global variable at offset -24794 of the structure in a4, then repeatedly indexes a table using that counter and stores a value into another table until the indexed entry equals the end marker.
- `double_value_call_routine`: Doubles a word from offset -24794 of the global variable in a4, calls a routine via a pointer, then branches to another routine when the doubled value does not match.
- `shift_table_entries`: It loops while a counter derived from a word at offset -24794 of a4 is nonzero.
- `fill_table_with_computed_values`: Reads a word at offset -24794 from a4 into d7, and while d7 is not zero it computes one hundred and sixty-five minus the word at a computed index, stores that result at another computed index, and decrements d7.
- `restore_registers_and_return_19`: Restores registers d5 through d7 from the stack, restores the stack pointer to its value before the routine was entered, and returns.
- `copy_table_range`: It loops while a counter is less than six thousand.
- `increment_register_d7`: Adds the decimal constant 100 to the word in register d7, then branches to an earlier routine.
- `update_table_via_index`: Decrements the word at offset -24794 of a4 and returns when it reaches zero, otherwise it indexes a table at offset 21224 of a4, computes two scaled values using the decimal constants 100 and 99, and stores the combined result back into the table.
- `restore_registers_and_return_20`: Restores the caller's stack frame by removing the saved registers d2 through d7 and a2 from the stack, then returns.
- `read_bytes_and_branch`: Reads four consecutive bytes from a structure and computes adjusted and subtracted values into local variables, then compares them and branches to one of several handlers.
- `setup_frame_and_check_value`: Sets up a stack frame, saves register a6, stores the decimal constant 58128 at offset -78 of a5, loads the decimal constant 1 into register d0, stores it at offset -76 of a5, compares the word at offset 12 of a5 with the decimal constant 1, and branches to a later routine when the values differ.
- `init_buffer_and_call_library`: Copies ten long words from a source table at offset -1600 of a4 into a buffer at offset -74 of a5, stores a byte of value four at offset -30 of a5, sets up a series of fields including a large value at offset -20, then calls the library at offset -84 before restoring registers and returning.
- `setup_and_call_routine`: Sets up a local frame, pushes a global address, and calls a routine, then branches away when the result is zero.
- `loop_index_tables`: Initializes a counter to zero and stores it in a global word, then returns immediately when the counter reaches a limit.
- `loop_call_routines_indexed`: Initializes registers, then loops calling several routines with scaled indices, updating table entries and a counter before looping.
- `increment_register_d7_again`: Increments the word in register d7 by the decimal constant 1, then branches to an earlier routine.
- `test_long_then_call`: Tests a long at offset -10 of a5 and branches away when zero.
- `jump_to_routine_6`: Jumps to a routine at a fixed address and does not return.
- `jump_to_address_zero_2`: Consists of a single unconditional jump to address zero, transferring control without modifying any register or memory.
- `jump_to_global_1240`: Unconditionally transfers control to the routine at the global address 1240 and never returns to this routine.
- `jump_indirectly_3`: Unconditionally jumps to the routine at the given address and never returns.
- `jump_to_global_3178`: Unconditionally transfers control to the routine at the global address 3178 and never returns to this routine.
- `jump_indirect_entry`: Consists of a single unconditional jump to a fixed address and never returns.
- `jump_to_routine_15`: Performs an unconditional jump to another routine, transferring control without returning.
- `indirect_jump_1ed0`: Performs an indirect jump to the address stored at the global variable at offset 1ED0 and does not return.
- `indirect_jump_via_a0`: Performs an indirect jump to the address held in register a0 and does not return.
- `jump_to_byte_table_loop`: Unconditionally transfers control to the routine at absolute address 670 and never returns.
- `jump_unconditionally`: Performs an unconditional jump to a fixed location and does not return.
- `indirect_jump_7e0`: Performs an indirect jump to the address stored at the global variable at offset 7e0 and does not return.
- `jump_to_global_41e`: Jumps to a routine located at the global address 1054 and never returns to this routine.
- `delegate_stack_restore`: Jumps to the routine that restores a value from the stack, compares a global variable with a word at offset 8 of the structure in a5 and adds a global constant, then returns.
- `call_library_stub`: Jumps to the library function stub that loads a library base from a global variable and calls the function at offset minus 306 of that base, returning its result to the caller.
- `jump_to_absolute_1380`: Unconditionally jumps to the code at the absolute address 1380 and never returns to this routine.
- `unconditional_jump`: Performs a single unconditional jump to another routine and returns, containing no other instructions.
- `do_nothing_return`: Immediately returns without executing any instructions.
- `jump_to_setup_handler`: Jumps to a routine at 3510.l, which clears a word at offset 25724 of a global variable, stores a long passed in the frame at offset 8 into offset 25720, and pushes two arguments.
- `jump_to_routine_12`: Unconditionally jumps to the routine at the fixed absolute address and never returns.
- `jump_to_handler_4`: Performs an unconditional indirect jump to the fixed absolute address, having loaded the constant 176 into the working register d0.
- `empty_routine_jumps`: This routine is empty, containing only a single jump instruction that transfers control elsewhere and returns immediately.
- `jump_to_dispatch_decrement`: Unconditionally transfers control to the routine at absolute address c12 and never returns.
- `empty_returns`: It is an empty routine that does nothing and returns immediately.
- `jump_to_handler_5`: Performs an unconditional indirect jump to the fixed absolute address, having loaded the constant 56 into the working register d0.
- `jump_to_absolute_42`: It unconditionally jumps to the code at the absolute address 66 and never returns to its caller.
- `jump_to_routine_17`: It is a jump to a fixed routine and returns immediately.
- `dispatch_table_entry`: Jumps to the routine at the global jump table entry and returns.
- `jump_to_divide`: Performs an unconditional jump to the fixed absolute address and never returns to the caller.
- `jump_to_absolute_zero`: It unconditionally jumps to the code at the absolute address 0 and never returns to its caller.
- `empty_returns_2`: This routine is empty; it does nothing and returns immediately.
- `jump_to_handler_6`: Performs an unconditional indirect jump to the fixed absolute address, having loaded the constant 216 into the working register d0.
- `jump_to_vector_532`: Performs an unconditional jump to the routine at absolute address 532 and never returns to this location.
- `test_byte_and_return`: Tests a byte at offset negative 72 of a structure and returns immediately when that byte is zero, otherwise it branches into another routine.
- `clamp_table_values_loop`: Loops over a table subtracting one from a counter each time, clamps four values against thresholds, then compares pairs of clamped values and branches away when the first pair is not smaller.
- `clamp_and_index_table`: Clamps four input values against thresholds, sets bits in a global byte for each clamped value, and computes an indexed table entry by subtracting, dividing, and adding a base.
- `clamp_message_values`: Reads two words from the message and returns to another routine when a global word is zero.
- `test_flag_bits_in_structure`: It saves several registers on the stack, computes an address from a structure in a4, and tests two bits of a long word at that address, branching away when a bit is not set.
- `save_stack_words_and_call`: Copies two stack words into global long variables at offsets 485a and 485c, saves a5, advances it, calls another routine, then restores a5 and returns.
- `index_table_by_scaled_value`: Tests two global words against fixed constants and a byte field of the structure in a5, then indexes a table at offset 23176 of the structure in a4 using a scaled value to load a byte.
- `load_constant_and_jump_2`: Loads the constant 216 into the working register d0 and then performs an unconditional indirect jump to the fixed absolute address, so it never continues executing the following instructions.
- `load_constant_and_jump_3`: Loads the constant 56 into the working register d0 and then performs an unconditional indirect jump to the fixed absolute address, so it never continues executing the following instructions.
- `load_constant_and_jump_4`: Loads the constant 176 into the working register d0 and then performs an unconditional indirect jump to the fixed absolute address, so it never continues executing the following instructions.
- `index_table_and_adjust_byte`: Reads a byte from the structure pointer in a0, sign-extends it, indexes into a table at offset 835 of the structure in a4, then tests a bit and subtracts a constant from the byte.
- `increment_and_store_byte`: Reads a byte from the structure pointer in a0, sign-extends it, stores it back, advances the pointer, and returns when the new byte is zero.
- `compute_and_store_offsets`: Divides a word from the stack by one hundred, then stores two computed offsets and three derived values into consecutive word slots of a structure pointed to by another stack value.
- `store_computed_structure_values`: Subtracts a global value from two stack words, converts each result into a structure offset, and writes four related values into consecutive slots of the pointed-to structure.
- `compute_clamped_quotient_2`: Reads a word from offset 8 of the structure in a5, subtracts 23 from it, divides the extended value by 16, adds a value from a global variable at offset -24896 of a4, clamps the result to at least 1, and returns it in d1.
- `compute_clamped_quotient`: Computes a quotient by subtracting 23 from d0, extending it to a longword, dividing by d2, and adding d3, then branches to a larger value handler when the result exceeds 98.
- `compute_divided_quotient_2`: Subtracts 23 from d0, extends it to a longword, divides by d2, adds d3, copies the result into d1, and falls through into another routine.
- `compute_divided_quotient`: Divides the 32-bit value at offset 16 of the structure in a5 by 8, adds a value from a global variable, and stores the quotient in register d1 and the low byte at offset -4 of the structure in a5.
- `compute_and_clamp_result`: Subtracts twenty from the value in d0, sign-extends it, divides by d3, adds d4, and stores the result in d2.
- `divide_add_and_branch`: Subtracts the decimal constant 20 from d0, sign-extends it, divides by d3, adds d4, and stores the result in d2 before branching into another routine.
- `multiply_and_add_operand`: Loads a constant into a register, multiplies it by one hundred, adds a second operand, and returns the resulting product in the return register.
- `call_with_zero_argument`: Tests a global word and branches away when zero, then calls a routine with a pushed zero, calls another with a pushed frame value, cleans the stack, unlinks the frame, and returns.
- `call_with_pointer`: Stores the constant 54 at offset negative two of the frame, calls a routine with the pointer from offset eight of a5, and returns immediately when that pointer is zero.
- `test_and_set_flag_byte`: Tests a byte at a negative offset of a base register, and if that byte is nonzero it jumps away; otherwise it sets the byte to one and returns its value.
- `translate_index_value`: Loads an index from offset eight of a5, subtracts two, and calls a routine at a fixed address to translate it into a value used in later arithmetic.
- `add_and_branch`: Adds the decimal constant 20 to d5 and then branches forward into the middle of another routine, so it performs no work of its own.
- `index_table_from_d7`: Tests the word at offset negative ten of the structure in a5 and jumps elsewhere when it is zero.
- `compare_increment_and_load`: Computes a value from d7 by dividing one past it by the decimal constant 50, compares it with d6, and if equal increments a global word at offset -24792, divides that value by 105, and loads a longword from a computed table address before restoring registers and returning.
- `divide_and_store_quotients_2`: Reads a word from offset 8 of the structure in a5, divides it by 100 twice, stores both quotients at offsets -2 and -4, then checks a global variable at offset -4848 of a4 and returns zero or a negated flag into d0.
- `restore_stack_and_compare`: Restores d2 from the stack, compares a global word at offset -24898 with a word at offset 8 of the structure in a5 and adds a global constant if it is smaller, then repeats the comparison with a second pair of values before clearing d0 and branching into another routine.
- `validate_and_dispatch_index`: Sets up a stack frame, stores the constant one at offset negative two of the structure in a5, and loads that value into d1.
- `increment_and_branch_2`: Increments the word at offset -2 of the structure in a5 by one, then jumps into the middle of another routine.
- `restore_stack_and_return_3`: Restores the caller's stack frame by removing the saved registers from the stack, clears register d0 by setting it to zero, and then returns.
- `store_digit_from_quotient`: Loads a word from offset 8 of the frame in a5, sign-extends it, divides it by ten, and if the quotient is nonzero stores its ones digit as an ASCII character through a global pointer.
- `write_space_and_digit`: Writes a space byte to a location from a global pointer, divides a stack word by a second operand, and stores the resulting digit as a character into the following byte.
- `divide_add_and_clamp`: Loads a word from offset 8 of the structure in a5, divides it by 100, adds the word at offset 10, stores the result at offset negative 2, and loads one into d2 when the value is below one.
- `jump_only`: Contains only a jump to a fixed address and performs no other operation.
- `jump_to_absolute_2`: It is a single instruction that jumps to an absolute address without returning.
- `jump_to_address_320`: Performs an unconditional jump to the address at 800, which is how the routine ends execution.
- `store_d0_in_table_slot_3`: Reads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then clears the low word of d0 and returns.
- `store_d0_in_table_slot_2`: Loads a pointer from a global variable, computes an index from the low word of d0, and stores d0 at the first matching slot of a table, then jumps to the address at offset 36.
- `decrement_and_compare_table`: Decrements a global counter at offset -24794 of a4, and when the previous value was nonzero it indexes a table at offset 21224 of a4 using the decremented value, compares the fetched word with the value at offset 8 of a5, and branches back to repeat the decrement when they differ.
- `check_byte_and_increment_counter`: Loads a pointer from a global variable, adds a zero offset to it, checks the byte it points to, and returns immediately if that byte is zero.
- `increment_and_store_counter`: Adds 20 to a value from register d7, compares the result with a word at offset -18594 of the structure in a4, and stores the incremented value back there when the comparison fails.
- `compare_index_and_return_flag`: Tests a word in a structure and, when it is not negative, compares a doubled index against a stored value.
- `validate_and_update_index`: Reads two bytes from a structure in a5 and stores them at offsets -11, -12 and -13, then compares values from indexed tables and, when they match, increments a counter at a global address and calls a routine to update a table.
- `validate_index_and_clear_byte`: Reads two bytes from indexed tables and compares them against stored values, jumping to a common exit when either check fails.
- `restore_registers_and_return_12`: Restores the caller's stack frame by removing the values it saved into registers d2 through d7 and a6, then returns.
- `validate_and_write_table`: Reads a byte from a table indexed by a value from a5, compares it against a second value, and branches away when they match.
- `store_one_at_computed_address`: Multiplies the value in d0 by 200, adds the result to the address at offset 23686 of the structure in a4, doubles a word stored at offset -6 of a5, adds that doubled value to the address, stores 1 at that address, and falls through to another routine.
- `check_table_bytes_and_call`: Tests a byte at an address computed from d5 and branches away when it is zero, then checks a second table byte and, when it equals 255, pushes d6 and d7, calls another routine, removes the pushed words, and falls through to the caller's continuation.
- `compare_table_word_multi_2`: Reads a byte from a table indexed by d5, and when that byte is not below two reads another byte from a second table, pushes several registers onto the stack, calls a routine, and restores the registers before returning.
- `compare_table_word`: Loads two words from the parameter block and computes a table index from one of them.
- `shift_words_downward`: Sets a counter to nineteen and, when the word at offset 0 of the table in a3 is nonzero, stores the word in register d7 at that location.
- `shift_table_words_downward`: Sets a counter to nineteen, then repeatedly copies a word from a table to the following location while the counter is not yet zero.
- `store_word_at_pointer`: Stores the word in register d7 at the memory location pointed to by register a3, restores the caller's registers from the stack, and returns.
- `compare_table_word_multi`: Loads three words from the parameter block and builds a table index from one of them, then offsets into the table using the second word.
- `shift_words_downward_small`: Sets a counter to four and, when the word at offset 0 of the table in a3 is nonzero, stores the word in register d7 at that location.
- `shift_table_words_downward_small`: Sets a counter to four, then repeatedly copies a word from a table to the following location while the counter is not yet zero.
- `restore_registers_and_return_13`: Restores registers d2, d4 through d7 and a3 from the stack, removes the frame pointer, and returns.
- `compute_subtraction_pair_2`: Reads words at offsets eight and ten of a5, divides each by one hundred using signed division, and subtracts one quotient from another, negating the result when it is negative before falling through to the caller's continuation.
- `compute_subtraction_pair`: Copies the long value in register d7 into register d0 and subtracts the word in register d5 from it.
- `compute_absolute_subtraction_2`: Copies the long value in register d6 into register d1 twice and subtracts the word in d4 from it, then compares the result with register d0 and, when it is not below, copies the long value in d7 into d0 twice and subtracts the word in d5.
- `compute_subtraction_result`: Copies a long value into the return register twice, subtracts a word from it, and then jumps to the shared exit routine.
- `compute_absolute_subtraction`: Copies a long value into the return register twice, subtracts a word from it, and if the result is negative it negates that value before jumping to the shared exit routine.
- `subtract_and_return_result`: Subtracts the word in register d4 from the value in register d6, places the result in register d0, restores the caller's registers from the stack, and returns.
- `compare_words_and_clear`: Sets up a stack frame and saves registers, loads a pointer and two words from the structure in a5, compares those words and branches away when they differ, otherwise clears a field and computes a pointer to clear a byte.
- `divide_and_index_lookup`: Extends and divides several words by one hundred to produce quotients, stores four results into negative offsets of the structure in a5, counts how many comparisons hold using two incrementing counters, builds an index from those counters, looks up a word through a computed pointer, stores it through a3, pushes two arguments, calls a routine, and branches based on the returned value.
- `negate_when_negative`: Tests a word far in a4 and branches when it is nonzero, then subtracts two words from a5, negating the result when it comes out negative.
- `subtract_and_negate_when_negative`: Subtracts a value from a word in a5, copies the result into another register, subtracts again, and negates the result when it is negative.
- `subtract_and_loop_until_zero`: Subtracts d0 from d1, compares the result with d2, and either updates d2, decrements it until it reaches zero, or returns zero, so it iterates while the values differ.
- `call_and_divide_result`: Tests a frame value and a register, then calls a routine and divides the result by eight before subtracting a computed value from a word read from a pointer.
- `compute_and_store_index_4`: Tests a word at offset -24796 of a4 and branches away when it is nonzero; otherwise it loads a word from a3, subtracts d5, masks the result with 7, calls a routine through a computed pointer, and stores 1 in d0 when the result is nonzero.
- `clear_or_set_value_by_pointer`: Tests a word in a structure and, when it is not zero, stores a maximum value at a computed pointer.
- `index_table_and_branch`: Compares a value loaded from a5 with another value from the same frame and branches away when it is not greater, otherwise it loads d7 into two registers, doubles it, indexes a table using that doubled value, reads a word from the entry, and jumps to the address stored there.
- `index_table_and_call_pair`: Computes an index by tripling a word from d6 and subtracting it from a base, then loads a word from a table entry in a4 and pushes it along with a word from offset 24918 of the same structure.
- `subtract_and_return_absolute`: Returns immediately when the word at offset -60dc of a4 is nonzero, otherwise subtracts the word at offset -a of a5 from the word at offset -c and returns its non-negative value.
- `compute_and_pass_pointer`: Computes a pointer by adding a word at offset -18574 of the structure in a4 to a word at offset -18594, masks the low three bits, stores the result at offset -18574, then calls a routine passing the masked value and a word at offset 24918 of the structure in a4.
- `clear_byte_at_computed_pointer_2`: Tests a word at offset -18594 of the structure in a4 and, when it is nonzero, jumps away to another routine.
- `clear_byte_at_computed_pointer`: Computes a pointer by adding a word at offset -18594 of the structure in a4 to a base address at offset -18574, clears the byte at that address, sets the return value in register d0 to zero, restores the caller's registers from the stack, and returns.
- `divide_value_by_hundred`: Reads a word from offset 8 of the structure in a5 and divides it by 100 to obtain a quotient and remainder.
- `return_one_value`: Sets the return value in register d0 to one, restores the caller's registers from the stack, and returns.
- `accumulate_table_values_2`: Reads a word from offset 8 of the structure in a5 and initializes a counter to nine.
- `copy_reg_to_return`: Copies the value from register d6 into register d0, restores the caller's registers from the stack, and returns.
- `increment_table_entry`: Reads a word from offset 8 of the structure in a5 and initializes a counter to nine.
- `increment_word_at_pointer`: Loads a pointer from offset 10 of the structure in a5, increments the word it references by one, and then jumps away to another routine.
- `restore_regs_and_return_2`: Restores the caller's saved registers from the stack, restores the frame pointer, and returns.
- `load_params_and_check_d7`: Reads three words from the parameter block at offsets 8, 10 and 12, checks whether the second word is nonzero and branches away when it is, then compares the first word to the value 65534 and branches away unless it matches.
- `index_table_and_load_value`: Links a frame, saves two registers, reads a word from offset 10 of the structure in a5, doubles it, and indexes a table using the result.
- `index_tables_and_compare`: Tests a word at offset 12 of the structure in a5 and branches away when it is zero.
- `load_nine_hundred_ninety_nine_2`: Stores the decimal constant 999 into d0 and then jumps away to another routine.
- `index_tables_and_multiply`: Reads a byte from a table indexed by a word at offset 8, multiplies it by 50 via a call, reads a word from a second table indexed by a doubled value at offset 10, multiplies it by 50 via another call, and branches to the caller's continuation.
- `load_nine_hundred_ninety_nine`: Loads the decimal constant 999 into d0, then restores the caller's registers and returns.
- `setup_d0_and_d2_when_d1`: Reads a word into d1 and branches away when it is not one, then loads the constant two into d0, copies a word from a global variable into d2, stores both values into the parameter block and jumps to the tail.
- `store_constants_when_d1_is_two`: Compares d1 with 2 and, when they differ, jumps away.
- `compute_scaled_table_values`: Initializes two 16-bit fields in a structure via a5, then computes scaled and divided products from two tables indexed by that value, compares them, and stores the smaller result before returning.
- `compute_scaled_value`: Subtracts 36 from the word at offset 12 of the structure in a5, doubles the result, and uses it to index a global table.
- `compare_divided_quotients`: Reads two 16-bit values from offsets 0 and 2 of the structure in a5 and divides each by 100 to get a quotient and remainder, then compares the two quotients and returns zero if they are equal.
- `handle_divide_by_hundred`: Saves register d2 on the stack, calls divide_value_by_hundred to divide a word at offset 8 of the structure in a5 by 100, then restores d2.
- `copy_words_and_clamp`: Copies two words from the structure in a5 to the destinations pointed to by a0, then calls a routine with the arguments one and that value.
- `clamp_and_store_words`: Clamps the word at offset -24898 of the structure in a4 to a minimum of 58, stores the clamped value as a word at offset -6 of the structure in a5, then clamps the word at offset -24896 of the structure in a4 to a minimum of 98 and stores either that value or the constant 1 at offset -36 of the structure in a5.
- `clamp_scale_and_call_routine`: Clamps a global word to a minimum, scales it by a constant, adds a table value, divides by the constant twice, stores intermediate results, calls a routine with two arguments, then restores registers and returns one.
- `divide_and_call_twice`: Reads a word from the parameter block, extends it and divides by one hundred, then pushes the quotient and calls a routine.
- `jump_indirect_only`: This routine is empty, containing only a single unconditional jump to a fixed address and never returning.
- `jump_to_multiply_and_combine`: Jumps to the routine referenced by the text string, which multiplies two word operands together and combines the partial products before restoring the saved pointers.
- `jump_to_update_counter_value`: Jumps to the routine that reads a word from a global variable at offset 572, multiplies it by 5981, adds three, masks the result, and stores the value back into the structure.
- `jump_to_global_routine`: Immediately transfers control to the routine at the global address 2202 and never returns to the caller.
- `jump_to_digit_store`: Immediately transfers control to the routine at the absolute address 43022 without executing any instructions of its own.
- `invoke_keyboard_handler`: Jumps to the keyboard handler at 16512 and never returns.
- `jump_to_copy_byte_when_nonzero`: Jumps to the routine that compares a byte at a pointer with zero and, when it is nonzero, decrements the pointer and copies a byte from another pointer into it.
- `jump_indirectly_4`: Performs an unconditional indirect jump to the fixed absolute address, doing nothing else.
- `unconditional_jump_exit`: Performs an unconditional jump to a fixed location in the program, ending the routine.
- `empty_routine_3`: This routine is empty: it contains only a jump to the start of the current routine, which immediately returns control to the caller without performing any operation.
- `jump_indirectly_always`: Performs an unconditional indirect jump to the address stored at the given location and never returns.
- `jump_to_absolute_58`: Unconditionally jumps to the code at the absolute address 58 and never returns to this routine.
- `store_index_in_table_7`: Loads a pointer from a global variable, computes an index from the low word of d0, stores d0 at the first matching slot of a table, then clears d0 and jumps to the handler routine.
- `push_args_and_call_4`: Sets up a stack frame, pushes a pointer from the structure in a5 and a pointer from a4, loads a one into d0 and pushes it, pushes two words from the structure in a5, calls a routine, cleans up the stack, removes the frame, and returns.
- `save_context_and_call`: Saves the caller's return address and two 16-bit values from the stack, then calls a routine that stores pointers and compares them, and finally restores the stack and returns.
- `push_args_and_call_3_2`: Sets up a stack frame, pushes a pointer from a4 and a pointer from the structure in a4, loads a one into d0 and pushes it, pushes two words from the structure in a5, calls a routine, cleans up the stack, removes the frame, and returns.
- `push_args_and_call_5`: Sets up a stack frame and pushes a value from a global variable at offset 19086 of the structure in a4, a pointer from a5, and the return address onto the stack, then calls another routine before restoring the stack and returning.
- `store_pointers_and_compare`: Saves register d7, stores a value of 30 at offset 19086 of a4, copies a pointer from a5, builds another pointer from a table, and stores several values into a4 before comparing two words.
- `clamp_result_to_125`: Pushes two words and calls a routine through a fixed address, then scales a value, clamps it to 125 when it is below that, and returns the clamped value.
- `compute_and_store_result_3`: Pushes two words and calls a routine, then computes a value from the result, stores a word from a4 at offset 19084 through a pointer, and returns zero when d7 is below 10000 otherwise it branches away.
- `increment_and_restore_d7`: Copies the 32-bit value from d7 into d0, increments that copy by one, restores the caller's registers from the stack, unwinds the frame, and returns.
- `compute_and_clamp_result_2`: Sets up a stack frame, saves d7, stores a value at offset 19086 of a4, copies a pointer from the structure in a5 to offset 19076, loads a word from offset 8 into d0 and another from a negative offset into d1, doubles d1, computes an indexed pointer, adds a stored pointer to it, stores a maximum value, saves several results, compares two values, and branches away when they differ.
- `clamp_result_to_125_2`: Pushes two registers, calls a routine, then doubles a value, adds 25, and when the result is below 125 it loads 125 into d0 and branches to a routine.
- `store_and_clamp_result`: Pushes two words and calls a routine through a fixed address, then pushes more values and calls another routine, stores a word at offset 19084 through a pointer, and clamps a value to 10000 when it is below that.
- `copy_register_and_return`: Copies the value in d7 into d0 twice, increments d0 by one, restores the caller's registers from the stack, and returns.
- `compute_and_negate_difference`: Reads two words from offsets 8 and 10 of the structure in a5, divides each by 100, subtracts the second quotient from the first, and stores the result in d6.
- `negate_d6_when_below_zero`: Copies d7 into d0, tests d6 and branches away when it is not below zero, otherwise negates a copy of d6 and falls through to the caller's continuation.
- `compare_and_set_flag_2`: Copies d6 into d1 and compares it with d0, branching away when d0 is not below d1; then tests d7 and branches away when d7 is below zero, otherwise loads zero into d0 and falls into the caller's continuation.
- `load_four_and_branch`: Loads the constant four into d0, then jumps into the middle of another routine.
- `compare_and_set_flag`: Compares d6 with d7 and branches into another routine when they differ, loads one into d0 when d7 is not below zero, and otherwise returns.
- `load_five_and_branch`: Loads the constant five into d0, then jumps into the middle of another routine.
- `compare_negated_d6`: Copies the value in d6 into d0 and negates it, then compares the result with d7.
- `load_three_and_branch`: Loads the constant three into d0, then jumps into the middle of another routine.
- `check_then_load_two`: Tests the word in d6 and, when it is greater than zero, loads the constant two into d0 before jumping into another routine.
- `load_six_and_restore`: Loads the constant six into d0, restores the caller's registers from the stack, unwinds the frame, and returns.
- `check_frame_and_load_value`: Saves registers and returns when the word at offset 10 of the frame is non-zero, otherwise it loads the constant 10000 into d0 and jumps to a routine.
- `compute_and_store_result_2`: Pushes two words and calls a routine, then scales a result by two and adds it to a value from the structure in a5, reading a byte from a table indexed by that sum.
- `decrement_call_and_branch`: Reads the word at offset 10 of the structure in a5, decrements it, pushes operands onto the stack, calls another routine, then increments the value and jumps back into another routine.
- `compare_word_and_store_aligned`: Loads a word at offset 19080 of a global variable and compares it against a register, returning when they differ.
- `call_routines_and_store`: Calls two routines through fixed addresses, divides the second result by eight and masks it, then stores the value at offset 19082 of the structure in a4 and decrements the word at offset 19086.
- `compare_and_select_value`: Saves d6, loads a word from offset 8 of the structure in a5, pushes several values onto the stack, calls a routine, restores the stack, compares the result with d5, and when the comparison is negative stores 1 in d0, otherwise it returns the original value.
- `call_and_decrement_counter`: Pushes several values onto the stack, calls a routine, restores the stack, stores a long into offset 19084 of a4, decrements the word at offset 19086, and returns zero when that word reaches zero.
- `compare_and_store_result`: Pushes a sequence of values onto the stack, calls another routine, then compares a value from the structure in a5 with a stack value and stores the resulting signed comparison result into a global variable before restoring the caller's registers.
- `store_and_restore_structure`: Stores the 16-bit value at offset -10 of the structure in a5 into the global variable at offset 19084 of a4, restores the caller's registers, unwinds the frame, and returns.
- `load_nine_nine_nine_and_restore_2`: Reads a word from offset 12 of the structure in a5 into d7, checks the word at offset 10 of a5, and when it is zero loads the constant 9992 into d0, restores the caller's registers from the stack, unwinds the frame, and returns.
- `compare_and_store_index`: Pushes two words onto the stack, calls a routine, and compares the result with a global variable at offset 19082 of a4, branching away when it is not greater.
- `decrement_and_push_word`: Decrements a word read from a global variable at offset 10 of a4, pushes that decremented value and the contents of d6 onto the stack, calls another routine, pops four words back off the stack, increments the counter, and jumps to a shared routine.
- `store_indexed_value_when_equal`: Compares a word at offset 1864 of the structure in a4 with d6 and branches away when they differ, otherwise adds 4 to a copy of d4, masks it with 7, stores the result at offset 1868, loads 1 into d0, and jumps to another routine.
- `index_and_check_table`: Reads a word at offset sixteen of a5, divides it by eight using a loop that counts down from seven, indexes a table at offset negative twenty four nine one four of a4, and branches to a caller routine when the result is zero.
- `call_and_load_maximum`: Pushes the value in d6 onto the stack, calls a routine at a fixed address, pops the value back, and loads nine thousand nine hundred ninety-two into d0 when the result is zero.
- `compare_and_load_large_value`: Reads the word at offset fourteen of the structure in a5 and compares it with d6.
- `compute_index_and_call`: Reads a word from a global variable at offset 10 of a5, decrements it, computes a value from d4 divided by two then multiplied by d7, masks the low three bits, pushes five words onto the stack, calls another routine, restores the stack pointer, increments the counter, and jumps to a shared routine.
- `compare_and_store_offset`: Compares a word at offset 4a88 of a4 with the value in d6, and otherwise loads d4 into d0, adds four, masks the low three bits, stores the result at offset 4a8c of a4, sets d0 to one, and jumps to a shared routine.
- `load_nine_nine_nine_and_restore`: Loads the constant 9992 into d0, restores the caller's registers from the stack, unwinds the frame, and returns.
- `setup_frame_and_test_word`: Sets up a frame and saves registers, tests a word from the structure in a5 and branches away when it is non-zero, otherwise it loads a constant into d0 and jumps to another routine.
- `call_and_index_byte_lookup`: Pushes a word at offset 19080 of a global variable and a word at offset 8 of the structure in a5, then calls a routine through a relative address and removes the arguments.
- `decrement_and_call_routine`: Decrements the word at offset 10 of the structure in a5, pushes it and the word at offset 8 onto the stack, calls a routine, then increments the decremented value and branches away.
- `compare_and_align_value`: Compares a word at offset 19088 of the structure in a4 with a register, and when they differ aligns a longword by four and stores the aligned value at offset 19092 of that structure.
- `call_routines_and_check_limit`: Pushes two values onto the stack, calls a routine, stores the result at offset 19082 of a4, pushes more values, calls another routine, and returns zero when the resulting value is at least 10000.
- `store_result_and_return`: Stores d4 at offset 19082 of a4, pushes several values including a word from offset 8 and a word from offset 10 of the structure in a5, calls a routine, restores the stack, restores d4 through d7, unlinks a5, and returns.
- `lookup_flag_and_store_index_2`: Reads a word from offset 12 of the structure in a5 into d7, checks the word at offset 10 of a5, and when it is nonzero reads a value from offset 8 of a5, adds it to a table entry selected by a computed index, then restores registers and returns.
- `lookup_flag_and_store_index`: Reads a 16-bit value from offset 8 of the structure in a5, adds it to a table entry selected by a computed index, then looks up a byte in a table using that sum and checks a flag byte; if the flag is set it stores a modified index into a global variable at offset 12 and returns 1, otherwise it returns.
- `decrement_and_push_word_2`: Decrements a word read from a global variable at offset 10 of a5, pushes that decremented value and the contents of d6 onto the stack, calls another routine, pops four words back off the stack, increments the counter, and jumps to a shared routine.
- `compute_and_store_index_2`: Compares a 16-bit value in d6 with a global variable and returns immediately if they differ, otherwise computes an index from the 16-bit value in d4 combined with the value in d7, masks it to a range of 0-7, and stores that byte into a global variable at offset 12.

The function list continues in: g2222-recovery-functions-2.md.
