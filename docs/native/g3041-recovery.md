# Game 3041 (castle kingdoms (1997)(mutation software)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03041). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (1 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 10 of 10 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 234 of 234 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 648 |
| data | 754091 |
| padding | 1765 |
| unused | 144384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Castle Kingdoms (1997)(Mutation Software).adf:Kingdoms | 1 | 216 | 5 |
| Castle Kingdoms (1997)(Mutation Software).adf:bootblock | 1 | 18 | 1 |

## Cases handled

- castle kingdoms (1997)(mutation software)/Castle Kingdoms (1997)(Mutation Software).adf:GameData.wad: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (522240 -> 51200 bytes)
- castle kingdoms (1997)(mutation software)/Castle Kingdoms (1997)(Mutation Software).adf:Kingdoms: Titanics Cruncher 1.1 (xfd) stream found by ancient scan (176392 -> 354476 bytes) but only 0 of 116 sampled windows are in reference RAM: not used

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Castle Kingdoms (1997)(Mutation Software).adf:Kingdoms

- `initialize_game_memory`: Allocates memory for game structures, initializes system resources including library handles, and sets up data processing operations on allocated memory regions before returning control to the caller.
- `frag_movem_move_jsr_movea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d1, then copies a location in the code to d2, then copies 5120 to d3, then calls another routine, then restores saved registers from the stack, then copies a location in the code to a0, then returns.
- `process_tile_data`: Processes tile data by loading values into registers, performing bit shifts and comparisons, then copying bytes between memory locations based on conditional logic and loop counters.
- `frag_moveq_lsl_bne_bsr_roxl_dbra_rts`: Short code fragment that loads 0 into d2, then shifts d0 left by 1, then branches if the last result was not zero, then calls another routine, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `frag_move_cmpa_bne_bsr_roxl_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares a0 with a location in the code, then branches if the last result was not zero, then calls another routine, then copies 16 to the condition codes, then rotates d0 left through the extend flag by 1, then returns.

### Castle Kingdoms (1997)(Mutation Software).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
