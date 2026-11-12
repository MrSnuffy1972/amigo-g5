# Game 2217 (tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02217). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802384 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 18 of 18 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 211 of 211 named (0 author symbols), 211 of 211 described (41 by rule, 90 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 26 of 26 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 26537 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5671 of 5671 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 357278 |
| code | 18264 |
| data | 1074730 |
| padding | 9584 |
| unused | 342528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| More | 19 | 2321 | 79 |
| Assign | 2 | 435 | 7 |
| Dir | 2 | 419 | 11 |
| Lab | 1 | 2 | 1 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 601 | 42 |
| Makedir | 2 | 211 | 4 |
| CD | 2 | 282 | 5 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 413 | 15 |
| IconX | 12 | 806 | 37 |
| Disk-Validator | 1 | 10 | 2 |
| Disk-Validator | 1 | 10 | 2 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf:bootblock | 1 | 11 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |
| TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf:pgame.BOLT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file C/Type (header 2284 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file C/EndCLI (header 696 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file C/Echo (header 992 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file TipOff.BOLT (header 154282 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file S/paranoimia (header 6920 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file S/startup-sequence (header 190 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file S/text2 (header 518 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file bb (header 169756 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: file sound.BOLT (header 70058 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: file tdat (header 12544 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: file Fluff.BOLT (header 373824 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: file seas (header 21948 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf:pgame.BOLT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 5-8 (bytes 2560-4608) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.27)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 220-224 (bytes 112640-115200) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.92)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 231-351 (bytes 118272-180224) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.88)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 360-362 (bytes 184320-185856) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.88)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 462-467 (bytes 236544-239616) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.57)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 473-780 (bytes 242176-399872) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.27)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 810-811 (bytes 414720-415744) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.94)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 814-814 (bytes 416768-417280) outside the filesystem hold data, not code (opcode density 0.016, entropy 3.47)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 826-827 (bytes 422912-423936) outside the filesystem hold data, not code (opcode density 0.039, entropy 5.74)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 834-835 (bytes 427008-428032) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.73)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 863-865 (bytes 441856-443392) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.82)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 888-888 (bytes 454656-455168) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.45)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 892-892 (bytes 456704-457216) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.48)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 896-896 (bytes 458752-459264) outside the filesystem hold data, not code (opcode density 0.031, entropy 6.10)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 924-924 (bytes 473088-473600) outside the filesystem hold data, not code (opcode density 0.012, entropy 4.54)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 946-946 (bytes 484352-484864) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.76)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 968-978 (bytes 495616-501248) outside the filesystem hold data, not code (opcode density 0.001, entropy 7.07)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 996-996 (bytes 509952-510464) outside the filesystem hold data, not code (opcode density 0.027, entropy 5.32)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1006-1006 (bytes 515072-515584) outside the filesystem hold data, not code (opcode density 0.031, entropy 6.84)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1008-1008 (bytes 516096-516608) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.75)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1397-1402 (bytes 715264-718336) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.30)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 1 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf: blocks 1408-1759 (bytes 720896-901120) outside the filesystem hold data, not code (opcode density 0.037, entropy 6.43)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 2-472 (bytes 1024-242176) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.91)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 893-893 (bytes 457216-457728) outside the filesystem hold data, not code (opcode density 0.039, entropy 6.62)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 927-934 (bytes 474624-478720) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.01)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 936-936 (bytes 479232-479744) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.16)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 940-940 (bytes 481280-481792) outside the filesystem hold data, not code (opcode density 0.020, entropy 7.17)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1309-1318 (bytes 670208-675328) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.84)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1320-1363 (bytes 675840-698368) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.27)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1367-1374 (bytes 699904-704000) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.47)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1463-1471 (bytes 749056-753664) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- tv sports basketball (1990)(cinemaware)[cr pna][v saddam-saddam ii](disk 2 of 2)/TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf: blocks 1474-1759 (bytes 754688-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.26)
- 2 never-loaded region(s), 26537 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 3 packed track region(s), 3072 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### More

