# Game 541 (Nigel Mansell World Championship): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00541). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1882048 top-level bytes in 319 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 317 of 317 named (22 author symbols), 317 of 317 described (38 by rule, 274 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 15608 of 15608 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 36468 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 30176 of 30176 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 585060 |
| code | 99402 |
| data | 1056111 |
| padding | 49315 |
| unused | 92160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell | 5 | 235 | 8 |
| 11 | 1 | 2 | 1 |
| Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:installharddisk | 2 | 607 | 22 |
| diskfont.library | 2 | 4480 | 170 |
| Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:Crystal | 2 | 85 | 3 |
| Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:iconx | 3 | 832 | 39 |
| reward.am | 1 | 601 | 30 |
| Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1218 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22106 | 24 |
| panel.raw#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| advert10.raw#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/carparts.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5591 -> 18192 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/gremlin.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5133 -> 40000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/highlite.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1730 -> 17880 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/control.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3107 -> 10880 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/cont_ic.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3494 -> 9880 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/gamepics.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10973 -> 32292 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/tunebits.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4591 -> 16540 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/globebas.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3061 -> 12510 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/flags.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (867 -> 5760 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/ren3.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2439 -> 32000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/titlescr.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5165 -> 10620 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/ren1.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5224 -> 32000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/gamesel.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7011 -> 24960 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/helmets.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2395 -> 6192 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/credits.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (12037 -> 40000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/world.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (18054 -> 62550 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/cars.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1020 -> 2688 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/congrats.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (20818 -> 40000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/weaticon.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1267 -> 8064 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/ren2.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2301 -> 32000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/graphics/circsel.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6512 -> 25090 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/maps/COUNTRIES.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2185 -> 5850 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/maps/trax.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2218 -> 6700 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/noises/mansell.mod: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (112504 -> 155160 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell1/noises/mod.mansell_equip: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (96964 -> 135020 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/panel.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4887 -> 25140 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/y-1arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4667 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/b-2arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4973 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/lhard.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1385 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/control.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3107 -> 10880 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/carback2.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5044 -> 13690 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/tyrechoo.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2116 -> 12390 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/mansell.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1520 -> 6450 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/g-1arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3923 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/b-cup.msk: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1127 -> 27000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/o-cup.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7468 -> 27000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/o-2arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5479 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert8.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2260 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/qualify.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2895 -> 11760 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert6.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4012 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/b-1arm.msk: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (662 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/o-1arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4663 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/sprites.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4353 -> 11856 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/slights.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (999 -> 3260 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/sparks.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (985 -> 3240 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/lsbend.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1056 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/carback4.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4978 -> 13690 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert10.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (786 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert2.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2939 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/g-cup.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6700 -> 27000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/chekflag.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3076 -> 10400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert45.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2559 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/qualcars.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (988 -> 4608 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/PITSSEQ.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4593 -> 12936 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/b-2arm.msk: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (758 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/carback1.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5294 -> 13690 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/b-cup.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7100 -> 27000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/rsign.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (729 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/pits.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (730 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert9.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6122 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/tyrewall.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1296 -> 7180 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/y-cup.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7396 -> 27000 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/rsbend.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1047 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/haybail.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6051 -> 12800 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/lights.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (660 -> 2510 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/b-1arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4278 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/clouds.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (2817 -> 4216 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/rhard.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1386 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert5.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3809 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/carfront.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3560 -> 7130 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/lsign.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (719 -> 7888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/y-2arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5284 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/carbits.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6551 -> 20780 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert11.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1211 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert3.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3520 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/g-2arm.blt: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4491 -> 20400 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/carback3.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4928 -> 13690 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert1.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6916 -> 11328 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK10.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1651 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track3.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (663 -> 1472 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track7.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (613 -> 3144 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track10.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (621 -> 2112 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK1.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1234 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track14.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (605 -> 3272 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track1.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (606 -> 1472 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track5.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (522 -> 3528 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track12.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (601 -> 3528 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track3.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (599 -> 3144 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track10.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (556 -> 3016 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK8.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1008 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track1.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (753 -> 5064 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/doitsu.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1169 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track8.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (758 -> 1664 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK6.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1479 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track15.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (663 -> 2368 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track9.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (673 -> 4936 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK13.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1197 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track6.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (669 -> 1344 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK4.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1483 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track13.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (540 -> 1408 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK11.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1432 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track4.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (762 -> 1536 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK2.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1347 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track11.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (549 -> 1472 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track15.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (497 -> 3272 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track2.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (546 -> 1408 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track6.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (597 -> 4424 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK0.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1401 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track13.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (544 -> 3144 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track4.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (506 -> 2760 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track11.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (579 -> 2888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK9.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1575 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track2.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (544 -> 3144 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/NIHON.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1195 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track9.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (597 -> 1472 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK7.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1263 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track16.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (609 -> 1344 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track0.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (470 -> 2888 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK14.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1365 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track7.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (546 -> 1280 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track14.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (732 -> 1664 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track8.bin: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (517 -> 3016 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK12.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (1543 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track5.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (614 -> 1536 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/TRACK3.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (983 -> 3640 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/tracks/track12.crd: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (637 -> 2432 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/noises/renault1.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11426 -> 11554 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/noises/prang3.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4913 -> 6164 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/noises/prang1.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6352 -> 6680 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/noises/swish.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4754 -> 5510 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/noises/RAIN.RAW: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11470 -> 11922 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/noises/prang2.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4100 -> 4200 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track10.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3219 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track1.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9378 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track8.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (8641 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track15.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6725 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track6.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5537 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track13.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3574 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track4.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (7202 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track11.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (9589 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track2.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5825 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track0.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6626 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track9.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3343 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track7.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4351 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track14.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (6050 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track5.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4040 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track12.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4849 -> 20480 bytes)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/horizons/track3.raw: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (3172 -> 20480 bytes)
- runtime segment an address: 597 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14743 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/panel.raw#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 2 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 2 of 2).adf:mansell2/graphics/advert10.raw#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf: blocks 634-634 (bytes 324608-325120) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- nigel mansell world championship (1992)(gremlin)[cr csl](disk 1 of 2)/Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 36468 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 3 packed track region(s), 10752 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_graphics_and_audio_state` (runtime segment an address): Updates the display using the Blitter and manages audio output by clearing DMA and volume registers. Kind: disk hardware/integrity check. Pass path: The routine likely validates the presence or specific timing/signature of the CIAB_PRB hardware; a successful check allows the execution to proceed to the main game/application logic, whereas a failure would trigger the 'unreachable' blocks.

