# Game 1217 (leisure suit larry 3 v1.003 (1989)(sierra)(disk 1 of 4)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01217). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 141 of 141 named (0 author symbols), 141 of 141 described (31 by rule, 110 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 502 of 502 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2790 of 2790 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6598 |
| code | 9418 |
| data | 559701 |
| padding | 4443 |
| unused | 321024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| type | 2 | 156 | 3 |
| ENDCLI | 2 | 117 | 2 |
| ECHO | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:LARRY3 | 3 | 942 | 55 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:PROG | 66 | 29 | 2 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:AMIGASND.DRV | 3 | 1030 | 74 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |

## Cases handled

- leisure suit larry 3 v1.003 (1989)(sierra)(disk 1 of 4)[a2]/Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:RESOURCE.001: high-entropy data file (7.68 bits/byte), classified as packed data
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- leisure suit larry 3 v1.003 (1989)(sierra)(disk 1 of 4)[a2]/Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:PROG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### type

- `manage_dos_file_operations`: This entry-point routine executes a sequence of instructions to perform DOS write operations and manages memory allocation using system libraries.
- `perform_dos_write_operation`: executes a sequence of instructions that uses a DOS library function to write data.
- `execute_write_operation`: Performs a write operation using a DOS library function via a sequence of six instructions.

### ENDCLI

- `execute_dos_write_sequence`: This entry-point routine executes a sequence of instructions to perform certain DOS write operations and manages memory allocation using a system library.
- `perform_dos_write_sequence`: Executes a two-instruction sequence that performs certain DOS write operations and manages memory allocation.

### ECHO

- `execute_dos_write_sequence_2`: This routine serves as a program entry point that executes a sequence of instructions to perform certain DOS write operations and manages memory allocation using a system library.
- `perform_write_operations`: This routine executes a sequence of instructions to perform DOS write operations and manages memory allocation using a system library.

### Disk-Validator

- `initialize_program_entry`: The routine performs a large sequence of instructions that establishes the initial execution state.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:LARRY3

