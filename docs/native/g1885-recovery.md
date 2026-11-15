# Game 1885 (fury of the furries (1994)(mindscape)[cr cls][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01885). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4510112 top-level bytes in 586 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 30 of 30 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 1096 of 1096 named (0 author symbols), 1096 of 1096 described (37 by rule, 1045 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1177 of 1177 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 477004 bytes, 110 functions |
| Independent decoder cross-check (Ghidra against capstone) | 41320 of 41320 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 188235 |
| code | 124650 |
| data | 3345971 |
| padding | 83768 |
| unused | 767488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:A | 4 | 160 | 5 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:B | 2 | 36 | 3 |
| Run | 2 | 458 | 12 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:D | 18 | 1892 | 133 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:E | 2 | 114 | 3 |
| Classic | 5 | 252 | 6 |
| disk-validator | 1 | 386 | 1 |
| Classic | 5 | 250 | 17 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:EndCLI | 2 | 182 | 3 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:getkey | 2 | 1297 | 25 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:loader | 1 | 17 | 2 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:wait | 1 | 87 | 2 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:fury | 17 | 24107 | 713 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:bootblock | 1 | 10 | 1 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 289 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 567 | 24 |
| Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:A#unpacked | 2 | 567 | 24 |
| intro.binc#unpacked (raw code file, load address unknown) | 1 | 968 | 10 |
| desert.binc#unpacked (raw code file, load address unknown) | 1 | 963 | 10 |
| MONTAGNE.binc#unpacked (raw code file, load address unknown) | 1 | 963 | 10 |
| LAGON.binc#unpacked (raw code file, load address unknown) | 1 | 968 | 10 |
| FORET.binc#unpacked (raw code file, load address unknown) | 1 | 971 | 10 |
| PYRAMIDE.binc#unpacked (raw code file, load address unknown) | 1 | 963 | 10 |
| USINE.binc#unpacked (raw code file, load address unknown) | 1 | 968 | 10 |
| VILLAGE.binc#unpacked (raw code file, load address unknown) | 1 | 963 | 10 |
| CASTLE.binc#unpacked (raw code file, load address unknown) | 1 | 967 | 10 |
| fin.binc#unpacked (raw code file, load address unknown) | 1 | 968 | 10 |
| TECKNO.binc#unpacked (raw code file, load address unknown) | 1 | 967 | 10 |

## Cases handled

- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:MOD/intro.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (30180 -> 43832 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:MOD/desert.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (29136 -> 41822 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA09.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2184 -> 9909 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA200.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (420 -> 7337 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA02.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1832 -> 9928 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA107.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1000 -> 8598 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA106.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (552 -> 7660 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA10.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2416 -> 9909 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA100.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (740 -> 8349 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA204.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (556 -> 8013 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA07.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2188 -> 9680 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA203.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (616 -> 7824 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA06.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2144 -> 11780 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA104.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (548 -> 8038 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA103.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (512 -> 7668 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA201.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (392 -> 7305 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA04.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2016 -> 11736 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA03.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2244 -> 10583 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA108.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (600 -> 7792 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA101.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (604 -> 8353 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA205.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (556 -> 8013 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA08.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1944 -> 9241 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA01.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2120 -> 11646 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA105.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (628 -> 7739 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA202.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (872 -> 8436 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA05.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2508 -> 11470 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA109.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (740 -> 7873 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA210.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (580 -> 7742 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DAT/DATA102.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (652 -> 8338 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:SPR/SPR00B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (29448 -> 76804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:SPR/SPR01A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (15748 -> 32770 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:SPR/SPR99.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (4296 -> 10498 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:SPR/SPR01B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (17816 -> 35196 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:SPR/SPR00A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (26796 -> 68328 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:DEC/DECOR01.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (43192 -> 54660 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/PRESENT1.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (38720 -> 42076 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/SPRITE3.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (21004 -> 29052 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/FONT.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (6568 -> 9892 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/machine.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (20496 -> 27092 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/TINIES.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (47148 -> 47834 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/LANGUE.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (6084 -> 9532 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/FBI.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (13800 -> 17830 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/freq.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (3204 -> 8578 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/PLANET.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (23100 -> 28946 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/MADEBY.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (20872 -> 24582 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/epileps.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (3752 -> 9434 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/hurle.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (21536 -> 26998 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/SPRITE2.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (14544 -> 28804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/SONS.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (8808 -> 13740 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/fontp.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (676 -> 3438 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:PIC/kalisto.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (18180 -> 23720 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/MONTAGNE.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (30072 -> 41666 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/LAGON.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (30180 -> 43832 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/FORET.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (36000 -> 45894 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/PYRAMIDE.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (33692 -> 45000 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA130.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (724 -> 7707 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA50.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1000 -> 8541 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA37.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2312 -> 10035 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA145.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (720 -> 8512 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA21.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2472 -> 10556 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA111.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (800 -> 9691 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA18.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1712 -> 9790 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA126.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (816 -> 7652 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA46.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1900 -> 9561 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA30.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2748 -> 10958 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA110.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (756 -> 7896 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA17.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1992 -> 10044 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA242.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (560 -> 7744 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA45.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2020 -> 9670 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA135.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (532 -> 7743 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA11.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1544 -> 9651 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA26.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2232 -> 10098 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA134.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (624 -> 7773 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA35.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2200 -> 10125 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA115.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (684 -> 7895 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA143.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (556 -> 7693 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA34.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (1616 -> 9563 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA15.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1836 -> 9824 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA123.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (696 -> 7751 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA43.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2252 -> 9726 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA42.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1556 -> 8798 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA132.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (564 -> 8181 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA39.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2468 -> 10172 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA147.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (688 -> 7721 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA23.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2284 -> 10103 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA131.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (560 -> 7717 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA38.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2560 -> 10425 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA128.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (824 -> 7665 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA32.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (1984 -> 9893 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA112.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (664 -> 7736 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA19.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1680 -> 10030 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA127.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (784 -> 7603 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA47.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2364 -> 9963 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA31.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2104 -> 10000 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA121.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (932 -> 7874 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA28.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2260 -> 11048 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA136.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (588 -> 7699 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA12.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2292 -> 10433 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA40.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2356 -> 10773 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA120.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (852 -> 7770 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA27.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2116 -> 10052 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA117.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (720 -> 7996 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA36.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (1832 -> 9787 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA116.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (532 -> 7739 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA144.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (588 -> 8396 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA20.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1636 -> 9575 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA125.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (596 -> 9148 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA16.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2156 -> 10463 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA124.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1192 -> 9311 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA241.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (804 -> 8593 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA44.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1888 -> 9446 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA25.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2344 -> 10345 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA149.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (556 -> 7704 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA133.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (504 -> 7665 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA24.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2376 -> 10441 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA148.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (592 -> 7685 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA114.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (700 -> 8046 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA142.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (432 -> 7761 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA129.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1232 -> 8052 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA49.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1796 -> 9607 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA33.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (2336 -> 10126 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA113.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (616 -> 8907 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA141.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (604 -> 7755 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA48.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1276 -> 8987 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA14.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2080 -> 10075 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA122.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (932 -> 7900 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA29.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1716 -> 9432 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA137.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (568 -> 7677 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA210.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (580 -> 7742 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA13.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2672 -> 10838 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA41.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2388 -> 9901 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA146.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1052 -> 8815 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dat/DATA22.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2372 -> 10728 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR08A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (20392 -> 43290 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR04B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (22108 -> 53002 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR04A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (16504 -> 37076 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR02B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (12232 -> 34328 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR00B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (29448 -> 76804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR08B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (21496 -> 49340 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR99.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (4296 -> 10498 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR03B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (15156 -> 37962 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR03A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (15332 -> 42922 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR02A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (12740 -> 29270 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:spr/SPR00A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (26796 -> 68328 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dec/DECOR04.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (41216 -> 54264 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dec/DECOR08.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (45032 -> 57162 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dec/DECOR03.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (39828 -> 53150 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:dec/DECOR02.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (49076 -> 55872 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/SPRITE3.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (21004 -> 29052 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/PRESENT5.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (24800 -> 27516 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/PRESENT4.lbmc: ByteKiller Data (xfd)-packed, unpacked with ancient (19436 -> 28106 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/PRESENT2.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (26824 -> 33408 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/PLANET.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (23100 -> 28946 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/SPRITE2.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (14544 -> 28804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:pic/PRESENT3.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (29468 -> 35530 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:MOD/USINE.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (33544 -> 45718 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:MOD/VILLAGE.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (31888 -> 43882 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA65.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2024 -> 9809 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA163.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (428 -> 7330 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA54.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2140 -> 10129 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA162.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (744 -> 8477 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA69.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1744 -> 9526 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA62.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2032 -> 9610 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA151.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (496 -> 7688 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA58.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1668 -> 9612 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA70.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1996 -> 9628 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA67.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1996 -> 9621 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA51.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2264 -> 10543 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA66.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2324 -> 10557 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA60.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1592 -> 9633 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA164.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (640 -> 7742 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA55.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2480 -> 11547 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA64.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1620 -> 9503 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA63.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2000 -> 9544 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA153.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (628 -> 7756 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA152.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (760 -> 8426 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA59.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2024 -> 9992 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA53.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2156 -> 10521 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA161.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (428 -> 7330 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA68.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1260 -> 9156 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA52.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1880 -> 10224 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA61.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1932 -> 9504 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA57.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2324 -> 10636 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA165.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (628 -> 7718 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DAT/DATA56.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1260 -> 9772 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR06A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (19148 -> 45612 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR00B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (29448 -> 76804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR06B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (15520 -> 36214 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR07A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (22636 -> 48670 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR99.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (4296 -> 10498 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR00A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (26796 -> 68328 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:SPR/SPR07B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (14240 -> 35384 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DEC/DECOR06.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (30372 -> 40838 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:DEC/DECOR07.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (37552 -> 47116 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:PIC/PRESENT7.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (25660 -> 30880 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:PIC/SPRITE3.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (21004 -> 29052 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:PIC/PLANET.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (23100 -> 28946 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:PIC/PRESENT6.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (30292 -> 34614 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:PIC/SPRITE2.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (14544 -> 28804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:MOD/CASTLE.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (26596 -> 36644 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:MOD/fin.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (30180 -> 43832 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:MOD/TECKNO.binc: ByteKiller Data (xfd)-packed, unpacked with ancient (29404 -> 44882 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA174.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (644 -> 7787 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA189.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2032 -> 9772 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA271.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (580 -> 7730 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA303.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1428 -> 8188 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA74.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1884 -> 9876 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA182.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (3288 -> 11898 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA73.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2464 -> 10349 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA178.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (624 -> 7735 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA82.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2032 -> 9772 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA190.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2596 -> 10947 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA171.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (512 -> 7705 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA78.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1916 -> 10083 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA186.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2696 -> 12360 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA301.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2036 -> 10038 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA90.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (3284 -> 11898 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA77.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1512 -> 9994 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA195.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (652 -> 7737 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA71.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2268 -> 10207 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA283.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2120 -> 9490 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA194.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (732 -> 8045 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA175.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (648 -> 7791 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA75.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2324 -> 11044 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA183.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2072 -> 10305 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA281.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2124 -> 9490 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA192.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (652 -> 7768 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA179.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (564 -> 7710 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA83.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2120 -> 9490 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA191.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (544 -> 7628 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA188.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2032 -> 9772 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA92.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1168 -> 8181 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA79.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1428 -> 8417 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA187.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2696 -> 12360 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA302.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1124 -> 9353 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA181.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (3288 -> 11898 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA285.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2072 -> 10305 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA88.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2696 -> 12360 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA72.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1268 -> 9664 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA180.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (640 -> 8709 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA177.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (528 -> 7713 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA81.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2072 -> 10305 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA176.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (576 -> 8049 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA80.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (1352 -> 8581 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA274.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (540 -> 7702 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA185.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2696 -> 12360 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA76.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2428 -> 10113 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA282.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2124 -> 9490 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA85.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (420 -> 7400 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA193.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (580 -> 7745 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DAT/DATA84.BINc: ByteKiller Data (xfd)-packed, unpacked with ancient (2596 -> 10947 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR05B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (4672 -> 13302 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR10B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (17760 -> 36158 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR00B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (29448 -> 76804 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR99.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (4296 -> 10498 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR05A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (10912 -> 28554 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR10A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (16400 -> 35782 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR09B.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (19768 -> 46524 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR00A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (26796 -> 68328 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:SPR/SPR09A.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (24544 -> 48842 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DEC/DECOR10.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (39836 -> 52566 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DEC/DECOR05.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (30112 -> 41408 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:DEC/DECOR09.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (43744 -> 51640 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:PIC/PRESENT8.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (28996 -> 34828 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:PIC/SPRITE3.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (21004 -> 29052 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:PIC/PRESENT9.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (29072 -> 33410 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:PIC/PLANET.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (23100 -> 28946 bytes)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:PIC/SPRITE2.LBMc: ByteKiller Data (xfd)-packed, unpacked with ancient (14544 -> 28804 bytes)
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 258 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 493 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 1 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:A: crunched executable decrunched by XFDMaster (CrunchMania Normal (xfd), 104032 -> 191980 bytes), 73 of 74 windows confirmed in reference RAM: recovered as a hunk executable
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 1 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:S/Classic: Imploder (xfd) stream found by ancient scan (4908 -> 7136 bytes) but only 2 of 3 sampled windows are in reference RAM: not used
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:s/Classic: Imploder (xfd) stream found by ancient scan (4908 -> 7136 bytes) but only 2 of 3 sampled windows are in reference RAM: not used
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:MOD/intro.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 2 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:MOD/desert.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/MONTAGNE.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/LAGON.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/FORET.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf:mod/PYRAMIDE.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:MOD/USINE.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf:MOD/VILLAGE.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:MOD/CASTLE.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:MOD/fin.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf:MOD/TECKNO.binc#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 9 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 6 jump target, 3 immediate
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 3 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 3 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 4 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 4 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 5 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 5 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 1 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:A: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- fury of the furries (1994)(mindscape)[cr cls][a2](disk 1 of 5)/Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:D: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- intro.binc#unpacked (raw code file, load address unknown)

## Copy protection found

- `initialize_disk_system` (E): Opens the disk device and library, then attempts to read a sector into a buffer at a fixed memory location to perform a checksum validation. Kind: disk/track signature check. Pass path: the branch following the string comparison (u4__sub_0_0492) which leads to the code_r0x010d53ea block.
- `initialize_graphics_and_library_state` (runtime segment an address): Allocates memory, copies a table of data into it, and performs several bitwise manipulations on structure offsets to set up color or display parameters. Kind: disk hardware/track check. Pass path: The routine proceeds to initialize graphics and library state (setting up pointers and memory structures) after the initial library/system calls, suggesting the check is integrated into the startup sequence..

## Functions


### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:A

- `execute_and_free_memory`: Calls a routine that repeatedly decrements a value until it reaches negative one, calls another routine, calls a third routine, clears a longword in a location pointed to by a2, and calls a routine from a table before returning.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `expand_compressed_data_block`: Reads a header from a data stream to determine a destination buffer size and a series of expansion rules, then executes a loop that calls another routine to fetch data and applies specific masking or repetition rules to write bytes into the destination buffer.
- `select_half_or_input_word`: Returns half of a second word when a flag in d7 equals one, otherwise returns the input word unchanged.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using an index passed in a register, and returns it ANDed with the input word.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:B

- `check_key_and_init`: Inspects a byte from the keyboard serial register and two bits in the joystick/mouse fire register; if they match a specific pattern, it updates the copper list, calls three code pointers stored in a global variable, and uses a library to manage memory or resources.
- `invert_and_increment_data`: Inverts the byte at the address in a5, increments the word at the address in a1, and moves a word from a constant address into a location pointed to by a0.
- `split_longword_to_globals`: Splits a 32-bit value from a register into two adjacent 16-bit global variables, storing the low and high words.

### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Copies the contents of d1 to d0 and then branches to another location.
- `initialize_entity_structure`: Initializes a structure by setting various offsets to zero or specific values, fetches data from tables using indices, and performs a loop to copy a sequence of bytes from one memory location to another.
- `update_entity_attribute_and_copy_byte_2`: Fetches a byte from a table using an index stored at offset 0 of a structure, stores it at offset 20, and increments a value at offset 24 by one.
- `update_entity_attribute_and_copy_byte`: Calculates a new value by adding ten to a longword at offset 12 of a structure in a1, stores this at offset 16, and then copies a byte from a table in a0 to a destination in a0 using an offset derived from the structure's values.
- `update_entity_state_and_call_handler`: Updates several fields in a structure pointed to by a0, including setting a byte at a calculated offset to 10 and copying longwords between offsets 52 and 56 of a1 into a0, then calls another routine and performs a loop that modifies values in a structure based on a comparison.
- `add_offset_to_d1`: Sets d1 to 1, adds the value at an offset of 54 from a1 to d1, and then branches to another location.
- `test_structure_longword`: Tests a longword at offset 48 of a structure in a1 and branches to another routine if the value is non-zero, otherwise it branches to a different location.
- `modify_structure_bitmask_and_branch`: Tests a longword at offset 48 of a structure in a1 against a constant; if the value is zero, it sets the lower 8 bits of that longword to 255 and branches to another routine, otherwise it compares the longword to 133 and branches to another routine.
- `update_entity_state_and_table_2`: Copies a longword from offset 64 of a structure to offset 108 of the same structure, prepares arguments for a function call, and then updates a value at offset 28 of the structure based on the function's return value.
- `update_entity_state_and_table`: Updates a structure in a table by calculating an index from two values, clears several fields in a target structure, and copies a block of data from one structure to another.
- `execute_complex_logic_sequence`: Calls several routines via a pointer, updates various table entries and counters based on values in a structure, and finally performs an indirect jump.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:D

- `initialize_system_and_start_frames`: Opens the exec and graphics libraries, configures audio channels, clears DMA, and arms the vertical-blank interrupt before entering a loop that indexes a table to dispatch per-frame setup routines and processes data records.
- `request_auto_window`: Calls a library routine to request an automatic window with a width of 500 and a height of 60.
- `increment_counters_and_call_handler`: Increments two 16-bit values in global memory, calls a routine, and if certain conditions are met, decrements a longword and two bytes in global memory before calling another routine.
- `tiny_andi`: Small routine of 1 instruction that masks d0 with 0.
- `tiny_ori`: Small routine of 1 instruction that merges 4 into d0.
- `dispatch_list_entry`: Walks a linked list of entries checking for a sentinel and a matching value, then calls the routine stored in the matching entry.
- `forward_register_pair`: Invokes another routine using the values already present in two data registers and returns those two registers concatenated as a single 64-bit result.
- `relay_register_values`: It calls another routine and returns the two data registers it leaves behind.
- `forward_registers`: Calls another routine, then returns the D0 and D1 registers unchanged.
- `forward_register_results`: Calls another routine, then returns the two data registers it left behind.
- `forward_other_routine_result`: It invokes another routine and then returns the two data registers it left behind.
- `load_table_words_and_setup_audio`: Loads two adjacent words from a table indexed by a masked value, sets a flag bit in a byte, and copies several audio and interrupt registers into place.
- `forward_routine_result`: Calls another routine and returns its result unchanged.
- `forward_registers_d0_d1`: It forwards the caller's D0 and D1 register values into another routine and then returns those same values unchanged in D0 and D1.
- `forward_registers_unchanged`: Calls another routine, then returns the D0 and D1 registers unchanged.
- `forward_d0_d1_result`: Calls another routine and forwards its result, returning the two-word value held in the D0 and D1 registers.
- `relay_routine_result`: It invokes another routine and forwards that routine's D0 and D1 register values as its own return value.
- `forward_audio_result`: Calls another routine and returns its result by passing the D0 and D1 registers back to the caller.
- `forward_register_result`: Calls another routine and returns the two data registers it leaves behind.
- `forward_routine_result_2`: It invokes another routine and then returns the two data registers it left behind.
- `pass_registers_through`: Calls another routine and returns its result unchanged, passing the D0 and D1 registers straight through.
- `forward_register_result_2`: Calls another routine and returns the values it placed in the D0 and D1 registers.
- `forward_registers_unchanged_2`: Calls another routine, then returns the D0 and D1 registers unchanged.
- `call_and_return_result`: Invokes another routine and returns its combined result in the two registers.
- `call_and_return_result_2`: Delegates to another routine and passes back its returned register values unchanged.
- `call_and_return_result_3`: Calls another routine and forwards its combined register result to the caller.
- `forward_routine_result_3`: Calls another routine and returns the values it left in the data registers.
- `call_setup_audio_registers_2_with`: Loads several constant values into registers and calls setup_audio_registers_2, then returns the original values of d0 and d1.
- `forward_routine_result_4`: Calls another routine and returns its result unchanged.
- `forward_and_return_registers`: Calls another routine and returns the two data registers it leaves behind.
- `forward_register_pair_2`: Loads several constant values into registers, calls another routine, and returns the values placed in the two lowest registers by that routine.
- `forward_registers_passthrough`: Calls another routine, then returns the D0 and D1 registers unchanged.
- `forward_two_registers`: Invokes another routine and returns its D0 and D1 register results.
- `forward_register_values`: It invokes another routine and then returns the two data registers it received as input unchanged.
- `forward_register_values_2`: It calls another routine and then returns the two data registers it received as input unchanged.
- `increment_and_call_subroutine`: Clears a longword in a global variable, then performs five iterations where it increments that global variable, moves a specific pointer from a table into a global variable, and calls another routine.
- `copy_tagged_records`: Scans a pointer-chained table for records delimited by magic four-character tags, extracts a length from one record, then copies and sign-extends the variable-length byte fields of that record into a destination buffer.
- `resolve_relative_pointers_and_decompress`: Searches a table for a specific four-byte marker, then calculates five absolute addresses by adding offsets to a base pointer found in the table.
- `decompress_rle_stream`: Decodes a run-length compressed byte stream, interpreting repeat counts and literal runs, and writes the expanded data into a fixed-stride destination buffer.
- `invert_and_store_global_pointer`: Bitwise-inverts a byte in a global variable and stores the address of that global variable into another global variable, then returns.
- `call_record_splitter`: Loads a pointer from a global variable into a0 and calls another routine.
- `copy_global_block`: Copies a fixed-size block of four-word entries from one global variable into another, then returns the incoming register values unchanged.
- `initialize_global_structures`: Waits for a flag bit to clear, then initializes a set of global structures by storing pointers, sizes, and sentinel values into their fields.
- `set_interrupt_flags`: Writes a sequence of fixed values into several global variables and sets an interrupt-enable register to arm the interrupt.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `wait_tof_increment_nibbles`: Waits for a vertical-blank timer interrupt and then compares each nibble of a source table against a parallel global table, incrementing the global entries when a source nibble is smaller.
- `increment_on_smaller_nibbles`: Compares each nibble of a source table against a parallel global table and increments the global entries when a source nibble is smaller, processing 32 entries in a loop.
- `wait_tof_decrement_nibbles`: Waits for a vertical-blank interrupt and then decrements each nibble of a global table by one whenever that nibble is nonzero, processing a fixed number of entries.
- `decrement_nibbles_in_place`: Decrements each nibble of a global table by one whenever that nibble is nonzero, processing 32 entries in a loop.
- `store_word_halves_to_records`: Splits each incoming word into its low and high halves and stores them into consecutive fixed-size records, advancing the source by a fixed stride each iteration.
- `iterate_and_clear_pointer_list`: Checks if a pointer in a global variable is null; if not, it iterates through a list of six pointers, incrementing the address by eight bytes each time, and returns.
- `free_memory_via_pointer`: Moves a pointer from a3 to a1, loads a longword from offset 4 of the pointer in a3 into d0, loads a library address into a6, calls a routine at an offset from a6, and then performs a relative branch.
- `allocate_and_read_file`: Allocates memory using a size derived from a global variable, writes the new size to a table, and calls a routine to read data from a file into a newly allocated buffer.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `open_system_libraries`: Opens exec and library entry points through stored function pointers and stores the returned handles into globals for dos, graphics, and intuition.
- `call_twice_indirect`: Invokes the same indirectly-called routine twice in succession through a pointer stored near the start of RAM, then returns.
- `process_word_data`: Processes a series of words by performing bitwise rotations and masking, then stores the results into a destination buffer using a loop.
- `select_half_or_input_word_2`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `rotate_and_mask_word`: Performs a series of bitwise rotations and arithmetic operations on a word in d6 using d1, d3, and d7, then masks the result with a value from a table in a4 and stores it in d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_audio_interrupt_handlers`: Populates a table with two routine addresses and a constant, clears several audio-related global variables by writing zero to them, resets the hardware audio registers to zero, and sets bit 1 of a flag in a hardware register before returning.
- `decrement_audio_flag`: Decrements a byte stored in RAM and, when that byte wraps to zero, clears it and resets a nearby flag, writing an audio channel register as part of the process.
- `setup_audio_registers_2`: Selects a word from a table using a masked index, sets a bit in a byte, and copies audio and interrupt control settings from a data table into the audio hardware and DMA registers before updating a global pointer.
- `configure_interrupt_handler_6`: Writes a byte to a flag register, moves a word from a table into a custom chip register, stores a long value into a global variable, sets a bitmask in a control register, and returns from an interrupt.
- `stop_audio_handler`: Clears a flag and copies values from RAM into global variables when that flag is set, then writes fixed values into a flag byte and an interrupt-enable register.
- `mute_audio_and_disable_dma`: Sets the audio volume registers to zero, sets the audio length registers to one, clears the DMA enable bit in the DMA control register, and stores a pointer from a table into a global variable before returning.
- `setup_interrupt_handler_5`: Sets a pointer in a global variable, performs a loop to clear a block of memory, and sets a flag in a hardware register.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `increment_and_conditional_call`: Increments a byte at a longword location, compares a byte at a nearby location to the value in d0, and if the comparison results in a carry, clears a byte at the longword location, tests a byte at another location, and calls another routine before branching.
- `call_subroutine_and_branch`: Calls a short subroutine and then performs a relative branch to another location.
- `update_audio_channels_2`: Sets the starting address for audio channel 0, audio channel 1, and audio channel 2 by loading the hardware register addresses into a6 and calling a nibble command dispatcher for each.
- `configure_audio_channels`: Loads several pointers from a table, calculates a value by reading a byte from a table and performing shifts and additions, clears a long variable, and then calls another routine three times with different addresses and hardware register pointers before branching to another routine.
- `load_entity_attributes`: Loads a pointer from a table into a structure, extracts an index from the high nibbles of two bytes, and uses that index to copy multiple attributes from a data table into the structure.
- `update_command_state_and_calculate_value`: Updates a structure by adding a value to offsets 4 and 36, copies a word from a table into offset 14, and calculates a percentage-based value using a multiplier and division by 100.
- `process_command_parameter_logic`: Validates a command index against a table, updates a structure with values from a pointer, and then performs conditional arithmetic and data movement based on the low nibble of a byte in the structure.
- `execute_nibble_command`: Reads a byte from offset 3 of a structure in a6, extracts the lower nibble to select a command, and then uses the upper nibble to perform specific arithmetic or bitwise operations on various fields within the structure or global variables.
- `process_command_structure`: Searches a table to validate an index, updates a structure with values from a pointer, and modifies DMA control registers.
- `delay_and_shift_value`: Runs two short delay loops, copies a set of RAM variables, and then repeatedly shifts a value left until it reaches zero.
- `dispatch_nibble_command_2`: Extracts a nibble from a byte at offset 2 of a structure in a6 and branches to different routines based on its value, or copies a word from offset 16 of the structure in a6 to offset 6 of the structure in a5.
- `copy_word_to_offset`: Copies a word from an offset of 16 in a structure pointed to by a6 into an offset of 6 in a structure pointed to by a5.
- `divide_and_branch`: Loads a byte from a PC-relative address, divides it by three, and branches based on the quotient or performs a bitwise shift on a byte from offset 3 of a structure in a6.
- `mask_and_branch`: Loads a byte from offset 3 of the structure in a6, masks the lower 4 bits, and branches to another routine.
- `store_word_from_register_2`: Stores a word from a register into an offset of 6 in a structure pointed to by a5.
- `search_table_for_word`: Calculates an offset into a table using a value from a register, then loops to find a word that is less than or equal to a value in a structure and stores it into another structure.
- `store_word_from_register`: Stores the value of d2 into an offset of 6 in a structure pointed to by a5.
- `update_structure_value`: Checks a global flag, modifies a word in a structure based on a byte from another structure, and stores the result into an offset of 6 in a structure in a5.
- `update_accumulator_and_clamp`: If a global byte is zero, it sets it to 255, adds a masked byte from offset 3 of the structure in a6 to offset 16 of the structure in a6, and if the result is at least 858, clamps offset 16 of the structure in a6 to 858; otherwise, it stores the lower 12 bits of the value at offset 16 of the structure in a6 into offset 6 of the structure in a5.
- `update_node_value`: Searches a table for a value matching a masked word from a structure in a6, then updates offset 24 of that structure with a value from the table and sets a status byte at offset 22 if a comparison fails.
- `clear_word_at_offset`: Clears a word at offset 24 of a structure pointed to by a6.
- `update_status_byte`: Loads a byte from offset 3 of the structure in a6; if it is zero, it returns, otherwise it stores it at offset 23 of the structure in a6 and clears the byte at offset 3 of the structure in a6.
- `update_timer_and_lookup_table_2`: Updates a 16-bit timer by adding or subtracting a value based on a flag, clamps it to a threshold, and then performs a loop to find a value in a table that is less than or equal to the timer.
- `update_timer_and_lookup_table`: Decrements a timer field, clamps it at a lower bound and resets it when exhausted, then derives an index from an input and a structure field.
- `update_nibble_fields`: Checks a byte at offset 3 of a structure, and if non-zero, updates offset 26 of the same structure by merging bits from offset 3 into the high or low nibbles of the existing value.
- `calculate_scaled_value`: Extracts a value from offset 27 of a structure in a6, performs bitwise operations and shifts, and then either returns a scaled value or sets d2 to 255 based on the contents of offset 30 of the same structure.
- `calculate_complement_byte`: Tests a byte at offset 27 of a structure in a6 and, if it is negative, calculates the two's complement of d0 and stores it in d2 before branching.
- `move_byte_and_branch`: Moves the byte from d0 into d2 and branches to another routine.
- `calculate_scaled_offset`: Loads a byte from offset 0 of the structure in a4 into d2, multiplies d2 by the lower 4 bits of the byte at offset 18 of the structure in a6, shifts the result right by 7, and adds this to the word at offset 16 of the structure in a6 if the byte at offset 27 of the structure in a6 is not negative.
- `subtract_and_advance_phase`: Subtracts one input value from another and stores the difference in a global variable.
- `call_and_branch`: Calls another routine and then branches to another routine.
- `branch_to_calculation`: Calls another routine and then branches to another location.
- `update_scaled_offset`: Modifies a byte at offset 28 of a structure in a6 by combining it with nibbles from offset 3, then calculates a new value using offset 29 and offset 30 to update the structure and a destination address in a5.
- `compute_subtraction_result`: Tests a byte at offset 27 of a structure in a6 and, if it is negative, subtracts d0 from a constant and stores the result in d2 before branching.
- `transfer_byte_and_branch`: Moves a byte from a data register into another data register and then performs a short branch into another routine.
- `calculate_scaled_offset_alt`: Loads a byte from offset 28 of the structure in a6 into d2, multiplies d2 by the lower 4 bits of the byte at offset 28 of the structure in a6, shifts the result right by 6, and adds this to the byte at offset 19 of the structure in a6 if the byte at offset 29 of the structure in a6 is not negative.
- `clamp_and_scale_value`: Clamps the difference of two inputs between zero and a fixed maximum, scales it by a global factor divided by one hundred into a word in a structure, and updates a byte by combining a shifted nibble with the existing byte.
- `adjust_offset_and_scale`: If the byte at offset 3 of the structure in a6 is non-zero, it moves that byte to offset 32 of the structure in a6, compares a shifted version of that byte to the word at offset 8 of the structure in a6, and if it is smaller, subtracts the shifted value from the word at offset 8 and adds a scaled version of the value to offset 4 of the structure in a6.
- `set_byte_at_offset`: Writes the value one to a byte at offset 8 of the structure in a6 and returns.
- `decrement_and_scale_byte_2`: If the high nibble of the byte at offset 3 of the structure in a6 is non-zero, it adds that nibble to offset 19 of the structure in a6 and clamps it at 64, otherwise it subtracts the lower nibble from offset 19 of the structure in a6 and clamps it at zero; it then calls another routine to scale the value at offset 19 of the structure in a6 and stores the result in offset 8 of the structure in a5.
- `decrement_and_scale_byte`: Decrements a byte field, clamps it at zero when it would go negative, then scales the result by a global factor divided by one hundred into a word in a structure.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears a global byte and stores a byte into a global address.
- `clamp_byte_and_scale`: Clamps a byte value between zero and a fixed maximum, stores it back into a structure field, then scales it by a global factor divided by one hundred into a word in a structure.
- `calculate_and_store_byte`: Loads a byte from offset 3 of the structure in a6, performs arithmetic involving a multiplication by 10 and an addition of the lower nibble, and if the result is not greater than 63, stores it in a global byte and a second global byte.
- `update_global_pointers`: Copies a byte from offset 3 of a structure in a6 to d0, and if d0 is not zero, writes d0 to a global longword and writes the original byte to another global longword.
- `dispatch_nibble_command_3`: Reads a byte from offset 2 of a structure in a6, extracts its lower nibble, and branches to different routines based on whether that nibble matches specific values.
- `dispatch_nibble_command`: Extracts the high nibble from a byte at offset 3 of a structure in a6 and branches to a specific routine based on whether the nibble matches a value from 1 to 15; otherwise, it returns.
- `update_global_from_ciaa_pra`: Reads a byte from the CIAA_PRA register, masks bit zero, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `mask_low_nibble_to_offset`: The routine stores the low nibble of a byte from an address derived from an uninitialized pointer into a byte at a fixed offset from that pointer.
- `process_conditional_byte`: Checks a global byte; if it is non-zero, it branches to another routine, otherwise it loads a byte from offset 3 of the structure in a6, masks the lower 4 bits, and if that is non-zero, checks a byte at offset 34 of the structure in a6, decrements it, and if the result is non-zero, stores a byte from offset 33 of the structure in a6 into a global byte and a second global byte.
- `store_byte_and_branch`: Stores the byte in d0 into offset 34 of the structure in a6 and branches to another routine.
- `store_nibble_via_pointer`: Writes a byte derived from a word at a fixed address shifted right by four positions into a location computed from a pointer in a6 plus an offset of 33.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `decode_and_copy_title_data`: Reads a nibble from a table entry and, if a global variable and the entry satisfy a modulo condition, copies two 32-bit words and one 16-bit word from the table through a pointer, performs two delay loops, writes a modified value to a global variable, and then copies two more 32-bit words from the table.
- `test_and_mask_byte`: Checks a global byte; if it is non-zero, it branches to another routine, otherwise it loads a byte from offset 3 of the structure in a6, masks the lower 4 bits, and branches to another routine.
- `check_and_mask_byte`: If a global byte is non-zero, it clears d0, loads a byte from offset 3 of the structure in a6 into d0, masks the lower 4 bits, and branches to another routine.
- `clear_byte_on_match`: If the lower 4 bits of the byte at offset 3 of the structure in a6 match a global byte, it clears the byte at offset 19 of the structure in a6 and sets a word in the structure in a5 to zero; otherwise it branches to another routine.
- `branch_on_nibble_match`: Loads a byte from an offset of 3 in a structure in a6, masks the lower nibble, and compares it to a value in a global variable; if they match, it pushes d1 to the stack and branches to another routine, otherwise it returns.
- `increment_nibble_value`: Checks a global variable, and if it is zero, loads a byte from an offset of 3 in a6, masks the lower nibble, checks another global variable, and if that is also zero, increments the masked nibble and stores it back in a global variable at offset 1167.
- `update_nibble_field`: Checks a global variable, and if it is zero, loads a byte from an offset of 3 in a6, masks the lower nibble, shifts it left by 4, masks the result, and performs a bitwise OR with a byte at offset 31 of a6; it then branches if the shifted value is zero.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:E

- `initialize_disk_system`: Opens the disk device and library, then attempts to read a sector into a buffer at a fixed memory location to perform a checksum validation.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `split_longword_to_two_shorts`: Splits a 32-bit value from a register into two 16-bit words, storing the low word at offset 6 and the high word at offset 2 of a destination address pointed to by a global pointer.

### Classic

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `decompress_and_update_data`: Calls two routines to load and decompress data, then enters a loop that calls another routine to add values from a source buffer to a destination buffer at specific offsets, finally calling a third routine if a condition is met.
- `load_address_and_zero_d0`: Loads the address of a value at a PC-relative offset into a1 and sets d0 to 2.
- `expand_compressed_data`: Decompresses a stream of data from a source buffer into a destination buffer using a variable-length encoding scheme involving bit-shifting and repeated additions.
- `push_registers_to_stack`: Clears d2, sets d0 to a constant, and pushes the values of d0, d1, and d2 onto the stack using a4 as a pointer before returning via a branch.

### disk-validator

- `update_disk_validation_data`: Calculates a checksum or validation value by performing arithmetic on values stored in a structure, updates a string buffer with character representations of these values, and executes several subroutines to manipulate bitmasks and data arrays based on the results.

### Classic

- `init_and_check_pattern_2`: Calls several routines and executes a loop three times, then clears a memory location.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `initialize_and_call_function`: Sets d5 to zero, increments d6, and calls a function pointer in a4 twice with modified a1 and d6 values; it then calls another routine, branches if d0 is positive, and if not, increments d5 in a loop until it reaches one, finally checking a value in a global structure and calling a function pointer at an offset from that structure before returning.
- `update_table_values_2`: Doubles d0 and branches if negative; otherwise, it calls another routine, moves d0 to d1, and if d1 is not zero, calls another routine, moves d0 to d6, calls a function pointer in a4, moves a1 to d3, calls another routine, and adds d0 to a value at an offset in a table determined by d2 and d3, looping until d1 reaches zero.
- `branch_and_call_routine`: If a condition is met, it sets a1 to the value in a3, calls another routine, and then performs an unconditional branch.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `copy_bytes_from_source`: If a longword in a6 matches a2, it returns; otherwise, it copies four bytes from a source buffer pointed to by a2 into a destination buffer pointed to by a1, decrements a counter in d0, and loops if the counter is not zero; if the counter reaches zero, it loads a longword from the destination buffer into d0 before returning.
- `expand_variable_length_bitstream`: Reads a control byte and a series of bits from a source buffer to reconstruct an integer value, using a sequence of shifts and conditional additions to handle variable bit lengths.
- `process_bitstream_loop`: Sets a loop counter to 2, adds a byte from d1 to itself, and if not zero, performs a loop that loads a byte from a buffer in a0, adds it to d1 with carry, adds d0 to d1 with carry, and decrements the counter.
- `set_loop_parameters_five`: Loads 5 into d0, 3 into d7, and branches to another routine.
- `set_loop_parameters_four`: Loads 4 into d0, 2 into d7, and branches to another routine.
- `set_loop_parameters_three`: Loads 3 into d0, 1 into d7, and branches to another routine.
- `expand_bitstream_variable_length_integer`: Reads a variable-length bitstream from a buffer in a0, using bitwise shifts and carry-based additions to reconstruct integer values.
- `update_byte_with_carry`: Adds a byte from d1 to itself and branches if the result is non-zero, otherwise it loads a byte from a buffer into d1, adds d1 to itself with carry, adds d2 to itself with carry, and branches to another location.
- `expand_bitstream_data`: Reads a bitstream from a source buffer to reconstruct values through bitwise shifts and expansions, using a control byte to determine bit counts and a table to look up expansion parameters.
- `decompress_bitstream_data`: Reads a control byte from a source buffer to determine the number of bits to process, then performs bitwise expansion and shifts to reconstruct data values.
- `push_registers_to_stack_2`: Pushes a byte from d0, a byte from d1, and a word from d2 onto the stack using a4 as a pointer, then restores d0 through d3, d5 through d6, and a3 through a4 from the stack before branching.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:EndCLI

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then if a condition is met, writes a header and fills a buffer with sequential words.
- `move_d1_to_d0`: Copies the value from d1 to d0 and branches to another routine.
- `execute_nested_calls`: Performs a series of indirect calls using a pointer from a structure, updates a value in a table based on a calculation, and decrements a value at an offset in a table before jumping to an indirect address.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:getkey

- `process_input_buffer`: Initializes a local structure, clears a memory range, and calls a routine to fetch input data.
- `process_key_message`: Stores a passed parameter at a fixed offset and then calls a routine through a pointer when it is non-null.
- `dispatch_key_handler`: Compares a value at offset 420 of a structure in a4 with a value in d0, and if the comparison is not less than or equal, calls a routine using a pointer from a4 plus an offset of 492.
- `return_immediately`: The routine executes a return instruction immediately upon entry, performing no other operations despite the instruction count suggesting additional code may be present or filtered.
- `call_external_function_conditionally`: Decrements the stack pointer by 4, checks if the resulting address is zero, and if not, calls a function via a pointer stored at a fixed negative offset from the stack pointer before returning.
- `free_linked_list`: It walks a linked list of pointers, clearing the head entry and invoking a routine at each node.
- `handle_key_allocation`: Checks a bit in a byte accessed through a register and, when it is clear, calls a routine to allocate memory.
- `read_and_store_key`: Reads a keyboard key and stores the resulting code in a global buffer.
- `load_stack_values`: Tests bit 0 of d6, then loads three values from the stack into d0, a0, and d1.
- `read_and_process_key`: Reads keystrokes from the keyboard and returns the processed key code, or a negative value when no usable input is available.
- `handle_key_messages`: Processes key messages by iterating through a linked list, calling a routine to handle input, and managing a buffer of characters.
- `process_key_messages`: Walks a linked list of message entries, and for each entry invokes a callback routine through a stored pointer depending on whether the entry's flag byte indicates a special or normal case.
- `tokenize_input_buffer`: Splits a text buffer into individual tokens, storing each token pointer in a table and decoding escape sequences like newline and special characters.
- `copy_and_process_key`: Copies a bounded run of bytes from a global buffer into a stack array, null-terminates it, then queries a task and library state to decide whether to trigger a callback and store the resulting string pointer.
- `get_list_element`: It checks whether a passed index falls within a range and, if so, walks a linked list decrementing the index until reaching the target node.
- `process_key_input`: Calls a routine via a pointer at a global address, checks a bitmask in d0, and if a condition is met, copies a string from a global buffer to a stack array and calls another routine via a pointer in a4.
- `find_byte_in_buffer`: The routine searches for a specific byte value within a memory buffer of a given length.
- `append_string_to_buffer`: The routine appends a source string to the end of a destination string.
- `allocate_memory_block`: Attempts to find a suitable free memory block in a linked list by checking size and a type field, splitting the block if it is larger than requested.
- `process_message_list`: Iterates through a linked list of messages at offset 308 of a4, calling a routine pointer for each entry, and then processes a second linked list at offset 44 of a4 by calling specific routines based on bitwise checks.
- `read_string_from_file`: Reads bytes from a file buffer into a destination buffer until a null terminator or a specific character is encountered.
- `process_file_key_input`: Manages a per-file state record identified by a pointer.
- `update_file_node_state`: Iterates a linked list to aggregate a status flag, then resets or updates a node's counters and status bytes based on helper return values and returns a status code.
- `map_scancode_to_key`: Takes a value in register d7 and maps it to a new value in d0 by performing range checks and looking up bytes in three different tables relative to a4.
- `call_dispatch_key_handler`: Pushes the contents of registers a0 and d0 onto the stack, calls another routine, and then returns.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:loader

- `initialize_library_and_process`: Calls four routines from a table, performs a loop that clears a range of memory, copies data from one location to another, and sets hardware registers.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:wait

- `setup_hardware_and_wait`: Calls several routines, clears a range of memory, copies data to a destination, and writes values to various hardware registers including bitplane and color registers.
- `tiny_not_addq`: Small routine of 2 instructions that inverts (a5), then adds 1 to (a2).

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:fury

- `branch_to_offset`: Performs a short branch to a location within the same routine.
- `increment_pointer_and_return`: Increments a byte at a location pointed to by a long word in a data structure, then returns if the carry flag is not set.
- `set_color_and_loop`: Stores a longword from a pointer at an offset of 2 in the stack into a global variable and writes the value 3840 to the color register, then enters an infinite loop.
- `execute_main_game_loop`: Polls hardware registers to check for specific bit patterns and enters a loop that repeatedly calls a large sequence of subroutines and functions.
- `initialize_memory_blocks`: Fills eight consecutive longwords starting at a memory location pointed to by a1 with zero, writes zero to a word at offset 1074 of a structure in d3, and writes the value 1024 to a word at offset 1078 of that same structure.
- `reset_assets_and_loop_handlers`: Clears two global variables, calls two routines, and sets four registers to specific values before calling another routine.
- `update_display_and_sync_state`: Checks if a global variable is less than 81 and a multiple of 10, then sets color register 0 and disables DMA. It performs conditional logic to swap a pointer and a flag, calls several routines to update memory, and writes a value to a table based on an offset derived from a pointer in a table.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_color_and_dma`: Writes a word to a hardware register and sets a word to zero in another hardware register.
- `split_byte_bits`: It reads a byte from a global variable, then splits each of its bits into separate global variables.
- `init_video_and_wait`: Initializes custom video registers and global state, then spins waiting for a vertical position flag before tuning a frequency value and finishing setup.
- `process_media_and_text`: Initializes globals, opens and reads a data file, replaces bracketed markers with characters, and blits media frames while polling a status register.
- `resolve_input_code`: Scans a block of one-byte flags and, for the first flag set to one, stores a corresponding digit or letter code into a global variable.
- `call_load_and_decompress_data`: Calls load_and_decompress_data and then continues execution into the subsequent routine.
- `call_reset_and_swap_record_table`: Calls reset_and_swap_record_table and then continues execution into the next routine.
- `load_demo_assets_and_reconfigure_display`: Checks a global variable for a specific value, then calls several routines to initialize memory and load assets from demo files.
- `copy_word_block`: Calculates a source address by adding a value from a table to a base address, then copies a fixed number of 32-bit words from the source to a destination table, advancing both pointers by a fixed word count at each step.
- `copy_buffer_records`: Iterates through a table of 10-byte records, checking a state flag at offset 9.
- `copy_table_window`: Checks two 16-bit values in a table; if they are both zero, it returns, otherwise it may increment a value or copy a block of words from one table to another while maintaining a sliding window of five consecutive values and then copying a block of longwords using a calculated count.
- `copy_diagonal_interpolated`: Copies a block of bytes from a source table to a destination table along a diagonal path by calculating the slope between two points.
- `process_and_copy_records`: Runs a multi-step setup sequence that waits for display-sync and video-blank counters, then decodes packed demo data and copies the results into a record table.
- `count_surrounding_collisions`: Clears three counters in global memory and then iterates through the eight tiles surrounding a central coordinate by incrementing and decrementing two offset variables.
- `update_joystick_status_flags`: Calls a routine to process joystick input, then updates a series of boolean flags in a table based on the resulting data, including toggling a specific flag and handling bitwise transformations of the input status.
- `update_position_bounds`: Stores a pair of coordinates into a table at an index derived from a global value, then compares two coordinate pairs and, when a distance condition holds, advances the position and records new coordinates in the table.
- `clamp_coordinate_to_bounds`: Clamps a pair of coordinates into a computed bounding box, adjusting the stored limits when the position falls outside the playable range, then restores a saved value and writes it into two global variables.
- `move_word_to_global`: Moves a 16-bit word from a global variable into register d1.
- `calculate_collision_and_update_position`: Calculates a 2D coordinate based on a division and bitwise operations, then checks if this coordinate falls within a specific range defined by a table lookup.
- `store_register_and_clear_globals`: Stores a 16-bit value from a register into a global variable and writes zero into a second global variable before returning.
- `process_state_machine`: Checks if a global word equals 5, then enters a loop that repeatedly calls another routine while a second global word is 1, or while it is not 1, depending on its initial value.
- `check_flags_and_clamp`: Checks a set of global flags and, when they match, sets a flag, clamps two other values to three, and invokes another routine when a third flag is nonzero.
- `copy_byte_blocks`: Copies a block of bytes from one memory table to another when the source block falls within a small indexed range, processing several such blocks in a loop.
- `process_scaled_table_values`: Skips out early unless a global flag holds an allowed value, then walks a table of records computing scaled values and calling another routine when the results fall within a computed range.
- `advance_interpolation_step`: Interpolates two values from a table using a weight and stores the results, then advances a counter that, when it reaches thirty-two, records several derived values and flags into a structure.
- `accumulate_set_bits_2`: Indexes a global table using a shifted register and a scaled index, tests a single bit extracted from a register against the fetched value, and updates several global variables based on the result.
- `accumulate_set_bits`: Indexes a global table using a shifted register and a scaled index, tests a single bit extracted from a register against the fetched value, and accumulates a result by adding fixed offsets for each matching bit before returning it.
- `advance_spawn_timer`: Decrements a 16-bit timer in a global variable and, if it reaches zero, searches a table of five-field records for an active and unlocked entry.
- `invoke_routine_for_flagged_records`: Walks a table of records, invoking another routine for each entry whose first field equals one or two and stopping when the pointer returns to the start.
- `copy_active_entries`: Iterates through a circular table of one-byte state flags, copying four bytes from a source block to a destination block for every entry marked with a value of 2.
- `process_grid_movement`: Iterates through a circular table of records, adjusting two coordinate values based on each record's direction code, then clamps the result against a limit and checks a bitmask at a computed offset.
- `decrement_and_invoke_globals`: If a global word is not greater than 0, it sets it to 40 and decrements it by 1; if a second global word is non-zero, it decrements that word, calculates an offset into a table using several global values, clamps two values from that table, and calls another routine.
- `update_entity_attribute_table`: Checks several global flags and a bit in a global variable, then iterates through a table of records to mark them and update a counter.
- `find_empty_slot_and_clamp`: Searches a table of records for an empty slot, clamps two incoming values against computed bounds, stores them, marks the slot used, and calls another routine.
- `increment_conditional_table_entries`: Iterates through a table of structures, checking if a 16-bit value at offset 4 is non-zero; if so, it increments a 16-bit value at offset 6 within that structure.
- `copy_data_block_loop`: Iterates through a list of structures, using a value from each structure to calculate an offset into a source table and then performs a series of unrolled 32-bit memory copies from that source to a destination buffer.
- `setup_blt_area_transfer`: Iterates through a table of structures to configure a Blitter area fill operation.
- `copy_linked_list_elements`: Iterates through a table of structures, and for each entry where a specific 16-bit field is non-zero, it uses values from the structure to calculate pointers and performs a series of repeated longword copies between two memory areas.
- `configure_blt_graphics_parameters`: Iterates through a table of structures to find an entry with a specific value, then configures the Blitter hardware registers with source, destination, and size parameters, including specific color and mode settings derived from the table entry.
- `copy_linked_list_elements_2`: Iterates through a list of structures, using values from each structure to calculate source and destination addresses for a series of longword copies.
- `update_blt_registers`: Checks a table of entries, and if an entry is valid, calculates several values to update the hardware Blitter and Copper registers.
- `handle_marked_record_and_store`: Walks a table of records and, upon finding one marked with a negative sentinel, calls another routine and stores a set of computed values into a destination buffer.
- `process_record_bounds`: Iterates through a table of records, updating a running position value by adding an offset stored in the record.
- `merge_string_bits`: Reads bytes from a source buffer, merges their bits into a destination buffer while tracking a run length, and finally resets several fields of a structure to fixed values.
- `find_longest_string_match`: Compares two byte strings to find the longest matching run, tracking the best match length and position in a global variable.
- `increment_counters_and_call_bit_shifter`: Checks if a value in a register is zero, otherwise increments one of two global 16-bit counters based on whether the value is less than nine, and then calls a routine to perform a bitwise shift and merge operation using a calculated loop count and a modified version of the input value.
- `shift_and_merge_bits`: Performs a bitwise shift and merge operation using a loop that decrements a counter and rotates bits between registers.
- `shift_merge_words`: The routine merges two 32-bit values by repeatedly taking the least significant bit of one word and the most significant bit of another, shifting both words, and writing the intermediate result through a pointer that is advanced each iteration.
- `decode_huffman_stream_2`: This routine calls another routine and then calls decode_huffman_stream to decode a Huffman-style bit stream, emitting literal bytes and back-references that copy previously decoded data into a growing output buffer.
- `check_table_range_flag`: Checks a flag and, if set, scans a repeating table of four-word entries to see whether a pair of coordinate fields fall outside a given range, setting the flag to a nonzero value when an out-of-range entry is found.
- `configure_blt_hardware`: Scans a table for specific byte values and, when a match is found, calculates and writes new values to the Blitter and Copper registers.
- `update_structure_data_via_pointer_loop`: Iterates through a list of structures, checking a status byte at offset 8; if the byte is 1, it performs a series of longword copies between two memory areas using an index derived from offset 10 and offset 12, then updates the status byte and offset 10 before continuing the loop.
- `mark_and_store_table_records`: Returns early when a global value is too small, calls another routine, then walks a table marking records and storing coordinate and offset values.
- `search_table_by_fields_2`: Walks a table of records by a fixed stride and returns whether it finds one that satisfies a set of field comparisons, stopping early when it finds a match or reaches the end.
- `update_attribute_table`: Increments a global counter and iterates through a table of records, calculating new values for two fields based on offsets in another table and writing them to a destination, while also updating two byte fields based on a bitmask derived from the loop counter.
- `update_sprite_attribute_tables`: Iterates through a list of structures to perform bulk memory copies of sprite data using varying word and longword transfer widths based on a calculated stride.
- `check_and_set_sentinel`: Checks if a word at offset 8 of a structure pointed to by a3 is negative one; if it is, it branches to another routine, otherwise it sets that word to zero and branches to another routine.
- `store_values_into_free_slot`: Scans a table of four-byte records until it finds one whose second field holds the sentinel value of negative one, then overwrites that record's fields with the two values passed in a pair of adjacent memory locations.
- `process_table_records`: Walks a circular table of records, and for each record whose control field is zero or one it loads a value from the record, calls another routine to process it, and marks the record as handled.
- `copy_values_between_tables`: Copies each two-byte value from one table into the second field of a second table whose entries are eight bytes apart, wrapping back to the start when it reaches the end.
- `rebuild_table_records`: Clears a table of six-byte records, then walks a second table copying each record's fields into a third table while running nested loops that adjust the copied values based on stored bounds.
- `process_record_countdowns`: Iterates over a table of six-byte records, skipping the last one, and for each occupied entry adjusts a countdown value while recursively processing related slots.
- `check_bitmask_matches_expected`: Resolves a bit index from a global value, masks four positions in a byte array using that bit, combines which positions are set into a four-bit result, and reports whether it matches the stored expected value.
- `check_masked_pattern`: Applies a single mask byte to four consecutive bytes spaced by a stride through a byte pointer, accumulating the set bits into a return value and incrementing a global counter when a match pattern is found.
- `check_masked_pattern_2`: Iterates through a sequence of bytes using a stride, checking if specific bits are set within each byte using a rotating mask.
- `apply_mask_and_call`: Applies a single mask byte to four consecutive bytes spaced by a stride through a byte pointer, and when a match pattern is found it invokes a routine from a jump table and returns the last masked byte.
- `apply_mask_and_call_2`: Applies a single mask byte to four consecutive bytes spaced by a stride through a byte pointer, and when a match pattern is found it invokes a routine from a jump table, returning the last masked byte.
- `apply_mask_and_call_3`: Iterates through a sequence of bytes using a stride, applying a rotating bit mask to check for a specific bit pattern across four spaced bytes.
- `apply_mask_and_call_4`: Iterates through a sequence of bytes using a stride, testing four spaced bytes against a rotating bit mask; if the mask matches a specific pattern, it calls a routine from a jump table and returns the last masked byte.
- `track_and_advance_entity`: Detects an entity positioned on a grid cell and, when it moves within range, advances its stored coordinates and updates the corresponding sprite position fields.
- `compare_coordinate_bounds`: Compares the coordinates read from two four-byte structures against a pair of global coordinates, applying a small offset to the upper bounds, and returns one when all four comparisons hold or zero otherwise.
- `find_enclosing_record`: When a global flag is set, walks a table of four-byte records looking for one whose coordinate range encloses a pair of global coordinates, returning one on a match or zero when none does.
- `increment_surrounding_collision_counts`: Iterates through the eight tiles surrounding a central coordinate by applying various offsets to the x and y values, calls a collision check routine for each, and increments a collision counter at offset 72 or 74 of the structure in a0 if a collision is detected.
- `check_entity_collision_and_update`: Compares bounding boxes of entities against a global table or specific coordinate ranges and, upon detecting an overlap or meeting a condition, copies a value from the source entity into a field at offset 102 of the target structure.
- `adjust_slot_index`: Decrements a byte counter at offset 82 of a structure and, when it reaches a threshold, adjusts a slot index by scanning a table of sentinel values.
- `update_collision_bounds`: Calculates a collision distance based on a value in a structure and iterates through a loop to adjust two 16-bit values at offsets 72 and 74 of a structure in a0.
- `classify_integer_ratio`: Normalizes two 16-bit global integers by taking their absolute values and then performs a series of ratio-based comparisons to return an index representing their relationship.
- `update_entity_collision_and_position`: Checks if an entity's position relative to a target is within a specific threshold, then updates the entity's coordinates and related bit-shifted values in a table.
- `check_coords_in_table`: Clears two global variables, then walks a table of fixed-size records and checks whether the coordinates stored in the current record fall inside a rectangle derived from the incoming pointer; if a record matches, it sets one of the cleared variables to one and returns, otherwise it advances to the next record until the table wraps around.
- `tick_and_update_entities`: Increments a global counter and, when it reaches a threshold, resets it, then walks a table of entity records applying per-record updates to coordinates, flags, and a shared value.
- `find_free_slot_and_store_coords`: Scans a table of records and finds the first slot whose state is not the sentinel value while its fields satisfy a set of coordinate comparisons against the two input structures.
- `store_flag_when_within`: Computes a value from three signed words taken from two incoming pointers and, when that value is large enough, stores the byte two into a location offset from the second pointer before returning.
- `update_counter_and_check_slot`: Decrements a byte counter when it is already positive, otherwise it initializes that counter from another byte.
- `store_scaled_values`: Finds the next free slot in a table of six-byte records, then stores four shifted input values there and computes a fifth field from global coordinates unless a flag selects a fixed value.
- `check_tile_and_read_collision_bits_2`: Iterates through a table of records, adding values from a tile lookup table to the record fields and storing the results as shifted coordinates in global variables.
- `copy_sprite_data_block`: Iterates through a linked list of structures, calculates a destination address based on an index and an offset, and performs a longword-based block copy of data from a source table to a destination table using a fixed-size unrolled loop.
- `update_blitter_registers`: Iterates through a table of records, calculating a value based on a pointer and a bit-shifted field, then writes several values to the hardware registers for the blitter and DMA controller.
- `check_flags_and_update_coordinate`: Runs a series of global flag checks before computing two shifted coordinates.
- `search_and_increment`: Searches a table by fields using several computed offsets and returns early when no match is found, otherwise increments a counter stored at a fixed offset within a structure.
- `find_free_tile_slot`: Computes a series of candidate positions by combining values from two records and a stored offset, then repeatedly queries a lookup table and inspects tile collision data at each candidate.
- `clamp_entity_offset`: Computes three candidate coordinate pairs by performing arithmetic on fields at offsets 70, 72, 74, and 76 of a structure, then for each candidate, performs a table lookup and a tile check.
- `adjust_and_find_enclosing_record`: Adjusts a pair of coordinates by a computed offset, then repeatedly probes a table of records to find one whose range encloses the adjusted coordinates, updating the stored values based on which record matches.
- `check_tile_collision`: Calls a lookup routine to find a matching record and then inspects a tile for collision bits, returning zero when the record is absent or the tile is blocked.
- `search_table_by_fields`: Walks a table of records by a fixed stride and returns whether it finds one that satisfies a set of field comparisons, stopping early when it reaches the end of the table.
- `reset_and_swap_record_table`: Clears a block of global memory and then reinterprets a large table as an array of records, byte-swapping each pair of fields, byte-swapping several longer regions, swapping two scattered bytes, and repeating this for every record until the whole table is processed.
- `advance_inactive_records`: Walks a repeating block of records and, for each one that is inactive but has a valid tile, advances a pointer to the next record using a stored offset.
- `process_active_inactive_records`: Performs two passes over a repeating block of records, advancing pointers for records that are active and have a valid tile, then repeating the process for records that are inactive.
- `advance_active_records`: Walks a repeating block of records and, for each one that is active but has a valid tile, advances a pointer to the next record using a stored offset.
- `setup_blitter_clip`: Clips a rectangular region against the current viewport and, if it lies within the visible area, configures the blitter with the computed source coordinates, dimensions, and modulo values.
- `copy_clipped_rectangle`: Checks if a source rectangle fits within a visible region defined by values in a0 and a global variable; if it fits, it performs a stride-based byte copy from a source buffer to a destination buffer using nested loops and unrolled transfers.
- `copy_if_record_marked`: Checks whether a record is marked and a flag is set, and if so calls a routine to copy a word block derived from the record's fields.
- `copy_and_clear_record_flags`: Walks a repeating block of records, copying a four-word block between two offsets when both a flag and a second flag are set, then clearing those fields when a specific flag equals two.
- `copy_region_block`: Copies a rectangular block of data from a source table to a destination table by calculating offsets from structure fields.
- `copy_structured_word_block`: Computes a table index from two fields of a structure pointed to by a register and derives a block length and a row offset from a four-word array.
- `load_and_cache_asset_pointer`: Calculates an offset into a table based on a value from a global variable, retrieves a pointer from that table, and if it differs from a stored pointer, updates the stored pointer and calls parse_and_decompress_asset_chunks.
- `advance_state_machine`: Drives a state machine by checking a global state variable and several flag variables; it iterates through the flags to find a matching condition, updates a global index with the result, and modifies other global state and table pointers accordingly.
- `check_tile_and_read_collision_bits`: Returns early when a stored value is out of a horizontal range, then inspects two bytes of a tile at a computed position and returns zero for several special tile markers.
- `check_tile_collision_bits`: Checks if a coordinate is within bounds, inspects a tile for specific bit patterns, and returns a value based on bitwise comparisons and a threshold.
- `sync_record_byte_from_globals`: Reads two adjacent words from a global area and scans a series of records spaced ten bytes apart.
- `update_bitmapped_graphics`: Iterates through a list of offsets and values to perform bitwise operations on a bitmapped graphic, modifying specific bits in a buffer and two additional memory locations based on a mode selected from a global variable.
- `extract_bit_patterns`: Iterates through a table of data, checking if specific bits are set at various offsets within a structure, and if a pattern is matched, stores an incrementing counter into a destination buffer.
- `reset_counters_and_clear_memory`: Clamps a short counter in a global variable to a minimum of 5, sets several other global short counters to specific values, and clears a block of 512 bytes starting at a global address.
- `reset_and_copy_structures`: Copies a large block of paired-word records from one table to another using a fixed stride of 40 bytes, then zeroes out several repeating structures by walking them with fixed strides and stores a division result into a field.
- `split_and_pack_nibbles`: Reads a 16-bit value from a table using an index from a global variable, splits it into three nibble groups, scales each by two and clamps them to a maximum of 15, 240, or 768, combines them into a single value, stores the result in a table at offset 10, and calls another routine.
- `split_pack_and_store_color`: Performs the same nibble splitting, scaling, and clamping as another routine, but also stores the combined result into a table at offset 10 and into a color register.
- `copy_table_bytes_by_division`: Divides a 16-bit value by 60 to calculate an offset into a source table, then copies 32 bytes from that table to a destination table at 40-byte intervals.
- `copy_table_pair`: Divides a value from a global variable by ten and uses the quotient as an offset to copy a block of bytes from a source table into a destination table, then repeats the process using the remainder of the division as the offset.
- `copy_digit_table_bytes`: Splits a 16-bit value into its tens and units digits by dividing by ten, then uses these digits as indices to copy bytes from two parallel tables into a series of global variables.
- `init_blitter_registers`: Calls a routine to program the blitter registers with specific control and size values based on the contents of several global variables.
- `program_blitter_registers`: Waits until a hardware flag bit is cleared, then programs the blitter with a set of control and size registers and returns without issuing a trigger.
- `clear_memory_and_run_handlers`: Clears a block of memory by writing zeros through a pointer in a loop, then repeatedly invokes a routine from a handler table until that routine returns nonzero, storing the result and calling two more handler-table routines.
- `setup_blitter_and_wait_flag`: Programs the blitter control and size registers with mode-dependent values, clears the display vector table, copies a block of entries into it, and then waits for a flag to clear while reading joystick state and disabling sprite pointers.
- `zero_fill_and_read_byte`: Zero-fills a fixed-size block of memory, then repeatedly calls a routine that reads a byte and, while it returns zero, invokes another routine.
- `conditionally_copy_buffer`: Reads a call result and, when it is zero, copies 27 bytes from one global buffer to another, otherwise records the result and invokes two referenced routines.
- `copy_table_block`: Determines a block size based on the input table pointer, copies a sequence of longwords from the source table to a destination table, clears a flag in a global variable, and writes a fixed value into a field of a structure before calling another routine.
- `call_pointer_table`: Loads four function pointers from a table of offsets stored in a global variable and executes each of them in turn before returning.
- `load_and_unpack_asset_data`: Checks a global flag to prevent re-execution, then retrieves a pointer from a table and calls a routine to parse a compressed data stream.
- `scan_records_match`: Walks a repeating table of records, comparing a field against a threshold and checking two coordinate bounds, and returns one when a matching record is found or zero after the whole table has been scanned.
- `find_camera_in_bounds`: Walks a repeating table of records, each 1688 bytes long, and checks whether any record is active and within a small range while a camera position falls inside the record's rectangular bounds.
- `handle_character_movement_collisions`: Checks if a global flag is set and another is clear, then calculates tile coordinates by dividing global values by thirty-two and calls validate_tile_and_collision_mask.
- `check_collision_with_objects`: Checks for collisions between a moving entity and a list of objects by iterating through a table and comparing coordinate ranges.
- `update_status_and_copy_value`: Stores the value 4 in a global variable, copies a 16-bit word from offset 2 of a structure in a0 to another global variable, and if two specific global variables are non-zero, sets the first one to 3.
- `update_state_and_bounds`: Sets a global 16-bit value to 1, then checks two global 16-bit values; if the first is non-zero, it is set to 3, and if the second is non-zero, it is cleared to 0 while the first is set to 3.
- `update_sprite_table_offsets`: Iterates through a table of sprite definitions to validate dimensions and update offset values in global variables based on the sprite's type and size.
- `set_blitter_window_registers`: Programs the blitter horizontal and vertical window registers to two fixed sizes depending on a global flag, then clears a custom register and calls another routine.
- `fill_bitmap_slots`: Fills five consecutive slots in a table with a pointer split into its high and low words, writes a fixed value into a following slot, and then stores a value into a global variable that controls the bitmap modulos.
- `dispatch_table_command_handler`: Checks a global variable against several constant values and branches to specific routines or performs data operations based on the match.
- `accumulate_byte_pairs_in_records`: Walks a repeating table of five-byte records, accumulating pairs of signed bytes from a source table into matching fields until a field overflows and is set to three, and it stops when the record pointer returns to its starting address.
- `accumulate_buffer_values`: Iterates through a table of offsets and values to add bytes from a source to a destination buffer, using a state variable to determine whether to add the values directly or add them multiplied by two.
- `add_values_to_buffer`: Iterates through a buffer, adding bytes from another source to the current buffer at offsets determined by the source values, and repeats this process using a different source until a limit is reached.
- `add_values_to_buffer_2`: Iterates through a table, adding bytes from a source to a destination buffer at specific offsets, and calls a routine to copy a block of data.
- `process_table_records_2`: If a global variable is zero, it calls another routine and a second routine, then performs a series of calculations involving multiplication and bit shifts to update two global variables based on values from a table.
- `accumulate_indexed_bytes_in_records`: Computes two differences between global values, clears a couple of fields, then walks a repeating table of five-byte records adding indexed bytes from two source tables into matching fields until the record pointer returns to its starting address.
- `increment_matching_record_field`: Walks a repeating table of five-byte records looking for one whose ninth byte is nonzero, then increments a field in the matching record and adjusts that record's ninth byte based on comparisons and a parity check.
- `add_buffers_marker`: Adds pairs of bytes from one buffer into another, incrementing a source byte each time, and writes a marker byte into the destination when a compared value exceeds a threshold before returning.
- `find_nonzero_clear`: Scans a repeating table of records looking for one whose field is nonzero, and once none is found it clears two global variables.
- `scan_tile_bit_match`: Splits a coordinate into a tile index and offset, then scans a small square region testing a bit table and stores the matching position and a flag into a result structure.
- `test_bit_accumulate`: Computes an index from two coordinate values, tests a bit derived from one coordinate against several byte tables, and accumulates the matching bits into a returned byte.
- `dispatch_table_entries`: Skips zeroed entries in a stride-padded table and invokes a dispatched routine for each entry whose two fields fall within a computed range, updating a running value.
- `load_table_by_counter`: Decrements a global 16-bit counter and, if the result is not a multiple of 30 and falls within a specific range, calculates an index by dividing the remainder by 5 and shifting it left.
- `advance_table_indices`: Walks through a table of short values four bytes at a time, reading a pair of indices to compute a pointer into a global table and storing a modified value there, until the pointer returns to the start.
- `process_dual_table_indices`: Iterates through a table of short values, using two indices to locate a value in a global table and either setting it to zero or incrementing it based on specific bitwise conditions and a global flag, then calls another routine.
- `process_hardware_interrupt`: Checks a bit in a hardware register and, if set, performs several comparisons of global variables, copies eight longwords from a table to a hardware register area, and calls another routine.
- `update_keyboard_register`: Reads a byte from the keyboard serial register, performs bitwise inversion and a rotation, and then stores a byte at an offset determined by the result into a memory location.
- `write_byte_and_return`: Stores a byte into the address pointed to by the sum of a0 and d0, then restores registers and returns via a jump.
- `clear_globals_and_blit`: Clears a run of consecutive global bytes in a loop, zeroes three specific global variables, and then copies that cleared byte region into several fixed global memory locations.
- `store_memory_values`: Stores two 32-bit values from memory into two global variables and returns.
- `clamp_and_store_scroll_offsets`: Clamps two scrolling offsets to their allowed ranges and accumulates them into running totals, then indexes a table using the clamped values to compute a pair of screen coordinates.
- `store_accumulated_32bit_value`: Loads a base value from a table, adds a fixed offset depending on which of two global pointers is in use, and stores the resulting 32-bit value split into consecutive little-endian words.
- `find_valid_adjusted_position`: Walks a repeating block of global data, and for each entry whose control byte is between zero and three, checks neighboring coordinates against limits and, when a target cell is unlocked, derives adjusted coordinates that fall within bounds.
- `update_interpolated_value`: Performs a loop that calculates an interpolated value using a scaling factor and a divisor from a table, updates a global value with the result, and decrements a counter in a global variable.
- `search_nearest_coordinate`: Performs a search by iterating through a range of coordinates to find a valid position, then calculates a distance-based value and updates several global variables and a table with the resulting coordinates and parameters.
- `copy_record_fields_when_flag_set`: Checks a global flag and, when it holds a specific value, copies fields from a table of five-byte records into a set of destination locations while decrementing a counter until the flag becomes zero.
- `compute_arccos_angle`: Computes the arccosine of the normalized dot product between two vectors using a fixed-point Taylor approximation, then stores the resulting angle in a global variable.
- `clamp_and_read_index`: Clamps a signed index into a bounded range by repeatedly adding and subtracting a stride, then returns the global value stored at the shifted table offset.
- `clamp_and_negate_index`: Clamps a signed index into a bounded range by repeatedly adding and subtracting a stride, then returns the negated global value stored at the shifted table offset.
- `execute_conditional_callbacks`: Checks if a global variable is 1 and if another global variable is neither 86 nor 87, then calls twenty-four different routines if those conditions are met.
- `test_and_adjust_indices`: Sets a global flag to zero and checks a condition to either use existing indices or calculate new ones by shifting and subtracting from global values.
- `find_and_set_record`: Walks a circular linked list of records, checking each entry against a set of field conditions, and sets a global value and flag when a matching record is found.
- `clamp_position_difference`: Calculates the absolute difference between two coordinate pairs, clamps each coordinate toward a target when the distance exceeds a threshold, and copies the target values back.
- `advance_state_machine_by_counter`: Checks a series of global flags and a counter against several fixed values, then returns early unless those conditions pass, after which it toggles a flag and increments another counter up to a maximum.
- `detect_tile_corner_collision`: When a mode flag is set and a direction flag selects a side, it computes two adjacent tile coordinates and probes a small grid of neighboring positions by querying tile collision data.
- `update_global_counters`: Calculates bounds from global variables, calls another routine to find matching records, and then modifies two global counters using conditional increments, decrements, and scaling by multiplication and division.
- `snap_coordinate_boundary`: Checks a global flag, computes the distance between two coordinates, and snaps a coordinate to a boundary offset while clearing a secondary value when within range.
- `enforce_coordinate_minimum`: Enforces a minimum value on two coordinates by clamping them, inverts the direction of each when a coordinate is reset, and adjusts the direction when a computed position falls below the current value.
- `store_scaled_position`: Computes a scaled position value and stores it into a coordinate only when it does not exceed a comparison threshold.
- `accumulate_record_differences`: Walks a circular list of records, accumulating the differences between their coordinate fields into running totals and storing the updated values back into the records.
- `validate_record_coordinates`: Walks a circular linked list of records, computing absolute differences between coordinate fields and verifying them against thresholds, returning a success flag when a valid record matches.
- `search_table_by_field_ranges`: Iterates through a table of records by a fixed stride, checking if specific bytes and words at fixed offsets satisfy certain conditions.
- `update_clamped_indices`: Updates two global index variables by adding or subtracting values derived from a structure passed in a3, using bit shifts to scale the differences.
- `update_table_values`: Checks several global variables and, if they match specific values, calls another routine, updates two global variables by shifting values from a table, and potentially modifies a third global variable based on a comparison.
- `unpack_and_store_divided_values`: Reads two packed values from a table, divides each by sixteen, and stores the results into two adjacent global variables.
- `update_packed_table_value`: Loads a packed value from a table, splits it into two components, increments or clears the value based on the component ranges, sets a global counter, and reports whether a change occurred.
- `store_surrounding_coordinates`: Computes four offset coordinate pairs around a central position and stores each pair into a destination table when a supporting routine reports success, clearing the destination when no table is available.
- `clear_record_field`: Walks through a sequence of six-byte records in a global table, comparing the two-byte field at offset four of each record, and stops when that field equals two.
- `process_counter_from_table_slot`: Reads a control flag from a table slot and increments one of two counters depending on its value, clamping each at ninety-nine, then writes a toggling flag back to the slot.
- `adjust_counter_by_table_values`: Skips work while a flag is set, then inspects two shifted table values and adjusts a counter by adding or subtracting six based on which value matches a sentinel.
- `check_position_blocked`: Checks whether a position lies outside a range or inside a tile marked with a special marker, returning one in those cases.
- `update_position_or_walk_structure`: Updates a position using scaled index lookups and a decaying value when a target differs, otherwise walks a linked structure adding a delta to each of its entries.
- `compute_and_store_position`: Performs scaled index lookups to compute a position, then records that position into a tracking variable when the guard flag is set.
- `update_physics_and_scaling`: Modifies three 16-bit values by applying a percentage-based scaling factor and clamps them within specific ranges.
- `adjust_global_counter`: Adjusts a global counter downward when a condition holds and upward when another condition holds, effectively nudging the value toward a target.
- `adjust_position_after_block`: Adds a movement delta to a pair of coordinates and checks if the resulting tile is blocked.
- `check_tile_passability`: Tests whether a tile position is passable by decoding a tile id and checking surrounding bitmap flags, returning a result code for the caller.
- `prepare_tile_offset`: Writes two to a global variable, shifts two values from a table right by five bits, adds seven to one of them, and branches to another routine.
- `scan_tile_passability_range`: Clears a flag and reads a value shifted right by five bits, then checks a condition flag.
- `update_entity_coordinates_via_collision`: Updates horizontal and vertical coordinate values in global variables by performing collision checks against a tilemap using a sub-routine, scaling the resulting displacement by a factor of fifty divided by one hundred.
- `validate_tile_and_collision_mask`: Calculates tile and collision bit positions from coordinate values, checks a tile type and collision bitmask, and then iterates through a table of records to verify if the coordinates fall within specific bounds and properties defined in the records.
- `split_and_store_coordinates`: Splits a pair of global coordinates by dividing them by a constant and stores the quotients along with values reduced by a fixed offset into globals.
- `fill_table_with_markers`: Fills a global table with a repeating pattern of marker values while computing index offsets from a running counter in nested loops.
- `load_sprite_frame_data`: Calculates a frame index by dividing a global counter by 40 and clamping it to a maximum of 480, then uses this index to copy a large block of 16-bit sprite data from a source table into a destination table.
- `fetch_table_entry_with_offset`: Calculates an index by shifting a value from a global variable, uses it to fetch a longword from a table via a pointer, and branches to another routine or performs a no-op based on a comparison of a global variable.
- `wait_flag_and_align_address`: Waits for a bit in a global variable to clear, then stores two incoming values into that variable and computes a twice-aligned address from a table offset plus an unmodified register, leaving the result stored twice.
- `compare_table_pointers`: Compares a longword at one address with a longword at another, and if they are not equal, branches to another routine; otherwise, it compares a word at a third address to a constant and branches accordingly.
- `scale_palette_entries_2`: Copies sixteen words from a source table to a destination table, then performs a linear interpolation for each word by scaling its individual nibbles using a factor and a complement factor.
- `scale_palette_entries`: When a pointer matches a stored value, scales a sixteen-step factor into sixteen palette entries by shifting each nibble, then copies those computed values into a second table.
- `copy_table_eight_words`: Reads eight consecutive words from a table pointed to by a global variable and stores them into a fixed sequence of memory locations.
- `copy_table_conditional_words`: Copies eight words from a table into one memory block and, only when a flag equals one, copies eight more words into a following block.
- `wait_mode_register_values`: Selects a pair of mode-register offsets depending on a mode value, then spins until that global register holds each of two expected values in turn.
- `wait_mode_register_alt_values`: Chooses a second pair of mode-register offsets when a mode value is two, then waits for the global register to reach each of the two selected values.
- `wait_mode_register_values_2`: Selects a pair of bitmask values based on a global variable, then spins until the specified hardware register matches each of the two expected values in turn.
- `setup_display_table`: If a global variable equals two, it copies a block of memory from one table to another, writes several values to custom video registers, and fills two sets of four consecutive short slots in a table using high and low halves of values from a source table; otherwise, it writes fixed values to custom video registers and a table.
- `read_coprocessor_and_clear`: Reads a word from a location near the coprocessor trigger register into a global variable and then clears that variable.
- `wait_for_cia_bit`: Loops until bit 6 of the CIAA Port A Data Register is cleared, then returns.
- `run_nested_routines`: Initializes two counter fields in a global structure, then repeatedly invokes another routine in nested loops until both counters reach their limits, and finally copies forty successive long values from one table into two other tables.
- `init_and_swap_globals`: It zeroes a global, points two globals at a shared memory region, invokes another routine, then copies a value from one global into a temporary and swaps that value into a second global.
- `init_buffer_and_load`: It points one global at a shared memory region, clears a second global, invokes another routine, and returns.
- `init_buffer_and_load_asset`: Sets a global variable to point to a specific memory address, clears a second global variable, and calls parse_and_decompress_asset_chunks.
- `parse_compressed_data_stream`: Reads a file using library calls and parses a custom data format containing tagged blocks.
- `parse_and_decompress_asset_chunks`: Opens a file and iterates through a data stream to identify specific chunk headers.
- `disable_sprite_pointers`: Disables all sprite pointer registers by writing zero to each of them and sets a flag in the DMA control register.
- `fill_table_of_multiples`: Fills a table of words with the multiples of a shifted input value by writing each product through a running pointer until the count is exhausted.
- `init_short_table`: Fills a table of fifty-one short values with a linear sequence where each entry equals its index multiplied by seventy-eight.
- `init_int_table`: Fills a table of six hundred integer values with a linear sequence where each entry equals its index multiplied by one hundred and sixty.
- `init_short_table_2`: Fills a table of ten short values with a linear sequence where each entry equals its index multiplied by one hundred and fifty-eight.
- `init_short_table_3`: Fills a table of ten short values with a linear sequence where each entry equals its index multiplied by one thousand six hundred and eighty-eight.
- `copy_shorts_stride`: Copies forty successive short values from a source table into a destination table, advancing the source by a stride while the destination advances by the sum of a global value and two.
- `blitter_copy_wait`: Waits for a hardware flag to clear, then programs the blitter registers and copies forty short values from one location to two others while advancing all pointers by a global value and two.
- `setup_blitter_and_copy_data`: Calculates source and destination addresses using tables and offsets, configures the blitter hardware registers, and then copies 64 words from a source to a destination using a loop.
- `copy_shorts_dual`: Copies forty successive short values from a source table into two destination tables, advancing the source by a stride while both destinations advance by a global value and two.
- `copy_table_to_two_destinations`: Copies a block of 16-bit words from a source table to two destination tables, offsetting each destination by a stride value and advancing all three pointers by that stride on every iteration.
- `copy_shorts_triple`: Copies forty successive short values from a source table into three destination tables, advancing the source by a stride while all destinations advance by a global value and two.
- `copy_bytes_offset`: Copies a byte from a source table to a destination table and then copies four more bytes at successive offsets of a stride value.
- `compare_and_return`: Compares a longword at a specific memory address to zero and returns if it is non-zero, otherwise it continues to the next instruction.
- `perform_line_drawing_collision_check`: Calculates the difference between two points and uses a fixed-point division loop to step through coordinates, updating two global pointers and calling check_tile_and_read_collision_bits to detect collisions during the traversal.
- `calculate_division_and_check_collision`: Swaps d0 and d1, performs a division of the high word of d0 by d1, and if d1 is at least 4, calls another routine to check collision bits and returns if the result is non-zero.
- `update_global_values`: Increments d1, adds d4 to a global word, adds d0 to d3, and conditionally adds d5 to another global word within a loop controlled by d2.
- `calculate_division_and_check_collision_2`: Swaps the high word of d0 and divides it by d1, then if d1 is at least 4, calls another routine to check collision bits and returns if the result is non-zero.
- `update_global_values_alternate`: Increments d1, adds d5 to a global word, adds d0 to d3, and conditionally adds d4 to another global word within a loop controlled by d2.
- `invoke_routine_by_count`: Computes the absolute differences between two pairs of stored values and selects the smaller count as a loop bound, then repeatedly invokes a routine from a dispatch table that many times, updating the stored pointers afterward.
- `calculate_indirect_jump_target`: Calculates a jump target by adding an offset from a table to a pointer, adjusting the pointer with a value derived from a bit-shifted register, and performs an indirect jump using a table in a5.
- `mask_four_bytes_stride`: Applies a single mask byte to four consecutive bytes spaced by a stride through a byte pointer, updating each in place.
- `mix_or_mask_bytes_stride`: Ores the first byte at a pointer while masking the next three spaced bytes by a stride, writing each result back.
- `mix_mask_or_bytes_stride`: Masks the first byte at a pointer, ores the second spaced byte, then masks the following two, writing each result back.
- `or_two_mask_two_stride`: Ores the first two bytes spaced by a stride at a pointer, then masks the next two, writing each result back.
- `mask_two_or_two_stride`: Masks the first two bytes spaced by a stride at a pointer, ores the third, then masks the fourth, writing each result back.
- `or_mask_alternate_stride`: Ores the first and third bytes spaced by a stride at a pointer while masking the second and fourth, writing each result back.
- `mask_and_set_bytes`: Performs bitwise AND and OR operations on three bytes at specific offsets from a pointer using values in registers d4, d5, and d6.
- `set_and_mask_bytes`: Performs bitwise OR operations on three bytes at specific offsets from a pointer using values in registers d4, d5, and d6.
- `mask_and_set_bytes_alt`: Performs bitwise AND operations on three bytes and a bitwise OR on a fourth byte at specific offsets from a pointer using values in registers d4, d5, and d6.
- `set_and_mask_bytes_alt`: Performs bitwise OR operations on two bytes and bitwise AND operations on two bytes at specific offsets from a pointer using values in registers d4, d5, and d6.
- `mask_set_mask_set_bytes`: Performs bitwise AND and OR operations on four bytes at specific offsets from a pointer using values in registers d4, d5, and d6.
- `set_set_mask_set_bytes`: Performs bitwise OR operations on two bytes, a bitwise AND on a third, and a bitwise OR on a fourth byte at specific offsets from a pointer using values in registers d4, d5, and d6.
- `mask_mask_set_set_bytes`: Performs bitwise AND operations on two bytes and bitwise OR operations on two bytes at specific offsets from a pointer using values in registers d4, d5, and d6.
- `apply_mixed_bitmasks`: Ors a byte from d4 into a memory location, then performs an and with d5 at an offset determined by d6, and finally ors d4 into two more locations at d6-based offsets.
- `apply_mixed_bitmasks_alt`: Ands a byte from d5 into a memory location, then ors d4 into two locations at d6-based offsets, and finally ors d4 into a third location at a d6-based offset.
- `apply_or_bitmasks`: Ors a byte from d4 into four different memory locations, each separated by a 16-bit offset stored in d6.
- `clear_table_byte_block`: Clears four consecutive bytes in a byte table by ANDing each with a mask value.
- `store_masked_bytes`: It writes a byte from a register into memory through a pointer, then writes a masked byte to the following location and two more masked bytes to the next two locations, advancing the pointer by the same amount each time.
- `apply_and_or_bitmasks`: Ands a byte from d7 into a memory location, ors d6 into the next location at a d1 offset, and thens ands d7 into two more locations at d1-based offsets.
- `apply_or_and_bitmasks`: Ors a byte from d6 into two memory locations at a d1 offset, and thens ands d7 into two more locations at d1-based offsets.
- `apply_and_or_bitmasks_alt`: Ands a byte from d7 into two memory locations at a d1 offset, ors d6 into the next location, and thens ands d7 into a final location at a d1 offset.
- `apply_or_and_bitmasks_alt`: Ors a byte from d6 into a memory location, ands d7 into the next location at a d1 offset, thens ors d6 into the next location ands d7 into a final location at a d1 offset.
- `apply_and_or_bitmasks_alt2`: Ands a byte from d7 into a memory location, ors d6 into two locations at a d1 offset, and thens ands d7 into a final location at a d1 offset.
- `apply_or_and_bitmasks_alt2`: Ors a byte from d6 into three memory locations at a d1 offset, and thens ands d7 into a final location at a d1 offset.
- `apply_and_or_bitmasks_alt3`: Ands a byte from d7 into three memory locations at a d1 offset, and thens ors d6 into a final location at a d1 offset.
- `apply_bitmask_pattern_one`: Ors a byte from d6 into a byte at the address in a3, then performs two bitwise AND operations with d7 at offsets of d1 from the original address, and finally ors a byte from d6 into the address at offset d1 from the second AND operation.
- `apply_bitmask_pattern_two`: Ands a byte from d7 into a byte at the address in a3, then performs two bitwise OR operations with d6 at offsets of d1 from the original address, and finally ands a byte from d7 into the address at offset d1 from the second OR operation.
- `apply_bitmask_pattern_three`: Ors a byte from d6 into a byte at the address in a3, then performs two bitwise OR operations with d6 at offsets of d1 from the original address, and finally ands a byte from d7 into the address at offset d1 from the second OR operation.
- `apply_bitmask_pattern_four`: Ands a byte from d7 into a byte at the address in a3, then performs two bitwise AND operations with d7 at offsets of d1 from the original address, and finally ors a byte from d6 into the address at offset d1 from the second AND operation.
- `init_repeating_struct_offsets`: Initializes a repeating structure by writing paired word fields at a fixed interval, then computes and stores offset values into a couple of global variables.
- `store_routine_pointers`: Reads three consecutive longwords from a routine dispatch table and stores each one into a separate global variable.
- `process_input_strings`: Performs several library calls, checks if a global byte matches specific characters, and if so, converts a string of decimal digits into a 16-bit integer and stores it in a global variable.
- `invoke_routines_and_store_2`: Performs two library calls, then reads a string from a global buffer and converts it from a decimal representation into a 16-bit integer stored in a global variable.
- `invoke_routines_and_store`: Invokes two routine pointers from a dispatch table, then stores a value derived from comparing a global variable against two character constants.
- `update_global_value_2`: Calculates a value by summing three words with different bit shifts, compares it to a global variable, and updates two global variables if a condition is met.
- `update_global_value`: Checks if a bit is set in a word at a pointer in d3, then clears a word at an offset of that pointer, performs a bitwise rotation and addition on the value, and finally updates a global variable and three other global variables with the result.
- `decode_huffman_stream`: Decodes a Huffman-style bit stream, emitting literal bytes and back-references that copy previously decoded data into a growing output buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `load_and_decompress_data`: Constructs a filename string by converting a counter into a three-digit decimal representation followed by a fixed extension, opens and reads the file, and then performs decompression by decoding a bit stream that uses either literal bytes or back-references to expand data into a buffer.
- `wait_mode_and_setup_media_frames`: Waits for the copper mode register to reach expected values for the current display mode, then copies a block of words from a source table into a destination block and clears the source.
- `setup_polygon_and_call`: Initializes a polygon vertex table with computed coordinate pairs, then calls two routine pointers from a dispatch table.
- `iterate_string_routines`: Walks a null-terminated string, then repeatedly calls a routine pointer from a dispatch table once per character.
- `update_display_and_state`: Calculates a maximum value from a set of global variables, configures the copper and DMA control registers, and executes a sequence of calls to update display elements and text strings based on a state variable.
- `modify_dma_control`: Decrements a 16-bit value in a global variable, and if it is greater than zero, writes one to a global variable, calls another routine, and writes one hundred eighty to a custom register.
- `check_modulo_ten_then_init`: Checks whether a global variable modulo ten is nonzero and returns early when it is, otherwise it points two global variables at a shared address, clears a flag, runs five other routines, and finally writes a fixed value and then zero into a global variable.
- `init_and_run_frame_loop`: Initializes memory pointers and registers, sets window coordinates, calls a dispatch routine, then loops waiting on a flag while repeatedly invoking helper routines.
- `process_coordinate_list`: Iterates through a list of coordinates, calling routine pointers with various decremented offsets and performing drawing operations.
- `call_dispatch_routine`: Calls a single routine pointer from a dispatch table and returns.
- `render_sprite_layers`: Iterates through several tables of sprite data, performing bitmapped graphics operations for each entry by loading dimensions, coordinates, and color data into registers before calling a routine to draw them.
- `manage_display_elements`: Updates several global counters, calls various routines to handle string matching, joystick status, and bitmapped graphics, and modifies hardware registers to control DMA and display modes.
- `call_indirect_global_func`: It invokes a function pointer stored in a global variable and then returns without doing anything else.
- `call_indirect_global_ptr`: It invokes a routine whose address is stored in a global variable, obtained by dereferencing a pointer located at a fixed offset from that global.
- `call_blt_bit_map_via_indirect_pointer`: Loads parameters into registers, including a width of 32 and a height of 48, and calls a routine via a pointer stored in a global variable.
- `run_timed_loop_until_flag`: Points two global variables at a shared address, clears a flag, runs several other routines, then runs a timed loop that decrements a counter until it reaches zero or until a status flag becomes one, returning early with a fixed value stored when that flag is set.
- `set_value_by_state_then_timed_loop`: Stores one of two fixed values into a global variable depending on whether another global equals two, points two globals at a shared address, sets a flag while running two routines, clears the flag, runs another routine, and then runs a timed loop that returns early when a status flag is set.
- `configure_copper_and_blitter`: Sets a hardware register value based on a global flag, performs several calls to update coordinates in a structure, fills a buffer with spaces using a loop, and manages DMA and coprocessor registers through a loop.
- `match_and_copy_string`: Compares a source string in a1 against a destination string at a zero address, and if they match, copies characters from the source to the destination while handling a specific character value by writing a space or zero.
- `match_and_copy_string_short`: Compares a source string in a1 against a destination string at a zero address, and if they match, copies characters from the source to the destination or writes a zero if a specific character is encountered.
- `iterate_entries_with_callback`: It walks through a table of pipe-separated entries, computing a position offset for each and invoking a callback routine for every character.
- `invoke_callback_per_char`: It measures the length of a NUL-terminated string, then repeatedly invokes a callback routine once for each character in the table.
- `compute_indices_and_call`: It derives two index values from a global counter and then invokes a callback routine four times in succession.
- `iterate_pipe_separated_entries`: It iterates through a string, calculating an offset based on the position of pipe characters and invoking a callback routine for each non-pipe character.
- `process_pipe_delimited_characters`: It iterates through a string, invoking a callback routine for each character and resetting the offset to a stored value whenever a pipe character is encountered.
- `compute_wrapped_offset`: Computes a wrapped value from a sum of three words from a pointer in d3, then applies a modulo-based remapping that mixes a divided remainder with a scaled quotient, and stores the result into a global variable at an offset of a pointer in d3.
- `decrement_counters_and_call`: It decrements two global counters if they are nonzero, then invokes a callback routine selected by an index.
- `update_wrapped_counter`: Writes a constant to an offset in a structure, increments a counter at another offset, calculates a value by subtracting one and shifting right, stores that value at a third offset, calls another routine, and if the counter reaches twelve, resets several global variables and specific structure offsets.
- `decrement_global_values`: Compares a 16-bit value at a global address to eight, and if it is smaller, subtracts 32 from two other global addresses and decrements two more global counters.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_wrapped_counter_with_check`: Writes a constant to an offset in a structure, increments a counter, calculates a value by subtracting one and shifting right, and if that value is five, updates the structure offset and a constant; it then calls another routine and, if the counter reaches twelve, resets several global variables and specific structure offsets.
- `increment_global_values`: Compares a 16-bit value at a global address to eight, and if it is smaller, adds 32 to two global addresses and increments two more global counters.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `increase_clamped_global_2`: Writes a constant to a structure offset, clears a counter offset, calls another routine, increments a global variable by four, and then clamps the value to a limit derived from a register.
- `increase_clamped_global`: Writes a constant to a structure offset, clears a counter offset, calls another routine, increments a global variable by four, and then clamps that global variable against a value derived from another global variable by shifting it left and subtracting eleven, finally clearing two other globals.
- `decrease_clamped_global_2`: Writes a constant to a structure offset, clears a counter offset, calls another routine, subtracts eight from a global variable, and clamps it to zero if it becomes negative.
- `decrease_clamped_global`: Writes a constant to a structure offset, clears a counter offset, calls another routine, subtracts four from a global variable, and if the result is negative, sets it to zero, then clears two other globals.
- `increment_position_global_2`: Writes a constant to a structure offset, clears a counter offset, calls another routine, and adds eight to a global variable.
- `increment_position_global`: Writes a constant to a structure offset, clears a counter offset, calls another routine, increments a global variable by four, and clears two other globals.
- `update_control_values`: Writes zero to a long variable at offset 31c, then if a word in d0 is one, it compares a word at offset 352 to 65535; if they are equal, it writes 88 to offset 30e, writes zero to offset 354, and branches to another routine.
- `read_wrapped_offset_2`: Sets a word at offset 350 to zero, and if a global word at offset 25ce is at least four, writes a constant to offset 31e and calls another routine.
- `read_wrapped_offset`: Writes a constant to a structure offset, clears a counter offset, and calls another routine.
- `increment_structure_offset`: Clears a counter offset, increments a structure offset by three, calls another routine, and if the structure offset equals a specific constant, branches to another routine.
- `increment_and_set_register`: Increments a 16-bit value in a global variable and, if it equals eight, writes a constant to another global variable.
- `reset_state_to_ten`: Clears two adjacent global variables and assigns the value ten to a third global variable.
- `update_status_flags`: Writes zero to a word at offset 828, then checks if d0 and a word at offset 850 are both equal to one; if they are, it writes zero to offset 884, 59 to offset 526, and branches to another routine.
- `reset_and_read_offset_2`: Sets a word at offset 350 to zero, and if a global word at offset 25ce is at least four, writes a different constant to offset 31e and calls another routine.
- `reset_and_read_offset`: Writes a constant to a structure offset, clears a counter offset, clears a second counter offset, and calls another routine.
- `update_offset_pointer_logic`: Compares a word in a global variable to 18465; if it is not equal, it writes 17444 to that global variable and calls another routine, otherwise it writes 0 to a global variable and returns.
- `update_offset_pointer_2`: Compares a word in a global variable to 18468; if it is not equal, it writes 15381 to that global variable and calls another routine, otherwise it returns.
- `update_offset_pointer`: Compares a word in a global variable to 15381; if not equal, it writes 15384 to that global variable and calls another routine, otherwise it returns.
- `increment_and_check_counter_5`: Increments a global variable by one, and if it equals eight, branches to another routine; otherwise, it writes a constant to a structure offset and calls another routine.
- `reset_state_to_twenty`: Clears two adjacent global variables and assigns the value twenty to a third global variable.
- `update_tile_coordinates_and_check`: Checks if two 16-bit values at specific offsets in a structure are zero; if not, it calculates tile coordinates by shifting and adding values, stores them in a global table, calls check_tile_and_read_collision_bits, and if a collision is detected and a specific flag is set, clears several global variables before returning.
- `increment_and_update_index`: Increments a 16-bit value in a structure and, if it is less than 60, resets a secondary field and calls another routine; otherwise, it performs a series of additions and subtractions using values from a structure and a global variable, then updates a 16-bit value in that structure and potentially recalculates a new index based on a division by 10.
- `update_global_if_not_one`: Compares a 16-bit value in a global variable to one, and if it is not equal, it writes the value eight to another global variable.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `reset_timer_and_trigger_event`: Checks if a value in a structure is zero, sets a field to three, and verifies a global status; if a specific timer value reaches a threshold, it resets a counter, sets a global value to 6979, and calls another routine.
- `write_offset_and_read`: Writes a constant to a structure offset, stores the contents of register d0 into a counter offset, and calls another routine.
- `check_and_reset_state_counters`: Checks if two specific global variables are equal to 1; if they are, it performs a series of divisions on a value in d0 to check if the result equals 10, then resets two global variables and calls another routine before returning.
- `setup_clamped_offset`: Initializes a global variable with a fixed value and another with the high half of an incoming word, then adjusts the second value when it exceeds a threshold before delegating to another routine.
- `reset_set_offset_clear_4`: Compares a word in a global variable at offset 25d0 to 2; if it is not equal, it returns.
- `set_offset_zero`: Sets a global variable to zero, assigns a fixed value to another, and calls another routine.
- `reset_set_offset_clear`: Resets one global variable to zero, assigns a fixed value to another, calls another routine, and then clears a third global variable.
- `initialize_wrapped_offset_parameters_3`: Compares a word in a global variable at offset 25d0 to 3; if it is not equal, it returns.
- `initialize_wrapped_offset_parameters`: Clears a 16-bit value in a global variable, sets another 16-bit global variable to 10291, and calls compute_wrapped_offset.
- `reset_set_offset_clear_alt`: Resets one global variable to zero, assigns a fixed value to another, calls another routine, and then clears a third global variable.
- `set_offset_fixed_alt_2`: Compares a word in a global variable at offset 25d0 to 4; if it is not equal, it returns.
- `set_offset_fixed_alt`: Sets a global variable to zero, assigns a fixed value to another, and calls another routine.
- `initialize_offset_pointer`: Compares a word in d0 to 30; if it is higher than 30, it returns, otherwise it clears a global variable, writes 6427 to a global variable and calls another routine.
- `check_range_and_initialize`: Checks if a word in d0 is between 35 and 53; if it falls within this range, it clears a global variable, writes 16673 to a global variable, and calls another routine, otherwise it returns.
- `initialize_wrapped_offset_parameters_2`: Clears a 16-bit value in a global variable, sets another 16-bit value in a different global variable, and calls compute_wrapped_offset.
- `reset_set_and_call_offset_compute`: Resets a 16-bit global variable to zero, assigns the value 6427 to another 16-bit global variable, calls compute_wrapped_offset, and then clears a third 16-bit global variable before returning.
- `calculate_offset_index_2`: Compares a word in a global variable at offset 25d0 to 5; if it is not equal, it returns.
- `calculate_offset_index`: Divides a word in d0 by 16, takes the upper 16 bits of the result, and if the value is not higher than 4, it writes 0 to a global variable, writes 258 to a global variable, and calls another routine, otherwise it writes 0 to a global variable, writes 258 to a global variable, and calls another routine.
- `set_offset_pointer_low`: Compares a word in d0 to 8; if it is not higher than 8, it returns, otherwise it clears a global variable, writes 2611 to a global variable and calls another routine.
- `validate_and_set_offset`: Compares a word in d0 to 12; if it is not higher than 12, it clears a global variable, writes 2614 to a global variable, and calls another routine, otherwise it returns.
- `setup_wrapped_offset_parameters`: Clears a 16-bit value in a global variable, writes the constant 2611 to another global variable, and calls compute_wrapped_offset.
- `update_threshold_values_2`: Compares a word in a global variable at offset 25d0 to 6; if it is not equal, it returns.
- `update_threshold_values`: Compares a word in d0 to 5; if it is higher, it writes 0 to offset 354 and 10774 to offset 31e of a global variable, then calls another routine.
- `set_high_threshold`: Compares a 16-bit value in d0 to 0; if it is higher, it writes 0 to offset 852 and 10313 to offset 800 of a global variable, then calls another routine.
- `set_high_threshold_value`: Compares d0 to 95, and if it is not less than or equal, it writes zero to offset 354, writes 13366 to offset 31e, and calls another routine.
- `set_offset_high_threshold`: Compares d0 to 90, and if it is not less than or equal, writes zero to offset 884, 10313 to offset 830, and calls another routine.
- `calculate_offset_index_subtracted`: Subtracts 10 from a word in d0, then performs two checks on the upper 16 bits of the result after dividing by 16; if either check fails, it writes 0 to a global variable, writes 11562 to a global variable, and calls another routine.
- `set_offset_pointer_mid`: Compares a word in d0 to 8; if it is not lower than 8, it returns, otherwise it clears a global variable, writes 11562 to a global variable and calls another routine.
- `validate_and_set_offset_alt`: Compares a word in d0 to 12; if it is not lower than 12, it clears a global variable, writes 11565 to a global variable, and calls another routine, otherwise it returns.
- `set_wrapped_offset_parameters`: Clears a 16-bit word in a global variable and sets another 16-bit word to 11562, then calls compute_wrapped_offset.
- `update_scaled_offset_two`: Compares a word in a global variable at offset 25d0 to 7; if it is not equal, it returns.
- `reset_offset_on_low_value_2`: Compares a word in register d0 to 5; if it is not less than 5, it clears a long variable at offset 354, stores 12842 at offset 31e, and calls another routine.
- `reset_offset_on_low_value_alt`: Compares a word in register d0 to 10; if it is not less than 10, it clears a long variable at offset 354, stores 12842 at offset 31e, and calls another routine.
- `reset_offset_on_low_value`: Compares a word in register d0 to 15; if it is not less than 15, it clears a long variable at offset 354, stores 12845 at offset 31e, and calls another routine.
- `reset_offset_if_not_less_than_twenty`: Compares a word in register d0 to 20; if it is not less than 20, it clears a long variable at offset 354, stores 12845 at offset 31e, and calls another routine, otherwise it returns.
- `reset_offset_if_under_twenty_nine`: Compares a word in d0 to 25; if d0 is not less than 25, it clears a long variable at offset 354, stores 12845 at offset 31e, calls another routine, and returns.
- `process_conditional_pointer`: Compares a word in d0 to 30, and if it is lower, writes 0 to a longword location, writes 12842 to a longword location, and calls another routine before returning.
- `process_divided_value`: Compares a word in d0 to 160; if it is lower, it subtracts 30 from d0, extends it to a long, divides it by 26, and if the upper word of the result is 2 or higher, it writes 0 to offset 354 and 12851 to offset 31e of a global variable, then calls another routine.
- `set_low_threshold`: Compares a 16-bit value in d0 to 4; if it is lower, it writes 0 to offset 852 and 12854 to offset 800 of a global variable, then calls another routine.
- `set_low_threshold_value`: Compares d0 to 6, and if it is not less than, it writes zero to offset 354, writes 13369 to offset 31e, and calls another routine.
- `set_offset_low_threshold`: Compares d0 to 8, and if it is not less than, writes zero to offset 884, 16723 to offset 830, and calls another routine.
- `update_offset_high`: Compares a word in d0 to 10; if it is not lower than 10, it writes 0 to a global variable, writes 12857 to a global variable, and calls another routine.
- `set_offset_pointer_high`: Compares a word in d0 to 12; if it is not lower than 12, it returns, otherwise it clears a global variable, writes 12854 to a global variable and calls another routine.
- `validate_and_set_offset_v2`: Compares a word in d0 to 14; if it is not lower than 14, it clears a global variable, writes 12851 to a global variable, and calls another routine, otherwise it returns.
- `set_wrapped_offset`: Stores zero into a global variable, stores 12842 into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `set_wrapped_offset_2`: Stores zero into a global variable, stores 12841 into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `update_scaled_offset_2`: Subtracts 60 from a word in a global variable and compares it to 60; if it is not lower than 60, it writes 0 to a global variable, writes 6427 to a global variable, and calls another routine, otherwise it writes 0 to a global variable, writes 6427 to a global variable, calls another routine, and then writes 0 to the original global variable.
- `set_scaled_offset_pointer`: Compares a word in d0 to 20; if it is not lower than 20, it returns, otherwise it shifts d0 right by 2, stores the result in a global variable, writes 15402 to a global variable and calls another routine.
- `validate_and_set_offset_v3`: Compares a word in d0 to 40; if it is not higher than 40, it clears a global variable, writes 15417 to a global variable, and calls another routine, otherwise it returns.
- `compute_shifted_wrapped_index`: Computes a shifted difference of a constant and an incoming word, stores the result into a global variable, stores 15402 into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `update_entity_grid_position`: Checks a global status value and a coordinate range, then calculates and stores new grid-based coordinates in a structure by shifting the current position and subtracting a constant.
- `update_sprite_animation_state`: Decrements a counter in a structure, calls another routine, and if that routine returns a non-zero value, it modifies a coordinate offset and sets a sprite index and animation frame based on a comparison of a global value against several thresholds.
- `clamp_and_set_wrapped_index`: Clamps a global variable to a maximum value when it exceeds 101, resets another global variable to zero, stores 17923 into a third global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `check_collision_and_update_tile_indices`: Calculates tile coordinates from two pointers by shifting and adding offsets, then calls another routine to check for collisions.
- `update_animation_frame`: Increments a counter in a structure, calls another routine, and if that routine returns a non-zero value, increments an offset by 32 and updates two global variables with specific values based on a comparison of a third global variable against several thresholds.
- `clamp_and_set_wrapped_index_2`: Clamps a global variable to a maximum value when it exceeds 101, resets another global variable to zero, stores 16700 into a third global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `update_tile_and_check_collision`: Checks a 16-bit value at a specific offset within a structure against several constants; if it matches none, it calculates a grid-based coordinate by shifting and subtracting from values in the structure, calls another routine, and then updates the structure's offset and a global variable based on the result of that call and a comparison of a third global variable.
- `set_offset_mid_threshold`: Compares d0 to 10, and if it is not greater than, writes zero to offset 884, 18457 to offset 830, and calls another routine.
- `update_offset_v4`: Compares a word in d0 to 20; if it is not higher than 20, it writes 0 to a global variable, writes 17932 to a global variable, and calls another routine.
- `set_offset_pointer_v4`: Compares a word in d0 to 30; if it is not higher than 30, it returns, otherwise it clears a global variable, writes 17935 to a global variable and calls another routine.
- `validate_and_set_offset_v4`: Compares a word in d0 to 35; if it is not higher than 35, it clears a global variable, writes 17938 to a global variable, and calls another routine, otherwise it returns.
- `reset_and_set_wrapped_index`: Resets two global variables to zero, stores 6427 into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `process_coordinate_and_collision`: Calculates two values by shifting and adding to offsets of a structure in d3, writes them to offsets 25ba and 25be of that structure, calls another routine, and if the return value is zero, it performs several checks on a value from offset 0350 of the structure to write zero to offset 0354 and a constant to offset 031e before calling another routine.
- `set_offset_alt_mid_threshold`: Compares d0 to 10, and if it is not greater than, writes zero to offset 884, 18491 to offset 830, and calls another routine.
- `update_offset_v5`: Compares a word in d0 to 20; if it is not higher than 20, it writes 0 to a global variable, writes 17962 to a global variable, and calls another routine.
- `set_offset_pointer_v5`: Compares a word in d0 to 30; if it is not higher than 30, it returns, otherwise it clears a global variable, writes 17965 to a global variable and calls another routine.
- `validate_and_set_offset_v5`: Compares a word in d0 to 35; if it is not higher than 35, it clears a global variable, writes 17968 to a global variable, and calls another routine, otherwise it returns.
- `reset_and_set_wrapped_index_2`: Resets two global variables to zero, stores 6427 into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `reset_offset_on_range`: Compares a word at offset 25d0 to 0; if it is not lower, it loads a word from offset 350 into d0, subtracts 60, shifts it right by 2, and if the result is not 4, it writes 0 to offset 354 and 6427 to offset 31e of a global variable, calls another routine, and then writes 0 to offset 350.
- `set_wrapped_offset_3`: Stores 24021 into a global variable, stores a word from d0 into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `update_threshold_state`: Compares a word at offset 25d0 to 3, and if it is not less than 3, checks if a word at offset 350 minus 60 is between 0 and 115; if it is, it clears offset 354, writes 6171 to offset 31e, calls another routine, and clears offset 350 before returning.
- `set_large_threshold_value`: Compares d0 to 5, and if it is not less than, it writes 25280 to offset 31e, writes zero to offset 354, and calls another routine.
- `set_offset_large_threshold`: Compares d0 to 10, and if it is not less than, writes 25283 to offset 830, zero to offset 884, and calls another routine.
- `calculate_offset_v6_v7`: Compares a word in d0 to 100; if it is not lower than 100, it subtracts 10 from d0, divides the result by 12, and if the upper 16 bits are not lower than 3, it writes 25286 to a global variable, writes 0 to a global variable, and calls another routine.
- `set_offset_pointer_v6`: Compares a word in d0 to 6; if it is not lower than 6, it returns, otherwise it writes 25289 to a global variable, clears a global variable and calls another routine.
- `validate_and_set_offset_v6`: Compares a word in d0 to 9; if it is not lower than 9, it writes 25292 to a global variable, clears another global variable, and calls another routine, otherwise it returns.
- `set_wrapped_offset_4`: Stores 25289 into a global variable, stores zero into another global variable, and then calls a routine that wraps a sum of two globals into a range and stores it back.
- `set_offset_pointer_v7`: Compares a word in d0 to 105; if it is not lower than 105, it returns, otherwise it writes 25283 to a global variable, clears a global variable and calls another routine.
- `validate_and_set_offset_v7`: Compares a word in d0 to 110; if it is not lower than 110, it writes 25280 to a global variable, clears another global variable, and calls another routine, otherwise it returns.
- `reset_set_offset_clear_2`: Resets one global variable to zero, assigns 6427 to another, calls another routine, and then clears a third global variable.
- `check_scaled_threshold`: Compares a word at offset 25d0 to 5, and if it is not greater, it subtracts 60 from a word at offset 350, compares it to 30, and if it is not higher, it writes zero to offset 354, writes 3888 to offset 31e, and calls another routine.
- `reset_set_and_call_compute_wrapped_offset`: Resets a 16-bit global variable to zero, assigns the value 6427 to another 16-bit global variable, calls compute_wrapped_offset, and then clears a third 16-bit global variable before returning.
- `process_scaled_data`: Compares a word at offset 9704 of a structure in d3 to 6, and if it is not greater, performs several checks on a value derived from a word at offset 880 of the structure, potentially writing a bit-shifted result to offset 884 and a constant to offset 798 before calling another routine.
- `compute_scaled_offset`: Derives a value from an incoming short by subtracting six, shifting right, and storing it in a global variable, then sets a second global to a fixed value before calling another routine.
- `reset_set_offset_clear_5`: Resets one global variable to zero, assigns a fixed value of 6427 to another, calls compute_wrapped_offset, and then clears a third global variable before returning.
- `process_complex_offset`: Compares a word at an offset of 6704 from a pointer in d3 to 7; if it is not greater than 7, it performs a series of checks involving subtracting 60 from a word in a global variable, dividing by 24 or 3, and calls another routine with various values written to global variables.
- `compute_divided_offset`: Derives a value from an incoming short by subtracting six, dividing by three, and storing the result in a global variable, then sets a second global to a fixed value before calling another routine.
- `reset_set_offset_clear_3`: Resets a global word to zero, assigns the value 6427 to another global word, calls compute_wrapped_offset, and then clears a third global word.
- `check_and_reset_offset`: Subtracts 60 from a word in a global variable and compares it to 96; if the result is less than 96 or less than 5, it branches to another routine, otherwise it clears a global variable, stores 11760 in another global variable, and calls another routine.
- `reset_offset_if_less_than_90`: Compares a 16-bit value in d0 to 90; if it is less than 90, it returns, otherwise it stores zero in a global variable, stores 11568 in another global variable, and calls another routine.
- `clamp_computed_offset`: Computes a masked and shifted value from an incoming short, overrides it to a fixed value when it equals three, stores the result in a global variable, sets a second global to a fixed value, and then calls another routine.
- `reset_set_and_clear_offset`: Resets a 16-bit global variable to zero, assigns the value 6427 to another 16-bit global variable, calls compute_wrapped_offset, and then clears a third 16-bit global variable.
- `verify_and_reset_parameters`: Compares a word at offset 5c30 to 4; if it is not equal, it branches elsewhere.
- `reset_offset_parameters`: Compares a word at offset cf6 to zero, and if it is not zero, it writes zero to offset 354, writes 6427 to offset 31e, calls another routine, and writes zero to offset 350.
- `reset_offset_values`: Subtracts 60 from a word at offset 848, and if the result is not below zero, writes zero to offset 884, 6939 to offset 798, calls another routine, and then writes zero to offset 848 and offset 9684.
- `update_offset_low`: Compares a word in d0 to 5; if it is not higher than 5, it writes 0 to a global variable, writes 8793 to a global variable, and calls another routine.
- `reset_offset_if_above_55`: Compares a word in d0 to 55; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 8793 in another global variable, and calls another routine.
- `reset_offset_if_bit_two_cleared`: Tests bit 2 of d0; if it is cleared, it returns, otherwise it stores zero in a global variable, stores 2617 in another global variable, and calls another routine.
- `set_offset_and_call`: Stores zero in a global variable, stores a fixed value in another global variable, and then calls another routine.
- `reset_offset_on_condition_mismatch`: Compares a word in a global variable to four; if it is not equal, it branches elsewhere.
- `reset_offset_if_low`: Subtracts 60 from a word at offset 350, compares it to 60, and if it is not higher, it writes zero to offset 354, writes 6427 to offset 31e, calls another routine, and writes zero to offset 350.
- `reset_offset_if_above_five_2`: Compares a word in d0 to 5; if it is higher, it branches to another routine, otherwise it clears a long variable at offset 354, stores 3879 in a long variable at offset 31e, and calls another routine.
- `reset_offset_if_above_ten`: Compares a word in d0 to 10; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 3882 in another global variable and calls another routine.
- `reset_offset_if_above_50`: Compares a word in d0 to 50; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 14125 in another global variable, and calls another routine.
- `reset_offset_if_greater_than_55`: Compares a 16-bit value in d0 to 55; if it is higher, it returns, otherwise it stores zero in a global variable, stores 3879 in another global variable, and calls another routine.
- `setup_offset_calculation_parameters`: Clears a 16-bit value in a global variable, sets a 16-bit value to 3879 in another global variable, and calls compute_wrapped_offset.
- `check_condition_and_reset_offset`: Compares a word at offset 25d0 to 5; if it is not equal, it branches elsewhere.
- `reset_offset_if_exceeds_sixty`: Subtracts 60 from a word in a global variable and compares it to 60; if the result is not negative, it clears a long variable at offset 354, stores 6417 in a word at offset 31e, calls another routine, and clears the word at offset 350 before returning, otherwise it returns.
- `reset_offset_if_above_five_alt`: Compares a word in d0 to 5; if it is higher, it branches to another routine, otherwise it clears a long variable at offset 354, stores 3891 in a long variable at offset 31e, and calls another routine.
- `reset_offset_if_above_ten_2`: Compares a word in d0 to 10; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 3894 in another global variable and calls another routine.
- `reset_offset_if_below_8`: Compares a word in d0 to 8; if it is lower, it branches to another routine, otherwise it clears a global variable, stores 6957 in another global variable, and calls another routine.
- `reset_offset_if_greater_than_55_2`: Compares a 16-bit value in d0 to 55; if it is higher, it returns, otherwise it stores zero in a global variable, stores 3891 in another global variable, and calls another routine.
- `prepare_wrapped_offset_parameters`: Clears a 16-bit word in a global variable, sets another 16-bit word in a global variable to 3891, and calls compute_wrapped_offset.
- `reset_offset_on_condition_2`: Compares a word at offset 25d0 to 6; if it is not equal, it compares a long variable at offset cf6 to 0, and if it is not 0, it clears a long variable at offset 354, stores 6715 at offset 31e, calls another routine, clears a long variable at offset 350, and returns.
- `reset_offset_on_threshold`: Subtracts 60 from a word at a global pointer and compares the result to 160.
- `calculate_divided_offset`: Compares d0 to 35, and if it is not less than, it extends d0 to a long, divides it by 5, writes the result to offset 354, writes 5159 to offset 31e, and calls another routine.
- `reset_offset_if_below_forty`: Compares a word in d0 to 40; if it is lower, it branches to another routine, otherwise it clears a long variable at offset 354, stores 6951 in a long variable at offset 31e, and calls another routine.
- `reset_offset_if_below_five`: Masks d0 with 15, compares it to 4, and if it is lower, branches to another routine; otherwise it clears a global variable, stores 6954 in another global variable and calls another routine.
- `reset_offset_if_below_8_2`: Compares a word in d0 to 8; if it is lower, it branches to another routine, otherwise it clears a global variable, stores 5159 in another global variable, and calls another routine.
- `reset_offset_if_less_than_12`: Compares a 16-bit value in d0 to 12; if it is lower, it returns, otherwise it stores zero in a global variable, stores 6957 in another global variable, and calls another routine.
- `set_offset_6957`: Clears a 16-bit word at offset 852 of a global structure, stores 6957 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_on_condition`: Compares a word at offset 25d0 to 7; if it is not equal, it compares a long variable at offset cf6 to 0, and if it is not 0, it writes 0 to offset 354 and 6427 to offset 31e of a global variable, calls another routine, and then writes 0 to offset 350.
- `reset_offset_if_below_eighty`: Loads a long variable from offset 350 into d0, subtracts 60, compares d0 to 80, and if d0 is below 80 it branches to another routine; otherwise it clears a long variable at offset 354, stores 6939 in a long variable at offset 31e, calls another routine, and clears a long variable at offset 350.
- `reset_offset_if_below_five_2`: Masks d0 with 15, compares it to 4, and if it is lower, branches to another routine; otherwise it clears a global variable, stores 6939 in another global variable and calls another routine.
- `reset_offset_if_above_55_2`: Compares a word in d0 to 55; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 10279 in another global variable, and calls another routine.
- `reset_offset_if_less_than_12_2`: Compares a 16-bit value in d0 to 12; if it is lower, it returns, otherwise it stores zero in a global variable, stores 5159 in another global variable, and calls another routine.
- `set_offset_5159`: Clears a 16-bit word at offset 852 of a global structure, stores 5159 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_if_not_one`: Compares a word at a global pointer to 1.
- `reset_offset_values_2`: Compares a word at a global pointer to zero, and if it is not zero, clears two words at offsets 354 and 31e, calls another routine, and then clears a word at offset 350.
- `reset_offset_if_below_sixty`: Loads a long variable from offset 350 into d0, subtracts 60, compares d0 to 60, and if d0 is below 60 it branches to another routine; otherwise it clears a long variable at offset 354, stores 6939 in a long variable at offset 31e, calls another routine, clears a long variable at offset 350, and clears a long variable at offset 25d6.
- `reset_offset_if_above_five`: Compares a word in d0 to 5; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 10279 in another global variable and calls another routine.
- `reset_offset_if_above_15`: Compares a word in d0 to 15; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 1322 in another global variable, and calls another routine.
- `reset_offset_if_bit_two_cleared_2`: Tests bit 2 of d0; if it is cleared, it returns, otherwise it stores zero in a global variable, stores 10285 in another global variable, and calls another routine.
- `set_offset_10285`: Clears a 16-bit word at offset 852 of a global structure, stores 10285 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `verify_offset_range_and_reset`: Compares a word at offset 25d0 to 0; if it is not equal, it branches elsewhere.
- `reset_offset_if_bit_two_cleared_3`: Tests bit 2 of d0; if it is cleared, it returns, otherwise it stores zero in a global variable, stores 15390 in another global variable, and calls another routine.
- `set_offset_15390`: Clears a 16-bit word at offset 852 of a global structure, stores 15390 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_if_above_three`: Compares a word at offset 25d0 to 3; if it is not equal, it loads a word from offset 350 into d0, subtracts 60, and if the result is not less than 25, it clears a long variable at offset 354, stores 6715 at offset 31e, calls another routine, clears a long variable at offset 350, and returns.
- `reset_offset_if_above_five_alt2`: Compares a word in d0 to 5; if it is lower, it branches to another routine, otherwise it clears a long variable at offset 354, stores 1316 in a long variable at offset 31e, and calls another routine.
- `reset_offset_if_below_ten`: Compares a word in d0 to 10; if it is lower, it branches to another routine, otherwise it clears a global variable, stores 1319 in another global variable and calls another routine.
- `reset_offset_if_above_5`: Compares a word in d0 to 5; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 1328 in another global variable, and calls another routine.
- `reset_offset_if_less_than_20`: Compares a 16-bit value in d0 to 20; if it is lower, it returns, otherwise it stores zero in a global variable, stores 1325 in another global variable, and calls another routine.
- `set_offset_1316`: Clears a 16-bit word at offset 852 of a global structure, stores 1316 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_on_bounds`: Compares a word at offset 25d0 to 4; if it is not equal, it loads a word from offset 350 into d0, subtracts 60, and if the result is not lower than 60, it writes 0 to offset 354 and 6427 to offset 31e of a global variable, calls another routine, and then writes 0 to offset 350.
- `reset_offset_if_above_4`: Compares a word in d0 to 4; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 42 in another global variable, and calls another routine.
- `reset_offset_if_greater_than_55_3`: Compares a 16-bit value in d0 to 55; if it is higher, it returns, otherwise it stores zero in a global variable, stores 1331 in another global variable, and calls another routine.
- `set_offset_1328`: Clears a 16-bit word at offset 852 of a global structure, stores 1328 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_on_large_value`: Compares a word at a global pointer to 5.
- `reset_offset_if_below_three`: Masks d0 with 7, compares it to 2, and if it is lower, branches to another routine; otherwise it clears a global variable, stores 39 in another global variable and calls another routine.
- `reset_offset_if_above_3`: Compares a word in d0 to 3; if it is higher, it branches to another routine, otherwise it clears a global variable, stores 42 in another global variable, and calls another routine.
- `update_offset_forty_two`: Compares a word in d0 to 6, and if it is not less than 6, clears a word at offset 884 of a global variable, stores 45 at offset 798 of a global variable, and calls another routine.
- `set_offset_42`: Clears a 16-bit word at offset 852 of a global structure, stores 42 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_if_threshold_met`: Compares a word at a global pointer to 6, and if it is not 6, compares a word at offset 350 minus 60 to 100; if the latter is not below 100, it clears a word at offset 354, stores 7459 at offset 31e, calls another routine, and clears a word at offset 350.
- `reset_offset_if_below_five_3`: Masks d0 with 15, compares it to 4, and if it is lower, branches to another routine; otherwise it clears a global variable, stores 48 in another global variable and calls another routine.
- `reset_values_on_threshold`: Compares a word in d0 to 8, and if it is less than 8, otherwise it clears a word at offset 876 of a global variable, stores 51 at offset 798 of a global variable, and calls another routine.
- `update_offset_fifty_one`: Compares a word in d0 to 12, and if it is not less than 12, clears a word at offset 884 of a global variable, stores 54 at offset 798 of a global variable, and calls another routine.
- `set_offset_51`: Clears a 16-bit word at offset 852 of a global structure, stores 51 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `reset_offset_if_below_hundred`: Loads a long variable from offset 350 into d0, subtracts 60, compares d0 to 100, and if d0 is below 100 it branches to another routine; otherwise it clears a long variable at offset 354, stores 6939 in a long variable at offset 31e, calls another routine, and clears a long variable at offset 350.
- `reset_offset_if_below_three_2`: Masks d0 with 7, compares it to 2, and if it is lower, branches to another routine; otherwise it clears a global variable, stores 2600 in another global variable and calls another routine.
- `reset_values_on_threshold_alt`: Compares a word in d0 to 4, and if it is less than 4, otherwise it clears a word at offset 876 of a global variable, stores 2602 at offset 798 of a global variable, and calls another routine.
- `update_offset_two_thousand_six_hundred`: Compares a word in d0 to 6, and if it is not less than 6, clears a word at offset 884 of a global variable, stores 2602 at offset 798 of a global variable, and calls another routine.
- `set_offset_2602`: Clears a 16-bit word at offset 852 of a global structure, stores 2602 at offset 798 of the same structure, and calls compute_wrapped_offset.
- `update_offset_on_match_variant_1`: Compares a word at offset d0c to 1; if it is not equal, it branches elsewhere.
- `reset_sprite_parameters`: Sets a 16-bit value in a global variable to zero, sets another 16-bit value in a global variable to eleven, and branches to another routine.
- `increment_and_check_timer`: Increments a 16-bit counter in a global variable and checks if it has reached 120; if so, it sets a status byte to 1 and clears two other 16-bit values.
- `update_status_on_conditions`: Writes zero to a long variable at offset 31c.
- `update_status_via_pointer`: Compares a word at a global pointer to 65535, and if it is not 65535, compares a word at offset 2 in d02 to 1; if both conditions are met, it stores 88 at offset 14 of d0e and branches to another routine.
- `reset_state_on_zero_2`: Compares a 16-bit value at a pointer to zero; if it is not zero, it writes forty to a location in a global variable and writes zero to two other global variables, otherwise it returns.
- `reset_state_on_zero`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it writes thirty to a location in d0, writes zero to a global variable at offset 354, and writes zero to a global variable at offset 350, otherwise it returns.
- `reset_state_on_condition`: Compares a global variable to one and another to sixty-four, and if both conditions are met, writes thirty-eight to one global variable and zero to two others.
- `conditional_reset_state_2`: Checks if four global variables match specific values and, if so, writes thirteen to one global variable and zero to two others.
- `conditional_reset_state`: Checks if four specific global variables match certain values or conditions, and if so, writes twenty-three to one global variable and clears two others.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `check_match_and_update_status`: Compares a word at offset 3372 to 1; if equal, it compares a word from offset 3304 to a word at offset 3276.
- `reset_sprite_parameters_2`: Sets a 16-bit value to zero at a specific offset in a global structure, sets a 16-bit value to eleven at another offset, and branches to another routine.
- `set_offset_and_check_status`: Writes 6424 to a word at offset 31e and calls another routine.
- `validate_and_set_status_word`: Compares a 16-bit value at a pointer in a global variable to 65535 and a 16-bit value at a second pointer to 1; if both conditions are met, it writes 88 to a 16-bit value at a third pointer and branches to another routine, otherwise it branches to another routine.
- `reset_counters_on_zero_match`: Compares a 16-bit value at a pointer in a global variable to zero; if it matches, it branches to another routine, otherwise it stores 30 in a global variable and clears two 16-bit values at offsets 348 and 352 of a global structure before returning.
- `compare_and_reset_counters`: Compares a 16-bit value at a global variable to 1 and a 16-bit value at another global variable to 64; if the first is 1 or the second is less than 64, it returns, otherwise it writes 38 to a global variable and 0 to two other global variables and returns.
- `set_state_to_thirteen_2`: Checks several global variables and, if they meet specific conditions, writes eleven to one global variable and zero to two others; otherwise, it writes thirteen to the first variable and zero to the others.
- `set_state_to_thirteen`: Assigns the value thirteen to one global variable, then clears the two adjacent global variables.
- `update_offset_on_match_variant_3`: Compares a word at offset d0c to 1; if it is not equal, it branches elsewhere.
- `reset_sprite_parameters_3`: Sets a 16-bit value to zero and another 16-bit value to eleven in global variables, then branches to another routine.
- `set_offset_and_validate_state`: Writes 6433 to a word at offset 31e and calls another routine.
- `validate_and_set_status_word_2`: Checks if a 16-bit value at a specific memory location is equal to sixty-five thousand five hundred thirty-five and if a 16-bit value in a register is equal to one; if both conditions are met, it writes eighty-eight to a word at a specific memory location and branches to another routine, otherwise it returns.
- `reset_state_on_dual_threshold`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it writes forty to a location in d0, writes zero to a global variable at offset 354, and writes zero to a global variable at offset 350.
- `reset_state_on_threshold`: Compares a 16-bit value in a global variable against 1 and 64; if the value is between 2 and 63 inclusive, it writes 38 to a global variable and clears two other global variables, otherwise it returns.
- `reset_state_to_zero_2`: Checks several global variables and, if they meet specific conditions, writes ten to one global variable and zero to two others; otherwise, it writes zero to the first variable and zero to the others.
- `reset_state_to_zero`: Clears three global variables by writing zero to each.
- `compare_values_and_branch`: Compares a word at offset d0c to 1; if it is not equal, it branches elsewhere.
- `update_pointer_value`: Compares a word at a longword location to 1, then either writes 0 or 1248 to a longword location and branches.
- `validate_tile_coordinates_and_update`: Checks if a pointer is non-zero, then calculates and stores two 16-bit values derived from bit-shifting two other values from the same pointer structure.
- `validate_and_set_status_value`: Compares a word at a global pointer to sixty-five thousand five hundred thirty-five and a word at another global pointer to one; if both conditions are met, it writes eighty-eight to a word at a third global pointer and branches to another routine, otherwise it branches to another routine.
- `reset_state_on_threshold_alt_2`: Compares a 16-bit value at a pointer to zero; if it is not zero, it compares a 16-bit value at another pointer to zero; if that is not zero, it writes thirty to a location in a global variable and writes zero to two other global variables, otherwise it returns.
- `reset_state_on_threshold_alt`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it compares a 16-bit value at a pointer in d1 to sixty-four; if that value is not less than sixty-four, it writes thirty-one to a location in d0, writes zero to a global variable at offset 354, and writes zero to a global variable at offset 350, otherwise it returns.
- `increment_wrapped_counter`: Adds a value from a global variable to another, and if a specific flag is set, adds thirty-two; it then wraps the value if it exceeds one hundred twenty-eight and increments a secondary counter that wraps at eight.
- `decrement_and_wrap_counter`: Checks several global variables and, if certain conditions are met, decrements a 16-bit counter in a global variable and wraps it from zero to seven; otherwise, it writes ten to one global variable and zero to two others.
- `increment_global_variable`: Increments a 16-bit value in a global variable by one.
- `store_and_clear_globals`: Writes the value ten into one global variable and clears two other global variables.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `validate_and_reset_state_2`: Compares a word at offset 3340 to 1; if it is 1, it compares a word from offset 3288 to a word at offset 3276, and if the former is not greater, it clears a word at offset 1364 and sets a word at offset 3310 to 21 before branching to another routine.
- `reset_sprite_parameters_4`: Sets a word in a global variable to zero, sets another word in a global variable to eleven, and branches to another routine.
- `update_status_and_jump`: Writes the word 6430 to a long variable at offset 31e, calls another routine, and then checks if words at offsets 5c30 and 352 are equal to 1 and if the word in d02 is 1; if all conditions are met, it writes 89 to offset d0e and jumps to another routine.
- `update_status_via_pointer_2`: Compares a word at a global pointer to 65535, and if it is not 65535, compares a word at offset 2 in d02 to 1; if both conditions are met, it stores 80 at offset 14 of d0e and branches to another routine.
- `reset_threshold_and_counters`: Compares a 16-bit value at a global pointer to zero; if not zero, it writes 40 to a global variable and 0 to two other global variables, then checks if a 16-bit value at a different global pointer is zero to decide whether to write 30 to the first global variable and 0 to the other two before returning.
- `check_threshold_and_reset`: Compares a 16-bit value at offset 6 of a pointer to 1 and a 16-bit value at offset 22 of a pointer to 64; if the first is not 1 or the second is not less than 64, it writes 38 to offset 14 of a pointer, 0 to offset 884 and 0 to offset 880, then returns.
- `update_state_constants`: Compares several 16-bit values at offsets of a pointer in a3; if specific conditions are met, it writes 21 to a global variable and clears two other global variables, otherwise it writes 23 to the first global variable and clears the other two.
- `set_and_clear_globals`: Stores a fixed value into a global variable and clears two other global variables.
- `validate_and_reset_offset_data`: Compares a word at offset 3292 to 1; if not equal, it branches elsewhere, otherwise it loads a word from offset 3288 into a register, compares it to a word at offset 3276, and if the value is not greater than the comparison, it clears a 16-bit value at offset 852 and sets a 16-bit value at offset 358 to 21 before branching to another routine.
- `reset_sprite_parameters_5`: Sets a 16-bit value in a global variable to zero, sets a 16-bit value in another global variable to eleven, and branches to another routine.
- `update_status_and_branch`: Writes the word 6436 to a long variable at offset 31e, calls another routine, and then checks if words at offsets 5c30 and 352 are equal to 1 and if the word in d02 is 1; if all conditions are met, it writes 89 to offset d0e and branches to another routine.
- `validate_and_set_status_value_2`: Checks if a word at a global pointer is not equal to sixty-five thousand five hundred thirty-five and if a word in a register is not equal to one; if both conditions are met, it writes eighty-eight to a word at a global offset and branches to another routine, otherwise it returns.
- `reset_and_update_counters`: Compares a 16-bit value at a pointer in d0 to zero; if not zero, it writes 40 to a 16-bit value at a pointer in d0 and clears two 16-bit values at pointers in d1 and d2, then compares a 16-bit value at a pointer in d0 to zero to decide whether to write 30 to that same pointer and clear the same two 16-bit values before returning.
- `validate_and_reset_counters`: Compares a 16-bit value at an offset of a pointer in d0 to 1 and a 16-bit value at an offset of a pointer in d1 to 64; if these conditions are met, it stores 38 into a 16-bit location at an offset of a pointer in d0, and clears two 16-bit global variables, otherwise it returns.
- `check_and_set_state`: Compares several 16-bit values at offsets of a pointer in a3; if certain conditions are met, it writes 20 to a global variable and clears two others, otherwise it clears the three global variables.
- `reset_state_to_zero_alt`: Clears three global variables by writing zero to each.
- `update_offset_on_match`: Compares a word at offset d0c to 1; if it is not equal, it branches elsewhere, otherwise it loads a word from offset ce8 into d0, compares d0 to the word at offset cec, and if d0 is not less than that value, it clears a long variable at offset 354, stores 11 at offset d0e, and branches to another routine.
- `set_pointer_value`: Writes 1280 to a longword location, compares a word at a longword location to 1, and then either writes 1240 to the longword location or branches.
- `update_tile_and_state_parameters`: Calculates tile-based coordinates by shifting values from a pointer, calls another routine twice to validate a range, and updates a series of constant values in a global variable based on a state index.
- `check_and_set_threshold_value`: Compares a word at a pointer in a global variable to sixty-five thousand five hundred thirty-five and compares a word in a register to one; if both conditions are met, it writes eighty-eight to a word at an offset in a global variable and branches to another routine, otherwise it returns.
- `reset_threshold_counters`: Checks if a 16-bit value at a global pointer is zero; if not, it writes 40 to a global variable and clears two 16-bit values at specific offsets in another global structure, otherwise it writes 30 to the first variable and clears the same two 16-bit values before returning.
- `reset_state_on_threshold_v2`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it compares a 16-bit value at a pointer in d1 to sixty-four; if that value is not less than or equal to sixty-four, it writes thirty-two to a location in d0, writes zero to a global variable at offset 354, and writes zero to a global variable at offset 350, otherwise it returns.
- `update_counters_with_loop`: Subtracts a 16-bit value from a global variable, adds 32 to that variable if a 16-bit value at offset 348 of a pointer is 1, and enters a loop that subtracts 128 from the variable and increments a second global variable until the variable is less than 128, resetting the second variable to 0 if it reaches 8.
- `decrement_state_counter`: Compares several 16-bit values at offsets of a pointer in a3; if conditions are met, it checks a global variable and either decrements it (wrapping to 7 if it reaches -1) or increments it, otherwise it writes 20 to a global variable and clears two others.
- `increment_global_word`: Increments a 16-bit global variable by one and then returns.
- `set_clear_globals`: It writes the value 20 into one global variable and clears two adjacent global variables, then returns.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `branch_on_zero`: Compares a word at a longword location to 0 and branches if it is equal, otherwise it branches to another location.
- `compare_and_branch_threshold`: Compares a 16-bit value at a location pointed to by a register to 16, then branches to another routine if it is less than 16, otherwise it branches to a different location.
- `branch_on_threshold`: Compares a long value at offset 212 to 65520, and if it is not greater than 65520, branches to another routine; otherwise it branches to a third routine.
- `check_threshold_and_clear_offsets`: Compares a long variable at offset 31e to 7680; if it is not equal, it branches to another routine; otherwise it checks if a long variable at offset 354 is not zero, and if it is not zero it branches to another routine; otherwise it stores 38 in a long variable at offset d0e, clears a long variable at offset 354, and clears a long variable at offset 350.
- `decrement_offset`: Decrements a 16-bit word at offset 852 of a global structure and calls compute_wrapped_offset.
- `update_pointer_thresholds`: Compares a word at offset 31e to 10240; if it is not equal, it branches to another routine.
- `increment_global_pointer`: Increments a 16-bit word at offset 354 of a structure pointed to by a3, calls another routine, then returns.
- `update_pointer_logic`: Compares a word at offset 876 of a global variable to 6, and if it is less than 6, otherwise it compares that same word to 0; if the word is not 0, it stores 7680 at offset 798 of a global variable, stores 11 at offset 876 of a global variable, and calls another routine.
- `decrement_global_pointer`: Decrements a 16-bit word at offset 354 of a structure pointed to by a3, calls another routine, then returns.
- `reset_pointer_if_not_eleven`: Compares a word at offset 884 of a global variable to 11, and if it is not equal to 11, stores 10240 at offset 798 of a global variable, clears the word at offset 884 of the global variable, and calls another routine.
- `increment_global_pointer_alt`: Adds one to a 16-bit word at offset 354 of a structure pointed to by a3, calls another routine, then returns.
- `store_byte_values`: It writes the value 31 into one global variable and then writes 0 into two adjacent global variables, then returns.
- `set_and_clear_globals_2`: Stores the value 32 into a global variable and resets two adjacent global variables to zero.
- `calculate_scaled_offset_2`: Loads a longword from a global variable, divides it by 1280, and if the result is less than 0, adds 36 to it; then it compares this value to 12, and if it is not less than 12, subtracts 12 from it and adds 1280 to a value at offset 798 of a global variable, otherwise it stores 4800 at offset 798 of a global variable.
- `set_global_pointer`: Moves a 16-bit word from d0 into offset 354 of a structure pointed to by a3, then calls another routine and returns.
- `update_control_state_a`: Writes 400 to a long variable, calls another routine, and then checks if three specific conditions are met by comparing values at a global variable, an offset in a pointer in d0, and a word in d02; if all are met, it writes 89 to d0e and branches to another routine, otherwise it branches to another location.
- `check_status_and_set_value`: Compares a word at a global variable to 65535 and branches if not equal; if the word at offset 2 in d0 is 1, it writes 88 to offset 14 of d0 and branches to another routine, otherwise it branches to another routine.
- `reset_threshold_values`: Compares a word at an offset of a pointer in d0 to zero, and if it is not zero, stores 30 at an offset of a pointer in d0, stores 0 at a global variable, and stores 0 at another global variable.
- `adjust_threshold_value`: Adds a word from a pointer in d0 to a global variable, then checks if the result is less than 128; if so, it subtracts 128 and increments another global variable, otherwise it resets that global variable to zero.
- `reset_dual_threshold_state`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it writes eleven to that location, writes zero to two global variables, and otherwise returns.
- `reset_state_on_dual_threshold_v4`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it writes eleven to a location in d0, writes zero to a global variable at offset 354, and writes zero to a global variable at offset 350.
- `check_value_limit`: Compares a 16-bit value at offset 22 of a pointer to 128 and returns if the value is greater than or equal to 128; otherwise it returns.
- `increment_global_counter`: Compares a 16-bit global variable to zero; if it is not zero, it increments the variable and branches to another routine if the value reaches 8, otherwise it returns.
- `clear_global_word`: Sets a 16-bit global variable to zero and then returns.
- `clear_and_set_globals`: It writes the value 33 into one byte of a global variable and then sets two adjacent global variables to zero.
- `update_control_state_b`: Writes 384 to a long variable, calls another routine, and then checks if three specific conditions are met by comparing values at a global variable, an offset in a pointer in d0, and a word in d02; if all are met, it writes 89 to d0e and branches to another routine, otherwise it branches to another location.
- `validate_and_set_status_value_3`: Checks if a pointer in a global variable is not equal to all ones and if a value in a second global variable is equal to one; if both conditions are met, it writes 88 to a third global variable and branches to another routine, otherwise it returns.
- `reset_threshold_and_clear_counters`: Compares a word at a pointer in d0 to zero; if not zero, it writes 30 to the location pointed to by d0, clears two global variables at offsets 350 and 354, and otherwise branches to another routine.
- `subtract_threshold_value`: Subtracts a word from a pointer in d0 from a global variable, then checks if the result is less than 128; if so, it subtracts 128 and increments another global variable, otherwise it resets that global variable to zero.
- `reset_state_on_dual_threshold_v5_2`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it writes twenty-one to that location and clears two global variables, otherwise it returns.
- `reset_state_on_dual_threshold_v5`: Compares a 16-bit value at a pointer in d0 to zero; if it is not zero, it writes twenty-one to a location in d0, writes zero to a global variable at offset 354, and writes zero to a global variable at offset 350.
- `check_global_word_threshold`: Compares a 16-bit global variable to 128; if the value is greater than or equal to 128, it branches to another routine, otherwise it returns.
- `increment_and_check_global_word`: Compares a 16-bit global variable to zero; if it is not zero, it increments the variable and then compares it to eight, returning if the new value is eight, otherwise it returns.
- `clear_global_word_2`: Sets a 16-bit global variable to zero and then returns.
- `set_and_clear_globals_3`: This routine zeroes two global variables and assigns the value 34 to a third global variable, then returns.
- `update_state_with_threshold`: Moves 1048 into offset 31e of a structure, calls another routine, and if a 16-bit value at offset d14 is not less than 16, checks if a 16-bit value at offset 354 is 4; if not, it increments a 16-bit value at offset 350 and resets it to 0 if it reaches 2, then increments offset 354.
- `decrement_state_and_counter`: Compares a global variable to two; if it is not two, it increments a second global variable and checks if that variable equals two, in which case it clears it and decrements the first global variable, otherwise it resets the second global variable to zero and decrements the first.
- `reset_state_on_pointer_value`: Writes thirty-five to a location in d0, writes four to a global variable at offset 354, and writes zero to a global variable at offset 350.
- `reset_values_on_zero`: Compares a 16-bit value at offset 10 of a pointer to 0; if it is not 0, it writes 30 to offset 14 of a pointer, 0 to offset 884 and 0 to offset 880, then returns.
- `reset_counters_on_zero`: Compares a 16-bit global variable to zero; if it is zero, it branches to another routine, otherwise it clears three global variables and returns.
- `check_and_reset_globals`: Compares a 16-bit global variable to 128; if it is greater than 128, it returns, otherwise it clears three global variables and returns.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_with_high_threshold`: Moves 3864 into offset 31e of a structure, calls another routine, and if a 16-bit value at offset d14 is not greater than -16, checks if a 16-bit value at offset 354 is 4; if not, it increments a 16-bit value at offset 350 and resets it to 0 if it reaches 2, then increments offset 354.
- `update_counter_and_reset_limit`: Compares a 16-bit value at an offset in a structure to two; if it is two, it increments a different 16-bit value in the same structure and checks if that new value is also two, in which case it clears the second value and decrements the first value before returning; otherwise, it returns.
- `update_state_values`: If a 16-bit value at a specific offset in a structure is zero, it writes 37 to that offset, 4 to a global variable, and 0 to another global variable; otherwise, it writes 81 to the offset, 0 to the first global variable, and 0 to the second global variable.
- `reset_counters_on_zero_check`: Compares a 16-bit word at a pointer in a longword to zero; if it is zero, it branches to another routine, otherwise it stores 30 in a 16-bit word at a pointer in a longword and clears two 16-bit values at specific memory locations before returning.
- `reset_counters_if_not_zero`: Compares a 16-bit value at a pointer in a longword to zero; if the value is zero, it clears three 16-bit values at specific memory locations, otherwise it returns.
- `reset_global_counters`: Compares a 16-bit value at a pointer in a longword to 128; if the value is greater than 128, it returns, otherwise it clears three 16-bit values at specific global memory locations and returns.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_with_fixed_threshold`: Moves 5144 into offset 31e of a structure, calls another routine, and if a 16-bit value at offset 354 is not 4, it increments a 16-bit value at offset 350 and resets it to 0 if it reaches 2, then increments offset 354.
- `update_structure_parameters`: Compares a 16-bit value at an offset in a structure against 16 and -15; it writes specific values to that offset and two global variables based on these comparisons, and if a separate global variable is non-zero, it writes 81 to the offset and 0 to the two global variables before returning.
- `reset_state_on_non_zero_check`: Compares a 16-bit value at a pointer in a data structure to zero; if not zero, it sets a 16-bit value to 30, clears two global variables at offsets 350 and 354, and returns.
- `clear_three_global_variables`: Compares a 16-bit value at a specific memory location to zero and, if it is not zero, clears three global variables by setting them to zero; otherwise it returns.
- `reset_globals_on_threshold`: Compares a 16-bit value at a memory location to 128, and if it is greater, clears three global variables by setting them to zero; otherwise it returns.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `update_control_state_c`: Writes 36 to a long variable, calls another routine, and then checks if three specific conditions are met by comparing values at a global variable, an offset in a pointer in d0, and a word in d02; if all are met, it writes 89 to d0e and branches to another routine, otherwise it branches to another location.
- `validate_and_set_status_word_3`: Checks if a word at a specific memory location is not equal to sixty-five thousand five hundred thirty-five and if a word at another location is not equal to one; if both conditions are met, it writes eighty-eight to a third memory location and branches to another routine, otherwise it returns.
- `reset_global_counters_conditionally`: Compares a 16-bit value at a global variable to zero; if it is not zero, it writes 40 to a global variable and clears two other global variables, otherwise it writes 30 to a global variable and clears the same two global variables.
- `modify_global_counters`: Compares a 16-bit value to 128; if it is less than 128, it returns, otherwise it clears two global variables and writes 35 to a specific offset in a structure.
- `clear_three_globals`: Compares a 16-bit value at offset 6 of a pointer to 0; if it is not 0, it writes 0 to offset 884, 0 to offset 880 and 0 to offset 14 of a pointer, then returns.
- `set_globals_on_threshold_2`: Compares a 16-bit global variable to 16; if it is less than 16, it branches to another routine, otherwise it writes 31 to a global variable and clears two others.
- `set_globals_on_threshold`: Compares a 16-bit value at a memory location to 65520, and if it is not greater, sets a global variable to 32 and clears two other global variables; otherwise it returns.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `validate_and_reset_structure`: Writes 36 to offset 31e of a structure, calls another routine, and then clears offset 31c; if a 16-bit value at offset cf6 is 0, it writes 38 to offset d0e, 0 to offset 354, and 0 to offset 350; otherwise, if offset dbe is 1, it writes 80 to offset d0e, 0 to offset 354, and 0 to offset 350.
- `validate_and_reset_state`: Compares a 16-bit value at offset 3126 of a pointer to 3, a 16-bit value at offset 22 of a pointer to 32, and a 16-bit value at offset 3072 of a pointer to 1; if all conditions are met, it writes 50 to offset 14 of a pointer, 0 to offset 884 and 0 to offset 880, then returns.
- `set_globals_on_match`: Compares a 16-bit global variable to 1; if it is equal, it branches to another routine, otherwise it writes 74 to a global variable and clears two others.
- `reset_globals_on_match`: Compares a 16-bit value at a memory location to 1, and if it is not equal, sets a global variable to 64 and clears two other global variables; otherwise it returns.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_on_match_2`: Writes 24 to offset 31e of a structure, calls another routine, and if a 16-bit value at offset cf6 is 0, it writes 52 to offset d0e, 2 to offset 354, and 0 to offset 350; otherwise, if offset dba is 1, it writes 62 to offset d0e, 0 to offset 354, and 0 to offset 350.
- `reset_state_on_match`: Compares a 16-bit value at a global variable to 1; if it is not 1, it returns, otherwise it moves 72 into register d0 and 0 into two different global variables.
- `update_state_from_conditions`: Compares a 16-bit value at a global variable to 32, then checks if another global variable is less than 3 and a third is equal to 1; if all conditions are met, it stores 52, 2, and 0 into three global variables before returning.
- `decrement_and_track_global`: Subtracts a 16-bit value from a global variable, and if the result is less than 128, subtracts 128 from that variable and increments another global variable until it reaches 4, at which point it resets to zero.
- `empty_stub_16`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_complex`: Writes 5120 into offset 31e of a structure, calls another routine, and if offset cf6 is 0, it writes 38 to offset d0e, 0 to offset 354, and 0 to offset 350; if offset dba is 1, it adds a 16-bit value from offset d14 to offset 350 and uses a loop to wrap the value around 128 while incrementing offset 354; if offset dbe is 1, it writes 66 to offset d0e, 2 to offset 354, and 0 to offset 350.
- `update_state_from_conditions_2`: Compares a 16-bit value at a global variable to 32, then checks if another global variable is less than 3 and a third is not equal to 1; if all conditions are met, it stores 65, 2, and 0 into three global variables before returning.
- `set_multiple_globals`: Assigns the value 67 to one global variable, the value 2 to a second global variable, and zero to a third global variable, then returns.
- `increment_and_track_global`: Adds a 16-bit value to a global variable, and if the result is less than 128, subtracts 128 from that variable and increments another global variable until it reaches 8, at which point it resets to zero.
- `empty_stub_17`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_18`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_logic`: Stores 6400 in a word at offset 31e of a structure in d3, calls another routine, and if a word at offset cf6 is zero, sets values at offsets d0e, 354, and 350; otherwise, it checks offsets dbc and dbe to set specific values at offsets d0e, 354, and 350.
- `update_state_from_conditions_3`: Compares a 16-bit value at a global variable to 32, then checks if another global variable is less than 3 and a third is not equal to 1; if all conditions are met, it stores 75, 2, and 0 into three global variables before returning.
- `set_globals_77_2`: Stores the value 77 into a global variable and sets two other global variables to 2 and 0.
- `decrement_and_track_global_eight`: Subtracts a 16-bit value from a global variable, and if the result is less than 128, subtracts 128 from that variable and increments another global variable until it reaches 8, at which point it resets to zero.
- `empty_stub_19`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_20`: Empty routine: returns to the caller immediately without doing anything.
- `update_state_alt`: Stores 1304 in a word at offset 31e of a structure in d3, calls another routine, and if a word at offset cf6 is zero, sets values at offsets d0e, 354, and 350; otherwise, it checks offset dba to set values at d0e, 354, and 350, or checks offset dbe to set values at d0e, 354, and 350.
- `reset_state_on_match_alt`: Compares a 16-bit value at a global variable to 1; if it is not 1, it returns, otherwise it moves 73 into register d0 and 0 into two different global variables.
- `update_state_on_match`: Checks if a 16-bit global variable is equal to 1; if so, it stores 82, 4, and 0 into three global variables before returning.
- `increment_and_track_global_four`: Adds a 16-bit value to a global variable, and if the result is less than 128, subtracts 128 from that variable and increments another global variable until it reaches 4, at which point it resets to zero.
- `empty_stub_21`: Empty routine: returns to the caller immediately without doing anything.
- `increment_counter_2d00`: Stores 11520 in a word at offset 31e of a structure in d3, calls another routine, and if a word at offset cf6 is zero, sets values at offsets d0e, 354, and 350, then increments the word at offset 350 if it is not equal to 2.
- `increment_and_check_limit`: Sets a global variable to 0, increments another global variable by 1, and returns if that variable is equal to 3.
- `set_globals_60_0`: It stores the value 60 into a global variable and sets two other global variables to 0.
- `initialize_and_increment_counter`: Writes 11520 to a longword in a global variable, calls another routine, and if a specific longword in a global variable is not zero, it sets two other global variables to zero and checks if the second one is two; otherwise, it increments the second global variable by one and returns.
- `decrement_and_check_limit`: Sets a global variable to 0, decrements another global variable by 1, and returns if that variable is equal to 65535.
- `set_globals_51_0`: Stores the value 51 into a global variable and sets two other global variables to 0.
- `increment_counter_2d12`: Stores 11538 in a word at offset 31e of a structure in d3, calls another routine, and if a word at offset cf6 is zero, sets values at offsets d0e, 354, and 350, then increments the word at offset 350 if it is not equal to 2.
- `increment_and_check_counter`: Clears a 16-bit word at a specific memory location, increments a 16-bit word at a subsequent location, and branches to another routine if that value reaches 3; otherwise it returns.
- `set_globals_60_0_2`: Stores the value 60 into a global variable and sets two other global variables to 0.
- `update_state_and_increment_counter`: Stores the constant 11538 into a longword at a global variable, calls another routine, and if a longword at a global variable is zero, it sets two 16-bit values at specific global offsets to 38 and 0 respectively, otherwise it increments a 16-bit value at a global offset and returns.
- `decrement_and_check_counter`: Writes zero to a 16-bit word in a global variable, decrements a 16-bit word at an offset of 4 in that same variable, and branches to another routine if the decremented value is equal to 65535; otherwise it returns.
- `set_globals_81_0`: Stores the value 81 into a global variable and sets two other global variables to 0.
- `increment_counter_2d09`: Stores 11529 in a word at offset 31e of a structure in d3, calls another routine, and if a word at offset cf6 is zero, sets values at offsets d0e, 354, and 350, then increments the word at offset 350 if it is not equal to 2.
- `reset_and_check_counter`: Clears a 16-bit word at a specific memory location, increments a 16-bit word at the following offset, and branches to another routine if that value equals 3; otherwise it returns.
- `set_globals_70_0`: Stores the value 70 into a global variable and sets two other global variables to 0.
- `update_state_and_increment_counter_2`: Writes 11513 to a longword in a global variable, calls another routine, and if a longword in a global variable is zero, it sets two 16-bit words in a global variable to 38 and 0 respectively, otherwise it increments a 16-bit word at offset 4 of a global variable and returns.
- `reset_and_check_counter_2`: Writes zero to a 16-bit word in a global variable, decrements a 16-bit word at an offset of 4 in the same structure, and branches to another routine if the decremented value equals 65535; otherwise it returns.
- `set_globals_51_0_alt`: Stores the value 51 into a global variable and sets two other global variables to 0.
- `increment_counter_2d1b`: Stores 11547 in a word at offset 31e of a structure in d3, calls another routine, and if a word at offset cf6 is zero, sets values at offsets d0e, 354, and 350, then increments the word at offset 350 if it is not equal to 2.
- `increment_and_check_counter_2`: Clears a 16-bit word at a global variable, increments a 16-bit word at another global variable, and branches if that value is 3; otherwise it returns.
- `set_globals_70_0_alt`: Stores the value 70 into a global variable and sets two other global variables to 0.
- `update_state_and_increment_counter_3`: Stores the constant 11547 into a longword at a global variable, calls another routine, and if a longword at a global variable is zero, it sets two global words to 38 and 0 respectively; otherwise, it sets a global word to 38 and increments another global word before returning.
- `decrement_and_check_counter_2`: Clears a 16-bit word at a global variable, decrements a 16-bit word at another global variable, and branches if that value is 65535; otherwise it returns.
- `set_globals_81_0_alt`: Stores the value 81 into a global variable and sets two other global variables to 0.
- `update_timer_and_check_counter`: Stores the word value twelve thousand eight hundred in a longword at offset three hundred eighteen, calls another routine, and if a word at offset eight hundred forty-eight is two, it returns; otherwise it increments the word at that offset and returns.
- `reset_and_check_counter_3`: Clears a 16-bit value in a global variable, increments a 16-bit value at an offset of 4 from that same variable, and branches to another routine if the incremented value equals 3; otherwise it returns.
- `set_global_51_and_clear_two_vars`: Stores the value 51 into a global variable and sets two other global variables to 0, then returns.
- `update_and_increment_counter`: Writes the constant 12800 to a longword in a global variable, calls another routine, and then checks if a 16-bit word at a global variable is equal to 2; if not, it increments that word by 1 and returns, otherwise it branches to another routine.
- `reset_and_check_counter_4`: Writes zero to a longword at a global variable, decrements a 16-bit word at a subsequent offset, and branches to another routine if that word equals sixty-five thousand five hundred thirty-five; otherwise it returns.
- `store_value_0x28`: Stores the constant value 40 into a global variable and then clears two adjacent global variables to zero.
- `update_timer_and_reset_counters`: Stores the word value twelve thousand eight hundred nine in a longword at offset three hundred eighteen, calls another routine, and if a word at offset eight hundred twenty-two is not zero, it branches; otherwise it writes values to three different global variables, checks if a word at offset eight hundred forty-eight is two, and either branches or increments that word before returning.
- `increment_and_check_counter_3`: Clears a 16-bit value at a global variable location, increments a 16-bit value at a second global variable location, and branches to another routine if the second value equals three; otherwise it returns.
- `set_globals_60_0_alt`: Stores the value 60 into a global variable and sets two other global variables to 0.
- `update_state_and_increment_counter_4`: Writes 12817 to a memory location, calls another routine, and if a 16-bit value at a global variable is not zero, writes 38 to a memory location, clears two 16-bit values at specific offsets in a global variable, and increments a 16-bit value at offset 4 of that same global variable before returning.
- `reset_and_decrement_counter`: Sets a 32-bit global variable to zero, decrements a 16-bit value at an offset of 4 from that variable, and branches to another routine if the decremented value reaches 65535; otherwise it returns.
- `store_value_0x28_2`: Stores the constant value 40 into a global variable and then clears two adjacent global variables to zero.
- `update_timer_and_reset_counters_2`: Stores the word value twelve thousand eight hundred eighteen in a longword at offset three hundred eighteen, calls another routine, and if a word at offset eight hundred twenty-two is not zero, it branches; otherwise it writes values to three different global variables, checks if a word at offset eight hundred forty-eight is two, and either branches or increments that word before returning.
- `increment_and_check_counter_4`: Clears a 16-bit value in a global variable, increments a 16-bit value at an offset of 4 in the same structure, and branches to another routine if the value reaches 3; otherwise it returns.
- `set_globals_70_0_0`: Stores the value 70 into one global variable and sets two other global variables to 0, then returns.
- `update_state_and_increment_counter_5`: Writes a 16-bit constant to a global variable, calls another routine, and if a specific 32-bit global variable is zero, resets two other global variables to zero and increments a third by one; otherwise, it increments the third variable and returns.
- `decrement_and_check_counter_3`: Decrements a 16-bit word at a memory location pointed to by a longword, then checks if that value is equal to sixty-five thousand five hundred and thirty-five; if so, it branches to another routine, otherwise it returns.
- `set_constant_and_clear_globals`: Stores the decimal value 40 into a global variable and clears two adjacent global variables to zero, then returns.
- `update_timer_and_reset_counters_3`: Stores the word value five thousand one hundred forty-four in a longword at offset three hundred eighteen, calls another routine, and if a word at offset eight hundred twenty-two is not zero, it branches; otherwise it writes values to three different global variables, checks if a word at offset eight hundred forty-eight is two, and either branches or increments that word before returning.
- `increment_and_check_five`: Clears a 16-bit word at a global variable, increments a 16-bit word at another global variable, and branches if that value is 5; otherwise it returns.
- `set_globals_81_0_0`: Stores the value 81 into one global variable and sets two other global variables to 0, then returns.
- `update_state_and_increment_counter_6`: Writes two 16-bit values to a global variable, calls another routine, and then either returns or increments a 16-bit value at an offset of 0 within a global variable after checking a 32-bit value at an offset of 6.
- `reset_and_check_pointer`: Sets a longword global variable to zero, decrements a 16-bit value at an offset of 4 from that same global variable, and branches to another routine if that value reaches 65535; otherwise it returns.
- `store_40_and_clear_two_globals`: Stores the constant value 40 into a global variable and clears two adjacent global variables to zero, then returns.
- `init_rastport_fields`: Initializes four parallel blocks of fields with fixed values and pointer slots, then invokes a routine through a computed function pointer.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 2 of 5).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_library_state`: Allocates memory, copies a table of data into it, and performs several bitwise manipulations on structure offsets to set up color or display parameters.
- `copy_and_sync_color_data`: Loops three times to read two 16-bit words from a table, calls adjust_nibbles to modify the first word, and then writes the result to both the original table and the hardware color registers.
- `adjust_nibbles`: Compares four nibbles extracted from two 16-bit words and modifies the nibbles of the first word by adding or subtracting 1, 16, or 256 based on their relationship to the nibbles of the second word, then returns the combined result.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_block`: Fills a block of memory starting at a location specified by a pointer with zeros using a loop that runs 8000 times.
- `open_libraries`: Calls two different functions from the library pointed to by a global variable and stores the returned pointers in two separate global variables.
- `process_string_until_newline`: Calls a routine to process a string, then loops through the string, incrementing a global variable by 16 for every newline character encountered until a zero terminator is reached.
- `count_string_bytes`: Walks a byte string from a0, incrementing a counter for each byte until it reaches a newline or a zero terminator, then calls a routine at a global offset when the count is non-zero.
- `setup_and_call_library_function`: Pushes several values onto the stack to populate a structure, then calls a routine via a pointer from a table and jumps to another routine via a pointer from a different table.
- `call_and_jump_library_function`: Calls a routine using a pointer from a table and then jumps to another routine using a pointer from the same table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_process_input`: Resets audio hardware registers, initializes four audio channels via a subroutine, and enters a loop that processes input and updates a counter.
- `increment_validation_counters`: Increments two global long-word counters and calls another routine.
- `disable_interrupts_and_save_registers`: Calls a routine to forbid interrupts, then stores the current value of the DMA controller register and the interrupt enable register into global variables with specific bitwise modifications applied to them.
- `call_permit_2`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `nested_table_update`: Executes a nested loop structure that calls a library function and then calls another routine if a specific bitmask match is found in a value at an offset of a structure.
- `increment_nibble_values`: Iterates through a table of 32 words, comparing the nibbles of a word from a global array with the nibbles of a word in the table, and increments the table value by 1, 16, or 256 if the table nibble is smaller.
- `iterate_and_validate_nibbles`: Executes a nested loop structure where the outer loop runs 31 times and the inner loop runs once, calling an Amiga OS function and then checking if bits 8 through 15 of a word at offset 4 of a pointer match a specific mask before calling another routine.
- `increment_nibble_values_2`: Iterates 31 times through a loop, comparing nibbles of a word from a routine to a word in a table, and adds 1, 16, or 100 to a value in the table based on the comparison.
- `iterate_table_with_mask_check`: Executes a nested loop structure that calls a library function and then calls another routine if a specific bitmask match is found in a value at an offset of 4 of a structure.
- `decrement_nibble_values`: Iterates through a table of 31 words, comparing the nibbles of a word from a global array with the nibbles of a word in the table, and decrements the table value by 1, 16, or 256 if the array nibble is smaller.
- `decrement_non_zero_nibbles_2`: Iterates through a loop 32 times, calling a library function and then calling another routine twice for two different table addresses if a specific bitmask match is found.
- `decrement_non_zero_nibbles`: Iterates through a table of 32 words and decrements the value at a specific offset in the table by 1, 16, or 256 for every non-zero nibble found in the table word.
- `set_bits_repeatedly`: Performs the bit set instruction on register d0 and bit 7 of register d7 thirty-two times.
- `write_long_values`: Writes four 32-bit values to a table starting at a global address by splitting each value into two 16-bit words and incrementing the destination pointer by 8 each iteration.
- `invert_global_value`: Bitwise-inverts a global variable and stores its address into a second global, returning through that pointer.
- `populate_long_word_table_2`: Loads a long word from a pointer in a global variable and calls another routine.
- `populate_long_word_table`: Iterates five times to store a long word into a table, where the lower 16 bits are written to an offset of 6 and the upper 16 bits are written to an offset of 2 within each 8-byte entry.
- `init_audio_channels`: Initializes the audio channels by writing fixed values into their length and volume registers and clears a set of RAM variables.
- `decrement_table_values`: Iterates through a table of four entries, decrementing the 16-bit value at offset 6 of each entry and, if the value reaches zero, setting the 16-bit value at offset 8 to zero.
- `setup_audio_registers`: Selects a pair of words from a table using a masked index, sets a flag bit in a byte, then copies several audio and interrupt registers from fixed offsets into custom hardware registers before setting a flag.
- `init_interrupt_handler`: Writes a sequence of fixed values into a flag byte and several global variables, then sets an interrupt-enable register, acting as a handler entry point.
- `update_audio_channels`: Checks four different global bytes; if a byte is non-zero, it clears it and loads a corresponding address and length from a table into the appropriate audio hardware registers.
- `reset_audio_and_flags`: Clears a run of global variables, sets several custom register flags to one, clears a bit in a flag byte, and writes a fixed value into a global variable, performing a broad hardware and memory reset.
- `decompress_data_block`: Checks a magic constant, then iterates through a source buffer to perform a run-length encoding decompression into a destination buffer.

### Fury of the Furries (1994)(Mindscape)[cr CLS][a2](Disk 1 of 5).adf:A#unpacked

- `initialize_hardware_and_loop_input`: Resets audio hardware, initializes four audio channels via a subroutine, and enters a loop that processes input and updates a counter.
- `increment_counters_and_jump`: Increments two global words, calls another routine, and then performs an absolute jump.
- `forbid_and_save_hardware_state`: Calls a routine to forbid interrupts, then stores the current value of a global variable, the interrupt enable register with bits 15 and 8 set, and the DMA controller register with bits 15 and 14 set into three global variables.
- `call_permit_3`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `sync_and_verify_table_entries`: Executes a nested loop structure that calls a library function and then calls another routine if a specific bitmask match is found in a value at an offset of 4 of a structure pointed to by a register.
- `increment_table_nibbles`: Iterates through a table of 32 words, comparing the low, middle, and high nibbles of a word from a global array with the corresponding nibbles of each table entry, and increments the table entry by 1, 16, or 256 if the table nibble is smaller.
- `wait_and_validate_nibbles`: Executes a nested loop structure where an outer loop runs 31 times and an inner loop runs once, calling a library function and then checking if bits 8 through 15 of a word at offset 4 of a pointer match a specific mask before calling another routine.
- `increment_nibble_based_values`: Iterates 31 times through a loop, comparing nibbles of a word from a table to a word in another table, and adds 1, 16, or 100 to a value in the second table based on the comparison.
- `iterate_table_with_wait`: Executes a nested loop structure that calls a library function and then calls decrement_table_nibbles if a specific bitmask match is found in a value at an offset of 4 of a structure.
- `decrement_table_nibbles`: Iterates through a table of 30 words, comparing the low, middle, and high nibbles of a word from a global array with the corresponding nibbles in the table, and decrements the table entry by 1, 16, or 256 based on the comparison results.
- `sync_and_decrement_table_values`: Executes a nested loop structure where the outer loop runs 32 times and the inner loop runs twice.
- `decrement_table_values_by_nibble_weight`: Iterates through a table of 32 words via a pointer in a1, and for each word, subtracts 1, 16, or 256 from the value at offset 2 based on whether the low, middle, or high nibbles are non-zero.
- `set_bits_repeatedly_2`: Performs the bit set operation on d0 using d7 thirty-two times.
- `populate_long_table_with_arithmetic`: Runs a loop four times to store a sequence of 32-bit values into a table starting at a global address, where each value is derived by adding 6640 to the previous one and is split into two 16-bit words stored at offsets 2 and 6 of each 8-byte entry.
- `invert_global_value_2`: Bitwise-inverts a global variable and stores its address into a second global, returning through that pointer if the inverted value is not zero, otherwise it returns.
- `call_split_word_table_populator`: Loads a long word from a global variable into a0 and calls another routine.
- `populate_split_word_table`: Iterates five times to store a long word into a table, where the lower 16 bits are written to an offset of 6 and the upper 16 bits are written to an offset of 2 within each 8-byte entry.
- `init_audio_channels_2`: Initializes the audio channels by writing fixed values into their length and volume registers, clears a set of RAM variables, configures DMA and interrupts, and sets bit 1 of a CIA register.
- `decrement_audio_channel_counters`: Iterates through a table of four entries, decrementing the 16-bit value at offset 6 of each entry and, if the value reaches zero, setting the 16-bit value at offset 8 of the hardware audio channel register to zero.
- `configure_audio_hardware`: Selects a pair of words from a table using a masked index, sets a flag bit in a byte, and then copies several audio and interrupt registers from a table into the audio channel and DMA control registers before updating a global pointer and setting a value in a custom hardware register.
- `configure_interrupt_and_dma_3`: Writes a fixed value to a control register, loads a DMA configuration from a relative address into the DMA control register, stores a value from a relative address into a global variable, and enables a specific interrupt bit before returning from an interrupt.
- `update_audio_channels_3`: Checks four global bytes; if a byte is non-zero, it clears it and loads a corresponding address and length from a table into the four audio channel hardware registers, then sets a bit in the interrupt request register and returns.
- `mute_audio_and_clear_fire_bit`: Sets all four audio volume registers to zero, sets all four audio length registers to one, clears bit 1 of the CIAA Port A register, and stores a pointer from a local table into a global variable.
- `decompress_data_block_2`: Compares a long word against a constant to verify a header, then iterates through a block to copy bytes or expand repeated values based on a control byte.

### intro.binc#unpacked (raw code file, load address unknown)

- `parse_audio_command_structure`: Parses an audio command structure to update hardware registers and DMA settings, then performs a loop to calculate and store four 32-bit audio parameters into a destination table based on arithmetic performed on bytes from a source structure.
- `process_audio_command_structure`: Parses a command structure to update audio hardware registers and DMA settings.
- `setup_interrupt_handler`: Writes a byte to a flag register, stores two values from the code into global variables, and enables a specific interrupt by writing to the interrupt mask and returning from an interrupt.
- `update_audio_hardware_registers_2`: Checks several bytes to determine if specific values should be written to hardware registers, then performs complex data manipulation on a structure and calls a routine.
- `process_table_entry`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs a series of bitwise operations and calls another routine via a pointer from a table, otherwise it updates a global variable and hardware registers before returning.
- `update_audio_register`: Sets a byte in a global variable to 19, calculates a value by combining a byte from a PC-relative offset with a byte from another PC-relative offset and adding 32768, writes this to a hardware register, stores a long value from a PC-relative offset into a global variable, writes 8192 to a hardware register, and returns from an interrupt.
- `update_structure_data`: Iterates through a list of pointers to update structure members using values from a table, then performs complex arithmetic to update various fields in a structure based on bitwise masks and multiplication, and finally updates a counter in a structure.
- `process_audio_command_sequence`: Reads a command byte from a structure, updates a state byte in a global structure, and executes a sequence of commands by iterating through a table to perform table lookups, arithmetic on a counter, or calling another routine.
- `dispatch_audio_command`: Calculates a jump target based on a byte from a structure, then jumps to that address.
- `calculate_audio_parameters`: Calculates and stores four 32-bit values into a destination table by performing arithmetic on bytes from a source structure, then updates hardware registers and a global variable.

### desert.binc#unpacked (raw code file, load address unknown)

- `process_audio_command_structure_2`: Parses an audio command structure to update hardware registers and DMA settings, then performs arithmetic on source bytes to calculate and store four 32-bit values into a destination table.
- `update_audio_hardware_registers_3`: Parses a data structure to update audio parameters, including volume, pitch, and sample data pointers, and writes values to the audio hardware registers and DMA controller.
- `configure_interrupt_handler`: Writes a byte to a flag register, loads a value from a table into a custom chip register, stores a value into a global variable, sets the interrupt mask, and returns from interrupt.
- `process_audio_table`: Updates several hardware registers from a table based on a byte in a structure.
- `dispatch_table_entry_and_update_hardware`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs bitwise operations to index a table and calls another routine via a pointer; otherwise, it decrements a byte at offset 103 of the structure in a1 and returns.
- `update_audio_hardware_registers`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 32768, writes this to the DMA control register, stores a long value from a PC-relative offset into a global variable, writes 8192 to the DMA control register, and returns from an interrupt.
- `update_structure_member_fields`: Iterates through a list of pointers to update specific members of structures using values from a source table, then performs complex bitwise and arithmetic operations to modify various fields within those structures, including scaling values via multiplication and updating counters.
- `execute_audio_command_sequence`: Reads a command byte from a structure, resets a state byte in a global structure, and then branches into a multi-stage handler that either updates a 16-bit value in a structure with bounds checking, performs table lookups to update a 16-bit value, or executes a loop of sub-commands by reading bytes from a table and calling another routine.
- `dispatch_audio_command_jump_table`: Parses a byte from a command structure to calculate an offset into a jump table and jumps to the resulting address.
- `update_audio_parameters`: Reads values from a structure to update audio volume and length registers, then performs a loop to write four 32-bit values to a destination array by adding a base offset and a value from a source table.

### MONTAGNE.binc#unpacked (raw code file, load address unknown)

- `process_audio_command_structure_3`: Parses an audio command structure to update hardware registers and DMA settings.
- `process_audio_command_data`: Parses a command structure to update audio hardware registers and DMA settings, performing data manipulation on a table of sound parameters and writing values to the audio hardware via a pointer in a6.
- `setup_interrupt_handler_2`: Writes a byte to a flag register, stores a pointer from a table into a global variable, sets a hardware register to a specific value, and enables interrupts via an exception return.
- `update_audio_hardware_registers_7`: Checks several flags in memory to conditionally update the audio channel address and length registers, the DMA controller, and the interrupt request register.
- `process_audio_table_entry`: Checks a byte at offset 2 of a structure in a1; if non-zero, it performs bitwise operations, uses a table to find an address, and calls another routine.
- `update_audio_register_2`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 33248, writes this to the DMA control register, stores a long value from a PC-relative offset into a global variable, writes 8192 to the DMA control register, and returns from an interrupt.
- `copy_channel_state_and_ack_irq`: Loops over a list of (destination, source) pointer pairs, copying a 32-bit value and a 16-bit value from each source structure into the destination, then restores registers, reloads a saved vector from a global into the level-2 interrupt vector at address 120, writes 8192 to INTREQ to clear the interrupt request, and returns from exception.
- `execute_audio_sequence`: Processes a command byte from a structure, updates a value in a structure, and uses a loop to either call itself or jump to a target address based on data from a table.
- `dispatch_audio_command_sequence`: Parses a command byte to select a jump target from a table and executes it, or performs a sequence of hardware initializations including setting custom chip registers, configuring DMA, and updating audio hardware registers and interrupt settings.
- `update_audio_parameters_2`: Updates audio volume and length registers from a structure and then performs a loop to write four 32-bit values to a destination array by adding a base offset and a value from a source table.

### LAGON.binc#unpacked (raw code file, load address unknown)

- `expand_audio_command_parameters`: Parses an audio command structure to update hardware registers and decrement a counter in a global variable.
- `update_audio_hardware_registers_4`: Processes an audio data structure by reading various parameters to update sample pointers, playback lengths, and volume levels, then writes values to the audio hardware registers and the DMA controller.
- `configure_interrupt_handler_2`: Tests a byte at a specific memory location, writes a value to a flag byte, stores two values from the instruction stream into global variables, and enables a specific interrupt by writing to the interrupt mask and returning from an interrupt.
- `update_audio_hardware_registers_8`: Checks several global flags to conditionally update the audio channel address and length registers, sets a DMA control register to enable DMA, and returns via an interrupt return instruction.
- `execute_table_entry_callback`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs bitwise operations to index a table and calls another routine via a pointer.
- `update_audio_register_3`: Sets a byte in a global variable to 19, calculates a value by combining a byte from a PC-relative offset with a byte from another PC-relative offset and adding 32768, writes this to a hardware register, stores a long value from a PC-relative offset into a global variable, writes 8192 to a hardware register, and returns from an interrupt.
- `update_structure_member_data`: Iterates through a list of pointers to update structure members using values from a table, then performs bitwise masking and multiplication to update various fields in a structure, and finally adjusts a counter in the structure.
- `dispatch_audio_commands`: Reads a byte from a structure, modifies a value at an offset in a structure, and uses a loop to execute commands by calling itself or jumping to an address derived from a table.
- `dispatch_audio_command_jump_table_2`: Extracts a value from a structure to calculate an offset into a jump table and jumps to the resulting address.
- `calculate_audio_parameters_2`: Calculates four 32-bit audio values by combining a base value with a scaled byte from a source table and stores them in a destination table, then updates hardware audio registers and a global variable.

### FORET.binc#unpacked (raw code file, load address unknown)

- `update_audio_hardware_parameters_2`: Checks a flag in a structure to determine if audio hardware needs initialization, then updates audio volume and length registers by reading from a table and writing to the hardware.
- `update_audio_hardware_parameters`: Updates a structure containing audio pointers and configuration values by reading from a table and writing to hardware registers, including the DMA controller and audio control registers.
- `configure_interrupt_and_dma`: Writes a byte to a flag register, loads a value from a table into a custom register, stores a constant into a global variable, and enables a specific interrupt by writing to the interrupt mask and returning from an interrupt.
- `update_audio_hardware_pointers`: Checks several bytes in memory to conditionally update four pairs of longword and word values in the audio hardware registers, then sets a bit in the DMA control register and executes an return from interrupt.
- `process_table_entry_and_update_hardware`: Reads a word from offset 2 of a structure in a1, performs bitwise operations to derive an index, and calls another routine via a pointer from a table.
- `update_audio_register_4`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 32768, writes this to a hardware register, stores a long value from a PC-relative offset into another global variable, writes 8192 to a hardware register, and returns from an interrupt.
- `update_structure_data_2`: Iterates through a list of pointers to a table, copying two values from the table into a structure pointed to by each entry.
- `update_audio_sample_pointer`: Updates a sample position value in a structure, clamping it between a minimum and maximum range, and then performs a loop to fetch and process sample data bytes through a series of indirect calls.
- `initialize_audio_and_dma_hardware`: Checks a flag in a structure, then initializes several audio hardware registers, configures DMA and interrupts, and performs a series of pointer updates and memory copies involving a table and a loop to populate a structure with data.
- `update_audio_parameters_3`: Updates audio volume and length registers from a structure, then executes a loop that reads four 32-bit values from a source table and writes them to a destination array by adding a base offset and a value from the table.

### PYRAMIDE.binc#unpacked (raw code file, load address unknown)

- `expand_audio_command_structure`: Checks a command structure to determine if it requires a decrement of a counter at offset 164, then iterates through a list of data to expand and store four 32-bit values into a destination table by performing arithmetic on bytes from the source structure.
- `process_audio_command_data_2`: Parses a command structure to update audio hardware registers and memory buffers, performing data manipulation on a table based on command types and indices.
- `configure_interrupt_handler_3`: Writes a byte to a flag register, loads a pointer from a table into a custom register, sets a DMA control register to enable specific channels, and enables interrupts via the interrupt mask register before returning from an interrupt.
- `update_audio_hardware_registers_9`: Checks several global flags to conditionally update the audio channel addresses and lengths, the DMA controller configuration, and the interrupt request register before returning via an interrupt return instruction.
- `execute_table_indexed_callback`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs bitwise operations to index a table and calls another routine via a pointer.
- `update_audio_register_5`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 33280, writes this to a hardware register, stores a long value from a PC-relative offset into a global variable, writes 8192 to a hardware register, and returns from an interrupt.
- `update_audio_parameters_9`: Iterates through a list of pointers to update structure members using values from a table, then performs complex arithmetic to update various fields in a structure based on bitwise masks and multiplication, and finally updates a counter in a structure.
- `handle_audio_data`: Manipulates a byte in a structure, performs bounds checking on a word in a structure, and uses a loop to process commands by calling itself or jumping to a target address.
- `dispatch_audio_command_jump`: Parses a byte from a structure to calculate a jump target using a table and performs an indirect jump.
- `update_audio_parameters_4`: Updates audio volume and length registers from a structure, then performs a loop to write four 32-bit values to a destination array by adding a base offset and a value from a source table.

### USINE.binc#unpacked (raw code file, load address unknown)

- `process_audio_command_structure_4`: Parses an audio command structure to update hardware registers and DMA settings, then executes a loop that calculates and stores four 32-bit audio parameters into a destination table based on values from a source structure.
- `update_audio_hardware_registers_5`: Processes an audio data structure by reading command bytes and parameters to update hardware registers, including the audio chip and DMA controller.
- `setup_interrupt_handler_3`: Writes a byte to a flag register, stores two values from the instruction stream into hardware registers and a global variable, and enables interrupts by returning from an exception.
- `update_audio_hardware_registers_10`: Checks several global flags to conditionally update the audio channel address and length registers, sets a bit in the DMA control register, and returns via an interrupt return instruction.
- `process_audio_table_entry_2`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs bitwise operations and calls another routine via a pointer from a table; otherwise it updates a global variable and hardware registers including DMA and audio control registers.
- `update_audio_register_6`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 33280, writes this to the DMA control register, stores a long value from a PC-relative offset into a global variable, writes 8192 to the DMA control register, and returns from an interrupt.
- `update_structure_data_3`: Iterates through a table of pointers to update specific members of a structure using data from another source, then manipulates various fields within each structure using bitwise masking, multiplication, and arithmetic, and finally updates a counter field in each structure.
- `run_audio_commands`: Updates a command byte in a structure, performs arithmetic on a value in a structure, and iterates through a loop to call itself or jump to a calculated address.
- `dispatch_audio_command_jump_table_3`: Parses a byte from a structure to calculate an offset into a jump table and jumps to the resulting address.
- `update_audio_parameters_5`: Calculates and stores four 32-bit values into a destination table by performing arithmetic on bytes from a source structure, then updates hardware registers and a global variable.

### VILLAGE.binc#unpacked (raw code file, load address unknown)

- `process_audio_command_structure_5`: Parses an audio command structure to update hardware registers and DMA settings.
- `process_audio_command_data_3`: Parses a command structure to update audio hardware registers and memory buffers, performing data manipulation on a table of values based on command types.
- `configure_interrupt_handler_4`: Writes a byte to a flag register, loads a value from a table into a custom register, stores a constant into a global variable, sets the interrupt mask, and returns from an interrupt.
- `update_audio_hardware_registers_11`: Checks several global flags to conditionally update the audio chip's channel address and length registers, sets the DMA controller to enable DMA, and performs an interrupt return.
- `dispatch_audio_table_entry`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs bitwise operations to select and call another routine via a table, otherwise it decrements a byte at offset 103 of the structure and returns.
- `update_audio_register_7`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 32768, writes this to a hardware register, stores a long value from a PC-relative offset into a global variable, writes 8192 to a hardware register, and returns from an interrupt.
- `dispatch_audio_channel_update`: Checks the CIA-B status register, then for each queued channel record copies a pointer pair and writes a 32-bit value and a 16-bit value from a source structure to the destination pointer, and finally restores registers, sets a vector slot, and clears the INTREQ interrupt bit before returning from the exception.
- `process_audio_stream`: Modifies a byte in a structure, performs arithmetic on a value in a structure, and uses a loop to execute commands by calling itself or jumping to an address from a table.
- `dispatch_audio_command_jump_table_4`: Calculates a jump target by masking a byte from a structure and adding an offset from a table, then jumps to that address.
- `update_audio_parameters_6`: Updates audio volume and length registers from a structure and performs a loop to write four 32-bit values to a destination array by adding a base offset and a value from a source table.

### CASTLE.binc#unpacked (raw code file, load address unknown)

- `update_audio_command_structure`: Checks a command structure to determine if it needs processing, decrements a counter in the structure, and calls a routine to update audio parameters.
- `process_disk_data_structure`: Parses a data structure to update various fields including pointers, lengths, and byte values based on specific opcode-like bytes.
- `configure_interrupt_and_dma_2`: Writes a byte to a flag register, loads a value from a PC-relative offset into the interrupt mask register, stores a value from another PC-relative offset into a global variable, sets the DMA control register to enable specific channels, and returns from an interrupt.
- `update_audio_hardware_registers_12`: Checks several status bytes to conditionally update the four audio channel long and word registers with values from a table, then enables DMA and performs an interrupt return.
- `process_table_entry_and_update_hardware_2`: Reads a word from offset 2 of a structure in a1, performs bitwise operations to calculate an index, and calls a routine via a pointer from a table.
- `update_audio_register_8`: Sets a byte in a global variable to 25, calculates a value by combining a byte from a PC-relative offset with a byte from another PC-relative offset and adding 33280, writes this to the DMA control register, stores a long value from a PC-relative offset into a global variable, writes 8192 to the DMA control register, and returns from an interrupt.
- `service_audio_interrupt_handler`: Interrupt handler that first reads CIA-B's interrupt control register to acknowledge it, then walks a list of pointer pairs copying a longword and a word from one structure into another, and finally clears the interrupt request by writing the INTREQ register and returns from the exception.
- `process_audio_command_structure_6`: Parses a command structure to execute various audio operations, including updating a counter within a specific range, selecting a function from a table via an index, or iterating through a sequence of sub-commands by calling itself recursively.
- `dispatch_audio_command_structure`: Parses a command structure to select a sub-routine via a jump table based on a nibble extracted from a byte, then performs complex pointer arithmetic and data movement to update a structure with various function pointers and hardware configuration values.
- `update_audio_parameters_7`: Updates audio volume and length registers from a structure, then executes a loop that writes four 32-bit values to a destination array by adding a base offset and a value from a source table.

### fin.binc#unpacked (raw code file, load address unknown)

- `update_audio_command_table`: Parses a command structure to update audio hardware registers and DMA settings, then iterates through a list of data to calculate and store four 32-bit audio parameters into a destination table.
- `update_audio_hardware_registers_6`: Processes an audio data structure to update hardware registers and DMA settings.
- `setup_interrupt_handler_4`: Writes a byte to a flag register, stores two values from a PC-relative table into hardware registers, writes a value to a global variable, and enables interrupts by returning from an exception.
- `update_audio_hardware`: Checks several bytes in a structure to conditionally update audio hardware registers, including channel and length registers, and performs complex data manipulation on a structure in a1 before returning.
- `process_audio_table_entry_3`: Checks a byte at offset 2 of a structure in a1, and if non-zero, performs bitwise operations to select an address from a table and calls another routine via a pointer; otherwise, it decrements a byte at offset 103 of the structure in a1 and returns.
- `update_audio_register_9`: Sets a byte in a global variable to 25, calculates a value by combining a byte from a PC-relative offset with a byte from another PC-relative offset and adding 32768, writes this to a hardware register, stores a long value from a PC-relative offset into a global variable, writes 8192 to a hardware register, and returns from an interrupt.
- `copy_audio_channel_registers`: Acknowledges the CIA-B interrupt by reading its control register twice, then loops a counted number of times over a table of pointer pairs.
- `execute_audio_command`: Increments a byte in a structure, then either updates a state byte, performs a jump to a table-based address, or executes a sequence of commands by iterating through a table and calling another routine.
- `dispatch_audio_command_jump_table_5`: Parses a byte from a command structure to calculate an offset into a jump table and jumps to the resulting address.
- `update_audio_hardware_and_parameters`: Calculates and stores four 32-bit values into a destination table by performing arithmetic on bytes from a source structure, then updates hardware registers and a global variable.

### TECKNO.binc#unpacked (raw code file, load address unknown)

- `update_audio_command_structure_2`: Checks a command structure to decrement a counter or call a sub-routine, then iterates through a data block to expand or transform nibble-based values into a new sequence of bytes.
- `process_disk_validation_data`: Iterates through a data structure to validate nibbles against a table, updates a global status byte via a hardware register, and performs conditional pointer arithmetic and data copying based on values within the structure.
- `configure_interrupt_handler_5`: Tests a byte at a global address, writes a value to a flag byte, loads a word from a relative offset into the interrupt mask register, stores a longword from a relative offset into a global variable, sets the interrupt mask to enable specific interrupts, and returns from interrupt.
- `configure_audio_registers`: Updates audio hardware registers based on values in a structure, performs bitwise operations and index calculations using a table in a2, and modifies a structure in a1.
- `process_table_entry_and_update_hardware_3`: Reads a word from offset 2 of a structure in a1, calculates an index via bitwise operations, and calls another routine via a pointer from a table.
- `configure_audio_interrupt`: Sets a byte in a global variable to 25, calculates a value by combining two bytes from PC-relative offsets and adding 32768, writes this value to the audio hardware register, stores a long value from a PC-relative offset into a global variable, writes 8192 to the audio hardware register, and returns from an interrupt.
- `service_audio_interrupt_handler_2`: Interrupt service routine that first tests the CIA-B interrupt control register to acknowledge the timer source, then saves registers and walks a table of paired pointers, copying a long and a word from each source structure into its destination.
- `execute_audio_command_subroutines`: Updates a byte at offset 7 of a structure, modifies a 16-bit value at offset 12 of a structure within specific bounds, or uses a nibble to index into a jump table to call a subroutine via a pointer.
- `dispatch_audio_command_subroutine`: Extracts a nibble from a byte to select a subroutine from a jump table via a PC-relative offset, otherwise it performs complex pointer arithmetic to update a structure with several function pointers and hardware register values.
- `update_audio_parameters_8`: Reads volume and length values from a structure to update hardware audio registers, then performs a loop to write four 32-bit values to a destination array by adding a base offset and a value from a source table.
