# Game 2780 (AztecChallenge v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02780). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (477561 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (9 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 267 of 267 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10923 |
| code | 754 |
| data | 465884 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 80 | 9 |
| AztecChallenge | 1 | 187 | 7 |

## Cases handled

- AztecChallenge_v1.0/AztecChallenge_v1.0.hdf: hard-disk image (AmigaDOS OFS, 512 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree AztecChallenge_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- AztecChallenge_v1.0/AztecChallenge_v1.0_hdf/AztecChallenge/data/AztecChallenge: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then calls another routine, then jumps to the address in a6.
- `frag_lea_move_rts`: Short code fragment that loads the address of a location in the code into a3, then copies a3 to the memory at an offset from a3, then loads the address of a location in the code into a3, then copies a3 to d1, then returns.
- `initialize_dos_library`: Opens the dos.library and sets up a message port for system communication, then waits for and processes incoming messages.
- `handle_message_port`: Processes a message from a port by clearing specific bytes in the message structure and updating a task pointer before executing a jump table.
- `frag_move_movea_jsr_bra`: Short code fragment that copies d0 to the memory at an offset from a4, then copies the memory at an offset from a4 to a0, then copies the memory at a4 to a1, then copies a5 to the memory at an offset from a4, then calls another routine, then jumps to another point in the code.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.

### AztecChallenge

- `frag_jsr_jmp`: Short code fragment that calls another routine, then jumps to another point in the code.
- `decode_and_process_data`: Decodes data through multiple conditional branches, performs arithmetic operations, handles loops with decrementing counters, and manages memory addressing for data processing.
- `load_and_shift_data`: Loads data from memory, performs bit operations, stores results to a register, and executes a return instruction.
- `process_bit_data`: Processes bit manipulation operations including shifts, conditional branches, subroutine calls, and loop control for data handling.
- `enable_interrupts_and_copy`: Enables interrupts by setting bit 15 of a hardware register, then copies data from one memory location to another using address pointers and performs a conditional jump based on a comparison between two address pointers.
- `frag_move_lea_cmpi_bcs_lsr_subq_bne_jmp`: Short code fragment that copies a0 to d0, then loads the address of a fixed memory address into a1, then compares d0 with 78848, then branches if the carry flag is set, then shifts d7 right by 1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 1 from d7, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_adda_subq_lsr_move_bne_jmp`: Short code fragment that adds d7 to a0, then adds d7 to a1, then subtracts 4 from a0, then subtracts 4 from a1, then shifts d7 right by 1, then copies the memory just below a0 (moving a0 down) to the memory just below a1 (moving a1 down), then subtracts 1 from d7, then branches if the last result was not zero, then jumps to another point in the code.
