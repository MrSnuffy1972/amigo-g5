# Game 4716 (PipeDream v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04716). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (974281 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 5 of 5 named (0 author symbols), 5 of 5 described (2 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 365146 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 25 of 25 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 48880 |
| code | 78 |
| data | 925295 |
| padding | 28 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 6 | 2 |
| PipeDream Data (raw code file, load address unknown) | 1 | 19 | 3 |

## Cases handled

- PipeDream_v1.0/PipeDream_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1056 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree PipeDream_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- PipeDream_v1.0/PipeDream_v1.0_hdf/PipeDream/PipeDream Data: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 365146 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `empty_routine`: the routine performs no observable work and ends without modifying registers or memory.

### PipeDream Data (raw code file, load address unknown)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `setup_and_invoke`: Loads an address into a register, stores that address at a specific offset in memory, and then triggers a software interrupt.
- `initialize_dma_channels`: Sets the status register to disable interrupts, clears all data registers, writes a specific value to the DMA control register to enable multiple channels, and stores the contents of all data registers to a memory location.
