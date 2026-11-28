# Game 3735 (Full Metal Planete): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03735). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902160 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (8 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 125 of 125 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 301 of 301 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 1090 |
| data | 886910 |
| padding | 336 |
| unused | 13824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Full Metal Planete.adf:bootblock | 1 | 156 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 151 | 7 |

## Cases handled

- Full Metal Planete/Full Metal Planete.adf: custom-format (non-DOS) disk, 1731 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 120 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Full Metal Planete/Full Metal Planete.adf: blocks 2-21 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.63)
- Full Metal Planete/Full Metal Planete.adf: blocks 1213-1215 (bytes 621056-622592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.52)
- Full Metal Planete/Full Metal Planete.adf: blocks 1217-1221 (bytes 623104-625664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.14)
- Full Metal Planete/Full Metal Planete.adf: blocks 1223-1226 (bytes 626176-628224) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.55)
- Full Metal Planete/Full Metal Planete.adf: blocks 1228-1236 (bytes 628736-633344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.95)
- Full Metal Planete/Full Metal Planete.adf: blocks 1238-1759 (bytes 633856-901120) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.07)
- 1 packed track region(s), 598016 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Full Metal Planete.adf:bootblock

- `frag_move_bsr_addq_movea_moveq_rts`: Short code fragment that copies a1 to the stack, then calls another routine, then adds 4 to a7, then copies d0 to a0, then loads 0 into d0, then returns.
- `load_amigados_hunk_executable`: Allocates memory for an AmigaDOS executable segment, reads hunk headers to determine segment size and flags, processes code, data, BSS, relocation and symbol blocks, and returns the first loaded segment.
- `initialize_io_request`: Initializes an I/O request structure with specific values and updates related global variables, then returns.
- `frag_link_move_bsr_subq_movea_addq_unlk`: Short code fragment that creates a stack frame in a5, then copies a2 to the stack, then calls another routine, then subtracts 1 from the memory at an offset from a2, then copies the memory at an offset from a2 to a6, then adds 4 to the memory at an offset from a2, then copies the memory at a6 to d0, then copies the top of the stack to a2, then removes the stack frame held in a5, then returns.
- `call_helper_repeatedly`: Calls its helper once for each unit of its argument and stops when the count reaches zero.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `call_do_i_o`: Wrapper that calls the exec library function DoIO and returns its result to the caller.
- `call_alloc_mem`: Wrapper that calls the exec library function AllocMem and returns its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state`: Checks the input state byte at offset -27404 of a4 and updates a boolean flag at offset -15440 of a4 to zero when the input is 16, or one when the input is 32.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_function_table`: Iterates through a table of function pointers, calling each function pointer if it is not null, and returns zero after processing all entries in the table.
- `frag_movea_lea_moveq_tst_bne_move_bra`: Short code fragment that copies the memory at an offset from a7 to a1, then loads the address of a fixed memory address into a0, then loads 7 into d0, then tests the memory at a0 (advancing a0), then branches if the last result was not zero, then copies a1 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_dbra_rts`: Short code fragment that loops back while counting down d0, then returns.
- `frag_movea_lea_moveq_cmpa_bne_clr_bra`: Short code fragment that copies the memory at an offset from a7 to a1, then loads the address of a fixed memory address into a0, then loads 7 into d0, then compares a1 with the memory at a0 (advancing a0), then branches if the last result was not zero, then clears the memory at an offset from a0, then jumps to another point in the code.
- `frag_dbra_rts_2`: Short code fragment that loops back while counting down d0, then returns.
- `call_with_pc_relative_pointer`: Loads a PC-relative pointer into a0, pushes it as an argument, calls another routine, removes the argument from the stack, and returns.
- `process_joystick_input`: Reads joystick and mouse input states, updates global position coordinates, and handles input bit manipulation for movement control.
