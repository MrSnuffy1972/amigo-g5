# Game 3577 (extase (1990)(virgin)[cr pdx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03577). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (0 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 22 of 22 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 80 |
| data | 901040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Extase (1990)(Virgin)[cr PDX].adf:bootblock | 1 | 22 | 1 |

## Cases handled

- extase (1990)(virgin)[cr pdx]/Extase (1990)(Virgin)[cr PDX].adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- 1 packed track region(s), 900096 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Extase (1990)(Virgin)[cr PDX].adf:bootblock

- `load_boot_block`: Loads and processes the boot block by executing I/O operations until successful, then scans for a specific hunk type code and jumps to the loaded code.
