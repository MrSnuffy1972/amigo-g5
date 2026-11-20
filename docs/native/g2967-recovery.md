# Game 2967 (Brainball v1.00): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02967). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (822676 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 37 of 37 named (0 author symbols), 37 of 37 described (34 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 189 of 189 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 80690 |
| code | 516 |
| data | 741470 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| BRAINBALL | 1 | 187 | 36 |

## Cases handled

- Brainball_v1.00/Brainball_v1.00.hdf: hard-disk image (AmigaDOS OFS, 1024 blocks): its filesystem parses and all 55 files in it are byte-identical to the extracted tree Brainball_v1.00_hdf, which is recovered file by file; no block outside the filesystem holds data
- Brainball_v1.00/Brainball_v1.00_hdf/Brainball/bdat/BRAINBALL: Double Action 1.0 (xfd) stream found by ancient scan (215968 -> 505212 bytes) but only 0 of 128 sampled windows are in reference RAM: not used
- Brainball_v1.00/Brainball_v1.00_hdf/Brainball/bdat/BRAINBALL: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware` (BRAINBALL): Sets up hardware registers by writing specific values to control DMA and interrupts, configures CIAB port B with a sequence of bytes, and enables a bit in CIAA port A before copying a block of code from one location to another. Kind: disk hardware check. Pass path: The routine initializes hardware and performs a jump through an unrecovered jumptable, suggesting a check on CIA registers (e.g., CIAB_PRB) that may be part of a disk or track validation mechanism. The check likely passes if the hardware st.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### BRAINBALL

- `initialize_hardware`: Sets up hardware registers by writing specific values to control DMA and interrupts, configures CIAB port B with a sequence of bytes, and enables a bit in CIAA port A before copying a block of code from one location to another.
- `frag_movea_move_sub_cmpa_bcs_subq_bne_b`: Short code fragment that copies a location in the code to a1, then copies a location in the code to d0, then subtracts a1 from d0, then compares a0 with a1, then branches if the carry flag is set, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 4 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_adda_move_subq_bne_bra`: Short code fragment that adds d0 to a0, then adds d0 to a1, then copies the memory just below a0 (moving a0 down) to the memory just below a1 (moving a1 down), then subtracts 4 from d0, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_subq_move_lea_moveq_jmp`: Short code fragment that subtracts 1 from d6, then copies d6 to d3, then loads the address of a location in the code into a6, then loads 8 into d4, then jumps to the address in a4.
- `frag_move_dbra_lea_bra`: Short code fragment that copies d6 to the memory just below a2 (moving a2 down), then loops back while counting down d3, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_bra`: Short code fragment that tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_moveq_lea_jmp`: Short code fragment that loads 3 into d4, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_addi_bra`: Short code fragment that adds 6 to d6, then jumps to another point in the code.
- `frag_moveq_lea_jmp_2`: Short code fragment that loads 10 into d4, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_moveq_bra`: Short code fragment that loads 16 into d4, then jumps to another point in the code.
- `initialize_and_process`: Initializes multiple pointers and processes data through a loop that shifts bits and updates a global variable, then jumps to another routine based on the processed value.
- `frag_cmp_bcs_beq_lea_jmp`: Short code fragment that compares d6 with 6, then branches if the carry flag is set, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_lea_jmp`: Short code fragment that loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_beq_lea_moveq_jmp`: Short code fragment that compares d6 with 2, then branches if the carry flag is set, then branches if the last result was zero, then loads the address of a location in the code into a6, then loads 1 into d4, then jumps to the address in a4.
- `frag_moveq_jmp`: Short code fragment that loads 2 into d4, then jumps to the address in a4.
- `frag_moveq_bra_2`: Short code fragment that loads 3 into d3, then jumps to another point in the code.
- `frag_moveq_tst_beq_bra`: Short code fragment that loads 1 into d3, then tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_tst_beq_lea_bra`: Short code fragment that tests d6, then branches if the last result was zero, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_beq_bra_2`: Short code fragment that tests d6, then branches if the last result was zero, then jumps to another point in the code.
- `frag_cmp_bcs_lea_jmp`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_lea_jmp_2`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads the address of a location in the code into a6, then jumps to the address in a5.
- `frag_cmp_bcs_moveq_lea_jmp`: Short code fragment that compares d6 with 3, then branches if the carry flag is set, then loads 8 into d4, then loads 10 into d3, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_add_lea_bra`: Short code fragment that adds d6 to d3, then loads the address of a location in the code into a6, then jumps to another point in the code.
- `frag_tst_bne_moveq_bra`: Short code fragment that tests d6, then branches if the last result was not zero, then loads 7 into d4, then loads 2 into d2, then jumps to another point in the code.
- `frag_moveq_move_lea_jmp`: Short code fragment that loads 8 into d4, then copies 130 to d2, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_add_movea_adda_move_dbra_bra`: Short code fragment that adds d2 to d6, then copies a2 to a6, then adds d6 to a6, then copies the memory just below a6 (moving a6 down) to the memory just below a2 (moving a2 down), then loops back while counting down d3, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 7 into d3, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 4 into d3, then jumps to another point in the code.
- `frag_moveq_cmp_bcs_beq_bra`: Short code fragment that loads 2 into d3, then compares d6 with 1, then branches if the carry flag is set, then branches if the last result was zero, then loads 3 into d3, then jumps to another point in the code.
- `frag_moveq_bra_5`: Short code fragment that loads 0 into d6, then jumps to another point in the code.
- `frag_lsl_beq_roxl_dbra_jmp`: Short code fragment that shifts d5 left by 1, then branches if the last result was zero, then rotates d6 left through the extend flag by 1, then loops back while counting down d4, then jumps to the address in a6.
- `frag_cmpa_beq_move_roxl_bra`: Short code fragment that compares a0 with a1, then branches if the last result was zero, then copies the memory just below a0 (moving a0 down) to d5, then copies d5 to the memory at an offset from a3, then rotates d5 left through the extend flag by 1, then jumps to another point in the code.
- `copy_and_compare`: Copies data between memory locations while advancing pointers, compares values from registers with memory contents, and branches based on comparison results to handle different conditions.
- `frag_move_movea_jmp`: Short code fragment that copies d0 to the memory just below a2 (moving a2 down), then copies a location in the code to a2, then copies 0 to the memory at an offset from a4 (2 times in a row), then jumps to the address in a2.
- `frag_move_subq_bne_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d2, then copies the memory just below a0 (moving a0 down) to d1, then copies d1 to the memory just below a2 (moving a2 down), then subtracts 1 from d2, then branches if the last result was not zero, then copies d1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_moveq_move_bra`: Short code fragment that loads 0 into d1, then copies the memory just below a0 (moving a0 down) to d2, then jumps to another point in the code.
