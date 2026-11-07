# Game 89 (FaeryTaleAdventure v1.0 0367): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00089). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1896906 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 273 of 273 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 739 of 739 named (0 author symbols), 739 of 739 described (70 by rule, 669 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2370 of 2370 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 15505 of 15540 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 147662 |
| code | 53540 |
| data | 750045 |
| padding | 4091 |
| unused | 941568 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Disk.1:fmain | 3 | 13938 | 360 |
| 9 | 1 | 51 | 2 |
| FaeryTaleAdventure.slave | 1 | 231 | 37 |
| Disk.1:bootblock | 1 | 10 | 1 |
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
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
| logged disk load from disk.2 | 1 | 0 | 0 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 633 | 112 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 278 | 221 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 4 |

## Cases handled

- 7 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 265 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 65 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 202 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1:devs/system-configuration: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: 262 logged loads cover 5632 bytes of the image
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 149-159 (bytes 76288-81920) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.69)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.11)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.15)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.57)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 896-917 (bytes 458752-470016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.01)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 931-955 (bytes 476672-489472) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.11)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 960-1171 (bytes 491520-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.37)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 1312-1370 (bytes 671744-701952) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.24)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/Disk.1: blocks 1376-1501 (bytes 704512-769024) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.53)
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/disk.2: 3 logged loads cover 0 bytes of the image
- FaeryTaleAdventure_v1.0_0367/FaeryTaleAdventure/disk.2: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `initialize_disk_validator`: The routine sets up a set of global variables and temporary buffers used for disk validation, writes a character derived from a counter into a table, clears and initializes several work areas, and then enters a loop that prepares bit-mask values for a subsequent check.

### Disk.1:fmain

