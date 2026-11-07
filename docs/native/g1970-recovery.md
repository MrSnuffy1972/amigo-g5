# Game 1970 (leisure suit larry 3 v1.003 (1989)(sierra)(disk 1 of 4)[h bttr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01970). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 143 of 143 named (0 author symbols), 143 of 143 described (31 by rule, 112 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 502 of 502 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3216 of 3216 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6830 |
| code | 10668 |
| data | 561763 |
| padding | 5507 |
| unused | 316416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 156 | 3 |
| ENDCLI | 2 | 117 | 2 |
| ECHO | 2 | 117 | 2 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:Type | 1 | 426 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:LARRY3 | 3 | 942 | 55 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:PROG | 66 | 29 | 2 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:AMIGASND.DRV | 3 | 1030 | 74 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |

## Cases handled

- leisure suit larry 3 v1.003 (1989)(sierra)(disk 1 of 4)[h bttr]/Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:RESOURCE.001: high-entropy data file (7.68 bits/byte), classified as packed data
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- leisure suit larry 3 v1.003 (1989)(sierra)(disk 1 of 4)[h bttr]/Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:PROG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `initialize_system_libraries_2`: Allocates memory and opens various libraries to perform DOS operations and data writes.
- `invoke_dos_write`: Calls a routine to perform a DOS write operation.
- `write_dos_data`: Performs a DOS write operation using a library call.

### ENDCLI

- `initialize_system_environment`: This routine serves as the primary entry point for the executable.
- `call_initialize_system_libraries`: This routine performs a single call to another routine.

### ECHO

- `initialize_system_libraries_echo`: Performs library initialization and memory allocation through a single sub-routine call.
- `execute_library_initialization`: Calls a routine that allocates memory and opens various libraries for DOS operations and data writes.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:Type

- `format_output_text`: Processes text patterns and numeric values to generate formatted strings.
- `format_output_string`: Formats strings and numbers using specific text patterns and hexadecimal characters through multiple library calls.

### Disk-Validator

- `execute_program_entry`: Serves as the initial entry point for the program.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:LARRY3