- `start_program_open_libraries`: Program entry that opens the DOS and Exec library calls, finds the current task, allocates memory and waits for a port message.
- `release_libraries_and_memory`: Closes a DOS file and a library, frees allocated memory and replies to a message port while the task is protected.
- `display_paged_file_contents`: Pages through a file on screen, showing prompts for quitting, end of file, file not found and binary-content warnings, and an error when a window cannot be opened.
- `relay_call_to_unnamed_routine`: A 16-instruction handler reached through a data pointer that makes one unnamed call.
- `dispatch_two_unnamed_calls`: A 41-instruction handler reached through a data pointer that makes two unnamed calls.
- `allocate_absolute_memory_block`: Calls the Exec absolute-address allocation stub and a library vector at offset minus 306, plus five unnamed routines.
- `dispatch_conditional_callback_chain`: Checks if a longword at offset 96 of a structure pointed to by a2 is greater than zero; if so, it calls a routine via a data pointer and then calls a rescheduling routine.
- `show_pager_help_prompt`: Handles pager messages, including a find-previous-page status, a press-h-for-help prompt, a can't-reopen-file error and an editor lookup through the ENV:EDITOR variable.
- `dispatch_four_unnamed_calls`: An address-taken routine of 34 instructions that makes four unnamed calls and is reached from two unnamed routines.
- `reschedule_then_call_routine`: Calls the Exec reschedule stub and one unnamed routine.
- `coordinate_two_unnamed_calls`: A 51-instruction handler that makes two unnamed calls and is called by two unnamed routines.
- `show_more_page_prompt`: A 51-instruction routine that references the text 'More' and makes five unnamed calls.
- `report_search_progress`: Prints the searching progress message with a name argument, using the text for a searching status, and makes six unnamed calls.
- `sequence_five_unnamed_calls`: A 32-instruction routine that makes five unnamed calls and is called by one unnamed routine.
- `call_two_unnamed_routines_first`: A 15-instruction routine that makes two unnamed calls and is called by one unnamed routine.
- `relay_two_unnamed_calls`: A 17-instruction routine that makes two unnamed calls and is called by one unnamed routine.
- `execute_subroutines`: This routine calls a routine that processes a pager request block and a routine that raises an exception after a check.
- `process_pager_request_block`: A 134-instruction routine called by three unnamed routines that makes two unnamed calls.
- `wrap_two_unnamed_calls`: A 12-instruction routine whose address is taken by a PC-relative load, called by four unnamed routines, that makes two unnamed calls.
- `wrap_single_unnamed_call`: A 12-instruction routine called by four unnamed routines that makes one unnamed call.
- `relay_call_via_data_pointer`: A 16-instruction routine reached through a data pointer and by three unnamed routines, which makes one unnamed call.
- `print_concatenated_text_pair`: Prints text built from two string fields using the text pattern for two string arguments, and makes one unnamed call.
- `raise_exception_and_return`: Calls the Exec exception stub and returns its result.
- `invoke_library_vector_entry`: Calls a library function at offset minus 624 from a library base held in a global, plus one unnamed routine.
- `schedule_then_call_library_vector`: A 36-instruction routine called by three unnamed routines that calls the Exec schedule stub and a library function at offset minus 624.
- `raise_exception_after_check`: A 131-instruction routine called by nine unnamed routines that calls the Exec exception stub and one unnamed routine.
- `relay_one_call_checked`: A 20-instruction routine called by two unnamed routines that makes one unnamed call.
- `stub_short_routine_no_calls`: A six-instruction routine called by one unnamed routine with no listed calls.
- `process_routine_without_listed_calls`: An 18-instruction routine called by two unnamed routines with no listed calls.
- `process_medium_body_no_calls`: A 36-instruction routine called by one unnamed routine with no listed calls.
- `invoke_library_function_276`: A 15-instruction routine called by three unnamed routines that calls a library function at offset 276.
- `enable_and_call_library_vectors`: An 81-instruction routine called by four unnamed routines that calls the Exec exit-interrupt and enable stubs, library functions at offsets 132, 138, 210 and 414, and six unnamed routines.
- `invoke_library_vector_294`: A 19-instruction routine called by three unnamed routines that calls a library function at offset 294 and one unnamed routine.
- `call_three_library_vectors`: A 37-instruction routine called by one unnamed routine that calls library functions at offsets 198, 210 and 294 and one unnamed routine.
- `call_five_library_vectors`: A 62-instruction routine called by two unnamed routines that calls library functions at offsets 198, 210, 366, 372 and 384, and two unnamed routines.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_switch`: Library stub: loads its arguments and the exec library base, calls Switch and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_alloc_abs`: Library stub: loads its arguments and the exec library base, calls AllocAbs and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `forward_common_call_stub`: A 10-instruction routine called by nine unnamed routines that makes one unnamed call and nothing else.
- `schedule_task_and_return`: A 19-instruction routine called by five unnamed routines that calls the Exec schedule stub and one unnamed routine.
- `format_text_and_reschedule`: A 22-instruction routine called by one unnamed routine that calls the Exec RawDoFmt text formatting function, the reschedule stub and a tiny routine.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `process_block_two_callers`: A 29-instruction routine called by two unnamed routines with no listed calls.
- `process_small_block_pair`: A 16-instruction routine called by two unnamed routines with no listed calls.
- `forward_single_unnamed_call`: A six-instruction routine that makes one unnamed call and has no other listed behaviour.
- `call_helper_return_register`: Pushes a data register onto the stack, shifts two data registers for a helper, calls that helper, moves the helper's result into a data register, restores the saved register, and returns.
- `negate_result_on_flag`: Handles the signs of two values, calls a helper, and negates the result when the flag the helper leaves in an address register is nonzero before returning.
- `relay_call_two_callers`: A 19-instruction routine called by two unnamed routines that makes one unnamed call.
- `format_message_with_rawdofmt`: A nine-instruction routine that calls the Exec RawDoFmt text formatting function and one unnamed routine.
- `stub_two_instruction_return`: A two-instruction routine called by one unnamed routine with no listed calls, so it has no behaviour to recover.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `invoke_library_vectors_in_block`: A 48-instruction routine called by one unnamed routine that calls library functions at offsets 198, 294, 330, 336 and 354, and one unnamed routine.
- `invoke_three_library_vectors`: A 20-instruction routine called by one unnamed routine that calls library functions at offsets 210, 336 and 360.
- `process_block_single_caller`: A 25-instruction routine called by one unnamed routine with no listed calls.
- `stub_brief_routine_single_caller`: A six-instruction routine called by one unnamed routine with no listed calls.

