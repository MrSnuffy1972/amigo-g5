# Game 2770 (AuntArcticAdventure v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02770). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2038853 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (4 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 130816 bytes, 9 functions |
| Independent decoder cross-check (Ghidra against capstone) | 184 of 184 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 776381 |
| code | 710 |
| data | 1261757 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| aunt_arctic_adventure (raw code file, load address unknown) | 1 | 182 | 9 |

## Cases handled

- AuntArcticAdventure_v1.0/AuntArcticAdventure_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 30 files in it are byte-identical to the extracted tree AuntArcticAdventure_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- AuntArcticAdventure_v1.0/AuntArcticAdventure_v1.0_hdf/AuntArcticAdventure/data/aunt_arctic_adventure: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 130816 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### aunt_arctic_adventure (raw code file, load address unknown)

- `frag_lea_bsr_bra`: Short code fragment that loads the address of a fixed memory address into a7, then calls another routine (7 times in a row), then jumps to another point in the code.
- `read_joystick_input`: Reads the joystick input from a hardware register, compares it against specific values, and updates a global variable based on the comparison results.
- `copy_animation_data`: Copies animation data from a table into specific memory locations using an index calculated from another global variable.
- `handle_input_events`: Reads input from hardware registers, compares the values against stored inputs, and calls another routine based on the comparison results.
- `initialize_coprocessor`: Initializes several hardware registers including coprocessor and DMA control registers with specific values to configure system interrupts and DMA channels.
- `frag_lea_clr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then clears the memory at a0, then clears the memory at an offset from a0 (7 times in a row), then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_block`: Clears a block of memory by setting each byte to zero, iterating through a specific number of bytes and locations.
- `copy_table_data`: Calculates an offset from a global table and copies several bytes from that table into a destination structure at specific intervals.