- `start_program`: The initial entry point of the program consisting of a single instruction.
- `initialize_startup_sequence`: Executes a short sequence of instructions related to a startup process.
- `initialize_system_libraries`: Executes a startup sequence and performs multiple library function calls via offsets and vectors.
- `execute_system_and_dos_wrappers`: This routine executes a sequence of system library initializations and file system operations including directory retrieval, file handle management, and input/output requests.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `perform_dos_operations`: Uses a DOS library to retrieve directory information, manage file handles, and execute various library functions using global base addresses.
- `retrieve_directory_info`: Retrieves the current directory via a DOS library function and calls other library routines using a global base.
- `execute_single_instruction`: An extremely short routine consisting of a single instruction.
- `initialize_system_libraries_4`: Performs system library initializations and file system operations using DOS library functions.
- `execute_two_instructions`: An extremely short routine consisting of two instructions.
- `execute_library_tasks`: Performs several short instruction sequences and calls a library function via a global base address.
- `execute_short_task`: This routine calls another routine that executes a thirteen-instruction sequence.
- `perform_short_task`: Executes a brief sequence of thirteen instructions.
- `perform_short_task_2`: Executes a brief sequence of twelve instructions.
- `manage_window_resources`: Interacts with icon library and performs DOS open operations and various library function calls.
- `return_from_subroutine`: This routine consists of a single instruction that returns control to a caller via a program counter relative jump.
- `invoke_file_release`: Calls a routine that closes a file handle and performs DOS unlock operations.
- `release_file_resources`: Closes a file handle and performs DOS unlock operations using several library functions.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: Calls a routine to close a file and returns the result.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `get_current_directory`: Calls a wrapper that retrieves the current directory via a DOS library function.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_offset_414`: Loads a library base from a global variable and calls a function at a specific offset.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_offset_198`: Executes a library function using a base address retrieved from a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_offset_294`: Performs a call to a library function at a specific offset from a global base address.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_offset_372`: Invokes a library function by accessing a base address stored in a global variable.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_offset_552`: Calls a function within a library using an offset from a global base pointer.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_offset_384`: Uses a global variable to locate a library base and calls a function at a specific offset.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_offset_78`: Calls a library function at a specific offset from a library base.
- `call_library_offset_198_2`: Executes a library routine using a specific offset from a library base.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:PROG

- `execute_program_entry_2`: Performs a sequence of instructions including memory freeing and relative address execution.
- `execute_eight_instructions`: Performs a sequence of eight instructions and is accessed via a relative address.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:AMIGASND.DRV

- `initialize_system_libraries_3`: This routine serves as the primary entry point for the executable.
- `copy_memory_blocks`: Performs memory block transfers by calling routines to move data between locations.
- `copy_memory_block_2`: Copies a block of data from one memory location to another using a loop.
- `transfer_memory_block`: This routine invokes a memory copy operation to move a block of data from one location to another.
- `copy_memory_block`: The routine copies a block of data from one memory location to another using a loop.
- `execute_fourteen_instructions`: A routine consisting of fourteen instructions that is called by one other routine and entered via a pointer in data.
- `execute_six_instructions`: A routine consisting of six instructions that is called by one other routine and entered via a pointer in data.
- `execute_tail_branching_sequence`: This routine executes ten instructions and performs a tail branch to another routine.
- `execute_two_instructions_2`: A routine consisting of two instructions that is entered via a pointer in data and called by one other routine.
- `manage_audio_subroutines`: Executes a sequence of tasks including memory block transfers, library function execution, and audio hardware configuration involving DMA and interrupt settings.
- `call_short_routine`: This routine executes five instructions and calls another routine that consists of ten instructions.
- `invoke_audio_subroutines`: Executes a sequence of three subroutines that manage audio channel zero registers, a short routine, and DMA and interrupt settings.
- `configure_audio_channel`: Executes a sequence of subroutines that manage audio channel zero registers and interrupt settings.
- `execute_nested_subroutines`: This routine executes two separate subroutines in sequence.
- `execute_short_routine_2`: This routine consists of ten instructions and calls another routine consisting of nine instructions.
- `configure_audio_hardware_parameters`: This routine modifies DMA control and interrupt enable registers, initializes three specific audio channels, and executes short sequences of nested instructions.
- `execute_nested_calls`: Calls two different subroutines to perform short sequences of instructions.
- `execute_two_instructions_3`: This routine executes two instructions by calling another routine that performs eight instructions.
- `execute_branching_sequence_2`: This routine consists of four instructions and performs a conditional branch.
- `execute_branching_sequence`: Executes a series of instructions and subroutines including a conditional branch.
- `call_subroutine`: Executes eight instructions and calls another routine.
- `execute_sequence_and_branch`: This routine executes six instructions and then calls another routine.
- `execute_five_instructions`: This routine executes five instructions and then calls another routine.
- `execute_four_instructions`: This routine executes four instructions and then calls another routine.
- `call_unnamed_routine`: Performs four instructions and calls another routine.
- `execute_multiple_subroutines`: Calls a sequence of three different subroutines.
- `call_multiple_subroutines`: Executes a routine of fourteen instructions and then calls a routine of eighteen instructions.
- `execute_fourteen_instructions_2`: Executes a routine consisting of fourteen instructions.
- `execute_instruction_sequence`: A short routine consisting of ten instructions that is entered via a pointer in a data table.
- `execute_two_instructions_4`: A two-instruction routine entered via a pointer in a data table.
- `execute_eight_instructions_2`: A routine consisting of eight instructions that is entered via a pointer in data and called by four other routines.
- `toggle_interrupt_state`: Modifies DMA control and interrupt enable registers.
- `execute_nine_instructions`: A routine consisting of nine instructions that is entered via a pointer in data and a handler pointer in data and is called by two other routines and a pointer in hunk 0 data.
- `toggle_hardware_interrupt_state`: This routine modifies the DMA control and interrupt enable registers.
- `modify_interrupt_registers`: Updates DMA control and interrupt enable registers through a three-instruction sequence.
- `execute_three_instructions`: A routine consisting of three instructions that is entered via a pointer in data and a handler pointer in data and is called by a pointer in hunk 0 data.
- `configure_audio_channel_zero`: A routine of eighteen instructions that disables and enables interrupts, modifies audio channel zero registers, and manages DMA and interrupt enable bits.
- `configure_audio_channel_one`: A routine of eighteen instructions that disables and enables interrupts, modifies audio channel one registers, and manages DMA and interrupt enable bits.
- `configure_audio_channel_two`: A routine of eighteen instructions that disables and enables interrupts, modifies audio channel two registers, and manages DMA and interrupt enable bits.
- `configure_audio_hardware`: Disables and enables interrupts while updating audio channel parameters, volume, and DMA and interrupt control registers.
- `toggle_dma_and_interrupts`: Modifies the DMA control and interrupt enable registers.
- `modify_dma_interrupts_wrapper`: Modifies DMA control and interrupt enable registers and calls another routine.
- `modify_dma_interrupts_3`: Modifies DMA control and interrupt enable registers and then executes a thirteen-instruction routine.
- `modify_dma_interrupts_2`: Modifies DMA control and interrupt enable registers and then calls another routine.
- `modify_dma_interrupts`: Modifies the DMA control and interrupt enable registers and calls another routine.
- `execute_short_routine`: A routine consisting of nine instructions.
- `execute_medium_routine`: A routine consisting of thirteen instructions.
- `initialize_audio_channels`: Calls routines to configure three specific audio channels and a general audio hardware setup.
- `execute_minimal_routine_wrapper_2`: This routine calls another routine via a wrapper.
- `execute_minimal_routine_wrapper`: A seven-instruction routine that calls a six-instruction routine.
- `execute_minimal_routine`: A routine consisting of six instructions.
- `update_indirect_value`: Modifies a value at a memory location pointed to by a global variable.
- `update_pointer_value_3`: Modifies a value at a memory location pointed to by a global variable.
- `update_pointer_value`: The routine modifies a value at a memory location pointed to by a global variable.
- `write_byte_via_pointer`: This routine stores a single byte from a register into a memory location specified by another register.
- `store_byte_to_memory`: Stores a single byte from a register into a memory location pointed to by another register.
- `write_byte_to_memory`: Stores a single byte from a register into a memory location pointed to by another register.
- `update_data_pointer`: Updates a value at a memory location using a pointer retrieved from a data structure.
- `update_memory_pointer`: Modifies a value at a memory location pointed to by a data pointer.
- `update_pointer_value_2`: The routine modifies a value at a memory location pointed to by a data pointer.
- `setup_interrupt_handler`: Frees memory, sets an interrupt vector, and modifies DMA and interrupt enable registers.
- `transfer_audio_data`: Performs memory block transfers and calls another short routine via pointers stored in data.
- `modify_dma_interrupts_4`: Modifies DMA control and interrupt enable registers before calling another routine.
- `invoke_interrupt_and_subroutine`: Calls a routine to modify DMA control and interrupt enable registers, then calls another routine consisting of eighteen instructions.
- `toggle_interrupt_state_2`: Modifies the DMA control and interrupt enable registers through a sub-routine.
- `toggle_hardware_interrupt_state_2`: This eight-instruction routine calls a function that modifies the DMA control and interrupt enable registers.
- `call_longer_routine`: Executes a routine consisting of eighteen instructions.
- `execute_longer_routine`: A routine consisting of eighteen instructions.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies 4 to d2, then returns.
- `configure_audio_hardware_2`: Allocates memory, sets an interrupt vector, and configures multiple audio channels by updating hardware registers and memory pointers.
- `execute_extended_task`: Calls a routine consisting of forty instructions.
- `execute_extended_routine`: A routine consisting of forty instructions.
- `call_twelve_instruction_executor`: This routine calls another routine that executes a sequence of twelve instructions.
- `execute_twelve_instructions`: Executes a sequence of twelve instructions.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[h BTTR].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