### Assign

- `process_block_two_unnamed_calls`: A 115-instruction routine called by two unnamed routines that makes two unnamed calls.
- `stub_forward_call_only`: A two-instruction routine that makes one unnamed call and nothing else.
- `update_entity_spatial_data`: Calculates and stores new coordinate offsets in a structure, clears two status fields, and calls update_entity_state_and_linked_list.
- `update_entity_state_and_linked_list`: Updates an entity's state by calling another routine with various parameters, manages a linked list by traversing and modifying pointers, and performs conditional updates to structure offsets based on comparison results.
- `update_entity_state_and_index`: Retrieves an index from a table, calls another routine, and if the index is zero, updates a structure's pointer and checks specific flags to potentially update a value at offset 476 in a structure with the constant 20.
- `update_entity_linked_list_and_state`: Swaps two indices in a table, performs a series of conditional calls to another routine, and manages a linked list by updating pointers and shifting elements.
- `update_and_execute_state`: Updates several values in a structure pointed to by a1 using offsets derived from a1 and a0, performs multiple conditional calls to a routine in a5 with varying parameters, and finally jumps to a routine in a6.

### Dir

- `initialize_library_access`: This routine calls a routine that opens libraries, finds a task, and allocates memory, and is also called by a forward call stub.
- `execute_relative_jump`: This routine consists of two instructions and is called by a routine that opens libraries and manages tasks.
- `initialize_entity_state_and_loop_entities`: Initializes several fields in a structure pointed to by a1, including setting a value at offset 224 to 0 and offset 244 to a fixed address, then enters a loop that checks flags in a structure pointed to by piVar4 and performs conditional updates and calls to other routines.
- `update_entity_data`: Tests a longword at offset 332 of a structure in a1; if it is zero, it branches to another location, otherwise it loads three longwords from offsets 86, 48, and 8 of the structure into d1, d2, and d3, loads the constant 368 into d0, calculates a new address in a4, calls a routine through a pointer in a5, stores the value from d1 into offset 320 of the structure in a1, and branches to another routine.
- `clear_entity_flag_if_set`: Tests a longword at offset 320 of a structure in a1; if non-zero, it clears that longword and branches to another routine, otherwise it branches to another routine.
- `update_entity_state_and_attributes`: Fetches a value from offset 8 of a structure, stores it at offset 320 of the same structure, and if the value is non-zero, performs three consecutive calls to a subroutine using specific offsets from a pointer in a second structure.
- `update_player_attribute_and_state`: Checks a value in a table at offset 120 of a structure; if it equals 2, it calls several subroutines to update various fields in a structure, including decrementing a byte in a table and updating a global variable with a value from a structure.
- `update_game_state_and_call_subroutines`: Checks a value in a table at offset 120 of a structure; if it equals 2, it calls several subroutines to update various fields in a structure, including storing a byte from a table into offset 360 and updating offsets 356, 340, 328, and 363 using values derived from the structure and table lookups.
- `process_entity_list_and_update_counters`: Checks if specific flags in a structure are zero and if a pointer in another structure is non-zero, then performs a series of calls to a routine via a pointer.
- `calculate_scaled_offset_and_call`: Adds a longword from an offset of 88 in a structure to a register, shifts the result right by two bits, and calls a routine using a pointer from an offset of 296 in another structure with a constant value in d0.
- `assign_and_jump`: Calls a routine in a5 twice with different parameters, and if the second call returns zero, sets a longword at offset 620 of the structure in a2 to 20 and sets a longword at offset 612 of the structure in a2 to a value from a1, then jumps to a routine in a6.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Execute

