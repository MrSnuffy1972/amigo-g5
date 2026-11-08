# Game 506 (Heimdall 2 - Into the Hall of Worlds): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00506). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3604480 top-level bytes in 138 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 56 of 56 named (0 author symbols), 56 of 56 described (8 by rule, 47 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 768 of 768 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 134027 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1729 of 1729 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 84957 |
| code | 5332 |
| data | 3238838 |
| padding | 23449 |
| unused | 251904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:heimdallII | 2 | 267 | 8 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:Fairlight_UK | 2 | 1199 | 36 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:installh2 | 2 | 223 | 8 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:bootblock | 1 | 10 | 1 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:bootblock | 1 | 10 | 1 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:bootblock | 1 | 10 | 1 |
| mid1txt.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| ut2txt.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:font1p.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:english.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:german.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:french.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:italian.pak#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| mid2.pbl#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:font1p.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (992 -> 2846 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:stattabs.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (342 -> 560 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:english.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4053 -> 7978 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:intro.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (123217 -> 226062 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:german.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4359 -> 8340 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:flags.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (41096 -> 77248 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:french.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4264 -> 8572 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:endseq.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (181135 -> 226466 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:lokiend.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (24436 -> 63720 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:seqtext.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (656 -> 900 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:hltbar.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (61 -> 1070 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:worlds/loki.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (32479 -> 59752 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:worlds/hall.dat: high-entropy data file (7.72 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:worlds/gamegfx.dat: high-entropy data file (7.88 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:worlds/hot.dat: high-entropy data file (7.92 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:italian.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (4301 -> 9018 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:heim4.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (50184 -> 88348 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:h2m_ext1.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25671 -> 38620 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:ursha4.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (45565 -> 88092 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:h2m_titl.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (51876 -> 69238 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/tal1.dat: high-entropy data file (7.77 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/hakrat.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40565 -> 75152 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid2.pbl: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (50625 -> 88576 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid0txt.dat: high-entropy data file (7.62 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid4.dat: high-entropy data file (7.82 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid2.dat: high-entropy data file (7.74 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/map_mid.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22137 -> 40078 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid0.dat: high-entropy data file (7.75 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/ander.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14181 -> 31272 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/worldeng.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 200 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/tal0.dat: high-entropy data file (7.79 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid3.dat: high-entropy data file (7.73 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/tal0txt.dat: high-entropy data file (7.74 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/map.dat: high-entropy data file (7.85 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid1.dat: high-entropy data file (7.81 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:heim4.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (50184 -> 88348 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:ursha4.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (45565 -> 88092 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/nif2.dat: high-entropy data file (7.72 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/nif1.dat: high-entropy data file (7.72 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/hakrat.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (40565 -> 75152 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ut3.dat: high-entropy data file (7.76 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ut1.dat: high-entropy data file (7.73 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/h2m_deth.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15607 -> 26440 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/giant.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (28768 -> 53324 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ut2txt.dat: high-entropy data file (7.60 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/nif2txt.dat: high-entropy data file (7.64 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ander.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14181 -> 31272 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ashok.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (27296 -> 56892 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/worldeng.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 200 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/nif0.dat: high-entropy data file (7.74 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ut2.dat: high-entropy data file (7.79 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ut0.dat: high-entropy data file (7.78 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/map.dat: high-entropy data file (7.85 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/map_ut.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (22098 -> 40078 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:heim4.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (50184 -> 88348 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:ursha4.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (45565 -> 88092 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/dakta.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (36450 -> 63832 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/sho2.dat: high-entropy data file (7.71 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/her3.dat: high-entropy data file (7.71 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/sho0.dat: high-entropy data file (7.75 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/loki.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (32479 -> 59752 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/her1.dat: high-entropy data file (7.75 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/kaylin.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (25362 -> 48692 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/map_her.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15648 -> 40078 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/ashok.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (27296 -> 56892 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/worldeng.pak: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (178 -> 200 bytes)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/sho1.dat: high-entropy data file (7.77 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/her2.dat: high-entropy data file (7.76 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/shop.dat: high-entropy data file (7.73 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/her0.dat: high-entropy data file (7.80 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 4 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:worlds/map.dat: high-entropy data file (7.85 bits/byte), classified as packed data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid1txt.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 3 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:worlds/ut2txt.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:font1p.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:english.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:german.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:french.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:italian.pak#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 2 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:worlds/mid2.pbl#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:heimdallII: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- heimdall 2 - into the hall of worlds (1994)(core design)(m4)[cr flt](disk 1 of 4)/Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:installh2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 8 never-loaded region(s), 134027 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:heimdallII

