# Game 2938 (BobsGarden v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02938). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (162583 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 9 of 9 named (0 author symbols), 9 of 9 described (5 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 225 of 225 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16046 |
| code | 556 |
| data | 145979 |
| padding | 2 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| BobsGarden | 2 | 216 | 6 |
| game.slave | 1 | 9 | 3 |

## Cases handled

- BobsGarden_v1.0/BobsGarden_v1.0.hdf: hard-disk image (AmigaDOS OFS, 192 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree BobsGarden_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- BobsGarden_v1.0/BobsGarden_v1.0_hdf/BobsGarden/BobsGarden: LeCrunch (xfd) stream found by ancient scan (37716 -> 77064 bytes) but only 0 of 32 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- BobsGarden_v1.0/BobsGarden_v1.0_hdf/BobsGarden/BobsGarden: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### BobsGarden

- `initialize_and_process`: Initializes system resources, allocates memory, processes data through multiple subroutines, and manages a main processing loop with various conditional branches.
- `frag_movea_subq_bcs_adda_bra_addq_rts`: Short code fragment that copies the memory at a5 to a1, then subtracts 4 from a1, then subtracts 1 from d1, then branches if the carry flag is set, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then jumps to another point in the code, then adds 4 to a1, then returns.
- `conditional_copy_loop`: Performs conditional copying operations between memory locations based on comparison results, with nested loops and memory address calculations.
- `frag_add_bne_move_addx_rts`: Short code fragment that adds d3 to d3, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d3, then adds d3 and the extend flag to d3, then returns.
- `loop_with_delay`: Executes a loop that decrements a counter and calls another routine until the counter reaches zero, while performing some graphics register operations.
- `copy_until_match`: Copies bytes from one buffer to another until a matching value is found, then continues copying a specified number of additional bytes.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_ori_pea_bra`: Short code fragment that merges 0 into d2, then merges 0 into d0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