- `execute_startup_logic_3`: Executes a single instruction that invokes startup logic and manages library-related file or direct operations.
- `handle_startup_logic`: Executes a short sequence of fourteen instructions and references a startup string.
- `execute_startup_logic`: Executes startup logic, loads a new segment, and invokes multiple library functions using offsets from a global base address.
- `execute_startup_logic_2`: Executes startup logic, loads a new segment, and invokes various library functions or wrappers related to file handling and directory retrieval.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `invoke_dos_library_functions`: Uses a global library base to invoke various library functions, including those for file handling, directory retrieval, and process creation.
- `invoke_library_functions`: Calls a DOS library wrapper to retrieve the current directory and invokes two library functions using offsets from a global library base.
- `execute_single_instruction`: Executes a single instruction that is targeted via a relative or absolute address.
- `execute_startup_logic_and_dos_operations`: This routine executes a sequence of instructions that invokes startup logic, loads a new segment, and manages library-related file or directory operations.
- `execute_two_instructions`: Executes a sequence of two instructions called by another routine.
- `execute_library_stubs`: Executes several short instruction sequences and calls a library function using an offset from a global base address.
- `execute_thirteen_instructions_2`: Executes a fixed sequence of thirteen instructions.
- `execute_thirteen_instructions`: Executes a sequence of thirteen instructions called by two different routines.
- `execute_twelve_instructions`: Executes a sequence of twelve instructions called by another routine.
- `invoke_library_functions_2`: Invokes several library functions using offsets from a global base address and calls a DOS open wrapper.
- `empty_routine`: The routine contains a single instruction and performs no operations on any registers or memory.
- `invoke_file_closing`: calls a routine that closes a file handle and triggers several library functions using offsets from a global base address.
- `handle_file_closing`: Closes a file handle and invokes several library functions using offsets from a global base address.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_handle`: This routine calls a wrapper for the operating system close function.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `get_current_directory`: Executes a single instruction that calls a wrapper for a DOS library function to retrieve the current directory.
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
- `call_library_offset_414`: Calls a library function using an offset from a base address stored in a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `invoke_library_function`: Invokes a library function using an offset relative to a base address stored in a global variable.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `invoke_library_function_2`: This routine calls a library function located at a fixed offset relative to a base address stored in a global variable.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_function`: This routine invokes a function from a library by accessing a base address stored in a global variable and applying a fixed offset.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `invoke_library_function_3`: loads a library base from a global variable and executes a library function at a fixed offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `invoke_library_function_4`: executes a single instruction that calls a library function via a base address stored in a global variable minus a fixed offset.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_offset_78`: Calls a library function using an offset from a base address stored in a global variable via a two-instruction sequence.
- `call_library_offset_198`: Calls a library function using an offset from a base address stored in a global variable via a three-instruction sequence.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:PROG

- `execute_program_entry`: Executes a sequence of instructions and calls a memory free function.
- `execute_eight_instructions`: Executes a sequence of eight instructions and is entered via a relative or even-address-taken mechanism.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:AMIGASND.DRV

- `perform_dos_write_operations`: This routine executes a sequence of instructions to perform certain disk write operations and manages memory allocation.
- `update_global_pointer_from_data`: This routine updates a global pointer by loading a new address from a data pointer.
- `update_global_pointer`: Loads a new address from a data pointer and updates a global pointer.
- `update_pointer_from_data_2`: Updates a global pointer by loading a new address from a data pointer.
- `update_pointer_from_data`: loads a new address from a data pointer and updates a global pointer.
- `execute_fourteen_instructions`: A routine consisting of fourteen instructions that is called via a pointer in data.
- `execute_six_instructions`: A routine consisting of six instructions that is called via a pointer in data.
- `execute_tail_branch_sequence`: This routine performs ten instructions and concludes by calling another routine via a tail branch.
- `execute_two_instructions_2`: A routine consisting of two instructions that is accessed via a pointer in data.
- `update_global_pointer_and_execute_stub`: Updates a global pointer with a new address from a data pointer and executes library stubs or a short routine via a pointer.
- `invoke_short_routine`: Executes a short routine via a pointer located in a data structure.
- `modify_audio_registers`: modifies audio channel zero registers and interrupt or dmaenable bits before executing a short routine via a pointer in a data structure.
- `configure_audio_and_execute_routine`: Modifies audio channel zero registers and interrupt or dmaenable bits before executing a short routine via a pointer in a data structure.
- `execute_small_routine_3`: Executes a short routine via a pointer in a data structure and modifies hardware interrupt and dmaenable bits.
- `execute_small_routine_2`: executes a short routine via a pointer located in a data structure.
- `execute_indirect_subroutine`: Calls another routine or a sequence of instructions by dereferencing a pointer stored in a data area.
- `call_indirect_from_data_2`: Dereferences a pointer from a data area to call another routine or a sequence of instructions.
- `execute_two_instructions_3`: executes a sequence of two instructions.
- `execute_small_pointer_routine`: Executes a short routine that is accessed via a pointer in a data area.
- `jump_to_indirect_address`: Loads an address from a pointer in a data area and jumps to it while modifying certain hardware registers.
- `execute_unnamed_routine`: This routine contains eight instructions and is called via a pointer in a data area.
- `execute_indirect_jump`: Loads an address from a pointer in a data area and jumps to it, while also modifying certain hardware registers.
- `execute_indirect_pointer`: Loads an address from a pointer stored in a data area and jumps to that address.
- `call_indirect_from_data`: This routine loads a value from a pointer in a data area and calls another routine.
- `dereference_pointer_and_call`: The routine loads a value from a pointer located in a data area and uses that value to call another routine.
- `execute_subroutine_via_pointer_2`: Loads a routine address from a pointer in a data area and executes a sequence of instructions.
- `execute_subroutine_via_pointer`: Executes a sequence of fourteen instructions by loading a routine address from a pointer stored in a data area.
- `execute_fourteen_instructions_2`: This routine executes a sequence of fourteen instructions and is called via a pointer stored in a data area.
- `execute_indirect_pointer_sequence`: This ten-instruction routine is invoked via a pointer stored in a data area.
- `update_interrupt_status_2`: Modifies certain hardware registers, specifically the interrupt enable and dmaenable bits, and calls another routine.
- `execute_eight_instructions_2`: A routine consisting of eight instructions that is accessed via a pointer in data.
- `update_interrupt_status`: This routine modifies certain hardware registers, specifically the interrupt enable and dmaenable bits.
- `execute_nine_instructions`: A routine consisting of nine instructions that is called via a pointer in data or a handler pointer in data.
- `update_interrupt_hardware`: Modifies the interrupt enable and dmaenable bits within certain hardware registers.
- `update_interrupt_enable_bits`: Modifies the interrupt enable and dmaenable bits within certain hardware registers.
- `execute_three_instructions`: A routine consisting of three instructions that is accessed via a pointer in data or a handler pointer in data.
- `configure_audio_channel_zero`: A routine that disables and enables interrupts, modifies DMA or interrupt enable settings, and writes to enough audio channel zero registers to configure it.
- `configure_audio_channel_one`: A routine that disables and enables interrupts, modifies DMA or interrupt enable settings, and writes to enough audio channel one registers to configure it.
- `configure_audio_channel_two`: A routine that disables and enables interrupts, modifies DMA or interrupt enable settings, and writes to enough audio channel two registers to configure it.
- `configure_audio_channel_three`: A routine that disables and enables interrupts, modifies DMA or interrupt enable settings, and writes to enough audio channel three registers to configure it.
- `update_hardware_interrupts`: modifies the interrupt enable bit and the dmaenable bit within the certain hardware registers.
- `update_interrupt_control_bits`: Modifies certain hardware register bits and calls another routine or a routine via a pointer in a data structure.
- `modify_hardware_interrupt_settings_2`: Modifies certain hardware register bits and calls another routine or a routine via a pointer in a data structure.
- `modify_hardware_interrupt_settings`: Modifies certain hardware register bits and calls another routine.
- `modify_hardware_interrupts`: Modifies certain hardware register bits and calls another routine.
- `execute_small_routine`: a short routine of nine instructions called via a pointer in a data structure.
- `execute_medium_routine`: a routine of thirteen instructions called via a pointer in a data structure.
- `configure_multiple_audio_channels`: This routine configures several audio channels by modifying DMA or interrupt settings and calling a large routine via a pointer in a data structure.
- `execute_short_sequence`: This routine executes a short sequence of instructions located via a pointer in a data structure.
- `execute_short_routine_2`: executes a short sequence of instructions located via a pointer in a data structure.
- `execute_short_routine`: a routine of six instructions called via a pointer in a data structure.
- `modify_indirect_target`: Updates a value at a location pointed to by a pointer in the data.
- `update_pointer_target`: Updates a value at a location pointed to by a pointer in the data.
- `update_pointer_value`: Updates a value at a location pointed to by a pointer in the data.
- `update_memory_value`: This routine updates a value at a memory location pointed to by a pointer in the data.
- `update_memory_location`: Updates a value at a memory location pointed to by a pointer in the data.
- `update_pointer_value_2`: updates a value at a memory location pointed to by a pointer in the data.
- `update_pointer_byte`: stores a byte from a register into a memory location pointed to by a data pointer.
- `write_byte_to_pointer_2`: Stores a byte from a register into a memory location pointed to by a data pointer.
- `write_byte_to_pointer`: stores a byte from a register into a memory location pointed to by a value in a data pointer.
- `update_interrupt_state`: Frees memory and sets an interrupt vector before calling a routine that modifies hardware interrupt and dmaenable bits.
- `update_pointer_and_execute_sequence`: Updates a global pointer by loading a new address from a data pointer and executes a sequence of instructions.
- `execute_unnamed_sequence`: Executes a routine called via a pointer in a data structure which then calls another routine.
- `update_hardware_and_execute_indirect`: Modifies the interrupt enable and dmaenable bits within certain hardware registers and calls another routine via a pointer located in a data structure.
- `modify_interrupt_and_dma_state`: modifies the interrupt enable bit and the dmaenable bit within certain hardware registers.
- `modify_hardware_interrupts_2`: Calls a routine that modifies the interrupt enable bit and the dmaenable bit within certain hardware registers.
- `invoke_longer_routine`: Calls an oridinary routine of eighteen instructions via a pointer located in a data structure.
- `execute_longer_routine`: a routine of eighteen instructions called via a pointer in a data structure.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies 4 to d2, then returns.
- `configure_audio_channels`: Allocates memory and sets an interrupt vector, then configures audio channel registers and updates memory or pointer values.
- `execute_large_routine_2`: Executes a routine of forty instructions that is called via a pointer in a data structure.
- `execute_large_routine`: a routine of forty instructions called via a pointer in a data structure.
- `invoke_medium_routine`: Executes a routine of twelve instructions that is called via a pointer in a data structure.
- `execute_medium_routine_2`: a routine of twelve instructions called via a pointer in a data structure.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)(Disk 1 of 4)[a2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
