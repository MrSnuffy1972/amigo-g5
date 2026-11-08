# Game 1747 (zany golf (1988)(electronic arts)[cr esi]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01747). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 59 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 105 of 105 named (26 author symbols), 105 of 105 described (14 by rule, 14 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 63 of 63 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 106865 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1618 of 1999 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 287820 |
| code | 7102 |
| data | 474419 |
| padding | 14019 |
| unused | 117760 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zany Golf (1988)(Electronic Arts)[cr ESI].adf:Zany | 15 | 1592 | 102 |
| Zany Golf (1988)(Electronic Arts)[cr ESI].adf:bootblock | 1 | 10 | 1 |
| QUICKSCORE.UNV (raw code file, load address unknown) | 1 | 0 | 0 |
| SPRITES8.SPR (raw code file, load address unknown) | 1 | 0 | 0 |
| FILE1 (raw code file, load address unknown) | 1 | 397 | 2 |
| HOLE6.REG (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf: file c/sigfile (header 93 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/SIGN9.UNV: high-entropy data file (7.64 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE8.UNV: high-entropy data file (7.78 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE6.UNV: high-entropy data file (7.69 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/SIGN5.UNV: high-entropy data file (7.65 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE4.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE2.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE0.UNV: high-entropy data file (7.81 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE9.UNV: high-entropy data file (7.78 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE7.UNV: high-entropy data file (7.79 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/SIGN6.UNV: high-entropy data file (7.61 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE5.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/MARQUEE.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE3.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE1.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/QUICKSCORE.UNV: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/SPRITES8.SPR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/FILE1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf:files/HOLE6.REG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[cr esi]/Zany Golf (1988)(Electronic Arts)[cr ESI].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.21)
- 4 never-loaded region(s), 106865 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `write_cia_drive_port` (Zany): The decompiler timed out, so only the listed facts are available. Kind: disk hardware/drive control routine. Pass path: The routine appears to be a low-level driver function for controlling the CIA (Complex Interface Adapter) chip to manipulate the disk drive (likely for custom track reading or motor control). In the context of Amiga software, routines that .
- `access_cia_ports_and_call` (Zany): The decompiler timed out, so only the listed facts are available. Kind: disk hardware/sync check. Pass path: the loop continues or the code proceeds to the next instruction after the call.
- `access_cia_ports_two_calls` (Zany): The decompiler timed out, so only the listed facts are available. Kind: disk hardware/timing check. Pass path: the loop continues or the code proceeds to u0__sub_1_0718.
- `access_ciab_port_two_calls` (Zany): The decompiler timed out, so only the listed facts are available. Kind: track/sector integrity and sync check. Pass path: The routine passes if the comparison `*(int *)(unaff_A6 + -0xc) == *(int *)(unaff_A6 + -4)` is true (no error flag set) AND the comparison `iVar2 != iVar1` is false (no discrepancy in expected vs actual data/count), allowing the code to ski.
- `access_ciab_port_three_calls` (Zany): The decompiler timed out, so only the listed facts are available. Kind: track/data integrity check. Pass path: the branch where iVar2 == iVar1 (skipping the while loop and the 15000 delay).
- `process_recursive_list` (Zany): Updates a global variable, performs arithmetic on a local structure, and iterates through a linked list or collection via recursion. Kind: recursive list/structure traversal for integrity or hardware validation. Pass path: The loop completes normally and the routine proceeds to the subsequent instructions (u0__sub_1_0718 and u0__sub_1_0700) without triggering a crash or infinite loop..
- `touch_ciab_port_five_callers` (Zany): The decompiler timed out, so only the listed facts are available. Kind: disk hardware/status check. Pass path: the code continues execution normally (the routine updates a status flag/variable in memory used for subsequent logic).
- `touch_ciab_port_six_callers` (Zany): The decompiler timed out, so only the listed facts are available. Kind: disk hardware/track check. Pass path: addq.l #an address, a7.
- `touch_ciab_port_then_call` (Zany): The decompiler timed out, so only the listed facts are available. Kind: disk hardware/port check. Pass path: addq.l #an address, a7.

## Functions


### Zany Golf (1988)(Electronic Arts)[cr ESI].adf:Zany

- `open_dos_library_at_entry`: The decompiler timed out, so only the listed facts are available.
- `verify_disk_signature`: Checks for a specific disk signature by calling a routine twice.
- `chain_three_calls_from_format`: The decompiler timed out, so only the listed facts are available.
- `pass_through_single_call`: The decompiler timed out, so only the listed facts are available.
- `pass_through_two_calls`: The decompiler timed out, so only the listed facts are available.
- `handler_entry_one_call`: The decompiler timed out, so only the listed facts are available.
- `pointer_entry_no_recorded_calls`: The decompiler timed out, so only the listed facts are available.
- `handler_entry_point_routine`: The decompiler timed out, so only the listed facts are available.
- `call_two_library_functions`: The decompiler timed out, so only the listed facts are available.
- `find_task_and_manage_interrupts`: The decompiler timed out, so only the listed facts are available.
- `handler_entry_no_recorded_calls`: The decompiler timed out, so only the listed facts are available.
- `refer_file1_text_and_call`: The decompiler timed out, so only the listed facts are available.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `reference_file1_path_library`: The decompiler timed out, so only the listed facts are available.
- `entry_with_no_recorded_uses`: The decompiler timed out, so only the listed facts are available.
- `handler_entry_two_calls`: The decompiler timed out, so only the listed facts are available.
- `DiskSig`: The decompiler timed out, so only the listed facts are available.
- `allocate_and_process_buffer_2`: Allocates two memory blocks and enters a nested loop structure to process data through multiple conditional routine calls.
- `allocate_and_process_buffer`: Allocates a memory block and enters a loop that performs multiple conditional calls to other routines while managing a counter.
- `make_eight_routine_calls`: The decompiler timed out, so only the listed facts are available.
- `make_four_routine_calls`: The decompiler timed out, so only the listed facts are available.
- `make_two_routine_calls`: The decompiler timed out, so only the listed facts are available.
- `validate_bounds_and_free_memory`: Performs a series of comparisons on values within a pointer-based data structure to select a status code, then calls three routines, frees two memory blocks, and returns the selected status code.
- `forward_one_call_routine`: The decompiler timed out, so only the listed facts are available.
- `fail_read_and_release_buffers`: Sets a local value to eighteen, calls two routines using a global variable and other parameters, frees two memory blocks, and calls a third routine with zero before returning eighteen.
- `release_buffers_and_fail`: Calls two routines using a global variable and a stored pointer, frees two memory blocks, and returns eighteen.
- `locate_sector_then_two_calls`: The decompiler timed out, so only the listed facts are available.
- `initialize_disk_io`: Allocates memory for standard I/O, opens a device, reads sectors into a buffer, and then shuts off the motor before freeing memory.
- `read_sectors_then_motor_off`: The decompiler timed out, so only the listed facts are available.
- `write_cia_drive_port`: The decompiler timed out, so only the listed facts are available.
- `single_call_routine_two_callers`: The decompiler timed out, so only the listed facts are available.
- `access_cia_ports_and_call`: The decompiler timed out, so only the listed facts are available.
- `access_cia_ports_two_calls`: The decompiler timed out, so only the listed facts are available.
- `make_two_calls_single_caller`: The decompiler timed out, so only the listed facts are available.
- `make_one_call_two_callers`: The decompiler timed out, so only the listed facts are available.
- `access_ciab_port_two_calls`: The decompiler timed out, so only the listed facts are available.
- `access_ciab_port_three_calls`: The decompiler timed out, so only the listed facts are available.
- `process_recursive_list`: Updates a global variable, performs arithmetic on a local structure, and iterates through a linked list or collection via recursion.
- `make_two_calls_two_callers`: The decompiler timed out, so only the listed facts are available.
- `touch_ciab_port_five_callers`: The decompiler timed out, so only the listed facts are available.
- `touch_ciab_port_six_callers`: The decompiler timed out, so only the listed facts are available.
- `call_stub_routines_only`: The decompiler timed out, so only the listed facts are available.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `touch_ciab_port_then_call`: The decompiler timed out, so only the listed facts are available.
- `access_ciaa_port_two_calls`: The decompiler timed out, so only the listed facts are available.
- `access_ciaa_port_one_caller`: The decompiler timed out, so only the listed facts are available.
- `test_byte_at_offset_one`: Loads a pointer from a global variable into a register, tests the byte at offset one of the pointed-to structure, and branches to another routine if that byte is zero; otherwise it returns.
- `make_three_calls_one_caller`: The decompiler timed out, so only the listed facts are available.
- `clear_record_flag_recursively`: Checks if a byte at a specific offset within a record is non-zero, and if so, sets it to zero before calling itself twice with the same parameter.
- `clear_record_flag_and_notify`: Clears the flag byte in the record named by the parameter, then passes that record to a helper twice.
- `FindSector`: The decompiler timed out, so only the listed facts are available.
- `branch_from_sector_search`: The decompiler timed out, so only the listed facts are available.
- `jump_to_find_sector`: The decompiler timed out, so only the listed facts are available.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a6, then returns.
- `make_two_calls_from_callers`: The decompiler timed out, so only the listed facts are available.
- `relay_single_unnamed_call`: A short routine of eight instructions, called from one unnamed routine, that makes a single call to another unnamed routine.
- `call_findsector_and_five_routines`: A 49-instruction routine called from one unnamed routine that calls FindSector together with five other unnamed routines.
- `invoke_execbase_plus_five_routines`: A 43-instruction routine called from two unnamed routines that calls the SecBase symbol and five other unnamed routines.
- `tail_jump_to_unnamed_routine`: A two-instruction stub called by one unnamed routine that passes control to one other unnamed routine.
- `run_three_step_shared_stub`: A three-instruction routine with no outgoing calls, reached from three unnamed routines.
- `SecBase`: The author symbol SecBase marks this fifteen-instruction routine, which is called from four unnamed routines and makes one call to another unnamed routine.
- `branch_tail_into_secbase`: A fifteen-instruction routine reached by a branch from the SecBase routine that ends with a tail branch back into SecBase.
- `relay_call_from_two_callers`: An eight-instruction routine reached from two unnamed routines and that makes one call to another unnamed routine.
- `dispatch_three_unnamed_calls`: A 19-instruction routine called from five unnamed routines that makes three calls to other unnamed routines.
- `chain_two_unnamed_calls`: A seven-instruction routine called from one unnamed routine that makes two calls to other unnamed routines.
- `forward_work_to_two_routines`: An eleven-instruction routine called from one unnamed routine that makes two calls to other unnamed routines.
- `evaluate_status_byte`: Compares bytes within a structure to specific values and stores a resulting status code in a local variable.
- `delegate_to_two_routines`: A nine-instruction routine called from one unnamed routine that makes two calls to other unnamed routines.
- `exit_three_instruction_leaf`: A three-instruction leaf routine with no outgoing calls, reached from one unnamed routine.
- `wrap_single_call_entry`: A 13-instruction routine that the author symbol table marks as an entry, called from two unnamed routines and making one call to another unnamed routine.
- `exit_four_instruction_leaf`: A four-instruction leaf routine with no outgoing calls, reached from one unnamed routine.
- `invoke_secbase_and_seven_routines`: A 58-instruction routine that the decoder found only as gap bytes, called from one unnamed routine.
- `call_two_gap_decoded_routines`: A 15-instruction routine that the decoder found only as gap bytes, called from one unnamed routine that makes two calls to other unnamed routines.
- `run_twelve_step_shared_leaf`: A twelve-instruction leaf routine with no outgoing calls, reached from four unnamed routines.
- `invoke_gap_decoded_routine`: A 28-instruction routine that the decoder found only as gap bytes, called from one unnamed routine that makes one call to another unnamed routine.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `_AllocMem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `_FreeMem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `_FindTask`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `_AllocSignal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `_FreeSignal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `_AddPort`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_RemPort`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `_OpenDevice`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `_DoIO`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `CreateStdIO`: The author-named CreateStdIO routine, eleven instructions long, called from one unnamed routine.
- `DeleteStdIO`: The author-named DeleteStdIO routine, fifteen instructions long, called from two unnamed routines.
- `ReadSecs`: The author-named ReadSecs routine, 21 instructions long, called from one unnamed routine.
- `WriteSecs`: The author-named WriteSecs routine, 21 instructions long.
- `FormatSecs`: The author-named FormatSecs routine, 24 instructions long, which makes calls to two unnamed routines.
- `handle_format_branch_calls`: An eight-instruction routine reached by a branch from FormatSecs that makes calls to two unnamed routines.
- `issue_io_request_once`: A five-instruction routine called from one unnamed routine that makes one call to the _DoIO library routine.
- `UpdateTD`: The author-named UpdateTD routine, 13 instructions long, whose only recorded call is to the _DoIO library routine.
- `MotorOff`: The author-named MotorOff routine, 13 instructions long, called from one unnamed routine.
- `DiskStatus`: The author-named DiskStatus routine, 16 instructions long, whose only recorded call is to the _DoIO library routine.
- `_CreatePort`: The author-named _CreatePort routine, 15 instructions long, called by CreateStdIO. It calls _AllocSignal and makes tail branches to two routines named _L6 and _L8, with a handler pointer referenced from data.
- `_L8`: The author-named _L8 routine, 16 instructions long, reached by a branch from _CreatePort.
- `_L7`: The author-named _L7 routine, 15 instructions long, reached by a branch from _L8.
- `_L6`: The author-named _L6 routine, six instructions long, reached by branches from _CreatePort, _L7 and _L8.
- `_DeletePort`: The author-named _DeletePort routine, 20 instructions long, called from DeleteStdIO. It calls _RemPort, _FreeSignal and _FreeMem, and a handler pointer is referenced from data.
- `_NewList`: The author-named _NewList routine, six instructions long, called from _L6 and making no further recorded calls.

### Zany Golf (1988)(Electronic Arts)[cr ESI].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### FILE1 (raw code file, load address unknown)

- `initialize_global_pointers`: Stores stack and register values into global variables and pointers before calling an indirect function.
- `configure_audio_and_dma_registers`: A 389-instruction routine reached by a pc-relative address reference and called from one unnamed routine.
