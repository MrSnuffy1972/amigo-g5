# Game 469 (Bubba 'n' Stix): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00469). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1819072 top-level bytes in 187 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 124 of 124 named (0 author symbols), 124 of 124 described (16 by rule, 106 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 415 of 415 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 193152 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3675 of 3950 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8924 |
| code | 13360 |
| data | 1736875 |
| padding | 23049 |
| unused | 36864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:INTRO | 2 | 79 | 3 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:Bubba | 3 | 202 | 2 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3658 | 117 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk1am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk6am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk4am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblkbam.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk5am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk3am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:fgplax1a.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2287 -> 19598 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:set1.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6868 -> 15916 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mfmap1.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6393 -> 40590 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mparasp1.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1342 -> 3392 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mplxobj1.hs: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (797 -> 2704 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk6am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4092 -> 6040 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk1am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11914 -> 33408 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:complete.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9876 -> 12544 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:level6.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (29974 -> 42930 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:sprites1.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2714 -> 5214 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mfmap6.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5463 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk1am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3357 -> 4560 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:fgplax1b.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (400 -> 2462 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:sprites1.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49404 -> 96384 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/titledlt.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21238 -> 57702 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/continue.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22753 -> 30412 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/titlepic.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7345 -> 11818 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/title.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (33588 -> 43062 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/getready.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2952 -> 3172 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/map.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22744 -> 33760 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:BubbaData/gameover.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2911 -> 3188 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mtrg1.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (866 -> 1108 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:lev1info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (895 -> 2408 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:sprites6.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2721 -> 4934 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:disk1.spk: high-entropy data file (7.85 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:endseq.bit: high-entropy data file (7.66 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:sprites6.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40190 -> 89064 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mtrg6.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1304 -> 1826 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mparasp6.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1570 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:set5.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13987 -> 27660 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:lev6info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (937 -> 4650 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk6am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14871 -> 39456 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:level1.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (23559 -> 30916 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:lev4info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (243 -> 1026 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrgb3.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1727 -> 2996 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmap3.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8603 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mparasp3.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1228 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk4am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8784 -> 22176 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:level2.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36160 -> 48732 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrgb1.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (688 -> 1222 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:disk2.spk: high-entropy data file (7.83 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:level3.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30461 -> 44582 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:sprites5.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3306 -> 5698 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:level4.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36565 -> 52314 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmapb4.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1077 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:bonus.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (19879 -> 35880 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:sprites3.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2702 -> 4458 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmapb2.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2436 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk4am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2042 -> 2744 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:setb.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7692 -> 16432 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblkbam.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11771 -> 32608 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mparaspb.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (736 -> 1280 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:sprites5.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (48322 -> 97800 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrg5.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1543 -> 2250 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mparasp5.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (924 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:levbinfo.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (571 -> 1960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:set4.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8546 -> 18102 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:sprites3.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (47506 -> 96172 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:lev5info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (247 -> 1734 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrg3.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1699 -> 2520 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrgb4.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (306 -> 478 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmap4.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7783 -> 40902 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk5am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11559 -> 33376 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:set2.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11877 -> 24278 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/titledlt.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21238 -> 57702 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/continue.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22753 -> 30412 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/titlepic.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7345 -> 11818 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/title.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (33588 -> 43062 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/getready.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2952 -> 3172 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/map.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22744 -> 33760 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:BubbaData/gameover.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2911 -> 3188 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:lev3info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1170 -> 4636 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrgb2.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (606 -> 944 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk3am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10589 -> 32128 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:spritesb.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2089 -> 3746 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblkbam.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2880 -> 3752 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmapb5.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3243 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:sprites4.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2747 -> 5318 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmapb3.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4675 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk5am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3584 -> 5576 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:spritesb.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28242 -> 71552 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmapb1.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3233 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk3am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3970 -> 5896 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mplxobjb.hs: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (468 -> 1268 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:sprites4.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (41241 -> 94776 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrg4.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1558 -> 2132 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mtrgb5.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (744 -> 1240 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mfmap5.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6533 -> 40896 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mparasp4.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (290 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:set3.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11272 -> 22476 bytes)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 136 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk1am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:mblk6am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk4am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblkbam.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk5am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf:mblk3am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.020, entropy 0.84)
- bubba 'n' stix (1994)(core design)[cr ltd](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bubba 'n' stix (1994)(core design)[cr ltd](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:Bubba: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 6 never-loaded region(s), 193152 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 4608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:INTRO

## Copy protection found

- `configure_ciab_port` (runtime segment an address): Writes a sequence of four bytes to the CIAB Port B data register. Kind: disk hardware/port configuration check. Pass path: moveq #an address, d0.

## Functions


### Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:INTRO

- `decompress_packed_data`: Reads a packed bitstream using a checksum-validated bit reader and expands it into a destination buffer starting at a specific memory address.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:Bubba

- `decompress_data_block`: Identifies a data structure via a magic constant, then performs a complex decompression loop that expands bytes using bit-shifting and arithmetic operations to populate a destination buffer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Bubba 'N' Stix (1994)(Core Design)[cr LTD](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a short branch to a specific address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_execute_subsystems`: Calls a routine via a pointer at an offset from a global variable, updates several global memory locations, and then enters a loop that calls manage_system_state_2, initialize_system_resources, and execute_indirect_function.
- `manage_system_state_2`: Configures hardware registers for color and DMA, updates sprite attributes, and manages memory buffers by calling routines to expand data structures, clear memory, and update global pointers.
- `update_game_state_and_tables`: Subtracts a value from a global variable, clears two 32-bit values at specific offsets within a structure, and performs a series of data copies between a global variable and a table using loops.
- `wait_for_non_zero`: Loops until a 16-bit value at a global variable is zero.
- `configure_blitter_registers`: The routine sets up Blitter control registers by writing specific values to global variables mapped to hardware registers, including source/destination pointers, size, and control words.
- `write_control_values`: Calls another routine and then stores the words 16 and 33520 into the global variables at offsets 0 and 4 of a structure starting at 33952.
- `copy_and_shift_data`: Uses a pointer from a table to copy words from one memory location to another in a loop, then calculates a value by subtracting a constant from d0, shifting it right, and storing it in a global variable.
- `clear_memory_table`: Iterates through a table of pointers and counts, clearing the memory pointed to by each pointer for a specified number of 16-bit words.
- `copy_memory_loop`: Loops through a table to copy 16-bit words from one memory location to another for a specified count.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `decrement_color_values`: Iterates through a list of memory locations to subtract a masked value from each, then shifts a global 16-bit value right by four bits and decrements a global counter.
- `update_color_cycle_value`: The routine shifts a global value right by four bits, replacing it with a default if zero, stores the result to a memory location, and decrements a counter in the adjacent memory word.
- `write_address_pointers`: Writes a 16-bit word and a 32-bit long to two specific global memory locations.
- `increment_color_values`: Iterates through a list of memory locations to increment values based on bitwise comparisons with a global table, then decrements a global counter.
- `decrement_global_word`: Decrements a 16-bit value at a global variable address.
- `add_color_values`: Iterates through a list of memory locations to add a masked value to entries if a bitwise comparison is non-zero, then shifts a global 16-bit value right by four bits and decrements a global counter.
- `update_color_cycle_value_2`: Shifts a 16-bit value in a register right by four bits, replaces it with 3840 if the result is zero, stores the result at the address in a0, and decrements the 16-bit word at the following address.
- `calculate_blitter_parameters`: Performs arithmetic on d0, d2, d6, and d7, and uses the results to load values from PC-relative tables into d0 or d7 and store them in a3, before branching to another routine or returning.
- `calculate_blitter_offset`: Performs arithmetic on d0, d2, and d6, and if d6 is non-zero, it loads a word from a PC-relative table into d0, swaps the halves of d0, and branches to another routine.
- `execute_blitter_transfer`: Configures Blitter control registers and performs multiple conditional memory transfers by writing to hardware registers and waiting for a status bit to clear.
- `update_sprite_attribute_table`: Reads a value from a table using an index derived from a register, then writes several calculated values and the table entry to a series of global variables.
- `configure_blt_and_update_registers`: Calculates various dimensions and offsets based on input registers to configure the Blitter, then writes values to the Blitter control registers, DMA control, and interrupt enable registers before returning.
- `parse_string_to_data_buffer`: Parses a string from a pointer in a0, converting characters such as spaces, punctuation, and digits into specific numeric values, and writes these values into a stack-allocated buffer.
- `dispatch_calculated_jump`: Clears a byte at a PC-relative address, calculates several values using a global variable and values from a table, and performs an indirect jump to a destination calculated from a base address and a value in d0.
- `update_clamped_values`: Calls another routine, compares a value from a global address with d2, and if it falls within a specific range, updates several global words and a table with values derived from d2 and d4.
- `branch_to_subroutine`: Performs a short branch to another routine.
- `calculate_shifted_values`: Shifts the value in d4 right by two and one bits respectively, adds the original d4 to the second result, and sets d2 to 32768 before branching.
- `adjust_longword_values`: Compares the sign of d3 and branches; if d3 is negative, it negates d0 and adds it to d4, then performs a comparison between d1 and d4 using absolute values to potentially update d4.
- `compare_longwords`: Compares two longwords in registers d1 and d4 and returns if d1 is greater than d4.
- `update_longword`: Checks if d4 is zero or negative; if d4 is positive, it adds d2 to d4 and clears d4 to zero before returning, otherwise it returns.
- `return_immediately`: The routine executes a return instruction to exit the function.
- `calculate_offset_values`: Calculates several values based on the sum of two global variables and stores them into a table at a specific offset from a pointer in a6, including bit-manipulated versions of the sum and a decremented value.
- `setup_blt_transfer`: Calculates a memory offset using a multiplication of two values, then configures hardware registers for a Blitter transfer by writing to various custom chip registers and a table of values.
- `update_sprite_attribute_table_2`: Calculates new attribute values using arithmetic on a global structure and a table, updates a series of longwords and words in a memory block, and performs an indirect call to another routine using a value retrieved from a table.
- `execute_indexed_dispatch`: Modifies two bytes in a global variable, then if a byte at an offset is not negative, it copies a longword to a destination, performs several additions using values from a table, and uses an indirect jump to execute a routine at an address derived from a table and register d2.
- `expand_indexed_byte_data`: Reads pairs of bytes from a source buffer to use as offsets into a table, then writes the resulting bytes from that table into a destination buffer.
- `execute_indirect_jump`: Stores the contents of a1 into a memory location pointed to by a5 and then performs an indirect jump to an address determined by a2.
- `copy_and_branch`: Copies a byte from register d0 to itself and branches to another routine.
- `does_nothing`: The routine consists of a series of instructions that perform no operations or data modifications.
- `transform_color_palette_entries`: Iterates through a block of 16-bit color data, applying arithmetic adjustments to the high and low nibbles to modify brightness and intensity values.
- `write_constant_to_memory`: Writes the word value 1 to a specific memory location.
- `execute_callback_chain`: Loads a byte from a table, modifies it, and uses it to index another table; it then conditionally calls a routine via a pointer, followed by up to two additional function calls via pointers stored in d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `fill_memory_with_zeros`: Fills a block of memory with 1280 zeroed words starting from a location specified by a pointer.
- `update_global_pointers`: Stores the value in a0 into one global variable and stores the value of a0 plus 1400 into another global variable.
- `restore_registers`: Restores the values of d0, d1, a0, a1, and a6 from the stack using a pointer in a7.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_audio_and_structure`: Checks a bit in a hardware register, then performs a series of conditional checks where it calls another routine to set specific values at offsets 150 and 154 of a structure in a6 and clears a byte at a global address.
- `branch_if_not_equal`: Branches to another location if the zero flag is not set.
- `update_structure_values`: Writes 1024 to an offset of 154 in a structure in a6, writes 8 to an offset of 150 in a6, clears a long value at a global address, moves d0 to an offset of 156 in a6, and branches to another routine.
- `copy_and_modify_data`: If a 16-bit word at a0 is not negative, it decrements a value at an offset of -2 from a0, then copies a word from a0 to a1, a long from a0 to a1, and a word from a0 to a1, finally adding d1 to a0 and returning.
- `clear_word_at_offset`: Clears a 16-bit word at offset 8 of the structure pointed to by a1 and returns.
- `initialize_system_resources`: Clears several global bytes and calls multiple routines to initialize system structures, configures input device parameters via a series of calls, and performs a bulk memory allocation and data structure setup involving a loop that clears a table and a complex sequence of memory writes and pointer updates.
- `process_bit_manipulation`: Clears a byte at a PC-relative address if it equals 31, then performs a series of bitwise XOR, rotation, and swap operations on values from a table, updates several global variables and memory locations, and calls another routine.
- `clear_pointer_if_set`: Checks a pointer stored at offset 8 from a base address in a1; if it is non-zero, it sets the pointer to zero and calls another routine, returning the result of that call or zero.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `clear_global_byte`: Clears a byte in a global variable and returns.
- `manage_system_state`: A large routine that interacts with hardware registers for color and DMA, calls numerous routines to update sprite attributes, parse strings, transform color palettes, execute callback chains, fill memory with zeros, expand buffers, and start a copper list.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `expand_and_reallocate_buffer`: Expands a data structure by calculating new sizes based on word values, performs multiple memory reallocations and copies, and updates a linked list or table of memory blocks.
- `contains_zeroed_data_table`: This routine consists of a sequence of sixteen instructions that perform a logical OR operation with zero on a data register, effectively acting as a table of sixteen zeroed words.
- `initialize_data_pointer`: Performs sixteen bitwise OR operations on register d0 with zero, then calculates a new address by adding a constant to a value from a global variable, stores this address in a global variable and another location, and branches to another routine.
- `copy_table_pointers`: Loads a pointer from a global variable into register a0, copies two longwords from global variables into the memory pointed to by a0, sets two registers, and branches to another routine.
- `setup_memory_pointers`: Loads a pointer into a0, calculates a new address by adding a constant to a value from a global variable, stores this address in a global variable, copies a longword from the memory at a0 to another global variable, and branches to another routine.
- `assign_fixed_pointers`: Loads a pointer into a0, stores a constant longword and a value from a global variable into the memory pointed to by a0, sets two registers, and branches to another routine.
- `calculate_offset_value`: Calculates a value by multiplying a word from a global variable by five, negating it, and adding a constant, then sets a register and branches to another routine.
- `process_resource_data`: Calculates a value based on a word from a global variable, calls several routines including one to update global pointers, and performs a loop that writes values from a table into a global variable before writing a byte to a hardware register and calling another routine.
- `dispatch_image_transformation_routine`: Reads a 16-bit word from a pointer in a0 to calculate an offset into a table in a1, then uses a portion of that word to select and jump to one of several subroutines that perform color matrix transformations on blocks of pixel data.
- `iterate_and_compare_table`: If d0 is zero, it enters a loop that iterates up to 31 or 15 times depending on a bit in d1, comparing a value from a table to d2 and updating d2 with the value from the table, then calls a function pointer from a global structure.
- `check_memory_availability`: Checks a global flag and a bit in d1 before calling a function pointer stored in a global structure to query available memory.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `compare_and_update_table`: Calls another routine, compares d0 and d2, and if d0 is smaller, calls a function via a pointer in a6; if d0 is not zero, it updates a table at a PC-relative address using values derived from d1 and d2.
- `loop_table_search`: Loads a table address into a2, sets d1 to 4, calls another routine, and loops back to the start of the routine if the result is non-zero.
- `load_address_and_constant`: Loads a constant into a2 from a PC-relative address and sets d1 to 2.
- `update_memory_table`: Increments d0 and masks it, then iterates through a table pointed to by a2 to find the smallest value within a range; if no value is found, it calls two different routines, otherwise it updates the table by adjusting values at the found location and returns a 64-bit value composed of d0 and d1.
- `decrement_and_call_free`: Decrements a global 16-bit counter and calls a function pointer if the counter remains positive.
- `branch_on_value`: Loads a 16-bit word from a PC-relative address into d0, and if d0 is not less than or equal to zero, calls another routine and branches to itself.
- `branch_to_offset_2`: Loads an offset into a4 and performs a short branch to another location.
- `branch_to_next_routine`: Performs a short branch to the next instruction.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_add_rts`: Small routine of 2 instructions that adds -(a3) to d0, then returns.
- `dispatch_via_table_search`: Iterates through a table of 32 entries using a dbra loop, comparing values to d0 and d3 to determine an offset, then performs an indirect jump using a value in a4 and an offset in d4.
- `add_and_store_pointer`: Adds the value in d0 to a 32-bit value pointed to by a2, stores the result back to that location, and then pushes the value of a0 onto the stack at the location pointed to by a2 minus 4.
- `find_minimum_and_store`: Iterates through a table of 16 entries to find the minimum value, then stores the value from d0 and the value from a0 at the address of the minimum entry found.
- `find_minimum_and_store_2`: Iterates through a table of 16 entries to find the minimum value, then stores the value from d0 and the value from a0 at the address of the minimum entry.
- `branch_to_previous_routine`: Performs a 32-bit addition of a value in d0 to a memory location pointed to by a2, stores the result back to that location, pushes a0 onto the stack, and then branches to a previous instruction.
- `parse_resource_header_and_search_table`: Checks a global variable for a null pointer and returns; otherwise, it concatenates two strings into a local buffer, searches a table for the minimum value among entries, and then iterates through a list of structures to find and process an entry matching that minimum value by calling another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a logical OR on d0 with zero.
- `perform_logical_or_alt`: Performs a logical OR on d0 with zero.
- `clear_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `execute_library_functions`: Calls a library function via a pointer in a6, stores the result in a global variable, and if non-zero, calculates a new value by adding 3 to an address and masking with a word, stores that value in another global variable, calls a second library function, and if that result is non-zero, calls a third library function using the first stored value as an argument.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_30_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `check_file_read_result`: Invokes a function pointer from a global table to perform a file read operation.
- `invoke_file_write_handler`: Calls a function pointer located within a global table structure using a value from a register, then compares the result with a value stored on the stack.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `prepare_library_call_arguments`: Sets registers d2 and d3 to zero and negative one respectively, then branches to another routine.
- `load_long_from_pointer`: Loads a pointer from a PC-relative address into a0, moves a long from an offset of 124 in a0 into d0, and returns.
- `configure_hardware_and_set_pointer`: Saves registers, clears a long at a global address, writes 33360 to the DMA control register and 49156 to the interrupt enable register, calls a function via a pointer in a6, sets a byte at a global address, writes 50 to a PC-relative address, stores that address in a global variable, and restores registers.
- `decrement_and_clear_global`: Decrements a byte in a global variable and, if the result is zero, clears a long value in another global variable.
- `disable_interrupts_and_set_hardware`: Calls a function via a pointer in a library base, increments a global variable, sets a value in the INTENA register, and clears a byte in a global variable.
- `clear_register_d0`: Performs a bitwise OR operation on register d0 with zero.
- `branch_to_subroutines`: Clears bit 0 of d0, modifies the stack pointer by adding 12, calls another routine, and then branches to either another routine or a different location.
- `parse_resource_header`: Checks a global variable for a specific magic number and, if matched, parses a structured data block by iterating through a list of entries.
- `execute_indirect_function`: Calls a function via a pointer stored at an offset from a global variable, and if a specific word at an offset from that same global variable matches 20217, calls the function again and updates the global variable with the result.
- `fill_memory_with_zeros_loop`: Fills two destination memory areas pointed to by registers a0 and a1 with zeros using a loop that runs 9600 times.
- `fill_memory_with_zeros_large_loop`: Fills two destination memory areas pointed to by registers a0 and a1 with zeros using a loop that runs 13408 times.
- `update_random_seed`: Performs bitwise XOR and shift operations on a series of four 16-bit values stored in a global table to update their state.
- `process_keyboard_input`: Reads a byte from the CIAA serial data register, rotates it, and stores it in a circular buffer if it meets specific conditions and the buffer is not full.
- `read_keyboard_buffer`: Checks if a read pointer in a table matches a global variable; if they differ, it retrieves a byte from the table, increments the pointer, and returns it, otherwise it returns zero.
- `save_register_context`: Stores the values of the data register d0, address registers a0, a1, and a2, and a constant into specific global memory locations.
- `update_pointer_and_branch`: Stores a long value into a global variable, loads a value from a memory location, and branches to another routine if the value is non-zero.
- `configure_ciab_port`: Writes a sequence of four bytes to the CIAB Port B data register.
- `initialize_and_call_routine`: Loads a PC-relative address into a4 and a5, clears a word at the address in a5, moves an address into d0 and d7, calls another routine, and if d0 is not negative, sets d1 to 2.