- `initialize_memory_block`: The routine computes an offset from a global variable, stores a derived address into a global, then clears a block of memory by writing zero.
- `initialize_input_device`: The routine clears a flag byte and obtains a handle via an indirect call, stores it, and if valid opens the device with another indirect call, sets up a small table of pointers and values, and then performs a series of indirect calls that configure the device and copy data from a memory area into a fixed-size buffer nine times.
- `close_input_device_2`: The routine checks several flag bytes in a global structure and, when bits are set, calls an indirect routine with pointers from the structure and constant values.
- `initialize_object_table`: The routine calls another routine with a fixed argument list, then calls another routine with a pointer derived from a global variable.
- `check_and_update_tile`: The routine checks the four orthogonal neighbours of a tile using another routine and then moves the tile index up and right if the neighbour check returns a specific value.
- `initialize_game_engine`: The routine clears a flag byte in a global structure and obtains a handle via an indirect call, then opens an input device with another indirect call.
- `select_and_draw_sprite_2`: The routine updates several global offsets and counters from its parameters, conditionally computes an index from global values, clears flags, calls other routines, and loops calling an indirect routine until it returns zero.
- `process_entity_collision_and_state`: The routine repeatedly selects a table based on a global counter and searches it for an entry matching a global value, then derives a state byte with several conditional adjustments and updates a global state variable, invoking a callback with a per-entity slot when the state changes.
- `update_animation_frame`: The routine reads a byte from a table using an index derived from a global variable, adds it to another byte from a global structure and stores the result in a global byte.
- `initialize_entity_entry`: The routine computes two short values from a global variable and a parameter using another routine, stores them into a per-entity slot in a global table along with several derived bytes from global state and a table lookup, and returns a flag indicating success or a failure after repeated attempts.
- `update_entity_state`: The routine computes an index from a global base and an indirect result, then conditionally clears a word and writes bytes to the indexed structure and to globals.
- `set_state_and_invoke_callbacks`: The routine stores a byte derived from its parameter into a global variable and conditionally copies another global value and calls three indirect routines from a table, then reads a value twice from an indirect routine, adds constants to two global shorts, clears several bytes and stores the parameter into multiple globals.
- `initialize_game_state`: The routine writes a series of constants to a set of global variables and then, depending on a short parameter, calls another routine or performs a block of initialization that updates counters, copies values from a table into globals, clears a small memory area, sets flags, and invokes several indirect routines with arguments derived from globals.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d7.
- `initialize_display_parameters`: The routine calls three indirect routines, then stores a derived short from the parameter into several globals and stores scaled versions of the parameter and an indirect result into other globals.
- `select_and_draw_sprite`: The routine computes an index using several global values and conditionals, then calls an indirect routine either with no arguments or with four pointers from a table based on a flag and a parameter, after optionally invoking another indirect routine.
- `update_position_and_state`: The routine reads two 16-bit values from a global variable, derives a small index from them and conditionally updates a global counter and calls another routine.
- `swap_pointers_and_copy`: The routine swaps two pointers stored in globals, copies data from the swapped pointer into two globals, calls three indirect routines with pointers to globals, restores a field in the pointer structure and calls another indirect routine.
- `open_input_device`: The routine clears two bytes, obtains a handle via an indirect call, stores it, and if valid opens the device with another indirect call, sets up message ports in globals, and conditionally configures the device and returns a status flag.
- `close_input_device`: The routine writes constants to fields of a structure pointed to by a global, calls three indirect routines with that pointer, and then calls an indirect routine with another global handle.
- `populate_and_render_text`: The routine iterates over a byte table indexed by a global short, and for each entry with a specific bit set it stores an index into a byte array and calls another routine with the index and a bit.
- `render_text_line`: The routine computes an index from a parameter and globals with several conditional adjustments, calls indirect routines to set up drawing parameters, performs two reads from an indirect routine, and issues several drawing calls with computed offsets and table data.
- `load_font_metrics_2`: The routine is a state machine that, in one state, prepares a set of pointers and then fills two seven-byte arrays by repeatedly calling an indirect routine with offsets, and also fills a six-byte array while tracking newline characters.
- `wait_for_callback_result`: The routine clears a global integer and repeatedly calls a function pointer while incrementing the integer until a limit is reached or a callback returns a specific value.
- `invoke_callback_with_state`: The routine stores a byte derived from its parameter into a global variable and conditionally copies another global value, then calls three other routines.
- `store_state_if_flag_clear`: The routine tests a byte in a global variable and, if a bit is clear, stores the parameter into a global word, clears a byte in a global variable and calls another routine.
- `load_font_metrics`: The routine fills two arrays of seven bytes each by calling an indirect routine with offsets, fills a six-byte array while tracking newline, stores several single-byte values derived from globals and indirect calls into globals.
- `loop_until_counter_limit`: The routine repeatedly calls another routine while a 16-bit value in a global variable is less than ten.
- `update_display_state`: The routine updates a set of cached 16-bit values from a table entry selected by an index stored in a global variable, invoking another routine to refresh display-related data when a value changes, and then processes a small fixed set of items by calling a getter and a handler, clearing a flag, and finally marks the index as processed.
- `conditionally_call_indirect`: If a flag byte in a global variable is set, the routine calls an indirect function pointer taken from a global variable with three arguments built from two tables indexed by param_1 and param_2.
- `initialize_globals_and_copy`: The routine calls an indirect routine three times with two global addresses and a constant, copies a dword from a table indexed by a short in a global variable into a global dword, then calls the indirect routine once more with another global address and a constant.
- `store_stack_word_to_pointer`: The routine stores the first word from the stack into the dword pointed to by an offset from a register, then returns.
- `process_entity_list`: The routine walks a linked list of entities in A0 and updates a target structure in A1.
- `increment_indexed_byte`: Reads a byte from a global variable, compares it with another byte from a global variable, and if they differ loads a byte from a table indexed by the first byte into a register.
- `update_pseudo_random`: Reads a 32-bit value from a global variable, multiplies it by a constant and adds one, stores the result back to the global variable, and returns a transformed value derived from the updated variable.
- `mask_random_with_parameter`: The routine calls a pseudo-random updater and returns the bitwise AND of the input parameter with the updated random value.
- `return_random_bit`: The routine calls a pseudo-random updater and returns the least significant bit of the updated random value.
- `return_random_two_bits`: The routine calls a pseudo-random updater and returns the two least significant bits of the updated random value.
- `return_random_three_bits`: The routine calls a pseudo-random updater and returns the three least significant bits of the updated random value.
- `return_random_six_bits`: The routine calls a pseudo-random updater and returns the six least significant bits of the updated random value.
- `return_random_byte`: The routine calls a pseudo-random updater and returns the updated random value unchanged.
- `combine_random_modulo_parameter`: The routine calls a pseudo-random updater and returns a combined 32-bit value formed from the remainder of the low 16 bits of the random value modulo the low 16 bits of the parameter together with the parameter.
- `format_number_into_buffer_2_2`: The routine calls an indirect routine via a global offset and returns a concatenated value from two input registers.
- `format_number_into_buffer_2`: The routine writes decimal digits of an input value into a buffer backwards, then fills remaining positions with spaces.
- `format_number_into_buffer`: The routine writes decimal digits of a value into a buffer backwards, and if the value is zero it fills the buffer with spaces by calling a memory fill routine.
- `fill_memory_with_spaces`: Writes the byte 32 to consecutive memory locations starting at a pointer offset by a decreasing short counter, decrementing the counter until it reaches -1.
- `nested_callback_loop`: Executes a triple nested loop that repeatedly calls several routines via function pointers stored relative to a global variable, with conditional extra calls when a counter exceeds a threshold, and returns the original input registers unchanged.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `parse_string_with_callbacks`: Iterates over a null-terminated string, handling a special byte by invoking a callback via a function pointer from a global variable, otherwise measures the length of the next segment and calls another routine with many arguments derived from global variables.
- `sample_pixel_from_map`: Computes a mask from two parameters and global offsets, indexes into tables via a global variable to obtain an index, then tests a byte from a table against the mask and returns a nibble if the test succeeds.
- `sample_pixel_from_map_2`: Computes a mask from two input registers and global offsets, indexes into tables via a global variable to obtain an index, then tests a byte from a table against the mask and returns a nibble if the test succeeds.
- `decrement_counter_and_call`: Calls a routine via a function pointer from a global variable and decrements a 32-bit counter stored in a global variable by four.
- `adjust_counter_and_call`: Calls a routine via a function pointer from a global variable and subtracts the sum of a global variable and a constant from another global counter.
- `chain_multiple_calls`: The routine calls another routine with a value loaded from a global variable followed by many calls to another routine and returns the concatenation of two input registers.
- `call_copy_block_to_multiple_buffers`: The routine calls another routine six times in a row and then returns.
- `copy_block_to_multiple_buffers`: Copies 16-bit words from a source pointer derived from a parameter and a register offset into five destination buffers using a loop that processes pairs of source words per iteration.
- `empty_routine`: The routine contains no instructions and immediately returns.
- `call_once_and_return`: The routine calls another routine with a value loaded from a global variable and returns the concatenation of two input registers.
- `call_handler_table_entries`: The routine calls another routine once with an argument derived from a global variable and then calls it repeatedly with no arguments.
- `call_copy_block_to_multiple_buffers_2`: The routine calls another routine once and then returns.
- `compute_tile_indices_and_fill`: The routine extracts high and low parts from two parameters, adjusts short values using a global variable, clamps the results, stores the combined indices into two locations in a global variable, then calls another routine 18 times in a nested loop.
- `decode_tile_bits`: The routine reads five indices from a global variable and uses them to address a table of words.
- `copy_tile_words`: The routine copies a block of word data from a source table to a destination table using calculated offsets derived from parameters and a global variable.
- `compute_tile_index`: The routine adjusts two input values using global offsets with sign handling, then combines them with a byte from a table indexed by the adjusted values to form a short result, adding a global offset.
- `fill_tile_buffer`: The routine loops over a range of values, computes an index using global offsets and a table lookup, and writes bytes from a table into a buffer of words.
- `copy_decompressed_bytes`: The routine copies bytes from a source pointer to a destination pointer, handling a run-length encoding where a negative byte indicates a repeat count.
- `scale_value_by_table`: The routine conditionally scales a short parameter by a value from a table indexed by the second parameter and adds the result to the first parameter, masking the result to fifteen bits.
- `scale_value_with_sign`: The routine conditionally scales a short parameter by a value from a table indexed by the second parameter, adds it to the first parameter, preserves the sign bit of the first parameter, and masks the result to fifteen bits.
- `mask_input_word_2`: The routine tests a bit in the input word and returns the word with the high bit set if the bit is set, otherwise it returns the word with the high bit cleared.
- `mask_input_word`: The routine returns the input word with the high bit cleared by masking with a constant.
- `adjust_position_counters_2`: The routine computes two signed deltas from the parameters and values read from globals, then conditionally updates two shorts in globals by setting them from the parameters with offsets or incrementing and decrementing them by one based on the delta ranges.
- `adjust_position_counters`: The routine conditionally increments or decrements two shorts stored in globals based on the sign of two input shorts, or sets them from input registers under certain thresholds.
- `adjust_counter_by_input_2_2`: The routine reads a short from a register and conditionally increments or decrements a short stored in a global variable based on the sign of a value derived from hardware registers.
- `adjust_counter_by_input_2`: The routine checks a short from a register and increments or decrements a short stored in a global variable depending on its sign, or otherwise stores a value derived from the register.
- `adjust_counter_by_input`: The routine checks an input short and increments or decrements a short stored in a global variable depending on its sign, or otherwise stores a computed value into the global short.
- `increment_counter_if_large_2`: The routine checks a short input value and increments or decrements a short value stored in a global variable depending on whether the input is greater than ten or less than negative ten.
- `increment_counter_if_large`: The routine checks an input word and, if it is greater than ten, increments a short value stored in a global variable.
- `handle_error_message`: The routine checks a parameter and, if it is not equal to two, calls three other routines with constant arguments and a global variable pointer.
- `select_blitter_mode_2`: The routine indexes a table with a short parameter, applies a series of range checks to choose a small index, then returns a combined value formed from that index and an input register.
- `select_blitter_mode`: Reads two custom chip registers and a global variable to compute an index, then looks up a value in a table or selects a mode based on another global variable.
- `process_joystick_input`: The routine reads bytes from JOY1DAT and custom hardware registers, derives difference values, indexes a table to obtain a short, and updates two shorts in global variables.
- `read_joystick_and_update_sprite`: The routine reads two bytes from JOY1DAT and custom+0, derives two difference values and uses them to index a table to obtain a short.
- `update_sprite_frame_index`: The routine computes a short from a global short, clamps it, updates another global short if changed and calls another routine with the new value.
- `draw_sprite_frame_2`: The routine clears a word in a global variable, conditionally writes a constant to another word in a global variable and calls another routine with a constant argument, then returns a value from the stack.
- `sample_pixel_from_map_2_2`: The routine calls another routine twice under conditional tests on the returned short value and may return zero, otherwise it returns the value from the first call.
- `compute_morphed_bitmap`: Takes two pointer arguments and three short arguments and performs a nested loop that writes the bitwise NOT of the AND of five source words into a destination word for each iteration.
- `setup_blitter_copy`: Calls library routines and writes several blitter registers in a loop over four entries, using a parameter to index a table and a global variable to compute source addresses.
- `set_blitter_regs`: Writes a constant and two parameters to a set of blitter registers and returns zero.
- `configure_blitter_mask`: Calls a library routine and writes blitter registers in a loop over four entries, using values from a global variable and a table to compute masks and addresses.
- `configure_blitter_single`: Calls a library routine if a hardware flag is set and writes a fixed set of blitter registers using values from global variables and a table.
- `setup_blitter_loop`: Calls a library routine if a hardware flag is set and writes blitter registers in a loop over four entries, using a parameter and values from a table to compute addresses.
- `setup_blitter_loop_alt`: Calls a library routine if a hardware flag is set and writes blitter registers in a loop over four entries, using a parameter and values from a table to compute addresses with source and destination swapped compared to the previous routine.
- `indirect_jump`: Performs an indirect call through a pointer stored in a global variable and then returns.
- `call_indirect_from_table`: The routine loads an address from a global variable and calls another routine through an indirect pointer.
- `adjust_sprite_offset_flags`: The routine computes signed deltas from a table entry, derives sign flags and a size flag, optionally inverts signs, indexes a table of codes and writes a byte derived from that code and a flag into an output structure, updating a byte in the table entry when a specific code is found.
- `compute_sprite_offset_flags_2_2`: The routine computes signed deltas from input words using a table indexed by a parameter, derives sign flags and a size flag, optionally inverts signs, and indexes a table of code pointers.
- `compute_sprite_offset_flags_2`: The routine computes signed deltas from input words, derives sign flags and a size flag, optionally inverts signs, indexes a table of code pointers and writes bytes to an output structure based on the selected code and a global function pointer result.
- `compute_sprite_offset_flags`: The routine computes signed deltas from input words, derives sign flags and a size flag, optionally inverts signs, indexes a table of code pointers and stores resulting bytes into memory pointed to by arguments.
- `store_byte_difference_2`: The routine calls a function pointer from a global variable, conditionally negates a byte, stores the masked sum of two bytes into a byte at an offset from a pointer argument, and conditionally stores a constant byte.
- `store_byte_difference`: The routine stores the masked difference of two uninitialized bytes into a byte at an offset from a pointer argument and conditionally stores a constant byte to another offset from the same pointer.
- `decrement_timer_and_dispatch_2`: The routine obtains an index via indirect calls, checks two flag bytes for specific values, decrements a short timer entry using a value from a parameter, and then calls another routine with computed arguments.
- `decrement_timer_and_dispatch`: The routine obtains an index via indirect calls, checks two flag bytes for specific values, decrements a short timer entry using a value from a structure and clears it if negative, then dispatches to another routine with different parameters based on two registers.
- `decrement_and_dispatch_updates`: The routine decrements a short in a table using a value from a global variable and clears a negative entry, then conditionally calls another routine with an offset derived from an indirect function pointer selected by input flags.
- `dispatch_entity_updates_conditionally`: The routine selects one of three indirect calls from a global variable based on two input values, each time adding a different constant to the returned value before passing it to another routine.
- `update_entity_with_conditional_offsets`: The routine branches on a register to call an indirect function pointer from a global variable with different constants and adds a constant offset to the result before passing it to another routine.
- `update_entity_state_2`: The routine calls an indirect function pointer from a global variable, then calls another routine with a constant and an offset value.
- `process_input_counts_3`: The routine loops over a range defined by a short in a global variable, repeatedly calling an indirect handler and incrementing two counters when specific bytes are found in tables.
- `process_input_counts`: The routine repeatedly calls a handler obtained from a base pointer and increments two counters based on byte values read from a table indexed by the handler result, then loops a number of times from a global variable.
- `loop_and_call_handlers`: The routine repeatedly calls a function pointer from a global variable, increments two counters based on byte values from a global variable, and under several conditions calls another routine with a global variable and a function pointer from a global variable, then returns.
- `process_input_counts_2`: The routine repeatedly calls an indirect handler, increments counters when specific bytes are found in tables, and under certain short conditions calls other routines with a global variable pointer and an indirect function.
- `display_battle_result_message_2`: The routine checks shorts in global variables and conditionally calls other routines with a global variable pointer and an indirect function.
- `display_battle_result_message`: The routine checks a short in a global variable and conditionally calls two routines with a global variable pointer and an indirect function pointer.
- `sample_pixel_from_map_2_2_2`: The routine checks bytes in a table indexed by a parameter and conditionally calls another routine with two parameters.
- `check_position_bounds_2_2`: The routine loops with an internal counter and skips iterations based on a short from a global variable and on byte values from a table indexed by an indirect function.
- `check_position_bounds`: The routine loops over an index and reads bytes from a table indexed by a function result to test for specific values.
- `check_position_bounds_2`: The routine loops with an internal counter and skips iterations based on a short from another pointer and on byte values from a table indexed by a handler result.
- `find_minimum_delta`: The routine initializes a short in a global to zero and loops with a counter, returning the parameter when a limit is reached.
- `compare_table_deltas_2`: The routine loops with an internal counter, checks a byte in a table indexed by an indirect function, and conditionally calls another routine with a parameter and a constant.
- `compare_table_deltas`: The routine obtains two pairs of values from a table using an indirect function, computes absolute differences, compares the differences and conditionally calls two indirect functions obtained from a base pointer before returning a value.
- `invoke_two_callbacks_2`: The routine tests a comparison between two registers and, if true, calls two indirect functions obtained from a base pointer, then returns.
- `invoke_two_callbacks`: The routine calls two indirect functions obtained from a base pointer and then returns.
- `update_table_entry`: The routine calls two indirect functions with values read from a table indexed by a parameter, then calls another routine with those results and the parameter.
- `store_two_word_values`: The routine stores two word inputs into two locations in a table indexed by a short field from a structure, using a stride of 22, and returns a constant.
- `draw_sprite_frame`: The routine loads four bytes from a table indexed by a parameter using a global variable, writes a value from a global variable to a location pointed to by another global variable, and calls another routine with several arguments derived from the loaded bytes and global variables.
- `fill_tile_map`: The routine sets a short in a global variable based on the value of another global short, clamps an input short to a range and optionally calls another routine, then loops twenty times calling two routines to obtain three values, combines them with the input short and stores the result into a table via a global variable.
- `randomize_tile_map`: The routine runs an outer loop twenty times, each iteration running an inner loop twenty times that calls a routine to obtain a word and stores it into a table via a global variable.
- `dispatch_message_handler_2_2`: The routine compares two shorts from a base pointer and either calls an indirect function or reads a byte from a table indexed by a short counter, increments and masks the counter, then switches on the byte to perform a series of indirect calls and data accesses using values from global variables.
- `dispatch_message_handler_2`: The routine reads a byte from a table indexed by a short counter, increments and masks the counter, then switches on the byte.
- `dispatch_message_handler`: The routine switches on an input value and for each case calls several other routines with constant arguments and calls a routine with values loaded from global variables, performing a conditional write to a global short in one case.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `store_input_byte`: The routine computes a wrapped index from a global short, compares it with another global short, and if they differ stores the input byte into a byte table indexed by the previous value and updates the global short.
- `initialize_display_2`: The routine computes the length of a null-terminated string passed as a parameter, then calls three other routines with arguments loaded from global variables and constants, performing initialization steps for display.
- `initialize_display`: The routine calls three other routines with arguments loaded from global variables and constants, performing initialization steps for display related data.
- `measure_string_and_forward`: The routine computes the length of a null-terminated string passed as a parameter by scanning for the terminator and then calls another routine with the string pointer and its length.
- `self_recursive_call`: The routine contains no instructions and performs no operations.
- `format_string_with_table_2`: The routine sets a pointer in a global to a buffer, then copies bytes from an input string to that buffer handling spaces, null and carriage return, and expands percent signs by copying from a table indexed by a short in a global.
- `parse_format_string_2_2`: The routine reads a byte from a pointer and writes it to a buffer pointed to by a global variable, handling spaces, carriage returns and null terminators, and updates a length counter.
- `parse_format_string_2`: The routine copies bytes from a source buffer through a pointer into a destination buffer, handling spaces, carriage returns and null terminators.
- `parse_format_string`: The routine copies bytes from a source buffer through a pointer into a destination buffer, handling spaces, carriage returns and null terminators, and updates counters.
- `parse_format_string_2_2_2`: The routine reads a byte from a pointer and writes it to a buffer pointed to by a global variable, handling spaces, carriage returns and nul.
- `format_string_with_table`: The routine copies characters from a source buffer into a destination buffer while tracking length via a global counter, inserts table entries when a percent sign is encountered, and terminates the output on null, carriage return or space.
- `empty_routine_2`: The routine contains three instructions and performs no observable work; it ends immediately without accessing memory, registers, or calling another routine.
- `format_string_with_table_2_2_2`: The routine copies characters from an input string into a buffer pointed to by a global variable, handling spaces, null and carriage return as delimiters.
- `format_string_with_table_2_2_3`: The routine copies characters from an input string into a buffer pointed to by a global variable, handling spaces, null and carriage return as terminators, and expands percent signs by copying bytes from a table entry selected by an index stored in a global variable.
- `format_string_with_table_2_2`: The routine sets a pointer in a global variable to a buffer and copies bytes from an input string to that buffer, handling spaces, null and carriage return.
- `call_other_routine_four_times`: The routine calls another routine four times in succession with different arguments derived from a global variable, a table indexed by a short in a global variable, the routine parameter, and a global variable again.
- `call_other_routine_three_times`: The routine calls another routine three times with arguments taken from a table indexed by a short in a global variable, a global variable, and the routine parameter.
- `call_other_routine_once`: The routine calls another routine once with an argument loaded from a table indexed by a short in a global variable.
- `setup_display_and_clear_fields`: The routine stores a pointer into a global variable, copies a value from an indirect pointer into another global, invokes three indirect routines with the stored pointer, writes a constant to a global word, calls another routine, invokes an indirect routine with three arguments, and stores a constant byte into a global variable.
- `initialize_tile_and_flag`: The routine calls another routine, stores a pointer into a global variable, writes a constant to a global word, calls another routine, and stores a constant byte into a global variable.
- `loop_decrementing_tile_fill`: The routine loops with a decreasing counter starting at 100 and stepping by -5 down to -1.
- `loop_incrementing_tile_fill`: The routine loops with an increasing counter starting at 0 and stepping by 5 up to 101.
- `clear_two_word_fields`: The routine stores zero into two 16-bit locations accessed through a pointer held in a global variable, then calls another routine.
- `copy_and_prepare_structures`: The routine copies a pointer from one global variable to another, calls another routine multiple times with constant and indexed arguments, stores a pointer into a global variable, reads values from globals and tables to pass as arguments, copies a word from one global to another, and calls another routine twice.
- `populate_struct_from_table`: The routine reads values via an indirect handler, stores a shifted short into a global array, copies a pointer from a global variable into another global array, and loads three bytes from a table indexed by the parameter into three 16-bit globals.
- `copy_struct_and_update_state`: The routine stores a pointer derived from a global variable into a global variable, conditionally calls another routine via a handler pointer, copies fourteen words from a source pointer to a destination pointer taken from a global variable, writes shifted parameters and a constant into fields of the pointed-to structure, and then calls another routine with that pointer.
- `copy_block_and_invoke_handler`: The routine copies fourteen words from a source pointer taken from a global variable to a fixed stack-relative area, clears a global word and writes a constant byte to another global location, then calls a handler routine via a pointer with the destination address.
- `empty_routine_19`: The routine is empty and performs no operations.
- `prepare_and_compute_bitmap`: The routine calls an indirect routine with a fixed address, clears a 16-bit global, then calls another routine with five arguments loaded from a set of global arrays indexed by the parameter.
- `conditional_display_update`: The routine checks a 16-bit field from a structure pointed to by a global variable for the value 2 and calls an indirect routine with a pointer from a global variable.
- `load_song_table_entries`: The routine opens a file referenced by a global variable, iterates up to twenty-eight times reading a length and data via handler pointers, stores computed offsets into a global table while checking a size limit, and closes the file handle.
- `initialize_screen_and_input`: The routine checks a global flag and, if clear, calls several handlers to set up a screen area and copy data from two sources into a buffer using a blit operation.
- `loop_over_table_and_blit`: The routine loops over a fixed range of indices, loads data from two tables indexed by the loop counter, conditionally invokes an indirect routine with many arguments and performs nested loops that call another routine and invoke an indirect routine with computed arguments, accumulating a value.
- `check_key_and_set_flag`: The routine calls a handler to obtain a value, stores a boolean result indicating whether the value equals a constant into a global byte, and returns that boolean.
- `allocate_and_copy_buffer`: The routine checks a pointer argument against a limit, allocates a buffer of a given size via a handler, copies the requested number of bytes from the original pointer into the new buffer, and returns the new pointer.
- `compute_triangle_orientation_flags`: The routine computes two signed 2-D cross products from four points derived from a parameter block and a global table, stores the results and sets two flag words based on whether each result is non-negative.
- `initialize_sprite_collision_state`: The routine writes a constant word to a global variable, then calls another routine twice with arguments derived from global variables and indexed data.
- `store_entry_from_table`: The routine copies a word from a table indexed by a parameter into a global word, adds a constant to another table word and stores it into a global word, stores a byte parameter into a global byte, and writes a constant byte to another global byte.
- `update_byte_via_table_2`: The routine obtains a base pointer via an indirect call, selects an offset using a word from a global variable and a table or a fixed offset, then writes either a constant byte or the routine parameter byte into the computed location based on a byte test.
- `update_byte_via_table`: The routine reads a word from a global variable, calls another routine to obtain a base pointer, adds an offset from a table indexed by the word, and then conditionally overwrites a byte at a fixed offset from the resulting pointer, otherwise stores a byte from a register into memory at a fixed offset.
- `store_byte_to_offset`: The routine stores a byte from a register into memory at a fixed offset from a register.
- `generate_sprite_collision_table`: The routine initializes a few global variables and, when a condition on a global flag and a counter is met, it fills a per-object table with ten random 16-bit values produced by a random routine via a function pointer, storing the values and a byte from a table.
- `update_sprite_collision_table`: The routine repeatedly reads two masked bits from a global variable, calls a routine through a pointer in a global variable to obtain two short values, adds the masked offsets, and loops until a collision test routine through another pointer returns zero.
- `process_sprite_table_entry_3`: The routine iterates over a table of sprite descriptors pointed to by a global variable and copies fields into a per-entry structure, updating position fields and a counter.
- `update_sprite_collision_and_animation`: The routine reads sprite data from a pointer stored in a global variable, updates position fields and a counter, and checks bounds against a global table.
- `process_sprite_table_entry_2`: The routine iterates over a table of sprite descriptors pointed to by a global variable and copies fields into a per-entry structure, updating position and animation counters.
- `update_sprite_animation_state_2`: The routine reads sprite data from a pointer stored in a global variable, updates position fields and a counter, and checks bounds against a global table.
- `update_sprite_animation_state`: The routine reads sprite data from a pointer stored in a global variable, updates position fields and a counter, and checks bounds against a global variable.
- `process_sprite_table_entry`: The routine processes a table of sprite descriptors pointed to by a global variable, copying fields into a per-entry structure, updating position offsets, a counter and type flags, and checks bounds against a global counter.
- `process_sprite_collision_and_animation_2`: The routine reads sprite data from a pointer stored in a global variable, updates position fields and a counter, and checks bounds against a global table.
- `process_sprite_collision_and_animation`: The routine reads sprite data from a pointer stored in a global variable, updates position fields and a counter, and checks bounds against a global table.
- `empty_routine_16`: The routine contains no usable instructions and returns immediately.
- `verify_command_input`: The routine reads a line of input via library vectors, echoes characters while building a buffer in a global variable, and compares the entered text against a null-terminated string from a table indexed by a global variable.
- `process_input_line`: Reads characters via library vectors, stores printable characters into a buffer in a global variable while updating a counter, and on carriage return compares the buffer with a string from a table.
- `read_input_and_dispatch`: The routine reads characters via library vectors, stores printable characters into a buffer in a global variable, updates a counter and calls another routine with the current length.
- `compare_input_and_dispatch`: Compares a buffer from a global variable with a string from a table and on match clears an entry and increments a counter in a global variable.
- `compare_and_process_input`: The routine compares a buffer in a global variable with a string from a table and on match clears an entry, increments a counter in a global variable and dispatches work via library calls and another routine.
- `process_input_loop`: The routine clears an entry in a table, increments a counter in a global variable and dispatches work via library calls and another routine.
- `call_library_cleanup_sequence`: The routine performs several calls to library vectors via a global base and via a pointer taken from a global variable, invoking cleanup functions in a repeated sequence.
- `store_and_dispatch_result`: The routine calls another routine with two parameters, stores the result into a global variable and if the stored value is non-zero calls another routine indirectly via a pointer obtained from a global variable table.
- `store_result_from_call`: Calls another routine with a global variable and a negative immediate, stores the result through a pointer derived from A4, and if the stored value is non-zero it conditionally calls another routine based on a value obtained by indexing two tables, then calls another routine with the stored value before returning it.
- `empty_routine_3`: The routine contains no usable instructions and ends immediately.
- `wait_flag_or_timeout_2`: Loops calling a library routine and incrementing a short counter on the stack until the counter exceeds a limit or a byte in a global variable becomes non-zero.
- `wait_for_flag_or_timeout`: The routine loops while a short counter in a global variable is less than a limit, calling a library routine each iteration and incrementing the counter.
- `wait_flag_or_timeout`: Loops calling a library routine and incrementing a short counter in a global variable until the counter exceeds a limit or a byte in a global variable becomes zero.
- `handle_disk_insert_prompt_3`: The routine clears a global flag, calls an indirect routine with two globals, then repeatedly polls a global variable via another routine.
- `handle_disk_insert_prompt_2`: The routine repeatedly polls a global variable and a table entry and, when a condition is met, writes a byte to a structure, copies a character from a table into a byte of a structure and then calls an indirect routine with a constant to initialize a global state variable.
- `handle_disk_insert_prompt`: The routine repeatedly polls a global variable and a table entry and, when a condition is met, writes a byte to a structure, copies a character from a table into a structure, selects a message string based on a flag and stores the result, then either processes a disk-related operation or updates display state and waits for a flag or timeout before returning.
- `process_dialogue_entry`: The routine repeatedly polls a global variable and a table entry and, when a condition is met, writes a byte to a structure, copies a character from a table into a structure field, calls another routine with a pointer and a constant to obtain a value, stores the result, and conditionally invokes several other routines with pointers and computed sizes derived from table entries.
- `initialize_game_state_3_2`: The routine copies a byte from a table into a global variable, then calls an indirect routine with a constant to initialize a global state variable, conditionally choosing the constant based on a flag.
- `initialize_game_state_3`: The routine initializes global state by calling an indirect routine with a constant and storing the result, then conditionally runs a secondary initialization path that copies data from tables indexed by short values into memory areas via repeated indirect calls, and finally dispatches a library routine with a saved pointer.
- `handle_save_error_and_init`: The routine references the text "ERROR: Couldn't save game." and calls initialize_display_2, initialize_game_state_2 and dispatch_library_call_minus_132.
- `initialize_game_state_2`: The routine loops calling another routine and storing the result into a global variable until the value is non-zero, then calls another routine with that value.
- `wait_for_disk_and_init`: The routine loops calling other routines and storing a returned value into a global variable until the value is non-zero, then calls another routine with that value.
- `initialize_display_state`: If a byte in a global variable is zero it clears several bytes and words in globals, calls other routines, sets bytes in globals and calls other routines with constants and addresses of global variables, then calls an indirect routine with two values loaded from globals.
- `conditional_indirect_call`: Selects between a library routine and another routine based on a byte in a global variable, calls it with two parameters, and if the returned word is negative stores the result of another routine into a global variable.
- `dispatch_library_call_minus_132_2`: The routine performs an indirect call through a pointer derived from a global variable, stores the returned word into a global variable, and if that word is negative it calls another routine and stores the returned long into a global variable.
- `write_bounds_to_table`: The routine computes an address in a table using an index, then stores four short values derived from the parameters with constant offsets into consecutive entries of that table.
- `update_display_state_2`: The routine updates several global offsets and counters from its parameters, conditionally computes an index from global values, clears a flag, and writes a series of bytes derived from a global pointer.
- `draw_winpic_screen`: Calls several indirect routines, stores a value from a global pointer into a global variable, calls routines to copy data into buffers, initializes words in globals, copies words from a global table into a buffer using counters, sets shorts in globals conditionally, copies buffers, and delays via library calls in a loop.
- `initialize_display_buffers_2`: Clears a range of words in a global buffer and copies words from another global table into it using two counters.
- `initialize_display_buffers`: The routine repeatedly calls a function pointer to obtain short values, stores them shifted into global variables, then enters a loop that calls function pointers to manipulate memory regions and update global words, copies words from a table into a buffer with an offset, and exits after a counter passes a threshold.
- `return_conditional_size`: The routine loads a constant 8 and returns it, or returns 10 if a byte fetched via an offset from a global variable and an argument is non-zero.
- `select_tile_map_2`: Loads a short from a global variable to choose an offset, masks another short from a global variable, and conditionally calls another routine with arguments derived from globals or constants, returning the result.
- `select_tile_map`: Masks a short from a global variable and conditionally calls another routine with arguments derived from global shorts or constant values, then returns the resulting word.
- `call_fill_tile_map`: The routine calls another routine with constant arguments and a pointer derived from a global variable, then returns.
- `select_fairy_sprite_entry`: The routine selects a sprite entry from a table using a value returned by another routine and writes a byte from a register into the entry.
- `handle_fairy_sprite_event`: The routine selects a sprite entry from a table using a value returned by another routine and writes a byte from a register into the entry.
- `compute_sprite_offset_flags_2_2_3`: The routine computes signed deltas from input words using a table indexed by a parameter, derives sign flags and a size flag, optionally invoking indirect calls.
- `compute_sprite_offset_flags_2_2_2`: The routine computes signed deltas from input words using a table indexed by a parameter, derives sign flags and a size flag, optionally invoking indirect calls.
- `call_empty_stub`: The routine contains no observable logic and immediately returns.
- `set_sprite_flag_2`: The routine checks if a register equals ten and another register is non-zero, and if so calls another routine.
- `dispatch_sprite_updates_by_type_2`: The routine branches on a type value in a register and, when a flag is non-zero, calls another routine and optionally invokes indirect calls.
- `dispatch_sprite_updates_by_type`: The routine branches on a type value in a register and, when a flag is non-zero, calls another routine and optionally invokes indirect calls through a table to update sprite-related data.
- `handle_sprite_flags`: The routine checks an input value against six and ten, computes signed deltas from input words using a table indexed by a parameter and derives sign flags and a size flag, optionally invokes another routine, tests if a register equals ten and another register is non-zero and if so calls another routine, and sets bytes in a structure pointed to by a register based on input values.
- `check_flags_and_update_sprite`: The routine tests an input value against six and ten.
- `conditionally_update_sprite_table`: The routine checks a register for non-zero and, if set, calls an indirect function pointer obtained from a global variable with argument two, then calls the same indirect pointer with a value loaded from a table indexed by the previous result plus an offset, and finally calls another routine before returning.
- `set_sprite_flag`: The routine checks if a register equals ten and another register is non-zero, and if so calls another routine.
- `decrement_counter_and_branch`: The routine subtracts a parameter from a global short, and if the result is negative it writes zero to the global short and calls another routine with a constant, otherwise it calls another routine with itself as argument.
- `empty_stub_3`: Contains no instructions that perform work; it simply calls another routine with itself as argument and returns.
- `decrement_word_and_update_state`: The routine decrements a word pointed to by a computed address using a global pointer and an offset, then calls three routines via pointers from a global variable, stores two returned words into two global variables, and returns.
- `update_two_word_state`: The routine calls three routines via pointers from a global variable, then stores the two returned words into two global variables and returns.
- `process_tile_map`: The routine computes an offset from its parameters and stores the result of a callback into a global variable.
- `process_script_state`: The routine clears a byte at offset one of the structure pointed to by A1, adds a 16-bit value from offset two to a 32-bit field at offset 16, and if the first byte is zero it calls another routine four times.
- `process_script_bytecode`: The routine processes a script state stored in a structure pointed to by an argument and updates output fields in another structure.
- `decrement_counter_and_update_state`: The routine writes constant words to two offsets of a structure pointed to by the first argument, then reads a byte from an offset of the second argument.
- `initialize_audio_and_dma`: The routine stores bytes to two offsets relative to a global base, writes a value to a hardware register, stores three parameters to three hardware registers, writes a constant to a global variable and to two hardware registers, and returns.
- `clear_audio_channel_flags`: The routine clears the least significant bit of a byte at an offset relative to a global base, writes zero to a hardware register and writes a constant to another hardware register, then returns.
- `store_four_parameters`: The routine stores the four input parameters to four distinct offsets relative to a global base and returns.
- `initialize_audio_channels_and_state`: The routine stores the four input parameters to multiple offsets relative to a global base, clears four hardware registers, copies four words from a table pointed to by a global variable into four offsets, writes constant bytes to four offsets, clears a series of words and bytes at offsets relative to the global base, writes a constant to a hardware register, and returns an input register unchanged.
- `reset_audio_volumes_and_flags`: The routine writes zero to four hardware registers, writes a constant to a hardware register, writes a constant byte to an offset relative to a global base, and returns.
- `setup_interrupt_handlers`: Stores parameters into global variables and initializes several fields of structures pointed to by global pointers, then calls library routines to add an interrupt server and set an interrupt vector, and writes to hardware registers INTENA and INTREQ.
- `install_interrupt_handler`: The routine calls another routine, then calls a function through a global variable, writes a constant word to an address derived from A0, and calls another function through a global variable.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_memory_and_init_globals`: The routine computes an offset from a global variable, stores a derived address into a global, then clears a block of memory by writing zero to successive words.
- `empty_stub_2`: The routine contains no instructions that affect memory or registers and returns immediately.
- `parse_and_copy_string`: The routine obtains a handle via a library call, stores it in a global variable and aborts if the handle is zero.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_3`: Small routine of 1 instruction that copies d0 to d5.
- `copy_string_pad_zeros`: The routine copies bytes from a source pointer to a destination pointer, decrementing a counter each iteration and stopping on a null byte or when the counter reaches zero.
- `copy_string_with_limit_2`: The routine finds the length of the destination string by scanning for a null terminator, then copies bytes from a source pointer to the destination up to a limit, terminating the result with a null if needed.
- `copy_string_with_limit`: The routine finds the length of the destination string by scanning for a null terminator, then copies bytes from a source pointer to the destination starting at the byte after the terminator, decrementing a counter each iteration and stopping on a null byte or when the counter reaches -1.
- `copy_string_to_buffer`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator, and returns the original destination pointer.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `perform_signed_division`: The routine determines the sign of two inputs, calls another routine to perform division, optionally negates the result based on the sign, and returns a 64-bit value formed by concatenating the adjusted result with the second input.
- `signed_divide_32`: Calls another routine and adjusts the sign of the result based on the sign of an input, then returns a 64-bit value formed by concatenating the adjusted result with another input.
- `call_and_return_concatenated`: The routine calls another routine and returns a 64-bit value formed by concatenating the value left in D1 by the call with the original D1 contents.
- `recursive_call`: The routine calls itself and then returns without performing any other operation.
- `perform_32bit_division`: The routine calculates the quotient and remainder of a 32-bit integer division using inputs in registers D0 and D1.
- `compute_string_length`: The routine receives a pointer to a zero-terminated byte string, walks forward byte by byte until it finds the terminating zero byte, and returns the number of bytes from the start pointer up to but not including the terminator.
- `invoke_callback_and_loop`: If a pointer in a global variable is non-zero it calls the routine through that pointer, then calls another routine that loops a fixed number of times checking fields in a structure.
- `process_message_loop`: The routine loops a fixed number of times calling another routine, checks several fields in a structure for non-zero values and conditionally calls other routines with values taken from that structure, and returns a value obtained from another routine.
- `validate_and_clear_table_entry`: The routine indexes a table using an input value and checks bounds and a non-zero entry.
- `dispatch_library_call_minus_36`: The routine performs an indirect call through a pointer derived from a global variable.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `dispatch_library_call_minus_132`: The routine performs an indirect call through a pointer derived from a global variable.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_150`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `dispatch_library_call_minus_84`: The routine performs an indirect call through a pointer derived from a global variable.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `dispatch_library_call_minus_30`: The routine performs an indirect call through a pointer derived from a global variable.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `dispatch_library_call_minus_60`: The routine performs an indirect call through a pointer derived from a global variable.
- `dispatch_library_call_minus_42`: The routine performs an indirect call through a pointer derived from a global variable.
- `indirect_call_via_table`: The routine performs an indirect call through a pointer loaded from a global variable offset, adding a constant to the loaded value before calling.
- `call_library_vector_156`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `call_indirect_through_table`: The routine calls another routine indirectly via a pointer obtained from a global variable table.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable table.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `call_indirect_via_table_2`: The routine calls another routine through an indirect pointer obtained from a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `allocate_and_initialize_node`: The routine obtains an identifier, allocates a block via a library call, fills fields of the block with the parameters and other values, and calls another routine with the block address.
- `initialize_structure_and_call`: The routine checks a field in the passed structure and conditionally calls another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `forward_call_with_constant`: The routine calls another routine via an indirect pointer obtained from a global variable offset, passing the incoming argument and a constant value.
- `forward_call_with_argument`: The routine calls another routine via an indirect pointer obtained from a global variable offset, passing the incoming argument unchanged.
- `allocate_and_initialize_block`: The routine returns zero if the first argument is zero.
- `initialize_message_structure_2`: If the parameter is non-zero it writes constant bytes and words to offsets in the structure pointed to by the parameter and then calls another routine through a pointer derived from a global variable, returning the result.
- `initialize_message_structure`: The routine writes constant bytes and words to offsets in a structure pointed to by A2 and then calls another routine through a pointer derived from A4.
- `dispatch_indirect_call_2`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there via an indirect jump.
- `call_indirect_from_table_2`: The routine loads a pointer from a global variable, adds an offset to it, and calls the routine found there via an indirect jump.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_indirect_through_table_2`: The routine calls another routine indirectly via a pointer obtained from a global variable table.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_indirect_library_stub`: The routine loads a library base from a global variable and performs an indirect call through a computed address derived from that base.
- `call_library_vector_210_2`: The routine loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: The routine stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_552_2`: The routine loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_474`: Library stub: loads a library base from a global variable and calls the library function at offset minus 474 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_258`: Library stub: loads a library base from a global variable and calls the library function at offset minus 258 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_indirect_library_stub_2`: The routine loads a library base from a global variable and performs an indirect call through a computed address derived from that base.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_462_2`: The routine loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_414_2`: The routine loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_198_2`: The routine loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result.
- `call_library_vector_468_2`: The routine loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result.
- `call_library_vector_360_2`: The routine loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `indirectly_call_handler`: The routine loads an address from a global variable via an offset from a register and calls another routine through an indirect pointer with an added offset.
- `call_indirect_via_table_3`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there through an indirect jump.
- `call_indirect_via_table_4`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine pointed to by the resulting address.
- `call_indirect_via_global_table`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at that address.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_indirect_via_table_5`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there via an indirect jump.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_234`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_indirect_via_table_6`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the resulting address as a function.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_402`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `call_indirect_via_table_7`: The routine loads a function pointer from a global table via an offset and calls another routine through it.
- `call_indirect_via_global_table_2`: The routine loads a pointer from a global variable and calls another routine through an indirect jump.
- `call_indirect_via_table_8`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at the resulting address.
- `invoke_indirect_handler`: The routine reads a pointer from a global variable, adjusts it, and calls the routine it points to.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `return_all_ones`: The routine returns the constant all-ones value.

