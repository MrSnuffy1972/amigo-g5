# Game 1041 (universe (1994)(core design)(m4)[cr psg]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01041). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (5411568 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 32 of 32 named (0 author symbols), 32 of 32 described (16 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 32 of 32 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 604 of 604 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 30727 |
| code | 2006 |
| data | 4696224 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 53 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 280 | 18 |

## Cases handled

- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: custom-format (non-DOS) disk, 1686 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 2 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 2 of 6).adf: custom-format (non-DOS) disk, 1094 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 3 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 3 of 6).adf: custom-format (non-DOS) disk, 1521 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 4 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 4 of 6).adf: custom-format (non-DOS) disk, 1739 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 5 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 5 of 6).adf: custom-format (non-DOS) disk, 1575 non-empty blocks: matched against track-loaded segments
- universe (1994)(core design)(m4)[cr psg](disk 6 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 6 of 6).adf: custom-format (non-DOS) disk, 1601 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 29687 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 29687 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 29687 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 29687 of its 29687 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: the boot block reads 4096 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- universe (1994)(core design)(m4)[cr psg](disk 1 of 6)/Universe (1994)(Core Design)(M4)[cr PSG](Disk 1 of 6).adf: the boot block reads 26112 bytes from disk offset 843264 to an address (found statically); recovered as a unit
- runtime segment an address (play-lgpwide): 26 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
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

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `manage_keyboard_interrupts`: If bit 3 of the interrupt request register is set, it manipulates keyboard serial data via the CIAA serial data register, updates the CIAA control register, clears the CIAA interrupt register, and sets bit 3 of the interrupt request register.
- `frag_move_cmp_beq_clr_bsr_tst_bne_rts`: Short code fragment that copies the memory at an offset from a5 to d0, then compares d0 with the memory at an offset from a5, then branches if the last result was zero, then clears the memory at an offset from a5, then calls another routine, then tests the memory at an offset from a5, then branches if the last result was not zero, then returns.
- `frag_movem_lea_addq_bsr_move_rte`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a5, then adds 1 to the memory at an offset from a5, then calls another routine, then copies 16 to a fixed memory address, then restores saved registers from the stack, then returns from the exception.
- `update_global_counter`: If the second byte of the video beam position register is less than 64, it compares a global variable with another global variable and increments or decrements the latter by 2 before calling another routine and setting bit 2 of the interrupt request register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_entity_audio_and_dma`: Iterates through a sequence of data structures to update audio registers and DMA controller settings based on values read from a table.
- `load_audio_parameters`: If a byte at a specific address is non-zero, it writes a value to offset 150 of the structure in a5 and then copies several 32-bit and 16-bit values from a table into the structure in a5.
- `frag_cmp_ble_move_bra`: Short code fragment that compares d4 with a fixed memory address, then branches if the comparison was less or equal, then copies a fixed memory address to d4, then copies d4 to the memory at an offset from a2, then jumps to another point in the code.
- `frag_st_bra`: Short code fragment that sets the memory at an offset from a6 to all ones, then jumps to another point in the code.
- `frag_st_move_lea_movea_bra`: Short code fragment that sets the memory at an offset from a6 to all ones, then copies 0 to d4, then copies a fixed memory address to d4, then copies a0 to the stack, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d4, then copies the top of the stack to a0, then copies d4 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies d4 to the memory at an offset from a6, then jumps to another point in the code.
- `update_entity_state_and_dispatch`: Updates a structure by comparing a 16-bit value at offset 36 with a value at offset 38, then performs a loop that processes a sequence of bytes to update various fields in a structure and calls a routine from a jump table based on a calculated index.
- `clamp_and_store_value`: Subtracts a byte from offset 3 of a structure from a 16-bit value at offset 24 of the same structure, then clamps the result to a maximum of 113 and stores it at offset 6 of another structure.
- `frag_moveq_move_add_cmpi_bmi_bra`: Short code fragment that loads 0 into d2, then copies the memory at an offset from a0 to d2, then adds d2 to the memory at an offset from a0, then compares the memory at an offset from a0 with 856, then branches if the last result was negative, then copies 856 to the memory at an offset from a0, then copies the memory at an offset from a0 to the memory at an offset from a4, then jumps to another point in the code.
- `calculate_offset_and_dispatch`: Determines a value from a table using an index derived from a structure, then performs a loop that processes a sequence of bytes to call another routine via a table.
- `fetch_table_value`: Calculates an index by masking and adding values from a structure, then uses this index to load a 16-bit word from a table into offset 6 of another structure.
- `frag_move_bra_2`: Short code fragment that copies the memory at an offset from a0 to the memory at an offset from a4, then jumps to another point in the code.
- `compute_scaled_offset`: Calculates a scaled value using a multiplication and bit-shift, updates offset 6 of a structure, and modifies offset 49 of a structure before calling another routine via a table.
- `update_state_and_dispatch`: Updates a 16-bit value at offset 6 of a structure, modifies a byte at offset 49 of a structure based on a bit-shifted value at offset 48, and performs conditional updates to offsets 4 and 8 of the structure before calling another routine via a table.
- `frag_clr_move_bra`: Short code fragment that clears the memory at an offset from a0, then copies the memory at an offset from a0 to the memory at an offset from a4, then jumps to another point in the code.
- `process_accumulator_and_dispatch`: Updates a 16-bit value at offset 24 of a structure by adding or subtracting a value from offset 28, then performs conditional updates to offsets 4, 5, and 8 of a structure before calling another routine via a table.
- `limit_and_assign_value`: Subtracts a 16-bit value from offset 24 of a structure, compares it to a value in d2, and if the result is greater, stores the value from d2 into offset 24 and clears offset 26 of the structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
