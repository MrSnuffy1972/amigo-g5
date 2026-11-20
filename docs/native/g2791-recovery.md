# Game 2791 (backlash (1987)(novagen)[cr lord vader]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02791). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 4 of 4 named (0 author symbols), 4 of 4 described (2 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 65 of 65 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 119 of 119 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 78 |
| code | 606 |
| data | 397707 |
| padding | 969 |
| unused | 501760 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Backlash (1987)(Novagen)[cr Lord Vader].adf:backlash | 2 | 109 | 3 |
| Backlash (1987)(Novagen)[cr Lord Vader].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Backlash (1987)(Novagen)[cr Lord Vader].adf:backlash

- `initialize_graphics`: Initializes graphics hardware registers including bitplane modes, display window settings, and color registers.
- `frag_move_movea_cmpa_bne_jmp`: Short code fragment that copies 9984 to the status register, then copies 32767 to a fixed memory address, then copies 24576 to a7, then copies 522 to a0, then copies 1024 to a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then compares a0 with 580, then branches if the last result was not zero, then jumps to another point in the code.
- `copy_palette_data`: Copies palette data from a source buffer to a destination address, handling byte-by-byte transfer with loop control.

### Backlash (1987)(Novagen)[cr Lord Vader].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
