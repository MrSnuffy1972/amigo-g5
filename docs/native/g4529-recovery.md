# Game 4529 (NinjaMission v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04529). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1005562 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 23 of 23 named (0 author symbols), 23 of 23 described (12 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 57050 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 291 of 291 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 368709 |
| code | 1062 |
| data | 635785 |
| padding | 6 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 291 | 23 |
| NINJA.BIN (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- NinjaMission_v1.0/NinjaMission_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1120 blocks): its filesystem parses and all 19 files in it are byte-identical to the extracted tree NinjaMission_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- NinjaMission_v1.0/NinjaMission_v1.0_hdf/NinjaMission/data/NINJA.BIN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 57050 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_bchg_not_bra`: Short code fragment that flips bit d1 of d4, then inverts an indexed entry from a0, then jumps to another point in the code.
- `initialize_game`: Initializes game structures and sets up jump table entries with specific addresses, then jumps to a global routine pointer.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a3, then copies 0 to the memory at an offset from a3 (4 times in a row), then returns.
- `frag_lea_moveq_rts`: Short code fragment that loads the address of a fixed memory address into a3, then loads 0 into d2, then returns.
- `wait_while_global_bit_set`: Busy-waits while bit six of a global flag word stays set, then returns.
- `initialize_system`: Saves all general-purpose registers and address registers on the stack, loads a pointer to a global variable into a0, copies a location in the code to a2, calls a routine via a pointer, checks if the result is zero and branches to a label if so, then calls another routine and restores all registers from the stack before returning.
- `copy_bytes_loop`: Copies bytes from one memory location to another using a loop that decrements a counter in d7, advances both pointers after each copy operation, and calls another routine before returning.
- `frag_movem_movea_move_eori_dbra_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a0, then copies 1 to d0, then exclusive-ors an address into the memory at a0 (advancing a0), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `setup_and_call`: Loads a pointer from a table, stores it in a global variable, then calls three routines through function pointers.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then calls another routine, then jumps to the address in a6.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a location in the code into a3, then copies a3 to the memory at an offset from a3, then loads the address of a location in the code into a3, then copies a3 to d1, then returns.
- `initialize_structure`: Allocates memory and initializes fields of a data structure, including a pointer chain and counter, then returns.
- `setup_system_structures`: Allocates memory using a library call, sets bit 4 of a byte at offset 16 of allocated structure, stores a pointer to a global variable at offset 28 of that structure, calls a routine via a pointer, updates two global pointers with the address of a global variable, writes two 32-bit values to a structure, and populates a third structure with the results of previous calls before calling another routine.
- `open_dos_library`: Opens the dos.library and sets up a message port for communication, then waits for a message and processes it by updating task pointers and status bytes.
- `load_and_jump`: Loads a pointer from a table and jumps to it, using values from a structure referenced via a5.
- `frag_move_movea_jsr_bra`: Short code fragment that copies d0 to the memory at an offset from a4, then copies the memory at an offset from a4 to a0, then copies the memory at an offset from a4 to a1, then copies a5 to the memory at an offset from a4, then calls another routine, then jumps to another point in the code.
- `copy_string_with_delimiter`: Copies bytes from two source buffers into a destination buffer, inserting a delimiter after the first copy if the second source length is non-zero, and null-terminates the result.
- `frag_moveq_move_bra`: Short code fragment that loads 0 into d0, then copies 103 to d1, then jumps to another point in the code.
- `validate_strings`: Checks two null-terminated strings for equality using a helper routine, and returns zero if either comparison matches.
- `frag_move_cmpm_bne_tst_movea_rts`: Short code fragment that copies a0 to the stack, then compares the memory at a1 (advancing a1) with the memory at a0 (advancing a0), then branches if the last result was not zero, then tests the memory at an offset from a0, then branches if the last result was not zero, then copies the top of the stack to a0, then returns.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
