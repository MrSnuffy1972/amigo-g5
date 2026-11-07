# Game 857 (adventures of robin hood, the (1991)(millennium)(m4)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00857). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901280 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 2 of 2 named (0 author symbols), 2 of 2 described (0 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3 of 3 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 42 of 42 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22700 |
| code | 150 |
| data | 785758 |
| unused | 92672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf:bootblock | 1 | 8 | 1 |
| logged disk load from Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 34 | 1 |

## Cases handled

- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: custom-format (non-DOS) disk, 1577 non-empty blocks: matched against track-loaded segments
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: the boot block reads 23040 bytes from disk offset 878080 to an address (found statically); recovered as a unit
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: 1 logged loads cover 23040 bytes of the image
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 294-300 (bytes 150528-154112) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.42)
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 490-519 (bytes 250880-266240) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.09)
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 527-556 (bytes 269824-285184) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.10)
- adventures of robin hood, the (1991)(millennium)(m4)[cr skr]/Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf: blocks 564-565 (bytes 288768-289792) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.07)
- 25 packed track region(s), 749056 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Adventures of Robin Hood, The (1991)(Millennium)(M4)[cr SKR].adf:bootblock

- `load_disk_data_and_init`: Fills a disk read request for 23040 bytes from a fixed offset into a load address, submits it through a device call, then zeroes 81 longwords, copies 31 longwords into place and calls a routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decode_table_coded_rows`: When a block counter expires, unpacks 16 rows of words from a byte stream, each value being a byte plus or minus a lookup-table entry, written at a fixed row stride.
