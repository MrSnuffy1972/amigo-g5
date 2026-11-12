# Game 1745 (zany golf (1988)(electronic arts)[a3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01745). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 58 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 62 of 62 named (23 author symbols), 62 of 62 described (12 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 720 of 720 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 106865 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1600 of 1981 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 287285 |
| code | 7030 |
| data | 473931 |
| padding | 13578 |
| unused | 119296 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zany Golf (1988)(Electronic Arts)[a3].adf:Zany | 15 | 1574 | 59 |
| Zany Golf (1988)(Electronic Arts)[a3].adf:bootblock | 1 | 10 | 1 |
| QUICKSCORE.UNV (raw code file, load address unknown) | 1 | 0 | 0 |
| SPRITES8.SPR (raw code file, load address unknown) | 1 | 0 | 0 |
| FILE1 (raw code file, load address unknown) | 1 | 397 | 2 |
| HOLE6.REG (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf: file c/sigfile (header 93 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/SIGN9.UNV: high-entropy data file (7.64 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE8.UNV: high-entropy data file (7.78 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE6.UNV: high-entropy data file (7.69 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/SIGN5.UNV: high-entropy data file (7.65 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE4.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE2.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE0.UNV: high-entropy data file (7.81 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE9.UNV: high-entropy data file (7.78 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE7.UNV: high-entropy data file (7.79 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/SIGN6.UNV: high-entropy data file (7.61 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE5.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/MARQUEE.UNV: high-entropy data file (7.75 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE3.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE1.UNV: high-entropy data file (7.77 bits/byte), classified as packed data
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/QUICKSCORE.UNV: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/SPRITES8.SPR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/FILE1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf:files/HOLE6.REG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- zany golf (1988)(electronic arts)[a3]/Zany Golf (1988)(Electronic Arts)[a3].adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.21)
- 4 never-loaded region(s), 106865 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Zany Golf (1988)(Electronic Arts)[a3].adf:Zany

- `open_dos_library_at_entry`: Program entry that opens the dos library by name through the exec library table, then calls a disk signature routine and two other routines.
- `poll_port_a_and_interrupts`: Reads the port A status register and writes the interrupt enable register.
- `verify_disk_and_enable_interrupts`: Displays a message requesting the original copy of the game, waits for a specific bit in the CIAA port A register to clear, and then calls a routine to handle disk signaling.
- `chain_two_subroutine_calls`: Calls two other routines in sequence and returns.
- `process_table_loop`: Performs a loop of 62 instructions that repeatedly loads a value from a table, processes it, and stores the result back through a pointer, using chained subroutine calls to helper routines and a data handler pointer.
- `run_short_handler_entry`: A sixteen-instruction routine reached through a handler pointer in the data and from three unnamed callers, and executed in the reference run.
- `call_two_library_functions`: Calls two functions through library offsets and is reached through a pointer in the first data hunk and from one unnamed routine.
- `wait_interrupt_and_store_byte`: Waits for an interrupt by writing to the interrupt enable register, then reaches a handler through a handler pointer and runs a seventeen-instruction routine that reads from a table and stores a byte from a register through a pointer.
- `load_handler_from_table_and_store_byte`: Reads a handler address from a table referenced by the text FILE1, then reaches that handler through a handler pointer and runs a seventeen-instruction routine that reads from a table and stores a byte from a register through a pointer.
- `enable_interrupts_store_byte`: Enables interrupts, then performs a 34-instruction sequence that reads from a table and stores a byte from a register through a pointer, referencing the FILE1 text.
- `execute_eight_instruction_routine`: An eight-instruction routine called from one unnamed routine, with no further calls, library use or hardware access recorded.
- `dispatch_handler_to_two_routines`: A seven-instruction routine reached through a handler pointer in the data and from one unnamed routine.
- `DiskSig`: The 202-instruction routine named DiskSig, which calls memory allocation and freeing library functions and ten other routines.
- `call_secbase_then_two_routines`: A 39-instruction routine called by DiskSig that calls a routine named SecBase and two other routines.
- `read_sectors_via_trackdisk`: Opens the trackdisk device, creates and deletes a standard I/O request, reads sectors and turns the motor off, with memory allocation and freeing calls.
- `update_cia_port_b_bits`: A 42-instruction routine called by DiskSig that writes the port B register of the second CIA chip.
- `forward_single_routine_call`: A 25-instruction routine called by DiskSig that calls one other routine.
- `poll_port_a_and_write`: A 40-instruction routine called by DiskSig and one unnamed routine that reads the port A register and writes the port B register of the second CIA chip.
- `run_port_b_update_sequence`: A 78-instruction routine called by DiskSig and one unnamed routine that writes the port B register of the second CIA chip.
- `set_port_b_output_bits`: A 19-instruction routine called from five unnamed routines that writes the port B register of the second CIA chip.
- `write_port_b_register_value`: A six-instruction routine called from six unnamed routines that writes the port B register of the second CIA chip.
- `return_zero_value`: It is a brief routine that simply returns the constant zero in the primary result register to its caller and performs no other recorded action.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `write_port_b_and_forward`: An 11-instruction routine called from two unnamed routines that writes the port B register of the second CIA chip and calls one other routine.
- `read_port_a_and_forward`: A 61-instruction routine called by DiskSig and one unnamed routine that reads the port A register, which holds the fire-button and disk status bits, and calls two other routines.
- `forward_three_routine_calls`: A 19-instruction routine called by DiskSig that calls three other routines.
- `forward_two_routine_calls`: A 16-instruction routine called by DiskSig that calls two other routines.
- `FindSector`: A 29-instruction routine named FindSector, reached from one unnamed routine.
- `call_findsector_and_secbase`: A 117-instruction routine called by DiskSig and one unnamed routine that calls FindSector, SecBase and four other routines.
- `SecBase`: A 30-instruction routine named SecBase, called from three unnamed routines.
- `forward_to_two_routines`: A 64-instruction routine called by one unnamed routine that calls two other routines.
- `execute_seventeen_instruction_routine`: A 17-instruction routine called from two unnamed routines, with no calls, registers or text recorded.
- `call_secbase_and_seven_routines`: A 58-instruction routine called by one unnamed routine that calls SecBase and seven other routines.
- `call_two_routines_once`: A 15-instruction routine called by one unnamed routine that calls two other routines.
- `execute_twelve_instruction_routine`: A 12-instruction routine called from three unnamed routines, with no calls, registers or text recorded.
- `return_through_frame_cleanup`: A 28-instruction routine called by one unnamed routine that calls a two-instruction routine which removes the stack frame and returns.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a6, then returns.
- `_AllocMem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `_FreeMem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `_FindTask`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `_AllocSignal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `_FreeSignal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `_AddPort`: Library stub: loads its arguments and the exec library base, calls AddPort and returns its result to the caller.
- `_RemPort`: Library stub: loads its arguments and the exec library base, calls RemPort and returns its result to the caller.
- `_OpenDevice`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `_DoIO`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `CreateStdIO`: A routine named CreateStdIO of eleven instructions that calls CreatePort.
- `DeleteStdIO`: It releases a previously created message port by freeing its memory and deleting the port structure, acting as the cleanup counterpart to the track disk read routine.
- `ReadSecs`: It submits a synchronous input-output request to the track disk device and waits for that request to complete.
- `WriteSecs`: It submits a write input-output request to the track disk device so that recorded sectors are transferred to the storage medium.
- `FormatSecs`: A routine named FormatSecs of 24 instructions that calls two other routines.
- `branch_to_two_routines`: An eight-instruction routine called by FormatSecs that calls two other routines.
- `issue_io_request`: It is a very short five-instruction routine that issues a single input-output request to a device and returns.
- `UpdateTD`: It queries the current state of the track disk device by submitting an input-output request that reports the disk status.
- `MotorOff`: It stops the drive mechanism by submitting an input-output request that powers the disk motor off.
- `DiskStatus`: It reads and reports the overall status of the track disk device by submitting an input-output request that returns the device condition.
- `_CreatePort`: A routine named CreatePort of 52 instructions, called from CreateStdIO. It calls the library functions for adding a port, allocating memory and a signal, finding a task, freeing a signal and creating a list.
- `_DeletePort`: A routine named DeletePort of 20 instructions, called from DeleteStdIO. It calls routines that free memory and a signal and remove a port.
- `_NewList`: A six-instruction routine named NewList, called from CreatePort, with no further calls recorded.

### Zany Golf (1988)(Electronic Arts)[a3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### FILE1 (raw code file, load address unknown)

- `chain_two_unnamed_calls`: An eight-instruction entry in a raw code file, called from one unnamed routine, that calls two other routines.
- `setup_audio_channels_and_dma`: A 389-instruction routine called by one unnamed routine that writes the four audio volume registers, the audio control register and the DMA control register.
