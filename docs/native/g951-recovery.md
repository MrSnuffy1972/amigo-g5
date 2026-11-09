# Game 951 (DeathBringer v1.0 Empire): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00951). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4124927 top-level bytes in 92 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 715 of 715 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 598 of 598 named (0 author symbols), 598 of 598 described (222 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1202 of 1202 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 395604 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8349 of 9672 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 855840 |
| code | 35196 |
| data | 2909492 |
| padding | 18735 |
| unused | 305664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| disk.1:DBIntro | 6 | 2734 | 110 |
| disk.1:DeathBringer | 1 | 120 | 3 |
| disk.1:ReadMe.ITALIANO | 1 | 47 | 1 |
| disk.1:EXIT | 2 | 117 | 2 |
| disk.1:DBGameOnly | 6 | 5605 | 212 |
| disk.1:ReadMe.FRANCAIS | 1 | 47 | 1 |
| disk.1:DBInstall | 1 | 260 | 6 |
| disk.1:ReadMe.ENGLISH | 1 | 47 | 1 |
| disk.1:ReadMe.ESPANOL | 1 | 47 | 1 |
| disk.1:DBBoot | 1 | 44 | 1 |
| disk.1:ReadMe.DEUTSCHE | 1 | 47 | 1 |
| game.slave | 1 | 273 | 37 |
| disk.1:bootblock | 1 | 10 | 1 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 268 | 215 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| SP.DAT (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- DeathBringer_v1.0_Empire/DeathBringer_v1.0_Empire.hdf: hard-disk image (AmigaDOS OFS, 4416 blocks): its filesystem parses and all 10 files in it are byte-identical to the extracted tree DeathBringer_v1.0_Empire_hdf, which is recovered file by file; no block outside the filesystem holds data
- 6 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 698 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 200 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- DeathBringer_v1.0_Empire/DeathBringer_v1.0_Empire_hdf/DeathBringerEmpire/data/disk.2:DATA/SP.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DeathBringer_v1.0_Empire/DeathBringer_v1.0_Empire_hdf/DeathBringerEmpire/data/disk.1:System-Configuration: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- DeathBringer_v1.0_Empire/DeathBringer_v1.0_Empire_hdf/DeathBringerEmpire/data/disk.1: 695 logged loads cover 0 bytes of the image
- DeathBringer_v1.0_Empire/DeathBringer_v1.0_Empire_hdf/DeathBringerEmpire/data/disk.2: 3 logged loads cover 0 bytes of the image
- DeathBringer_v1.0_Empire/DeathBringer_v1.0_Empire_hdf/DeathBringerEmpire/data/disk.2: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 1 never-loaded region(s), 395604 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- disk.1:DBIntro

## Copy protection found

- none found

## Functions


### disk.1:DBIntro

- `intro_main_sequence_and_chunky_check`: Stores d0, calls setup and init routines, sets two flags, runs several intro subroutines, conditionally calls two external routines, then returns a pointer if the stored value equals the CHMP tag.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `allocate_two_intro_buffers`: Requests 33000 bytes and stores the pointer at 380, then requests 32000 bytes and stores it at 384, returning the second allocation result.
- `zero_filled_data_area`: This single zero word decodes as ori.b #0,d0; it is a zero-filled data area referenced by address, not executable code.
- `copy_intro_data_to_chip_ram`: Calls setup routines, checks a signature longword, then copies words from a data block to chip memory offset by 32000 bytes and calls a final routine.
- `open_exec_alloc_and_store_pointer`: Opens exec.library, allocates 2 bytes of memory, closes the library, stores the saved pointer value at the allocation and returns the address after it.
- `zero_filled_data_word`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `close_library_free_mem_and_return`: Opens exec.library, frees the memory block pointed to by the saved pointer, closes the library, and returns the freed block pointer in d6.
- `zero_word_data_slot_at_017c`: This single zero word decodes as an ori.b immediate instruction but is really a zero-filled data slot referenced by address from other code, not executable code.
- `zero_word_data_slot_at_0180`: This one zero word is a zero-filled data area entry that other routines point to by address; it decodes as an ori.b immediate but is data, not code.
- `zero_filled_data_area_2`: A single zero word that decodes as ori.b #0,d0; this address is a zero-filled data area referenced by pointers, not executable code.
- `copy_nine_buffers_via_block_copy`: Loads nine fixed source addresses into a0 one at a time and calls the block copy routine for each, then returns.
- `copy_and_interleave_bitmap_planes`: Copies 4000 longwords from a0 into the buffer at the pointer stored in the fixed address, then interleaves plane data into a0 in 200 rows of 10 longwords.
- `open_graphics_library_and_init_display`: Loads the graphics library base from the absolute address, calls the display setup routine, then calls the library open routine and returns.
- `call_add_int_server`: Library stub: loads its arguments and the exec library base, calls AddIntServer and returns its result to the caller.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `fill_four_display_entries`: Writes four pairs of words from a0 into consecutive eight-byte entries starting at the display structure, advancing a0 by forty each time.
- `open_graphics_library_and_store_base`: Calls OldOpenLibrary for graphics.library via the exec base, stores the returned base in a global and points two base fields at the display structure.
- `open_graphics_library_store_base`: Opens graphics.library via exec OldOpenLibrary, stores the returned base at absolute address 388, and sets both pointer fields at offsets 50 and 54 of that base to zero.
- `open_graphics_library_set_buffer_pointer`: Opens graphics.library via exec OldOpenLibrary, stores the base at absolute address 388, and writes the value at absolute address 380 plus 32000 into offsets 50 and 54 of that base.
- `wait_for_fire_button_release`: Reads CIAA port A bit 6 and branches back to itself while the bit is set, then returns once the fire button is released.
- `close_intuition_library_via_exec`: Loads exec base from address 4, opens intuition.library by name, closes the returned library base, then closes the original library base through exec.
- `intro_main_loop_with_fire_button_check`: Runs the intro sequence: sets up display pointers, waits for input, checks CIAA_PRA fire button bit and a flag, then loops until exit or fire is pressed.
- `intro_script_interpreter_loop`: Reads script words from a pointer table, dispatches positive, negative and zero commands, decrements a counter, and exits when escape is detected.
- `wait_until_counter_reaches_d0`: Compares the word at absolute address a fixed address with d0 and branches back to itself while that memory word is less than d0, then returns.
- `zero_filled_data_area_3`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `run_intro_script_stream`: Resets the script pointer, reads words from the stream via a1/a0, dispatches each command, waits on d0, and sets the flag at 888 when the terminator is reached.
- `zero_filled_data_area_4`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `copy_random_tiles_and_advance_script`: Copies ten random 2560-byte tile blocks into a buffer, then reads script bytes via a2, sets a flag and pointer on negative bytes, calls helpers, and stores a2 back when a counter reaches twenty.
- `poll_fire_button_and_escape_key`: Loops up to 1500 times calling helpers, checks CIAA_PRA fire button bit, reads a key, sets the flag at 888 when escape is pressed, and returns on any key or timeout.
- `repeat_50_times_call_c80`: Loads 50 into d7, calls the routine at c80, then decrements d7 and repeats until it reaches minus one, finally returning to the caller.
- `data_table_after_intro_code`: The bytes decode as ori instructions on d0 and a0, indicating data rather than executable code, so this is a data table following the intro code.
- `load_intro_pointers_and_call_ea`: Loads a1 from memory at 2b222, a0 with address 17830 and a2 with address 63c, then calls the routine at ea and returns.
- `init_intro_frame_counters`: Writes 132 to the word at 1672, zero to the words at 1674 and 1676, then calls three subroutines and returns.
- `zero_filled_data_area_5`: Two zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `step_intro_frame_until_132`: Adds 4 to the word at 1672, tests the word at an address, increments the word at 1674, calls five subroutines, and loops while 1672 is below 132.
- `intro_frame_loop_counter`: Decrements the longword at address 688 by four, calls five subroutines, tests the word at 2b084, increments the word at 68c, and loops while the value at 688 stays above four.
- `select_sixth_or_seventh_sprite_and_draw`: Reads a counter, wraps it past eight, subtracts six when above five, scales it by thirty-two into two sprite offsets, sets width one hundred sixty-four, stores the sprite base pointer and calls three drawing routines.
- `zero_filled_data_area_11`: This single zero word decodes as an ori.b immediate instruction, but the bytes are data, not executable code.
- `setup_intro_sprite_offsets`: Calls the palette setup, copies the current frame word into three display buffers, clamps a counter to eight and six, scales it by thirty-two into the sprite offset words, then fills two buffer structures via the blitter helper.
- `set_palette_from_frame_plus_twenty`: Loads source, destination and copper pointers, adds twenty to the frame word at 0688 and stores the result into two palette words before jumping to the palette routine.
- `data_table_decoded_as_ori_instructions`: These five words are data bytes, not executable code: the ori immediates and offsets are meaningless, so this is a table entry rather than a routine.
- `set_palette_offset_and_upload_two_blocks`: Reads a word from memory, adds 32, stores it into two fields of a parameter block, then uploads two palette blocks to the copper via two calls.
- `copper_list_register_writes`: Writes immediate values into the copper list at a0 and a0 plus d0, setting bit and move instructions for display hardware.
- `upload_two_palette_blocks_fixed_addresses`: Loads fixed source and destination addresses into a0 and a1, calls the upload routine twice with two different parameter blocks, then returns.
- `reset_three_sprite_positions_from_offset`: Reads a global word, subtracts eight with a floor at zero, stores it into six globals, then calls a routine three times to set up three sprite entries.
- `clamp_two_sprite_offsets_from_global`: Adds twenty and thirty-two to a global word, stores the results, and when the global is below the limits stores the remaining counts and calls a routine twice.
- `call_two_intro_setup_routines`: Calls the routine at offset c92 then the routine at offset c2e, then returns to its caller.
- `blit_masked_tile_rows_to_screen`: Reads two source coordinates and a row count from a2, computes tile offsets into a0 and a1, then for each row combines four masked source words and writes them into four destination words.
- `draw_character_glyph_to_bitmap`: Uses d7 as a character code to locate a glyph in the font and a position in the bitmap, then merges the glyph's four bitplanes into the destination bytes, advancing the cursor and wrapping on carriage return.
- `toggle_active_bitmap_and_clear_flag`: Tests the active-screen flag, clears the current bitmap, swaps the a6 bitmap pointer between the two stored buffers and sets the flag to the opposite value.
- `clear_and_test_2b22a_flag`: Clears the word at absolute address 2b22a, tests it, and branches back to itself while it is zero, otherwise returns to the caller.
- `call_10e8_for_two_vectors`: Loads the longword at absolute address 17c into a6 and calls the routine at 10e8, then repeats with the longword at 180 before returning.
- `call_cca_for_two_vectors`: Loads the longword at absolute address 17c into a1 and calls the routine at cca, then repeats with the longword at 180 before returning.
- `load_intro_palette_and_call_setup`: Loads the address of the intro palette into a0 and a second table into a2, then calls two subroutines to set up the display before returning.
- `draw_sprite_from_table_at_position`: Decrements d7, stores it, indexes a sprite table with d7 and a coordinate table with d0, computes screen offsets from d1 and d2, then calls the blitter routine to draw the sprite.
- `blit_masked_sprite_to_screen`: Clips the sprite rectangle against screen bounds, computes screen and mask pointers, then rotates and masks four bitplanes into the display buffer using dbra loops.
- `masked_or_plane_blit_loop`: Reads pairs of longwords from a0, combines them with and/or/not masks and rotates, then writes the masked results into bitplanes at a1 and a2, advancing by a3 and looping over d2 and d3.
- `clear_8000_longwords_at_a6`: Copies a6 into a0 and writes zero longwords to 8000 consecutive longword slots starting at a6, decrementing d0 from 7999 to -1 before returning.
- `call_two_helpers_and_test_d0_byte`: Calls two unnamed subroutines in sequence, then compares the low byte of d0 against zero and returns with the comparison flags set.
- `read_ciaa_keyboard_serial`: Reads the CIAA serial data register, inverts and shifts the byte, returning 27 for the escape key, 1 for other keys, or 0 when no key is ready.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `next_xor_scroll_byte`: Loads the scroll pointer, reads a byte and the byte 512 further on, returns their XOR, and advances the stored pointer by one.
- `intro_frame_tick_and_palette_toggle`: Saves all registers, increments a frame counter, calls two optional update routines when their flags are set, and toggles two palette words while decrementing a countdown.
- `create_message_port_with_signal`: Allocates a signal bit, allocates 34 bytes of chip memory, fills a message port structure with the signal number, task pointer, port list and reply port, then returns the port in d0 and a0.
- `free_message_port_and_signal`: If a0 is non-null, removes its reply port, marks the port and its signal byte as invalid, frees the allocated signal bit and frees the 34-byte memory block.
- `advance_sound_frame_and_trigger_voices`: Increments the frame counter, and when it wraps calls the per-voice update routine for all four audio channels, then advances the sample offset and pattern position.
- `decode_and_play_audio_voice_command`: Reads a 4-byte voice command from the pattern data, looks up the instrument, sets AUDx pointers and lengths, handles effects, and updates the DMA control register.
- `clamp_target_and_set_direction_flag`: Reads a6 offset zero masked to twelve bits, stores it at offset 24, compares with offset 16 and sets byte 22 as a direction flag or clears offset 24 when equal.
- `dispatch_joystick_command_nibble`: Reads the low nibble of a6 offset 2 and branches to handlers that adjust offsets 16 and 18, toggle CIAA_PRA fire bit, or set sound and volume bytes in a4.
- `init_sound_channels_and_clear_volumes`: Scans a table for the highest byte, builds thirty channel pointers into a4 offsets, sets CIAA_PRA bit, and clears the four audio volume registers and sound state bytes.
- `silence_audio_channels_and_enable_dma`: Clears the volume registers of all four audio channels to zero, then writes 15 to DMACON with the high bit set to enable the DMA channels, and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `disable_and_point_to_custom_chipset`: Loads the exec library base into a6, saves d0-d1 and a0-a1, calls the library disable routine, restores the registers and loads the custom chipset base into a6.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `blit_masked_tile_column_to_bitmap`: Saves registers, computes source and destination bitmap offsets from a1 and a0, then loops 28 times rotating masked source longs by a nibble offset and writing them into the destination bitmap.
- `setup_sprite_from_script_words`: Reads coordinate and size words from a2, converts them to bitmap pointers in a0 and a1, calls the blit setup, then stores sprite pointers, height, modulo, control and data words into the a6 structure.
- `setup_multi_part_sprite_from_script`: Reads script words from a1, computes a source bitmap offset from a0, calls the blit setup, sets sprite control and palette words, then stores four successive sprite parts into the a6 structure.
- `setup_sprite_sequence_from_struct`: Reads a sprite descriptor at a0, computes source pointer and offsets, writes bitplane pointers, modulo and control words into the a6 copper-like structure, then steps through four frames using d0 bits and calls two subroutines.
- `zero_word_data_padding`: A single zero word decoded as ori.b #0,d0; it is data padding referenced by address, not executable code, and changes nothing.
- `zero_filled_data_table`: Forty-two zero words decoded as ori.b #0,d0; this is a zero-filled data table referenced by address, not working code, and clears nothing.
- `dispatch_indexed_table_entry_2`: Saves d0-d1/a0, and if the word in d0 is nonzero, indexes a table at the given base by that value times four and calls the routine whose pointer is stored there with d1 set to minus one.
- `start_sound_slot_from_stream`: Scans four sound slots backwards for a free one, fills its fields from the byte stream at a2 and the sample table, marks it active and stores the caller's word.
- `clear_matching_slot_flags`: Walks four slot records spaced forty-two bytes apart, and for each whose word matches the argument and whose low flag nibble is set, clears that nibble and calls the slot update routine.
- `run_slot_script_opcodes`: Scans four slot records, calls the per-slot update, and when a slot's countdown expires reloads its script pointer and decodes opcode bytes to set position, delay and sprite fields.
- `read_word_high_byte_first`: Reads two bytes from (a2) into d5, shifting the first left eight bits, then adds d5 to the word at offset 24 of a3 when bit 7 of d4 is clear.
- `store_word_or_call_combine`: Stores d5 into the word at offset 24 of a3, then if bit 6 of d4 is clear reads a big-endian word from (a2) and calls the routine at 2780 with it and the stored word.
- `animate_object_script_list`: Walks a list of object records stepping backwards by 42 bytes, copying script pointers and running per-object animation scripts until the count in d6 is exhausted, then returns.
- `update_object_animation_timers`: Decrements the horizontal and vertical animation timers of the object at 8(a5), advances its position within bounds, bounces or stops it per flag bits, and returns.
- `random_value_in_range`: Calls the random generator, then divides the result by the difference between the two word arguments and returns the remainder offset by the low bound in d0.
- `advance_pc_relative_counter`: Loads the address of a nearby counter into a0, adds its low longword to its high longword, stores the sum back, and returns the swapped result in d0.
- `set_flag_and_state_if_active`: Tests byte at offset 2 of a1; if nonzero, sets bit 5 there, writes 1 to offset 22 of a1, and returns 1 in d0.
- `zero_filled_data_area_6`: Four zero words that decode as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `zero_fill_data_block_a`: Four zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_fill_data_block_b`: Four consecutive zero words forming a data area pointed to by handler pointers; the bytes are data, not runnable instructions.
- `zero_fill_data_block_c`: Another four zero words used as addressable data by callers; decoding them as ori.b is meaningless, so treat as a data table.
- `zero_filled_data_area_12`: Four zero words that decode as ori.b instructions; this range is zero-filled data referenced by address, not executable code.
- `zero_filled_data_block_second`: Another four zero words decoding as ori.b instructions; a zero-filled data region pointed to by handler data, not real code.
- `zero_filled_data_block_third`: A third run of four zero words decoding as ori.b instructions; zero-filled data referenced by address rather than executable instructions.
- `test_d0_bit_in_d2_and_d4_bit0`: Tests bit 0 of d0 against bit 0 of d2 and tests bit 0 of d4, setting the condition codes only; no registers or memory are changed.
- `open_audio_device_and_init_channels`: Sets the CIAA LED bit, opens audio.device, allocates three channel structures and two IO blocks, links them with per-channel data and returns.
- `close_audio_device_and_free_channels`: Loops over three channel slots, closing the audio device and freeing each allocated memory block, then clears the CIAA LED bit and returns.
- `set_channel_command_b_and_dispatch`: Looks up a channel structure from the table indexed by the word argument, sets its command field to 11, clears its flag byte, then calls the structure's function pointer at offset minus 30.
- `configure_two_channel_commands`: If the long argument is nonzero, sets the indexed channel's command to 11 and clears its flag, calls it, then optionally sets a second channel's command to 3 with flag 16 and copies five arguments into its fields before calling it.
- `set_channel_command_c_with_coordinates`: Looks up a channel structure by the word index, sets its command field to 12, clears its flag byte, stores two word arguments into its coordinate fields and calls its function pointer at offset minus 30.
- `open_dos_library_and_load_dbmusic`: Opens dos.library via a6, then opens dbmusic.dat with mode 1005, reads an address bytes into the buffer at offset 1766, and closes the file handle.

### disk.1:DeathBringer

- `open_libraries_and_run_game_tasks`: Saves the stack pointer, opens intuition and dos libraries, finds the current task, allocates and runs the DBIntro and DBGameOnly tasks via Supervisor, then closes libraries and returns the saved stack value.
- `reset_and_jump_to_vector_table`: Loads a base address, subtracts a value from it, reads a pointer from offset four, decrements it by two, issues a reset instruction and jumps to that address.
- `close_exec_and_free_loaded_library`: Adjusts a0 by subtracting four, saves it and the following longword, opens exec.library, frees the saved memory block, then closes the library and returns.

### disk.1:ReadMe.ITALIANO

- `open_dos_and_run_deathbringer_task`: Saves the stack pointer, finds the current task, opens dos.library, loads and runs the DeathBringer file, then closes the library and restores the stack.

### disk.1:EXIT

- `load_and_relocate_deathbringer_segment`: Calls a library to allocate memory, copies segment data, relocates longwords, sets up the segment list and returns zero on success or minus one on failure.
- `move_d1_to_d0_and_branch`: Copies the longword in d1 into d0 as a return value, then branches unconditionally to another routine's code.

### disk.1:DBGameOnly

- `jump_to_absolute_zero_entry`: Program entry that jumps to the absolute address zero, transferring control away immediately without touching any registers.
- `open_dos_library_and_init_game`: Saves a7, clears a flag, opens dos.library via exec, stores the base, checks the task's CLI field, fetches the current directory and calls two game setup routines.
- `close_lib_forbid_and_reply_msg`: Calls a cleanup routine, closes the library at the first global pointer, then forbids the OS and replies the message at the second global pointer before restoring the saved stack pointer.
- `copy_task_name_to_buffer`: Loads the task pointer from absolute address 356, copies bytes from a0 into the buffer at pc-relative 283 until a zero byte, then calls library offset -90 with d0 zero and d1 set to 50.
- `open_libs_init_interrupts_and_start`: Opens graphics and intuition libraries storing handles, saves VHPOSR and a library byte, calls init routines, finds the task, sets a flag, then jumps to the main game loop or exits with code 100.
- `shutdown_game_and_release_resources`: Calls cleanup routines, closes the audio device if flagged, then closes the two exec libraries stored in globals and returns.
- `per_frame_update_and_counter_increment`: Saves registers, runs three subroutines when the word at 2304 is zero, increments the word and long counters, restores registers and returns zero.
- `return_zero_in_d0`: Loads zero into d0 as the return value and falls through to the following routine.
- `poll_vblank_flag_until_clear`: Clears the longword at absolute address 628, tests it, and loops with dbra d0 while it stays zero, then returns.
- `open_file_with_mode_1005`: Saves d1-d2/a0-a1/a6, loads d2 with 1005 and a0 with the filename, calls dos.Open via a6, returns -1 in d0 on failure.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `seek_file_offset_d1_mode_d2`: Saves d1-d3/a0-a1/a6, moves d1 into d2 and d0 into d1, calls dos.Seek through a6, restores registers and returns.
- `open_seek_read_close_file`: Opens a file by name with mode 1005, seeks to a position, reads bytes into a buffer, compares the read count and closes the file, returning the byte count or a negative error code.
- `open_file_and_store_handle`: Saves registers, opens a file with mode 1006 via dos.Open, stores the returned handle in d4 and returns it, or returns minus one on failure.
- `seek_compare_and_close_file`: Seeks the open file handle in d4 to the offset in d3, compares the resulting position with d3, closes the file and returns minus one on mismatch or zero on success.
- `alloc_mem_for_30_byte_node`: Saves d2/a2/a6, requests 48 bytes of exec memory with attributes 65537, stores the a0 pointer and size 48 at offsets 14 and 18, sets byte 8 to 5, and returns the block in a0.
- `free_30_byte_node_mark_invalid`: If a0 is nonzero, sets byte 8 to 255 and longs at offsets 20 and 24 to -1, then frees the block via exec.FreeMem using the size word at offset 18.
- `build_joystick_byte_from_port_bits`: Reads word at offset 12, swaps bits 0 and 8, tests CIAA_PRA bit 7 and the swapped bits, and writes the resulting joystick byte to three consecutive bytes at the target address.
- `empty_stub_return_d0_byte`: Contains only an ori.b with zero immediate on d0, which leaves d0 unchanged, then returns immediately; effectively an empty routine.
- `alloc_mem_10002_or_exit`: Calls exec AllocMem for 65538 bytes, tests the returned pointer in d0, and if null jumps to an exit path; otherwise returns the allocation pointer.
- `close_workbench_and_init_buffers`: Closes the WorkBench via intuition, copies 298 words from a source buffer to a destination, allocates three memory blocks, stores display pointers and clears a global structure.
- `setup_display_buffers_and_workbench`: Frees two display buffers, checks a flag, allocates a buffer, copies 298 words of graphics data into it, and sets two copper pointer fields in the display structure.
- `free_memory_if_pointer_nonzero`: Tests d1 for zero, and if nonzero passes it to exec FreeMem via the library vector at offset -210 from the exec base.
- `swap_display_buffers_and_reset_flags`: Swaps the two longword display buffer pointers, toggles a byte flag, sets a word to one, clears another word, and preserves a0.
- `flush_sprite_pointers_to_copper_list`: If the pending flag is set, clears it, writes four sprite pointer pairs from the sprite table into the copper list, then points both copper sprite registers at the list base and returns.
- `load_a1_from_absolute_pointer`: Loads the longword stored at absolute address 1414 into address register a1 and then falls through into the following routine.
- `set_bit8_and_call_delay`: Clears d0, sets bit 8 of d0, saves d1 on the stack, loads d1 with 16000, calls the delay routine, restores d1 and returns.
- `init_sound_channel_from_d0_d1`: Saves a6, calls the setup routine, stores a1 and d0 into the sound structure at offsets 84 and 64, clears words at 66 and 102, sets words at 68 and 70 to minus one, stores d1 at 88, then calls the audio device through a6 and restores a6.
- `load_sound_base_pointer`: Loads the longword stored at absolute address 1404 into a0 and falls through into the following routine without returning.
- `init_sound_channel_with_sample`: Loads a sample pointer and period, calls the setup routine, stores a0 and a1 at offsets 80 and 84, sets words at 68 and 70 to minus one, clears words at 100 and 102, sets word 64 to 4080, stores d0 at 88, then calls the audio device and restores a6.
- `wait_blitter_idle_with_ownblitter`: Saves d0-d2 and a0-a2, calls the graphics library OwnBlitter through a6, then polls the custom chip DMACONR bit 6 until the blitter is idle before restoring registers and returning.
- `wait_blitter_idle_poll_dmaconr`: Saves a6, points a6 at the custom chip base, reads DMACONR and loops while bit 6 is set, then restores a6 and returns.
- `release_saved_blitter_owner_slot`: Checks the longword at absolute address 272; if it is not minus two, copies it to address 2134 and writes minus two back to address 272, then returns.
- `copy_display_list_pointer_if_valid`: Compares the longword at offset 856 with minus two; if not equal, copies it to the longword at offset 272, then returns.
- `build_display_list_from_pair_table`: Reads word pairs from the table before a0, writing copper-style move instructions to the buffer at the global pointer, recursing for each entry until a zero second word, then appends a terminator and returns.
- `emit_sixteen_sprite_control_pairs`: Writes sixteen consecutive sprite control words starting at 384, incrementing by two, paired with successive words read from a1 into the buffer at a0, then returns.
- `zero_fill_data_word_a`: Contains a single zero word that decodes as an immediate OR into d0, so it is zero-filled data referenced by address, not executable code.
- `zero_fill_data_block_b_2`: Four zero-filled words that decode as immediate OR instructions into d0, forming a data block pointed to by handlers rather than runnable code.
- `zero_fill_data_word_c`: One zero word at an address taken by data pointers, decoding as an immediate OR into d0, so it is zero-filled data rather than code.
- `zero_filled_data_word_at_0932`: This is a zero word in a zero-filled data area, decoded as ori.b with immediate zero on d0; it is data, not executable code.
- `zero_fill_data_word_at_0936`: One zero word in the same zero-filled data area, reached by address from two callers; the bytes are data, not code.
- `zero_fill_data_word_at_093a`: Another zero word in the zero-filled data region, referenced as a handler pointer and by address; it is data, not executable code.
- `zero_filled_data_area_7`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `save_regs_set_d7_call_sub`: Pushes d1, d7, a0-a2 and a6, saves d7 again, sets d7 to minus one, calls the routine at the following address, then restores d7 and the saved registers and returns.
- `set_d7_zero_branch_forward`: Sets d7 to zero and branches forward past the following instructions into another routine; it does not call anything itself.
- `set_d7_loop_count_300`: Loads the immediate value 300 into the low word of data register d7, then falls through into the following routine.
- `poll_input_with_retry_loop`: Calls three subroutines, stores the result in a global longword, retries a fourth call while d7 counts down, and returns zero or the last result.
- `read_masked_input_byte`: Reads a byte from a global, masks it with another byte, and returns either a shifted table byte, a fixed longword, or zero depending on the mask and a flag word.
- `or_byte_d0_with_itself`: Performs a bitwise OR of the low byte of d0 with itself, setting the condition codes from that byte and leaving d0 otherwise unchanged.
- `read_serial_byte_with_timeout`: Saves registers, checks a serial status word, calls the serial device read vector with a one-byte buffer, and returns the received byte in d0 or zero on timeout, clearing the status word.
- `clear_serial_status_word`: Writes all ones to the serial status word and clears the following word, then returns to the caller.
- `open_console_and_input_devices`: Opens console.device and input.device via DoIO, stores the returned IO request pointers in globals, sets the input request's reply port and length, then jumps to the main game loop.
- `close_console_and_input_devices`: Sends a final DoIO to the input request, closes both device IO requests with CloseDevice, frees their memory and the console request, then returns.
- `check_byte4_equals_two_and_flag`: Loads byte at offset 4 of a0 into d0, compares it with 2, and if equal tests a word flag; returns zero in d0 when the flag is clear.
- `return_a0_in_d0`: Copies the address in a0 into d0 and returns, giving callers the pointer as a return value.
- `select_sound_by_command_byte`: Checks command byte in d0 and fields at offsets 6 and 8 of a0, storing sound parameters to globals and returning zero or a0.
- `handle_sound_command_36_or_37`: Compares d1 with command bytes 54 and 55, branches for those, otherwise stores d1 and d0 to sound globals and returns zero.
- `load_view_and_clear_display_flag`: Sets d0 to -1, stores it at absolute address 900, then calls the graphics LoadView vector through a6 with a1 from offset 34, preserving a6 and returning the original a0.
- `reset_screen_and_clear_object_fields`: Clears the word at the global screen location, loads the object pointer from the global vector, zeroes its offset 50 and 54 longwords, and returns zero in d0.
- `retry_until_input_or_abort`: Saves d0, calls two subroutines, and if the second returns nonzero returns it; otherwise checks a flag byte and loops decrementing d0 until it expires or the flag is negative, returning -an address on abort.
- `set_d0_minus_one_call_c06`: Loads minus one into d0, calls the routine at address c06, branches if the result is less than or equal to zero, then returns.
- `update_three_entity_slots`: Saves registers, calls setup and check routines, then for three slots at offsets zero, 34 and 68 loads a pointer from offset 4, calls the slot routine and stores the result, stopping on negative values.
- `call_slot_routine_three_times`: Loads the addresses of three entity slots at offsets zero, 34 and 68 into a0 and calls the same routine for each slot in turn.
- `release_lock_and_return_status`: Reads a longword from the address in a0; if it is non-negative, calls a subroutine, then writes minus one back to that address and returns the value.
- `open_deathbringer2_file_and_check`: Calls the Amiga library open routine with a filename pointer and mode, then closes the file if opened and returns one on success, zero otherwise.
- `set_d0_to_forty_one`: Loads the immediate value forty-one into data register d0 and then falls through to the following code.
- `add_pc_relative_pair_to_counter`: Loads the two longwords at the given program-counter-relative offsets, adds them together, stores the sum back to the counter longword, then swaps d0 and returns.
- `test_word_flag_and_branch_if_zero`: Loads the word at the program-counter-relative offset into d0 and branches to the add routine if it is zero, otherwise returns immediately.
- `zero_d0_and_set_d3_to_fifteen`: Sets d0 to zero and d3 to fifteen, then falls through into the following routine without returning.
- `copy_words_to_two_tables_and_offset`: Saves a2, then for d3+1 iterations copies the word at a0 to a1, to a2 (post-increment) and to 64 bytes past a1, advancing a0 by two and a1 by six bytes; restores a2 and returns.
- `zero_filled_data_area_three_words`: Three zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_second_block`: Another three zero words decoding as ori.b #0,d0; a zero-filled data block whose address is taken as a handler pointer, not code.
- `set_bit_zero_in_d0`: Sets bit 0 of data register d0 using an OR immediate, then falls through into the following routine.
- `alloc_mem_thunk_with_padding`: A block of no-op OR instructions precedes loading the exec library base into a6, reading the stack argument into d0, and jumping to AllocMem unless the argument equals minus one.
- `set_bit17_d1_and_jump_library`: Sets bit 17 of d1, then jumps through the library vector at offset -216 from a6, transferring control to that library routine.
- `allocate_two_mem_blocks_and_link`: Clears a flag, allocates two exec memory blocks of up to 65536 bytes each, links each into a free list with size and next pointer, and stores them in a table indexed by a counter.
- `free_all_memory_list_blocks`: Walks the linked list of allocated memory blocks, calling exec FreeMem on each block's data pointer, then follows the next pointer until the list ends.
- `insert_block_into_sorted_free_list`: Takes a block pointer in d0, optionally checks exec TypeOfMem for chip memory, then inserts the block into the sorted free list and merges it with adjacent free neighbours.
- `insert_free_block_into_list`: Takes a block pointer in d0, optionally queries exec TypeOfMem, then links the block into the free list at the appropriate address, merging with adjacent free neighbours and returning 1 in d0.
- `set_alloc_failure_flag`: Writes the immediate value 1 as a word to the global allocation-state word, then falls through into the following routine.
- `allocate_memory_block_with_fallback`: Rounds the requested size in d0 up to even bytes, searches the free list for a fitting block, splits or unlinks it, and on failure calls exec AvailMem, AllocMem and FreeMem to obtain memory, returning the block address plus four in d0.
- `set_flag_and_call_allocator`: Stores the immediate value 1 into the longword at memory location d94, saves d1 on the stack, calls the routine at 110c, then pops the result into d0 and branches to 11ac.
- `compute_tile_source_address`: Adds the word at offset 4 of a0 to d0 and the word at offset 6 to d1, then multiplies d1 by the word at offset 6 of the pointer in a0 and returns the resulting byte address in a0 after shifting d0 right by three.
- `decompress_rle_tilemap_to_screen`: Saves registers, calls the setup routine, then expands run-length encoded bytes from a2 into the tile buffer using the width and stride fields of a1, advancing rows and columns until the map is filled.
- `run_text_editor_input_loop`: Initialises the text buffer cursor fields, then repeatedly reads a character, calls the editor routine and redraws until the exit flag is set.
- `edit_text_buffer_handle_key`: Handles a key in d0 and modifier in d1 against the text buffer at a2, updating cursor, insert and delete positions and setting the redraw flag.
- `render_text_string_to_bitmap`: Saves all registers, computes the destination bitmap address from the font metrics in a1 and coordinates in d0-d2, then draws each character of the string at a0 using the glyph routine until a null or newline.
- `blit_tile_rows_to_screen`: Uses d0 masked to seven bits times 32 as an offset into a2, then copies eight rows of four bytes each into a3 with d3 and d4/d5 row strides.
- `build_sprite_columns_and_scale_table`: Reads 36 bytes from a1, computes sprite column offsets and widths, calls a blitter routine, fills three column pointers, builds bit masks and a 36 by 60 scaling table.
- `clamp_accumulator_to_plus_minus_fifteen`: Adds d0 to the word at address 2, then clamps that value to the range minus fifteen through fifteen and returns.
- `apply_gear_shift_and_copy_vehicle_stats`: Reads the pending gear delta, clamps it against the max gear, adds or subtracts the gear table entry from speed, acceleration and related globals, then copies the selected gear's 36 words into the live stat block.
- `compare_d0_against_limit_and_return`: Compares the word in d0 with the word stored at the fixed memory location, branches away if d0 is less than or equal, otherwise returns immediately.
- `store_byte_at_doubled_d0_offset`: Doubles the word in d0, then writes the low byte of d1 into memory at a0 plus that doubled offset and returns.
- `blit_weapon_sprites_into_bitplanes`: For each of 36 weapon entries, reads a byte from the weapon table, sets its high bit, computes a source offset from the entry index, then masks and ORs rotated sprite words into three bitplane destinations pointed to by a6.
- `draw_weapon_icon_rows_to_blitter`: Loops 36 times over icon table entries, computes bitplane offsets, programs blitter registers with masks and modulo values, calls the blit routine for three rows, then calls DisownBlitter via the graphics library.
- `wait_for_weapon_select_keypress`: Draws eight weapon icons from a table, then polls the keyboard routine repeatedly, returning early when the key code is 68 or 69, otherwise decrementing the icon counter and repeating.
- `center_prompt_and_read_key`: Saves registers, computes centered screen coordinates from a rectangle at a0, draws a prompt, waits for a keypress and compares it against several accepted key codes.
- `swap_globals_and_show_abaddon_text`: Calls setup routines, loads the ABADDON LORD OF VIOLENCE text with color and position parameters, then jumps to the display routine.
- `set_d2_and_jump_to_text_printer`: Loads d2 with 20 and branches to the text printing routine, passing a0 as the string pointer and d0/d1 as the position.
- `write_five_digit_decimal_to_two_buffers`: Divides d0 by ten five times, converting each remainder to an ASCII digit and storing it backwards into a0-1 and a0+33, then returns.
- `select_highscore_entry_and_check_name`: Scans the highscore table for the player's score, shifts entries down, inserts the new score, blanks the name buffer, then compares typed characters against the expected name.
- `reset_display_and_load_palette`: Clears d0 and d1, loads a0 with the display base and a1 with the palette area, calls the setup routine, then loads a0 with the sprite base and returns.
- `draw_tiled_bitmap_with_rle`: Calls two setup routines, clears d0 and d1, loads source and destination pointers, then branches to a tile blitter that expands run-length encoded bytes into the destination bitmap.
- `run_intro_sequence_and_reset_state`: Calls the tiled bitmap drawer and another setup routine, loads a pointer into a0, calls two more routines, then branches away; also resets a state word when it equals minus two.
- `invert_eight_scanline_cells`: Computes a screen address from d4 and d3 globals plus row and column offsets, then toggles every byte of eight cells spaced by the row stride.
- `copy_state_bytes_and_clear_counters`: Copies two bytes from fixed memory into two other fixed addresses, loads a word into d0, writes byte 4 to a fixed address, then clears a word and a byte in memory.
- `main_game_loop_state_machine`: Saves game state to a stack frame, then runs the main loop: polls input, handles keys, updates counters and screen state, and calls many subroutines until exit.
- `subtract_hundred_and_return_on_borrow`: Calls a subroutine, subtracts 100 from d0, branches away if a borrow occurs, otherwise returns to the caller.
- `set_palette_register_and_branch`: Writes the immediate value to the custom chip register at offset 2192, loads 28 into d0 and 255 into d1, then branches to another routine.
- `draw_screen_or_copy_tilemap`: Tests the sign of a word flag; if negative it calls a setup routine and draws a screen region, otherwise it copies 1360 longwords from one buffer to another.
- `walk_linked_list_and_dispatch`: Stores the pointer from a global into a work variable, then follows the four-byte next pointers of a linked list, calling one of two routines depending on the sign of each node's first byte.
- `copy_d0_to_d3`: Copies the longword in d0 into d3 and falls through into the following routine.
- `wait_for_timer_then_store_result`: Compares the longword at address 276 with the value at 9512, and while the difference is negative calls the routine at 1926, storing its result at 9514 when it succeeds; finally clears address 276 and returns.
- `decrement_shot_timer_and_animate_sprite`: Decrements the byte at the global timer; when it expires reloads it with four, then adjusts the sprite frame counter at offset three and the animation byte at offset twenty-nine of the object pointed to by a0, setting bit flags in (a0) when the animation ends.
- `adjust_vertical_bounds_and_add_speed`: Compares byte at offset 28 of the object in a0 against two threshold bytes; increments or decrements the matching threshold, then adds the high nibble of byte 29 shifted right four bits to the word at the speed accumulator.
- `adjust_vertical_offset_by_speed`: Reads a word from offset 8 of a structure, subtracts two global words and a fixed bias, then uses a doubled global speed value to index a table and clamp the resulting offset, otherwise it calls another routine to return a value of plus or minus one.
- `blit_masked_sprite_with_own_blitter`: Owns the blitter, computes modulo and mask values from the sprite descriptors, programs BLTCON and pointer registers, runs four blit passes, then disowns the blitter and restores registers.
- `expand_2bpp_tile_to_bitplanes`: Reads a tile header from a2, writes width and height to the destination, then expands packed 2-bit pixel bytes through a lookup table into two bitplanes stored backwards in memory.
- `setup_level_graphics_and_palette`: Loads level graphics and palette data into memory, clears counters and flags, draws the level display, then jumps to the main loop with a0 and a1 set to graphics pointers.
- `draw_text_line_at_586`: Sets d1 to the text length and a1 to the text buffer, then branches to the text drawing routine to render the line.
- `format_score_as_five_digits`: Divides d0 by ten repeatedly, adds sixteen to each remainder and writes five digit characters backwards into the buffer ending at the address loaded from pc-relative a0.
- `update_score_and_health_displays`: Compares the current score and health values with cached copies, sets dirty bits in the display flags, calls the digit formatter and redraws the score and health bars when they changed.
- `redraw_score_and_health_bars`: Copies the score and health bar tiles from the shadow buffer to the screen buffer, then redraws changed bar segments using the current score and health values.
- `set_cell_bits_across_six_rows`: For six rows spaced 160 bytes apart, computes d1 AND the row word AND NOT the OR of three offset words, then ORs the result into the words at offsets 80 and 120, advancing a0 by 2.
- `clear_cell_bits_across_six_rows`: For six rows spaced 160 bytes apart, computes d1 OR offset 40 OR NOT the AND of three words, then ANDs the result into the words at offsets 80 and 120, stepping a0 back by 2.
- `clear_single_cell_bits`: Reads the word at offset 40, ANDs it with offsets 80 and 120, inverts, ORs with the word at a0, ANDs into offset 120, then advances a0 by 2.
- `merge_clear_bits_into_a0_offset_78`: Reads a word at a0, ORs it with the word at offset 120, inverts it, ANDs with words at offsets 40 and 80, then ORs the result into offset 120 and advances a0 by two.
- `load_runescr_cmp_and_run_script`: Calls a setup routine, allocates a buffer, loads the runescr.cmp file, stores the pointer, then runs the script loop and exits through the shared cleanup path.
- `build_runes_lnn_offset_table`: Loads the runes.lnn file, then walks its records storing each record pointer into the table at a6 and advancing by the record length word until the end is reached.
- `key_command_dispatch`: Compares d0 against key codes P, Y, U, ], T and backslash, then clears or sets bytes at three global addresses and returns.
- `sprite_frame_setup_from_stream`: Reads four bytes from a1 into d4-d7, adds offsets from a0, optionally negates d4 based on a2 flags, then branches to sprite drawing code.
- `walk_sprite_list_and_draw`: Loads the sprite list head from a global, tests each entry's sign byte, skips the current a0 entry, calls the draw routine and follows the next pointer.
- `pop_d2_d4_read_a2_offset_d3`: Pops d2 and d4 from the stack, loads a byte from a2 plus signed d2 offset into d3, then returns to the caller.
- `check_sprite_bounds_against_d4_d5_d6_d7`: Tests a3 flag bits and compares a3 width and height fields against d4, d5, d6 and d7 to decide whether the object is outside the view, returning one or zero.
- `set_flag_clear_bit3_decrement_d2_branch`: Sets bit 3 at offset two of a2, subtracts two from d2, and branches away if the result is positive or zero, otherwise sets a byte flag and jumps elsewhere.
- `check_a3_entity_and_a2_flags`: Compares a3 with the value 5880 and, if equal, tests bit 0 of two bytes at offsets 2 and 3 of a2, branching accordingly without returning a value.
- `apply_damage_to_a3_health`: Tests a2 flag bits, optionally adjusts d1 from a global shift, then subtracts d1 from byte 29 of a3, storing the result or branching to the overflow routine and setting a3 bit 5.
- `drain_global_energy_into_a3`: If a3 is not the global entity, adds the global byte to a word at a fixed address and to byte 3 of a3, calling the overflow routine on carry, then clears byte 29 and sets a3 bit 6.
- `set_low_bit_and_fetch_table_byte`: ORs the low byte of d0 with 1, sign-extends it to a word, then loads a byte from a PC-relative table indexed by d2 into d1 and branches away.
- `zero_filled_data_area_two_words`: Two zero words that decode as ori.b instructions; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_one_word`: A single zero word that decodes as an ori.b instruction; this is zero-filled data referenced by address, not executable code.
- `decode_and_apply_sprite_frame_commands`: Reads a byte from a0, saves it, calls a decode routine, then walks the word stream at a1 clearing flag bits and advancing a1 past skip, branch and sprite-control words while updating the object's frame pointer.
- `test_bit11_then_compare_tile_bytes`: Tests bit 11 of d0, optionally calls the routine at the branch target, then compares the low bytes of d1 and d3 and returns.
- `test_bit10_dispatch_tile_compare`: Tests bit 10 of d0 and returns if clear, otherwise masks d0 to a table index, jumps through a3 to a compare routine and stores d3 into the a2/d2 table before branching back.
- `copy_d1_to_d3_and_return`: Copies the low byte of d1 into d3 and returns immediately to the caller; used as a tiny handler pointer.
- `update_entity_physics_and_animation`: Reads scripted bytes from a1 to move an entity at a0, updates its position, velocity, animation frame and sprite offsets, and dispatches sub-commands via a jump table.
- `zero_filled_jump_table_data`: Ten consecutive zero words that decode as ori.b instructions; this is a zero-filled data area referenced by address, not executable code.
- `zero_filled_data_area_8`: Six zero words decode as ori.b #0,d0 instructions; this range is a zero-filled data area referenced by address, not executable code.
- `zero_word_data_table`: A large block of zero words decodes as ori.b #0,d0 to data registers; it is a data table referenced by address, not working code.
- `load_level_and_build_entities`: Saves d0, calls two setup routines, then copies level data from an address/LEVEL00.LEV into entity structures, allocating memory and initialising sprite and position fields.
- `free_two_pointers_and_clear_pointer_array`: Releases the two global pointers through the free routine and clears them, then loops over thirty-six longwords clearing each and freeing it, finally walking the linked list at the global head freeing marked nodes except the sentinel.
- `insert_timer_node_by_priority`: Walks the linked list at the timer base, unlinks the head node, then reinserts it and the remaining nodes in priority order by comparing byte fields, updating the global list head and the saved pointer.
- `load_level_resource_by_index`: Uses d0 as an index into the a2 pointer table, returns the cached resource or allocates seven hundred ninety-nine thousand nine hundred ninety-nine bytes, stores the word count and returns the data pointer in a1.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a1 one word at a time, using a dbra loop counter, and returns.
- `relocate_level_object_pointer_tables`: Walks 36 pointer entries in the object table, and for each non-null object reads its word count and stores relocated data pointers into the target table indexed by the first word.
- `fill_empty_slots_from_bitmask`: Scans d0 slots of a0, and where a slot is zero and the current bit of the d4 mask is set, calls the allocator and stores its result back into that slot.
- `load_and_allocate_object_resource`: Looks up a resource by d7 and d3, reads its size word, allocates memory, copies the data in, and returns the allocated pointer in d0 or a negative error.
- `scan_three_object_lists_for_collision`: Saves registers, walks three lists of object entries at fixed memory areas, calls the collision test on each non-empty entry, and returns zero if none collide or the first nonzero result.
- `zero_filled_data_area_9`: This single zero word decodes as an ori.b immediate instruction but is a zero-filled data area referenced by address, not executable code.
- `scan_bitmasked_entries_against_mask`: Walks d1 entries from a0, skipping zero entries and those whose bit in the d4 mask is set, calling the collision test; advances the mask every 32 entries and returns the first hit or zero.
- `clear_slot_and_relink_free_list`: Compares a2 with a fixed value; if equal it calls a routine and clears the longword before a0, otherwise clears that longword and, when d0 is nonzero, walks a linked list to insert the block and update neighbour links.
- `retry_allocate_until_success`: Saves a2 and d0, sets a2 to a fixed value, then repeatedly calls an allocation routine and a second routine until the allocation returns nonzero, restoring the stack and a2 before returning.
- `advance_animation_frame_and_draw_sprite`: Reads a0 animation state, decrements the frame delay, advances the frame index and wraps it, looks up sprite data and offsets, then calls the draw routine with computed d0 and d1 coordinates.
- `advance_animation_script_frames`: Walks the animation script list at the global pointer, decrements each entry's frame counter, and copies the selected frame's two bytes into the output buffer indexed by the entry's id.
- `build_sprite_control_blocks`: Computes sprite dimensions from the global count, allocates a block, writes width and height words, fills control words with minus one, then processes the sprite list until the terminator.
- `blit_scroll_strip_to_screen`: Sets up a blit from the scroll buffer to the screen using the global scroll offsets, writing bitplane pointers and blitter registers, then waits for completion.
- `update_player_shot_direction_and_sound`: Compares a0 with the player object, toggles a flag when the shot id matches, advances a scripted byte sequence, and builds a direction byte from object offsets and a distance table.
- `aim_and_fire_player_weapon`: Reads input, computes a direction from the joystick, draws the weapon, waits for fire, then adjusts the aim with left and right keys and stores the resulting angle.
- `split_direction_into_quarter_steps`: Copies d2 into d3, shifts d3 right then left by two bits and calls the direction routine between the two shifts.
- `index_word_table_by_masked_d3`: Masks d3 with 60, then loads the longword at offset a6 plus that masked value into a2, indexing a word table.
- `call_helper_then_add_forty_to_d0`: Saves d0-d5 and a0 on the stack, calls a subroutine, restores the registers, adds 40 to the low word of d0 and returns.
- `count_down_byte_with_addx_loop`: Masks d0 to a byte, sets d1 to 114 and d5 to 213, then loops adding 41 to d1 with carry into d5 until d0 counts down to zero.
- `create_message_port_node`: Allocates a 34-byte exec message port, stores the reply port and signal bit, links the port list, and returns the port pointer in d0 or zero on failure.
- `free_message_port_node`: If a0 is nonzero, removes its reply port, marks the port and signal fields as unused, frees the signal bit and releases the 34-byte memory block.
- `zero_filled_data_area_13`: This single zero word decodes as an ori.b immediate instruction, but the bytes are a zero-filled data area referenced by address, not executable code.
- `zero_word_data_table_2`: Forty-two ori.b zero instructions that change nothing; the bytes are a data table referenced by address, not executable code.
- `dispatch_indexed_handler_by_word`: Saves d0-d1/a0, tests the low word of d0, and if nonzero pushes a table entry from a base plus d0 times four and calls the dispatcher with d1 set to minus one.
- `scan_four_slots_activate_first_free`: Walks four slot records backwards from a fixed base, incrementing each slot's state byte and, on the first free slot, fills it from the passed parameter block and returns.
- `load_sound_sample_descriptor`: Reads three bytes from the source pointer into the structure, uses the third as an index into a sample table to store the sample data pointer and length, then advances the source pointer.
- `init_weapon_slot_from_table`: Uses d0 to index a table of longword pointers, stores the chosen pointer and fixed values into the a3 structure, copies a2 and a5 fields, then restores registers and returns.
- `clear_matching_slot_flags_2`: Scans four entries spaced forty-two bytes apart, and for each whose word matches the argument and whose low nibble is set, clears that byte and calls the slot update routine.
- `dispatch_script_bytecode_commands`: Walks the script byte stream at offset 14 of the object pointed to by a3, decoding opcode nibbles and updating fields such as position, sprite and counters, then jumps to the shared dispatcher.
- `jump_table_dispatch_by_opcode`: Compares d0 against 13, shifts it left by one, loads a word from the table at pc offset 593a and jumps to the resulting handler address.
- `release_sprite_and_clear_active_flag`: Checks the low nibble of byte 2 of the a3 object, and if bit 4 is set pushes its fields onto the stack, calls the sprite release routine, then clears that bit.
- `push_anim_params_and_call_5f7e`: Loads a byte from offset 35 of a3 into d0, shifts it right twice, then pushes that word, the word at offset 24 of a3 and d6 before calling the routine at 5f7e and cleaning six stack bytes.
- `tick_two_axis_motion_counters`: Decrements the byte counters at offsets 33 and 40 of the a2 structure, reloading their reload values and calling the motion update routines when they reach zero, then returns.
- `advance_axis_position_with_bounds`: Adds the word at offset 30 of a2 to the position at offset 24, compares it against the bounds at offsets 26 and 28, and branches to the bounce or clamp routines when out of range.
- `copy_velocity_x_to_position_x`: Copies the word at offset 28 of the a2 structure into offset 24, then branches to the shared continuation at the later address.
- `conditional_copy_velocity_x_to_position_x`: Tests the word at offset 30 of a2; if positive, copies the word at offset 28 into offset 24, then branches to the shared continuation.
- `copy_velocity_y_and_flip_direction_bits`: Copies the word at offset 26 of a2 into offset 24, then conditionally negates the word at offset 30 and clears bit 6 of the byte at offset 34 based on flag bits.
- `update_animation_frame_and_direction`: Clears word at offset 30 of a2, sets byte 22 when bit 5 of byte 34 is set, decrements the frame counter at byte 40 and reloads it from byte 39, then either calls the direction routine with bytes 36 and 37 or advances the position byte 35.
- `advance_position_and_bounce_at_limits`: Adds byte 38 of a2 to position byte 35, compares it against limits at bytes 36 and 37, and on reaching a limit resets the position and may negate the step at byte 38 or set byte 22.
- `clamp_position_to_upper_limit`: Copies byte 37 of a2 into position byte 35, tests the step byte 38, and if it is positive copies byte 37 again before branching onward.
- `copy_heading_and_apply_flags`: Copies byte at offset 36 of a2 to offset 35, conditionally negates byte at offset 38 and clears flag bits in byte at offset 41, then sets byte at offset 22 to one.
- `scaled_value_between_bounds`: Calls a random generator, then divides the result by the difference between two stack arguments and returns the remainder added to the lower bound in d0.
- `advance_pc_relative_counter_2`: Loads a pc-relative pointer to a counter, adds its second longword to the first, stores the sum back and returns the swapped result in d0.
- `set_flag_and_mode_byte`: Tests byte at offset 2 of a1; if nonzero, sets bit 5 of that byte, writes 1 to offset 22 of a1, and returns 1 in d0.
- `zero_filled_data_area_first`: Four zero words that decode as ori.b instructions; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_second`: Another four zero words decoding as ori.b instructions; a zero-filled data region pointed to by handlers, not executable code.
- `zero_filled_data_table_at_5bf0`: Four zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_fill_data_at_5c00`: Four consecutive zero words forming a data table region pointed to by handler pointers; the bytes are padding data rather than runnable instructions.
- `zero_fill_data_at_5c10`: Another four zero words in the same zero-filled block, addressed via absolute lea and handler pointer; data bytes, not code.
- `zero_filled_data_block_b`: Another four zero words decoding as ori.b #0,d0; a separate zero-filled data region pointed to by handler pointers, not code.
- `zero_filled_data_area_second_block_2`: Another four zero words decoding as ori.b #0,d0; a zero-filled data block pointed to by handler data, not real instructions.
- `zero_data_with_btst_decoding`: Two words that decode as btst.l d0,d2 and btst.b #0,d4; data bytes referenced by address rather than executable code.
- `open_audio_channels_and_init_voices`: Sets CIAA_PRA bit 1, allocates four audio channel blocks and two IO requests via exec, opens audio.device, and initializes each voice structure with pointers and volume fields.
- `close_audio_device_and_free_voices`: Loops over four voice slots, closes the audio device, frees the allocated channel and IO request blocks with exec.FreeMem, and clears CIAA_PRA bit 1 if the flag is unset.
- `set_voice_period_and_trigger_output`: Reads a voice index argument, loads the matching voice structure, stores period 11 and clears a byte, then calls the audio device output routine through its library vector.
- `post_entity_message_and_optional_second`: Looks up an entity pointer from a table by index, sets its message field to eleven and clears a byte, sends it via its message port, then if a flag is set fills a second entity's fields from the stack arguments and sends that too.
- `post_entity_message_with_two_words`: Looks up an entity pointer from a table by index, sets its message field to twelve, clears a byte, stores two word arguments into its fields and sends it through its message port.

### disk.1:ReadMe.FRANCAIS

- `readme_french_startup_and_cleanup`: Saves the stack pointer, opens dos.library, checks the task, opens and reads the DeathBringer file, closes libraries and restores the stack before returning.

### disk.1:DBInstall

- `hard_disk_installer_main`: Program entry that opens the DBringer directory, creates the installation window, copies game data files, builds RUNES and an address paths, and closes libraries before returning.
- `strip_newlines_ensure_colon`: Scans the string at a0 replacing newline bytes with zero, counts colons in d0, and appends a colon if none were found.
- `join_path_with_slash`: Copies the string at a0 to a2, appends a slash unless the last copied character was a colon, then appends the string at a1.
- `load_copied_string_pointer`: Loads the address of the string at the given program counter offset into a0 and falls through to the next routine.
- `copy_file_with_progress_window`: Opens the source and destination files, reads and writes chunks through a buffer, closes both handles, and branches to a message routine on success or failure.
- `strlen_then_write_text`: Scans the null-terminated string at a0 counting bytes in d3, then jumps to the library write call with d1 and d2 as arguments.

### disk.1:ReadMe.ENGLISH

- `open_dos_library_and_run_deathbringer`: Saves the stack pointer, opens dos.library, finds the current task, opens the DeathBringer file, reads and runs it, then closes the library and returns.

### disk.1:ReadMe.ESPANOL

- `open_dos_library_run_deathbringer`: Saves the stack pointer, opens dos.library, loads and runs the DeathBringer file with arguments, then closes the library and restores the stack before returning.

### disk.1:DBBoot

- `boot_deathbringer_and_set_task_name`: Opens dos.library, finds the current task, opens the DeathBringer file, sets the task name to DBRG, copies the task structure and returns a status code.

### disk.1:ReadMe.DEUTSCHE

- `open_dos_library_and_run_main_loop`: Saves the stack pointer, opens dos.library, finds the current task, forbids multitasking, opens a message port, allocates and runs a message loop, then closes the library and restores the stack.

### game.slave

- `return_minus_one_immediately`: Loads the value minus one into d0 and returns immediately, giving the caller a failure result.
- `branch_to_offset_212`: Unconditionally branches to the code at offset 212, transferring execution without touching any registers or memory.
- `load_three_boot_strings_and_call`: Loads three program-counter-relative string pointers into a1 in turn and calls the same subroutine after each, then executes a nop.
- `compare_strings_at_indexed_offsets`: Saves d0 in d7, uses d0 times four as an offset to read a length byte, compares that many bytes from a0 against a1, and on a full match calls the routine at the d1 offset before returning the saved d0.
- `null_check_a0_then_jump_a6`: Returns via the a6 jump table if a0 is null, otherwise loads the byte at a0 plus d2 into d3 and d1 before jumping through a6.
- `patch_track_loader_and_init_game`: Saves all registers, writes a jump instruction to a fixed offset in the loaded module, checks and patches a magic longword, then sets up game vectors and jumps through the library table.
- `set_chip_ram_buffers_and_enter_main`: Loads the base pointer from offset 32, adds an address to form a chip RAM address, stores it at two offsets in the a5 structure, adds an address and calls the setup routine before jumping to a5.
- `set_a3_to_audio_region_and_jump`: Loads the immediate value an address into address register a3 and then jumps through the a5 jump table to continue execution.
- `read_status_word_mask_interrupt_bits`: Loads a long word from program-relative memory into d0 and clears bits 4 through 6, returning the masked value to the caller.
- `acknowledge_interrupt_from_stack`: Reads a word from the top of the stack, writes it to the INTREQ hardware register to acknowledge interrupts, advances the stack pointer by two and returns.
- `build_stack_frame_and_return_zero`: Makes room on the stack, saves the return address and a3, loads a program-relative address into a3, stores it in the frame, sets d0 to zero and d1 to a2, then returns.
- `set_a1_offset_e_flag_and_return`: Pops return address into d1, pushes a pc-relative pointer, restores the return address, sets bit 1 of the byte at offset 14 from a1, then returns.
- `load_custom_chip_base_and_branch`: Loads the custom chip register base address into a0 and branches to a pc-relative target, touching the custom hardware register area.
- `store_d0_to_pc_relative_slot`: Loads a pc-relative address into a6, stores d0 there, loads the cia-b base address into a6, then returns.
- `jump_via_cop2lch_vector`: Loads a longword from a program-counter-relative table into the COP2LCH hardware register and jumps through the address held in a0.
- `copy_a0_offset6_byte_to_a6_800`: Reads the byte at offset 6 from the structure pointed to by a0 and stores it at offset 2048 from a6, then returns.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_d0_against_21000_return_4_or_1`: Loads a longword from a program-counter-relative location into d1, compares it with an address, and returns 4 if equal or 1 otherwise in d0.
- `store_minus_two_markers_and_reset_counter`: Writes the value minus two into two longwords around a3, stores the advanced a3 pointer into offsets of a4 and a2, clears a word at offset 386 of a4, and skips fourteen bytes of return address.
- `copy_child_node_pointers_to_a3_offsets`: Reads longwords at a1 offsets 4 and 8; if nonzero, follows each pointer and copies the longword at offset 4 into a3 offsets 50 and 54 respectively, then skips fourteen bytes of return address.
- `fill_two_longwords_minus_one_before_a3`: Advances a3 by twelve, then stores minus one as a longword twice at descending addresses below a3 using a dbra loop over d0, leaving a3 unchanged and returning.
- `compute_level_offset_from_header`: Reads three longs from the structure pointed to by the PC-relative pointer, multiplies the first by 1440, adds the second, scales by 60, adds the third divided by 50, and stores the result at offset 198 of a2.
- `select_disk_prompt_message`: Clears the message byte at offset 31 of a1, checks flags at offset 64 of a3, optionally sets message 29, calls a helper, and otherwise picks a message index from a table or calls the disk library to display text.
- `message_index_data_table`: The bytes are data decoded as code, not executable instructions; they form a small table of message indices referenced by the disk prompt routine.
- `set_bit_from_a3_offset_into_a1_field`: Clears d0, reads byte at offset 67 from a3 as bit index, tests that bit in a program-counter-relative byte, stores 0 or -1 into offset 32 of a1, then adds 44 to the return address and returns.
- `push_pc_relative_long_plus_offset_return`: Pushes the longword at the program-counter-relative location onto the stack, adds a large constant to that pushed value, and returns, effectively returning a computed address.
- `push_pc_relative_target_and_return`: Pushes the address formed by the program counter plus the immediate offset onto the stack and returns, effectively jumping to that computed target.
- `toggle_interrupt_bit_and_clear_flag`: Clears bit 1 of the byte at offset 64 of a3, conditionally increments the counter at offset 294 and calls the shared routine, then clears a bit in a table indexed by the byte at offset 67 of a3 and sets bit 1 again.
- `disable_set_irq_bit_enable`: Saves a6, masks d0 to two bits, calls exec.Disable, stores d1 into a table at offset minus five plus d0 and sets the matching bit, then calls exec.Enable and restores a6.
- `load_interrupt_mask_and_branch`: Loads the immediate value 65537 into d1 and branches to the shared interrupt setup code at the later address.
- `set_d1_0x118_and_branch`: Loads the immediate value 280 into data register d1 and branches unconditionally to the code at the following routine.
- `clear_a1_offset_c_and_store_a4`: Clears the longword at offset 12 of the structure pointed to by a1, loads 12 into d4, and stores a4 into the address referenced by the pc-relative lea before returning.
- `zero_filled_data_area_10`: This single zero word decodes as ori.b with zero immediate; it is a zero-filled data area referenced by address, not executable code.
- `dispatch_via_a4_check_and_a5_vector`: Compares a4 with the value at the pc-relative longword, jumps through the a5 offset vector if unequal, otherwise saves all registers, loads d0 from (a1), conditionally calls a routine when d1 is nonzero, restores registers and jumps through a6.
- `push_pc_relative_longword_plus_32`: Pushes the longword stored at the pc-relative address onto the stack, adds 32 to that pushed value, and returns it to the caller.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_8`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_runtime_entry_point`: Unconditionally jumps to the absolute longword address in the runtime segment, transferring control to the external entry point.
- `undecodable_bytes_9`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_10`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_11`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_12`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_13`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_shared_runtime_entry`: Performs an unconditional jump to another routine in the runtime segment, transferring execution without saving registers or returning.
- `jump_to_alternate_runtime_entry`: Performs an unconditional jump to a different routine in the runtime segment, passing control onward without touching registers or the stack.
- `jump_to_third_runtime_entry`: Performs an unconditional jump to a further routine in the runtime segment, handing off execution with no register or memory changes.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_14`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_to_shared_runtime_entry_one`: Performs an unconditional jump to a shared runtime entry point, transferring execution without saving registers or returning to the caller.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_shared_runtime_entry_two`: Performs an unconditional jump to a second shared runtime entry point, passing control onward without touching registers or the stack.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_15`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_16`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_17`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_18`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_19`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_20`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_21`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_to_shared_runtime_entry_three`: Performs an unconditional jump to a third shared runtime entry point, handing off execution directly with no register or memory changes.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_22`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_41`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_42`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_23`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_24`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_43`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_25`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_44`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_45`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes_26`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_46`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_47`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_48`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_49`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_50`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_51`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_52`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_53`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_54`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_55`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_56`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_57`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_58`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_59`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_60`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_61`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_62`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_63`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_64`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_65`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_66`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_67`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_68`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_69`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_70`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_71`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_72`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_73`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_74`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_75`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_76`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_77`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_78`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_79`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_80`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_81`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_82`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_83`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_84`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_85`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_86`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_87`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_88`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_89`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_90`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_91`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_92`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_93`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_94`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_95`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_96`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_97`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_98`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_99`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_100`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_101`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_102`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_103`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_104`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_105`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_106`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_107`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_108`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_109`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_110`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_111`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_112`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_113`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_114`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_115`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_116`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_117`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_118`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_119`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_120`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_121`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_122`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_123`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_124`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_125`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_126`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_127`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_128`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_129`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_130`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_131`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_132`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_133`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_134`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_135`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_136`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_137`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_138`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_139`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_140`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_141`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_142`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_143`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_144`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_145`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_146`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_147`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_148`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_149`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_150`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_151`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_152`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_153`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_154`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_155`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_156`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_157`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_158`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_159`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_160`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_161`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_162`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_163`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_164`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_165`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_166`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_167`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_168`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_169`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_170`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_171`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_172`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_173`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_174`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `load_ed_into_d0_and_branch`: Loads the immediate value 237 into data register d0, then branches unconditionally to a shared routine elsewhere in the segment.
- `load_two_into_d0_and_branch`: Loads the immediate value 2 into data register d0, then branches unconditionally to the same shared routine used by the neighbouring entries.
- `load_ee_into_d0_and_branch`: Loads the immediate value 238 into data register d0, then branches unconditionally to the shared routine at the common target.
- `set_d0_ef_and_branch`: Loads the immediate value 239 into d0 and then branches unconditionally to another routine, transferring control without returning here.
- `nop_then_branch_forward`: Executes a no-operation instruction and then branches unconditionally to another routine, transferring control without returning here.
- `dispatch_indexed_table_entry`: Saves registers, computes a stack-relative pointer, loads a table base, indexes it by d0 shifted left twice, fetches an entry into a4, calls a library routine with d0 set to 12, and returns d1 in d0.
- `jump_vector_entry_175`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_176`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_177`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_178`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `push_five_string_pointers_and_jump`: Pushes five different string pointers onto the stack one at a time, each preceded by decrementing a7 by eight, then branches to the shared continuation at the end of the block.
- `push_string_and_branch_to_shared_exit`: Decrements the stack pointer by eight, pushes the address of a fixed string, then branches to a shared exit routine.
- `push_string_set_frame_and_return`: Decrements the stack pointer by eight, pushes a fixed string address, branches to a shared exit, then stores a5 in the frame, loads a local a5 and returns.
- `restore_a5_and_return`: Pops a5 from the stack and returns to the caller, restoring the caller's frame pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
