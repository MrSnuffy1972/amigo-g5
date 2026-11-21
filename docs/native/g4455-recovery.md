# Game 4455 (Moonwalker v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04455). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4006075 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 42 of 42 named (0 author symbols), 42 of 42 described (17 by rule, 20 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 152832 bytes, 20 functions |
| Independent decoder cross-check (Ghidra against capstone) | 663 of 663 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8577 |
| code | 2452 |
| data | 3807089 |
| padding | 53 |
| unused | 187904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 308 | 22 |
| Disk.1 bytes 0-768 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 1024-5632 (code never loaded by a reference run) | 1 | 355 | 20 |
| Disk.1 bytes 764672-781568 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 811776-820224 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 820480-872960 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 652544-712960 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 713728-722944 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: raw disk image (889856 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: raw disk image (889856 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Moonwalker_v1.0/Moonwalker_v1.0.hdf: hard-disk image (AmigaDOS OFS, 4320 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree Moonwalker_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: bytes 0-768 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: bytes 1024-5632 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: bytes 764672-781568 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: bytes 811776-820224 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: bytes 820480-872960 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: bytes 652544-712960 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: bytes 713728-722944 look like code no run loaded: recovered position-independently (load address unknown)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: 5 logged loads cover 83200 bytes of the image
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: chunks 44-46 (bytes 11264-12032) outside the filesystem hold data, not code (opcode density 0.052, entropy 3.84)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: chunks 49-1260 (bytes 12544-322816) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.87)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: chunks 1264-2557 (bytes 323584-654848) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.72)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: chunks 2564-2694 (bytes 656384-689920) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.84)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: chunks 2696-2985 (bytes 690176-764416) outside the filesystem hold data, not code (opcode density 0.034, entropy 6.81)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.1: chunks 3054-3169 (bytes 781824-811520) outside the filesystem hold data, not code (opcode density 0.045, entropy 6.02)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: 2 logged loads cover 69632 bytes of the image
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: chunks 44-45 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.05)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: chunks 49-1558 (bytes 12544-399104) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.22)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: chunks 1561-2523 (bytes 399616-646144) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.91)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: chunks 2525-2534 (bytes 646400-648960) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.07)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: chunks 2537-2547 (bytes 649472-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.84)
- Moonwalker_v1.0/Moonwalker_v1.0_hdf/Moonwalker/Disk.2: chunks 2825-2903 (bytes 723200-743424) outside the filesystem hold data, not code (opcode density 0.013, entropy 4.92)
- 6 never-loaded region(s), 148224 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_cia_port_b_wait` (Disk.1 bytes 1024-5632 (code never loaded by a reference run)): Writes an inverted combination of a fixed mask and a shared selector to a CIA port B register, clears a shared flag when set, and repeatedly calls another routine until a result register becomes zero. Kind: disk track check. Pass path: The check passes if cmpi.w #an address, d0 branches to an address (skipping the load routine), indicating a valid disk or track..
- `frag_move_or_not_bchg_bsr_lea_subq_rts` (Disk.1 bytes 1024-5632 (code never loaded by a reference run)): Short code fragment that copies 137 to d0, then merges a location in the code into d0, then inverts d0, then copies d0 to a fixed memory address, then flips bit 0 of d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then subtracts 1 from the memory at a0, then returns. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB to check disk hardware state, and if the check passes (the branch after the call is taken), it continues execution normally. The bchg/bra pattern suggests a conditional check that may be part of a manual/code-w.
- `frag_move_or_not_bchg_bsr_lea_addi_rts` (Disk.1 bytes 1024-5632 (code never loaded by a reference run)): Short code fragment that copies 139 to d0, then merges a location in the code into d0, then inverts d0, then copies d0 to a fixed memory address, then flips bit 0 of d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then adds 1 to the memory at a0, then returns. Kind: disk hardware check. Pass path: CIAB_PRB is set to bVar1, then XORed with 1, and u2__sub_014E() is called; the check passes if the routine continues normally without branching or trapping.
- `initialize_and_process_dma` (Disk.1 bytes 1024-5632 (code never loaded by a reference run)): Initializes DMA settings and processes data blocks while checking for specific conditions, updating counters and returning based on loop iterations. Kind: disk track sync/checksum check. Pass path: The routine successfully processes all tracks and checksums, returning 0 on success (normal execution path). If any checksum fails, it jumps to `u2__l0_0360` and eventually returns -1..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_and_execute`: Calls system routines to read data from disk, processes memory addresses, and sets up jump tables before calling another routine with specific parameters.
- `setup_system_registers`: Writes specific values to system control registers including a coprocessor and DMA configuration, then jumps to an address loaded from a register.
- `configure_jump_tables`: Sets up jump table entries by writing jump instructions and addresses to memory locations, then configures system registers before jumping to a fixed address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `setup_and_execute_call`: Sets up memory locations with specific values to create jump tables, then executes a function call through a register and cleans up the stack.
- `store_and_call_trap`: Stores a byte value from a register into a memory location and then executes a system trap instruction with a specific parameter.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `initialize_dma_and_read`: Sets up DMA configuration, calculates memory addresses using multiplication, and calls a system routine to read data from disk while handling error conditions.
- `setup_and_execute`: Restores registers from stack, clears a global variable, loads addresses from a table, and calls another routine with the loaded data.
- `frag_movem_lea_move_clr_movea_jsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then copies 200 to d0, then clears d1, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_lea_move_clr_movea_jsr_rts_2`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then copies 200 to d0, then clears d1, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then calls another routine, then returns.
- `execute_multiple_setup_calls`: Performs multiple setup operations by loading addresses and values into registers, then calls other routines with these parameters to configure memory locations.
- `mask_and_store_pointers`: Reads pointer values from two memory locations, applies a bitmask to each, stores the masked results back to their respective locations, and returns the original value in a register.
- `mask_and_update_pointer_chain`: Reads pointer values from memory locations, applies a bitmask to each, updates the pointers with the masked values, and stores the results back to their respective locations.
- `frag_cmp_bne_move_adda_cmpa_rts`: Short code fragment that compares d0 with the memory at a0, then branches if the last result was not zero, then compares d1 with the memory at an offset from a0, then branches if the last result was not zero, then copies d2 to the memory at a0 (advancing a0), then copies d3 to the memory at a0 (advancing a0), then adds 2 to a0, then compares a1 with a0, then branches if the last result was not zero, then returns.
- `frag_cmp_bne_move_adda_cmpa_rts_2`: Short code fragment that compares d0 with the memory at a0, then branches if the last result was not zero, then compares d1 with the memory at an offset from a0, then branches if the last result was not zero, then copies 20153 to the memory at a0 (advancing a0), then copies d3 to the memory at a0 (advancing a0), then adds 2 to a0, then compares a1 with a0, then branches if the last result was not zero, then returns.
- `frag_movem_lea_cmpi_bne_move_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then compares the memory at a1 with 4153, then branches if the last result was not zero, then copies 20153 to the memory at a1 (advancing a1), then copies a0 to the memory at a1, then restores saved registers from the stack, then returns.
- `frag_movem_move_not_ror_cmp_bne_pea_add`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then inverts d0, then rotates d0 right by 1, then compares d0 with a location in the code, then branches if the last result was not zero, then pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
- `frag_bne_pea_move_addi_rts`: Short code fragment that branches if the last result was not zero, then pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
- `frag_cmpi_bne_move_movem_rts`: Short code fragment that compares d0 with 88, then branches if the last result was not zero, then copies 240 to a fixed memory address, then restores saved registers from the stack, then returns.

### Disk.1 bytes 1024-5632 (code never loaded by a reference run)

- `load_and_trap`: Loads an address into a register, stores that address in a memory location, and then executes a system trap instruction.
- `frag_move_lea_moveq_dbra_jmp`: Short code fragment that copies 9984 to the status register, then loads the address of a fixed memory address into a0, then loads the address of a location in the code into a1, then loads 16 into d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then loads the address of a location in the code into a1, then jumps to another point in the code.
- `frag_lea_move_dbra_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then copies 1023 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then jumps to another point in the code.
- `setup_memory_regions`: Initializes memory pointers, sets specific values in memory locations, and calls other routines to process data with conditional branching based on results.
- `frag_move_beq_bsr_bra_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then returns.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (movep.w) and then falls through into the code that follows, with no return of its own.
- `process_input_data`: Saves and restores registers, calls another routine with data from a table, then calls multiple routines with values from the stack before making a recursive call to itself.
- `combine_alternating_bits`: Combines two word streams into a destination block using a bit mask of 21845, processing data in a loop until a counter reaches zero.
- `frag_move_andi_beq_rts`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 2, then branches if the last result was zero, then copies 32770 to a fixed memory address, then copies 2 to a fixed memory address, then returns.
- `frag_lea_move_btst_beq_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 48 to the memory at a0, then copies 11 to the memory at an offset from a0, then copies 137 to the memory at an offset from a0, then tests bit 0 of the memory at an offset from a0, then branches if the last result was zero, then copies 136 to the memory at an offset from a0, then returns.
- `set_cia_port_b_wait`: Writes an inverted combination of a fixed mask and a shared selector to a CIA port B register, clears a shared flag when set, and repeatedly calls another routine until a result register becomes zero.
- `frag_move_or_not_bchg_bsr_lea_subq_rts`: Short code fragment that copies 137 to d0, then merges a location in the code into d0, then inverts d0, then copies d0 to a fixed memory address, then flips bit 0 of d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then subtracts 1 from the memory at a0, then returns.
- `frag_move_or_not_bchg_bsr_lea_addi_rts`: Short code fragment that copies 139 to d0, then merges a location in the code into d0, then inverts d0, then copies d0 to a fixed memory address, then flips bit 0 of d0, then copies d0 to a fixed memory address, then calls another routine, then loads the address of a location in the code into a0, then adds 1 to the memory at a0, then returns.
- `check_and_call_routine`: Reads a value from a hardware register, checks if a specific bit is clear, and if so, repeatedly calls another routine until the bit becomes set.
- `initialize_and_process_dma`: Initializes DMA settings and processes data blocks while checking for specific conditions, updating counters and returning based on loop iterations.
- `compare_strings_ignoring_case`: Compares two zero-terminated strings byte by byte, converting lowercase letters to uppercase before comparison, and returns one if the strings match or zero otherwise.
- `process_dma_colors`: Initializes DMA settings, processes color data by writing values to hardware registers, and updates counters while checking for specific conditions.
- `update_dma_counter`: Decrements a counter and checks if it reaches zero, then performs arithmetic operations on memory addresses and calls another routine with the results.
- `chain_two_routine_calls`: Calls two other routines in sequence and returns without performing any additional operations.
- `frag_move_andi_beq_bset_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 4096, then branches if the last result was zero, then masks d0 with 255, then sets bit 9 of d0, then copies d0 to a fixed memory address, then returns.