## Functions


### Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:mansell

- `copy_memory_block_structure`: Iterates through a linked list of memory blocks to identify specific data structures and performs a block-based memory copy operation.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `fill_figure_table`: Reads a count, fills a table with values from a stack buffer, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `traverse_and_free_memory`: Iterates through a linked list of memory blocks, performing calculations on pointers and calling a routine to free memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### 11

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:installharddisk

- `codesection`: Jump stub: passes control straight on to main.
- `main`: A main routine that executes a sequence of steps including setup, screen displays, path validation, and processing a list before closing down.
- `IntroScreen`: Prints text to the screen, enables gadgets, and waits for an event.
- `PathSelect`: Clears a window and displays text and gadgets to allow a user to select a destination path.
- `ValidatePath`: Checks a pathname for a valid drive designation and directory structure using system calls.
- `ProcessList`: Iterates through a list of files and directories, performing disk access, directory creation, or file copying based on the character values encountered.
- `GetDisk`: Attempts to lock a disk and displays a prompt if the disk is not present.
- `MakeSubDir`: Attempts to create a new directory and appends a forward slash to a path string if successful.
- `ParentDir`: Modifies a global path string by searching for and truncating characters at directory separators or drive delimiters.
- `CopyAFile`: Copies a string from one location to another and then attempts to perform a file copy operation.
- `Copy`: Performs file operations and handles error reporting if reading or writing fails.
- `FinalScreen`: Clears the window and displays a completion message with an acknowledgment gadget.
- `GadgetsOn`: Calls two specific offsets within the intuition library to enable gadgets.
- `EraseWindow`: Draws an image to clear a window and then destroys gadgets.
- `WaitEvent`: Repeatedly retrieves and replies to messages from a queue until no more messages are available.
- `Print`: Library stub: loads its arguments and the intuition library base, calls PrintIText and returns its result to the caller.
- `Setup`: Initializes system libraries, opens a screen and window, loads a font, and allocates a memory buffer.
- `KillGadgets`: Calls a specific offset within the intuition library to destroy gadgets.
- `CloseDown`: Unlocks directories, restores a task window, closes fonts, and releases various system resources and libraries.
- `ILBM2RAW`: Parses an ILBM file by locating specific chunks, converts packed color data into a different format, and unpacks bitplanes into a destination buffer.
- `UnPackRow`: Decompresses a run-length encoded byte stream from a source pointer into a destination buffer.
- `palette`: The routine is empty due to corrupted instruction data at its entry point.

### diskfont.library

