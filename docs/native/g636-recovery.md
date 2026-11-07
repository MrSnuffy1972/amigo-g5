# Game 636 (RocketRanger v1.5b 1641): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00636). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4111716 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 364 of 364 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 758 of 758 named (0 author symbols), 758 of 758 described (85 by rule, 671 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 8094 of 8410 (96.24%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16772 of 16772 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 138963 |
| code | 53810 |
| data | 3856629 |
| padding | 1898 |
| unused | 60416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| EndCLI | 2 | 117 | 2 |
| Disk.1:RR | 6 | 15697 | 701 |
| game.slave | 1 | 517 | 53 |
| Disk.1:bootblock | 1 | 10 | 1 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 9 | 1 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |

## Cases handled

- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.1: file KBlocks.0 (header 337702 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.1: file KBlocks.1 (header 392987 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: file KBlocks.0 (header 438719 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: file KBlocks.1 (header 438201 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641.hdf: hard-disk image (AmigaDOS OFS, 4416 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree RocketRanger_v1.5b_1641_hdf, which is recovered file by file; no block outside the filesystem holds data
- 5 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 360 logged WHDLoad disk loads recovered as units built from the disk image bytes
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.1: 231 logged loads cover 35840 bytes of the image
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.1: blocks 23-28 (bytes 11776-14848) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.51)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.1: blocks 80-671 (bytes 40960-344064) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.22)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.1: blocks 895-1670 (bytes 458240-855552) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.61)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: 129 logged loads cover 63488 bytes of the image
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 15-879 (bytes 7680-450560) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.29)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 903-938 (bytes 462336-480768) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.01)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 942-977 (bytes 482304-500736) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.66)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 980-980 (bytes 501760-502272) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.05)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 983-984 (bytes 503296-504320) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.57)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 989-991 (bytes 506368-507904) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.44)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 994-998 (bytes 508928-511488) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.67)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: blocks 1102-1758 (bytes 564224-900608) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.89)
- RocketRanger_v1.5b_1641/RocketRanger_v1.5b_1641_hdf/RocketRanger/Disk.2: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### EndCLI

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### Disk.1:RR