- `setup_execution_environment`: This routine calls a routine that opens libraries and manages tasks, and is also reachable via a forward call stub.
- `modify_instruction_operand`: updates an instruction or a piece of code by modifying its immediate value or target address.

### Copy

- `initialize_program_environment`: A routine that serves as the program entry point to open libraries, locate the current task, and manage memory allocation and port messages.
- `modify_instruction_operand_2`: updates an instruction or operand via a relative or direct memory reference.
- `update_structure_and_fetch_table_byte`: Calculates a value based on a pointer in a1, stores it in a structure at offset 12, and clears several fields in a structure at offset 532 and 680.
- `extract_event_data_pair`: Calculates an index into a table using a value from a structure, retrieves a pointer from that table, and then fetches two consecutive bytes from the memory location pointed to by that retrieved pointer to store them in a structure.
- `execute_dual_parameter_calls`: Calculates a value by adding one to a longword at offset 508 of a structure in a1, then calls another routine with this value in d1 and a pointer from offset 536 of a4 in a4.
- `update_structure_pointer_and_call_handler`: Adds a value from a structure offset to a register, then calls a routine using a pointer from a second structure.
- `dispatch_event_type_handler`: Compares a value loaded from offset 544 of a structure in a1 against several constants and branches to specific handlers if the value matches 35, 37, 40, or 124; otherwise, it branches to a default location.
- `increment_counter_and_validate_table`: Increments a longword in a global variable, compares it to a value in a structure, and if the value is smaller, fetches a byte from a table using a scaled offset from another structure member.
- `branch_on_offset_test`: Tests a longword at offset 684 of a2 and branches to another routine if it is not zero; otherwise, it sets d1 to 0 and jumps to the address stored in a6.
- `process_input_sequence_table`: Increments a counter in a structure and reads a byte from a table using an index derived from multiple offsets, storing the result in another structure.
- `branch_to_offset_20c`: Performs a short branch to a relative offset.
- `fetch_and_compare_byte`: Fetches a byte from a table using an index calculated from a structure in a1 and compares it to a value in a structure in a2; if they match, it fetches another byte from a different table and calls another routine.
- `validate_input_byte_constant`: Loads a longword from offset 552 of a structure in a1 and compares it against a series of specific constant values, branching to different locations depending on whether the value matches 35, 37, 39, 40, 63, or 124.
- `branch_with_addition`: Adds a longword from offset 352 of a structure in a1 to d1 and branches to another routine.
- `branch_to_offset`: Performs a short branch to a different location.
- `store_constant_to_structure`: Tests a longword in d1 and, if it is non-zero, stores the value negative one into a longword at offset 720 of the structure in a2, then branches to another routine.
- `compare_and_copy_loop`: Copies a longword from offset 684 of a2 to offset 4 of a1, then enters a loop that compares a value from a table in a0 with a value from a1, and if they are not equal, copies a longword to a table location.
- `increment_and_copy_longword`: Increments a longword at offset 2ac of the structure in a2, calculates an offset using a value from offset 2a8 of a2, and copies a longword from a1 to the calculated destination in a0 before jumping to a routine in a6.
- `compare_and_set_byte`: Compares a longword from offset 708 of a2 with a longword from offset 704 of a2, and if the first is not greater, sets offset 700 of a2 to negative one and branches.
- `increment_and_copy_byte`: Increments a longword at offset 2c0 of the structure in a2, calculates an offset using a value from offset 2b4 of a2, and copies a byte from a0 to offset 2bc of the structure in a2 before jumping to a routine in a6.
- `call_helper_if_match`: Compares a longword at offset 2bc of the structure in a2 with 39, and if they are equal, calls a routine in a5 twice with d0 set to 12, then jumps to a routine in a6.
- `copy_and_dispatch_routine`: Copies a longword from offset 704 of the structure in a2 to offset 4 of a1, copies a longword from offset 700 of the structure in a2 to offset 0 of a1, and calls a routine using a pointer in a5.
- `set_mask_and_jump`: Sets a longword at offset 2c8 of the structure in a2 to negative one and then jumps to a routine in a6 after moving a value from a1 into d1.
- `call_helper_twice_and_jump`: Calls a routine in a5 twice with different d0 values and a4 offsets, then jumps to a routine in a6.
- `execute_conditional_logic`: Calls another routine with a pointer offset by 132 from a4, updates a long value from a pointer into a1, and if a value at offset 700 of a2 is not 41, writes -1 to offset 712 of a2 before calling another routine and jumping to a location in a6.
- `dispatch_by_id`: Compares a longword from offset 4 of a1 against several constants and branches to different routines or sets offset 712 of a2 to negative one based on the match.
- `execute_conditional_subroutine`: Clears a longword at offset 4 of a structure in a1, calls a subroutine using a pointer from a4 minus 132, stores the result in offset 8 of a1, and then performs a second call using a pointer from a4 plus 176 if a longword at offset 700 of a2 matches specific values.
- `update_table_and_call`: Calculates an offset by adding a longword from a1 to a bit-shifted longword from offset 728 of the structure in a2, stores a byte from offset 707 of the structure in a2 into a table at the calculated offset, copies a longword from offset 704 of the structure in a2 to the address in a1, and calls a routine using a pointer in a5.
- `call_with_parameters`: Loads a longword from offset 704 of a2 and a longword from offset 8 of a1 into d2 and d1, then calls a routine using a pointer from a4 plus an offset of 128.
- `branch_immediately`: Performs a no-operation and branches to another routine.
- `swap_bytes_in_loop`: Enters a loop that reads a byte from a table at an offset calculated from a1 and a2, stores it in offset 8 of a1, then writes a byte from offset 7 of a1 back to the table at the same offset, updating a1 with the value from offset 8 until a1 is zero.
- `store_d1_and_jump`: Stores a longword from d1 into offset 8 of a1 and, if d1 is non-zero, branches to another routine; otherwise, it moves d2 into d1 and jumps to the address in a6.
- `branch_to_another_routine`: Performs a short branch to another routine.
- `copy_bytes_until_zero`: Enters a loop that reads a byte from a table at an offset calculated from a1 and a2, stores it in a1, and continues if the byte is not zero, then finally writes a byte from offset 7 of a1 back to the table at the calculated offset.
- `clear_and_fill_table`: Clears several longwords in a structure in a2, copies a byte from a table in a0 to offset 716 of a2 and offset 8 of a1, then enters a loop that writes zero to a table in a0.
- `perform_multiple_calls`: Calls another routine three times with different offsets of a4 and different d0 values, then performs a bitwise NOT on a value from offset 712 of a2 and jumps to a location in a6.
- `copy_and_add_to_table`: Reads two bytes from a table in a0, stores them in a1, and enters a loop that copies bytes from the table to another location before adding two bytes and storing the result in the table.
- `add_and_store_byte`: Adds a byte from offset 8 of a1 to a byte from offset 12 of a1, then stores the result in a table at an offset in a0 calculated by shifting the value at offset 4 of a1 left by two.
- `process_table_elements`: Copies a byte from a table to a location in a0, then enters a loop that calls another routine and writes a byte from d1 into a table at an offset calculated from a1 and a2, incrementing a value in a1 until a comparison with offset 8 of a1 fails.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.
- `update_structure_pointers_2`: Calls a routine using a pointer from offset 284 of a2, and if the result is not zero, writes a longword from offset 604 of a2 into a table in a0 and updates offset 604 of a2.
- `update_structure_pointers`: Writes a long value from offset 27c of a2 into a table at an offset in a0 calculated from offset 4 of a1, then writes the value from offset 4 of a1 into the table at the calculated offset, and finally stores the value from offset 4 of a1 into offset 27c of a2.