- `library_return_error`: Library code that returns a constant value of negative one.
- `find_matching_font_entry`: Searches a list for an entry whose key and low flag bits match the given record.
- `compare_packed_halves_equal`: Library code that compares two 32-bit values by cross-multiplying their 16-bit halves.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `initialize_library_structure`: Library code that populates a structure with values obtained from multiple sequential function calls.
- `execute_cleanup_sequence_2`: Executes a sequence of four function calls retrieved from offsets within a structure.
- `execute_cleanup_sequence`: Library code that executes a specific sequence of four function calls via pointers stored in a structure.
- `cleanup_library`: As library code, this routine calls a cleanup function and performs several other function calls via pointers.
- `execute_library_cleanup`: As library code, this routine calls a cleanup function via a pointer and then performs memory deallocation.
- `library_cleanup_routine`: As library code, this routine calls a function via a pointer in a global structure and then performs a memory free operation.
- `manage_resource_list`: As library code, this routine iterates through a linked list to remove elements and manages a status flag.
- `update_status_flag`: As library code, this routine either performs a series of function calls to manage a list or sets a high bit in a byte within a structure.
- `return_from_subroutine`: This routine is an empty function that performs no operations and immediately returns to its caller.
- `set_byte_flag`: As library code, this routine sets the highest bit of a byte at a specific offset from a pointer.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `decrement_and_conditional_call`: Library code that decrements a short integer at a fixed offset and calls another routine if the new value is zero and a specific bit is set.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `allocate_glyph_metadata_2`: Calculates the memory requirements for a font glyph based on its dimensions and attributes, then allocates a buffer and populates a metadata structure with dimensions, string data, and pixel buffer pointers.
- `allocate_glyph_metadata`: Calculates the required memory size for a font glyph based on its dimensions and attributes, then allocates a buffer and populates a metadata structure with dimensions, string data, and pixel buffer pointers.
- `initialize_glyph_metadata_structure`: Calculates the memory requirements for a font glyph and populates a metadata structure with dimensions, string data, and pixel buffer pointers.
- `allocate_glyph_bitmap_buffer_2`: Calculates the required memory for a font bitmap by iterating through glyph data and then allocates a buffer to store the pixel data.
- `allocate_glyph_bitmap_buffer`: Calculates the required memory for a font bitmap by iterating through glyph data and then allocates a buffer to store the pixel data.
- `calculate_glyph_bounding_box`: Iterates through a linked list of glyph data structures to calculate the total dimensions and memory requirements.
- `update_font_glyph_data`: Iterates through a source data buffer to calculate and store modified pixel or attribute values into a destination font table.
- `calculate_font_glyph_dimensions`: Iterates through a linked list of data structures to calculate the total size and copy elements into a new memory buffer.
- `reconstruct_font_glyph_data`: Iterates through a linked list of data structures to calculate total size and copy elements into a new memory buffer.
- `calculate_callback_size`: Library code that traverses a linked list of structures to calculate a total size.
- `process_callback_chain`: Library code that traverses a linked list of structures, calculates a total size, and copies the elements into a new memory buffer.
- `execute_callback_chain_3`: Library code that clears two memory locations and then calls a function via a pointer within a structure.
- `execute_callback`: As library code, this routine calls another routine and returns zero.
- `process_callback_chains`: As library code, this routine iterates through a linked list of pointers to execute a sequence of function calls.
- `execute_callback_chain_2`: Iterates through a linked list of function pointers stored in a structure and executes each one.
- `execute_callback_chain`: As part of a library, this routine checks a pointer and executes a sequence of function calls through offsets in a structure.
- `negate_library_result_if_negative`: Calls a library routine for a word result and negates that result when the input word is negative.
- `multiply_signed_32bit_to_64bit`: Multiplies two signed 32-bit values by splitting each into 16-bit halves, sums the partial products, and fixes the sign of the 64-bit result.
- `update_glyph_string_buffer`: Compares a new string against an existing buffer to determine the required length and then shifts or copies the string data into a memory block.
- `recalculate_font_buffer_offsets`: Compares two strings and, upon a match, updates a font data buffer by calculating new memory offsets and copying character data.
- `compare_and_update_font_buffer`: Compares two strings and, if they match, updates a font data buffer by calculating new offsets and copying character data.
- `realign_glyph_memory_buffer_2`: Iterates through a linked list of data blocks to shift memory offsets and perform byte-by-byte copies of glyph data.
- `realign_glyph_memory_buffer`: Iterates through a linked list of data blocks to shift memory offsets and perform byte-by-byte copies of glyph data.
- `reorganize_font_buffer_data`: Iterates through a list of data blocks to perform memory shifts and byte-by-byte copies.
- `modify_font_buffer`: This routine manipulates a buffer of font data by shifting or copying bytes based on a specified length.
- `realign_font_glyph_buffer`: Iterates through a linked list of data blocks to shift memory offsets and perform byte-by-byte copies of glyph data.
- `update_font_glyph_buffer`: Updates a font structure by adjusting memory offsets and copying glyph data from a source buffer.
- `populate_font_data_2`: Library code that copies a block of bytes from a source to a destination and updates a structure with metadata.
- `populate_font_data`: As library code, this routine copies data into a destination structure and may expand a buffer using a linked list.
- `copy_linked_list`: This library routine copies data from one list to another by following pointers until a null pointer is encountered.
- `store_word_to_offset`: This library routine stores a 16-bit value into a specific offset within a structure.
- `mask_font_glyph_byte`: Fetches a byte from a pointer located within a structure and stores it at a destination offset after clearing the most significant bit.
- `return_zero`: This library routine returns zero.
- `revalidate_font_glyph_list`: Iterates through a linked list of font glyph structures to check specific bit flags and update internal counters.
- `validate_font_glyph_list`: Iterates through a linked list of font glyph structures to check specific bit flags and update internal counters.
- `iterate_font_glyph_data`: Traverses a linked list of font glyph structures to check specific bit flags and update internal counters.
- `traverse_font_glyph_list`: Iterates through a linked list of font glyph structures to check specific bit flags and update internal counters.
- `render_font_glyphs`: Iterates through a list of character data to draw glyphs into a buffer.
- `update_font_glyph_metadata`: Iterates through a sequence of glyph data to update character properties and bounds based on specific bitwise flags.
- `search_font_table_2`: Iterates through a font data structure to find a matching character sequence.
- `validate_glyph_metadata`: Iterates through a sequence of glyph data to verify character properties, bounds, and specific bitwise flags.
- `process_font_glyph_metadata`: Iterates through a sequence of glyph data to verify character properties and bounds.
- `validate_font_glyph_data`: Iterates through a sequence of glyph data to verify character properties and bounds.
- `iterate_font_glyph_table`: Iterates through a table of glyph data to validate and update character properties.
- `return_zero_value`: Returns a constant value of zero immediately without performing any other operations.
- `load_font_configuration`: As library code, this routine iterates through a list of items to read and store configuration parameters into a structure.
- `read_template_arguments`: This library routine reads X and Y DPI, dot pitch, and symbol set arguments through library calls, storing the values into a structure.
- `validate_and_load_font_metadata`: Checks the length of a font identifier string and iterates through a linked list of font nodes to extract and copy specific glyph metadata and pixel data into a new structure.
- `import_font_resource`: Parses a font data structure to extract character glyphs and metadata.
- `reallocate_font_buffer`: This routine manages the memory allocation for font data by checking current capacity and expanding the buffer if necessary.
- `parse_font_glyph_data`: Iterates through a linked list of font nodes to extract character glyph metadata and pixel data.
- `copy_font_nodes`: This library code traverses a linked list to copy data from nodes into a new memory area and then iterates through the original list to release resources.
- `free_font_list`: As library code, this routine traverses a linked list to copy its nodes and then iterates through the list to release resources via several function calls.
- `free_font_list_and_return`: Frees each node of a temporary linked list through library calls, releases the working resources, and returns the stored head pointer.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a7).
- `call_library_vector_690`: Library stub: loads a library base from a global variable and calls the library function at offset minus 690 of that base, returning its result to the caller.
- `update_font_resource_buffer`: Checks the validity of a font resource and performs a relative pointer adjustment on a buffer of data.
- `manage_resource_state`: This library code performs conditional logic to update pointers, release pending resources, and manage internal state based on the results of library function calls.
- `release_pending_resource`: As library code, this routine calls a library function to process a resource, then conditionally releases a pending resource and clears a pointer if a specific flag is set.
- `return_stored_result_after_release`: Releases a pending resource when the flag is set and returns the stored result value.
- `sync_glyph_metrics_and_bitmap`: Updates a font character's metric table with width and offset values and performs a bitwise transfer of glyph bitmap data into a destination buffer.
- `update_glyph_bounding_box`: Calculates the dimensions and positioning of a character glyph by comparing pixel bounds and updates the font structure's metrics.
- `update_font_glyph_metrics`: Calculates and updates bounding box dimensions and character positioning data within a font structure.
- `unpack_compressed_font_data`: Iterates through a source buffer to extract bit-packed data using bitwise shifts and masks, storing the resulting values into a destination table.
- `process_bitstream_data`: As library code, this routine performs bitwise shifts and masks to extract data from a source buffer and stores the results into a destination table.
- `extract_bitstream_data`: This library code performs bitwise shifts and masks to extract data from a source buffer and stores the results into a destination table.
- `update_glyph_metrics`: As library code, this routine writes width and offset values into a table based on a character code, potentially clearing existing entries if a condition is met.
- `store_saved_glyph_offsets`: Writes the saved offset and width values into the metric tables at the given slot and returns the result.
- `reset_font_metadata_table`: Clears a global table of font entries and populates a metadata structure by querying font dimensions and character properties.
- `initialize_font_resource_metadata`: Clears a local table of entries and populates a metadata structure by querying font dimensions and character properties.
- `populate_font_metadata`: Extracts font dimensions, character counts, and extent information from a font resource to update a local metadata structure.
- `validate_font_metrics`: Checks font dimensions and character counts against specific thresholds to set status flags.
- `update_font_metadata_2`: Updates a font structure's dimensions and bounding box information by querying glyph data and iterating through character metrics.
- `calculate_font_bounding_box_2`: Determines the minimum and maximum glyph dimensions within a font structure.
- `update_font_metrics_2`: Iterates through a list of glyph dimensions to calculate the minimum and maximum width and height required for a font.
- `calculate_font_bounding_box`: Iterates through a list of glyph dimensions to determine the minimum and maximum width and height required for a font.
- `initialize_font_metadata_2`: Initializes a font metadata structure by calculating string lengths, setting scaling values, and populating dimensions.
- `update_font_metadata_table`: Updates a font metadata structure by copying dimensions, calculating string lengths, and populating fixed-point scaling values.
- `initialize_font_metadata`: Initializes a font structure by copying string data, setting fixed-point dimensions, and populating a metadata table.
- `search_and_initialize_font`: Iterates through a font entry table to find a matching font based on specific attributes and validates its integrity.
- `validate_and_initialize_font`: Iterates through a list of font entries to find a match based on specific attributes and validates the font's integrity.
- `validate_and_initialize_font_2`: Iterates through a list of font entries to find a match based on specific attributes and validates the font's integrity.
- `initialize_font_glyph_metrics`: Initializes a font structure by resetting various extent and bounding box fields.
- `calculate_glyph_metrics`: Iterates through a character set to determine the vertical extents and bounding box dimensions of each glyph.
- `calculate_font_metrics`: Iterates through a character data table to determine bounding box dimensions, widths, and height statistics.
- `update_font_metrics`: Iterates through a table of font character data to calculate and update bounding box dimensions, widths, and height statistics.
- `compute_glyph_bounding_box`: Iterates through character data to determine the minimum and maximum vertical extents and calculates the resulting bounding box dimensions.
- `build_glyph_path_structure`: Allocates and initializes a complex data structure representing a glyph's geometric path.
- `initialize_glyph_path_metadata`: Initializes a data structure containing glyph path information, including vertex counts and coordinate offsets.
- `calculate_glyph_path_midpoints`: Iteratively calculates midpoint coordinates for a glyph's path data by processing vertex pairs.
- `refine_glyph_path_coordinates`: Iteratively calculates and updates midpoint coordinates for a glyph's path data.
- `calculate_font_glyph_metrics`: Iterates through a list of character data to calculate bounding box dimensions and glyph offsets.
- `subdivide_glyph_path_points`: Recursively calculates intermediate coordinate points by performing midpoint divisions between existing points in a glyph's path data.
- `interpolate_glyph_step_values_2`: Iteratively calculates intermediate coordinate and metric values by performing midpoint divisions between two sets of points.
- `interpolate_glyph_step_values`: Iteratively calculates intermediate coordinate and metric values by performing midpoint divisions between two sets of points.
- `interpolate_font_glyph_coordinates`: Calculates intermediate coordinate values by performing a mid-point division between two points to facilitate smooth transitions or sub-pixel positioning.
- `interpolate_font_glyph_metrics`: Calculates sub-pixel or fractional character spacing by interpolating between two sets of glyph metric values.
- `interpolate_glyph_subdivision_points`: Iteratively calculates intermediate coordinate and metric values by performing midpoint divisions between two sets of points.
- `calculate_font_extent_and_finalize`: Calculates the total width of a font based on character counts and updates the header extent.
- `update_font_metadata`: This library routine modifies several fields in a structure, including a counter and bit flags, before recursively calling a font matching routine.
- `find_matching_font`: This library routine searches for a font entry by matching a key and flag bits, then performs conditional recursive calls based on the result.
- `return_zero_value_2`: This library routine returns a zero value to the caller.
- `find_font_entry_by_name`: Copies the requested name into a buffer and walks the font list through library calls.
- `find_font_resource`: Searches for a font resource by iterating through a linked list of entries and matching specific attribute and identifier criteria.
- `search_font_resource_list`: Iterates through a linked list of font entries to find a match based on specific attribute and identifier criteria.
- `search_font_list`: This library routine iterates through a list of font entries to find a match based on specific attribute and identifier criteria.
- `find_font_entry`: Library code that iterates through a font list to find an entry matching a specific name and criteria.
- `iterate_font_list`: Iterates through a list of font entries to find a match based on specific attribute and identifier criteria.
- `traverse_font_bit_tree`: Performs a recursive depth-first traversal of a bit-mapped tree structure to process font data.
- `process_font_bits`: This library routine performs a nested loop to manipulate bits in a byte and updates a table based on the results of recursive calls.
- `iterate_font_bits`: This library routine performs a nested loop to process bitwise data, modifying a byte in a table based on specific bit flags.
- `return_zero_restore_regs_unlink`: Sets D0 to zero, restores D6-D7/A2-A3 from the stack, unlinks the A5 frame, and returns.
- `recursively_search_font_entries`: This library routine iterates through a table to find the entry with the highest value by calling a comparison function.
- `find_maximum_font`: Library code that scans a table to identify an entry with the highest value.
- `scan_font_table`: Library code that iterates through a table to find the entry with the maximum value.
- `search_font_table`: This library routine iterates through a table to find an entry with a maximum value and performs conditional operations on the results.
- `search_max_font`: Library code that scans a table to identify an entry with the highest value.
- `find_max_font_entry`: This library routine scans a table to identify an entry with the highest value, handling special termination markers.
- `synchronize_font_resource_state`: Validates and synchronizes font resource properties, including dimensions and character data, while managing internal status flags and memory.
- `update_font_resource_state`: Validates and synchronizes font resource properties, including dimensions and character data, while managing internal status flags and memory pointers.
- `resolve_font_resource`: Attempts to locate and validate a font resource by checking dimensions and character properties against a requested specification.
- `manage_font_resource_2`: Library code that performs font resource allocation and lookup by traversing a bit-mapped tree and managing conditional resource releases.
- `manage_font_resource`: Library code that performs font resource allocation and lookup by traversing a bit-mapped tree and managing conditional resource releases.
- `process_font_request`: Library code that manages font resource allocation and lookup by searching through font entries and handling conditional resource releases.
- `release_font_request_and_return`: Runs the lookup routine, releases the temporary font resource when one was allocated, closes the request through a library call, and returns the lookup routine's result.
- `map_input_byte`: Modifies a byte by applying a bitmask based on specific range checks.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d7 to d0, then copies (a7)+ to d7, then returns.
- `compare_byte_sequences_2`: Compares two null-terminated sequences of bytes for equality and returns a boolean result.
- `compare_string_sequences`: This library routine compares two null-terminated sequences of bytes to determine if they are identical.
- `compare_byte_sequences`: This library routine compares two null-terminated sequences of bytes for equality and returns a boolean result.
- `return_one_value`: Returns one immediately without any other work.
- `copy_null_terminated_string`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compare_bytes_lexicographically`: This library routine compares two byte strings and returns a value indicating if the first is less than, equal to, or greater than the second.
- `check_status_flags`: Checks the status of the zero and negative flags to return either all ones or one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_string_to_buffer`: The routine finds the terminating zero byte of the destination buffer and copies bytes from the source pointer into that location until a zero byte is copied.
- `find_byte_in_string_2`: This library routine searches for the first occurrence of a specific byte within a null-terminated string and returns its pointer.
- `find_byte_in_string`: This library routine searches for the first occurrence of a specific byte within a null-terminated string.
- `forward_call_with_two_args`: Forwards its two arguments to another routine and returns nothing.
- `find_last_byte_occurrence`: This library routine scans a null-terminated string to find the last occurrence of a specified byte.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `fill_memory_block`: This library routine fills a specified number of consecutive memory locations with a single byte value.
- `fill_bytes_loop_tail`: As part of a library, this routine stores a byte value into consecutive memory locations, one per loop step, for the count given.
- `compute_fixed_point_product`: This library routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.
- `calculate_fixed_point_reciprocal_2`: Performs complex bit-shifting and division to compute a reciprocal value using fixed-point arithmetic.
- `negate_fixed_point`: This library routine computes the negative of a 64-bit value by manipulating its high and low 32-bit components.
- `compute_fixed_point_reciprocal`: Calculates the reciprocal of a fixed-point value using bitwise normalization and division.
- `calculate_fixed_point_reciprocal`: This library routine performs complex bit-shifting and division to compute a reciprocal value using fixed-point arithmetic.
- `calculate_reciprocal_fixed_point`: This library routine performs a complex calculation involving bit-shifting and division to compute a reciprocal or a related fixed-point value based on input registers.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.

### Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:Crystal

- `depack_packed_stream_backward`: Decodes a packed stream by calling a bit-reader refill and a multi-bit read routine, copying literal and back-reference runs backwards into memory.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:iconx

- `parse_command_line_arguments`: Parses a command line string by scanning for space delimiters and special characters to populate a structured list of argument pointers.
- `invoke_indirect_pointers`: Invokes two function pointers stored at specific offsets within a structure pointed to by a register.
- `call_indirect_pointers`: Invokes two function pointers stored at offsets within a structure pointed to by a register.
- `update_structure_field`: Retrieves a pointer from a structure, performs a nested pointer dereference, and writes a constant value to a specific offset within that structure before calling another routine.
- `process_icon_resource_buffer`: Parses a structured data block to extract and concatenate string segments into a local buffer.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d1.
- `reconstruct_and_cleanup_string_buffer`: Constructs a new string by concatenating segments from a global variable and a provided source into a destination buffer.
- `traverse_linked_list`: A routine that recursively traverses a linked list structure.
- `process_string_segments`: Iterates through a series of string segments, performing concatenation, length calculations, and character replacements such as changing forward slashes to colons.
- `concatenate_string_segments`: Calculates the length of a string and performs multiple string copy operations using various combinations of the provided parameters.
- `parse_numeric_string`: Parses a string containing a sign and numeric characters, converting the characters into an integer value.
- `calculate_string_length`: Iterates through a null-terminated string to calculate its length in bytes.
- `copy_string`: Copies a null-terminated string from a source buffer to a destination buffer.
- `copy_string_to_end`: Finds the null terminator of a string and then copies that string to a destination buffer.
- `compare_strings_with_underscore`: Compares two strings while checking for an underscore character, returning a difference value based on the comparison result.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_to_pointer`: Stores a single byte from a register into a memory location pointed to by a register.
- `call_other_routine_with_stack`: The routine calls another routine, passing the two incoming parameters and a pointer to a local stack slot.
- `compute_fixed_point_product_2`: The routine takes two 32-bit fixed-point values from registers and returns their product as a 32-bit fixed-point value.

### reward.am

- `initialize_reward_graphics_data`: Populates multiple global variables and structures by extracting values from a table and a series of pointers.
- `write_coordinate_data_table`: Populates a series of memory locations with constant byte values and high-order bytes extracted from specific memory addresses.
- `fetch_and_store_table_entry`: Fetches a 32-bit value from a table and stores it in a global variable.
- `fetch_and_store_table_entry_2`: Fetches a 32-bit value from a table and stores it in a different global variable.
- `fetch_and_store_table_entry_3`: Fetches a 32-bit value from a table and stores it in another global variable.
- `fetch_table_entry`: Uses a value from a register to calculate an offset and returns a 32-bit value from a global table.
- `store_table_values`: This routine iterates through a series of pointers in a table and stores the values found at those locations into a global structure.
- `decode_and_loop_data`: Calls a routine to decompress a data stream and then enters an infinite loop if the decompression result is invalid.
- `decode_compressed_data`: Calls a routine to decode a compressed data stream and enters an infinite loop if the returned value is invalid.
- `update_structure_fields`: Calculates a value based on several inputs and writes it along with multiple other values into a structure at specific offsets.
- `initialize_data_structure`: Waits for a specific bit in a memory location to be set, then populates a structure with a pointer, a 32-bit value, and several 16-bit constants.
- `infinite_loop`: An unreachable routine that enters an infinite loop.
- `call_empty_routine`: The routine calls itself recursively five times before returning.
- `split_word_into_halfwords`: Stores a 16-bit value into two different offsets within a structure using the high and low parts of a register.
- `update_interrupt_state_and_counter`: Checks a bit in a global variable and, if set, updates hardware registers and increments a counter.
- `update_hardware_state`: Modifies hardware registers and global variables, including an increment to a counter and a check that may trigger a recursive call.
- `swap_two_globals`: Swaps the values of two consecutive global variables.
- `iterate_structure_list`: Iterates through a list of pointers in increments of eight bytes, calling another routine for each non-zero pointer found.
- `decrement_bounded_counter_2`: Decrements a global variable and resets it to zero if it reaches a specific threshold or if a condition is met.
- `decrement_bounded_counter`: This routine decrements a global variable and resets it to zero if it reaches a specific threshold or if a condition is met.
- `decrement_global_counter`: Decrements a global variable by one.
- `increment_bounded_counter`: Increments a global variable and resets it to zero if it reaches a specific value.
- `increment_global_counter`: Increments a global variable by one.
- `copy_data_block`: Copies a series of 16-bit words from a table offset into a destination buffer.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call_2`: Masks a word in one register with a word in another.
- `return_empty_routine_2`: This routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: This routine iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.