- `initialize_system_state`: Clears a block of memory, updates several global variables, and executes a series of function pointers based on conditional checks.
- `initialize_recursive_object_hierarchy`: Recursively allocates and initializes a tree-like structure of objects by fetching child nodes and setting up coordinate data.
- `swap_and_execute_callbacks`: Swaps two pairs of 32-bit values in memory, executes a loop of function calls through a pointer, and then calls another function using a value from a structure.
- `decrement_and_check_counter_2`: Decrements a 16-bit value in a structure and calls itself recursively if the value reaches zero.
- `clear_nested_pointers`: Calls a routine using a pointer from a structure and then clears that pointer, repeating this process recursively for several other pointers within the same structure.
- `process_object_hierarchy`: Recursively traverses linked lists to modify specific bytes in nodes and manages a loop that executes a function via a pointer while updating internal state variables.
- `call_subroutine_indirect`: Calls another routine using a pointer derived from a register and a constant offset.
- `allocate_and_populate_array`: Allocates a memory block and populates an array within that block by repeatedly calling another routine to fetch values.
- `call_subroutine_with_parameter`: calls another routine using a function pointer stored in a table with a specific constant passed as an argument.
- `pass_parameters_to_subroutine`: passes multiple arguments through to another routine.
- `blit_area_with_clipping`: Calculates clipping boundaries and source/destination offsets for a bitplane-based area transfer.
- `search_and_reorder_list`: Allocates a small memory block, searches a list for a match, and if found, updates the list structure and performs a recursive operation.
- `search_linked_list`: Traverses a linked list to find a node matching a specific criteria using a comparison function, returning the found node or a default pointer.
- `update_flag_and_multiply`: Sets a bit in a byte, then retrieves two 16-bit values from a nested structure, multiplies them, and passes the result to a function before clearing a value.
- `multiply_and_store_result_2`: Multiplies two short integers retrieved from a nested pointer structure and stores the result in a local variable before calling another routine.
- `multiply_and_store_result`: multiplies two short integers retrieved from a nested pointer structure and stores the result in a local variable before calling another routine and clearing a value in a table.
- `update_status_and_draw_batch`: modifies a byte in a structure, checks a condition involving a pointer and a zero value, and if met, calls another routine to perform a blitter operation and update a value in a table.
- `initialize_resource_pointer_2`: Checks if a value at an offset is zero, and if so, attempts to allocate memory and assign it to that offset before performing a conditional routine call.
- `process_conditional_array_element`: checks if a pointer is non-zero, calls another routine with an offset, and if a specific table value is non-zero, calls a third routine before returning a value from a fourth routine.
- `clear_pointer_after_call`: Calls a function using a pointer from a structure and then clears that pointer.
- `process_and_increment_node`: Calls two other routines, retrieves a value from a structure, increments it, and passes it to another function.
- `traverse_linked_list_5`: iterates through a linked list of pointers and calls another routine for each node.
- `check_range_overlap_2`: Compares the boundaries of two structures to determine if they overlap and stores their pointers in global variables if they do.
- `check_range_overlap`: compares the boundaries of two structures to determine if they overlap and stores their pointers in global variables if they do.
- `check_value_overlap`: Compares two sets of values to determine if their ranges overlap, storing two pointers in global variables if they do.
- `compare_and_store_registers`: Compares two short values and, if the condition is met, stores two registers into a global memory location.
- `store_registers_to_memory`: stores the contents of two registers into a global memory location.
- `traverse_linked_list_6`: Iterates through a linked list, checking a bit flag in a structure to decide whether to recurse or continue traversal.
- `traverse_list_with_flag_check`: Iterates through a linked list, checking a bit flag in a structure and calling another routine until a null pointer or a specific condition is met.
- `traverse_list_nested_loop`: Iterates through a linked list using nested loops and conditional checks on a bit flag to decide whether to call another routine.
- `traverse_linked_list`: iterates through a linked list, checking a bit flag in a structure and calling another routine until a null pointer or a specific condition is met.
- `return_zero`: This routine is empty and simply returns a value of zero to the caller.
- `traverse_linked_list_recursively`: Recursively traverses linked lists while checking bit flags in structures to perform conditional updates to global pointers.
- `traverse_conditional_list`: Iterates through a linked list and performs conditional checks on bit flags within structures to determine whether to update global pointers or return a status.
- `return_false_stub`: A one-instruction routine that loads zero into the return register and returns, with no other effect.
- `check_list_status`: Calls another routine and returns the result of a comparison against zero.
- `return_zero_constant_2`: This routine is empty and simply returns zero to its caller.
- `traverse_linked_list_7`: Iterates through a linked list or performs a recursive search based on a bit flag in a structure, potentially updating global pointers.
- `return_zero_constant_3`: This is an empty routine that simply returns zero to its caller.
- `traverse_linked_list_4`: Recursively traverses a linked list by following pointers until a null pointer is encountered, returning zero if the list is empty or one if a condition is met.
- `traverse_linked_list_2`: Recursively traverses a linked list by following pointers until a null pointer is encountered.
- `return_zero_2`: This routine is empty and simply returns zero to the caller.
- `test_bit_in_buffer`: Checks if a specific bit is set in a byte retrieved from a buffer at an offset determined by multiple function calls.
- `check_bit_flag`: Retrieves a byte from a memory location calculated via multiple pointers and performs a bitwise check against a value derived from a function call.
- `return_zero_3`: This routine performs no operations and returns zero to the caller.
- `clear_memory_table`: Sets several consecutive words in a memory location to zero and calls another routine.
- `copy_and_reset_data`: Copies a block of data from one table to another and resets specific short integer values in a structure if certain conditions are met.
- `pack_nibbles_to_table`: Reads bytes from a source table and packs their high nibbles or certain bit patterns into a destination table.
- `clear_destination_table`: Sets a range of memory worth of bytes to zero.
- `calculate_scaled_values`: Processes two source tables to generate a destination table of values by performing bitwise shifts and division by a global variable.
- `update_table_state`: Decrements global counters and either copies a source table to a destination or performs arithmetic and bitwise operations on table elements.
- `recursively_check_and_copy`: Recursively checks a byte flag in a structure and calls a routine to copy or process memory blocks if a bit is set.
- `copy_memory_block_2`: Copies a certain number of words from a source pointer to a destination pointer.
- `update_list_and_table`: Iterates through a list of structures to perform insertions or deletions in a byte array, then updates a table using values from another table.
- `initialize_byte_table`: Stores a sequence of bytes into a table where each byte is the index multiplied by two.
- `copy_data_blocks`: Copies four double-word blocks from a source table to a destination table based on an offset.
- `call_external_routine`: Calls a routine through a pointer with a specific argument.
- `increment_and_check_counter`: Increments a global variable and calls another routine if the variable was zero before the increment.
- `decrement_and_check_counter`: Decrements a global variable and calls another routine if the variable reaches zero, ensuring the value does not become negative.
- `calculate_clipping_offsets`: Calculates clipping boundaries and calls a routine to perform a bitplane-based area transfer.
- `allocate_and_insert_node`: Allocates a new memory block and inserts it into a linked list based on a comparison of a short integer value stored within the nodes.
- `find_matching_node`: Traverses a linked list and returns a pointer to a node if certain values in the node match the provided arguments.
- `remove_list_node`: Searches a linked list for a node matching specific criteria and removes it by updating pointers and calling a routine to free the memory.
- `traverse_and_free_list`: Recursively traverses a linked list and calls a routine to free each node.
- `execute_recursive_pointer_update`: Checks a value at a memory offset and calls a function via a pointer if the value is non-zero.
- `process_conditional_recursion_2`: Checks if a global pointer and a parameter are both non-zero before performing a recursive call.
- `check_and_clear_pointer`: Checks if a global pointer is non-zero, calls a routine using that pointer, and then clears the pointer.
- `allocate_and_populate_object_array`: Allocates a memory block based on the sum of two counts and populates it by iterating through two separate loops to fetch and store data from pointers.
- `validate_asset_result`: Checks the return value of another routine and returns one if it matches a specific constant, otherwise returns zero or one based on the result.
- `clear_and_process_array`: Clears a series of word pairs in an array and then calls a routine to process specific elements using a product of two short integers.
- `remove_node_from_list`: Searches a linked list for a specific value and removes the matching node by updating the preceding node's pointer.
- `swap_list_pointers`: Iterates through a linked list to find a match based on a comparison function and swaps pointers to reorder the list.
- `draw_blitter_sprite_batch`: Configures and executes multiple Blitter operations to draw a series of graphical elements.
- `cleanup_entity_state`: Executes several cleanup routines via pointers stored in a structure and clears various global pointers and flags.
- `execute_callback_list_2`: Iterates through a sequence of function pointers stored in a structure and executes each associated routine.
- `initialize_data_table`: Calls subroutines to retrieve values, performs calculations to update several memory locations, and clears a sequence of sixteen longwords in a table.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `compare_function_results`: Iterates through a null-terminated string and calls a function via a pointer for each character, breaking if the returned values differ.
- `return_immediately`: The routine is empty and immediately returns to its caller.
- `allocate_and_initialize_structure_2`: Allocates a memory block, sets several multi-byte values within that block, and updates a pointer to link it to another structure.
- `find_list_node`: Iterates through a linked list using a comparison function to locate a specific node.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `nop_padding_routine`: This routine is empty and performs no operations.
- `clear_structure_pointers`: Checks two pointers within a structure, calls a routine to process one if it is not null, and then clears both pointers.
- `remove_and_clear_node`: Removes a node from a linked list and performs cleanup on the structure associated with that node.
- `traverse_and_clear_list`: Iterates through a linked list, calling a routine on each node before moving to the next, and returns zero.
- `nop_padding_sequence`: This routine is empty and consists of non-operational instructions used as padding in the executable.
- `clamp_value_to_range`: Retrieves a value from a function and ensures it does not exceed a specified limit by adding an offset and applying a ceiling.
- `set_global_byte_via_pointer`: Sets a single byte in a global memory location to one.
- `insert_into_list`: Attempts to find or create a node in a list based on short integer comparisons and copies several fields from a source structure into the new node.
- `delete_list_node`: Calls another routine to remove a specific node from a linked list using a pointer and a value.
- `clear_list_pointer`: Traverses a linked list and sets the pointer of a specific list head to zero.
- `update_list_node_data`: Searches for a node in a list and, if found, copies two multi-byte values from a source structure into the node and clears two other fields.
- `initialize_object_properties`: Calls a function to obtain an object pointer and then populates several fields within that object using values returned by other routines.
- `update_object_high_word`: Extracts the upper 16 bits from a 32-bit value and stores them into a specific offset of an object, then clears a global variable.
- `calculate_value_difference`: Compares a 16-bit value derived from a 32-bit source against a global variable and, if they differ, stores the difference into an object field and clears a global variable.
- `execute_conditional_logic`: Checks a bit in a byte, then conditionally calls another routine with either one or two arguments and sets two global variables.
- `update_conditional_state`: Checks a bit in a byte and, if set, calls another routine and updates several global variables and two object fields with high-word values.
- `set_object_field_max`: Stores the value 65535 into a specific offset of an object.
- `store_high_word`: Extracts the upper 16 bits from a 32-bit value and stores them into a global variable.
- `copy_32bit_value`: Calls a function to obtain an object pointer and then stores a 32-bit value into a field within that object.
- `update_object_status_word`: Calls a function to obtain an object pointer and stores a 16-bit value into an object field, setting it to one if the value is zero.
- `store_dual_high_words`: Calls a function to obtain an object pointer and stores the upper 16 bits of two different 32-bit values into consecutive fields of that object.
- `set_global_byte`: Sets a single byte in a global memory location to one.
- `recurse_unconditionally`: The routine immediately calls itself, creating an infinite recursion loop.
- `set_global_byte_via_pointer_2`: Sets a single byte in a global memory location to one using a pointer retrieved from a data structure.
- `set_global_flag_via_pointer`: Sets a single byte in a global memory location to one using a pointer retrieved from a data table.
- `decrement_and_iterate_counter`: Decrements a value in a structure and sets a global byte based on whether the value is less than one, then iterates through a list using a counter until a limit is reached.
- `process_node_list`: iterates through a list of structures, performing different operations based on a bit flag and updating coordinate values within each structure.
- `allocate_and_initialize_object_node`: Allocates a memory block and a secondary object structure to store coordinate data and bitmask flags.
- `traverse_and_free_list_2`: Iterates through a linked list, calling specific routines to process or free data members within each node before clearing the list head.
- `search_table_for_values`: Decrements a counter and searches a table of short integers for a descending sequence of values, storing a pointer to the matching entry in a destination array.
- `search_conditional_table`: Searches a table for a value based on a parameter, potentially performing a second search in a different table if a specific bit is set in a byte within a structure.
- `load_and_parse_asset_data`: Loads a data block from a file, validates a magic signature, and parses a structured header to extract dimensions and memory allocation requirements.
- `expand_compressed_data_stream`: Decompresses a custom data stream by reading bytes and handling a specific escape character that indicates a repeated sequence.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `process_table_counters`: Increments a global counter and iterates through a list of four elements, decrementing each byte and calling a function if a byte reaches zero.
- `initialize_data_structure_2`: Calls a function via a pointer to retrieve a value, then initializes a series of bytes in a table and sets several fields in a structure, including a pointer to itself.
- `reset_global_pointer`: Calls two routines using pointers from a table and then sets a global variable to all ones.
- `initialize_and_update_entity`: Initializes an entity with specific state flags, animation parameters, and data pointers through a series of conditional checks.
- `initialize_entity_state`: Calls a routine with a pointer and specific parameters, then executes another routine twice before returning a value from a global variable.
- `clear_and_call_twice`: Sets a word in a table to zero and then calls a routine twice via a pointer.
- `traverse_linked_list_3`: Iterates through a linked list to find an entry matching specific byte criteria and returns a pointer to it.
- `process_entity_cleanup`: Retrieves two pointers from a global structure, executes a cleanup routine on the second pointer, and updates a status flag and data members based on the result.
- `reset_entity_state`: Checks for the presence of two active pointers and executes their associated cleanup routines before clearing the pointers and a status flag in a data structure.
- `perform_blt_operations`: Uses hardware registers to perform bitplane operations and iterates through a list to update global variables and structure members using values from the provided parameters.
- `update_table_data`: Loops through a list of data, calling a routine for each entry and writing values to a structure based on whether a numeric field is less than one.
- `update_data_structures`: Iterates through a list of pointers, updating a structure with values from the list and calling a routine for each entry.
- `get_active_object`: Attempts to retrieve an object from a table using an index, checking both the index and its neighbor, and returns a pointer to it.
- `update_sprite_dimensions`: Calculates new dimensions for an object based on a table and parameters, then updates its bounding box or visibility area.
- `manage_object_state`: Updates an object's properties, performs several function calls using values from a table, and enters a loop that waits for a specific condition to be met.
- `invalidate_object_reference`: Sets a value in a structure to all ones and attempts to find and invalidate an object from a table.
- `find_nearest_object`: Iterates through a table of coordinates to find the index of the object with the smallest Manhattan distance to a target position.
- `clear_status_table`: Calls a routine to clear a specific byte in a table.
- `zero_memory_block`: Sets several individual bytes and a word in a memory area to zero.
- `update_status_values`: Stores a byte from a parameter into a global variable and updates two other global bytes based on the value of that byte and the result of another routine.
- `increment_status_byte`: Calls a routine and stores the result incremented by one into a global byte.
- `update_status_byte`: Stores a byte from a parameter into a global variable and updates two other global bytes based on that value.
- `update_state_and_callback`: Stores a parameter into a data structure, increments a byte in memory, and executes a function through a pointer.
- `process_status_changes`: Decrements a byte in memory if certain conditions are met and updates a global byte based on another byte's value.
- `call_conditional_pointer`: Checks if a byte matches a specific value or has a specific bit set before calling a subroutine via a pointer.
- `execute_conditional_callback_4`: Checks a bit in a byte and, if set, calls a subroutine using a pointer retrieved from a data structure.
- `execute_six_instructions`: A routine consisting of six instructions that is called by three other routines.
- `accumulate_and_reset_value`: Adds the value from one memory location to another and then clears the source location.
- `copy_memory_word`: Copies a word from one memory location to another.
- `evaluate_conditional_bounds`: Compares a sum of two values against a third value and checks a short value to return a boolean result.
- `tiny_moveq_unlk_rts`: Small routine of 3 instructions that loads 0 into d0, then removes the stack frame held in a5, then returns.
- `update_entity_state_machine_2`: Manages an entity's state by checking a status field and executing a switch-based logic branch.
- `update_entity_state_machine`: Increments a state variable within an entity structure and executes specific logic branches based on the current state.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `update_entity_state_and_position`: Processes entity logic through a switch-based state machine that updates coordinate values, modifies timer or counter variables, and calls routines to handle sprite or object positioning and rendering.
- `increment_and_call_function`: Increments a short value at a memory location and then calls a function using parameters retrieved from a pointer.
- `evaluate_conditional_recursion`: Retrieves an object from a table and performs a series of conditional function calls.
- `return_from_subroutine_2`: performs a return from a subroutine.
- `xor_buffer_with_rotated_value`: Iterates through a memory buffer, performing a bitwise XOR on each byte with a value that rotates left by fifteen bits each iteration.
- `update_status_from_function`: Clears a byte, checks a value within a specific range, and if so, calls a function to update a bit in a table.
- `increment_table_entry`: Calls a function and stores the returned value plus ten into a table at an offset determined by the input parameter.
- `clear_table_entries`: Clears several fields in a table at an offset determined by an input parameter and updates a specific entry based on a function return value.
- `adjust_value_based_on_comparison`: Compares values from two tables and increments or decrements a value in one table based on the result of the comparison.
- `update_value_and_accumulator`: Subtracts a value from a table entry and adds that value to a global accumulator, potentially performing a second subtraction and addition if a specific condition is met.
- `store_masked_value`: Combines a parameter with a value from a function call and stores the result into a table using a bit-shifted index.
- `find_nearest_index`: Iterates through a table to find the index of an entry that minimizes the combined distance from two coordinate sets, updating a global variable with the result.
- `calculate_state_value`: Determines a value based on various conditions and table lookups, then recursively calls itself with the calculated value.
- `decrement_and_clamp`: Subtracts a value from a global variable and ensures the result does not fall below a specific minimum threshold.
- `accumulate_and_reset`: Adds a table value to a global variable, applies an offset, clamps the result to a maximum value, and then resets the original table entry to zero.
- `initialize_input_buffer_and_mapping`: Clears several data tables and initializes a set of character-based mapping values.
- `update_entity_timers`: Checks a value in a table and either copies a value from another table or calls a routine to calculate a new value, which is then added to a third location.
- `decrement_and_loop`: decrements a 16-bit value passed in a register and calls another routine repeatedly until the value reaches zero.
- `decrement_counter_loop`: Calls another routine and decrements a value in a structure until it reaches zero.
- `update_entity_state_and_timers`: Manages entity lifecycle by incrementing internal counters, updating state-based timers, and adjusting values in a table based on conditional comparisons.
- `extract_bit_mask`: Returns a specific bit field from the provided parameter using a bitwise AND operation.
- `calculate_average_nibble`: Calculates the average of the four nibbles within a word and expands the result into a new bit pattern.
- `populate_data_table_2`: Fills a table of thirty words by repeatedly calling another routine.
- `populate_data_table`: Fills a table with values from another routine in a loop and initializes several global word values.
- `update_object_pointers`: Retrieves an object and sequentially updates three of its internal pointer fields by calling another routine for each.
- `execute_and_clear_pointers`: Checks three consecutive memory locations for non-zero pointers, calls the functions they point to, and then clears those locations.
- `fetch_offset_data`: Checks a calculated offset against a limit and returns either a 32-bit value from a table or a constant.
- `calculate_offset_value_3`: Calculates a 32-bit value by combining a table entry with an offset derived from multiple bytes and a word.
- `get_offset_value`: Calculates an offset using a global word and returns a 32-bit value composed of that offset and a word from a table.
- `calculate_offset_value_4`: Calculates a value by combining bytes from a table using a multiplier and stores the result in a memory location.
- `call_wrapper_routine`: Calls another routine and returns.
- `calculate_offset_value_2`: Sets a word to zero, calls another routine, and then calculates a new word value using a combination of three bytes from memory.
- `initialize_entity_pointers`: Performs a series of conditional subroutine calls and updates several function pointers within a memory block based on a returned value.
- `execute_multiple_callbacks`: Executes a series of subroutine calls, including one via a function pointer stored in memory.
- `execute_callback_chain_2`: This routine performs a recursive sequence of calls to itself.
- `execute_callback_loop`: Sets status flags and enters a loop that executes subroutines through a table of function pointers based on a conditional value.
- `reset_structured_data`: Calls subroutines to clear memory and copies values between memory locations before setting specific words to one.
- `load_pointer_from_table`: Stores a value into a global variable and uses it to fetch a 32-bit pointer from a table based on an indexed lookup.
- `clear_global_variables`: Sets four different global word-sized variables to zero.
- `call_function_with_pointer`: Checks if a pointer is non-null and then calls a function using that pointer and an incremented version of its first element.
- `process_object_data`: Retrieves an object and performs several function calls to manipulate its state.
- `manage_object_state_2`: Retrieves an object and performs conditional function calls to update global variables.
- `execute_callback_and_clear`: Checks if a global pointer is non-zero, calls the function pointed to with two arguments, and then clears that pointer.
- `parse_nibble_stream`: Reads bytes from a table and reconstructs them into a new stream by combining nibbles from consecutive reads.
- `extract_nibble`: Returns either the high nibble, the low nibble, or the full byte of a value based on two input conditions.
- `update_dynamic_buffer_state`: Checks a condition based on a global value and performs a recursive update of a data structure.
- `execute_callback`: Calls a function through a pointer and returns.
- `execute_callback_and_reset`: Calls a function using a pointer and a short value from a table, clears the pointer, and then resets a different global word to zero.
- `update_data_tables`: Iterates through two separate source tables to call a function for each entry and stores the results in two destination tables.
- `evaluate_condition_state`: Calls a function with three parameters and returns one based on the result and the state of two other global variables.
- `process_audio_callback`: Updates a global flag, calls a function via a pointer, and executes a routine multiple times based on certain global conditions.
- `return_zero_4`: An empty routine that returns zero.
- `execute_conditional_callback_3`: Sets a flag in a global variable, executes a function pointer with three parameters, and returns a boolean based on a comparison of several global values.
- `return_zero_5`: This routine is empty and simply returns a zero value to the caller.
- `repeat_callback_four_times`: Calls a specific function four times in succession and then sets a global variable to zero.
- `update_audio_parameters_3`: performs bitwise transformations on a table of values, uses a function pointer to generate random numbers, and updates a global table and a counter.
- `generate_audio_sequence`: Uses a random number generator to select values from a table, updates a hardware register, and performs bitwise operations on a table entry.
- `generate_pseudo_random_audio_sequence`: This routine uses a series of calls to a random number generator to select values from a table and update hardware registers.
- `update_audio_registers`: Updates audio period registers and manipulates bitwise data in a table to drive sound generation.
- `execute_four_callbacks`: Executes a function pointer four times in a loop and then clears a global variable.
- `loop_callback_four_times`: Executes a function within a loop that runs four times and then clears a global variable.
- `update_audio_and_counters`: Checks a global counter against thresholds to either execute a series of function calls via a pointer table or perform conditional function calls based on a return value.
- `execute_audio_callbacks`: Calls a function through a pointer stored in a table four times and increments a global integer by five.
- `execute_audio_callback`: Calls a function through a pointer stored in a table three times and then increments a global integer by five.
- `execute_audio_callback_and_increment`: Calls a function pointer stored in a table three times and increments a global integer by five.
- `increment_audio_callback`: Calls a function via a pointer three times and increments a global integer by five.
- `execute_callbacks_and_increment`: Calls a routine via a pointer twice and increments a global integer by five.
- `execute_callback_and_increment`: Calls a function through a pointer twice and increments a global variable by five.
- `increment_global_counter_2`: calls a function through a pointer twice and increments a global variable by five.
- `increment_global_counter`: Calls a routine through a pointer and increments a global variable by five.
- `execute_callback_and_increment_value`: Calls a function through a pointer retrieved from a table and increments a global variable by five.
- `update_state_and_counter`: Calls a function through a pointer to update a global variable and increments another global variable by five.
- `increment_and_check_counter_2`: Increments a counter and, if below a threshold, calls several functions based on a return value; it also modifies a global variable if the counter exceeds a limit.
- `process_data_loop`: Iterates through a table to copy bytes to a structure and performs conditional subroutine calls based on table contents.
- `process_parameter_range`: Calls a function with a value from a table and a parameter, then calls another function using the first parameter and a result from a second function call.
- `process_entity_collision_and_state`: Checks for valid entity pointers and iterates through a list of active objects to handle state transitions and collision-based logic.
- `process_entity_collision_and_state_2`: Checks for valid entity pointers and iterates through a list of active objects to handle state transitions and collision-based logic.
- `process_bitmask_loop`: If a global value is not negative one, it clears a word, calculates a bitmask, calls a function with that mask, and then loops while calling another function until it returns zero.
- `call_indirect_function`: Calls a function through a pointer stored in a memory location.
- `clear_word_at_offset`: Stores a zero word into a memory location at a specific offset.
- `update_object_state`: Compares a parameter against a global value, traverses a linked list, and executes a sequence of function calls using data from a calculated table offset.
- `execute_callback_and_clear_2`: If a global variable is non-zero, it calls a function using that variable and another value as arguments, then clears the global variable.
- `dispatch_entity_event`: Checks conditions involving a table offset and executes a sequence of subroutines, including one with a specific argument and others via pointers.
- `update_rect_and_blit`: Calculates a new position and size for a rectangle, performs several blit operations using different parameters, and updates the target area.
- `blit_rect_and_copy`: Performs a blit operation on a rectangle and then copies data from a source to a destination.
- `center_blit_rectangle`: Calculates a centered position for a rectangle based on a width difference and performs a blit operation.
- `blit_with_offset`: Performs a blit operation using a specific width and then updates a position by subtracting a value from that width.
- `parse_string_and_blit`: Iterates through a string, performing blit operations for each segment and handling special characters to adjust rectangle dimensions.
- `apply_rotation_transform`: Iterates through a table of words, applying a mathematical transformation to each value to perform a rotation-like effect, and updates two global shorts.
- `recalculate_value_with_modulo`: Obtains a value from another routine and performs a calculation involving division and modulo using a part of a provided parameter.
- `update_table_element`: Updates a value in a table by adding the value of a preceding element to it, using a wrapping index logic.
- `allocate_hierarchical_structures`: Allocates multiple memory blocks using a library call and performs a series of nested routine calls to initialize and link them together.
- `recursively_clear_nested_pointers`: Checks two specific offsets within a structure for non-zero values and calls itself recursively if they are found.
- `return_immediately_2`: The routine performs no operations and returns.
- `return_immediately_3`: This routine is empty and performs no actions.
- `return_constant_2`: The routine returns a constant value.
- `check_status_condition`: Performs a loop to check memory values and returns a constant based on a byte comparison.
- `evaluate_state_loop_2`: Iterates through a loop checking memory values and returns specific constant values based on byte comparisons.
- `evaluate_status_constant`: Iterates through a loop checking memory values and returns one of several constant integer values based on specific byte and word comparisons.
- `return_constant_byte`: The routine returns a single byte with a constant value.
- `evaluate_conditional_state`: Checks various values in a memory structure to return one of several specific integer constants.
- `evaluate_conditional_loop`: Checks a byte value and enters a nested loop that monitors memory locations and executes subroutines based on their state.
- `evaluate_state_condition`: Checks a series of flags and values within a data structure to determine a return status.
- `evaluate_state_loop`: Iterates through a loop based on a word value, performing conditional checks and returning different constant values based on memory contents.
- `null_routine_6`: The routine is empty and performs no operations before returning.
- `execute_indirect_call`: The routine calls a function via a pointer, modifies a word in memory, calls another function with several parameters, and then restores the original word value.
- `register_routine_pointer`: Obtains a pointer from another routine and stores the address of itself into an offset of that pointer.
- `set_callback_pointer`: Selects a parameter based on a bit in a global variable and stores a value from that parameter into a global location.
- `store_parameter_to_global`: Stores a word from a parameter into a global variable.
- `perform_recursive_calls`: Calls a function via a pointer with several arguments and then performs two recursive calls to itself.
- `process_conditional_recursion`: Calls two different functions via pointers and, if the second returns zero, performs a recursive sequence involving a third function call.
- `compare_and_recurse`: Compares a parameter to the result of a function call and, if they differ, executes a recursive sequence that includes a subtraction operation.
- `branch_on_bit_test`: Calls a function via a pointer and, if the result is zero, branches to different function calls based on a bit test of a parameter before recursing.
- `recurse_with_parameter_check`: Executes a function call and, if it returns zero, uses a bit test on a parameter to select between two different function calls before performing recursive calls.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `update_object_state_and_graphics`: Updates object properties and status flags within a data structure, performs conditional checks against a memory-resident list, and synchronizes visual or positional data through several subroutines.
- `process_callback_and_object_list`: Iterates through a list of function pointers and executes them, followed by a cleanup or notification routine.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `allocate_block_with_size_header`: Allocates a memory block through the system allocator and, if the allocation succeeds, writes the first argument plus four into the block's first longword.
- `free_block_if_nonnull`: Frees the memory block at the pointer argument through the system FreeMem routine, doing nothing when the pointer is null.
- `reset_and_initialize_block`: Clears bits in a byte, fills a memory block with zeros, and calls a routine via a pointer.
- `initialize_audio_resource_2`: Attempts to initialize a system component and sets a flag while clearing specific bits in a byte if the initialization succeeds.
- `reset_state_and_call_subroutine`: Sets a short value to zero, calls another routine, and then conditionally calls another routine based on a bitwise check of a byte.
- `manage_object_state_and_callback`: Checks a status flag in a structure; if zero, it executes a function via a global pointer, calls itself recursively, and then executes another function via a pointer.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `allocate_block_with_size_header_2`: Allocates a memory block through the system allocator and, if the allocation succeeds, writes the first argument plus four into the block's header.
- `execute_and_reset_globals`: Executes a routine through a pointer, resets a global variable to zero, and clears another global variable.
- `compare_strings_case_insensitive`: The routine compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `fetch_task_pointer`: Fetches a function pointer from a global table and executes it.
- `call_system_alert`: Calls a system routine through a pointer stored in a global variable.
- `manage_dynamic_callback_chain`: Allocates a memory block and executes a sequence of conditional function calls through various pointers and tables.
- `build_file_path_string`: Concatenates three strings into a destination buffer, ensuring a separator is present between the first and second strings.
- `call_conditional_function`: Calls a routine through a pointer if a specific offset in a structure is non-zero, otherwise calls a different routine using a pointer from a global variable.
- `execute_conditional_callback`: Checks if a function pointer stored in a table is null.
- `execute_conditional_callback_2`: Checks if a function pointer stored in a table is null.
- `execute_indirect_routine`: Executes a routine through a pointer stored at an offset from a global variable.
- `call_indirect_pointer_2`: Checks if a pointer at a specific offset from a global variable is null, then either calls a routine through a pointer at a different offset from that same global variable or calls a routine through a pointer stored at the offset of the first global variable.
- `parse_and_initialize_resource_header`: Parses a data structure by checking for specific magic constants and iterates through a linked list of nodes.
- `call_other_routine_and_return_zero`: the routine calls another routine and then returns zero.
- `execute_infinite_recursion`: Calls itself indefinitely without performing any other operations.
- `mask_lower_bytes`: Calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `recursive_call_2`: The routine calls itself indefinitely.
- `execute_conditional_callback_2_2`: Checks if a function pointer stored in a table is null and performs conditional writes to a memory location.
- `loop_until_zero`: Repeatedly calls another routine while a value in a structure is not zero.
- `decrement_and_callback_2`: Decrements a value in a structure, and if it reaches zero, iterates through a table to find a matching ID, calls a routine via a pointer, and performs linked list manipulation.
- `reset_channel_tables`: Checks a table value and, if non-zero, resets a global variable and table entries before iterating through a list to reset channels and update data.
- `reset_channel_table`: Checks a value in a table, and if non-zero, resets a global variable and a value in the table, then iterates through a list to reset channel statuses.
- `process_data_table`: Iterates through a table of eight entries, clearing specific offsets within each entry and populating them based on values from a secondary source table.
- `update_channel_data`: Copies a block of data from one location to another, then iterates through a list to update several values including a counter, a status byte, and a pointer derived from a table.
- `process_linked_list_nodes`: Iterates through a linked list to check node values, calls functions through pointers, and performs string copies and memory updates based on specific conditions.
- `clear_table_entries_2`: Iterates through a list of entries, setting a specific word offset to zero within each entry.
- `decrement_and_callback`: Decrements a value in a structure, and if it reaches zero, iterates through a table to find a matching ID, calls a routine via a pointer, and clears the structure.
- `traverse_and_copy_linked_list`: Iterates through a linked list structure, performing conditional recursive calls and copying string data from one memory location to another if specific criteria are met.
- `free_linked_list`: Iterates through a linked list of pointers, calling a routine to free each memory block, and finally clears the head pointer of the list.
- `execute_system_alert_2`: Sets a value to zero and a pointer to a routine, then traverses a linked list to call a routine through a pointer and potentially update a list element.
- `initialize_audio_resource`: Sets up several global variables and memory locations, opens a device and resource, and attempts to initialize a system component using a function pointer.
- `release_system_resources`: Closes a device, frees a signal, and calls functions through pointers to clean up memory and state based on global flags.
- `execute_callback_list`: Iterates through a linked list and calls a routine through a pointer stored in each list element.
- `update_audio_channel_parameters`: Calculates and updates audio channel properties such as frequency and volume by iterating through a list of active requests.
- `signal_event_if_set`: Checks a flag at an offset from a pointer and, if non-zero, calls a system routine through a pointer in a global variable.
- `update_audio_channels`: Iterates through a list of structures to update audio channel settings and bitmasks, potentially calling a routine for each active channel and interacting with DMA and audio hardware registers.
- `manage_channel_requests`: Processes requests by updating a table with new data and pointers, or searches for an available channel in a table based on specific status criteria.
- `reset_channel_status`: Sets a byte at a calculated offset in a table to zero, or changes it to two if the following byte is one.
- `clear_channel_and_call_callback`: Sets a byte at a calculated offset in a table to zero, and if the following byte is non-zero, sets the byte to three, calls a routine via a pointer at a specific offset, and then clears the bytes.
- `clear_status_loop`: If a global variable is non-zero, it resets that variable and a value in a table, then repeatedly calls another routine until it returns zero.
- `reset_status_loop`: Checks if a global variable is non-zero, resets it and a table entry, and then enters a loop calling another routine until it returns zero.
- `check_routine_result`: Calls another routine and returns a value based on whether the result is zero.
- `conditional_routine_execution`: Calls a routine and, if the result meets certain criteria, executes further routines and checks a value in a structure.
- `update_sprite_animation_state`: Updates animation frame counters and sprite data based on a control byte and a lookup table.
- `copy_buffer_metadata`: Copies a block of data from a source structure to a destination structure if a counter is less than four.
- `execute_empty_routine`: An empty routine that performs no operations and returns immediately.
- `replace_and_decompress_object`: Searches a linked list for an object with a matching identifier and replaces it with a new instance, updating reference counts.
- `call_external_routine_2`: Calls another routine and then stores a zero value into a location pointed to by a register.
- `process_dma_transfer`: Performs a task that involves hardware registers for DMA and audio, then checks a counter in a memory block and calls a routine to free memory if the counter reaches zero.
- `manage_linked_list_and_decompress_data`: Iterates through a linked list to find and replace a specific object, updating reference counts and pointers.
- `update_global_status`: Stores a value from a register into a global variable and sets a local variable based on whether that value is zero.
- `update_node_data`: Traverses a list to find a node matching a specific pointer, then updates a value in a structure or increments a counter in a linked element.
- `process_sprite_command_buffer`: Parses a command structure to calculate sprite positioning, scaling, and memory offsets.
- `update_resource_pointer`: Decrements a counter in a structure and, if it reaches zero, copies data from a table into a destination.
- `allocate_memory_and_enable_dma`: Allocates a memory block with a size header and enables a DMA channel by writing to a hardware register.
- `execute_unreachable_code`: performs a call to another routine and then stores zero into a location pointed to by a register.
- `decrement_and_free_block`: Decrements a value at an offset within a memory block and frees the block if the value reaches zero.
- `process_routine_result`: Calls another routine and stores its return value into a global variable, then returns a combined value based on whether that result was zero.
- `call_and_nullify_pointer`: Executes another routine and then stores zero into the memory location pointed to by a register.
- `return_immediately_4`: This routine performs no operations and returns to the caller immediately.
- `update_resource_pointer_variant`: Decrements a counter in a structure and, if it reaches zero, copies data from a table into a destination using a different offset calculation.
- `copy_table_data`: Calculates an offset into a table to copy several words of data from one location to another.
- `nullify_pointer`: Calls another routine and then stores zero into the memory location pointed to by a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_pointer`: calls a routine through a pointer stored at an offset from a global variable.
- `execute_indirect_function`: Fetches a function pointer from a global variable and executes it.
- `execute_indirect_function_pointer`: Fetches a function pointer from a global table and executes it.
- `execute_indirect_function_2`: Loads a function pointer from a global variable and calls it.
- `execute_system_alert`: Calls a routine through a pointer stored in a global variable.
- `save_register_context`: stores a large block of registers and local data into a structure pointed to by a register, then calls an indirect routine and performs conditional logic based on the result.
- `initialize_resource_pointer`: Calls a routine via a pointer, and if successful, performs several operations including setting a value in a structure and updating a global variable.
- `set_resource_flag`: Sets a specific bit in a byte at an offset from a global variable and then clears that global variable.
- `initialize_resource_header`: This routine calls a function to parse and initialize a resource header by validating magic constants and traversing a linked list of nodes.
- `execute_recursive_call`: This routine performs a direct recursive call to itself before returning.
- `execute_recursive_call_3`: This routine performs a direct recursive call to itself before returning.
- `reset_audio_channel_tables`: Checks a table value and, if non-zero, resets a global variable and table entries before iterating through a list to reset channels and update them.
- `trigger_channel_table_reset`: Checks a value within a table and, if non-zero, resets a global variable and a table entry before iterating through a list to reset a channel.
- `execute_recursive_call_4`: The routine performs a self-referential call to itself.
- `reset_status_and_loop`: Checks if a global variable is non-zero, resets that variable and a value in a table, then repeatedly calls another routine until it returns zero.
- `reset_and_loop_status`: Checks a global variable, resets it and a table entry, and then executes a loop that calls another routine until it returns zero.
- `decrement_table_values`: checks a global variable, and if non-zero, decrements a value in a table by seven and calls another routine in a loop up to thirty-seven times.
- `clear_recursive_memory_offsets`: Performs a direct recursive call to itself before clearing three specific memory locations using offsets from a register.
- `recursively_update_resource_state`: Compares a passed parameter against a global short and performs recursive calls.
- `manage_resource_state`: Compares a parameter against a global short, performs recursive calls, and updates a global pointer using values from a table.
- `execute_recursive_calls`: Performs two consecutive calls to itself.
- `call_indirect_routine`: Calls a routine using a pointer stored at an offset from a register.
- `execute_recursive_calls_2`: Performs a sequence of recursive calls to itself using a value returned from a previous call as an argument.
- `call_indirect_routine_alt`: Calls a routine using a pointer stored at a different offset from a register.
- `initialize_and_update_audio`: initializes several global variables and function pointers if a specific flag is zero, then calls a routine and conditionally executes other routines based on the values of two global variables.
- `execute_double_call`: calls a routine twice via a pointer and then stores a value into a memory location.
- `update_audio_volume_levels`: Calculates and updates audio volume values based on a counter and modifies hardware audio registers.
- `update_audio_parameters`: calculates new values based on a global variable, updates that variable with incremental steps, and calls a routine if a threshold is exceeded.
- `execute_double_call_2`: calls a routine twice via a pointer and then stores a value into a different memory location.
- `calculate_audio_volume`: Calculates a volume level based on a value stored at an offset from a pointer, updates a global variable, and increments or resets that value.
- `update_audio_volume_register`: Calculates a volume value based on a counter, updates a hardware register, and resets the counter when it exceeds a limit.
- `update_volume_parameter`: calculates a value based on a global variable, increments that variable, and resets it to zero if it exceeds a specific limit.
- `update_conditional_state_2`: Checks a global short value, updates several memory locations, and performs conditional subroutine calls based on the results of those calls and a secondary short value.
- `return_constant_byte_2`: Returns a constant byte value to the caller.
- `clear_two_words`: Sets two consecutive words in memory to zero using an offset from a pointer.
- `update_progress_bar_state`: Calculates and updates the visual dimensions and scaling values for progress bar elements.
- `update_ui_text_elements_3`: Calculates coordinate offsets and updates a series of text strings or UI elements by writing specific values to memory locations and calling.
- `update_ui_progress_bars`: Calculates and updates the width and position of progress bar elements by manipulating coordinate offsets and scaling values.
- `render_ui_text_elements`: Calculates coordinate offsets and updates a series of text strings or UI elements by writing specific values to memory locations and calling.
- `update_ui_text_elements_2`: Calculates coordinate offsets and updates a series of text strings or UI elements by writing specific values to memory locations and calling.
- `update_ui_text_elements`: Calculates coordinate offsets and updates a series of text strings or UI elements by writing specific values to memory locations and calling a routine to render them.
- `tiny_moveq_unlk_rts_2`: Small routine of 3 instructions that loads 1 into d0, then removes the stack frame held in a5, then returns.
- `update_ui_progress_bars_2`: Updates the visual state of progress bar elements by calculating coordinate offsets and scaling values.
- `dispatch_event_handler`: Calls another routine and returns the constant value seventeen.
- `initialize_and_update_object_state`: Initializes a block of state data and performs conditional updates to object properties based on a state counter.
- `initialize_sprite_attribute_table`: Sets up a block of data containing sprite properties and calls several routines to update sprite attributes and positions.
- `return_one`: Returns the integer value one.
- `update_sprite_attribute_data`: Initializes a block of sprite property data and updates specific attribute values based on a state variable.
- `initialize_and_update_object_state_2`: Initializes an object's data structure with specific bitmasks and dimensions, then updates its position and state based on values retrieved.
- `update_sprite_attributes`: Initializes a data structure with specific bitmasks and dimensions, then retrieves values from a table to update sprite or object properties.
- `return_constant_one`: The routine returns the integer value one to its caller.
- `update_object_state_and_position`: Initializes an object's data structure with specific bitmasks and dimensions, then updates its position and state based on values retrieved from a table.
- `clear_and_check_status`: Clears two words in memory using offsets, calls another routine, and returns a value based on a subsequent memory check.
- `clear_memory_word`: Clears a word to zero at a location calculated by adding a negative offset to a value held in a register.
- `process_nested_conditions`: Clears a global word and another memory location, then executes a series of conditional subroutine calls based on a short value.
- `dispatch_conditional_sequence`: Sets a word in a global variable, calls subroutines, and returns a constant value based on a short value in memory.
- `execute_sequence_and_return`: Calls two subroutines and returns a constant byte value.
- `render_nested_text_elements`: Increments a counter and, if below a threshold, recursively processes text segments by updating coordinate offsets and calling a string parsing routine.
- `clear_memory_word_pair`: Clears two consecutive words in memory by writing zero to them using a pointer stored in a register.
- `evaluate_conditional_decrement`: Checks multiple conditions involving global memory and decrements a word value if specific criteria are met, returning zero or one.
- `process_complex_state`: Updates multiple memory locations, performs conditional subroutine calls, and executes a loop that interacts with various function pointers and global values.
- `clear_single_word`: Sets a single word in memory to zero using an offset from a pointer.
- `update_object_transform_and_scale`: Updates an object's position and scaling parameters by calculating coordinate differences and applying a scaling factor derived from the minimum dimension.
- `calculate_object_scaling_and_draw_2`: Determines a scaling factor by finding the minimum absolute difference between two coordinate components and dividing it by eight.
- `calculate_object_scaling_and_draw`: Calculates a scaling factor by determining the minimum absolute difference between two coordinate components and dividing it by eight.
- `calculate_object_scaling_factor`: Determines a scaling factor by comparing the absolute differences between coordinate components and dividing the smaller difference by eight.
- `update_object_velocity_and_scaling`: Calculates object movement by comparing fractional offsets and updating integer coordinates.
- `update_object_subpixel_and_position`: Calculates subpixel movement by dividing accumulated fractional offsets and updates the object's integer coordinates.
- `update_object_motion_and_lifecycle`: Updates an object's position by adding velocity components to its coordinates and manages a countdown timer.
- `update_object_motion_and_timer`: Calculates new coordinates for an object by adding velocity values to its current position and manages a countdown timer.
- `update_object_position_and_timer`: Updates an object's coordinates and decrements a counter based on a calculated interval.
- `decrement_and_call_pointer`: Decrements a value at a memory offset and, if it reaches zero, performs several data transfers and calls a function via a pointer.
- `execute_function_pointer`: Calls a function pointer using three values retrieved from a table.
- `resolve_entity_state_index`: Iterates through a table using a function pointer to find an index that satisfies a specific condition.
- `iterate_and_resolve_entity_state`: Increments a counter in a loop to iterate through a table, performing conditional checks against a value returned by a function pointer.
- `update_entity_status_variant`: Performs conditional status updates and writes values to a table based on byte comparisons.
- `update_entity_status_2`: Updates various status bytes and words based on conditional checks of a table and returns a value.
- `manage_entity_status_primary`: Updates several global variables, copies a byte from one location to another, and executes a conditional subroutine sequence.
- `set_status_from_table`: Writes a value to a table using an index and updates a global word based on a value from a table.
- `update_entity_status`: Sets a global word based on a value retrieved from a table and performs conditional updates to other memory locations.
- `manage_entity_status_secondary`: Sets a word in a global variable, updates status values, copies a byte, and performs conditional subroutine calls.
- `manage_entity_status_tertiary`: Sets a word in a global variable, modifies status bytes, copies a byte, and executes a conditional subroutine sequence.
- `update_status_logic`: Compares two bytes, updates a global word based on the comparison, and performs conditional status management.
- `set_status_twelve`: Sets a global word to twelve and conditionally updates a global word and byte before performing several status management operations.
- `set_status_nine`: Sets a global word to nine and conditionally updates a global word and byte before performing several status management operations.
- `set_entity_status`: Sets a global word to a value determined by an input parameter and updates several other memory locations based on conditional checks.
- `manage_entity_status`: Sets a global short and byte, copies a byte from one location to another, and conditionally executes subroutines while updating a global byte.
- `execute_conditional_handler`: Checks two global short values to decide which function pointer to call, then increments a counter and calls another function.
- `execute_and_increment_counter`: Executes two function pointers, updates two global variables, and increments a counter until it reaches a specific value before calling another routine and a final function pointer.
- `call_function_twice`: Executes a function pointer stored in memory twice.
- `check_nested_object_status`: Calls another routine and, if the result is non-zero, performs a second call with specific offsets and returns one if that also succeeds.
- `return_zero_6`: This routine performs no operations and returns a value of zero to the caller.
- `process_event_sequence`: Sets a global byte, calls a subroutine with an argument, and if the subsequent return value is non-zero, executes two more subroutines before clearing a global short.
- `process_complex_event`: Updates a counter, executes subroutines based on a byte comparison, and returns a value determined by the state of a function pointer.
- `manage_entity_event_2`: Increments a counter in a table and executes different subroutine sequences based on the state of several memory locations.
- `manage_entity_event`: Copies a global word to a local variable, increments a counter in a table, and executes subroutines based on the value of that counter.
- `store_result_to_memory`: Executes three subroutines and stores the result of the final call into a global byte.
- `dispatch_status_event`: Sets a global word, executes a subroutine, and then branches to different subroutine calls based on a short value.
- `execute_status_sequence`: Executes two subroutines with specific arguments and returns a constant value.
- `execute_simple_callback`: Calls another routine and returns.
- `update_entity_state_logic_3`: Evaluates conditional flags and function pointers to update an entity's internal state machine.
- `update_entity_state_logic_2`: Evaluates conditional flags and function pointers to update an entity's internal state machine.
- `update_entity_state_logic`: Manages an entity's state machine by evaluating conditional flags and executing specific subroutines based on memory values.
- `execute_state_logic`: Updates a counter and executes multiple subroutines to manage state transitions and return a status value.
- `process_state_logic`: Manages complex state transitions by updating counters and executing a sequence of subroutines based on memory values.
- `update_state_via_pointer`: Retrieves a value from a sequence of subroutine calls and stores it into a memory location calculated from an offset and a register value.
- `dispatch_status_update`: Updates a global word and executes a sequence of subroutines based on a value retrieved from a table.
- `execute_status_sequence_4`: Updates a global word to either zero or fifteen and executes a sequence of subroutines based on a table lookup.
- `run_status_sequence`: Sets a global word to zero, executes a subroutine, then sets it to a different value before calling two more subroutines.
- `execute_fixed_sequence`: Sets a global word to a specific value and executes a sequence of three subroutines.
- `execute_status_sequence_3`: Sets a global word to either zero or fifteen and executes a sequence of subroutines based on an input parameter.
- `return_four`: Returns the constant value four.
- `dispatch_indirect_call_2`: Sets a global word to zero, calls a subroutine through a pointer, and executes a subroutine based on the returned value.
- `return_constant_fifteen`: Calls another routine and returns the constant value fifteen.
- `return_constant_fifteen_2`: Loads the value fifteen into a data register and returns from the subroutine.
- `return_constant_fifteen_3`: Executes a subroutine through a switch statement and returns a constant byte value.
- `tiny_moveq_unlk_rts_3`: Small routine of 3 instructions that loads 15 into d0, then removes the stack frame held in a5, then returns.
- `clear_word_array`: Fills a series of twenty-seven words in memory with zero using a loop.
- `handle_complex_event`: Updates multiple memory locations and executes a complex sequence of conditional subroutine calls and table updates.
- `execute_two_calls`: Calls two subroutines, one with a specific argument, and returns a constant value.
- `execute_simple_call`: Calls a subroutine with a specific argument and returns a constant byte value.
- `update_byte_from_call`: Sets a global word to three, executes two subroutines, stores a returned byte into a memory location, and executes another subroutine.
- `set_word_to_one`: Sets a single word in memory to one using an offset from a pointer.
- `handle_status_event`: Calls a subroutine via a pointer using two short values, clears two global words, and performs conditional subroutine calls based on a global word.
- `update_complex_state`: Updates multiple memory locations, clears two global words, and executes different subroutine sequences based on a conditional check of a global word.
- `branch_on_condition`: Clears two global words and executes different sequences of subroutines based on a conditional check of a global word.
- `perform_status_update`: Executes a subroutine, sets a global byte, calls another subroutine with an argument, and executes a third subroutine before returning a constant value.
- `dispatch_status_logic`: Calls a subroutine via a pointer, clears a global word, and executes different subroutine sequences based on the value of a global short word.
- `update_status_byte_via_pointer`: Executes several subroutine calls and stores a returned byte into a memory location calculated by adding an offset to a value held in a register.
- `return_constant_two`: Returns the integer value two to the caller.
- `store_returned_byte`: Executes two subroutines and stores the byte returned by the second one into a memory location.
- `return_two`: Returns the constant value two.
- `manage_conditional_state`: Sets a word in a global variable, performs several subroutine calls, and conditionally stores a returned byte into a global variable.
- `update_status_and_return`: Sets a word in a global variable, calls two subroutines, and stores a returned byte into another global variable.
- `copy_and_validate_data`: Copies five words from one location to another, clears three longwords, and returns a value based on comparisons of two memory values.
- `return_constant_seventeen`: Returns the constant value seventeen.
- `clear_structured_data`: Clears a word at an offset, followed by two loops that zero out five pairs of words and three longwords in memory.
- `process_status_loop`: Executes a loop that increments a counter while checking global short values, then performs conditional subroutine calls based on those values.
- `evaluate_status_conditions`: Sets a global word to three and executes different subroutine sequences based on comparisons of two different global short values.
- `evaluate_status_threshold`: Checks a short value against a threshold and executes different subroutine sequences based on the result.
- `check_and_reset_status`: Checks a short value at a memory offset, calls a subroutine if non-zero, and sets a global byte to zero.
- `initialize_and_test_flag`: Sets a word to zero, sets another to fifteen, calls another routine twice, and returns a value based on a memory check.
- `return_constant_twenty_six`: Returns the constant value twenty-six.
- `initialize_and_test_flag_alt`: Sets a word to zero, calls another routine twice, and returns a value based on a memory check.
- `return_constant_value`: The routine returns a fixed integer value to its caller.
- `dispatch_event_handler_alt`: Calls another routine and returns the constant value twenty-three.
- `update_graphics_parameters`: Increments a counter, performs several function calls with specific parameters, and modifies three global values before calling another routine.
- `initialize_state_values`: Stores several constant values into a data structure and performs recursive subroutine calls based on a return value.
- `execute_status_sequence_2`: Writes a value to a global word, calls a subroutine with an argument, and then calls another subroutine.
- `populate_and_dispatch_event`: Sets a word in a global variable, calls several subroutines, and executes two functions via pointers stored in memory.
- `set_status_and_clear_table`: Sets a word in a global variable, calls two subroutines, and clears a word in a global variable.
- `update_audio_sequence`: Increments a global counter and updates a global variable based on its value, while calling subroutines via pointers and potentially executing a reset sequence.
- `update_audio_volume`: Calculates a value based on a global short, updates a global variable, and increments a counter.
- `update_audio_parameters_2`: Calculates values based on a global short, updates a global variable, and increments a counter.
- `call_pointer_with_table_data`: Passes three values from a table into a function called via a pointer.
- `execute_reset_sequence`: Calls two subroutines via pointers and then stores a zero into a global word.
- `update_audio_parameters_4`: Updates a global variable using mathematical operations based on a global short, calls several subroutines, and modifies audio volume registers.
- `calculate_audio_volume_2`: Updates a global variable using mathematical operations based on a global short and calls another routine using values from a table.
- `process_audio_parameters`: Updates a global variable using mathematical operations based on a global short and calls another routine using three values from a table.
- `increment_and_call_function_2`: Increments a global short value and calls a function using three values from a table, returning the result of a comparison.
- `return_zero_value`: This is an empty routine that returns zero to its caller.
- `execute_parameterized_sequence`: Calls two subroutines with a constant integer argument and then stores a zero into a global word.
- `update_audio_parameters_5`: Calculates a value based on a global short using several conditional ranges, increments that short by two, and updates audio hardware registers.
- `update_audio_envelope`: Calculates a value based on a global short using several conditional ranges, increments that short by two, and calls a function via a pointer if the short is below a specific threshold.
- `iterate_audio_table_3`: Calculates a value based on a global short, increments that short by two, and calls a function via a pointer if the short is below a specific limit.
- `iterate_audio_table_2`: Calculates a value based on a global short, increments that short by two, and calls a function via a pointer if the short is below a specific value.
- `calculate_audio_envelope`: Calculates a value based on a global short, increments that short, and calls a routine via a pointer if a condition is met.
- `update_audio_volume_and_period`: Increments a value in memory and uses it to calculate a new volume and period for an audio channel.
- `update_audio_period_and_volume`: Calculates a new period value for an audio channel based on a global counter, increments that counter, and calls a function via a pointer if the counter remains below a threshold.
- `iterate_audio_table`: Calculates values based on a global short, increments that short by two, and calls a function via a pointer if the short is below a specific limit.
- `invoke_pointer_via_table`: Retrieves three values from a table and passes them to a function pointer.
- `perform_double_call`: Executes two function pointers stored in memory and clears a word in a global variable.
- `process_bitmask_table`: Checks specific bits in a parameter to trigger initial routines, then iterates through a table of pointers to execute a routine until a bitmask is exhausted or a limit is reached.
- `process_nested_function_calls`: Calls a routine and, if the result is non-zero, proceeds to call other routines using parameters and pointers, eventually storing a result in a pointer-passed variable.
- `clear_and_call_routine`: Calls a routine, then calls another routine with a specific value, and finally clears a global short value if it is non-zero.
- `manage_conditional_resource_calls`: Calls a routine and, if successful, executes several function calls via a pointer table to retrieve a value, which is then used to conditionally trigger further recursive calls.
- `populate_data_table_3`: Sets a value in a global variable and then loops a variable number of times to call a routine and store its result into a table.
- `iterate_pointer_table`: Loops through a table of data, passing two values from each entry as arguments to a subroutine.
- `set_constant_value`: Calls two subroutines and stores the value sixteen into a global word.
- `update_status_word`: Executes two function pointers and writes a specific sixteen-bit constant to a global variable.
- `transfer_function_pointers`: Iterates through a table of pointers, calls a subroutine for each, and stores the returned values into another table.
- `return_immediately_7`: This routine performs no operations and returns to its caller immediately.
- `update_physics_coordinates`: Calculates and updates two sets of coordinate values by applying velocity, acceleration, and friction-like damping logic.
- `execute_stored_callbacks`: Calls a subroutine pointed to by a memory location twice and then stores a constant value into a global variable.
- `process_audio_counter`: Performs calculations using two characters and a function return value to update a global variable, then decrements a global short.
- `decrement_audio_counter_2`: Calls a routine via a pointer multiple times, performs calculations using two characters, and decrements a global short.
- `decrement_audio_counter`: Calculates a value for a global variable based on two characters and decrements a global short if it is not zero.
- `decrement_global_counter`: Calculates a value for a global variable based on two characters and then decrements a global short if it is not zero.
- `decrement_global_short`: Calculates a value for a global variable based on a character and a stack value, then decrements a global short if it is not zero.
- `decrement_conditional_value`: Checks if a specific global short value equals eleven to execute a subroutine, then decrements that value if it is non-zero.
- `clear_memory_blocks`: Iterates four times to write zero to two consecutive memory locations offset from a base address.
- `initialize_pattern_table`: Allocates a memory block via a system call and fills it with a repeating sequence of incrementing nibbles.
- `release_memory_block`: Checks if a pointer in a global variable is non-zero and, if so, calls a routine to free that memory with a specific size.
- `call_dma_reset_routine`: Calls a routine that clears a table entry and resets two hardware registers based on an offset.
- `invoke_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `reset_dma_parameters_2`: A 46-instruction routine that clears a value from a table and writes zero to two specific hardware registers based on an input offset.
- `reset_dma_parameters_large`: A 107-instruction routine that clears a value from a table and writes zero to two specific hardware registers based on an input offset.
- `update_dma_registers`: Calculates an offset based on an input parameter to write a value to a hardware register and stores a pointer into another hardware register.
- `zero_memory_blocks`: Runs a loop four times to write zero into two consecutive four-byte memory blocks.
- `set_dma_control`: Reads a value from a table and performs a bitwise OR with a constant before writing it to a global variable.
- `reset_dma_parameters`: Clears a value from a table and writes zero to two specific hardware registers based on an input offset.
- `set_memory_range`: Iterates through a memory range, writing a specific byte value to each address.
- `fill_memory_block`: Fills a memory block with a specific byte for a specified number of iterations.
- `convert_to_uppercase`: Checks if an input byte is a lowercase ASCII letter and converts it to uppercase if it is.
- `convert_to_lowercase`: Checks if an input byte is an uppercase ASCII letter and converts it to lowercase if it is.
- `find_byte_in_string`: Scans a zero-terminated string for the first occurrence of a specific byte and returns a pointer to it or zero if not found.
- `initialize_and_call_library`: Clears a memory block, updates global variables, and executes a function pointer from a library based on a bitmask check.
- `return_immediately_5`: This routine is empty and performs no operations before returning to its caller.
- `tokenize_string_into_array`: Parses a string by splitting it into individual tokens based on whitespace or quoted substrings.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer until a null terminator is reached or a specified maximum length is exceeded.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `update_window_data_2`: Retrieves a value from a global variable and uses it to traverse multiple pointers to update a structure with data from a library call.
- `update_window_data`: Retrieves data from a structure through multiple pointer dereferences and updates a global structure with values from a library call.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_format_string_2`: Stores a parameter into a global location, calls another routine, and then clears that global location.
- `increment_and_store_byte`: The routine increments a value at a memory location and then stores a byte from a register into another memory location pointed to by the incremented value.
- `call_and_concatenate`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register.
- `parse_format_string`: Parses a format string containing specifiers to extract and convert values from a source list.
- `perform_signed_division`: Calculates the sign of two inputs, calls a division routine, applies the sign to the result, and concatenates it with the second input.
- `perform_signed_division_2`: Calculates a 32-bit integer division and returns a 64-bit value containing the signed quotient and the original divisor.
- `call_and_return_concatenated`: Calls another routine and returns a 64-bit value formed by concatenating the original contents of a register with the value left in that register by the call.
- `recursive_call`: A routine that calls itself indefinitely without performing any other operations.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `execute_callback_chain`: The routine checks if a pointer in a global location is non-zero and executes it, then proceeds to execute another function pointer stored in a different global location.
- `access_table_element`: The routine uses an index to locate an entry in a table, validates the index against a limit, and if valid, executes a function pointer stored at that entry before clearing the entry.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_load_seg`: Wrapper that calls the dos library function UnLoadSeg and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `execute_indirect_call_2`: Performs an indirect call to a function pointer stored at an offset from a base address.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `allocate_and_initialize_structure`: Allocates a memory block via a library call and populates it with several bytes and a pointer before potentially recursing.
- `update_structure_fields`: Recursively calls itself if a specific offset in a structure is non-zero, then writes constant values to several offsets and calls two other functions.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `call_with_constant`: Calls another routine passing a constant value as an argument.
- `call_via_pointer`: Calls another routine using a pointer provided in a parameter.
- `initialize_data_structure`: If the first parameter is non-zero, it calls a function to allocate or retrieve a structure and populates its fields with provided values.
- `initialize_structure_defaults`: Sets a byte and two longwords to specific values within a structure and then calls another routine.
- `set_structure_defaults`: Sets specific byte and longword values within a structure and then calls another routine.
- `call_library_function`: Calls a function through a pointer retrieved from a library base address.
- `initialize_and_process_chain`: Copies a block of data from a table, calls a routine with that data, and if successful, updates a structure and calls several other routines.
- `pass_parameter_to_routine`: Calls another routine using a single parameter.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `execute_indirect_jump`: Performs an indirect call to a function pointer located at a specific offset.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_call_short`: Executes a single instruction to call a function via an indirect pointer.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `execute_indirect_call_single`: Performs an indirect call to a function address derived from a base pointer.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `execute_indirect_call_3`: Performs an indirect call to a function address retrieved from an offset of a value in a register.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `execute_indirect_call_4`: Calls a function located at a specific offset from a value stored in a register.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_function_2`: Retrieves a function pointer from a table and calls it using an offset.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a function through a pointer retrieved from a table using a specific offset.
- `call_library_function_4`: Executes a function by accessing a pointer from a table via an offset.
- `call_library_function_5`: Performs an indirect call to a function found at a table offset.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_function_6`: Calls a function located at a calculated offset within a table.
- `call_library_function_7`: Uses a pointer from a table to call another routine at a specific offset.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_function_8`: Executes a routine by calling a function pointer retrieved from a table offset.
- `call_indirect_function_2`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_indirect_function_3`: Executes a subroutine by calling a pointer found at a specific offset within a table.
- `call_indirect_function_4`: Retrieves a function address from a table and jumps to it.
- `call_indirect_function_5`: Calls a function through a pointer located at a specific offset from a table base.
- `call_indirect_function_6`: Uses a pointer from a table to perform an indirect function call.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_indirect_function_7`: Performs an indirect jump to a routine address stored in a table.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_indirect_function_8`: Calls a subroutine by dereferencing a pointer located at an offset within a table.
- `call_indirect_function_9`: Executes a function call using a pointer retrieved from a table structure.
- `call_indirect_function_10`: Performs an indirect call to a routine address stored in a table.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_indirect_function_11`: Jumps to a function address obtained from a table via an offset.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_indirect_offset_c6`: Performs an indirect call to a function pointer located at a specific offset within a structure pointed to by a register.
- `call_indirect_offset_cc`: Executes a call to a function address retrieved from a table using an offset.
- `call_indirect_offset_d2`: Calls a function through a pointer found at a specific offset in a data structure.
- `execute_indirect_function_pointer_2`: Fetches a pointer from a nested memory structure and executes the function located at that address.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_indirect_offset_156`: Performs an indirect call to a routine located at a fixed offset from a base pointer.
- `call_indirect_offset_180`: Calls a function by dereferencing a pointer stored at a specific offset in a table.
- `clear_all_pointers_2`: Sets several global pointers to zero, attempts to allocate memory via multiple subroutine calls, and sets a global flag if successful.
- `clear_all_pointers`: Checks several global pointers and calls various routines to free or process them before setting all those pointers and a final global variable to zero.
- `sync_memory_block`: Compares a memory block with a global buffer, copies the source block to the buffer, and updates several global variables with values from a structure.
- `manage_conditional_callback`: Checks global values to decide whether to call a function via a pointer, allocate memory, or update a structure and return a pointer.
- `execute_recursive_call_2`: This routine performs a direct recursive call to itself using the provided parameters before returning.
- `calculate_delta_value`: Computes the difference between two values in a table and updates a value in a structure based on the result and an input parameter.
- `update_parameter_and_check_result_2`: Compares two global short values and checks if a structure member matches a parameter; if not, it updates the structure and calls a function.
- `nop_padding_sequence_8`: This routine is empty and consists of non-functional instructions used as padding in the executable gap.
- `retrieve_global_pointer`: Returns a value from a specific global pointer or a value from a global variable based on a condition.
- `return_global_pointer`: Returns the value stored in a specific global pointer.
- `dispatch_conditional_callback`: Calls a subroutine with an input parameter if certain global conditions are met, or calls a memory release routine if the parameter is non-zero.
- `set_state_and_call`: Stores a word and a long value into a structure pointed to by a global variable, then calls a function through a pointer.
- `reset_state_conditionally`: If a global short value is non-zero, it updates a structure with specific values, calls a function, and then updates the structure again with different values before calling the function a second time.
- `copy_and_uppercase_string`: Copies up to four bytes from a source string to a local buffer while converting lowercase ASCII characters to uppercase.
- `reset_state_conditionally_2`: If a global short value is non-zero, it updates a structure with specific values, calls a function, and then updates the structure again with new values.
- `update_parameter_and_check_result_loop`: Executes a loop up to eight times, calling a routine with incrementing offsets and writing results to a global memory location.
- `update_parameter_and_check_result`: Compares two global short values and checks if a structure member matches a parameter; if not, it updates the structure and calls a function, then sets a global value based on the function's return value.
- `search_integer_array_2`: Iterates backward through an array of long values to check if a specific value exists within a specified count.
- `search_integer_array`: Iterates backward through an array of long values to check if a specific value exists within the specified count.
- `initialize_and_call_function_2`: Checks a global variable to see if a library base is initialized, performs setup if not, and then calls a function using an offset from a retrieved pointer.
- `initialize_and_dispatch_callback`: Checks if a specific global value is zero, and if so, executes a sequence of initialization routines and updates the value.
- `initialize_and_call_handler`: Checks a global variable and, if zero, initializes a structure by calling several functions before executing a function pointer stored in a table.
- `initialize_and_call_function`: Checks a global variable, potentially initializes a structure via several function calls, and then calls a function using a value from a table.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_calls`: Stores the current routine address and a register value into global variables, then performs three consecutive indirect function calls.
- `update_table_pointers`: Calculates a value based on a global variable and stores it into two different offsets within a table before performing an indirect call.
- `execute_indirect_jump_2`: Performs an indirect jump to an address determined at runtime.
- `set_register_and_return_offset`: If a register is zero, it copies the value of another register into it, then returns that register's value decremented by eight.
- `mask_global_variable`: Returns a value from a global variable after applying a bitwise AND mask.
- `clear_interrupt_request_register`: This routine is empty and performs no operations, but is triggered by an external event that interacts with the interrupt request register.
- `store_stack_to_global`: Stores a word from the stack into a global variable.
- `return_zero_constant`: The routine immediately returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `nop_placeholder_routine`: This routine is empty and performs no operations before returning to the caller.
- `copy_list_elements`: Allocates a new structure and copies elements from one list to another using a function call to validate entries.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `find_node_by_string`: Iterates through a linked list and returns a pointer to a node if a string stored within the node matches a provided input string.
- `store_register_to_global`: The routine copies the value from D0 into a global variable.
- `call_indirect_via_global`: Loads a value from a global variable and performs an indirect call to that address.
- `store_byte_to_offset`: Stores a byte from a source pointer into a destination offset within a structure.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `check_display_mode_flag`: The routine loads a constant value and checks a global variable against two specific values.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: Copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `check_collision_flags`: XORs a global state word with a value from a linked list node and checks for specific bit flags.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 8 to a7, then returns.
- `copy_word_if_valid`: Checks if two pointer arguments are non-null.
- `compute_and_store_byte_logic`: Calls a function pointer from a global structure and combines its result with register values using bitwise logic.
- `compare_input_to_one`: The routine returns a negative short when the input value is greater than one and zero otherwise.
- `rotate_and_compare_byte`: Performs a bitwise rotation on a byte and compares the result to a global variable.
- `nop_padding_sequence_9`: This routine is empty and consists of two instructions that serve as padding within the executable.
- `conditionally_update_global`: Checks a byte value and, if non-zero, stores a word from the stack into a global variable before returning.
- `calculate_table_offset`: Reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `process_disk_status`: Evaluates status flags from a structure and a global variable to update a byte in a structure and potentially call a function or convert a value to a character string.
- `toggle_byte_flag`: Flips the top bit of the byte at offset 65 in the structure addressed by A3.
- `set_flag_from_bit`: Reads a bit from a global variable using an index derived from a byte in another memory location.
- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current function.
- `return_from_subroutine_4`: This routine performs a simple return to the calling function.
- `toggle_bit_and_increment`: Checks a specific bit in a global variable based on an offset; if the bit is set, it clears the bit, modifies a flag in a structure, and increments a value in a structure.
- `nop_placeholder_routine_2`: The routine is empty and performs no operations before returning to the caller.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `perform_no_operation`: This routine is empty and returns immediately to the caller.
- `clear_offset_and_save_register`: The routine stores zero to a word-sized location at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `execute_indirect_call_5`: Performs an indirect call to a function pointer stored in memory.
- `dispatch_indirect_call`: Compares a value against a global variable and performs an indirect call via a pointer stored in memory.
- `return_immediately_6`: The routine is empty and returns to the caller.
- `initialize_graphics_structure_2`: Calls two subroutines and populates a structure with constant values and a pointer.
- `update_global_function_pointer`: Checks if a global variable matches the current routine and, if not, updates the global variable and another location with the previous value.
- `initialize_graphics_structure`: Checks a global variable, allocates a structure, populates it with a series of constant values, and updates a global pointer.
- `enter_infinite_loop_20`: The routine executes an unconditional infinite loop.
- `modify_ciaa_register`: The routine sets a bit in a global variable, enters an infinite loop, and then clears that bit in the global variable.
- `check_global_condition`: checks if a bit is set in a global variable and, if a bit-rotated version of that variable matches a specific value, calls another routine.
- `wait_for_hardware_state`: Modifies global values, enters a loop checking a bit-shifted value, and then updates a global variable using a value from memory and a bitwise OR operation.
- `enter_infinite_loop`: Executes an unconditional infinite loop.
- `enter_infinite_loop_2`: The routine enters a loop that never terminates.
- `halt_infinite_loop`: Executes an unconditional infinite loop without modifying hardware registers.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Disk.1

- `calculate_offset_value`: performs bitwise operations and arithmetic on a value retrieved from a table using an index and a base pointer.