### Makedir

- `initialize_program_execution`: A large routine that acts as a program entry point and calls a routine that opens libraries and manages task memory.
- `execute_stub_call`: A two-instruction routine that is called by the program entry routine.
- `update_structure_offsets`: Updates a value in a structure by adding a pointer to it and shifting it right by two, clears offsets 208 and 212 of a structure in a1, and performs a series of calls to update offsets 212 and 208 based on results from a5.
- `execute_sequence_of_calls`: Performs a series of calls to another routine with various d0 and a4 values, updating offsets 216, 212, 208, and 216 of a1 based on the results, and performs conditional logic based on the value at offset 216 of a1.

### CD

- `initialize_program_environment_2`: This entry-point routine performs initial system setup, including opening required libraries and allocating memory via port messages.
- `initialize_program_execution_2`: This routine contains two instructions and is called during the initial opening of system libraries.
- `initialize_and_update_object_metadata`: Updates a bit in a byte via a pointer, calculates and stores several word-aligned offsets into a structure pointed to by a1, and performs a series of calls to another routine to update various fields in a structure pointed to by a2 based on conditional checks of table values.
- `process_indexed_data`: Calls a routine using an offset from a4, then checks a value in a table using an index from a1; if that value is non-zero, it calls another routine to update offset 608 of a2 and potentially performs further calls to update offset 608 and offset 20 of a1.
- `handle_complex_data_updates`: Calls another routine with various parameters, updates offset 440 of a1, performs several calls to another routine using values from a1 and a2, and conditionally updates a value in a table and offset 600 of a2.

