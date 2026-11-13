# Game 1041 (universe (1994)(core design)(m4)[cr psg]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01041). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5407188 top-level bytes in 16 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 17 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (6 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 32 of 32 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 329 of 329 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29567 |
| code | 1026 |
| data | 4693984 |
| padding | 627 |
| unused | 681984 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf:bootblock | 1 | 65 | 2 |
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf:bootblock | 1 | 18 | 1 |
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf:bootblock | 1 | 18 | 1 |
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 4 of 6).adf:bootblock | 1 | 18 | 1 |
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 5 of 6).adf:bootblock | 1 | 18 | 1 |
| Universe (1994)(Core Design)(M4)[cr PSG](Disk 6 of 6).adf:bootblock | 1 | 18 | 1 |
| logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf boot-block load | 1 | 116 | 2 |
| logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 32 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: custom-format (non-DOS) disk, 1686 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: custom-format (non-DOS) disk, 1094 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: custom-format (non-DOS) disk, 1521 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 4 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 4 of 6).adf: custom-format (non-DOS) disk, 1739 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 5 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 5 of 6).adf: custom-format (non-DOS) disk, 1575 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 6 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 6 of 6).adf: custom-format (non-DOS) disk, 1601 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 15822 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: the boot block reads 4096 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: the boot block reads 26112 bytes from disk offset 843264 to an address (found statically); recovered as a unit
- 32 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 5 first-run image unit(s) in 1 version group(s); 334 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: 2 logged loads cover 30208 bytes of the image
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: blocks 11-12 (bytes 5632-6656) outside the filesystem hold data, not code (opcode density 0.031, entropy 2.96)
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: blocks 22-1646 (bytes 11264-843264) hold a RNC1: Rob Northen RNC1 Compressor stream (832000 -> 40064 bytes)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.25)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 22-904 (bytes 11264-463360) hold a RNC1: Rob Northen RNC1 Compressor stream (452096 -> 57800 bytes)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 920-924 (bytes 471040-473600) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 940-944 (bytes 481280-483840) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 960-964 (bytes 491520-494080) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 980-984 (bytes 501760-504320) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1000-1004 (bytes 512000-514560) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1020-1024 (bytes 522240-524800) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1040-1044 (bytes 532480-535040) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1060-1064 (bytes 542720-545280) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1080-1084 (bytes 552960-555520) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1100-1104 (bytes 563200-565760) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1120-1124 (bytes 573440-576000) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1140-1144 (bytes 583680-586240) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1160-1164 (bytes 593920-596480) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1180-1184 (bytes 604160-606720) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1200-1204 (bytes 614400-616960) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1220-1224 (bytes 624640-627200) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1240-1244 (bytes 634880-637440) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1260-1264 (bytes 645120-647680) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1280-1284 (bytes 655360-657920) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1300-1304 (bytes 665600-668160) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1320-1324 (bytes 675840-678400) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1340-1344 (bytes 686080-688640) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1360-1364 (bytes 696320-698880) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1380-1384 (bytes 706560-709120) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1400-1404 (bytes 716800-719360) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1420-1424 (bytes 727040-729600) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1440-1444 (bytes 737280-739840) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1460-1464 (bytes 747520-750080) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1480-1484 (bytes 757760-760320) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1500-1504 (bytes 768000-770560) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1520-1524 (bytes 778240-780800) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1540-1544 (bytes 788480-791040) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1560-1564 (bytes 798720-801280) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1580-1584 (bytes 808960-811520) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1600-1604 (bytes 819200-821760) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1620-1624 (bytes 829440-832000) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1640-1644 (bytes 839680-842240) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1660-1664 (bytes 849920-852480) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1680-1684 (bytes 860160-862720) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1700-1704 (bytes 870400-872960) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1720-1724 (bytes 880640-883200) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: blocks 1740-1744 (bytes 890880-893440) outside the filesystem hold data, not code (opcode density 0.008, entropy 2.30)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.66)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 22-1517 (bytes 11264-777216) hold a RNC1: Rob Northen RNC1 Compressor stream (765952 -> 64336 bytes)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1520-1521 (bytes 778240-779264) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1540-1541 (bytes 788480-789504) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1560-1561 (bytes 798720-799744) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1580-1581 (bytes 808960-809984) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1600-1601 (bytes 819200-820224) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1620-1621 (bytes 829440-830464) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1640-1641 (bytes 839680-840704) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1660-1661 (bytes 849920-850944) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1680-1681 (bytes 860160-861184) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1700-1701 (bytes 870400-871424) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1720-1721 (bytes 880640-881664) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: blocks 1740-1741 (bytes 890880-891904) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.90)
- universe (1994)(core design)(m4)[cr psg](disk 4 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 4 of 6).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.016, entropy 2.42)
- universe (1994)(core design)(m4)[cr psg](disk 4 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 4 of 6).adf: blocks 22-1759 (bytes 11264-901120) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.31)
- universe (1994)(core design)(m4)[cr psg](disk 5 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 5 of 6).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.012, entropy 2.48)
- universe (1994)(core design)(m4)[cr psg](disk 5 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 5 of 6).adf: blocks 22-1595 (bytes 11264-817152) hold a RNC1: Rob Northen RNC1 Compressor stream (805888 -> 59998 bytes)
- universe (1994)(core design)(m4)[cr psg](disk 6 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 6 of 6).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.016, entropy 2.11)
- universe (1994)(core design)(m4)[cr psg](disk 6 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 6 of 6).adf: blocks 22-1621 (bytes 11264-830464) hold a RNC1: Rob Northen RNC1 Compressor stream (819200 -> 200796 bytes)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf:bootblock

- `setup_and_jump_to_entry`: Calls a routine that clears a structure, then opens and closes the trackdisk device and performs a series of library calls to set up and tear down a device interface.
- `init_struct_and_call_lib`: Initializes a structure pointed to by a1 by storing a word at offset 28 and longs at offsets 36, 40 and 44, then jumps to a routine at offset -456 of the exec library in a6.

### Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Universe (1994)(Core Design)(M4)[cr PSG](Disk 4 of 6).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Universe (1994)(Core Design)(M4)[cr PSG](Disk 5 of 6).adf:bootblock

- `boot_entry_4`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Universe (1994)(Core Design)(M4)[cr PSG](Disk 6 of 6).adf:bootblock

- `boot_entry_5`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `index_table_and_mask_word`: Loads a word from a table indexed by a doubled value and bitwise ANDs it with a second word, then returns the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: Performs an infinite loop by branching to itself.
- `fill_memory_on_interrupt`: Checks bit 3 of the interrupt request register and, if set, stores the value from a register into thirteen consecutive 4-byte locations starting at an offset from a pointer in a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_store_constant`: Increments a 16-bit value at an offset of 8 from a global pointer, calls another routine, and stores the constant 16 into the interrupt request register before returning.
- `check_vposr_and_store_interrupt`: Checks a byte from the video pointer register and, depending on its value, calls another routine before storing the constant 4 into the interrupt request register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_if_zero`: Tests a byte in a global variable and branches if it is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_if_not_zero`: Tests a byte in a global variable and returns if it is not zero, otherwise it branches to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
