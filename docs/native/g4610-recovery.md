# Game 4610 (ork (1991)(psygnosis)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04610). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802368 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (0 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 12 of 12 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 46 of 46 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 122975 |
| code | 222 |
| data | 1679171 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf:bootblock | 1 | 26 | 1 |
| logged disk load from Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |

## Cases handled

- ork (1991)(psygnosis)[cr skr](disk 1 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- ork (1991)(psygnosis)[cr skr](disk 2 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- ork (1991)(psygnosis)[cr skr](disk 1 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf: the boot block reads 35328 bytes from disk offset 524288 to an address (found statically); recovered as a unit
- ork (1991)(psygnosis)[cr skr](disk 1 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf: the boot block reads 92672 bytes from disk offset 393216 to an address (found statically); recovered as a unit
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ork (1991)(psygnosis)[cr skr](disk 1 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf: 2 logged loads cover 128000 bytes of the image
- ork (1991)(psygnosis)[cr skr](disk 1 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 949-1023 (bytes 485888-524288) hold ASCII text
- ork (1991)(psygnosis)[cr skr](disk 1 of 2)/Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1093-1759 (bytes 559616-901120) hold ASCII text
- 2 packed track region(s), 1292288 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Ork (1991)(Psygnosis)[cr SKR](Disk 1 of 2).adf:bootblock

- `initialize_dma_and_interrupts`: Sets up DMA and interrupt controllers with specific values, performs memory allocation, and updates a global pointer with the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_render_data`: Sets multiple global variables to a fixed value and performs a loop that copies data between memory regions based on conditions.
