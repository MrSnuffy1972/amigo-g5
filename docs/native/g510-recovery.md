# Game 510 (Historyline 1914-1918): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00510). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6315008 top-level bytes in 1154 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 32 of 32 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 748 of 748 named (0 author symbols), 748 of 748 described (27 by rule, 718 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 461 of 461 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 469860 bytes, 461 functions |
| Independent decoder cross-check (Ghidra against capstone) | 20960 of 22504 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 668202 |
| code | 28200 |
| data | 4875096 |
| padding | 169046 |
| unused | 574464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:install | 4 | 1616 | 63 |
| Run | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| Assign | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| BBStart | 5 | 546 | 24 |
| Execute | 2 | 117 | 2 |
| Copy | 2 | 117 | 2 |
| BBCopy | 4 | 463 | 27 |
| IconX | 12 | 806 | 37 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:flt.exe | 1 | 99 | 3 |
| Run | 2 | 117 | 2 |
| BBstart | 5 | 546 | 24 |
| Echo | 2 | 117 | 2 |
| execute | 2 | 117 | 2 |
| BBCopy | 4 | 463 | 27 |
| EndIf | 1 | 2 | 1 |
| If | 2 | 117 | 2 |
| IconX | 12 | 806 | 37 |
| BBAsk | 3 | 47 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:bootblock | 1 | 10 | 1 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 970 | 17 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:intro.dwn#unpacked (raw code file, load address unknown) | 1 | 7228 | 186 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO.DWN#unpacked (raw code file, load address unknown) | 1 | 6930 | 197 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:HL.DWN#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUCHR8.LIB#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:HL.DWN#unpacked (raw code file, load address unknown) | 1 | 9141 | 78 |

## Cases handled

- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: file TXT/H43.LBM (header 25311 bytes, blocks hold 1952): its blocks are judged as raw blocks outside the filesystem
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: file TXT/H42.LBM (header 26605 bytes, blocks hold 4392): its blocks are judged as raw blocks outside the filesystem
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:CHAR6.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (426 -> 1108 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/HLINTRO2.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (786 -> 4756 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/BANGPAL.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (430 -> 1664 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/AUTO.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (51 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/SCROLL.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (17979 -> 40724 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/MORD.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (90639 -> 116752 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/BUCH1B.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (77284 -> 114794 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/BUCH1B.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/BANG.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (53853 -> 106202 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/gun.fx: TPWM: Turbo Packer-packed, unpacked with ancient (4066 -> 4550 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/BUCH1A.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (130281 -> 247037 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/CHR14.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (2030 -> 5834 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/HLINTRO3.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (567 -> 2708 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/bang.txt: TPWM: Turbo Packer-packed, unpacked with ancient (154 -> 422 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/HLINTRO1.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1051 -> 6804 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/AUTO.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (99101 -> 153520 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/MORD.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (51 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/FRONT.FX: TPWM: Turbo Packer-packed, unpacked with ancient (44639 -> 59376 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/BUCH1A.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:INTRO/hlintro4.sng: TPWM: Turbo Packer-packed, unpacked with ancient (1367 -> 8852 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:intro.dwn: TPWM: Turbo Packer-packed, unpacked with ancient (21050 -> 34704 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/CRASH312.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (9081 -> 9596 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/PIANO2.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (7186 -> 6972 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/STRB1L3.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (5489 -> 5488 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/SNARE3.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (6333 -> 7240 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/TUBA2B.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (2505 -> 4216 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/KICK1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (1328 -> 3808 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/GUIT1MID.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (7296 -> 10080 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/HARP1HI.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (6026 -> 8784 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/ERAIN1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (38417 -> 44260 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/HARP1MID.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (5837 -> 9392 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTINST/STRM2L4.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11535 -> 11168 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:CHAR6.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (426 -> 1108 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BUCH3.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (79744 -> 116429 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BUCH2A.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BUCH2B.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (140675 -> 230885 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/RED.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BUCH3.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (55 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BASE.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (74 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/buch3.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1923 -> 3576 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BUCH2B.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (76 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/BUCH2A.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (59969 -> 87709 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:INTRO/LAENDER.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (17924 -> 25522 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO.DWN: TPWM: Turbo Packer-packed, unpacked with ancient (18050 -> 29176 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/THEENDB.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (100789 -> 165977 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/VERTRAG.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (24655 -> 27266 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EISENBAH.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/STAFF.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (840 -> 2023 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EURO.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (79 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/VERTRAG.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/vertraga.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1009 -> 1483 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/THEEND.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/KRATER.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (311 -> 673 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EISEN2.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (77 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EISEN0.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (25 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/THEENDA.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (66014 -> 143265 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/WAREND.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1902 -> 3119 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/KRATER.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (77 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/vertragb.txt: TPWM: Turbo Packer-packed, unpacked with ancient (500 -> 674 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/VERTRAG2.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EURO.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (91733 -> 145610 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/STAFF.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (2387 -> 14996 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EXTRO.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1345 -> 8852 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EISEN1.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (77 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/EISENBAH.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (27316 -> 33558 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO/KRATER.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (11377 -> 15976 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXTINST/STRB1L3.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (5486 -> 5488 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXTINST/SNARE1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (3652 -> 4340 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXTINST/TUBA2B.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (2504 -> 4216 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXTINST/KICK1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (1328 -> 3808 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXTINST/STRM2L4.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11535 -> 11168 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0815.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (935 -> 1330 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H40.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (20880 -> 22674 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H07.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (24691 -> 27848 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0615.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1143 -> 1663 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1215.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1518 -> 2268 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H05.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (27882 -> 28934 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/ZWTXT.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (75 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0316.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1779 -> 2731 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0415.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1378 -> 2138 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0914.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (6781 -> 10636 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1115.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1309 -> 1885 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H03.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (25758 -> 27208 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H10.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (17672 -> 20678 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0215.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1582 -> 2479 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H38.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (28429 -> 29374 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1014.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1548 -> 2501 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H01.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (24483 -> 27024 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H36.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (22882 -> 25184 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0115.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1564 -> 2446 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/ZWTXT1.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (949 -> 5780 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1115.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3343 -> 4790 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1214.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (6270 -> 9826 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0914.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1415 -> 2343 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0216.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3495 -> 5800 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0315.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (5068 -> 7330 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1014.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3307 -> 4540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H41.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (9310 -> 10848 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H08.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (19419 -> 23388 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0115.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (4528 -> 7066 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0715.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1256 -> 2033 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0814.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1601 -> 2559 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H06.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (26088 -> 26950 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0515.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1530 -> 2367 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0915.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7295 -> 10636 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1214.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1987 -> 3171 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H04.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (21056 -> 22928 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0315.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1695 -> 2722 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0216.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1227 -> 1744 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0715.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3630 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0814.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (17450 -> 25804 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H39.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (26585 -> 28420 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1015.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (810 -> 1206 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1114.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1323 -> 1878 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H02.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (22897 -> 24368 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H37.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (24099 -> 26752 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0116.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1380 -> 2079 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/ZWTXT2.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1043 -> 5780 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H00.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (23188 -> 24722 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H44.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (27734 -> 28130 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0316.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3534 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0415.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7745 -> 10696 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0915.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1324 -> 1919 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/1114.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7114 -> 10636 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/H09.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (26463 -> 27762 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0116.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (2859 -> 3880 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:TXT/0215.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (4097 -> 5560 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:ZWI/ZEPP.FX: TPWM: Turbo Packer-packed, unpacked with ancient (15462 -> 14212 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:ZWI/ZEPP.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (79 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf:ZWI/ZEPP.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (116382 -> 213382 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0617.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1288 -> 1964 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0716.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1262 -> 1807 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1217.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1682 -> 2675 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H24.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (29282 -> 29766 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H31.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (29667 -> 30454 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H14.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (27099 -> 27912 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0318.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1335 -> 2068 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0417.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1591 -> 2442 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0516.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1764 -> 2581 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0817.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3209 -> 4790 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0916.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3084 -> 4480 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H22.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (18421 -> 23040 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H12.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (22596 -> 25810 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0118.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (891 -> 1264 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0217.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1425 -> 2036 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0617.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7707 -> 10466 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0716.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (4045 -> 5620 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1016.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1551 -> 2488 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H20.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (25210 -> 27494 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0117.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1176 -> 1736 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1117.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (6715 -> 9676 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1216.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3267 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H28.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (15827 -> 19576 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0817.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1167 -> 1678 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0916.txt: TPWM: Turbo Packer-packed, unpacked with ancient (966 -> 1415 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H19.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (28167 -> 29342 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0218.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3807 -> 5800 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0317.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3878 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0416.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3193 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1016.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3946 -> 5260 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H26.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (20200 -> 24734 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H17.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (25182 -> 26318 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0117.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3856 -> 4900 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0717.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1620 -> 2557 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H32.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (22852 -> 23638 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H15.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (30636 -> 31320 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0517.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (888 -> 1166 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0616.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1722 -> 2532 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0917.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (4312 -> 5800 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1117.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (752 -> 1019 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1216.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1769 -> 2877 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H23.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (26996 -> 27902 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H30.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (24234 -> 27164 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H13.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (25800 -> 26374 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0416.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1202 -> 1873 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0218.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1426 -> 2192 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0317.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1044 -> 1434 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0816.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (4043 -> 5800 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0717.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3563 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1017.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1476 -> 2200 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1116.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1494 -> 2203 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H21.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (28231 -> 29728 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H11.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (31288 -> 30244 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0616.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (11632 -> 16972 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1217.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (4072 -> 5800 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H29.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (25182 -> 27270 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0917.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1217 -> 1720 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0318.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7121 -> 10636 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0417.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7002 -> 9626 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1017.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (8204 -> 11236 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/1116.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7277 -> 10586 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H27.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (26152 -> 27258 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0816.txt: TPWM: Turbo Packer-packed, unpacked with ancient (1266 -> 1908 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H18.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (22281 -> 24388 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/0118.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3235 -> 4840 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H25.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (23621 -> 24618 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:TXT/H16.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (17176 -> 22730 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:ZWI/SHIP.FX: TPWM: Turbo Packer-packed, unpacked with ancient (37107 -> 39950 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:ZWI/SHIP.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (747 -> 1966 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:ZWI/SHIPA.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (57019 -> 77037 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf:ZWI/SHIP.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (79 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:HL.DWN: TPWM: Turbo Packer-packed, unpacked with ancient (92191 -> 196256 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0518.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (927 -> 1307 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0718.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (7242 -> 10576 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0418.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (5528 -> 11310 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/H45.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (12701 -> 15662 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/H34.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (18182 -> 20694 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0618.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (856 -> 1163 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0418.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1235 -> 1745 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0618.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (3602 -> 5800 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/H35.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (31086 -> 30284 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/0718.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (785 -> 971 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:TXT/H33.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (22621 -> 25788 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/CRASH312.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (9079 -> 9596 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/C5PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (10409 -> 11812 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/C4PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (10357 -> 11676 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/STRB1L3.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (5490 -> 5488 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/C3PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11414 -> 11980 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/SNARE1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (3649 -> 4340 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/TUBA2B.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (2505 -> 4216 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/KICK1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (1328 -> 3808 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWIINST/STRM2L4.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11535 -> 11168 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OBRUST.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (16091 -> 16438 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OFRA.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (19345 -> 21861 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OFRA.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/RICHT.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (17534 -> 21696 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/ORDEN_A.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (25453 -> 24642 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/HQ.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/RICHT.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (2502 -> 13114 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/ALLE.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (78 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OGER.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (33600 -> 48308 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OGER.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OGER2.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OFRA.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1314 -> 8852 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/ORDEN_F2.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (869 -> 1783 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OFRA.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (10879 -> 18388 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/ORDEN_F1.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (1347 -> 2339 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/HQ_FRA.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (1346 -> 2710 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OGER.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1073 -> 6804 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/HQ.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (67226 -> 90808 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OGER.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (10070 -> 21040 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/ALLE.FX: TPWM: Turbo Packer-packed, unpacked with ancient (37107 -> 39950 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OFRA2.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/RICHT.FX: TPWM: Turbo Packer-packed, unpacked with ancient (37107 -> 39950 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/HQ_GER.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (2046 -> 4200 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/ALLE.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (80393 -> 112112 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/RICHT.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (2012 -> 4455 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/RICHT.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/SIEG.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (519 -> 1684 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:ZWI/OF1.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/STATS.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (67 -> 72 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/BIGUNIT.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (318 -> 392 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/UMAPDAW2.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (597 -> 1624 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/PARTW.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (643 -> 1400 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/newshop.lib: TPWM: Turbo Packer-packed, unpacked with ancient (11636 -> 13726 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/CURSOR.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (2076 -> 5220 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/UMAPDASA.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (837 -> 2324 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/PARTS.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (643 -> 1400 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/MNUFAHN.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (384 -> 2142 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/UMAPDAWA.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (921 -> 2398 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/UNIT.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (38850 -> 96412 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/EXP.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (314 -> 560 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/PARTW.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (47500 -> 87232 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/NEWCHAR6.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (441 -> 1110 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/PARTS.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (48943 -> 89248 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/NEWUNIT.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (5663 -> 10305 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/UNIT.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (1212 -> 2448 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/bum.lib: TPWM: Turbo Packer-packed, unpacked with ancient (1161 -> 3616 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:LIB/UMAPDAS2.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (576 -> 1604 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUTIT.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (3505 -> 8756 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:AMOK.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (49 -> 36 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:HEXMSK00.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (71 -> 96 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUMISC.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (1293 -> 6664 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:CODES.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (500 -> 720 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUCHR18.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (6886 -> 14576 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUSCORE.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (16519 -> 21220 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUCHR8.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (1498 -> 4636 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUCHR14.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (2030 -> 5834 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ONGAME3.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1074 -> 6804 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:WW1GNDW.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (270 -> 1050 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:HEXMSK20.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (71 -> 96 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ONGAME1.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (949 -> 5780 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUFAHN.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (1912 -> 7502 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:GAME.TXT: TPWM: Turbo Packer-packed, unpacked with ancient (1539 -> 2190 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:WW1GNDS.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (269 -> 1050 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/CRASH312.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (9079 -> 9596 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/C5PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (10409 -> 11812 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/C4PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (10357 -> 11676 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/STRB1L3.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (5490 -> 5488 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/C3PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11414 -> 11980 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/SNARE1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (3649 -> 4340 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/TUBA2B.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (2505 -> 4216 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/KICK1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (1328 -> 3808 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ZWIINST/STRM2L4.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11535 -> 11168 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUMAUS.DAT: TPWM: Turbo Packer-packed, unpacked with ancient (250 -> 484 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNU.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1400 -> 8852 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUHI.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1008 -> 6804 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:zwi/OFRA.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1314 -> 8852 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:zwi/OGER.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1073 -> 6804 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:zwi/SIEG.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (519 -> 1684 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:GAME.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (6476 -> 14582 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/27.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/34.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1629 -> 3204 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/33.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (77 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/34.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/18.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1169 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/16.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (110 -> 220 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/40.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (86 -> 160 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/17.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/42.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1150 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/69.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1122 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/67.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (75 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/23.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (57 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/25.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (808 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/42.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/06.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (134 -> 220 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/08.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (668 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/25.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/08.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/31.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (69 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/32.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/16.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1507 -> 3204 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/58.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (66 -> 112 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/14.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (139 -> 196 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/40.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1641 -> 3204 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/67.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1098 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/16.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/65.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (66 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/21.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (116 -> 184 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/23.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (966 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/40.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/06.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1271 -> 3204 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/23.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/49.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (93 -> 208 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/05.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (73 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/06.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/31.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1016 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/30.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/14.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1375 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/56.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 136 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/58.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (569 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/12.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (177 -> 268 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/39.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (75 -> 112 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/65.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (820 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/14.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/21.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1606 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/63.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (118 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/48.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (688 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/21.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/47.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (57 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/03.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (108 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/71.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (182 -> 268 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/04.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/54.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (86 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/10.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (146 -> 244 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/12.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1252 -> 2564 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/56.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1215 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/37.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (76 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/39.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (726 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/12.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/63.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1477 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/39.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/45.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (114 -> 196 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/03.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (901 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/01.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (139 -> 208 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/47.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (993 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/46.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/02.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/71.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (2399 -> 6148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/28.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (105 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/52.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (85 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/10.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1636 -> 3204 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/54.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1111 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/35.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (62 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/37.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (757 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/10.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/61.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (660 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/37.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/60.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (182 -> 232 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/19.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (135 -> 244 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/43.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (113 -> 208 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/45.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1695 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/01.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (780 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/44.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/26.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (79 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/28.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1087 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/50.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (114 -> 160 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/09.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (107 -> 160 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/52.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (871 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/28.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/35.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (619 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/35.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/19.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1530 -> 2564 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/60.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (2075 -> 4100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/17.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (97 -> 136 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/41.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (98 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/18.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/43.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1515 -> 2564 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/68.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (46 -> 76 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/24.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (67 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/26.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (749 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/43.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/07.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (83 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/09.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (758 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/50.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1031 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/26.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/33.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (800 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/32.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (78 -> 136 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/09.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/33.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/17.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1129 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/59.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (68 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/15.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (83 -> 136 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/41.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1172 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/68.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (438 -> 516 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/66.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (68 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/22.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (101 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/24.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (479 -> 772 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/41.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/07.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1025 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/24.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/07.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/30.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (123 -> 184 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/32.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (730 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/31.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/15.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (736 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/57.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (57 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/59.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (781 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/13.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (74 -> 136 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/66.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (727 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/15.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/22.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1676 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/64.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (59 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/20.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (140 -> 196 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/05.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (661 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/49.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (955 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/22.COM: TPWM: Turbo Packer-packed, unpacked with ancient (189 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/48.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (73 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/04.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (137 -> 184 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/05.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/30.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1342 -> 3204 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/55.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (45 -> 76 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/57.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (381 -> 516 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/11.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (79 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/13.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (753 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/38.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (135 -> 184 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/13.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/64.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (601 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/20.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1166 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/62.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/20.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/46.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (88 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/04.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1018 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/02.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (104 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/47.COM: TPWM: Turbo Packer-packed, unpacked with ancient (214 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/70.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (57 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/03.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/29.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (74 -> 124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/53.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (84 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/11.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (629 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/55.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (437 -> 772 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/36.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (153 -> 208 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/38.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1394 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/11.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/62.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1375 -> 6148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/38.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/61.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (64 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/44.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (142 -> 196 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/00.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (62 -> 100 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/46.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1764 -> 3076 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/02.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (742 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/70.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (487 -> 772 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/45.COM: TPWM: Turbo Packer-packed, unpacked with ancient (214 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/01.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/27.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (83 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/29.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (776 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/51.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (54 -> 88 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/53.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (917 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/29.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/34.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (139 -> 196 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/36.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1234 -> 2564 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/36.COM: TPWM: Turbo Packer-packed, unpacked with ancient (215 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/18.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (153 -> 232 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/42.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (135 -> 196 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/19.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/00.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (498 -> 772 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/44.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (1178 -> 2052 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/69.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (124 -> 160 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/25.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (103 -> 172 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/27.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (899 -> 1540 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/00.COM: TPWM: Turbo Packer-packed, unpacked with ancient (212 -> 306 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/08.SHP: TPWM: Turbo Packer-packed, unpacked with ancient (112 -> 148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MAP/51.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (481 -> 1156 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:NEWUNIT.LBM: TPWM: Turbo Packer-packed, unpacked with ancient (3054 -> 9894 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUICON.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (2030 -> 6628 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/CRASH312.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (9081 -> 9596 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/PIANO2.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (7187 -> 6972 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/TAMBU1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (2340 -> 3244 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/TOM1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (3115 -> 5124 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/C5PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (10409 -> 11812 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/C4PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (10357 -> 11676 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/STRB1L3.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (5489 -> 5488 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/C3PIANO.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11414 -> 11980 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/SNARE1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (3652 -> 4340 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/TUBA2B.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (2504 -> 4216 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/KICK1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (1328 -> 3808 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/BPIANO1.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (4266 -> 4652 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:INST/STRM2L4.RAW: TPWM: Turbo Packer-packed, unpacked with ancient (11535 -> 11168 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:mnudata.dat: TPWM: Turbo Packer-packed, unpacked with ancient (1383 -> 4878 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:ONGAME2.SNG: TPWM: Turbo Packer-packed, unpacked with ancient (1220 -> 7828 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:newunit.dat: TPWM: Turbo Packer-packed, unpacked with ancient (5357 -> 9807 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:unit.dat: TPWM: Turbo Packer-packed, unpacked with ancient (1462 -> 3570 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:STATMAP.FIN: TPWM: Turbo Packer-packed, unpacked with ancient (140 -> 516 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:BB.FX: TPWM: Turbo Packer-packed, unpacked with ancient (13195 -> 12148 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:lib/NEWCHAR6.LIB: TPWM: Turbo Packer-packed, unpacked with ancient (441 -> 1110 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:HL.DWN: TPWM: Turbo Packer-packed, unpacked with ancient (94204 -> 205088 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:HLLOGO.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (10224 -> 15010 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:ZWI/DEPOTR.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (27981 -> 62972 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:ZWI/DEPOTL.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (27965 -> 62673 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:ZWI/DEPOT.PAL: TPWM: Turbo Packer-packed, unpacked with ancient (80 -> 64 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:BB.ANI: TPWM: Turbo Packer-packed, unpacked with ancient (6701 -> 19015 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:fight/waw.seq: TPWM: Turbo Packer-packed, unpacked with ancient (1008 -> 1812 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:fight/WHMEERHI.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (5766 -> 7324 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:fight/AHMEERHI.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (12700 -> 14342 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:fight/WHHIMMEL.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (10457 -> 12032 bytes)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:fight/AHHIMMEL.IFF: TPWM: Turbo Packer-packed, unpacked with ancient (16474 -> 17888 bytes)
- runtime segment an address: 365 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 1 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:intro.dwn#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO.DWN#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf:HL.DWN#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf:MNUCHR8.LIB#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 7 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:HL.DWN#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 2 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: blocks 6-10 (bytes 3072-5632) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.88)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: blocks 1015-1015 (bytes 519680-520192) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.27)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: blocks 1017-1017 (bytes 520704-521216) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.19)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: blocks 1650-1650 (bytes 844800-845312) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.62)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: blocks 1661-1759 (bytes 850432-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.93)
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 3 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 3 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 4 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 4 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 5 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 5 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- historyline 1914-1918 (1992)(blue byte)[cr flt](disk 6 of 7)/Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 6 of 7).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 2 never-loaded region(s), 200892 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:intro.dwn#unpacked (raw code file, load address unknown)

## Copy protection found

- `update_device_parameter_table` (EXTRO.DWN#unpacked (raw code file, load address unknown)): Updates a global table with a sequence of parameter values and checks a stack-based structure for specific command identifiers. Kind: manual/code-wheel/colour-card lookup. Pass path: the branch where the routine proceeds to load or check subsequent parameters (avoids the infinite loop or error-exit paths).

## Functions


### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:install

- `initialize_library_environment`: Performs a series of indirect calls via pointers in a global structure to handle library initialization and window management, potentially cleaning up or updating pointers.
- `store_register_to_global_2`: The routine stores the address of a global variable into another global variable and then calls another routine via a pointer.
- `store_register_to_global_and_call`: Stores the address of a global variable into a pointer and calls another routine via an indirect pointer lookup from a table.
- `store_address_to_global`: Stores the address of a global variable into a pointer and then executes another routine via an indirect call or a table-based pointer.
- `store_register_to_global`: Stores the value of a register into a global variable and calls another routine via a pointer.
- `set_global_pointer`: Stores the address of a global variable into a pointer and calls another routine via a pointer.
- `find_and_check_task`: Uses a global pointer to locate a task and checks a certain offset in the task structure, potentially calling other functions based on that check.
- `open_window_after_setup`: Calls a setup helper and, on success, opens a window through the library routine at vector offset minus 204, stores the window and its related pointer and returns zero.
- `call_library_functions`: Calls a library function using an offset from a global pointer and then calls another routine via a pointer.
- `execute_multiple_calls`: Performs a sequence of three indirect calls using offsets from a global pointer.
- `execute_two_calls`: Performs a sequence of two indirect calls using offsets from a global pointer.
- `call_indirect_routine`: Calls another routine using an offset from a global pointer.
- `execute_indirect_function`: Loads a function pointer from a table using an offset and executes it.
- `draw_border_indirect`: Performs two indirect calls using offsets from a global pointer, one of which is a border drawing function.
- `open_workbench_and_layout`: Opens the Workbench screen, stores pointers, and calculates layout values or structure fields.
- `configure_structure_parameters`: Updates a structure or global state with calculated offsets and values based on a check of a global variable, then clears several fields in a pointed-to structure.
- `update_structure_fields`: Calculates several or certain values based on a return or a global and stores them into a structure or a table.
- `update_state_and_library_vector`: Updates several fields in a structure, including a pointer to a library function, and modifies a counter or dimension value.
- `execute_library_sequence`: Retrieves a value from an offset of a register and performs a sequence of four indirect calls using offsets from a global pointer.
- `execute_indirect_functions`: Loads a pointer from a global variable and calls several functions at specific offsets from that pointer or a second global variable.
- `execute_install_sequence`: Performs a sequence of indirect calls and string modifications while checking return codes or status flags from a structure or a global pointer.
- `modify_string_character`: Checks if a string or pointer or or certain return value is valid and then replaces a character at a specific index with a null terminator.
- `normalize_directory_path_name`: Appends a colon or slash separator to a device or directory name when one is missing, creates the directory through helpers, and returns a pointer to the resulting name or an empty string if creation fails.
- `write_install_batch_file`: Writes the install batch file line by line through helpers, comparing each returned value with the expected count and finishing with a closing step.
- `call_library_and_store_address`: Stores a value into a global variable and then calls a function pointer retrieved from a global offset.
- `copy_string_address_registers`: Copies a zero-terminated string from the source address register to the destination address register, including the terminator.
- `count_string_characters`: Counts the characters of a zero-terminated string in the address register, excluding the terminator, and returns the count.
- `call_library2_vector_minus`: Calls the library function at vector offset minus 54 of the second library base stored in the state block, returning nothing.
- `find_string_character_index`: Searches for the index of a specific character within a string and returns -1 if the string terminator is reached first.
- `copy_files_with_info_icons`: Walks the names of a directory listing, appends each name to a path and runs a copy step through helpers.
- `run_install_steps`: Runs a sequence of installation steps through helper calls, repeating thesequence while the helpers report success.
- `increment_progress_counter_then_call`: Increments a counter in a global variable and then calls a helper routine.
- `update_progress_value_and_call`: Increments a second counter when a flag is set, calculates a progress value based on a minimum of one, and calls a helper.
- `call_helper_pass_through`: Calls another routine and returns the data register pair unchanged.
- `pass_through_register_pair`: Calls another routine and returns the data registers unchanged.
- `execute_subroutine_and_return_result`: Calls another routine and returns its 64-bit result.
- `execute_subroutine_and_return_result_2`: calls another routine and returns its 64-bit result.
- `calculate_scaled_dimensions`: Calculates a scaled dimension or window area based on a ratio and certain pointers, then executes several indirect functions via a global pointer.
- `check_abort_request_messages`: Drains pending window messages, and when a close-window message arrives or gadget item 6 is selected, asks the user to confirm breaking the installation and returns a non-null value when the answer is yes.
- `initialize_global_string`: Fills a range of memory with spaces and then iterates through that memory until a null terminator is found.
- `allocate_work_buffer_or_fail`: Allocates a memory buffer through the exec library and returns zero on success, or a pointer to a not-enough-memory message when allocation fails.
- `free_work_buffer`: Releases the work buffer through the exec library when one is held, clears its pointer and returns zero.
- `free_second_work_buffer`: Releases the second work buffer through the exec library when one is held, clears its pointer and related counters, and returns zero.
- `reset_install_counters`: Sets three state variables to zero and returns nothing.
- `call_function_if_not_zero`: Checks if a global variable is non-zero and calls a function at an offset from a global pointer if so.
- `call_function_and_reset_variable`: Checks if a global variable is non-zero, calls a function at an offset from a global pointer, and clears that global variable.
- `execute_conditional_callback`: Checks if a global variable is non-zero, calls a routine located via an offset from a global pointer, and resets that global variable to zero.
- `verify_and_clear_global_variable`: Checks if a global variable is non-zero, calls a function, and if the return value does not match a register, returns the address of a global variable and leaves it otherwise cleared.
- `validate_range_and_set_global`: Checks a global variable, calls a function, and validates a range or value before clearing a global variable and returning a result.
- `set_flag_if_tag_matches`: Checks if a global variable is non-zero, then sets that variable to negative one or zero depending on whether a longword at a specific offset in a structure matches a certain tag.
- `check_install_status`: Checks a global variable and calls a function, potentially clearing a global variable and setting another if a certain condition or return value is met.
- `update_global_pointer`: Updates a global variable with a return value from a function call and conditionally resets a global variable to a constant.
- `parse_newline_terminated_buffer`: Allocates or checks memory, and then walks a text buffer, copying characters to a global buffer until a newline or end-of-buffer is reached.
- `count_newline_characters`: Resets a global counter to zero and then walks a text buffer of a given length, incrementing the counter if a newline character is encountered.
- `execute_nested_functions`: Calls a sequence of functions using offsets from a global pointer and returns a concatenated value containing a global variable and a register.
- `expand_rle_encoded_stream_2`: Decompresses an RLE-encoded or ormulated stream by reading a control byte and either copying a single byte or repeating a byte a certain number of times.
- `copy_data_with_bounds_check_2`: Copies a block of memory from a source pointer to a destination pointer while checking a counter against a limit.
- `copy_decompress_buffer`: An ormulated routine that copies or decompresses data from a source buffer to a destination buffer using a length-prefixed or encoded-length instruction stream, while updating a counter of processed bytes.
- `copy_data_with_bounds_check`: copies a block of memory from a source pointer to a destination pointer while checking a counter against a limit.
- `decompress_rle_data`: This routine implements a decompression or data expansion algorithm, using a control byte to determine the number of bytes to copy or repeat.
- `unpack_and_copy_data`: Decompresses or unpacks a stream of encoded data and writes the resulting bytes to a destination buffer.
- `expand_rle_encoded_stream`: Decompresses a run-length or ormulated encoded stream by copying or repeating bytes baseden on a control byte.
- `save_cpu_registers_to_context`: Stores several CPU registers into consecutive longwords of a structure and clears a global flag.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_and_branch`: Copies a fixed number of words from a source offset to a stack-based pointer and then calls another routine via a pointer.

### type

- `initialize_global_data_tables_2`: Iterates through a linked list or table to find a maximum value and then populates a memory block with an incrementing sequence of words.
- `copy_stack_data`: Copies a fixed number of words from a memory location to the stack and then calls an indirect function.

### Assign

- `initialize_library_structure`: Calculates the maximum size of a linked list of structures and populates a table with a sequence of incrementing or decrementing values.
- `copy_library_config_block_2`: copies a fixed-size block of sixteen bytes from a memory location offset or a global structure into the stack.

### endcli

- `initialize_global_table_and_stack`: Iterates through a linked list or table to find a maximum value, then populates a memory region with an incrementing sequence and copies a block of data from a source pointer to a destination pointer.
- `copy_library_init_block`: Copies a block of sixteen bytes from a global memory location to the stack before executing a function pointer.

### BBStart

- `bootstrap_and_launch_program_2`: Opens the DOS library, retrieves the startup message or default program name, and executes a specified program or script with its arguments before cleaning up library handles and restoring task state.
- `receive_startup_message_args_2`: Finds the current task, waits for its startup message, copies the argument string or the default program name into a buffer, and returns zero or minus one.
- `apply_relocation_table_entries_2`: Verifies a header magic number, then iterates through a byte-encoded table of offsets to apply relative address relocations to a destination pointer.
- `read_file_until_complete_2`: Opens a named file and calls a routine in a loop until it reports completion, returning the final status.
- `free_buffer_and_clear_pointer_2`: Frees a buffer stored in a context slot through a library call if the slot is non-zero, then clears that slot.
- `read_8k_buffer_block`: Checks an active flag, then reads an 8192-byte block from a file handle into a buffer after a 40-byte header.
- `update_tpwm_tag_match_flag_4`: When a context flag is set, stores minus one or zero depending on whether the TPWM tag is found at byte offset 40 of the header structure.
- `clear_byte_at_offset`: Stores a zero byte into a memory location relative to a register.
- `update_decoder_state_on_open_success`: Stores the file handle returned by a file opening routine into a global or context-based integer and, if the handle is non-zero, sets a sequence of three state flags to a specific pattern before returning zero.
- `decompress_bitstream_control_byte`: Processes a compressed data stream by parsing control bytes to determine if a certain number of bytes should be copied or if a certain amount of data should be expanded.
- `decompress_data_stream_5`: Iteratively processes a data stream, checking a carry flag to either copy a single byte or a variable-length sequence of bytes while decrementing a counter.
- `decompress_data_stream_3`: Processes a data stream by reading or skipping bytes and copying or expanding certain sequences based on a control byte.
- `expand_compressed_sequence`: Reads a control byte to determine a copy length or skip length, then copies or skips bytes from a source or destination buffer.
- `decompress_data_stream_6`: Processes a compressed data stream by copying or expanding bytes based on a control byte and a counter.
- `save_registers_and_clear_flag_4`: Clears a byte in the context structure and saves a selection of data registers and address registers into a sequence of longwords within that structure.
- `extract_file_metadata`: Opens a file, locks it, and retrieves a specific value from a structure offset within the file handle.
- `read_from_dos_handle_2`: Calls the DOS Read routine using a stored file handle and a destination buffer.
- `close_file_handle`: Closes a file handle previously passed via the stack or a pointer.
- `open_dos_file_handle_2`: Opens a file using the file name located in the stack frame and returns the resulting handle.
- `record_library_result_to_table`: Increments a global counter and, if the library call return value is non-zero, stores a passed parameter and that return value into a table at an offset calculated from the counter.
- `record_library_result`: Increments a global counter and, if a library call returns a non-zero value, stores a passed parameter and that return value into a table.
- `iterate_descending_table_calls`: Resets a global counter to zero and iterates backwards, calling a library function using a pointer to a table and a descending index.
- `decrement_and_call_if_nonzero`: Decrements a global variable and calls a routine pointer stored in another global variable if the decremented value is not zero.
- `copy_memory_block_longwords`: Copies a block of memory from a source address to a destination address using an optimized approach that transfers large longword-aligned chunks before handling remaining bytes.

### Execute

- `initialize_program_data_structures`: Iterates through a linked list or table to find a maximum value, then populates a memory-resident table with an incrementing sequence of integers.
- `copy_library_config_block_3`: copies a block of sixteen bytes from a global structure offset to the stack, then executes a function pointer or a routine via a register.

### Copy

- `initialize_table_and_copy_data`: iterates through a linked list or table to find a maximum value, populates a memory range with an incrementing sequence, and performs a block copy of data or pointers using a loop-controlled offset.
- `copy_memory_block_8`: copies a fixed number of dwords from a source location or a global structure to the stack.

### BBCopy

- `initialize_program_entry`: Performs library calls to open and close libraries, potentially handling errors or returning a status code.
- `write_null_terminated_string_2`: Iterates through a null-terminated string and passes the resulting pointer to a DOS write routine.
- `copy_file_entries_loop`: Loops over a file list, reading or skipping entries and handling buffers or file handles, and finally releases resources.
- `check_free_memory_threshold_2`: compares the remaining free memory, calculated by subtracting a stored initial amount from the value returned by AvailMem, against a stored threshold.
- `allocate_copy_buffer_2`: Calls a memory allocation routine and checks if the returned pointer is null, returning a pointer to a global error flag if the allocation fails or a null pointer if it succeeds.
- `free_copy_buffer_2`: Checks if a certain flag is set, then calls a routine to free a memory block and clears the flag.
- `free_secondary_buffer_2`: Checks if a secondary buffer exists, frees it using a pointer-based function call, and clears the stored flag/pointer.
- `close_input_file_handle_4`: Checks if a certain flag is set, then closes the input file handle using a function pointer and clears the flag.
- `close_input_file_handle_5`: Checks if a certain flag is set, and if so, calls a function pointer to close an input file handle before clearing that flag.
- `close_input_file_handle_6`: Checks if a certain flag is set, then closes the file handle using a function pointer and clears the flag.
- `update_buffer_offset_and_write`: adjusts a global buffer offset or resets it if it exceeds a certain threshold, then calls a DOS write routine and returns a pointer to a global or zero if the written byte count matches the expected offset/limit.
- `decrement_remaining_bytes_and_read`: Reduces a global byte counter by a fixed amount and uses a library call to read a chunk of data, returning an error if the remaining count or the read result is invalid.
- `search_for_colon_in_string`: Iterates through a string in memory until a colon or a null terminator is encountered.
- `open_copy_destination_file_2`: Opens the destination file through DOS Open and returns a null pointer on success, or a pointer to the error flag if it fails.
- `open_copy_source_file_2`: Looks up the source file index, then opens the source file through DOS Open, returning an error flag when the file is missing or cannot be opened.
- `parse_filelist_line`: Loads a file list into memory and copies the next newline-terminated entry into an output buffer, returning an error if the end of the list or an allocation failure is encountered.
- `count_newline_characters_3`: Iterates through a buffer of a specified length and increments a global counter for every newline character encountered.
- `find_string_index_2`: Searches for a specific character within a null-terminated string and returns its offset or returns negative if a null terminator is reached.
- `create_directory_path_tree_2`: Iteratively creates each directory level of a path by searching for path separators, calling a directory creation routine, and releasing any locks acquired.
- `examine_file_lock_status`: Uses DOS functions to check file status and attempts to lock or unlock a file, returning a status code.
- `parse_copy_command_arguments_2`: Checks the argument count and certain characters, then parses or stores argument fields and returns a flag pointer or an error address.
- `parse_decimal_string`: Scans a string for a sequence of decimal digits, calculating their integer value and returning the result.
- `extract_quoted_string`: Iterates through a string to extract characters between quotes or up to a space, storing the result in a destination buffer.
- `compare_strings_case_insensitive_3`: Compares a string against a global string byte by byte, normalizing or converting lowercase or uppercase ASCII characters to a common case b.
- `compare_strings_case_insensitive`: Compares a string against a global string byte by byte, normalizing or converting lowercase or uppercase ASCII characters to a common case before comparison.
- `copy_string_until_null`: Copies bytes from a source pointer in a register to a destination pointer in another register until a null byte is detected in the source.
- `store_standard_io_handles_2`: Fetches the standard input and output handles via DOS library calls and stores them into separate global variables.

### IconX

- `initialize_program_execution_environment`: This routine initializes a global control structure by retrieving library pointers and setting up task or directory-related parameters.
- `restore_patch_state_2`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer, then conditionally invokes several cleanup callbacks stored in a table.
- `dispatch_cli_or_workbench_start_2`: Chooses between a command-line start and a Workbench start.
- `build_command_string`: Builds a command string from a template and a name, checking for a dot prefix, and executes the command through the DOS library.
- `install_library_functions_2`: Creates and installs a library from a descriptor by setting up a function table and returning the result.
- `scan_ram_entries_for_names_2`: Allocates a buffer and walks a linked list, copying entry names that exceed a certain range into the buffer and performing string replacements.
- `join_strings_into_buffer_2`: Copies one string into a destination buffer and joins a second string onto it, then passes the buffer and the combined length on to a further helper.
- `count_string_length_2`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string_2`: The routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call_2`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal_2`: Parses a signed decimal integer from a null-terminated string, handling an optional leading plus or minus sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot_2`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer_2`: The routine invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, passing the buffer and inverted checksum to another routine if non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry_2`: Calls a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer_2`: The routine stores a byte from a register into the memory location pointed to by a register.

### Disk-Validator

- `format_numeric_string`: Converts an integer value into a character string and stores it in a buffer.

### Ram-Handler

- `execute_indirect_call`: Performs an indirect call to a function pointer located at an offset from a global variable and then calls another indirect pointer.

### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:flt.exe

- `initialize_and_copy_buffer`: Initializes a memory buffer by copying data from a global source and subsequently performs bit-level manipulation and bit-shifting to store bit-packed or concatenated values into a destination buffer.
- `store_shifted_value`: Stores a word-sized value, where the lower byte is taken from a memory location and the upper byte is part of a concatenated result, and returns a bit-shifted version of that value.
- `format_numeric_string_2`: Converts an integer value into a character string and stores it in a buffer.

### Run

- `initialize_global_tables`: Iterates through a linked list or table to find a maximum value, then populates a secondary table with an incrementing sequence.
- `copy_stack_block_and_call`: copies a block of sixteen bytes from a memory location or table into the current stack frame before executing a function pointer.

### BBstart

- `bootstrap_and_launch_program`: Opens the DOS library, fetches the startup message, and launches a named script or program with its arguments through a library call, then restores saved task fields and closes the library.
- `receive_startup_message_args`: Finds the current task, waits for its startup message, copies the argument string or the default program name MAIN.PRG into a buffer, and returns zero or minus one.
- `apply_relocation_table_entries`: Checks a header marker, then walks a byte-encoded table of offsets, adding the load base to each longword it points at and skipping forward for the escape code.
- `read_file_until_complete`: Opens a named file, then calls a subroutine in a loop until it reports completion, returning the final status.
- `free_buffer_and_clear_pointer`: Frees the buffer stored in a context slot through a library call when the slot is non-zero, then clears that slot.
- `read_8k_block_into_buffer`: When the active flag is set, reads one 8192-byte block from the file handle into the buffer after a 40-byte header, reduces the remaining byte count, and returns an error if the read is short.
- `update_tpwm_tag_match_flag`: When a context flag is set, stores minus one or zero depending on whether the TPWM tag is found at byte offset 4of the header structure.
- `clear_context_byte`: Sets a byte at a specific offset from a register to zero.
- `initialise_decoder_state_flags`: Stores the result of an allocation call in the context; when that succeeds it sets three decoder state flags and returns zero, otherwise it returns minus one.
- `decompress_bitstream_lzw_variant`: Processes a compressed data stream by scanning a bitstream or control byte to determine if bytes should be copied directly or expanded from a dictionary or previously seen sequence.
- `decompress_data_stream_7`: Checks a carry flag to determine whether to copy a single byte or a sequence, and updates a counter or writes a byte to a destination.
- `expand_compressed_bitstream`: Decodes a bitstream-based compression format by checking control bits to determine whether to copy a single byte or an expanding sequence of bytes.
- `expand_compressed_sequence_2`: Copies a variable number of bytes from a source or destination or expands a sequence based on a control byte and a counter.
- `process_decompress_stream`: Executes a loop that copies or expands data sequences while checking a counter and updating a pointer or a global variable.
- `save_registers_and_clear_flag`: Clears a flag in the context and copies nine incoming register values into consecutive longwords of the context structure.
- `get_file_size_via_examine`: Allocates a file-info block, locks the named path, examines it, unlocks it and returns the size field from that block, freeing the block afterwards; returns minus one if any step fails.
- `read_from_dos_handle`: Calls the DOS Read routine on the stored file handle, passing the destination buffer and byte count.
- `close_dos_handle`: Calls the DOS Close routine on the handle passed in the stack frame to release a file.
- `open_dos_file_handle`: Calls the DOS Open routine on the file name in the stack frame and returns the resulting handle.
- `store_parameter_to_table`: Increments a global counter and stores a provided parameter and a value from a library call into a table.
- `record_library_result_to_table_2`: Increments a global counter and, if the library call return value is non-zero, stores a passed parameter and that return value into a table.
- `clear_and_iterate_table`: Resets a global counter and iterates backwards to call a library routine using pointers to a table.
- `decrement_and_call_routine`: Decrements a global counter and calls a library routine if the counter is not zero.
- `copy_memory_block_optimized`: Copies a block of memory from a source to a destination using a combination of 48-byte (twelve-longword) or 16-byte (four-longword) or 4-byte (one-longword) transfers if alignment permits, followed by a byte-by-byte copy for any remaining or unaligned bytes.

### Echo

- `initialize_global_table_and_copy_data`: Iterates through a linked list or table to find a maximum value, then populates a global array with a sequence of incrementing values.
- `copy_library_config_block`: copies a block of sixteen bytes from a memory location offset by a base pointer into a stack-allocated buffer before executing a function pointer.

### execute

- `initialize_library_and_setup_table`: Iterates through a linked list or table to find a maximum value, then populates a new table or memory block with an incrementing sequence of or even-numbered integers.
- `copy_library_context_block`: copies a fixed number of words from a source offset within a structure or table to a destination on the stack, then executes a function pointer.

### BBCopy

- `open_library_and_check_status`: Opens a library and checks the returned pointer, recursively calling another routine and potentially closing a library or returning a status code.
- `write_null_terminated_string`: Measures a null-terminated string and writes it to the stored output handle through DOS Write.
- `process_file_list_entries`: Iterates through a list of file entries, performing memory allocation and buffer management, and handles file or directory operations or errors.
- `check_free_memory_threshold`: Compares the free memory reported by exec AvailMem, less a recorded starting amount in kilobyte units, against a stored limit; returns zero when enough memory remains and one otherwise.
- `allocate_copy_buffer`: Allocates the copy buffer through exec AllocMem, returning null on success or a pointer to the error flag, which maps to the 'Not enough memory for buffer' message.
- `free_copy_buffer`: Frees the copy buffer through exec FreeMem when it is set, then clears the stored pointer.
- `free_secondary_buffer`: Frees the secondary buffer through exec FreeMem when it is set, then clears the stored pointer.
- `close_input_file_handle`: Closes the input file handle through DOS Close when it is set, then clears it.
- `close_input_file_handle_2`: Checks if a file handle or flag is non-zero, calls the DOS close function using a pointer stored in a table, and resets the flag to zero.
- `close_input_file_handle_3`: Checks if a file handle-related flag is non-zero, and if so, executes the DOS close function on the stored handle before clearing the flag.
- `write_8k_chunk_to_file`: Writes the next chunk of at most 8192 bytes to the output handle through DOS Write, reduces the remaining byte count, and returns an error pointer when fewer bytes were written than requested.
- `read_8k_chunk_from_file`: Reads up to 8192 bytes from the input handle through DOS Read, reduces the remaining byte count and returns an error when a short read leaves nothing to copy.
- `split_device_name_at_colon`: Scans the argument for a colon separating a device name from its path, terminates the name at the colon and returns zero, or minus one when no colon is present.
- `open_copy_destination_file`: Opens the destination file through DOS Open and returns a null pointer on success, or a pointer to an error flag if the open fails.
- `open_copy_source_file`: Looks up the source file index, then opens the source file through DOS Open, returning an error flag when the file is unable to be opened.
- `read_next_copy_filelist_line`: Loads the file list into allocated memory on first use via exec AllocMem and DOS Open and Read, then copies the next newline-terminated entry into an output buffer, failing at end of list.
- `count_newline_characters_2`: Walks a text buffer of a given length and adds up its newline characters into a global counter.
- `find_string_index`: Searches a null-terminated string for a specific character and returns the number of bytes skipped before that character or the null terminator is found.
- `create_directory_path_tree`: Creates each directory level of a path with DOS CreateDir, unlocking the lock returned by each call, and accepts the 'already exists' error codes as success; any other failure returns minus one.
- `examine_and_lock_files`: Uses DOS Examine and Lock to check or manipulate file status, returning an error or a joined value depending on the success of the library calls.
- `parse_copy_command_arguments`: Checks if a certain number of arguments or a question mark or certain string patterns are provided, then returns a pointer to a global error flag if the arguments are invalid.
- `parse_decimal_number_arg`: Parses decimal digits from the start of an argument string up to a newline, accumulating the value and returning it, or the input value unchanged when no digits arepresent.
- `extract_string_between_quotes`: Iterates through a string to find or copy characters located between double quotes into a destination buffer, terminating with a null byte.
- `compare_strings_case_insensitive_2`: Compares a string against a global string byte by byte, normalizing or converting lowercase or uppercase ASCII characters to a common case.
- `compare_strings_ignore_case`: Compares two strings byte by byte while ignoring case-specific bit differences.
- `copy_bytes_until_null`: The routine copies bytes from a source pointer in A0 to a destination pointer in A1 until a zero byte is encountered in the source.
- `store_standard_io_handles`: Calls DOS Input and Output to fetch the standard input and output handles and stores each one in its own global slot.

### EndIf

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### If

- `initialize_global_data_tables`: Iterates through a linked list or table to find a maximum value, then populates a memory block with a sequence of incrementing or fixed values.
- `copy_stack_memory_block`: copies a fixed number of words from a memory location or table into the stack.

### IconX

- `initialize_program_environment`: Initializes the application state by checking for a command-line or Workbench-based execution, parsing command-line arguments, and setting up task-related pointers and memory structures.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer, then conditionally invokes several cleanup callbacks from a table.
- `dispatch_cli_or_workbench_start`: Checks a parameter to decide between a command-line or Workbench start, then parses strings and calls library functions or supervisor routines.
- `build_wait_and_endcli_command`: Builds a command string from a template and a name, including a wait command and an endcli redirection, then executes it via the DOS library if certain conditions or name prefixes are met.
- `install_library_functions`: Creates and installs a library from a descriptor by making the library structure and setting up its function table, returning a pointer to the new library or zero if no descriptor is provided.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying names that are outside the control range into a buffer, and performs string replacements or joins before passing the buffer to other routines.
- `join_strings_into_buffer`: Copies a source string into a destination buffer and appends a second string to the end of that buffer, then returns the new total length.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: Library code that copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the null terminator of a string and then calls another routine using that pointer and a second argument.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `execute_indirect_function_2`: loads a function pointer from a global table and executes it.
- `execute_function_pointer`: Executes a function pointer retrieved from a global variable.
- `execute_indirect_function_3`: Loads a function pointer from a location relative to a global pointer and executes it.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `execute_indirect_pointer`: Fetches a function pointer from a global table and executes it.
- `call_library_vector_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `execute_library_creation`: Executes a function pointer retrieved from a global variable offset.
- `execute_function_generation`: Performs a call to a function pointer located at a fixed offset from a global variable.
- `execute_resident_initialization`: Invokes a routine via a pointer stored in a global variable.
- `execute_enable_logic`: Calls a routine using a pointer calculated from a global variable.
- `execute_memory_allocation`: Triggers a call to a routine pointer found in a global variable.
- `execute_memory_deallocation`: Executes a routine through a pointer derived from a global variable.
- `execute_entry_allocation`: Calls a routine pointer stored at a specific offset in a global variable.
- `recurse_with_stack_slot`: Calls another routine and passes a pointer to a specific location on the stack as an argument.
- `execute_library_get_function`: Calls a function pointer retrieved from a global variable.
- `execute_library_get_function_alt`: Invokes a routine pointer that is stored in a global variable.
- `execute_library_get_function_ext`: Executes a routine pointer located at a certain offset from a global variable.
- `execute_stored_function`: Loads a function pointer from a global variable and executes it.
- `execute_indirect_function_4`: loads a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_5`: Loads a function pointer from a location relative to a global pointer and executes it.
- `execute_indirect_function_6`: loads a function pointer from a global pointer-to-pointer and executes it.
- `execute_indirect_function_7`: loads a function pointer from a global table and executes it.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `store_byte_to_pointer`: Stores a byte from a register into a memory location pointed to by another register.
- `invoke_table_entry`: Retrieves a function pointer from an offset within a global variable and calls that function.
- `store_register_to_pointer`: copies a byte from a data register into a memory location pointed to by an address register.

### BBAsk

- `initialize_system_state`: Initializes several global variables and performs a sequence of library calls to check or assign pointers, including memory management and library opening/closing.

### Disk-Validator

- `format_string_and_initialize_table`: Converts a numerical value into a string and stores it in a buffer, then initializes a memory-mapped table or array with specific bitmask patterns and loop counters.

### Ram-Handler

- `execute_indirect_call_2`: Loads a function pointer from a global variable and executes it.

### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:bootblock

- `check_library_status`: Calls a function from a library and returns a certain value if the return value is zero, otherwise returns a different constant value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_runtime_memory_tables`: Clears or initializes several large memory blocks with specific constant patterns, populates a bit-manipulated lookup table, and prepares a table of incrementing or fixed memory addresses.
- `initialize_address_table`: Populates a table with a sequence of incrementing or fixed memory addresses and certain constant values.
- `decrement_and_update_address_table`: Decrements a global counter and, if it reaches zero, resets it to a fixed value and increments a secondary global variable.
- `update_address_table_and_config`: Updates a table of pointers with offset-based addresses and decrements a global counter, resetting it and incrementing a secondary variable upon expiration.
- `update_global_table_and_loop`: Decrements a global variable and, if it reaches zero, initializes a table with specific even-spaced values using a loop.
- `update_pointer_and_search`: Decrements a global variable and, if it reaches zero, searches through a table for a specific byte, then updates a pointer and a global variable with a value or an offset-based address.
- `update_transform_parameters`: Updates a set of transformation or projection parameters in a memory block, potentially loading a new value from a table or resetting certain parameters if a global counter reaches zero or a specific character (space) is encountered in a buffer or table.
- `apply_bit_mask_clamping`: Iterates through two buffers and modifies certain bit-fields in the first buffer to match or move toward the bit-fields in the second buffer.
- `calculate_geometry_and_set_params`: Calculates new dimension or offset values based on input registers and updates a block of global variables or a structure with the calculated results.
- `reinitialize_engine_state`: Checks a magic constant in a structure, then updates multiple pointers and offsets relative to that structure.
- `clear_global_buffers`: Clears several global memory locations or buffers to zero and modifies a global variable and a pointer.
- `execute_bytecode_instruction`: Decodes and executes a custom instruction stream by parsing command opcodes and associated operands.
- `update_virtual_machine_state`: Modifies a virtual register or counter or a memory-resident value based on instructions decoded from a custom bytecode stream.
- `process_bytecode_instruction`: Parses and executes a custom instruction stream by updating a program counter and a secondary pointer or counter based on opcode-specific logic.
- `process_instruction_stream`: Iterates through a custom instruction or command table, decoding or decrementing certain counters and updating a pointer-based execution or data-loading state.
- `load_hardware_control_values`: Loads specific constant values into a global variable and certain memory-mapped registers or pointers.
- `set_global_constant`: Sets a global variable to a specific constant value.

### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 1 of 7).adf:intro.dwn#unpacked (raw code file, load address unknown)

- `allocate_resource_memory_blocks_2`: Checks for available memory or resource blocks and, if successful, initializes a structure with zeroed or fixed-value members or or error-handling logic.
- `allocate_and_initialize_memory_block`: Calculates a memory offset and checks if a block of a certain size can be allocated.
- `allocate_resource_memory_blocks`: Checks for available memory or resource blocks and, if successful, initializes a structure with zeroed or fixed-value members.
- `execute_sequential_subroutines`: Executes a sequence of two subroutines and then calls two additional subroutines at fixed memory locations.
- `execute_indirect_calls`: Performs a sequence of indirect function calls using a pointer and hardcoded addresses.
- `execute_indirect_calls_2`: Performs a sequence of indirect function calls using a pointer and a hardcoded address.
- `execute_function_pointer_2`: Loads a function pointer from a memory location and executes it.
- `populate_string_from_function`: Calls a function pointer stored in a global variable and stores the returned value into a global variable, advancing a pointer through a source string until a null terminator is found.
- `populate_string_buffer`: Reads a value from a function call and stores it into a buffer, continuing until a null terminator or a specific value is encountered.
- `populate_string_with_offset`: Calls a function pointer stored in a global variable and stores the result into a memory location relative to the current instruction, advancing a pointer through a source string until a null terminator is found.
- `iterate_and_call_function`: Iterates through a list of integers starting from a global variable, calling a function pointer if the value is non-zero and stopping if a negative value is encountered.
- `iterate_and_call_with_offset`: Iterates through a list of integers starting from a memory location relative to the current instruction, calling a function pointer if the value is non-zero and stopping if a negative value is encountered.
- `set_global_constants`: Sets two global variables to a specific constant value.
- `set_relative_and_global_constants`: Sets a memory location relative to the current instruction and a global variable to a specific constant value.
- `set_pointer_constants`: Sets a memory location pointed to by a register and a relative memory location to a specific constant value.
- `assign_function_return_to_global`: Calls a routine using a constant offset and stores the returned value into a global variable.
- `assign_function_call_to_global`: Calls a routine at a memory location determined by a register offset and stores the returned value into a global variable.
- `initialize_data_table`: Writes a sequence of addresses and a global variable address into a list starting at a pointer held in a register.
- `initialize_table_start`: Stores a routine address and several constants or a global variable pointer into a table starting at a pointer in a register.
- `initialize_table_extended`: Stores a routine address and a long list of constants or a global variable pointer into a table starting at a pointer in a register.
- `initialize_data_table_2`: populates a memory block or table with a sequence of routine or function pointers and constant values using a base pointer.
- `initialize_table_offset_c`: Stores a routine address and several constants or a unable pointer into a table starting at a pointer in a register.
- `populate_function_table`: stores a sequence of routine addresses and a global variable pointer into a table or structure located via a pointer in a register.
- `populate_function_pointer_table`: Stores a sequence of routine or global variable addresses into a table or structure located via a pointer in a register.
- `populate_function_table_2`: Stores a sequence of function pointers and a global variable pointer into a table or structure.
- `initialize_function_pointer_table`: Populates a table or structure with a sequence of function pointers and a global variable pointer.
- `populate_function_table_3`: stores a sequence of routine addresses and a global variable into a table or structure located at an offset from a base pointer.
- `populate_function_pointer_table_2`: stores a routine address and several global variable pointers or constant values into a table or structure referenced by a register.
- `store_initial_vector_element`: Stores a routine address and several data values into a table using a pointer.
- `store_vector_element_subset`: Stores a routine address and several data values into a table, excluding certain early indices.
- `store_vector_element_subset_2`: Stores a routine address and several data values into a table, excluding even earlier indices.
- `initialize_function_table`: stores a routine address and several global variables or constants into a table or structure pointed to by a register.
- `initialize_function_table_2`: stores a routine address and several global variable pointers into a table or structure, and assigns constant integer values to certain offsets.
- `initialize_function_table_3`: Stores a relative or absolute address and a series of pointers to a global variable into a table or structure, followed by a constant value.
- `initialize_function_table_4`: Stores the address of another routine and a global variable into a table or structure, followed by a constant value and a repeated global variable pointer.
- `initialize_function_table_5`: Stores the address of a routine and a global variable into a table or structure, then writes a constant value to a subsequent memory location.
- `populate_function_table_4`: Stores the address of a routine and several pointers to a global variable into a table, followed by a constant value.
- `store_vector_element_subset_3`: Stores a routine or value into a table using a pointer, excluding even earlier indices or data.
- `store_address_and_value`: Stores a routine address and a global variable pointer into a table, followed by storing a constant integer.
- `store_address_and_value_2`: Stores a routine address and a constant integer into a table.
- `store_routine_address`: Stores a routine address into a table.
- `initialize_string_data`: Performs multiple writes to a table or memory area involving a global variable and a routine, then executes a routine called via a pointer from a global variable.
- `modify_routine_address`: Calculates a routine address, modifies it with a bitmask, and stores it into a table, followed by a sequence of writes to a table and calling a routine via a pointer.
- `setup_interrupt_handlers`: Populates several structures or tables with pointers to a routine and a fixed value, then calls a function that manages interrupt or resource allocation.
- `initialize_hardware_resources`: Initializes a resource structure and configures DMA and interrupt enable bits in hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `increment_and_branch_conditional`: Increments a global variable if a pointer does not point to null, then conditionally calls two different routines.
- `increment_global_counter`: Increments a value at a specific memory location and conditionally executes a routine if a certain global variable or pointer is non-zero.
- `execute_conditional_pointers`: Checks a pointer in a table and a separate pointer, executing them if they are not null.
- `execute_two_routines`: Executes a routine and then executes a routine stored at a certain memory location if that pointer is not null.
- `execute_conditional_routine`: Executes a routine stored at a certain memory location if that pointer is not null.
- `update_string_counter_table`: Iterates through a null-terminated string and increments values in a table of counters, resetting certain counter or flag-related memory locations based on bitwise checks or string-length-related conditions.
- `update_string_counters`: increments or resets a counter-based table or array-based string-length tracking logic based on a conditional check of a global variable or pointer-referenced value.
- `update_string_counters_2`: Iterates through a null-terminated string and increments corresponding counters in a table, resetting any counter that exceeds a certain threshold.
- `update_string_length_and_flags`: calculates a bitmask or length-based value and updates several memory-resident flags or byte values.
- `parse_and_increment_string_metadata`: Parses a null-terminated string or a sequence of bytes, updating a counter-based tracking table and performing conditional bitwise checks or byte assignments based on the string content or a global variable.
- `update_input_state_flags`: Checks or clears specific flags in a data structure based on the state of certain joystick buttons and mouse movements.
- `update_string_counters_3`: Iterates through a null-terminated string to increment or reset values in a table of counters.
- `update_bitmask_counters`: Updates a series or a set of counters by incrementing or clearing values based on bitwise checks of a global variable and a counter-reset check if a limit is exceeded.
- `update_string_counter_and_flags`: Updates a series of counters or flags in memory based on certain bitmask checks and then iterates through a string, incrementing a counter or zeroing a value for every non-zero byte encountered.
- `update_string_length_counters`: iterates through a null-terminated string to increment or reset values in a secondary buffer, while performing conditional bitwise checks on a global variable to modify certain memory-resident flags or values.
- `process_input_and_update_state`: Checks the status of certain buttons and mouse movements to update a set of flags and counters in a data structure.
- `rotate_global_bits`: Rotates the bits of a global byte and updates a global variable with the result if certain conditions are met.
- `update_bit_state`: Performs a bitwise rotation or shift on a global byte, updates a memory location, and conditionally modifies bits in a register and other memory locations based on the result.
- `set_bit_or`: Performs a bitwise OR operation between a register value and a global variable, storing the result back to a global memory location.
- `bitwise_or_memory`: Performs a bitwise OR operation between a register and a value stored at a memory location offset by a pointer.
- `set_bit_and`: Performs a bitwise AND operation between a register value and a global variable, storing the result back to a global memory location.
- `bitwise_and_memory`: Performs a bitwise AND operation between a register and a value stored at a memory location offset by a pointer.
- `empty_routine`: the routine is empty and performs no operations before returning.
- `load_conditional_byte`: checks if a byte in a register falls within a specific range and, if so, loads a byte from a table into a memory location using an offset, otherwise stores a certain byte value.
- `load_from_table`: Loads a value from a table or array into a memory location using an offset calculated from two registers.
- `update_audio_bitmask`: Iterates through a loop to update a bitmask in a register and writes specific values to an audio-related hardware register.
- `process_bit_stream`: Iterates through a sequence of instructions to extract bits and updates a bitmask in a global variable based on certain conditions.
- `allocate_and_initialize_resource_struct`: Checks for available memory or resource blocks and, if successful, initializes a structure with zeroed or fixed-value members.
- `store_and_recurse`: Stores stack-based pointers into a global structure, calls another routine, and then conditionally calls it again or recursively.
- `copy_word_data`: Copies a sequence of words or bytes from a memory location pointed to by a register into a global structure.
- `search_word_table`: Iterates through a table of words, comparing each word to a value in a register, and returns if a match is found.
- `copy_nested_blocks`: Uses a nested loop to iterate through a block of memory, decrementing a counter or pointer-based value, and calls another routine during the process.
- `copy_memory_block_5`: Copies a block of memory from a source to a destination, potentially handling or skipping certain bytes based on a condition.
- `expand_bit_stream`: Reads a source buffer and writes a processed or expanded version of the data into a global buffer.
- `update_global_fields`: Passes certain values or memory contents as arguments to another routine that writes to a global structure.
- `initialize_render_buffer_and_table`: Calculates a buffer size based on a masked or aligned pointer and populates a memory-mapped structure with a sequence of incrementing values, a copy of a source table, and a zeroed-out or initialized buffer or pointer list.
- `calculate_rect_bounds`: Calculates and stores width, height, and coordinate offsets into a global structure based on certain conditional checks.
- `capture_return_value_result`: Calls a routine and returns its lower 32-bit value or returns zero if the upper bits of the result are non-zero.
- `store_returned_value`: Checks the upper 32 bits of a return value and stores the lower 32 bits into a global variable if the upper bits are zero.
- `parse_script_command`: Parses a custom script or command buffer, identifying control characters to update internal pointers, string offsets, and table-based configuration values.
- `set_table_values`: Calculates a value based on a pointer and a register, then writes a series of values to a table or a set of memory locations.
- `parse_numeric_string`: Converts a string of numeric characters into an integer value, handling different string lengths.
- `parse_numeric_string_3`: Parses a string of characters, skipping certain whitespace or delimiters, and converts numeric characters into an integer using a lookup table.
- `parse_string_and_update_table`: Iterates through a buffer, checking for certain characters and using a table to look up values that trigger a secondary routine.
- `transform_and_copy_data_buffer`: Modifies a global table using a mask-based transformation and copies the resulting data block to a new destination.
- `copy_memory_block`: Copies a block of bytes from a source address to a destination address, where the amount or offset is determined by a value in a table.
- `apply_transform_mask`: Performs a complex loop that modifies a global table by adding and subtracting values derived from a source pointer.
- `find_string_match`: Compares a value from a pointer-based list against a register and decrements a counter until a match or certain limit is reached.
- `extend_string_buffer`: Appends a null-terminated string and a fixed byte sequence to a buffer starting at a global variable.
- `initialize_string_buffer`: Copies a null-terminated string into a global buffer and appends a fixed sequence of bytes.
- `copy_string_with_initialization`: Initializes a global buffer with a fixed string and copies data from a source pointer until a null terminator or a specific condition is met.
- `return_negative_one`: Returns a value of negative one.
- `initialize_global_parameters`: Copies a byte and a word from fixed offsets relative to the routine's address into global variables and calls another routine.
- `clear_four_word_table`: Clears a table of four-word entries by writing zero to each entry.
- `reset_audio_volumes`: Sets all audio volume registers to zero and clears certain global or RAM-based status flags.
- `disable_dma_channels`: Sets audio volume registers to zero and disables DMA or sets a global flag to a specific value.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `search_string_for_colon`: Searches through a string for a colon character and performs conditional checks or calls other routines based on the result.
- `parse_string_and_execute`: Scans a string for a delimiter and executes a series of functions based on the return values or certain flags or pointers.
- `execute_stored_function_2`: Checks if a global variable contains a non-zero pointer and executes the function pointed to using the current stack pointer as an argument, then clears that global variable.
- `update_memory_range`: Checks a flag, calls a routine to perform an operation on a memory range, and adjusts a global pointer or checks for certain bounds or return values.
- `update_tpwm_tag_match_flag_2`: If a certain flag is set, stores minus one or zero depending on whether the TPWM tag is found at byte offset 4if of the header structure.
- `clear_context_flag`: Sets a byte in a structure offset from a pointer to zero and returns zero.
- `initialize_decoder_state_flags`: Executes a routine and stores its return value in a global variable; if the return value is non-zero, it sets several bytes in a local context to specific values.
- `expand_bitstream_data_2`: Processes a bit-stream or compressed data buffer by checking bit flags and copying a calculated number of bytes from a source pointer to a destination pointer.
- `decompress_data_stream_4`: Implements a decompression or data copying loop that reads a byte, checks a carry flag, and either copies a certain number of bytes or jumps to another routine.
- `decompress_data_stream`: A routine that implements a decompression or data copying loop that reads or writes bytes baseden on certain flags and pointer increments.
- `expand_encoded_data`: A routine that processes a data stream, copying or expanding bytes from a source or destination and updating a counter or pointer.
- `decompress_bitstream_data_3`: Implements a bit-stream decompression or expansion routine that uses a control-byte-driven loop to copy or expand a sequence of bytes.
- `save_registers_and_clear_flag_2`: Clears a byte in the context structure and copies nine registers into consecutive longwords of a structure pointed to by a value in the context.
- `capture_return_value`: Calls a routine and returns its lower 32-bit value, or returns a zero value if the upper bits of the result are non-zero.
- `get_structure_member`: Calls several functions to check status and eventually returns a longword from a specific offset within a structure pointed to by a returned pointer.
- `release_resource`: Calls a library function passing a pointer to the stack.
- `compare_library_return_value`: Calls a library function and returns zero if the return value matches a provided parameter, otherwise returns minus one.
- `call_indirect_function`: loads a function pointer from a global variable and executes it using the current stack pointer as an argument.
- `invoke_function_via_pointer`: Loads a function pointer from a global variable and executes it using the current stack pointer as an argument.
- `call_indirect_function_2`: loads a function pointer from a global memory location and executes it using the current stack pointer as an argument.
- `initialize_system_globals`: Calls several library functions, finds a device name, sets several global variables to a certain value, and modifies a hardware register.
- `write_multiple_globals`: Stores a value from a register into a location relative to the program counter and then writes that same value to five different global variables.
- `write_subset_globals`: Stores a value from a register into a location relative to the program counter and then writes that same value to four different global variables.
- `write_subset_globals_reduced`: Stores a value from a register into a location relative to the program counter and then writes that same value to three different global variables.
- `write_subset_globals_minimal`: Stores a value from a register into a location relative to the program counter and then writes that same value to two different global variables.
- `write_subset_globals_minimal_v2`: Stores a value from a register into a location relative to the program counter and then writes that same value to one global variable.
- `write_subset_globals_single`: Stores a value from a register into a location relative to the program counter and then writes that same unable value to one global variable while accessing a keyboard serial register.
- `wait_for_flag_and_call`: Sets several global or hardware registers, then loops until a specific bit in a global variable or hardware register is cleared before calling two different functions.
- `update_blt_transfer_parameters`: Updates a block transfer or drawing routine by setting a pattern, source, and destination address, and then iterates through a loop to update a linked list of transfers.
- `shift_data_in_loop`: Iterates through a loop to move data from a source table to a destination, while simultaneously overwriting the source data with previously stored values.
- `copy_data_to_table`: Iterates through a loop to copy a sequence of values from a global buffer to a destination table located relative to a global pointer.
- `find_device_name`: Calls a library function to find the name or pointer associated with a specific string, such as a disk device.
- `enumerate_device_list`: Iterates through a list of pointers, checks a specific bit in a structure member, and calls a function to process or find a device name if certain conditions are met.
- `copy_string`: copies a null-terminated string from a source address to a destination address.
- `set_context_parameters`: If a certain value is non-zero, it calls another routine and then writes several values, including a bit-manipulated value, into a structure or context.
- `copy_blt_source_to_destination`: Copies a block of data from a source address to a destination address using a loop of word-sized or long-word-sized transfers.
- `initialize_and_expand_bitset`: Initializes a memory block and performs a block copy-and-expand operation, where bits from a source buffer are or-ed into a destination bitset, effectively spreading or setting bits or flags across a destination range.
- `copy_and_modify_bit`: Copies a block of data from a source pointer to a destination pointer while setting or clearing a specific bit in a byte at a fixed offset.
- `populate_blt_control_list`: Iterates through a linked list of source and destination parameters to calculate and write new block transfer control parameters and destination pointers.
- `draw_pattern_to_screen`: Iterates through a source pattern or sprite-like data, calculating screen-space or even-tiled coordinates and using the bitboard or block transfer orer to draw pixels or blocks to a specific screen area.
- `generate_render_pattern_table`: Calculates and populates a lookup table used for scaling or rotating a bitmap pattern.
- `render_character_bitmaps`: Iterates through a source buffer to check for specific character matches and, upon finding a mismatch, performs bitwise OR/AND operations to modify a bitmask in a destination buffer.
- `modify_bit_stream`: Iterates through a buffer to flip or set or clear a specific bit in each byte based on a bitmask and a counter.
- `update_blt_registers`: Calculates a value based on a range and updates several bitplane-related or blt-related registers or memory locations using a loop.
- `wait_for_flag`: Polls a global variable until a specific bit is cleared.
- `apply_bit_mask`: Modifies a bit in a range of memory addresses by setting or clearing a bit based on a bitmask, or toggling a bit if a certain condition is met.
- `check_bounds_range`: Verifies if two input values fall within a specific range defined by a table.
- `check_upper_bounds`: Verifies that two input values do not exceed certain limits stored in a table.
- `initialize_data_structure`: Calculates several values and writes them, along with bitwise inverted values, into a specific structure or block of memory.
- `update_buffer_range`: Updates a range of values in a buffer by comparing two values and incrementing or decrementing a counter and a pointer if certain conditions are met.
- `assign_global_variable`: Assigns the return value of another routine to a global variable.
- `copy_memory_block_6`: Copies a block of or word-sized data from a source pointer to a destination pointer if the source pointer address is above a certain threshold.
- `setup_dma_transfer`: Initializes a DMA-related global or register with a value and sets a DMA control register to enable certain channels.
- `store_memory_values`: Copies several word-sized values from a source buffer or memory location to a destination buffer using an offset from a global pointer.
- `populate_offset_table`: Fills a table with incrementing values based on a certain stride.
- `calculate_buffer_parameters`: Computes several values for a buffer or transfer, including a count and a size, based on a source value.
- `populate_table_with_offset_values`: Calculates a base value and populates a table with incrementing values using a pointer and a counter.
- `execute_subroutine_5`: Calls another routine with a pointer to a stack location and returns zero.
- `copy_variable_length_data`: Reads a byte and uses its bit state or value to determine how many subsequent bytes to copy from a source pointer to a destination pointer.
- `calculate_blt_clipping_parameters`: Calculates new source and destination coordinates, width, and height for a block transfer, applying clipping or adjustments if the requested area falls outside the valid display or window boundaries.
- `update_sprite_attribute_table`: iterates through a counter or a bitmask to update a table of sprite or object attributes, including position, or a bit-pattern or color, and an or-ed value.
- `update_blt_parameters_2`: Calculates and writes new source, destination, and control parameters to a block transfer structure, then updates a linked list of transfer requests.
- `configure_blt_transfer_loop`: Configures a block transfer or drawing routine by setting a pattern, source, and destination address, then iterates through a loop to update transfer parameters.
- `increment_string_offset`: Increments a value based on a string length and a previously calculated offset.
- `configure_blt_transfer`: Configures a block transfer or drawing routine by calculating source/destination addresses and setting a bitmask-based color or pattern.
- `copy_data_block`: Copies a block of data from a source pointer to a destination buffer and a global pointer.
- `store_word_sequence`: stores a fixed number of words from a source pointer into a destination buffer starting at an offset from a global variable.
- `apply_bit_mask_to_buffer`: Zeros a range of memory and then iteratively updates a buffer by masking and or-ing bits until a certain hardware or register state condition is no longer met.
- `execute_empty_routine`: The routine is empty and performs no operations or data transfers.
- `empty_routine_2`: the routine is empty and performs no operations or data transfers.
- `set_flag_and_recurse`: Sets a global byte to a negative value and calls another routine twice if the byte was zero.
- `clear_flag_and_recurse`: Sets a global byte to zero and calls another routine twice if the byte was non-zero.
- `wait_for_vposr_sync`: Polls a condition comparing a global variable against a value derived from a buffer size calculation.
- `wait_for_vposr_mask`: Polls a global variable until certain bits match a value derived from the video or certain hardware register state.
- `expand_integer_array`: Iterates through an array of integers to split or store their lower and upper halves into a separate destination buffer.
- `swap_memory_blocks`: Swaps a block of longwords between two memory locations and conditionally swaps a single additional longword.
- `update_global_state`: Updates several fields in a global structure with values from a routine call and a table lookup, then enters a loop that writes a value to a global pointer until a certain value becomes zero.
- `write_global_fields`: Writes specific values or results of a routine call into a global structure and then enters a loop that updates a field in that structure until a certain value becomes zero.
- `wait_for_bit_clear`: Polls a global variable until a specific bit is cleared.
- `fetch_masked_global_value`: Waits for a bit in a global variable to clear, then loads a 16-bit value from a table using a masked or offset-based index.
- `store_function_result`: Calls a function via a pointer and, if the return value is non-zero, stores that value and an incremented index-based offset into a table.
- `append_result_to_table`: Calls a function and, if the return value is non-zero, stores the return value and a parameter into a table, while incrementing a global index pointer.
- `copy_memory_block_2`: copies a block of memory from a source address to a destination address using an optimized unrolled loop or word-aligned transfers.

### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 2 of 7).adf:EXTRO.DWN#unpacked (raw code file, load address unknown)

- `allocate_resource_memory_blocks_3`: Allocates and initializes a sequence of memory blocks or resource structures, calculating total required size and setting or clearing certain global members or flags.
- `allocate_memory_blocks`: Sequentially allocates or reallocates multiple blocks of memory, checking for enough available space and updating global pointers and size tracking variables.
- `initialize_resource_memory_layout`: Calculates and allocates a series of contiguous memory blocks or resource structures, updating a global pointer with the new memory bounds and setting or clearing certain structure members.
- `initialize_hardware_resources_2`: Configures hardware DMA and interrupt enable bits and initializes a resource structure.
- `execute_indirect_calls_3`: Performs a sequence of indirect function calls, including a call to an address relative to the current instruction.
- `execute_indirect_calls_limited`: Performs a sequence of indirect function calls, including a call to a hardcoded address.
- `execute_function_pointer_3`: Executes a routine or calls another routine using an offset from a base address.
- `parse_numeric_string_2`: Iterates through a string, calling another routine to get a value and storing that value into a global variable or a table.
- `convert_integer_to_string`: Iterates through a table, calling a routine to retrieve a value and storing it in a global variable until a null terminator or a zero-return is encountered.
- `populate_string_with_function_pointers`: Iterates through a string, calling a library function via a pointer and storing the return value into a memory location relative to the current instruction.
- `process_string_elements`: Iterates through a sequence of integers, calling another routine if an element is non-zero, until a negative value is encountered.
- `iterate_and_call_functions`: Iterates through a list of integers, calling a function via a pointer if the integer is non-zero, and stops if a negative value is encountered.
- `set_fixed_values`: Sets two specific global memory locations to a constant value.
- `set_global_constant_2`: Stores a specific constant value into a memory location relative to the current instruction and into a global variable.
- `set_pointer_constant`: Stores a constant value into a memory location pointed to by a register and also stores that same constant at a relative address.
- `call_with_argument`: Calls another routine passing a constant and stores the return value into a global variable.
- `execute_indirect_call_3`: Calculates a target address by adding an offset from a register to the current instruction pointer, executes the routine at that location, and stores the result in a global variable.
- `initialize_table_pointers`: Populates a series of memory locations or a table with a mix of pointers to a global variable and constant addresses.
- `init_table_with_constants`: Stores a routine address, several constants, and several global variable pointers into a table starting at a pointer in a register.
- `init_extended_table`: Stores a routine address, a long list of constants, and several global variable pointers into a table starting at a pointer in a register.
- `init_data_table`: Populates a memory block with a routine address, several constants, and several global variable pointers using a base pointer.
- `init_table_with_offset`: Stores a routine address, several constants, and several global variable pointers into a table starting at a pointer in aregister.
- `populate_function_table_5`: Stores a routine address, several constants, and several global variable pointers into a table or structure located via a pointer in a register.
- `initialize_function_pointer_table_2`: Populates a dispatch table or structure with a sequence of routine or global variable addresses using a pointer held in a register.
- `populate_dispatch_table`: Populates a dispatch table with a mixture of function pointers and global variable pointers.
- `populate_dispatch_table_2`: populates a table with a sequence of function pointers and a global variable pointer.
- `populate_function_table_6`: stores a routine address, a constant, and several pointers to a global variable into a table.
- `populate_function_pointer_table_3`: stores a routine address and several pointers to a global variable and a constant into a table.
- `initialize_vector_table`: stores a routine address and a sequence of a global variable and constant values into a table using a pointer.
- `initialize_vector_table_entries`: stores a routine address and a sequence of global variable pointers or constant values into a table starting at an offset from a pointer.
- `populate_vector_table_subset`: Stores a routine address and several global variables or constants into a table or vector-like structure starting at an offset from a pointer.
- `populate_function_table_entry`: Stores a routine address and several global variables or constants into a table or structure pointed to by a register.
- `populate_dispatch_table_3`: stores a routine address and several global variable pointers into a table, while also assigning constant integer values to specific offsets.
- `initialize_function_table_tail`: stores a routine address, several pointers to a global variable, and a constant into a table.
- `populate_function_table_entry_2`: Stores the address of another routine and a global variable into a table, followed by repeated copies of that global variable and a constant value.
- `populate_function_table_entry_3`: Stores the address of another routine and a global variable into a table, then writes a constant value to a subsequent memory location.
- `populate_function_table_7`: Stores a routine address and three pointers to a global variable into a table, followed by a constant value.
- `store_vector_element`: Stores a routine address and two pointers to a global variable into a table, followed by a constant value.
- `store_address_and_value_3`: Stores a routine address and one pointer to a global variable into a table, followed by a constant value.
- `store_address_and_constant`: Stores a routine address and a constant value into a table.
- `store_routine_address_2`: stores the address of another routine into a table using a pointer held in a register.
- `execute_sequence_of_calls`: Sets up a sequence of memory structures containing pointers and constants, then executes several routine calls that use a global pointer or a table.
- `modify_routine_address_2`: Loads a word from a table, applies a bitmask, and stores it back to a routine address, followed by multiple writes to a table and calls to indirect pointers.
- `setup_interrupt_vectors`: Initializes several interrupt service routine structures by setting a priority, a mask, and a pointer to a routine.
- `initialize_resource_struct`: Writes a value to a hardware register, then executes several indirect calls using pointers or offsets from a global variable.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `execute_conditional_routines`: Checks if a value at a pointer is non-zero, increments a global variable, and executes two potentially conditional routine calls.
- `increment_global_value`: Increments a word-sized value at a global memory location and conditionally executes routines based on the state of certain pointers.
- `execute_conditional_routines_2`: Checks if a routine pointer from a table and a global pointer are non-zero before executing them.
- `execute_two_routines_2`: Executes a routine and then executes another routine if a certain pointer is not null.
- `execute_conditional_routine_2`: Checks if a routine pointer at a specific offset is not null and, if so, executes it.
- `update_string_counter_table_2`: Iterates through a string-like buffer to increment values in a destination table of counters.
- `update_string_counters_4`: iterates through a null-terminated string and increments corresponding values in a table of counters, resetting any counter that exceeds a predefined limit.
- `increment_string_counters`: Iterates through a null-terminated string and increments or resets values in a table of counters.
- `update_string_metadata`: Calculates the length of a null-terminated string and updates a set of conditional flags and length-based bitmasks in memory.
- `parse_and_increment_string_metadata_2`: Iterates through a null-terminated string or byte sequence, incrementing a counter in a tracking table and resetting or updating associated metadata flags or pointers based on certain bitwise conditions or string length or termination.
- `update_input_flags_and_counters`: Checks the state of certain joystick buttons and mouse movements to update or clear specific flags and counters in a data structure.
- `update_string_frequency_counters`: This routine parses a null-terminated string or a sequence of characters, incrementing or resetting values in a frequency or counter table.
- `reset_and_increment_counters`: Checks specific bits in a global variable to reset or clear certain memory-resident counters or flags, then enters a loop that increments a sequence of counters in a table until a certain threshold or a null terminator is reached.
- `update_string_counter_and_flags_2`: Updates a series of global flags and counters based on bitmask checks, then iterates through a string to increment a corresponding counter or reset a counter if a threshold is exceeded.
- `update_string_length_counters_2`: Iterates through a null-terminated string to increment or reset values in a secondary buffer, while performing conditional bitwise checks on a global variable or a memory-resident flag.
- `update_input_state_flags_2`: Checks the status of mouse and button input via hardware registers to update a set of flags and counters in a data structure.
- `update_global_with_bit_rotation`: Checks a global variable and replaces its value with a bit-rotated version of itself, then recursively calls another routine.
- `rotate_and_update_bits`: Performs a bitwise rotation on a global byte and updates a series of memory locations using bitwise or/and or with a register.
- `apply_bitwise_or_to_global`: Performs a bitwise OR operation between a register and a global variable and stores the result back to a global memory location.
- `apply_bitwise_or_to_memory`: Performs a bitwise OR operation between a register and a value stored at a memory location, then stores the result back to that location.
- `apply_bitwise_and_to_global`: Performs a bitwise AND operation between a register and a global variable and stores the result back to a certain global memory location.
- `apply_bitwise_and_to_memory`: Performs a bitwise AND operation between a register and a value stored at a memory location, then stores the result back to that location.
- `update_bit_state_flags`: Updates specific memory locations by performing bitwise AND or OR operations with values derived from a global variable.
- `load_byte_from_table`: Checks if a byte is within a specific range and, if so, loads a byte from a table into a memory location using an offset; otherwise, stores a default value.
- `load_indexed_word`: loads a word from a table or array into a memory location using a base pointer and an offset.
- `process_bit_stream_and_audio`: Iterates through a sequence of instructions to extract bits and updates a bitmask in a global variable while modifying an audio-related register.
- `update_audio_bitmask_loop`: Calculates an initial bitmask, then enters a loop that extracts bits to update a bitmask and certain memory or audio registers.
- `allocate_and_initialize_memory_buffer`: Calculates a required memory size, checks for available space, and initializes a block of memory with certain offsets and a loop that populates or checks memory or table entries.
- `store_stack_pointers`: Stores current stack pointer values into a global structure and calls another routine based on a check of the return value.
- `store_table_values`: Reads several values from a memory location and writes them into a structure or a table.
- `search_for_value`: Iterates through a table of words and exits if a matching value is found in a register.
- `nested_loop_decrement_pointers`: Performs nested loops that decrement a pointer and a counter-based value until a certain limit is reached.
- `copy_memory_with_conditional_expansion`: Copies a certain number of bytes from one memory location to another, or performs a different copy operation if a certain byte value is negative.
- `unpack_half_word_data`: Reads a source buffer and writes a modified or bit-shifted version of the data to a global buffer.
- `invoke_with_parameters`: Calls another routine using certain values loaded from a memory location offset by a pointer.
- `initialize_buffer_and_address_table`: Calculates a new memory alignment or offset and populates a series of data structures and tables with calculated or copied values.
- `calculate_rect_bounds_2`: Calculates and stores new width, height, and offset values into a global structure based on certain conditional checks.
- `filter_recursive_return`: Recursively calls another routine and checks if the upper 32 bits of the returned 64-bit value are non-zero.
- `update_global_from_return`: Evaluates the upper 32 bits of a return value and, if zero, writes the lower 32 bits to a global variable.
- `parse_script_control_commands`: Parses a string-based command buffer, handling specialized instruction characters like a terminator, a jump or pointer redirect, and an alert or attribute flag.
- `setup_and_wait_for_flag`: Sets several memory locations to specific values and enters a loop that waits for a global variable to reach a certain state via another routine.
- `parse_string_to_int`: Parses a string of digits and converts it into a numeric integer value.
- `parse_string_to_int_2`: Parses a string of digits, using a lookup table to handle certain characters, and converts the parsed sequence into a numeric integer value.
- `parse_string_table_entries`: Iterates through a buffer to find a terminator or a delimiter, and recursively calls another routine if a non-zero value is found in a lookup table or pointer array.
- `tiny_moveq`: Small routine of 1 instruction that loads 20 into d7.
- `empty_routine_4`: This routine is empty and contains only an infinite loop.
- `copy_memory_block_3`: Copies a block of bytes from a source address to a destination address.
- `apply_string_transformation`: Reads from a source buffer to accumulate a sum and then writes a transformed value back to a destination buffer using a table-based lookup.
- `search_string_for_match`: Iterates through a memory range, decrementing a counter and comparing values until a match or a limit is reached.
- `modify_bitmask_and_branch`: Decrements a counter in a buffer and clears a bit in a bitmask, then conditionally calls another routine or jumps to a global variable.
- `load_data_from_relative_address`: Loads a byte and a word from relative offsets or certain memory locations into global variables.
- `clear_four_word_table_2`: Clears a table of certain number of four-word entries by writing zero to each entry.
- `reset_audio_and_globals`: Sets several global variables and audio volume registers to zero or a fixed maximum value.
- `reset_audio_dma`: Sets several audio volume registers and a DMA control register or a global variable to zero or a fixed value.
- `update_audio_registers`: Iterates through a loop to write data from a global buffer into certain audio hardware registers and updates a global counter.
- `calculate_string_offset`: Reads a value from a table and calculates a new offset or value based on certain bits, potentially updating a global variable.
- `process_string_logic`: Scans a string for a colon delimiter and executes a series of conditional checks and calls based on the result.
- `evaluate_string_and_branch`: Identical to the previous routine, this performs string scanning and conditional branching or execution of other routines.
- `find_string_colon_delimiter`: Searches for a colon character in a null-terminated string.
- `process_string_and_call_table`: Identical to routine R4, this routine scans a string for a delimiter and executes functions, potentially updating a global table or checking a structure member.
- `invoke_and_clear_pointer`: Calls a routine via a pointer if the pointer is not zero, then clears that pointer in memory.
- `validate_tag_and_decrement_pointer`: Checks a flag and a tag, then decrements a global pointer, returning a specific constant or a result from another routine.
- `update_tpwm_tag_match_flag_3`: If a certain flag is set, checks if a specific four-byte tag exists at a certain offset within a structure and stores the result as a boolean or zero.
- `clear_context_byte_2`: Sets a byte at a specific offset relative to a pointer to zero and returns zero.
- `initialize_decoder_state_flags_2`: Executes a routine, stores its result, and sets or clears certain flags in a structure based on whether the result is non-zero.
- `expand_bitstream_control_sequence`: Processes a bitstream to extract control flags and copy a variable number of bytes from a source buffer to a destination buffer.
- `expand_bitstream_data`: Processes a bitstream or compressed data stream, copying or writing bytes to a destination buffer based on certain bit flags and counters.
- `decompress_data_stream_2`: Performs a complex bit-stream or compressed data expansion, copying or writing bytes to a destination buffer.
- `decompress_bitstream_data`: Implements a custom bitstream or Huffman-style decompression routine that reads a certain number of bits or bytes and writes the resulting decompressed bytes to a destination buffer.
- `decompress_bitstream_data_4`: Implements a bitstream decompression routine that expands compressed or bit-flagged data into a destination buffer using a counter-based loop.
- `save_registers_and_clear_flag_3`: Clears a flag in the context and copies several data registers and address registers into a structure pointed to by a pointer in a register.
- `filter_recursive_return_value`: This routine recursively calls another routine and checks if the upper 32 bits of the returned 64-bit value are non-zero.
- `get_offset_from_struct`: Calls several library functions to check or manipulate a stack-based structure, and if successful, retrieves a longword from a specific offset in a structure returned by a library call.
- `call_library_function`: Executes a library function using a pointer or a stack-based argument.
- `compare_library_result`: Calls a library function and compares its return value against a provided parameter, returning a failure code if they do not match.
- `execute_library_routine`: Invokes a library routine using a pointer or a stack-based argument.
- `execute_library_routine_stack`: Invokes a library routine passing a pointer to a stack-based structure.
- `call_external_function`: calls a function pointer stored in a global table using the current stack pointer as an argument.
- `setup_device_and_interrupts`: Uses library calls to find a device name and sets several global variables and hardware registers, including disabling or enabling interrupts via a register.
- `update_multiple_globals`: Stores a word from a register into a local memory location and several global variables.
- `update_multiple_globals_subset`: Stores a word from a register into a local memory location and a subset of the global variables.
- `update_multiple_globals_subset_small`: Stores a word from a register into a local memory location and a smaller subset of the global variables.
- `update_partial_globals`: Stores a word from a register into a local memory location and a limited number of global variables.
- `update_fewer_globals`: Stores a word from a register into a local memory location and a single global variable.
- `update_remaining_globals`: Stores a word from a register into a local memory unable to any certain number of global variables and accesses a keyboard register.
- `set_hardware_and_wait`: Sets certain global variables and hardware registers, then waits in a loop until a specific bit in a global variable or register is cleared, before calling two other routines.
- `update_device_parameter_table`: Updates a global table with a sequence of parameter values and checks a stack-based structure for specific command identifiers.
- `rotate_global_buffer`: Iteratively moves values from a source table to a destination table, swapping or shifting values using a global pointer.
- `copy_to_table`: Iteratively copies a sequence of words from a global pointer or table to a destination table.
- `find_device_name_2`: Calls a routine that uses a library function to look up a string name for a specific device.
- `iterate_device_list`: Iterates through a list of device pointers, checking a specific flag in a structure, and calls a routine if certain conditions or device flags are met.
- `copy_string_2`: Copies a null-terminated string from a source pointer to a destination pointer.
- `expand_bit_stream_data`: Iterates through a source buffer and, if a certain bit is set, copies a variable number of bytes from a calculated offset into a destination buffer.
- `store_parameter`: Stores a passed parameter into a global variable.
- `update_position_and_flags`: Updates a global position or a set of status flags based on a calculated offset or button input.
- `update_status_flags_and_copy_bytes`: Updates several global status flags based on the sign of register values and copies two bytes from a structure, potentially calling another routine.
- `set_structure_members_from_globals`: If a certain global variable is non-zero, it executes another routine and writes a combination of global values into a structure or table.
- `update_blt_sprite_attributes`: Updates a block of memory containing sprite or bitplane attribute data by copying a large chunk of data from a source table to a destination buffer.
- `write_split_data_to_ram`: Checks a global flag, then writes a 32-bit value split into two 16-bit halves into a destination buffer in RAM, and clears the flag.
- `update_pointer_and_mask`: Calls a routine and, if the result is non-zero, stores the result into a memory location and updates a hardware-related mask or register or a pointer-based table.
- `update_pointer_and_mask_vposr`: Calls a routine and, if the result is non-zero, stores a 32-bit value split into two 16-bit halves at a certain offset, then modifies a hardware-related mask or register and resets a global value.
- `initialize_pointer_table`: Checks if a pointer is null, allocates or retrieves a pointer, clears the pointed-to memory, and then populates a table with this pointer and a calculated offset.
- `set_pointer_and_fill_data`: Stores a pointer from a routine call into a global, and if valid, clears the pointed-to memory, calls another routine, and then copies a 32-bit value into a series of 8-byte-strided memory locations.
- `decompress_bitstream_data_5`: Iterates through a source buffer to expand a bitstream, copying a variable number of bytes or skipping certain offsets based on certain bit or value checks.
- `set_blt_parameters`: Waits for a hardware register bit to clear, then writes a series of hardcoded or calculated values, including a pointer-based offset, into a destination buffer.
- `update_table_entry`: Increments a global counter and stores two parameters into a table at an offset calculated from that counter, provided a certain condition is met.
- `update_blitter_parameter_table`: Iterates through a linked list of structures to calculate and store new source and destination coordinates, or masks, into a destination table.
- `draw_bitmask_sprite_pattern`: Iterates through a source buffer or pattern, calculating a destination or source offset and using the bitplane-based block transfer or blitter-like logic to write pixel or mask data to a destination buffer, while updating hardware registers to handle block transfers or bitplane-specific masking or drawing.
- `generate_line_drawing_table`: Calculates and populates a table of coordinates used for drawing lines.
- `render_bitmask_font_glyph`: Iterates through a source buffer to perform bitwise masking or setting of bits in a destination buffer, effectively drawing or clearing pixels or pixels-per-pixel or bits-per-pixel, using a scanline-based or tile-based lookup logic.
- `modify_bitmask_buffer`: Iterates through a buffer to flip, set, or clear a specific bit in each byte based on a bitmask or a flag.
- `configure_blt_parameters`: Calculates a range or length, checks a global flag, and then writes various bitmask and size values into a structure, including updates to certain hardware registers.
- `wait_for_flag_clear`: Busy-waits until a specific bit in a global variable is cleared.
- `apply_bit_mask_to_table`: Uses a bit-masking or bit-toggling logic to modify specific bits in a table, where the bit or bit-set/clear operation depends on a certain condition and a value-based bit-shift.
- `validate_range_bounds`: Checks if a provided value or range falls within certain global memory boundaries.
- `verify_range_limits`: Checks if a value or range is within certain global memory limits.
- `set_range_parameters_in_struct`: Calculates a range, calls another routine, and then populates several fields in a structure or global memory block with calculated values, including a bit-inverted value from a table.
- `populate_offset_table_2`: Calculates and writes a sequence of incrementing or offset-based values or addresses into a destination buffer and a table.
- `assign_routine_to_global`: Calls a routine and stores its return value into a global variable.
- `copy_memory_block_7`: Checks if a pointer is within a certain range, calls a routine to get a destination, and then copies a block of memory from a source pointer to that destination.
- `set_dma_config`: Updates a global variable and a DMA-related register with a specific value.
- `write_memory_constants`: Writes several hardcoded or previously calculated values or bit-shifted values into a destination buffer at a fixed offset from a global pointer.
- `populate_address_table`: Fills a table with incrementing or offset-based values or addresses.
- `calculate_buffer_parameters_2`: Performs arithmetic on global variables to calculate buffer sizes and offsets.
- `fill_memory_with_offset_sequence`: Calculates an initial value and then populates a destination buffer with a sequence of incrementing values.
- `execute_empty_call`: Calls another routine with a specific stack-based address and returns zero.
- `decompress_bitstream_data_2`: Decompresses a custom bitstream or run-length encoded buffer by checking carry flags or bit-shifts to determine copy lengths and destination pointer increments.
- `calculate_blitter_bounds`: Calculates and writes new blitter or sprite-related parameters, such as source/destination coordinates and or mask/width/height values, into a structure.
- `update_sprite_attribute_table_2`: iterates through a table to calculate and store modified sprite or object attribute data, including bit-shifted or masked values, into a destination structure.
- `update_blitter_sprite_parameters`: Calculates and writes new blitter or sprite-related parameters, such as source/destination coordinates and or mask/width/height values, into a linked list or buffer structure.
- `update_blt_parameters_loop`: Updates blitter-related registers or a table with values derived from a pointer and a counter, or iterates through a loop to perform similar updates.
- `increment_string_counter`: Iterates through a string and increments a value based on a counter and the string length.
- `update_blt_parameters`: Calculates bitplane-related values and updates registers used for a bitboard or blitter-based copy operation.
- `copy_input_to_buffer`: Copies a sequence of words from a source pointer to a destination buffer and a global variable.
- `copy_to_global_buffer`: Copies a fixed number of words from a source pointer to a destination located at a certain offset from a global variable.
- `iteratively_reduce_nibble_values`: This routine clears a block of memory and then repeatedly iterates through it, masking and decrementing the nibbles of each word until all memory contents reach zero.
- `do_nothing`: the routine is empty and performs no operations.
- `empty_routine_3`: The routine is empty and performs no operations.
- `conditionally_execute_routines`: Checks a global flag and, if it is zero, sets it to a certain value and calls another routine twice.
- `conditionally_reset_flag`: Checks a global flag and, if it is non-zero, sets it to zero and calls another routine twice.
- `wait_for_vposr_sync_2`: Busy-waits until a calculated value matches a bit-masked version of a hardware-related calculation.
- `wait_vposr_mask`: Loops until a specific bitmask or certain bits in the vertical blank or or video pointer register match a value in a global variable.
- `expand_and_copy_table`: Reads values from a source table, calculates an offset, and writes the result as a 32-bit or 16-bit value into a destination buffer while decrementing a counter.
- `swap_memory_blocks_2`: Swaps a range of memory between two pointers and conditionally swaps an additional pair of words if a certain global variable is non-zero.
- `update_data_structure_values`: Stores a value and a zeroed word into a structure offset, then updates a pointer or index in a loop based on a global variable check.
- `store_register_to_structure`: Updates several fields in a structure using values from registers and a passed parameter before looping based on a global variable.
- `wait_for_flag_clear_2`: Checks a global variable and loops until a specific bit is cleared.
- `fetch_table_value`: Waits for a bit flag to clear and then fetches a value from a table using an index derived from the stack.
- `store_call_result`: Calls a function pointer from a table and, if the result is non-zero, stores that result and an incremented index-based offset into a table.
- `store_call_result_2`: Calls a function pointer from a table and, if the result is non-zero, stores that result and an incremented index into a table.
- `copy_memory_block_4`: copies a block of memory from a source address to a destination address using anunaligned-safe or word-aligned or block-unrolled loop.

### Historyline 1914-1918 (1992)(Blue Byte)[cr FLT](Disk 7 of 7).adf:HL.DWN#unpacked (raw code file, load address unknown)

- `calculate_relative_offset_3`: Calculates a relative address using a program counter-relative load or move instruction.
- `execute_small_routine`: A routine consisting of four instructions that calls another routine.
- `calculate_relative_address_2`: This routine performs a single instruction and returns.
- `disable_dma_and_interrupts_2`: A short routine that calls a routine to disable certain DMA channels and certain interrupts via hardware registers.
- `load_relative_address`: Performs a relative load of a memory address.
- `load_relative_address_2`: Executes three instructions to load a relative address.
- `execute_single_instruction_5`: A routine consisting of six instructions that calls another routine.
- `return_from_subroutine`: The routine contains a single instruction and returns control to the caller.
- `load_relative_address_3`: Executes two instructions to load a relative address.
- `open_library_routine`: Executes nineteen instructions and calls a library function to open a library.
- `execute_single_instruction`: A single instruction routine called by another routine.
- `close_library_routine`: Executes twelve instructions and calls a library function to close a library.
- `execute_two_instructions`: Executes two instructions.
- `set_constants`: A routine consisting of six instructions called by a routine that sets a memory location and a global variable.
- `execute_single_instruction_2`: performs a single instruction and returns.
- `update_register_pointer`: loads a new address into a register using a relative offset.
- `load_address_relative`: Loads a relative address into a register.
- `load_relative_address_4`: calculates a target address using a PC-relative load and calls another routine.
- `load_relative_address_5`: calculates a new address using a PC-relative load and jumps to another routine.
- `load_relative_address_6`: calculates a new address using a PC-relative load or add instruction.
- `copy_register_to_memory`: copies a value from a register into a memory location using a relative address.
- `move_immediate_value`: loads a constant value into a register and then calls another routine.
- `transfer_register_to_memory`: copies a value from a register into a memory location.
- `load_address_relative_2`: Loads a relative address into a register.
- `load_target_address`: calculates a relative address and loads it into a register.
- `update_pointer_address`: Loads a new address into a register using a relative offset.
- `load_address_relative_3`: Loads an address or offset into a register using a pc-relative or direct method and calls another routine.
- `get_target_address`: Uses a single instruction to load an address into a register before calling another routine.
- `set_register_pointer`: Loads a value or address into a register and then transfers control to another routine.
- `prepare_subroutine_argument`: Loads an address into a register and calls another routine.
- `set_register_with_address`: Loads a constant or address into a register and calls another routine.
- `load_address_to_register`: Performs an address-loading operation into a register and then calls another routine.
- `set_pointer_and_call`: Loads a new address into a register and calls another routine.
- `load_address_for_call`: Loads an address into a register and jumps to another routine.
- `set_register_with_offset`: Loads an address or offset into a register and calls another routine.
- `initialize_register_address`: Uses a single instruction to load an address into a register and calls another routine.
- `execute_subroutine`: A three-instruction routine that calls another routine.
- `load_relative_address_7`: calculates a new address using a relative load instruction and calls another routine.
- `access_hardware_register`: A two-instruction routine that accesses a custom hardware register, called by a routine that formats numeric strings.
- `execute_subroutine_2`: A two-instruction routine that calls another routine.
- `register_interrupt_handlers`: Adds or sets an interrupt server and updates an interrupt vector using library calls.
- `disable_dma_and_interrupts`: Disables certain DMA channels and certain interrupt disable or enable certain interrupts via hardware registers.
- `return_void`: the routine is empty and performs no operations before returning to the caller.
- `call_subroutine_four`: This routine calls another routine.
- `execute_multiple_instructions`: A routine consisting of five instructions that calls two other single-instruction routines.
- `invoke_subroutine_chain`: This routine executes a sequence of instructions that calls another routine, which then calls a third routine.
- `execute_subroutine_3`: A three-instruction routine that calls another routine, called by a stub and one unnamed routine.
- `execute_subroutine_4`: An eight-instruction routine that calls another routine, called by two unnamed routines.
- `execute_single_instruction_3`: A one-instruction routine called by one unnamed routine.
- `calculate_relative_offset`: calculates a relative offset using a load effective address instruction.
- `execute_null_checked_routine`: Checks if a pointer or a global variable is non-null and, if so, branches to the routine located at that pointer.
- `calculate_relative_offset_2`: calculates a memory address relative to the current program counter and calls another routine.
- `load_relative_address_8`: Loads a relative address into a register using a certain-length instruction.
- `read_input_hardware`: Reads movement from a joystick port and certain hardware buttons via a CIA register.
- `execute_single_instruction_4`: Performs a single instruction routine.
- `update_string_metadata_2`: Updates a counter-based tracking table while parsing a sequence of bytes.
- `modify_input_state_flag`: This two-instruction routine modifies a bit or a byte within a data structure.
- `load_relative_address_9`: A two-instruction routine that loads a relative address into a register.
- `process_input_devices`: Reads status or movement from the mouse port and certain hardware buttons via a CIA register.
- `call_register_update`: A routine that calls another routine.
- `access_keyboard_register`: The routine touches the keyboard serial register and calls or calls other logic-heavy routines.
- `move_register_to_memory`: copies a value from a register into a memory location.
- `execute_nested_logic`: A routine that calls another routine and a larger routine that executes complex logic.
- `load_address_via_relative_offset`: calculates a target address using a relative offset and calls another routine.
- `update_register_state`: A five-instruction routine called by multiple locations that calls another routine.
- `execute_complex_logic`: A large thirty-nine instruction routine called by two other routines.
- `load_relative_address_11`: Loads a relative address and calls another routine.
- `load_relative_address_10`: A five-instruction routine that loads a relative address and calls another routine.
- `execute_mid_length_routine_2`: A routine that calls a longer routine that calls another routine.
- `execute_mid_length_routine`: A twenty-six instruction routine that calls another routine.
- `execute_large_subroutine_2`: A large routine that calls another large routine and two other unnamed routines.
- `do_nothing_pcrel`: This routine contains four instructions and is entered via a PC-relative load, but performs no visible operations or calls.
- `calculate_relative_address`: calculates a new address using a PC-relative load or move instruction.
- `transfer_register_to_memory_2`: copies a value from a register into a memory location.
- `compute_relative_address`: calculates a new address or offset relative to the program counter and passes it to another routine.
- `transfer_register_to_memory_3`: stores a value from a register into a memory location.
- `call_internal_routine`: the routine executes a single internal subroutine call and returns.
- `execute_large_subroutine`: A large routine that calls two other unnamed routines.