- `process_hunk_type_table`: Walks a table of AmigaDOS hunk type codes and rewrites it in place, copying relocation data and calling a library routine when it encounters a data or bss hunk marker.
- `index_table_branch_hw`: Computes an index from a1 by subtracting four and indexing a table, then branches to a hardware library routine when the value in d1 is positive and otherwise returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `rotate_shift_accumulate_2`: Loads a length from the stack, adjusts it, and uses it to index into a buffer while calling a routine that rotates and accumulates a global variable.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_shift_accumulate`: Runs a loop that rotates a global variable right by a shift amount, fetching new data through a0 when the shifted value becomes zero, and accumulates the bits into a result in d0.
- `store_packed_word_and_trap`: Copies a word from the address in a2 to the 16-bit value at offset 197 of the structure in a2, sets bits 4 and 3 of the byte at offset 2 of the structure in a2, triggers a system trap, and combines the result in d0 with d1 using a bitwise OR.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:Fairlight_UK

- `execute_startup_sequence`: Runs a loop 32767 times, then calls six different routines in sequence before returning.
- `setup_graphics_and_interrupts`: Checks a bit in a library-provided value, then performs several library calls to manage memory and pointers.
- `initialize_library_pointers`: Calls another routine, clears a bit in a global variable, and sets two 16-bit fields in a structure to 32767.
- `clear_hardware_registers`: Runs a loop eight times, calling another routine each iteration and clearing two 32-bit words at the address pointed to by the hardware register at offset 144.
- `init_table_load_pointer`: Writes four constant words into a table, clears a word through a0, then calls another routine and loads a pointer from a global variable into d0.
- `store_split_word`: Stores the lower word of d0 at offset 6 of the structure in a1 and the upper word of d0 at offset 2 of the structure in a1.
- `clear_memory_block`: Loads an address from a PC-relative location, then clears a long word at that address for 5159 iterations, advancing the pointer each time.
- `process_buffers_sequentially`: Loads five different buffer addresses in sequence and calls another routine for each of them before returning.
- `process_encoded_bytes`: Searches a buffer for the byte 253, writes 3 at that position, then scans for the byte 254 while incrementing the following byte, and finally writes a computed value based on a subtraction and a right shift before checking for the end marker 255.
- `process_data_table_2`: Calls several routines to process a data table, then increments a global counter and resets it when it reaches fifty while a status byte holds a marker.
- `process_data_table`: Increments a global word by four, then enters a loop that calls another routine eighty times, potentially updating a pointer from a table based on a value in d0.
- `copy_data_from_table`: Reads a byte from the buffer in a0 and, depending on its value, returns a constant, adds a pointer, or copies eight bytes from a table into a destination pointed to by a1, advancing with different strides.
- `push_store_pointer_branch`: Pushes a0 onto the stack, loads a pointer from a PC-relative address into d0, writes the value four to that address, and branches to the following routine.
- `check_and_set_flag`: Checks a word at a PC-relative address and, if it is zero, writes the value 2 to that same address; otherwise, it enters an infinite loop.
- `update_split_values`: Calls a routine that sets a flag, loads two PC-relative addresses into a0 and a1, calls a routine that compares and adjusts two 16-bit words, and returns if the zero flag is set, otherwise it loops back to the first instruction.
- `update_split_values_double`: Calls a routine that sets a flag, loads a pair of PC-relative addresses into a0 and a1, calls a routine that compares and adjusts two 16-bit words, loads a different pair of PC-relative addresses into a0 and a1, calls that routine again, and returns if the zero flag is set, otherwise it loops back to the first instruction.
- `adjust_split_values_pair`: Loads the word at offset 20fa and the word at offset 2130 into a0 and a1, calls a routine that adjusts each nibble of the first word toward the matching nibble of the second word, then loads the word at offset 20f6 into a0 and the word at offset 2130 into a1 and calls the same routine again, returning if the zero flag is set.
- `update_split_values_double_2`: Calls check_and_set_flag twice, each time with a different pair of PC-relative addresses loaded into a0 and a1, then calls increment_split_value with a second pair of PC-relative addresses, and returns if the zero flag is set.
- `increment_split_value`: Compares two 16-bit words and, if they differ, adjusts each nibble of the first word toward the matching nibble of the second word, then stores the combined result back to the location pointed to by a0.
- `handle_interrupt_and_hardware`: Checks a bit in an interrupt status register and, if set, writes values to CIAA registers, waits for a hardware bit to be set, then restores registers and performs an interrupt return.
- `handle_interrupt_request`: Reads the interrupt status register and masks two bits, then branches based on which bits are set.
- `update_status_and_flag`: Checks a byte in a6 and, if it matches a specific value, modifies a PC-relative word and clears the byte in a6, provided certain bits in a4 and a5 are set; otherwise it sets a global word to 65535 and clears the byte.
- `branch_into_routine`: Contains a single branch instruction that jumps forward into the middle of another routine and does not call anything itself.
- `jump_to_routine`: Jumps to another routine and does not return.
- `init_audio_and_clear_table`: Reads a byte from a table at offset 952 and counts how many entries are greater than zero, then builds a pointer into a structure and clears a 30-word region through it.
- `initialize_and_dispatch_split_values`: Initializes four 16-bit fields in a structure by calling a value handler for each, then reads a table index from the structure, computes a pointer into a table, and clears two fields before calling a second value handler for the remaining fields.
- `dispatch_split_value_handler_2_2`: Reads a 16-bit field and a command byte from the structure in a4, then dispatches based on the low nibble of the command byte to add or subtract a value, and otherwise updates fields in the structure in a4 and in the structure in a6.
- `dispatch_split_value_handler`: Reads a 16-bit field and a command byte from the structure in a4, dispatches based on the low nibble of the command byte to add or subtract a masked value from a stored word with clamping, copies a character byte to offset 23, or searches a table of 74 entries for the first value greater than a masked field, then writes the resulting value to a location computed from a6 and d5.
- `dispatch_split_value_handler_2`: Reads a 16-bit field and a command byte from the structure in a4, then dispatches based on the low nibble of the command byte to add or subtract a value from that field, clamping the result to a range and storing the final value at offset 6 of the structure in a6.
- `resolve_structured_table_entry`: Searches a table of 74 entries for the first value greater than a masked 16-bit field in the structure in a4, then adjusts the index by one if a flag bit at offset 18 is set, and stores the matched value at offset 12.
- `clamp_and_search_table`: Checks a word in the structure in a4 and returns a passed word if it is zero, otherwise adds or subtracts a byte from a stored value and clamps it within bounds.
- `adjust_split_value_store`: Reads a byte from offset 27 of the structure in a4, masks it and uses it as an index into a table to load a byte, multiplies that byte by the low nibble of a byte at offset 26, scales the product, and stores the adjusted result into a 16-bit value addressed by a6 and d5.
- `dispatch_split_command_handler`: Reads a byte from a structure in a4 and dispatches based on its low nibble, adjusting a split value, toggling a bit in CIAA_PRA, or updating stored bytes, otherwise it returns.
- `store_masked_nibble`: Reads a byte at offset 3 of the structure in a4, masks its low nibble with 15, and stores the result at offset 18 of the same structure, then returns.
- `update_pointer_and_check_bounds`: Extracts the high nibble of a status byte and, when it is nonzero and a flag at offset 35 is set, adds a value from a table to that byte and clears the flag, then adjusts a pointer stored at offset 36 so it does not pass a computed address, and inverts the byte it points to.
- `loop_compare_byte_self`: Runs a loop five times, each iteration comparing a byte at offset 6 of the structure in a6 against itself and branching away when the comparison is equal, then returns.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:installh2

- `copy_and_align_data_blocks`: Walks a linked list of header structures, verifies each one's magic value, and when valid copies a block of bytes from the source to a destination buffer while aligning both pointers to even addresses, then copies additional data from a second source into the destination.
- `extract_flag_bit_from_table`: Scans a table of word pairs, comparing a masked value from a register with each pair, and when a match is found reads a byte from the entry and subtracts it from a byte in a register.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: Masks a word in a register with a global mask word, and if a byte in one register is less than a byte in another register, it calls a routine and returns its returned word, otherwise it returns the masked word.
- `compute_byte_index`: Computes a byte index from d1 and d7, reads two bytes from the table pointed to by a3, combines them into a 16-bit value in d6, then subtracts d1 from 16 and returns.
- `fill_figure_table`: Reads a count, fills a stack buffer with values from a routine, and then iterates through a loop to write bit masks and derived values into a table based on a key list.
- `process_linked_list_zero`: Walks a linked list of structures while zeroing their fields, then for each entry multiplies a stored pointer by four and adds the result into a table indexed by a counter in a5.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 2 of 4).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 3 of 4).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Heimdall 2 - Into the Hall of Worlds (1994)(Core Design)(M4)[cr FLT](Disk 4 of 4).adf:bootblock

- `boot_entry_4`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
