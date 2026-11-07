# Game 902 (rally championships (1994)(flair)[cr pdy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00902). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3610064 top-level bytes in 213 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 58 of 58 named (0 author symbols), 58 of 58 described (7 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 646 of 646 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1407 of 1407 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 343004 |
| code | 5606 |
| data | 3163008 |
| padding | 55950 |
| unused | 42496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:PDY-RALLY.EXE | 3 | 160 | 5 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE | 3 | 243 | 10 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:CopStart | 1 | 14 | 1 |
| Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 791 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 179 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:PRES.EXE: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:OPTION.EXE: high-entropy data file (7.64 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Crash.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Victory.iff: high-entropy data file (7.90 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Timeout.iff: high-entropy data file (7.88 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/WorldImage.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/InsideLogo.iff: high-entropy data file (7.82 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/WorldPodio.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/SelectRally.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/OutFuel.iff: high-entropy data file (7.92 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/SinglePodio.iff: high-entropy data file (7.91 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/Mod.Racer: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:Data/practice.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:flagselect.exe: high-entropy data file (7.69 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop1.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribc.Dat: high-entropy data file (7.62 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carriba.Dat: high-entropy data file (7.70 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:PorscheScel.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop8.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop6.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:EscortScel.iff: high-entropy data file (7.86 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:DeltaScel.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Care.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop4.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribf.Dat: high-entropy data file (7.60 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carc.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop2.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carribd.Dat: high-entropy data file (7.65 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Cara.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:CalibraScel.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:CelicaScel.iff: high-entropy data file (7.89 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Copiloti.iff: high-entropy data file (7.92 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop7.iff: high-entropy data file (7.84 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:peugeotScel.iff: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carf.Dat: high-entropy data file (7.75 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop5.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Card.Dat: high-entropy data file (7.75 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Shop3.iff: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf:Carb.Dat: high-entropy data file (7.77 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:AlteScenary1.raw: high-entropy data file (7.73 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracke.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackp.tab: high-entropy data file (7.65 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw2.raw: high-entropy data file (7.70 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackc.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackn.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw0.raw: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracka.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackl.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp2.raw: high-entropy data file (7.79 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackj.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp0.raw: high-entropy data file (7.80 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackh.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:AlteScenary2.raw: high-entropy data file (7.69 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackf.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw3.raw: high-entropy data file (7.74 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackd.tab: high-entropy data file (7.64 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracko.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryDw1.raw: high-entropy data file (7.84 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackb.tab: high-entropy data file (7.68 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackm.tab: high-entropy data file (7.64 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp3.raw: high-entropy data file (7.80 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackk.tab: high-entropy data file (7.64 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:ScenaryUp1.raw: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:tracki.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf:trackg.tab: high-entropy data file (7.63 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:ScenaryDw4.raw: high-entropy data file (7.79 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackz.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:scenaryup6.raw: high-entropy data file (7.73 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:ScenaryUp4.raw: high-entropy data file (7.80 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackt.tab: high-entropy data file (7.68 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackr.tab: high-entropy data file (7.66 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:trackq.tab: high-entropy data file (7.65 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Grats1.Sampl: high-entropy data file (7.88 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod2.pack: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod1.pack: high-entropy data file (7.85 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Mano321go.raw: high-entropy data file (7.78 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod6.pack: high-entropy data file (7.88 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod3.pack: high-entropy data file (7.88 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod0.pack: high-entropy data file (7.87 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/mod4.pack: high-entropy data file (7.86 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:tracks.tab: high-entropy data file (7.67 bits/byte), classified as packed data
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:scenarydw6.raw: high-entropy data file (7.79 bits/byte), classified as packed data
- runtime segment an address: 88 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 153 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:AlteScenary6.raw: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:AlteScenary4.raw: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Jump.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Narrow.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf:data/Caution.sampl: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- rally championships (1994)(flair)[cr pdy](disk 2 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 2 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- rally championships (1994)(flair)[cr pdy](disk 3 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- rally championships (1994)(flair)[cr pdy](disk 4 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- rally championships (1994)(flair)[cr pdy](disk 1 of 4)/Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:GO.EXE: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
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

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:CopStart

- `open_and_close_library_probe`: Opens a library by name through the exec OldOpenLibrary call and, on success, stores a pointer to a shared variable in the library's base field before closing it.

### Rally Championships (1994)(Flair)[cr PDY](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_counters_and_mode_loop`: Opens a library through exec, saves one of its fields and writes the interrupt-enable register, then initialises a set of counters.
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
- `tiny_ori`: Small routine of 1 instruction that merges 5 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
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
