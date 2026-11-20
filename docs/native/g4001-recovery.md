# Game 4001 (IndianaJones&TheFateOfAtlantisAction v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04001). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2053900 top-level bytes in 14 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 29 of 29 named (0 author symbols), 29 of 29 described (18 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 309 of 309 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21164 |
| code | 780 |
| data | 2028697 |
| padding | 2235 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Indy-IV | 6 | 230 | 17 |
| game.slave | 1 | 69 | 11 |
| disk.1:bootblock | 1 | 10 | 1 |

## Cases handled

- IndianaJones&TheFateOfAtlantisAction_v1.0/IndianaJones&TheFateOfAtlantisAction_v1.0_hdf/IndianaJonesAtlantisAct/disk.1:Indy IV/disk_00.cnc: CNC data cruncher (xfd)-packed, unpacked with ancient (786219 -> 3736 bytes)
- IndianaJones&TheFateOfAtlantisAction_v1.0/IndianaJones&TheFateOfAtlantisAction_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree IndianaJones&TheFateOfAtlantisAction_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- IndianaJones&TheFateOfAtlantisAction_v1.0/IndianaJones&TheFateOfAtlantisAction_v1.0_hdf/IndianaJonesAtlantisAct/disk.1:Indy IV/Indy-IV: Imploder (xfd) stream found by ancient scan (50784 -> 111660 bytes) but only 0 of 49 sampled windows are in reference RAM: not used
- IndianaJones&TheFateOfAtlantisAction_v1.0/IndianaJones&TheFateOfAtlantisAction_v1.0_hdf/IndianaJonesAtlantisAct/disk.1:Indy IV/Indy-IV: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Indy-IV

- `free_memory_blocks_loop`: Calls one routine, then calls the exec library's memory-free routine in a loop of three passes.
- `frag_move_lea_movea_adda_dbra_addq_rts`: Short code fragment that copies d6 to d7, then loads the address of a location in the code into a1, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d7, then adds 4 to a1, then returns.
- `process_value_loop`: Processes a value through a loop that iterates twice, performing arithmetic operations and conditional subroutine calls based on the value's sign.
- `process_and_accumulate`: Processes a value based on its sign and performs conditional operations including subroutine calls, arithmetic calculations, and memory writes.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_pattern_data`: Processes pattern data by decoding bits from a buffer and copying decoded bytes to another location while updating counters and pointers.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern`: Processes bits from a buffer using bit manipulation to decode pattern data, updating counters and pointers while handling carry flags during arithmetic operations.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_pattern_loop`: Processes bits from a buffer using bit manipulation and stores decoded bytes into another buffer while updating multiple counters and pointers.
- `decode_pattern`: Decodes a pattern from a buffer using bit manipulation and stores decoded bytes into another buffer while updating multiple counters and pointers.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `compare_and_patch`: Compares bytes from two memory locations and patches a code structure when the bytes match.
- `frag_move_lea_rts`: Short code fragment that copies 20217 to the memory at a1 (advancing a1), then loads the address of a location in the code into a2, then copies a2 to the memory at a1 (advancing a1), then returns.
- `frag_movem_movea_adda_move_jmp`: Short code fragment that saves registers on the stack, then copies a1 to a0, then adds 36780 to a0, then copies 20081 to the memory at a0, then restores saved registers from the stack, then jumps to the address in a1.
- `setup_and_execute`: Sets up global variables with addresses from the code segment, then calls several functions through function pointers at specific offsets.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_movem_move_tst_beq_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then saves registers on the stack, then copies the memory at a1 to d0, then tests d1, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then calls another routine, then jumps to the address in a6.
- `frag_move_bra`: Short code fragment that copies 65537 to d1, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 280 to d1, then jumps to another point in the code.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.

### disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