### EndIf

- `stub_two_instruction_entry`: A two-instruction program entry in a utility file with no listed calls or references.

### If

- `initialize_program_environment_3`: This routine serves as the initial entry point that opens required libraries, identifies the current task, and manages memory allocation and port message waiting.
- `execute_instruction_pair`: performs two instructions called via a relative or absolute pointer.
- `update_entity_state_and_attributes_2`: Updates a structure by clearing several fields and calculating a new value based on a register, then performs a series of conditional checks against a table to update various offsets in the structure based on character matches and non-zero values found in an entity data table.
- `evaluate_bitmask_threshold`: Copies a longword from a1 to d1, shifts d1 left by 2, and tests a bit in a table at offset 8 of a0 using the shifted value.
- `test_table_bit_and_compare`: Copies a longword from a1 to d1, shifts d1 left by 2, and tests a bit in a table at offset 12 of a0 using the shifted value.
- `update_entity_event_chain`: Updates a chain of pointers within a structure by fetching values from a table based on indices stored in the structure and compares specific bytes in that table to the value 36.
- `process_entity_event_chain`: Calls a routine using parameters from a structure in a1 and a2, then checks a table at offset 20 of a0 using a value from a1 to determine if a bit is set.
- `branch_to_offset_2`: Returns immediately by performing a short branch to a relative offset.
- `check_status_and_compare`: Copies a longword from a1 to d1, shifts d1 left by 2, and tests a bit in a table at offset 24 of a0 using the shifted value.
- `dispatch_entity_event_callback`: Checks a bit in a table using an index from a pointer in a1 and, if set, calls process_entity_event_callback with a constant value of 404 and updates a1.
- `evaluate_entity_status_flag`: Compares a longword at offset 390 of the structure in a1 with one, then checks if that longword is zero; if it is not zero and not equal to one, it loads negative one into d1 and branches to another routine, otherwise it branches to another routine.
- `process_entity_event_callback`: Checks a table using an index from a pointer in a1 to see if a bit is set; if so, it calls a routine with a constant value of 404, updates a longword at offset 360 of the structure in a1 with the result, and if that result is non-zero, calls another routine with a constant value of 408 and returns -1.
- `update_bitmask_status`: Sets a longword at offset 328 of a1 to zero, checks a bit in a table using a value from a1 shifted left by 2, and if that bit is zero, clears offset 364 of a1, sets offset 600 of a2 to 10, and calls a subroutine using a pointer from a4.
- `synchronize_bitmask_values`: Enters a loop that calls a routine and stores the result in offset 600 of a2, then performs several calls to update offset 360 of a1 and various offsets of a1 based on bitwise operations and comparisons of values retrieved from a2 and a1.
- `poll_and_update_bitmask`: Enters a loop that calls a routine and stores the result in offset 600 of a structure in a2, repeating until the result is neither 10 nor -1.

