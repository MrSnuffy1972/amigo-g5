# Game 3190 (ConanTheCimmerian v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03190). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (8234160 top-level bytes in 107 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 49 of 49 named (0 author symbols), 49 of 49 described (25 by rule, 18 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 43186 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1401 of 1401 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3066981 |
| code | 4134 |
| data | 5076022 |
| padding | 22511 |
| unused | 64512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Stack | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Disk.1:Conan | 7 | 250 | 17 |
| Disk.1:Game | 7 | 250 | 17 |
| game.slave | 1 | 72 | 9 |
| Disk.1:bootblock | 1 | 10 | 1 |
| Disk.1:DIALOG.RES (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1: file game.dat (header 22528 bytes, blocks hold 22936): its blocks are judged as raw blocks outside the filesystem
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0.hdf: hard-disk image (AmigaDOS OFS, 8864 blocks): its filesystem parses and all 12 files in it are byte-identical to the extracted tree ConanTheCimmerian_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1:Conan: Imploder (xfd) stream found by ancient scan (38748 -> 81948 bytes) but only 0 of 39 sampled windows are in reference RAM: not used
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1:Game: Imploder (xfd) stream found by ancient scan (90352 -> 208560 bytes) but only 0 of 100 sampled windows are in reference RAM: not used
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1:DIALOG.RES: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1: blocks 1160-1198 (bytes 593920-613888) outside the filesystem hold data, not code (opcode density 0.014, entropy 3.87)
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1: blocks 1201-1202 (bytes 614912-615936) outside the filesystem hold data, not code (opcode density 0.020, entropy 4.08)
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1: blocks 1204-1209 (bytes 616448-619520) outside the filesystem hold data, not code (opcode density 0.014, entropy 4.13)
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.2: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.3: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.4: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1:Conan: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ConanTheCimmerian_v1.0/ConanTheCimmerian_v1.0_hdf/ConanTheCimmerian/data/Disk.1:Game: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 43186 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Stack

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Port-Handler

- `validate_hunk_type`: Compares loaded hunk values with AmigaDOS executable and relocation type codes, then initializes various data structure offsets based on the comparison results.

### Disk-Validator

- `initialize_disk_validator`: Sets up disk validation parameters including counters and offsets, initializes data structures, and prepares for disk read operations.

### Ram-Handler

- `frag_ori_movem_movea_move_jsr_suba_jmp`: Short code fragment that merges 12 into d0, then saves registers on the stack, then copies d1 to a0, then copies d2 to a1, then copies d3 to d0, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then subtracts a0 from a0, then jumps to the address in a6.

### Disk.1:Conan

- `run_hdinstall_entry`: Program entry for the HDinstall tool.
- `frag_move_lea_movea_adda_dbra_addq_rts`: Short code fragment that copies d6 to d7, then loads the address of a location in the code into a1, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d7, then adds 4 to a1, then returns.
- `process_loop_with_conditions_2`: Processes data in a loop with conditional branches based on sign and equality checks, calling another routine multiple times and performing operations including arithmetic and memory access.
- `process_loop_with_conditions`: Processes data in a loop with conditional branches based on sign and equality checks, calling another routine multiple times and performing arithmetic operations on values stored in memory.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_pattern_data`: Processes pattern data by decoding bits from a buffer using shift and add operations, copying decoded bytes to another location while updating counters and handling carry flags.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_pattern_bits_2`: Processes bits from a buffer using shift and add operations, decodes pattern data, and copies decoded bytes to another location while updating a counter.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_pattern_bits`: Processes bits from a buffer using shift and add operations, decodes pattern data, and copies decoded bytes to another location while updating counters and pointers.
- `decode_pattern`: Decodes a pattern from a buffer, processes bits using shift and add operations, and copies decoded bytes to another location while updating counters and pointers.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Disk.1:Game

- `run_hdinstall_entry_2`: Program entry for the HDinstall tool.
- `frag_move_lea_movea_adda_dbra_addq_rts_2`: Short code fragment that copies d6 to d7, then loads the address of a location in the code into a1, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d7, then adds 4 to a1, then returns.
- `initialize_and_process`: Initializes a counter and processes data through multiple subroutine calls while updating registers and memory locations based on conditional checks, then clears cache if a version condition is met.
- `process_data_loop`: Processes data in a loop based on conditional checks, updating values in memory and calling another routine multiple times before branching to different code paths.
- `frag_beq_movea_bsr_bra_2`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts_2`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_pattern_data_2`: Processes pattern data by copying bytes from a source buffer to a destination while decoding bits using shift and add operations, updating counters and handling carry flags during bit manipulation.
- `shift_and_accumulate`: Processes bits from a buffer using shift and add operations, decoding pattern data and copying decoded bytes to another location while updating counters.
- `frag_moveq_bra_4`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_6`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_pattern_bits_3`: Processes bits from a buffer using shift and add operations, decodes pattern data, and copies decoded bytes to another location while updating counters.
- `frag_add_bne_move_addx_bra_2`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_bit_pattern`: Processes bits from a buffer using shift and add operations, decodes pattern data, and copies decoded bytes to another location while updating counters and pointers.
- `decode_pattern_bits_4`: Processes pattern data by decoding bits from a buffer using shift and add operations, copying decoded bytes to another location while updating pointers and counters.
- `frag_move_movem_bra_2`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_lea_bsr_nop`: Short code fragment that loads the address of a location in the code into a1, then calls another routine, then loads the address of a location in the code into a1, then calls another routine, then loads the address of a location in the code into a1, then pauses for one no-operation step.
- `compare_length_prefixed_bytes_and_recurse`: Uses d0 as an index into a table, compares a counted byte string at a1 against it, and on a full match calls the routine at the d1-indexed entry before restoring d0 from d7.
- `validate_and_modify_data_2`: Checks a specific word value at a calculated offset from a base address and modifies bytes, words, or longwords based on the comparison result.
- `validate_and_modify_data`: Checks specific longword values at calculated offsets from a base address and modifies bytes, words, or longwords based on comparison results.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_movem_move_tst_beq_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then saves registers on the stack, then copies the memory at a1 to d0, then tests d1, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then calls another routine, then jumps to the address in a6.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
