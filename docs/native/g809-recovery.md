# Game 809 (alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00809). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (967280 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 34 of 34 units identical (bytes and relocations), 99 raw-word lines |
| Functions named and described | pass, 314 of 314 named (0 author symbols), 314 of 314 described (17 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3526 of 3528 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 93696 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6086 of 6939 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 14722 |
| code | 29184 |
| data | 851398 |
| padding | 808 |
| unused | 71168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf boot-block load | 1 | 92 | 2 |
| logged disk load from Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf boot-block load | 1 | 0 | 0 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1622 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 230 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1079 | 79 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1642 | 66 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf bytes 61952-62464 (code never loaded by a reference run) | 1 | 0 | 0 |
| Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf blocks 22-119 (bytes 11264-61440) (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |
| Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf blocks 388-471 (bytes 198656-241664) (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: custom-format (non-DOS) disk, 1618 non-empty blocks: matched against track-loaded segments
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: the boot block reads 5120 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: the boot block reads 6656 bytes from disk offset 4608 to an address (found statically); recovered as a unit
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
- runtime segment an address (play-rs-m1500-unpk00): 1260 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 26 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 226 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 185 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 512 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1182 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 17 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: bytes 61952-62464 look like code no run loaded: recovered position-independently (load address unknown)
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
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: 3 logged loads cover 10752 bytes of the image
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf blocks 22-119 (bytes 11264-61440) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 50176 bytes decode)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 123-123 (bytes 62976-63488) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.92)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 153-163 (bytes 78336-83968) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.74)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 182-184 (bytes 93184-94720) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 186-188 (bytes 95232-96768) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 190-197 (bytes 97280-101376) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 200-201 (bytes 102400-103424) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 204-206 (bytes 104448-105984) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 208-210 (bytes 106496-108032) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 212-219 (bytes 108544-112640) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 222-223 (bytes 113664-114688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 226-228 (bytes 115712-117248) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 230-232 (bytes 117760-119296) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 234-241 (bytes 119808-123904) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 244-245 (bytes 124928-125952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 248-250 (bytes 126976-128512) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 252-254 (bytes 129024-130560) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 256-263 (bytes 131072-135168) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 266-267 (bytes 136192-137216) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 270-272 (bytes 138240-139776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 274-276 (bytes 140288-141824) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 278-285 (bytes 142336-146432) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 288-289 (bytes 147456-148480) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 292-294 (bytes 149504-151040) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 296-298 (bytes 151552-153088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 300-307 (bytes 153600-157696) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 310-311 (bytes 158720-159744) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 314-316 (bytes 160768-162304) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 318-320 (bytes 162816-164352) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 322-329 (bytes 164864-168960) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 332-333 (bytes 169984-171008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 336-338 (bytes 172032-173568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 340-342 (bytes 174080-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 344-351 (bytes 176128-180224) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 354-355 (bytes 181248-182272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 358-360 (bytes 183296-184832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 362-364 (bytes 185344-186880) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 366-373 (bytes 187392-191488) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 376-377 (bytes 192512-193536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 380-382 (bytes 194560-196096) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 384-386 (bytes 196608-198144) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf blocks 388-471 (bytes 198656-241664) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 43008 bytes decode)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 1550-1552 (bytes 793600-795136) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 1554-1561 (bytes 795648-799744) outside the filesystem hold data, not code (opcode density 0.040, entropy 1.49)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 1564-1565 (bytes 800768-801792) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.91)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 1568-1570 (bytes 802816-804352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.38)
- alien breed (1991)(team 17)(disk 1 of 2)[cr skr][t +10 legend]/Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf: blocks 1572-1574 (bytes 804864-806400) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- 3 never-loaded region(s), 93696 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 3 packed track region(s), 631296 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `prepare_cia_disk_and_custom_regs` (runtime segment an address): Loads custom chip base, CIA-B and CIA-A addresses into a6, a5 and a4, reads the vector at address 1012 into a3, sets d7 and d6 to fixed constants, then sets bits in CIA-A port A and returns. Kind: disk hardware/track check. Pass path: sub_0952 or sub_0714.

## Functions


### Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf:bootblock

- `boot_block_io_raster_wait`: Fills an I/O request with a fixed size and offset, runs the exec DoIO call, then loops with a delay and waits until a raster-position field reaches a fixed line.
- `allocate_memory_then_issue_io`: Restores a block of saved values, allocates memory through exec and waits until the result is nonzero, then sets up an I/O request and issues the exec DoIO call.

### logged disk load from Alien Breed (1991)(Team 17)(Disk 1 of 2)[cr SKR][t +10 Legend].adf boot-block load

- `rotate_checksum_word_through_extend`: Pops a longword from the stack pointed to by a0, XORs it into d5, sets the extend flag, rotates the value right through extend by one bit and returns the result in d0.
- `shift_d0_through_a0_with_eor_carry`: Shifts d0 right by d1 bits, pulling new bits from memory at a0 and mixing them into d5 with eor, rotating the carry through d2 and returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_flag_then_load_seven_registers`: Tests the word at memory location 2e92e and branches away if nonzero; otherwise loads d0 through d7 with the constants zero, 1adb0, zero, 3, 1, 1, zero and zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_16_words_then_set_global`: Loops calling another routine until it returns zero, then zeroes sixteen words starting at a fixed memory address, calls a routine and stores a constant in a global.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `read_ciaa_fire_button_to_d0`: Saves d2-d7 and a2-a6 on the stack, tests bit 7 of the CIA-A port A register, and sets d0 to zero if clear or minus one if set before branching on.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_call_five_subroutines`: Stores d2 through d7 and a0 into fixed memory slots, then calls five subroutines in sequence without restoring the saved registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `save_registers_and_call_five_routines`: Stores d3, d4, d5, d6, d7 and a0 into fixed memory locations, then calls five subroutines in sequence before continuing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_all_registers_and_return`: Pops d0-d7 and a0-a6 from the stack, restoring every data and address register, then returns to the caller.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `read_fire_button_into_d0`: Saves d2-d7 and a2-a6 on the stack, tests bit 7 of the CIA-A port A register, and sets d0 to zero if the fire button is pressed or to minus one otherwise.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_immediate_return`: An empty routine that returns immediately without touching any registers or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_play_runtime_and_jump`: Calls three setup routines in sequence, the last with the address of the following instruction, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_audio_period_and_call_tail`: Writes the value 16384 to offset 36 of the a6 structure, calls a nearby routine, then restores d2-d7 and a2-a6 from the stack and returns.
- `toggle_bit3_on_a5_byte`: Sets the byte at a5 to all ones, clears bit 3, rotates d0 by eight bits, sets bit 3 again, and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_six_registers_and_call_170c`: Loads d1 through d7 from six longwords near address an address, clears d4, then calls the routine at 170c and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_22_then_restore_registers`: Calls the routine at offset 22, then pops d2-d7 and a2-a6 from the stack and returns to the caller.
- `toggle_bit3_on_a5_byte_2`: Sets the byte at a5 to all ones, clears bit 3, rotates d0 by eight bits, sets bit 3 again and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_sprite_vectors_and_call_setup`: Copies four longwords from fixed memory locations into d3, d4, d5 and d6, clears d4, loads d7 from another fixed location, then calls the setup routine at the given address and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `toggle_audio_filter_bit_in_a5`: Sets the byte at a5 to all ones, clears bit 3, rotates d0 by eight bits, then sets bit 3 again and returns the rotated byte in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_fire_buttons_and_return`: Returns immediately, but the following code tests bit 7 then bit 6 of the CIA-A port register to read the joystick fire buttons.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_and_set_a5_flag_bit0`: Polls bit 0 of the byte at offset 7169 from a5 until it is set, then sets bit 0 of the byte at a5 and returns.
- `delay_loop_262144_iterations`: Loads an address into d0, decrements d0 until it reaches zero, then returns to the caller.
- `init_sound_state_and_setup_pointers`: Tests the word at the sound state address, sets it to one if clear, loads a0 and a1 pointers, calls a subroutine, then loads a0 and sets d0 and d1 constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_2a792_asset_and_check`: Loads the pointer at absolute address 2a792 into a0, calls a subroutine, tests the returned d0 and branches away if negative, otherwise copies a0 to a2, loads address 2aaf6 into a0, calls another subroutine and clears the long at 2b00c.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_2a852_flag_and_2ad00_buffer`: Branches if the zero flag is clear, sets the word at 2a852 to one, loads pointers 2a856 and 2ad00 into a0 and a1, calls a subroutine, then writes the long 2b60e and word 4 into the structure at 2aaf2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_2aaf6_asset_and_clear_2b00c`: Calls a subroutine, tests the returned d0 and branches away if negative, otherwise copies a0 to a2, loads address 2aaf6 into a0, calls another subroutine and clears the long at 2b00c.
- `init_level_and_load_abk_data`: Writes 2 to a hardware register, calls a setup routine, loads a pointer and value from fixed memory, calls a search routine, then clears the register and returns.
- `insert_sorted_entry_into_table`: Scans up to ten 8-byte entries in the table at a0 for the first whose key is not greater than d0, shifts later entries down, stores d0 and fills the entry's name bytes with spaces or 65 markers, returning d0 or -1.
- `zero_filled_data_area`: A single zero word decoded as an ori instruction; this is a zero-filled data area referenced by address, not executable code.
- `advance_three_frame_counters`: Increments three word counters in memory, wrapping the first at 24, the second at 7, and setting a fourth word to 1, then returns.
- `set_flag_store_a2_call_3e1a_setup_lea`: Writes 1 to a word flag, stores a2 to memory, saves a0, calls a subroutine, restores a0, loads two address registers and sets d0 to 8.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_two_vertical_blank_toggles`: Polls the VHPOSR hardware register waiting for value 255 then value 0, decrementing d0 from 2 and returning after two such waits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_five_entries_to_four_tables`: Copies five eight-byte entries from four offsets of the pointer at a fixed longword into four destination tables, converting the first two via a subroutine and copying the last two byte by byte.
- `zero_word_data_padding`: A single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_block`: Nineteen zero words that decode as ori.b #0,d0; this is a zero-filled data table referenced by address, not working code.
- `zero_filled_data_word_at_0370`: Contains a single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_7`: A single zero word that decodes as an ori instruction; this address is a zero-filled data area referenced by other code, not executable code.
- `save_regs_store_d0_clear_two_longs_call`: Saves all registers on the stack, stores d0 to a global, clears two adjacent long globals, points another global at them, calls a subroutine, then restores registers and returns.
- `format_score_into_decimal_string`: Reads a 32-bit score from memory, converts it to decimal digits by repeated subtraction of place values, writes the digits into a buffer, and pads or clears the remaining bytes.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `shift_score_digits_left_and_blank`: Scans six bytes of a digit buffer for a zero terminator, then shifts each digit one position left and writes a space at the start.
- `zero_filled_data_area_2`: Contains only a zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_word_data_at_abs_addq_target`: A single zero word that decodes as ori.b with immediate zero to d0; it is zero-filled data referenced by absolute addq, clr and cmpi operands, not executable code.
- `player_select_menu_loop`: Runs the player-select screen: writes copper list entries, polls JOY1DAT and CIAA_PRA for joystick and fire input, updates the selected player count and cursor, plays sound, and returns after both buttons are pressed.
- `zero_word_data_at_abs_clr_target`: A single zero word decoding as ori.b immediate zero to d0; this is zero-filled data pointed to by absolute clr, move and tst operands rather than a real routine.
- `joystick_input_dispatch_entry`: Entry point whose address is taken by several callers; it reads the joystick port movement register and branches into the shared input handling code.
- `player_select_menu_loop_2`: Reads joystick port registers to move a selection cursor between zero and three, updates sprite pointers, waits on fire, and stores the chosen player count.
- `cursor_move_left_and_fire_check`: Decrements the menu selection when the joystick moves left, clamps it between zero and three, then reads the joystick register to test for fire.
- `animate_weapon_sprite_and_advance_state`: Steps a sprite index through a 13-entry table, writes bitplane pointers and modulo to the copper list, calls the sprite blitter, and advances or clears the animation state counter.
- `zero_filled_data_area_3`: This single zero word decodes as ori.b #0,d0; the bytes are a zero-filled data area referenced by address, not executable code.
- `blit_title_screen_and_check_fire`: Sets up blitter pointers, modulo and size registers to copy three screen strips, waits for blits, then reads CIAA fire buttons and stores a state flag.
- `init_level_graphics_and_copper`: Copies sprite and tile data into buffers, writes copper list pointers and palette entries, sets several game state words, then installs a copper list at the hardware coprocessor address.
- `load_and_show_title_screen`: Calls setup routines, loads a graphics file into memory, points the copper list register at it, then displays and frees the screen before returning.
- `toggle_and_build_sprite_pointers`: Flips two toggle words and, if enabled, writes three sprite descriptor pointer pairs into the sprite table at successive offsets.
- `zero_filled_data_area_4`: A single zero word that decodes as ori.b #0,d0; this is zero-filled data referenced by address, not executable code.
- `write_three_plane_offsets`: Writes the low word of d0 to offset 6 and the high word to offset 2 of a0, advances d0 by 9600 and a0 by 8, repeating three times.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `fill_134_longwords_pattern`: Fills 134 longwords starting at the address in a0 with an incrementing pattern, resetting the value at two checkpoints, then returns.
- `install_vblank_chain_and_enable_ints`: Tests the word at a fixed long address, and if zero stores a pointer, saves INTENAR, swaps the level 3 vector to a new chain, then enables interrupts and copper DMA.
- `restore_vblank_vector_and_dma`: If the flag word is zero, disables level 3 interrupts, restores the saved level 3 vector, re-enables interrupts and sets DMACON to enable copper and bit 4.
- `vblank_joystick_and_sound_tick`: Checks CIAA fire bits, calls several subroutines, sets a counter, delays briefly, then restores saved registers and writes to the custom chip register.
- `vblank_intreq_ack_and_rte`: Checks bit 5 of the INTREQ register; if set, writes 32 to INTREQ to acknowledge the interrupt and returns from exception, otherwise falls through to copy two words and set a pointer before returning.
- `advance_sprite_frame_pointer`: If the enable word is zero it resets the frame pointer and copies a default word to two globals; otherwise it increments a counter, wraps it at three, reloads the pointer from a table when the word is negative, and stores the next word into the sprite frame global.
- `compare_sprite_palette_and_copy`: When both enable flags are set, it stores four to a control word, resets the pointer if empty, then compares eight words of two palettes nibble by nibble writing sign bytes and copies the source words onward before clearing the flag.
- `poll_flag_after_two_calls`: Calls two subroutines, then tests the word at memory location 2c6e2 and branches back to repeat the two calls while it is zero, otherwise returns.
- `test_flag_branch_to_loop`: Tests the word at memory location 2c6e2 and branches to the polling loop when it is zero, otherwise returns immediately.
- `clear_word_stride_four_loop`: Clears the word at offset two from a0, advances a0 by four bytes and decrements d0, repeating until d0 reaches zero, then returns.
- `build_nibble_difference_table`: Saves registers, compares each word of two arrays nibble by nibble and writes -1, 0 or 1 per nibble into a table, then copies words from a0 into the second array.
- `apply_nibble_deltas_to_array`: When enabled, steps a counter and, on wrap, adds stored nibble deltas to the second array's words, counting matches and setting the done flag when all match.
- `copy_words_with_stride_to_a1`: Copies d0 words from the a0 array into every other word of the a1 buffer, advancing a1 by six bytes per word, then returns.
- `init_palette_fade_from_a1`: Clears 48 longs of fade workspace, converts each source word from a1 into three 4-bit nibbles scaled by fifteen and stores them, then saves d0, a0 and a1 pointers and clears the fade counter.
- `step_fade_toward_target_palette`: When fade mode is two and idle, advances the frame counter and, per colour nibble differing from the target at a1, adds the delta from the workspace to the destination word at a0, marking completion when all match.
- `init_palette_fade_from_a0`: Clears 48 longs of fade workspace, splits each source word at a0 into nibbles scaled by fifteen and stores them, then copies raw nibble masks into the workspace and saves the a0 pointer with the fade counter cleared.
- `advance_digit_animation_frame`: When the animation state equals three and the busy flag is clear, increments a frame counter, and if it reaches the limit resets it and shifts digit nibbles in the sprite data using two tables, setting the busy flag when no digits changed.
- `zero_word_data_slot_b8e`: A single zero word that decodes as ori.b #0,d3; it is a zero-filled data slot referenced by address, not executable code.
- `zero_word_data_slot_b92`: A single zero word that decodes as ori.b #0,d0; it is a zero-filled data slot referenced by address, not executable code.
- `zero_word_data_at_abs_movea_target`: One zero word that decodes as ori.b with zero immediate to d0; it is zero-filled data addressed by absolute move and movea operands, not executable instructions.
- `zero_word_data_slot_two`: Another single zero word decoding as ori.b #0,d0; a zero-filled data slot whose address is taken by several routines.
- `zero_filled_data_block_with_rts`: Twenty-four zero words followed by an rts byte pattern; this is a zero-filled data block referenced by address, not real code.
- `data_table_zero_words_first`: Forty-eight zero data words decoded as ori.b instructions; this is a data table referenced by address, not executable code, and it changes no registers.
- `data_table_zero_words_second`: Another forty-eight zero data words decoded as ori.b instructions; a second data table reached by address, not working code, touching nothing.
- `apply_queued_command_to_target`: Reads a command block at a0: if the pending pointer is null it uses the stored value, otherwise it advances the pointer and reads its word; then it ORs the flags into the target's word at offset 14 and stores the high and low words at offsets 2 and 6.
- `update_sprite_from_animation_frame`: Reads a sprite descriptor at a0, advances its animation pointer when the frame counter expires, then writes position, flip and palette bits into the hardware sprite block at a1 and returns.
- `zero_word_data_slot_a`: This single zero word decodes as an ori.b immediate instruction but is a zero-filled data slot referenced by address from three routines, not executable code.
- `zero_word_data_slot_207a`: A single zero word that decodes as ori.b #0,d0; it is a zero-filled data slot whose address is stored by other code, not executable code.
- `zero_word_data_slot_b`: This single zero word decodes as an ori.b immediate instruction but is a zero-filled data slot referenced by address from two routines, not executable code.
- `zero_filled_data_block_2`: Twenty-four zero words followed by an rts byte pattern; the bytes are a zero-filled data block referenced by address, not executable code.
- `check_flag_then_call_sound_and_helpers`: Tests the longword at memory address 2aada; if nonzero it branches away, otherwise it calls the routine at 2d918 and three local subroutines before returning.
- `clear_three_buffers_by_offset_list`: Reads a base pointer and count, then for each offset in a table clears one byte in three buffers spaced three thousand bytes apart, finishing with rts.
- `transform_and_plot_sprite_offsets`: Adds base coordinates to a table of offsets, wraps x, y and z into range, stores the results, scales x and y by z, adds a screen origin and calls the plot routine for each entry.
- `plot_pixel_in_three_bitplanes`: Bounds-checks x and y, computes a byte offset and bit position, sets the bit in up to three bitplanes based on the depth value, and stores the offset in the output list.
- `toggle_level_palette_and_set_vectors`: Inverts a flag word; depending on its state copies six longwords from one table to another and stores two handler pointers, then returns.
- `run_two_setup_calls_and_return`: Calls two setup routines in sequence and returns to its caller without touching any registers itself.
- `seed_rng_and_fill_three_terrain_rows`: Seeds the random generator from a constant, then fills three 408-entry arrays with random values offset by fixed amounts.
- `clear_two_buffers_and_build_copper_lists`: Clears two 36864-byte buffers, then writes copper instruction pairs into two lists, storing move and wait opcodes with buffer addresses and incrementing the address by 12288 between entries.
- `set_bit_zero_of_d0`: ORs the immediate value 1 into the low byte of d0, setting bit zero, then falls through to the following code.
- `or_immediate_into_memory_and_d0`: ORs the immediate value 144 into the word at address a0, then ORs a large immediate into d0, and falls through to the next routine.
- `advance_three_stream_pointers_load_words`: For each of three stream pointers, if the pointed word equals minus thirty-two it resets the pointer to a shared table, then reads the next word into a destination and advances the pointer by two.
- `blit_text_glyphs_with_blitter`: Reads a text string and font table, looks up each character, and programs the custom blitter registers to draw glyphs into the bitplanes, advancing coordinates per character and line.
- `blitter_fill_data_table`: This range is a data table of zero words that the blitter reads by address; the decoded ori instructions change nothing and are not executable code.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `poll_fire_and_advance_script_pointer`: Reads the script pointer, checks the next two entries against two opcodes, calls the script interpreter, then polls CIAA_PRA fire buttons and runs the frame update until the counter expires.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `blit_three_planes_to_bitmap`: Sets up three bitplane pointers and blits a 36 by 41 block into the display bitmap using BLTCON0, BLTDMOD, BLTDPTH and BLTSIZE, advancing the source by 9600 bytes each pass.
- `blit_single_plane_block`: Blits one 36 by 41 bitplane block from a fixed source into the display bitmap via BLTCON0, BLTDMOD, BLTDPTH and BLTSIZE, advancing the source pointer by 9600 bytes.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `scale_d0_d1_seeded_random`: Adds d0 to d1, saves both to memory at a fixed longword pair, masks d0 low byte with 14 and sets bit 5, then doubles and accumulates the pair into a new value stored back and returned in d0.
- `increment_d0_by_one`: Adds one to the low word of d0 and falls through to whatever follows, with no return instruction of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `zero_word_data_table`: Fifteen ori.b zero instructions are data words decoded as code, a table referenced by address rather than executable code that changes nothing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `multiply_d0_by_three_plus_d1`: Copies d0 and d1 to d2/d3, doubles them with addx, adds the originals, swaps halves and stores the result pair to memory, returning d1 in d0.
- `increment_d0_modulo_divide`: Adds one to the low word of d0, calls a subroutine, then divides the swapped d0 by that value and returns the remainder in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `level_setup_sequence_with_dma_and_blits`: Clears state words, calls many setup routines, loads copper and blitter registers, copies graphics blocks and waits on a flag before returning.
- `jump_to_second_stage_entry`: Calls one routine then jumps to another address, transferring control onward without returning to the caller.
- `main_loop_keyboard_command_dispatch`: Runs the game main loop: calls many subsystem routines, reads CIAA_PRA fire buttons and keyboard scancodes, and branches to handlers that set flags, tweak object fields and restart the loop.
- `check_bit_in_object_flags`: Loads a word from offset 256 of a0, tests bit d0 of the word at offset 402, and either adds fourteen to the longword at a1 or clears it before returning.
- `save_stack_words_to_chip_ram`: Copies 50 longwords from the supervisor stack downward into chip memory at the address in a0, then installs a new level-1 trap vector pointing at the routine at offset 748 and issues trap 0.
- `restore_stack_words_from_chip_ram`: Sets a7 to the top of chip memory, copies 50 longwords from there into the buffer at a0, restores the old trap vector from a1, resets a7 and jumps to the startup code.
- `reset_two_script_stream_offsets`: Copies a shared script table into two channel buffers, then sets both stream offsets to fourteen and returns.
- `copy_seven_words_to_sprite_offsets`: Copies seven consecutive words from the table pointed to by a1 into fixed offsets in the structure pointed to by a0, then returns.
- `clear_seven_global_longwords`: Writes zero to seven consecutive global longword variables in memory and returns to the caller.
- `advance_and_compare_pointer_table`: Increments a counter word, stores a hardware word into the current table entry, compares adjacent longwords, clears the entry on match, otherwise advances the table pointer and checks the next entry's sign.
- `advance_script_pointer_and_fetch_word`: If the script-active flag is set, clears the audio register, decrements the delay counter and, when it expires, advances the script pointer by four, loads the next word into the audio register and reloads the delay, setting a stop flag on a zero word.
- `vertical_blank_interrupt_dispatch`: Saves all registers, checks the vertical blank and copper interrupt bits, then runs the per-frame game update: sound, sprite, timer, level and input routines, decrements counters, and acknowledges the interrupt before returning from the exception.
- `set_sound_mode_from_playback_flag`: Tests the sound-enabled word and returns if clear; otherwise sets the sound mode word to one, or to two when the playback status word is negative, then returns.
- `run_four_init_subroutines`: Calls four unnamed initialization subroutines in sequence via bsr.w, then returns to the caller.
- `clear_sixty_nine_kilobyte_buffer`: Writes zero bytes to a large memory buffer starting at a fixed address, decrementing a counter of twenty-seven thousand and ninety-six bytes until done, then returns.
- `build_seven_hundred_entry_pointer_table`: Fills a table of seven hundred longwords with increasing addresses spaced one hundred sixty bytes apart, starting from a fixed base value, then returns.
- `pointer_table_fill_data_words`: Data words decoded as code; the bytes are a table of values that other code refers to by address, not executable instructions.
- `fill_pointer_table_until_negative`: Loads a table address into a0 and a start value into d0, then stores d0 into each longword, adding 248 each time, stopping when a negative longword is found.
- `fill_second_pointer_table_with_stride`: Loads a second table address into a0 and value into d0, stores d0 into successive longwords adding 672 each time, stopping when d0 reaches the end value.
- `fill_word_table_with_step_16`: Loads the address of a word table into a0, then writes d0 starting at zero and increasing by 16 into successive words until d0 reaches 288, resetting d0 to zero and branching back to the test.
- `padding_data_table_ff_terminated`: Fifty-one no-operation ori.b zero instructions followed by one ori.b with immediate 255 on d0; the bytes are padding data, not executable code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `clear_globals_and_init_two_structures`: Clears four longword globals, then calls the same routine twice with a0 pointing at two different data blocks, calls another routine, and returns.
- `init_audio_and_clear_slot`: Writes two pairs of fixed pointers into four longword globals, calls a setup routine, then clears a word at offset 316 of the second buffer unless a mode word equals two.
- `reset_entity_state_block`: Using a0 as the structure base, clears and sets many word and long fields (counters, limits, flags, timers) and copies two words from offsets 300 and 302 into offsets 428 and 432.
- `initialise_level_and_display_state`: Clears two globals, sets a count and a mask, writes two bytes, installs five pointers into globals, loads four word arguments into d0-d3 and calls three subroutines.
- `init_level_one_state`: Clears several game state words, sets the level counter to 66, stores five graphics pointers, then calls three setup routines and sets a 500-frame timer before returning.
- `init_level_two_state`: Sets the level counter to 67, clears two state words, stores five graphics pointers, loads sprite coordinates into d0-d3, calls three setup routines and returns.
- `init_level_three_state`: Sets the level counter to 68, stores five graphics pointers, calls three setup routines, clears eight longwords of state memory and calls a final setup routine before returning.
- `init_level_1_state_and_assets`: Writes level identifier 69 and clears two words, sets timer 20 and size 768, stores five asset pointers, then calls three setup routines with d0-d3 set to 944, 800, 48 and 0.
- `init_level_2_state_and_assets`: Writes level identifier 70 and clears two words, sets timer 20 and size 1024, stores five asset pointers, then calls three setup routines with d0-d3 set to 368, 912, 32 and 16.
- `run_pending_callback_if_set`: Tests the longword at a fixed global; if zero it returns, otherwise it loads that pointer into a0, clears the global and jumps to the address in a0.
- `zero_filled_data_area_8`: A single zero word that decodes as an ori instruction; the bytes are data referenced by address, not executable code.
- `zero_fill_word_at_2422`: This single zero word decodes as an ori.b immediate instruction but is really zero-filled data referenced by address, not executable code.
- `clear_two_words`: Clears two 16-bit words in a global variable, moves two bytes from a global variable into registers, stores those bytes into two consecutive global variable locations, and branches to another routine.
- `advance_animation_frame_and_swap_bitmap`: Checks the animation enable word, sets up a source and destination bitmap pointer, calls the blit routine, advances the frame counter to 25 and cycles the two-byte frame index, then jumps to the display routine with two table pointers.
- `animation_frame_pointer_table`: Ten ori.b instructions whose immediate bytes form a table of longword pointers used as animation frame addresses, not executable code.
- `zero_filled_data_word`: A single zero word that decodes as ori.b with zero immediate; it is zero-filled data referenced by address, not executable code.
- `zero_fill_word_at_259e`: This lone zero word decodes as an ori.b immediate but serves as zero-filled data pointed to by an absolute move, not as code.
- `read_vposr_into_global`: Saves d0, reads the VPOSR hardware register, shifts and masks it to a vertical position value, stores it in memory, restores d0 and returns.
- `compute_vertical_position_delta`: Reads VPOSR, masks the vertical position, stores it, compares it with the previous value and stores the wrapped difference in memory before returning.
- `call_subroutine_with_data`: Saves d0 through d2 to the stack, loads a 32-bit value from a global variable into d0, calls another routine, and restores the registers before returning.
- `zero_filled_data_area_5`: A single zero word that decodes as ori.b #0,d0; this range is zero-filled data referenced by address, not executable code.
- `wait_for_vposr_line_300_countdown`: Polls VPOSR until the vertical position equals 300, then waits until it differs, decrements d0 and repeats until d0 reaches zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_player_sprite_and_copper_colors`: Reads two global position longs, derives tile and pixel offsets, writes a packed nibble byte to the sprite control register, computes a clamped vertical offset and writes copper color and sprite pointer registers for the player.
- `select_and_copy_player_sprite_frames`: Checks two object status words, compares two frame counters, picks one of two sprite frame tables, then calls the fill and copy routines to write sprite data into the player and background buffers.
- `fill_seventeen_sprite_words_with_blank`: Writes the value 152 into seventeen consecutive words starting at the address in a0, blanking a sprite or object record, then returns to the caller.
- `init_sprite_descriptor_table`: Stores d0 into the first byte at a0, sets byte one to 1, then writes sixteen word constants into offsets 4 through 64 of the a0 structure.
- `copy_sprite_words_skip_first`: Copies sixteen words from a0 offsets 2 through 62 into the same offsets of a1, leaving the first word of each structure untouched.
- `bit_test_and_set_data_bytes`: Tests bit d0 of the word before a0, then ORs immediate bytes 34 and 64 into d0; the bytes are data, not executable code.
- `ori_immediate_sequence_data_table`: Eleven ori.b immediate instructions with fixed byte values; the bytes are data decoded as code, not an executable routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_audio_interrupts_and_call_init`: Enables interrupt bit 14 in INTENA, restores registers from the stack, loads address 512 into a2, calls two library routines, stores the result, then disables that interrupt and returns.
- `loop_clear_and_draw_rows`: Saves registers, calls a setup routine, then loops over d7 rows calling a draw routine with a3 advancing by 512 bytes and d6 incrementing, finishing with a cleanup call and register restore.
- `verify_disk_block_checksum`: Checks a block index against 1760, then loops up to five times reading two 512-byte blocks, XOR-combining masked words into a buffer and comparing checksums, returning zero on match or minus one.
- `xor_words_into_checksum`: Saves d1, d2 and a0, computes a word count from d0 shifted right twice minus one, XORs that many longwords from a0 into d0, masks with d7 and returns.
- `select_and_store_disk_track`: Saves registers, compares the current track byte with the requested one, writes a drive command to hardware, sends the track word, reads back the result and stores the new track and offset.
- `scan_entity_table_match_id`: Saves registers, scans a table of word IDs at the base pointer for a match with d6, and on a match stores the adjusted entry pointer into a table indexed by a value returned from the decode routine.
- `decode_packed_pair_bits`: Reads two longwords from the address in a0, masks both with d7, doubles the first, ORs in the second, and returns the combined value in d0 while preserving d1.
- `program_blitter_shift_and_wait`: Writes blitter control, data and pointer registers via a6, starts a shifted blit, then polls the status register until bit 2 clears or a counter expires, restoring the control word.
- `prepare_cia_disk_and_custom_regs`: Loads custom chip base, CIA-B and CIA-A addresses into a6, a5 and a4, reads the vector at address 1012 into a3, sets d7 and d6 to fixed constants, then sets bits in CIA-A port A and returns.
- `reset_disk_and_wait_for_ready`: Saves registers, calls the CIA setup routine, drives CIA-B port B low, clears bit 7, waits in a delay loop, then either sets bit 3 or calls the follow-up routine before restoring registers and returning.
- `save_regs_and_start_disk_reset`: Pushes d0-d7 and a3-a6, calls the CIA setup routine, writes a low value to CIA-B port B, sets bit 7 there, then branches into the disk wait code.
- `wait_while_a4_bit5_set`: Reads the byte at a4 and branches back to itself while bit 5 is set, then returns to the caller.
- `set_a5_bit1_wait_a4_bit4_then_delay`: Sets bit 1 of the byte at a5, calls a routine while bit 4 of the byte at a4 is set, then busy-waits with d0 counting down from 49152 and clears the word at the absolute address before returning.
- `update_sound_state`: Saves d0 through d4 to the stack, checks a global word, sets or clears bit 2 of a byte at a pointer in a5, and calls various routines based on comparisons and bit tests.
- `decrement_and_store`: Runs a loop 48832 times, stores the value from d2 into a global variable, and restores registers from the stack before returning.
- `decrement_sound_channel_counter`: Compares the word at the sound counter to 160; if below, clears bit 1 of the byte at (a5), calls the sound update routine, then adds 2 to the counter and returns.
- `increment_sound_channel_counter`: Compares the word at the sound counter to 160; if below, sets bit 1 of the byte at (a5), calls the sound update routine, then subtracts 2 from the counter and returns.
- `wait_for_vblank_and_update_sound`: Saves d0, toggles bit 0 of (a5), writes 100 and 8 to two a5 offsets, clears and polls the vertical blank flag, then sets bit 0 and calls the sound routine before restoring d0.
- `poll_input_and_dispatch_command`: Checks several globals and a countdown, reads a byte from a fixed address, stores it via a pointer, scans a command table comparing bytes, then calls the matching handler and the system call at a fixed address.
- `clear_command_buffer_words`: Stores the address a fixed address into a fixed address, loads a6 with that address and clears 64 words starting there, then returns.
- `translate_string_via_table`: Walks a list of pointers to strings, replaces each byte through the translation routine, and advances until a negative terminator byte and a negative pointer are reached.
- `lookup_char_in_translation_table`: Scans a byte table at a5 comparing each entry with d0, and returns the matching byte from the parallel table at a6, or the last entry if none match.
- `install_interrupt_vector_and_flag`: Writes the value 1 to a hardware register and stores the address of a code block into a vector slot, then returns.
- `poll_two_custom_chips_and_patch_vectors`: Tests the high word at two custom chip registers; if either is negative, writes the same long pointer to two custom chip vector locations, then returns.
- `set_dma_and_copper_list_pointers`: Writes the DMA control register to enable channels, loads two copper list pointers and a copper jump value, stores d1 to a custom register, then returns.
- `advance_shared_seed_with_carry_math`: Loads a shared long seed, masks and sets bits, doubles it with carry propagation, adds it back, stores the updated seed and returns the high long in d0.
- `increment_d0_divide_by_new_value`: Adds one to the low word of d0, copies it to d2, calls a subroutine if nonzero, then divides the swapped d0 by d2 and returns the remainder in d0.
- `store_incremented_d0_to_global`: Writes d0 into a5, ORs an immediate into memory before a0, adds an immediate to memory at a4, adds d0 to d4, increments d0 and stores it to a fixed global address.
- `loop_call_until_d0_equals_one`: ORs zero into d0, repeatedly calls a subroutine and decrements d0 until it becomes zero, then returns.
- `frame_save_call_update_and_restore`: Saves all data and address registers on the stack, calls the update routine, stores its result in a global, and if a flag is set calls three further routines before restoring registers and returning.
- `wrap_angle_until_within_twelve`: Repeatedly calls the angle fetch routine, compares the returned word with the stored angle, adds 310 when below it, and loops until the difference is at least 12.
- `zero_filled_data_word_2`: A single zero word that decodes as ori.b with immediate zero; it is zero-filled data referenced by address, not executable code.
- `read_vposr_vertical_position`: Reads the VPOSR hardware register, shifts the value right by 8 bits and masks it to 9 bits, returning the vertical beam position in d0.
- `init_sound_channels_and_clear_counters`: Writes the same longword value to five sound chip register pairs and clears five word counters in memory, then returns.
- `copy_graphics_and_init_display`: Calls two library routines, copies 2152 words from one memory block to another, sets a display flag, stores a value and jumps onward to another routine.
- `reset_state_and_dispatch_via_vector`: Clears a status longword, selects one of two object tables by sign, calls setup and dispatch routines, stores returned pointers and returns.
- `save_d2_and_call_conditional_update`: Stores d2 to a memory longword, tests a status word and, when zero, calls two library routines and a local update before returning.
- `load_script_pointer_and_call_loader`: Loads a0 with the address 70472, sets d0 to 22 and d1 to 82, then calls the loader at 6e6 and returns.
- `reset_level_state_and_restore_scroll`: Calls the sound setup, clears several game state words, saves and clears two object scroll offsets, plays a sound, then restores saved scroll positions and writes back the level and scroll state variables.
- `call_audio_then_free_a1`: Pushes a1, calls the audio routine at a fixed library address, restores a1, copies it to a0, pushes a0, calls the free routine, restores a0 and returns.
- `call_second_audio_then_free_a1`: Pushes a1, calls a different audio routine at a fixed library address, restores a1, copies it to a0, pushes a0, calls the free routine, restores a0 and returns.
- `wait_for_raster_line_3`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `start_copper_list_2`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `init_sprite_dimension_tables`: Writes constant width, height, offset and stride values into several global parameter blocks, then returns to the caller.
- `copy_player_sprite_dims_and_run_script`: Copies sprite dimension words from two player structures into globals, clamps them to six, then steps through a script table calling entries until five run or a flag stops it.
- `ori_byte_immediate_into_d0`: Performs a bitwise OR of the immediate value 64 with the low byte of data register d0, then falls through into following code.
- `ori_byte_8e_into_d0`: Performs a bitwise OR of the immediate byte 142 into the low byte of data register d0, leaving the upper bits unchanged.
- `ori_byte_c4_into_d0`: Performs a bitwise OR of the immediate byte 196 into the low byte of data register d0, leaving the upper bits unchanged.
- `ori_byte_20_into_d0`: Performs a bitwise OR of the immediate byte 32 into the low byte of data register d0, leaving the upper bits unchanged.
- `set_d0_bits_1e`: ORs the low byte of d0 with the immediate value 30, setting those bits in d0.
- `set_d0_bits_40`: ORs the low byte of d0 with the immediate value 64, setting those bits in d0.
- `set_d0_bits_04`: ORs the low byte of d0 with the immediate value 4, setting those bits in d0.
- `set_bit2_in_d0_byte`: ORs the low byte of d0 with the immediate value 4, setting bit 2, then falls through to the next routine.
- `set_bit5_in_d0_byte`: ORs the low byte of d0 with the immediate value 32, setting bit 5, then falls through to the next routine.
- `set_bits_in_d0_byte_then_padding`: ORs the low byte of d0 with 6, then executes five further ORs with zero that leave d0 unchanged before falling through.
- `set_d0_bits_0x40`: ORs the immediate value 64 into the low byte of d0, setting bit 6 and leaving the rest of d0 unchanged.
- `set_d0_bits_0x8e`: ORs the immediate value 142 into the low byte of d0, setting bits 1, 2, 3 and 7 of that byte.
- `set_d0_bits_0xc4`: ORs the immediate value 196 into the low byte of d0, setting bits 2, 6 and 7 of that byte.
- `set_bit5_in_d0_low_byte`: ORs the low byte of d0 with the immediate value 32, setting bit 5 and leaving the rest of d0 unchanged.
- `set_bits1_to4_in_d0_low_byte`: ORs the low byte of d0 with the immediate value 30, setting bits 1 through 4 and leaving the rest of d0 unchanged.
- `set_bit6_in_d0_low_byte`: ORs the low byte of d0 with the immediate value 64, setting bit 6 and leaving the rest of d0 unchanged.
- `set_bit2_in_d0_low_byte`: ORs the low byte of d0 with the immediate value 4, setting bit 2, then falls through to the next instruction.
- `set_bit2_in_d0_low_byte_alt_entry`: ORs the low byte of d0 with the immediate value 4, setting bit 2, entered at a separate address and falling through onward.
- `set_bit5_in_d0_low_byte_2`: ORs the low byte of d0 with the immediate value 32, setting bit 5, then falls through to the following instruction.
- `clear_bitplane_pixel_column`: Uses d0 as a bit index to build a mask and clears that bit in eight bytes spaced 38 bytes apart in the bitmap pointed to by the absolute address, touching d0 and memory.
- `zero_filled_data_area_6`: This single zero word decodes as an empty instruction; the bytes are a zero-filled data area referenced by address, not executable code.
- `clear_bitplane_pixel_column_with_base`: Loads the bitmap base address into a1, computes byte offset and bit mask from d0, then clears that bit in eight bytes spaced 38 bytes apart and returns.
- `clear_flag_and_set_ffff_word`: Clears the byte at a fixed global address and writes the word value 65535 to another fixed global, then returns.
- `compare_and_adjust_color_nibbles`: When the mode word is one and the flag is clear, increments a counter and, once it reaches the limit, compares three nibbles of each entry against a reference, adding per-nibble deltas and counting matches before calling a routine.
- `accumulate_nibble_deltas_into_table`: When the mode word is two and the flag is clear, increments a counter and, once it reaches the limit, adds per-nibble deltas from a source table into a destination table, decrementing a remaining count for each mismatch before calling a routine.
- `unpack_3plane_tiles_to_nibbles`: Clears 48 longs at a fixed buffer, then converts each of d0 tile words from a0 into three nibble values stored in a table, and finally stores d0, a0 and a zero word in globals before returning.
- `advance_and_apply_tile_deltas`: When the mode word is 3 and the done flag is clear, increments a counter, and once it reaches a limit subtracts per-tile deltas from the nibble table, updates the source tile words, and calls a routine when all tiles are consumed.
- `mode_word_ori_b_2_data`: This single instruction ORs the immediate value 2 into the low byte of d0; it is a data word reached by absolute addressing rather than a real routine.
- `zero_filled_data_word_at_56e4`: A single zero word that decodes as ori.b with immediate zero to d0; it is data referenced by address, not executable code.
- `zero_filled_data_word_at_56e8`: A single zero word decoding as ori.b immediate zero to d0; this is zero-filled data pointed to by movea references, not code.
- `zero_filled_data_word_at_56ec`: A single zero word that decodes as ori.b immediate zero to d0; it is zero-filled data reached by move and movea references, not code.
- `padding_ori_b_zero_table`: Twenty-four ori.b instructions with zero immediate followed by rts; the bytes are padding data, not executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `wait_flag_then_enable_interrupts`: Tests the word at a fixed memory location, branches away if it is nonzero, otherwise writes the interrupt enable value to the interrupt enable register and returns.
- `keyboard_interrupt_ack_and_serial_read`: Checks a flag word and returns early if set; otherwise disables the level 2 interrupt, saves all registers, reads the CIAA serial keyboard register, stores the decoded key code into one of two memory bytes, then reads the CIAA interrupt control register and acknowledges the keyboard interrupt.
- `reset_game_state_if_flag_set`: Tests the word at a fixed address and, if nonzero, clears several game state words and bytes, sets a few bytes to 1 and 6, calls two subroutines, and sets the word at a fixed address to 1.
- `stop_audio_and_reset_sound_state`: If the word at a fixed address is nonzero, clears a fixed address, calls a routine four times with d0=76 and d2=0..3, writes 12 to a fixed address, calls a fixed address, clears a fixed address, calls a routine, sets four audio period registers to 2, enables DMA with 15, and clears several sound state words.
- `disable_audio_dma_and_interrupts`: If the word at a fixed address is zero, reads a fixed address into d0, writes it to DMACON, and if bit 3 is set writes 1024 to INTREQ and 33792 to INTENA, then clears a fixed address.
- `zero_word_data_slot`: Contains a single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_word_data_slot_second`: Contains a single zero word that decodes as ori.b #1,d0; it is zero-filled data referenced by address, not executable code.
- `check_player_zone_and_spawn_enemy`: Tests game state flags and player position bounds, then increments a counter and calls the spawn routine with parameters 39 or 76 when conditions match.
- `queue_sound_sample_for_playback`: Checks the audio-ready flag, rejects duplicate or currently playing sample pointers, derives a sample index from the audio hardware register, calls the sample setup routine and stores the new sample pointer.
- `clear_audio_buffer`: Clears a block of memory using a loop, sets a global variable to a constant, and updates another global variable with a pointer if a condition is met.
- `audio_interrupt_sample_mixer`: Handles the level 3 audio interrupt by checking INTREQR, copying sample data into the audio chip registers, updating playback pointers and lengths, and acknowledging the interrupt.
- `copy_sample_block_to_aud3_and_start`: Disables audio interrupt, copies up to 512 bytes from a0 into the audio buffer, sets AUD3 pointer and length, enables the interrupt and stores playback state.
- `zero_filled_data_area_word`: Single zero word that decodes as ori.b #0,d0; it is zero-filled data referenced by address, not executable code.
- `zero_filled_data_area_word_2`: Another single zero word decoding as ori.b #0,d0; zero-filled data referenced by address rather than a real routine.
- `zero_filled_data_area_9`: This single instruction is a zero word that decodes as an ori.b immediate, so the range is zero-filled data referenced by address rather than executable code.
- `clear_pointer_and_flag_block`: Clears ten longword pointer slots and two words in the global state block, then returns; the leading zero word is data padding.
- `claim_state_slot_or_clear_flag`: Tests a global longword; if zero, stores a6 into two globals, clears a word and sets a flag word to one, otherwise clears the flag word.
- `zero_filled_data_word_first`: A single zero word that decodes as ori.b with immediate zero on d0; it is data referenced by address, not executable code.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `zero_word_data_at_15fe`: One zero word at this address, decoding as ori.b #0,d0; it is part of a zero-filled data area, not a real routine.
- `zero_word_data_at_1602`: This address holds a lone zero word read as ori.b #0,d0; the bytes are data referenced by address, not code.
- `zero_word_data_at_1606`: Another single zero word decoding as ori.b #0,d0; this is zero-filled data pointed to by address, not a real routine.
- `clear_pending_audio_pointer_and_flag`: Loads the longword at the audio pointer slot into a6, and if nonzero clears that slot and branches away; otherwise returns immediately.
- `advance_animation_frame_chain`: Walks the animation node at the global pointer, increments its frame counter, and when the frame count exceeds the node's limit calls the frame handler then follows the next-node pointer or clears the animation globals.
- `dispatch_input_code_to_animation`: Checks the input lock word, saves all registers, reads the pending input code and branches on its value to select an animation or skip, then falls through to the shared frame advance.
- `apply_input_animation_step`: Reads the pending input code and step counter, clears the step when it reaches two, sign-extends both values and calls the animation step routine, then increments and wraps the step counter before restoring registers.
- `dispatch_key_event_by_d0_and_d2`: Checks a global flag, handles key code in d0 and mode in d2, stores d0 into two global words, then calls another routine with registers saved.
- `perform_bitwise_or`: Performs a bitwise OR operation on register d0 with zero.
- `key_code_data_word`: A single word holding the value 66 that decodes as ori.b #66,d0; it is a data word referenced by address, not executable code.
- `configure_audio_channel_from_table`: Selects one of four audio channels from d2, looks up its parameter block in a table, programs the channel's location, length, period and volume registers, and enables the matching DMA bit.
- `audio_channel_zero_data_table`: Two zero words that decode as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `zero_word_data_at_244e`: One zero word referenced by an absolute move; the bytes are padding data rather than a real instruction sequence.
- `zero_word_data_at_2452`: A lone zero word whose address is taken by an absolute move; it is filler data, not a working routine.
- `zero_word_data_at_2456`: This single zero word decodes as an ori.b immediate instruction but is zero-filled data referenced by address, not executable code.
- `zero_word_data_at_245a`: A zero word in the zero-filled data area, referenced by two routines via absolute move and negate, not real code.
- `zero_filled_data_word_at_245e`: A single zero word that decodes as ori.b #0,d0; it is data referenced by address, not executable code.
- `zero_filled_data_pair_at_2462`: Two consecutive zero words decoding as ori.b #0,d0; this is zero-filled data referenced by address, not code.
- `zero_filled_data_word_at_246a`: One zero word decoding as ori.b #0,d0; it is part of a zero-filled data area reached by address, not a routine.
- `zero_word_data_at_246e`: Another zero word inside the zero-filled data region, reached only through an absolute address reference from one routine.
- `zero_word_data_at_2472`: Another single zero word decoding as ori.b #0,d0; a zero-filled data slot reached by address from one caller.
- `data_word_ori_7c_at_2476`: One data word decoding as ori.b #124,d0; referenced by two callers as an address, it is table data rather than code.
- `update_four_audio_channel_voices`: Scans four 14-byte voice records, decrements duration counters, and writes pitch and volume words to the four audio channel registers when envelopes are active.
- `stop_voice_and_signal_dma_done`: Clears the voice active byte, then depending on d1 clears a status word and writes a DMA or interrupt control value to signal completion.
- `silence_all_audio_volumes`: Writes zero to the four audio channel volume registers and sets the DMA control register to disable audio DMA channels.
- `build_audio_channel_sample_pointers`: Reads a module header at a fixed address, finds the largest sample length, computes a base pointer into sample data and stores fourteen per-instrument sample pointers into a table at a fixed address.
- `release_four_audio_channel_buffers`: Scans four channel slots, and for each non-empty slot clears its pointer and copies eight longwords from the slot into the pointed buffer, zeroing the source.
- `per_frame_audio_channel_mixer`: Checks two enable flags, calls a setup routine, then for four DMA audio channels updates period, volume and sample pointers and calls volume helpers, restarting playback when the frame counter expires.
- `update_four_audio_channels_from_song_data`: Reads song pattern bytes from the module table, updates four audio channel structures and CIAA port A fire bits, and advances the pattern position counters.
- `spawn_entity_from_type_table`: Reads entity type byte at offset 3 of a2, indexes the type table at a3, copies eight longwords from a5 into the entity, sets up animation pointers and speeds in a1 and a2, and writes d1 with bit 15 set to DMACON.
- `load_period_word_into_channel_six`: Sign-extends d4, doubles it as a word index into the period table at a fixed address, and stores the resulting period word at offset six of the channel structure pointed to by a1.
- `update_four_audio_channels`: Loops over the four Amiga audio channel registers, checks per-channel enable flags and calls the channel update routine for each active channel, advancing the channel, config and state pointers.
- `advance_entity_animation_frames`: Steps the three animation counters at offsets 24, 22 and 20 of a2, reads frame bytes from the table at a3 indexed by the type, updates a1 offsets 8 and 6, and shifts or negates bytes in the a5 buffer.
- `decompress_bitstream_to_buffer`: Reads a bitstream from the source pointer, decodes literal and back-reference tokens, and writes the expanded bytes backwards into the destination buffer using the a0 structure fields.
- `rotate_right_through_extend_long`: Pops a longword from the stack pointed to by a0 into d0, sets the condition codes to extend, rotates d0 right through extend by one bit and returns.
- `decompress_rle_bitstream_to_buffer`: Reads a bit stream from a1, expands run-length encoded bytes into the buffer at a6 bounded by a2, using d0-d7 and a0-a3, and returns zero on success or minus one when the source is exhausted.
- `relocate_hunk_segments_by_type`: Validates the hunk header value, walks the segment table building relocation entries on the stack, then calls the per-hunk relocation routine for each segment until the terminating hunk is reached.
- `branch_to_38ec`: Branches unconditionally to the code at offset 38ec, transferring execution without touching any registers or memory.
- `branch_to_38fc`: Branches unconditionally to the code at offset 38fc, transferring execution without touching any registers or memory.
- `branch_to_shared_continuation`: A single unconditional branch instruction that jumps forward to a shared continuation point, transferring control without touching any registers or memory.
- `branch_to_392a`: Performs an unconditional branch to the code at offset 392a, transferring execution without touching any registers or memory.
- `branch_to_38f8`: Performs an unconditional branch to the code at offset 38f8, transferring execution without touching any registers or memory.
- `branch_to_394a`: Performs an unconditional branch to the code at offset 394a, transferring execution without touching any registers or memory.
- `skip_a0_by_quadrupled_longword`: Reads a longword from (a0), doubles it twice, adds the result to a0 to skip that many bytes, sets d0 to 1 and returns.
- `copy_longwords_then_zero_fill`: Reads a count from (a0), copies that many longwords from (a0) to the buffer at 4(a5), then zero-fills the remaining longwords counted by (a5), returning 1 in d0.
- `zero_buffer_then_add_offset_table`: Zeroes a count of longwords at 4(a5), then for each entry reads an index, scales it by eight, adds the stack-frame longword to the target buffer, and returns 1 in d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_copper_list_and_init_registers`: Writes eight longword constants into memory starting at address 512, then loads d0 through d7 with fixed values and returns.
