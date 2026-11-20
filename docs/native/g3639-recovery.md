# Game 3639 (FinalCountdown v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03639). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2926222 top-level bytes in 265 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 13 of 13 named (0 author symbols), 13 of 13 described (10 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 123350 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 286 of 286 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 902100 |
| code | 830 |
| data | 2023289 |
| padding | 3 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 83 | 9 |
| Space | 2 | 203 | 4 |
| Stuff (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- FinalCountdown_v1.0/FinalCountdown_v1.0.hdf: hard-disk image (AmigaDOS OFS, 3424 blocks): its filesystem parses and all 263 files in it are byte-identical to the extracted tree FinalCountdown_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- FinalCountdown_v1.0/FinalCountdown_v1.0_hdf/FinalCountdown/data/gfx/Stuff: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- FinalCountdown_v1.0/FinalCountdown_v1.0_hdf/FinalCountdown/data/Space: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 123350 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_movem_move_tst_beq_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then saves registers on the stack, then copies the memory at a1 to d0, then tests d1, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then calls another routine, then jumps to the address in a6.
- `frag_lea_move_rts`: Short code fragment that loads the address of a location in the code into a3, then copies a3 to the memory at an offset from a3, then loads the address of a location in the code into a3, then copies a3 to d1, then returns.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `decode_string_entry`: Processes a string entry by shifting a register value, accessing memory locations, and comparing bytes until a delimiter is found.
- `frag_movea_move_bra`: Short code fragment that copies a1 to a0, then copies d2 to d0, then jumps to another point in the code.
- `compare_string_entries`: Compares a string from a buffer with entries in a table, advancing through the table until a match is found or the end is reached, then calls a function if the string matches an entry.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Space

- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `frag_lsl_bne_move_ori_roxl_rts`: Short code fragment that shifts d6 left by 1, then branches if the last result was not zero, then copies the memory at a4 (advancing a4) to d6, then merges 16 into the condition codes, then rotates d6 left through the extend flag by 1, then returns.
- `frag_moveq_lsl_bne_move_ori_roxl_dbra_r`: Short code fragment that loads 0 into d0, then shifts d6 left by 1, then branches if the last result was not zero, then copies the memory at a4 (advancing a4) to d6, then merges 16 into the condition codes, then rotates d6 left through the extend flag by 1, then rotates d0 left through the extend flag by 1, then loops back while counting down d5, then returns.
- `process_hunk_relocation`: Processes Amiga DOS hunk relocation data by reading type codes and handling different relocation scenarios based on comparisons with known hunk type values, while managing memory blocks and updating pointers accordingly.
