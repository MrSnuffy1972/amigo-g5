# Game 3532 (EmpireStrikesBack v1.2 1235): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03532). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1013597 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 768 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7 of 7 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 13429 |
| code | 18 |
| data | 962261 |
| padding | 1 |
| unused | 37888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |
| Disk.1 bytes 0-768 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: raw disk image (440320 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235.hdf: hard-disk image (AmigaDOS OFS, 1088 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree EmpireStrikesBack_v1.2_1235_hdf, which is recovered file by file; no block outside the filesystem holds data
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: bytes 0-768 look like code no run loaded: recovered position-independently (load address unknown)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: 1 logged loads cover 768 bytes of the image
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 40-248 (bytes 10240-63744) outside the filesystem hold data, not code (opcode density 0.017, entropy 4.83)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 289-289 (bytes 73984-74240) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 292-1234 (bytes 74752-316160) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.85)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1240-1377 (bytes 317440-352768) outside the filesystem hold data, not code (opcode density 0.037, entropy 5.32)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1391-1393 (bytes 356096-356864) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.76)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1395-1403 (bytes 357120-359424) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.82)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1405-1408 (bytes 359680-360704) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.68)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1410-1417 (bytes 360960-363008) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.13)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1426-1434 (bytes 365056-367360) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.72)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1439-1450 (bytes 368384-371456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.67)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1458-1466 (bytes 373248-375552) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.92)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1489-1494 (bytes 381184-382720) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.54)
- EmpireStrikesBack_v1.2_1235/EmpireStrikesBack_v1.2_1235_hdf/EmpireStrikesBack/Disk.1: chunks 1502-1719 (bytes 384512-440320) outside the filesystem hold data, not code (opcode density 0.015, entropy 6.86)
- 1 never-loaded region(s), 768 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
