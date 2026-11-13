# Game 902 (rally championships (1994)(flair)[cr pdy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00902). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3610064 top-level bytes in 321 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 175 of 175 named (0 author symbols), 175 of 175 described (9 by rule, 115 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 646 of 646 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 3900 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3496 of 3496 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 285512 |
| code | 5722 |
| data | 3220384 |
| padding | 55950 |
| unused | 42496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:PDY-RALLY.EXE | 3 | 160 | 5 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE | 3 | 258 | 12 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:CopStart | 1 | 14 | 1 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 804 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 179 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE#unpacked | 2 | 2061 | 115 |
| font8102.raw#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| font8102_2.raw#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:PRES.EXE: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:OPTION.EXE: high-entropy data file (7.64 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Crash.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (38996 -> 50382 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Victory.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (35720 -> 46966 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/font8102.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (1812 -> 2600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Timeout.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (37460 -> 50872 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/WorldImage.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (39644 -> 44678 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/InsideLogo.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (19344 -> 25100 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/WorldPodio.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (39720 -> 48838 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/SelectRally.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (48056 -> 54630 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/OutFuel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (39920 -> 52054 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/SinglePodio.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (41160 -> 53636 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/NextRace.txt: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (1044 -> 9620 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Mod.Racer: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (95972 -> 140006 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/font8102_2.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (924 -> 1300 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/practice.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (24556 -> 34604 bytes)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:flagselect.exe: high-entropy data file (7.69 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop1.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (44900 -> 53530 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribc.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (5744 -> 21560 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carriba.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (6944 -> 22880 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:PorscheScel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (24596 -> 31110 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop8.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (39236 -> 48836 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Gommee.Raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2260 -> 10240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop6.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (44372 -> 54254 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:EscortScel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (24760 -> 30588 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:DeltaScel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (21744 -> 29598 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Care.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (38184 -> 71960 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Gommec.Raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2712 -> 10240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop4.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (45424 -> 54500 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribf.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (5796 -> 22040 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carc.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (35088 -> 68200 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Gommea.Raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2512 -> 10240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop2.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (45532 -> 54542 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribd.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (6796 -> 23400 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Cara.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (33884 -> 66760 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:CalibraScel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (23700 -> 30790 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribb.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (6664 -> 24120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:CelicaScel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (21960 -> 28894 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Gommef.Raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (1760 -> 10240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Copiloti.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (33456 -> 51262 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop7.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (42528 -> 52356 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:peugeotScel.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (22032 -> 30586 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carf.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (36868 -> 76160 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Gommed.Raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2684 -> 10240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop5.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (44892 -> 54100 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Card.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (33092 -> 66880 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Gommeb.Raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2864 -> 10240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop3.iff: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (45372 -> 54394 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribe.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (5684 -> 22440 bytes)
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carb.Dat: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (35952 -> 72520 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:AlteScenary1.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (4728 -> 5120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracke.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (11520 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackp.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9180 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw2.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (125644 -> 204800 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackc.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (10928 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackn.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (8840 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw0.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (117608 -> 204800 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracka.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (8928 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackl.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9108 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp2.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (31704 -> 153600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackj.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (10268 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp0.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (40304 -> 153600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackh.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (10304 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:AlteScenary2.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2180 -> 5120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackf.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (8192 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw3.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (87992 -> 204800 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:AlteScenary0.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2744 -> 5120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackd.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9708 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracko.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9060 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw1.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (136232 -> 204800 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackb.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9048 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackm.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (8228 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp3.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (62880 -> 153600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackk.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9604 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp1.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (64824 -> 153600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracki.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (9348 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:AlteScenary3.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (1900 -> 5120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackg.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (8344 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:ScenaryDw4.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (108364 -> 204800 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:BonusSC1.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (684 -> 1120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackz.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (7888 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:scenaryup6.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (29844 -> 153600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:ScenaryUp4.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (58732 -> 153600 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackt.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (10196 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:BonusSC3.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (652 -> 1120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackr.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (10316 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackq.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (11648 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:BonusSC0.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (664 -> 1120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/NumeriSmall.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (188 -> 440 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Grats1.Sampl: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (11416 -> 14212 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod2.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (37848 -> 50860 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod1.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (27072 -> 48980 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Mano321go.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (26860 -> 56284 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod6.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (42312 -> 54952 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/JumpTable: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (148 -> 248 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod3.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (32404 -> 50344 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/MaskBlock.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (568 -> 4480 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Tabella_Seno: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (3608 -> 5120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/freccesp.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (8864 -> 43818 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod0.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (37496 -> 54222 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/NumeriBig.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (584 -> 1760 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod4.pack: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (39164 -> 52770 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Marce.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (548 -> 1288 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/pannellon.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (412 -> 6840 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Contagiri.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (2876 -> 18240 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:BonusSC6.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (652 -> 1120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:BonusSC4.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (740 -> 1120 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:tracks.tab: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (10552 -> 40320 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:scenarydw6.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (116868 -> 204800 bytes)
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:BonusSC2.raw: (Dtpk) Data Cruncher (xfd)-packed, unpacked with ancient (740 -> 1120 bytes)
- runtime segment an address: 88 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 153 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:PDY-RALLY.EXE: CrunchMania Normal (xfd) stream found by ancient scan (2716 -> 63080 bytes) but only 1 of 2 sampled windows are in reference RAM: not used
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE: crunched executable decrunched by XFDMaster (ProPack (RNC) (xfd), 13372 -> 39064 bytes), 9 of 12 windows confirmed in reference RAM: recovered as a hunk executable
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/font8102.raw#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/font8102_2.raw#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 address, 1 jump target
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:AlteScenary6.raw: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:AlteScenary4.raw: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Jump.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Narrow.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Caution.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 3900 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 2048 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:PDY-RALLY.EXE

- `program_entry_free_memory`: Calls several unnamed routines, invokes a function pointer kept in a shared variable and zeroes a longword through a pointer.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `expand_packed_offset_table`: Walks a packed stream of counts and offsets, copying blocks of longwords and single bytes and adding a computed value into table slots at each offset.
- `select_half_or_input_word`: Returns the input word unchanged unless a flag equals one, in which case it returns a second word shifted right by one bit.
- `mask_with_table_word`: Looks up a word in a fixed table at an index that is doubled into a byte offset, and returns it ANDed with a caller-supplied word.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE

- `scan_tagged_block_and_unpack`: Calls the exec library AllocMem through a shared function pointer, then walks a linked list of blocks for an entry carrying a four-byte tag.
- `match_table_entry_extract_bits`: Scans a table of word pairs for the first entry whose masked value equals its stored companion, then reads a count and a shift from that entry.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: ANDs two 16-bit values, then if one byte is less than another byte it calls an unnamed routine and returns that routine's result.
- `return_empty_routine`: The shown pseudo-code is only a return, so it changes no memory or registers.
- `build_canonical_code_table`: Reads a count and fills a stack buffer of code lengths, then assigns bit-reversed canonical codes for each length and stores them with their masks in a table.
- `apply_longword_relocation_list`: When a marker word matches a fixed value, it clears the marker and walks lists of offsets, adding a base address to the longword at each offset.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `bounds_check_trap_handler`: Raises trap six when an index is negative or exceeds the bound stored at its target, and otherwise falls through into bad instruction data.
- `expand_packed_offset_table_2`: unpacks a stream of counts and offsets to populate a table with copied data and a computed offset.
- `check_task_attribute_and_jump`: Calls a library routine to find a task, checks a byte at an offset within the resulting pointer, and if that byte is not zero, restores registers and jumps to a new address; otherwise it restores registers and returns.
- `execute_library_functions_via_pointer`: Loads a library base address from a global variable into a6, then calls two different functions from that library using offsets of 284 and 372 relative to a0, which points to an offset of 92 from a4, before jumping to a location stored in a global variable.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:CopStart

- `open_and_close_library_probe`: Opens a library by name through the exec OldOpenLibrary call and, on success, stores a pointer to a shared variable in the library's base field before closing it.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_counters_and_mode_loop`: Opens a library through exec, saves one of its fields and writes the interrupt-enable register, then initialises a set of counters.
- `find_task_and_dispatch`: Calls a library routine to find a task, checks a byte at an offset within the resulting pointer, and if that byte is not zero, restores registers and jumps to a specific address; otherwise it restores registers and returns.
- `manage_game_state_loop`: Initializes audio and library pointers, then enters a main loop that dispatches sub-programs based on flags.
- `launch_practice_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_world_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_outoftime_program`: Calls an unnamed routine, runs an indirect call through a pointer and stores its return value in a shared variable, then sets a shared flag to one and then calls a second unnamed routine.
- `launch_damage_program`: Calls an unnamed routine, runs an indirect call through a pointer and stores its return value in a shared variable, then sets a shared flag to one and then calls a second unnamed routine.
- `launch_outoffuel_program`: Calls an unnamed routine, runs an indirect call through a pointer and stores its return value in a shared variable, then sets a shared flag to one and then calls a second unnamed routine.
- `launch_winner_single_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_winnerall_world_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_flair_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_inside_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `patch_and_launch_music_file`: Allocates and checks memory through exec library calls, overwrites four code words with no-operation instructions and sets a field before calling a routine.
- `launch_pres_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_option_program_save_state`: Calls an unnamed routine, then an indirect routine that returns a 64-bit result whose halves are stored in shared variables.
- `launch_select_rally_program`: Calls an unnamed routine, runs an indirect call through a pointer and stores its return value in a shared variable, then sets a shared flag to one and calls one final routine.
- `launch_select_car_save_state`: Calls an unnamed routine, then an indirect routine that returns a 64-bit result whose halves are stored in shared variables.
- `launch_next_race_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `launch_shop_program_loop`: Repeatedly runs an indirect call through a pointer, restoring three saved shared values around each run and setting a shared flag, and repeats while a count is nonzero.
- `launch_game_program_add_result`: Calls an unnamed routine, then an indirect routine that returns a 64-bit result whose halves are stored in shared variables.
- `launch_victory_program`: Calls two unnamed routines around an indirect call through a pointer and sets a shared flag to one.
- `add_result_to_two_totals`: Clears a shared variable, calls an unnamed routine and adds the value it returns to two shared totals.
- `compare_and_store_packed_record`: Reads a packed record of four decimal fields from a table selected by two indices and compares it with a value in a data register.
- `read_packed_record_plus_offset`: Reads the same packed decimal record as its neighbouring routine and returns the record's value with a fixed offset of four thousand added.
- `launch_flag_select_program`: Runs a short counted loop that repeatedly waits on a shared status bit and writes a set of values, then calls the referenced program through an indirect call and clears a shared variable.
- `call_two_unnamed_routines`: Calls two unnamed routines in order and returns.
- `wait_for_two_masked_values`: Busy-waits until a masked shared value equals one fixed value, then waits again until it equals a second fixed value.
- `fill_entries_with_stepped_values`: Writes successive values into the low and high words of eight-byte table entries, adding a fixed step each time, for a fixed count of entries.
- `open_intuition_call_close`: Opens intuition.library through exec and, if the open succeeds, calls a routine at a fixed library offset, runs one unnamed routine and closes the library again.
- `step_random_and_divide`: Advances a shared pseudo-random value by a shift-and-XOR step, then returns that value divided by the given divisor, packed as a quotient and a remainder in one result.
- `choose_routine_by_flag`: Calls one of two unnamed routines depending on whether a shared value is zero.
- `insert_score_into_top_table`: Scans a table of records for the first entry whose stored value is not greater than the new value, then shifts the later records down and stores the new pair at that position.
- `initialise_audio_slot_table`: Scans a byte table for the first entry above the running value, then clears a block of slots and writes pointers to each slot into a table, stepping through offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `render_text_glyph_rows`: Opens a library through exec and loops over a character sequence, finding each character's entry in a 73-byte lookup table.
- `install_interrupt_server_safely`: Writes disable values to the interrupt and DMA control registers, forbids multitasking through exec, then installs an interrupt handler with the exec AddIntServer call.
- `write_four_fixed_values`: Writes four fixed values into shared variables and returns.
- `write_two_value_pairs`: Writes two fixed value pairs into shared variables and returns.
- `wait_for_shutdown_flag`: Waits until a shared variable equals minus one and then loops endlessly.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_value_return_minus_one`: Writes a fixed value into a shared variable and returns a result whose high word is all ones.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE#unpacked

- `initialize_and_dispatch_main_loop`: Opens required libraries, configures the interrupt enable register, and enters a nested loop structure that branches into different execution modes based on a global variable.
- `test_and_jump_to_task`: Calls another routine via a pointer from a global variable, then checks if a value at an offset of 172 from the returned pointer is zero; if not zero, it restores registers and jumps to another routine.
- `wait_for_message_and_get_msg`: Waits for a message port and retrieves the next message from the queue using a pointer stored at an offset in a register, then returns.
- `initialize_assets_and_process_tasks`: Opens libraries, configures interrupt enable registers, and enters a loop that processes tasks based on a mode value in a global variable.
- `load_practice_asset`: Calls a routine with d1 set to 4108, calls a routine at the address in a0, sets a global variable at offset 4160 to 1, and calls another routine.
- `load_world_asset`: Calls a routine with d1 set to 4124, calls a routine at the address in a0, sets a global variable at offset 4160 to 1, and calls another routine.
- `load_out_of_time_asset`: Calls a routine with d1 set to 4003, loads a value from a global variable into d0, calls a routine at the address in a5, sets a global variable at offset 4160 to 1, stores d0 in a global variable at offset 4166, and calls another routine.
- `load_damage_asset`: Calls a routine with d1 set to 4107, loads a value from a global variable into d0, calls a routine at the address in a5, sets a global variable at offset 4160 to 1, stores d0 in a global variable at offset 4166, and calls another routine.
- `load_out_of_fuel_asset`: Calls a routine with d1 set to 4087, loads a value from a global variable into d0, calls a routine at the address in a5, sets a global variable at offset 4160 to 1, stores d0 in a global variable at offset 4166, and calls another routine.
- `load_winner_single_asset`: Calls a routine with d1 set to 4016, loads d0 and d1 from global variables, calls a routine at the address in a5, sets a global variable at offset 4160 to 1, and calls another routine.
- `load_winner_world_asset`: Calls a routine with d1 set to 3980, loads d0 and d1 from global variables, calls a routine at the address in a5, sets a global variable at offset 4160 to 1, and calls another routine.
- `load_flair_asset`: Calls a routine with d1 set to 3821, calls a routine at the address in a0, sets a global variable at offset 4160 to 1, and calls another routine.
- `execute_task_pointer`: Loads a library base pointer into a6, calls a routine with an address in d1, then calls a routine at the address in a0, writes 1 to a global variable, and calls another routine before returning.
- `allocate_and_initialize_memory`: Allocates memory using library calls, calls three routines via pointers, writes 20081 to four consecutive words at an offset of 156 from a pointer in a0, calls a routine at a0, writes 1 to a global variable, and calls another routine before returning.
- `execute_task_with_parameter`: Loads a library base pointer into a6, calls a routine with an address in d1, calls a routine at the address in a0, writes 1 to a global variable, and calls another routine before returning.
- `update_and_store_task_data`: Calls a routine with 3386 in d1, clears registers a1 through a6, calls a routine at a0, writes 1 to a global variable, and updates five global longwords using values from a0 and other global variables before calling another routine.
- `execute_task_and_store_result`: Loads a library base pointer into a6, calls a routine with an address in d1, calls a routine at a0, writes 1 to a global variable, and stores the result in a0 into a global variable before calling another routine.
- `update_task_pointers_from_data`: Calls a routine with 3841 in d1, loads six longwords from a table, clears registers a1 through a5, calls a routine at a0, writes 1 to a global variable, and updates five global longwords using the returned values and original table data before calling another routine.
- `calculate_and_call_task`: Calls a routine with 3861 in d1, performs arithmetic to calculate offsets into a table, loads a value from that table, calls a routine at a0, writes 1 to a global variable, and calls another routine before returning.
- `update_task_pointers`: Loads several longwords from a table into registers, calls a routine at a pointer in a0, updates a global variable with the result, and writes several longwords back to the table before looping or returning.
- `process_asset_table`: Clears several registers, calls a subroutine, calculates an offset into a table using a multiplication and a bit shift, and then calls another routine with several global pointers before updating multiple global variables and calling a final subroutine.
- `execute_callback_and_set_flag`: Loads a pointer into a register, calls a routine, executes a function pointer found in a register, sets a 16-bit value in a global variable to 1, and calls another routine before returning.
- `update_global_accumulators`: Sets a global variable to zero, calls another routine, and then adds the returned value from d1 to two different global variables.
- `calculate_and_store_time_data`: Calculates a value based on a table lookup and several bytes, then divides the result into four bytes stored at a pointer derived from d0.
- `calculate_offset_value`: Calculates a value by accessing a table using two offsets derived from global variables, performs arithmetic on bytes retrieved from the table, and returns the result.
- `configure_blitter_and_execute_task`: Calls two routines to process data, manipulates a global counter, configures hardware registers including the blitter and DMA controller, calls a routine with 3846 in d1, and updates a global variable with the value in d7 before calling another routine.
- `update_table_ranges`: Calls another routine twice with different parameters and different source and destination table addresses.
- `wait_for_status_bits`: Loops until specific bits in a global variable match two different required patterns.
- `update_table_elements`: Loops through a table, storing the lower 16 bits of a register at offset 6 and the upper 16 bits at offset 2 of each entry, while adding a value to the register in each iteration.
- `manage_library_handle`: Calls a routine via a pointer from a library base, and if the result is non-zero, calls another routine with two addresses in a6 and a1, then negates a word from a structure and calls a third routine before closing the library.
- `calculate_division_result`: If d1 is positive, it doubles a value from a global variable, optionally performs an XOR with a constant, stores the result back to the global variable, and returns the quotient and remainder in d0.
- `shift_list_elements_2`: Checks if a global variable is non-zero, then either calculates an offset into a table and calls another routine or calls another routine with a different table address.
- `shift_list_elements`: Searches a list for a value, and if a condition is met, shifts elements within the list and updates the list head and tail pointers.
- `check_interrupt_and_dispatch_audio`: Checks if bit 5 of the interrupt request register is set and if a global variable is greater than zero, then calls increment_and_dispatch_audio if both conditions are met; otherwise it returns.
- `reset_audio_dma_state`: Searches a table of 128 bytes for the largest value, then uses its position to index a second table and stores pointers into it in a third table.
- `increment_and_dispatch_audio`: Increments a byte in a global variable, compares it to another byte, and if the value is not below the threshold, clears the global variable and calls another routine if a specific byte is non-zero.
- `dispatch_audio_updates`: Calls update_audio_period_registers and then branches to another routine via a short jump.
- `update_audio_period_registers`: Sets the period registers for four audio channels by calling a command dispatcher for each channel.
- `update_audio_channels`: Fetches a byte from a table using an offset, calculates a 16-bit value by shifting and adding a constant, clears a global word, and calls another routine four times to update each audio channel register with specific data.
- `load_entity_data_from_table`: Fetches a pointer from a table using a value derived from bytes at offsets 2 and 0 of a structure in a6, then populates the structure with data from a secondary table in a3.
- `update_structure_and_dispatch_command`: Calculates and stores two new values in a structure at offsets 10 and 36, then updates a value at offset 14 from a table.
- `update_hardware_and_dispatch_command_2`: Searches a table for a value matching a masked word from a structure, then updates the DMA controller and various fields in a structure or global variable.
- `jump_to_command_handler`: Calls another routine and then branches to another routine.
- `update_hardware_and_dispatch_command`: Searches a table for a value matching a masked word from a structure, then updates the DMA controller and various fields in a structure or global memory based on a command byte.
- `update_audio_parameters`: Updates audio hardware registers with values from a table, modifies a global counter, and performs conditional logic based on a byte value and a global variable.
- `dispatch_structure_command_by_nibble`: Reads a 16-bit word from offset 2 of a structure in a6 and uses its lower 12 bits to determine a command type.
- `copy_word_to_offset`: Stores a 16-bit word from an offset of a pointer in a6 into offset 6 of a pointer in a5.
- `check_division_result`: Loads a byte from a constant offset, divides it by three, swaps the result, and branches based on whether the result is zero or two; otherwise, it masks the lower four bits of a byte from a structure in a6 and returns.
- `mask_and_branch`: Clears d0, reads a byte from offset 3 of a structure in a6 into d0, masks the lower 4 bits of d0, and branches to another routine.
- `copy_word_to_offset_2`: Loads a 16-bit word from an offset of 16 in a structure pointed to by a6 and stores it into offset 6 of a structure pointed to by a5.
- `search_table_for_value`: Calculates a table address using a byte from a structure, then loops up to 37 times to find a word in that table that is less than or equal to a value from a6, storing the result in d2.
- `store_d2_to_offset`: Stores the 16-bit contents of d2 into offset 6 of the address pointed to by a5.
- `subtract_and_clamp_word`: Subtracts a value from a word in a structure, clamps the result to a minimum of 289 if it is below that value, and stores the lower 12 bits of the result into a structure at offset 6 of a5.
- `add_and_clamp_word`: Adds a value to a word in a structure, clamps the result to a maximum of 856 if it is above that value, and stores the lower 12 bits of the result into a structure at offset 6 of a5.
- `search_table_for_command`: Calculates an offset into a table using a byte from offset 18 of a structure in a6, searches the table for a 16-bit value matching the lower 12 bits of a 16-bit word at offset 2 of the structure, and if found, stores the result at offset 24 of the structure, clears byte 22 of the structure, and sets byte 22 to 1 if the found value is less than the 16-bit word at offset 16 of the structure.
- `clear_word`: Clears a 16-bit word at offset 24 of the structure in a6 and returns.
- `update_and_clear_byte`: Reads a byte from offset 3 of a structure in a6 into d0; if d0 is not zero, it stores d0 in offset 23 of the structure and clears the byte at offset 3, otherwise it returns immediately.
- `calculate_index_and_store_2`: Updates a value in a structure based on a byte and a word, then performs a loop to find a value in a table based on a calculated index and stores the result into a structure.
- `calculate_index_and_store`: Subtracts a value from a word, performs a loop to find a value in a table based on a calculated index, and stores the result into a structure.
- `update_nibbles_in_structure`: Checks a byte at offset 3 of a structure in a6; if non-zero, it modifies the nibbles of a byte at offset 26 of the structure using bits from the byte at offset 3 and then writes the result back to offset 26.
- `calculate_scaled_value`: Extracts a value from a byte at offset 27 of a6, uses it to index a table via a4, and performs arithmetic based on a mode byte at offset 30 of a6 before storing the result in a structure at offset 6 of a5.
- `subtract_from_byte`: Tests a byte at an offset from a6 and branches if positive; otherwise, it subtracts d0 from 255, stores the result in d2, and branches.
- `move_byte_and_branch`: Moves a byte from d0 to d2 and branches to another routine.
- `subtract_and_advance_phase_2`: Multiplies a value derived from the low nibble of a byte at offset 26 of a6 by a byte from a table at offset 0 of a4, then adds the result to a word at offset 16 of a6 or branches.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `branch_to_subroutine`: Calls another routine and then branches to another routine.
- `call_update_and_branch`: Calls another routine and then branches to another location.
- `update_clamped_value`: Modifies a byte at offset 28 of a6 by combining nibbles from a byte at offset 3 of a6, then calculates a scaled value using a table and a mode byte at offset 30 of a6, finally clamping the result between 0 and 64 and storing it at offset 8 of a5.
- `clamp_byte_value`: Tests a byte at an offset from a6 and branches if positive; otherwise, it subtracts d0 from 255, stores the result in d2, and branches.
- `move_byte_and_branch_alt`: Moves a byte from d0 to d2 and branches to another routine.
- `clamp_and_update_value_2`: Multiplies a value derived from the low nibble of a byte at offset 28 of a6 by a byte from a table at offset 0 of a4, then adds the result to a word at offset 19 of a6 or branches.
- `clamp_and_update_value`: Calculates the difference between two values, clamps the result between zero and sixty-four, stores it in a structure, and updates a byte by merging a shifted nibble from a neighboring byte.
- `set_word_2`: If a byte at offset 3 of a6 is non-zero, it stores that byte at offset 32 of a6, then checks if a bit-shifted version of that byte is greater than or equal to a word at offset 8 of a6, otherwise it subtracts the shifted value from the word and adds a shifted version of the value to a longword at offset 4 of a6.
- `set_word`: Stores the word value 1 at offset 8 of the structure in a6 and returns.
- `decrement_and_clamp_byte_2`: Extracts the high nibble of a byte at offset 3 of a6, adds it to a byte at offset 19 of a6, clamps that byte to 64 if it exceeds that value, and finally stores the byte in a word at offset 8 of a5.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_and_store_byte`: Reads a byte from a structure, clamps its value to a maximum of sixty-four, and stores the result in both a byte and a word at different offsets.
- `calculate_and_store_byte`: Calculates a value by multiplying the result of a logical shift right by 4 on a byte from offset 3 of a structure in a6 by 10, adds the lower 4 bits of that byte to the result, and if the result is not greater than 63, stores it in a global byte and a subsequent global byte before returning; otherwise it branches to another routine.
- `update_status_byte`: Reads a byte from offset 3 of a structure in a6, and if it is not zero, clears a longword at a specific address and moves the byte to the address immediately preceding it.
- `dispatch_command_by_type`: Reads a byte from offset 2 of a structure in a6, masks it, and branches to one of several different routines based on its value, otherwise it branches to another routine.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `load_library_resource`: Opens a library, locks a file to examine its attributes, and then opens the file to read data into a newly allocated memory block.
- `close_library_and_load_registers`: Loads a library base into a6, loads a value into a1, calls a routine at an offset from a6, calls another routine via a branch to subroutine, and finally loads two 32-bit values from global variables into d0 and d1 before returning.
- `load_and_unpack_resource`: Reads a longword from a file, increments a value in a table, allocates memory, reads more data from the file, closes the file, calls two routines via a library base in a6, and stores three longwords into a structure before calling another routine and branching.
- `unpack_variable_length_bitstream_2`: Reads a bitstream from a source buffer and packs the extracted bits into a destination buffer using a variable bit-length scheme.
- `unpack_bitstream_data`: Calls another routine, then loops using d1 as a counter to shift d0 left and rotate d3 into d0, finally storing the resulting byte from d3 into a destination buffer pointed to by a2.
- `unpack_variable_length_bitstream`: Extracts bits from a source buffer in a0 and packs them into a destination buffer in a2 using a variable bit-length scheme.
- `initialize_bitstream_extraction`: Sets a counter in register d1 to 2, calls unpack_bitstream_to_buffer, increments register d2 by 7, and moves the result into register d3 before returning.
- `extract_bitstream_to_buffer`: Calls a routine to refill a bit register with 8 bits from a source, then extracts bits from that register to populate a destination buffer via a loop.
- `process_bit_shift`: Shifts the value in d0 right by one bit and calls another routine if the result is not zero, otherwise it sets d1 to 14 and returns.
- `unpack_bitstream_to_buffer`: Processes a bitstream by shifting bits from a source and calling shift_pointer_value and refill_bit_register to manage bit extraction, then copies bytes from a source buffer in a2 to a destination buffer by decrementing the source pointer and using a counter in d3.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `refill_bits_backward_until_count`: Repeatedly refills a bit register from the longword before the source pointer in a0 and shifts bits out, returning when a count in d1 runs out.
- `refill_bit_register`: Decrements a counter in d1, clears d2, and shifts d0 left by one bit, then loops using d1 as a counter or returns.
- `process_library_and_pointer`: Calls two subroutines, loads a library base into a6, and if a value in d0 is not zero, stores it in a global variable before calling another subroutine and branching.
- `lock_and_execute_task`: Stores a pointer in a global variable, calls another routine, and then enters a loop that calls a library routine via a6 using a pointer from a global variable until d0 is non-zero, at which point it stores d0 in a global variable, calls another routine, and finally calls a third routine before returning.
- `copy_bytes_decrementing`: Copies a byte from a source pointed to by a0 to an offset of -8 from that same pointer, then increments a0 and decrements a counter in d1, looping until the counter is no longer positive.
- `get_task_attribute`: Calls a library routine to find a task, reads a longword from an offset within that task, stores it in a global variable, and then sets the value at that same offset within the task to all ones.
- `find_task_and_link_node`: Loads a library base into a6, calls a routine via a6, stores the result in a global variable, sets a pointer in a1, and then performs a series of calls and branches involving a library base and a global variable.
- `initialize_pointer_tables`: Calls two different routines to process data at two different memory locations, then copies a longword from offset 50 of a structure to a new location and updates that same offset with the new address.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `update_table_elements_2`: Loops through a table, storing the lower 16 bits of a register at offset 6 and the upper 16 bits at offset 2 of each entry, while adding a value to the register in each iteration.
- `configure_blt_hardware`: Searches for a specific character in a string, calculates an address based on a value derived from that character, and then updates several Blitter hardware registers and DMA registers.
- `wait_multiple_raster_lines`: Loops a specified number of times, calling another routine in each iteration.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `load_and_unpack_resource_segment`: Opens a file and loads a segment into memory, then iterates through a data table to allocate memory blocks based on specific magic constants and copies unpacked bitstream data into those blocks.
- `process_data_table`: Calls a routine with several parameters, checks if a value at a pointer matches a constant, and if so, enters a loop that performs additions on a table of values using a counter and a secondary table.
- `execute_library_calls`: Calls three different library functions using offsets from a pointer in d0, then stores the result of the last call into a global variable and returns with a pointer shifted left by 2.
- `clear_memory_blocks`: Clears nine consecutive longwords starting from a specific memory location and then clears four additional longwords at a different location.
- `loop_unpack_bitstream_data`: Loads several longwords from memory into a structure pointed to by a0 and a5, calls another routine, and then enters a loop that decrements d0 and increments a4 until d0 reaches zero.
- `unpack_variable_bit_stream`: Reads a bitstream from a source buffer and unpacks it into a destination buffer based on a length and bit-width specified in a control structure.
- `fetch_shifted_bit_word`: Fetches a longword from the address in a0, shifts it left by one bit, and loops using d1 as a counter or returns.
- `close_file_and_loop`: Decrements a word in d1, clears d2, and shifts d0 right by 1; if d0 is not zero, it performs a loop using d1 as a counter, otherwise it calls a routine using a pointer from a constant to close a file and returns.
