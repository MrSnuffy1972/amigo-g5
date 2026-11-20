# Game 3129 (chubby gristle (1988)(grandslam)[t]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03129). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (3 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 15 of 15 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 154 of 154 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 592 |
| data | 223019 |
| padding | 925 |
| unused | 676352 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Chubby Gristle (1988)(Grandslam)[t].adf:cg | 2 | 144 | 11 |
| Chubby Gristle (1988)(Grandslam)[t].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- chubby gristle (1988)(grandslam)[t]/Chubby Gristle (1988)(Grandslam)[t].adf:cg: BK: ByteKiller stream found by ancient scan (205500 -> 374016 bytes) but only 0 of 121 sampled windows are in reference RAM: not used
- chubby gristle (1988)(grandslam)[t]/Chubby Gristle (1988)(Grandslam)[t].adf:cg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware` (cg): Sets up hardware registers including enabling DMA and interrupts, configures display and memory pointers, then jumps to another routine. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected disk hardware signature.

## Functions


### Chubby Gristle (1988)(Grandslam)[t].adf:cg

- `prompt_user_input`: Opens system libraries, displays a trainer prompt message to the user, reads input character, processes response, and jumps to another routine based on input.
- `initialize_hardware`: Sets up hardware registers including enabling DMA and interrupts, configures display and memory pointers, then jumps to another routine.
- `frag_cmpa_bcs_movea_move_bne_jmp`: Short code fragment that compares a0 with 18944, then branches if the carry flag is set, then copies 18944 to a2, then copies the memory at a0 (advancing a0) to the memory at a2 (advancing a2), then compares a0 with a1, then branches if the last result was not zero, then jumps to another point in the code.
- `decompress_and_copy_data`: Copies data from one memory location to another while performing bit manipulation operations, handles conditional execution based on bit values, and manages data flow through multiple buffers.
- `frag_moveq_bra`: Short code fragment that loads 8 into d1, then loads 8 into d4, then jumps to another point in the code.
- `read_and_process_bits`: Reads bit values using a helper routine with different bit counts, compares results against thresholds, and branches to different code paths based on comparison outcomes.
- `update_dma_control`: Updates DMA control registers and manages a loop that modifies memory locations based on decrementing counters, then performs conditional jumps based on comparison results.
- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `handle_keyboard_input`: Reads keyboard data from CIA registers, updates color values, sets interrupt request flags, and returns the original value in a register.

### Chubby Gristle (1988)(Grandslam)[t].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
