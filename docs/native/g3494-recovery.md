# Game 3494 (DynastyWars v1.1 0862): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03494). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1853727 top-level bytes in 53 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 15 of 15 named (0 author symbols), 15 of 15 described (11 by rule, 4 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 99180 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 114 of 114 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 687264 |
| code | 540 |
| data | 1165916 |
| padding | 7 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| BOOT.PRG | 1 | 112 | 14 |
| CDYNASTY.PRG (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- DynastyWars_v1.1_0862/DynastyWars_v1.1_0862.hdf: hard-disk image (AmigaDOS OFS, 2080 blocks): its filesystem parses and all 51 files in it are byte-identical to the extracted tree DynastyWars_v1.1_0862_hdf, which is recovered file by file; no block outside the filesystem holds data
- DynastyWars_v1.1_0862/DynastyWars_v1.1_0862_hdf/DynastyWars/data/CDYNASTY.PRG: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 99180 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### BOOT.PRG

- `initialize_and_enable`: Initializes several system registers including a DMA control register and an interrupt enable register, then enters a loop that calls another routine and sets up the processor state.
- `increment_color_forever`: Increments a register value by a fixed amount and writes it to a hardware color register in an infinite loop without any conditional branches.
- `set_color_loop`: Writes a fixed color value to a hardware register and then enters an infinite loop that increments a counter and writes it to the same register.
- `process_data_and_update`: Calls multiple routines sequentially, checking for zero results after each call, and performs memory operations on global variables before updating a hardware register with an incrementing value.
- `frag_move_movea_jsr_tst_beq_rts`: Short code fragment that copies a fixed memory address to d0, then copies 2 to d1, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then tests d0, then branches if the last result was zero, then returns.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `frag_movea_clr_jsr_move_tst_rts`: Short code fragment that copies 582 to a1, then clears d0, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then tests d0, then returns.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `frag_move_movea_jsr_tst_rts`: Short code fragment that copies a fixed memory address to d1, then copies 1005 to d2, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then tests d0, then returns.
- `frag_move_movea_jsr_tst_rts_2`: Short code fragment that copies a fixed memory address to d1, then copies 1006 to d2, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then tests d0, then returns.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `frag_move_movea_jsr_tst_rts_3`: Short code fragment that copies a fixed memory address to d1, then copies a fixed memory address to d2, then copies a fixed memory address to d3, then copies a fixed memory address to a6, then calls another routine, then copies d0 to a fixed memory address, then tests d0, then returns.
- `frag_move_movea_jsr_tst_rts_4`: Short code fragment that copies a fixed memory address to d1, then copies a fixed memory address to d2, then copies a fixed memory address to d3, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `frag_move_andi_bne_rts`: Short code fragment that copies a fixed memory address to d7, then masks d7 with 16384, then branches if the last result was not zero, then returns.