### FaeryTaleAdventure.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `store_globals_and_indirect_jump`: The routine copies a global variable to another global variable, stores an input value into a global variable, and then performs indirect calls through computed addresses derived from a pointer and a global variable.
- `write_pointers_and_indirect_jump`: The routine writes the same computed address into two locations of an unrecovered jump table and then performs an indirect call through that table after calling another routine.
- `indirect_jump_via_table`: The routine performs an indirect call through an unrecovered jump table and then returns.
- `mask_global_variable`: The routine returns the value of a global variable with bits cleared by an AND mask.
- `return_stack_value`: The routine returns the value passed on the stack without modification.
- `store_stack_to_global`: The routine copies a 16-bit value from the stack to a global variable.
- `return_immediately_zero`: The routine returns zero without performing any operations or modifying data.
- `set_flag_in_struct`: The routine receives a pointer in a register and sets bit 1 in the byte at offset 14 of the pointed-to structure.
- `empty_routine_17`: The routine contains no observable work and returns immediately to its caller.
- `store_register_to_global`: The routine copies the value from D0 into a global variable and performs no other observable work.
- `store_global_then_indirect_jump`: The routine copies a global variable to another global variable and then performs an indirect call through an unrecovered jump table.
- `copy_byte_via_pointers`: The routine stores a byte from a register through a pointer.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `check_global_constant`: The routine compares a global variable to a specific constant value.
- `initialize_pointer_chain`: The routine writes a sentinel value into a pointer and stores the address of the next entry into three fields in two different structures.
- `copy_nested_pointers`: The routine copies 32-bit values from pointers referenced at offsets four and eight of a source structure into fields of a destination structure.
- `compare_input_to_one`: The routine returns a negative short when the input value is greater than one and zero otherwise.
- `set_vhposr_bit`: The routine sets a specific bit in the vertical position register via a pointer offset.
- `calculate_table_offset`: The routine reads values from a global table and performs arithmetic operations including division, multiplication, and addition.
- `handle_disk_message`: The routine reads bytes from a structure and a global variable, conditionally writes a byte to a structure field, may perform indirect calls, and returns the byte from that field.
- `toggle_byte_flag`: The routine reads a byte from memory, toggles its high bit, writes the modified byte back, and returns a value indicating whether the original high bit was clear.
- `set_flag_from_bit`: The routine reads a bit from a global variable using an index derived from a byte in another memory location.
- `describe_empty_routine`: The routine contains three instructions and performs no observable work; it is effectively empty.
- `return_early_from_routine`: The routine performs a short sequence that ends by returning to the caller.
- `clear_bit_and_update_counter`: The routine tests a bit in a global variable derived from a byte in a structure, clears that bit in the global variable, updates a flag byte in the structure and increments a counter in the structure, then returns.
- `empty_routine_sub_03ce`: The routine contains no instructions that operate on data and returns immediately to its caller.
- `do_nothing_stub`: The routine executes five instructions but performs no observable work, ending without modifying memory, registers, or calling another routine.
- `empty_routine_18`: The routine contains no instructions that perform work and returns immediately to its caller.
- `clear_offset_and_save_register`: The routine stores zero to a word-sized location at an offset from a pointer in a register, then saves the value of another register into a global variable before returning.
- `dispatch_indirect_call`: The routine checks a global variable against a known value and then performs an indirect call through a pointer derived from a register.
- `dispatch_based_on_global`: The routine checks a register against a known value and performs an indirect call through a pointer derived from a register, with conditional early returns via other routines.
- `return_immediately_2`: The routine contains three instructions and performs no work other than returning to its caller.
- `clear_global_and_conditional_dispatch`: The routine clears a global variable to zero and then tests bytes from a table indexed by an input register.
- `clear_dword_via_pointer`: The routine stores zero to a dword located at an offset from a pointer passed in a register and then returns.
- `copy_dword_and_clear_word`: The routine clears a word at an offset from a register and copies a dword from an input pointer to a location derived from another register, then returns.
- `clear_word_via_register`: The routine stores zero to a word at an offset from a register and then returns.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Disk.1