### IconX

- `open_libraries_and_wait_port`: A 132-instruction program entry that opens the DOS and Exec libraries, finds the current task, allocates memory and waits for a message port.
- `release_libraries_and_memory_2`: Closes a DOS file and a library, frees allocated memory and replies to a message port while the task is protected.
- `check_workbench_and_open_console`: Checks that the program was started from Workbench, reporting a used-from-Workbench-only message and a not-enough-memory error.
- `launch_command_shell_with_wait`: Builds a command line containing the text for a wait of 999 minutes and an endcli redirect to the null device, then calls a library function at offset 318 and eleven unnamed routines.
- `install_library_and_functions`: Calls the Exec make-library and make-functions routines and two unnamed routines to set up a library structure.
- `initialise_resident_ram_module`: Uses the text for the RAM device and calls the make-functions, init-resident and free-memory routines, plus five unnamed routines.
- `invoke_two_unnamed_routines`: A 20-instruction routine called by two unnamed routines that makes two unnamed calls.
- `stub_short_routine_four_callers`: A seven-instruction routine called by four unnamed routines with no listed calls.
- `stub_short_routine_two_callers`: A six-instruction routine called by two unnamed routines with no listed calls.
- `forward_call_three_callers`: An 11-instruction routine called by three unnamed routines that makes one unnamed call.
- `write_data_from_memory`: copies a block of bytes from a memory location to another or a destination buffer.
- `call_supervisor_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the Supervisor function and returns its result.
- `call_exit_intr_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the interrupt-exit function and returns its result.
- `call_schedule_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the Schedule function and returns its result.
- `call_reschedule_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the Reschedule function and returns its result.
- `call_dispatch_stub`: A five-instruction Exec library stub that loads its arguments and the library base, calls the Dispatch function and returns its result.
- `call_exception_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the Exception function and returns its result.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the Enable function and returns its result.
- `call_alloc_mem_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the AllocMem function and returns its result.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry_stub`: A six-instruction Exec library stub that loads its arguments and the library base, calls the AllocEntry function and returns its result.
- `forward_call_single_caller`: A 10-instruction routine called by one unnamed routine that makes one unnamed call.
- `invoke_library_vector_198_stub`: A six-instruction stub that calls a library function at offset minus 198 from a library base and returns its result.
- `invoke_library_vector_210_stub`: A seven-instruction stub that calls a library function at offset minus 210 from a library base and returns its result.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `invoke_library_vector_414_stub`: A six-instruction stub that calls a library function at offset minus 414 from a library base and returns its result.
- `invoke_library_vector_552_stub`: A seven-instruction stub that calls a library function at offset minus 552 from a library base and returns its result.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `format_message_and_call_two`: A 22-instruction routine that calls the Exec RawDoFmt text formatting function and two unnamed routines.
- `return_immediately_stub`: A two-instruction routine that returns at once with no listed calls.
- `format_message_with_rawdofmt_2`: A nine-instruction routine that calls the Exec RawDoFmt text formatting function and one unnamed routine.
- `load_address_relative`: This routine performs a relative load or address calculation using a single instruction.

### Disk-Validator

- `forward_call_nine_instructions`: A nine-instruction routine that makes one unnamed call and has no other listed behaviour.
- `stub_single_instruction_return`: A single-instruction routine reached from a tiny exclusive-or return routine.

### Disk-Validator

- `call_return_stub`: This routine executes a single instruction that calls another routine which immediately returns.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d5.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### TV Sports Basketball (1990)(Cinemaware)[cr PNA][v Saddam-Saddam II](Disk 2 of 2).adf:bootblock

- `call_dos_library_function`: A ten-instruction routine that refers to the DOS library name and calls a library function at offset minus 96 from a library base.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_pointer_then_hang`: Calls a code pointer four times in a row, then loops forever without returning.
