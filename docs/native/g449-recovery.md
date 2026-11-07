# Game 449 (Wizkid): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00449). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 272 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 38 of 38 named (0 author symbols), 38 of 38 described (3 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 300 of 300 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 537238 bytes, 6 functions |
| Independent decoder cross-check (Ghidra against capstone) | 851 of 899 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 41673 |
| code | 2414 |
| data | 1457142 |
| padding | 50131 |
| unused | 250880 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| bowen | 2 | 228 | 11 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKid | 1 | 498 | 19 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCode#unpacked (raw code file, load address unknown) | 1 | 50 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo5E#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCoC0#unpacked (raw code file, load address unknown) | 1 | 50 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCo80#unpacked (raw code file, load address unknown) | 1 | 50 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemoC0#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo80#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:Wiz+Awave: IMP: File Imploder-packed, unpacked with ancient (3200 -> 4398 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:BDSprites: IMP: File Imploder-packed, unpacked with ancient (1404 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:eeorr1b.s: IMP: File Imploder-packed, unpacked with ancient (5098 -> 5716 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCode: IMP: File Imploder-packed, unpacked with ancient (14224 -> 30258 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:sfx1: IMP: File Imploder-packed, unpacked with ancient (6542 -> 9368 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizCry: IMP: File Imploder-packed, unpacked with ancient (3182 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:eeeek.s: IMP: File Imploder-packed, unpacked with ancient (3484 -> 4960 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:mod.hbd: IMP: File Imploder-packed, unpacked with ancient (27536 -> 40651 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:creeek.s: IMP: File Imploder-packed, unpacked with ancient (3108 -> 3236 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:cherry2c.s: IMP: File Imploder-packed, unpacked with ancient (4698 -> 5114 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:Arcade: IMP: File Imploder-packed, unpacked with ancient (1158 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:coininslot.s: IMP: File Imploder-packed, unpacked with ancient (6130 -> 6336 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:jahr.s: IMP: File Imploder-packed, unpacked with ancient (7800 -> 9803 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:letterbox2.s: IMP: File Imploder-packed, unpacked with ancient (4332 -> 5312 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AShip5: IMP: File Imploder-packed, unpacked with ancient (1860 -> 16000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:canopen2.s: IMP: File Imploder-packed, unpacked with ancient (2848 -> 3212 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:raspberry.s: IMP: File Imploder-packed, unpacked with ancient (14196 -> 14266 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo5E: IMP: File Imploder-packed, unpacked with ancient (73974 -> 135168 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:bell.s: IMP: File Imploder-packed, unpacked with ancient (6734 -> 6972 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:BDScreen: IMP: File Imploder-packed, unpacked with ancient (3828 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:chew2.s: IMP: File Imploder-packed, unpacked with ancient (1496 -> 2242 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCoC0: IMP: File Imploder-packed, unpacked with ancient (14226 -> 30258 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCo80: IMP: File Imploder-packed, unpacked with ancient (14224 -> 30258 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:bog_1shot.s: IMP: File Imploder-packed, unpacked with ancient (26096 -> 29472 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:UsingRam: IMP: File Imploder-packed, unpacked with ancient (4914 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:DiskPrompt: IMP: File Imploder-packed, unpacked with ancient (1266 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:doggie.s: IMP: File Imploder-packed, unpacked with ancient (1788 -> 1842 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:splash3.s: IMP: File Imploder-packed, unpacked with ancient (2484 -> 2680 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:Stage: IMP: File Imploder-packed, unpacked with ancient (6226 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:Arcade+s: IMP: File Imploder-packed, unpacked with ancient (7378 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:touchtone1.s: IMP: File Imploder-packed, unpacked with ancient (2098 -> 3650 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:pull.s: IMP: File Imploder-packed, unpacked with ancient (2758 -> 3076 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:thunder.s: IMP: File Imploder-packed, unpacked with ancient (10650 -> 12084 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:cash.s: IMP: File Imploder-packed, unpacked with ancient (4778 -> 5954 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:StageIntro: IMP: File Imploder-packed, unpacked with ancient (13228 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemoC0: IMP: File Imploder-packed, unpacked with ancient (77500 -> 155648 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:door.s: IMP: File Imploder-packed, unpacked with ancient (946 -> 982 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:pop1.s: IMP: File Imploder-packed, unpacked with ancient (1414 -> 1662 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:keydisk: high-entropy data file (7.77 bits/byte), classified as packed data
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo80: IMP: File Imploder-packed, unpacked with ancient (78132 -> 155648 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:draclafeko.s: IMP: File Imploder-packed, unpacked with ancient (11902 -> 15446 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:1812bit2: IMP: File Imploder-packed, unpacked with ancient (6664 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:mod.1812: IMP: File Imploder-packed, unpacked with ancient (116956 -> 151496 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:TheWizCastA: IMP: File Imploder-packed, unpacked with ancient (13318 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:fuse1b.s: IMP: File Imploder-packed, unpacked with ancient (2132 -> 2530 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:TheWizCastB: IMP: File Imploder-packed, unpacked with ancient (11712 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:magic1d.s: IMP: File Imploder-packed, unpacked with ancient (11776 -> 13922 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:chew3.s: IMP: File Imploder-packed, unpacked with ancient (1464 -> 2128 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:coininslot2.s: IMP: File Imploder-packed, unpacked with ancient (5262 -> 5828 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:ghost2b.s: IMP: File Imploder-packed, unpacked with ancient (4340 -> 6538 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:BootCamp: IMP: File Imploder-packed, unpacked with ancient (17656 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:tiles: IMP: File Imploder-packed, unpacked with ancient (13328 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:wellscreen: IMP: File Imploder-packed, unpacked with ancient (13246 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:scmask: IMP: File Imploder-packed, unpacked with ancient (200 -> 8000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:wizball: IMP: File Imploder-packed, unpacked with ancient (5790 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:features+top: IMP: File Imploder-packed, unpacked with ancient (10698 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.jail: IMP: File Imploder-packed, unpacked with ancient (25660 -> 40506 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:circus: IMP: File Imploder-packed, unpacked with ancient (13054 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.doggi2: IMP: File Imploder-packed, unpacked with ancient (18788 -> 29655 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:logo: IMP: File Imploder-packed, unpacked with ancient (8992 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata10: high-entropy data file (7.81 bits/byte), classified as packed data
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.neli2: IMP: File Imploder-packed, unpacked with ancient (17960 -> 38903 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:BoatRace: IMP: File Imploder-packed, unpacked with ancient (22684 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:vol: IMP: File Imploder-packed, unpacked with ancient (22820 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:dreamseq: IMP: File Imploder-packed, unpacked with ancient (5422 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:graveyard: IMP: File Imploder-packed, unpacked with ancient (16152 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.ocean3: IMP: File Imploder-packed, unpacked with ancient (18378 -> 35888 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.jng6: IMP: File Imploder-packed, unpacked with ancient (41256 -> 57478 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:cave+well+shop: IMP: File Imploder-packed, unpacked with ancient (9326 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:jitbsprites: IMP: File Imploder-packed, unpacked with ancient (4878 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ship: IMP: File Imploder-packed, unpacked with ancient (14944 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:dogroom: IMP: File Imploder-packed, unpacked with ancient (5282 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:wizsprites: IMP: File Imploder-packed, unpacked with ancient (13182 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:shop: IMP: File Imploder-packed, unpacked with ancient (21884 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.wiz8: IMP: File Imploder-packed, unpacked with ancient (18276 -> 38422 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel0: IMP: File Imploder-packed, unpacked with ancient (4178 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata0: IMP: File Imploder-packed, unpacked with ancient (478 -> 814 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.grave: IMP: File Imploder-packed, unpacked with ancient (27876 -> 40650 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel1: IMP: File Imploder-packed, unpacked with ancient (6444 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata1: IMP: File Imploder-packed, unpacked with ancient (764 -> 2580 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel2: IMP: File Imploder-packed, unpacked with ancient (4426 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata2: IMP: File Imploder-packed, unpacked with ancient (580 -> 1534 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel3: IMP: File Imploder-packed, unpacked with ancient (2142 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata3: IMP: File Imploder-packed, unpacked with ancient (904 -> 2708 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel4: IMP: File Imploder-packed, unpacked with ancient (8012 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata4: IMP: File Imploder-packed, unpacked with ancient (686 -> 1720 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:tree: IMP: File Imploder-packed, unpacked with ancient (19004 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel5: IMP: File Imploder-packed, unpacked with ancient (6866 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata5: IMP: File Imploder-packed, unpacked with ancient (896 -> 2766 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel6: IMP: File Imploder-packed, unpacked with ancient (14780 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel7: IMP: File Imploder-packed, unpacked with ancient (9406 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata7: IMP: File Imploder-packed, unpacked with ancient (554 -> 1458 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:oxygum: IMP: File Imploder-packed, unpacked with ancient (14228 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel8: IMP: File Imploder-packed, unpacked with ancient (1304 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata8: IMP: File Imploder-packed, unpacked with ancient (902 -> 2368 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:xlevel9: IMP: File Imploder-packed, unpacked with ancient (15860 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:ldata9: IMP: File Imploder-packed, unpacked with ancient (776 -> 1882 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:face: IMP: File Imploder-packed, unpacked with ancient (13542 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:jail: IMP: File Imploder-packed, unpacked with ancient (14968 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:drawup: IMP: File Imploder-packed, unpacked with ancient (17194 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.reg6: IMP: File Imploder-packed, unpacked with ancient (23464 -> 41798 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:Aliens: IMP: File Imploder-packed, unpacked with ancient (9870 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:Bogs: IMP: File Imploder-packed, unpacked with ancient (4516 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:mod.drums: IMP: File Imploder-packed, unpacked with ancient (16106 -> 24726 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:stiles: IMP: File Imploder-packed, unpacked with ancient (2200 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 2 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:wizmap: IMP: File Imploder-packed, unpacked with ancient (14578 -> 40000 bytes)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCode#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo5E#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCoC0#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCo80#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemoC0#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- wizkid (1992)(ocean)(m3)[cr tkk][m bowen](disk 1 of 2)/Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo80#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 6 never-loaded region(s), 537238 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### bowen

- `initialize_library_headers`: Verifies the presence of required librarys by checking for specific magic strings in the library headers and parses a resource table.
- `empty_routine`: the routine contains no executable instructions or functional logic.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations.
- `empty_routine_3`: the routine contains no executable instructions.
- `empty_routine_4`: the routine contains no valid instructions and performs no operations.
- `empty_routine_5`: This routine contains no executable instructions.
- `execute_invalid_instruction`: This routine stops execution due to encountering bad instruction data.
- `empty_routine_6`: The routine contains no executable instructions.
- `unknown_routine`: The routine contains no recoverable instructions; the disassembly is truncated and no operations can be determined.
- `execute_invalid_instruction_2`: The routine contains a single bad instruction that prevents further execution.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKid

- `setup_system_resources`: Performs library opening and resource unpacking while modifying a global jump table.
- `modify_jump_table`: Based on a global variable, this routine writes specific values to a large jump table and then performs an indirect call.
- `unpack_lpm_resource`: Checks for an illegal file magic number and, if present, decompress or deallocate a resource or chunk of data.
- `initialize_hardware_and_memory`: Clears a block of memory, writes a routine address to a hardware register or a table, and modifies several hardware registers including DMA and interrupt disable bits.
- `set_bit_and_fill`: Sets the high bit of a word at a certain memory location and writes a maximum signed word value to several consecutive memory locations.
- `set_values_to_max`: stores a maximum signed word value into three consecutive word-sized locations relative to a pointer.
- `initialize_memory_values`: Stores a specific constant value into a memory location and sets certain memory or register-relative offsets to zero.
- `set_bit_in_memory`: sets the high bit of a word at a memory location pointed to by a register.
- `wait_for_null_byte`: loops until a null byte is encountered at a specific offset relative to a pointer.
- `write_constant_to_memory`: stores a constant byte value into a word-sized location relative to a pointer.
- `write_space_to_memory`: Writes a space character to a memory location relative to a global pointer.
- `check_hardware_and_set_value`: Checks certain bits in a hardware register and, if a condition is met, writes a value to a memory location relative to a pointer.
- `modify_byte_and_loop`: performs bitwise operations on a byte in memory, updates a global variable based on another memory value, and executes a fixed-count loop.
- `find_string_null_terminator`: Searches through a string in memory until a null terminator or a specific condition is met, potentially calling another routine during the process.
- `copy_string_to_memory`: compares a string against a global variable and then copies bytes from a source string to a destination using a fixed offset increment.
- `modify_and_format_buffer`: Checks a global length or value, applies an XOR-based modification to a memory buffer, and then converts a numeric value into an ASCII-encoded string stored back in that buffer.
- `format_string_with_initialization`: Initializes a memory location to zero and then executes a routine that converts numeric data into an ASCII string format in a global buffer.
- `process_string_and_format_output`: Performs an XOR-based modification on a memory buffer if a certain value is present, then proceeds to format numeric data into an ASCII string.
- `format_numeric_string`: Iteratively calculates digits using division and modulo, then stores the ASCII character representation of those digits into a global buffer.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 2 of 2).adf:bootblock

- `check_library_function_result`: calls a function from a library and returns a certain value based on whether the return or a comparison or that function equals zero.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCode#unpacked (raw code file, load address unknown)

- `unidentified_raw_routine`: this routine contains fifty instructions and acts as the start of a raw code region.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo5E#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCoC0#unpacked (raw code file, load address unknown)

- `unidentified_raw_routine_2`: this routine contains fifty instructions and acts as the start of a raw code or code-section region.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:AstCo80#unpacked (raw code file, load address unknown)

- `modify_memory_via_pointer`: stores a byte from a register into a memory location or a table using a pointer.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemoC0#unpacked (raw code file, load address unknown)

- `execute_single_instruction`: This routine contains a single instruction and lacks any visible calls or register-modifying side effects.

### Wizkid (1992)(Ocean)(M3)[cr TKK][m Bowen](Disk 1 of 2).adf:WizKidDemo80#unpacked (raw code file, load address unknown)

- `execute_single_instruction_2`: This routine consists of one instruction and does not perform any library calls or register-based operations.
