# Game 2821 (barbarian ii (1991)(psygnosis)(m4)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02821). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3635824 top-level bytes in 139 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 21 of 21 units identical (bytes and relocations), 68 raw-word lines |
| Functions named and described | pass, 177 of 177 named (0 author symbols), 177 of 177 described (39 by rule, 108 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3132 of 3132 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 583548 bytes, 49 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9338 of 9338 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 517926 |
| code | 26090 |
| data | 2692080 |
| padding | 17264 |
| unused | 382464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf:bootblock | 1 | 22 | 1 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:bootblock | 1 | 10 | 1 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:bootblock | 1 | 10 | 1 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf:bootblock | 1 | 221 | 1 |
| logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf boot-block load | 1 | 136 | 5 |
| logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf boot-block load | 1 | 94 | 3 |
| logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf boot-block load | 1 | 2234 | 17 |
| logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1597 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1362 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 75 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 104 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1511 | 26 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address (raw code file, load address unknown) | 1 | 122 | 9 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address (raw code file, load address unknown) | 1 | 352 | 17 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf:blocks 264-1306 (bytes 135168-669184)#unpacked (raw code file, load address unknown) | 1 | 1048 | 23 |

## Cases handled

- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1727 non-empty blocks: matched against track-loaded segments
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 4 of 4)(intro)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (16004 -> 33748 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (81004 -> 137742 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (9748 -> 21032 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (55560 -> 82444 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (12612 -> 19962 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (5116 -> 8024 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (3224 -> 4918 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (5864 -> 12472 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (4212 -> 7516 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (28628 -> 40064 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (9016 -> 16368 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (9496 -> 16752 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (146756 -> 203906 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (780 -> 2534 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (1852 -> 3052 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (11964 -> 21240 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (22996 -> 44056 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (2204 -> 3484 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (12668 -> 22692 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (60292 -> 86530 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (10496 -> 19476 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (5440 -> 9344 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (3596 -> 6500 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (2676 -> 3784 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (14256 -> 33164 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (51588 -> 96826 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (7004 -> 12582 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (3496 -> 6848 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (42184 -> 82482 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (14680 -> 23122 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (36024 -> 59996 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (13056 -> 27872 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (3024 -> 6036 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (1732 -> 2926 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (7640 -> 13316 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (900 -> 2120 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (27216 -> 62142 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (12536 -> 21588 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (19256 -> 50784 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (12968 -> 22232 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (24208 -> 40332 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (1620 -> 3244 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (24352 -> 54476 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (23988 -> 47584 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (3264 -> 9304 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (46732 -> 83760 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (50112 -> 77922 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (3688 -> 7798 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (84904 -> 135196 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (25748 -> 47072 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: ByteKiller Data (xfd)-packed, unpacked with ancient (59732 -> 95800 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (14732 -> 28412 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (34716 -> 65592 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:an address: PowerPacker Data (xfd)-packed, unpacked with ancient (9540 -> 18088 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: the boot block reads 8704 bytes from disk offset 6144 to an address (found statically); recovered as a unit
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: the boot block reads 4096 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 4 of 4)(intro)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf: the boot block reads 405504 bytes from disk offset 450560 to an address (found statically); recovered as a unit
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 4 of 4)(intro)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf: the boot block reads 512 bytes from disk offset 0 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 578 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 905 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 73 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 95 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1469 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 2 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying code at an address (38 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (49 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (11 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (48 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: 2 logged loads cover 12800 bytes of the image
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: blocks 10-11 (bytes 5120-6144) hold ASCII text
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: blocks 264-1306 (bytes 135168-669184) hold a BK: ByteKiller stream (534016 -> 377800 bytes)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf:blocks 264-1306 (bytes 135168-669184)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: blocks 1308-1468 (bytes 669696-752128) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.20)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: blocks 1476-1517 (bytes 755712-777216) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.31)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: blocks 1524-1664 (bytes 780288-852480) hold ASCII text
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 1 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf: blocks 1671-1759 (bytes 855552-901120) hold ASCII text
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 3 of 4)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf: blocks 971-971 (bytes 497152-497664) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.19)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 4 of 4)(intro)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf: 2 logged loads cover 406016 bytes of the image
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 4 of 4)(intro)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf: blocks 2-879 (bytes 1024-450560) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.46)
- barbarian ii (1991)(psygnosis)(m4)[cr skr](disk 4 of 4)(intro)/Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf: blocks 1672-1759 (bytes 856064-901120) hold ASCII text
- 2 never-loaded region(s), 581706 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 116224 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_screen_coordinates` (runtime segment an address): Initializes screen coordinate values by performing division operations and storing results in a table structure, then it sets bits in a hardware register. Kind: disk hardware check. Pass path: tst.l d0; bmi.b an address (first call site) or tst.l d0; bmi.b an address (second call site) or tst.l d0; bmi.b an address (third call site) - the branch is taken when check passes, indicating successful hardware verification.
- `calculate_screen_coordinates` (runtime segment an address): Processes input data to compute screen coordinate values by performing division operations and storing results in a table structure, then iteratively updates coordinates while comparing memory locations for equality. Kind: disk hardware check. Pass path: The routine reads CIAB_PRB (disk hardware port) and uses its value in calculations, suggesting a check against physical disk drive state or hardware-specific data. The call from sub_1342 followed by setup of a4 and bit manipulation suggests.
- `decompress_bitstream_data` (runtime segment an address): Scans a buffer for a specific marker, performs bitwise transformations and XOR operations to expand compressed data into a destination table using multiple loop iterations. Kind: disk hardware sync/check. Pass path: The routine completes successfully without triggering any failure conditions, returning normally after processing all tracks and verifying checksums..
- `frag_move_ori_rts` (an address (raw code file, load address unknown)): Short code fragment that copies a fixed memory address to a fixed memory address (4 times in a row), then copies 16 to a fixed memory address, then copies 2 to a fixed memory address, then copies 32767 to a fixed memory address, then copies 33024 to a fixed memory address, then merges 120 into a fixed memory address, then returns. Kind: disk hardware check. Pass path: The routine sets up hardware registers and manipulates CIAB_PRB, suggesting a check that verifies disk hardware state or presence of specific hardware (like a color card or manual wheel) before proceeding. The 'pass' would be if the hardwar.
- `pulse_port_bit_then_delay` (an address (raw code file, load address unknown)): Clears bit 7 of a global CIA port image, runs a fixed countdown of one hundred iterations that clears bit 3 on each pass, then calls one further routine. Kind: disk hardware check. Pass path: CIAB_PRB is read and modified to clear bit 7, then a delay loop runs with bit 4 cleared each iteration. If the check passes, execution continues to u17__sub_0082()..
- `wait_while_port_bit_set` (an address (raw code file, load address unknown)): While bit 4 of a global status byte is set, sets the low two bits to three and calls one further routine, touching the CIA port A and port B registers. Kind: disk hardware check. Pass path: the loop exits when bit 4 of CIAA_PRA is cleared (i.e., the disk motor is off), allowing execution to continue.
- `wait_for_port_bit_and_pulse` (an address (raw code file, load address unknown)): Waits for a specific bit in a hardware port to clear, then performs a sequence of bit clearing and setting operations on another port while counting down. Kind: disk hardware check. Pass path: The routine waits for a bit in CIAB_PRB to clear (bit 4, as per bclr.b #an address, an address.l), then pulses the port to trigger a disk operation. If the check passes, execution continues normally after the call..
- `configure_disk_controller` (an address (raw code file, load address unknown)): Initializes disk controller registers by clearing specific bits, setting control words, and waiting for status bits to indicate completion before proceeding with further operations. Kind: disk track check. Pass path: The routine returns normally (via 'return' instruction) when the check passes, indicating successful completion of the disk hardware verification..
- `initialize_dma_and_interrupts` (an address (raw code file, load address unknown)): Sets up DMA and interrupt control registers by setting specific bits in various hardware registers, including enabling DMA channels and interrupts. Kind: disk hardware check. Pass path: bra.w an address (branch taken on successful hardware check).

## Functions


### Disk-Validator

- `initialize_game_state`: Sets up initial game parameters including screen buffer pointers, counter values, and memory mappings.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf:bootblock

- `initialize_audio`: Sets up audio registers and performs DMA initialization by configuring volume levels, enabling DMA channels, and setting interrupt masks.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 3 of 4).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf:bootblock

- `boot_block_initialization`: Performs boot block initialization by setting up I/O requests, calling system functions, and copying data from memory locations to various registers and structures.

### logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf boot-block load

- `shift_and_process_bits_2`: Performs bit shifting operations on a longword value while decrementing a counter, handling carry propagation through multiple iterations.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by 1, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `shift_and_process_bits`: Performs bit shifting operations on a longword value while decrementing a counter, handling carry propagation through multiple iterations until the counter reaches zero.
- `invert_word_values`: Inverts the value in register d6 a total of 32 times using the not.w instruction, effectively toggling all bits in the word register.

### logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 4 of 4)(Intro).adf boot-block load

- `copy_init_data`: Copies 645 bytes of initialization data from program memory to a working buffer starting at address 4096, then sets up a jump target and calls another routine.
- `copy_and_process_data`: Copies bytes from one memory location to another while processing data based on specific conditions and updates pointers and counters during the operation.
- `frag_adda_cmpa_bgt_movea_bra`: Short code fragment that adds an address to a0, then compares a1 with a4, then branches if the comparison was greater, then copies a1 to a0, then adds a location in the code to a0, then jumps to another point in the code.
- `copy_and_process_data_2`: Copies data bytes from a source buffer to a destination while processing control codes, updating pointers and counters during the operation.
- `frag_moveq_bsr_tst_beq_addi_bra`: Short code fragment that loads 8 into d1, then calls another routine, then tests d2, then branches if the last result was zero, then adds 10 to d2, then jumps to another point in the code.
- `frag_moveq_bsr_tst_beq_addi_bra_2`: Short code fragment that loads 12 into d1, then calls another routine, then tests d2, then branches if the last result was zero, then adds 265 to d2, then jumps to another point in the code.
- `process_data_chunk_2`: Processes a data chunk by copying bytes from a source buffer to a destination, updating pointers and counters during the operation, and handles conditional branching based on bit tests.
- `frag_lsr_bne_bsr_bcs_moveq_bra`: Short code fragment that shifts d0 right by 1, then branches if the last result was not zero, then calls another routine, then branches if the carry flag is set, then loads 0 into d5, then loads 3 into d1, then jumps to another point in the code.
- `frag_lsr_bne_bsr_bcs_moveq_bra_2`: Short code fragment that shifts d0 right by 1, then branches if the last result was not zero, then calls another routine, then branches if the carry flag is set, then loads 72 into d5, then loads 6 into d1, then jumps to another point in the code.
- `process_data_chunk`: Processes a data chunk by copying bytes from a source buffer to a destination, updating pointers and counters during the operation, and handling conditional logic based on byte values.
- `frag_move_roxr_rts`: Short code fragment that copies the memory just below a0 (moving a0 down) to d0, then copies d0 to d7, then copies d3 to the memory at a5, then copies 16 to the condition codes, then rotates d0 right through the extend flag by 1, then returns.
- `frag_moveq_bsr_add_bra`: Short code fragment that loads 4 into d3, then loads 1 into d1, then calls another routine, then adds d2 to d3, then loads 7 into d1, then jumps to another point in the code.
- `frag_subq_moveq_lsr_bne_bsr_addx_dbra_r`: Short code fragment that subtracts 1 from d1, then loads 0 into d2, then shifts d0 right by 1, then branches if the last result was not zero, then calls another routine, then adds d2 and the extend flag to d2, then loops back while counting down d1, then returns.
- `frag_moveq_bsr_move_addi_bra`: Short code fragment that loads 8 into d1, then calls another routine, then copies d2 to d3, then adds 14 to d3, then loads 7 into d1, then jumps to another point in the code.
- `frag_lsr_bne_bsr_bcs_moveq_bra_3`: Short code fragment that shifts d0 right by 1, then branches if the last result was not zero, then calls another routine, then branches if the carry flag is set, then loads 2 into d3, then loads 7 into d1, then jumps to another point in the code.
- `frag_moveq_bsr_move_dbra_cmpa_blt_bra`: Short code fragment that loads 1 into d3, then loads 7 into d1, then calls another routine, then copies an indexed entry from a2 to the memory just below a2 (moving a2 down), then loops back while counting down d3, then compares a1 with a2, then branches if the comparison was less, then jumps to another point in the code.
- `update_global_variables`: Updates global variables with bitwise operations involving input values, memory locations, and hardware register contents, performing complex bit manipulation logic.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_fill_table`: Clears a global variable table of 128 entries and fills a memory table with sequential values starting from zero, incrementing by two hundred each step, for a total of 199 iterations.
- `fill_memory_table`: Fills a memory table with sequential values starting from zero, increments by two hundred each step, for a total of 199 iterations.
- `copy_and_update_pointers`: Copies word values from a source area to a destination area with offset, updates pointers in global variables, and manages a counter that decrements until zero.
- `initialize_and_loop_2`: Initializes a destination pointer to a fixed address, sets up a counter, and performs a loop that copies data and updates the counter until it reaches zero.
- `frag_clr_bclr_beq_bsr_cmpi_bcs_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then clears bit 0 of a fixed memory address, then branches if the last result was zero, then calls another routine, then compares a fixed memory address with 200, then branches if the carry flag is set, then returns.
- `check_and_clear_flags`: Checks three global variables for zero value and clears corresponding flags in other global variables, then branches to a common routine.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `process_input_loop`: Processes input events in a loop, checking conditions based on a global variable and updating a counter, then calls a function pointer from the global variable table and returns a value from memory.
- `initialize_blitter`: Sets multiple blitter hardware registers to specific values including control and dimension parameters, then clears several global variables before returning.
- `process_input_events`: Processes input events in a loop, checking conditions based on a global variable and updating a counter, then calls a function pointer from a table.
- `clear_blitter_registers`: Clears blitter-related hardware registers by writing zero values to multiple register locations, then returns immediately.
- `return_constant`: Returns the constant value 95 directly without performing any operations on input parameters or global state.
- `rotate_and_store`: Performs a series of bitwise right rotate operations on a counter value, stores the result into memory at offset 2048, and returns a modified version of the counter.
- `frag_move_mulu_addi_rts`: Short code fragment that copies a fixed memory address to d0, then multiplies d0 by 1509 (unsigned), then adds 41 to d0, then copies d0 to a fixed memory address (2 times in a row), then returns.
- `frag_eori_btst_beq_neg_andi_move_rts`: Short code fragment that exclusive-ors 64 into d0, then tests bit 6 of d0, then branches if the last result was zero, then negates d0, then masks d0 with 127, then copies a location in the code to d0, then returns.
- `conditional_exit`: Compares a register value with a global variable and returns immediately if they are equal, otherwise it does nothing.
- `process_object_data_2`: Reads object properties from a table, modifies flags and position values based on conditions, and invokes another routine for specific entries.
- `process_object_data`: Processes object data based on a lookup table index and stores multiple values at calculated memory locations.
- `clear_memory_block`: Clears a block of memory starting at address 9978 by writing zero values to 32 consecutive longwords.
- `initialize_video_mode`: Initializes video hardware registers for display configuration and clears a memory block with specific patterns.
- `process_data_with_function_call`: Processes input data through several arithmetic operations, performs conditional function calls based on test results, and returns modified register values.
- `update_memory_block`: Updates a memory block by copying data from a table and performs conditional operations based on a global variable value.
- `frag_move_dbra_rts`: Short code fragment that copies d0 to the memory at a0 (advancing a0), then copies d1 to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `handle_joystick_input`: Reads joystick status from hardware registers, processes button states and timing information, updates screen buffer locations, and modifies interrupt control registers.
- `read_joystick_data`: Clears specific bits in a hardware port register, reads joystick movement data from custom registers, combines movement values with button states, and writes processed data to output registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_var`: Initializes a global variable with a bitwise OR operation involving input data and constant values, then performs conditional updates based on the variable's current value and hardware register states.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_joystick_input`: Reads joystick and mouse input from hardware registers, processes button states, and updates global variables based on input conditions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `data_table_stub_2`: Contains a series of no-op instructions that serve as a data table rather than executable code, with each instruction being an ori.b #0 to a data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_potentiometer_input`: Processes input from potentiometer hardware registers, performs bit manipulation on input values, and writes results to a hardware output register.
- `update_game_state_2`: Processes a global variable's bitfield values, modifies data based on comparisons, and updates hardware registers including audio and display control.
- `update_bitfield_values`: Updates a bitfield structure by processing values from memory locations, performing arithmetic operations, and modifying data based on comparison results.
- `update_collision_data`: Loads collision-related parameters from global variables, performs arithmetic operations on them, stores intermediate results into a table, and updates a global counter before returning.
- `handle_tile_operations_2`: Processes tile bit manipulation using a global variable and table lookups, updating memory locations with shifted bit values.
- `update_tile_bits`: Processes tile bit manipulation using a global variable and table lookups, updating memory locations with shifted bit values.
- `frag_move_clr_rts`: Short code fragment that copies 16384 to a fixed memory address (4 times in a row), then clears a fixed memory address (4 times in a row), then returns.
- `update_animation_states`: Updates multiple animation state variables based on conditional checks and arithmetic operations, then processes bit flags to control execution of additional routines.
- `frag_or_sub`: Short code fragment that merges d7 into the memory just below a1 (moving a1 down), then subtracts the memory just below a2 (moving a2 down) from d0, then subtracts d0 from the memory just below a3 (moving a3 down), then subtracts the memory just below a4 (moving a4 down) from d1.
- `process_tile_count`: Processes a tile count value by clamping it to a maximum of 80, then performs multiple operations based on bit shifts and modulo calculations, calling another routine with varying parameters while updating counters.
- `handle_tile_operations`: Handles tile operations by limiting input values to 56, performing iterative calls to another routine with calculated offsets, and executing additional calls based on remaining counter values.
- `process_tile_data`: Processes 9 words of tile data from a table using bit manipulation and stores results into memory locations offset by 64, 128, 192, and 256 from the base address in a1.
- `copy_tile_bytes`: Copies 32 bytes from a table to memory locations offset by 64, 128, 192, and 256 from the base address in a1.
- `process_tile_bytes`: Reads four bytes from a table using calculated offsets and passes each byte to another routine for processing, while updating memory pointers and decrementing a loop counter.
- `frag_cmp_blt_sub_addq_bra_addi_rts`: Short code fragment that compares d0 with d2, then branches if the comparison was less, then subtracts d2 from d0, then adds 1 to d1, then jumps to another point in the code, then adds 48 to d1, then returns.
- `frag_move_cmpi_beq_cmp_bne_rts`: Short code fragment that copies an indexed entry from a0 to d0 (2 times in a row), then compares a fixed memory address with 30, then branches if the last result was zero, then compares d3 with 65535, then branches if the last result was not zero, then copies 65535 to d3, then returns.
- `normalize_coordinates`: Reads coordinate values from global memory locations, performs arithmetic operations including subtraction and bit masking, and stores the results back to those locations.
- `frag_cmp_bgt_sub_bpl_blt_bra`: Short code fragment that compares d0 with 383, then branches if the comparison was greater, then subtracts d4 from d1, then branches if the last result was not negative, then compares d1 with an address, then branches if the comparison was less, then jumps to another point in the code.
- `process_collision_response`: Reads collision data from a table, updates position and velocity values based on input, and stores intermediate results in a local structure.
- `process_collision_data`: Reads collision-related data from a structure, updates position and velocity values based on input, and stores intermediate results in a local structure.
- `update_position_and_velocity`: Updates position and velocity values based on input data and global state, then stores intermediate results in a local structure.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `update_audio_pointer`: Reads a value from a table using a counter and updates an audio pointer variable, then calls a function with that value before adding the original value to another global variable.
- `initialize_audio_channels`: Initializes audio channel parameters by setting volume registers to zero, clearing multiple control fields in a data structure, and configuring DMA control register with a specific value.
- `update_linked_list_nodes`: Iterates through a list of structures, updating values based on calculations from a source pointer and potentially setting a third value to a constant plus 60 if a specific field equals 2.
- `mute_audio_channels`: Clears the four audio volume registers, sets the DMA control register to 15, and stores 65535 in a global variable.
- `increment_frame_counter`: Checks a word at offset 72 of a structure in a4; if it is non-zero, it branches; otherwise, it increments a word at offset 74, checks if it equals 6, and if so, clears the word at offset 74 before calling another routine.
- `update_audio_sequence`: Manages audio sequence updates by incrementing counters, checking conditions against predefined limits, and updating pointers and data structures when sequence boundaries are reached.
- `configure_audio_channel`: Calculates an audio channel pointer using input parameters, clears a volume register, and updates channel registers for one of four audio channels while modifying a structure in memory.
- `handle_audio_control`: Processes different control codes from input data to set various audio parameters, update global variables, modify hardware registers including joystick status, and adjust volume levels for specific audio channels.
- `update_audio_channels`: Updates audio period values for four channels, reloads audio sample pointers and lengths for active channels, sets DMA control register based on active channels, and configures volume levels for all four audio channels.
- `frag_lea_move_jsr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 31 to d7, then calls another routine, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d7, then returns.
- `clear_structure_fields`: Clears multiple word and byte fields in a data structure pointed to by a0, including specific offsets for counters, flags, and status bytes.
- `frag_move_lea_bra`: Short code fragment that copies 11 to d0, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_tst_bpl_lea_dbra_moveq_ext_rts`: Short code fragment that tests the memory at a0, then branches if the last result was not negative, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d0, then loads 255 into d0, then sign-extends d0, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `unpack_compressed_stream`: Decompresses a packed data stream from the end by reading flag bits that determine whether to copy previous output bytes or write literal bytes, using bit reader helpers and folding packed data into a checksum.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_decode`: Copies data from a source pointer to a destination buffer while decoding compressed bit patterns, updating pointers and counters during the process.
- `decode_and_copy_data`: Processes compressed data by decoding bit patterns and copying bytes between memory locations, updating pointers and counters during the operation.
- `frag_lsr_beq_rts_move_roxr_2`: Short code fragment that shifts d5 right by 1, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then returns.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (subq.w) and then falls through into the code that follows, with no return of its own.
- `bit_shift_loop`: Performs a bit shifting operation on a value stored in memory, rotating bits through a register while decrementing a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_display_dma`: Sets up DMA channel configuration registers with specific values derived from hardware address calculations, then configures display DMA channels and waits for vertical blank interrupts while processing joystick input data through lookup tables.
- `setup_blitter_parameters`: Configures blitter hardware registers with specific values for addressing, dimensions, and control settings, while waiting for a vertical blank interrupt flag to clear before proceeding.
- `setup_blitter_parameters_2`: Configures blitter hardware registers with different parameter values than the previous routine, including addressing, dimensions, and control settings, while waiting for a vertical blank interrupt flag to clear before proceeding.
- `poll_interrupt_flags`: Polls a global variable's least significant bit in a loop, waiting for it to change from set to clear and back to set states, repeating a specific number of times determined by an input value.
- `frag_btst_bne_beq_dbra_rts`: Short code fragment that tests bit 0 of a fixed memory address, then branches if the last result was not zero, then tests bit 0 of a fixed memory address, then branches if the last result was zero, then loops back while counting down d0, then returns.
- `read_joystick_input`: Reads joystick input data from hardware registers, processes the values through lookup tables, combines them with fire button status, and returns the combined input byte.
- `initialize_render_data_2`: Writes a series of 16-bit values to a global table, including memory addresses and offsets, then stores the table's base address in another.
- `initialize_render_data`: Writes a series of 16-bit values to a global table, including memory addresses and offsets, then stores the table's base address in another global variable.
- `initialize_render_table`: Populates a render table with word-sized values derived from memory addresses and constants, including offsets and swapped register contents, then stores pointers to this table and updates a global counter.
- `initialize_dma_channels`: Sets up DMA channel configuration registers with specific values derived from hardware address calculations, then configures display DMA channels and enables interrupts and DMA functionality.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (negx.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `step_palette_components_in_place`: Walks a table of colour entries and decrements each non-zero 4-bit component when the direction value is negative, or steps each component toward a target table when it is positive.
- `configure_display_dma_channels`: Reads hardware status registers to determine display configuration, then sets up DMA channel pointers for multiple bitplane layers based on the current video mode and screen settings.
- `depack_bitstream_backwards`: Unpacks a compressed stream from its end by reading flag bits that choose between copying a run of earlier output bytes and writing literal bytes, using the bit-reader helpers and folding the packed data into a checksum.
- `bk_refill_bit_buffer_3`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_3`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `frag_move_clr_lea_st_rts`: Short code fragment that copies 2 to a fixed memory address, then clears a fixed memory address (4 times in a row), then loads the address of a fixed memory address into a6, then copies 33360 to a fixed memory address, then sets a fixed memory address to all ones, then returns.
- `set_screen_coordinates`: Initializes screen coordinate values by performing division operations and storing results in a table structure, then it sets bits in a hardware register.
- `calculate_screen_coordinates`: Processes input data to compute screen coordinate values by performing division operations and storing results in a table structure, then iteratively updates coordinates while comparing memory locations for equality.
- `frag_move_btst_bne_bclr_nop_bset_bsr_rt`: Short code fragment that copies the memory at an offset from a4 to d0, then tests bit 0 of d0, then branches if the last result was not zero, then clears bit 1 of the memory at a5, then clears bit 0 of the memory at a5, then pauses for one no-operation step, then sets bit 0 of the memory at a5, then calls another routine, then returns.
- `set_control_bits_and_wait`: Sets specific control bits in a memory location, then waits for a particular bit to clear before proceeding with further operations.
- `pulse_bit0_for_half_count`: Clears bit one of a control byte, halves a word value, then repeatedly clears and sets bit zero while calling a subroutine, until the count is zero.
- `write_control_bytes_wait_bit0`: Writes four control values to bytes at fixed offsets from an address register, then polls one of those bytes until bit zero is set.
- `reorganize_buffered_data_blocks`: Calculates block offsets based on modulo operations, iterates through source buffers to copy data into destination tables, and handles boundary conditions for data reorganization.
- `decompress_bitstream_data`: Scans a buffer for a specific marker, performs bitwise transformations and XOR operations to expand compressed data into a destination table using multiple loop iterations.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address (raw code file, load address unknown)

- `frag_move_bsr_bra`: Short code fragment that copies a0 to a fixed memory address, then calls another routine (5 times in a row), then jumps to another point in the code.
- `frag_move_ori_rts`: Short code fragment that copies a fixed memory address to a fixed memory address (4 times in a row), then copies 16 to a fixed memory address, then copies 2 to a fixed memory address, then copies 32767 to a fixed memory address, then copies 33024 to a fixed memory address, then merges 120 into a fixed memory address, then returns.
- `pulse_port_bit_then_delay`: Clears bit 7 of a global CIA port image, runs a fixed countdown of one hundred iterations that clears bit 3 on each pass, then calls one further routine.
- `delay_then_wait_port_bit`: Runs a fixed countdown of three thousand iterations, then spins until bit 5 of a global variable that holds the CIA port A status is clear.
- `wait_while_port_bit_set`: While bit 4 of a global status byte is set, sets the low two bits to three and calls one further routine, touching the CIA port A and port B registers.
- `wait_for_port_bit_and_pulse`: Waits for a specific bit in a hardware port to clear, then performs a sequence of bit clearing and setting operations on another port while counting down.
- `configure_disk_controller`: Initializes disk controller registers by clearing specific bits, setting control words, and waiting for status bits to indicate completion before proceeding with further operations.
- `wait_for_status_bit_set`: Spins until bit 4 of a global status byte is set.
- `initialize_dma_and_interrupts`: Sets up DMA and interrupt control registers by setting specific bits in various hardware registers, including enabling DMA channels and interrupts.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 2 of 4).adf:an address (raw code file, load address unknown)

- `frag_bra_2`: Short code fragment that jumps to another point in the code (2 times in a row).
- `setup_cia_timer`: Configures CIA timer registers with specific values, waits for timer interrupt conditions while polling status bits, and handles timer overflow scenarios through repeated polling loops.
- `write_port_sequence_clearing_bits`: Writes six port values in turn to the same byte register at a fixed offset, each with a different single bit cleared, and ends with all bits set.
- `frag_btst_beq_moveq_bsr_bra_clr_rts`: Short code fragment that tests bit 4 of the memory at an offset from a4, then branches if the last result was zero, then loads 1 into d0, then calls another routine, then jumps to another point in the code, then clears a fixed memory address, then returns.
- `frag_move_tst_bpl_bsr_sub_beq_add_neg`: Short code fragment that copies d0 to d6, then tests a fixed memory address, then branches if the last result was not negative, then calls another routine, then subtracts a fixed memory address from d6, then branches if the last result was zero, then adds d6 to a fixed memory address, then copies d6 to d0, then branches if the last result was not negative, then negates d0.
- `initialize_cia_timer_2`: Configures CIA timer registers with specific values, waits for timer interrupt conditions while polling status bits, and handles timer overflow by clearing interrupt flags and repeating the process for a specified number of iterations.
- `configure_cia_timer_and_wait`: Initializes CIA timer registers with specific values, then waits for a timer interrupt condition to occur while polling a status bit.
- `xor_checksum_longword_run`: XORs a run of longwords starting at the input address into one accumulator, with the count taken from the high bits of a word argument.
- `combine_masked_longword_pair`: Combines two adjacent longwords by masking each with a mask register and OR-ing the results, shifting the first one left by one bit.
- `decode_mfm_word_run`: Reads a pair of longwords from two halves of a buffer, decodes them into one longword, writes it through an output pointer, and repeats for a count derived from a register.
- `frag_move_lsr_bsr`: Short code fragment that copies d0 to d3, then shifts d0 right by 1, then calls another routine, then copies d3 to d0, then calls another routine.
- `update_flag_bit_no_calls`: Clears bit seven of a byte at a pointer and sets it again when bit six is clear and bit zero of the preceding byte is clear.
- `process_masked_longword`: Combines a longword with a mask using exclusive or and addition, shifts the result, sets bit thirty-one, masks it back, ORs in the masked value, then clears bit thirty-one if bit zero of the preceding byte is set before storing the result through a0.
- `initialize_multiple_structures`: Checks a bit in a global variable and, depending on the result, populates one of three different structure configurations at a6 with various constants and pointers.
- `initialize_cia_timer`: Configures CIA timer registers with specific values, waits for timer interrupt conditions while polling status bits, and handles timer overflow.
- `check_interrupt_flags`: Tests specific bits in a memory location and branches based on the results, then performs several register writes and finally branches to an address based on conditional tests.
- `initialize_memory_blocks`: Fills memory regions with specific longword patterns, initializes structure data based on global variable values, and calls multiple routines to process different memory areas with varying parameters.

### Barbarian II (1991)(Psygnosis)(M4)[cr SKR](Disk 1 of 4).adf:blocks 264-1306 (bytes 135168-669184)#unpacked (raw code file, load address unknown)

- `initialize_game_state_2`: Sets up initial game parameters including screen buffer pointers, counter values, and memory mappings.
- `swap_palette_entries`: Exchanges values between pairs of memory locations in a palette data structure, effectively swapping color component values for animation or rendering effects.
- `clear_and_test_word`: Clears a 16-bit word at an offset of 1412 from a5 and returns if the resulting value is zero, otherwise it returns.
- `update_sprite_attributes`: Constructs sprite attribute entries by calculating vertical positions and color values, then populates a buffer with these attributes using loops.
- `initialize_and_loop`: Clears a 16-bit word at offset 5c2 of the structure in a5, then enters a loop that calls another routine a number of times determined by a value at offset 64c of a5.
- `scale_color_components`: Processes color component values from a source pointer, scales them using a multiplication factor, clamps results to a maximum of 15, and stores the final 12-bit packed value at a destination pointer.
- `update_frame_counter`: Increments a frame counter and conditionally updates other game state variables based on comparison results with threshold values, potentially triggering data decompression operations.
- `decode_data_stream`: Processes a compressed data stream by reading bytes from a source pointer and writing decoded values to a destination pointer, with various control codes that dictate how many bytes to copy or skip.
- `initialize_parameters`: Calculates and stores several parameters including scaled values, bit-shifted results, and array indices based on input word values, then performs a series of memory operations to write data into a buffer.
- `decode_compressed_data`: Decompresses run-length encoded data using bit manipulation and stores results at a specified memory location.
- `initialize_game_structure`: Initializes various fields in game structures including pointers, word values, and function pointers, then calls other initialization routines to set up game state.
- `update_game_state`: Reads input flags from a structure, updates a counter and a status byte based on bit tests, then jumps to a function pointer table based on a value from another structure.
- `frag_movea_move_rts`: Short code fragment that copies a location in the code to a0, then copies 127 to the memory at an offset from a0, then copies 136 to the memory at an offset from a0, then copies 0 to the memory at an offset from a0, then returns.
- `update_input_flags`: Reads a byte from a memory location, rotates it right by one bit, inverts the result, and updates flags based on comparison operations with other values.
- `frag_moveq_movea_move_bsr_dbra_rts`: Short code fragment that loads 0 into d1, then copies a location in the code to a0, then copies the memory at an offset from a0 to d1, then calls another routine, then loops back while counting down d1, then returns.
- `increment_conditional_value`: Checks a 32-bit value at offset 1602 of a5; if bit 23 is not set and bit 1 is not set, it stores the value doubled; otherwise, if bit 1 is set, it increments the value and stores it back at offset 1602.
- `handle_collision_event`: Checks collision flags and timing values in a structure, updates position-related fields, and manages state transitions for collision handling based on bit test results.
- `update_position_values`: Updates position-related values in a structure by adding components, clamping results to a range, and storing intermediate calculations in related memory locations.
- `initialize_structure_fields_2`: Initializes multiple data structure fields with specific values and calls another routine for each field set, updating pointers and configuration flags.
- `clear_structure_fields_2`: Clears specific word-sized and byte-sized fields in a data structure pointed to by a0, then returns.
- `xor_checksum_longword_run_2`: XORs a run of longwords starting at the input address into one accumulator, with the count taken from the high bits of a word argument.
- `combine_masked_longword_pair_2`: Combines two adjacent longwords by masking each with a mask register and OR-ing the results, shifting the first one left by one bit.
- `initialize_structure_fields`: Initializes various fields in a data structure including pointers, counters, and configuration values based on input parameters and performs bit manipulation operations.
