# Game 3566 (EvilDawn v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03566). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (767210 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 54 of 54 named (0 author symbols), 54 of 54 described (37 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 31296 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2571 of 2571 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 299073 |
| code | 11508 |
| data | 456626 |
| padding | 3 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 52 | 10 |
| B | 2 | 465 | 43 |
| SPACE (raw code file, load address unknown) | 1 | 2054 | 1 |

## Cases handled

- EvilDawn_v1.0/EvilDawn_v1.0.hdf: hard-disk image (AmigaDOS OFS, 832 blocks): its filesystem parses and all 27 files in it are byte-identical to the extracted tree EvilDawn_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- EvilDawn_v1.0/EvilDawn_v1.0_hdf/EvilDawn/data/SPACE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then calls another routine, then jumps to the address in a6.
- `install_pc_relative_pointer_into_global`: Loads a pc-relative address into a3, stores that address into memory at a3 minus offset, then loads another pc-relative address into a3 and moves it into d1 before returning.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `frag_or_ori_addi_pea_move_rts`: Short code fragment that merges d0 into d4, then merges 0 into d7, then merges 0 into d0, then merges 171 into d0, then merges 0 into d2, then adds 14 to the memory just below a2 (moving a2 down), then pushes the address of a location in the code onto the stack, then copies 20153 to the memory at an offset from a3, then copies the top of the stack to the memory at an offset from a3, then returns.
- `patch_interrupt_vectors`: Checks a global variable for a specific value and if matching, patches three interrupt vector locations with jump instructions and their corresponding return addresses.
- `frag_move_andi_movea_rts`: Short code fragment that copies d0 to the memory at an offset from a0, then masks d0 with an address, then copies d0 to a2, then returns.
- `frag_move_andi_movea_rts_2`: Short code fragment that copies the memory at an offset from a0 to d0, then masks d0 with an address, then copies d0 to a2, then returns.
- `frag_move_andi_movea_rts_3`: Short code fragment that copies the memory at an offset from a0 to d0, then masks d0 with an address, then copies d0 to a2, then returns.

### B

- `setup_game_environment`: Initializes game resources by allocating memory, setting up task structures, and managing system ports while performing various setup operations before entering the main game loop.
- `frag_tst_beq_jsr_movea_moveq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then calls another routine, then copies a fixed memory address to a1, then calls another routine, then loads 0 into d0, then copies a fixed memory address to a7, then returns.
- `combines_bitplane_rows_into_buffer`: Loops over a block of 16-bit words starting at a fixed source address, ORs five source rows together and inverts the result into one destination row, and copies the source rows to five further destination rows at fixed offsets.
- `call_super_state`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_user_state`: Wrapper that calls the exec library function UserState and returns its result to the caller.
- `frag_movea_lea_moveq_jsr_move_beq_rts`: Short code fragment that copies a fixed memory address to a6, then loads the address of a location in the code into a1, then loads 0 into d0, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `scan_directory_handle`: Locks a directory using a fixed path string, examines its contents to retrieve file information, and stores the resulting lock handle for later use in file operations.
- `scan_directory_entries`: Repeatedly calls the DOS directory-scan function on a directory handle, and for each entry found calls two other routines; when the scan returns zero it reads the DOS error code, then closes the handle.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `frag_move_movea_jsr_tst_rts`: Short code fragment that copies a1 to d1, then copies d0 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `writes_null_terminated_string`: Counts the bytes of the null-terminated string at the address in d0 by scanning forward one byte at a time, then passes the string and that count to the DOS write call on the standard output handle.
- `frag_move_bsr`: Short code fragment that copies 10 to d0, then calls another routine, then copies 13 to d0.
- `frag_movem_move_lea_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a1, then copies d0 to the memory at a1, then copies a1 to d2, then copies 1 to d3, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_move_movea_jsr_tst_rts_2`: Short code fragment that copies a fixed memory address to d1, then copies 500 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `handle_character_input`: Clears a memory location at a fixed address, calls another routine to process input, handles special control characters by branching to different routines, and stores character data sequentially in memory.
- `frag_clr_suba_move_movem_rts`: Short code fragment that clears the memory at a2 (advancing a2), then subtracts 1036 from a2, then copies a2 to a fixed memory address, then restores saved registers from the stack, then returns.
- `process_input_byte`: Compares an address with a fixed value and branches based on equality, writes specific byte values to a register before calling other routines, and manages a decrementing counter in a register.
- `frag_move_lea_movea_jsr_clr_rts`: Short code fragment that copies 1 to d3, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a1, then copies a1 to d2, then copies a fixed memory address to a6, then calls another routine, then clears d0, then copies a fixed memory address to d0, then returns.
- `frag_move_bsr_beq_rts`: Short code fragment that copies 1006 to d2, then calls another routine, then branches if the last result was zero, then copies an address to d2, then copies an address to d3, then calls another routine (2 times in a row), then returns.
- `initialize_game_resources`: Initializes various game data sections by copying blocks of memory from fixed source addresses to destination addresses, with each operation involving a specific size and a check for successful completion before proceeding to the next block.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_3`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_4`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_5`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_6`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_7`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_8`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_9`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_10`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_11`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_12`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_13`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_14`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_15`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_16`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_17`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_18`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_19`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_20`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.

### SPACE (raw code file, load address unknown)

- `initialize_hardware`: Sets up system registers including DMA and interrupt control, clears memory regions, and performs a series of hardware initialization operations before jumping to another routine.