- `unknown_routine`: The routine is empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_word_and_long_via_pointers`: The routine writes a word from a register to an offset within a structure pointed to by a register and writes a long from a register to an offset within another structure pointed to by a register, then returns.
- `call_library_stub`: The routine calls a library routine and then returns without further processing.
- `set_flag_via_library`: The routine calls another routine and then sets a byte pointed to by a register to the logical OR of its current value with 32.
- `decrement_pointer_byte`: The routine writes constants to two words of a structure pointed to by a register, decrements a byte pointed to by another register, updates two more words of the structure based on the decremented byte, and writes another constant to a word of the structure.
- `write_hardware_registers`: The routine stores bytes and words to a global variable and several memory locations that map to Amiga hardware registers for audio channels and DMA and interrupt control, using parameters and an uninitialized register as sources.
- `clear_audio_and_dma`: The routine writes zero to four audio volume hardware registers and a constant to a control register, writes a constant to a byte pointed to by a register, and returns.
- `call_indirect_routines_and_store`: The routine calls two indirect routines obtained from a global variable, stores a constant word through a pointer in A0, and returns.
- `negate_and_combine`: The routine calls another routine, conditionally negates a value from the call result based on a sign test, and returns a combined 32-bit value formed from that result and an input.
- `perform_32bit_division_2`: The routine computes the quotient and remainder of a 32-bit division using inputs in registers, employing a bit-shifting loop for large divisors and standard modulo arithmetic for smaller ones, and returns a concatenated 64-bit result.
- `compute_string_length_2`: The routine scans a null-terminated string pointed to by a parameter and returns the number of characters before the terminator.
- `indirect_call_via_table_2`: The routine performs an indirect call through a function pointer loaded from a global variable offset, with no other observable work.
- `call_indirect_table_entry`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at that address.
- `indirect_jump_via_table_2`: The routine loads a pointer from a global variable, adds a constant offset, and performs an indirect call to the resulting address.
- `call_indirect_via_table_9`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `indirect_call_via_global_table`: The routine loads a pointer from a global variable via an offset from a4 and calls another routine through an indirect jump.
- `initialize_pointer_array_2`: The routine writes the address of its pointer argument into the first element of a table, stores a value four bytes higher in the same element, clears the next element and then calls another routine indirectly via a global table.
- `initialize_structure_fields`: The routine conditionally calls another routine based on a field in the argument, writes constant values to two fields of the argument structure, calls another routine with a field value, and then calls an indirect routine with the argument.
- `call_indirect_via_table_10`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found at the resulting address.
- `call_indirect_via_global_table_3`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the resulting address as a function.
- `call_through_global_pointer`: The routine calls another routine through a pointer obtained from a global variable using an offset from a4, passing its parameter unchanged.
- `conditionally_clear_structure_fields`: If the argument is non-zero the routine writes a byte and two words to fields of the pointed-to structure and then calls an indirect routine with the argument and a word from the structure; otherwise it returns zero.
- `clear_structure_fields`: The routine writes constant values to three fields of a structure pointed to by a register and then calls an indirect routine.
- `call_indirect_via_global_table_4`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there indirectly.
- `call_indirect_from_global_offset`: The routine loads a pointer from a global variable offset by a constant, adds a constant to it, and calls the resulting address as a function.
- `call_indirect_from_table_3`: The routine loads a pointer from a global variable offset by a constant, adds a constant to it, and calls the resulting address as a function pointer.
- `call_indirect_via_table_11`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found there via an indirect jump.
- `call_indirect_via_global_table_5`: The routine loads a pointer from a global variable, adds an offset to it, and calls the resulting routine.
- `initialize_pointer_array`: The routine writes the address of its pointer argument into the first element, stores a value four bytes higher in the same element, clears the next element, and stores the original pointer address in the third element.
- `call_indirect_via_table_12`: The routine loads a function pointer from a table referenced through a global variable, adjusts it with a constant offset, and calls the resulting routine indirectly.
- `indirectly_call_via_table`: The routine loads a pointer from a global variable via an offset from a register and performs an indirect call through that pointer with an additional offset.
- `call_indirect_via_table_13`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the resulting address indirectly.
- `call_indirect_via_table_14`: The routine loads a pointer from a global variable, adds an offset, and calls the resulting address indirectly.
- `call_indirect_via_table_15`: The routine loads a pointer from a table indexed by a global variable and calls an indirect routine through that pointer.
- `indirect_jump_through_table`: The routine loads a pointer from a global variable, adds a constant offset to it, and then performs an indirect jump through a table to another routine.
- `unknown_indirect_jump`: The routine loads a pointer from a global variable, adds a constant offset to it, and then calls the routine pointed to by the resulting address.
- `call_indirect_via_table_16`: The routine reads a pointer from a global variable table, adds a fixed offset to it, and calls the resulting address indirectly.
- `call_indirect_via_global_table_6`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `invoke_indirect_through_table`: The routine loads a pointer from a global variable via an offset, adds a constant, and calls the resulting address indirectly.
- `call_indirect_through_pointer`: The routine loads a pointer from a global variable, adjusts it, and calls the routine it points to.
- `call_indirect_via_table_17`: The routine loads an address from a global variable and calls another routine through that address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `store_word_with_or_mask`: The routine tests a byte derived from an input word with a mask and, if the test passes, stores the masked word to a location derived from a pointer and copies a value from a register to another location.
- `store_word_from_byte_stream`: The routine checks a byte from a pointer and, if the check passes, builds a word from the high byte of an input word and that byte, stores it via a pointer, and advances the pointer.
- `update_pointer_and_byte`: The routine checks a signed byte through a pointer and, if it is non-negative, stores a 16-bit value built from the high byte of an input register and that byte into a location addressed by another pointer plus an offset.
- `store_word_with_mask`: The routine checks a condition on a word from a register and, if true, writes the word ORed with a constant to a location pointed to by a pointer plus an offset, and copies a value from a register to another location pointed to by a pointer plus an offset.
- `store_high_byte_and_advance_pointer`: The routine checks a signed byte pointed to by a pointer.
- `store_high_byte_and_advance_pointer_2`: The routine checks a signed byte through a pointer and, if it is non-negative, stores a 16-bit value formed from the high byte of an input register and that byte into a global variable, then advances a pointer stored in another global variable by one byte.
- `store_byte_from_register`: The routine checks a signed byte pointed to by a pointer and, if it is non-negative, writes a 16-bit value composed of the high byte of a register and that byte into a 16-bit field located at an offset from a pointer.
- `store_word_with_mask_2`: The routine masks a 16-bit value from a register with a constant and stores the result into a word field of a structure pointed to by a pointer in a register.
- `store_high_byte_and_advance_pointer_3`: The routine combines the high byte of a word from a register with a byte from a pointer, stores the resulting word into a global variable, and then advances a pointer stored in another global variable to the next byte.
- `conditionally_store_word_and_pointer`: The routine checks a byte derived from an input word and, if the check passes, stores the modified word through a pointer and stores a value from a register through another pointer.
- `store_word_with_masked_byte`: The routine masks a word from a register with a constant and stores the result to a word at an offset from a pointer.
- `store_word_with_mask_3`: The routine conditionally writes a masked word from a register to a location computed from a pointer and an offset, and stores a value from a register to another location computed from a different pointer and an offset.
- `store_byte_from_register_2`: The routine checks a signed byte pointed to by a pointer and, if it is non-negative, stores a 16-bit value built from the high byte of a register and that byte into a global variable, then updates another pointer to the next byte.
- `store_word_with_mask_4`: The routine masks a 16-bit value from a register with a constant and stores the result into a word field of a structure pointed to by a pointer in a register.
- `store_byte_from_register_3`: The routine checks a signed byte pointed to by a pointer and, if it is non-negative, stores a 16-bit value built from the high byte of a register and that byte into a global variable, then updates a pointer stored in another global variable to point to the next byte.
- `set_flagged_word_and_copy_pointer`: The routine checks a condition on a word argument and, if true, stores the word ORed with a constant into a word field of a structure pointed to by a pointer in a register, and copies a pointer value from a register into a dword field of another structure pointed to by a register.
- `configure_audio_hardware`: The routine writes constants to bytes in a global variable and to several memory locations that map to Amiga audio hardware registers, using parameters for some of those writes, and then returns.
- `reset_audio_channels`: The routine writes four parameters to a global variable and several memory locations that map to Amiga audio hardware registers, clears four audio channel registers, copies four words from a table pointed to by a global variable into audio-related memory, sets four bytes to 255, and clears a set of audio-related words and bytes, then writes a constant to an audio control register.
- `clear_audio_and_dma_registers`: The routine writes zero to four audio volume hardware registers and a constant to a control register and writes a constant byte through a pointer derived from A4.
- `set_audio_and_dma`: The routine writes parameters to a few RAM locations, clears audio and DMA registers, writes constants to a table pointed to by a register and to a second table, then stores the result of a call to another routine and updates a global variable and a RAM byte.
- `unreached_stub`: The routine is marked as unreached and contains no observable work in the decompiled view; it is effectively empty.
- `multiply_32bit_words`: The routine computes a 32-bit multiplication of the low words of two inputs combined with high-word contributions and returns a 64-bit concatenation of the product and the second input.
- `conditional_negate_and_concat`: The routine calls another routine, conditionally negates its result based on the signs of two inputs, and returns a concatenation of the possibly negated result with the second input.
- `conditionally_negate_and_combine`: The routine calls another routine, conditionally negates a value from the call result based on the sign of an input, and returns a combined 32-bit value from the possibly negated value and an input.
- `indirect_call_via_table_3`: The routine performs an indirect call through a function pointer loaded from a global variable with a constant offset and does no other observable work.
- `indirect_call_via_table_2_2`: The routine performs an indirect call through a function pointer loaded from a global variable with a different constant offset and does no other observable work.
- `call_indirect_table_entry_2`: The routine loads a pointer from a global variable, adds a constant offset, and calls the routine found at that address with no other observable work.
- `call_indirect_via_table_9_2`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly with no other observable work.
- `indirect_call_via_global_table_2`: The routine loads a pointer from a global variable via an offset from a4 and calls another routine through an indirect jump with no other observable work.
- `indirect_call_via_global_table_3`: The routine loads a pointer from a global variable via an offset from a4 and calls another routine through an indirect jump.
- `call_indirect_via_table_21`: The routine calls an indirect routine obtained by adding a constant to a value read from a global variable.
- `allocate_and_initialize_structure`: The routine obtains an index via a call, allocates a structure via an indirect call, fills several fields of the structure with the parameters and values from other calls, and then calls another routine with the structure or a field of it.
- `conditionally_initialize_structure_fields`: If a field of the argument is non-zero the routine calls another routine with the argument, writes a byte and two words to fields of the argument, calls another routine with a byte from the argument, and then calls an indirect routine with the argument and a constant.
- `call_indirect_via_global_table_7`: The routine loads a function pointer from a global variable using an offset from a register, adds a constant offset to it, and calls the resulting address with a parameter and a constant.
- `call_through_global_pointer_2`: The routine obtains a function pointer from a global variable using an offset from a register and calls it with its parameter unchanged.
- `conditionally_call_and_write_struct`: If the first parameter is zero the routine returns zero.
- `clear_structure_fields_3`: The routine clears a byte field and two word fields in the structure pointed to by its argument and then calls another routine indirectly through a global variable.
- `unknown_indirect_call`: The routine loads an address from a global variable, adjusts it, and calls the routine found there.
- `clear_structure_fields_2`: The routine clears three fields of a structure pointed to by a register by writing constant values to them and then calls an indirect routine via a table.
- `call_indirect_via_table_22`: The routine performs an indirect call through a table pointer derived from a register, adding an offset to the table entry before invoking the target routine.
- `call_indirect_from_global_offset_2`: The routine loads a pointer from a global variable offset by a constant, adds a constant to it, and calls the resulting address as a function.
- `indirect_call_via_global_table_4`: The routine loads a pointer from a global variable using an offset from a register, adds a constant offset to it, and performs an indirect call to the resulting address with no arguments.
- `indirect_call_via_global_table_5`: The routine loads a pointer from a global variable using an offset from a register, adds a different constant offset to it, and performs an indirect call to the resulting address with no arguments.
- `call_indirect_via_table_12_2`: The routine loads a function pointer from a table referenced through a global variable, adjusts it with a constant offset, and calls the resolved function.
- `call_via_table_pointer`: The routine performs an indirect call through a pointer obtained from a global variable via an offset from a register.
- `call_indirect_via_global_offset`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the resulting address indirectly.
- `call_indirect_via_table_13_2`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the resulting address indirectly.
- `call_indirect_via_table_14_2`: The routine loads a pointer from a global variable, adds an offset, and calls the resulting address indirectly.
- `call_indirect_via_table_18`: The routine loads a pointer from a global variable, adds a constant offset, and calls another routine indirectly through the resulting address.
- `call_indirect_via_table_19`: The routine loads a pointer from a table indexed by a global variable, adds a constant offset, and calls another routine indirectly through the resulting address.
- `call_indirect_via_table_15_2`: The routine loads a pointer from a table indexed by a global variable and calls an indirect routine through that pointer.
- `call_indirect_via_table_15_3`: The routine loads a pointer from a table using a global variable as index, computes an adjusted address, and calls an indirect routine through that pointer.
- `jump_indirect_through_table`: The routine loads a pointer from a global variable, adds a constant offset, and performs an indirect jump through a table to another routine.
- `indirect_jump_through_table_2`: The routine loads a pointer from a global variable, adds a constant offset to it, and then performs an indirect jump through a table to another routine.
- `call_indirect_via_global_ptr`: The routine loads a pointer from a global variable, adds a constant offset to it, and then calls the routine pointed to by the resulting address.
- `call_indirect_from_global`: The routine loads a pointer from a global variable, adds a constant offset to it, and then calls the routine pointed to the resulting address.
- `call_via_global_pointer`: The routine loads a pointer from a global variable, adds a constant offset to it, and then calls the routine pointed to by the resulting address.
- `call_indirect_from_global_table`: The routine loads a pointer from a global variable, adds a constant offset to it and calls the routine pointed to by the resulting address.
- `call_indirect_via_global`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine pointed to by the resulting address indirectly.
- `call_indirect_via_table_20`: The routine reads a pointer from a global variable table, adds a fixed offset, and calls the resulting address indirectly.
- `call_indirect_via_table_16_2`: The routine loads a pointer from a global variable table, adds a fixed offset to it, and calls the resulting address indirectly.
- `call_indirect_via_global_2`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `call_indirect_via_global_table_6_2`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `execute_indirect_from_global`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `indirect_call_via_global_table_6`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `call_indirect_table_entry_3`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `call_indirect_via_global_3`: The routine loads a pointer from a global variable, adds a constant offset, and calls the resulting address indirectly.
- `invoke_indirect_through_table_2`: The routine loads a pointer from a global variable via an offset, adds a constant, and calls the resulting address indirectly.
- `invoke_indirect_through_table_3`: The routine loads a pointer from a global variable via an offset, adds a constant, and calls the resulting address indirectly.
- `call_indirect_via_table_17_2`: The routine loads an address from a global variable and calls another routine through that address.
- `call_indirect_via_global_table_8`: The routine loads an address from a global variable, adds an offset to it, and calls another routine through that address via an indirect call.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `infinite_loop`: The routine contains an empty infinite loop and performs no other operations.
- `infinite_loop_routine`: The routine contains no instructions and consists of an empty infinite loop.
- `halt_on_bad_data`: The routine contains no valid instructions; the disassembly ends with a bad instruction marker.
- `handle_bad_instruction`: The routine contains no valid instructions and immediately encounters bad instruction data, so it performs no observable work.
- `spin_forever`: The routine executes a do-while true loop with no body.
- `infinite_loop_stub`: The routine contains an empty loop that never terminates.
- `infinite_loop_stub_2`: The routine contains an empty loop that never terminates.
- `other_routine`: The routine contains an infinite loop with no operations.
- `infinite_loop_stub_3`: The routine contains an unconditional infinite loop and performs no other operations.
- `empty_routine_20`: The routine contains no instructions and returns immediately.
- `decode_gap_as_code`: The routine contains no valid instructions and therefore performs no operations.
- `infinite_loop_stub_4`: The routine contains no executable operations and enters an unconditional infinite loop, so it performs no observable work and never returns.
- `infinite_loop_stub_5`: The routine contains an unconditional infinite loop and performs no other operations.
- `other_routine_2`: The routine contains an infinite loop with no operations; it never returns.
- `unreachable_code_fragment`: The routine contains no valid instructions and the disassembly ends immediately with a bad instruction marker.
- `handle_bad_instruction_2`: The routine contains no valid instructions and ends immediately with a bad instruction, so it performs no observable operations.
- `decode_gap_as_code_2`: The routine contains no valid instructions and the disassembly is truncated.
- `handle_bad_instruction_3`: The routine contains no valid instructions and is truncated by the decompiler.
- `spin_forever_2`: The routine executes a do-while loop that never exits.
- `empty_routine_21`: The routine contains no instructions and returns immediately.
- `empty_routine_22`: The routine contains no instructions and performs no operations.
- `empty_return_routine_2`: The routine performs no operations and returns immediately to its caller.
- `infinite_wait_loop`: The routine contains no instructions that modify memory or registers and consists of an unconditional infinite loop.
- `infinite_loop_2`: The routine contains a do-while loop that always evaluates to true, so execution never leaves the loop.
- `handle_bad_data`: The routine contains no valid instructions and the disassembler stops decoding, so it performs no observable operations on registers or memory.
- `empty_routine_23`: The routine contains no instructions and performs no operations.
- `infinite_loop_stub_6`: The routine contains an unconditional infinite loop with no operations.
- `infinite_loop_routine_2`: The routine contains an unconditional infinite loop with no operations.
- `decode_gap_as_code_3`: The routine contains no valid instructions and is truncated by the disassembler.
- `spin_forever_3`: The routine executes a do-while loop that always tests true, so it spins indefinitely without touching memory or registers.
- `handle_bad_instruction_4`: The routine contains no valid instructions; the disassembler stops at a bad instruction and the control flow is truncated.
- `infinite_loop_stub_7`: The routine contains an empty loop that never exits.
- `infinite_loop_stub_8`: The routine contains no executable instructions and immediately enters an infinite loop with no operations on data.
- `infinite_loop_stub_9`: The routine contains no executable operations and immediately enters an infinite loop, so it performs no observable work and never returns.
- `infinite_loop_routine_3`: The routine contains an infinite loop that never exits and performs no operations.
- `spin_do_nothing_loop`: The routine contains an infinite loop with no operations.
- `infinite_loop_routine_4`: The routine contains an infinite loop with no operations.
- `infinite_loop_stub_10`: The routine contains no executable instructions and consists of an infinite loop that never exits.
- `halt_on_bad_data_2`: The routine contains bad instruction data and is truncated by the disassembler.
- `infinite_loop_unreached`: The routine decodes as an infinite do-nothing loop.
- `infinite_loop_executed`: The routine decodes as an infinite do-nothing loop.
- `infinite_loop_stub_11`: The routine contains no executable operations and enters an unconditional infinite loop, performing no work and never returning.
- `infinite_loop_stub_12`: The routine contains an empty loop that never exits.
- `empty_routine_4`: The routine contains no usable instructions in the recovered disassembly.
- `infinite_loop_stub_13`: The routine contains no instructions and immediately enters an infinite loop, performing no operations on data or registers.
- `halt_on_bad_data_unreached`: The routine contains bad instruction data and is truncated by the disassembler.
- `empty_routine_5`: The routine contains no usable instructions and ends immediately.
- `infinite_loop_stub_14`: The routine contains an unconditional infinite loop with no operations, so it never returns and performs no observable work on memory or registers.
- `spin_forever_4`: The routine contains a single instruction that forms an infinite loop with no operations.
- `infinite_loop_routine_5`: The routine contains only an infinite loop with no operations.
- `infinite_loop_3`: The routine contains no instructions that modify memory or registers; it executes an endless loop with no side effects.
- `infinite_loop_routine_6`: The routine contains only an infinite loop with no operations.
- `halt_on_bad_data_3`: The routine decodes to bad instruction data and the disassembler truncates control flow, indicating no valid 68000 operation is executed.
- `infinite_loop_stub_15`: The routine contains no instructions that modify memory or registers and consists of an unconditional infinite loop.
- `other_routine_3`: The routine contains an infinite loop with no operations, so it never returns.
- `spin_forever_5`: The routine contains only an infinite loop with no operations.
- `empty_routine_24`: The routine contains no instructions and performs no operations.
- `infinite_loop_stub_16`: The routine contains an empty infinite loop with no operations.
- `infinite_loop_stub_17`: The routine contains an empty infinite loop with no operations and no exit.
- `handle_bad_instruction_5`: The routine contains no recoverable instructions; control flow ends immediately due to bad instruction data.
- `infinite_loop_4`: The routine consists of a single instruction that forms an infinite loop.
- `infinite_loop_stub_18`: The routine contains no instructions that modify memory or registers and consists of an unconditional infinite loop.
- `handle_bad_data_2`: The routine contains no valid instructions and the decompiler stops the control flow because the instruction stream is invalid.
- `unknown_routine_2`: The routine contains no valid instructions and is empty.
- `infinite_loop_routine_7`: The routine contains an empty infinite loop with no operations.
- `decode_gap_as_code_4`: The routine contains no valid instructions and the disassembly ends immediately.
- `infinite_loop_stub_19`: The routine contains an unconditional infinite loop with no operations.
- `infinite_loop_stub_20`: The routine contains no operations and then enters an unconditional infinite loop.
- `handle_bad_data_3`: The routine contains no valid instructions and is effectively empty.
- `spin_forever_6`: The routine contains a single instruction that forms an infinite loop with no operation.
- `other_routine_4`: The routine contains an unconditional infinite loop and performs no other work.
- `infinite_loop_stub_21`: The routine contains an unconditional infinite loop with no operations.
- `halt_on_bad_data_4`: The routine is decoded as containing bad instruction data, so control flow is truncated.
- `infinite_loop_stub_22`: The routine contains an empty infinite loop with no operations and no exit condition.
- `infinite_loop_5`: The routine contains an empty loop that repeats forever and performs no operations.
- `empty_routine_6`: The routine contains no executable instructions in the recovered disassembly; the decompiler reports bad instruction data and truncates control flow.
- `infinite_loop_stub_23`: The routine contains no executable operations and consists of an unconditional infinite loop.
- `truncate_on_bad_data`: The routine contains no valid instructions and the disassembly ends with a bad instruction marker, so control flow is truncated.
- `other_routine_5`: The routine contains no executable instructions and performs no operations.
- `infinite_loop_stub_24`: The routine contains no executable instructions and immediately enters an endless loop, performing no operations on data or registers.
- `handle_bad_instruction_6`: The routine contains no valid instructions and the decompiler truncates control flow because of bad instruction data.
- `infinite_loop_stub_25`: The routine contains no instructions and consists of an empty infinite loop.
- `handle_bad_data_4`: The routine contains no valid instructions and the disassembler stops at a bad instruction.
- `infinite_loop_stub_26`: The routine contains no instructions and consists of an empty infinite loop.
- `handle_bad_data_5`: The routine contains no valid instructions and the decompiler truncates control flow.
- `decode_gap_as_code_5`: The routine contains no valid instructions and is empty.
- `infinite_loop_routine_8`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_routine_9`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_6`: The routine contains no instructions and consists of an empty infinite loop.
- `decode_unreachable_bytes`: The routine consists of a single instruction that the disassembler cannot decode, so control flow is truncated immediately.
- `loop_forever`: The routine executes an empty infinite loop with no operations inside the loop body.
- `unreachable_truncated_routine`: The routine is unreachable and contains no valid instructions.
- `infinite_loop_stub_27`: The routine contains no instructions and therefore performs no operation.
- `infinite_loop_routine_10`: The routine contains no instructions and consists of an empty loop that never terminates.
- `infinite_loop_stub_28`: The routine contains no instructions and immediately enters an infinite loop, performing no operations on data.
- `unreached_gap_truncated`: The routine is a single instruction that Ghidra cannot decode, so control flow is truncated and the routine does nothing observable.
- `handle_invalid_data`: The routine contains no valid instructions and the decompiler stops with a bad instruction warning.
- `infinite_loop_stub_29`: The routine contains no instructions and consists of an empty loop that never exits.
- `empty_routine_7`: The routine contains no usable instructions; the disassembly ends immediately with a bad instruction marker, so it performs no observable operations on registers or memory.
- `infinite_loop_7`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_8`: The routine is a one-instruction endless loop that does not modify registers or memory.
- `infinite_loop_9`: The routine is an unreached one-instruction endless loop with no side effects.
- `empty_routine_8`: The routine contains no usable instructions and ends immediately.
- `halt_on_bad_data_5`: The routine is an unreached location with a single bad instruction.
- `infinite_loop_10`: The routine is a one-instruction endless loop with no operations.
- `infinite_loop_11`: The routine is an unreached one-instruction endless loop.
- `infinite_loop_12`: The routine consists of a single instruction that loops forever with no effect.
- `halt_on_bad_data_6`: The routine is an unreached location with a single bad instruction that truncates control flow.
- `infinite_loop_13`: The routine is a one-instruction endless loop that does nothing.
- `infinite_loop_unreached_2`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_executed_2`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_stub_30`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_31`: The routine contains only an unconditional branch that repeats forever, performing no operations on memory or registers.
- `spin_forever_loop`: The routine contains an unconditional infinite loop with an empty body.
- `infinite_loop_14`: The routine contains an empty loop that repeats forever.
- `infinite_loop_stub_32`: The routine contains no instructions and consists of an empty infinite loop.
- `bad_instruction_executed`: The routine is decoded as containing bad instruction data and the control flow is truncated.
- `handle_baddata`: The routine contains no valid instructions and ends immediately.
- `infinite_loop_stub_33`: The routine contains no instructions and consists only of an infinite loop that never exits.
- `infinite_loop_15`: The routine contains no instructions that change state and loops forever.
- `infinite_loop_16`: The routine contains no instructions that change state and loops forever.
- `infinite_loop_stub_34`: The routine contains no instructions and consists of an empty infinite loop.
- `handle_bad_data_6`: The routine contains no valid instructions and ends immediately.
- `decode_unreachable_gap`: The routine is a single unrecognized instruction that causes the disassembler to stop decoding.
- `infinite_loop_routine_11`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_35`: The routine contains no instructions and is effectively empty.
- `handle_bad_instruction_data`: The routine contains no valid instructions and the decompiler truncates control flow because of bad instruction data.
- `infinite_loop_stub_36`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_stub_37`: The routine contains no instructions that modify memory or registers and consists of an unconditional infinite loop.
- `infinite_loop_17`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_routine_12`: The routine contains no instructions and therefore performs no operation; it is an empty routine that would loop forever if entered.
- `infinite_loop_stub_38`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_18`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_stub_39`: The routine contains no instructions; the disassembly shows only an empty loop that never terminates.
- `infinite_loop_stub_40`: The routine contains no instructions and is never reached.
- `other_routine_6`: The routine contains no instructions and therefore performs no operation.
- `bad_instruction_halt`: The routine contains data that is not a valid instruction, causing the disassembler to report a bad instruction and truncate control flow.
- `infinite_loop_stub_41`: The routine contains no instructions and is effectively empty.
- `infinite_loop_stub_42`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_19`: The routine contains a single instruction that forms an endless loop with no operations on registers, memory, or library calls.
- `infinite_loop_routine_13`: The routine contains no instructions and consists of an empty infinite loop.
- `spin_forever_7`: The routine contains an empty infinite loop and performs no operations on data, registers, or memory.
- `infinite_loop_routine_14`: The routine contains an empty loop that repeats forever.
- `infinite_empty_loop`: The routine contains no executable statements; it is an empty loop that never exits, so it performs no observable operation on memory or registers.
- `infinite_loop_stub_43`: The routine contains no instructions and immediately enters an infinite loop.
- `infinite_loop_stub_44`: The routine contains no instructions and immediately enters an endless loop, performing no operations on data or registers.
- `infinite_loop_stub_45`: The routine contains no instructions and immediately enters an infinite loop.
- `infinite_loop_stub_46`: The routine contains no instructions that modify data or call other code; it is a single unconditional branch that repeats forever, so execution never progresses beyond the entry point.
- `halt_on_bad_data_7`: The routine is identified as containing bad instruction data and the disassembler truncates control flow, indicating no further processing.
- `empty_routine_9`: This routine contains no usable instructions.
- `decode_unreachable_gap_2`: The routine contains no valid instructions and ends immediately.
- `spin_forever_8`: The routine executes an infinite loop with an empty body.
- `truncate_control_flow`: The routine contains no valid instructions and the decompiler inserts a control-flow truncation.
- `infinite_loop_empty`: The routine contains no instructions and consists of an infinite loop.
- `infinite_loop_routine_15`: The routine contains no instructions and therefore performs no operations.
- `infinite_loop_routine_16`: The routine contains no instructions that modify memory or registers and consists of a single unconditional branch to itself, forming an endless loop.
- `halt_on_bad_data_8`: The routine contains no readable instructions.
- `bad_data_handler`: The routine contains no valid instructions and immediately stops execution.
- `infinite_loop_stub_47`: The routine contains no instructions and consists of an empty infinite loop.
- `infinite_loop_20`: The routine contains no instructions and forms an infinite loop.
- `infinite_loop_stub_48`: The routine contains no instructions and immediately enters an infinite loop.
- `infinite_loop_21`: The routine contains no instructions and forms an infinite loop.
- `infinite_loop_stub_49`: The routine contains no executable instructions and immediately enters an infinite loop, performing no operations on data or registers.
- `infinite_loop_22`: The routine contains an empty loop that repeats forever.
- `other_routine_7`: The routine contains a single instruction that creates an infinite loop, so it never returns.
- `run_infinite_loop`: The routine contains no instructions and therefore performs no operation.
- `bad_instruction_halt_2`: The routine contains bad instruction data and the control flow is truncated.
- `bad_instruction_halt_3`: The routine contains bad instruction data and the control flow is truncated.
- `handle_bad_instruction_data_2`: The routine contains no valid instructions and the decompiler stops the control flow because of bad instruction data.
- `infinite_loop_23`: The routine contains no instructions and forms an endless loop.
- `infinite_loop_routine_17`: The routine contains an empty loop that never exits.
- `infinite_loop_24`: The routine contains no instructions and forms an endless loop.
- `halt_on_bad_data_9`: The routine is decoded as bad instruction data and control flow is truncated.
- `halt_baddata`: A routine that immediately stops execution due to an unrecognized instruction.
- `halt_on_bad_data_10`: The routine is decoded as bad instruction data and control flow is truncated.
- `empty_routine_10`: The routine contains no usable instructions; the disassembler reports bad instruction data and the control flow is truncated immediately, so the routine performs no observable operations.
- `infinite_loop_stub_50`: The routine contains only an unconditional branch that loops forever.
- `handle_bad_instruction_7`: The routine contains no valid instructions and ends immediately.
- `empty_routine_11`: The routine contains no usable instructions and ends immediately because the control flow is truncated by bad instruction data.
- `infinite_loop_unreached_3`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_25`: The routine contains no instructions and immediately enters an infinite loop.
- `infinite_loop_26`: The routine contains an empty loop that never exits.
- `infinite_loop_27`: The routine contains no instructions and consists of an empty infinite loop.
- `bad_instruction_executed_2`: The routine consists of a single bad instruction that causes control flow to be truncated.
- `infinite_loop_stub_51`: The routine contains a single unconditional branch that makes it loop forever.
- `run_forever_loop`: The routine contains a single unconditional loop that never exits.
- `spin_forever_9`: The routine contains no instructions and therefore performs no operation.
- `infinite_loop_stub_52`: The routine contains no instructions and is effectively empty.
- `infinite_loop_routine_18`: The routine contains an empty loop that never exits, performing no operations on data or registers.
- `infinite_loop_unreached_4`: The routine contains a single instruction that forms an endless loop with no operations.
- `infinite_loop_stub_53`: The routine contains an empty loop that repeats forever, performing no operations on data or registers.
- `bad_instruction_executed_3`: The routine is decoded as containing bad instruction data and the control flow is truncated.
- `bad_instruction_unreached`: The routine is decoded as containing bad instruction data and the control flow is truncated.
- `handle_bad_data_7`: The routine contains no valid instructions and is truncated by the disassembler because the byte stream is not decodable.
- `empty_routine_12`: The routine contains no valid instructions and immediately terminates with a bad instruction.
- `infinite_loop_stub_54`: The routine contains no instructions and consists of an empty infinite loop.
- `decode_unreachable_bytes_2`: The routine consists of a single bad instruction at the entry point, so the disassembly stops immediately and no further operations are performed.
- `infinite_loop_routine_19`: The routine contains an empty loop that never terminates.
- `infinite_loop_stub_55`: The routine contains no instructions that modify memory or registers and consists of an endless loop that never exits.
- `infinite_loop_28`: The routine contains no instructions that change state and loops forever.
- `decrement_word_at_offset_2`: The routine calls another routine and returns a zero value to its caller.
- `decrement_word_at_offset_3`: The routine calls another routine and returns the value produced by that call.
- `decrement_word_at_offset_4`: The routine calls another routine and returns the value left in data register D1.
- `decrement_word_at_offset_5`: The routine calls another routine and returns the value produced by that call.
- `decrement_word_at_offset_6`: The routine calls another routine and returns the value produced by that call.
- `decrement_word_at_offset_7`: The routine calls another routine and returns the value it receives in a data register.
- `decrement_word_and_return_zero`: The routine calls another routine that decrements a 16-bit value at a fixed positive offset from a base pointer register and then returns a zero value in D1.
- `decrement_word_at_offset_8`: The routine calls another routine and returns the value produced by that call in D1.
- `decrement_word_at_offset_9`: The routine calls another routine and returns the value produced by that call in a data register.
- `call_other_routine_and_return`: The routine calls another routine and returns the value produced by that call.
- `decrement_word_at_offset_10`: The routine calls another routine and returns the value produced by that call in D1.
- `return_zero_constant`: The routine returns a constant zero without performing any operations or modifying data.
- `increment_word_at_offset`: The routine increments a 16-bit value located at a fixed positive offset from a base pointer register in place and then returns.
- `decrement_word_at_offset`: The routine decrements a 16-bit value located at a fixed positive offset from a base pointer register and then returns a zero value.
- `infinite_loop_stub_56`: The routine contains no executable code and immediately enters an infinite loop that never returns.
- `unreachable_code_block`: The routine contains no valid instructions and is never reached.
- `decode_and_truncate`: The routine contains no valid instructions and is truncated by the disassembler, so it performs no observable operations.
- `infinite_wait_loop_2`: The routine contains no operations and enters an unconditional infinite loop, performing no work and never returning.
- `loop_indefinitely`: The routine contains an infinite loop with no operations and calls a routine that performs no operations other than returning to its caller.
- `empty_return_routine`: The decompiled pseudo-code shows it performs no operations other than returning to the caller.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a5, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `nop_instruction`: The routine executes a single no-operation instruction and does nothing else.
- `return_without_effect`: The routine contains no instructions and therefore performs no operation.
- `check_routine_entry`: The routine contains no instructions in the provided disassembly and performs no operation on its data.
