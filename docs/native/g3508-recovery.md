# Game 3508 (eco - a game of survival (1987)(ocean)[cr defjam][ks1.2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03508). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902832 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 14 of 14 named (0 author symbols), 14 of 14 described (5 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 261 of 261 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 329 of 329 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 87158 |
| code | 1196 |
| data | 10934 |
| padding | 728 |
| unused | 802816 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ECO - A Game of Survival (1987)(Ocean)[cr Defjam][KS1.2].adf:DEFJAM | 1 | 185 | 5 |
| ECO - A Game of Survival (1987)(Ocean)[cr Defjam][KS1.2].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 43 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 79 | 3 |

## Cases handled

- runtime segment an address (play-rs-m1500): 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 77 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- eco - a game of survival (1987)(ocean)[cr defjam][ks1.2]/ECO - A Game of Survival (1987)(Ocean)[cr Defjam][KS1.2].adf:DEFJAM: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_graphics` (DEFJAM): Allocates memory at a fixed address, initializes hardware registers including interrupt and DMA control, copies data from a source buffer to a destination buffer, and sets up coprocessor pointers for graphics rendering. Kind: disk hardware check. Pass path: The routine initializes graphics and sets up custom hardware (CIA, CUSTOM registers), then checks CIAA_PRA until bit 6 is clear. If this check passes (i.e., the bit becomes zero), it continues execution normally. The presence of CIA registe.

## Functions


### ECO - A Game of Survival (1987)(Ocean)[cr Defjam][KS1.2].adf:DEFJAM

- `initialize_graphics_2`: Allocates memory at a fixed address using a system call, initializes hardware registers including interrupt and DMA control, and copies data from a source buffer to a destination buffer.
- `decode_bitstream`: Processes compressed bit data from a source buffer, shifts bits right with carry, and writes decoded bytes to a destination buffer.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.
- `initialize_graphics`: Allocates memory at a fixed address, initializes hardware registers including interrupt and DMA control, copies data from a source buffer to a destination buffer, and sets up coprocessor pointers for graphics rendering.

### ECO - A Game of Survival (1987)(Ocean)[cr Defjam][KS1.2].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_frame_routines_2`: Calls multiple routines in sequence, updates a coprocessor pointer with a fixed address, and then calls additional routines to handle frame processing.
- `execute_frame_routines`: Calls multiple routines in sequence, updates a coprocessor pointer with a fixed address, and then calls additional routines to handle frame processing and rendering operations.
- `frag_move_movea_cmpa_bne_rts`: Short code fragment that copies 0 to d0, then copies an address to a0, then copies the memory at a0 (advancing a0) to d0, then compares a0 with an address, then branches if the last result was not zero, then returns.
- `frag_jsr_movea_move_lea_rts`: Short code fragment that calls another routine, then copies a fixed memory address to a6, then copies 5 to d0, then loads the address of a fixed memory address into a1, then calls another routine, then copies 7 to a fixed memory address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move`: Short code fragment that copies an address to a fixed memory address, then copies an address to a fixed memory address (2 times in a row), then copies a4 to d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_tile_data`: Processes tile data by reading from memory locations, performing bit manipulation operations on coordinates, calculating offsets based on global variables, and updating a table with computed values.
- `update_animation_state`: Reads animation frame data from memory, calculates coordinate offsets using lookup tables, updates global variables with computed values, and performs bit manipulation operations on frame coordinates.
- `frag_clr_bsr_nop_bra`: Short code fragment that clears a fixed memory address (2 times in a row), then calls another routine, then pauses for one no-operation step, then jumps to another point in the code.
