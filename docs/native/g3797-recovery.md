# Game 3797 (german jack (19xx)(r. noch)[hd][cardsets]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03797). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6 of 6 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10 of 10 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 359168 |
| code | 26 |
| data | 29985 |
| padding | 1477 |
| unused | 510464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| German Jack (19xx)(R. Noch)[HD][cardsets].adf:bootblock | 1 | 10 | 1 |

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### German Jack (19xx)(R. Noch)[HD][cardsets].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
