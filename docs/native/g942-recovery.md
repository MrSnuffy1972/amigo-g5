# Game 942 (bubba 'n' stix (1994)(core design)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00942). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1807920 top-level bytes in 187 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (4 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1023 of 1023 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 193152 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1562 of 1562 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8371 |
| code | 5200 |
| data | 1734930 |
| padding | 23067 |
| unused | 36352 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:PaRaDoX | 2 | 267 | 8 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:Bubba | 3 | 202 | 2 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 283 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 800 | 12 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk1am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk6am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk4am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblkbam.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk5am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk3am.chr#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:fgplax1a.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2287 -> 19598 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:set1.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6868 -> 15916 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mfmap1.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6393 -> 40590 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mparasp1.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1342 -> 3392 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mplxobj1.hs: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (797 -> 2704 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk6am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4092 -> 6040 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk1am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11914 -> 33408 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:complete.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9876 -> 12544 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:level6.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (29974 -> 42930 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:sprites1.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2714 -> 5214 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mfmap6.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5463 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk1am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3357 -> 4560 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:fgplax1b.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (400 -> 2462 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:sprites1.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49404 -> 96384 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/titledlt.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21238 -> 57702 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/continue.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22753 -> 30412 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/titlepic.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7345 -> 11818 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/title.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (33588 -> 43062 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/getready.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2952 -> 3172 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/map.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22744 -> 33760 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:BubbaData/gameover.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2911 -> 3188 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mtrg1.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (866 -> 1108 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:lev1info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (895 -> 2408 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:sprites6.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2721 -> 4934 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:disk1.spk: high-entropy data file (7.85 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:endseq.bit: high-entropy data file (7.66 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:sprites6.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40190 -> 89064 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mtrg6.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1304 -> 1826 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mparasp6.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1570 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:set5.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (13987 -> 27660 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:lev6info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (937 -> 4650 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk6am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14871 -> 39456 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:level1.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (23559 -> 30916 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:lev4info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (243 -> 1026 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrgb3.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1727 -> 2996 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmap3.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8603 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mparasp3.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1228 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk4am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8784 -> 22176 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:level2.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36160 -> 48732 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrgb1.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (688 -> 1222 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:disk2.spk: high-entropy data file (7.83 bits/byte), classified as packed data
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:level3.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30461 -> 44582 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:sprites5.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3306 -> 5698 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:level4.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36565 -> 52314 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmapb4.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1077 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:bonus.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (19879 -> 35880 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:sprites3.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2702 -> 4458 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmapb2.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2436 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk4am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2042 -> 2744 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:setb.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7692 -> 16432 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblkbam.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11771 -> 32608 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mparaspb.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (736 -> 1280 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:sprites5.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (48322 -> 97800 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrg5.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1543 -> 2250 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mparasp5.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (924 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:levbinfo.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (571 -> 1960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:set4.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8546 -> 18102 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:sprites3.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (47506 -> 96172 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:lev5info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (247 -> 1734 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrg3.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1699 -> 2520 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrgb4.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (306 -> 478 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmap4.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7783 -> 40902 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk5am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11559 -> 33376 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:set2.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11877 -> 24278 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/titledlt.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (21238 -> 57702 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/continue.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22753 -> 30412 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/titlepic.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7345 -> 11818 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/title.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (33588 -> 43062 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/getready.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2952 -> 3172 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/map.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22744 -> 33760 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:BubbaData/gameover.np: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2911 -> 3188 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:lev3info.obj: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1170 -> 4636 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrgb2.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (606 -> 944 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk3am.chr: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10589 -> 32128 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:spritesb.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2089 -> 3746 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblkbam.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2880 -> 3752 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmapb5.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3243 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:sprites4.ant: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2747 -> 5318 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmapb3.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4675 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk5am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3584 -> 5576 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:spritesb.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28242 -> 71552 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmapb1.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3233 -> 40960 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk3am.pts: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3970 -> 5896 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mplxobjb.hs: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (468 -> 1268 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:sprites4.adt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (41241 -> 94776 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrg4.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1558 -> 2132 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mtrgb5.trg: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (744 -> 1240 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mfmap5.dat: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6533 -> 40896 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mparasp4.mac: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (290 -> 2048 bytes)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:set3.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11272 -> 22476 bytes)
- runtime segment an address: 224 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 380 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk1am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:mblk6am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk4am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblkbam.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk5am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf:mblk3am.chr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.020, entropy 0.84)
- bubba 'n' stix (1994)(core design)[cr pdx](disk 2 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- bubba 'n' stix (1994)(core design)[cr pdx](disk 1 of 2)/Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:Bubba: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 6 never-loaded region(s), 193152 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 4608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:PaRaDoX

- `load_hunk_executable`: Program entry that walks AmigaDOS hunk headers, comparing type codes, and allocates and frees memory through exec.
- `delay_countdown_loop`: Counts a 16-bit register down from its starting value through the full range until it wraps, producing a fixed delay with no other effect.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `decode_bit_packed_run_block`: Reads a length from the input word, then repeatedly calls a shifted-flag check and writes the decoded values to an output buffer.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `pull_bits_from_shift_register`: Pulls bits from a multi-word shift register using a bit count held in one register, reloading a longword when the register runs empty.
- `invalidate_caches_then_fault`: Traps into the system to invalidate cache lines and then reaches a bad-data fault.

### Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:Bubba

- `walk_startup_pointer_list`: Program entry that walks a zero-terminated list of pointers, using each entry's value to choose a branch of work and calling one routine as it goes.
- `stub_no_operation_return`: Contains only a return and performs no work, so it acts as an empty callback target.

### Bubba 'N' Stix (1994)(Core Design)[cr PDX](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_copper_and_input_loop`: Sets up two copper pointers, disables an interrupt, and waits for the blitter before running a loop.
- `wait_status_then_set_bit`: Waits until a status byte equals a fixed value and a flag byte is nonzero, then returns the input word with the bit at the input index set.
- `wait_status_flag_ready`: Waits until a status byte equals a fixed value and a flag byte is nonzero, then returns without changing anything.
- `run_until_flag_clears`: Sets a global to one, then repeatedly calls two routines, one at a fixed address, until the global reads zero.
- `initialize_copper_list_and_data`: Populates a memory range with incrementing longwords and subsequently writes a series of orw commands or orw-like values and a certain bitmask or constant into a secondary table or copperlist buffer.
- `advance_timed_event_sequence`: Decrements a delay counter and, when it reaches zero, advances through a table of timed entries to the first entry above the current value.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `select_copper_pointer_for_frame`: Reduces a position word modulo 80, then selects one of three copper-list addresses by a toggle and a state flag.
- `pack_sprite_control_bytes`: Writes four bytes into a record: the low bytes of two position values and their sum, plus a flags byte that records the high bit of each value.
- `step_position_state_machine`: Writes a value into two copper pointer slots and moves a position by four units in a direction set by a state value.
- `init_record_defaults_after_wait`: Waits until a status bit clears, then sets three longwords of a record to fixed defaults.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_audio_dma_state`: Builds table entries from a sorted byte table, clearing each entry's words at a fixed stride and resetting several globals.
- `tick_audio_and_reload_pointers`: Increments a tick counter and, when it reaches its limit, runs four update routines, pauses with two countdown loops, and writes the four audio channel address register pairs from table values.
- `apply_music_channel_effect`: Runs the nibble-selected effect step for one audio channel, updating a word from a table indexed by a global counter modulo 3.
- `load_audio_instrument_entry`: Reads a 30-byte entry from an instrument table and copies its start address, length, volume and period fields into the channel state.
- `apply_effect_command_to_record`: Applies the nibble-selected effect step to one object record, updating its position word from a table indexed by a nibble and the global counter.
- `copy_record_position_word`: Copies the 16-bit field at offset 16 of the record into the output slot passed by the caller and returns.
- `find_threshold_table_entry`: Scans a threshold table selected by a record index for the first entry at or below the record's masked position value, falling back to a fixed entry when none match.
- `step_position_toward_target`: Moves a 16-bit position toward its target by a per-step delta, snapping to the target and clearing the target field once it is reached.
- `compute_wave_offset_step`: Computes an offset from a phase byte and a 4-bit amplitude, taken from a table, a ramp or a fixed maximum depending on a mode field, and applies the sign.
- `run_record_effect_command`: Runs a command selected by the low nibble of a record's command byte: command 9 scales a parameter into a counter, command 11 stores a fixed value, and command 13 combines two nibbles and clamps at 63.
- `store_record_low_nibble`: Copies the low nibble of the record's parameter byte into a second record field and returns.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
