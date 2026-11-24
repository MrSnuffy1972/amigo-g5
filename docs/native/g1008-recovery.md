# Game 1008 (turbo trax (1995)(arcane entertainment)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01008). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2752736 top-level bytes in 173 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 52 of 52 units identical (bytes and relocations), 193 raw-word lines |
| Functions named and described | pass, 1054 of 1054 named (0 author symbols), 1054 of 1054 described (263 by rule, 630 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7143 of 7143 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 32376 of 32376 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 571055 |
| code | 62864 |
| data | 2045951 |
| padding | 26274 |
| unused | 46592 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Echo | 2 | 241 | 10 |
| Copy | 2 | 1671 | 5 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe | 2 | 267 | 8 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad | 2 | 267 | 8 |
| Logo.PAK#unpacked | 1 | 164 | 10 |
| Shopper.PAK#unpacked | 1 | 586 | 23 |
| DiskSwop.PAK#unpacked | 1 | 114 | 5 |
| TrackSel.PAK#unpacked | 1 | 747 | 32 |
| Stat.PAK#unpacked | 1 | 590 | 18 |
| Options.PAK#unpacked | 1 | 1508 | 95 |
| assign | 2 | 435 | 7 |
| Echo | 2 | 241 | 10 |
| Lab | 1 | 2 | 1 |
| Skip | 2 | 220 | 2 |
| Execute | 1 | 1182 | 33 |
| Copy | 2 | 1671 | 5 |
| MakeDir | 1 | 112 | 1 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 413 | 15 |
| IconX | 2 | 404 | 8 |
| LinkUp.PAK#unpacked | 1 | 647 | 29 |
| ppmore | 2 | 233 | 8 |
| Echo | 2 | 241 | 10 |
| Lab | 1 | 2 | 1 |
| Copy | 2 | 1671 | 5 |
| EndPart.PAK#unpacked | 1 | 442 | 16 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 67 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 125 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 270 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 397 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 638 | 83 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1130 | 63 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1143 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5791 | 163 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe#unpacked | 6 | 7941 | 286 |
| Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad#unpacked | 1 | 503 | 28 |

## Cases handled

- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy5.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (22952 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy3.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (22248 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy1.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (29316 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/2indycarSP.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (24636 -> 34816 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/2indyblocks.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (112420 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy4.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (24636 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/3indycarBB.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (30928 -> 40960 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:indy/indy2.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (20324 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Speech.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (52564 -> 30592 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Logo.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (25152 -> 65544 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Shopper.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (60988 -> 83232 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/DiskSwop.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (8456 -> 52260 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/TrackSel.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (43660 -> 151444 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Stat.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (35980 -> 85132 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:pak/Options.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (66356 -> 229484 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:pak/LinkUp.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (19968 -> 97264 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/2woodblocks.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (123052 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood4.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (16696 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood2.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (23272 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/3jeepBB.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (59460 -> 81920 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood5.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (16856 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/2jeepSP.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (49304 -> 69632 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood3.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (21108 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:wood/wood1.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (19312 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow4.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (20320 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/34x4BB.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (61252 -> 81920 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow3.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (18000 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow1.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (25324 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/2snowblocks.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (105096 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow5.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (18920 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/24x4SP.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (51016 -> 69632 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 2 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 2 of 3).adf:snow/snow2.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (17508 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand5.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (22508 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand3.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (27804 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand1.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (23956 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/2dunebuggySP.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (22808 -> 34816 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/2sandblocks.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (109744 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand4.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (17552 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/3dunebuggyBB.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (27588 -> 40960 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:sand/sand2.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (21752 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city5.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (11096 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city3.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (14452 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/3sportsBB.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (26760 -> 40960 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city2.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (12600 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city4.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (12296 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/2sportsSP.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (22592 -> 34816 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/city1.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (13796 -> 129500 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:city/2cityblocks.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (97984 -> 128480 bytes)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:pak/EndPart.PAK: ByteKiller Data (xfd)-packed, unpacked with ancient (28780 -> 274704 bytes)
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 43 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 21 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 87 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 248 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 304 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 172 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk04): 625 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 454 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 4570 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 40296 -> 85360 bytes), 37 of 39 windows confirmed in reference RAM: recovered as a hunk executable
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad: crunched executable decrunched by XFDMaster (PowerPacker 4.0 (xfd), 19888 -> 53188 bytes), 12 of 13 windows confirmed in reference RAM: recovered as a hunk executable
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:c/ppmore: PowerPacker 3.0 (xfd) stream found by ancient scan (7180 -> 10412 bytes) but only 0 of 6 sampled windows are in reference RAM: not used
- self-modifying code at an address (17 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (37 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500-unpk00)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (50 bytes change between runtime segment an address (play-rs-m1500-unpk00) and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (54 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (43 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (57 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (60 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (46 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (61 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (59 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk01)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (36 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (36 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (52 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- turbo trax (1995)(arcane entertainment)[cr flt](disk 1 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- turbo trax (1995)(arcane entertainment)[cr flt](disk 3 of 3)/Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 3 of 3).adf:c/ppmore: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_audio_and_interrupts` (runtime segment an address (play-rs-m1500-unpk01)): Configures audio hardware registers, sets up interrupt handlers, initializes DMA channels, and prepares sound output by configuring multiple audio registers with specific values. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB registers in a specific sequence and sets up audio DMA and interrupts, suggesting a check on disk hardware or track integrity. The successful path would proceed with setting up audio channels and DMA, indicatin.
- `initialize_game_loop` (runtime segment an address (play-rs-m1500)): Initializes global variables and hardware registers including DMA control and serial peripheral settings, then enters a main game loop that processes input conditions and updates game state based on structured data values. Kind: disk hardware ADKCON check. Pass path: The routine initializes ADKCON and waits for a specific VPOSR value, suggesting a track or disk integrity check. If the check passes (i.e., no protection triggered), execution continues normally through the game loop..
- `initialize_system_and_manage_resources` (TurboTrax.Exe#unpacked): Initializes system libraries and clears global data before managing memory allocation and resource loading. Kind: disk hardware check. Pass path: The routine initializes system resources and manages hardware (ADKCON, custom registers), then enters a loop checking for specific conditions like VPOSR sync, and handles various subroutines that suggest a disk integrity or track validation.
- `initialize_graphics_system_2` (Galahad#unpacked): Configures custom hardware registers for graphics display, sets up interrupt vectors, and initializes audio DMA channels while waiting for a specific vertical blanking period. Kind: disk hardware check. Pass path: The routine initializes hardware registers related to CIAB and CUSTOM chipset, and sets up DMA and interrupt configurations. If this check passes (i.e., the expected hardware state is found), it proceeds with initializing audio channels and.

## Functions


### Echo

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies the contents of D1 into D0 and then performs a short branch.
- `update_trajectory_data_3`: Processes trajectory-related data by reading from tables, updating position values, and performing conditional calculations based on stored values.
- `update_trajectory_data`: Processes trajectory-related data by reading from tables, updating position values, and performing conditional calculations based on stored comparisons, then stores the results back to a structure.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position`: Updates a position value based on comparisons between stored values and calculates a new position, then stores the result back to the structure.
- `frag_move_bra_2`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `copy_and_compare`: Copies data between memory locations within a structure and performs a comparison operation, branching based on the result of that comparison.
- `frag_move_lsl_tst_bne_moveq_movea_jsr_b`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 10 into d1, then copies 352 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `conditional_execute`: Executes a series of function calls based on conditional checks involving global variables and register values, then jumps to another location.

### Copy

- `copy_files_to_destination`: Iterates through a list of files and copies them to a specified directory.
- `call_two_callbacks_with_flag`: If D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `call_function_table`: The routine receives a count in D0 and a pointer to a table in A0.
- `check_bitmask_range`: The routine tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning a flag indicating the result.
- `compare_strings_case_insensitive`: The routine compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe

- `process_amiga_hunk_relocations`: Parses a data stream containing AmigaDOS hunk types to perform memory allocation, data relocation, and bitstream decompression.
- `decrement_and_loop`: Decrements a 16-bit value in D1 and loops until the value reaches zero, otherwise it returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_bitstream_data_2`: Processes a bitstream by reading a value from a pointer in a0, calculating an offset, and performing a loop to copy or move bytes into a destination buffer pointed to by a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `decompress_bitstream_data`: Performs bitwise rotations on a value retrieved from a table and bitwise-ORs the result into a destination memory range.
- `execute_trap_instruction`: Moves a 16-bit word from an offset of 3085 in A2 to D4, performs a bitwise OR on D1 with 112, calls a trap, and performs a bitwise OR on D1 with the result of the trap.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad

- `parse_amiga_hunk_data_2`: Parses a data stream containing AmigaDOS hunk type codes to perform memory allocation and relocation.
- `traverse_linked_list`: Decrements a counter in a loop and performs pointer arithmetic on a linked list, calling two different routines and writing a value to a global variable based on a pointer comparison.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decompress_variable_length_bitstream`: Reads a value from a pointer in a0, calculates a destination address and a count based on that value, and then enters a loop to write bytes to memory at a3.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_register_to_register`: Subtracts the value of d4 from d0.
- `decompress_bitstream_data_3`: Processes a bitstream by reading a longword from a pointer in a0, performing bitwise rotations and shifts, and writing the resulting bits into a destination buffer.
- `execute_trap_instruction_2`: Moves a word from an offset of a2 into d4, performs a bitwise OR on d0 with a constant, calls a trap, and performs a bitwise OR on d1 with d0.

### Logo.PAK#unpacked

- `jump_to_address_2`: Performs a jump to an address stored in a longword at offset 6.
- `execute_conditional_subroutine_sequence`: Clears a word in a global variable and executes a sequence of three subroutine calls, where each call is preceded by loading a value into a register and followed by a check of the global variable to determine if further subroutine calls should be executed.
- `wait_for_input_sequence`: Polls a hardware register in a loop until specific bit patterns are detected, then modifies a global variable.
- `adjust_table_values_8`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `populate_pointer_table_2`: Fills a table with pointers to a global variable at fixed intervals and waits for a specific bit pattern in a memory location.
- `loop_subroutine_calls`: Repeatedly calls another routine until a specific condition is met.
- `initialize_pointer_table`: Populates a table with memory addresses and sets two global word values before entering a loop of subroutine calls.
- `loop_input_sequence_processing`: Repeatedly executes a sequence of input polling and data nibble adjustment routines until a specific condition is met.
- `fill_address_list`: Writes a series of memory addresses into a table with a specific stride and updates two global variables with constant values.
- `process_input_nibble_sequence`: Repeatedly calls wait_for_input_sequence and adjust_table_values_8 using two different sets of parameters until the value in register d1 is zero.

### Shopper.PAK#unpacked

- `jump_to_external_routine`: Performs a long jump to an external address.
- `initialize_and_store_registers`: Increments several register values, stores them into global memory locations, and performs a sequence of subroutine calls.
- `rotate_and_reset_byte`: Returns a bit-rotated value derived from the bitwise inverse of a global variable.
- `initialize_hardware_parameters`: Sets various values in a structure based on an input, waits for a specific bit pattern in a hardware register, and then updates a function pointer.
- `update_input_state_2`: Reads a word from a hardware register and performs bitwise operations to update bytes and shorts within a structure, potentially incrementing a counter in the structure if a calculated value matches a stored value.
- `initialize_memory_tables_2`: Populates a table with incrementing long values split into word components and clears a subsequent block of memory.
- `decompress_data_block`: Iterates through a table to perform bit-shifting and extraction to populate a destination buffer from a source table.
- `branch_to_offset`: Loads a longword from a global variable into a register, adds a constant to a pointer, and branches to another routine.
- `process_pixel_data`: Performs a complex loop involving division and bitwise operations to write processed data into a memory buffer.
- `wait_for_status_and_check_flag`: Polls a memory location until a specific bitmask is matched, then executes a series of subroutines and checks a global flag and a character value.
- `copy_memory_block`: Copies thirty-two bytes from a source pointer to a destination pointer using a loop.
- `configure_render_parameters`: Populates a structure with several constants, a pointer to a global variable, and an integer value.
- `execute_repeated_routines`: Calls another routine multiple times in two separate loops based on values retrieved from a global table.
- `initialize_hardware_structure`: Initializes a large memory structure with specific constants and pointers, including setting up two source and destination addresses.
- `check_status_and_increment`: Uses values from a table to load a pointer, then checks a global variable and a value in a memory location, potentially resetting or incrementing a value at a specific offset.
- `setup_bitstream_render`: Initializes several global variables and a structure with specific values and a function pointer.
- `finalize_bitstream_render`: Writes several values to a structure using global variables and then clears those globals.
- `initialize_data_structure`: Sets several fields in a memory block and executes a loop to populate a table using bitwise operations and an incrementing pointer.
- `update_state_logic`: Modifies a global index based on comparisons between two values and increments a value stored in a table.
- `wait_for_bit_pattern_2`: Enters a loop that polls a specific offset within a memory block until two different bit patterns are detected sequentially.
- `adjust_table_values_3`: Iterates through two tables and modifies the values in the second table by adding or subtracting increments based on differences in nibbles and bits.
- `execute_repetitive_subroutines`: Repeatedly calls two specific subroutines within a loop until a condition based on a register value is met.
- `execute_repeated_subroutines`: Repeatedly calls two separate subroutines in a loop until a specific byte value in a register becomes zero.

### DiskSwop.PAK#unpacked

- `jump_to_entry_point`: Performs an unconditional jump to a location specified by a longword at a fixed offset from the current instruction pointer.
- `store_and_call_routines`: Stores a word from d0 into a global variable and calls two other routines.
- `adjust_table_values_4`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `initialize_data_tables`: Iterates through nested loops to populate multiple memory tables with specific byte patterns and then stores a pointer to a global variable into a structure.
- `execute_loop_routine`: Repeatedly calls another routine until a specific status flag is cleared.

### TrackSel.PAK#unpacked

- `transfer_and_rotate_global_values`: Stores values from registers into global variables, calls store_and_rotate_values, writes two 16-bit values to a structure at offsets 1168 and 1172 using a pointer in a5, and returns a 16-bit value.
- `execute_repeated_calls_2`: Calls another routine a fixed number of times in succession before returning.
- `insert_string_constants`: Writes five word constants into a buffer at increments of two words and calls another routine between each write.
- `format_string_buffer`: Copies a string into a buffer and then calls another routine multiple times to insert specific characters at fixed positions.
- `convert_nibble_to_ascii`: Splits a byte from d0 into two nibbles and stores them as ASCII characters in the memory location pointed to by a3.
- `store_and_rotate_values`: Stores several values into global variables and then copies a subset of those values to a different memory location before returning a value.
- `rotate_global_bit_2`: Performs a bitwise rotation on the complement of a byte from the keyboard serial register, writes several bytes of 255 to the keyboard serial register and a global variable, and writes 0 to a global variable.
- `initialize_hardware_parameters_2`: Sets various values in a structure based on an input, waits for a specific bit pattern in a hardware register, and then updates a function pointer.
- `clear_and_populate_tables`: Zeros out a block of memory, populates a table with incrementing values, and waits for a hardware register bit pattern before storing a pointer to a global variable.
- `process_data_structure`: Waits for a hardware register condition, toggles a byte, and then copies and transforms data from a source structure into a destination table.
- `expand_bit_packed_data`: Iterates through a source buffer to expand bit-packed byte values into a larger destination table.
- `update_status_counters`: Updates a structure based on bitwise operations and hardware register values, incrementing specific counters or resetting them based on comparison results.
- `update_status_counters_2`: Updates bytes and shorts in a structure based on the value of a global variable, including incrementing or decrementing counters with wrap-around logic.
- `adjust_table_values_5`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `execute_recursive_calls`: Performs a sequence of five consecutive recursive calls to itself.
- `fill_memory_blocks`: Fills multiple memory blocks with the value 255 using a nested loop structure.
- `copy_expanded_data`: Uses an index from a global variable to locate a source pointer and copies four longwords from that location to a destination buffer.
- `frag_eori_move_ori`: Short code fragment that exclusive-ors 0 into d2, then copies d2 to the memory at a3, then merges 144 into d0, then merges 158 into d0, then merges 30 into d0.
- `perform_recursive_calls`: This routine executes a single recursive call to itself followed by five subsequent calls to itself before returning.
- `execute_recursive_call`: Calculates several addresses based on a value in d0 and calls another routine.
- `initialize_data_structures_5`: Populates a series of memory offsets with specific constant values and pointers within a loop.
- `load_table_entry`: Retrieves a value from a table based on an offset and uses it to configure a memory structure.
- `populate_data_structure_2`: Writes several constant values and a calculated offset into a memory structure, then stores a calculated function pointer at a specific offset within that structure.
- `wait_for_register_toggle`: Loops until a hardware register toggles between two specific bit patterns a set number of times.
- `loop_subroutine_calls_2`: Calls adjust_table_values_5 twice with a count of 2, then calls wait_for_register_toggle with a count of 32 using two tables as arguments, repeating this sequence until the return value in register d1 is zero.
- `loop_subroutine_calls_3`: Calls adjust_table_values_5 twice with different parameters and then calls wait_for_register_toggle, repeating this sequence until the return value in d1 is zero.
- `pack_status_word`: Calculates a byte by adding a constant to an input byte and checks for overflow and specific bit states.
- `store_routine_results`: Calls another routine eight times and stores the returned values into a table.
- `store_routine_results_2`: Executes another routine eight times and stores the returned values into a table, then performs a swap of two global variables.
- `run_double_calls`: Executes another routine twice in a loop until a register is zero and returns a value.
- `copy_table_entry`: Calculates an offset into a table using a byte from a register and copies sixteen bytes from that location to a destination.
- `execute_complex_polling_loop`: Executes a series of calls and enters a loop that checks a byte at offset 1166 of a structure in a5 and 1167 of a5, performing further calls and checking a bit in a hardware register.

### Stat.PAK#unpacked

- `parse_string_input`: Reads characters from a sequence of subroutine calls and writes them into a buffer pointed to by a global variable, handling numeric digits, specific character mappings from a table, and space characters.
- `parse_ascii_input_stream`: Processes a stream of input bytes to construct an ASCII string in a memory buffer.
- `rotate_and_reset_byte_2`: Returns a bit-rotated value derived from the bitwise inverse of a global variable.
- `populate_and_mask_data`: Populates a table with pointers to a global variable and performs complex bitwise masking and data streaming operations using a source table.
- `copy_data_block_2`: Copies multiple blocks of data from a source table to a global variable using nested loops.
- `copy_data_conditionally`: Copies data from a source table to a global variable, using a conditional branch to determine the copy method.
- `update_memory_pattern`: Increments a global variable and periodically writes a specific sequence of bytes to a calculated memory location.
- `copy_memory_block_2`: Copies a block of thirty-two bytes from a source pointer in a0 to a destination starting at a global variable in a1.
- `poll_sequential_bit_patterns`: Enters a loop that polls a specific offset within a memory block until two different bit patterns are detected sequentially, repeating this process for a fixed number of iterations determined by a value in register d7.
- `search_data_table`: Iterates through a structured data list to find a matching byte within a specific range and calculates a value based on table offsets.
- `parse_data_structure`: Iterates through a data stream to find specific byte markers and updates a global variable based on the results.
- `calculate_offset_data`: Calculates several values based on global variables and offsets, then writes these values into a structure, potentially within a loop.
- `calculate_and_store_offsets`: Performs arithmetic on input values to calculate offsets and stores the results into a structure.
- `process_data_stream`: Iterates through a data stream, loading bytes into global variables and performing arithmetic operations based on the results of a subroutine call.
- `call_adjust_nibbles`: Calls another routine to adjust nibble values in a table.
- `wait_and_adjust_nibbles`: Waits for a bit pattern in memory and then calls a routine to adjust nibble values.
- `setup_adjust_table_values_6`: Loads two table addresses into registers a0 and a1, sets a word value of sixteen in register d0, and calls adjust_table_values_6.
- `adjust_table_values_6`: Iterates through two tables and modifies the values in the second table by adding or subtracting increments based on differences in nibbles and bits.

### Options.PAK#unpacked

- `initialize_and_loop_options_structure`: Stores two pointers into global variables, calls another routine, and enters a loop that repeatedly calls another routine with different values at offset 6 of a structure, updating offsets 0, 4, 6, and 18 based on the results of those calls and the values read from the structure.
- `clear_memory_and_call`: Fills a memory range with zeros and executes a sequence of function calls through pointers if certain conditions are met.
- `execute_conditional_pointers`: Checks a bit in a global structure and, if set, executes two functions via pointers stored within that structure.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_function`: Checks a value in a structure and, if non-zero, calls a function pointer stored in a global table.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `close_library_function`: Checks a value in a structure and, if non-zero, calls a function pointer from a global table to close a resource.
- `poll_linked_list_nodes`: Iterates through a linked list by checking a pointer at a specific offset within a structure.
- `append_node_to_linked_list`: Initializes a new three-word structure using values from registers and appends it to the end of a linked list pointed to by a structure at offset 40 of a5.
- `store_pointer_to_structure`: Stores the contents of a register into a structure at offset 40.
- `remove_linked_list_node`: Searches a linked list for a matching value and removes that node from the list.
- `allocate_and_insert_node`: Calls a memory allocation routine via a pointer and, if successful, calls another routine.
- `free_memory_on_condition`: Calls another routine and, if the result is non-zero, calls a memory free routine via a pointer.
- `assign_allocated_pointer`: Calls a memory allocation routine via a pointer and stores the resulting pointer into a location pointed to by a register.
- `clear_and_call_function`: Clears two words at a memory location pointed to by a register, calls another routine, and then calls a function pointer from a global variable if the previous routine returned a non-zero value.
- `store_function_result`: Calls a function pointer from a table and stores the returned value into a memory location offset from a register.
- `zero_and_call_function`: Stores zero into a memory location offset from a register, then calls a function pointer from a table.
- `conditionally_call_function`: Checks if three different values are non-zero and then calls a function pointer from a table.
- `conditionally_call_function_v2`: Checks if two values are non-zero and then calls a function pointer from a table.
- `execute_conditional_sequence`: Calls two function pointers from a table based on the return value of a first function call, and returns a value that may be modified by the results.
- `allocate_and_initialize_structure`: Clears a block of memory in a loop, calls a subroutine to allocate memory, and then conditionally populates a structure with the result.
- `update_structure_values`: Writes a longword and a word to a structure pointed to by a5, calls another routine, and then either calls another routine or sets d0 to a constant.
- `execute_conditional_updates`: Copies a value from a structure into a register, calls two routines, and if a comparison fails, adds a constant to a register and returns.
- `execute_conditional_sequence_2`: Clears a memory range using a loop, calls a routine, and if the result is non-negative, updates a structure and calls two more routines.
- `compare_and_execute_subroutines`: Loads a pointer and a longword from a structure in a4 into a1 and d0, calls two subroutines, and then calls another routine if the longword in d0 does not match the value at offset 4 of the structure.
- `verify_double_function_call`: Calls a function and, if the result is non-zero, calls two more functions and compares the second result against a register value before returning a concatenated result.
- `iterate_string_and_call`: Initializes a global variable via a function pointer if it is zero, iterates through a null-terminated string, and then executes a function pointer.
- `read_file_data`: Reads data from a file using DOS library calls, copies the data to a buffer, and continues reading until end of file is reached.
- `frag_btst_bne_move_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then copies 64 to the memory at an offset from a6, then returns.
- `frag_move_andi_cmpi_bne_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with an address, then compares d0 with 76800, then branches if the last result was not zero, then returns.
- `save_hardware_state`: Saves current hardware register values including DMA and interrupt enable states, then performs additional operations to configure system registers.
- `restore_hardware_state_3`: Restores saved hardware register values including DMA and interrupt enable states, configures coprocessor addresses, and sets specific register bits for system operation.
- `execute_conditional_loop_2`: Stores two pointers into global variables, performs a series of subroutine calls with varying parameters, and updates specific offsets within a structure based on a loop condition.
- `initialize_memory_blocks`: Zeroes out a block of memory, populates an integer table with incrementing values, fills a short array with incrementing values, and updates several pointers and status registers.
- `apply_relative_offsets_to_multiple_lists`: Calls another routine five times, each time passing a different address in a register.
- `add_base_address_to_table_2`: Iterates through a list of values in memory, adding the starting address of the list to each individual element until a negative value is encountered.
- `apply_relative_offsets_to_two_lists`: Calls another routine twice, each time passing a different address in a register.
- `apply_relative_offsets_2`: Iterates through a list of shorts and adds the base address of the list to specific longword entries at fixed intervals of six bytes.
- `process_structured_data`: Waits for a specific bitmask in a register, toggles a byte, and then populates a series of memory locations using values from a source structure.
- `execute_multiple_subroutines`: Calls four different routines in a repeating sequence three times.
- `fill_memory_with_ones_2`: Fills sixteen bytes of memory starting from a pointer derived from a global structure with the value negative one.
- `decrement_and_recurse`: Calls itself recursively and decrements a word value at a specific offset until it becomes negative or a byte in a table reaches zero.
- `initialize_data_structures_3`: Sets several pointers and zero-initializes two distinct blocks of memory using a loop.
- `update_state_and_clear_buffer`: Selects a source value based on a flag, stores it and two pointers into a table, and clears a memory block through a pointer.
- `update_sprite_attributes`: Performs bitwise rotations on a value and writes the result into a memory block via a pointer.
- `update_sprite_attribute_list`: Calculates and writes new coordinate and attribute data into a structure, likely for sprite or object management.
- `render_bitplane_data`: Iterates through a structured data table to extract bitmask values and write them to a memory buffer.
- `read_joystick_input`: Reads the state of the joystick port and CIAA fire buttons to update internal status variables.
- `update_input_state`: Copies a byte from one memory location to another and retrieves a word from a table based on a masked value.
- `adjust_table_values_7`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `invoke_structured_data_processing`: Calls a routine that processes structured data based on bitmask validation and memory population, then pushes a longword from offset 1e of a structure in a6 onto the stack.
- `check_table_value`: Calls two routines, accesses a pointer from a global table, calls a third routine, and then compares a word from that pointer to a value in a5.
- `perform_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `increment_table_index`: Calls two routines, accesses a pointer from a global table, calls a third routine, and then increments a word at that pointer before comparing it to a constant.
- `compare_and_return_status`: Calls three routines, accesses a pointer from a global table, calls a fourth routine, and then compares a word from that pointer to a value in a5.
- `decrement_word_at_pointer`: Subtracts four from a 16-bit word at the address held in a pointer.
- `verify_table_limit`: Calls three routines, accesses a pointer from a global table, calls a fourth routine, and then compares a word at that pointer to a constant.
- `increment_word_at_pointer`: Adds four to a 16-bit word at the address held in a pointer.
- `update_attribute_tables`: Iterates through nested tables to perform bitwise rotations and updates memory blocks with calculated values based on pointer offsets and register contents.
- `update_counter_logic`: Calls a routine, accesses a pointer from a global table, modifies a word at an offset of that pointer based on a value in a5, and calls another routine or pushes a value to the stack.
- `restore_registers`: Restores the contents of registers d0 through d7 and a0 through a6 from the stack and returns.
- `check_and_call_handler`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine with a0 pointing to a table at 7fe, a1 pointing to a table at 16 from a4, and d0 set to 32.
- `check_and_call_handler_2`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine with a0 pointing to a table at 7be, a1 pointing to a table at 16 from a4, and d0 set to 32.
- `check_and_call_twice`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine twice with a0 and a1 pointing to specific tables and d0 set to 32.
- `check_and_call_twice_2`: Checks if a byte at an offset of 8 from a pointer in a5 is non-zero, and if so, calls another routine twice with a0 pointing to a table at 7be and a1 pointing to a table at 73a, and d0 set to 32.
- `apply_conditional_writes`: Checks a byte at an offset from a register, and if it is non-zero, compares a word from a structure to zero and performs writes to a structure if the comparison matches.
- `set_conditional_values_2`: Compares a byte in d0 to 1; if it is not equal, it returns, otherwise it writes 1 to a word at offset 18 from a5 and stores a byte from a0 at offset 24 from a0.
- `set_conditional_values`: If a byte passed in a register equals two, it writes three to a word at an offset from a pointer and writes a byte to another offset from a pointer.
- `process_conditional_byte`: Checks a byte at offset 9 of a structure in a5; if it is not zero, it compares a word from offset 4 of a structure in a0 to zero, and if it matches, stores d0 at offset 25 of a0, updates a global pointer, and calls another routine.
- `update_byte_on_match_2`: Compares a byte in d0 to one, and if it matches, stores d0 at offset 25 of a structure in a0, updates a global pointer, calls another routine, and stores the byte at offset 25 of a0 again.
- `update_byte_on_match`: Compares a byte in d0 to two, and if it matches, stores d0 at offset 25 of a structure in a0, updates a global pointer, calls another routine, and stores the byte at offset 25 of a0 again.
- `set_byte_on_match`: If a byte passed in a register equals three, it writes a byte to a specific offset from a pointer.
- `calculate_coordinate_differences`: Calculates absolute differences between two sets of values stored in global tables, then stores the results in several other global variables.
- `compare_and_add_values`: Loads two words from a table, compares them to two other words, and if they match, adds a word from a third table to d0 if certain conditions are met.
- `perform_subtraction_and_branch`: Adds a word from a table at 378a4 to d1, subtracts a word from a table at 3789e from d3, and branches to another routine.
- `branch_if_non_zero`: Tests a 16-bit word in d2 and, if it is not zero, adds a longword from a table at offset 378a4 to d1 before branching to another location; otherwise it returns.
- `reset_option_settings`: Adds a word from a table at 378a2 to d0, adds a word from a table at 378a0 to d3, stores d3 in a global variable, and copies d0 and d1 to a global table within a loop.
- `poll_and_clear_globals`: Clears several global variables and enters nested loops that poll other routines until specific global values match or a byte is set to a specific value.
- `store_global_pointer`: Stores a longword from a global variable into a memory location at an offset of 10 from a pointer retrieved from a structure at offset 20 of another global pointer, then calls another routine.
- `poll_and_increment_buffer`: Polls a memory location for specific bit patterns in a loop and increments two counters.
- `copy_memory_block_3`: Copies a block of data from one memory location to another using a loop.
- `poll_and_increment_offset`: Iteratively checks a memory location for a bitmask and updates a counter and an offset value.
- `clear_memory_block_2`: Fills two memory blocks with zeros using a loop.
- `swap_and_poll_buffer`: Clears a byte at offset 24 of a table, enters a loop that swaps a value between two global variables, and continues as long as a byte in a table remains zero.
- `update_conditional_word_2`: Tests a byte at an offset from a pointer, and if it is zero, writes a word to another offset of that pointer and a byte to a location from a register.
- `update_conditional_word`: Compares a byte in d0 to 1 and, if it is not equal, returns; otherwise it writes the word 2 to an offset of c from a5 and stores a byte from a0 at offset 24 of a0.
- `update_data_structure_3`: Compares a byte in d0 to 2; if it is not equal, it returns, otherwise it writes 3 to a word at an offset from a5 and stores a byte from a0 at offset 24 from a0.
- `update_data_structure`: If a byte in a register equals three, it writes a word to one offset and a byte to another offset in memory.
- `update_data_structure_conditional`: Checks if a byte at offset 9 from a5 is non-zero, then checks if a word at offset 4 from a0 is zero; if both are true, it writes 2 to a word at offset 14 from a5 and stores a byte from a0 at offset 24 from a0.
- `update_data_structure_2`: If a byte in a register equals one, it writes a word to one offset and a byte to another offset in memory.
- `verify_conditional_logic`: Checks a byte at offset 9 of a structure in a5, and if it is zero, it sets a byte at offset 24 of a structure in a0, performs several calls, and updates a pointer at offset 0 of a structure in a1.
- `conditional_wait_and_set`: Sets a byte value to 255 when input equals one, then performs multiple wait operations and function calls based on conditional logic.
- `set_byte_value`: If a byte in a register equals two, it writes a byte to a specific offset in memory.
- `wait_for_vblank`: Polls the vertical position register for two specific scanline values within a loop, then writes two values to the DMA control and interrupt enable registers after checking a bit in the status register.
- `wait_for_vblank_and_set_registers`: Polls the vertical position register for specific values through a loop, then modifies two words in a global structure by applying bitwise masks and setting specific bits, and finally checks a bit in a hardware register before completing the second modification.

### assign

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header of sequential words into a buffer.
- `transfer_register_and_branch`: Moves the contents of register d1 into register d0 and then performs a short branch to another routine.
- `initialize_track_data`: Sets up initial track-related data in a structure, including clearing specific memory locations and performing several memory operations based on calculated offsets.
- `update_assignment_2`: Processes assignment data based on table lookups and conditional checks, updating various fields in a structure.
- `update_assignment`: Updates a data structure field based on indexed table lookups and conditional checks, then performs additional assignments and status updates.
- `update_assignment_fields`: Updates fields in a data structure based on indexed table lookups and conditional checks, then performs additional assignments and status updates.
- `update_structure_fields_4`: Updates multiple fields in a data structure based on indexed values from arrays, clears certain flags, and performs several conditional operations before making multiple library calls.

### Echo

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, then writes a header containing sequential words into a buffer.
- `transfer_register_and_branch_2`: Moves the contents of register d1 into register d0 and then performs a relative branch to another routine.
- `update_trajectory_data_4`: Processes trajectory-related data by reading from tables, updating position values, and performing conditional calculations based on stored data.
- `update_trajectory_data_2`: Reads trajectory-related data from a table using index values from a structure, updates position and velocity fields in the structure based on table lookups and calculations, then performs conditional updates to a counter field depending on comparison results.
- `frag_move_bra_3`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position_counter`: Updates a position counter by incrementing it with a value from memory, then stores the result back to the same location and performs a comparison with another memory value before branching.
- `frag_move_bra_4`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `compare_and_copy_values`: Copies values between structure fields, compares them with a threshold, and performs conditional operations including memory reads and library calls based on the comparison results.
- `frag_move_lsl_tst_bne_moveq_movea_jsr_b_2`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 10 into d1, then copies 352 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `conditional_library_calls`: Performs conditional library calls based on a global flag check, and makes additional library calls with specific parameters before returning.

### Lab

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### Skip

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value in a field, then writes a header and fills a buffer with sequential words.
- `transfer_register_and_branch_3`: Moves the contents of data register 1 into data register 0 and branches to another routine.

### Execute

- `initialize_command_execution_environment`: Allocates and initializes a data structure for command processing by populating it with function pointers and clearing internal buffers.
- `process_command_state_machine`: Evaluates a command structure by executing various function pointers based on a status code.
- `process_input_buffer`: Iterates through a buffer of input characters, comparing them against specific boundary values and updating a data structure.
- `validate_single_character_input`: Checks if a string length is exactly one; if so, it retrieves a byte from a table at offset 120 of a structure and returns it, otherwise it performs a series of function calls and returns a specific error code.
- `check_single_char_string_length`: Calculates the length of a null-terminated string and, if the length is exactly one, returns a byte from a table at offset 120 of a structure passed via a pointer in a3.
- `initialize_variables`: Initializes several variables in a structure pointed to by a5, setting counters and a status flag to zero before proceeding to another routine.
- `process_input_sequence_4`: Processes a sequence of input values stored in a structure, updating counters and handling specific input conditions.
- `process_input_sequence_2`: Processes a sequence of input values stored in a structure, updating counters and handling specific input conditions.
- `frag_jsr_move_moveq_tst_bne_bra`: Short code fragment that calls another routine, then copies d0 to d1, then loads 255 into d2, then calls another routine, then tests d6, then branches if the last result was not zero, then jumps to another point in the code.
- `process_input_state`: Updates an input state counter and stores a value at a specific offset in a structure, then performs conditional checks on the stored value and calls other routines based on the results.
- `load_and_compare`: Loads a value from a memory location relative to a3 into d0, compares it with d7, and branches to a specific address if they are not equal.
- `frag_moveq_move_cmp_bne_movea_jsr_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a3 to d0, then compares d7 with d0, then branches if the last result was not zero, then copies the memory at an offset from a3 to a6, then calls another routine, then copies d0 to d1, then calls another routine, then copies d0 to d7, then jumps to another point in the code.
- `frag_moveq_move_cmp_bne_tst_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a3 to d0, then compares d0 with d7, then branches if the last result was not zero, then tests d4, then branches if the last result was zero, then loads 0 into d4, then jumps to another point in the code.
- `frag_tst_beq_moveq_cmp_bne_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then loads 34 into d0, then compares d7 with d0, then branches if the last result was not zero, then loads 2 into d0, then jumps to another point in the code.
- `process_input_sequence_6`: Processes input values by comparing them against specific constants and updating counters, while handling input conditions through subroutine calls.
- `frag_move_jsr_moveq_cmp_bne_bra`: Short code fragment that copies d7 to d0, then calls another routine, then loads 78 into d1, then compares d0 with d1, then branches if the last result was not zero, then loads 10 into d7, then jumps to another point in the code.
- `process_input_sequence_5`: Processes input values by comparing them against specific constants and updating counters, while handling input conditions through subroutine calls.
- `process_input_sequence_3`: Processes a sequence of input values stored in a structure, updating counters and handling specific input conditions.
- `frag_tst_bne_moveq_cmp_jsr_move_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then loads 61 into d0, then compares d7 with d0, then branches if the last result was not zero, then calls another routine, then copies d0 to d1, then calls another routine, then loads 254 into d0, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `check_condition_loop`: Compares a value in d5 with another value in d6, and if the first is less than or equal to the second, it sets d0 to zero and branches to a specific address.
- `store_and_increment`: Stores the byte from d0 at the address in a3 plus the offset in d5, then increments a value in a2 and returns.
- `frag_link_movem_movea_move_moveq_bra`: Short code fragment that creates a stack frame in a5, then saves registers on the stack, then copies the memory at an offset from a5 to a3, then copies the memory at an offset from a5 to d7, then copies the memory at an offset from a5 to a2, then loads 0 into d6, then jumps to another point in the code.
- `load_and_branch`: Loads a byte from a memory location using an index from another register, stores that value on the stack, and then branches to another routine.
- `update_counter`: Increments a counter in a register, loads a memory address into another register, tests a byte at that address, and branches back if the byte is non-zero.
- `execute_callback_pair`: Extracts a pointer from offset 4 of a structure, calls a routine at an offset within that pointer, and then calls a second routine using the original structure and a pointer located at offset 16.
- `execute_callback_pair_2`: Extracts a pointer from offset 4 of a structure and calls several different routines through pointers found at various offsets within that pointer, then calls another routine using a value from a register and a pointer from a structure offset.
- `execute_offset_callbacks`: Calls two different routines using pointers located at specific offsets within a provided structure.
- `store_registers_to_pointer`: Copies the contents of D1 through D7, A1 through A6, and the value from the stack into successive entries of the pointer passed in A0, then returns.
- `normalize_param_and_copy_word`: Ensures the second argument is non-zero and then copies the first word of the structure pointed to by the first argument into a word located at an offset within that structure, returning a combined value formed from the adjusted second argument and a word from the structure.
- `normalize_signed_operands`: Negates the values in D0 and D1 if they are negative to ensure they are positive before calling another routine, then restores the original sign of the result based on the input signs.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend in D0 by a 32-bit divisor in D1 using a normalized fixed-point method, returning a 64-bit result with the quotient in the high 32 bits and the remainder in the low 32 bits.

### Copy

- `copy_files_to_destination_2`: Iterates through a list of files and copies them to a specified directory.
- `call_two_callbacks_with_flag_3`: If the value in D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `call_function_table_2`: Receives a count in D0 and a pointer to a table in A0, then for each iteration calls a function from the table at offset -948 of the library base using a pointer derived from the table and an incrementing index.
- `check_bitmask_range_2`: Tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning one if the value is within the range and the bitmask check passes, otherwise it returns zero.
- `compare_strings_case_insensitive_2`: Compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal, and returns the signed difference between the first non-matching characters, or zero if the strings are equal up to the null terminator.

### MakeDir

- `set_error_and_cleanup`: Initializes an error code and, if a library base is present, it clears a small stack buffer, opens a DOS library context, and iterates over a null-terminated list of strings to attempt an operation, updating an error flag and status code before closing the context and releasing resources.

### EndIf

- `clear_d0_register`: Clears the D0 register and returns.

### If

- `find_max_and_prepare_buffer_5`: Walks a linked list of entries to find the maximum value in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_d1_to_d0`: Moves the value from D1 into D0 and then branches to another routine.
- `update_trajectory_data_6`: Updates trajectory-related data in a structure based on conditions and values from another structure, including setting flags and copying indexed entries.
- `frag_move_lsl_tst_beq_moveq_cmp_bgt_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then loads 10 into d2, then compares d2 with the memory at an offset from a2, then branches if the comparison was greater, then loads 255 into d1, then jumps to another point in the code.
- `frag_move_lsl_tst_beq_moveq_cmp_bgt_bra_2`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then loads 20 into d2, then compares d2 with the memory at an offset from a2, then branches if the comparison was greater, then loads 255 into d1, then jumps to another point in the code.
- `process_trajectory_data`: Loads trajectory-related data from a table using a scaled index, validates the data against a fixed constant, and stores results at specific offsets in a structure.
- `validate_trajectory_data`: Checks trajectory-related data at multiple offsets in a structure, compares values with a fixed constant, and sets a result based on the comparisons.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_move_lsl_tst_beq_moveq_cmp_bne_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then loads 1 into d2, then compares d2 with the memory at an offset from a1, then branches if the last result was not zero, then loads 255 into d1, then jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_moveq_cmp_beq_tst_bne_bra`: Short code fragment that loads 1 into d1, then compares d1 with the memory at an offset from a1, then branches if the last result was zero, then tests the memory at an offset from a1, then branches if the last result was not zero, then loads 255 into d1, then jumps to another point in the code.
- `update_trajectory_flag`: Loads a value from a table using a scaled index, checks if it's zero, and if not, stores the value at another location before setting a flag to -1.
- `initialize_trajectory_data`: Initializes trajectory-related data by clearing a flag, checking a condition, and setting multiple values in memory structures based on that condition.
- `process_input_loop`: Processes input data through a series of conditional jumps and function calls, updating memory locations with results from those calls.
- `process_input_condition`: Processes input data by comparing values in memory with constants, and based on those comparisons, either continues processing or jumps to different sections of code.

### IconX

- `run_command_with_template_options`: Executes a command using a template of options by iterating through a list of arguments, performing various library operations including string copying, memory allocation, and window management, and handles errors or delays based on the command's execution status.
- `call_library_entry_point`: Calls one Amiga library function through the library base passed as an argument, then returns.
- `build_linked_entry_list`: Allocates a memory block through an Amiga library call, stores a count derived from register D0 in it, and walks a linked list of entries, allocating a node for each entry with a non-zero field and copying a value into it.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `copy_bytes_with_termination`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered.
- `copy_c_string`: Advances to the terminating zero of the string at A0, then copies the bytes from A1 onto its end, including the zero.
- `compute_fixed_point_product`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.

### LinkUp.PAK#unpacked

- `jump_to_address_3`: Performs a long jump to a specific address.
- `initialize_global_state`: Sets several global variables based on the value of a byte in a register and stores multiple values into a global memory range.
- `initialize_memory_tables`: Fills a global memory range with zeros, populates a table with incrementing integer values, and fills another structure with incrementing short values.
- `add_base_addresses`: Calls a routine twice to add the contents of two different memory addresses to a table.
- `add_base_address_to_table`: Iterates through a list of values in memory, adding the starting address of the list to each individual element.
- `apply_relative_offsets_3`: Calls a routine to add the address at a specific memory location to a list of values.
- `apply_relative_offsets`: Iterates through a list of shorts and adds the base address of the list to specific longword entries at fixed intervals.
- `process_data_table`: Waits for a specific bitmask in a memory location, toggles a byte, and then populates a table with values derived from a source structure.
- `initialize_data_structures_4`: Stores a memory address into a location at offset 0 of a structure and calls four different routines three times each.
- `fill_memory_with_ones`: Fills a block of eight longwords with all bits set to one.
- `initialize_data_structures`: Clears several memory blocks and sets up multiple pointers and counters within a structure.
- `update_state_pointers`: Updates a set of pointers and clears a block of memory based on a conditional check of a byte value.
- `rotate_and_merge_data`: Performs bitwise rotations on a value retrieved from a table and bitwise-ORs the result into a destination memory range.
- `update_structure_list_data`: Calculates updated values for a series of fields within a structure based on arithmetic operations on input registers and table lookups.
- `decode_compressed_sprite_data`: Iterates through a structured table to extract and decompress bitstream data into a destination buffer.
- `joystick_input_process`: Reads joystick data from hardware registers, processes the input to determine movement direction, and updates internal state variables based on the input.
- `frag_clr_move_btst_beq_bra`: Short code fragment that clears the memory at an offset from a0, then copies the memory at an offset from a0 to the memory at an offset from a0, then clears the memory at an offset from a0, then tests bit 8 of d1, then branches if the last result was zero, then copies 2 to d3, then jumps to another point in the code.
- `frag_btst_beq_move_tst_bra`: Short code fragment that tests bit 0 of d1, then branches if the last result was zero, then copies 7 to d3, then tests bit 9 of d0, then branches if the last result was zero, then copies 4 to d4, then tests d3, then branches if the last result was zero, then copies 255 to d4, then jumps to another point in the code.
- `update_counter_condition`: Checks a bit in d0, updates a counter in d4 based on conditions, adds d4 to d3, compares the result with a value at offset 4 from a0, and increments another counter if they match.
- `frag_move_clr_rts`: Short code fragment that copies d3 to the memory at an offset from a0, then clears the memory at an offset from a0, then returns.
- `copy_and_lookup`: Copies a byte from one memory location to another and retrieves a word from a table based on a masked value.
- `adjust_table_values`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `execute_repeated_calls`: Decrements a value provided in a register and executes another routine repeatedly until the counter reaches zero.
- `execute_conditional_loop`: Repeatedly calls another routine twice per iteration until a specific byte value becomes zero.
- `invoke_nibble_adjustment`: This routine acts as a wrapper that calls a procedure to modify nibble bits within a second array based on comparisons with a first array.
- `toggle_and_loop_global`: Performs a series of subroutine calls and decrements a local counter in a loop, bitwise inverting a global variable when the counter reaches zero.
- `process_table_data`: Retrieves a value from a table using an offset and calls another routine with parameters derived from a pointer.
- `loop_and_modify_pointers`: Executes a loop that decrements a counter and modifies two global pointers by fixed offsets while calling several subroutines.
- `parse_and_store_data`: Converts character data into numeric values, stores them in a table, and performs a loop to process data based on the resulting values.

### ppmore

- `parse_amiga_hunk_data`: Parses a data structure containing AmigaDOS hunk types, performing memory relocations and copying data blocks based on hunk codes.
- `decrement_counter_loop`: Decrements a 16-bit counter in a register and loops until it reaches zero.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `expand_compressed_bitstream`: Reads a length value from a pointer in a0, calculates a destination offset, and enters a loop that extracts bit-packed data into a buffer pointed to by a3.
- `check_shifted_flag_3`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `subtract_word`: Subtracts the value in register d4 from the value in register d0.
- `extract_bitstream_and_shift`: Extracts bits from a source pointed to by a0 and shifts them into a destination register using a bit count in d4, looping d0 times and updating a flag based on the extracted bits.
- `execute_trap_and_or`: Moves a word from an offset of a2 into d4, calls a trap, and performs a logical OR of d2 into d0.

### Echo

- `find_max_and_prepare_buffer_6`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `move_and_branch_2`: Moves the value from register d1 into register d0 and branches to another location.
- `update_trajectory_data_5`: Processes trajectory-related data by reading from tables, updating position values, and performing conditional calculations based on stored values.
- `update_position_data`: Loads position values from a table using indexed addressing, updates stored position and delta values, then performs boundary checks and calculations on the position data before storing results back to the structure.
- `frag_move_bra_5`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `validate_and_update`: Initializes a memory location with a constant value, performs multiple conditional tests on memory locations, and updates another memory location based on comparison results.
- `frag_move_bra_6`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_and_execute`: Updates several memory locations in a1, performs comparisons, and calls a function multiple times based on conditional checks and loop iterations.
- `frag_move_lsl_tst_bne_moveq_movea_jsr_b_3`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was not zero, then loads 10 into d1, then copies 352 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `conditional_execution`: Checks a condition in d1, and if it's not met, executes a series of function calls before jumping to another address.

### Lab

- `clear_d0_and_return`: Returns immediately after clearing register d0.

### Copy

- `copy_files_to_destination_3`: Iterates through a list of files and copies them to a specified directory.
- `call_two_callbacks_with_flag_2`: If the value in D0 is non-zero, it calls a callback via a pointer in A1 at offset -126 of the library base, then calls another routine from the same library at offset -828.
- `call_function_table_3`: The routine receives a count in D0 and a pointer to a table in A0.
- `check_bitmask_range_3`: The routine tests whether an input value in D0 lies within a numeric range and whether a specific bit in a bitmask is set, returning a flag indicating the result.
- `compare_strings_case_insensitive_3`: Compares two byte strings pointed to by A0 and A1 character by character, treating uppercase and lowercase letters as equal.

### EndPart.PAK#unpacked

- `jump_to_address`: Performs a long jump to an address stored in a register.
- `execute_subroutines`: Calls two subroutines and then returns.
- `wait_and_assign_pointer`: Loops until a specific bit pattern is detected in a memory location, then writes a constant word to a structure and assigns a global variable address to another structure.
- `poll_hardware_bit`: Calls three other routines and then loops until bit 7 of the CIAA_PRA register is cleared.
- `populate_data_structure`: Waits for a specific bit pattern in a value at an offset of a structure in A6, then swaps two longwords in a table and fills a second table with incrementing values split into high and low words.
- `populate_calculated_table`: Fills a table with values derived from a combination of loop counters and results from another routine.
- `find_minimum_bit_index`: Iterates through a list of words to find the lowest bit index set in each word and tracks the minimum index found across the entire list.
- `update_global_counter`: Increments a global variable, performs conditional checks, and returns a value derived from a calculation involving that variable.
- `clear_memory_block`: Calculates a memory address by adding a value from a global variable to a base address stored in another global variable, then clears a large contiguous range of memory starting at an offset from that address using multiple block moves.
- `copy_data_block`: Copies twenty-two words from a calculated memory location to another location.
- `initialize_structure_fields`: Writes several constant values, pointers, and calculated offsets into a memory structure.
- `sum_table_values`: Iterates through a data stream to extract values and accumulate their sum into a local variable.
- `populate_data_list`: Repeatedly writes a series of pointers and associated values into a structure at a calculated offset.
- `loop_and_store_word`: Repeatedly calls another routine and stores a word from a structure into a register until a condition is met.
- `wait_for_bit_pattern`: Loops until a specific bit pattern is detected within a word at a memory location.
- `adjust_table_values_2`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_custom_stack_pointer`: Adjusts the stack pointer by four bytes, writes a specific memory address to a global variable, and saves several registers to the stack before returning.
- `frag_move_ori_eori`: Short code fragment that copies d0 to d2, then merges 0 into d0, then exclusive-ors 0 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `initialize_hardware`: Reads a hardware register value, checks if it matches a specific pattern, and if so, opens a library and stores its handle in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_block_4`: Clears a block of memory by writing zero values to a series of 32-bit words starting at a specified address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_library_and_registers`: Opens a library, initializes global variables with library node values, sets multiple register fields to specific constants, clears a table of 24 words, and configures audio-related settings based on library availability.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_display_values`: Writes a word value from a register to three consecutive memory locations with specific offsets, then performs bitwise OR operations on two words from memory and writes them back to different offsets, followed by a library call and memory copies.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_call_2`: Initializes a memory range with zero values, sets specific values in a structure based on a flag, and calls another routine.
- `restore_system_2`: Resets various hardware registers and executes a sequence of function calls to restore the system state.
- `getvbr`: Reads the vector base register through the exec Supervisor call when the CPU-type flag is set, and stores the result at the address held in a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_input`: Waits for a specific input condition by polling a hardware register and decrementing counters until a threshold is reached, then returns a combined value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: Initializes game state variables and calls several subroutines to set up the game environment before starting gameplay.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_input_loop_2`: Processes input from hardware registers in a loop, handles conditional execution based on button states, and manages global variables during execution.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_game_loop`: Manages the main game loop by calling input handling routines and processing game events until certain conditions are met, then returns a combined value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_sprite_positions`: Updates sprite position data by copying values from one memory location to another and continues processing until a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_input_and_update`: Handles input processing with nested loops, updates global variables, and manages function calls based on button states and conditional checks.
- `toggle_flag_and_update`: Toggles a flag bit in memory, and if the result is zero, updates multiple memory locations with a constant value before returning.
- `decrement_and_update`: Decrements a counter and checks if it becomes negative, and if so, updates multiple memory locations with a constant value before returning.
- `split_nibbles_into_table`: Calls a routine 32 times, then iterates over a table of 32 words, splitting each word into four-bit nibbles and storing them with their negated values into a destination structure.
- `frag_moveq_move_andi_cmp_bgt_lsr_dbra_r`: Short code fragment that loads 2 into d3, then copies d0 to d1, then masks d1 with 15, then compares d2 with d1, then branches if the comparison was greater, then copies d1 to d2, then shifts d0 right by 4, then loops back while counting down d3, then returns.
- `update_game_objects`: Updates game object states by decrementing timers and modifying values in arrays based on comparison results, then returns a specific value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_game_data`: Processes game data by iterating through arrays, updating values based on comparisons, and handling multiple data structures in nested loops before returning.
- `decrement_and_process_2`: Decrements a counter in a global variable and processes data from a table based on the counter value, modifying values in the table according to bitwise operations.
- `setup_audio_and_interrupts`: Configures audio hardware registers, sets up interrupt handlers, initializes DMA channels, and prepares sound output by configuring multiple audio registers with specific values.
- `initialize_data_structure_2`: Initializes a data structure with sequential values by filling arrays with incrementing numbers and storing pointers to memory locations.
- `frag_move_movea_bra`: Short code fragment that copies 32767 to the memory at an offset from a6 (2 times in a row), then copies a location in the code to a2, then copies a location in the code to the memory at an offset from a2 (2 times in a row), then copies 1 to a fixed memory address, then copies 0 to a fixed memory address, then jumps to another point in the code.
- `lbl000352`: Writes the constant 255 to one variable and 32 to a global variable, which the listed facts associate with the interrupt request register.
- `frag_movem_move_lea_st_rte`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then loads the address of a location in the code into a0, then sets the memory at a0 to all ones, then restores saved registers from the stack, then copies 8192 to a fixed memory address, then returns from the exception.
- `decode_and_render_text`: Decodes text data from a buffer, processes character codes, and renders characters into a display buffer using lookup tables and bit manipulation.
- `synchronize_frame_with_wait`: Synchronizes frame timing by calling a wait function multiple times, updating values in a structure with computed results based on bitwise operations.
- `synchronize_frame_with_wait_reverse`: Synchronizes frame timing by calling a wait function multiple times, updating values in a structure with computed results based on bitwise operations and conditional comparisons.
- `wait_tof`: Clears a frame flag byte and spins until that byte becomes non-zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_callback_sequence_2`: Checks a flag in a structure and, if set, performs a series of calls to another routine while updating a table entry with a calculated value based on the current state.
- `execute_and_store_results_2`: Calls a function pointer from a structure and stores the resulting values and a calculated sum into a table at specified offsets.
- `check_and_copy_data`: Reads a global variable and, if it equals one, copies data from a structure pointed to by a table into another structure, then updates several global variables with values from the source structure.
- `adjust_velocity_based_on_condition`: Adjusts a velocity value in a structure based on a global condition, either increasing or decreasing the value with clamping logic.
- `frag_movea_move_bra_3`: Short code fragment that copies a fixed memory address to a4, then copies a fixed memory address to d2, then jumps to another point in the code.
- `frag_movea_move_bra_4`: Short code fragment that copies a fixed memory address to a4, then copies a fixed memory address to d2, then jumps to another point in the code.
- `calculate_offset_value_2`: Calculates a value based on a global variable and a table, then stores the result into a structure with clamping logic applied to the computed value.
- `initialize_audio_channels`: Processes sample metadata to calculate audio channel offsets and lengths, then configures DMA and audio hardware registers for playback.
- `frag_lea_move_moveq_rts`: Short code fragment that loads the address of a fixed memory address into a5, then copies 8192 to the memory at an offset from a5, then copies a location in the code to a fixed memory address, then loads 0 into d0, then copies d0 to the memory at an offset from a5 (4 times in a row), then copies 15 to the memory at an offset from a5, then returns.
- `read_dma_status`: Reads the DMA status register and stores the value in a global variable, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_counters`: Increments multiple global counters and updates DMA-related registers based on their values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_counters_2`: Increments multiple global counters used for DMA timing and conditionally calls data decoding routines based on counter values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dma_counters`: Increments multiple global counters and updates DMA-related registers based on their values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_track_data_2`: Updates track-related data by decoding variable-length information and modifying values based on conditional checks, then performs bit manipulation on a table entry before returning.
- `decode_variable_length_data`: Reads a header byte to determine operand size and type, updates a pointer and length value in memory, then performs an indirect jump to a calculated address.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `frag_nop_bra_2`: Short code fragment that pauses for one no-operation step, then jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `frag_nop_bra_3`: Short code fragment that pauses for one no-operation step, then jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `frag_nop_bra_4`: Short code fragment that pauses for one no-operation step, then jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_clr_move_cmpi_bge_rts`: Short code fragment that clears the memory at an offset from a1, then copies the memory at an offset from a1 to d1, then compares d1 with 32, then branches if the comparison was greater or equal, then copies d1 to the memory at an offset from a0, then copies d1 to the memory at a0, then returns.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `frag_tst_beq_cmpi_add_lea_move_addq_rts`: Short code fragment that tests d0, then branches if the last result was zero, then compares d0 with 70, then branches if the last result was zero, then adds the memory at an offset from a1 to d0, then loads the address of an indexed entry from a0 into a4, then copies a4 to the memory at an offset from a1, then adds 4 to the top of the stack, then returns.
- `compare_and_update_pointer`: Compares word values against multiple memory locations in a structured sequence, updating pointers when conditions are met.
- `update_pointer_and_compare`: Adjusts a pointer by subtracting eighteen from its value, compares a word at that location with a register value, and branches if the register value is less than the word.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a1, then returns.
- `frag_move_sub_andi_moveq_cmp_bpl_rts`: Short code fragment that copies the memory at an offset from a1 to d1, then subtracts the memory at a1 from d1, then masks d1 with 4095, then loads 113 into d2, then compares d1 with d2, then branches if the last result was not negative, then copies d2 to d1, then copies d1 to the memory at an offset from a1, then copies d1 to the memory at an offset from a3, then returns.
- `clamp_velocity`: Clamps a velocity value between zero and 888, stores the clamped value back to the structure, and updates another location with the same value.
- `process_movement`: Processes movement calculations by negating a value when certain conditions are met, then performs additional arithmetic operations on values from memory and stores results.
- `frag_tst_bpl_neg_rts_2`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not negative, then negates the memory at an offset from a1, then returns.
- `validate_and_accumulate`: Loads a word from offset twenty-two of a structure into a register, checks if it's zero, and branches to another location if so.
- `bound_and_update`: Updates a value with a clamped result, clears specific fields in the structure, and performs a lookup operation based on the clamped value before storing it.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `split_word_into_fields_2`: Reads a word from a structure, extracts its low and high nibbles, stores the low nibble as a word when nonzero, and stores the high nibble shifted right by two as a byte.
- `compute_and_store_masked_value`: Loads the byte at offset twenty-nine of a structure into a register, masks it with 127, loads a byte from offset twenty-seven of the same structure into another register, and checks if it's zero.
- `frag_tst_bpl_not_bra_3`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not negative, then inverts d0, then jumps to another point in the code.
- `update_position_and_sign_2`: Calculates a signed offset using bit manipulation and multiplication, stores it in a structure, and updates a byte in a structure by adding two other bytes.
- `frag_move_lea_tst_beq_mulu_lsr_movea_rt`: Short code fragment that copies a0 to the stack, then loads the address of a location in the code into a0, then tests the memory at a0, then branches if the last result was zero, then copies the memory at an offset from a1 to d1, then multiplies d1 by a location in the code (unsigned), then shifts d1 right by 5, then copies d1 to the memory at an offset from a3, then copies the top of the stack to a0, then returns.
- `clamp_and_scale`: Performs clamping and scaling operations on a value derived from memory and registers, clears a field in the structure, and stores results to multiple locations.
- `split_nibbles_into_fields_2`: Reads a word from a structure, extracts its low and high nibbles, stores the low nibble as a word when nonzero, and stores the high nibble shifted right by two as a byte.
- `process_byte_operations`: Processes byte values from memory locations using bitwise operations and conditional branches to set flags and update registers.
- `frag_tst_bpl_not_bra_4`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not negative, then inverts d0, then jumps to another point in the code.
- `scale_and_accumulate_value`: Shifts a word in a register right by two bits, uses that as an index to read a byte from memory, multiplies it with a word at offset thirty-six of a structure, shifts the result right by seven bits, and checks if a byte at offset thirty-five of the same structure is positive.
- `scale_and_accumulate`: Scales a constant value using multiplication and bit shifting, stores the result in a structure, and accumulates a byte value from another structure.
- `update_flags_and_position`: Updates flags based on conditional checks, performs bit manipulation operations, and adjusts position values in a structure while maintaining state information.
- `frag_move_sub_ble_add_rts_2`: Short code fragment that copies the memory at an offset from a1 to d1, then subtracts d1 from the memory at an offset from a1, then branches if the comparison was less or equal, then adds d1 to d1, then adds d1 to the memory at an offset from a1, then returns.
- `frag_move_rts`: Short code fragment that copies 1 to the memory at an offset from a1, then returns.
- `frag_clr_move_add_st_rts_2`: Short code fragment that clears the memory at an offset from a1, then copies the memory at an offset from a1 to d1, then adds d1 to d1, then copies d1 to the memory at an offset from a0, then sets the memory at an offset from a0 to all ones, then returns.
- `calculate_and_store_pointer`: Calculates an address by scaling a value and adding it to a global pointer, stores the calculated address in memory, and performs additional operations on another structure.
- `frag_clr_move_rts_moveq_and_lsr_jmp`: Short code fragment that clears the memory at an offset from a1, then clears the memory at an offset from a0, then copies the memory at an offset from a1 to the memory at an offset from a0, then clears the memory at an offset from a0, then returns, then loads 240 into d1, then masks d1 with the memory at an offset from a1, then shifts d1 right by 2, then jumps to another point in the code.
- `frag_bra_23`: Short code fragment that jumps to another point in the code.
- `frag_bra_24`: Short code fragment that jumps to another point in the code.
- `frag_bra_25`: Short code fragment that jumps to another point in the code.
- `frag_bra_26`: Short code fragment that jumps to another point in the code.
- `frag_bra_27`: Short code fragment that jumps to another point in the code.
- `frag_bra_28`: Short code fragment that jumps to another point in the code.
- `frag_bra_29`: Short code fragment that jumps to another point in the code.
- `frag_bra_30`: Short code fragment that jumps to another point in the code.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `frag_nop_bra_5`: Short code fragment that pauses for one no-operation step, then jumps to another point in the code.
- `frag_bra_31`: Short code fragment that jumps to another point in the code.
- `frag_bra_32`: Short code fragment that jumps to another point in the code.
- `frag_bra_33`: Short code fragment that jumps to another point in the code.
- `frag_bra_34`: Short code fragment that jumps to another point in the code.
- `frag_bra_35`: Short code fragment that jumps to another point in the code.
- `frag_clr_moveq_and_move_rts`: Short code fragment that clears the memory at an offset from a1, then loads 15 into d2, then masks d2 with the memory at an offset from a1, then copies d2 to the memory at an offset from a1, then returns.
- `frag_clr_moveq_and_bne_bclr_rts`: Short code fragment that clears the memory at an offset from a1, then loads 1 into d2, then masks d2 with the memory at an offset from a1, then branches if the last result was not zero, then clears bit 1 of a fixed memory address, then returns.
- `set_joystick_bit`: Sets a specific bit in the CIAA_PRA register based on the value in d2, then returns immediately.
- `update_position_word_and_clamp`: Checks if a register value is zero, and branches if so.
- `frag_clr_moveq_and_move_rts_2`: Short code fragment that clears the memory at an offset from a1, then loads 15 into d2, then masks d2 with the memory at an offset from a1, then copies d2 to the memory at an offset from a1, then returns.
- `frag_clr_moveq_and_move_btst_beq_st_rts`: Short code fragment that clears the memory at an offset from a1, then loads 3 into d2, then masks d2 with the memory at an offset from a1, then copies d2 to the memory at an offset from a1, then tests bit 2 of the memory at an offset from a1, then branches if the last result was zero, then sets the memory at an offset from a1 to all ones, then returns.
- `frag_clr_rts_3`: Short code fragment that clears the memory at an offset from a1, then returns.
- `frag_clr_moveq_and_mulu_addi_move_rts`: Short code fragment that clears the memory at an offset from a1, then loads 15 into d2, then masks d2 with the memory at an offset from a1, then multiplies d2 by 72 (unsigned), then adds 596 to d2, then copies d2 to the memory at an offset from a1, then returns.
- `update_timer_value`: Clears a word at offset 4 of a1, performs bitwise AND with a word at offset 6 of a1 and d2, then updates memory locations based on the result.
- `frag_move_rts_5`: Short code fragment that copies a location in the code to the memory at an offset from a1, then returns.
- `frag_clr_moveq_and_move_btst_beq_st_rts_2`: Short code fragment that clears the memory at an offset from a1, then loads 15 into d2, then masks d2 with the memory at an offset from a1, then copies d2 to the memory at an offset from a1, then tests bit 2 of the memory at an offset from a1, then branches if the last result was zero, then sets the memory at an offset from a1 to all ones, then returns.
- `frag_clr_rts_4`: Short code fragment that clears the memory at an offset from a1, then returns.
- `frag_move_andi_beq_tst_bne_clr_rts`: Short code fragment that copies 36 to the memory at an offset from a1, then masks the memory at an offset from a1 with 15, then branches if the last result was zero, then tests d0, then branches if the last result was not zero, then clears the memory at an offset from a1, then returns.
- `frag_addq_clr_andi_neg_bsr_subq_rts_2`: Short code fragment that adds 4 to a1, then clears the memory at a1 (advancing a1), then masks the memory at a1 with 15, then negates the memory at an offset from a1, then calls another routine, then subtracts 6 from a1, then returns.
- `initialize_parameters`: Sets a word at offset 4 of a1 to 44, masks a word at offset 6 of a1 with 15, and clears multiple memory locations if the masked value is zero.
- `frag_subq_bne_clr_rts_2`: Short code fragment that subtracts 1 from the memory at a1, then branches if the last result was not zero, then clears the memory just below a1 (moving a1 down), then clears the memory at an offset from a1, then clears the memory at an offset from a3, then returns.
- `load_audio_data`: Sets a word at offset 4 of a1 to 48, tests d0 for zero, and if not zero, performs bitwise AND with 15, then loads data from memory.
- `frag_clr_rts_5`: Short code fragment that clears the memory at an offset from a1, then returns.
- `frag_subq_bne_clr_move_bra_2`: Short code fragment that subtracts 1 from the memory at a1, then branches if the last result was not zero, then clears the memory at an offset from a1, then copies the memory at an offset from a1 to the memory at an offset from a3, then jumps to another point in the code.
- `frag_clr_moveq_and_move_rts_3`: Short code fragment that clears the memory at an offset from a1, then loads 15 into d1, then masks d1 with the memory at an offset from a1, then copies d1 to the memory at an offset from a0, then clears the memory at an offset from a0, then returns.
- `frag_btst_beq_move_nop_rte`: Short code fragment that tests bit 1 of a fixed memory address, then branches if the last result was zero, then copies 25 to a fixed memory address, then copies a location in the code to a fixed memory address (2 times in a row), then copies 8192 to a fixed memory address, then pauses for one no-operation step, then returns from the exception.
- `configure_audio_channels_2`: Tests a bit in CIAB_ICR register, and if set, copies multiple audio channel parameters to custom registers and updates global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call_2`: Fills a memory range with zeros, then conditionally calls several functions based on their return values.
- `frag_lea_movea_moveq_btst_beq_jsr_move`: Short code fragment that loads the address of a location in the code into a5, then copies a fixed memory address to a6, then loads 0 into d1, then tests bit 0 of the memory at an offset from a6, then branches if the last result was zero, then calls another routine (2 times in a row), then copies d1 to the memory at an offset from a5, then returns.
- `call_open_library_3`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `frag_tst_beq_movea_jsr_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `call_open_library_4`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `frag_tst_beq_movea_jsr_rts_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `frag_move_beq_movea_bsr_bra_rts`: Short code fragment that copies the memory at an offset from a5 to d0, then branches if the last result was zero, then copies d0 to a0, then copies the memory at an offset from a0 to a0, then calls another routine, then jumps to another point in the code, then returns.
- `insert_node_list`: Initializes a new node structure and inserts it into a linked list at the head position.
- `frag_move_rts_6`: Short code fragment that copies a1 to the memory at an offset from a5, then returns.
- `remove_linked_list_node_2`: Searches a linked list for a matching value and removes that node from the list, returning the next node in the chain.
- `allocate_and_insert_node_2`: Saves registers on the stack, loads an address from memory into a register, adds twelve to a value in a register, calls a memory allocation routine via a pointer, and checks if the result is zero.
- `frag_movem_lea_bsr_tst_beq_movea_jsr_rt`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a5, then calls another routine, then tests d0, then branches if the last result was zero, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `assign_allocated_pointer_3`: Saves registers on the stack, loads an address from memory into a register, adds twelve to a value in a register, calls a memory allocation routine via a pointer, and checks if the result is zero.
- `clear_and_call_function_2`: Saves registers on the stack, loads an address from memory into a register, copies the address from another location to the first register, clears two long words at that location, calls another routine, and if the result is non-zero, calls a function pointer from a global variable.
- `store_function_result_3`: Calls a function pointer from a table and stores the returned value into a memory location offset from a register.
- `frag_movem_lea_movea_move_clr_jsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a5, then copies the memory at an offset from a5 to a6, then copies the memory at an offset from a0 to d1, then clears the memory at an offset from a0, then calls another routine, then restores saved registers from the stack, then returns.
- `conditionally_call_function_2`: Checks if three different values are non-zero and then calls a function pointer from a table.
- `conditionally_call_function_v2_2`: Checks if two values are non-zero and then calls a function pointer from a table.
- `execute_conditional_sequence_3`: Calls function pointers from a table based on a conditional result, updates a global variable with the return value, and returns the modified value.
- `initialize_and_clear`: Initializes a data structure by clearing memory, performs allocation and initialization calls, and handles error conditions with return values.
- `execute_function_sequence`: Saves registers, loads addresses from memory, performs function calls, and manages conditional execution based on return values.
- `frag_movea_move_bsr_cmp_bne_adda_unlk_m`: Short code fragment that copies the memory at an offset from a4 to a1, then copies the memory at an offset from a4 to d0, then calls another routine (2 times in a row), then compares d0 with the memory at an offset from a4, then branches if the last result was not zero, then adds 20 to a5, then removes the stack frame held in a5, then restores saved registers from the stack, then returns.
- `initialize_and_validate`: Initializes a 20-byte structure starting at an address in a5, then validates a returned value from another routine by comparing it with a stored result and returning the validated value or -1.
- `process_and_compare_values`: Loads an address from memory into a register, loads a long word from another location into a register, calls two routines with the register value, compares a long word at another location with the register value, and branches if they're not equal.
- `store_function_result_5`: Stores a value from a register at an offset in a structure, then calls a function and checks its result before returning.
- `init_and_iterate`: Initializes a global variable via a function pointer if it is zero, then iterates through a null-terminated string.
- `copy_string_and_null_terminate`: Initializes a global variable via a function pointer if it is zero, reads data into a buffer, copies characters from the buffer to another location until null terminator is found, and returns.
- `frag_btst_bne_move_rts_4`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then copies 64 to the memory at an offset from a6, then returns.
- `validate_tile_offset`: Checks if a specific tile offset matches a predefined value and returns immediately if not, otherwise continues execution.
- `initialize_and_loop`: Initializes multiple fields in a structure, then performs a loop that updates specific offsets based on conditional checks and calls another routine with varying parameters.
- `initialize_memory_blocks_2_2`: Clears a memory block, populates integer and short tables with incrementing values, calls functions, and updates pointers and status registers.
- `call_multiple_routines_with_pointers`: Calls another routine multiple times with different pointer values loaded into register a0.
- `frag_move_bmi_add_bra_rts_2`: Short code fragment that copies a0 to d0, then copies the memory at a0 to d1, then branches if the last result was negative, then adds d0 to the memory at a0 (advancing a0), then jumps to another point in the code, then returns.
- `frag_lea_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `frag_move_subq_add_adda_dbra_rts`: Short code fragment that copies a0 to d0, then copies the memory at a0 (advancing a0) to d1, then subtracts 1 from d1, then adds d0 to the memory at a0, then adds 6 to a0, then loops back while counting down d1, then returns.
- `check_and_toggle_flag`: Waits for a specific value in a register, toggles a byte flag, and then copies data from a structure to global variables while updating a counter.
- `call_multiple_routines`: Calls eight different routines in sequence without any conditional logic or return value handling.
- `frag_movea_move_moveq_dbra_rts`: Short code fragment that copies the memory at an offset from a5 to a4, then copies the memory at an offset from a4 to a4, then copies 15 to d0, then loads 255 into d1, then copies d1 to the memory at a4 (advancing a4), then loops back while counting down d0, then returns.
- `decrement_and_recurse_2`: Calls multiple routines sequentially, initializes a counter at offset 16 of a structure to a fixed value, decrements the counter in a loop, and exits when the counter becomes negative or a byte flag reaches zero.
- `initialize_memory_blocks_2`: Clears two distinct memory blocks of 16 longwords each, and sets up pointers to these blocks in global variables.
- `swap_and_clear`: Selects a pointer based on a flag, stores it and another pointer into global variables, then clears a memory block pointed to by the selected pointer.
- `update_sprite_attributes_2_2`: Performs bitwise operations on input values, calculates memory addresses, and writes rotated bit patterns to a memory block via a pointer.
- `update_sprite_attributes_2`: Calculates new coordinate and attribute data based on input values and table lookups, then writes the results into a structure.
- `expand_bitfield_data`: Reads compressed bitfield values from a table, performs bitwise operations to expand them, and writes the results to a memory buffer.
- `read_joystick_input_2`: Reads the joystick port and CIAA fire buttons to update internal status variables.
- `update_input_state_3`: Copies a byte from one memory location to another, clears a word value, and retrieves a word from a table based on a masked input.
- `adjust_table_values_9`: Iterates through two tables and adjusts values in the second table based on differences in specific bit nibbles and bits.
- `frag_bsr_movea_pea_move_rts`: Short code fragment that calls another routine, then copies the memory at an offset from a5 to a4, then pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a4 to the stack, then returns.
- `frag_bsr_movea_subq_move_cmp_bgt_rts`: Short code fragment that calls another routine (2 times in a row), then copies a fixed memory address to a0, then calls another routine, then copies a fixed memory address to a0, then subtracts 4 from the memory at an offset from a0, then copies the memory at an offset from a0 to d1, then compares d1 with the memory at an offset from a5, then branches if the comparison was greater, then returns.
- `frag_bsr_movea_pea_move_rts_2`: Short code fragment that calls another routine, then copies the memory at an offset from a5 to a4, then pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a4 to the stack, then returns.
- `frag_bsr_movea_addq_cmpi_blt_rts`: Short code fragment that calls another routine (2 times in a row), then copies a fixed memory address to a0, then calls another routine, then copies a fixed memory address to a0, then adds 4 to the memory at an offset from a0, then compares the memory at an offset from a0 with 312, then branches if the comparison was less, then returns.
- `check_and_update_flag`: Calls a routine to wait for a specific register value, loads a pointer from a structure, calls another routine to process data, then compares a word value with a memory location and returns a flag.
- `frag_moveq_subq_rts`: Short code fragment that loads 0 into d0, then subtracts 4 from the memory at an offset from a0, then returns.
- `check_and_load`: Loads a pointer from a global variable, checks if a specific value at an offset in the structure pointed to is less than or equal to 312, and if so, sets a register to 1 before returning.
- `frag_addq_moveq_rts`: Short code fragment that adds 4 to the memory at an offset from a0, then loads 0 into d0, then returns.
- `rotate_and_update`: Processes nested data tables to calculate rotated bit patterns and updates memory with computed values based on pointer offsets.
- `manage_counter_state`: Saves registers to the stack, calls another routine to initialize data, loads a pointer from global memory, reads a word value, performs conditional arithmetic on a counter, and handles linked list operations.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `frag_tst_beq_lea_movea_move_bsr_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then loads the address of a fixed memory address into a0, then copies the memory at an offset from a4 to a1, then copies 32 to d0, then calls another routine, then returns.
- `frag_tst_beq_lea_movea_move_bsr_rts_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then loads the address of a fixed memory address into a0, then copies the memory at an offset from a4 to a1, then copies 32 to d0, then calls another routine, then returns.
- `check_and_call_twice_3`: Checks if a byte at an offset from a pointer is non-zero and calls another routine twice if true.
- `check_and_call_twice_4`: Checks if a byte at an offset from a pointer is non-zero and calls another routine twice if true.
- `set_state_flags`: Checks a byte from a table and updates a global variable and another memory location based on its value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_loop`: Initializes global variables and hardware registers including DMA control and serial peripheral settings, then enters a main game loop that processes input conditions and updates game state based on structured data values.
- `initialize_structures`: Initializes multiple structures with predefined values based on a global variable.
- `frag_movem_jsr_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then calls another routine, then returns.
- `frag_tst_bne_bsr_movem_jsr_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then calls another routine, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_lea_btst_bne_move_rts`: Short code fragment that loads the address of a fixed memory address into a6, then tests bit 14 of the memory at an offset from a6 (2 times in a row), then branches if the last result was not zero, then copies 15 to the memory at an offset from a6, then returns.
- `update_game_state_2`: Processes game state updates based on input conditions and modifies counters for different game elements.
- `validate_and_copy_data`: Checks if a value at a calculated memory location is less than or equal to a global variable, and if so, copies data from a fixed table into that location.
- `set_status_values`: Sets multiple word values in a structure based on conditions, then calls functions to process data and update global variables.
- `process_data_conditions`: Processes conditional logic based on structure values, calls functions, updates global variables, and modifies structure fields before calling another routine.
- `initialize_random_sequence`: Initializes a random sequence by setting up global variables and performing bitwise operations on data read from a table, then stores the results into memory locations.
- `frag_bsr_lea_move_rts`: Short code fragment that calls another routine, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then calls another routine, then loads the address of a fixed memory address into a0, then calls another routine, then copies a2 to the memory at an offset from a5, then returns.
- `reset_global_variables`: Calls a function through a pointer and resets multiple global variables to zero or specific values when the function result is not four.
- `initialize_and_call`: Initializes two global variables with specific values, calls a routine multiple times, and then executes a function pointer from a register before returning.
- `setup_and_execute`: Sets a global variable to three, calls several routines including some library functions, stores a value at a specific memory offset, and then executes a function pointer from that location.
- `decompress_bitstream`: Reads compressed data from a source table using bitwise operations to reconstruct bytes and writes them to a destination buffer.
- `process_relocation_data`: Processes relocation data by reading a header value, validating a tag, and performing address adjustments on a linked list of entries before returning.
- `check_timer_value`: Reads a timer value from a hardware register and stores it in a global variable when a specific condition is met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_memory_and_call_3`: Fills a memory range with zeros using a loop, then conditionally calls multiple routines based on test results, returning zero if conditions are not met.
- `frag_lea_movea_moveq_move_rts`: Short code fragment that loads the address of a location in the code into a5, then copies a fixed memory address to a6, then loads 0 into d1, then copies d1 to the memory at an offset from a5, then returns.
- `call_open_library_7`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `frag_tst_beq_movea_jsr_rts_5`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `call_open_library_8`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `frag_tst_beq_movea_jsr_rts_6`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `frag_move_beq_movea_bsr_bra_rts_3`: Short code fragment that copies the memory at an offset from a5 to d0, then branches if the last result was zero, then copies d0 to a0, then copies the memory at an offset from a0 to a0, then calls another routine, then jumps to another point in the code, then returns.
- `insert_linked_list_node_2`: Initializes a new node structure with pointers and inserts it into a linked list stored at a specific offset within a structure.
- `remove_linked_list_node_3`: Searches a linked list for a matching entry, removes the node from the list, and returns the next node in the sequence.
- `allocate_and_insert_node_3`: Calls a memory allocation routine and, if successful, executes another routine to insert a node into a linked list.
- `free_memory_and_call`: Retrieves a memory size value, calls a routine to remove a node from a linked list, and then frees the memory block if the size is non-zero.
- `assign_allocated_pointer_4`: Allocates memory using a pointer, calls another routine if allocation succeeds, and stores the resulting pointer into a location pointed to by a register.
- `clear_and_call_function_3`: Clears two words at a memory location, calls a routine to remove a node from a linked list, and then frees memory if the previous routine returned a non-zero value.
- `store_function_result_4`: Calls a function pointer from a table using a library interface, stores the returned value into a memory location offset from a register, and returns.
- `zero_and_call_function_2`: Stores zero into a memory location offset from a register, then calls a function pointer from a table using a library interface.
- `validate_and_call`: Checks if three values are non-zero and then calls a function from a table.
- `check_and_call`: Validates that two values are non-zero and then invokes a function pointer located at a specific offset from a table.
- `execute_conditional_sequence_4`: Calls function pointers from a table based on a condition, and returns a value that may be modified by the results.
- `initialize_and_assign`: Clears a 20-byte block starting at an address in a5, then calls another routine and stores its result in memory.
- `move_and_branch_3`: Moves data between register pairs, stores values at memory locations with specific offsets, and performs conditional branching based on test results.
- `validate_and_branch`: Loads values from memory locations, calls two routines, compares results, and branches based on the comparison before returning.
- `initialize_and_execute`: Initializes a stack area by clearing bytes, calls another routine, and then executes a sequence of function pointers from a table based on return values and comparisons.
- `store_and_compare`: Stores a value from a register into a structure at a specific offset, then calls two routines and compares their results against the original value before returning the final result.
- `init_and_iterate_3`: Initializes a global variable via a function pointer if it is zero, then iterates through a null-terminated string starting at a0, and finally executes another function pointer.
- `copy_string_and_update`: Copies a null-terminated string from a table to a buffer and updates a global variable with the result of a hardware register read.
- `frag_btst_bne_move_rts_2`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then copies 64 to the memory at an offset from a6, then returns.
- `frag_move_andi_cmpi_bne_rts_2`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with an address, then compares d0 with 76800, then branches if the last result was not zero, then returns.
- `load_and_sync_hardware`: Loads several values from a table into local variables, updates hardware registers, and calls another routine twice after setting a global variable to a specific value.
- `restore_hardware_state`: Restores values from local variables back into a table, updates hardware registers, and modifies two global variables with bitwise operations.
- `wait_for_vblank_and_update`: Polls a hardware register until a specific bit pattern is detected, then updates a global variable and sets a hardware interrupt enable register.
- `wait_for_vblank_and_store`: Polls a hardware register until a specific bit pattern is detected, then stores a value into a global variable and updates a table entry.
- `sync_and_set_hardware`: Loops through two synchronization checks on a hardware register before updating a global variable and a hardware register.
- `sync_and_update_hardware`: Loops through two different hardware register bit patterns multiple times, updates a global variable, waits for a bit to clear, and then modifies another global variable.
- `update_dma_parameters_2`: Updates several global variables with register values, performs bit tests on a hardware register, and configures DMA transfer parameters based on conditional logic.
- `update_coordinates`: Updates coordinate values in a structure based on calculated differences and performs boundary checks on horizontal and vertical positions.
- `update_dma_parameters`: Updates several global variables with register values, performs bit tests on a hardware register, and configures DMA transfer parameters before looping through a table.
- `frag_move_btst_bne_movem_clr_rts`: Short code fragment that copies a4 to d0, then tests bit 14 of the memory at an offset from a6 (2 times in a row), then branches if the last result was not zero, then copies 0 to the memory at an offset from a6, then copies several registers to or from memory, then copies 2583 to the memory at an offset from a6, then clears the memory at an offset from a5, then returns.
- `configure_dma_transfer`: Sets up DMA transfer parameters by loading addresses and values from global variables, clears a status byte, and configures transfer counts based on conditional logic.
- `adjust_position_and_update`: Adjusts position values based on comparison results, updates memory locations with calculated addresses, and sets global variables to indicate state changes.
- `update_position_and_sync`: Updates position values based on comparison results, calculates new addresses from a table, and sets global variables to indicate state changes.
- `update_sprite_attributes_3`: Updates sprite attribute data by reading from a table and writing to a memory buffer, managing state transitions for the data pointer and handling coordinate offsets based on control flags.
- `initialize_data_blocks_2`: Sets up two memory blocks by initializing their pointer fields and clearing thirty-two bytes of data in each block using a loop.
- `compute_position_offsets`: Calculates and stores position-related offsets and coordinates based on input values and table lookups, then updates memory pointers with computed addresses and values.
- `render_sprite_pixels`: Calculates sprite pixel data for rendering by performing bit operations on coordinate values, accesses a lookup table, and writes combined pixel data to memory locations.
- `update_sprite_attributes_4`: Calculates new screen coordinates and sprite attribute data for an object based on velocity and position inputs.
- `decompress_bitstream_2`: Reads compressed data using a variable-length encoding scheme from a source table and writes decompressed values to a destination buffer.
- `update_track_data`: Updates multiple track-related data entries by loading values from specific memory locations and storing them into global variables, then performs a loop that modifies a sequence of memory addresses with incrementing values.
- `frag_move_bmi_add_bra_rts_3`: Short code fragment that copies a0 to d0, then copies the memory at a0 to d1, then branches if the last result was negative, then adds d0 to the memory at a0 (advancing a0), then jumps to another point in the code, then returns.
- `initialize_track_elements`: Initializes track element data structures by setting specific values and updating pointers based on calculated offsets, then performs bitwise operations and conditional updates on related data.
- `update_bitmask_state_2`: Updates specific bits within two data structures based on the bitwise decomposition of a value in a register, performing conditional bitwise OR and AND operations on various offsets to synchronize or toggle state flags between two memory blocks.
- `update_game_state_3`: Updates a game state counter and manages conditional logic based on its value, while initializing and populating data structures with predefined values.
- `frag_lea_movea_move_clr_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a5, then copies a5 to a0, then copies 1971 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `check_and_set_flag`: Reads a value from a global variable, masks it with a fixed address, compares it to a fixed address, and if not equal, loops back to recheck; when equal, it checks again against a fixed address and sets a flag in memory.
- `initialize_track_data_5`: Initializes two tables with incrementing values starting from zero, then updates global variables by adding their addresses to their current values, and finally calls another routine.
- `initialize_rendering_data`: Initializes rendering-related data tables and global pointers when a specific condition is met, including copying color data, setting up memory mappings, and populating offset arrays for graphics processing.
- `initialize_track_data_4`: Initializes multiple global variables with values from a structure referenced through a5, then performs conditional logic based on values at specific offsets to set additional variables before returning.
- `multiply_table_values`: Multiplies values in a table by four using a loop that decrements a counter and advances a pointer through the table entries.
- `initialize_object`: Initializes object properties based on state and index values, populates object structure with calculated coordinates and flags, then performs conditional data copying and updates.
- `initialize_object_structure_2`: Clears a block of memory starting at the address in a1 and populates it with default integer and byte values, including specific constants and zero-initializations for various fields.
- `setup_and_execute_2`: Writes fixed values to a global memory location and clears a following 4-byte field, then calls another routine.
- `frag_lea_move_bra_4`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then jumps to another point in the code.
- `frag_lea_move_bra_5`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then jumps to another point in the code.
- `frag_lea_move_bra_6`: Short code fragment that loads the address of a fixed memory address into a0, then copies d1 to the memory at an offset from a0, then copies d1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_lea_addi_bra`: Short code fragment that copies 35884 to d0, then loads the address of a fixed memory address into a0, then copies a fixed memory address to d1, then adds 48 to d1, then copies d1 to the memory at an offset from a0, then loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_lea_addi_bra_2`: Short code fragment that copies 35853 to d0, then loads the address of a fixed memory address into a0, then copies a fixed memory address to d1, then adds 48 to d1, then copies d1 to the memory at an offset from a0, then loads the address of a fixed memory address into a0, then copies 14662 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_movea_move_lea_bra_5`: Short code fragment that copies the memory at an offset from a5 to a1, then copies the memory at an offset from a1 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `setup_and_execute_2_2`: Loads a pointer from offset 4 of a structure into a register, calculates an address by adding an offset to that pointer, reads a word value from the calculated address, and stores a fixed long value at a global memory location.
- `frag_movea_move_lea_bra_6`: Short code fragment that copies the memory at an offset from a5 to a1, then copies the memory at an offset from a1 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_movea_move_lea_bra_7`: Short code fragment that copies the memory at an offset from a5 to a0, then copies the memory at an offset from a0 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_movea_move_lea_bra_8`: Short code fragment that copies the memory at an offset from a5 to a0, then copies the memory at an offset from a0 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `clear_and_process`: Clears multiple memory locations at specific offsets from a5, then initializes a 64-element array by clearing its elements.
- `wait_for_hardware_status_3`: Sets a hardware register to enable DMA channels, waits for a specific bit pattern to appear in a video status register, and then stores a long value from a structure into another hardware register.
- `frag_movem_jsr_lea_move_rte`: Short code fragment that saves registers on the stack, then calls another routine, then loads the address of a fixed memory address into a6, then copies 32 to the memory at an offset from a6, then restores saved registers from the stack, then returns from the exception.
- `frag_lea_move_lsr_mulu_adda_add_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies d0 to d2, then copies d1 to d3, then shifts d2 right by 4, then shifts d3 right by 4, then multiplies d3 by 370 (unsigned), then adds d3 to a0, then adds d2 to d2, then adds d2 to a0, then returns.
- `process_input_data`: Processes input data from a buffer by reading bytes and performing conditional operations based on the byte values, updating various structure offsets during the process.
- `frag_move_add_pea_rts`: Short code fragment that copies d2 to d3, then adds d3 to d3 (2 times in a row), then pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then returns.
- `validate_and_update_2`: Tests conditions on data at specific offsets, performs comparisons with immediate values, and updates memory locations with calculated or fixed values based on branching logic.
- `calculate_difference`: Calculates a difference between two values and returns a modified result based on comparison and arithmetic operations with carry handling.
- `frag_tst_bne_move_lsr_sub_rts`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was not zero, then copies the memory at an offset from a4 to d3, then shifts d3 right by 6, then subtracts d3 from the memory at an offset from a4, then copies 64 to the memory at an offset from a4, then returns.
- `initialize_timer_fields`: Sets specific timer-related fields in a structure to predefined values and performs conditional operations based on comparison results with other data structures.
- `update_timer_values`: Updates timer-related structure fields by setting a specific value, performing bit manipulation on a register, and conditionally calling a function based on a status byte.
- `frag_move_lsl_clr_cmpi_blt_st_rts`: Short code fragment that copies the memory at an offset from a4 to d3, then shifts d3 left by 1, then copies d3 to the memory at an offset from a4, then clears the memory at an offset from a4, then compares the memory at an offset from a4 with 3, then branches if the comparison was less, then copies 3 to the memory at an offset from a4, then sets the memory at an offset from a4 to all ones, then returns.
- `adjust_position_and_state`: Adjusts position values in a structure by performing arithmetic operations, clears certain flags, updates multiple structure fields, and conditionally calls a function based on a status byte.
- `initialize_and_notify`: Initializes structure fields when specific conditions are met, performs conditional function calls, and updates global variables with fixed values based on a status byte.
- `update_object_counter`: Checks a condition in a global variable, updates another global variable, decrements a counter, and adjusts a value in memory based on arithmetic operations.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `update_object_properties`: Updates object properties in a table by reading from a buffer, performing arithmetic operations, and calling a function when a specific condition is met.
- `frag_cmpi_bne_move_rts_3`: Short code fragment that compares the memory at an offset from a4 with 41, then branches if the last result was not zero, then copies 40 to the memory at an offset from a4, then returns.
- `validate_and_update_score`: Validates input data against multiple structure values and updates score-related fields when validation passes, then calls a function after reaching a specific count.
- `frag_cmpi_bne_move_rts_4`: Short code fragment that compares the memory at an offset from a4 with 40, then branches if the last result was not zero, then copies 42 to the memory at an offset from a4, then returns.
- `frag_andi_addi_sub_move_clr_st_rts_2`: Short code fragment that masks d1 with 65520, then adds 16 to d1, then subtracts the memory at an offset from a3 from d1, then copies d1 to the memory at an offset from a4, then clears the memory at an offset from a4, then sets d7 to all ones, then returns.
- `frag_andi_sub_move_clr_st_rts_2`: Short code fragment that masks d1 with 65520, then subtracts the memory at an offset from a3 from d1, then copies d1 to the memory at an offset from a4, then clears the memory at an offset from a4, then sets d7 to all ones, then returns.
- `calculate_and_set_properties`: Reads data from tables using indexed addressing, performs bit manipulation and arithmetic operations, and updates multiple fields in a structure with calculated values.
- `frag_move_sub_cmpi_blt_bra`: Short code fragment that copies the memory at an offset from a4 to d2, then subtracts d6 from d2, then compares d2 with 65520, then branches if the comparison was less, then compares d2 with 0, then branches if the comparison was less, then compares d2 with 16, then branches if the comparison was less, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies 65535 to d3, then jumps to another point in the code.
- `frag_move_bra_11`: Short code fragment that copies 1 to d3, then jumps to another point in the code (2 times in a row).
- `frag_sub_clr_bmi_cmpi_bpl_move_bra_2`: Short code fragment that subtracts d0 from d1, then clears d2, then clears d3, then subtracts d1 from d2, then branches if the last result was negative, then compares d2 with 16, then branches if the last result was not negative, then copies 1 to d3, then jumps to another point in the code.
- `set_boundary_conditions`: Sets boundary conditions for structure fields based on register comparisons, updates multiple offsets with calculated values, and sets a status flag when specific conditions are met.
- `process_input_data_2`: Initializes flags in a structure, calls multiple routines to update state counters and process data, reads a word value from the structure, and performs conditional arithmetic operations on fields based on the read value.
- `clear_collision_flags`: Clears collision detection flags in multiple object structures and performs bitwise operations on coordinate differences to determine collision status, then updates object attributes based on comparison results.
- `update_object_position`: Updates object position and attribute data based on comparison with reference values, then performs bitwise checks on coordinate offsets to determine movement behavior.
- `update_object_data_2`: Updates object position and attribute data based on comparison with reference values, then performs bitwise checks on coordinate offsets to determine matching states.
- `update_structure_field_2`: Reads a byte value from a structure, performs bit masking and shifting operations to extract a short value, stores the result at offset 94 of the structure, and conditionally updates other fields based on comparison results.
- `update_structure_fields_5`: Reads a long value from a structure, performs bit shifts and masks to extract two short values, stores the first result at offset 56 of the structure, and conditionally stores the second result at offset 32 when a byte at offset 43 is zero.
- `update_clamped_value_2`: Reads a short value from a structure, calls another routine to adjust and clamp the value, then stores the result back to the structure.
- `adjust_and_clamp_value`: Reads multiple fields from a structure, performs arithmetic operations on a value based on conditional logic, and clamps the result between bounds defined by structure fields before returning the adjusted value.
- `decrement_state_counter_2`: Decrements a word value at offset 66 of a structure after checking multiple conditions, including a byte value at offset 82 and word values at offsets 16 and 22, then calls another routine to process the decrement.
- `process_state_counters_2`: Iterates through a table to decrement a counter and updates various fields in a structure based on comparisons with table-referenced values.
- `update_speed_counter`: Updates a speed counter based on table-referenced values and internal state flags, with conditional adjustments for acceleration and deceleration.
- `decrement_counter`: Decrements a counter at offset 64 in a structure and resets multiple fields to default values when the counter reaches zero or a specific threshold.
- `update_input_state_4`: Processes input data by reading bytes and performing conditional operations based on the byte values, updating various structure fields.
- `update_accumulated_totals_2`: Performs vector multiplication operations and accumulates results into 32-bit fields in a structure.
- `perform_vector_multiplication`: Multiplies two signed 16-bit values from tables with a register value and accumulates the results into a 32-bit value.
- `adjust_position_and_counter`: Decrements a word counter at offset 92 in a structure, sets a byte at offset 42 to 255 if the counter is zero, and performs arithmetic on a short value at offset 40 based on the absolute difference from another field.
- `calculate_movement_state_2`: Determines a direction value based on coordinate differences and a counter, updates multiple structure fields including a counter and concatenated values, and performs conditional logic based on state flags.
- `decrement_counters`: Decrements a counter in a table and resets multiple fields based on comparisons between a global variable and a value at an offset, with conditional logic for different states.
- `process_multiple_structures`: Processes multiple structures sequentially by calling another routine with different address offsets, only executing the loop when a specific word value equals four.
- `rotate_global_bit_3`: Performs a bitwise rotation on the complement of a global variable and returns the result.
- `update_input_flags`: Reads bits from a global variable and updates table entries based on the bit states, then sets a specific flag in another table entry.
- `update_input_flags_2`: Reads bits from a global variable at offset 24, updates flags in a table at offset 48 based on the bit states, and sets a value in another global variable at offset 50.
- `frag_cmpi_bne_lea_beq_jsr_bra`: Short code fragment that compares the memory at an offset from a5 with 4, then branches if the last result was not zero, then loads the address of a fixed memory address into a6, then compares the memory at an offset from a5 with 0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `check_and_set_flags`: Checks multiple conditions involving structure fields and global variables, then sets specific flags and values based on the results.
- `handle_input_and_render`: Handles input processing and rendering logic by checking joystick states, updating sprite attributes, and managing state transitions based on comparison results and bit tests.
- `process_position_coordinates_2`: Processes coordinate values by performing arithmetic operations on input parameters, applies boundary checks, calculates bit patterns for movement, and updates memory pointers with calculated values.
- `process_position_coordinates`: Processes coordinate values by performing arithmetic operations on input parameters, applies boundary checks, calculates bit patterns for memory operations, and updates position-related data in a structure.
- `update_position_data_2`: Processes position coordinates and updates related data structures based on input values, including boundary checks and arithmetic operations.
- `process_track_segments`: Processes track segment data from a table, calculating positions and updating global variables with computed values for rendering or simulation purposes.
- `read_joystick_input_3`: Reads joystick status from hardware registers and sets a byte value based on button states, then stores the result at offset 178 of a structure pointed to by a4.
- `find_closest_object`: Scans through a table of objects to find the closest one to a reference point, calculates distance using absolute differences in coordinates, and stores both the closest object pointer and its distance in global variables.
- `update_collision_state`: Updates collision detection states for two game objects based on their positions and orientations, then increments a counter in a global variable.
- `update_animation_frame`: Updates animation frame counters and sets sprite coordinates based on current state and global variable values.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 421 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `swap_and_update_pointers`: Swaps two pointers based on a condition at offset 63 of a structure, stores the swapped pointers into global variables, then calls three routines and sets a specific value in memory.
- `compare_and_copy_data`: Compares a calculated value against entries in a table and performs recursive operations to transfer data based on the comparison results.
- `copy_vertex_data`: Copies vertex data from a global table to a destination buffer, processing multiple pairs of 32-bit values.
- `construct_sprite_descriptor`: Builds a sprite descriptor by combining input values with data from a lookup table and copying vertex information.
- `calculate_composite_value_2`: Performs arithmetic operations on input values and returns a concatenated 32-bit result containing bitwise flags.
- `compute_sprite_position`: Calculates sprite position coordinates and stores scaled values into a structure based on input parameters.
- `initialize_game_state_3`: Stores word values from registers into specific offsets of a structure, calls multiple routines to update game state, sets a memory location to a fixed value, and toggles a bit flag in the structure.
- `check_status_bits_2`: Loads a byte from a specific offset of a structure, performs bit tests on that byte, and conditionally calls multiple routines based on the results of those bit tests.
- `scale_coordinate_deltas`: Calculates scaled differences between two sets of coordinates, storing the results and direction flags in a structure.
- `initialize_sprite_data_3`: Initializes sprite data structures by setting up vertex coordinates and configuring sprite properties based on input, then processes a grid of data points to update counters and store coordinate values.
- `initialize_sprite_data_2`: Initializes sprite data structures by linking to a free list, setting up vertex coordinates, and configuring sprite properties based on input values.
- `manage_sprite_animation`: Processes sprite animation data by updating counters and memory locations, handles conditional logic for animation states, and manages linked list operations for sprite data structures.
- `update_tile_mask`: Calculates a bit position based on input coordinates and sets that bit in a 32-bit word within a table.
- `update_sprite_data`: Processes sprite animation states by decrementing counters and checking collision boundaries, updating sprite properties when conditions are met.
- `perform_collision_check`: Executes collision detection logic by comparing various structure members and returning early if conditions are not met.
- `frag_movem_tst_beq_rts_2`: Short code fragment that restores saved registers from the stack, then tests d0, then branches if the last result was zero, then returns.
- `adjust_masked_offset_three`: Performs boundary checks on structure members and updates a masked offset field by adding or subtracting three.
- `validate_and_adjust_masked_offset`: Performs multiple boundary checks using arithmetic comparisons on values within two structures, then updates a masked offset field by either adding or subtracting two based on the comparison of calculated results.
- `adjust_masked_offset_three_alt`: Checks specific structure bounds and updates a masked offset field by three using a different set of offsets, with similar logic to other boundary validation routines.
- `update_masked_offset_field`: Performs a series of boundary checks between two structures and, if conditions are met, updates a masked offset field by adding or subtracting three while also updating several bytes within a structure.
- `initialize_structure`: Initializes a structure element by setting its first byte to 255, calculating position values based on input coordinates, and calling a rendering function.
- `frag_lea_move_movea_clr_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then copies 2 to d0, then copies the memory at a0 (advancing a0) to a1, then clears the memory at an offset from a1, then loops back while counting down d0, then returns.
- `initialize_and_update_structure`: Initializes a structure element by setting its first byte to 255, calculates and stores position values based on input coordinates, and updates counters and flags in the structure.
- `process_active_elements`: Iterates through active elements in a data structure, calls an external function with calculated values, decrements a counter, and manages element state transitions based on counter values.
- `calculate_scaled_deltas_2`: Calculates scaled differences between coordinate values, determines direction flags based on comparison results, stores the absolute values in a structure, and updates position-related fields with calculated deltas.
- `calculate_difference_2`: Calculates the difference between two word values, performs conditional arithmetic operations, and stores results in memory locations.
- `calculate_scaled_deltas`: Performs arithmetic operations on coordinate values, applies boundary checks, calculates scaled differences, and stores results in global variables while handling conditional negations based on bit flags.
- `increment_table_values_3`: Iterates through two arrays of words and adjusts the values in the second array to incrementally approach the values in the first array based on bitwise differences at different bit positions.
- `manage_animation_counters`: Manages animation frame counters and timing values, updates global variables with computed results from table lookups, and performs bitwise operations on status flags during rendering cycles.
- `update_global_values`: Updates global variables and performs bitwise operations on registers while managing state transitions based on comparison results.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:TurboTrax.Exe#unpacked

- `initialize_system_and_manage_resources`: Initializes system libraries and clears global data before managing memory allocation and resource loading.
- `setup_sprite_attributes`: Sets up sprite attributes by iterating through a table, calculating values based on input parameters, and writing specific values into memory locations associated with sprite data.
- `frag_movem_jsr_bsr_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then calls another routine, then returns.
- `poll_global_flag_and_recurse_2`: Polls a global variable in a loop until it becomes zero, then calls itself twice recursively.
- `wait_and_set_dma_2`: polls a bit in a global variable until it is cleared, then writes a value to that variable.
- `format_timer_string`: Converts numeric values from a data structure into a formatted string of characters.
- `search_and_overwrite_string`: Searches through a table of data structures for a specific 4-byte pattern.
- `update_state_parameters_2`: Sets several values in a structure based on specific status checks and then stores values from a function call into a table.
- `update_status_flags_2`: Checks a status value and updates various global variables or increments a value in a structure before calling another routine.
- `decompress_bitstream_data_8`: Performs bitstream decompression by reading encoded values and applying bitwise shifts and arithmetic to reconstruct bytes.
- `store_pointer_to_structure_2`: Calls another routine and stores the resulting value into a structure offset.
- `reset_system_globals`: Calls a function via a pointer, checks its result, and if not equal to four, resets multiple global variables to zero or specific values based on input parameters.
- `load_logo_data`: Performs a sequence of data decompression, memory relocation, and hardware synchronization tasks to load a logo file.
- `dispatch_callback_sequence`: Sets a global variable, performs several subroutine calls, stores a value into a structure, and executes a function pointer stored at that location.
- `decompress_bitstream_data_4`: Reads compressed bitstream data from a source table, performs bitwise operations to reconstruct bytes, and writes the decompressed bytes to a destination buffer.
- `relocate_hunk_data`: Iterates through a memory structure to find a specific hunk type and updates relocation addresses within that block.
- `process_bitstream_data`: Performs bitwise shifts and arithmetic operations on a data stream to extract and store values into a table.
- `update_bitstream_state`: Updates a global state variable using bitwise shifts and arithmetic based on a carry flag.
- `initialize_library_pointers`: Clears a block of memory with zeros and attempts to open multiple system libraries.
- `clear_resource_pointers`: Clears a global byte and iterates through a linked list to call specific function pointers.
- `frag_lea_movea_moveq_move_rts_2`: Short code fragment that loads the address of a location in the code into a5, then copies a fixed memory address to a6, then loads 0 into d1, then copies d1 to the memory at an offset from a5, then returns.
- `call_open_library_5`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `frag_tst_beq_movea_jsr_rts_3`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `call_open_library_6`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `frag_tst_beq_movea_jsr_rts_4`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a5 to a1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `frag_move_beq_movea_bsr_bra_rts_2`: Short code fragment that copies the memory at an offset from a5 to d0, then branches if the last result was zero, then copies d0 to a0, then copies the memory at an offset from a0 to a0, then calls another routine, then jumps to another point in the code, then returns.
- `insert_linked_list_node`: Initializes a new node and inserts it into a linked list stored within a structure.
- `unlink_node`: Removes a node from a linked list by updating the previous node's pointer to skip the current node, and returns the value stored in the removed node's data field.
- `allocate_and_insert`: Allocates memory using a pointer, and if successful, inserts a new node into a linked list.
- `free_memory_if_needed`: Calls another routine and, if the result is non-zero, frees memory using a pointer from a table; otherwise it returns.
- `assign_allocated_pointer_2`: Calls a memory allocation routine via a pointer and stores the resulting pointer into a location pointed to by a register.
- `clear_and_free_2`: Clears two longwords at a memory location pointed to by a register, then calls a routine and frees memory if the routine returns a non-zero value.
- `store_function_result_2`: Calls a function pointer from a table and stores the returned value into a memory location offset 4 from the address in a0.
- `zero_and_call_function_3`: Calls another routine, stores zero into a memory location offset from a register, and then calls a function pointer from a table.
- `validate_and_invoke_3`: Checks if three values are non-zero and invokes a function pointer from a table.
- `validate_and_invoke_4`: Checks if two values are non-zero and then invokes a function pointer from a table.
- `process_file_lock`: Acquires a file lock using a system call, checks its validity, and performs conditional operations based on the result before releasing the lock.
- `initialize_and_process`: Initializes a memory block with zeros, calls a routine to process data, and stores the result in a global variable if it's non-negative.
- `compare_and_return_result_2`: Writes values to a memory location, calls a routine, and returns a value if it matches a specific entry in a structure.
- `compare_and_call_function_2`: Calls another routine, compares its result to a value in a structure, and conditionally calls a function pointer from a table.
- `clear_and_execute_sequence`: Clears a stack area, calls a routine, and executes a sequence of function pointers from a table based on return values and comparisons.
- `validate_and_compare`: Calls a routine to validate input, and if successful, calls two more routines to process data and compares their results against a register value before returning a combined result.
- `init_and_iterate_2`: Initializes a global variable using a function pointer if it equals zero, then iterates through a null-terminated string starting at a0.
- `read_and_copy_string`: Initializes a global variable via a function pointer if it is zero, reads data into a buffer, and copies characters from the buffer to another location until null terminator is found.
- `frag_btst_bne_move_rts_3`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then copies 64 to the memory at an offset from a6, then returns.
- `frag_move_andi_cmpi_bne_rts_3`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with an address, then compares d0 with 76800, then branches if the last result was not zero, then returns.
- `initialize_hardware_registers_2`: Copies values from a table into local variables, updates hardware registers, and calls another routine twice after setting specific register values.
- `restore_hardware_state_2`: Copies data from a table to memory locations, updates hardware registers with values from the table, and modifies two global variables using bitwise operations.
- `install_interrupt_handler_2`: Writes a routine address into a table, waits for a specific hardware register value, and then updates a global variable with a constant value.
- `wait_for_vblank_and_store_2`: Polls a hardware register until a specific bit pattern is detected, then stores a value into a global variable and updates a table entry.
- `sync_and_set_hardware_2`: Performs twelve iterations of vertical retrace synchronization by polling a global variable's value, then configures interrupt and DMA control registers.
- `wait_for_vblank_and_update_dma`: Waits for vertical blank periods by polling a global variable until specific values are detected, then configures hardware registers to enable interrupts and set up DMA channels.
- `update_offset_data_4`: Performs arithmetic on structure members and updates them based on comparisons between values in a structure and a table.
- `update_offset_data_limited`: Updates several structure members using arithmetic and conditional checks against a constant value.
- `populate_offset_data_table`: Initializes several global short values and then iterates through a loop to populate a structure with values derived from a table and an incrementing index.
- `reset_table_values_2`: Checks a bit in a memory location and, if set, clears several fields within a structure pointed to by a register.
- `populate_structure_from_table`: Initializes a structure by iterating through a loop and adding values from a table to a base value.
- `update_offset_data`: Calculates differences between short values, updates internal offsets and pointers, and modifies structure fields based on conditional checks.
- `update_wrapped_offset`: Calculates a wrapped value from a short, updates a structure with new offsets and a pointer if a threshold is exceeded.
- `update_sprite_attribute_table`: Updates a list of sprite or object attributes by iterating through a data table and writing values to a memory buffer.
- `initialize_data_blocks`: Sets pointers and zero-valued fields within two distinct memory blocks, then clears thirty-two bytes of data in each block using a loop.
- `update_calculated_values`: Performs arithmetic operations using values from a table and various offsets to update several fields in a structure, including a pointer swap based on a conditional byte check.
- `render_object_attributes`: Calculates sprite attributes and performs bitwise operations to update memory based on object coordinates and velocity.
- `update_object_sprite_attributes`: Calculates new screen coordinates and sprite attribute data for an object based on velocity and position inputs.
- `decompress_bitstream_data_3_2`: This routine performs a bitstream decompression of a data block using a variable-length encoding scheme.
- `initialize_offset_table`: Populates a table with a sequence of incrementing values starting from a specific constant.
- `frag_move_bmi_add_bra_rts`: Short code fragment that copies a0 to d0, then copies the memory at a0 to d1, then branches if the last result was negative, then adds d0 to the memory at a0 (advancing a0), then jumps to another point in the code, then returns.
- `initialize_asset_metadata_tables_3`: Populates multiple memory tables with specific multi-byte identifiers and status flags.
- `update_bitmask_state`: Updates specific bits within two data structures based on the bitwise decomposition of a value in a register.
- `update_animation_state`: Updates animation state variables based on timer values and toggles flags, then performs sprite data initialization using lookup tables.
- `clear_memory_block_3`: Fills a block of memory with zeros using a loop.
- `wait_for_hardware_status`: Polls a memory location for specific bit patterns before storing a pointer and a word value to a table.
- `initialize_data_tables_3`: Populates a table of pointers with incrementing offsets and initializes another table with a sequence of incrementing values.
- `initialize_asset_tables`: Performs bitwise transformations on a memory block, copies data from a source table to a destination buffer using a repeating pattern, and populates multiple global tables with incrementing integer sequences.
- `update_object_metadata`: Copies multiple values from a pointer-based structure into several global variables and updates a table of pointers and values based on a conditional check.
- `scale_table_values`: Iterates through a table of words, multiplying each value by four.
- `populate_object_data_structures`: Initializes a data structure by copying values from a table based on indices provided in a source structure.
- `initialize_object_structure`: Clears a large block of memory and populates a data structure with various default integer and byte values.
- `process_pak_files`: Executes a complex sequence of bitstream decompression, hunk relocation, and conditional hardware status updates for various data files.
- `frag_lea_move_bra`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then jumps to another point in the code.
- `frag_lea_move_bra_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then jumps to another point in the code.
- `frag_lea_move_bra_3`: Short code fragment that loads the address of a fixed memory address into a0, then copies d1 to the memory at an offset from a0, then copies d1 to a fixed memory address, then jumps to another point in the code.
- `iterate_audio_resource_loading`: Executes a loop that repeatedly calls a routine and an indirect function pointer based on the return value of the first call.
- `iterate_and_execute_subroutines`: Updates a global variable and a character value in a table before entering a loop that repeatedly calls three subroutines and a function pointer.
- `frag_movea_move_lea_bra`: Short code fragment that copies the memory at an offset from a5 to a1, then copies the memory at an offset from a1 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `update_global_pointer_and_execute`: Calculates an address using a pointer-based offset and stores it in a global variable, then enters a loop that repeatedly calls various routines and updates a byte in memory via the calculated address.
- `frag_movea_move_lea_bra_2`: Short code fragment that copies the memory at an offset from a5 to a1, then copies the memory at an offset from a1 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_movea_move_lea_bra_3`: Short code fragment that copies the memory at an offset from a5 to a0, then copies the memory at an offset from a0 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_movea_move_lea_bra_4`: Short code fragment that copies the memory at an offset from a5 to a0, then copies the memory at an offset from a0 to d0, then loads the address of a fixed memory address into a0, then copies 0 to the memory at an offset from a0, then jumps to another point in the code.
- `initialize_data_structures_7`: Clears numerous fields within a structure, zeros out a global array, and initializes a table of function pointers.
- `wait_for_hardware_status_2`: Sets a global variable, waits for a specific bit pattern to appear in a hardware register, and then stores a long value from a pointer into another global variable.
- `call_and_update_global`: Calls another routine and then stores a constant value into a global variable.
- `frag_lea_move_lsr_mulu_adda_add_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies d0 to d2, then copies d1 to d3, then shifts d2 right by 4, then shifts d3 right by 4, then multiplies d3 by 370 (unsigned), then adds d3 to a0, then adds d2 to d2, then adds d2 to a0, then returns.
- `update_structure_data`: Modifies several fields within a structure based on conditional checks and performs multiple subroutine calls to process data.
- `execute_proximity_logic`: This routine calls a function that evaluates a value within a structure to update a global variable based on its magnitude.
- `check_proximity_conditions`: Tests various conditions related to proximity and position, and branches based on comparison results.
- `update_speed_limit`: Updates a speed limit value based on conditions and stores a status byte, then calls another routine to process the updated values.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `frag_tst_bne_move_lsr_sub`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was not zero, then copies the memory at an offset from a4 to d3, then shifts d3 right by 6, then subtracts d3 from the memory at an offset from a4.
- `frag_move_rts_2`: Short code fragment that copies 64 to the memory at an offset from a4, then returns.
- `set_structure_parameters`: Sets multiple fields in a structure and performs bitwise masking on a value if certain conditions are met.
- `set_mask_and_update_flag`: Sets a 16-bit value to all ones at an offset from a pointer, performs a bitwise mask on a value, and updates a global variable if a specific byte matches a constant.
- `update_structure_fields`: shifts a word value, clears a word, and conditionally updates two fields in a structure based on a comparison.
- `update_structure_fields_2`: Updates several fields in a structure, including a subtraction based on a bit-shifted value, bitwise shifts, and conditional assignments based on a byte check.
- `copy_data_and_set_globals`: Copies a 16-bit value from one offset to another within a structure and updates three global variables if specific conditions are met.
- `check_and_decrement`: Checks a condition in a global variable and updates multiple counters, then decrements values in memory locations based on comparison results.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_and_copy_structure`: Decrements one value and increments another within a structure, then copies a 16-bit value from a table-indexed location into a pointer-based structure.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares the memory at an offset from a4 with 41, then branches if the last result was not zero, then copies 40 to the memory at an offset from a4, then returns.
- `process_boundary_conditions`: Checks a character flag and performs complex arithmetic involving a global table to update a 32-bit value and several byte fields within a structure.
- `frag_cmpi_bne_move_rts_2`: Short code fragment that compares the memory at an offset from a4 with 40, then branches if the last result was not zero, then copies 42 to the memory at an offset from a4, then returns.
- `calculate_offsets`: calculates and stores two different values into a structure using arithmetic between registers and a table.
- `calculate_masked_offset`: calculates a value by subtracting a table entry from a masked register value and stores it in a structure.
- `calculate_masked_offset_alt`: calculates a value by subtracting a value from a table at an offset from a masked register and stores it in a structure.
- `subtract_and_store_value`: Subtracts a value from a pointer-referenced short and stores the result in a structure offset.
- `update_structure_from_table`: Compares a value from a table against a register and updates two different structure offsets based on the comparison results.
- `update_structure_with_threshold`: Checks a table value against a register and updates structure offsets if specific inequality conditions are met.
- `update_offset_values`: Compares values from a table against nibbles in two input registers and updates specific offsets in a memory structure using the difference between a register and a table entry.
- `update_structure_on_increase`: Updates structure offsets using a table value and a register if the register value is less than or equal to the table value.
- `calculate_offset_from_table`: Uses a value from a table and arithmetic on input registers to calculate and store multiple 16-bit offsets into a structure based on several conditional branches.
- `update_structure_offsets`: Calculates and stores 16-bit values into a structure by combining masked register values with entries from a table and subtracting values from another pointer.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d2 to d1, then returns.
- `frag_subq_add_move_movea_andi_bra`: Short code fragment that subtracts 4 from d1, then adds d1 to d1, then copies a location in the code to d6, then adds d1 to d1, then copies a location in the code to a0, then copies an indexed entry from a0 to d0, then masks d0 with 31, then jumps to another point in the code.
- `reset_and_calculate`: Clears specific memory locations, performs bit operations on a value from memory, calculates differences between coordinates, and stores results based on comparison outcomes.
- `frag_cmpi_blt_move_bra`: Short code fragment that compares d4 with 13, then branches if the comparison was less, then copies d6 to d0, then copies 2 to the memory at an offset from a4, then jumps to another point in the code.
- `update_proximity_state_3`: Stores a masked register value into a memory structure and updates several fields including a status flag based on a calculated difference.
- `frag_move_bra_8`: Short code fragment that copies 65535 to d3, then jumps to another point in the code.
- `frag_move_bra_9`: Short code fragment that copies 1 to d3, then jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_sub_clr_bmi_cmpi_bpl_move_bra`: Short code fragment that subtracts d0 from d1, then clears d2, then clears d3, then subtracts d1 from d2, then branches if the last result was negative, then compares d2 with 16, then branches if the last result was not negative, then copies 1 to d3, then jumps to another point in the code.
- `validate_coordinate_range`: Compares a value with a threshold, sets a flag based on the comparison result, and performs calculations to determine coordinate boundaries for further processing.
- `frag_move_bra_10`: Short code fragment that copies 65535 to d3, then jumps to another point in the code.
- `frag_move_clr_st_rts`: Short code fragment that copies 1 to d3, then copies d3 to the memory at an offset from a4, then copies 2 to the memory at an offset from a4, then clears the memory at an offset from a4, then sets the memory at an offset from a4 to all ones, then returns.
- `adjust_global_counters`: Updates several global variables based on conditional checks and performs arithmetic on a global variable using the absolute value of another.
- `detect_sprite_collision`: Compares the positional and attribute data of objects in a list to identify overlaps.
- `update_object_attributes`: Iterates through a list of object structures to update their positional and attribute data based on comparison results.
- `compare_sprite_data`: Iterates through a list of object structures to compare their positional and attribute data against a reference set.
- `update_structure_field`: Updates a value in a structure by subtracting one field from another and conditionally adding a constant and updating a secondary integer field.
- `update_structure_fields_3`: Extracts bit-shifted values from a structure and stores them into different offsets within the same structure.
- `update_clamped_value`: Stores a short value into a structure and then updates an integer field by adding or subtracting a value, ensuring the result remains within a specified range.
- `calculate_clamped_value`: Performs complex arithmetic on a value using multiple structure fields and applies clamping logic to ensure the result stays within bounds.
- `decrement_state_counter`: Decrements a value in a structure based on specific conditional checks and resets other fields if a condition is met.
- `process_state_counters`: Iterates through a table to decrement a counter and updates various fields in a structure based on comparisons with table-referenced values.
- `process_state_and_counters`: Updates various status flags and counters in a structure based on comparisons between table-referenced values and internal state.
- `decrement_and_reset_counter`: Decrements a counter in a structure and resets multiple fields to default values once the counter reaches zero or a specific threshold.
- `update_display_coordinates`: Updates a set of coordinate and timing values within a structure using BCD-adjusted arithmetic and relative differences.
- `update_accumulated_totals`: Updates several fields in a structure by adding results from a multiplication routine and performing bitwise shifts and divisions.
- `multiply_and_shift_result`: Multiplies two values and performs bitwise shifts to return a concatenated 64-bit result.
- `update_structure_values_2`: Modifies a structure by decrementing a counter or setting a byte to a specific value, then performs arithmetic on a short value within the structure based on its absolute difference from another field.
- `calculate_movement_state`: Determines a direction value based on a comparison and updates several structure fields, including a counter and a concatenated value.
- `update_state_counters`: Decrements values in a table based on a comparison between a global variable and a value at an offset, potentially resetting multiple fields if certain conditions are met.
- `check_structure_field`: Calls another routine and returns if a specific short value at an offset within a structure does not equal four.
- `rotate_global_bit`: Performs a bitwise rotation on the complement of a global variable and returns the result.
- `wait_for_input_flags`: Polls specific bits in a memory location and writes values to a table based on the state of those bits.
- `process_input_sequence`: Polls bits in a memory location and writes a value derived from a byte at a specific offset into a table.
- `check_input_state`: Checks a short value at an offset within a structure and calls another routine based on the result.
- `check_conditional_flags`: Checks values in a table and polls bits in a memory location to conditionally write values to a table.
- `handle_input_events`: Polls various bits in a memory location and updates a table and a global variable based on the resulting bit patterns.
- `update_entity_state_and_sync_buffers`: Updates entity status flags and timers based on specific field values and synchronizes data between structures.
- `process_object_list`: Iterates through a list of four pointers to perform arithmetic on structure members and update a global integer variable.
- `calculate_object_attributes`: Performs complex arithmetic on structure members to update coordinate-related data and write values to a memory table.
- `update_object_sprite_attributes_2`: Calculates new sprite or object properties including position, scaling, and texture data based on input parameters and a lookup table.
- `update_sprite_attribute_buffer`: Iterates through a data structure to calculate sprite position and attribute offsets.
- `update_status_byte`: Checks specific bits in a global variable and updates a byte in memory with corresponding values.
- `find_nearest_object`: Iterates through a table of pointers to find an entry where the sum of the absolute differences of two coordinate pairs is minimized.
- `detect_entity_collision`: Iterates through a list of entities to find two objects with sequential frame or state indices.
- `update_state_parameters`: Modifies values in a structure based on a global counter and specific status flags.
- `copy_memory_block_4`: Copies a fixed number of words from one memory location to another.
- `assign_pointer_references`: Sets two pointer fields in a structure to point to a global variable based on a flag.
- `search_and_copy_data_table`: Compares a calculated value against entries in a table and performs recursive calls to transfer data from a source table to a destination buffer based on the comparison results.
- `populate_data_table`: Transfers several 16-bit values from memory into a destination table.
- `transfer_data_list`: Iterates through a list of pointers to copy data from a source table to a destination buffer.
- `calculate_composite_value`: Performs arithmetic on input values and returns a concatenated 32-bit result containing bitwise flags.
- `update_pointer_list`: Calculates offsets and stores a series of 16-bit pointer pairs into a destination table.
- `loop_bit_inversion`: Stores two words into a memory structure and enters a loop that performs several calls and inverts a byte at a specific offset until a condition is met.
- `check_status_bits`: Checks a byte at a specific offset within a structure for various bit flags and returns.
- `calculate_relative_offsets`: Computes the difference between two sets of coordinates and stores the resulting scaled values and direction flags into a structure.
- `initialize_and_populate_data_table`: Initializes a linked list structure and then iterates through a large data table to extract specific bit-masked values.
- `manage_linked_list_node`: Swaps two pointers in a global structure and initializes a new node with values derived from input registers and a data table.
- `process_linked_list`: Iterates through a linked list to copy data between structures and decrement counters while managing list pointers.
- `set_bit_in_mask`: Calculates a bit position based on input values and sets that bit in a 32-bit word within a table.
- `update_linked_list_counters`: Decrements a value in a global structure and, if certain conditions are met, modifies counters and calls another routine.
- `search_list_for_match`: Iterates through a list of pointers to find an entry that satisfies multiple byte and short value comparisons.
- `frag_movem_tst_beq_rts`: Short code fragment that restores saved registers from the stack, then tests d0, then branches if the last result was zero, then returns.
- `calculate_collision_bounds`: Performs arithmetic operations on coordinate values, compares results against thresholds, and updates memory locations with computed values when conditions are met.
- `calculate_offset_and_update`: Calculates an offset value by shifting bits and subtracting values, updates memory locations with computed results, and modifies a counter based on comparison outcomes.
- `frag_tst_beq_move_addq_andi_moveq_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a4 to d0, then adds 3 to d0, then masks d0 with 31, then copies d0 to the memory at an offset from a4, then loads 1 into d0, then returns.
- `compute_object_adjustment`: Calculates adjusted coordinates using bit shifts and arithmetic operations, stores results in memory, and updates a counter based on comparison outcomes.
- `compute_adjustment_and_store`: Computes an adjustment value using bit operations and subtraction, stores the result in memory, and updates a counter based on comparison with another value.
- `frag_tst_beq_move_addq_andi_moveq_rts_2`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a4 to d0, then adds 2 to d0, then masks d0 with 31, then copies d0 to the memory at an offset from a4, then loads 1 into d0, then returns.
- `process_coordinate_update`: Processes coordinate differences through bit operations and comparisons, then updates memory locations with calculated values and modifies object properties accordingly.
- `process_coordinate_difference`: Processes coordinate differences by performing bit operations and comparisons, then updates memory locations with calculated values and modifies counters accordingly.
- `frag_tst_beq_move_addq_andi_moveq_rts_3`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a4 to d0, then adds 3 to d0, then masks d0 with 31, then copies d0 to the memory at an offset from a4, then loads 1 into d0, then returns.
- `handle_state_transition`: Checks object state conditions, updates coordinates, decrements timers, and modifies object properties based on comparison results before returning a status value.
- `handle_object_state_transition`: Handles object state transitions by checking flags, updating coordinates, decrementing timers, and modifying object properties based on comparison results.
- `frag_tst_beq_move_addq_andi_moveq_rts_4`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then copies the memory at an offset from a4 to d0, then adds 3 to d0, then masks d0 with 31, then copies d0 to the memory at an offset from a4, then loads 1 into d0, then returns.
- `update_countdown_timer`: Checks specific flags and a value in a global variable to initialize a countdown sequence with calculated values.
- `clear_global_bytes_2`: Sets eight bytes of a global variable to zero using a loop.
- `update_table_entry`: Checks a byte in a table; if it is zero, it initializes a structure with calculated values and updates a global counter.
- `manage_object_lifecycle`: Manages the lifecycle of objects by iterating through a list, processing each object's state, updating counters, and clearing flags when objects reach their final state.
- `calculate_relative_coordinates_2`: Calculates relative coordinates by performing arithmetic operations and bitwise shifts, then stores the results in memory locations after a pointer.
- `compute_difference_and_compare`: Subtracts values from memory locations, performs conditional checks on the difference, and sets a register to a specific value based on comparison results.
- `calculate_relative_coordinates`: Calculates relative coordinates by performing arithmetic operations and bitwise shifts, then stores the results in memory locations after applying conditional negations.
- `increment_table_values`: Iterates through two arrays of words and adjusts the values in the second array to incrementally approach the values in the first array based on bitwise differences.
- `execute_state_machine_loop`: Iterates through a sequence of function calls and conditional logic to update structure members and global variables.
- `increment_table_values_2`: Adjusts values in two different tables toward target values based on specific conditions and updates status flags.
- `parse_audio_sample_table`: Iterates through a data structure to calculate sample lengths and offsets, populating a table of audio channel descriptors.
- `initialize_hardware_registers`: Sets several memory locations and hardware registers to specific constant values.
- `initialize_track_data_3`: Initializes track configuration data by decrementing a counter and branching based on its value, then processes movement parameters through a loop.
- `initialize_track_data_2`: Initializes track configuration data by decrementing a counter and branching based on its value, then processes movement parameters through table lookups and updates structure fields with calculated values.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `process_track_data_2`: Processes track configuration data by calling routines based on table lookups, updates structure fields with movement parameters, and modifies bit flags in a global variable.
- `frag_move_tst_beq_subq_bra`: Short code fragment that copies a location in the code to the memory at a0, then tests the memory at an offset from a0, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a0, then jumps to another point in the code.
- `update_track_config`: Reads track parameters from a table and updates structure fields with movement data, then clears a configuration flag in a global variable.
- `update_track_parameters_2`: Processes track configuration data through a loop, updating structure fields with movement parameters and modifying bit flags in a global variable.
- `frag_moveq_add_beq_cmpi_bne_clr_move_br`: Short code fragment that loads 0 into d0, then adds a location in the code to d0, then branches if the last result was zero, then compares d0 with 64, then branches if the last result was not zero, then clears the memory at an offset from a0 (2 times in a row), then loads 255 into d7, then copies d7 to the memory at an offset from a0, then jumps to another point in the code.
- `decrement_and_process`: Decrements a word value at offset -2 of a structure, clears another word at offset 34, then processes data from a table through multiple iterations, updating registers and memory locations based on conditional checks and bit operations.
- `perform_bit_shift_operations`: Executes bit shift and arithmetic operations on register values, performs conditional branches based on sign tests, and updates another register with the final computed value.
- `update_track_parameters`: Processes track configuration data through a loop, updating structure fields with movement parameters and modifying bit flags in a global variable.
- `process_track_data`: Reads track configuration data from a buffer, processes movement parameters, and updates structure fields accordingly.
- `frag_add_bmi_move_moveq_jmp`: Short code fragment that adds d0 to d0, then branches if the last result was negative, then copies d0 to the memory at a1, then copies the memory at a2 (advancing a2) to the memory at an offset from a1, then copies a2 to the memory just below a1 (moving a1 down), then copies d0 to d2, then loads 0 into d0, then jumps to another point in the code.
- `frag_clr_move_rts`: Short code fragment that clears the memory at a1, then copies a2 to the memory just below a1 (moving a1 down), then copies d0 to the memory at an offset from a1, then returns.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `frag_nop_bra`: Short code fragment that pauses for one no-operation step, then jumps to another point in the code.
- `frag_move_sub_beq_bmi_lsr_bra`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then copies the memory at an offset from a1 to d0, then copies the memory at a0 to d2, then subtracts a location in the code from d2, then copies a location in the code to d2, then branches if the last result was zero, then branches if the last result was negative, then shifts d1 right by 4, then jumps to another point in the code.
- `frag_andi_movea_add_move_rts`: Short code fragment that masks d1 with 15, then copies the memory at an offset from a1 to a4, then adds d1 to d1, then copies an indexed entry from a4 to the memory at an offset from a3, then returns.
- `frag_move_rts_3`: Short code fragment that copies d0 to the memory at an offset from a3, then returns.
- `clamp_difference_value`: Calculates the difference between two values in a structure, clamps the result to a minimum threshold, and stores it in a new location.
- `frag_move_add_cmpi_bmi_clr_rts`: Short code fragment that copies the memory at an offset from a1 to d1, then adds the memory at a1 to d1, then compares d1 with 856, then branches if the last result was negative, then copies 856 to d1, then clears the memory at an offset from a1, then copies d1 to the memory at an offset from a1, then copies d1 to the memory at an offset from a3, then returns.
- `update_velocity_sign`: Reads a word from offset 6 of the structure in a1, and if it is not zero, tests the word at offset 30 of that structure.
- `frag_tst_bpl_neg_rts`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not negative, then negates the memory at an offset from a1, then returns.
- `frag_move_beq_bmi_add_cmp_blt_bra`: Short code fragment that copies the memory at an offset from a1 to d2, then branches if the last result was zero, then copies the memory at an offset from a1 to d1, then branches if the last result was negative, then adds the memory at an offset from a1 to d1, then compares d1 with d2, then branches if the comparison was less, then jumps to another point in the code.
- `update_acceleration`: Adds a value from offset 20 of the structure in a1 to d1, compares that sum with d2, and if the sum is less than or equal to d2, clears fields at offsets 22 and -2 of that structure.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `split_word_into_fields`: Reads a word from offset 6 of the structure in a1, and if it is nonzero, stores its low nibble as a word at offset 36 when that nibble is nonzero, and stores its high nibble shifted right by 2 as a byte at offset 34 when that nibble is nonzero.
- `calculate_position_offset`: Calculates a signed offset using multiplication and bit shifts, then updates a value in a structure and increments a byte in another structure.
- `frag_tst_bpl_not_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not negative, then inverts d0, then jumps to another point in the code.
- `update_position_and_sign`: Calculates a signed offset using bit manipulation and multiplication, stores it in a structure, and updates a byte in a structure by adding two other bytes.
- `scale_value_into_structure`: Multiplies a value from a structure by a global variable, shifts the result, and stores it in another structure if a pointer is non-zero.
- `frag_bsr`: Short code fragment that calls another routine.
- `compare_and_adjust_value`: Reads a word from the structure in a1, adds a byte from offset 5 of that structure to it, and if the result is negative, sets d1 to 64.
- `zero_structure_fields`: Sets multiple fields within a structure and a separate memory location to zero.
- `split_nibbles_into_fields`: Reads a word from offset 6 of the structure in a1, and if it is nonzero, stores its low nibble as a word at offset 42 and its high nibble shifted right by 2 as a byte at offset 40.
- `frag_moveq_and_move_beq_add_subq_st_bra`: Short code fragment that loads 127 into d0, then masks d0 with the memory at an offset from a1, then copies the memory at an offset from a1 to d2, then branches if the last result was zero, then adds d0 to d0, then subtracts 1 from d2, then branches if the last result was zero, then sets d0 to all ones, then jumps to another point in the code.
- `frag_tst_bpl_not_bra_2`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not negative, then inverts d0, then jumps to another point in the code.
- `process_shifted_value`: Shifts d0 right by 2 bits, reads a byte from an array at offset 64 of a0 using that shifted value as index, multiplies it with a word from offset 36 of a1, shifts the result right by 7 bits, and if the byte at offset 35 of a1 is positive, negates the result.
- `frag_moveq_cmp_bls_move_mulu_lsr_add_rt`: Short code fragment that loads 40 into d1, then compares d0 with d1, then branches if the comparison was lower or same (unsigned), then copies d1 to d0, then multiplies d1 by a location in the code (unsigned), then shifts d1 right by 5, then copies d1 to the memory at an offset from a3, then copies the memory at an offset from a1 to d0, then adds d0 to the memory at an offset from a1, then returns.
- `update_and_process`: Tests if d0 is not zero, and if so, reads a byte from offset 46 of the structure in a1, and if that byte is nonzero, reads another byte from an array at offset 48 of a0 using that byte as index, adds it to the byte at offset 47 of that structure.
- `frag_move_sub_ble_add_rts`: Short code fragment that copies the memory at an offset from a1 to d1, then subtracts d1 from the memory at an offset from a1, then branches if the comparison was less or equal, then adds d1 to d1, then adds d1 to the memory at an offset from a1, then returns.
- `frag_move_rts_4`: Short code fragment that copies 1 to the memory at an offset from a1, then returns.
- `frag_clr_move_add_st_rts`: Short code fragment that clears the memory at an offset from a1, then copies the memory at an offset from a1 to d1, then adds d1 to d1, then copies d1 to the memory at an offset from a0, then sets the memory at an offset from a0 to all ones, then returns.
- `frag_movea_add_adda_move_rts`: Short code fragment that copies a location in the code to a1, then adds d1 to d1, then adds d1 to a1, then copies a1 to the memory at an offset from a0, then returns.
- `frag_move_mulu_lsr_clr_rts`: Short code fragment that copies the memory at an offset from a1 to d1, then copies d1 to the memory at an offset from a1, then multiplies d1 by a location in the code (unsigned), then shifts d1 right by 5, then copies d1 to the memory at an offset from a3, then clears the memory at an offset from a1, then returns.
- `frag_clr_move_rts_2`: Short code fragment that clears the memory at an offset from a1, then clears the memory at an offset from a0, then copies the memory at an offset from a1 to the memory at an offset from a0, then clears the memory at an offset from a0, then returns.
- `frag_moveq_and_lsr_jmp`: Short code fragment that loads 240 into d1, then masks d1 with the memory at an offset from a1, then shifts d1 right by 2, then jumps to another point in the code.
- `compute_division_result`: Loads a word from memory into d1, subtracts a word from a0 from it, divides the result by a word from a1, swaps the result, and if the result is not zero, reads a byte from offset -8 of a1 and performs an OR operation with a byte at offset 47 of a0.
- `frag_addq_clr_andi_bsr_subq_rts`: Short code fragment that adds 4 to a1, then clears the memory at a1 (advancing a1), then masks the memory at a1 with 15, then calls another routine, then subtracts 6 from a1, then returns.
- `frag_addq_clr_andi_neg_bsr_subq_rts`: Short code fragment that adds 4 to a1, then clears the memory at a1 (advancing a1), then masks the memory at a1 with 15, then negates the memory at an offset from a1, then calls another routine, then subtracts 6 from a1, then returns.
- `frag_subq_bne_clr_rts`: Short code fragment that subtracts 1 from the memory at a1, then branches if the last result was not zero, then clears the memory just below a1 (moving a1 down), then clears the memory at an offset from a1, then clears the memory at an offset from a3, then returns.
- `frag_move_tst_beq_andi_add_addq_rts`: Short code fragment that copies 48 to the memory at an offset from a1, then tests d0, then branches if the last result was zero, then masks the memory at an offset from a1 with 15, then branches if the last result was zero, then adds the memory at an offset from a1 to d0, then copies an indexed entry from a0 to the memory at an offset from a1, then adds 4 to a7, then adds 6 to the top of the stack, then returns.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a1, then returns.
- `frag_subq_bne_clr_move_bra`: Short code fragment that subtracts 1 from the memory at a1, then branches if the last result was not zero, then clears the memory at an offset from a1, then copies the memory at an offset from a1 to the memory at an offset from a3, then jumps to another point in the code.
- `enable_dma_channels`: Sets a bit in a hardware register to enable DMA channels and updates a global variable.
- `configure_audio_channels`: Updates the length and location registers for four audio channels and modifies several memory locations.
- `execute_callback_sequence`: Checks a flag in a structure and, if set, performs a series of calls to another routine and updates a table entry.
- `execute_and_store_results`: Calls a function pointer from a structure and stores the resulting values and a calculated sum into a table.
- `update_object_data`: Checks a condition in a global variable and, if met, copies data from a pointer-based structure to another location and updates several global values.
- `decrement_and_check_counter`: Decrements a global variable and sets a byte in a structure to zero if the counter falls below zero and a specific byte is non-zero.
- `update_structure_field_clamped`: Updates a short value within a structure by either decrementing it, incrementing it with a maximum limit, or resetting it to a specific value based on the state of a global variable and a conditional flag.
- `update_structure_value`: Depending on a global variable, this routine either decrements a short value within a structure or increments it and clamps the result to a maximum value.
- `decrement_structure_value`: Decrements a short value at a specific offset within a structure and clamps it to zero if it becomes negative.
- `frag_movea_move_bra`: Short code fragment that copies a fixed memory address to a4, then copies a fixed memory address to d2, then jumps to another point in the code.
- `frag_movea_move_bra_2`: Short code fragment that copies a fixed memory address to a4, then copies a fixed memory address to d2, then jumps to another point in the code.
- `calculate_offset_value`: Calculates a value based on a global variable and a table, then stores the result into a structure with clamping logic.

### Turbo Trax (1995)(Arcane Entertainment)[cr FLT](Disk 1 of 3).adf:Galahad#unpacked

- `call_supervisor_3`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `save_sys_2`: Checks the chip revision bits and, when the graphics library is found, saves some of its system values after waiting for the display.
- `restore_system`: Resets various hardware registers and executes a sequence of function calls to restore the system state.
- `getvbr_3`: Reads the vector base register through the exec Supervisor call when the CPU-type flag is set, and stores the result at the address held in a register.
- `initialize_game_state_2`: Writes specific byte values to memory locations, performs a loop that waits for a system flag, and executes conditional operations based on test results.
- `initialize_and_wait`: Writes specific byte values to memory locations and performs a loop that waits for a system flag, then executes conditional operations based on joystick input.
- `update_and_reset_system`: Updates global variables with specific values, performs conditional updates based on test results, and executes a loop that waits for system flags before returning.
- `toggle_and_increment_2`: Toggles bit 0 of the word at offset 14 of a5, and if the result is zero, increments the word at offset 18 of a5 by 2, then copies a word from memory to four locations in an array starting at offset 168 of a6.
- `toggle_and_decrement_2`: Toggles bit 0 of the word at offset 16 of a5, and if the result is zero, decrements the word at offset 18 of a5 by 2, then copies a word from memory to four locations in an array starting at offset 168 of a6.
- `split_nibbles_into_table_3`: Processes 32 words from a source table, splits each word into four-bit nibbles, and stores the negated values of these nibbles into a destination structure with specific offsets.
- `frag_moveq_move_andi_cmp_bgt_lsr_dbra_r_3`: Short code fragment that loads 2 into d3, then copies d0 to d1, then masks d1 with 15, then compares d2 with d1, then branches if the comparison was greater, then copies d1 to d2, then shifts d0 right by 4, then loops back while counting down d3, then returns.
- `frag_subq_beq_rts`: Short code fragment that subtracts 1 from the memory at an offset from a5, then branches if the last result was zero, then returns.
- `update_and_compare_2`: Updates multiple memory locations with decrementing values and performs bitwise comparisons between word values, modifying another set of memory locations based on comparison results.
- `update_and_compare`: Reads a word value from a memory location, performs a bitwise AND with another value, compares the result with a value at another location, and if they don't match, adds a third value to the first location.
- `frag_subq_beq_st_rts`: Short code fragment that subtracts 1 from the memory at an offset from a5, then branches if the last result was zero, then sets d2 to all ones, then returns.
- `subtract_nibble_values`: Iterates through a table of words, performs bitwise subtractions on specific nibble values based on their content, and updates the table entries accordingly.
- `initialize_graphics_system_2`: Configures custom hardware registers for graphics display, sets up interrupt vectors, and initializes audio DMA channels while waiting for a specific vertical blanking period.
- `initialize_sprite_data`: Populates a sprite data table with word values derived from shifted and incremented pointers, then writes a series of word pairs to another table, followed by storing a pointer and a word value to offsets within a structure.
- `reset_hardware_registers`: Resets various hardware registers including CIAB control and interrupt registers, and executes a sequence of function calls to restore system state.
- `lbl000352_2`: Writes the constant 255 to one variable and 32 to a global variable, which the listed facts associate with the interrupt request register.
- `frag_movem_move_lea_st_rte_2`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then loads the address of a location in the code into a0, then sets the memory at a0 to all ones, then restores saved registers from the stack, then copies 8192 to a fixed memory address, then returns from the exception.
- `initialize_and_process_2`: Initializes a memory area with zeros, then processes input data by reading bytes and handling different character types through conditional branches, updating memory locations based on character values and offset calculations.
- `frag_lea_sub_lsr_suba_bra`: Short code fragment that loads the address of the memory at an offset from a1 into a1, then subtracts d0 from d7, then shifts d0 right by 1, then subtracts d0 from a1, then jumps to another point in the code.
- `copy_data_with_offset`: Copies data from a table to a destination buffer with specific offset calculations, handling multiple iterations and conditional branches based on input values.
- `frag_movea_tst_bne_movem_rts`: Short code fragment that copies the top of the stack to a1, then tests d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `wait_for_frame_flag`: Clears a frame flag byte and spins until that byte becomes non-zero, then performs additional operations based on comparison results with specific constants.
- `wait_for_frame_flag_reverse`: Clears a frame flag byte and spins until that byte becomes zero, then performs additional operations based on comparison results with specific constants.
- `wait_tof_2`: Clears a frame flag byte and spins until that byte becomes non-zero.
