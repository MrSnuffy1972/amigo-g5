# Game 1733 (vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01733). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1392768 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 81930 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 163852 of 163855 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 163943 of 163956 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15003 |
| code | 491998 |
| data | 639495 |
| unused | 246272 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf boot-block load | 1 | 38 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 163855 | 1 |

## Cases handled

- vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]/Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf: custom-format (non-DOS) disk, 1277 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 124 64 KB pages each with >= 29491 distinct PCs executed, 2031616 of its 2215452 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 124 64 KB pages each with >= 29491 distinct PCs executed, 2031616 of its 2215452 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 124 64 KB pages each with >= 29491 distinct PCs executed, 2031616 of its 2215452 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 124 64 KB pages each with >= 29491 distinct PCs executed, 2031616 of its 2215452 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]/Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf: the boot block reads 15360 bytes from disk offset 653312 to an address (found statically); recovered as a unit
- runtime segment an address: 163851 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]/Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf: 1 logged loads cover 15360 bytes of the image
- vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]/Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf: blocks 853-853 (bytes 436736-437248) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.20)
- vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]/Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf: blocks 879-879 (bytes 450048-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.83)
- vroom (1991)(lankhor)[cr vf - company][h prx][t +9 atx]/Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf: blocks 1310-1310 (bytes 670720-671232) outside the filesystem hold data, not code (opcode density 0.039, entropy 1.64)
- 5 packed track region(s), 636928 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf:bootblock

- `configure_display_registers_at_boot`: Boot block code that writes the display window start and stop registers, the data fetch start and stop registers, the bitplane control and bitplane modulo registers, and the DMA control register.
- `swap_word_halves_then_call`: Issues three requests to the library routine at offset 456 using request-like structures, then swaps the two 16-bit halves of each of 256 consecutive 32-bit words in a buffer.

### logged disk load from Vroom (1991)(Lankhor)[cr VF - Company][h PRX][t +9 ATX].adf boot-block load

- `stub_no_decompiled_function`: The decompiler produced no function body for these 10 executed instructions, so no behaviour is recoverable from the evidence.
- `conditionally_call_two_routines`: Calls one unnamed routine when a condition on a data register is not met, and always calls a second unnamed routine.
- `update_status_and_store_word`: Stores a word from a register into a location pointed to by a5 and performs a bitwise OR on a byte at the address in a0 using the value 252.
- `call_routine_then_unreadable_tail`: Calls one unnamed routine, which copies register values into memory, and then part of the routine is unreadable in the decompiler output.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (ori.b, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_large_runtime_region`: A routine of an address instructions that the reference run executed, entered from outside the static flow.
