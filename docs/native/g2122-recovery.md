# Game 2122 (zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02122). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 1 of 1 named (0 author symbols), 1 of 1 described (1 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6 of 6 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10 of 10 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 26 |
| data | 870886 |
| unused | 30208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)/Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1699 non-empty blocks: matched against track-loaded segments
- zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)/Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf: blocks 24-575 (bytes 12288-294912) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.94)
- zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)/Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf: blocks 577-581 (bytes 295424-297984) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.27)
- zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)/Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf: blocks 583-1359 (bytes 298496-696320) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.18)
- zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)/Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf: blocks 1364-1369 (bytes 698368-701440) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.10)
- zool 2 (1993)(gremlin)[cr cls][t +12 znt](disk 1 of 3)/Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf: blocks 1371-1729 (bytes 701952-885760) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.92)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Zool 2 (1993)(Gremlin)[cr CLS][t +12 ZNT](Disk 1 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
