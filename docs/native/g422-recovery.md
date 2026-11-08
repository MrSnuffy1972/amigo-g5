# Game 422 (Xtreme Racing V1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00422). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2738872 top-level bytes in 193 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 610 raw-word lines |
| Functions named and described | pass, 255 of 255 named (0 author symbols), 255 of 255 described (21 by rule, 225 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3981 of 3987 (99.85%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8156 of 8453 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 56527 |
| code | 28620 |
| data | 2343243 |
| padding | 50898 |
| unused | 259584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:c | 2 | 117 | 2 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:FrontEnd | 19 | 207 | 7 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Race | 10 | 27 | 2 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Takeover.library | 1 | 371 | 28 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:XTremeRacing.library | 5 | 1413 | 40 |
| execute | 1 | 888 | 12 |
| execute | 1 | 888 | 12 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:bootblock | 1 | 18 | 1 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:bootblock | 1 | 18 | 1 |
| XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1646 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 322 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2094 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 298 | 30 |

## Cases handled

- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Controls.raw: high-entropy data file (7.88 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/SmallIcons.raw: high-entropy data file (7.85 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/NotPlaying.raw: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Backdrop.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/PlayerSelect.raw: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/TrackLoading.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Logo.raw: high-entropy data file (7.98 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/End.raw: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Font.raw: high-entropy data file (7.83 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Vehicles.raw: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Portraits.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Gfx/Icons.raw: high-entropy data file (7.89 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Modules/Mod.Techno: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Modules/Mod.Harlem: high-entropy data file (7.98 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Modules/Mod.Theme: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Modules/Mod.Dance: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Modules/Mod.Jungle: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Modules/Mod.Harsh: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/CASTLEFLATS2.Chk: high-entropy data file (7.95 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/ISLANDSOLIDS1.Chk: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/VOLCANICFLATS1.Chk: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/NUCLEARFLATS3.Chk: high-entropy data file (7.90 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/VOLCANICSOLIDS1.Chk: high-entropy data file (7.89 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/NUCLEARFLATS1.Chk: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/ISLANDFLATS3.Chk: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/ISLANDFLATS1.Chk: high-entropy data file (7.92 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/CASTLESOLIDS1.Chk: high-entropy data file (7.92 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/NUCLEARSOLIDS1.Chk: high-entropy data file (7.85 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/ISLANDSOLIDS2.Chk: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/VOLCANICFLATS2.Chk: high-entropy data file (7.91 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/CASTLEFLATS1.Chk: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/NUCLEARFLATS2.Chk: high-entropy data file (7.91 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tiles/ISLANDFLATS2.Chk: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Forbidden_Castle_2.Trk: high-entropy data file (7.85 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Grasslands_2.Trk: high-entropy data file (7.90 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Toxic_Refinery_2.Trk: high-entropy data file (7.84 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Guts'n'Goo_D.Trk: high-entropy data file (7.81 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Volcanic_Badlands_1B.Trk: high-entropy data file (7.69 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Forbidden_Castle_1.Trk: high-entropy data file (7.88 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Grasslands_1.Trk: high-entropy data file (7.87 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Toxic_Refinery_1.Trk: high-entropy data file (7.86 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Toxic_Arena_D.Trk: high-entropy data file (7.75 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Tracks/Volcanic_Badlands_2B.Trk: high-entropy data file (7.83 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Backgrounds/NUCLEARBG.bck: high-entropy data file (7.87 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Backgrounds/ISLANDBG.bck: high-entropy data file (7.92 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Grasslands_2.raw: high-entropy data file (7.95 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Forbidden_Castle_1.raw: high-entropy data file (7.95 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Grasslands_1.raw: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Toxic_Refinery_1.raw: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Toxic_Arena_D.raw: high-entropy data file (7.91 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Volcanic_Badlands_2B.raw: high-entropy data file (7.91 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Volcanic_Badlands_1B.raw: high-entropy data file (7.84 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Forbidden_Castle_2.raw: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Toxic_Refinery_2.raw: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Layouts/Guts'n'Goo_D.raw: high-entropy data file (7.75 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Worlds/Volcanic_Badlands.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Worlds/Grasslands.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Worlds/Toxic_Refinery.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Worlds/DeathMatch.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 2 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:Worlds/Forbidden_Castle.raw: high-entropy data file (7.98 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/ROADFLATS2.Chk: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SPACESOLIDS1.Chk: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SPACEFLATS2.Chk: high-entropy data file (7.75 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/ROADSOLIDS1.Chk: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SEAFLATS2.Chk: high-entropy data file (7.90 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/ROADFLATS5.Chk: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/ROADFLATS3.Chk: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SEASOLIDS1.Chk: high-entropy data file (7.89 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/ROADFLATS1.Chk: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SPACEFLATS3.Chk: high-entropy data file (7.80 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SPACEFLATS1.Chk: high-entropy data file (7.89 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SEAFLATS3.Chk: high-entropy data file (7.92 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/SEAFLATS1.Chk: high-entropy data file (7.87 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tiles/ROADFLATS4.Chk: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Road_Circuit_2.Trk: high-entropy data file (7.86 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Spaced_Out_D.Trk: high-entropy data file (7.81 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Road_Rage_D.Trk: high-entropy data file (7.68 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Tropical_Islands_2.Trk: high-entropy data file (7.88 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Floating_City_2.Trk: high-entropy data file (7.84 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Road_Circuit_1B.Trk: high-entropy data file (7.87 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Road_Circuit_1.Trk: high-entropy data file (7.89 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Cape_Fear_D.Trk: high-entropy data file (7.64 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Tropical_Islands_1.Trk: high-entropy data file (7.87 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Tracks/Floating_City_1.Trk: high-entropy data file (7.85 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Backgrounds/SPACEBG.bck: high-entropy data file (7.93 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Backgrounds/SEABG.bck: high-entropy data file (7.92 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Backgrounds/ROADBG.bck: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Cape_Fear_D.raw: high-entropy data file (7.87 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Tropical_Islands_1.raw: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Floating_City_1.raw: high-entropy data file (7.92 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Spaced_Out_D.raw: high-entropy data file (7.77 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Road_Rage_D.raw: high-entropy data file (7.61 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Tropical_Islands_2.raw: high-entropy data file (7.94 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Floating_City_2.raw: high-entropy data file (7.82 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Road_Circuit_1B.raw: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Road_Circuit_2.raw: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Layouts/Road_Circuit_1.raw: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Modules/Mod.Rock: high-entropy data file (7.99 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Worlds/Floating_City.raw: high-entropy data file (7.96 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Worlds/Tropical_Islands.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Worlds/Road_Circuit.raw: high-entropy data file (7.98 bits/byte), classified as packed data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 3 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:Worlds/DeathMatch.raw: high-entropy data file (7.97 bits/byte), classified as packed data
- runtime segment an address: 951 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 26 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 306 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1788 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:FrontEnd: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- xtreme racing v1.0 (1995)(silltunna software)(aga)(disk 1 of 3)/XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Race: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:c

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a long value from register d1 to d0 and then performs a short branch.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:FrontEnd

- `execute_initialization_sequence`: Saves registers to the stack, calls three instances of another routine, calls a routine via a pointer in a1, clears a long value at the address pointed to by a2, calls two library functions, and restores registers before returning.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.
- `initialize_and_populate_data_tables`: Initializes a table with zero values, then populates multiple tables using several calls to other routines.
- `shift_bit_stream`: Shifts bits from a register and updates a value using a bitmask or a value from a table.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `initialize_table_values`: Iteratively calculates and stores new values into a table using a pointer.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Race

- `execute_and_free`: Calls three different routines, uses the result of the second call to clear a memory area, and then calls a fourth routine before returning.
- `decrement_to_minus_one_2`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:Takeover.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `allocate_library_resources`: As library code, it calls a memory allocation routine with specific parameters, stores the result in a structure, and if successful, calls another routine to populate a pointer before conditionally calling a third routine.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_library_resource`: As library code, it decrements a 16-bit word at offset 32 of a structure in a6 and, if the result is zero and bit 3 of a byte at offset 34 is set, calls another routine.
- `cleanup_library_resource`: As library code, it saves registers, moves a pointer from a structure into a6, and if a word at offset 32 of that structure is zero, sets bit 3 of a byte at offset 34 of the structure.
- `library_cleanup_routine`: As library code, it calls a routine via a pointer in a structure, clears a global pointer, and then conditionally calls another routine to free memory or adjust a pointer based on values in a structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `dispatch_library_request`: As library code, it saves registers, calls a routine at an offset from a global variable, calls two other subroutines, and restores registers before returning.
- `library_reset_routine_2`: As library code, it calls two subroutines, calls a routine via a pointer from a hardware register, and writes a value from a structure to a hardware register before returning.
- `execute_library_subroutine`: As library code, it saves registers, calls a subroutine at a fixed offset, and restores registers before returning.
- `configure_hardware_registers_2`: As library code, it calls a subroutine and returns.
- `library_dispatch_routine`: As library code, it loads a pointer from a global variable, stores a value from an offset of that pointer into another global, and then calls two different routines using the pointer.
- `library_reset_routine`: As library code, it checks if a global variable is non-zero, calls two routines via a pointer, calls a third routine with a pointer from a global variable, clears the global variable, and writes a value to a hardware register.
- `library_allocate_and_check_flags`: As library code, it calls a memory availability routine four times with different flags to populate a structure, then checks a bitfield in a global variable to write one of several constant values to a structure offset.
- `update_library_state_2`: As library code, it calls three routines via pointers from a global variable and a hardware register, writes a value from a hardware register to a global variable, and calls another routine if that value is non-zero.
- `tiny_suba`: Small routine of 1 instruction that subtracts a0 from a0.
- `update_library_state`: As library code, it calls a library routine, stores a pointer in a structure, writes bitwise-inverted values to hardware registers, and calls two other routines.
- `return_negative_one_2`: As library code, it returns a 32-bit value of negative one.
- `configure_hardware_registers`: This library routine performs bitwise operations on two input words to write values to the hardware registers at offsets 244, 240, and 246, calls two other routines, and then writes values derived from two global pointers into the hardware registers at offsets 244 and 240 after setting bit 15.
- `execute_library_calls`: This library routine calls four different functions via pointers retrieved from a6 and a global variable before returning.
- `swap_memory_blocks_2`: This library routine performs a loop 256 times to swap 32-bit words between a destination buffer at offset 48 of a5 and a source buffer starting at a1.
- `query_library_attributes`: As library code, it calls two separate routines in succession and then returns.
- `update_hardware_and_resource_flags`: Calls a routine to check two global variables and execute a function based on their values, then calls a routine to write 16-bit values from two global variables into the CIAA and CIAB registers by splitting them into bytes.
- `open_resources_2`: This library routine clears two global variables, then calls a function to populate the first global variable, and if successful, calls the same function again to populate the second global variable.
- `query_resource_attributes`: This library routine checks two global variables; if they are non-zero, it calls a function using the value at offset -18 of the global variable and stores the result in another global variable.
- `set_resource_flags`: This library routine checks two global variables; if non-zero, it calls a function using a value from a corresponding global variable with bit 7 set and returns.
- `read_cia_ports`: This library routine reads bytes from CIAA and CIAB registers, modifies bit 4 of the port registers, and stores the resulting byte values and combined 16-bit values into several global variables.
- `write_cia_ports`: This library routine writes 16-bit values from two global variables into CIAA and CIAB registers by splitting them into bytes, and also writes bytes from two other global variables into CIAA and CIAB registers.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:XTremeRacing.library

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `initialize_library_data`: As library code, it populates a structure with several values and pointers, then enters a loop that copies data from a table into a destination buffer using an offset calculated from a register.
- `increment_and_clear_bits`: This library routine increments a 16-bit word at offset 32 of a6 and clears bit 3 of a byte at offset 14 of a6.
- `decrement_resource_counter_2`: This library routine decrements a 16-bit word at offset 32 of a structure in a6 and, if the result is zero and bit 3 of a byte at offset 14 of the structure is set, calls another routine before returning.
- `release_library_resources_2`: As library code, it sets bit 3 of a byte at offset 20 of a structure, and if a word at offset 32 is zero, calls a routine, then calls a memory deallocation routine using a size calculated from two 16-bit values in the structure.
- `initialize_library_resources`: This library routine attempts to open two libraries via a function pointer, and if both succeed, it calls a third function and performs data movement between a structure and a global variable.
- `close_library_resources`: This library routine checks three pointers in a structure; if they are non-zero, it performs data movement or calls a function via a pointer from a6 to close the associated resources.
- `deallocate_library_resource_structure`: Validates a pointer against a structure member and then iterates through multiple lists and individual pointers within that structure, calling a memory freeing routine for each non-zero pointer found and clearing the pointer to zero.
- `free_library_resources_2`: Iterates through several lists and individual pointers within two structures, calling a memory freeing routine for each non-zero pointer found and clearing the pointer to zero.
- `free_library_resources`: Iterates through a list of pointers defined by a count at a specific offset in a structure, calling a function to free each non-zero pointer and clearing the entry.
- `free_library_resource_list_2`: Iterates through several memory pointers stored at specific offsets within two structures to free them, including a loop that processes up to six entries in a table.
- `free_library_resource_list`: Loops up to six times, calling a function through a pointer stored in a global variable to free memory pointed to by an offset in a table, then clears that pointer.
- `release_library_resources`: Checks several pointers in a structure and calls a function through a global pointer to free associated memory or resources, clearing the pointers after each call.
- `get_structure_member`: Returns a 32-bit word from an offset of 48 in a structure pointed to by a6.
- `handle_file_operations`: Performs a sequence of file-related operations using a function table, including opening, reading, seeking, and closing files based on various status checks and error codes.
- `validate_and_load_resource`: Validates a file header against a constant, performs a sequence of file operations including reading and seeking via a function table, and manages memory allocation and deallocation based on the success of these operations.
- `set_dos_status`: Calls a function through a pointer in a structure to perform an operation, then sets a status value in a structure to either seven or six based on the result.
- `set_status_value`: Stores the value 6 into a 32-bit word at offset 24 of a structure in a4 and returns it.
- `store_structure_pointers`: Stores three pointers from a1, a2, and a0 into offsets 46, 50, and 54 of a structure pointed to by a6.
- `get_structure_member_offset_4e`: Returns a 32-bit word from offset 46 of a structure pointed to by a4.
- `jump_to_another_routine`: As library code, it performs a jump to another routine.
- `validate_data_header_and_jump`: Compares a longword at the address pointed to by a0 against a constant; if they match, it calls validate_and_process_data, otherwise it returns immediately.
- `tiny_ori`: Small routine of 1 instruction that merges 95 into d0.
- `validate_and_process_data`: As library code, it loads a longword from a pointer in a0, compares it to a constant, and if they match, it loads a word and two longwords from the incremented pointer into d1 and d2 before calling another routine with a0 as an argument.
- `process_data_tables`: Performs multiple table operations including clearing a memory area, calling several subroutines to fill or manipulate buffers, and copying bytes from a source to a destination based on a loop counter.
- `shift_and_index_table`: Shifts a 16-bit accumulator right by one bit each iteration while building a shifted value in a counter, decrementing a loop counter and reloading bytes from a source table when it reaches zero.
- `lookup_table_masked_value_2`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `store_and_accumulate_table_values`: Loops a fixed number of times storing successive values from a table into a destination buffer while accumulating them into a running sum.
- `fill_array_with_accumulated_values`: Initializes local counters and fills a destination buffer from a source pointer with accumulated sums for a fixed number of iterations.
- `store_null_data_table`: Acts as a data table containing a sequence of null bytes used by another routine via a relative address.
- `load_resource_data`: Calls a function via a pointer to retrieve data, then calls another routine via a library function to allocate memory, and finally populates a structure with the retrieved values and pointers.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `execute_callback_pair`: Extracts a pointer from an offset within a structure and calls two different routines via pointers found at specific offsets.
- `copy_string_data`: Loops through a source buffer, reading bytes and copying them to a destination buffer until a specific byte is encountered or a limit is reached.
- `store_word_and_long`: Stores a word of zero at the address in a2 incremented by two and stores the value from d1 into the address in a0.
- `update_accumulated_value`: Checks if a value in d3 is negative, and if so, adds a value from a structure at offset 8 to d2; otherwise it returns immediately.
- `update_minimum_value`: Adds the value at the address in a0 to d2, compares the result with the value at offset 8 of the structure in a0, and if the result is less than or equal to that value, stores the value from offset 8 of the structure into the address in a0.
- `copy_buffer_subset`: Copies a sequence of bytes from a source buffer to a destination buffer if the starting index and length satisfy specific conditions.
- `store_long_value`: Stores the value from d1 into the address in a0.

### execute

- `initialize_command_execution_environment`: Allocates and initializes a data structure for command processing by populating it with function pointers and clearing internal buffers.
- `process_command_state_machine`: Evaluates a command structure by executing various function pointers based on a status code.
- `process_input_buffer`: Iterates through a buffer of input characters, comparing them against specific boundary values and updating a data structure.
- `validate_and_fetch_single_char`: Checks if a string length is exactly one by calling a routine to calculate length and then compares the result against ten or negative one.
- `check_single_char_string_length`: Calculates the length of a null-terminated string and, if the length is exactly one, returns a byte from a table at offset 120 of a structure passed via a pointer in a3.
- `execute_callback_pair_2`: Extracts a pointer from offset 4 of a structure, calls a routine at an offset within that pointer, and then calls a second routine using the original structure and a pointer located at offset 16.
- `execute_callback_pair_4`: Extracts a pointer from a structure and calls several different routines through pointers found at various offsets within that structure.
- `execute_offset_callbacks`: Calls two different routines using pointers located at specific offsets within a provided structure.
- `store_registers_to_pointer`: The routine copies the contents of D1, D2, D3, D4, D5, D6, D7, A1, A2, A3, A4, A5 and A6 and the value from the stack into successive entries of the pointer passed in A0, and then returns.
- `normalize_param_and_copy_word`: Ensures the second argument is non-zero and then copies the first word of the structure pointed to by the first argument into a word located at an offset within that structure.
- `normalize_fixed_point_divide_2`: Handles signed 32-bit division by converting operands to positive values, calling a normalization routine, and then applying the appropriate sign to the result.
- `normalize_fixed_point_divide`: The routine divides a 32-bit dividend in d0 by a 32-bit divisor in d1 using a normalized fixed-point method.

### execute

- `initialize_file_buffer_and_callbacks`: Allocates a buffer, populates it with specific byte values, and executes a series of callback pairs while performing file operations and string comparisons.
- `process_input_event_structure`: Checks a status value within a structure against several constants and, if none match, performs a series of library calls to retrieve and validate data.
- `process_input_buffer_2`: Iterates through a buffer of characters, comparing them against boundary bytes and control characters while executing callback functions through a pointer at offset 4 of a structure.
- `validate_string_length`: Calls a routine to check a string length from a table at offset 120 of a structure in a6, and if the length is zero, performs several library calls and updates a value at offset 24 of the structure in a7 before returning a value based on a comparison with 10.
- `find_string_length`: Calculates the length of a null-terminated string and returns a value from a table if the length is exactly one.
- `execute_callback_pair_3`: Extracts a pointer from offset 4 of a structure, calls a routine at an offset within that pointer, and then calls a second routine using the original structure and the first routine's return value as arguments.
- `execute_callback_pair_extended`: Extracts a pointer from offset 4 of a structure and executes a series of conditional callbacks via function pointers located at various offsets within that pointer.
- `execute_offset_callbacks_2`: Calls two different routines using pointers located at specific offsets within a provided structure and returns.
- `save_cpu_context_to_pointer`: Copies the contents of data registers d1 through d7, address registers a1 through a6, and the value at the top of the stack into successive memory locations pointed to by the address passed in a0, then returns.
- `increment_and_copy_structure_word`: Ensures the second argument is at least one, then copies the word at offset zero of the structure pointed to by the first argument into offset zero of that same structure, and returns a 32-bit value combining the adjusted second argument and the word from the structure.
- `prepare_signed_division_operands`: Determines the sign of the operands in d0 and d1, negates them to ensure they are positive, and calls a normalization and division routine, subsequently restoring the correct sign to the result based on the original input signs.
- `divide_32bit_fixed_point`: Performs a 32-bit division of a dividend in d0 by a divisor in d1 using a normalization process to align the divisor.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 2 of 3).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### XTreme Racing v1.0 (1995)(Silltunna Software)(AGA)(Disk 3 of 3).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_execute_callback_chain`: Sets a global variable to a specific value, performs a bitwise OR operation on it, and then executes a sequence of calls to another routine, including a loop that repeatedly calls a routine from a global variable and updates that global variable with a new pointer from a table if a condition is met.
- `initialize_system_resources`: Calls several library routines to allocate memory, find tasks, and set vectors, while updating various global pointers and hardware registers including INTENA.
- `cleanup_system_resources`: Performs a sequence of calls to free memory, update hardware registers, and clear various global variables and pointers.
- `call_library_and_set_byte`: Calls a library function with two word-sized arguments and stores a byte from the return value into a global variable.
- `update_hardware_registers`: Calls a routine using a pointer from a global variable, modifies a bit in a value read from a hardware register, and writes that value back to the same register, then calls two other routines.
- `configure_hardware_registers_3`: Writes values to three hardware registers, calls a routine, writes a byte to a global variable, writes two words to offsets 96 and 9a of a structure in a6, and calls another routine via a pointer at offset 4c of a structure in a0.
- `modify_and_fill_data`: Calls two routines, sets a word at a global variable to zero, modifies a word at offset 52 of a structure in a6 by ORing it with a constant, and calls a third routine using a pointer from a table.
- `sync_structure_member_values`: Reads 32-bit values from a structure pointed to by a global pointer, splits them into two 16-bit words, and writes those words into various global variables.
- `call_external_routine`: Calls a routine at a specific address and then returns.
- `wait_and_set_pointer`: Loops while bits 8 through 15 of a longword at an offset of 4 from a6 are non-zero, then stores the value a fixed address into both a global variable and a location at offset 128 of the structure in a6.
- `handle_interrupt_request`: Checks a bit in a hardware register, and if not set, writes a value to a hardware register, calls two routines, calls a function from a global variable, and writes a value to a hardware register before returning via an interrupt return.
- `allocate_dual_pointers`: Calls two different routines and stores their return values into two separate global longwords, then returns.
- `free_four_memory_blocks_2`: Checks four pairs of global variables; if a pointer is non-zero, it calls a routine using a pointer from a table to free the memory and then clears that global variable.
- `release_memory_resources_2`: If a global variable is non-zero, it calls a routine at offset -138 from a library base, calls another routine at offset -122 from a different library, and sets a pointer at offset 162 of a structure in a0.
- `free_memory_via_pointer`: If a global variable is non-zero, it calls a routine using a pointer found at an offset of -130 from that global variable, then clears the global variable and two other global variables before returning.
- `manage_hardware_and_memory`: Checks a bit in a hardware register, updates a global word if it differs from a value at a specific location, calls a library function to free memory, calculates an offset to update a byte in a hardware register, and calls three other routines before storing a pointer into a global structure.
- `reset_and_call_library`: Clears a longword at an offset of 20 from a pointer, calls a routine, and if the return value is non-zero, calls a library function using a pointer from a global variable before clearing the global variables.
- `free_memory_via_pointer_2`: Checks a global variable for a non-zero value, calls a function at an offset of 210 from that address, and then clears the global variable and another associated global variable before returning.
- `manage_memory_allocation`: Calls a routine using a pointer at an offset of -48 from a global variable; if the result is non-negative, it calls another routine at offset -99, and if that result is non-zero, it calls a third routine at offset -54; if the third routine returns zero, it calls a fourth routine at offset -210.
- `allocate_and_reallocate_memory`: Attempts to allocate a memory block of a size specified in a register, then if successful, attempts to reallocate that same block to a new size; if the reallocation fails, it frees the original block and returns zero, otherwise it returns the pointer to the memory.
- `populate_constant_subset`: Calls a library routine, calls another routine, and then performs four consecutive library calls to store values from various source pointers into a destination structure.
- `check_and_release_memory`: Checks if a global variable is zero, and if so, calls three different routines and then calls a fourth routine using a pointer from a global variable.
- `store_function_results`: Calls a routine and stores the 16-bit result in a structure at offset 92, then calls the same routine again and stores the result in a structure at offset 94.
- `initialize_hardware_and_memory`: If a global variable is non-zero, it calls two routines via offsets -138 and -198 from a library base, sets a byte at a hardware register, and if certain conditions are met, performs several calls to functions and loops until a bitmask in a hardware register is cleared.
- `release_memory_resources`: If a global variable is non-zero, it calls a routine at offset -138 from a library base, calls a routine at offset -122 from a different library base if a value in a global variable is non-negative, and calls a routine at offset -90 if another global variable is non-zero.
- `execute_callback_chain`: Repeatedly calls a routine from a global variable and, if a condition is met, updates that global variable with a new pointer from a table and calls a second routine from that table, looping until the global variable becomes zero.
- `store_function_results_2`: Populates a structure at a fixed address with various constants and pointers, calls another routine, and then stores the results of that call into specific offsets within the same structure.
- `store_function_results_subset`: Populates a structure at a fixed address with a subset of constants and pointers, calls another routine, and then stores the results of that call into specific offsets within the same structure.
- `copy_string_limited`: Copies bytes from the source pointed to by a0 to the destination pointed to by a1 using a counter in d0, stopping if a null byte is encountered or the counter reaches zero, then writes a null byte to the destination if the loop finishes without finding a null byte.
- `compare_memory_blocks`: Saves A0 and A1 to the stack, compares bytes at the locations pointed to by A0 and A1 while incrementing both pointers, and branches based on the comparison result.
- `restore_registers_and_return`: Clears register zero and restores registers zero and one from the stack before returning.
- `allocate_and_initialize_structure`: Calls a memory allocation routine, then if successful, calls another routine to check a condition, calls a free routine, and if a specific condition is met, allocates a new block of 65536 bytes and initializes its first three longwords with values from d2 and d3.
- `call_free_twice`: If d1 is not zero, calls a routine twice using a pointer from d1 and its second longword as the first argument, and the third longword as the second argument, then returns zero.
- `filter_bytes_to_buffer`: Iterates through a source buffer starting at the address in d1, comparing bytes against a value in a table, and copies non-matching bytes to a destination buffer in a2 until a newline character is found or the end of the source is reached.
- `build_bitmask_from_table_2`: Decrements a global variable and, if it is zero, iterates through a table of three entries to build a bitmask in a global variable by ORing values fetched from a secondary table.
- `build_bitmask_from_table`: Iterates through a table of three entries, where each entry contains a byte and a sub-table of up to four bytes, to build a bitmask in a global variable by checking if bytes are non-zero.
- `calculate_bitfield_value`: Reads a word from an offset of 10 from a structure in a6 and performs bitwise tests to return a value of 0, 1, 2, 4, 8, or 10 based on specific bit patterns.
- `calculate_bitfield_value_offset`: Reads a word from an offset of 12 from a structure in a6 and performs bitwise tests to return a value of 0, 1, 2, 4, 8, or 10 based on specific bit patterns.
- `check_keyboard_interrupt`: Checks if a specific bit is set in a hardware register and if two specific bits are set in a byte from a global variable, then calls another routine and writes a value to a hardware register before returning via an interrupt return.
- `update_keyboard_buffer`: Reads a byte from the keyboard serial register, performs bitwise operations, and then either stores a byte into a buffer at an index or sets a byte to 255 based on the input value.
- `update_byte_at_index`: If the byte in d1 is less than 104 and a specific condition is met, it stores the byte from d1 into a buffer at the index provided by d3 and updates a global variable.
- `search_table_for_byte`: Searches a table starting at a fixed address for a byte matching d1, and if a byte matching negative one is found, stores that index in d3 before returning.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to d2, then returns.
- `fill_table_with_bytes`: Clears two global longwords to zero, then fills eight bytes starting at a fixed address with the value 255, and finally writes 255 to a specific offset in that table.
- `find_byte_index`: Searches a table starting at a fixed address for a byte matching the lower byte of d0, returning 1 if found or 0 if the search reaches eight entries without a match.
- `modify_ciaa_control`: Clears bit 6 and sets bit 5 of a global variable located at a fixed address.
- `initialize_data_tables`: Populates two separate tables with memory addresses and function pointers using a loop controlled by a value from a global variable.
- `clear_cache_and_set_pointer`: Calls several routines including a library function to clear a cache, performs a bitwise operation on a value from a library call, and if a specific bitmask matches, stores a longword into a global variable.
- `process_data_list`: Loads a structure from a pointer at a fixed address, calls two routines if a value at offset 12 is non-zero, updates a global variable with a value from offset 20 of the structure, and enters a loop that calls a routine from a list, increments a global variable, and decrements a counter from offset 24 of the structure.
- `process_data_loops`: Calls two routines, then enters a loop that calls a function from a pointer in a list, increments a global variable, and decrements a counter, and finally calls a routine if a global variable matches a specific value.
- `process_data_structure`: Stores a pointer from a register into a global variable, updates a global variable with a value from an offset of the pointer, and if a specific byte in a table is one, decrements a value in the structure and writes two to a table entry before calling two other routines.
- `check_word_equality`: Compares the word at a specific memory location to zero and returns if they are equal, otherwise it returns.
- `update_memory_values`: Writes 1 to an offset of 2 from a pointer, copies a word from a table to an offset of 62 from another pointer, writes 7 to an offset of 14 from the first pointer, and writes 3 to a global variable.
- `check_global_word`: Compares a longword at a global location to zero and returns if it is equal, otherwise it returns.
- `load_pointer_to_a0`: Loads the address of a global variable into a0 and calls another routine.
- `test_and_branch_on_zero`: Compares a word in a global variable to zero and branches to another routine if the value is zero; otherwise it returns.
- `initialize_data_structure`: Sets a word at offset 2 of a structure to zero, copies a word from one structure to another, sets a word at offset 14 of the first structure to 7, and calls another routine.
- `check_global_word_zero_2`: Compares a word in a global variable to zero and returns if it is zero; otherwise it returns.
- `fetch_and_call_subroutine`: Loads the address of a global variable into a register, calls another routine, and returns.
- `check_global_word_zero`: Compares a word in a global variable to zero and branches to another routine if they are equal; otherwise it returns.
- `reset_memory_values`: Writes 0 to an offset of 2 from a pointer, writes 3 to a global variable, and writes 255 to a global variable.
- `process_multiple_data_sets`: Performs two loops to load word pairs from source tables and call subroutines, then optionally performs a third set of operations involving multiplication and a scaled offset calculation.
- `load_and_scale_values`: Calculates a pointer using d0 and a global address, loads two words from that pointer into d2 and d3, and calls two other routines.
- `set_structure_fields`: Loops while bit 6 of an offset of 2 in a structure in a6 is set, then writes several values to offsets 44, 64, 62, 60, 66, and 5c of that structure.
- `calculate_and_store_data`: Performs arithmetic on d2 and d3, loops while bit 6 of an offset of 2 in a structure in a6 is set, then stores several values including a calculated d3 into the structure in a6 at offsets 66, 64, 80, 76, 72, 84, and 5e.
- `set_structure_fields_variant`: Loops while bit 6 of an offset of 2 in a structure in a6 is set, then writes several values to offsets 44, 64, 62, 60, 66, and 5c of that structure, using a different value for the final offset.
- `calculate_and_store_data_variant`: Performs arithmetic on d2 and d3, loops while bit 6 of an offset of 2 in a structure in a6 is set, then stores several values including a calculated d3 into the structure in a6 at offsets 66, 64, 80, 76, 72, 84, and 5e, using the same logic as R5.
- `clear_three_bytes`: Writes 0 to three different global variables at offsets 31524, 31539, and 31561.
- `allocate_and_initialize_buffers`: Performs a series of conditional checks on a global variable to call a routine that allocates memory, then updates several global pointers with the results of subsequent allocation calls.
- `free_four_memory_blocks`: Checks four pairs of global variables; if a pointer is non-zero, it calls a routine using a pointer from a table to free the memory and then clears the global pointer.
- `decrement_global_word`: If a word in a global variable is non-zero, it decrements that word by 1 and returns.
- `initialize_system_state`: Calls three routines, performs a library call using pointers from global variables, and then stores a longword and several words into a series of global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_flag`: If a byte from a register is two, it clears bit 5 of a global variable, performs a loop that decrements a counter seven times, clears bit 5 of the global variable again, and sets the global variable to all ones; otherwise it returns immediately.
- `update_status_word`: If a byte from a register is not two, it returns that byte; otherwise it sets bit 7 and clears bit 6 of a global variable, sets the global variable to a constant, and writes a value to an offset of 156 in a structure pointed to by a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_and_write`: Increments a long value in a global variable and writes the value 2048 to an offset of 156 in a structure pointed to by a6.
- `initialize_hardware_state`: Writes values to a structure in a6, updates several global variables, calls another routine, and modifies bits in a hardware register before returning.
- `restore_hardware_state`: If a global variable is non-zero, it writes values to a structure in a6, updates several global variables, calls another routine, and clears several global variables.
- `load_table_value`: Loads a 16-bit word from a table using an index and an offset, then stores it at an offset of 50 in a structure pointed to by a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_dual_buffers`: Clears two 16-bit words at offsets in two different buffers, then updates those buffers with a value from d2, setting specific bits based on whether the value exceeds 255 or 511, and performs a bitwise shift and bit setting on the results.
- `fill_pointer_tables`: Fills a table with eight long values starting from a base address plus an offset, then fills another table with five long values starting from a value stored in a global variable.
- `update_dual_buffer_pointers`: Writes a series of 16-bit words and their swapped counterparts to various global memory locations and then calls a routine to update two buffers with calculated values.
- `calculate_structure_metrics`: If a pointer in a0 matches a specific value, it initializes a structure and performs several integer divisions and multiplications to store calculated results at various offsets within that structure.
- `calculate_rotated_division`: Reads a 16-bit value from a hardware register, rotates it right by eight bits, adds it to a value in a global variable, stores the result back in the global variable, and returns the quotient of that value divided by d0.
- `zero_data_block`: Contains a sequence of seven instructions that perform a logical OR on register d0 with zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_attribute_lists`: Iterates through two different lists using counters from a4, calling various routines to process strings and update data structures based on the results.
- `process_string_list`: Clears a global longword, enters a loop that calls a routine to compare strings, and uses the results to update various word-sized values at offsets of a structure pointed to by a4.
- `update_difficulty_table`: Performs a series of string comparisons and memory operations to update a structure at offset 7596 of a4, using a loop that increments a counter at offset 24232.
- `update_season_results`: Iterates through a series of string and memory operations to update a structure at offset 336 of a4, using a loop that increments a counter at offset 24232.
- `update_data_structures`: Enters a loop that performs multiple string comparisons and memory operations, using calculated offsets to write words into a structure pointed to by a4, and increments a counter in a local stack variable.
- `populate_difficulty_results_table`: Iterates through difficulty settings by comparing strings and calls a routine to retrieve data, then stores eight bytes of retrieved data into a table at an offset calculated from a parameter and a global counter.
- `update_race_cup_results`: Iterates through a list of race results, calling another routine to retrieve data and then storing eight bytes of character data into a table at an offset calculated from a base pointer in a4 and a parameter.
- `update_entity_attribute_table`: Iterates through a loop up to 80 times, calling another routine to fetch data and then updating a series of short and longword attributes within a table at a specific offset in a structure pointed to by a4.
- `process_dual_buffer_metadata`: Iterates through a sequence of up to sixteen entries, performing string comparisons and memory copies to populate a data table.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `compare_case_insensitive_strings`: Compares two byte strings character by character, converting uppercase letters to lowercase for comparison, and returns the difference between the first mismatching characters or zero if equal.
- `find_substring`: Searches for the first occurrence of one null-terminated string within another null-terminated string and returns a pointer to the start of the match or zero if not found.
- `copy_n_terminated_bytes`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, stopping early if a counter reaches zero and returning the original destination pointer.
- `setup_buffer_pointers`: Clears a longword at offset 9420 from a pointer in a4, copies a pointer to offset 9424 of that same pointer, and calls another routine with the new pointer and a stack address.
- `increment_and_get_byte`: Increments a long value at an offset of 24236 from a pointer in a4, then retrieves a byte from the address stored at the incremented location and returns either that byte or negative one if the byte is zero.
- `decrement_global_pointers`: Decrements two long values at offsets 5eac and 5eb0 from a pointer in a4.
- `parse_range_string_data`: Parses a string to extract numeric values and checks if a value provided by another routine falls within specified numeric ranges.
- `parse_data_buffer`: Iterates through a buffer, calling another routine for each byte, and updates a counter and a pointer.
- `compare_and_store_d4`: Compares d4 against the constants 48 and 55, and if d4 is within range, stores the value of d4 into a location pointed to by a value on the stack.
- `process_buffer_and_store_result`: Calculates a value based on a bitwise AND of a register and 7, calls a routine via a pointer, and if certain conditions are met, stores the result into a structure pointed to by a3.
- `branch_on_stack_value`: Tests a longword at an offset of the stack pointer and branches if it is zero; otherwise it moves the value of d6 into a memory location and branches to another routine.
- `store_buffer_state`: Stores the contents of two registers into a location pointed to by a6 and a stack location, then returns a value incremented by one.
- `increment_stack_pointer_and_data`: Increments a byte at an offset of the stack, compares it to 94, and if they are equal, increments a longword at a stack offset and moves a pointer.
- `compare_byte_and_branch`: Compares a byte at a pointer from the stack to 93 and branches to another routine if they are not equal.
- `check_and_update_pointer`: Checks a byte at a pointer from a stack offset; if it is non-zero, it copies the value of d4 to a location at a stack offset and returns.
- `increment_and_compare_byte`: Increments a byte at an offset of the stack, compares it to 93, and if they are not equal, moves a longword from one stack offset to another.
- `compare_and_clear_byte`: Compares a byte from a pointer to a constant, and if they match and d6 is non-zero, checks if a pointer in a3 is non-null to clear a byte at its destination, then stores a value from d4 into a global variable.
- `clear_and_increment_pointer`: Compares d0 and d4; if they are equal and a pointer in a3 is not null, it clears a byte at the address in a1 (derived from a3), stores d4 in a stack location, and increments a value at a stack offset.
- `iterate_and_fill_buffer`: Loops through a buffer, calling a routine via a pointer for each byte, and if a condition is met, writes a byte to a destination pointed to by a3 and increments that pointer.
- `branch_to_data_offset`: Moves a longword from an offset of the stack to another stack offset and then branches to another location.
- `process_data_buffer`: Iterates through a buffer starting at a pointer in a stack offset, compares bytes to a constant, and updates a global variable and a pointer in the stack based on comparison results.
- `process_data_buffer_2`: Iterates through a buffer, calling a routine for each byte, and updates a pointer and a counter, eventually clearing a byte at a pointer and updating a value on the stack.
- `parse_data_buffer_2`: Iterates through a buffer, calling a routine for each byte and performing various checks against a table and constants, and calls another routine to handle specific data patterns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_6`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_7`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `decrement_resource_counter`: Decrements a 16-bit word at offset 38 of a structure in a6 and, if the result is zero and bit 3 of a byte at offset 34 is set, calls another routine.
- `set_resource_flag`: Saves registers, moves a pointer from a structure into a6, and if a word at offset 32 of that structure is zero, sets bit 3 of a byte at offset 34 of the structure.
- `release_resources`: Calls a routine via a pointer in a5, then if a value at a specific offset in a5 is non-zero, calls another routine, clears a global pointer, and clears a value at offset 44 of a5.
- `execute_callback_sequence_2`: Calls three routines, including two subroutines and one via a pointer from a hardware register, and returns.
- `cleanup_and_update_hardware_2`: Calls two subroutines, calls a routine via a pointer from a hardware register, and writes a value from a structure to a hardware register before returning.
- `execute_callback_sequence`: Loads a pointer from a global variable, sets a global variable to a value at offset 22 of that pointer, and then calls three routines using pointers found at offsets negative 222 and negative 270 from the loaded pointer.
- `cleanup_and_update_hardware`: If a global variable is non-zero, it calls two routines using pointers from a global structure, calls a third routine using a pointer from that structure and the global variable, clears the global variable, and writes a value from offset 38 of a6 to the hardware register at a fixed address.
- `synchronize_hardware_and_memory`: Calls three library routines, performs a memory swap if a value at an offset is non-zero, and then writes several bitwise-inverted values to hardware registers and calls two other routines.
- `return_negative_one`: Returns negative one in d0.
- `execute_hardware_sequence`: Writes bitwise-inverted values to hardware registers, calls two subroutines, and then performs a conditional memory swap and several library calls.
- `swap_memory_blocks`: Swaps 256 longwords between a destination table at offset 48 of a5 and a source table at a1, using a loop controlled by a value at offset 48 of a5.
- `call_status_updates`: Calls two separate routines in succession and then returns.
- `execute_sequential_subroutines`: Calls two separate subroutines in sequence and then returns.
- `open_resources`: Clears two global longwords, then calls a routine to open a resource and stores the resulting pointer in the first global variable; if successful, it calls the same routine again to open a second resource and stores the result in the second global variable.
- `query_resource_status`: If the first global variable is non-zero, it calls a routine using a pointer from that variable and stores the 16-bit result in a global variable; it then performs the same check and call for the second global variable.
- `call_two_pointers`: Checks if a pointer at a global offset is non-zero, then calls a routine at that pointer with bit 7 set in the argument.
- `update_cia_registers`: Reads two bytes from CIAA, modifies bit 4 of each byte, and writes them back to the same locations.
- `write_cia_registers`: Writes two 16-bit values from global variables to CIAB by splitting them into bytes, and writes two bytes from global variables to CIAA.
- `jump_to_hardware_routine`: Performs a long jump to a fixed address.
- `dispatch_hardware_cleanup_and_update`: Calls cleanup_and_update_hardware, executes a routine via a pointer stored in a global variable, and updates that global variable with a value from a structure offset.
- `dispatch_callback_sequence`: Calls three routines, including two subroutines and one via a pointer retrieved from a global variable with an offset of 132, then returns.
- `jump_to_callback_sequence`: Jumps to another routine that executes a sequence of three callbacks, including two subroutines and one via a pointer from a hardware register.
- `jump_to_address`: Performs an absolute jump to a specific address.
