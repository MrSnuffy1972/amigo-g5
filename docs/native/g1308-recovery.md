# Game 1308 (laser squad (1989)(microillusions)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01308). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8 of 8 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 30 |
| data | 900578 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Laser Squad (1989)(MicroIllusions)[a].adf:bootblock | 1 | 8 | 2 |

## Cases handled

- laser squad (1989)(microillusions)[a]/Laser Squad (1989)(MicroIllusions)[a].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- laser squad (1989)(microillusions)[a]/Laser Squad (1989)(MicroIllusions)[a].adf: blocks 2-1289 (bytes 1024-660480) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.95)
- 1 packed track region(s), 240128 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Laser Squad (1989)(MicroIllusions)[a].adf:bootblock

- `jump_to_unnamed_routine`: Boot block entry of one instruction that transfers control to a single unnamed routine.
- `dispatch_to_unnamed_routine`: Seven-instruction boot block routine whose address is taken by a pc-relative load and which is called by two unnamed routines.
