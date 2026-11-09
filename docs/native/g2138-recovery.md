# Game 2138 (alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02138). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (967280 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 31 of 31 units identical (bytes and relocations), 95 raw-word lines |
| Functions named and described | pass, 316 of 316 named (0 author symbols), 316 of 316 described (31 by rule, 276 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3557 of 3559 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6122 of 7192 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 14844 |
| code | 30208 |
| data | 862248 |
| padding | 76 |
| unused | 59904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf:bootblock | 1 | 17 | 1 |
| logged disk load from Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf boot-block load | 1 | 438 | 5 |
| logged disk load from Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 30 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1859 | 77 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1575 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 230 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1079 | 79 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1642 | 66 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |

## Cases handled

- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1640 non-empty blocks: matched against track-loaded segments
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: the boot block reads 10240 bytes from disk offset 63488 to an address (found statically); recovered as a unit
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: the boot block reads 1024 bytes from disk offset 73728 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500-unpk00): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1262 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 26 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 226 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 185 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 512 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1211 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (12 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (31 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (32 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (63 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (16 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (47 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (30 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (14 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: runtime segment an address (play-rs-m1500-unpk00) +2, runtime segment an address (play-rs-m1500-unpk00) +3962, runtime segment an address (play-rs-m1500-unpk00) +882
- self-modifying code at an address (39 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: runtime segment an address (play-rs-m1500-unpk00) +2, runtime segment an address (play-rs-m1500-unpk00) +8, runtime segment an address (play-rs-m1500-unpk00) +E, runtime segment an address (play-rs-m1500-unpk00) +14
- self-modifying code at an address (26 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: runtime segment an address (play-rs-m1500-unpk00) +14, runtime segment an address (play-rs-m1500-unpk00) +26, runtime segment an address (play-rs-m1500-unpk00) +14, runtime segment an address (play-rs-m1500-unpk00) +26
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (45 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (40 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (55 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: 2 logged loads cover 11264 bytes of the image
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 123-123 (bytes 62976-63488) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.92)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 153-163 (bytes 78336-83968) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.74)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 182-184 (bytes 93184-94720) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 186-188 (bytes 95232-96768) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 190-197 (bytes 97280-101376) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 200-201 (bytes 102400-103424) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 204-206 (bytes 104448-105984) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 208-210 (bytes 106496-108032) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 212-219 (bytes 108544-112640) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 222-223 (bytes 113664-114688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 226-228 (bytes 115712-117248) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 230-232 (bytes 117760-119296) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 234-241 (bytes 119808-123904) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 244-245 (bytes 124928-125952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 248-250 (bytes 126976-128512) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 252-254 (bytes 129024-130560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 256-263 (bytes 131072-135168) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 266-267 (bytes 136192-137216) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 270-272 (bytes 138240-139776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 274-276 (bytes 140288-141824) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 278-285 (bytes 142336-146432) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 288-289 (bytes 147456-148480) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 292-294 (bytes 149504-151040) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 296-298 (bytes 151552-153088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 300-307 (bytes 153600-157696) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 310-311 (bytes 158720-159744) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 314-316 (bytes 160768-162304) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 318-320 (bytes 162816-164352) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 322-329 (bytes 164864-168960) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 332-333 (bytes 169984-171008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 336-338 (bytes 172032-173568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 340-342 (bytes 174080-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 344-351 (bytes 176128-180224) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 354-355 (bytes 181248-182272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 358-360 (bytes 183296-184832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 362-364 (bytes 185344-186880) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 366-373 (bytes 187392-191488) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 376-377 (bytes 192512-193536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 380-382 (bytes 194560-196096) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 384-386 (bytes 196608-198144) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 1550-1552 (bytes 793600-795136) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 1554-1561 (bytes 795648-799744) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 1564-1565 (bytes 800768-801792) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 1568-1570 (bytes 802816-804352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)[cr skr][t +4 goonies](disk 1 of 3)/Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf: blocks 1572-1574 (bytes 804864-806400) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- 5 packed track region(s), 735744 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_cia_bit` (runtime segment an address): Reads a byte from an offset of 7424 from the address in a5, masks it to keep only the two highest bits, sets bit 3, and writes the result back to the same location. Kind: disk hardware/sync check. Pass path: the branch following the bit manipulation (e.g., bra.b an address in sub_0952).

## Functions


### Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf:bootblock

- `delay_beam_then_clear_memory`: Runs a timed loop that waits on the video beam position between passes, after an exec device request through a library vector.

### logged disk load from Alien Breed (1991)(Team 17)[cr SKR][t +4 Goonies](Disk 1 of 3).adf boot-block load

- `copy_block_to_low_memory`: Copies 108 bytes from a fixed source area into low memory starting at byte offset 326, one byte per loop pass.
- `shift_inverted_flag_or_clear`: Saves a flag byte, clears the global and masks one bit from another value.
- `interleave_word_pairs_into_buffer`: Writes 48 word pairs into a fixed destination buffer, each pair holding a value from a register and the next word read from a source pointer.
- `draw_text_glyphs_to_bitplane`: Walks a zero-terminated string and copies the eight-row glyph of each character from a font table into a 40-byte-per-row bitplane.
- `stub_bad_instruction_data`: Only a stub: decoding reached bad instruction data, so the routine is truncated and no behaviour can be recovered from it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_parameter_registers`: Tests a longword in a global variable and branches to another routine if non-zero; otherwise, it loads seven specific constant values into registers d0 through d7 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_loop_and_clear_memory`: Calls another routine in a loop until it returns zero, then clears sixteen bytes of memory starting at a specific address using a loop, calls another routine, and sets a global variable to a constant value.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `check_ciaa_status_bit`: Saves registers to the stack, tests bit 7 of the CIAA register, and if set, loads zero into d0; otherwise, it loads 255 into d0 and branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_and_call_subroutines`: Stores the contents of registers d2, d3, d4, d5, d6, d7, and a0 into a series of global variables, then calls five different subroutines in sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_list_2`: Points the copper at a new copper list by writing its address to COP1LC, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_registers_and_call_subroutines_2`: Stores the contents of registers d3, d4, d5, d6, d7, and a0 into a series of global memory locations and then calls five consecutive subroutines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_and_return`: Restores d0 through d7 and a0 through a6 from the stack and returns.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `test_joystick_fire_bit`: Tests bit 7 of the CIAA Port A register and sets register d0 to zero if the bit is set, otherwise it sets register d0 to 255 and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_player_status`: Reads a byte from a memory location pointed to by a register, performs a bitwise AND with a constant, and stores the result into a different memory location pointed to by another register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_three_routines`: Calls three different routines in sequence and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_value_and_call_subroutine`: Writes the value 16384 to an offset of 36 from a6, calls another routine, restores registers from the stack, and returns.
- `modify_byte_via_a5`: Stores a byte into the address in a5, clears bit 3 of that byte, rotates d0 right by 8 bits, sets bit 3 of that byte, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_registers_and_call`: Loads several values from a global table into registers d1 through d7, clears d4, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_subroutine_and_restore`: Calls another routine, restores registers from the stack, and returns.
- `modify_copper_control_bit`: Clears bit 3 of a byte at the address held in a5, rotates the value in d0 by 8 bits, and then sets bit 3 of the same byte at the address held in a5 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_subset_of_registers_and_call`: Loads values from a global table into registers d3 through d7, clears d4, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bit_three_at_a5`: Clears bit 3 of a byte at the address held in a5, rotates the value in d0 by 8 bits, and then sets bit 3 of that same byte before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_ciaa_bits`: Returns immediately, otherwise it checks bit 7 and bit 6 of the CIAA register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_set_bit`: Loops until bit 0 of a byte at an offset of a5 is set, then sets bit 0 of that byte and returns.
- `decrement_d0_loop`: Decrements d0 from an address down to 0 in a loop and returns.
- `update_status_and_call`: Checks a word in a global variable, sets it to 1 if it is 0, then loads a pointer from a global variable into a0, loads a pointer into a1, calls another routine, and loads a constant into d0 and 4 into d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_pointer_and_call`: Loads a pointer from a global variable into a0, calls another routine, checks the sign of d0, and if not negative, sets a2 to a0, loads a new address into a0, calls another routine, and clears a global variable before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_pointer`: If a condition is not met, it writes 1 to a global variable, loads a pointer from another global variable into a0, loads a pointer from a third global variable into a1, calls another routine, then writes a 32-bit value from d0 into a structure at offset 6 and offset 2 of the structure pointed to by a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_clear_global`: Calls another routine, checks if d0 is negative, if not it stores a0 in a2, loads a pointer into a0, calls another routine, and clears a long value in a global variable before returning.
- `initialize_and_process_list`: Sets a long global variable to 2, calls another routine, loads d0 from a global variable and a0 from another, calls another routine, checks if d0 is negative, if not it stores a0 in a2, loads a pointer into a0, calls another routine, and clears a long global variable before returning.
- `shift_list_elements`: Loops up to 10 times to check if a value at a0 is less than d0; if so, it returns -1.
- `perform_zero_or`: Performs a bitwise OR on d0 with 0.
- `increment_and_reset_counters`: Performs a bitwise OR on d0 with 0, then increments and potentially resets two different word counters in global variables based on comparison with 24 and 7 respectively, and sets a third global variable to 1.
- `setup_and_call_subroutine`: Writes 1 to a global variable, stores a2 in another global variable, pushes a0 onto the stack, calls another routine, restores a0 from the stack, loads a pointer into a1, calls another routine, and finally loads two pointers into a0 and a1 and sets d0 to 8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_vblank`: Loops while the value at the VHPOSR register is not 255 or 0, decrementing d0 each time, and returns when d0 reaches 0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_copper_parameter_tables`: Reads data from a pointer stored in a global variable and copies it into four separate destination tables using four consecutive loops.
- `zero_data_register_d0`: Performs a logical OR operation on register d0 with zero.
- `repeat_zero_or`: Performs a bitwise OR on d0 with 0 nineteen times.
- `clear_data_register_d0_7`: Performs a logical OR operation on register d0 using a zero constant.
- `clear_d0_byte`: Performs a logical OR operation on the least significant byte of the data register zero with zero.
- `initialize_global_state`: Saves registers to the stack, stores the value of register d0 into a global variable, clears two global longwords, sets a third global longword to the address of the second, calls a subroutine, and restores the registers.
- `format_numeric_string`: Converts a long value into a string of decimal digits stored in a buffer, handling carries and padding with zeros.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `rotate_string_bytes`: Checks a byte at a pointer, and if it is non-zero, enters a loop that decrements a counter and rotates six bytes of data at that pointer, inserting a space at the start, until the counter reaches zero or a zero byte is encountered.
- `zero_data_register_d0_2`: Performs a logical OR operation on data register zero with zero, effectively ensuring the register contains zero.
- `perform_zero_or_instruction`: Performs an or operation on register d0 with zero.
- `manage_copper_and_joystick_state`: Initializes a copper list and enters a loop that monitors joystick input and vertical blanking status to dynamically update copper register values.
- `clears_data_register_zero_2`: Performs a logical OR operation on the data register zero with zero.
- `read_joystick_input`: Reads the state of the first joystick port and updates a global variable with the input value.
- `update_display_mode_parameters_2`: Calculates and writes two 32-bit values to global memory locations based on a multiplier and an offset, then checks the Amiga custom chip status registers to adjust a counter in a global variable and potentially increments a value in a register.
- `update_display_mode_parameters`: Checks a value in a register against a constant and adjusts a counter in a global variable, then reads the Amiga custom chip status to determine a multiplier.
- `calculate_and_update_display_parameters`: Calculates a value based on a constant and a multiplier, then stores it into two memory locations using bitwise operations.
- `perform_logical_or_zero_on_d0`: Performs a logical OR operation on the contents of register d0 using a zero constant.
- `setup_blitter_graphics`: Configures hardware registers for DMA and blitting, performs several blitter operations using specific source and destination addresses, and waits for joystick button bits to be set before returning.
- `initialize_data_tables`: Performs multiple memory copies and table updates using various subroutines, then clears specific global words and sets a hardware register.
- `update_display_and_random_state`: Updates coordinate tables, configures display and interrupt states by writing to a hardware register, updates structure data, and updates a pseudo-random state by applying a bitmask and multiplying a value by an address.
- `update_coordinate_tables`: Checks a word at a pointer, and if it is non-zero, calculates three values based on a constant and stores them as words in a table at a specific offset.
- `clear_data_register_zero_10`: Performs a bitwise OR operation on data register zero with zero, effectively leaving the register unchanged.
- `write_split_d0_values`: Runs a loop three times, storing the low word of d0 at offset 6 of a structure and the high word of d0 at offset 2 of the same structure, incrementing d0 by 9600 and the pointer by 8 each iteration.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `fill_memory_with_values`: Runs a loop 134 times, writing specific 32-bit values to a table starting at a global variable, alternating between a value derived from d1 and a constant value.
- `reconfigure_display_and_interrupt_state`: Checks a longword in a global variable; if non-zero, it updates a pointer in a global table, modifies the display enable and interrupt enable bits in the hardware, and performs a sequence of register writes and global variable updates before returning.
- `reconfigure_display_and_interrupts`: Checks a global word; if zero, it updates a pointer in a global table, modifies the display enable and interrupt enable bits in the hardware registers, and sets the interrupt mask.
- `process_conditional_state_updates`: Checks bits 5, 6, and 7 of a global variable to determine whether to execute a sequence of subroutines, update a global word to 321, and perform arithmetic on a global value.
- `handle_interrupt_and_copy_data`: Checks bit 5 of the interrupt request register; if set, it writes 32 to the custom chip register and returns from interrupt, otherwise it copies a word from a global variable to two different locations and writes a constant to another global variable.
- `manage_data_buffer`: Checks a word in a global variable; if it is non-zero, it increments a counter in a global variable, and if that counter reaches 3, it clears it, loads a pointer from a global variable into a0, checks if the value at a0 is negative, and if so, updates the pointer in a global variable and copies the value at a0 to a global variable.
- `process_structure_list`: Checks two global words, and if both are non-zero, iterates through a list of pointers to compare values and store results in a destination table.
- `wait_for_condition`: Calls two routines and then enters a loop that checks a word in a global variable, branching back to the start of the routine if the value is 0.
- `check_global_word`: Checks if a word in a global variable is non-zero, otherwise it returns immediately.
- `clear_word_at_offset`: Runs a loop d0 times, clearing the 16-bit word at offset 2 of the structure pointed to by a0 and incrementing the pointer by 4 each iteration.
- `compare_and_copy_data`: Saves registers to the stack, sets a word in a global variable to 1, copies d0 to a global variable, stores pointers in globals, and enters a loop that compares nibbles of values from two lists, writing 0, 255, or 1 to a destination buffer based on the comparison, then copies values from a source list to a destination list.
- `update_nibbles_conditionally`: Checks two global variables, increments a counter in a global variable, and enters a loop that compares nibbles of values from two lists, updating a destination list with modified values if the nibbles do not match, then performs a division and sets two global variables if a condition is met.
- `copy_bytes_to_table`: Runs a loop d0 times, copying a byte from the address in a0 to the address in a1, incrementing a0 by 1 and a1 by 4 each iteration.
- `calculate_nibble_averages`: Sets a word in a global variable to 2, clears a block of memory, and enters a loop that calculates and stores divided nibble values from a list into a new buffer, then updates several global variables with pointers and a counter.
- `blend_nibble_data`: Checks two global variables, increments a counter in a global variable, and enters a loop that compares nibbles of values from two lists, adding values from a buffer to a destination list if the nibbles differ, and finally sets two global variables if a specific count is reached.
- `process_data_tables`: Clears a memory range using a loop, then processes two sets of data from a structure in a0 to populate two different tables by performing bitwise operations and divisions.
- `update_table_values`: Checks specific global conditions and then iterates through a structure in a0, modifying its contents by subtracting values from a second table based on bitwise masks.
- `tiny_ori_12`: Small routine of 1 instruction that merges 0 into d3.
- `nullify_data_register_d0`: Performs a logical OR operation on data register zero with zero, effectively clearing the register.
- `zero_data_padding_3`: This routine contains a single instruction that performs a logical OR operation on a data register with zero, effectively leaving the register unchanged.
- `zero_data_padding_4`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero, effectively leaving the register unchanged.
- `perform_multiple_zero_ors`: Performs twenty-four consecutive bitwise OR operations on register d0 with zero and then returns.
- `perform_forty_eight_zero_ors`: Performs forty-eight consecutive bitwise OR operations on register d0 with zero.
- `update_structure_from_pointer`: Checks a value at offset 16 of a structure in a0; if non-zero, it copies a pointer from offset 16 to offset 20 and updates offset 24 with a value from the pointed-to memory.
- `update_structure_data`: If a longword at offset 16 of the structure in a0 is non-zero, it branches to another routine; otherwise, it updates a structure at offset 8 of a0 by performing a bitwise OR on a word at offset 14 of the target structure with a word from offset 6 of the source structure, and then stores a longword from offset 12 of the source structure into offsets 2 and 6 of the target structure.
- `calculate_structure_offsets`: Updates various fields in a structure pointed to by a1 based on values in a0, including conditional logic for bit manipulation and coordinate-like calculations.
- `zero_data_padding_5`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero, effectively acting as a null operation within a data segment.
- `zero_data_padding_6`: This routine consists of a single instruction that performs a logical OR operation on a data register with zero.
- `zero_data_byte_in_d0`: Performs a logical OR operation on the least significant byte of register d0 with zero.
- `clear_data_register_d0_8`: Repeatedly performs a logical OR operation with zero on the data register d0 before returning.
- `execute_conditional_subroutines_2`: Tests a word in a global variable and, if zero, calls three consecutive subroutines before returning.
- `clear_memory_buffers`: Clears three separate memory areas by iterating through a list of offsets provided by a global pointer.
- `update_position_data`: Loads four values from a table, performs arithmetic and bounds checking on them, writes the results back to a table, and calls another routine within a loop.
- `set_bitfield_data`: Checks if two input values are within specific bounds, calculates a destination index, and sets bits in three different memory planes based on a bitmask.
- `copy_conditional_table`: Checks a bitwise inverted word from a global variable; if non-zero, it copies six longwords from one table to another and updates two global pointers, otherwise it copies six longwords from a different table and updates the pointers.
- `call_subroutines`: Calls two other routines and then returns.
- `fill_random_tables`: Performs a series of bitwise transformations on a constant and calls another routine to generate and store values into three consecutive tables using loops.
- `clear_and_fill_tables`: Clears two large memory areas using a loop, then populates two tables with specific sequences of word values and memory addresses.
- `set_bit_in_d0`: Sets the lowest bit of d0.
- `set_memory_and_d0`: Sets a byte at the address in a0 to a specific value and sets several bits in d0.
- `advance_pointer_lists`: Checks three global pointers; if a pointer matches a specific value, it resets it to a global variable and copies the next word from that new location into a global variable.
- `render_sprite_data_table`: Iterates through a data table to find a specific byte, then uses the Blitter to draw multiple sprite instances by calculating screen positions and setting Blitter parameters.
- `contains_zero_byte_data_table`: This routine consists of a sequence of thirty-two bytes containing only zero values, acting as a data table rather than executable code.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `process_pointer_list`: Iterates through a linked list of pointers, performing checks on each element and calling other routines to handle data processing and sprite rendering.
- `wait_for_raster_line_3`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `perform_blitter_loop`: Sets blitter registers and performs a loop of three blitter operations using a source address in a0 that increments by 9600 each iteration.
- `perform_blitter_loop_alt`: Sets blitter registers and performs a loop of three blitter operations using a source address in a0 that increments by 9600 each iteration, using a different initial source address.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `update_pseudo_random_state`: Reads a 32-bit value from a global pointer, applies a bitmask and bitwise OR to the lower bits, multiplies the resulting value by an address using a series of shifts and additions, and stores the updated 32-bit state back to the global pointer.
- `tiny_addq`: Small routine of 1 instruction that adds 1 to d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `multiply_and_store_state`: Multiplies the values in d0 and d1 by two using carry-propagating addition and stores the result in a global table.
- `reduce_result_modulo_increment_2`: Adds one to an input word and, if the result is non-zero, calls another routine and returns its result modulo that incremented value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_state_and_execute_sequence`: Clears several consecutive global variables, sets a global variable to 1, and then executes a long sequence of subroutine calls that vary based on a conditional check of a global variable.
- `execute_jump_sequence`: Calls a subroutine at a specific address and then jumps to another subroutine.
- `process_input_and_update_state`: Reads the status of the joystick and mouse fire buttons from the CIAA Port A data register, calls a routine to set constant values, calls a routine to execute a stored function if a global variable is non-zero, and calls a routine to update a state counter based on a global variable.
- `update_value_based_on_comparison`: Compares a word at offset 256 of a structure in a0 to eight; if the result is not positive, it increments the value, tests a bit in a word at offset 386 of a0, and either adds 14 to a location in a1 or clears a location in a1 before returning.
- `copy_and_call_routine`: Loads a pointer into a0, performs a loop 49 times to push values onto the stack, moves a value into a1, and calls another routine.
- `copy_and_jump`: Sets a7 to a specific address, disables interrupts, copies 49 longwords from the address in a7 to the address in a0 using a loop, stores a1 at offset 128 of a structure, sets a7 to a new address, and jumps to another routine.
- `set_constant_values`: Calls two unnamed routines and then stores the value 14 into two different global variables.
- `copy_structure_data`: Copies seven 16-bit words from a source structure pointed to by a1 into a destination structure pointed to by a0 at various offsets.
- `clear_memory_block`: Clears seven consecutive 32-bit words starting at a specific memory location and returns.
- `update_pointer_structure`: Increments a 16-bit value at an offset of 2 from a pointer stored in a global variable, and if another global variable is non-zero, copies that value to the pointer location and performs further conditional checks and pointer updates.
- `decrement_and_load_buffer`: Decrements a global variable and, if it reaches zero, loads a new value from a pointer and updates a global variable with a value from a table.
- `handle_state_transitions`: Checks bits in a global variable and, depending on which bits are set, executes various routines, modifies the global variable, and performs conditional increments or resets.
- `set_status_word`: Tests a word in a global variable and, based on the result and a sign bit check, writes either one or two to a global variable.
- `call_subroutines_sequentially`: Calls four different subroutines in sequence and then returns.
- `fill_memory_with_zeros`: Fills a block of memory starting at a specific address with 26520 zero bytes using a loop and returns.
- `fill_memory_with_incrementing_values`: Fills a memory block starting at a global variable with 700 longwords, where each value is 160 greater than the previous one, starting from a constant value.
- `fill_memory_with_sequence`: Fills a large memory block starting at a global variable with a sequence of 32-bit values, where each value is 248 greater than the previous one, using a loop.
- `update_pointer_list`: Checks if a pointer in a global variable is non-zero, then either stores a calculated 32-bit value into that location and increments the pointer, or branches to another routine.
- `manage_pointer_range`: Checks a global pointer, and if not null, stores a specific 32-bit value at that location, increments the pointer, and then compares the new pointer against a calculated limit to decide whether to reset the pointer to a base value.
- `increment_word_counter`: Checks a 16-bit value at a global pointer, and if it is not negative, stores zero at that location and increments a counter, provided the counter does not exceed 288.
- `return_constant_byte`: Returns the value 255 into the accumulator and exits.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_globals_and_call_subroutines`: Clears four longwords in a global memory area, calls a subroutine twice with a specific address in a0, and then calls another subroutine before returning.
- `set_structure_field`: Sets two global variables to specific addresses and then, if a global variable is not equal to two, writes 65535 to an offset of 316 in a structure pointed to by a register.
- `initialize_structure_fields`: Writes several constant values to various offsets within a structure pointed to by a0, including two values copied from a table and another from a global variable.
- `setup_and_call_routines`: Sets several global variables to specific values and then calls two different routines.
- `setup_and_call_routines_variant`: Sets several global variables to specific values, calls two different routines, and then sets a global variable to 500.
- `initialize_data_pointers`: Writes several constant values and memory addresses into a sequence of global variables before calling two other routines.
- `setup_and_call_routines_variant_3`: Sets several global variables to specific values, calls two different routines, clears several global variables, and then calls another routine.
- `populate_data_table_and_call_subroutines`: Writes several constant values into a sequence of global memory locations and then calls two separate subroutines.
- `store_init_values_and_call_helpers`: Writes a fixed sequence of constant values into global variables, then calls two external helper routines and calls itself recursively.
- `execute_stored_function`: Checks if a global variable is non-zero, and if so, clears the global variable and jumps to the address stored within it.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a bitwise OR operation on the d0 register with zero.
- `copy_bytes_and_branch`: Clears two words in a global variable, copies two bytes from a global variable into d0 and d1, stores those bytes into two other global locations, and branches to another routine.
- `update_state_counter`: Checks a global variable, and if non-zero, performs several operations including incrementing a counter, calling another routine, and updating various global variables based on the results.
- `perform_multiple_logical_ors`: Performs ten consecutive bitwise OR operations on the d0 register using different constant byte values.
- `clear_d0_register`: Performs a bitwise OR operation on register d0 with zero.
- `clear_data_register_d0`: Sets the lowest byte of data register d0 to zero.
- `extract_vposr_bits`: Clears d0, pushes it to the stack, reads the VPOSR register, shifts it right by 8 bits, masks it with 511, stores the result in a global variable, and restores d0 before returning.
- `extract_vposr_bits_2`: Clears d0, pushes it to the stack, reads the VPOSR register, shifts the value right by 8 bits, masks it with 511, and stores the result in a global variable.
- `call_subroutine_with_data`: Saves d0 through d2 to the stack, loads a 32-bit value from a global variable into d0, calls another routine, and restores the registers before returning.
- `clear_data_register_d0_2`: Performs a logical OR operation on data register d0 with zero.
- `decrement_counter_with_wait`: Decrements a 16-bit value in a local variable in a loop, waiting for a specific bit pattern in a global variable to change from and then back to a value where bits 8 through 16 equal 300 before each decrement.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_dynamic_offset_and_pointer`: Calculates a value based on a global variable and a table entry, then performs a series of arithmetic operations to update a global variable with multiple offset-based increments.
- `copy_table_data`: Compares a global variable against other values to select a source address, then copies sixteen 16-bit words from that source address into a series of global variables.
- `fill_buffer_with_constant`: Stores the constant value 152 into 18 consecutive 16-bit words starting at the address in a0.
- `initialize_data_structure`: Stores a byte from d0 at the address in a0, sets the byte at offset 1 to 1, and then populates various 16-bit words at specific offsets within the structure pointed to by a0.
- `copy_memory_block`: Copies 16 consecutive 16-bit words from the memory block starting at the address in a0 to the memory block starting at the address in a1.
- `modify_register_bits`: Tests bit 0 of d0 using the value at the address in a0, then sets bits 1, 5, and 6 of d0.
- `set_multiple_bits`: Sets various bits in d0 using a sequence of logical OR operations with specific byte constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_interrupt_and_call_subroutine`: Enables interrupts by writing to the INTENA register, saves registers, calls a subroutine, stores the result in a global variable, calls another subroutine, restores interrupts to a disabled state, and returns.
- `loop_and_call_subroutine`: Executes a loop that calls a subroutine a number of times determined by a value in a register.
- `verify_checksum_and_process_data`: Performs a loop to calculate an XOR checksum of words from a list in a0, compares it to a value, and if valid, processes data by ORing masked values and XORing elements from two structures.
- `xor_checksum_loop`: Calculates a checksum by looping through a list of words pointed to by a0, performing an XOR operation on each, and then masks the result with d7 before returning.
- `process_data_and_wait`: If a global variable is not 255 or does not match another global variable, it writes a value to a structure in a6, calls a subroutine, enters a loop that increments a pointer until a matching word is found, calls a subroutine, and updates a global variable and a calculated value; otherwise it returns.
- `process_data_blocks`: Iterates through a range of memory, comparing values to find sequences of identical words, and then performs calculations to update a table based on the results of another routine.
- `combine_masked_values`: Reads two 32-bit words from the address in a0, masks both with d7, doubles the first masked value, and returns the result of ORing the two values.
- `wait_for_bit_clearance`: Performs several writes to a structure pointed to by a6 and then enters a loop that decrements a counter until bit 1 of the 16-bit word at offset 30 of that structure is cleared.
- `set_cia_bit`: Reads a byte from an offset of 7424 from the address in a5, masks it to keep only the two highest bits, sets bit 3, and writes the result back to the same location.
- `modify_byte_and_wait`: Clears seven 32-bit words at a memory location, calls a routine to wait for bit 5 of a byte pointed to by a4 to be set, and performs bitwise manipulations on a byte at a pointer in a5 before returning.
- `modify_byte_and_branch`: Saves registers to the stack, calls a subroutine, writes the value 253 to a byte at a pointer in a5, sets bit 7 of that same byte, and branches to another routine.
- `wait_for_bit`: Loops until bit 5 of the byte pointed to by a4 is set, then returns.
- `set_bit_and_loop`: Sets bit 0 of a byte at a pointer in a5, checks bit 4 of a byte at a pointer in a4, and if set, calls a subroutine and branches back to the start of the routine; otherwise it pushes d0 to the stack, runs a loop 16384 times, restores d0, clears a 32-bit global variable, and returns.
- `update_status_bits`: Sets bit 2 of a byte at a pointer in a5, and if a bit in a 32-bit value is set, clears bit 4 of that same byte.
- `decrement_and_store`: Runs a loop 48832 times, stores the value from d2 into a global variable, and restores registers from the stack before returning.
- `increment_counter_and_clear_bit`: Compares a 16-bit global variable to 160; if it is less than 160, it clears bit 1 of a byte at a pointer in a5, calls a subroutine, and increments the global variable by 2; otherwise it returns.
- `decrement_counter_and_set_bit`: Compares a 16-bit global variable to 160; if it is less than 160, it sets bit 1 of a byte at a pointer in a5, calls a subroutine, and decrements the global variable by 2; otherwise it returns.
- `set_values_and_loop`: Sets bit 0 and then clears bit 1 of a byte at a pointer in a5, writes 100 and 8 to specific offsets of that pointer, sets a global variable to 0, loops until bit 0 of a byte at offset 7169 is set, and then sets bit 0 of the byte at a5 again.
- `compare_and_clear_table`: Compares a byte from a global variable against a table, and if certain conditions are met, loops through a table to compare bytes, calls another routine, and potentially clears 64 words of data starting at a pointer in a global variable.
- `clear_word_table`: Stores a pointer in a global variable, then loops 64 times to clear each word of the data at that pointer to zero.
- `search_string_table`: Loads a pointer from a table, retrieves a byte, calls another routine, and then loops through a sequence of bytes, incrementing pointers and checking values until a termination condition is met.
- `find_byte_in_string`: Compares a byte in d0 against characters in two consecutive tables, incrementing a pointer and returning the address of the first matching byte or the end of the second table.
- `set_global_values`: Sets a global variable to 1 and stores a constant address in another global variable, then returns.
- `update_conditional_pointers`: Checks if a global variable is negative; if so, it stores a specific address in another global variable.
- `configure_hardware_registers`: Writes specific values to several hardware registers and a global variable.
- `step_pseudo_random_state`: Advances a 32-bit state held in two globals with a multiply-by-constant and carry-propagating add, then returns the updated state.
- `reduce_result_modulo_increment`: Increments a 16-bit value in d0, and if the result is non-zero, calls another routine and returns the result of a division using the incremented value as the divisor.
- `update_register_and_memory`: Performs a bitwise OR on a word at a pointer in a0, subtracts a constant from a value at a pointer in a4, and stores an incremented value of d0 into a global variable.
- `loop_until_call_returns`: Calls another routine in a loop, decrementing d0 each time, until the called routine returns a non-zero value.
- `update_raster_state_and_callbacks`: Calls a routine to update conditional pointers, stores a value from d0 into a global variable, and then conditionally calls three routines and a callback list based on a check of a global variable.
- `adjust_raster_value`: Calls a routine to get a raster value, compares it to a value from a global variable, and enters a loop that adds 214 to d0 and subtracts the global value until d0 is within a specific range relative to the global value.
- `perform_bitwise_or`: Performs a bitwise OR operation on d0 with zero.
- `get_raster_line`: Reads a long value from a hardware register, shifts it right by 8 bits, masks the result to 9 bits, and returns it in d0.
- `initialize_pattern_and_clear_flags`: Sets five global longwords to a specific repeating pattern and clears five 16-bit values in a separate global memory area before returning.
- `copy_data_and_run_logic`: If a global variable is zero, it copies 2152 words from one memory location to another using a loop, then calls another routine and performs conditional logic based on a global variable's value.
- `execute_conditional_subroutines`: Sets a global variable to zero, calls another routine, and if that global variable is non-zero, calls another routine, performs conditional logic based on the value, and calls a final routine.
- `transfer_and_test_data`: Stores the value of d2 into a global variable, tests a word at a specific location, and if that word is non-zero, calls two different routines and another routine.
- `execute_setup_sequence`: Loads a pointer into a0, sets d0 to 22 and d1 to 82, and calls another routine.
- `perform_complex_data_manipulation`: Executes a sequence of operations including a loop that calls a subroutine, calls subroutines via the stack, and clears bits in a table based on a bitmask calculated from d0.
- `call_subroutines_via_stack`: Pushes the value in a1 onto the stack, calls another routine, restores a1 from the stack, moves it to a0, pushes a0 onto the stack, calls another routine, and restores a0 from the stack before returning.
- `chain_call_via_pointer`: Pushes a pointer from register a1 onto the stack, calls a routine, restores the pointer to register a1, moves it to register a0, pushes it onto the stack again, calls a second routine, restores the value to register a0, and returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_bit_tables`: Sets a series of constant values into various global variables, including several 16-bit values and bitmasks.
- `execute_callback_list`: Sets several global variables, then enters a loop that iterates up to five times, calling routines from a table and checking a global variable for a non-zero value to determine whether to return early.
- `tiny_ori_2`: Small routine of 1 instruction that merges 64 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 142 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 196 into d0.
- `tiny_ori_5`: Small routine of 1 instruction that merges 32 into d0.
- `tiny_ori_6`: Small routine of 1 instruction that merges 30 into d0.
- `set_bit_64`: Sets bit 6 in the byte in d0.
- `tiny_ori_7`: Small routine of 1 instruction that merges 4 into d0.
- `set_bit_2`: Sets bit 2 in the byte in d0.
- `set_bit_5`: Sets bit 5 in the byte in d0.
- `set_multiple_bits_2`: Sets bit 2 in the byte in d0 and then performs five operations that logically result in no change to the byte in d0.
- `set_bit_six_of_d0`: Sets bit six of data register zero using a logical OR operation.
- `set_bits_8e`: Sets bits 1, 3, 5, and 7 in the byte in d0.
- `set_bits_c4`: Sets bits 2, 6, and 7 in the byte in d0.
- `set_d0_bit_5`: Sets bit 5 of register d0.
- `set_d0_bits_4_5`: Sets bits 4 and 5 of register d0.
- `set_d0_bit_6`: Sets bit 6 of register d0.
- `set_d0_bit_2`: Sets bit 2 of register d0.
- `set_bit_two_in_d0`: Performs a bitwise OR operation on register d0 with the constant four.
- `set_bit_5_of_d0`: Sets the fifth bit of data register zero using a logical OR operation.
- `clear_bits_in_table`: Calculates a bitmask based on register d0 and clears the corresponding bit in a byte at a calculated pointer, then clears the same bit at offsets 38, 76, 114, 152, 190, 228, and 266 from that pointer.
- `clear_d0_bits`: Performs a logical OR of register d0 with zero.
- `clear_bits_in_table_extended`: Calculates a bitmask based on register d0 and clears the corresponding bit in a byte at a calculated pointer, then clears the same bit at offsets 38, 76, 114, 152, 190, 228, and 266 from that pointer and calls another routine.
- `update_status_and_word`: Sets the byte at a global variable to the sign bit of the longword at a global variable, then writes a 16-bit value of all ones to another global variable.
- `update_nibble_data`: Checks specific global conditions and then enters a loop that compares nibbles of 16-bit words in a table, updating the table with values from another source if the nibbles do not match.
- `accumulate_nibble_differences`: Checks global conditions and then enters a loop that compares nibbles of 16-bit words in a table, adding values from a global array to a destination table at specific offsets whenever a mismatch is found.
- `convert_pixel_data`: Clears a block of memory, then performs two loops to transform data from a source table into a destination buffer by performing arithmetic shifts and bitwise operations on 16-bit words.
- `subtract_pixel_values`: Checks two global variables and a counter, then iterates through a table to subtract values from a destination buffer based on non-zero bit patterns in a source table, calling another routine before returning.
- `tiny_ori_8`: Small routine of 1 instruction that merges 2 into d0.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `provide_zero_nibble_value`: Acts as a data source providing a zero value for nibble-based comparison and accumulation operations used by update_nibble_data and accumulate_nibble_differences.
- `provide_nibble_difference_values`: Acts as a data table containing three byte values used by update_nibble_data, accumulate_nibble_differences, convert_pixel_data, and subtract_pixel_values to perform arithmetic operations on individual nibbles.
- `return_byte_from_register`: Returns the byte stored in register d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `enable_interrupts_conditionally`: Tests a longword in a global variable and, if it is zero, sets bit 14 of the INTENA register before returning.
- `update_global_variable`: Performs several bitwise operations and assignments on a global variable using values derived from bit shifts and logical operations, then returns.
- `reset_multiple_globals`: If a global variable is non-zero, it clears several global variables and sets a specific global variable to 1, then calls another routine twice and sets a global variable to 1.
- `initialize_hardware_and_globals`: If a global variable is non-zero, it clears a global variable, calls another routine four times, sets a global variable to 12, calls another routine, clears a global variable, sets several global variables to 2 or 15, and clears four more global variables.
- `configure_dma_and_interrupts`: Tests a longword in a global variable and, if it is zero, copies a word from a global variable to the DMA control register, then checks bit 3 of that word to decide whether to write specific values to the DMA mask and INTENA registers and clear a global variable before returning.
- `zero_data_area`: This area contains zeroed bytes that serve as padding or placeholder values for other routines to access via absolute addressing.
- `tiny_ori_9`: Small routine of 1 instruction that merges 1 into d0.
- `check_global_conditions`: Evaluates multiple conditions on a global variable and, if a specific global variable reaches a certain value, calls another routine.
- `update_pointer_reference`: Checks if a global variable is non-zero and compares several global pointers before updating a global pointer with the address of that variable.
- `clear_audio_buffer`: Clears a block of memory using a loop, sets a global variable to a constant, and updates another global variable with a pointer if a condition is met.
- `update_audio_sample_buffer`: Copies blocks of data from a source table to a destination buffer based on a decreasing counter in a global variable, then updates the third audio channel length, volume, and period registers and modifies interrupt settings.
- `copy_audio_sample_data`: Copies a specified number of words from a source buffer into a destination table in RAM, processing the data in blocks of 32 words.
- `clear_data_register_zero_2`: Performs a bitwise OR operation on data register zero with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero using a zero constant.
- `clear_data_register_zero_3`: Sets the least significant byte of data register zero to zero.
- `clear_multiple_globals`: Sets eleven different global variables to zero.
- `update_global_state`: Checks a global variable and, depending on its value, updates several other global variables with the contents of register a6 or zero, then returns.
- `clear_data_register_d0_3`: Performs a logical OR operation on register d0 with zero to clear its contents.
- `clear_data_register_zero_4`: Performs a logical OR operation on data register zero with zero to clear its value.
- `clear_data_register_zero_5`: Performs a logical OR operation on data register zero with zero to ensure its value is zeroed.
- `zero_data_padding`: This area contains a single instruction that performs a logical OR on register d0 with zero.
- `zero_data_padding_2`: This area contains zeroed bytes used for padding or as a null data block.
- `clear_pointer_and_reset_flag`: Clears register d0, loads a pointer from a global variable into a6, and if that pointer is non-zero, clears the memory at that location and returns; otherwise it returns immediately.
- `manage_linked_list`: Manages a structure by initializing several global variables if a pointer is non-zero, or traversing a linked list by updating a pointer to the next element in the list.
- `process_conditional_logic`: Checks a global variable and, if it is not zero, performs several conditional checks on a value, potentially clearing a global variable or incrementing a global variable and resetting it to zero if it reaches two, then calls two other routines.
- `increment_global_counter`: Checks a global variable and, if it is not zero and a condition on another value is met, clears that value and increments a global counter that wraps back to zero after reaching two.
- `store_input_value`: Checks a global variable and, depending on the value of a register, updates a global variable with the contents of that register or sets a specific global variable to three.
- `clear_d0_register_3`: Clears the least significant byte of register d0.
- `tiny_ori_10`: Small routine of 1 instruction that merges 66 into d0.
- `load_audio_sample_parameters`: Calculates an offset into a table based on an input value and loads various audio properties, including period, volume, and sample data pointers, into global variables and hardware registers.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `clear_data_register_d0_4`: Performs a bitwise OR operation on data register d0 with zero.
- `clear_data_register_zero_6`: Performs a logical OR operation on the least significant byte of data register zero using a zero constant.
- `clear_data_register_d0_5`: Clears the data register d0 by performing a bitwise OR operation with zero.
- `clear_d0_register_4`: Performs a logical OR operation on the d0 register with zero, effectively clearing the register.
- `clear_data_register_zero_7`: Performs a bitwise OR operation on data register zero using a zero constant.
- `clear_d0_register_2`: Performs a bitwise OR operation on register d0 with zero twice.
- `clear_data_register_d0_6`: Clears the contents of data register d0 by performing a logical OR operation with zero.
- `clear_data_register_zero_8`: Sets the lowest byte of data register zero to zero.
- `clear_data_register_zero_9`: Clears the least significant byte of the data register zero using a logical OR operation.
- `tiny_ori_11`: Small routine of 1 instruction that merges 124 into d0.
- `update_audio_parameter_table`: Iterates through a table of structures to decrement counters and update audio period and volume registers using values stored at specific offsets within each structure.
- `set_status_flags`: Writes zero to a memory location pointed to by a0 and sets various global variables to specific constants based on the value of d1.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `calculate_address_table`: Updates a global variable based on two memory locations, loops through a table to find a maximum value, and then performs a loop that writes calculated addresses into a series of global variables using offsets from a table.
- `compact_pointer_table`: Scans four entries of a table; for each non-null pointer, clears the entry and copies eight words from the pointed-to data into the table slot.
- `update_dma_sprite_hardware_parameters`: Iterates through a table of structures to update hardware DMA registers, including position and property values, while conditionally calling update_animation_channels and update_dma_sprite_parameters based on specific byte flags within each structure.
- `update_entity_state_from_table`: Iterates through a structure using a loop that checks hardware register status, then performs complex calculations to update a data structure by reading values from a table and writing them to various offsets, including bitwise operations on a global variable and updating a 16-bit value with a bit set.
- `update_dma_sprite_parameters`: Updates a structure containing sprite or object properties by reading values from a data table and writing them to a memory block, including calculations for color and position.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `update_animation_channels`: Iterates through three structures in a loop, checking specific bytes at offset 26 and offset 32, and calls another routine if a byte is non-zero.
- `update_three_channel_animations`: Advances three independent animation channels held in a structure: each channel counts down a delay, then steps an index through a table, computes a scaled value from a global table and writes it to an output field, wrapping the index and optionally clearing the channel's enable flag.
- `expand_bitstream_data`: Reads a bitstream by right-shifting a longword and calls shift_and_branch to extract bits, using the bit values to calculate an offset into a table and writing bytes to a destination buffer.
- `shift_and_branch`: Loads a longword from a pointer in a0 into d0, performs a logical right shift by 1 on d0, and returns.
- `loop_bit_shifts`: Performs a loop that right-shifts a value in d0 and calls another routine when d0 reaches zero, repeating for a count specified in d1.
- `process_hunk_data`: Checks if a longword at a0 equals 979, then iterates through a list of values to calculate offsets and calls another routine repeatedly using values derived from the list.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `branch_to_nearby_instruction`: Performs a short relative branch to an instruction located slightly before the current routine.
- `branch_to_offset_14332`: Performs a short relative branch to a location 14332 bytes ahead in the code segment.
- `branch_to_middle_of_routine`: Performs a short branch to a location within another routine.
- `branch_to_next_routine`: Returns a constant zero and then performs a short branch to another routine.
- `branch_to_middle_of_routine_2`: Performs a short branch to an offset within another routine.
- `calculate_offset_and_return_one`: Loads a longword from a pointer in a0, doubles it twice, adds the result to a0, and returns one in d0.
- `return_constant_zero`: Returns a constant value of zero.
- `zero_out_memory_block`: Uses a value from a0 as a count to fill a memory block pointed to by a value at offset 4 of a structure in a5 with zeros, then returns one in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_data_sequence`: Writes several longwords to a global variable and returns a longword constant.