### Nigel Mansell World Championship (1992)(Gremlin)[cr CSL](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_graphics_and_audio_state`: Updates the display using the Blitter and manages audio output by clearing DMA and volume registers.
- `process_input_and_reset_state`: Scans a memory range for a specific byte value and, if found, clears a block of memory and resets several global variables.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `update_audio_state`: Updates a global counter and performs conditional initialization of memory locations and audio parameter structures.
- `update_audio_parameter_state`: Modifies audio parameter values by applying arithmetic operations or table lookups based on a nibble-derived index.
- `update_audio_parameter_structure`: Updates a structure by fetching values from a table based on a nibble-derived index and calculating new duration or volume parameters.
- `process_nibble_indexed_value`: Extracts a nibble from a byte to determine an operation type, then modifies a value within a structure using addition, subtraction, or table-based searching.
- `copy_structure_member`: A routine that copies a two-byte value from one memory location to another within a structure.
- `search_table_and_update_value`: Iterates through a table of word values to find the first entry that exceeds a threshold value.
- `update_accumulator_and_search_table`: This routine modifies a value within a structure by adding or subtracting a byte based on a flag, clamping it to a maximum limit.
- `calculate_scaled_offset`: Calculates a value using bitwise operations and scaling factors from a structure, then stores the result into a destination offset.
- `process_input_command_nibbles`: Parses a command byte by splitting it into high and low nibbles to execute different logic branches.
- `mask_and_store_byte`: Extracts the lower four bits of a byte from a structure and stores the result into a different offset within the same structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `bitwise_or_constant`: Performs a bitwise OR operation on a value in a register with a constant value and returns the result.
- `tiny_ori_2`: Small routine of 1 instruction that merges 3 into d0.
- `analyze_memory_segment`: The routine consists of 35 instructions and appears to do nothing observable from its disassembly alone; it contains no library calls and accesses no registers in a way that reveals a purpose.
- `integer_sqrt_approximation`: The routine computes an integer square root approximation for an unsigned input value using a bit-length estimate followed by a binary search-like iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_bit_in_memory`: Sets a specific bit within a byte located at an offset from a pointer.
- `propagate_bitmask_fields`: Copies specific bits from a four-byte source pointed to by a pointer into three output bytes at a base address plus an offset.
- `propagate_collision_bits`: Reads a bitfield from a source byte pointer and, using a set of bit indices supplied in registers, sets corresponding bits in target bytes located at offsets from a base pointer.
- `propagate_bitmask_bytes`: Reads a four-byte source block via a pointer and tests individual bits selected by several registers.
- `copy_collision_bitmap`: Copies bits from a source bitfield in one register to target bytes at a destination address by testing individual bits and ORing the results into the destination bytes.
- `propagate_bitfield_mappings`: The routine reads bits from a source bitfield at a byte pointer and, based on bit indices stored in registers, conditionally sets specific bits in a series of destination bytes located at increasing offsets from a base pointer.
- `propagate_bitmask_bytes_2`: Reads bits from three consecutive bytes pointed to by a pointer and, using several input bit positions, sets corresponding bits in three bytes located at offsets six, seven and eight from a base pointer.
- `copy_bitfield_to_bytes`: Tests individual bits within four consecutive bytes pointed to by a source pointer and, if set, sets corresponding bits in three destination bytes at specific offsets from a base pointer.
- `propagate_bitmask_bytes_3`: The routine tests individual bits within three consecutive source bytes using bit indices stored in registers and, if a bit is set, performs an OR operation to set a corresponding mapped bit in three consecutive destination bytes.
- `map_bitfield_to_bytes`: Reads bits from four consecutive bytes pointed to by a pointer and, depending on the values of several registers, sets corresponding bits in three bytes located at offsets five, six and seven from a base pointer.
- `propagate_bitmask_flags`: The routine reads a four-byte bitmask from a pointer and, using several input registers as bit selectors, sets corresponding bits in three bytes located at offsets five, six and seven from a base pointer.
- `copy_bitfield_to_bytes_2`: Copies bits selected by eight bit indices from four consecutive bytes pointed to by a pointer into three consecutive bytes pointed to by an integer offset.
- `map_bitfield_to_bytes_2`: Copies bits from four source bytes pointed to by a pointer, using bit positions derived from registers, into three destination bytes pointed to by an integer offset.
- `map_collision_bits`: The routine reads a source byte stream through a pointer and, based on a set of bit tests driven by several registers, sets individual bits in three consecutive bytes pointed to by another pointer.
- `propagate_bitmask_to_bytes`: Copies bits from four bytes pointed to by a pointer into three bytes at an offset from a pointer.
- `propagate_bitmask_bytes_4`: Copies bits from three source bytes pointed to by a pointer using a set of bit indices supplied in registers, and ORs the corresponding bits into three destination bytes located at offsets 3, 4, 5 and 6 from a base pointer.
- `copy_bits_to_bytes`: The routine copies bits from four consecutive bytes pointed to by a pointer using bit positions derived from registers, and sets corresponding bits in three consecutive bytes pointed to by another pointer.
- `propagate_bitmask_to_bytes_2`: Reads bits from four consecutive bytes pointed to by a source pointer and, using a set of bit positions, conditionally sets bits in four consecutive bytes pointed to by a destination base address.
- `copy_bits_to_output_bytes`: Copies bits from four input bytes pointed to by a pointer into four output bytes pointed to by an integer pointer.
- `copy_bitfield_to_bytes_3`: Copies bits selected by a set of bit indices from four consecutive bytes pointed to by a pointer into three consecutive bytes pointed to by an integer pointer, setting destination bits according to the source bits.
- `propagate_input_bitmask`: The routine copies bits from four consecutive bytes pointed to by a pointer using offsets derived from several registers, setting corresponding bits in four consecutive bytes pointed to by an index.
- `copy_bitplane_mask`: Copies bits from a four-byte source pointed to by a pointer into four consecutive destination bytes.
- `propagate_bitmask_bytes_5`: The routine reads a source byte stream via a pointer and, using a set of bit positions supplied in registers, conditionally sets bits in multiple destination bytes pointed to by an offset.
- `map_bitfield_to_bitfield`: Copies bits from a 32-bit source area into a destination area using a specific permutation of bit indices.
