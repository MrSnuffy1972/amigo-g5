# Game 1074 (ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01074). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4537632 top-level bytes in 166 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 164 of 164 named (0 author symbols), 164 of 164 described (12 by rule, 150 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1715 of 1715 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 100357 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3526 of 3526 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 126618 |
| code | 13486 |
| data | 3754859 |
| padding | 43117 |
| unused | 599552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:Install | 2 | 523 | 24 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:START | 10 | 253 | 7 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:BANANA.EXE | 6 | 160 | 5 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 959 | 60 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 721 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 896 | 34 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:RUNNER.CO (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:EN1.FIC (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:RAZ.FIC (raw code file, load address unknown) | 1 | 0 | 0 |
| Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:RAZDEM.FIC (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:MAIN.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:INTRO1AM.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:INTRO3AM.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:LOGO.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:FINAM.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:INTRO5AM.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:INTRO4AM.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:IO.CO: high-entropy data file (7.61 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:GERDEP.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:CREATE.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:EAFF.CO: high-entropy data file (7.69 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PP3BAN.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPOBS.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:ACCUEIL.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:ACTORS.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPECHAP.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPINT.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:MESSAGE.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:GESORT.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:MARCHAND.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PARAM.CO: high-entropy data file (7.65 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPBIBLI.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:FRISE.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:EATT.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:GPTAV.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPMATTER.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:TEXTIN.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:MUSIC1.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:SCOMB.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPPALAIS.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:MUSIC2.CO: high-entropy data file (7.73 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:SOURIS.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:TAVERNE.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:PPTH.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:OBJET.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:DEADAM.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:BUSTE.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VJARDIN.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VRAMPART.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VPERSO1.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:SVILLE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VHOUSE1.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VWALL.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VPERSO2.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:FVILLE.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:SVILLE2.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VHOUSE2.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VPERSO3.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VBIBLI.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VCASINO.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VARCHE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VHOUSE3.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VPERSO4.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VARCHE2.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:SPACE.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VARCADE.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VPERSO5.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:LEZARD1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:GUARD1.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VTHEATRE.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:GUARD2.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VDEC1.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:ALSTAR.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VOBS.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VPALAIS.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VDEC2.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:MOLTUS.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VOLEUR1.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:MAP.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VMATTER.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 3 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 3 of 5).adf:VBANK.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:ZOLTAR.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:FLY5.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:FORET.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:OURS9.CO: high-entropy data file (7.84 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:FFORET.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:GEANT9.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:JUNGLE.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:SPACE.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:SNAKE3.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:MONTAGNE.CO: high-entropy data file (7.91 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:ZOUAV3.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:FEE5.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:FJUNGLE.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:LEO3.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:CHAOS5.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:SJUNGLE.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:BEE5.CO: high-entropy data file (7.71 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:SFORET.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:GREEM5.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:GENERAT9.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:LOUP9.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:OURS5.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 4 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 4 of 5).adf:SMONTAGN.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:SORCIER7.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:REDFI7.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:FDJLAVE.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJCLOCK.CO: high-entropy data file (7.76 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:WITCH7.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:SPACE.CO: high-entropy data file (7.66 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJTRAP.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:FIREBALL.CO: high-entropy data file (7.85 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJ2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJDEC.CO: high-entropy data file (7.80 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJ3.CO: high-entropy data file (7.88 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:FDJ1.CO: high-entropy data file (7.83 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:PRINCESS.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:PCAVE.CO: high-entropy data file (7.78 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:SPIDER7.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJ4.CO: high-entropy data file (7.90 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:FDJ2.CO: high-entropy data file (7.70 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:BIGSTONE.CO: high-entropy data file (7.79 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:BUSTE2.CO: high-entropy data file (7.89 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:FDJ3.CO: high-entropy data file (7.75 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:CHAOS7.CO: high-entropy data file (7.81 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:FDJ4.CO: high-entropy data file (7.68 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJ1.CO: high-entropy data file (7.87 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:WIZARD7.CO: high-entropy data file (7.72 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJLAVE.CO: high-entropy data file (7.82 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DJDEC2.CO: high-entropy data file (7.74 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:DRAGON7.CO: high-entropy data file (7.86 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:SKELET7.CO: high-entropy data file (7.77 bits/byte), classified as packed data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 5 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 5 of 5).adf:PLASMA7.CO: high-entropy data file (7.61 bits/byte), classified as packed data
- runtime segment an address: 444 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 274 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 592 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:RUNNER.CO: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:EN1.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:RAZ.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 2 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 2 of 5).adf:RAZDEM.FIC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf: blocks 1738-1759 (bytes 889856-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.35)
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:START: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- ishar 3 - the seven gates of infinity (1994)(silmarils)(m3)[cr psg](disk 1 of 5)/Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:BANANA.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 4 never-loaded region(s), 100357 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:Install

- `execute_installer_logic`: Initializes the installation environment by calling several library functions to manage tasks and messages, then executes a sequence of internal routines to process installation paths and strings.
- `get_valid_message`: Loops until a message orgetmsg result passes a check for a specific type and certain non-control characters.
- `parse_and_copy_file_path`: Opens a file, reads its metadata, and copies a string from a buffer into a global variable using a loop.
- `parse_install_path`: Iterates through a string, replacing a character with a slash and updating global pointers or values based on string content.
- `call_print_stub`: Invokes a routine that loads arguments and calls a text printing function.
- `assign_address_to_global`: Stores the address of a global variable into another global variable and then calls itself recursively.
- `call_auto_request`: Calls a routine via a pointer-offset from a global address.
- `copy_string_and_call_functions`: Copies a string from a source pointer to a destination, then calls two different functions using pointers retrieved from a global variable.
- `negate_and_or_d6`: Negates a 16-bit word in d6 and performs three logical OR operations with zero on d0 using values from a table.
- `call_and_store_result`: Loads a library base address into a6, loads a pointer into a1, calls a routine at an offset from a6, stores the returned value in a global variable, and branches to another routine if the return value is zero.
- `release_library_resource`: Calls a routine via a pointer in a structure and, if the return value is non-zero, stores that value in a global variable before returning.
- `call_and_store_window_handle`: Calls a routine via a pointer in a global variable and, if the return value is not zero, stores that value into another global variable before returning.
- `call_print_i_text`: Library stub: loads its arguments and the intuition library base, calls PrintIText and returns its result to the caller.
- `close_resources`: Calls routines to close a library and a window.
- `execute_if_less_than_threshold`: Checks if a value at a global variable offset is less than a specific constant and, if so, calls another routine.
- `search_char_and_call_routine`: Calls a routine and, if the return value is non-zero, searches a string for a specific byte and calls another routine if the byte is found.
- `execute_window_close`: Calls two routines, performs arithmetic on the returned value in d0, calls a third routine, stores the result in a global variable, and calls a routine via a pointer in a global variable before returning.
- `assign_global_pointers`: Sets several global pointers to the address of a global variable based on certain input values.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `assign_address_on_condition`: Sets a global pointer to the address of a global variable if certain input values are present.
- `copy_string_buffer`: Copies a string or buffer and appends a newline and spaces to a destination location.
- `or_zero_into_d0`: Performs a logical OR of zero into d0.
- `duplicate_string_and_execute_callbacks`: Duplicates a string onto itself, then calls two different functions using pointers relative to a global variable if a returned character is non-zero.
- `call_display_alert`: Executes a routine via a pointer-offset from a global address, which calls a display alert function.

### Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:START

- `initialize_game_state`: Executes a sequence of function calls and checks a global pointer to swap a memory value if a specific constant is found.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_compressed_data`: Calls a routine via a pointer in a4 multiple times, then uses the results to add values to a destination buffer at an offset determined by a value in d0.
- `load_address_and_set_register`: Loads an address into a1 and sets d0 to two.
- `expand_compressed_data_block`: Decompresses a data block by reading a series of control bytes that specify bit-shifts and lengths, then writes the expanded bytes to a destination buffer.
- `push_registers_and_branch`: Pushes a byte from a pointer in a2, sets a byte in d0 to 113, and pushes the values of d0, d1, and d2 onto a stack pointed to by a4.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:BANANA.EXE

- `free_memory_clear_slot`: Saves the registers on the stack, calls two unnamed routines, then calls another unnamed routine and frees a memory block through exec.FreeMem.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `expand_compressed_data_stream`: Reads a header from a data stream to determine a sequence of expansion rules, then iterates through the stream to write bytes to a destination buffer.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `mask_word_with_table_entry`: Looks up a word in a fixed table using an index passed in a register, and returns it ANDed with the input word.

### Ishar 3 - The Seven Gates of Infinity (1994)(Silmarils)(M3)[cr PSG](Disk 1 of 5).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_function_pointer_table`: Calculates an offset based on a value from a global variable, stores this offset back into a table, and then writes a word and a pointer to a new location determined by that offset.
- `compare_and_swap_byte`: Compares a byte at a negative offset from a pointer in a3 with seven, and if it matches, calls two routines with different addresses in a2 before checking if a byte at a0 is twenty-four and potentially writing ten to it, finally returning a value derived from a byte at a3.
- `compare_memory_blocks`: Compares two memory blocks using a loop and returns a value based on the comparison results.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `modify_stack_and_push_address`: Sets bits in d1, pushes an address onto the stack, and restores registers.
- `initialize_system_pointers_2`: Stores the current stack pointer into a global variable, writes several longwords to specific global memory locations, and sets two bytes at a global address before jumping to another routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_graphics_and_memory_resources`: Calls a library function to find a task, validates a structure at offset 172, and proceeds to call several other routines to allocate memory, verify library availability, and calculate offsets for graphics-related structures based on memory capacity and specific hardware-related constants.
- `invoke_two_function_pointers`: Calls two library function pointers at fixed negative offsets from a base register, then returns.
- `load_library_pointer`: Loads a pointer from a global variable into a1, calls a function via a pointer in a6, and if the return value is not zero, stores it into two different global variables before returning.
- `verify_library_and_cache`: Checks bit 3 of a word at offset 296 of a structure in a6 and, if set, calls a function via a pointer in a6 and stores the result in a global variable before returning.
- `verify_and_allocate_library_resources`: Validates the availability of several libraries and checks for sufficient chip memory before calculating and storing multiple pointer offsets and memory boundaries in global variables.
- `open_keyboard_device`: Calls two routines using a pointer from a global variable, sets up a structure at a global address with a pointer, a constant ten, a pointer to another global, and a constant thirteen, then calls a third routine and returns if the result is zero.
- `read_joypad_data`: Moves a longword from the joypad data register to a global variable.
- `setup_relocation_tables`: Initializes several global pointers, allocates memory via a library, and sets up relocation or header-related pointer offsets.
- `copy_and_set_pointers`: Copies a block of memory from a source to a destination, updates several global pointers with calculated offsets, calls another routine, and then modifies hardware registers and a global variable before calling another routine.
- `search_table_for_match`: Compares a word at a global address with d1 and loops through a table, incrementing the pointer and a counter until a match is found or a limit is reached.
- `return_max_word`: Sets d0 and d1 to all ones and returns.
- `set_d1_constant`: Stores the constant value negative 64 in register d1 and returns.
- `test_and_conditionally_return`: Tests a 16-bit value in a register and returns immediately if the value is negative or zero; otherwise, it returns.
- `call_indirect_function`: Calls a function pointer stored in a memory location and returns the result.
- `wait_for_bit_four`: Checks bit 4 of a hardware register and returns if it is not set, otherwise it loops.
- `wait_for_flag`: Loops until the lowest bit of a certain hardware register is set.
- `set_interrupt_enable_bit`: Writes a value to a hardware register to enable certain bits, such as disabling or enabling interrupts.
- `update_pointer_table`: Iterates through a list of pointers and updates the target or destination using a linear transformation or offset.
- `update_global_state`: Modifies a global variable and writes a sequence of two bytes to a memory location.
- `reset_and_recurse_pointer`: Sets three bytes at specific global addresses to zero, calls a routine via a pointer in a6, updates a global pointer with the result, and then calls a routine at a new address stored in a global variable.
- `set_system_state`: Calls a function through a pointer, updates a global variable, and calls another function with an argument.
- `increment_and_recurse`: Increments a global variable, calls another routine, and then enters a loop that calls itself three times, followed by two additional routine calls.
- `write_intena_constant`: Writes the constant value 32 to the interrupt enable register and returns.
- `check_global_byte`: Tests a byte at a global address and returns immediately if it is zero, otherwise it returns.
- `update_channel_countdown_flags`: Indexes a table by a shifted register value, decrements a byte at offset 1 when two conditions hold, then writes derived bit-mask values to interrupt and DMA control registers depending on further flag checks.
- `load_address_to_a0`: Loads the address stored in a global variable into a0 and returns immediately.
- `store_byte_and_call`: Stores a byte from d0 into a global variable and calls another routine using a function pointer retrieved from a table.
- `store_register_and_call_function`: Stores the contents of a data register into a global variable and then calls a function pointer retrieved from a table or offset.
- `update_global_with_conditional`: Checks if a register contains a specific value and, if so, assigns a different value to a global variable before calling another routine.
- `iterate_until_value`: Calls another routine in a loop until the return value is non-zero, then calls that same routine in a second loop until the return value is zero.
- `update_global_buffer`: Updates a global variable or a buffer-like structure with new values based on certain character checks and then calls a function pointer.
- `traverse_string_null`: Traverses a buffer by reading bytes and calling another routine for every non-null byte encountered.
- `calculate_scaled_offsets`: Clamps register values and stores the results multiplied by a constant into two global variables.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_position_limits`: Updates two global variables by adding or subtracting a value, then clamps them within a certain range, and finally clears certain bits in a third global variable.
- `scale_input_values`: Multiplies two input registers by two and stores the results in two global variables.
- `compare_and_reset_table`: Iterates through a table to compare values, and if a certain condition is met, updates a global variable or resets a portion of a table to zero.
- `set_status_flags`: Calls a function pointer and then modifies a global variable or sets specific bits within it based on the status of certain flags.
- `read_joystick_status`: Loads a byte from the CIAA Port A Data register into d2, tests bit 0 of d0, and branches.
- `test_joystick_bits`: Reads a word from the joystick data register and sets bits 0 or 1 of d0 based on whether bit 1 or bit 0 of the input is set.
- `test_and_set_bits`: Tests bits zero, nine, and eight of a register and sets bits zero or two of another register based on the results.
- `set_register_bit`: Tests bit eight of d1, and if set, tests d2 and potentially sets bit seven of d0 before returning.
- `execute_permitted_function`: Checks if a global variable is greater than one, and if so, calls a function pointer located at an offset from that variable.
- `execute_indirect_function`: Checks if a global variable exceeds a certain threshold, then executes a function pointer located at an offset relative to that variable.
- `copy_string_to_global`: Copies a null-terminated string from a source pointer in a0 to a global buffer if a value at a global address is less than 2.
- `call_dos_open_and_check`: Calls a library function that opens a file and checks the return value, while also performing a check on a function pointer offset.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `call_external_function`: Calls a function pointer located at an offset from a value stored in a global variable.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `iterate_message_list`: Calls a function pointer from a table, then enters a loop that repeatedly calls the same function pointer from a structure until a specific bit pattern is found in a value at offset 20 of that structure.
- `restore_registers`: Restores d0 through d7 and a0 through a6 from the stack and returns immediately.
- `update_global_on_condition`: Calls a routine, calls a function pointer, calls another function pointer, and updates a global variable to 2 if a value at an offset of a global variable exceeds 4999, then calls another routine and modifies a byte in a global variable.
- `initialize_system_pointers`: Stores the stack pointer into a global variable, writes several longwords to specific global memory locations, sets two bytes at global addresses to one, and calls three different routines before loading two addresses into registers and clearing a 16-bit register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_global_variable`: Sets a byte at a global address to 255 and then clears two subsequent bytes at a global address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_indirect_jump`: Loads a word from a table using a0 and d0 as an index and jumps to the resulting address.
- `execute_nested_routines`: Calls a routine and passes its return value as an argument to another call, then calls a third routine before returning.
- `initialize_data_structure`: Initializes a series of global variables and populates a table with incrementing or constant values.
- `process_string_and_update_state`: Selects a status byte based on global variable checks, copies a string from a pointer into a destination table, and replaces a character at a specific offset with the selected status byte.
- `update_global_byte`: Sets a global variable to 1, checks its high nibble against a constant, and performs conditional loops that call other routines to modify memory via pointer-based checks.
- `compare_and_fetch_byte`: Compares two pointers and, if the first is not greater than or equal to the second, returns a byte from the memory pointed to by the second pointer; otherwise, it calls another routine and branches back.
- `write_byte_with_bounds_check`: Writes a byte to a destination if a calculated address range check passes.
- `process_byte_stream`: Calculates a destination address using a pointer and a longword, then enters a loop that writes bytes to that destination based on values from a source table and a counter.
- `tiny_moveq`: Small routine of 1 instruction that loads 3 into d0.
- `calculate_byte_difference_and_rotate`: Subtracts a byte in register d7 from a byte in register d0, and if the result is not negative, returns the original value in d0; otherwise, it performs a series of bitwise rotations and swaps on d5 and d7 before returning a calculated value.
- `invoke_and_return_pair`: Calls another routine and returns a concatenated 64-bit value formed from two 32-bit registers.
- `search_sorted_table`: Performs a binary search through a table of word-sized values to find a match for a given value.
- `initialize_list_structure`: Iterates through a memory range, writing incrementing offsets and zero-values to a list or buffer, and updates certain global flags.
- `insert_object_into_sorted_list`: Increments a global counter and adds a value to a list, then performs a sorted insertion of a 16-bit value into a linked list by shifting existing elements.
- `initialize_and_copy_object_structure`: Initializes a structure by zeroing a range of memory and setting various fields including a pointer, a constant, and several 16-bit values, then performs a memory copy loop to move data from a source to a destination based on calculated offsets.
- `dispatch_system_state`: Stores a pointer from a0 into a structure, clears a byte at offset 12B0, and updates a 16-bit value at offset 13E8.
- `execute_conditional_function`: Calls a function pointer and subsequently calls a second function pointer if the first call returns a non-zero value.
- `execute_error_loop`: Loads a longword from a global variable, calls another routine, and if the return value is negative, sets d0 to 14 and calls another routine before looping back.
- `execute_error_loop_alt`: Extends d0 to a longword, calls another routine, stores the result in a global variable, and if the result is negative, sets d0 to 8 and calls another routine before looping back.
- `retry_function_call_on_error`: Calls a routine and stores the result in a global variable; if the result is negative, it calls another routine with the value 8 in a register and repeats the process via a loop.
- `call_and_test_d0`: Calls another routine, tests the result in d0, and returns if the result is negative, otherwise it branches to another location.
- `execute_subroutine_and_branch`: Sets d0 to 9, calls a routine through a pointer, and branches to another location.
- `execute_error_loop_extended`: Loads a longword from a global variable, calls another routine, and if the result is negative, sets d0 to 13 and calls another routine before looping back.
- `reset_memory_blocks`: Calls another routine four times, each time passing a different address from a sequence of four memory locations in a3.
- `reset_memory_block`: Clears or initializes a specific block of memory by writing zero or a specific constant value to multiple offsets from a pointer.
- `process_clamped_values`: Tests three 16-bit values at offsets 1414, 1418, and 1416 of a structure.
- `update_clamped_counter`: Adds a value from one global variable to another, clamps the result to a maximum of 32767 if it exceeds a third global variable, decrements a fourth global variable, and calls another routine twice.
- `decrement_global_and_call`: Decrements a global variable and calls another routine twice.
- `call_and_clear_byte`: Calls a routine and then clears a byte in a global variable.
- `handle_error_and_jump`: Clears a byte in a global variable, calls several routines using pointers from memory, writes three 16-bit values to specific global locations, and finally jumps to a routine via a pointer.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `increment_global_on_condition`: Calls another routine and, if the returned value is non-zero, increments a global variable by a large constant.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_and_table_data`: Initializes a table with a loop that sets certain offsets to a constant, populates a secondary table with incrementing addresses, and performs a series of library calls and pointer updates, including setting a function pointer to a specific routine and clearing certain global or structure-based flags.
- `perform_logical_or`: Performs a bitwise OR operation on register d0 with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `divide_and_mask_bits`: Divides register d1 by d0, performs a bitwise AND on d0 with itself, and then performs a bitwise OR on d0 with 64.
- `check_fire_button_and_reassign_routine`: Checks if a specific bit is set in a pointer-relative offset of a structure, then if a routine pointer is non-null, sets it to null and calls it, followed by incrementing or setting that routine pointer to a new address or a constant, and finally writes a value to a structure offset and returns.
- `tiny_ori_2`: Small routine of 1 instruction that merges 201 into d0.
- `clears_data_register_zero`: Sets the least significant byte of data register zero to zero.
- `conditionally_update_function_pointer`: Checks bit 0 of a byte in a global variable and, if set, calls a function pointer and stores the return value in a global variable.
- `clears_data_register_zero_2`: Performs a logical OR operation on data register zero with zero.
- `parse_string_and_update_pointer`: Iterates through a string to find a null terminator, calls another routine if a string is found, and updates a global pointer to the end of the string or a default address.
- `parse_string_and_update_table`: Iterates through a string, and if a character is neither a null terminator nor a space, calculates a new address and certain bit-masked values, then updates several offsets in a structure pointed to by a6, including a loop that waits for a specific bit in a structure member to clear.
- `initialize_table_values`: Sets several offsets in a structure pointed to by a6 to specific constants or values derived from global variables, using a loop that waits for a bit in a structure member to clear.
- `set_structure_parameters`: Loads a constant address into a register, sets a register to a constant, and updates several offsets in a structure pointed to by a6 with constants, including a conditional check on a bit in a structure member.
- `reset_structure_members`: Waits for a bit or a flag in a structure in a6, then clears a 16-bit value at offset 102, sets a 32-bit value at offset 64, and writes a 32-bit value and a 16-bit value to offsets 84 and 88 before waiting again and writing a different 32-bit value and 16-bit value to offsets 84 and 88.
- `decrement_and_iterate_counter`: Decrements a global counter and, if zero, resets a pointer to a table, copies a value from that table to a global, and increments the pointer.
- `update_global_index`: Swaps a global variable with another global variable, then calculates a new value using a global variable, a table, and an offset, and stores this value in a global, then updates a global with a value from a table using an offset or an index.
- `process_table_data`: Waits for a flag in a structure in a6, then enters a nested loop that calculates a value based on a table, a constant, and a pointer, and for each iteration, waits for a flag, reads a 16-bit value from a table, and writes a 32-bit value and a 16-bit value to a structure, while decrementing counters and adjusting pointers.
- `branch_to_offset_c40`: Performs a relative branch to a specific offset.
- `branch_to_hardware_configuration`: Performs a short branch to a location within the routine that configures hardware and DMA.
- `configure_hardware_and_dma`: Performs complex arithmetic on data structures to update various memory locations, configures DMA and audio registers, and modifies hardware control registers.
- `dispatch_opcode_table`: Reads a byte from an offset of two in a5, masks the high nibble, and shifts it to calculate an index.
- `update_pointer_and_counter`: If a word at offset 3 of a3 is non-zero, it adds a value derived from a byte at offset 2 of a5 to a word at offset 12 of a5, updates offset 8 of a4 with that word if it is less than 64, and branches back to the start.
- `process_data_stream_and_update_pointers_2`: Iterates through a loop of structures, performing arithmetic on bytes at offset 13 and 5, updating 16-bit values at offsets 6 and 8, and calling another routine before updating pointers and decrementing a counter.
- `clamp_and_store_value`: Adds a byte from offset 2 of a structure in a5 to a word at offset 10 of the same structure, compares the result to 856, and if it is greater, stores 856 into offset 10 of the structure and offset 6 of a4 before branching.
- `store_word_and_branch`: Stores a word from offset 10 of a5 into offset 6 of a4 and branches to another routine.
- `process_data_stream_and_update_pointers`: Iterates through a loop of four structures, updating a byte at offset 3 and potentially decrementing a counter at offset 47.
- `update_structure_data`: Reads a byte from offset 2 of a structure in a5 and, if non-zero, writes it to offset 23 of the same structure.
- `transfer_byte_and_branch`: Moves a byte from offset 13 of a structure in a5 to offset 13 of the same structure and branches to another routine.
- `swap_nibbles_in_structure_byte`: Reads a byte from offset 2 of a structure, and if it is non-zero, swaps its high and low nibbles and stores the result into the byte at offset 24 of the same structure.
- `execute_jump_table_entry`: Extracts a byte from a structure at offset 2, masks the high nibble, and uses the result to perform an indirect jump through a table.
- `update_structure_counter`: Adds a value derived from a byte at offset 2 of a structure in a5 to a word at offset 12 of the same structure, then updates the word at offset 12 if it is less than 64.
- `update_index_and_bit_flip`: Updates a byte at an offset in a structure using a value from a global table, and if a signed check passes, updates a pointer in the structure and flips the bits of the byte at that pointer's location.
- `configure_dma_transfer`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers.
- `load_audio_register_values`: Loads several 32-bit or 16-bit values from a series of memory locations into a block of RAM and sets a global variable to a fixed value, while modifying the low 16-bits of the first audio channel and the interrupt clear register.
